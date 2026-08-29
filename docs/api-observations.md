# API observations

Things learned about the eToro API that the OpenAPI document does not say, or
says incorrectly. Kept separate from `docs/spec/` (which is verbatim upstream)
and from `docs/overrides/` (which is pipeline input).

**Provenance:** everything below was recorded against API **v1.158.0** while
hand-curating the original per-domain schema extracts. The snapshot is now at
**v1.355.0**. Notes marked ⚠️ describe something the newer spec states
differently — but a spec change is not proof the *service* changed, and several
of these notes exist precisely because the spec was wrong. Re-verify against a
live response before relying on either version.

---

## Identity

- `meResponse` returns the **authenticated** user (gcid + realCid + demoCid).
  Use it to discover your own customer IDs.
- `User` is the lightweight profile shape used inside posts, comments, feeds
  and copy metadata.
- `PublicAggregatedInfoUser` is the rich profile (~30 fields) with verification
  status, account state, GDPR info and copy-trading attributes.
- Three integer-encoded enums are embedded here: `AccountStatus` (2 variants),
  `PlayerStatus` (15), `PlayerStatusReason` (43, values 0–42). Wire format is
  the integer. These are the three that still use `x-rust-type` overrides.
- **Casing differs per endpoint:** `meResponse` uses lowercase `cid`
  (`gcid`/`realCid`/`demoCid`) while `PublicAggregatedInfoUser` uses `CID`
  (`realCID`/`demoCID`/`masterAccountCid`). Same concept, different spelling.
Confirmed against a live `GET /api/v1/me` response (2026-08-23, API v1.355.0):

- **`middleName` is sent as `""`, not `null`, when absent.** The spec declares
  it `nullable: true`, so the generated field is `Option<String>` — but the
  absent case arrives as `Some("")` and never as `None`. Code matching on
  `Some(..)` still has to handle the empty string. Pinned by
  `me_contract_uses_expected_path_and_headers`.
- **`dateOfBirth` is a plain date** (`"1985-06-06"`), not RFC 3339. Leaving it
  as `String` is correct; adding `format: date-time` would make the response
  fail to decode.
- The response decodes with **no hand-written validation** in `me()` — every
  field the client relies on is non-`Option`, so serde enforces presence. A
  missing required field produces `missing field \`gcid\`` wrapped in the
  `get_json` decode context, which is actionable enough to rely on.
- `demoCid` is populated even when the token holds **no demo scope**. You can
  see the demo account without being able to act on it.

- ⚠️ `UserRole` was a standalone integer enum (11 variants: Regular, PI,
  Moderator, Anonymous, eToroTeam, eTorian, CopyPortfolio, Depositor, Admin,
  Verified, Analyst). Note `User.roles` was an array of **strings** while the
  standalone `UserRole` was `type: integer` — the same enum encoded differently
  in different contexts. At v1.355.0 the standalone component is gone and
  `User.roles` is an inline string enum.

## Market data

Spec-vs-reality, all verified empirically:

- **`fields=` is marked required on `GET /api/v1/market-data/search` but is not
  enforced** — a request without it returns 200.
- The default search response (no `fields=`) returns ~50 analytic and
  market-cap fields **not described in the `Instrument` schema**, many with
  hyphenated names like `peRatio-TTM-System-Avg` that need `#[serde(rename)]`.
- The default search response includes a **system-aggregate pseudo-instrument
  with a negative `instrumentId` (-100000)** carrying crypto-market-wide stats
  (Bitcoin dominance and similar). Filter out negative IDs.
- Standard fields (`symbol`, `displayname`, `exchangeID`, `instrumentType`)
  return **null unless explicitly listed in `fields=`**.
- Default `pageSize` is 20; `totalItems` can exceed 11,000. Always paginate or
  filter narrowly.
- Live rates use `instrumentID` (capital ID), unlike `instrumentId` in the
  search response.
- Live rates mark several fields obsolete (`unitMargin*`, `*Discounted`).
- `candlesResponse` nests `candles` inside `candles`: outer items are
  per-instrument, inner items per-time-period. Outer uses `instrumentId`, inner
  uses `instrumentID`. Yes, really.
- `closingPricesResponse` is a **bare array**, no envelope. Prices can be `-1`
  as a sentinel for "no data" — worth an `Option<Decimal>` via a custom
  deserializer rather than letting `-1` through as a price.
- The standalone `Market` schema differs from the inline market object inside
  `WatchlistItemDto`, despite the shared name.
- ⚠️ `MarketAssetType` and `MarketEventTag` were integer-encoded enums with
  string-named variants (wire format the integer index). At v1.355.0 they are
  inline `type: string` enums.

Modelling advice that still holds:

