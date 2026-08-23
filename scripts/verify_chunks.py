#!/usr/bin/env python3
"""
Verify staged spec chunks against fingerprints taken from the MCP docs server.

Why this exists instead of `sha256sum`: the MCP documentation sandbox's text
tools are character-oriented, not byte-oriented. Its `wc -c` returns a
character count, and its `sha256sum` does not digest the UTF-8 byte stream --
for content containing non-ASCII characters (the spec is full of em-dashes) a
correct transcription still produces a different digest there. Pure-ASCII
chunks happen to agree, which makes the trap easy to miss.

So the fingerprint is computed over Unicode *code points*, identically on both
sides, and never touches an encoding:

    remote (jq):  .value | tostring | explode
                  | reduce .[] as $c (0; (. * 131 + $c) % 1000000007)

    local (here): same rolling hash over json.dumps(value, separators=(",", ":"),
                  ensure_ascii=False)

Both also report a code-point length, so a mismatch says whether content moved
or only changed.

Usage:
    python3 scripts/verify_chunks.py FINGERPRINTS.txt [--staging DIR]

FINGERPRINTS.txt holds one `<schemaName> <length> <hash>` per line, as emitted
by the jq command documented in .claude/commands/refresh-spec.md.
"""

import argparse
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
MODULUS = 1000000007
MULTIPLIER = 131


def codepoint_hash(text: str) -> int:
    value = 0
    for ch in text:
        value = (value * MULTIPLIER + ord(ch)) % MODULUS
    return value


def fingerprint(schema) -> tuple:
    """(code-point length, rolling hash) of a schema's canonical JSON."""
    text = json.dumps(schema, separators=(",", ":"), ensure_ascii=False)
    return len(text), codepoint_hash(text)


def load_staged(staging: Path) -> dict:
    merged: dict = {}
    for path in sorted(staging.glob("schemas-*.json")):
        for name, schema in json.loads(path.read_text()).items():
            if name in merged:
                raise ValueError(f"duplicate schema across chunks: {name}")
            merged[name] = schema
    return merged


def parse_fingerprints(path: Path) -> dict:
    expected = {}
    for line in path.read_text().splitlines():
        parts = line.split()
        if len(parts) != 3:
            continue
        name, length, digest = parts
        expected[name] = (int(length), int(digest))
    return expected


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("fingerprints", type=Path)
    parser.add_argument("--staging", type=Path, default=REPO_ROOT / "docs" / "spec" / ".staging")
    args = parser.parse_args()

    expected = parse_fingerprints(args.fingerprints)
    actual = load_staged(args.staging)

    missing = sorted(set(expected) - set(actual))
    extra = sorted(set(actual) - set(expected))
    mismatched = [
        (name, expected[name], fingerprint(actual[name]))
        for name in sorted(set(expected) & set(actual))
        if fingerprint(actual[name]) != expected[name]
    ]

    for name in missing:
        print(f"MISSING  {name}", file=sys.stderr)
    for name in extra:
        print(f"EXTRA    {name}", file=sys.stderr)
    for name, want, got in mismatched:
        print(f"MISMATCH {name}: expected len={want[0]} hash={want[1]}, got len={got[0]} hash={got[1]}", file=sys.stderr)

    if missing or extra or mismatched:
        print(
            f"\n{len(missing)} missing, {len(extra)} extra, {len(mismatched)} mismatched "
            f"of {len(expected)} expected schemas",
            file=sys.stderr,
        )
        return 1

    print(f"ok: all {len(expected)} schemas match the upstream fingerprints")
    return 0


if __name__ == "__main__":
    sys.exit(main())
