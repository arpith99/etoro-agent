# Code walkthrough

A guided tour of the existing code: what each part does, why it is written the
way it is, and which Rust idioms are worth carrying forward. Where
[`architecture.md`](architecture.md) states the invariants, this document
explains the implementation that upholds them.

Describes the tree as of commit `945d525` (2026-08-16), at which point the
suite is 20 tests green: 10 library unit tests, 3 in `main.rs`, 7 contract
tests.

## 1. The crate's shape

`Cargo.toml` declares one package that builds **two things**: a library
(`src/lib.rs`) and a binary (`src/main.rs`). That split is the most important
structural decision in the project.

```rust
// src/lib.rs
extern crate self as etoro_agent;
pub mod client;
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
| `anyhow` | One error type (`anyhow::Error`) for the whole app, with `.context()` chaining. Fine for a binary; a library published for others would want typed errors instead (`thiserror`). |
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
let sensitive = |s: &str| -> Result<header::HeaderValue> {
    let mut value = header::HeaderValue::from_str(s)?;
    value.set_sensitive(true);
    Ok(value)
};
```

A **closure returning `Result`**, so `?` works inside it. `set_sensitive` does
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
ensure!(
    base_url.scheme() == "https" || (base_url.scheme() == "http" && is_loopback(&base_url)),
    "refusing to send API credentials over non-HTTPS base URL {base_url}"
);
```

`ensure!` is anyhow's `if !cond { return Err(...) }`. The rule is
*credentials-shaped*, not URL-shaped: because auth headers are baked into the
client, any origin it is pointed at receives them, so plaintext is only
tolerable where the bytes cannot leave the machine. `is_loopback` handles
`localhost`, anything parsing as a loopback `IpAddr` (all of `127.0.0.0/8`, not
just `127.0.0.1`), and strips `[...]` for IPv6 literals such as `[::1]`.

### `get_json` — the one place HTTP happens

```rust
async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<(T, String)>
```

**Generic over the response type**, private, and returns the decoded value
**plus the request ID**. Three things to notice:

**`DeserializeOwned` rather than `Deserialize<'de>`.** The bound means "can be
deserialized without borrowing from the input buffer" — necessary because
`body` is a local that dies at the end of the function, so a type borrowing
`&str` slices out of it could not outlive it.

**The request ID is returned, not swallowed.** Every error message in every
endpoint method carries it, so a support question of the form "which request?"
has an answer. Threading it through the return type is the plumbing cost.

```rust
let body = response.bytes().await?;
if !status.is_success() { bail!("GET {url} returned {status} ...: {}", body_excerpt(&body)); }
let value = serde_json::from_slice(&body).with_context(|| ...)?;
```

**This is why `.json()` is not used.** `reqwest`'s `.json()` consumes the
response and, on failure, says nothing about what the body actually contained.
Reading bytes first allows the *same* bytes to serve both paths: an error
status prints an excerpt of the real body, and a decode failure is reported
with serde's own reason. The distinction between "invalid JSON" and "valid JSON
that no longer matches our types" is the difference between a five-minute and a
two-hour debugging session, and `schema_mismatch_is_reported_as_decode_failure_not_invalid_json`
pins it.

`with_context(|| ...)` takes a **closure**, so the `format!` runs only on the
error path; the eager `.context("...")` would allocate on every successful call.

`body_excerpt` caps at 512 *chars*, not bytes (`String::from_utf8_lossy` then
`.chars()`, so it cannot split a multi-byte character), and appends `…` only if
there was more. The `chars.by_ref().take(n)` idiom consumes 512 and then asks
the *same* iterator whether anything remains.

### The two endpoint methods, and why they differ

```rust
pub async fn watchlists(&self) -> Result<WatchlistsResponse> {
    match response.is_succeeded {
        Some(true) => {}
        Some(false) => bail!("... API-level failure ({detail}; request ID {request_id})"),
        None => bail!("watchlists response omitted isSucceeded ..."),
    }
    let status = response.status.with_context(|| ...)?;
    ensure!((200..300).contains(&status), "...");
```

