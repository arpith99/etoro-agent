# Code walkthrough

A guided tour of the existing code: what each part does, why it is written the
way it is, and which Rust idioms are worth carrying forward. Where
[`architecture.md`](architecture.md) states the invariants, this document
explains the implementation that upholds them.

Describes the tree as of the typed-errors change (2026-08-23), on API
v1.355.0. The suite is 26 Rust tests green (10 library unit, 3 in `main.rs`,
11 contract, 2 doctests) plus 40 Python tests for the pipeline scripts.

## 1. The crate's shape

`Cargo.toml` declares one package that builds **two things**: a library
(`src/lib.rs`) and a binary (`src/main.rs`). That split is the most important
structural decision in the project.

```rust
// src/lib.rs
extern crate self as etoro_agent;
pub mod client;
pub mod error;
pub mod types;
```

The library is the real product; `main.rs` is a *consumer* of it, exactly like
a future test, strategy engine, or second binary. This is why
`tests/client_contract.rs` can write `use etoro_agent::client::EtoroClient;` —
integration tests in `tests/` link against the library as an external crate,
just as a stranger's code would.

Two subtleties in those few lines:

**`extern crate self as etoro_agent;`** — cargo-typify emits *absolute* paths
in the generated code (`::etoro_agent::types::manual::Numeric`). Inside the
crate that defines it, `::etoro_agent` does not normally resolve; a crate
cannot refer to itself by name by default. This line creates that alias.

**`#![doc = include_str!("../README.md")]`** — the README becomes the crate's
front-page documentation. The payoff is that the ` ```rust,no_run ` block at
`README.md:39` is compiled by `cargo test --doc`, so the README example cannot
silently rot. `no_run` means *compile it, do not execute it* — correct here,
since running it would make live API calls.

### Dependencies worth understanding

| Crate | Why it is there |
|---|---|
| `thiserror` | Typed errors for the library: `ClientError` for construction, `ApiError` for requests. Callers match on variants instead of parsing strings. |
| `anyhow` | Kept for the *binary* only, where a human reads the message. `ApiError` implements `std::error::Error`, so `?` converts it in `main.rs` with no glue. |
| `serde_json` with `arbitrary_precision` | Makes serde_json keep a JSON number's exact text instead of parsing it into `f64`. This is a *global* feature: it changes how every number in the crate is handled. |
| `rust_decimal` with `serde-arbitrary-precision` | The other half of that bridge. |
| `reqwest` with `json` | The `json` feature is currently unused — nothing in `src/` calls `.json()`, because the client deserializes manually (see below). |
| `uuid` v4 | Request IDs. |
| `dotenvy` | Loads `.env` in `main.rs` only. The library never reads the environment; credentials are always *passed in*. |

## 2. `src/client.rs` — the heart

### Construction

`new()` delegates to `with_base_url()` with the production URL, so all the real
logic lives in one place.

```rust
let sensitive = |header: &'static str, value: &str| -> Result<_, ClientError> {
    let mut value = header::HeaderValue::from_str(value)
        .map_err(|source| ClientError::InvalidCredential { header, source })?;
    value.set_sensitive(true);
    Ok(value)
};
```

A **closure returning `Result`**, so `?` works inside it. It takes the header
name as well as the value purely so the error can say *which* credential was
malformed — the earlier string-based version could not tell them apart.
`set_sensitive` does
two real things: it stops the value being HPACK-indexed on HTTP/2 (so the key
is not cached in a shared compression table), and it makes `HeaderValue`'s
`Debug` print `Sensitive` instead of the key — so an accidental
`dbg!(headers)` cannot leak credentials into logs.

These go into `default_headers`, so **every** request the client makes carries
them. Authentication cannot be forgotten when adding an endpoint, which is why
a new endpoint method is so short.

```rust
let base_url: Url = format!("{}/", base_url.trim_end_matches('/')).parse()?;
```

Normalizing to exactly one trailing slash, because of how `Url::join` behaves:

- base `…/prefix/` + `"api/v1/me"` → `…/prefix/api/v1/me` — correct
- base `…/prefix` (no slash) + `"api/v1/me"` → `…/api/v1/me` — **`prefix` is eaten**
- base `…/prefix/` + `"/api/v1/me"` (leading slash) → `…/api/v1/me` — **also eaten**

The `format!` fixes the second case permanently. The third is a per-call-site
responsibility: **API paths must not start with a slash.** The
`http://localhost:8080/prefix` case in the loopback test exercises exactly this.

