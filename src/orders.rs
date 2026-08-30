//! Order submission: what may be asked for, and what came back.
//!
//! Two things here, and they exist for opposite reasons.
//!
//! [`MarketBuy`] is **narrower** than the API. eToro's unified order schema has
//! fifteen optional fields governed by a dozen mutual-exclusion rules — provide
//! `symbol` or `instrumentId` but never both, `amount` or `units` or
//! `contracts`, `triggerRate` only for `mit`, `limitRate` only for `limitIOC`,
//! `stopLossRate` required when leverage exceeds 1 — and the generated type
//! makes every one of them `Option`, so all of the invalid combinations are
//! expressible. This one expresses a single valid order and nothing else.
//!
//! [`OrderStatus`] is **wider** than the crate's usual rule. Every other enum
//! here is closed, so an unrecognised value fails the whole response and
//! surfaces spec drift. That rule is wrong for order status, and the reason is
//! the one that matters: by the time a status is being read, the write has
//! already happened. Refusing to decode an unknown status would leave a live
//! order — possibly a filled one, with real money in it — unobservable. So an
//! unknown id is carried rather than rejected.

use serde::{Deserialize, Serialize};

use crate::types::manual::Numeric;

/// Where an order is in its lifecycle.
///
/// Ids are eToro's, documented on `GetOrderInfoStatus.id`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Received,
    Placed,
    Filled,
    Rejected,
    PartiallyFilled,
    PendingCancel,
    Canceled,
    Expired,
    CanceledPartiallyFilled,
    RejectedPartiallyFilled,
    WaitingForMarket,
    PendingTriggeredRate,
    /// A status id this crate does not know. See the module docs for why this
    /// is carried rather than refused.
    Unknown(i32),
}

impl OrderStatus {
    pub fn from_id(id: i32) -> Self {
        match id {
            1 => Self::Received,
            2 => Self::Placed,
            3 => Self::Filled,
            4 => Self::Rejected,
            5 => Self::PartiallyFilled,
            6 => Self::PendingCancel,
            7 => Self::Canceled,
            8 => Self::Expired,
            9 => Self::CanceledPartiallyFilled,
            10 => Self::RejectedPartiallyFilled,
            11 => Self::WaitingForMarket,
            12 => Self::PendingTriggeredRate,
            other => Self::Unknown(other),
        }
    }

    pub fn id(self) -> i32 {
        match self {
            Self::Received => 1,
            Self::Placed => 2,
            Self::Filled => 3,
            Self::Rejected => 4,
            Self::PartiallyFilled => 5,
            Self::PendingCancel => 6,
            Self::Canceled => 7,
            Self::Expired => 8,
            Self::CanceledPartiallyFilled => 9,
            Self::RejectedPartiallyFilled => 10,
            Self::WaitingForMarket => 11,
            Self::PendingTriggeredRate => 12,
            Self::Unknown(id) => id,
        }
    }

    /// Whether the order has stopped moving, so polling can end.
    ///
    /// [`PartiallyFilled`] is deliberately **not** terminal, and it is the one
    /// genuinely uncertain case: it may mean "some filled, the rest was
    /// killed", which is terminal, or "still working", which is not. Polling a
    /// finished order once more costs one request against a 60/60s pool.
    /// Stopping early on a live one loses sight of the rest of a fill. The
    /// cheap mistake is the right one to make, and demo trading will settle
    /// which it actually is.
    ///
    /// [`Unknown`] is not terminal either, for the same reason: an unrecognised
    /// state is not evidence that the order has finished.
    ///
    /// [`PartiallyFilled`]: Self::PartiallyFilled
    /// [`Unknown`]: Self::Unknown
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Filled
                | Self::Rejected
                | Self::Canceled
                | Self::Expired
                | Self::CanceledPartiallyFilled
                | Self::RejectedPartiallyFilled
        )
    }

    /// Whether any quantity actually executed.
    ///
    /// Distinct from "did it succeed": a cancelled-partially-filled order both
    /// failed *and* left a position open, and code that treats it as a plain
    /// failure will leave that position unmanaged.
    pub fn filled_any(self) -> bool {
        matches!(
            self,
            Self::Filled
                | Self::PartiallyFilled
                | Self::CanceledPartiallyFilled
                | Self::RejectedPartiallyFilled
        )
    }

    /// Whether the order was refused outright, with nothing executed.
    pub fn is_rejected(self) -> bool {
        matches!(self, Self::Rejected)
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Received => "Received",
            Self::Placed => "Placed",
            Self::Filled => "Filled",
            Self::Rejected => "Rejected",
            Self::PartiallyFilled => "PartiallyFilled",
            Self::PendingCancel => "PendingCancel",
            Self::Canceled => "Canceled",
            Self::Expired => "Expired",
            Self::CanceledPartiallyFilled => "CanceledPartiallyFilled",
            Self::RejectedPartiallyFilled => "RejectedPartiallyFilled",
            Self::WaitingForMarket => "WaitingForMarket",
            Self::PendingTriggeredRate => "PendingTriggeredRate",
            Self::Unknown(_) => "Unknown",
        }
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(id) => write!(f, "Unknown({id})"),
            known => write!(f, "{} ({})", known.name(), known.id()),
        }
    }
}

