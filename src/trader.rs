//! Turning a strategy's target into an instruction, and watching what happens.
//!
//! Deliberately split in two. [`decide`] and [`plan`] are **pure**: they take
//! the portfolio as data and return what should happen, touching nothing. Only
//! [`await_terminal`] talks to the API, and it only reads.
//!
//! That split is not tidiness. The interesting failures here — acting twice,
//! acting on a position something else opened, acting on a target the backtest
//! never modelled — are decision failures, not transport failures, and a
//! decision that can only be exercised by placing a real order is a decision
//! that never gets tested.
//!
//! **Nothing in this module places or closes anything.** The safety rails that
//! have to exist before it can are milestone 6's: hard limits, a cost check, an
//! audit log, and approval before anything runs unattended.

use std::time::Duration;

use crate::backtest::Strategy;
use crate::client::EtoroClient;
use crate::data::{Bar, SeriesChoice};
use crate::error::ApiError;
use crate::orders::{ClosePosition, MarketBuy, OrderError, OrderHandle, OrderStatus, status_of};
use crate::types::manual::Numeric;
// The types `portfolio()` actually returns. `PortfolioResponse` inlines its
// `clientPortfolio` rather than referencing the `ClientPortfolio` component,
// and — a genuine oddity of the spec — the *real* portfolio's positions are
// typed as `TradingDemoApiPosition`. Following the response rather than the
// similarly-named components is what keeps this compiling against reality.
use crate::types::components::{PortfolioResponseClientPortfolio, TradingDemoApiPosition};

/// An open position in one instrument, reduced to what a decision needs.
#[derive(Debug, Clone, PartialEq)]
pub struct Holding {
    pub position_id: i64,
    pub instrument_id: i32,
    pub units: Numeric,
    /// USD allocated, including the initial investment.
    pub amount: Option<Numeric>,
}

/// What a target implies, given what is already held.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// The position already matches the target.
    Hold,
    Open(MarketBuy),
    Close(ClosePosition),
}

impl Action {
    /// A one-line description, for printing a plan before anyone acts on it.
    pub fn describe(&self) -> String {
        match self {
            Self::Hold => "hold - the position already matches the target".to_owned(),
            Self::Open(order) => format!(
                "OPEN instrument {} for ${}",
                order.instrument_id(),
                order.amount().0
            ),
            Self::Close(close) => format!(
                "CLOSE position {}{}",
                close.position_id(),
                if close.is_full_close() {
                    " entirely"
                } else {
                    " partially"
                }
            ),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TraderError {
    /// The strategy asked for a partial position.
    ///
    /// Refused rather than rounded. The engine models fractional weights
    /// exactly, and a live trader that silently rounded 0.4 to "all in" would
    /// be running a different strategy from the one that was backtested —
    /// while every report continued to describe the backtested one. Scaling
    /// positions live is real work; pretending it is done is not.
    #[error("a live target must be 0 or 1, got {target}; fractional positions are not implemented")]
    FractionalTarget { target: f64 },

    /// Something opened a position this trader would never have opened.
    ///
    /// A short, or leverage, in an instrument a long-only strategy manages.
    /// The safe response is to stop: the account contains a bet whose intent
    /// is unknown here, and closing it might be exactly wrong.
    #[error(
        "position {position_id} in instrument {instrument_id} is not a plain long; refusing to act"
    )]
    UnexpectedPosition {
        position_id: i64,
        instrument_id: i32,
    },

    /// More than one open position in the same instrument.
    ///
    /// "Go flat" is then ambiguous, and closing one of two would leave the
    /// account half in a position the strategy believes it has exited.
    #[error(
        "{count} open positions in instrument {instrument_id}; refusing to guess which to close"
    )]
    MultiplePositions { instrument_id: i32, count: usize },

    /// A position the API described too incompletely to act on.
    #[error("position in instrument {instrument_id} omitted {field}")]
    IncompletePosition {
        instrument_id: i32,
        field: &'static str,
    },

    #[error("the strategy produced no target: {0} bars is too few")]
    NoHistory(usize),

    #[error(transparent)]
    Order(#[from] OrderError),
}

