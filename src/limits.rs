//! Hard limits, checked before anything is sent.
//!
//! These are brakes on *this program*, not opinions about markets. Each one
//! bounds a specific way an automated trader goes wrong: a sizing bug that
//! turns $100 into $10,000, a signal that flickers and trades forty times in an
//! afternoon, a slow accumulation of positions nobody is watching, and the case
//! where you simply want it to stop right now.
//!
//! **Only opening is constrained.** Closing is always permitted — except under
//! the kill switch — because every limit here exists to bound risk, and
//! refusing to close increases it. A daily order cap that stopped an exit would
//! be a limit that traps you in a position, which is the opposite of a brake.

use std::path::{Path, PathBuf};

use rust_decimal::Decimal;

use crate::costs::CostEstimate;
use crate::trader::Action;
use crate::types::manual::Numeric;
use crate::types::tags::trading_real::PortfolioResponse;

/// The bounds an action is checked against.
///
/// No `Default`. Every field is a number about real money, and inheriting one
/// silently is how a limit ends up being whatever the author happened to type
/// while testing.
#[derive(Debug, Clone, PartialEq)]
pub struct Limits {
    /// Most that may go into a single new position, in USD.
    pub max_position_usd: Numeric,
    /// Most that may be held across all positions at once, in USD.
    pub max_exposure_usd: Numeric,
    /// Most orders that may be **submitted** in one UTC day.
    pub max_orders_per_day: u32,
    /// Most of an order's value that may be consumed by up-front cost.
    ///
    /// A fraction rather than an amount, because the same $0.50 of spread is
    /// negligible on $2500 and half a percent on $100. Checked separately from
    /// [`Self::check`] because it needs a live quote, and the rest of the rules
    /// deliberately need nothing at all.
    pub max_cost_fraction: Decimal,
    /// While this file exists, nothing is sent at all.
    ///
    /// A file rather than a config flag or an environment variable, because
    /// those all require reaching the program: a file can be created from any
    /// shell, over ssh from a phone, while a run is already in progress, by
    /// somebody who has never read this code.
    ///
    /// Note the failure direction: presence stops trading, so a wiped disk
    /// resumes it. The alternative — requiring a file to be present before
    /// trading — fails safe but is forgotten far more often, and a rail that
    /// gets disabled for being annoying protects nothing.
    pub kill_switch: PathBuf,
}

/// Why an action was refused.
#[derive(Debug, Clone, PartialEq)]
pub enum Breach {
    /// The kill switch file exists. Blocks closes as well as opens: it means
    /// "this program is not to act", and if you no longer trust it to open a
    /// position you should not trust it to choose when to exit one either.
    /// Closing by hand is always available.
    KillSwitch {
        path: PathBuf,
    },
    PositionTooLarge {
        requested: Numeric,
        max: Numeric,
    },
    ExposureTooLarge {
        open: Numeric,
        requested: Numeric,
        max: Numeric,
    },
    TooManyOrdersToday {
        submitted: usize,
        max: u32,
    },
    CostTooHigh {
        cost: Numeric,
        notional: Numeric,
        fraction: Decimal,
        max: Decimal,
    },
    /// The quote contained a component this crate could not classify.
    ///
    /// Refused rather than assumed harmless: an unrecognised component may be
    /// a daily fee being counted as a one-off, and the whole point of the
    /// check is knowing what is being paid.
    UnclassifiedCost {
        name: String,
    },
    /// A cost that could not be totalled -- see
    /// [`CostError`](crate::costs::CostError).
    CostNotComparable {
        detail: String,
    },
}

impl std::fmt::Display for Breach {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KillSwitch { path } => write!(
                f,
                "kill switch {} exists; remove it to allow trading",
                path.display()
            ),
            Self::PositionTooLarge { requested, max } => write!(
                f,
                "a position of ${} exceeds the ${} single-position limit",
                requested.0, max.0
            ),
            Self::ExposureTooLarge {
                open,
                requested,
                max,
            } => write!(
                f,
                "${} already open plus ${} requested exceeds the ${} total exposure limit",
                open.0, requested.0, max.0
            ),
            Self::TooManyOrdersToday { submitted, max } => {
                write!(f, "{submitted} orders already submitted today, limit {max}")
            }
            Self::CostTooHigh {
                cost,
                notional,
                fraction,
                max,
            } => write!(
                f,
                "${} of cost on a ${} order is {:.3}% of it, over the {:.3}% limit",
                cost.0,
                notional.0,
                fraction * Decimal::ONE_HUNDRED,
                max * Decimal::ONE_HUNDRED
            ),
            Self::UnclassifiedCost { name } => write!(
                f,
                "the quote contains an unrecognised cost component {name:?}; refusing until \
                 it is known whether it is charged once or daily"
            ),
            Self::CostNotComparable { detail } => {
                write!(f, "the quoted cost could not be totalled: {detail}")
            }
        }
    }
}

