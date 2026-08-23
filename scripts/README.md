# The schema pipeline

Everything in `src/types/` is generated. This document explains how, why the
stages are split where they are, and which traps have already cost someone an
afternoon.

## The three stages

```text
[1 RETRIEVE]  eToro API Docs MCP server
                 └─ chunked jq dump (agent) ─┐
                                             ├─> scripts/fetch_spec.py
                                             │      └─> docs/spec/*.json   [committed]
[2 EXTRACT]   docs/spec/* + docs/overrides/schemas.json
                 └─ scripts/build_typify_input.py ─> docs/domains.json        [committed]
                                                     $WORK/etoro-components.json
[3 CODEGEN]   cargo typify ─> src/types/components.rs
              build_typify_input.py --emit-tags ─> src/types/tags.rs
```

Stages 2 and 3 are one command:

```sh
./scripts/regenerate-types.sh
```

Stage 1 runs only when you want newer upstream schemas.

## Stage 1 — retrieval (agent-assisted)

This is the one stage that is not a plain script, and it is worth knowing why
before trying to "fix" it.

The upstream OpenAPI document is served through the `etoro-api-docs` MCP
server. Probing `api-portal.etoro.com` for a public copy returns **403**
(Cloudflare bot protection) for a default user agent, and **404** with a
browser user agent at `/openapi/api-reference/openapi.json`,
`/api-reference/openapi.json`, `/openapi.json`, `/docs.json` and `/mint.json`.
There is no URL a shell script can fetch. **Do not add a scraper that works
around the bot protection.**

So retrieval is driven by an agent with MCP access:

```
/refresh-spec
```

See `.claude/commands/refresh-spec.md` for the procedure. It dumps the
component schemas and an operation index in chunks, then runs:

```sh
python3 scripts/fetch_spec.py          # assemble docs/spec/ from the chunks
python3 scripts/fetch_spec.py --check  # re-validate what is committed
```

`fetch_spec.py` rejects duplicate or missing chunk ranges, fails on any
dangling `$ref`, and cross-checks the assembled counts against the header.

### Trap: the MCP sandbox is character-oriented, not byte-oriented

Chunk contents pass through the agent's context on the way to disk, so they
must be *proved* byte-faithful. The obvious tool is the wrong one.

That sandbox's `wc -c` returns a **character** count, and its `sha256sum` does
not digest the UTF-8 byte stream. For any chunk containing non-ASCII — and the
spec is full of em-dashes — a perfectly correct transcription produces a
different digest there than locally. Pure-ASCII chunks agree, which is exactly
what makes the trap easy to trust by accident.

`scripts/verify_chunks.py` fingerprints over Unicode **code points** instead,
computed identically on both sides and never touching an encoding:

```
remote (jq):  .value | tostring | explode
              | reduce .[] as $c (0; (. * 131 + $c) % 1000000007)
local:        the same rolling hash over json.dumps(..., ensure_ascii=False)
```

### The snapshot

`docs/spec/` is committed, so builds and regeneration are fully offline and
upstream drift shows up as a reviewable diff:

| File | Contents |
|---|---|
| `schemas.json` | all component schemas, verbatim upstream |
| `operations.json` | per-operation index: path, method, tags, scopes, rate-limit pool, and the request/response `$ref`s that seed each tag's closure |
| `_meta.json` | source, retrieval date, API version, counts, chunk manifest |

`operations.json` also records eToro's **rate-limit pools**, including which
endpoints share a budget — order execution is 20 requests/60s shared across
about ten endpoints, market data is a separate 120/60s pool, and everything
else draws on a shared 60/60s default. That is the input a future retry policy
needs.

## Stage 2 — extraction

`build_typify_input.py` merges the override layer onto the snapshot, applies
the schema transforms, and writes the typify input plus `docs/domains.json`.

### The override layer

`docs/overrides/schemas.json` holds the `x-rust-type` stamps that point at
hand-written types in `src/types/manual.rs`. It exists because `docs/spec/` is
verbatim upstream and is overwritten by every refresh, so a local change to a
schema has nowhere else to live.

There are currently three, all nullable integer enums: without the override
typify emits a transparent newtype wrapping an `Option` wrapping the enum,
which then gets wrapped in *another* `Option` at each field site.

`Numeric` does **not** rely on this file — every `type: number` is redirected
by a bulk transform, so the decimal guarantee holds with or without any
override.

The loader accepts several files in this directory (keyed by schema name, so
grouping is organizational only and stamping the same schema twice is an
error), but one file is enough at this size. Behavioural notes about the API
live in `docs/api-observations.md`; they are documentation, not pipeline
input.

**An override naming a schema that does not exist is a hard error.** This is
the guard that matters: without it, upstream renaming a type would silently
detach a `Numeric` or manual-enum override, and the field would quietly become
an `f64` or a duplicate generated enum. It has already fired in anger — the
v1.355.0 refresh found 16 stamps whose components upstream had deleted.

### The transforms, and why the order is load-bearing

