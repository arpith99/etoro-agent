# eToro Agent

A Rust foundation for an algorithmic trading agent against the
[eToro Public API](https://api-portal.etoro.com). The current implementation is
read-only: it authenticates, fetches watchlists and portfolio state, validates
the response envelopes, and exposes generated API wire types. It does not place
orders or run a trading strategy.

## Setup

1. Generate API keys at eToro Settings → Trading → API Key Management. Create a Demo + Read key first.
2. Copy your keys into `.env` (gitignored):
   ```
   ETORO_API_KEY=<from "Public Key" at the top of API Key Management>
   ETORO_USER_KEY=<from the row of your generated key>
   ```
3. `chmod 600 .env`
4. `cargo run`

Use Demo + Read credentials while developing. By default the program prints
only response summaries.

## Configuration

| Variable | Required | Purpose |
|---|---|---|
| `ETORO_API_KEY` | yes | Public API key sent as `x-api-key` |
| `ETORO_USER_KEY` | yes | User key sent as `x-user-key` |
| `ETORO_DUMP_RESPONSES` | no | Set to `1`, `true`, or `yes` to write full responses |

Full responses can contain sensitive financial data. They are gitignored and,
on Unix, created with mode `0600`. Existing files are also reset to `0600`
before being overwritten.

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

Every request gets a UUID request ID. Transport, HTTP, JSON, and response
validation errors include that ID where available. HTTP error messages include
at most the first 512 characters of the response body. The client has a
30-second request timeout; retry and rate-limit policies are not implemented.

See [Architecture and safety](docs/architecture.md) for component boundaries,
wire-format invariants, and the constraints to preserve when adding endpoints.

## Generated types

Strongly-typed Rust models for the eToro API live in `src/types/`, generated from
the per-domain JSON schema files in `docs/`:

| Module | Source | Schemas |
|---|---|---|
| `agent_portfolios` | `docs/agent_portfolios-schema.json` | 10 |
| `feeds_posts` | `docs/feeds_posts-schema.json` | 53 (+12 inlined) |
| `identity` | `docs/identity-schema.json` | 10 |
| `market_data` | `docs/market_data-schema.json` | 17 |
| `portfolio` | `docs/portfolio-schema.json` | 7 (+6 inlined) |
| `trading` | `docs/trading-schema.json` | 25 (+5 inlined) |
| `watchlists` | `docs/watchlists-schema.json` | 3 |

Selected cross-domain types have a single owner module and are reused elsewhere
(for example, `Market` is owned by `market_data`). This avoids distinct Rust
types representing the same API object.

### Regenerating after schema changes

Prerequisite (one-time):

```sh
cargo install cargo-typify --version 0.6.2 --locked
```

Then run:

```sh
scripts/regenerate-types.sh
```

This:
1. Preprocesses `docs/*-schema.json` into typify-compatible JSON Schema 2020-12
   documents (`scripts/typify_prep.py`): inlines transitive `$ref` dependencies
   across sibling files, rewrites OpenAPI ref paths to `$defs`, normalizes
   `nullable: true` → JSON Schema null type union, normalizes integer-encoded
   string enums.
2. Runs `cargo typify` per domain → `src/types/<domain>.rs`.
3. Runs `cargo check` to verify the generated code compiles.

### Known gotchas in the generated types

- **Integer-encoded enums** (`MarketAssetType`, `PostType`, `UserRole`, etc.)
  use hand-written overrides in `types/manual.rs`. They accept either the
  integer wire value or the string name and serialize as the integer value.
- **`feeds_posts-schema.json` has two manual edits** (`Post.type` and
  `Post.metadata`): the inline shapes were replaced with `$ref`s to the
  standalone `PostType` / `PostMetadata` schemas to avoid duplicate type
  generation. See the `_replaced_inline` annotations in the source. The result:
  `Post.metadata` exposes the (always-`None`-for-non-Article-posts) `article`
  field, and `Post.type` exposes all 8 variants instead of just `Default`.
- **Numeric price/amount fields use `Numeric`**, a
  [`rust_decimal::Decimal`](https://crates.io/crates/rust_decimal) newtype. Its
  serde adapter and `serde_json` are configured for arbitrary-precision JSON
  numbers, so values do not pass through `f64`.

## Client contract tests

`cargo test --all-targets` runs the scalar wire-type tests and local HTTP
contract tests. The latter verify endpoint paths, authentication/request-ID
headers, error context, response validation, and decimal precision against
sanitized fixtures under `tests/fixtures/`. They never call the live API.

Run the complete local quality gate with:

```sh
cargo fmt --all -- --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

## Project layout

```
docs/                       # JSON schema files (source of truth for types/)
  architecture.md           # runtime boundaries and safety invariants
  *-schema.json             #   per-domain extracts from the eToro OpenAPI spec
  all-schemas-index.json    #   compact index of all 125 unique schemas
scripts/
  typify_prep.py            # preprocesses schemas → typify-compatible inputs
  regenerate-types.sh       # full regeneration pipeline
src/
  lib.rs                    # reusable client and public wire-type modules
  main.rs
  types/                    # generated; do not edit by hand
    mod.rs                  #   (committed; declares the submodules below)
    manual.rs               #   exact numerics and integer/string enum overrides
    agent_portfolios.rs
    feeds_posts.rs
    identity.rs
    market_data.rs
    portfolio.rs
    trading.rs
    watchlists.rs
tests/
  client_contract.rs        # local mock HTTP contract tests
  fixtures/                 # sanitized API-shaped responses
```