/// How to ask the API about an order.
///
/// An enum rather than two optional arguments because the two are documented as
/// mutually exclusive, and this makes "both" and "neither" unrepresentable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderHandle {
    /// The numeric id returned by a successful submission.
    OrderId(i64),
    /// The `x-request-id` sent *with* the submission.
    ///
    /// The handle that matters for recovery: it exists before the request is
    /// sent, so it is the only way to find out what happened to an order whose
    /// response was lost to a timeout or a dropped connection.
    ReferenceId(uuid::Uuid),
}

/// An order eToro has accepted, and the handle for asking what became of it.
///
/// Separate from the wire response because that type makes every field
/// optional, including both identifiers. This one guarantees a usable handle
/// exists — `reference_id` is the value *we* generated before sending, so it is
/// present even if the response body was empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedOrder {
    /// The numeric id, when the response carried one.
    pub order_id: Option<i64>,
    /// The `x-request-id` sent with the submission.
    pub reference_id: uuid::Uuid,
    /// eToro's correlation token, for support requests.
    pub token: Option<String>,
}

impl AcceptedOrder {
    /// How to look this order up: the numeric id when there is one, otherwise
    /// the reference we chose.
    ///
    /// Preferring `order_id` because it is the API's own identity for the
    /// order; falling back to the reference is what makes an accepted order
    /// findable even when the submission response was lost or empty.
    pub fn handle(&self) -> OrderHandle {
        match self.order_id {
            Some(id) => OrderHandle::OrderId(id),
            None => OrderHandle::ReferenceId(self.reference_id),
        }
    }
}