1. `normalize_enums` — first, because the OpenAPI `x-enumNames` extension must
   be renamed to typify's `x-enum-varnames` *before* step 2 strips unknown
   `x-*` keys. Also converts the spec's malformed
   `{type: integer, enum: ["Open", "Close"]}` into a string enum, unless an
   `x-rust-type` override claims it.
2. `rewrite_refs` — point `$ref`s at `#/$defs/`, drop annotations typify would
   reject.
3. `pin_x_rust_type_versions` — see the version trap below.
4. `annotate_numeric` — redirect every `type: number` to `manual::Numeric`.
5. `normalize_exclusive_bounds` — convert draft-04 `exclusiveMinimum: true` to
   the 2020-12 numeric form.
6. `normalize_nullable` — last, so it sees the final type and enum shape.

### Trap: an unsatisfied x-rust-type version fails silently

typify treats the `version` in an `x-rust-type` annotation as a semver
**requirement**. If the `--crate` flag passed to `cargo typify` does not
satisfy it, typify does not warn — it ignores the override and generates its
own type, which for a monetary field means `f64` and a silent loss of the
decimal guarantee. Both the Python script and the shell script therefore read
the version from the same place, `[package] version` in `Cargo.toml`.

### Trap: draft-04 exclusive bounds

OpenAPI 3.0 spells an exclusive bound as a boolean modifier on `minimum`;
JSON Schema 2020-12 makes `exclusiveMinimum` carry the bound itself. typify
parses 2020-12 and rejects the boolean with a bare `data did not match any
variant of untagged enum Schema` plus a line number — no mention of the
offending keyword. Transform 5 exists for this.

## Stage 3 — codegen

One `cargo typify` run produces `src/types/components.rs` from the spec's flat
component namespace, then a **second pass** writes `src/types/tags.rs`.

Two passes because typify renames schemas on the way to Rust (`meResponse`
becomes `MeResponse`, `AgentPortfolioApi_ErrorResponse` becomes
`AgentPortfolioApiErrorResponse`). The facades can only be written once there
is a real generated file to resolve every re-export against; a schema that maps
to no emitted type is an error rather than a silently missing export.

### Why one module plus facades

Upstream keeps a single flat namespace of component schemas. Generating one
Rust module per tag would manufacture an ownership problem that does not exist
there — `Instrument`, `Market`, `User` and `Position` are reachable from many
tags, and each would become a distinct, incompatible Rust type. So codegen
emits one `components` module, and `tags.rs` gives each API tag a thin facade
re-exporting the types its operations reach:

```rust
use etoro_agent::types::tags::identity::MeResponse;
use etoro_agent::types::tags::watchlists::WatchlistsResponse;
```

Tag-aligned paths, exactly one definition per type, and no owner map to
maintain. Types generated from *inline* objects (for example
`WatchlistsResponseException`) are not component schemas and so appear only in
`components`.

Schemas that no operation reaches are printed as `orphans` on every run. They
are still generated; they just get no facade. Most are path/query parameter
enums, plus a few response bodies the operation index records as an inline
array rather than a direct `$ref`.

### Formatting

The pipeline runs `cargo fmt --all` after generation. typify's own output is
not rustfmt-clean, which would otherwise fail `cargo fmt --check` on every
regeneration. rustfmt is deterministic, so this keeps the run idempotent —
regenerating twice produces identical bytes.

## Running a refresh end to end

```sh
/refresh-spec                  # agent-driven; updates docs/spec/
./scripts/regenerate-types.sh  # deterministic; updates src/types/
cargo test --all-targets
```

Expect compilation to break when upstream adds required fields or renames
types. That is the pipeline working as intended: it surfaces drift instead of
hiding it. Read the errors before "fixing" the generator.

## Adding an endpoint

1. Confirm the operation exists in `docs/spec/operations.json` (path, tags,
   response `$ref`, scopes, rate-limit pool). If it is missing, refresh the
   snapshot first.
2. Find its response type in the tag facade named by the operation's tag.
   No regeneration is needed — every component schema is already generated.
3. Add a narrowly typed client method and validate the response envelope after
   deserialization. Envelope conventions differ per endpoint: `WatchlistsResponse`
   carries `isSucceeded`/`status`, `PortfolioResponse` carries only
   `clientPortfolio`. Read the schema; do not copy an existing method's checks.
4. Add a synthetic fixture and a contract test covering path, headers, response
   shape, and failure behaviour.
5. Keep live API calls and credentials out of automated tests.

## Prerequisites and gotchas

```sh
cargo install cargo-typify --version 0.6.2 --locked
```

The version is pinned and enforced: codegen output is not stable across typify
versions, so an unpinned upgrade would produce a large spurious diff.

`src/types/components.rs` is about 1.8 MB, over jj's default 1 MiB
snapshot limit. This repo raises it (`jj config set --repo
snapshot.max-new-file-size 4MiB`); that setting lives in `.jj/repo/config.toml`
and is not shared through git, so a fresh clone needs it again.

Tests for these scripts:

```sh
python3 -m unittest discover -s scripts/tests -v
```
