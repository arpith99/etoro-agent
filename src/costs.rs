//! What an order would cost, asked before it is placed.
//!
//! `POST /api/v2/trading/info/costs` takes **the same body as the order
//! endpoint**, so the thing being priced is the exact order about to be sent
//! rather than an approximation of it. That is worth more than it sounds: the
//! spread and fees on a $100 order can be a large fraction of any edge a simple
//! strategy produces, and the observed spreads across ordinary names span
//! roughly 50x.
//!
//! The breakdown separates two kinds of cost, and the distinction runs through
//! everything downstream:
//!
//! - **Up-front** — `markup`, `marketSpread`, `transactionFee`, `sdrt`. Paid
//!   once, when the order executes. This is what the backtest's [`CostModel`]
//!   already models as a charge on turnover.
//! - **Carried** — `overnightFee`, `overWeekendFee`. Paid repeatedly, for as
//!   long as the position stays open. **The backtest does not model these at
//!   all**, so any strategy holding a position that incurs them is being
//!   measured optimistically, by an amount that grows with the holding period.
//!
//! [`CostModel`]: crate::backtest::CostModel

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::types::manual::Numeric;

/// A component of the quoted cost.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CostType {
    /// A price or currency-conversion markup applied by eToro.
    Markup,
    /// The market spread included in the execution price.
    MarketSpread,
    /// A fee for executing the trade, where one applies.
    TransactionFee,
    /// Charged for keeping the position open overnight.
    OvernightFee,
    /// Charged for keeping the position open over a weekend.
    OverWeekendFee,
    /// Stamp duty reserve tax on eligible settled buys.
    Sdrt,
    /// A component this crate does not recognise.
    ///
    /// Carried rather than refused, for the same reason
    /// [`OrderStatus`](crate::orders::OrderStatus) is open: a cost check that
    /// fails to decode is a cost check that cannot happen, and an order placed
    /// without one is worse than an order priced with an unlabelled component
    /// in it. See [`CostEstimate::has_unclassified`].
    Unknown(String),
}

impl CostType {
    fn parse(text: &str) -> Self {
        match text {
            "markup" => Self::Markup,
            "marketSpread" => Self::MarketSpread,
            "transactionFee" => Self::TransactionFee,
            "overnightFee" => Self::OvernightFee,
            "overWeekendFee" => Self::OverWeekendFee,
            "sdrt" => Self::Sdrt,
            other => Self::Unknown(other.to_owned()),
        }
    }

    /// Whether this is charged for every day the position stays open, rather
    /// than once when it is created.
    pub fn is_carried(self_: &Self) -> bool {
        matches!(self_, Self::OvernightFee | Self::OverWeekendFee)
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Markup => "markup",
            Self::MarketSpread => "marketSpread",
            Self::TransactionFee => "transactionFee",
            Self::OvernightFee => "overnightFee",
            Self::OverWeekendFee => "overWeekendFee",
            Self::Sdrt => "sdrt",
            Self::Unknown(name) => name,
        }
    }
}

impl std::fmt::Display for CostType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // `pad`, not `write_str`: a manual Display impl that writes directly
        // silently ignores the formatter's width, so `{:<16}` does nothing and
        // every column after it drifts.
        f.pad(self.name())
    }
}

/// One line of the breakdown.
#[derive(Debug, Clone, PartialEq)]
pub struct CostComponent {
    pub kind: CostType,
    pub amount: Numeric,
    pub currency: String,
}

/// The wire shape, kept private so the public type can carry a parsed
/// [`CostType`] rather than a string.
#[derive(Debug, Deserialize)]
struct WireResponse {
    #[serde(default, rename = "instrumentId")]
    instrument_id: Option<i64>,
    #[serde(default)]
    symbol: Option<String>,
    #[serde(default)]
    costs: Vec<WireComponent>,
}

#[derive(Debug, Deserialize)]
struct WireComponent {
    #[serde(default, rename = "costType")]
    cost_type: Option<String>,
    #[serde(default)]
    amount: Option<Numeric>,
    #[serde(default)]
    currency: Option<String>,
}

/// What eToro says the proposed order would cost.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(from = "WireResponse")]
pub struct CostEstimate {
    pub instrument_id: Option<i64>,
    pub symbol: Option<String>,
    pub components: Vec<CostComponent>,
}