/// The single open long in `instrument_id`, if there is one.
///
/// Every failure here is a refusal rather than a guess. Positions are the one
/// input this code does not control: a human, another program, or an earlier
/// version of this one can all have opened something, and a trader that
/// assumes otherwise will act on it.
pub fn holding_for(
    portfolio: &PortfolioResponseClientPortfolio,
    instrument_id: i32,
) -> Result<Option<Holding>, TraderError> {
    let matching: Vec<&TradingDemoApiPosition> = portfolio
        .positions
        .iter()
        .filter(|position| position.instrument_id == Some(i64::from(instrument_id)))
        .collect();

    let position = match matching.as_slice() {
        [] => return Ok(None),
        [only] => *only,
        many => {
            return Err(TraderError::MultiplePositions {
                instrument_id,
                count: many.len(),
            });
        }
    };

    // `is_buy` absent is not treated as long: an unstated direction is exactly
    // the case where assuming is expensive.
    let unlevered = position
        .leverage
        .is_none_or(|leverage| leverage.0 == rust_decimal::Decimal::ONE);
    if position.is_buy != Some(true) || !unlevered {
        return Err(TraderError::UnexpectedPosition {
            position_id: position.position_id.unwrap_or_default(),
            instrument_id,
        });
    }

    let field = |field: &'static str| TraderError::IncompletePosition {
        instrument_id,
        field,
    };
    Ok(Some(Holding {
        position_id: position.position_id.ok_or_else(|| field("positionID"))?,
        instrument_id,
        units: position.units.ok_or_else(|| field("units"))?,
        amount: position.amount,
    }))
}

/// What to do, given a target and what is held.
///
/// Pure, and the only place the open/close asymmetry is decided.
pub fn decide(
    target: f64,
    held: Option<&Holding>,
    instrument_id: i32,
    allocation: Numeric,
) -> Result<Action, TraderError> {
    // Compared exactly, not with a tolerance. A strategy that means "all in"
    // returns 1.0; anything else is either a fractional target this cannot
    // honour or a bug, and both deserve to stop rather than be rounded into
    // something plausible.
    let wants_long = if target == 1.0 {
        true
    } else if target == 0.0 {
        false
    } else {
        // Also catches NaN, since every comparison against it is false.
        return Err(TraderError::FractionalTarget { target });
    };

    Ok(match (wants_long, held) {
        (true, None) => Action::Open(MarketBuy::new(i64::from(instrument_id), allocation)?),
        (false, Some(holding)) => Action::Close(ClosePosition::all(
            holding.position_id,
            holding.instrument_id,
        )?),
        // Already where it wants to be, in both directions. Without this the
        // strategy would re-buy on every bar it stayed long.
        (true, Some(_)) | (false, None) => Action::Hold,
    })
}

/// What the strategy wants right now, and what that implies.
#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    pub target: f64,
    pub held: Option<Holding>,
    pub action: Action,
}

/// Runs `strategy` over `bars` and works out the resulting instruction.
///
/// The strategy sees the **whole** stored series, because live it is standing
/// on the last bar rather than walking through history. That is the same
/// prefix contract the backtest enforces: everything up to and including the
/// most recent bar, and nothing after it.
pub fn plan(
    portfolio: &PortfolioResponseClientPortfolio,
    instrument_id: i32,
    bars: &[Bar],
    series: SeriesChoice,
    strategy: &mut dyn Strategy,
    allocation: Numeric,
) -> Result<Plan, TraderError> {
    if bars.is_empty() {
        return Err(TraderError::NoHistory(0));
    }
    let held = holding_for(portfolio, instrument_id)?;
    let target = strategy.target(bars, series);
    let action = decide(target, held.as_ref(), instrument_id, allocation)?;
    Ok(Plan {
        target,
        held,
        action,
    })
}

