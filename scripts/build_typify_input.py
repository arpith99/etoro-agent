#!/usr/bin/env python3
"""
Build the cargo-typify input from the committed spec snapshot.

Replaces the old scripts/typify_prep.py. The transforms it applied are still
here and still run in the same order; what is gone is cross-file pooling and
the hand-maintained SHARED_TYPE_OWNERS map, both of which existed only because
schemas used to be split across seven hand-curated files. Upstream keeps one
flat namespace of component schemas, so we generate one Rust module from it and
express the API's tags as thin facade modules that re-export their subset. That
means exactly one `Instrument` type instead of one per tag that mentions it.

Inputs:
    docs/spec/schemas.json     upstream component schemas (verbatim)
    docs/spec/operations.json  operation index, used to seed each tag's closure
    docs/overrides/*.json      hand-authored x-rust-type stamps

Outputs:
    <workdir>/etoro-components.json   self-contained JSON Schema 2020-12 doc
    docs/domains.json                 tag -> module slug + schema closure
    src/types/tags.rs                 generated facade modules

Usage:
    python3 scripts/build_typify_input.py WORKDIR   # pass 1: typify input
    python3 scripts/build_typify_input.py --emit-tags  # pass 2: facades

Two passes because typify renames schemas (meResponse -> MeResponse), so the
facades can only be written once the generated file exists to check against.
"""

import copy
import json
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
SPEC_DIR = REPO_ROOT / "docs" / "spec"
OVERRIDES_DIR = REPO_ROOT / "docs" / "overrides"
DOMAINS_PATH = REPO_ROOT / "docs" / "domains.json"
TAGS_RS_PATH = REPO_ROOT / "src" / "types" / "tags.rs"
CARGO_TOML = REPO_ROOT / "Cargo.toml"

CRATE_NAME = "etoro-agent"
REF_PREFIX = "#/components/schemas/"

# Every tag in the v1.355.0 spec. Kept explicit so a new upstream tag shows up
# as a test failure (unknown tag) rather than a silently missing module.
ALL_TAGS = [
    "Agent Portfolios",
    "App Data",
    "Balances",
    "Cash Accounts",
    "Clubs",
    "Copy Trading",
    "Copy Trading - Demo",
    "Identity",
    "Market Data",
    "Notifications",
    "PI Data",
    "PortfolioSearch",
    "Price Alerts",
    "Rankings",
    "SSO - Applications",
    "SSO - Scopes",
    "Social Feeds",
    "Sub-Accounts",
    "Sub-Accounts - eToro Trading",
    "Top Assets",
    "Trading - Demo",
    "Trading - Real",
    "Transfer",
    "User Stats",
    "Users Info",
    "Watchlists",
]

# Tags whose mechanical slug would be unreadable or ambiguous. Anything not
# listed here goes through the generic rule.
SLUG_OVERRIDES = {
    "PortfolioSearch": "portfolio_search",
    "PI Data": "pi_data",
    "SSO - Applications": "sso_applications",
    "SSO - Scopes": "sso_scopes",
}


def crate_version() -> str:
    """The [package] version from Cargo.toml.

    typify treats an x-rust-type version as a semver *requirement*: if the
    --crate flag passed to cargo typify does not satisfy it, the override is
    silently ignored and typify generates its own type instead (f64 for
    numbers). Every annotation is therefore pinned to this one value.
    """
    match = re.search(r'^\[package\]\s*$.*?^version\s*=\s*"([^"]+)"', CARGO_TOML.read_text(), re.M | re.S)
    if not match:
        sys.exit("could not find [package] version in Cargo.toml")
    return match.group(1)


def rust_type_name(schema_name: str) -> str:
    """The Rust identifier typify generates for a component schema.

    typify runs schema names through heck's UpperCamelCase, so `meResponse`
    becomes `MeResponse` and `AgentPortfolioApi_ErrorResponse` becomes
    `AgentPortfolioApiErrorResponse`. The facades re-export by the generated
    name, not the schema name, so this has to agree with heck exactly -- and
    emit_tags verifies every result against the real generated file rather
    than trusting it.
    """
    words: list = []
    for token in re.split(r"[^A-Za-z0-9]+", schema_name):
        if not token:
            continue
        # Split on lower/digit -> upper, and on an uppercase run followed by a
        # lowercase (so "NOCFoo" splits as "NOC" + "Foo").
        token = re.sub(r"([a-z0-9])([A-Z])", r"\1 \2", token)
        token = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1 \2", token)
        words.extend(token.split())
    return "".join(word[:1].upper() + word[1:].lower() for word in words)


def slugify(tag: str) -> str:
    if tag in SLUG_OVERRIDES:
        return SLUG_OVERRIDES[tag]
    slug = re.sub(r"[^a-z0-9]+", "_", tag.lower()).strip("_")
    return slug