- For symbol resolution pass `internalSymbolFull=AAPL` **and**
  `fields=instrumentId,internalSymbolFull,displayname,symbol`, then verify the
  exact `internalSymbolFull` match.
- Treat all `Instrument` fields as optional — none are required by spec, and
  reality confirms many are null.

## Trading

- Two parallel naming conventions coexist: the user-facing camelCase set
  (`createOrderRequest`, `getOrderResponse`) and .NET-namespaced internal DTOs
  (`eToro.Trading.DistributedServices.WebApi.API.DTO.Requests.*`). API users
  call the former.
- Heavy casing inconsistency: `instrumentId` vs `instrumentID` vs
  `InstrumentId` vs `InstrumentID` across schemas.
- **All price/amount/rate fields are `float` in the spec.** This is why the
  pipeline redirects every `type: number` to `manual::Numeric`; never let one
  through as `f32`/`f64`.
- `orderType` is `MKT`/`LMT` and `executionType` is `GTC`/`IOC` as string
  enums — **but** `Order`, `OrderForOpen`, `OrderForClose` and
  `OrderForOpenInfoResponse` used **integer** `orderType`/`executionType`. Same
  concept, different wire format per endpoint.
- `Position` has ~35 fields and `Mirror` ~26, many marked obsolete. Model
  defensively and read the descriptions.
- ⚠️ `TradeDirection` (Long/Short) and `TradeType` (Open/Close) were
  integer-encoded; at v1.355.0 they are inline string enums.

## Portfolio

- `PortfolioResponse` and `PortfolioResponseWithPnl` both wrap a single
  `clientPortfolio`; the WithPnl variant is for endpoints that include P&L.
- `getUserDailyGainResponse` uses `oneOf`: it is **either** a list of gain
  entries **or** an object `{gain: number}`. Needs an untagged enum.
- `PortfolioResponse` inlines the whole shape while `PortfolioResponseWithPnl`
  uses `$ref`s — same data, different definition style. The inlined version
  uses `orderID`/`instrumentID`/`CID` (capital) where the `$ref`'d `Order`
  schema uses lowercase. Pick per the endpoint you actually call.
- `GeCopiersResponse` looks like a spec typo for `GetCopiersResponse`.

## Social feeds

- Largest domain in the API. Two parallel post shapes: `Post` (modern, uses
  `$ref`s) and `DiscussionsPost` (older, duplicates structures inline). Two
  parallel list-response shapes likewise.
- `Comment` is **recursive** — `replies` contains further comments. Needs
  `Box` to break the cycle in Rust.
- `Discussion.reason` has **no schema at all** and is described as "can be a
  string or an object". This is the field that motivated the untyped-nullable
  handling in the pipeline: forcing `type: null` on it made typify emit `()`,
  which rejects every real value.
- ⚠️ Many small integer-encoded enums lived here — `PostType` (8),
  `MediaType` (4), `EditStatus` (3), `VideoSource` (3), `ParentType` (4),
  `ReasonType` (7), `EmotionType` (1), `ArticleRating` (2), `ArticleStatus` (3),
  `CopyType` (2) — all wire-encoded as the integer index. At v1.355.0 every one
  is an inline `type: string` enum.

## Agent portfolios

- An agent-portfolio is a programmatically managed copy-trading portfolio with
  its own scoped user tokens.
- **The `userToken` secret is only returned at creation time.** Store it
  immediately; it cannot be retrieved later.
- `investmentAmountInUsd` is taken from the **caller's** account; the
  agent-portfolio receives its own separate virtual balance
  (`agentPortfolioVirtualBalance`) used for proportional position sizing.
- Scope IDs seen at v1.158.0: 200 = `real:read`, 201 = `demo:read`,
  202 = `real:write`, 203 = `demo:write`. ⚠️ v1.355.0 documents scope *names*
  and marks the numeric `scopeIds` field deprecated.

---

## Retired `x-rust-type` stamps

Sixteen schemas carried hand-written `x-rust-type` stamps pointing at
`int_or_string_enum!` types in `src/types/manual.rs`. At the v1.355.0 refresh
upstream deleted all sixteen standalone components and inlined them at each use
site as plain `type: string` enums, so there is nothing left to stamp and the
declared wire format is no longer the integer index.

`ApplicationSource`, `ArticleRating`, `ArticleStatus`, `CopyType`,
`EditStatus`, `EmotionType`, `MarketAssetType`, `MarketEventTag`, `MediaType`,
`ParentType`, `PostType`, `ReasonType`, `TradeDirection`, `TradeType`,
`UserRole`, `VideoSource`.

The Rust types remain in `manual.rs`, unreferenced, because they accept **both**
the integer and the string form. If a live response ever fails to decode one of
these fields as a string, that is the tolerance to reach for.
