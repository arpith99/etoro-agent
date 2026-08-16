#!/usr/bin/env bash
#
# Regenerate Rust types from the per-domain JSON schema files in docs/.
#
# Pipeline:
#   1. scripts/typify_prep.py     — preprocess docs/*-schema.json into typify-
#                                   compatible JSON Schema docs in a private
#                                   temporary work directory.
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

# The x-rust-type overrides (Numeric, manual enums, shared-type owners) name
# this crate at a version; typify only honours them if the --crate flag
# satisfies that version. typify_prep.py pins every annotation to the version
# in Cargo.toml, so read the same value here rather than hardcoding it.
crate_version=$(sed -nE '/^\[package\]/,/^\[/{s/^version *= *"([^"]+)".*/\1/p}' Cargo.toml | head -n 1)
if [[ -z "$crate_version" ]]; then
    echo "error: could not read [package] version from Cargo.toml" >&2
    exit 1
fi

# One module per docs/<domain>-schema.json. Deriving the list from the files
# (instead of a hardcoded array) means a new domain file cannot be preprocessed
# but silently never generated; the mod.rs check below keeps it exported too.
domains=()
for schema in docs/*-schema.json; do
    d=$(basename "$schema" -schema.json)
    [[ "$d" == "all-schemas-index" ]] && continue
    domains+=("$d")
done

for d in "${domains[@]}"; do
    if ! grep -qE "^pub mod ${d};" src/types/mod.rs; then
        echo "error: docs/${d}-schema.json exists but src/types/mod.rs has no 'pub mod ${d};'" >&2
        exit 1
    fi
done

work_dir=$(mktemp -d -t etoro-typify.XXXXXX)
trap 'rm -rf "$work_dir"' EXIT

echo "==> Preprocessing schemas (crate version $crate_version)"
python3 scripts/typify_prep.py "$work_dir"

echo
echo "==> Running cargo typify per domain"
mkdir -p src/types
for d in "${domains[@]}"; do
    input="$work_dir/etoro-${d}-typify.json"
    output="src/types/${d}.rs"
    if [[ ! -f "$input" ]]; then
        echo "  ✗ $d: missing preprocessed input ($input)" >&2
        exit 1
    fi
    if cargo typify --no-builder --crate "etoro-agent@${crate_version}" "$input" --output "$output" 2> "$work_dir/typify-err-${d}.log"; then
        lines=$(wc -l < "$output")
        echo "  ✓ $d: $lines lines"
    else
        echo "  ✗ $d:" >&2
        sed 's/^/      /' "$work_dir/typify-err-${d}.log" >&2
        exit 1
    fi
done

echo
echo "==> Verifying compilation"
cargo check

echo
echo "Done. Generated files:"
ls -la src/types/