/// The status of a looked-up order, or `None` when the API omitted it.
///
/// A free function rather than a method because `GetOrderInfoResponse` is
/// generated and regenerated from the spec snapshot.
pub fn status_of(
    info: &crate::types::tags::trading_real::GetOrderInfoResponse,
) -> Option<OrderStatus> {
    info.status.as_ref()?.id.map(OrderStatus::from_id)
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum OrderError {
    #[error("{field} must be greater than zero, got {value}")]
    NotPositive { field: &'static str, value: String },

    #[error("{field} cannot be negative, got {value}")]
    Negative { field: &'static str, value: String },

    #[error("an instrument id must be positive, got {0}")]
    InvalidInstrument(i64),

    /// Refused before the id ever reaches a URL path. It is an integer, so it
    /// cannot escape the path the way a string could -- but a non-positive id
    /// is not a position, and sending one asks the API a meaningless question.
    #[error("a position id must be positive, got {0}")]
    InvalidPosition(i64),
}

/// A market order that opens a long position, sized in cash.
///
/// Deliberately the only order this crate can express so far, and the choice is
/// not arbitrary:
///
/// - `action` may be `open` or `close` in the schema, but *"currently only
///   `open` is supported in this endpoint"*. Closing goes through
///   `POST /api/v1/trading/execution/market-close-orders/positions/{id}`.
/// - `transaction` accepts four values, of which *"only `buy` and `sellShort`
///   are supported; `sell` and `buyToCover` are rejected"*. The strategies here
///   are long-only, so `buy`.
/// - Sizing by `amount` rather than `units`, because a $100–500 account thinks
///   in cash and eToro supports fractional shares.
/// - `settlementType` is left unset, which v2 permits. On v3 it *must* be set
///   for everything except `mit` orders, and a wrong value is *"rejected during
///   execution after the order has already been accepted with an order id"* —
///   a failure mode worth avoiding until eligibility data is being read.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketBuy {
    action: &'static str,
    transaction: &'static str,
    #[serde(rename = "instrumentId")]
    instrument_id: i64,
    #[serde(rename = "orderType")]
    order_type: &'static str,
    amount: Numeric,
    #[serde(rename = "orderCurrency")]
    order_currency: &'static str,
    #[serde(rename = "stopLossRate", skip_serializing_if = "Option::is_none")]
    stop_loss_rate: Option<Numeric>,
    #[serde(rename = "takeProfitRate", skip_serializing_if = "Option::is_none")]
    take_profit_rate: Option<Numeric>,
}

impl MarketBuy {
    /// Invest `amount` in `instrument_id` at the market price.
    pub fn new(instrument_id: i64, amount: Numeric) -> Result<Self, OrderError> {
        if instrument_id <= 0 {
            // Negative ids are the search endpoint's pseudo-instruments, which
            // are aggregate statistics and not tradeable.
            return Err(OrderError::InvalidInstrument(instrument_id));
        }
        if amount.0 <= rust_decimal::Decimal::ZERO {
            return Err(OrderError::NotPositive {
                field: "amount",
                value: amount.0.to_string(),
            });
        }
        Ok(Self {
            action: "open",
            transaction: "buy",
            instrument_id,
            order_type: "mkt",
            amount,
            // "Only USD is currently supported."
            order_currency: "usd",
            stop_loss_rate: None,
            take_profit_rate: None,
        })
    }

    /// Attaches a stop-loss rate, at which the position closes automatically.
    pub fn with_stop_loss(mut self, rate: Numeric) -> Result<Self, OrderError> {
        Self::check_non_negative("stopLossRate", rate)?;
        self.stop_loss_rate = Some(rate);
        Ok(self)
    }

    /// Attaches a take-profit rate.
    pub fn with_take_profit(mut self, rate: Numeric) -> Result<Self, OrderError> {
        Self::check_non_negative("takeProfitRate", rate)?;
        self.take_profit_rate = Some(rate);
        Ok(self)
    }

    pub fn instrument_id(&self) -> i64 {
        self.instrument_id
    }

    pub fn amount(&self) -> Numeric {
        self.amount
    }

    fn check_non_negative(field: &'static str, rate: Numeric) -> Result<(), OrderError> {
        if rate.0 < rust_decimal::Decimal::ZERO {
            return Err(OrderError::Negative {
                field,
                value: rate.0.to_string(),
            });
        }
        Ok(())
    }
}

/// An order the API can price or place.
///
/// Deliberately small: enough for [`order_cost`] and the audit log to describe
/// an order without knowing which kind it is, and not enough to build one.
///
/// [`order_cost`]: crate::client::EtoroClient::order_cost
pub trait OrderRequest: Serialize {
    fn instrument_id(&self) -> i64;
    fn amount(&self) -> Numeric;
    /// A one-line description, for prompts and audit entries.
    fn describe(&self) -> String;
}

impl OrderRequest for MarketBuy {
    fn instrument_id(&self) -> i64 {
        self.instrument_id
    }

    fn amount(&self) -> Numeric {
        self.amount
    }

    fn describe(&self) -> String {
        format!(
            "OPEN LONG instrument {} for ${}",
            self.instrument_id, self.amount.0
        )
    }
}

/// A market order that opens a **short** position, sized in cash.
///
/// Separate from [`MarketBuy`] rather than a direction field on it, because the
/// two are not the same shape: `stopLossRate` is documented as *"Required …
/// when transaction is sellShort"*, so a short without one is not an order the
/// API will accept. Making it a constructor argument rather than an `Option`
/// puts that in the type instead of in a validation message.
///
/// The asymmetry is real rather than bureaucratic. A long's worst case is
/// −100%; a short's loss is unbounded, which is why the stop is not optional.
///
/// **`settlementType` is left unset**, which v2 permits. Which value a short
/// actually needs comes from `POST /api/v2/trading/info/eligibility` →
/// `leverageConfigs[].settlementType`, since the spec says valid values differ
/// *"per instrument, direction and leverage"*. Until that is read, this can be
/// **priced** but is deliberately not placeable — see
/// [`place_order`](crate::client::EtoroClient::place_order).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketShort {
    action: &'static str,
    transaction: &'static str,
    #[serde(rename = "instrumentId")]
    instrument_id: i64,
    #[serde(rename = "orderType")]
    order_type: &'static str,
    amount: Numeric,
    #[serde(rename = "orderCurrency")]
    order_currency: &'static str,
    /// Not an `Option`: the API requires it for a short.
    #[serde(rename = "stopLossRate")]
    stop_loss_rate: Numeric,
    #[serde(rename = "takeProfitRate", skip_serializing_if = "Option::is_none")]
    take_profit_rate: Option<Numeric>,
}

impl MarketShort {
    /// Short `instrument_id` with `amount`, stopping out at `stop_loss_rate`.
    ///
    /// The stop must sit *above* the entry price for a short, which cannot be
    /// checked here -- the entry price is not known until the order executes.
    /// The API validates the relationship; this validates that a price was
    /// given at all and that it is one.
    pub fn new(
        instrument_id: i64,
        amount: Numeric,
        stop_loss_rate: Numeric,
    ) -> Result<Self, OrderError> {
        if instrument_id <= 0 {
            return Err(OrderError::InvalidInstrument(instrument_id));
        }
        if amount.0 <= rust_decimal::Decimal::ZERO {
            return Err(OrderError::NotPositive {
                field: "amount",
                value: amount.0.to_string(),
            });
        }
        if stop_loss_rate.0 <= rust_decimal::Decimal::ZERO {
            return Err(OrderError::NotPositive {
                field: "stopLossRate",
                value: stop_loss_rate.0.to_string(),
            });
        }
        Ok(Self {
            action: "open",
            transaction: "sellShort",
            instrument_id,
            order_type: "mkt",
            amount,
            order_currency: "usd",
            stop_loss_rate,
            take_profit_rate: None,
        })
    }

    pub fn with_take_profit(mut self, rate: Numeric) -> Result<Self, OrderError> {
        if rate.0 < rust_decimal::Decimal::ZERO {
            return Err(OrderError::Negative {
                field: "takeProfitRate",
                value: rate.0.to_string(),
            });
        }
        self.take_profit_rate = Some(rate);
        Ok(self)
    }

    pub fn stop_loss_rate(&self) -> Numeric {
        self.stop_loss_rate
    }
}

impl OrderRequest for MarketShort {
    fn instrument_id(&self) -> i64 {
        self.instrument_id
    }

    fn amount(&self) -> Numeric {
        self.amount
    }

    fn describe(&self) -> String {
        format!(
            "OPEN SHORT instrument {} for ${}, stop {}",
            self.instrument_id, self.amount.0, self.stop_loss_rate.0
        )
    }
}

/// A request to close all or part of an open position.
///
/// A separate type from [`MarketBuy`] because it is a separate *endpoint*, not
/// merely a separate direction: the unified order endpoint rejects `sell` and
/// `buyToCover` today, so closing goes through
/// `POST /api/v1/trading/execution/{demo/}market-close-orders/positions/{id}`.
/// A long-only strategy therefore needs two paths for its two directions,
/// which is worth knowing before writing the loop that drives them.
#[derive(Debug, Clone, PartialEq)]
pub struct ClosePosition {
    position_id: i64,
    instrument_id: i32,
    units: Option<Numeric>,
}

/// The body eToro expects, which is **PascalCase** unlike every other request
/// in this API. Kept private so the casing cannot be got wrong at a call site.
#[derive(Debug, Serialize)]
struct ClosePositionBody {
    #[serde(rename = "InstrumentID")]
    instrument_id: i32,
    /// Absent means close the whole position, which is not the same as zero.
    #[serde(rename = "UnitsToDeduct", skip_serializing_if = "Option::is_none")]
    units_to_deduct: Option<Numeric>,
}

impl ClosePosition {
    /// Closes the entire position.
    pub fn all(position_id: i64, instrument_id: i32) -> Result<Self, OrderError> {
        if position_id <= 0 {
            return Err(OrderError::InvalidPosition(position_id));
        }
        if instrument_id <= 0 {
            return Err(OrderError::InvalidInstrument(i64::from(instrument_id)));
        }
        Ok(Self {
            position_id,
            instrument_id,
            units: None,
        })
    }

    /// Closes `units` of the position, leaving the rest open.
    pub fn units(position_id: i64, instrument_id: i32, units: Numeric) -> Result<Self, OrderError> {
        if units.0 <= rust_decimal::Decimal::ZERO {
            return Err(OrderError::NotPositive {
                field: "UnitsToDeduct",
                value: units.0.to_string(),
            });
        }
        Ok(Self {
            units: Some(units),
            ..Self::all(position_id, instrument_id)?
        })
    }

    pub fn position_id(&self) -> i64 {
        self.position_id
    }

    /// Whether this closes the position outright.
    pub fn is_full_close(&self) -> bool {
        self.units.is_none()
    }

    pub(crate) fn body(&self) -> impl Serialize + use<> {
        ClosePositionBody {
            instrument_id: self.instrument_id,
            units_to_deduct: self.units,
        }
    }
}

/// What came back from a close request.
///
/// Hand-written rather than generated: the response is an inline anonymous
/// object with no component schema, and its casing (`positionID`, `orderID`,
/// `CID`) does not match the similarly-named `OrderForClose` component.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ClosedOrder {
    #[serde(rename = "orderID")]
    pub order_id: Option<i64>,
    #[serde(rename = "positionID")]
    pub position_id: Option<i64>,
    #[serde(rename = "instrumentID")]
    pub instrument_id: Option<i32>,
    #[serde(rename = "unitsToDeduct")]
    pub units_to_deduct: Option<Numeric>,
    #[serde(rename = "statusID")]
    pub status_id: Option<i32>,
}