# ---------------------------------------------------------------------------
# Override layer
# ---------------------------------------------------------------------------


def load_overrides(items) -> dict:
    """Merge override files into one {schema name: partial schema} mapping.

    `items` is a sequence of (filename, parsed JSON). Grouping into files is
    organizational only, so the same schema being stamped twice is an error
    rather than a last-writer-wins merge.
    """
    stamps: dict = {}
    origin: dict = {}
    for filename, payload in items:
        for name, partial in (payload.get("schemas") or {}).items():
            if name in stamps:
                raise ValueError(f"{name} is stamped in both {origin[name]} and {filename}")
            stamps[name] = partial
            origin[name] = filename
    return stamps


def _deep_merge(base: dict, patch: dict) -> dict:
    out = copy.deepcopy(base)
    for key, value in patch.items():
        if isinstance(value, dict) and isinstance(out.get(key), dict):
            out[key] = _deep_merge(out[key], value)
        else:
            out[key] = copy.deepcopy(value)
    return out


def apply_overrides(schemas: dict, stamps: dict) -> dict:
    """Deep-merge hand-authored partials onto the upstream schemas.

    An override naming a schema that no longer exists is a hard error. That is
    the guard for the dangerous case: upstream renames a type, the stamp stops
    matching, and a Numeric or manual-enum override silently disappears.
    """
    unknown = sorted(set(stamps) - set(schemas))
    if unknown:
        raise ValueError(f"override targets unknown schema: {', '.join(unknown)}")
    merged = copy.deepcopy(schemas)
    for name, partial in stamps.items():
        merged[name] = _deep_merge(merged[name], partial)
    return merged


# ---------------------------------------------------------------------------
# Reference closure
# ---------------------------------------------------------------------------


def find_refs(node, out: set) -> None:
    if isinstance(node, dict):
        ref = node.get("$ref")
        if isinstance(ref, str) and ref.startswith(REF_PREFIX):
            out.add(ref[len(REF_PREFIX):])
        for value in node.values():
            find_refs(value, out)
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
        for ref in refs - visited:
            visited.add(ref)
            queue.append(ref)
    return visited


def build_domains(operations, schemas: dict) -> dict:
    """Map each tag to its module slug and full schema closure."""
    domains: dict = {}
    reached: set = set()
    for op in operations:
        for tag in op.get("tags") or []:
            entry = domains.setdefault(tag, {"slug": slugify(tag), "operations": [], "schemas": set()})
            entry["operations"].append(f"{(op.get('method') or '').upper()} {op.get('path')}")
            seeds = {
                ref[len(REF_PREFIX):]
                for ref in (op.get("requestSchemas") or []) + (op.get("responseSchemas") or [])
                if isinstance(ref, str) and ref.startswith(REF_PREFIX)
            }
            entry["schemas"] |= transitive_deps(seeds, schemas)

    for entry in domains.values():
        entry["operations"] = sorted(set(entry["operations"]))
        entry["schemas"] = sorted(entry["schemas"])
        reached |= set(entry["schemas"])

    domains["_orphans"] = sorted(set(schemas) - reached)
    return domains


# ---------------------------------------------------------------------------
# Transforms (order is load-bearing; see main)
# ---------------------------------------------------------------------------


def normalize_enums(node) -> None:
    """Two malformed enum patterns appear in the eToro spec.

    1. `{type: integer, enum: ["Open", "Close"]}` -- values are strings but the
       type says integer. The intent is "wire format is integer; these are the
       variant names". Schemas carrying an x-rust-type override (pointing at an
       int-or-string enum in src/types/manual.rs) are left alone; anything else
       becomes a plain string enum so JSON Schema accepts it.
    2. `{type: integer, enum: [1, 2], x-enumNames: [...]}` -- well-formed, but
       typify's extension is `x-enum-varnames`, so rename it before
       rewrite_refs strips every x-* it does not recognise.
    """
    if isinstance(node, dict):
        enum = node.get("enum")
        type_ = node.get("type")
        if isinstance(enum, list) and enum:
            if type_ == "integer" and all(isinstance(v, str) for v in enum):
                if "x-rust-type" not in node:
                    node["type"] = "string"
            elif type_ == "integer" and all(isinstance(v, int) or v is None for v in enum):
                if "x-enumNames" in node and "x-enum-varnames" not in node:
                    node["x-enum-varnames"] = node.pop("x-enumNames")
        for value in node.values():
            normalize_enums(value)
    elif isinstance(node, list):
        for item in node:
            normalize_enums(item)


