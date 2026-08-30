# eToro Agent

A Rust foundation for an algorithmic trading agent against the
[eToro Public API](https://api-portal.etoro.com). The current implementation is
read-only: it authenticates, fetches watchlists and portfolio state, validates
the response envelopes, and exposes generated API wire types. It does not place
orders or run a trading strategy.

## Setup

1. Generate API keys at eToro Settings → Trading → API Key Management. Create a Demo + Read key first.
2. Copy your keys into `.env` (gitignored):
   ```sh
   ETORO_API_KEY=<from "Public Key" at the top of API Key Management>
   ETORO_USER_KEY=<from the row of your generated key>
   ```
3. `chmod 600 .env`
4. `cargo run`

Use Demo + Read credentials while developing. By default the program prints
only response summaries.

## Commands

```sh
cargo run                              # account summary, and price AAPL
cargo run -- prices AAPL,MSFT,TSLA     # live bid/ask from eToro
cargo run -- fetch-bars AAPL,MSFT      # daily bars from Tiingo, last five years
cargo run -- fetch-bars AAPL 2020-01-01 2024-12-31
cargo run -- gaps AAPL                 # overnight vs intraday return split
cargo run -- chart AAPL                # draw a stored series in the terminal
cargo run -- chart AAPL --html         # interactive candlesticks -> aapl.html
```

`chart` reads the local store only — no network and no credentials — and prints
the series' `PriceBasis` in its header, because two series can look identical
and mean different things. It reports the largest date gap, which is how a
vendor outage or a bad date filter becomes visible; a bar count alone looks the
same whether the fetch worked or not.

`gaps` splits each session's return into the part earned while the market was
shut (`open / previous close`) and the part earned while it was open
(`close / open`). The two compound back to the close-to-close return exactly,
because the opening prices cancel — so it is a decomposition, not an estimate.
It uses the total-return series where one exists, since on as-traded prices an
ex-dividend date reads as an overnight loss no holder suffered.

`--html` writes a self-contained interactive candlestick page — zoom, pan and
crosshair — with the charting library embedded rather than fetched from a CDN,
so it opens offline and renders the same a year from now. When the source
carries an adjusted close, both series are written into the page and a button
switches between them: on an as-traded series a split reads as a crash that
never happened, and being able to flip between the two makes that visible
rather than a footnote. See [`assets/README.md`](assets/README.md) for the
vendored library and its licence.

The page has a crosshair OHLC legend, a volume pane, SMA overlays (20/50/200),
range presets and a log-scale toggle. Its CSS and JavaScript are real files
under `assets/` rather than strings inside `format!`, so they can be linted and
read; `render_html` only fills in the data and the title.

`fetch-bars` reaches Tiingo only and needs no eToro credentials — historical
bars come from a data vendor rather than the broker, for reasons set out in
[the roadmap](docs/roadmap.md). Series are merged into the store on each run,
so re-fetching an overlapping range updates rather than duplicates.

## Configuration

| Variable | Required | Purpose |
|---|---|---|
| `ETORO_API_KEY` | for eToro commands | Public API key sent as `x-api-key` |
| `ETORO_USER_KEY` | for eToro commands | User key sent as `x-user-key` |
| `TIINGO_API_KEY` | for `fetch-bars` | Tiingo token, sent as an `Authorization` header |
| `ETORO_DUMP_RESPONSES` | no | Set to `1`, `true`, or `yes` to write full responses |
| `ETORO_AGENT_STORE` | no | Bar store root; defaults to `market-data/` |

Full responses can contain sensitive financial data. They are gitignored and,
on Unix, created with mode `0600`. Anything already at that path (an older
dump, or a symlink) is removed first and never written through.

## Library usage

The package exposes `EtoroClient` and all generated wire types as a library:

```rust,no_run
use anyhow::Result;
use etoro_agent::client::EtoroClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = EtoroClient::new("api-key", "user-key")?;
    let portfolio = client.portfolio().await?;

    if let Some(portfolio) = portfolio.client_portfolio {
        println!("open positions: {}", portfolio.positions.len());
    }

    Ok(())
}
```

Endpoint methods return [`ApiError`](src/error.rs), which callers can match on
rather than parse:

```rust,no_run
# use etoro_agent::{client::EtoroClient, error::ApiErrorKind};
# async fn example(client: &EtoroClient) {
match client.watchlists().await {
    Ok(response) => { /* ... */ }
    Err(error) if error.is_retryable() => {
        // 429, 5xx, timeout — error.retry_after() says how long the API asked for
    }
    Err(error) => match error.kind {
        ApiErrorKind::Forbidden { .. } => { /* the key likely lacks a scope */ }
        ApiErrorKind::Decode(_) => { /* the spec snapshot has drifted */ }
        _ => { /* ... */ }
    },
}
# }
```

Every request carries a UUID request ID, and every error carries it back along
with the URL. HTTP error messages include at most the first 512 characters of
the response body; API-level failures (`isSucceeded: false`) carry the
`exception` reason and message. Client construction returns a separate
`ClientError`, since nothing there is retryable.

The client has a 30-second request timeout; retry and rate-limit policies are
not implemented, though `is_retryable()` and `retry_after()` are the inputs one
would need. `EtoroClient::with_base_url` refuses plain `http` for anything but
loopback hosts, so credentials cannot be sent in cleartext by a mistyped URL.

See [Architecture and safety](docs/architecture.md) for component boundaries,
wire-format invariants, and the constraints to preserve when adding endpoints.

See [Roadmap](docs/roadmap.md) for the planned path to a live strategy, and the
API constraints that shape it.

## Generated types

Strongly-typed Rust models live in `src/types/`, generated from `docs/spec/` —
a committed snapshot of the upstream eToro OpenAPI document (currently API
**v1.355.0**: 242 component schemas, 169 operations).

Upstream keeps one flat namespace of component schemas, so codegen emits a
single `types::components` module and one thin facade per API tag re-exporting
the types that tag's operations reach:

```rust
use etoro_agent::types::tags::identity::MeResponse;
use etoro_agent::types::tags::watchlists::WatchlistsResponse;
```

That gives tag-aligned paths while keeping exactly one definition per type, so
a type reachable from several tags (`Instrument`, `Market`, `User`) is still a
single Rust type. Types generated from inline objects rather than named
schemas — `WatchlistsResponseException`, for instance — live only in
`types::components`.

| Facade | Tag | Types | Operations |
|---|---|---|---|
| `types::tags::agent_portfolios` | Agent Portfolios | 13 | 7 |
| `types::tags::app_data` | App Data | 1 | 1 |
| `types::tags::balances` | Balances | 9 | 6 |
| `types::tags::cash_accounts` | Cash Accounts | 9 | 1 |
| `types::tags::clubs` | Clubs | 13 | 1 |
| `types::tags::copy_trading` | Copy Trading | 8 | 5 |
| `types::tags::copy_trading_demo` | Copy Trading - Demo | 8 | 5 |
| `types::tags::identity` | Identity | 1 | 1 |
| `types::tags::market_data` | Market Data | 9 | 8 |
| `types::tags::notifications` | Notifications | 3 | 2 |
| `types::tags::pi_data` | PI Data | 1 | 1 |
| `types::tags::portfolio_search` | PortfolioSearch | 3 | 1 |
| `types::tags::price_alerts` | Price Alerts | 8 | 4 |
| `types::tags::rankings` | Rankings | 8 | 7 |
| `types::tags::social_feeds` | Social Feeds | 18 | 37 |
| `types::tags::sso_applications` | SSO - Applications | 10 | 5 |
| `types::tags::sso_scopes` | SSO - Scopes | 2 | 1 |
| `types::tags::sub_accounts_etoro_trading` | Sub-Accounts - eToro Trading | 9 | 6 |
| `types::tags::trading_demo` | Trading - Demo | 41 | 16 |
| `types::tags::trading_real` | Trading - Real | 41 | 16 |
| `types::tags::transfer` | Transfer | 11 | 7 |
| `types::tags::user_stats` | User Stats | 13 | 4 |
| `types::tags::users_info` | Users Info | 10 | 6 |
| `types::tags::watchlists` | Watchlists | 21 | 18 |

Hand-written wire behaviour (`Numeric`, the integer-encoded enums) lives in
`src/types/manual.rs`, attached through `x-rust-type` stamps in
`docs/overrides/`. Places where the API's behaviour differs from its spec are
recorded in [`docs/api-observations.md`](docs/api-observations.md). See
[`scripts/README.md`](scripts/README.md) for the full pipeline.

### Regenerating

Prerequisite (one-time):

```sh
cargo install cargo-typify --version 0.6.2 --locked
```

Then:

```sh
./scripts/regenerate-types.sh
```

It validates the committed snapshot, merges `docs/overrides/`, applies the
schema transforms, runs `cargo typify`, generates the tag facades, formats the
output, and runs `cargo check`. The run is idempotent — regenerating twice
produces identical bytes.

Refreshing the snapshot itself from upstream is a separate, agent-driven step
(`/refresh-spec`), because the OpenAPI document is only served through the
eToro API Docs MCP server. [`scripts/README.md`](scripts/README.md) covers
both, along with the traps worth knowing before changing any of it.

### Known gotchas in the generated types

- **Closed enums.** An unrecognised enum value anywhere in a response fails the
  whole call. That is deliberate — silently mapping unknown values would hide
  spec drift — but an eToro-side addition surfaces as a decode failure until
  the snapshot is refreshed.
- **Integer-encoded enums are mostly historical now.** At v1.355.0 upstream
  deleted the standalone integer-enum components and inlined them as plain
  `type: string` enums, so only three (`PublicAggregatedInfo*`) still use the
  hand-written `int_or_string_enum!` overrides. The rest remain in
  `src/types/manual.rs`, unreferenced, because they also accept the integer
  form — reach for them if a live response turns out to still send integers.
  See the status note in that file.
- **Orphan schemas.** Regeneration prints any schema no operation reaches.
  They are still generated, just not re-exported by a tag facade.

## Client contract tests

`cargo test --all-targets` runs the scalar wire-type tests and local HTTP
contract tests. The latter verify endpoint paths, authentication/request-ID
headers, error context, response validation, and decimal precision against
sanitized fixtures under `tests/fixtures/`. They never call the live API.
`cargo test --doc` compiles the snippet in this README (`--all-targets` does
not include doctests).

Run the complete local quality gate with:

```sh
cargo fmt --all -- --check
cargo test --all-targets
cargo test --doc
cargo clippy --all-targets -- -D warnings
```

## Project layout

```text
assets/                     # chart page CSS/JS + vendored Lightweight Charts (Apache 2.0)
market-data/                # generated: local bar store (gitignored)
docs/
  architecture.md           # runtime boundaries and safety invariants
  code-walkthrough.md       # guided tour of the implementation
  domains.json              # generated: tag -> module slug + schema closure
  roadmap.md                # plan from read-only client to a live strategy
  spec/                     # committed OpenAPI snapshot (source of truth)
    schemas.json            #   component schemas, verbatim upstream
    operations.json         #   operation index: tags, scopes, rate-limit pools
    _meta.json              #   API version, retrieval date, counts
  overrides/                # hand-authored x-rust-type stamps and field notes
scripts/
  README.md                 # the pipeline, end to end
  fetch_spec.py             # assemble + validate the snapshot
  verify_chunks.py          # prove a retrieval was byte-faithful
  build_typify_input.py     # overrides + transforms -> typify input, facades
  regenerate-types.sh       # full regeneration pipeline
  tests/                    # unit tests for the above
src/
  lib.rs                    # reusable client and public wire-type modules
  main.rs
  types/
    mod.rs                  #   (committed; declares the modules below)
    manual.rs               #   exact numerics and integer/string enum overrides
    components.rs           #   generated; every component schema
    tags.rs                 #   generated; one facade module per API tag
tests/
  client_contract.rs        # local mock HTTP contract tests
  fixtures/                 # sanitized API-shaped responses
```
