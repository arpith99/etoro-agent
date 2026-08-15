# Architecture and safety

This project is the read-only API foundation for an eToro trading agent. Its
current responsibility ends at fetching and validating account state. Strategy
evaluation, persistence, scheduling, and order execution are not implemented.

## Runtime flow

```text
.env / process environment
          |
          v
       main.rs  -- optional private JSON dumps
          |
          v
     EtoroClient -- authentication, request ID, timeout, HTTP errors
          |
          v
  public-api.etoro.com
          |
          v
 generated response types -- serde decoding and envelope validation
```

`src/lib.rs` exports the reusable client and wire types. `src/main.rs` is a
small executable that loads credentials, calls the two implemented read-only
endpoints, and prints aggregate counts.

`EtoroClient::with_base_url` exists for API-compatible sandboxes and local
contract tests. Production callers should normally use `EtoroClient::new`,
which selects `https://public-api.etoro.com`.

## Client guarantees

The client currently exposes:

| Method | HTTP endpoint | Validation |
|---|---|---|
| `watchlists()` | `GET /api/v1/watchlists` | `isSucceeded` must be true and the embedded status must be 2xx |
| `portfolio()` | `GET /api/v1/trading/info/portfolio` | `clientPortfolio` must be present |

Both requests send `x-api-key`, `x-user-key`, and a fresh `x-request-id`.
Authentication header values are marked sensitive in `reqwest`, and requests
have a 30-second timeout. Non-successful HTTP responses return the status,
request ID, and a response-body excerpt capped at 512 characters.

Retries are deliberately absent. A future retry policy must account for eToro
rate limits and must never retry a non-idempotent trading request without an
explicit idempotency design.

## Numeric invariant

API fields declared as JSON `number` are redirected by the schema preprocessor
to `types::manual::Numeric`, which wraps `rust_decimal::Decimal`.

`serde_json` and `rust_decimal` both use their arbitrary-precision JSON
features. Consequently, a value such as
`0.1234567890123456789012345678` is decoded and encoded without first becoming
an `f64`. Do not replace this adapter with `rust_decimal::serde::float`: doing
so would reintroduce precision loss at the wire boundary.

Decimal representation does not replace domain validation. Before order
execution is added, prices, units, leverage, and monetary amounts still need
instrument-specific scale, range, and sign checks.

## Generated types

The JSON schema fragments under `docs/` are the source of truth for generated
Rust modules. `scripts/typify_prep.py`:

1. resolves transitive references across domains;
2. converts OpenAPI nullability into JSON Schema null unions;
3. redirects shared types to one owner module;
4. redirects numeric fields and malformed integer enums to manual types; and
5. emits self-contained temporary JSON Schema documents.

`scripts/regenerate-types.sh` requires cargo-typify 0.6.2, regenerates each
domain, and runs `cargo check`. Generated domain modules should not be edited by
hand. Manual wire behavior belongs in `src/types/manual.rs` and must be wired in
through `x-rust-type` schema annotations.

## Sensitive data

Credentials belong only in the gitignored `.env` file or the process
environment. Use mode `0600` for `.env`.

Full watchlist and portfolio responses are not written unless
`ETORO_DUMP_RESPONSES` is set to `1`, `true`, or `yes`. On Unix, dump files are
created or corrected to mode `0600`. They remain local debugging artifacts and
must not be converted into test fixtures. Contract fixtures under
`tests/fixtures/` are synthetic and contain no account data.

## Adding an endpoint

When extending the client:

1. add or update the source schema and regenerate the types;
2. add a narrowly typed client method;
3. validate required success-envelope fields after deserialization;
4. add a synthetic fixture and a local contract test for the path, headers,
   response shape, and failure behavior; and
5. keep live API calls and credentials out of automated tests.

Write endpoints require additional design before implementation: demo-account
enforcement, idempotency, position and loss limits, an explicit approval mode,
audit logging, and rate-limit handling.
