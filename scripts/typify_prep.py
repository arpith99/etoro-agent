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

Outputs to /tmp/etoro-<domain>-typify.json.
"""
import copy
import json
import sys
from pathlib import Path

DOCS_DIR = Path("docs")
TMP_DIR = Path("/tmp")

if not DOCS_DIR.is_dir():
    sys.exit(f"docs/ not found at {DOCS_DIR.resolve()} (run from etoro-agent/)")

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
            else:
                # No type, no $ref -> just allow null
                node["type"] = "null"
            # If there's an enum and we made the type nullable, add null to the enum too
            if "enum" in node and isinstance(node["enum"], list) and None not in node["enum"]:
                node["enum"] = node["enum"] + [None]
        for v in node.values():
            normalize_nullable_in_place(v)
    elif isinstance(node, list):
        for item in node:
            normalize_nullable_in_place(item)


def normalize_enums_in_place(node) -> None:
    """
    Two malformed enum patterns appear in the eToro spec:

    1. `{type: "integer", enum: ["Open", "Close"]}` — the enum values are strings
       but the type says integer. The intent is "wire format is integer; these are
       the variant names". Standard JSON Schema rejects this. Convert to plain
       string enums; the actual wire format may need a #[serde_repr] hand-fix
       later.
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
    # Order matters: strip annotations & rewrite refs first; then handle enums
    # (which may rename a kept extension); then normalize nullability so the
    # nullable handling sees the latest type/enum shape.
    rewrite_refs_in_place(defs)
    normalize_enums_in_place(defs)
    normalize_nullable_in_place(defs)

    out_doc = {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$defs": defs,
    }

    out_path = TMP_DIR / f"etoro-{domain}-typify.json"
    out_path.write_text(json.dumps(out_doc, indent=2))

    extra = sorted(all_needed - own_names)
    msg = f"  {domain}: own={len(own_names)}, total={len(all_needed)}"
    if extra:
        msg += f" (+inlined: {', '.join(extra)})"
    print(msg)
    print(f"    -> {out_path}")
