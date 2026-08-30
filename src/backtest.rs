//! An offline backtest over stored bars.
//!
//! Three things make a backtest lie: lookahead bias, unadjusted prices, and
//! absent costs. The data layer already answers the second — [`SeriesChoice`]
//! is a required argument everywhere, so a series is never read on the wrong
//! basis by accident. This module is arranged so the other two are hard to get
//! wrong rather than merely warned about.
//!
//! Lookahead is prevented by what a [`Strategy`] is handed: a *prefix* of the
//! series, ending on the bar the decision is made from. There is no later bar
//! to read, so the usual mistake — computing a signal from a column and
//! forgetting to shift it — has nothing to act on.
//!
//! Costs are prevented from vanishing by [`run`]'s signature: [`CostModel`] is
//! a positional argument with no `Default`, so a frictionless backtest has to
//! be spelled [`CostModel::frictionless`] on purpose.
//!
//! Nothing here touches the network. It reads [`Bar`]s that are already
//! stored, which is what makes a run repeatable.

use chrono::NaiveDate;

use crate::analysis::SideStats;
use crate::data::{Bar, SeriesChoice};

/// A rule for how much of the instrument to hold.
///
/// Deliberately expressed as a *target weight* rather than as buy and sell
/// orders. Sizing then lives in the engine, which is also where turnover — and
/// therefore cost — is measured, so a strategy cannot accidentally trade more
/// than it says it does.
pub trait Strategy {
    /// A label for reports.
    fn name(&self) -> String;

    /// The fraction of equity to hold, in `[0, 1]`, decided from the last bar
    /// of `history` and filled on a later one.
    ///
    /// `history` is a prefix ending on the decision bar: the future does not
    /// exist yet from in here. That is the structural defence against
    /// lookahead bias, and it is why this takes a slice rather than the whole
    /// series plus an index — an index can be read past.
    ///
    /// `series` is handed over rather than chosen here, and for the same kind
    /// of reason: a strategy that picked its own could signal on total-return
    /// prices while the engine filled on as-traded ones, and the two series
    /// look identical from the outside. Passing it makes them impossible to
    /// disagree about. Read prices with [`Bar::ohlc`], never the raw fields.
    ///
    /// `&mut self` so a strategy may carry state between bars (a running mean,
    /// a cooldown counter) instead of recomputing from the prefix each time.
    ///
    /// Returning anything outside `[0, 1]`, including `NaN`, aborts the run.
    /// Clamping would turn a strategy bug into a plausible equity curve.
    fn target(&mut self, history: &[Bar], series: SeriesChoice) -> f64;
}

/// Always fully invested. The benchmark, expressed as a strategy.
///
/// Useful beyond documentation: run through the engine with no costs it must
/// reproduce the benchmark series exactly, which is the cheapest possible
/// check that the accounting is not drifting.
#[derive(Debug, Clone, Copy, Default)]
pub struct BuyAndHold;

impl Strategy for BuyAndHold {
    fn name(&self) -> String {
        "buy & hold".to_owned()
    }

    fn target(&mut self, _history: &[Bar], _series: SeriesChoice) -> f64 {
        1.0
    }
}

/// What trading costs, as fractions of the notional traded.
///
/// Both are charged on **turnover**, not per trade: moving from 20% to 30% of
/// equity costs a tenth of what opening a full position does, because that is
/// what actually changes hands. Charging per trade flatters strategies that
/// trim and punishes ones that scale in.
///
/// There is no `Default` on purpose — see [`frictionless`].
///
/// [`frictionless`]: Self::frictionless
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostModel {
    /// Half the quoted spread, as a fraction of price. Paid on entry and again
    /// on exit, which is what makes it half rather than whole: each leg
    /// crosses from mid to one side.
    pub half_spread: f64,
    /// Commission as a fraction of notional. Zero for eToro real stocks.
    pub commission: f64,
    /// Financing charged for each **calendar day** a position is held, as a
    /// fraction of its value.
    ///
    /// Zero for an ordinary unleveraged long, which is why the engine got this
    /// far without it. Non-zero for anything eToro quotes an `overnightFee` or
    /// `overWeekendFee` on -- CFDs, leverage, and shorts. Without this term a
    /// strategy holding such a position is measured optimistically by an amount
    /// that grows with the holding period, so the error is smallest exactly
    /// where it is easiest to notice and largest where it is not.
    ///
    /// Charged on calendar days rather than bars, so a Friday-to-Monday hold
    /// costs three days. The weekend fee is then not a special case; it is
    /// simply the weekend.
    pub carry_per_day: f64,
}