Matching on `Option<bool>` gives **three** cases, and all three are handled
distinctly. `None` is not folded into `false`: a missing `isSucceeded` means the
response is not the shape we believe it is, which is a different bug from eToro
reporting a failure. `Option<bool>` is a three-state value, and treating it as
two loses information.

Note also that HTTP 200 does **not** imply success: eToro can return `200 OK`
with `isSucceeded: false` inside. Checking the envelope is not paranoia.

```rust
pub async fn portfolio(&self) -> Result<PortfolioResponse> {
    ensure!(response.client_portfolio.is_some(), "...");
```

Much thinner, and not from laziness. `WatchlistsResponse` has
`exception`/`isSucceeded`/`meta`/`status` fields; `PortfolioResponse` has
**only** `clientPortfolio`. Different eToro teams, different envelope
conventions — the validation follows the schema. Any new endpoint therefore
starts with the question *what does this response's envelope actually
guarantee?*, not with copying an existing method.

`describe_exception` uses a **let-else**:

```rust
let Some(exception) = exception else {
    return "no exception details".to_owned();
};
```

Bind-or-diverge, keeping the happy path unindented. A test asserts the
no-exception case still produces a useful message.

## 3. `src/types/` — generated versus hand-written

`mod.rs` is 20 lines: `pub mod manual;` plus seven generated domains. Two
crate-level `allow`s live there with comments explaining them — `dead_code`
because most generated types are not used yet, `derivable_impls` because typify
writes out `Default` impls by hand and Clippy objects. Scoping them to this
module keeps strict linting in force for hand-written code.

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
the consequence — these enums are **closed**. If eToro adds a fifteenth
`MarketAssetType`, the whole response fails to decode. That is a deliberate
trade (loud spec drift over silent data loss) recorded in `architecture.md`, and
it remains an open decision.

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

`docs/*-schema.json` are the source of truth, extracted from eToro's portal.
Each carries a `_meta` block recording the source, the API version
(`v1.158.0`), and hand-written notes; the `market_data` one documents several
places where the search endpoint's behavior differs from its spec.

`scripts/typify_prep.py` turns them into self-contained JSON Schema 2020-12
documents: resolving `$ref`s across domain files, rewriting
`#/components/schemas/X` to `#/$defs/X`, converting OpenAPI `nullable` into
JSON Schema null-unions, redirecting every `type: number` to `Numeric`, and
pinning `x-rust-type` versions.

That last step is the subtlest thing in the repository, and its docstring
explains why:

> typify treats the version as a semver *requirement*; if it is not satisfied
> by the `--crate` flag, the override is **silently ignored** and typify falls
> back to generating its own type (f64 for numbers, ...).

A silent fallback to `f64` would quietly undo the entire decimal guarantee. The
script therefore reads the version from `Cargo.toml`, and
`regenerate-types.sh` reads it from the same place — one source of truth, no
drift.

The shell script's other defensive touches: cargo-typify pinned to exactly
`0.6.2` (codegen output is not stable across versions); the domain list derived
by globbing `docs/` rather than hardcoded; a **refusal to run** if a schema file
exists without a matching `pub mod` in `mod.rs`, so a new domain cannot be
silently generated but never compiled; `mktemp -d` with a `trap` cleanup; and a
closing `cargo check`.

## 7. Loose threads

Patterns worth keeping: comments explain *why* rather than *what*; tests pin the
edges of guarantees rather than only happy paths; each security decision
(sensitive headers, HTTPS guard, `O_EXCL`) has a test; the library never touches
the environment.

Open items, in rough priority order:

1. **`reqwest`'s `json` feature is unused** — a one-line removal.
2. **`anyhow` everywhere** suits a binary, but as soon as something needs to
   *react* to a failure differently (retry a 429, re-authenticate on 403),
   typed errors are required. Today a caller can only match on error strings.
3. **`portfolio()`'s thin validation** is schema-driven, but it means the
   client's guarantees vary per endpoint. Worth stating explicitly as more
   endpoints are added.
4. **Closed enums** — the parked decision. It will bite eventually; the question
   is whether it bites loudly now or quietly later.
5. **No retries and no rate-limit awareness** — deliberate and documented.
   `get_json` is the single chokepoint where that policy would live, which is a
   useful property of the current design.