impl Limits {
    /// Whether `action` may proceed.
    ///
    /// `open_exposure` and `orders_today` are passed in rather than read here,
    /// so the whole rule is pure and every branch is testable without a
    /// filesystem, an account, or a clock. The one exception is the kill
    /// switch, which is a filesystem question by design.
    pub fn check(
        &self,
        action: &Action,
        open_exposure: Numeric,
        orders_today: usize,
    ) -> Result<(), Breach> {
        if self.kill_switch.exists() {
            return Err(Breach::KillSwitch {
                path: self.kill_switch.clone(),
            });
        }

        // Closing reduces risk, so nothing below applies to it. Holding sends
        // nothing at all.
        let Action::Open(order) = action else {
            return Ok(());
        };

        let requested = order.amount();
        if requested.0 > self.max_position_usd.0 {
            return Err(Breach::PositionTooLarge {
                requested,
                max: self.max_position_usd,
            });
        }
        // Saturating, so an absurd amount reports the limit it broke rather
        // than panicking on overflow.
        let after = open_exposure.0.saturating_add(requested.0);
        if after > self.max_exposure_usd.0 {
            return Err(Breach::ExposureTooLarge {
                open: open_exposure,
                requested,
                max: self.max_exposure_usd,
            });
        }
        // `>=` because this order would be the next one, not the last one.
        if orders_today >= self.max_orders_per_day as usize {
            return Err(Breach::TooManyOrdersToday {
                submitted: orders_today,
                max: self.max_orders_per_day,
            });
        }
        Ok(())
    }

    /// Whether the quoted cost of an order is acceptable.
    ///
    /// Separate from [`Self::check`] because it needs a live quote from
    /// `POST /trading/info/costs`, while every other rule here is pure. Call
    /// both: this one says nothing about size, exposure or the kill switch.
    ///
    /// Only up-front cost is compared. The carried cost is reported by
    /// [`CostEstimate::per_day`] and deliberately not gated here, because a
    /// daily fee is not comparable to a one-off without knowing the holding
    /// period — and the backtest that would supply it does not model carry at
    /// all yet. A non-zero `per_day` should be surfaced to a human rather than
    /// silently accepted or silently refused.
    pub fn check_cost(&self, estimate: &CostEstimate, notional: Numeric) -> Result<(), Breach> {
        if let Some(component) = estimate
            .components
            .iter()
            .find(|component| matches!(component.kind, crate::costs::CostType::Unknown(_)))
        {
            return Err(Breach::UnclassifiedCost {
                name: component.kind.name().to_owned(),
            });
        }

        let fraction =
            estimate
                .upfront_fraction_of(notional)
                .map_err(|error| Breach::CostNotComparable {
                    detail: error.to_string(),
                })?;
        // No notional means no fraction to compare; size is `check`'s job.
        let Some(fraction) = fraction else {
            return Ok(());
        };
        if fraction > self.max_cost_fraction {
            return Err(Breach::CostTooHigh {
                cost: estimate
                    .upfront()
                    .map_err(|error| Breach::CostNotComparable {
                        detail: error.to_string(),
                    })?,
                notional,
                fraction,
                max: self.max_cost_fraction,
            });
        }
        Ok(())
    }

    /// Whether the kill switch is currently engaged.
    pub fn stopped(&self) -> bool {
        self.kill_switch.exists()
    }

    pub fn kill_switch_path(&self) -> &Path {
        &self.kill_switch
    }
}

