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

## Not part of the pipeline: `compare_candle_sources.py`

A diagnostic, not a build step. Nothing in `src/types/` depends on it and
`regenerate-types.sh` never calls it.

It answers one question the OpenAPI document cannot: **are eToro's candles
adjusted for splits and dividends?** `candlesResponse` carries raw
open/high/low/close with no adjustment field and the docs are silent, so the
only way to know is to measure. The script pulls eToro's daily candles and the
same date range from Tiingo, then compares eToro's closes against *both* Tiingo
series -- raw `close` and `adjClose`. Whichever it tracks is the convention
eToro uses.

```sh
# .env supplies ETORO_API_KEY, ETORO_USER_KEY and TIINGO_API_KEY
python3 scripts/compare_candle_sources.py NVDA AAPL
```

This matters more than history depth. Backtesting an unadjusted 4:1 split
presents a 75% single-bar drawdown that a strategy will trade as though it were
real, and the result looks plausible rather than broken.

Choose symbols with a **split** inside the roughly four-year window eToro
exposes; a split moves the price by an integer factor and is unmistakable.
Without a split or dividend the two Tiingo series are identical, nothing can be
concluded, and the script says `inconclusive` rather than guessing.

### Trap: eToro 403s the default Python user agent

`urllib` announces itself as `Python-urllib/3.x` and eToro's edge rejects it --
the same bot protection that 403s a plain `curl`. `reqwest` sends no
`User-Agent` at all, which is why the Rust client never hit this. The script
sets its own; `SPIKE_USER_AGENT` overrides it if that stops being accepted.

Findings belong in [`../docs/api-observations.md`](../docs/api-observations.md),
not here.

## Not part of the pipeline: `compare_close_to_quote.py`

Asks whether eToro's candle close is the bid, the mid, or the last trade.

```sh
python3 scripts/compare_close_to_quote.py AAPL TSLA MBLY
```

eToro's daily closes sit about 0.16% *below* Tiingo's, consistently across
unrelated tickers. That is 25-50x the intraday spreads measured on the same
instruments and far too small to be a session misalignment, which leaves
"eToro reports the bid, and spreads widen at the close" as the leading
explanation. This checks it against eToro's own live quote, sidestepping the
cross-vendor question entirely.

**Run it while the market is closed.** With the market open, the newest candle
is a previous session and the live quote has moved since, so the comparison
measures intraday drift rather than quote convention. The script compares the
quote's timestamp against the candle's date and says so when they disagree.

Its own caveat, which the tests pin: a tight quote cannot answer the question.
AAPL's intraday spread is about 0.003%, so bid, mid and ask are all within
tolerance of each other and every one of them "matches". Prefer instruments
with a wide quote -- MBLY's is roughly 0.116%.

## Not part of the pipeline: `check_chart_page.sh`

Loads a generated chart page in headless Chrome and fails on console errors.

```sh
cargo run -- chart AAPL --html /tmp/aapl.html
./scripts/check_chart_page.sh /tmp/aapl.html
```

The Rust tests can check that the page *contains* the right strings; they
cannot check that it *runs*. Two bugs got past them, and both produced a page
that looked structurally perfect and rendered nothing:

- the chart was created 0x0. Lightweight Charts defaults to `width:0,height:0`
  and does not read its container's size unless given `autoSize: true`. The
  series toggle was firing the whole time, redrawing a canvas with no height.
- `layout.textColor` was set to `currentColor`. The library parses colours
  itself and rejects CSS keywords, throwing out of `createChart` and aborting
  the entire script -- so the heading and the warning rendered and nothing else
  did.

Hence the check is for things only the script creates: at least one `<button>`
and one `<canvas>`. Static markup survives a thrown script and proves nothing.

Not in the default gate, because it needs a browser installed; it exits 0 with
a SKIP when none is found.

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