impl CostModel {
    /// Costs implied by a quoted spread, as a fraction of mid.
    ///
    /// Measurements from live `rates` on 2026-08-30 span roughly 50×, from
    /// 0.003% on AAPL to 0.163% on a watchlist mid-cap, so this belongs
    /// per-instrument. One constant across a basket is simultaneously far too
    /// pessimistic at the liquid end and dangerously optimistic at the thin
    /// end — and the thin end is where a marginal strategy will look best.
    pub fn from_spread(spread: f64) -> Self {
        Self {
            half_spread: spread / 2.0,
            commission: 0.0,
            carry_per_day: 0.0,
        }
    }

    /// The same costs with a daily financing rate attached.
    ///
    /// The figure comes from `overnightFee` in a
    /// [`CostEstimate`](crate::costs::CostEstimate), divided by the position
    /// value it was quoted against.
    pub fn with_carry(mut self, per_day: f64) -> Self {
        self.carry_per_day = per_day;
        self
    }

    /// No trading costs at all.
    ///
    /// Named rather than defaulted, and named unpleasantly, because the
    /// commonest way to get a wonderful backtest is to forget that trading is
    /// not free. Reaching for this should feel like a decision.
    pub fn frictionless() -> Self {
        Self {
            half_spread: 0.0,
            commission: 0.0,
            carry_per_day: 0.0,
        }
    }

    /// Fraction of equity consumed by a weight change of `turnover`.
    fn charge(&self, turnover: f64) -> f64 {
        turnover * (self.half_spread + self.commission)
    }
}

/// Where a decision gets filled.
///
/// This is the single most consequential choice in the harness, and the
/// [`gaps`](crate::analysis::gaps) command exists partly to quantify what it
/// costs: for several names the overnight leg carries most of the return.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillPrice {
    /// Decide on a close, fill at the next session's open.
    ///
    /// The honest default, and structurally pessimistic: the gap between the
    /// close you decided on and the open you filled at is unavailable to you,
    /// because there was no moment in between at which you could trade.
    NextOpen,
    /// Decide on a close and fill at that same close.
    ///
    /// The convention behind the usual `signal.shift(1)` harness. It assumes
    /// you can transact at a print you have watched approach, which hands the
    /// strategy every overnight gap that follows a signal. Kept as the
    /// optimistic bound: the spread between the two fill models is the honest
    /// error bar on any result.
    ///
    /// Note there is deliberately no "next close" — filling a whole session
    /// later than [`NextOpen`] is strictly more delay, so it bounds nothing.
    ///
    /// [`NextOpen`]: Self::NextOpen
    SameClose,
}

impl FillPrice {
    /// Bars between the bar a decision is made on and the bar it fills on.
    fn lag(self) -> usize {
        match self {
            Self::NextOpen => 1,
            Self::SameClose => 0,
        }
    }

    fn price(self, bar: &Bar, series: SeriesChoice) -> f64 {
        let [open, _, _, close] = bar.ohlc(series);
        match self {
            Self::NextOpen => open,
            Self::SameClose => close,
        }
    }

    fn field(self) -> &'static str {
        match self {
            Self::NextOpen => "open",
            Self::SameClose => "close",
        }
    }
}

