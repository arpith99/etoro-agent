#!/usr/bin/env python3
"""
Preprocess our per-domain OpenAPI schema fragments into typify-compatible
JSON Schema 2020-12 documents.

For each input docs/<domain>-schema.json:
  1. Strip the _meta block.
  2. Compute the transitive $ref closure across all sibling files (so the
     output is self-contained even when a domain references types defined
     elsewhere).
  3. Rewrite $ref paths from "#/components/schemas/Foo" -> "#/$defs/Foo".
  4. Wrap in {"$schema": ..., "$defs": {...}} so cargo typify can consume it.

Usage: typify_prep.py [OUTPUT_DIR]

Writes <OUTPUT_DIR>/etoro-<domain>-typify.json. When OUTPUT_DIR is omitted a
fresh private temporary directory is created (never a fixed path under /tmp,
which another local user could pre-create or swap between our write and the
cargo typify read).
"""
import copy
import json
import re
import sys
import tempfile
from pathlib import Path

DOCS_DIR = Path("docs")
CARGO_TOML = Path("Cargo.toml")

if not DOCS_DIR.is_dir():
    sys.exit(f"docs/ not found at {DOCS_DIR.resolve()} (run from etoro-agent/)")

if len(sys.argv) > 2:
    sys.exit(__doc__)
OUT_DIR = Path(sys.argv[1]) if len(sys.argv) == 2 else Path(tempfile.mkdtemp(prefix="etoro-typify-"))
OUT_DIR.mkdir(parents=True, exist_ok=True)


def crate_version() -> str:
    """The [package] version from Cargo.toml — the single source of truth that
    every x-rust-type annotation must agree with. typify treats the version as
    a semver *requirement*; if it is not satisfied by the --crate flag passed
    to cargo typify, the override is silently ignored and typify falls back
    to generating its own type (f64 for numbers, a fresh enum, ...)."""
    match = re.search(r'^\[package\]\s*$.*?^version\s*=\s*"([^"]+)"', CARGO_TOML.read_text(), re.M | re.S)
    if not match:
        sys.exit("could not find [package] version in Cargo.toml")
    return match.group(1)


CRATE_NAME = "etoro-agent"
CRATE_VERSION = crate_version()

# 1. Build pool of all schemas from all domain files
pool: dict = {}
domain_files: list[Path] = []
for f in sorted(DOCS_DIR.glob("*-schema.json")):
    if "all-schemas-index" in f.name:
        continue
    domain_files.append(f)
    data = json.loads(f.read_text())
    for k, v in data.items():
        if k == "_meta":
            continue
        if k in pool and pool[k] != v:
            print(f"WARN: schema {k} differs between files; using first definition", file=sys.stderr)
        pool.setdefault(k, v)

print(f"Pool: {len(pool)} unique schemas across {len(domain_files)} domain files")


def find_refs(node, out: set) -> None:
    if isinstance(node, dict):
        ref = node.get("$ref")
        if isinstance(ref, str) and ref.startswith("#/components/schemas/"):
            out.add(ref.rsplit("/", 1)[-1])
        for v in node.values():
            find_refs(v, out)
    elif isinstance(node, list):
        for item in node:
            find_refs(item, out)


def transitive_deps(seed: set, pool: dict) -> set:
    visited = set(seed)
    queue = list(seed)
    while queue:
        name = queue.pop()
        if name not in pool:
            print(f"  WARN: dangling ref to {name!r} (not in pool)", file=sys.stderr)
            continue
        refs: set = set()
        find_refs(pool[name], refs)
        for r in refs - visited:
            visited.add(r)
            queue.append(r)
    return visited