/// Total USD currently committed across every open position.
///
/// Sums the `amount` eToro reports, which it documents as including the initial
/// investment. Positions that omit it contribute nothing, which understates
/// exposure — so this is a floor, not a measurement, and a limit built on it is
/// looser than it looks. It is still worth having: the case it catches is a
/// bug that opens position after position, and that shows up here immediately.
pub fn open_exposure(portfolio: &PortfolioResponse) -> Numeric {
    let Some(client_portfolio) = portfolio.client_portfolio.as_ref() else {
        return Numeric(Decimal::ZERO);
    };
    Numeric(
        client_portfolio
            .positions
            .iter()
            .filter_map(|position| position.amount)
            .fold(Decimal::ZERO, |total, amount| {
                total.saturating_add(amount.0)
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orders::{ClosePosition, MarketBuy};

    fn numeric(text: &str) -> Numeric {
        Numeric(text.parse().unwrap())
    }

    fn limits() -> Limits {
        Limits {
            max_position_usd: numeric("500"),
            max_exposure_usd: numeric("1000"),
            max_orders_per_day: 4,
            max_cost_fraction: "0.01".parse().unwrap(),
            // A path nothing creates, so the switch reads as disengaged.
            kill_switch: std::env::temp_dir()
                .join(format!("etoro-agent-absent-{}", uuid::Uuid::new_v4())),
        }
    }

    fn open(amount: &str) -> Action {
        Action::Open(MarketBuy::new(1001, numeric(amount)).unwrap())
    }

    fn close() -> Action {
        Action::Close(ClosePosition::all(555, 1001).unwrap())
    }

    struct TouchedFile(PathBuf);

    impl TouchedFile {
        fn new() -> Self {
            let path =
                std::env::temp_dir().join(format!("etoro-agent-stop-{}", uuid::Uuid::new_v4()));
            std::fs::write(&path, b"stop").unwrap();
            Self(path)
        }
    }

    impl Drop for TouchedFile {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    #[test]
    fn an_ordinary_order_within_every_bound_is_allowed() {
        assert_eq!(
            limits().check(&open("100"), numeric("200"), 1),
            Ok(()),
            "nothing here is breached"
        );
    }

    #[test]
    fn a_single_position_cannot_exceed_its_limit() {
        // The sizing-bug case: a decimal point in the wrong place.
        assert_eq!(
            limits().check(&open("5000"), numeric("0"), 0),
            Err(Breach::PositionTooLarge {
                requested: numeric("5000"),
                max: numeric("500")
            })
        );
        // Exactly at the limit is allowed; the limit is a maximum, not a gap.
        assert_eq!(limits().check(&open("500"), numeric("0"), 0), Ok(()));
    }

    #[test]
    fn exposure_counts_what_is_already_open() {
        // Each order is individually fine and the total is not, which is how
        // an accumulation bug actually presents.
        assert_eq!(
            limits().check(&open("500"), numeric("600"), 0),
            Err(Breach::ExposureTooLarge {
                open: numeric("600"),
                requested: numeric("500"),
                max: numeric("1000")
            })
        );
        assert_eq!(limits().check(&open("400"), numeric("600"), 0), Ok(()));
    }

    #[test]
    fn the_daily_cap_counts_the_order_about_to_be_sent() {
        let limits = limits();
        assert_eq!(limits.check(&open("100"), numeric("0"), 3), Ok(()));
        // The fourth order is the limit, so a fourth already sent means stop.
        assert_eq!(
            limits.check(&open("100"), numeric("0"), 4),
            Err(Breach::TooManyOrdersToday {
                submitted: 4,
                max: 4
            })
        );
    }

    #[test]
    fn closing_is_never_blocked_by_a_risk_limit() {
        // Every bound is breached at once, and the exit still goes through:
        // refusing to close would trap the position the limits exist to bound.
        let limits = limits();
        assert_eq!(limits.check(&close(), numeric("999999"), 999), Ok(()));
        assert_eq!(limits.check(&Action::Hold, numeric("999999"), 999), Ok(()));
    }

    #[test]
    fn the_kill_switch_stops_closing_too() {
        let stop = TouchedFile::new();
        let limits = Limits {
            kill_switch: stop.0.clone(),
            ..limits()
        };
        assert!(limits.stopped());

        for action in [open("1"), close(), Action::Hold] {
            assert_eq!(
                limits.check(&action, numeric("0"), 0),
                Err(Breach::KillSwitch {
                    path: stop.0.clone()
                }),
                "{action:?} should be stopped"
            );
        }
    }

    #[test]
    fn a_breach_says_which_number_broke_and_what_the_limit_was() {
        // The message is the whole interface at 3am.
        let message = limits()
            .check(&open("5000"), numeric("0"), 0)
            .unwrap_err()
            .to_string();
        assert!(message.contains("5000"), "{message}");
        assert!(message.contains("500"), "{message}");
    }

    fn quote(components: &[(&str, &str)]) -> CostEstimate {
        CostEstimate {
            instrument_id: Some(1001),
            symbol: Some("AAPL".to_owned()),
            components: components
                .iter()
                .map(|(kind, amount)| crate::costs::CostComponent {
                    kind: match *kind {
                        "marketSpread" => crate::costs::CostType::MarketSpread,
                        "overnightFee" => crate::costs::CostType::OvernightFee,
                        other => crate::costs::CostType::Unknown(other.to_owned()),
                    },
                    amount: numeric(amount),
                    currency: "USD".to_owned(),
                })
                .collect(),
        }
    }

    #[test]
    fn a_cost_that_would_eat_the_edge_is_refused() {
        let limits = limits();
        // $0.30 on $100 is 0.3%, inside the 1% default.
        assert_eq!(
            limits.check_cost(&quote(&[("marketSpread", "0.30")]), numeric("100")),
            Ok(())
        );
        // The same $2 on $100 is 2%, and is not.
        let breach = limits
            .check_cost(&quote(&[("marketSpread", "2.00")]), numeric("100"))
            .unwrap_err();
        assert!(matches!(breach, Breach::CostTooHigh { .. }), "{breach}");
        // The message has to name both numbers to be worth reading.
        assert!(breach.to_string().contains("2.000%"), "{breach}");

        // And the same absolute cost passes on a larger order, which is the
        // whole reason the limit is a fraction.
        assert_eq!(
            limits.check_cost(&quote(&[("marketSpread", "2.00")]), numeric("2500")),
            Ok(())
        );
    }

    #[test]
    fn a_daily_fee_does_not_count_against_the_up_front_gate() {
        // Carry is real but is not comparable to a one-off without a holding
        // period, and the backtest does not model it. It must be surfaced,
        // not silently gated.
        let estimate = quote(&[("marketSpread", "0.30"), ("overnightFee", "0.50")]);
        assert_eq!(limits().check_cost(&estimate, numeric("100")), Ok(()));
        assert_eq!(estimate.per_day().unwrap(), numeric("0.50"));
    }

    #[test]
    fn an_unrecognised_cost_component_stops_the_order() {
        // It may be a daily fee counted as a one-off. Knowing what is being
        // paid is the entire point of asking.
        let breach = limits()
            .check_cost(&quote(&[("borrowFee", "0.01")]), numeric("100"))
            .unwrap_err();
        assert_eq!(
            breach,
            Breach::UnclassifiedCost {
                name: "borrowFee".to_owned()
            }
        );
        assert!(breach.to_string().contains("borrowFee"), "{breach}");
    }

    #[test]
    fn costs_that_cannot_be_totalled_refuse_rather_than_pass() {
        let mut estimate = quote(&[("marketSpread", "0.30"), ("marketSpread", "1.00")]);
        estimate.components[1].currency = "GBP".to_owned();
        assert!(matches!(
            limits().check_cost(&estimate, numeric("100")),
            Err(Breach::CostNotComparable { .. })
        ));
    }

    #[test]
    fn exposure_sums_the_positions_that_report_an_amount() {
        use crate::types::components::{PortfolioResponseClientPortfolio, TradingDemoApiPosition};

        let position = |amount: Option<Numeric>| TradingDemoApiPosition {
            amount,
            ..Default::default()
        };
        let response = PortfolioResponse {
            client_portfolio: Some(PortfolioResponseClientPortfolio {
                positions: vec![
                    position(Some(numeric("100.50"))),
                    position(Some(numeric("249.50"))),
                    // Contributes nothing, so the total is a floor.
                    position(None),
                ],
                ..Default::default()
            }),
        };
        assert_eq!(open_exposure(&response).0.to_string(), "350.00");

        // No portfolio at all is zero rather than an error: a limit that
        // cannot be evaluated should not be the thing that stops an exit.
        assert_eq!(
            open_exposure(&PortfolioResponse::default()),
            Numeric(Decimal::ZERO)
        );
    }
}