```rust
if !(base_url.scheme() == "https" || (base_url.scheme() == "http" && is_loopback(&base_url))) {
    return Err(ClientError::InsecureBaseUrl { url: base_url.to_string() });
}
```

`InsecureBaseUrl` is deliberately a different variant from `InvalidBaseUrl`:
the URL parsed perfectly well and we are *refusing* it. The rule is
*credentials-shaped*, not URL-shaped: because auth headers are baked into the
client, any origin it is pointed at receives them, so plaintext is only
tolerable where the bytes cannot leave the machine. `is_loopback` handles
`localhost`, anything parsing as a loopback `IpAddr` (all of `127.0.0.0/8`, not
just `127.0.0.1`), and strips `[...]` for IPv6 literals such as `[::1]`.

### `get_json` — the one place HTTP happens

```rust
async fn get_json<T: DeserializeOwned>(&self, path: &str)
    -> Result<(T, RequestContext), ApiError>
```

**Generic over the response type**, private, and returns the decoded value
**plus a `RequestContext`** carrying the request ID and URL. Three things to
notice:

**`DeserializeOwned` rather than `Deserialize<'de>`.** The bound means "can be
deserialized without borrowing from the input buffer" — necessary because
`body` is a local that dies at the end of the function, so a type borrowing
`&str` slices out of it could not outlive it.

**The request ID is returned, not swallowed.** Every error carries it, so a
support question of the form "which request?" has an answer. It travels inside
`RequestContext` so that an endpoint method raising an envelope error can
attach the same ID and URL `get_json` would have used — `ctx.malformed("...")`
rather than re-deriving them.

```rust
let status = response.status();
let retry_after = response.headers().get(header::RETRY_AFTER)…;   // before bytes()!
let body = response.bytes().await.map_err(…Transport…)?;

if !status.is_success() {
    return Err(ctx.error(match status {
        StatusCode::TOO_MANY_REQUESTS => ApiErrorKind::RateLimited { retry_after },
        StatusCode::FORBIDDEN         => ApiErrorKind::Forbidden { body },
        status                        => ApiErrorKind::Http { status, body },
    }));
}
let value = serde_json::from_slice(&body).map_err(…Decode…)?;
```

Note the header read placed *before* `bytes()`: that call consumes the
response, so reading `Retry-After` afterwards is impossible and
`RateLimited { retry_after }` could only ever be `None`.

**This is why `.json()` is not used.** `reqwest`'s `.json()` consumes the
response and, on failure, says nothing about what the body actually contained.
Reading bytes first allows the *same* bytes to serve both paths: an error
status prints an excerpt of the real body, and a decode failure is reported
with serde's own reason. The distinction between "invalid JSON" and "valid JSON
that no longer matches our types" is the difference between a five-minute and a
two-hour debugging session, and `schema_mismatch_is_reported_as_decode_failure_not_invalid_json`
pins it.

`body_excerpt` caps at 512 *chars*, not bytes (`String::from_utf8_lossy` then
`.chars()`, so it cannot split a multi-byte character), and appends `…` only if
there was more. The `chars.by_ref().take(n)` idiom consumes 512 and then asks
the *same* iterator whether anything remains.

### The two endpoint methods, and why they differ

```rust
pub async fn watchlists(&self) -> Result<WatchlistsResponse, ApiError> {
    match response.is_succeeded {
        Some(true) => {}
        Some(false) => return Err(ctx.error(ApiErrorKind::ApiFailure { detail })),
        None => return Err(ctx.malformed("watchlists response omitted isSucceeded")),
    }
    let Some(status) = response.status else {
        return Err(ctx.malformed("watchlists response omitted status"));
    };
```

Matching on `Option<bool>` gives **three** cases, and all three are handled
distinctly. `None` is not folded into `false`: a missing `isSucceeded` means the
response is not the shape we believe it is, which is a different bug from eToro
reporting a failure. `Option<bool>` is a three-state value, and treating it as
two loses information.

