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
# Requires: python3, cargo-typify 0.6.2 (see TYPIFY_VERSION below).

set -euo pipefail

cd "$(dirname "$0")/.."

TYPIFY_VERSION="0.6.2"

if ! command -v cargo-typify > /dev/null; then
    echo "error: cargo-typify not found. Install with: cargo install cargo-typify --version $TYPIFY_VERSION --locked" >&2
    exit 1
fi

installed_typify_version=$(cargo typify --version | awk '{print $2}')
if [[ "$installed_typify_version" != "$TYPIFY_VERSION" ]]; then
    echo "error: cargo-typify $TYPIFY_VERSION is required (found $installed_typify_version)." >&2
    echo "Install with: cargo install cargo-typify --version $TYPIFY_VERSION --locked --force" >&2
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
