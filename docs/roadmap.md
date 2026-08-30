# Roadmap: from read-only client to a live strategy

The goal is a minimum working library, a simple strategy validated by a
backtest, a period of paper trading, and finally a small live allocation
($100–500). This document records the plan and — more importantly — the facts
that shaped it, so the reasoning can be re-examined rather than re-derived.

Written 2026-08-29, against eToro API v1.355.0.

## The central decision: three layers, not one vendor

The project currently has one provider doing three jobs. eToro is good at two
of them and poor at the third.

| Layer | eToro's fitness | Decision |
|---|---|---|
| Historical bars | Poor — capped at 1000 candles, no date range, no backward pagination, adjustment status unknown | Use a dedicated data vendor |
| Backtest engine | n/a | Write it in Rust, in this repo |
| Live execution and monitoring | Good — commission-free real stocks, fractional sizing, WebSocket order and price feeds | Keep eToro |

Switching brokers to fix a *data* problem would discard the generated types and
the schema pipeline to solve something a different HTTP endpoint solves. The
layers are kept separate so each can be replaced independently.

## Constraints established from the spec

These are the findings that constrain the plan. Each is checkable against
`docs/spec/` or the API docs.

**Historical data is a trailing window, not a queryable range.**
`GET /api/v1/market-data/instruments/{instrumentId}/history/candles/{direction}/{interval}/{candlesCount}`
takes only those three parameters. `candlesCount` maxes at 1000 and there is no
`from`/`to`; `direction` is the sort order of the returned set, not an offset.
So the most recent 1000 bars are reachable and nothing older, ever — roughly
four years of daily bars. Two consequences: candles must be persisted locally
from the first fetch, because the window rolls forward and the oldest bar
becomes permanently unreachable; and a backtest over the most recent four years
on instruments chosen today is survivorship-biased across a single market
regime. It can demonstrate the machinery is correct. It cannot distinguish "the
strategy works" from "the market went up."

**Split and dividend adjustment is the highest-impact correctness trap.**
`candlesResponse` carries raw open/high/low/close with no adjustment field.
Backtesting a 4:1 split on unadjusted prices presents a 75% single-bar drawdown
that a strategy will happily trade. This is the strongest argument for a real
data vendor: adjusted series are bought, not derived.

**API keys are environment-scoped.** From the authentication docs: *"Each key
can only be used for one environment. If you need to use both, please create
two keys."* Paper trading therefore needs a **second key** (Demo + Write), not
additional scopes on the existing one. The current key is Real + Write, meaning
the only environment presently reachable for writes is the one holding real
money. This resolves the open question recorded in earlier sessions.

**`x-request-id` is the idempotency key.** The order endpoints require a unique
GUID *"for idempotency"*, and `referenceId` in the lookup response echoes it
back — *"the only handle you have if this response is lost."* This settles the
retry design: a **retried** write reuses its request ID, while a new write mints
a fresh one. `get_json` currently generates a new UUID per call, which is
correct for reads and wrong for retried writes.

**Order submission is asynchronous.** A 202 means accepted, not executed.
Outcomes are confirmed via `GET /api/v2/trading/info/orders:lookup` by `orderId`
or `referenceId`. Relevant `status.id` values: 3 Filled and 5 PartiallyFilled
are terminal successes; 4 Rejected and 10 RejectedPartiallyFilled carry the
reason in `status.errorCode`/`errorMessage`; 1 Received, 2 Placed, 11
WaitingForMarket and 12 PendingTriggeredRate are still in flight; 6 PendingCancel,
7 Canceled and 9 CanceledPartiallyFilled cover cancellation.

**A WebSocket API exists.** Live bid/ask per instrument, and a `private` topic
(requires prior Authenticate) pushing order and position updates. Fill
monitoring does not have to be built on polling.

**Rate limits are pooled, not per-endpoint.** Order execution is 20 requests/60s
shared across about ten endpoints; market data is a separate 120/60s pool;
everything else draws on a shared 60/60s default. Full membership lists are in
`docs/spec/operations.json`.

**Costs should be queried, not estimated.** `POST /api/v2/trading/info/costs`
returns a what-if cost breakdown, and `POST /api/v2/trading/info/eligibility`
reports permissions, limits, and available leverage. At $100–500, spread and
fees dominate almost any signal a simple strategy produces, so these are inputs
to the strategy, not diagnostics.