Note also that HTTP 200 does **not** imply success: eToro can return `200 OK`
with `isSucceeded: false` inside. Checking the envelope is not paranoia.

```rust
pub async fn portfolio(&self) -> Result<PortfolioResponse, ApiError> {
    if response.client_portfolio.is_none() {
        return Err(ctx.malformed("portfolio response omitted clientPortfolio"));
    }
```

Much thinner, and not from laziness. `WatchlistsResponse` has
`exception`/`isSucceeded`/`meta`/`status` fields; `PortfolioResponse` has
**only** `clientPortfolio`. Different eToro teams, different envelope
conventions — the validation follows the schema. Any new endpoint therefore
starts with the question *what does this response's envelope actually
guarantee?*, not with copying an existing method.

`me()` is the third answer to that question and the most interesting: it
validates **nothing**. Upstream marks its nine useful fields required, so they
decode as plain values and serde rejects an incomplete response before the
method body runs. The check did not disappear — it moved into the type, where
it is enforced at every use site instead of once per call. The cost is that
serde's message becomes the only diagnostic, which is why a contract test
drops `gcid` and pins that the error names the missing field.

`exception_detail` converts the generated envelope into the error type's own
`ExceptionDetail`, so `src/error.rs` never mentions a generated type and does
not have to move when the schema snapshot is refreshed. Its `Display` uses a
**let-else** pattern in the same spirit as the original:

```rust
let Some(exception) = exception else {
    return "no exception details".to_owned();
};
```

Bind-or-diverge, keeping the happy path unindented. A test asserts the
no-exception case still produces a useful message.

## 3. `src/types/` — generated versus hand-written

