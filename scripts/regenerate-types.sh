#!/usr/bin/env bash
#
# Regenerate Rust types from the per-domain JSON schema files in docs/.
#
# Pipeline:
#   1. scripts/typify_prep.py     — preprocess docs/*-schema.json into typify-
#                                   compatible JSON Schema docs in /tmp/.
#   2. cargo typify (per domain)  — emit Rust into src/types/<domain>.rs.
#
# Run from the etoro-agent/ project root.
# Requires: python3, cargo-typify (install: `cargo install cargo-typify`).

set -euo pipefail

cd "$(dirname "$0")/.."

if ! command -v cargo-typify > /dev/null; then
    echo "error: cargo-typify not found. Install with: cargo install cargo-typify" >&2
    exit 1
fi

DOMAINS=(agent_portfolios feeds_posts identity market_data portfolio trading watchlists)

echo "==> Preprocessing schemas"
python3 scripts/typify_prep.py

echo
echo "==> Running cargo typify per domain"
mkdir -p src/types
for d in "${DOMAINS[@]}"; do
    input="/tmp/etoro-${d}-typify.json"
    output="src/types/${d}.rs"
    if [[ ! -f "$input" ]]; then
        echo "  ✗ $d: missing preprocessed input ($input)" >&2
        exit 1
    fi
    if cargo typify --no-builder --crate etoro-agent@0.1.0 "$input" --output "$output" 2> /tmp/typify-err-${d}.log; then
        lines=$(wc -l < "$output")
        echo "  ✓ $d: $lines lines"
    else
        echo "  ✗ $d:" >&2
        sed 's/^/      /' /tmp/typify-err-${d}.log >&2
        exit 1
    fi
done

echo
echo "==> Verifying compilation"
cargo check

echo
echo "Done. Generated files:"
ls -la src/types/