def normalize_nullable_in_place(node) -> None:
    """Convert OpenAPI 3.0 `nullable: true` to JSON Schema 2020-12 null type union."""
    if isinstance(node, dict):
        if node.pop("nullable", None) is True:
            current_type = node.get("type")
            if isinstance(current_type, str):
                node["type"] = [current_type, "null"]
            elif isinstance(current_type, list):
                if "null" not in current_type:
                    current_type.append("null")
            elif "$ref" in node:
                # Bare $ref + nullable -> oneOf with null branch
                ref = node.pop("$ref")
                node.setdefault("oneOf", []).extend([{"$ref": ref}, {"type": "null"}])
            elif not any(k in node for k in ("oneOf", "anyOf", "allOf")):
                # No type at all: the spec means "anything, or null". Forcing
                # `type: null` here would turn it into "null only" (typify then
                # emits `()`, rejecting every real value — Discussion.reason hit
                # this). Leaving it untyped is already null-inclusive; typify
                # maps it to serde_json::Value.
                pass
            # If there's an enum and we made the type nullable, add null to the enum too
            if "enum" in node and isinstance(node["enum"], list) and None not in node["enum"]:
                node["enum"] = node["enum"] + [None]
        for v in node.values():
            normalize_nullable_in_place(v)
    elif isinstance(node, list):
        for item in node:
            normalize_nullable_in_place(item)


def x_rust_type(path: str) -> dict:
    return {"crate": CRATE_NAME, "version": CRATE_VERSION, "path": path}


NUMERIC_X_RUST_TYPE = x_rust_type("etoro_agent::types::manual::Numeric")

# Cross-domain types: each appears in 2+ schema files. To avoid generating
# distinct-but-identical Rust structs in each module (so e.g. trading::Market
# != feeds_posts::Market for type-checking), pick one owner module per type.
# Non-owning modules redirect to the owner via x-rust-type.
SHARED_TYPE_OWNERS = {
    "Market":              "market_data",
    "Avatar":              "market_data",
    "Svg":                 "market_data",
    "MarketEventMetadata": "market_data",
    "Mirror":              "trading",
    "Order":               "trading",
    "OrderForOpen":        "trading",
    "OrderForClose":       "trading",
    "OrderForCloseMultiple":"trading",
    "OrderMetadata":       "trading",
    "Position":            "trading",
    "TradeMetadata":       "trading",
    "User":                "identity",
}


def redirect_shared_types_in_place(defs: dict, current_domain: str) -> None:
    """Stamp x-rust-type on shared types when this module is NOT the owner,
    so typify references the owner module's struct instead of generating a
    duplicate one here."""
    for name, owner in SHARED_TYPE_OWNERS.items():
        if name in defs and current_domain != owner and "x-rust-type" not in defs[name]:
            defs[name]["x-rust-type"] = x_rust_type(f"etoro_agent::types::{owner}::{name}")


def pin_x_rust_type_versions_in_place(node) -> None:
    """Hand-stamped x-rust-type annotations in docs/*-schema.json carry a
    literal version. Rewrite every one that targets this crate to the current
    Cargo.toml version so a version bump cannot silently disable overrides."""
    if isinstance(node, dict):
        override = node.get("x-rust-type")
        if isinstance(override, dict) and override.get("crate") == CRATE_NAME:
            override["version"] = CRATE_VERSION
        for v in node.values():
            pin_x_rust_type_versions_in_place(v)
    elif isinstance(node, list):
        for item in node:
            pin_x_rust_type_versions_in_place(item)


def annotate_numeric_in_place(node) -> None:
    """
    Bulk-redirect every `{type: "number"}` schema (with any format or none) to
    our `Numeric` newtype (Decimal with exact JSON-number serde). Without
    this every monetary / rate / quantity field would land as f32 / f64 in the
    generated code, which loses precision well before realistic trading-account
    sizes.

    A handful of `type: number` fields (e.g. putTradeRequest.positionId) are
    really integers per usage, but Decimal represents them losslessly too —
    the cost of the broad rule is minor and the safety upside is large.

    Skips schemas that already carry x-rust-type so callers can pin individual
    fields to a different type if needed.
    """
    if isinstance(node, dict):
        if node.get("type") == "number" and "x-rust-type" not in node:
            node["x-rust-type"] = dict(NUMERIC_X_RUST_TYPE)
        for v in node.values():
            annotate_numeric_in_place(v)
    elif isinstance(node, list):
        for item in node:
            annotate_numeric_in_place(item)