`mod.rs` declares three modules: `manual` (hand-written), `components` (every
component schema, generated) and `tags` (generated facades re-exporting each
API tag's subset of `components`). Two crate-level `allow`s live there with
comments explaining them — `dead_code` because most generated types are not
used yet, `derivable_impls` because typify writes out `Default` impls by hand
and Clippy objects. Scoping them to this module keeps strict linting in force
for hand-written code.

The single-namespace-plus-facades shape mirrors upstream: the eToro spec keeps
one flat pool of component schemas, so generating a module per tag would
manufacture an ownership problem — `Instrument`, `Market` and `User` are each
reachable from several tags and would become distinct, incompatible Rust types.
One `components` module and thin `pub use` facades give tag-aligned paths with
exactly one definition per type, and no owner map to maintain.

### `manual.rs` — the escape hatch

Generated code cannot be edited, since regeneration would clobber it. The
pipeline therefore supports an override mechanism: schemas carry an
`x-rust-type` annotation naming a type in this crate, and typify references it
instead of generating its own.

**`Numeric`** is a newtype over `rust_decimal::Decimal` with hand-written
`Serialize`/`Deserialize` delegating to `rust_decimal::serde::arbitrary_precision`.
A JSON number goes from wire text straight to `Decimal`, **never through
`f64`**. `0.1` as an `f64` is `0.1000000000000000055511151231257827`; accumulate
a few thousand of those across positions and the P&L is wrong in a way that is
hard to trace.

`impl Deref for Numeric` allows calling `Decimal` methods directly on a
`Numeric` — convenient, mildly frowned upon outside smart-pointer types, and
defensible for a transparent newtype.

The tests are notable for their honesty.
`numeric_rounds_beyond_28_significant_digits` does not assert that everything is
exact; it *pins the point where the guarantee stops*, with a comment saying that
if the test fails, the documented contract needs revisiting.
`numeric_rejects_out_of_range_magnitude` pins that oversized values are a clean
error rather than a silent wrap. Tests as executable documentation of limits,
not only of successes.

**`int_or_string_enum!`** is a `macro_rules!` macro. The pattern reads:

```rust
$vis:vis enum $name:ident {
    $($variant:ident = $int:expr => $str:literal),+ $(,)?
}
```

`$(...),+` means "one or more, comma-separated"; `$(,)?` permits a trailing
comma. From one declaration it generates the enum, an `as_str()`, a `Serialize`
that always writes the integer, and a `Deserialize` accepting **either** the
integer or the string name (via an internal `#[serde(untagged)]` helper). It
exists because eToro's spec declares `type: integer` while listing string
variant names, and different endpoints have been observed doing different
things.

The failure arms matter as much as the success arms: an unexpected value
produces `invalid integer for TradeDirection: 99`, never a silent default. Note
the consequence — these enums are **closed**. If eToro adds a fifteenth asset
type, the whole response fails to decode. That is a deliberate trade (loud spec
drift over silent data loss) recorded in `architecture.md`, and it remains an
open decision.

Most of this macro's output is now **unreferenced**, and the reason is a good
illustration of the pipeline earning its keep. At v1.355.0 upstream deleted the
standalone integer-enum components and inlined them at each use site as plain
`type: string` enums, so 16 of the 19 `x-rust-type` stamps had nothing left to
attach to — caught immediately by the override guard rather than by a silent
behaviour change. Only the three `PublicAggregatedInfo*` enums are still wired
in. The rest stay in the file because they encode something the spec no longer
states: the integer value each variant maps to, and the observation that some
endpoints sent integers where others sent names. If a live response ever fails
to decode one of these as a string, that tolerance is the thing to reach for.

## 4. `src/main.rs` — thin, except for one careful part

Load `.env`, read two variables, build the client, make two calls, print counts.
`dotenvy::dotenv().ok()` deliberately ignores the error: no `.env` file is fine
when the variables are in the real environment.

The interesting code is `write_private`:

```rust
match std::fs::remove_file(path) {
    Ok(()) => {}
    Err(error) if error.kind() == ErrorKind::NotFound => {}   // fine, nothing there
    Err(error) => return Err(error.into()),                   // a real problem
}
let mut file = OpenOptions::new().write(true).create_new(true).mode(0o600).open(path)?;
file.set_permissions(Permissions::from_mode(0o600))?;
```

The threat: if the dump path is a symlink pointing at, say,
`~/.ssh/authorized_keys`, a naive `fs::write` follows it and destroys the
target. So: unlink first, then `create_new(true)` — that is `O_CREAT | O_EXCL`,
which **never follows a symlink** and **fails** if anything reappears at the
path in the gap. The `set_permissions` after `.mode(0o600)` is needed because
`mode` is filtered through the process umask; this pins the exact bits.
Permissions are set *before* any bytes are written, so there is no window in
which the file exists world-readable with data in it.

The `Err(error) if ... => {}` line is a **match guard**. It distinguishes "not
there" from "there but unremovable", rather than swallowing both with
`let _ = remove_file(path)`.

The three tests assert *outcomes* — mode is `0600`; the symlink's target still
holds its original bytes — not implementation details. `TempPath` with a `Drop`
impl cleans up even when an assertion panics.

`#[cfg(unix)]` / `#[cfg(not(unix))]` provide two implementations; the non-Unix
one is a plain `fs::write` with none of these guarantees.

## 5. `tests/client_contract.rs` — testing HTTP without HTTP

`serve_once` spawns a thread with a real `TcpListener` on port 0 (the OS picks a
free port, so parallel tests cannot collide), hands back the
`http://127.0.0.1:PORT` base URL, reads one request, sends one canned response,
and exits. The client under test is the **real** client making a **real** TCP
request; only the server is fake. That is why these tests catch things a
Rust-level mock never would, such as the actual request line and headers.

The accept loop earns its comment:

```rust
// `TcpListener::accept` has no timeout, so a client that never connects
// would park this thread forever and turn a failing test into a hang.
listener.set_nonblocking(true).unwrap();
let deadline = Instant::now() + Duration::from_secs(5);
```

A test that *hangs* is much worse than one that fails, since CI times out with
no useful output. Non-blocking accept against a deadline converts the hang into
an assertion failure.

Requests are returned over an `mpsc::channel` so the test thread can assert on
what the server received. The seven tests cover: the happy path with exact
request line and headers (watchlists); decimal precision surviving the round
trip (portfolio); HTTP errors surfacing status, body, and request ID; API-level
failure both with and without exception details; decode-versus-invalid-JSON
wording; and the HTTPS guard's allow and reject lists.

The fixtures are **synthetic** — `"Sanitized demo watchlist"`,
`"fixture-watchlist"`, CID `123456` — and carry deliberately absurd precision
(`100.0000000000000000000001`) so the `Numeric` guarantee is exercised
end-to-end through real HTTP, not only in a unit test. They must never be
replaced with a real dump.

## 6. The regeneration pipeline

Three stages, documented in full in [`scripts/README.md`](../scripts/README.md):

```text
[retrieve]  MCP docs server → docs/spec/*.json        (agent-driven, committed)
[extract]   + docs/overrides/schemas.json → typify input  (build_typify_input.py)
[codegen]   → src/types/components.rs + tags.rs       (cargo typify + second pass)
```

`docs/spec/` is a committed snapshot of the upstream OpenAPI document — 242
component schemas and a 169-operation index at API v1.355.0. Committing it is
what makes stages 2 and 3 deterministic and offline: builds never touch the
network, and upstream drift arrives as a reviewable diff instead of a surprise.

Retrieval is the one stage that is not a plain script. The document is only
served through the eToro API Docs MCP server; probing the docs host for a
public copy returns 403 behind bot protection, or 404 at every plausible path.
So `/refresh-spec` drives it, and `scripts/fetch_spec.py` assembles and
validates the result.

`scripts/build_typify_input.py` merges `docs/overrides/schemas.json` onto the
snapshot and applies six transforms whose **order is load-bearing**: enums
first, because the `x-enumNames` → `x-enum-varnames` rename must happen before
unknown `x-*` keys are stripped; nullability last, so it sees the final type
and enum shape.

Two guards in that script are worth understanding, because both have already
caught real drift:

**An override naming a schema that does not exist is a hard error.** Without
it, upstream renaming a type would silently detach a `Numeric` or manual-enum
override, and a monetary field would quietly become an `f64`. At the v1.355.0
refresh this fired on 16 of 19 stamps at once — upstream had deleted those
integer-enum components and inlined them as string enums.

**Every facade re-export is resolved against the real generated file.** typify
renames schemas on the way to Rust (`meResponse` → `MeResponse`), so facade
generation is a second pass that runs after codegen; a schema mapping to no
emitted type fails the build rather than vanishing from the public API.

The subtlest thing in the repository is still the `x-rust-type` version pin:

> typify treats the version as a semver *requirement*; if it is not satisfied
> by the `--crate` flag, the override is **silently ignored** and typify falls
> back to generating its own type (f64 for numbers, ...).

A silent fallback to `f64` would quietly undo the decimal guarantee, so both
the Python script and the shell script read the version from the same
`Cargo.toml` field.

The shell script's other defensive touches: cargo-typify pinned to exactly
`0.6.2` (codegen output is not stable across versions); a `mktemp -d` workdir
with a `trap` cleanup; `cargo fmt` after generation, because typify's output is
not rustfmt-clean and would otherwise fail the gate on every run; and a closing
`cargo check`. Running it twice produces identical bytes.

## 7. Loose threads

Patterns worth keeping: comments explain *why* rather than *what*; tests pin the
edges of guarantees rather than only happy paths; each security decision
(sensitive headers, HTTPS guard, `O_EXCL`) has a test; the library never touches
the environment.

Open items, in rough priority order:

1. **`reqwest`'s `json` feature is unused** — a one-line removal.
2. ~~**`anyhow` everywhere**~~ — done. `src/error.rs` now defines `ClientError`
   and `ApiError`; callers branch with `is_retryable()` / `retry_after()` rather
   than matching strings. The variants are shaped by the decisions a caller
   makes, not by the 15 places the client can fail.
3. **`portfolio()`'s thin validation** is schema-driven, but it means the
   client's guarantees vary per endpoint. Worth stating explicitly as more
   endpoints are added.
4. **Closed enums** — the parked decision. It will bite eventually; the question
   is whether it bites loudly now or quietly later.
5. **No retries and no rate-limit awareness** — deliberate and documented.
   `get_json` is the single chokepoint where that policy would live, which is a
   useful property of the current design. `docs/spec/operations.json` now
   records eToro's actual rate-limit pools, including which endpoints share a
   budget, so that policy no longer has to be guessed at.
