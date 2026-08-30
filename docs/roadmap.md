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

### 3. Backtest engine — **done 2026-08-30**

`src/backtest.rs`: a `Strategy` trait, an event loop over stored bars, and a
report putting the strategy next to buy-and-hold. Entirely offline, so a run is
repeatable.

The loop was never the hard part. The three things that make a backtest lie are
lookahead bias, unadjusted prices, and absent costs, and the design spends
itself on making each one hard to commit rather than documented as a hazard.

**Lookahead is prevented by the trait's shape.** `fn target(&mut self, history:
&[Bar]) -> f64` hands the strategy a *prefix* ending on the decision bar. The
usual mistake — computing a signal from a column and forgetting to shift it —
has nothing to act on, because the future is not in the argument. A slice
rather than "the whole series plus an index" specifically because an index can
be read past.

**Costs are prevented from vanishing by the signature.** `CostModel` is a
positional argument to `run` with no `Default`, so a free backtest must be
spelled `CostModel::frictionless()`, and the report says so in its output.
Charges land on **turnover**, not per trade: going 20% → 30% costs a tenth of
opening a full position, because that is what changes hands. `from_spread`
halves the quoted spread, since each leg crosses from mid to one side, and the
observed 50× range across ordinary names is why the model is per-instrument.

**The fill model turned out to be the one real judgement call.** Two variants,
and the gap between them is the honest error bar on any result:

- `FillPrice::NextOpen` — decide on a close, fill at the next open. The
  pessimistic default. The gap between the close you decided on and the open
  you filled at is unavailable to you, because there was no moment in between
  at which you could trade. Returns are measured fill-to-fill, open to open, so
  a position held overnight still eats the gap it could not act on. That is the
  true cost of open fills rather than an approximation of it.
- `FillPrice::SameClose` — decide and fill on the same close. This is what the
  usual `signal.shift(1)` harness actually models, and it hands the strategy
  every overnight gap that follows a signal.

There is deliberately no "next close": filling a whole session after
`NextOpen` is strictly more delay, so it bounds nothing. The earlier sketch
listed one, and it was wrong.

Two smaller decisions. A target outside `[0, 1]`, `NaN` included, aborts the
run rather than being clamped — a clamp turns a strategy bug into a plausible
equity curve. And the benchmark is left *uncosted*, flattering it by the one
spread a real holder pays on entry; that bias runs against the strategy, which
is the direction to be wrong in when the question is "did this beat doing
nothing".

Reuse worth noting: `SideStats` from the gap analysis already computed
annualised return, volatility and `return_over_vol`, so the engine only added
max drawdown and the trade log. It is now `SideStats::from_returns`, taking its
window length from the slice — the two must agree for annualisation to mean
anything, and a caller that can disagree eventually will.

**Done.** Eleven tests, including a three-bar case computed by hand, a proof
that an always-long strategy reproduces the benchmark bit-for-bit when trading
is free, and one asserting the exact prefixes a strategy was handed.

### 4. Strategy #1 — dual SMA crossover — **done 2026-08-30**

`src/strategy.rs` holds `Sma` — long while the fast average is above the slow
one, flat otherwise — plus `sweep_sma` for the parameter grid.
`etoro-agent backtest SYM[,SYM...]` runs it.

```
etoro-agent backtest AAPL                     one full report
etoro-agent backtest AAPL,MSFT,NVDA           one row per name
etoro-agent backtest NVDA --sweep --trades    the grid, and every fill
etoro-agent backtest MBLY --spread 0.116      the spread actually quoted
```

Scaffolding, not an edge, and it behaved like scaffolding. Over five years of
stored total-return bars at 0.02% spread, filled at the next open, SMA 20/100
beat buy-and-hold on return per unit of volatility in **4 of 8** names
(NVDA 1.26 vs 1.10, GOOG 0.82 vs 0.57, AMD 0.83 vs 0.59, INTC 0.48 vs 0.23)
and lost in the other four, badly on AAPL (0.23 vs 0.56) and TSLA (−0.23 vs
0.13). That is the expected shape: a trend rule in a five-year window that
mostly trended up, thinning both the returns and the drawdowns.

**The sweep is the part worth having.** A 5×5 grid of window pairs turns "this
cell worked" into a question about the neighbourhood:

- NVDA: median 1.29 across 24 pairs against 1.10 for holding — a broad region,
  not a lucky cell, with 20/100 sitting *below* its own grid median.
- AAPL: median 0.26 against 0.56 — broadly and consistently worse, which is a
  cleaner verdict than any single cell could give.

Neither is evidence about the strategy in general. Eight US large-cap tech
names over one regime, chosen with hindsight, are nowhere near eight
independent observations, and the roadmap's survivorship warning applies in
full. What the exercise establishes is that the harness reports losses as
losses, which was the point.

**One design hole closed while building this.** `Strategy::target` now takes
`SeriesChoice` as an argument rather than letting a strategy pick its own. A
strategy that chose could signal on total-return prices while the engine filled
on as-traded ones, and the two series are indistinguishable from the outside —
exactly the failure mode `PriceBasis` exists to prevent, reintroduced one layer
up. Same fix as `SideStats::from_returns`: if two things must agree, do not
give callers two places to say them.

`--spread` takes a percentage and rejects anything above 1%, because
`--spread 20` meaning twenty basis points would otherwise charge twenty percent
and produce a plausible-looking disaster.

**Done.** Equity curve, trade log and sweep all come out of stored data, with
no network access anywhere in the path.

### 5. Demo key and paper trading — **in progress**

Requires generating a second API key (Demo + Write) — an external dependency
with lead time. **Still outstanding, and it blocks the rest of this milestone.**

- ~~Introduce an `Environment` (Demo | Real) fixed at client construction, with
  every write path gated on it.~~ **Done 2026-08-30**, and the mechanism turned
  out to be stronger than "a gate". Checking the spec's 169 operations showed
  that **eToro separates the two accounts by URL, not only by key**:
  `POST /api/v2/trading/execution/orders` spends real money and
  `POST /api/v2/trading/execution/demo/orders` does not. So `Environment`
  selects the path, and a client built for `Demo` has no reachable
  real-money endpoint — a property of the type rather than a rule someone has
  to remember at each call site.

  The `demo` segment is **not** inserted at a consistent position
  (`/trading/info/demo/portfolio`, `/trading/execution/demo/orders`,
  `/trading/demo/positions/{id}`), and `/trading/info/real/pnl` names both
  environments explicitly. Any derivation rule would need exceptions and the
  cost of one mistake is a real order, so `EtoroClient::path` takes **both**
  spellings written out at the call site, where a reviewer can see the complete
  set of URLs a method can reach without leaving the line.

  `portfolio()` was reading the real account unconditionally before this.

  `ETORO_ENVIRONMENT` is required with no default: the two candidate defaults
  are "silently does nothing useful" and "silently points at real money".
- ~~`me()` returns the token's scopes, so the declared environment can be
  cross-checked against the key at startup rather than trusted.~~ **Done**:
  `verify_environment()` parses the scopes (`etoro-public:<resource>:<action>`,
  where the resource is `real`, `demo`, `trade.real`, `trade.demo`, or
  environment-neutral) and fails with `EnvironmentMismatch` naming both what
  was declared and what the token carries. Neutral scopes answer "no
  environment" rather than being guessed at, so a key with read-only feed
  access cannot pass the check by accident.
- ~~Order submission plus the asynchronous status state machine above, using
  `referenceId` as the recovery handle when a response is lost.~~
  **Done 2026-08-30** for opening: `src/orders.rs` and
  `EtoroClient::{place_order, lookup_order}`.

  Three things this settled. **`x-request-id` is a caller argument on writes**,
  not generated inside the client — a retry must reuse its original id, and
  that is impossible to express if the client mints its own. **Both identifiers
  on an accepted order are optional** in the schema, so `AcceptedOrder` carries
  the reference *we* chose and is guaranteed to have a handle even when the
  response body is empty; the contract tests cover the empty-`{}` case
  explicitly. And **`OrderStatus` is deliberately open** where every other enum
  in the crate is closed: by the time a status is being read the write has
  happened, so refusing to decode an unrecognised id would make a live order
  invisible.

  `MarketBuy` is far narrower than the API — one order shape, market, long,
  sized in cash — because the unified schema has fifteen optional fields under
  a dozen mutual-exclusion rules and the generated type makes every invalid
  combination expressible.

  ~~Still to do: closing.~~ **Done**: `close_position` and `ClosePosition`.
  It is a different *endpoint*, not a different argument — the order endpoint
  rejects `sell` and `buyToCover` today — so opening and closing genuinely do
  not share a path, and a long-only strategy drives two. The body is
  PascalCase (`InstrumentID`, `UnitsToDeduct`) unlike every other request in
  the API, so it is built by a private type rather than at call sites, and an
  absent `UnitsToDeduct` means "all of it" where zero would mean "none".
  A close is asynchronous too: eToro's own documented example returns
  `statusID: 1` (Received), so it polls exactly like an open.
- The decision layer is `src/trader.rs`, and it is **pure**: `holding_for`,
  `decide` and `plan` take the portfolio as data and return what should happen.
  Only `await_terminal` talks to the API, and it only reads. The split is not
  tidiness — the interesting failures here are decision failures, and a
  decision that can only be exercised by placing a real order never gets
  tested.

  Three refusals encoded there, each one a case where guessing is expensive:
  a **fractional target** is rejected rather than rounded, because a live
  trader that turned 0.4 into "all in" would be running a different strategy
  from the backtested one while every report still described the backtested
  one; **more than one position** in an instrument stops the decision, because
  "go flat" is then ambiguous and closing one of two leaves the account half in
  a position the strategy believes it exited; and a **short or leveraged
  position** stops it too, because something else placed that bet and closing
  it might be exactly wrong. An absent `isBuy` counts as unknown, not as long.

- `etoro-agent plan SYM` reads the live portfolio and prints what the strategy
  would do. It places nothing and needs no write scope, and it says so in its
  own output every time. Verified against the demo account.
- No CLI command places orders, deliberately: approval mode and the hard
  limits are milestone 6, and a way to fire an order from a shell prompt should
  not exist before them.
- ~~Order status polling~~ **done**: `await_terminal` returns the last status
  seen, so an order still in flight at the deadline is reported as such rather
  than mistaken for a failure. An order that has not finished is not an order
  that did not happen.
- ~~The retry policy lands here, now that the idempotency rule is known.~~
  **Done**: `src/retry.rs`. `with_retry` generates **one** request id and hands
  the same one to every attempt, so correct reuse is the path of least
  resistance instead of something to remember — the consequence of forgetting
  being a duplicate position rather than an error. `delay_before` is pure, so
  the whole decision is tested without waiting for it. A `Retry-After` from the
  API overrides the computed backoff in both directions, but is still capped,
  because an hour-long header would otherwise park an unattended agent. There
  is no `Default`: `RetryPolicy::once()` and `::standard()` are both named.

Then run it on demo, unattended, for a meaningful period — and compare actual
demo fills against what the backtest predicted for the same period. That
comparison is the real validation, and it catches the cost and slippage errors
a backtest structurally cannot.

**Done when** demo has run unattended long enough to produce a fill-vs-backtest
comparison.

### 6. Live, small

Only after milestone 5 has produced that comparison. Prerequisites:

- ~~Hard limits: maximum position size, maximum orders per day, maximum total
  exposure, and a kill switch.~~ **Done 2026-08-30**: `src/limits.rs` and
  `src/audit.rs`, both checked by `plan` before it reports a verdict.

  **Only opening is constrained.** Closing is always permitted — kill switch
  aside — because every limit exists to bound risk and refusing to close
  increases it. A daily order cap that stopped an exit would be a limit that
  traps you in a position.

  The **kill switch is a file**, not a flag or a variable, because those all
  require reaching the program: a file can be created from any shell, over ssh,
  while a run is in progress, by somebody who has never read this code. It
  blocks closing too — it means "this program is not to act", and closing by
  hand is always available. It fails open (a wiped disk resumes trading), which
  is the deliberate trade against a present-to-trade token that fails safe but
  gets forgotten.

  The **audit log is also the state**. The daily cap counts what the log says
  was submitted, so it survives a restart, a crash, or two copies running at
  once; an in-memory counter would reset at the moment it mattered most. It
  counts *submissions*, not settlements — an order whose outcome was never
  recorded still consumed a slot and may well have filled. And an unparseable
  line is an error rather than a skipped line, because quietly counting fewer
  orders than were placed is the one failure a cap must not have.

  Refusals are logged as loudly as submissions: "did nothing today" and "was
  stopped twelve times" look identical in a portfolio and are not the same
  situation.
- ~~A cost check via `POST /api/v2/trading/info/costs` before every order, with
  a refusal path when cost is large relative to expected edge.~~
  **Done 2026-08-30**: `src/costs.rs`, `EtoroClient::order_cost`, and
  `Limits::check_cost`.

  The endpoint takes **the same body as the order endpoint**, so what is priced
  is the exact order about to be sent rather than an approximation. It is a
  POST that is a *query*, so it mints its own request id and stays outside the
  idempotency discipline, and it has its own 20/60s quota — pricing an order
  does not spend the budget needed to place it.

  **The breakdown splits one-off from carried cost**, and that distinction
  matters beyond this check: `markup`, `marketSpread`, `transactionFee` and
  `sdrt` are paid once and are what the backtest's `CostModel` already models;
  `overnightFee` and `overWeekendFee` are charged for every day a position
  stays open and **the backtest does not model them at all**. `plan` prints a
  warning when `per_day` is non-zero, because a strategy holding such a
  position is being measured optimistically by an amount that grows with the
  holding period. This is also the exact gap that has to be closed before
  shorting can be backtested honestly.

  The gate is a **fraction** of order value, not an amount: the same $0.50 of
  spread is half a percent on $100 and negligible on $2500. Only up-front cost
  is gated — carry is not comparable to a one-off without a holding period, so
  it is surfaced to a human rather than silently accepted or refused. An
  unrecognised cost component is refused outright, since it may be a daily fee
  being counted as a one-off.

  Live on demo, a $2500 AAPL buy quotes four components, all zero. ⚠️ Demo may
  not quote realistic costs; re-check on real before relying on the figures.
- ~~Approval mode first — print the intended order and require confirmation —
  before anything runs unattended.~~ **Done 2026-08-30**: `etoro-agent trade`.

  `plan` and `trade` share one `prepare` path, so the two cannot drift. A
  `trade` that checked slightly different things from the `plan` it printed
  would make the printed plan a lie, which is the one thing an approval prompt
  must never be.

  Order of operations, which is the safety property: decide → print → check
  every rail → ask a human → **log the intent** → send → poll → log the
  outcome. Anything sent before a refusal has been checked, or before the
  intent is written down, can happen without a record of why.

  The confirmation phrase is more than a keystroke — `yes` on demo, and
  `yes, real money` on real. A prompt answered by reflex is not approval.
  `--unattended` skips it and is **refused outright on the real account**: the
  roadmap says approval mode comes first, and "first" has to mean something a
  flag cannot skip past.

  `settle` takes the submission `Result` rather than a success, because a
  *failed* submission is exactly the case that must still be looked up and
  recorded — the request may have arrived and executed with its response lost
  coming back. The submission error surfaces only after the outcome has been
  written down.

  Retries are `RetryPolicy::once()`, not `standard()`. Retrying a write is safe
  only if eToro's idempotency behaves as documented, and that has not been
  observed here yet; until demo shows a reused id producing one order rather
  than two, the honest policy is not to retry.

  Found while testing: the flag guards matched `--`, so a mistyped short flag
  like `-y` was silently swallowed as a positional and reinterpreted as a
  *source name*. All four parsers now reject anything beginning with `-`.
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