## Milestones

Milestones 2–4 are broker-agnostic and cost nothing if the venue later changes.

### 1. Query parameters and live market data — **done 2026-08-30**

`get_json` takes query parameters; `rates()`, `search()` and `resolve_symbol()`
are implemented, and the binary prices a comma-separated symbol list
(`cargo run -- AAPL,GOOG,MSFT`) as N resolutions plus one batched `rates` call.
`instruments()` turned out to be unnecessary — `search()` covers symbol
resolution — and `candles()` is deferred, since the data layer is moving off
eToro anyway.

Two things this settled that later milestones depend on:

- **The percent-encoded comma is accepted.** `Url::query_pairs_mut` emits
  `instrumentIds=1001%2C1002`, and eToro decodes it, so `style: form,
  explode: false` parameters need no special-casing.
- **`GET /market-data/search` emits `instrumentId` twice in one item**, which
  serde's derived `Deserialize` rejects. That endpoint alone decodes through
  `serde_json::Value`; everything else stays strict, because strictness is what
  surfaces spec drift. Both findings are in
  [`api-observations.md`](api-observations.md).

### Observed spreads — an input to milestone 3

Measured live on 2026-08-30, as a fraction of mid:

| Instrument | Spread | % of mid |
|---|---|---|
| AAPL | 0.01 on 319.70 | 0.003% |
| MSFT | 0.03 on 513.59 | 0.006% |
| INTC | 0.01 on 89.49 | 0.011% |
| MBLY | 0.01 on 8.60 | 0.116% |
| (watchlist mid-cap) | 0.02 on 12.27 | 0.163% |

A ~50× range across ordinary large- and mid-caps. A backtest carrying one flat
spread will be far too pessimistic at the liquid end and dangerously optimistic
at the thin end, so the cost model should be per-instrument and seeded from
observed rates rather than a single constant.

### 2. Candle store — **done 2026-08-30**

A local store, merge-on-refetch, keyed by (instrument, interval, timestamp).
Necessary because the API window rolls forward; every fetch not persisted is
history permanently lost.

This is also where the data source is chosen.

**Tiingo** is the choice, for a reason beyond history depth: it returns
`close` *and* `adjClose` per bar, plus the `divCash` and `splitFactor` that
explain the difference. That corporate-action metadata is what makes it usable
as a **referee** — a series to validate any other source against. A vendor that
returns only a price leaves you exactly where eToro does, holding an unlabelled
number.

**Stooq was ruled out** on 2026-08-30. Two reasons, the second sufficient on
its own:

- Its CSV endpoint serves a JavaScript proof-of-work challenge rather than
  data, even with a browser user agent. Getting past it means defeating a bot
  protection, which is out of scope here for the same reason it was for the
  eToro docs portal. This may be IP- or region-dependent, so it is worth
  re-checking before treating it as permanent.
- It carries no per-bar corporate-action fields, so it can neither answer the
  adjustment question nor be checked against a source that can.

`scripts/compare_candle_sources.py` was the spike, and it has run. **eToro
candles are split-adjusted but not dividend-adjusted** — a price-return series
matching Tiingo's `close`, never `adjClose`. Evidence and the ~0.16% tracking
error are in [`api-observations.md`](api-observations.md).

That settles the storage schema, and the conclusion is: **do not bake the
adjustment decision into storage.** Persist Tiingo's `close` *and* `adjClose`
alongside `divCash` and `splitFactor`, so price-return versus total-return
becomes a query-time choice. Storing one adjusted series discards the
information needed to recover the other, and which one a strategy should use is
not knowable in advance — a momentum rule wants price return, anything holding
dividend payers for the yield wants total return.

**Done.** `src/data/` holds the layer: a `BarSource` trait with a `Tiingo`
implementation, and a `BarStore` trait with a newline-delimited-JSON
`FileStore`. `cargo run -- fetch-bars AAPL,MSFT` wires them together.

Two decisions worth carrying forward. `PriceBasis` is part of the trait
contract rather than documentation, because which convention a series follows
is not observable from the data — and eToro and Tiingo genuinely differ.
And the store's path layout, `<source>/<symbol>/<interval>.ndjson`, makes
mixing conventions within one series impossible rather than merely
discouraged; a mismatched merge is refused outright.