impl ClosedOrder {
    /// The closing order's status, on the same scale as [`OrderStatus`].
    ///
    /// A close is asynchronous exactly like an open -- eToro's own example
    /// returns `statusID: 1` (Received) -- so the same polling applies.
    pub fn status(&self) -> Option<OrderStatus> {
        self.status_id.map(OrderStatus::from_id)
    }
}

/// The envelope a close request returns.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CloseAccepted {
    #[serde(rename = "orderForClose")]
    pub order: Option<ClosedOrder>,
    pub token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn numeric(text: &str) -> Numeric {
        Numeric(text.parse().unwrap())
    }

    #[test]
    fn every_documented_status_id_round_trips() {
        for id in 1..=12 {
            let status = OrderStatus::from_id(id);
            assert_ne!(status, OrderStatus::Unknown(id), "id {id} is documented");
            assert_eq!(status.id(), id);
        }
    }

    #[test]
    fn an_unknown_status_is_carried_rather_than_refused() {
        // The whole point: by the time a status is read, the order exists. A
        // status this crate cannot name must not make the order invisible.
        let status = OrderStatus::from_id(99);
        assert_eq!(status, OrderStatus::Unknown(99));
        assert_eq!(status.id(), 99);
        assert_eq!(status.to_string(), "Unknown(99)");
        // And it must not be mistaken for a finished order.
        assert!(!status.is_terminal());
        assert!(!status.filled_any());
    }

    #[test]
    fn polling_stops_only_on_states_that_cannot_change() {
        for terminal in [
            OrderStatus::Filled,
            OrderStatus::Rejected,
            OrderStatus::Canceled,
            OrderStatus::Expired,
            OrderStatus::CanceledPartiallyFilled,
            OrderStatus::RejectedPartiallyFilled,
        ] {
            assert!(terminal.is_terminal(), "{terminal}");
        }
        for in_flight in [
            OrderStatus::Received,
            OrderStatus::Placed,
            OrderStatus::PendingCancel,
            OrderStatus::WaitingForMarket,
            OrderStatus::PendingTriggeredRate,
            // Deliberately non-terminal; see is_terminal's documentation.
            OrderStatus::PartiallyFilled,
        ] {
            assert!(!in_flight.is_terminal(), "{in_flight}");
        }
    }

    #[test]
    fn a_failed_order_can_still_have_left_a_position_open() {
        // The trap this method exists for: treating these as plain failures
        // leaves real units unmanaged.
        assert!(OrderStatus::CanceledPartiallyFilled.filled_any());
        assert!(OrderStatus::RejectedPartiallyFilled.filled_any());
        assert!(OrderStatus::Filled.filled_any());
        assert!(OrderStatus::PartiallyFilled.filled_any());

        assert!(!OrderStatus::Rejected.filled_any());
        assert!(!OrderStatus::Canceled.filled_any());
        assert!(!OrderStatus::Expired.filled_any());
        assert!(!OrderStatus::Placed.filled_any());

        // And "rejected" means nothing executed, unlike the partial variant.
        assert!(OrderStatus::Rejected.is_rejected());
        assert!(!OrderStatus::RejectedPartiallyFilled.is_rejected());
    }

    #[test]
    fn a_market_buy_serialises_to_the_documented_shape() {
        let order = MarketBuy::new(1001, numeric("100.50")).unwrap();
        let json = serde_json::to_value(&order).unwrap();
        // Compared against a parsed *string*, not the `json!` macro: with
        // `arbitrary_precision` the macro's `100.50` literal normalises to
        // `100.5`, while `Numeric` preserves the scale it was given. The
        // difference is the whole reason `Numeric` exists.
        let expected: serde_json::Value = serde_json::from_str(
            r#"{"action":"open","transaction":"buy","instrumentId":1001,
                "orderType":"mkt","amount":100.50,"orderCurrency":"usd"}"#,
        )
        .unwrap();
        assert_eq!(json, expected);
        // Absent rather than null: the schema rejects several fields when they
        // are supplied at all, so an explicit null is not the same as omitting.
        assert!(!json.to_string().contains("stopLossRate"));
    }

    #[test]
    fn the_amount_keeps_its_exact_decimal_scale_on_the_wire() {
        // Money must never round-trip through f64. `100.10` as a float is
        // 100.099999999999994315658113919198513031005859375.
        let order = MarketBuy::new(1001, numeric("100.10")).unwrap();
        assert!(
            serde_json::to_string(&order)
                .unwrap()
                .contains("\"amount\":100.10"),
            "{}",
            serde_json::to_string(&order).unwrap()
        );
    }

    #[test]
    fn risk_levels_are_attached_only_when_asked_for() {
        let order = MarketBuy::new(1001, numeric("100"))
            .unwrap()
            .with_stop_loss(numeric("95.5"))
            .unwrap()
            .with_take_profit(numeric("120"))
            .unwrap();
        let json = serde_json::to_string(&order).unwrap();
        assert!(json.contains("\"stopLossRate\":95.5"), "{json}");
        assert!(json.contains("\"takeProfitRate\":120"), "{json}");
    }

    #[test]
    fn an_order_that_could_not_execute_is_refused_before_sending() {
        assert_eq!(
            MarketBuy::new(1001, numeric("0")),
            Err(OrderError::NotPositive {
                field: "amount",
                value: "0".to_owned()
            })
        );
        assert!(MarketBuy::new(1001, numeric("-5")).is_err());

        // Negative ids are the search endpoint's aggregate pseudo-instruments.
        assert_eq!(
            MarketBuy::new(-100000, numeric("100")),
            Err(OrderError::InvalidInstrument(-100000))
        );
        assert!(MarketBuy::new(0, numeric("100")).is_err());

        let valid = MarketBuy::new(1001, numeric("100")).unwrap();
        assert!(valid.clone().with_stop_loss(numeric("-1")).is_err());
        assert!(valid.with_take_profit(numeric("-1")).is_err());
    }

    #[test]
    fn a_short_cannot_be_built_without_a_stop() {
        // Not a validation message but a signature: the API documents
        // stopLossRate as required for sellShort, and a short's loss is
        // unbounded, so there is no sensible default to fall back to.
        let short = MarketShort::new(1001, numeric("100"), numeric("120")).unwrap();
        assert_eq!(short.stop_loss_rate(), numeric("120"));

        assert!(MarketShort::new(1001, numeric("100"), numeric("0")).is_err());
        assert!(MarketShort::new(1001, numeric("100"), numeric("-5")).is_err());
        assert!(MarketShort::new(1001, numeric("0"), numeric("120")).is_err());
        assert!(MarketShort::new(0, numeric("100"), numeric("120")).is_err());
    }

    #[test]
    fn a_short_serialises_as_sell_short_with_its_stop() {
        let short = MarketShort::new(1001, numeric("2500"), numeric("351.67")).unwrap();
        let expected: serde_json::Value = serde_json::from_str(
            r#"{"action":"open","transaction":"sellShort","instrumentId":1001,
                "orderType":"mkt","amount":2500,"orderCurrency":"usd",
                "stopLossRate":351.67}"#,
        )
        .unwrap();
        assert_eq!(serde_json::to_value(&short).unwrap(), expected);
        // settlementType is deliberately absent: v2 allows that, and which
        // value a short needs comes from the eligibility endpoint.
        assert!(
            !serde_json::to_string(&short)
                .unwrap()
                .contains("settlement")
        );
    }

    #[test]
    fn both_order_kinds_describe_their_direction() {
        // The description reaches an approval prompt and an audit entry, so
        // "long" and "short" have to be visible in it.
        let long = MarketBuy::new(1001, numeric("100")).unwrap();
        let short = MarketShort::new(1001, numeric("100"), numeric("120")).unwrap();
        assert!(long.describe().contains("LONG"), "{}", long.describe());
        assert!(short.describe().contains("SHORT"), "{}", short.describe());
        assert!(
            short.describe().contains("stop 120"),
            "{}",
            short.describe()
        );
        assert_eq!(long.amount(), short.amount());
        assert_eq!(
            OrderRequest::instrument_id(&long),
            OrderRequest::instrument_id(&short)
        );
    }

    #[test]
    fn a_full_close_omits_the_unit_count_rather_than_sending_zero() {
        let close = ClosePosition::all(2150941015, 1111).unwrap();
        assert!(close.is_full_close());
        let json = serde_json::to_string(&close.body()).unwrap();
        // Absent means "all of it"; zero would mean "close nothing".
        assert_eq!(json, r#"{"InstrumentID":1111}"#);
    }

    #[test]
    fn a_partial_close_sends_the_pascal_case_keys_this_endpoint_wants() {
        let close = ClosePosition::units(2150941015, 1111, numeric("2.5")).unwrap();
        assert!(!close.is_full_close());
        assert_eq!(
            serde_json::to_string(&close.body()).unwrap(),
            r#"{"InstrumentID":1111,"UnitsToDeduct":2.5}"#
        );
    }

    #[test]
    fn a_close_that_could_not_refer_to_anything_is_refused() {
        assert_eq!(
            ClosePosition::all(0, 1111),
            Err(OrderError::InvalidPosition(0))
        );
        assert!(ClosePosition::all(-1, 1111).is_err());
        assert!(ClosePosition::all(1, 0).is_err());
        // Zero units would serialise as a close that closes nothing.
        assert!(ClosePosition::units(1, 1111, numeric("0")).is_err());
        assert!(ClosePosition::units(1, 1111, numeric("-2")).is_err());
    }

    #[test]
    fn a_close_response_decodes_the_apis_own_example() {
        // Copied from the endpoint's documented example, casing included.
        let accepted: CloseAccepted = serde_json::from_str(
            r#"{"orderForClose":{"positionID":2150941015,"instrumentID":1111,
                "unitsToDeduct":2,"orderID":13904638,"orderType":19,"statusID":1,
                "CID":7765437,"openDateTime":"2025-04-02T16:07:54.0880338Z",
                "lastUpdate":"2025-04-02T16:07:54.0880338Z"},
                "token":"5fe065bc-f6f9-4897-a2ce-c4fccef73ff8"}"#,
        )
        .unwrap();
        let order = accepted.order.unwrap();
        assert_eq!(order.order_id, Some(13904638));
        assert_eq!(order.position_id, Some(2150941015));
        // Received, not Filled: a close is asynchronous like an open, so this
        // still has to be polled.
        assert_eq!(order.status(), Some(OrderStatus::Received));
        assert!(!order.status().unwrap().is_terminal());
    }

    #[test]
    fn a_handle_cannot_name_both_identifiers_at_once() {
        // The API documents orderId and referenceId as mutually exclusive; the
        // type is what makes "both" and "neither" impossible to send.
        let by_id = OrderHandle::OrderId(42);
        let by_reference = OrderHandle::ReferenceId(uuid::Uuid::nil());
        assert_ne!(by_id, by_reference);
    }
}