/// One change of position, at the price and on the date it filled.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Trade {
    /// The bar the fill happened on, which is later than the bar the decision
    /// was made on unless [`FillPrice::SameClose`] is in use.
    pub date: NaiveDate,
    pub price: f64,
    pub from: f64,
    pub to: f64,
    /// Charged for this fill, as a fraction of equity.
    pub cost: f64,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum BacktestError {
    #[error("a {fill:?} backtest needs at least {needed} bars, and got {got}")]
    TooShort {
        fill: FillPrice,
        needed: usize,
        got: usize,
    },

    /// A zero or negative price is not a market event; it is a broken series.
    /// Dividing by one produces an infinite return that poisons every
    /// statistic downstream, so the run stops here instead.
    #[error("bar {date} has a {field} price of {price}, which cannot be traded at")]
    BadPrice {
        date: NaiveDate,
        field: &'static str,
        price: f64,
    },

    #[error(
        "strategy asked for a target of {target} on {date}; targets are fractions of equity in [0, 1]"
    )]
    BadTarget { date: NaiveDate, target: f64 },
}

/// The outcome of a run: what it earned, what it paid, and every fill.
#[derive(Debug, Clone, PartialEq)]
pub struct Backtest {
    pub strategy_name: String,
    pub fill: FillPrice,
    pub cost_model: CostModel,
    /// Holding periods measured, one fewer than the bars used.
    pub segments: usize,
    /// First and last fill dates, which bound the window actually traded —
    /// shorter than the series by [`FillPrice::lag`] bars at the front.
    pub first: NaiveDate,
    pub last: NaiveDate,
    /// Net of costs.
    pub strategy: SideStats,
    /// Gross buy-and-hold over exactly the same segments.
    ///
    /// Uncosted, which flatters it by the one spread a real holder would pay
    /// on entry. That bias runs against the strategy, which is the direction
    /// to be wrong in when the question is "did this beat doing nothing".
    pub benchmark: SideStats,
    pub max_drawdown: f64,
    pub benchmark_max_drawdown: f64,
    /// Mean weight held, i.e. the fraction of the window spent in the market.
    pub exposure: f64,
    /// Summed absolute weight changes. A full round trip is 2.0.
    pub turnover: f64,
    /// Turnover cost charged, as a fraction of starting equity.
    pub costs_paid: f64,
    /// Financing charged for holding, as a fraction of starting equity.
    ///
    /// Reported apart from [`Self::costs_paid`] because the two respond to
    /// different things: turnover cost falls if you trade less, carry falls
    /// only if you hold for less time, and a strategy can easily improve one
    /// while making the other worse.
    pub carry_paid: f64,
    /// Cumulative equity from 1.0, stamped with the date of each fill.
    pub equity: Vec<(NaiveDate, f64)>,
    pub trades: Vec<Trade>,
}