def rewrite_refs(node) -> None:
    """Point $refs at $defs and drop annotations typify would reject."""
    keep = {"x-enum-varnames", "x-rust-type"}
    if isinstance(node, dict):
        for key in [k for k in node if isinstance(k, str) and k.startswith("_")]:
            del node[key]
        for key in [k for k in node if isinstance(k, str) and k.startswith("x-") and k not in keep]:
            del node[key]
        ref = node.get("$ref")
        if isinstance(ref, str):
            node["$ref"] = ref.replace(REF_PREFIX, "#/$defs/")
        for value in node.values():
            rewrite_refs(value)
    elif isinstance(node, list):
        for item in node:
            rewrite_refs(item)


def pin_x_rust_type_versions(node, version: str) -> None:
    if isinstance(node, dict):
        override = node.get("x-rust-type")
        if isinstance(override, dict) and override.get("crate") == CRATE_NAME:
            override["version"] = version
        for value in node.values():
            pin_x_rust_type_versions(value, version)
    elif isinstance(node, list):
        for item in node:
            pin_x_rust_type_versions(item, version)


def annotate_numeric(node, version: str) -> None:
    """Redirect every `type: number` to the exact-decimal Numeric newtype.

    Without this, every monetary / rate / quantity field lands as f32 or f64 in
    the generated code, losing precision well before realistic account sizes.
    Schemas that already carry an x-rust-type are skipped so individual fields
    can be pinned elsewhere.
    """
    numeric = {"crate": CRATE_NAME, "version": version, "path": "etoro_agent::types::manual::Numeric"}
    if isinstance(node, dict):
        if node.get("type") == "number" and "x-rust-type" not in node:
            node["x-rust-type"] = dict(numeric)
        for value in node.values():
            annotate_numeric(value, version)
    elif isinstance(node, list):
        for item in node:
            annotate_numeric(item, version)


def normalize_exclusive_bounds(node) -> None:
    """Convert draft-04 / OpenAPI 3.0 exclusive bounds to JSON Schema 2020-12.

    The older dialect spells an exclusive bound as a boolean modifier on
    `minimum` / `maximum`; 2020-12 makes `exclusiveMinimum` / `exclusiveMaximum`
    carry the bound themselves. typify parses 2020-12 and rejects the boolean
    with a bare "data did not match any variant of untagged enum Schema", which
    points at a line number rather than the offending keyword.
    """
    if isinstance(node, dict):
        for bound, exclusive in (("minimum", "exclusiveMinimum"), ("maximum", "exclusiveMaximum")):
            if isinstance(node.get(exclusive), bool):
                is_exclusive = node.pop(exclusive)
                if is_exclusive and bound in node:
                    node[exclusive] = node.pop(bound)
        for value in node.values():
            normalize_exclusive_bounds(value)
    elif isinstance(node, list):
        for item in node:
            normalize_exclusive_bounds(item)


def normalize_nullable(node) -> None:
    """Convert OpenAPI 3.0 `nullable: true` into a JSON Schema null union."""
    if isinstance(node, dict):
        if node.pop("nullable", None) is True:
            current = node.get("type")
            if isinstance(current, str):
                node["type"] = [current, "null"]
            elif isinstance(current, list):
                if "null" not in current:
                    current.append("null")
            elif "$ref" in node:
                ref = node.pop("$ref")
                node.setdefault("oneOf", []).extend([{"$ref": ref}, {"type": "null"}])
            elif not any(k in node for k in ("oneOf", "anyOf", "allOf")):
                # No type at all: the spec means "anything, or null". Forcing
                # type:null here would mean "null only" -- typify then emits
                # `()`, rejecting every real value (Discussion.reason hit this).
                # Untyped is already null-inclusive; typify maps it to Value.
                pass
            if "enum" in node and isinstance(node["enum"], list) and None not in node["enum"]:
                node["enum"] = node["enum"] + [None]
        for value in node.values():
            normalize_nullable(value)
    elif isinstance(node, list):
        for item in node:
            normalize_nullable(item)


# ---------------------------------------------------------------------------
# Facade module generation
# ---------------------------------------------------------------------------


def generated_type_names(components_rs: Path) -> set:
    """Public item names actually emitted by cargo typify."""
    pattern = re.compile(r"^pub (?:struct|enum|type) ([A-Za-z0-9_]+)", re.M)
    return set(pattern.findall(components_rs.read_text()))


def override_paths(stamps: dict) -> dict:
    """schema name -> in-crate path, for schemas typify does not generate.

    A schema carrying an x-rust-type is deliberately *not* emitted into
    components.rs; typify points at our type instead. The facades therefore
    have to re-export it from that path (manual::) rather than components::.
    """
    paths = {}
    for name, partial in stamps.items():
        path = (partial.get("x-rust-type") or {}).get("path")
        if path:
            paths[name] = path.replace("etoro_agent::", "crate::", 1)
    return paths