def normalize_enums_in_place(node) -> None:
    """
    Two malformed enum patterns appear in the eToro spec:

    1. `{type: "integer", enum: ["Open", "Close"]}` — the enum values are strings
       but the type says integer. The intent is "wire format is integer; these are
       the variant names". Standard JSON Schema rejects this. Schemas that carry
       an x-rust-type override (hand-stamped in docs/*-schema.json, pointing at
       an int-or-string enum in src/types/manual.rs) are left alone; anything
       else is converted to a plain *string* enum so JSON Schema accepts it —
       and, being string-only, would reject an integer on the wire. Add a
       manual override when adding such a schema.
    2. `{type: "integer", enum: [1, 2], x-enumNames: ["Open", "Closed"]}` —
       well-formed integer enum with name annotations as an OpenAPI extension.
       typify's known enum-name extension is `x-enum-varnames` (not
       `x-enumNames`); rename so the names survive.
    """
    if isinstance(node, dict):
        enum = node.get("enum")
        type_ = node.get("type")
        if isinstance(enum, list) and enum:
            if type_ == "integer" and all(isinstance(v, str) for v in enum):
                # If the schema is being overridden via x-rust-type, leave it
                # alone — typify won't generate a Rust type for it. Otherwise
                # convert to a plain string enum so JSON Schema accepts it.
                if "x-rust-type" not in node:
                    node["type"] = "string"
            elif type_ == "integer" and all(isinstance(v, int) or v is None for v in enum):
                if "x-enumNames" in node and "x-enum-varnames" not in node:
                    node["x-enum-varnames"] = node.pop("x-enumNames")
        for v in node.values():
            normalize_enums_in_place(v)
    elif isinstance(node, list):
        for item in node:
            normalize_enums_in_place(item)


def rewrite_refs_in_place(node) -> None:
    if isinstance(node, dict):
        # Strip our human-only annotations (keys starting with _) that typify rejects
        for k in [k for k in node if isinstance(k, str) and k.startswith("_")]:
            del node[k]
        # Strip OpenAPI extensions (x-*) except those we deliberately keep
        KEEP_EXTENSIONS = {"x-enum-varnames", "x-rust-type"}
        for k in [k for k in node if isinstance(k, str) and k.startswith("x-") and k not in KEEP_EXTENSIONS]:
            del node[k]
        ref = node.get("$ref")
        if isinstance(ref, str):
            node["$ref"] = ref.replace("#/components/schemas/", "#/$defs/")
        for v in node.values():
            rewrite_refs_in_place(v)
    elif isinstance(node, list):
        for item in node:
            rewrite_refs_in_place(item)


# 2. For each domain, build self-contained JSON Schema doc
for f in domain_files:
    domain = f.stem.replace("-schema", "")
    data = json.loads(f.read_text())
    own_names = {k for k in data if k != "_meta"}
    all_needed = transitive_deps(own_names, pool)

    defs = {name: copy.deepcopy(pool[name]) for name in sorted(all_needed) if name in pool}
    # Order matters: enums first, because normalize_enums renames the OpenAPI
    # `x-enumNames` extension to typify's `x-enum-varnames` and rewrite_refs
    # strips every x-* it does not know (so it must see the renamed key).
    # Nullability last so it sees the final type/enum shape.
    normalize_enums_in_place(defs)
    rewrite_refs_in_place(defs)
    redirect_shared_types_in_place(defs, domain)
    pin_x_rust_type_versions_in_place(defs)
    annotate_numeric_in_place(defs)
    normalize_nullable_in_place(defs)

    out_doc = {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$defs": defs,
    }

    out_path = OUT_DIR / f"etoro-{domain}-typify.json"
    out_path.write_text(json.dumps(out_doc, indent=2))

    extra = sorted(all_needed - own_names)
    msg = f"  {domain}: own={len(own_names)}, total={len(all_needed)}"
    if extra:
        msg += f" (+inlined: {', '.join(extra)})"
    print(msg)
    print(f"    -> {out_path}")