/// Runs `strategy` over `bars` and reports what it would have done.
///
/// Every argument is required. `series` because a price series does not say
/// what its prices mean, and `costs` because the alternative is a backtest
/// that quietly assumed free trading.
pub fn run(
    bars: &[Bar],
    series: SeriesChoice,
    strategy: &mut dyn Strategy,
    costs: &CostModel,
    fill: FillPrice,
) -> Result<Backtest, BacktestError> {
    let lag = fill.lag();
    // One bar to decide from, plus two to measure a holding period between.
    let needed = lag + 2;
    if bars.len() < needed {
        return Err(BacktestError::TooShort {
            fill,
            needed,
            got: bars.len(),
        });
    }

    // Validated up front rather than inside the loop: a broken price near the
    // end would otherwise be reported only after a strategy had been run over
    // the whole series, and after a partial result had been built from it.
    let mut prices = Vec::with_capacity(bars.len());
    for bar in bars {
        let price = fill.price(bar, series);
        if price.is_nan() || price <= 0.0 {
            return Err(BacktestError::BadPrice {
                date: bar.date,
                field: fill.field(),
                price,
            });
        }
        prices.push(price);
    }

    let mut weight = 0.0;
    let mut equity = 1.0;
    let (mut turnover, mut costs_paid, mut exposure) = (0.0, 0.0, 0.0);
    let mut carry_paid = 0.0;
    let mut strategy_returns = Vec::with_capacity(bars.len() - needed + 1);
    let mut benchmark_returns = Vec::with_capacity(strategy_returns.capacity());
    let mut trades = Vec::new();
    let mut curve = vec![(bars[lag].date, equity)];

    for i in lag..bars.len() - 1 {
        // The bar this decision may see up to. Everything after it is, at this
        // point in the simulation, the future.
        let decision = i - lag;
        let target = strategy.target(&bars[..=decision], series);
        if !(0.0..=1.0).contains(&target) {
            // Also catches NaN, since every comparison against it is false.
            return Err(BacktestError::BadTarget {
                date: bars[decision].date,
                target,
            });
        }

        let traded = (target - weight).abs();
        let cost = costs.charge(traded);
        if traded > 0.0 {
            trades.push(Trade {
                date: bars[i].date,
                price: prices[i],
                from: weight,
                to: target,
                cost,
            });
        }
        turnover += traded;
        costs_paid += cost;
        weight = target;
        // `abs`, because a short is exposure too. A no-op while targets are
        // confined to [0, 1], and load-bearing the moment they are not.
        exposure += weight.abs();

        // Fill to fill, not close to close: the holding period is bounded by
        // the prices actually transacted at, so the return earned and the
        // costs charged refer to the same instants.
        let asset = prices[i + 1] / prices[i] - 1.0;
        // Calendar days, not bars: a position held over a weekend is financed
        // for three days, and a market holiday costs the same as any other day
        // the position is open.
        let days = (bars[i + 1].date - bars[i].date).num_days().max(0) as f64;
        let held = weight.abs() * costs.carry_per_day * days;
        carry_paid += held;
        let net = weight * asset - cost - held;

        equity *= 1.0 + net;
        curve.push((bars[i + 1].date, equity));
        strategy_returns.push(net);
        benchmark_returns.push(asset);
    }

    let segments = strategy_returns.len();
    Ok(Backtest {
        strategy_name: strategy.name(),
        fill,
        cost_model: *costs,
        segments,
        first: bars[lag].date,
        last: bars[bars.len() - 1].date,
        strategy: SideStats::from_returns(&strategy_returns),
        benchmark: SideStats::from_returns(&benchmark_returns),
        max_drawdown: max_drawdown(&strategy_returns),
        benchmark_max_drawdown: max_drawdown(&benchmark_returns),
        exposure: exposure / segments as f64,
        turnover,
        costs_paid,
        carry_paid,
        equity: curve,
        trades,
    })
}

/// Worst peak-to-trough fall, as a negative fraction.
///
/// Measured on the equity curve rather than on prices, so it reflects what the
/// strategy actually lived through: a rule that sits flat during a crash has
/// no drawdown from it, whatever the instrument did.
fn max_drawdown(returns: &[f64]) -> f64 {
    let (mut equity, mut peak, mut worst) = (1.0_f64, 1.0_f64, 0.0_f64);
    for r in returns {
        equity *= 1.0 + r;
        peak = peak.max(equity);
        worst = worst.min(equity / peak - 1.0);
    }
    worst
}

impl Backtest {
    /// Whether the strategy earned more per unit of volatility than holding.
    ///
    /// Return alone is the wrong test: a rule that is in the market a third of
    /// the time and earns half as much has done something, and one that
    /// doubles the return by doubling the risk has not.
    pub fn beats_benchmark(&self) -> bool {
        self.strategy.return_over_vol() > self.benchmark.return_over_vol()
    }

