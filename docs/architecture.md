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

`src/lib.rs` exports the reusable client, its typed errors, and the wire types.
`src/main.rs` is a small executable that loads credentials, calls the three
implemented read-only endpoints, and prints aggregate counts. It keeps `anyhow`
for reporting, which works unchanged because the library's errors implement
`std::error::Error`.

`EtoroClient::with_base_url` exists for API-compatible sandboxes and local
contract tests. Production callers should normally use `EtoroClient::new`,
which selects `https://public-api.etoro.com`. Because every request carries
credentials, `with_base_url` accepts only `https` origins, or plain `http` on
loopback hosts (`localhost`, `127.0.0.0/8`, `::1`) for local mocks.

## Client guarantees

The client currently exposes:

| Method | HTTP endpoint | Validation |
|---|---|---|
| `watchlists()` | `GET /api/v1/watchlists` | `isSucceeded` must be true and the embedded status must be 2xx; on failure the `exception` reason/message/invalidItems are carried in the error |
| `portfolio()` | `GET /api/v1/trading/info/portfolio` | `clientPortfolio` must be present |
| `me()` | `GET /api/v1/me` | none by hand — upstream marks nine fields required, so serde enforces presence before the method returns |

Every request sends `x-api-key`, `x-user-key`, and a fresh `x-request-id`.
Authentication header values are marked sensitive in `reqwest`, and requests
have a 30-second timeout.

Failures are typed, in `src/error.rs`. `ClientError` covers construction
(nothing there is retryable); `ApiError` covers requests and carries the
request ID and URL alongside an `ApiErrorKind`. The variants are shaped by the
decisions a caller makes rather than by the places the client can fail:
`RateLimited` is separate from `Http` because it alone carries `Retry-After`,
and `Forbidden` is separate because eToro returns 403 — not 401 — when a key
lacks scope. Nuance beyond the variants lives in methods, so the branch exists
once: `ApiError::is_retryable()` and `ApiError::retry_after()`.

Non-successful HTTP responses carry the status, the request ID, and a
response-body excerpt capped at 512 characters. A 2xx body that does not match
the generated types is reported as a decode failure carrying serde's own reason
(for example an unknown enum variant or a missing required field), not as
"invalid JSON". A 2xx body that decodes but violates an envelope invariant is
`Malformed`, which is distinct from the API deliberately reporting failure
(`ApiFailure`): the first means the snapshot has drifted, the second means the
API said no.

Enum types are closed: an unrecognised variant anywhere in a response fails
the whole call. That is deliberate for now — silently mapping unknown values
would hide spec drift — but it means an eToro-side addition (a new watchlist
type, asset class, ...) surfaces as a decode failure until the schema and
`manual.rs` are updated.

Retries are deliberately absent, but the inputs a policy needs now exist:
`is_retryable()` / `retry_after()` on the error, and per-endpoint rate-limit
pools in `docs/spec/operations.json` (order execution is 20 requests/60s shared
across about ten endpoints; market data is a separate 120/60s pool; everything
else draws on a shared 60/60s default). `get_json` remains the single chokepoint
where such a policy would live. It must never retry a non-idempotent trading
request without an explicit idempotency design.

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

Everything in `src/types/` is generated from `docs/spec/`, a committed snapshot
of the upstream eToro OpenAPI document. `scripts/README.md` documents the
pipeline in full; the parts that constrain the rest of the system are:

- **The snapshot is the source of truth.** `docs/spec/schemas.json` holds the
  component schemas verbatim; `docs/spec/operations.json` indexes every
  operation with its tags, scopes, rate-limit pool, and request/response
  `$ref`s. Refreshing it is agent-driven (`/refresh-spec`) because the document
  is only served through the eToro API Docs MCP server. Everything downstream
  is deterministic and offline.
- **Local schema changes live in `docs/overrides/`**, keyed by schema name:
  `x-rust-type` stamps pointing at `src/types/manual.rs`. An override naming a
  schema that no longer exists is a hard error, so upstream renaming a type
  cannot silently detach one. Behaviour observed against the live API — the
  places where the spec is wrong — is recorded separately in
  [`api-observations.md`](api-observations.md).
- **One namespace, tag-aligned views.** Upstream keeps a single flat component
  namespace, so codegen emits one `types::components` module and
  `types::tags::<tag>` facades that re-export the types each tag's operations
  reach. There is exactly one `Instrument` type, not one per tag that mentions
  it. Types generated from inline objects appear only in `components`.
- **Generated files are not edited by hand.** Manual wire behaviour belongs in
  `src/types/manual.rs`, wired in through an `x-rust-type` override.

`scripts/regenerate-types.sh` runs the whole thing and is idempotent:
regenerating twice produces identical bytes.

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

1. confirm the operation is in `docs/spec/operations.json` and find its
   response type in the matching `types::tags::<tag>` facade — every component
   schema is already generated, so no regeneration is normally needed. Only
   refresh the snapshot if the operation is missing;
2. add a narrowly typed client method;
3. validate required success-envelope fields after deserialization — envelope
   conventions differ per endpoint, so read the schema rather than copying an
   existing method's checks;
4. add a synthetic fixture and a local contract test for the path, headers,
   response shape, and failure behavior; and
5. keep live API calls and credentials out of automated tests.

Write endpoints require additional design before implementation: demo-account
enforcement, idempotency, position and loss limits, an explicit approval mode,
audit logging, and rate-limit handling.
