# eToro Agent

An algorithmic trading agent in Rust against the [eToro Public API](https://api-portal.etoro.com).

## Setup

1. Generate API keys at eToro Settings → Trading → API Key Management. Create a Demo + Read key first.
2. Copy your keys into `.env` (gitignored):
   ```
   ETORO_API_KEY=<from "Public Key" at the top of API Key Management>
   ETORO_USER_KEY=<from the row of your generated key>
   ```
3. `chmod 600 .env`
4. `cargo run`

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
| `watchlists` | `docs/watchlists-schema.json` | 8 |

Cross-domain types (e.g. `Market`) are duplicated across modules so each module
is self-contained — no cross-module imports needed.

### Regenerating after schema changes

Prerequisite (one-time): `cargo install cargo-typify`

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
  are emitted as **string** enums because the source spec mixes `type: integer`
  with string-valued enum entries. The actual wire format is the integer index;
  add `#[serde_repr]` (via the [`serde_repr`](https://crates.io/crates/serde_repr)
  crate) or a custom (de)serializer when you need to (de)serialize them
  correctly.
- **`feeds_posts-schema.json` has two manual edits** (`Post.type` and
  `Post.metadata`): the inline shapes were replaced with `$ref`s to the
  standalone `PostType` / `PostMetadata` schemas to avoid duplicate type
  generation. See the `_replaced_inline` annotations in the source. The result:
  `Post.metadata` exposes the (always-`None`-for-non-Article-posts) `article`
  field, and `Post.type` exposes all 8 variants instead of just `Default`.
- **All numeric price/amount fields are `f64`** in the generated code (the spec
  uses `type: number, format: float|double`). For real trading code, convert to
  [`rust_decimal::Decimal`](https://crates.io/crates/rust_decimal) before any
  arithmetic.

## Project layout

```
docs/                       # JSON schema files (source of truth for types/)
  *-schema.json             #   per-domain extracts from the eToro OpenAPI spec
  all-schemas-index.json    #   compact index of all 128 schemas
scripts/
  typify_prep.py            # preprocesses schemas → typify-compatible inputs
  regenerate-types.sh       # full regeneration pipeline
src/
  main.rs
  types/                    # generated; do not edit by hand
    mod.rs                  #   (committed; declares the submodules below)
    agent_portfolios.rs
    feeds_posts.rs
    identity.rs
    market_data.rs
    portfolio.rs
    trading.rs
    watchlists.rs
```