/// Polls until an order stops moving, or the attempts run out.
///
/// Submission is asynchronous: acceptance says only that eToro has the order.
/// Returns the last status seen, so an order still in flight at the deadline is
/// reported as such rather than mistaken for a failure — an order that has not
/// finished is not an order that did not happen.
///
/// Reads only. Draws on a 60 / 60 s pool shared with two other lookups, so
/// `interval` should be seconds rather than milliseconds.
pub async fn await_terminal(
    client: &EtoroClient,
    handle: OrderHandle,
    interval: Duration,
    max_polls: u32,
) -> Result<Option<OrderStatus>, ApiError> {
    let mut last = None;
    for poll in 0..max_polls.max(1) {
        if poll > 0 {
            tokio::time::sleep(interval).await;
        }
        let info = client.lookup_order(handle).await?;
        last = status_of(&info);
        if last.is_some_and(OrderStatus::is_terminal) {
            break;
        }
    }
    Ok(last)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn numeric(text: &str) -> Numeric {
        Numeric(text.parse().unwrap())
    }

    fn position(instrument_id: i64, position_id: i64) -> TradingDemoApiPosition {
        TradingDemoApiPosition {
            instrument_id: Some(instrument_id),
            position_id: Some(position_id),
            units: Some(numeric("3")),
            amount: Some(numeric("100")),
            is_buy: Some(true),
            leverage: Some(numeric("1")),
            ..Default::default()
        }
    }

    fn portfolio(positions: Vec<TradingDemoApiPosition>) -> PortfolioResponseClientPortfolio {
        PortfolioResponseClientPortfolio {
            positions,
            ..Default::default()
        }
    }

    fn holding() -> Holding {
        Holding {
            position_id: 555,
            instrument_id: 1001,
            units: numeric("3"),
            amount: Some(numeric("100")),
        }
    }

    #[test]
    fn a_target_already_matched_produces_no_order() {
        // The bug this prevents is expensive and quiet: without it a strategy
        // that stays long re-buys on every single bar.
        assert_eq!(
            decide(1.0, Some(&holding()), 1001, numeric("100")).unwrap(),
            Action::Hold
        );
        assert_eq!(
            decide(0.0, None, 1001, numeric("100")).unwrap(),
            Action::Hold
        );
    }

    #[test]
    fn going_long_opens_and_going_flat_closes() {
        let open = decide(1.0, None, 1001, numeric("250")).unwrap();
        let Action::Open(order) = &open else {
            panic!("expected an open, got {open:?}");
        };
        assert_eq!(order.instrument_id(), 1001);
        assert_eq!(order.amount(), numeric("250"));

        let close = decide(0.0, Some(&holding()), 1001, numeric("250")).unwrap();
        let Action::Close(request) = &close else {
            panic!("expected a close, got {close:?}");
        };
        assert_eq!(request.position_id(), 555);
        // "Go flat" means all of it, never a part.
        assert!(request.is_full_close());
    }

    #[test]
    fn a_fractional_target_is_refused_rather_than_rounded() {
        // Rounding would run a different strategy from the backtested one
        // while every report kept describing the backtested one.
        for target in [0.5, 0.01, 0.99, -0.0001, 1.5, f64::NAN] {
            assert!(
                matches!(
                    decide(target, None, 1001, numeric("100")),
                    Err(TraderError::FractionalTarget { .. })
                ),
                "{target} should not be tradeable"
            );
        }
    }

    #[test]
    fn an_empty_portfolio_holds_nothing() {
        assert_eq!(holding_for(&portfolio(vec![]), 1001).unwrap(), None);
        // A position in a different instrument is not this one's.
        assert_eq!(
            holding_for(&portfolio(vec![position(2002, 1)]), 1001).unwrap(),
            None
        );
    }

    #[test]
    fn a_single_long_is_read_back_in_full() {
        let found = holding_for(&portfolio(vec![position(1001, 555)]), 1001)
            .unwrap()
            .unwrap();
        assert_eq!(found, holding());
    }

    #[test]
    fn two_positions_in_one_instrument_stop_the_decision() {
        // Closing one of two would leave the account half in a position the
        // strategy believes it has exited.
        assert_eq!(
            holding_for(&portfolio(vec![position(1001, 1), position(1001, 2)]), 1001),
            Err(TraderError::MultiplePositions {
                instrument_id: 1001,
                count: 2
            })
        );
    }

    #[test]
    fn a_position_this_trader_would_not_have_opened_stops_the_decision() {
        // A short, or a leveraged position, means something else placed this
        // bet. Closing it might be exactly the wrong move.
        let mut short = position(1001, 7);
        short.is_buy = Some(false);
        assert!(matches!(
            holding_for(&portfolio(vec![short]), 1001),
            Err(TraderError::UnexpectedPosition { position_id: 7, .. })
        ));

        let mut levered = position(1001, 8);
        levered.leverage = Some(numeric("5"));
        assert!(matches!(
            holding_for(&portfolio(vec![levered]), 1001),
            Err(TraderError::UnexpectedPosition { position_id: 8, .. })
        ));

        // An unstated direction is not assumed to be long.
        let mut silent = position(1001, 9);
        silent.is_buy = None;
        assert!(matches!(
            holding_for(&portfolio(vec![silent]), 1001),
            Err(TraderError::UnexpectedPosition { position_id: 9, .. })
        ));
    }

    #[test]
    fn a_position_missing_the_fields_needed_to_close_it_is_refused() {
        let mut anonymous = position(1001, 5);
        anonymous.position_id = None;
        assert_eq!(
            holding_for(&portfolio(vec![anonymous]), 1001),
            Err(TraderError::IncompletePosition {
                instrument_id: 1001,
                field: "positionID"
            })
        );
    }

    #[test]
    fn a_plan_reports_the_target_alongside_the_action() {
        use crate::backtest::BuyAndHold;
        use crate::types::manual::Numeric as N;

        let bars = [Bar {
            date: "2026-08-28".parse().unwrap(),
            open: N("100".parse().unwrap()),
            high: N("100".parse().unwrap()),
            low: N("100".parse().unwrap()),
            close: N("100".parse().unwrap()),
            volume: None,
            total_return_close: None,
            dividend_cash: None,
            split_factor: None,
            split_adjusted_volume: None,
        }];

        let plan = plan(
            &portfolio(vec![]),
            1001,
            &bars,
            SeriesChoice::Reported,
            &mut BuyAndHold,
            numeric("100"),
        )
        .unwrap();

        assert_eq!(plan.target, 1.0);
        assert_eq!(plan.held, None);
        assert!(matches!(plan.action, Action::Open(_)));
        assert!(plan.action.describe().contains("OPEN instrument 1001"));
    }
}