def render_tags_rs(domains: dict, available: set, overrides: dict | None = None) -> str:
    """One facade module per tag, re-exporting the types its operations reach.

    Resolution is checked against the real generated names: a schema that maps
    to no emitted type and has no override is an error, not a silently missing
    re-export.
    """
    overrides = overrides or {}
    unresolved: dict = {}
    lines = [
        "// Generated by scripts/build_typify_input.py -- do not edit by hand.",
        "//",
        "// One module per eToro API tag, re-exporting the types that tag's",
        "// operations reach. Every type is defined once in super::components;",
        "// these are views onto it, so a type shared by several tags stays a",
        "// single Rust type.",
        "",
    ]
    for tag in sorted(t for t in domains if not t.startswith("_")):
        entry = domains[tag]
        resolved = []
        for schema_name in entry["schemas"]:
            if schema_name in overrides:
                resolved.append(overrides[schema_name])
                continue
            rust_name = rust_type_name(schema_name)
            if rust_name in available:
                resolved.append(f"crate::types::components::{rust_name}")
            else:
                unresolved.setdefault(schema_name, rust_name)
        if not resolved:
            continue
        lines.append(f"/// `{tag}`")
        lines.append(f"pub mod {entry['slug']} {{")
        for path in sorted(set(resolved)):
            lines.append(f"    pub use {path};")
        lines.append("}")
        lines.append("")

    if unresolved:
        detail = ", ".join(f"{schema} -> {rust}" for schema, rust in sorted(unresolved.items()))
        raise ValueError(f"no generated type for: {detail}")
    return "\n".join(lines)


def emit_tags() -> int:
    """Second pass: write the facades once typify has produced real names."""
    components_rs = REPO_ROOT / "src" / "types" / "components.rs"
    if not components_rs.exists():
        sys.exit(f"error: {components_rs} not found; run cargo typify first")
    domains = json.loads(DOMAINS_PATH.read_text())
    stamps = load_overrides([(p.name, json.loads(p.read_text())) for p in sorted(OVERRIDES_DIR.glob("*.json"))])
    TAGS_RS_PATH.write_text(
        render_tags_rs(domains, generated_type_names(components_rs), override_paths(stamps))
    )
    modules = sum(1 for line in TAGS_RS_PATH.read_text().splitlines() if line.startswith("pub mod "))
    print(f"Wrote {TAGS_RS_PATH} ({modules} tag modules)")
    return 0


def main() -> int:
    if len(sys.argv) == 2 and sys.argv[1] == "--emit-tags":
        return emit_tags()
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    workdir = Path(sys.argv[1])
    workdir.mkdir(parents=True, exist_ok=True)

    version = crate_version()
    schemas = json.loads((SPEC_DIR / "schemas.json").read_text())
    operations = json.loads((SPEC_DIR / "operations.json").read_text())

    override_files = [(p.name, json.loads(p.read_text())) for p in sorted(OVERRIDES_DIR.glob("*.json"))]
    stamps = load_overrides(override_files)
    schemas = apply_overrides(schemas, stamps)
    print(f"Applied {len(stamps)} override stamp(s) from {len(override_files)} file(s)")

    unknown_tags = sorted({t for op in operations for t in (op.get("tags") or [])} - set(ALL_TAGS))
    if unknown_tags:
        sys.exit(f"error: spec has tags not listed in ALL_TAGS: {', '.join(unknown_tags)}")

    domains = build_domains(operations, schemas)
    DOMAINS_PATH.write_text(json.dumps(domains, indent=2, ensure_ascii=False) + "\n")
    orphans = domains["_orphans"]
    print(f"Mapped {len(domains) - 1} tags; {len(orphans)} schema(s) reached by no operation")
    if orphans:
        print("  orphans: " + ", ".join(orphans))

    defs = copy.deepcopy(schemas)
    # Order matters. Enums first: the x-enumNames -> x-enum-varnames rename must
    # happen before rewrite_refs strips unknown x-* keys. Nullability last, so
    # it sees the final type/enum shape.
    normalize_enums(defs)
    rewrite_refs(defs)
    pin_x_rust_type_versions(defs, version)
    annotate_numeric(defs, version)
    normalize_exclusive_bounds(defs)
    normalize_nullable(defs)

    out_path = workdir / "etoro-components.json"
    out_path.write_text(
        json.dumps({"$schema": "https://json-schema.org/draft/2020-12/schema", "$defs": defs}, indent=2) + "\n"
    )
    print(f"Wrote {out_path} ({len(defs)} schemas)")
    print("Run with --emit-tags after cargo typify to write src/types/tags.rs")
    return 0


if __name__ == "__main__":
    sys.exit(main())