impl From<WireResponse> for CostEstimate {
    fn from(wire: WireResponse) -> Self {
        Self {
            instrument_id: wire.instrument_id,
            symbol: wire.symbol,
            components: wire
                .costs
                .into_iter()
                .map(|component| CostComponent {
                    kind: CostType::parse(component.cost_type.as_deref().unwrap_or("")),
                    // A component with no amount contributes nothing rather
                    // than failing the estimate; the currency defaults to the
                    // only one orders may be placed in.
                    amount: component.amount.unwrap_or(Numeric(Decimal::ZERO)),
                    currency: component.currency.unwrap_or_else(|| "USD".to_owned()),
                })
                .collect(),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum CostError {
    /// Components denominated in different currencies.
    ///
    /// Refused rather than summed. Adding 10 USD to 5 GBP produces a number
    /// that is not money, and the gate this feeds would then be comparing it
    /// against an order amount in a third unit.
    #[error("cost components span several currencies ({0}); refusing to add them together")]
    MixedCurrencies(String),
}

impl CostEstimate {
    #[cfg(test)]
    fn from_wire(wire: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(wire)
    }

    /// Cost paid once, when the order executes.
    ///
    /// Unrecognised components are counted here rather than dropped or treated
    /// as carried. That is the conservative direction: over-counting the
    /// up-front figure produces a refusal, while under-counting the carried one
    /// is a silent underestimate that only shows up in the account weeks later.
    pub fn upfront(&self) -> Result<Numeric, CostError> {
        self.sum(|kind| !CostType::is_carried(kind))
    }

    /// Cost charged for each day the position stays open.
    ///
    /// Zero for an ordinary unleveraged long. Non-zero is the signal that the
    /// backtest is not modelling something: it charges costs on turnover only,
    /// so a position carrying a daily fee is measured optimistically by an
    /// amount proportional to how long it is held.
    pub fn per_day(&self) -> Result<Numeric, CostError> {
        self.sum(CostType::is_carried)
    }

    /// Whether any component could not be classified.
    ///
    /// A caller that wants to fail closed should check this: an unrecognised
    /// component may be a daily fee counted as a one-off.
    pub fn has_unclassified(&self) -> bool {
        self.components
            .iter()
            .any(|component| matches!(component.kind, CostType::Unknown(_)))
    }

    /// The up-front cost as a fraction of `notional`.
    ///
    /// `None` when `notional` is zero, since every order is then infinitely
    /// expensive and saying so is less useful than saying nothing.
    pub fn upfront_fraction_of(&self, notional: Numeric) -> Result<Option<Decimal>, CostError> {
        if notional.0 <= Decimal::ZERO {
            return Ok(None);
        }
        Ok(Some(self.upfront()?.0 / notional.0))
    }

    fn sum(&self, include: impl Fn(&CostType) -> bool) -> Result<Numeric, CostError> {
        let mut currencies: Vec<&str> = self
            .components
            .iter()
            .filter(|component| component.amount.0 != Decimal::ZERO)
            .map(|component| component.currency.as_str())
            .collect();
        currencies.sort_unstable();
        currencies.dedup();
        if currencies.len() > 1 {
            return Err(CostError::MixedCurrencies(currencies.join(", ")));
        }

        Ok(Numeric(
            self.components
                .iter()
                .filter(|component| include(&component.kind))
                .fold(Decimal::ZERO, |total, component| {
                    total.saturating_add(component.amount.0)
                }),
        ))
    }

    /// A fixed-width breakdown, for printing before anyone approves anything.
    pub fn report(&self) -> String {
        let mut out = String::new();
        for component in &self.components {
            out.push_str(&format!(
                "    {:<16}{:>12} {}{}\n",
                component.kind,
                component.amount.0,
                component.currency,
                if CostType::is_carried(&component.kind) {
                    "  (per day held)"
                } else {
                    ""
                },
            ));
        }
        if self.components.is_empty() {
            out.push_str("    no cost components quoted\n");
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn numeric(text: &str) -> Numeric {
        Numeric(text.parse().unwrap())
    }

    fn estimate(components: &[(&str, &str)]) -> CostEstimate {
        CostEstimate {
            instrument_id: Some(1001),
            symbol: Some("AAPL".to_owned()),
            components: components
                .iter()
                .map(|(kind, amount)| CostComponent {
                    kind: CostType::parse(kind),
                    amount: numeric(amount),
                    currency: "USD".to_owned(),
                })
                .collect(),
        }
    }

    #[test]
    fn one_off_and_daily_costs_are_kept_apart() {
        // The distinction the backtest depends on: it charges turnover only,
        // so anything in per_day is a cost it does not model.
        let costs = estimate(&[
            ("marketSpread", "0.30"),
            ("markup", "0.05"),
            ("overnightFee", "0.02"),
            ("overWeekendFee", "0.06"),
        ]);
        assert_eq!(costs.upfront().unwrap(), numeric("0.35"));
        assert_eq!(costs.per_day().unwrap(), numeric("0.08"));
    }

    #[test]
    fn an_ordinary_long_carries_nothing_daily() {
        let costs = estimate(&[("marketSpread", "0.30"), ("sdrt", "0.50")]);
        assert_eq!(costs.per_day().unwrap(), numeric("0"));
        assert!(!costs.has_unclassified());
    }

    #[test]
    fn an_unrecognised_component_counts_against_the_up_front_budget() {
        // Conservative on purpose: over-counting up-front causes a refusal,
        // under-counting the daily figure is a silent underestimate.
        let costs = estimate(&[("marketSpread", "0.30"), ("borrowFee", "1.00")]);
        assert_eq!(costs.upfront().unwrap(), numeric("1.30"));
        assert_eq!(costs.per_day().unwrap(), numeric("0"));
        assert!(costs.has_unclassified(), "and it is flagged, not hidden");
        assert_eq!(
            costs.components[1].kind,
            CostType::Unknown("borrowFee".to_owned())
        );
    }

    #[test]
    fn costs_in_different_currencies_are_refused_rather_than_added() {
        let mut costs = estimate(&[("marketSpread", "0.30"), ("transactionFee", "1.00")]);
        costs.components[1].currency = "GBP".to_owned();
        assert_eq!(
            costs.upfront(),
            Err(CostError::MixedCurrencies("GBP, USD".to_owned()))
        );

        // A zero component in another currency is not a conflict: it adds
        // nothing, so there is nothing to get wrong.
        costs.components[1].amount = numeric("0");
        assert_eq!(costs.upfront().unwrap(), numeric("0.30"));
    }

    #[test]
    fn the_cost_fraction_is_what_gates_an_order() {
        let costs = estimate(&[("marketSpread", "0.30"), ("markup", "0.20")]);
        // $0.50 on a $100 order is 0.5%.
        assert_eq!(
            costs.upfront_fraction_of(numeric("100")).unwrap(),
            Some("0.005".parse().unwrap())
        );
        // The same cost on a $2500 order is a fifth of that, which is the
        // whole reason this is a fraction and not an amount.
        assert_eq!(
            costs.upfront_fraction_of(numeric("2500")).unwrap(),
            Some("0.0002".parse().unwrap())
        );
        assert_eq!(costs.upfront_fraction_of(numeric("0")).unwrap(), None);
    }

    #[test]
    fn the_documented_response_shape_decodes() {
        let estimate = CostEstimate::from_wire(
            br#"{"instrumentId":1001,"symbol":"AAPL","lastUpdated":"2026-08-30T12:00:00Z",
                 "costs":[{"costType":"marketSpread","amount":0.30,"currency":"USD"},
                          {"costType":"overnightFee","amount":0.0125,"currency":"USD"}]}"#,
        )
        .unwrap();
        assert_eq!(estimate.symbol.as_deref(), Some("AAPL"));
        assert_eq!(estimate.components.len(), 2);
        // Scale preserved, as everywhere else money is handled.
        assert_eq!(estimate.per_day().unwrap().0.to_string(), "0.0125");
        let report = estimate.report();
        assert!(report.contains("(per day held)"));
        // Columns must actually line up: `Display` impls that ignore the
        // formatter's width are easy to write and invisible until printed.
        let columns: Vec<usize> = report
            .lines()
            .map(|line| line.find("USD").expect("every line names a currency"))
            .collect();
        assert!(
            columns.windows(2).all(|pair| pair[0] == pair[1]),
            "amounts are not aligned:\n{report}"
        );
    }

    #[test]
    fn a_response_with_no_components_is_not_an_error() {
        // "This order costs nothing" is a real answer, and distinct from a
        // failed request.
        let estimate = CostEstimate::from_wire(br#"{"instrumentId":1001}"#).unwrap();
        assert_eq!(estimate.upfront().unwrap(), numeric("0"));
        assert!(estimate.report().contains("no cost components"));
    }
}