    /// A fixed-width report, strategy against benchmark.
    pub fn report(&self) -> String {
        let pct = |v: f64| format!("{:+.2}%", v * 100.0);
        let vol = |v: f64| format!("{:.1}%", v * 100.0);
        let ratio = |v: f64| {
            if v.is_finite() {
                format!("{v:.2}")
            } else {
                "-".to_owned()
            }
        };
        let row = |name: &str, side: &SideStats, drawdown: f64| {
            format!(
                "{:<14}{:>10}{:>8}{:>9}{:>10}{:>13}\n",
                name,
                pct(side.annualised),
                vol(side.volatility),
                ratio(side.return_over_vol()),
                pct(drawdown),
                pct(side.compounded),
            )
        };

        let mut out = format!(
            "{:<14}{:>10}{:>8}{:>9}{:>10}{:>13}\n",
            "", "annual", "vol", "ret/vol", "maxDD", "compounded"
        );
        out.push_str(&row(&self.strategy_name, &self.strategy, self.max_drawdown));
        out.push_str(&row(
            "buy & hold",
            &self.benchmark,
            self.benchmark_max_drawdown,
        ));

        out.push_str(&format!(
            "\n{} sessions, {:.1} years, {} to {}\n",
            self.segments,
            self.segments as f64 / crate::analysis::TRADING_YEAR,
            self.first,
            self.last,
        ));
        out.push_str(&format!(
            "{} fill{}, {:.1} round trips, {:.0}% of the time in the market\n",
            self.trades.len(),
            if self.trades.len() == 1 { "" } else { "s" },
            self.turnover / 2.0,
            self.exposure * 100.0,
        ));
        if self.cost_model.carry_per_day != 0.0 {
            out.push_str(&format!(
                "{:.2}% paid in financing at {:.4}% per day held\n",
                self.carry_paid * 100.0,
                self.cost_model.carry_per_day * 100.0,
            ));
        }
        out.push_str(&format!(
            "{:.2}% paid in costs at {:.3}% half-spread + {:.3}% commission, filled {}\n",
            self.costs_paid * 100.0,
            self.cost_model.half_spread * 100.0,
            self.cost_model.commission * 100.0,
            match self.fill {
                FillPrice::NextOpen => "at the next open",
                FillPrice::SameClose => "at the deciding close",
            },
        ));

        // Stated as a verdict because the alternative is reading two rows of
        // numbers and believing the one you hoped for.
        out.push_str(&format!(
            "-> {} {} on return per unit of volatility\n",
            self.strategy_name,
            if self.beats_benchmark() {
                "beats buy & hold"
            } else {
                "LOSES TO buy & hold"
            },
        ));
        if self.cost_model == CostModel::frictionless() {
            out.push_str("! frictionless: no spread and no commission were charged\n");
        }
        if self.fill == FillPrice::SameClose {
            out.push_str(
                "! filled at the deciding close, so every overnight gap after a signal\n\
                   was captured; re-run with the next open for the pessimistic bound\n",
            );
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::manual::Numeric;

    /// Open and close are all this module reads; high and low are filled in
    /// consistently so the bars stay valid rather than to be used.
    fn bar(date: &str, open: &str, close: &str) -> Bar {
        let n = |v: &str| Numeric(v.parse().unwrap());
        let (o, c) = (open.parse::<f64>().unwrap(), close.parse::<f64>().unwrap());
        Bar {
            date: date.parse().unwrap(),
            open: n(open),
            high: n(&format!("{}", o.max(c))),
            low: n(&format!("{}", o.min(c))),
            close: n(close),
            volume: None,
            total_return_close: None,
            dividend_cash: None,
            split_factor: None,
            split_adjusted_volume: None,
        }
    }

    /// Plays back a fixed list of targets, one per decision.
    struct Fixed {
        targets: Vec<f64>,
        next: usize,
    }

    impl Fixed {
        fn new(targets: &[f64]) -> Self {
            Self {
                targets: targets.to_vec(),
                next: 0,
            }
        }
    }

    impl Strategy for Fixed {
        fn name(&self) -> String {
            "fixed".to_owned()
        }

        fn target(&mut self, _history: &[Bar], _series: SeriesChoice) -> f64 {
            let target = self.targets[self.next];
            self.next += 1;
            target
        }
    }

    /// Records the last bar of every prefix it is handed.
    #[derive(Default)]
    struct Recorder {
        seen: Vec<(usize, NaiveDate)>,
    }

    impl Strategy for Recorder {
        fn name(&self) -> String {
            "recorder".to_owned()
        }

        fn target(&mut self, history: &[Bar], _series: SeriesChoice) -> f64 {
            self.seen
                .push((history.len(), history.last().unwrap().date));
            0.0
        }
    }

    fn close(result: f64, expected: f64) -> bool {
        (result - expected).abs() < 1e-12
    }

    #[test]
    fn a_known_answer_case_computes_by_hand() {
        // Fills at the open: 100, then 110, then 132.
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "110", "120"),
            bar("2026-08-05", "132", "130"),
        ];
        // 1% half-spread, so opening a full position costs 1% of equity.
        let costs = CostModel {
            half_spread: 0.01,
            commission: 0.0,
            carry_per_day: 0.0,
        };
        let result = run(
            &bars,
            SeriesChoice::Reported,
            &mut BuyAndHold,
            &costs,
            FillPrice::NextOpen,
        )
        .unwrap();

        // One holding period: filled at 110 on the 4th, marked at 132 on the
        // 5th. The 3rd's open is only the bar the decision was made from.
        assert_eq!(result.segments, 1);
        assert!(close(result.benchmark.compounded, 0.2), "132/110 - 1");
        // 20% earned, 1% paid to get in.
        assert!(close(result.strategy.compounded, 0.19));
        assert!(close(result.costs_paid, 0.01));
        assert!(close(result.turnover, 1.0));
        assert_eq!(
            result.equity,
            vec![
                ("2026-08-04".parse().unwrap(), 1.0),
                ("2026-08-05".parse().unwrap(), 1.19),
            ]
        );

        let trade = result.trades[0];
        assert_eq!(result.trades.len(), 1);
        assert_eq!(trade.date, "2026-08-04".parse().unwrap());
        assert!(close(trade.price, 110.0) && close(trade.from, 0.0) && close(trade.to, 1.0));
    }

