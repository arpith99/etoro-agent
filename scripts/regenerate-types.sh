#!/usr/bin/env bash
#
# Regenerate Rust types from the committed OpenAPI snapshot in docs/spec/.
#
# Pipeline:
#   1. scripts/build_typify_input.py  — merge docs/overrides/ onto the
#                                       snapshot, apply the schema transforms,
#                                       emit a self-contained JSON Schema doc
#                                       into a private temporary work
#                                       directory, plus docs/domains.json and
#                                       the generated src/types/tags.rs.
#   2. cargo typify                   — emit Rust into src/types/components.rs.
#
# Refreshing the snapshot itself is a separate, agent-driven step; see
# .claude/commands/refresh-spec.md and scripts/README.md.
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

# The x-rust-type overrides (Numeric, manual enums) name this crate at a
# version; typify only honours them if the --crate flag satisfies that version,
# and silently generates its own type (f64 for numbers) otherwise.
# build_typify_input.py pins every annotation to the version in Cargo.toml, so
# read the same value here rather than hardcoding it.
crate_version=$(sed -nE '/^\[package\]/,/^\[/{s/^version *= *"([^"]+)".*/\1/p}' Cargo.toml | head -n 1)
if [[ -z "$crate_version" ]]; then
    echo "error: could not read [package] version from Cargo.toml" >&2
    exit 1
fi

for required in docs/spec/schemas.json docs/spec/operations.json; do
    if [[ ! -f "$required" ]]; then
        echo "error: $required not found. Run the /refresh-spec command first." >&2
        exit 1
    fi
done

echo "==> Validating the committed snapshot"
python3 scripts/fetch_spec.py --check

work_dir=$(mktemp -d -t etoro-typify.XXXXXX)
trap 'rm -rf "$work_dir"' EXIT

echo
echo "==> Building typify input (crate version $crate_version)"
python3 scripts/build_typify_input.py "$work_dir"

echo
echo "==> Running cargo typify"
input="$work_dir/etoro-components.json"
output="src/types/components.rs"
if cargo typify --no-builder --crate "etoro-agent@${crate_version}" "$input" --output "$output" 2> "$work_dir/typify-err.log"; then
    echo "  ✓ $output: $(wc -l < "$output") lines"
else
    sed 's/^/      /' "$work_dir/typify-err.log" >&2
    exit 1
fi

# Second pass. typify renames schemas on the way to Rust (meResponse ->
# MeResponse), so the facades can only be written now that there is a real
# generated file to resolve every re-export against. A schema that maps to no
# emitted type fails here rather than becoming a silently missing re-export.
echo
echo "==> Generating tag facades"
python3 scripts/build_typify_input.py --emit-tags

# typify's output is not rustfmt-clean (it wraps some long signatures
# differently), which would fail the `cargo fmt --check` gate on every
# regeneration. rustfmt is deterministic, so formatting here keeps the
# generated files stable and the pipeline idempotent.
echo
echo "==> Formatting generated code"
cargo fmt --all

echo
echo "==> Verifying compilation"
cargo check

echo
echo "Done. Generated files:"
ls -la src/types/
