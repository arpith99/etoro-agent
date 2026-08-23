#!/usr/bin/env python3
"""
Assemble and validate the committed eToro OpenAPI snapshot in docs/spec/.

This script does NOT reach the network. The upstream document is only
available through the eToro API Docs MCP server -- there is no public URL a
shell script can fetch (see scripts/README.md for the probe results) -- so
retrieval is agent-driven: an agent with MCP access writes chunk files into
docs/spec/.staging/ and then runs this script to merge, validate, and install
them. Splitting it this way keeps the un-automatable leg small and makes the
result verifiable offline by anyone.

Staging layout produced by the /refresh-spec command:

    docs/spec/.staging/meta.json           {openapi, info, servers, source, counts}
    docs/spec/.staging/schemas-NN.json     object: schema name -> schema
    docs/spec/.staging/operations-NN.json  array of operation records

Usage:
    python3 scripts/fetch_spec.py            assemble staging -> docs/spec/
    python3 scripts/fetch_spec.py --check    re-validate the committed snapshot
"""

import argparse
import datetime
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
SPEC_DIR = REPO_ROOT / "docs" / "spec"
STAGING_DIR = SPEC_DIR / ".staging"
REF_PREFIX = "#/components/schemas/"


def assemble_schemas(chunks) -> dict:
    """Merge schema chunks into one name-sorted mapping.

    Duplicate names are an error rather than a last-writer-wins merge: a name
    appearing twice means the chunk ranges overlapped, which would also mean
    some other range was skipped.
    """
    merged: dict = {}
    for index, chunk in enumerate(chunks):
        if not isinstance(chunk, dict):
            raise ValueError(f"chunk {index} is not a JSON object")
        for name, schema in chunk.items():
            if name in merged:
                raise ValueError(f"duplicate schema: {name}")
            merged[name] = schema
    return {name: merged[name] for name in sorted(merged)}


def assemble_operations(chunks) -> list:
    """Concatenate operation chunks, sorted by (path, method)."""
    merged: dict = {}
    for index, chunk in enumerate(chunks):
        if not isinstance(chunk, list):
            raise ValueError(f"chunk {index} is not a JSON array")
        for op in chunk:
            key = (op.get("path"), op.get("method"))
            if key in merged:
                raise ValueError(f"duplicate operation: {key[1]} {key[0]}")
            merged[key] = op
    return [merged[key] for key in sorted(merged, key=lambda k: (k[0] or "", k[1] or ""))]


def _collect_refs(node, out: set) -> None:
    if isinstance(node, dict):
        ref = node.get("$ref")
        if isinstance(ref, str) and ref.startswith(REF_PREFIX):
            out.add(ref[len(REF_PREFIX):])
        for value in node.values():
            _collect_refs(value, out)
    elif isinstance(node, list):
        for item in node:
            _collect_refs(item, out)


def validate_refs(schemas: dict) -> None:
    """Every internal $ref must resolve inside this snapshot.

    A dangling ref means the snapshot is incomplete -- typify would fail much
    later with a far less obvious message. All offenders are reported at once
    so a truncated retrieval is diagnosed in one pass.
    """
    found: set = set()
    _collect_refs(schemas, found)
    dangling = sorted(found - set(schemas))
    if dangling:
        raise ValueError("dangling $ref: " + ", ".join(dangling))


def _read_chunks(pattern: str) -> tuple:
    paths = sorted(STAGING_DIR.glob(pattern))
    if not paths:
        raise SystemExit(f"error: no staging files matching {pattern} in {STAGING_DIR}")
    return [json.loads(path.read_text()) for path in paths], [p.name for p in paths]


def _write_json(path: Path, payload) -> None:
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n")


def assemble() -> int:
    if not STAGING_DIR.is_dir():
        raise SystemExit(
            f"error: {STAGING_DIR} not found. Run the /refresh-spec command to populate it."
        )

    staged_meta = json.loads((STAGING_DIR / "meta.json").read_text())
    schema_chunks, schema_files = _read_chunks("schemas-*.json")
    operation_chunks, operation_files = _read_chunks("operations-*.json")

    schemas = assemble_schemas(schema_chunks)
    operations = assemble_operations(operation_chunks)
    validate_refs(schemas)

    expected = staged_meta.get("counts", {})
    for label, actual in (("schemas", len(schemas)), ("operations", len(operations))):
        want = expected.get(label)
        if want is not None and want != actual:
            raise SystemExit(
                f"error: expected {want} {label} but assembled {actual}. "
                "A chunk range was probably skipped -- re-run the retrieval."
            )

    SPEC_DIR.mkdir(parents=True, exist_ok=True)
    _write_json(SPEC_DIR / "schemas.json", schemas)
    _write_json(SPEC_DIR / "operations.json", operations)
    _write_json(
        SPEC_DIR / "_meta.json",
        {
            "source": staged_meta.get("source", "eToro API Docs MCP: /openapi/api-reference/openapi.json"),
            "retrieved": datetime.date.today().isoformat(),
            "openapi_version": staged_meta.get("openapi"),
            "etoro_api_version": (staged_meta.get("info") or {}).get("version"),
            "title": (staged_meta.get("info") or {}).get("title"),
            "servers": staged_meta.get("servers"),
            "counts": {"schemas": len(schemas), "operations": len(operations)},
            "chunks": {"schemas": schema_files, "operations": operation_files},
        },
    )

    print(f"docs/spec/schemas.json    {len(schemas)} schemas")
    print(f"docs/spec/operations.json {len(operations)} operations")
    print(f"docs/spec/_meta.json      eToro API {(staged_meta.get('info') or {}).get('version')}")
    print(f"\nStaging can now be removed: rm -rf {STAGING_DIR}")
    return 0


def check() -> int:
    missing = [n for n in ("schemas.json", "operations.json", "_meta.json") if not (SPEC_DIR / n).exists()]
    if missing:
        print(f"error: docs/spec/ is missing {', '.join(missing)}", file=sys.stderr)
        return 1

    schemas = json.loads((SPEC_DIR / "schemas.json").read_text())
    operations = json.loads((SPEC_DIR / "operations.json").read_text())
    meta = json.loads((SPEC_DIR / "_meta.json").read_text())

    try:
        validate_refs(schemas)
    except ValueError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1

    for label, actual in (("schemas", len(schemas)), ("operations", len(operations))):
        want = meta.get("counts", {}).get(label)
        if want != actual:
            print(f"error: _meta says {want} {label}, found {actual}", file=sys.stderr)
            return 1

    print(
        f"ok: eToro API {meta.get('etoro_api_version')} "
        f"({len(schemas)} schemas, {len(operations)} operations, retrieved {meta.get('retrieved')})"
    )
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--check", action="store_true", help="validate the committed snapshot and exit")
    args = parser.parse_args()
    return check() if args.check else assemble()


if __name__ == "__main__":
    sys.exit(main())