    #[test]
    fn always_long_reproduces_the_benchmark_when_trading_is_free() {
        let bars = [
            bar("2026-08-03", "100", "101"),
            bar("2026-08-04", "104", "103"),
            bar("2026-08-05", "99", "108"),
            bar("2026-08-06", "111", "110"),
        ];
        let result = run(
            &bars,
            SeriesChoice::Reported,
            &mut BuyAndHold,
            &CostModel::frictionless(),
            FillPrice::NextOpen,
        )
        .unwrap();

        // The strongest cheap check on the accounting: with nothing deducted
        // and nothing held back, the two must agree to the last bit.
        assert!(close(
            result.strategy.compounded,
            result.benchmark.compounded
        ));
        assert!(close(result.max_drawdown, result.benchmark_max_drawdown));
        assert!(close(result.exposure, 1.0));
    }

    #[test]
    fn a_strategy_that_steps_aside_avoids_the_fall() {
        // Opens 100, 110, 121, 100: up 10%, then down 17.4%.
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "110", "110"),
            bar("2026-08-05", "121", "121"),
            bar("2026-08-06", "100", "100"),
        ];
        let result = run(
            &bars,
            SeriesChoice::Reported,
            &mut Fixed::new(&[1.0, 0.0]),
            &CostModel::frictionless(),
            FillPrice::NextOpen,
        )
        .unwrap();

        assert!(close(result.strategy.compounded, 0.1), "held the rise only");
        assert!(close(result.benchmark.compounded, 100.0 / 110.0 - 1.0));
        // Flat through the fall, so it never drew down at all.
        assert!(close(result.max_drawdown, 0.0));
        assert!(result.benchmark_max_drawdown < -0.17);
        assert!(close(result.exposure, 0.5));
    }

    #[test]
    fn costs_are_charged_on_turnover_rather_than_per_trade() {
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "100", "100"),
            bar("2026-08-05", "100", "100"),
            bar("2026-08-06", "100", "100"),
        ];
        // Half in, held, then out: two fills, but only one round trip of
        // notional. Charging per trade would bill this the same as 0 -> 1 -> 0.
        let result = run(
            &bars,
            SeriesChoice::Reported,
            &mut Fixed::new(&[0.5, 0.5, 0.0]),
            &CostModel {
                half_spread: 0.01,
                commission: 0.0,
                carry_per_day: 0.0,
            },
            FillPrice::SameClose,
        )
        .unwrap();

        assert_eq!(result.trades.len(), 2, "the unchanged target is not a fill");
        assert!(close(result.turnover, 1.0));
        assert!(close(result.costs_paid, 0.01));
    }

    #[test]
    fn a_strategy_only_ever_sees_bars_up_to_its_decision() {
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "101", "101"),
            bar("2026-08-05", "102", "102"),
            bar("2026-08-06", "103", "103"),
        ];
        let mut recorder = Recorder::default();
        run(
            &bars,
            SeriesChoice::Reported,
            &mut recorder,
            &CostModel::frictionless(),
            FillPrice::NextOpen,
        )
        .unwrap();

        // Filling at the next open means the last bar is never decided from:
        // there is no session after it to fill in.
        assert_eq!(
            recorder.seen,
            vec![
                (1, "2026-08-03".parse().unwrap()),
                (2, "2026-08-04".parse().unwrap()),
            ]
        );
    }

    #[test]
    fn the_two_fill_models_differ_by_exactly_the_overnight_gap() {
        // Flat sessions, all the movement in the gaps, so the difference
        // between the models is isolated.
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "120", "120"),
            bar("2026-08-05", "150", "150"),
        ];
        let costs = CostModel::frictionless();
        let at_open = run(
            &bars,
            SeriesChoice::Reported,
            &mut BuyAndHold,
            &costs,
            FillPrice::NextOpen,
        )
        .unwrap();
        let at_close = run(
            &bars,
            SeriesChoice::Reported,
            &mut BuyAndHold,
            &costs,
            FillPrice::SameClose,
        )
        .unwrap();

        // Deciding on the 3rd's close, the open filler buys at 120 and holds
        // to 150. The close filler buys at 100 on the 3rd itself and holds to
        // 120, then decides again -- two periods, and it captures the first
        // gap the other one paid for.
        assert!(close(at_open.strategy.compounded, 0.25));
        assert_eq!(at_open.segments, 1);
        assert!(close(at_close.strategy.compounded, 0.5));
        assert_eq!(at_close.segments, 2);
    }

    #[test]
    fn a_series_too_short_to_hold_anything_is_refused() {
        let bars = [bar("2026-08-03", "100", "100"), bar("2026-08-04", "1", "1")];
        let error = run(
            &bars,
            SeriesChoice::Reported,
            &mut BuyAndHold,
            &CostModel::frictionless(),
            FillPrice::NextOpen,
        )
        .unwrap_err();
        assert_eq!(
            error,
            BacktestError::TooShort {
                fill: FillPrice::NextOpen,
                needed: 3,
                got: 2
            }
        );

        // The same two bars are enough when the fill is on the deciding close.
        assert!(
            run(
                &bars,
                SeriesChoice::Reported,
                &mut BuyAndHold,
                &CostModel::frictionless(),
                FillPrice::SameClose,
            )
            .is_ok()
        );
    }

    #[test]
    fn an_untradeable_price_stops_the_run() {
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "0", "110"),
            bar("2026-08-05", "120", "120"),
        ];
        let error = run(
            &bars,
            SeriesChoice::Reported,
            &mut BuyAndHold,
            &CostModel::frictionless(),
            FillPrice::NextOpen,
        )
        .unwrap_err();
        assert_eq!(
            error,
            BacktestError::BadPrice {
                date: "2026-08-04".parse().unwrap(),
                field: "open",
                price: 0.0
            }
        );
    }

    #[test]
    fn a_target_outside_the_allowed_range_stops_the_run() {
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "110", "110"),
            bar("2026-08-05", "120", "120"),
        ];
        for target in [2.0, -0.5, f64::NAN] {
            let error = run(
                &bars,
                SeriesChoice::Reported,
                &mut Fixed::new(&[target]),
                &CostModel::frictionless(),
                FillPrice::NextOpen,
            )
            .unwrap_err();
            assert!(
                matches!(error, BacktestError::BadTarget { date, .. }
                    if date == "2026-08-03".parse().unwrap()),
                "{error} for {target}"
            );
        }
    }

    #[test]
    fn financing_is_charged_for_every_calendar_day_a_position_is_held() {
        // Friday to Monday: three days of financing, not one bar's worth.
        // The weekend fee is not a special case, it is simply the weekend.
        let bars = [
            bar("2026-08-07", "100", "100"), // Friday
            bar("2026-08-10", "100", "100"), // Monday
            bar("2026-08-11", "100", "100"), // Tuesday
        ];
        let costs = CostModel::frictionless().with_carry(0.01);
        let result = run(
            &bars,
            SeriesChoice::Reported,
            &mut BuyAndHold,
            &costs,
            FillPrice::SameClose,
        )
        .unwrap();

        // Segment one spans three calendar days, segment two spans one.
        assert!(close(result.carry_paid, 0.04), "{}", result.carry_paid);
        // Prices never moved, so financing is the entire loss.
        assert!(close(
            result.strategy.compounded,
            (1.0 - 0.03) * (1.0 - 0.01) - 1.0
        ));
    }

    #[test]
    fn a_strategy_that_is_flat_pays_no_financing() {
        // Carry is charged on what is held, so sitting in cash is free -- the
        // property that makes carry different from a cost on turnover.
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "100", "100"),
            bar("2026-08-05", "100", "100"),
        ];
        let result = run(
            &bars,
            SeriesChoice::Reported,
            &mut Fixed::new(&[0.0, 0.0, 0.0]),
            &CostModel::frictionless().with_carry(0.05),
            FillPrice::SameClose,
        )
        .unwrap();
        assert!(close(result.carry_paid, 0.0));
    }

    #[test]
    fn financing_and_turnover_costs_are_reported_apart() {
        // They respond to different things: trading less cuts one, holding for
        // less time cuts the other, and a strategy can improve one while
        // making the other worse.
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "100", "100"),
            bar("2026-08-05", "100", "100"),
        ];
        let result = run(
            &bars,
            SeriesChoice::Reported,
            &mut BuyAndHold,
            &CostModel::from_spread(0.02).with_carry(0.01),
            FillPrice::SameClose,
        )
        .unwrap();

        assert!(
            close(result.costs_paid, 0.01),
            "one entry at a 1% half-spread"
        );
        assert!(close(result.carry_paid, 0.02), "two days held");
        let report = result.report();
        assert!(report.contains("paid in financing"), "{report}");
        assert!(report.contains("paid in costs"), "{report}");
    }

    #[test]
    fn a_zero_carry_rate_changes_nothing_and_is_not_mentioned() {
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "110", "110"),
            bar("2026-08-05", "120", "120"),
        ];
        let result = run(
            &bars,
            SeriesChoice::Reported,
            &mut BuyAndHold,
            &CostModel::frictionless(),
            FillPrice::NextOpen,
        )
        .unwrap();
        assert!(close(result.carry_paid, 0.0));
        assert!(close(
            result.strategy.compounded,
            result.benchmark.compounded
        ));
        assert!(!result.report().contains("financing"));
    }

    #[test]
    fn a_spread_becomes_half_of_itself_on_each_leg() {
        let costs = CostModel::from_spread(0.002);
        assert!(close(costs.half_spread, 0.001));
        // A round trip pays the whole spread, once in and once out.
        assert!(close(costs.charge(2.0), 0.002));
    }

    #[test]
    fn the_report_names_the_verdict_and_the_assumptions() {
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "110", "110"),
            bar("2026-08-05", "120", "120"),
        ];
        let report = run(
            &bars,
            SeriesChoice::Reported,
            &mut BuyAndHold,
            &CostModel::frictionless(),
            FillPrice::SameClose,
        )
        .unwrap()
        .report();

        assert!(report.contains("buy & hold"), "{report}");
        assert!(report.contains("frictionless"), "{report}");
        assert!(report.contains("overnight gap"), "{report}");
    }
}