Not SQLite: at daily resolution a symbol is roughly a thousand rows a decade,
so indexes buy nothing, and SQLite has no decimal type — prices in a `REAL`
column would silently reintroduce the `f64` loss the wire types exist to
prevent. Revisit at intraday resolution, where a symbol-year is ~100k rows;
`BarStore` being a trait makes that an implementation swap.

### 3. Backtest engine

A `Strategy` trait plus an event loop over stored bars, producing a trade log
and summary statistics. Entirely offline.

Make the cost model a **required** constructor argument rather than an optional
one, so a zero-cost backtest is something that must be asked for explicitly.
The usual failure mode is a backtest that quietly assumed free trading.

For long-only daily bars this is a few hundred lines; the loop is not where the
difficulty lives. The traps taken on by writing it rather than using a mature
framework are lookahead bias, price adjustment, and cost modelling. Buying
adjusted data removes the worst of the three.

**Done when** a known-answer case passes in `cargo test`.

### 4. Strategy #1 — dual SMA crossover

Long-only, daily bars, a handful of large caps.

This is scaffolding, not an edge. Its purpose is to exercise the engine end to
end so that a real idea later lands on a harness that has been proven.

**Done when** an equity curve and trade log can be produced from stored data.

### 5. Demo key and paper trading

Requires generating a second API key (Demo + Write) — an external dependency
with lead time, worth starting early.

- Introduce an `Environment` (Demo | Real) fixed at client construction, with
  every write path gated on it. `me()` returns the token's scopes, so the
  declared environment can be cross-checked against the key at startup rather
  than trusted.
- Order submission plus the asynchronous status state machine above, using
  `referenceId` as the recovery handle when a response is lost.
- The retry policy lands here, now that the idempotency rule is known.

Then run it on demo, unattended, for a meaningful period — and compare actual
demo fills against what the backtest predicted for the same period. That
comparison is the real validation, and it catches the cost and slippage errors
a backtest structurally cannot.

**Done when** demo has run unattended long enough to produce a fill-vs-backtest
comparison.

### 6. Live, small

Only after milestone 5 has produced that comparison. Prerequisites:

- Hard limits: maximum position size, maximum orders per day, maximum total
  exposure, and a kill switch.
- A cost check via `POST /api/v2/trading/info/costs` before every order, with a
  refusal path when cost is large relative to expected edge.
- Approval mode first — print the intended order and require confirmation —
  before anything runs unattended.
- Audit logging of every submitted order and its resolved outcome.

## The broker question, deferred

Staying with eToro is right for a daily-bar strategy at this size. The
conditions that would change that:

- **Intraday or minute-resolution strategies.** eToro's data limits get worse,
  not better. Alpaca is the natural alternative: a free paper account as a
  first-class environment and deep bar history with real date ranges.
  Eligibility depends on residency and should be checked before planning
  around it.
- **Global market breadth with serious execution.** IBKR, at the cost of a
  running gateway process and a thin Rust ecosystem around it.
- **Crypto being acceptable.** Binance or Kraken offer years of free candles
  with proper date ranges and no authentication, 24/7 markets that remove
  market-hours logic entirely, and minimum sizes that suit $100 far better than
  equities do. This is the lowest-friction path available.

If a switch happens, the backtest engine, strategy trait, candle store, and
error taxonomy all port. The 242 generated types do not.

## Open items

- ~~Confirm whether eToro candles are split- and dividend-adjusted.~~ Done:
  split-adjusted, not dividend-adjusted. And a candle's `close` is the last
  *bid* of the session, which is why eToro's closes run ~0.16% under Tiingo's.
  Both in [`api-observations.md`](api-observations.md).
- `docs/spec/operations.json` does not capture request parameters, only tags,
  scopes, rate limits, and schema `$ref`s. Endpoints with query or path
  parameters currently require reading the docs directly. Extending the
  projection is worthwhile once more parameterised endpoints are in use.
- eToro minimum trade size per asset is not exposed in the schemas reviewed so
  far; `POST /api/v2/trading/info/eligibility` is the authority.
- `reqwest`'s `json` feature is enabled but unused.
