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
which selects `https://public-api.etoro.com`. Because every request carries
credentials, `with_base_url` accepts only `https` origins, or plain `http` on
loopback hosts (`localhost`, `127.0.0.0/8`, `::1`) for local mocks.

## Client guarantees

The client currently exposes:

| Method | HTTP endpoint | Validation |
|---|---|---|
| `watchlists()` | `GET /api/v1/watchlists` | `isSucceeded` must be true and the embedded status must be 2xx; on failure the `exception` reason/message/invalidItems are included in the error |
| `portfolio()` | `GET /api/v1/trading/info/portfolio` | `clientPortfolio` must be present |

Both requests send `x-api-key`, `x-user-key`, and a fresh `x-request-id`.
Authentication header values are marked sensitive in `reqwest`, and requests
have a 30-second timeout. Non-successful HTTP responses return the status,
request ID, and a response-body excerpt capped at 512 characters. A 2xx body
that does not match the generated types is reported as a decode failure with
serde's reason (for example an unknown enum variant), not as "invalid JSON".

Enum types are closed: an unrecognised variant anywhere in a response fails
the whole call. That is deliberate for now — silently mapping unknown values
would hide spec drift — but it means an eToro-side addition (a new watchlist
type, asset class, ...) surfaces as a decode failure until the schema and
`manual.rs` are updated.

Retries are deliberately absent. A future retry policy must account for eToro
rate limits and must never retry a non-idempotent trading request without an
explicit idempotency design.

## Numeric invariant

API fields declared as JSON `number` are redirected by the schema preprocessor
to `types::manual::Numeric`, which wraps `rust_decimal::Decimal`.

`serde_json` and `rust_decimal` both use their arbitrary-precision JSON
features. Consequently, a value such as
`0.1234567890123456789012345678` is decoded and encoded without first becoming
an `f64`, and scientific notation (`5.06e-6`) is accepted. Do not replace this
adapter with `rust_decimal::serde::float`: doing so would reintroduce precision
loss at the wire boundary.

The guarantee is bounded by `Decimal`: 28 significant digits (a 29th is
rounded half-up, see `numeric_rounds_beyond_28_significant_digits`) and a
magnitude below 2^96 ≈ 7.9e28 (larger values are a deserialization error and
fail the whole response). Both bounds are far beyond trading amounts, but keep
them in mind before mapping analytics-style fields such as market caps.

Decimal representation does not replace domain validation. Before order
execution is added, prices, units, leverage, and monetary amounts still need
instrument-specific scale, range, and sign checks.

## Generated types

The JSON schema fragments under `docs/` are the source of truth for generated
Rust modules. `scripts/typify_prep.py`:

1. resolves transitive references across domains;
2. renames the OpenAPI `x-enumNames` extension to typify's `x-enum-varnames`
   (before stripping unknown `x-*` keys) so integer-enum names survive;
3. converts OpenAPI nullability into JSON Schema null unions, leaving untyped
   `nullable` schemas untyped (→ `serde_json::Value`) rather than null-only;
4. redirects shared types to one owner module;
5. pins every `x-rust-type` annotation to the `[package]` version in
   `Cargo.toml` (typify silently ignores an override whose version requirement
   the `--crate` flag does not satisfy);
6. redirects every `type: number` field to `manual::Numeric`; integer-encoded
   string enums are redirected only where a `x-rust-type` annotation has been
   stamped by hand on the standalone schema in `docs/*-schema.json` — the
   remainder become string-only enums; and
7. emits self-contained JSON Schema documents into a private temporary
   directory supplied by the shell script (never a fixed path under `/tmp`).

`scripts/regenerate-types.sh` requires cargo-typify 0.6.2, derives the domain
list from `docs/*-schema.json`, refuses to run if `src/types/mod.rs` does not
export one of them, passes the Cargo.toml version to `cargo typify --crate`,
regenerates each domain, and runs `cargo check`. Generated domain modules
should not be edited by hand. Manual wire behavior belongs in
`src/types/manual.rs` and must be wired in through `x-rust-type` schema
annotations.

## Sensitive data

Credentials belong only in the gitignored `.env` file or the process
environment. Use mode `0600` for `.env`.

Full watchlist and portfolio responses are not written unless
`ETORO_DUMP_RESPONSES` is set to `1`, `true`, or `yes`. On Unix, dump files are
created with mode `0600` via `O_CREAT | O_EXCL`; whatever previously occupied
the path (including a symlink) is unlinked first, so a dump can never be
written through a link into another file. They remain local debugging artifacts and
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
