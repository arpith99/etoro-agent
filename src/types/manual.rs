//! Hand-written types that override typify-generated ones via `x-rust-type`.
//!
//! These exist because cargo typify can't emit what we need:
//!
//! - **Integer-encoded enums.** The eToro spec declares enum types as
//!   `type: integer` but lists string-valued variants. The wire format is
//!   actually the integer index. Some endpoints (e.g. watchlists) are observed
//!   to return the string name instead. We deserialize either form and always
//!   serialize as the integer.
//! - **Exact JSON numbers.** Monetary values use `Decimal` and serde_json's
//!   arbitrary-precision number representation, avoiding an intermediate f64.
//!   "Exact" is bounded by `Decimal` itself: 28 significant digits (more are
//!   rounded, half-up) and a magnitude below 2^96 ≈ 7.9e28 (larger values are
//!   a deserialization error, which fails the whole response).
//!
//! Wired in by adding `x-rust-type` to the corresponding schema in
//! `docs/*-schema.json`; the regen pipeline passes the matching `--crate` flag
//! to cargo typify so the generated code references these types directly
//! instead of emitting its own.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

// ---------------------------------------------------------------------------
// Numeric — a Decimal newtype with exact JSON-number serde
// ---------------------------------------------------------------------------

/// Drop-in replacement for `f32`/`f64` in fields that represent monetary amounts,
/// rates, units, leverage, percentages, etc. — anywhere arithmetic precision
/// matters. Wraps [`rust_decimal::Decimal`] but (de)serializes as a JSON number
/// to match the eToro wire format, instead of Decimal's default string round-trip.
/// The arbitrary-precision serde adapter preserves the JSON number's decimal
/// representation without converting it through `f64`, including scientific
/// notation such as `5.06e-6`.
///
/// Limits (inherited from `Decimal`): more than 28 significant digits are
/// rounded, and a magnitude of 2^96 (≈ 7.9e28) or above is rejected. Both are
/// far outside realistic account and price ranges, but a rejected value fails
/// deserialization of the *entire* response, so keep the limit in mind for
/// any analytics-style field (market caps, volumes) that may be added later.
///
/// Wired in by the preprocessor: every JSON Schema with `type: "number"` gets
/// an `x-rust-type` pointing here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Numeric(pub rust_decimal::Decimal);

impl Serialize for Numeric {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        rust_decimal::serde::arbitrary_precision::serialize(&self.0, s)
    }
}

impl<'de> Deserialize<'de> for Numeric {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        rust_decimal::serde::arbitrary_precision::deserialize(d).map(Numeric)
    }
}

impl From<Numeric> for rust_decimal::Decimal {
    fn from(n: Numeric) -> Self {
        n.0
    }
}

impl From<rust_decimal::Decimal> for Numeric {
    fn from(d: rust_decimal::Decimal) -> Self {
        Numeric(d)
    }
}

impl std::ops::Deref for Numeric {
    type Target = rust_decimal::Decimal;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Numeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

/// Defines an integer-encoded enum that can deserialize from either the integer
/// index (per spec) or the string variant name (observed in some responses),
/// and always serializes as the integer.
macro_rules! int_or_string_enum {
    (
        $(#[$enum_attr:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident = $int:expr => $str:literal),+ $(,)?
        }
    ) => {
        $(#[$enum_attr])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(i32)]
        $vis enum $name {
            $($variant = $int,)+
        }

        impl $name {
            #[doc = "Variant string name (matches the spec's enum entries)."]
            pub fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $str,)+
                }
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_i32(*self as i32)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                #[derive(Deserialize)]
                #[serde(untagged)]
                enum Either<'a> {
                    Int(i64),
                    Str(::std::borrow::Cow<'a, str>),
                }
                match Either::deserialize(d)? {
                    $(Either::Int(n) if n == $int as i64 => Ok(Self::$variant),)+
                    $(Either::Str(s) if s == $str => Ok(Self::$variant),)+
                    Either::Int(n) => Err(<D::Error as ::serde::de::Error>::custom(
                        format!(concat!("invalid integer for ", stringify!($name), ": {}"), n)
                    )),
                    Either::Str(s) => Err(<D::Error as ::serde::de::Error>::custom(
                        format!(concat!("invalid string for ", stringify!($name), ": {:?}"), s)
                    )),
                }
            }
        }
    };
}

// ---------------------------------------------------------------------------
// trading domain
// ---------------------------------------------------------------------------

int_or_string_enum! {
    pub enum TradeDirection {
        Long  = 0 => "Long",
        Short = 1 => "Short",
    }
}

int_or_string_enum! {
    pub enum TradeType {
        Open  = 0 => "Open",
        Close = 1 => "Close",
    }
}

// ---------------------------------------------------------------------------
// market_data domain
// ---------------------------------------------------------------------------

int_or_string_enum! {
    pub enum MarketAssetType {
        Stocks                     =  0 => "Stocks",
        Bonds                      =  1 => "Bonds",
        Etf                        =  2 => "ETF",
        Index                      =  3 => "Index",
        Warrants                   =  4 => "Warrants",
        Options                    =  5 => "Options",
        Futures                    =  6 => "Futures",
        Cfd                        =  7 => "CFD",
        Trs                        =  8 => "TRS",
        Forex                      =  9 => "FOREX",
        CommodityMetals            = 10 => "CommodityMetals",
        CommodityEnergyAgriculture = 11 => "CommodityEnergyAgriculture",
        CryptoCoin                 = 12 => "CryptoCoin",
        Nft                        = 13 => "NFT",
    }
}

int_or_string_enum! {
    pub enum ApplicationSource {
        EToro  = 0 => "eToro",
        Delta  = 1 => "Delta",
        Gatsby = 2 => "Gatsby",
    }
}

int_or_string_enum! {
    pub enum MarketEventTag {
        Reports      = 0 => "Reports",
        Dividends    = 1 => "Dividends",
        Split        = 2 => "Split",
        ReverseSplit = 3 => "ReverseSplit",
    }
}

// ---------------------------------------------------------------------------
// feeds_posts domain
// ---------------------------------------------------------------------------

int_or_string_enum! {
    pub enum PostType {
        Default     = 0 => "Default",
        Share       = 1 => "Share",
        MarketEvent = 2 => "MarketEvent",
        Trade       = 3 => "Trade",
        Order       = 4 => "Order",
        Copy        = 5 => "Copy",
        Poll        = 6 => "Poll",
        Article     = 7 => "Article",
    }
}

int_or_string_enum! {
    pub enum EmotionType {
        Like = 0 => "Like",
    }
}

int_or_string_enum! {
    pub enum ParentType {
        Unknown = 0 => "Unknown",
        Post    = 1 => "Post",
        Comment = 2 => "Comment",
        Reply   = 3 => "Reply",
    }
}

int_or_string_enum! {
    pub enum ReasonType {
        None            = 0 => "None",
        Owner           = 1 => "Owner",
        LikedPost       = 2 => "LikedPost",
        LikedComment    = 3 => "LikedComment",
        TaggedInPost    = 4 => "TaggedInPost",
        TaggedInComment = 5 => "TaggedInComment",
        Comment         = 6 => "Comment",
    }
}

int_or_string_enum! {
    pub enum MediaType {
        None  = 0 => "None",
        Link  = 1 => "Link",
        Image = 2 => "Image",
        Video = 3 => "Video",
    }
}

int_or_string_enum! {
    pub enum VideoSource {
        None    = 0 => "None",
        YouTube = 1 => "YouTube",
        Vimeo   = 2 => "Vimeo",
    }
}

int_or_string_enum! {
    pub enum ArticleRating {
        Bearish = 0 => "Bearish",
        Bullish = 1 => "Bullish",
    }
}

int_or_string_enum! {
    pub enum ArticleStatus {
        Draft     = 0 => "Draft",
        Published = 1 => "Published",
        Deleted   = 2 => "Deleted",
    }
}

int_or_string_enum! {
    pub enum EditStatus {
        None      = 0 => "None",
        Edited    = 1 => "Edited",
        Moderated = 2 => "Moderated",
    }
}

int_or_string_enum! {
    pub enum CopyType {
        Start = 0 => "Start",
        Stop  = 1 => "Stop",
    }
}

// ---------------------------------------------------------------------------
// identity domain
// ---------------------------------------------------------------------------

// Three well-formed integer enums (the spec lists `enum: [1, 2, ...]` with the
// names in `x-enumNames`). typify generates an awkward newtype-around-Option
// wrapper for these because the source schema is `nullable: true`, so we
// override with hand-written enums and let typify wrap them in Option at the
// field site.

int_or_string_enum! {
    pub enum PublicAggregatedInfoAccountStatus {
        Open   = 1 => "Open",
        Closed = 2 => "Closed",
    }
}

int_or_string_enum! {
    pub enum PublicAggregatedInfoPlayerStatus {
        Normal                       =  1 => "Normal",
        Blocked                      =  2 => "Blocked",
        ChatBlocked                  =  3 => "ChatBlocked",
        BlockedUponRequest           =  4 => "BlockedUponRequest",
        Warning                      =  5 => "Warning",
        BlockedUnderInvestigation    =  6 => "BlockedUnderInvestigation",
        ScalpersBlock                =  7 => "ScalpersBlock",
        BlockedPayPalInvestigation   =  8 => "BlockedPayPalInvestigation",
        TradeBlock                   =  9 => "TradeBlock",
        DepositBlocked               = 10 => "DepositBlocked",
        SocialIndex                  = 11 => "SocialIndex",
        CopyBlock                    = 12 => "CopyBlock",
        PendingVerification          = 13 => "PendingVerification",
        BlockedFailedVerification    = 14 => "BlockedFailedVerification",
        BlockTrading                 = 15 => "BlockTrading",
    }
}

int_or_string_enum! {
    pub enum PublicAggregatedInfoPlayerStatusReason {
        None                  =  0 => "None",
        FailedVerification    =  1 => "FailedVerification",
        ExpiredDocument       =  2 => "ExpiredDocument",
        CloseAccountByUser    =  3 => "CloseAccountByUser",
        Risk                  =  4 => "Risk",
        Chargeback            =  5 => "Chargeback",
        AmlAccountClosed      =  6 => "AMLAccountClosed",
        Hrc                   =  7 => "HRC",
        Underage              =  8 => "Underage",
        Deceased              =  9 => "Deceased",
        Aml                   = 10 => "AML",
        AmlReview             = 11 => "AMLreview",
        OffMarketAbuse        = 12 => "OffMarketAbuse",
        Overpayment           = 13 => "Overpayment",
        RiskCheck             = 14 => "RiskCheck",
        ThirdParty            = 15 => "ThirdParty",
        PayPalInvestigation   = 16 => "PayPalInvestigation",
        NocNofRfi             = 17 => "NOC_NOF_RFI",
        WchMatch              = 18 => "WCHMatch",
        Other                 = 19 => "Other",
        RightToBeForgotten    = 20 => "RightToBeForgotten",
        SelfService           = 21 => "SelfService",
        ByRequest             = 22 => "ByRequest",
        AchChargeback         = 23 => "ACHChargeback",
        PwmbChargeback        = 24 => "PWMBChargeback",
        Abuse                 = 25 => "Abuse",
        AffiliateAccount      = 26 => "AffiliateAccount",
        PendingDocs           = 27 => "PendingDocs",
        EmployeeAccount       = 28 => "EmployeeAccount",
        PiAccount             = 29 => "PIAccount",
        CheckoutChargeback    = 30 => "CheckoutChargeback",
        CheckoutRetrievel     = 31 => "CheckoutRetrievel",
        CheckoutCaptureDecline= 32 => "CheckoutCaptureDecline",
        EToroMoneyRestriction = 33 => "EToroMoneyRestriction",
        AbusiveTrading        = 34 => "AbusiveTrading",
        HackedAccount         = 35 => "HackedAccount",
        PartnersAndPis        = 36 => "PartnersAndPIs",
        CsManagementDecision  = 37 => "CS_ManagementDecision",
        Deposits              = 38 => "Deposits",
        Kyc                   = 39 => "KYC",
        AccountClosed         = 40 => "AccountClosed",
        Tax                   = 41 => "Tax",
        Corporate             = 42 => "Corporate",
    }
}

int_or_string_enum! {
    pub enum UserRole {
        Regular       =  0 => "Regular",
        Pi            =  1 => "PI",
        Moderator     =  2 => "Moderator",
        Anonymous     =  3 => "Anonymous",
        EToroTeam     =  4 => "eToroTeam",
        ETorian       =  5 => "eTorian",
        CopyPortfolio =  6 => "CopyPortfolio",
        Depositor     =  7 => "Depositor",
        Admin         =  8 => "Admin",
        Verified      =  9 => "Verified",
        Analyst       = 10 => "Analyst",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_from_integer() {
        let v: TradeDirection = serde_json::from_str("0").unwrap();
        assert_eq!(v, TradeDirection::Long);
        let v: TradeDirection = serde_json::from_str("1").unwrap();
        assert_eq!(v, TradeDirection::Short);
    }

    #[test]
    fn deserialize_from_string() {
        let v: TradeDirection = serde_json::from_str(r#""Long""#).unwrap();
        assert_eq!(v, TradeDirection::Long);
        let v: MarketAssetType = serde_json::from_str(r#""CryptoCoin""#).unwrap();
        assert_eq!(v, MarketAssetType::CryptoCoin);
    }

    #[test]
    fn serialize_as_integer() {
        let s = serde_json::to_string(&TradeDirection::Short).unwrap();
        assert_eq!(s, "1");
        let s = serde_json::to_string(&MarketAssetType::CryptoCoin).unwrap();
        assert_eq!(s, "12");
    }

    #[test]
    fn numeric_round_trip() {
        let n: Numeric = serde_json::from_str("3.14").unwrap();
        assert_eq!(n.to_string(), "3.14");
        let s = serde_json::to_string(&n).unwrap();
        assert_eq!(s, "3.14");
    }

    #[test]
    fn numeric_handles_integer_input() {
        let n: Numeric = serde_json::from_str("42").unwrap();
        assert_eq!(n.to_string(), "42");
    }

    #[test]
    fn numeric_preserves_more_precision_than_f64() {
        const PRECISE: &str = "0.1234567890123456789012345678";
        let n: Numeric = serde_json::from_str(PRECISE).unwrap();
        assert_eq!(n.to_string(), PRECISE);
        assert_eq!(serde_json::to_string(&n).unwrap(), PRECISE);
    }

    #[test]
    fn numeric_accepts_scientific_notation() {
        let n: Numeric = serde_json::from_str("5.06e-6").unwrap();
        assert_eq!(n.to_string(), "0.00000506");
    }

    // Documents the edge of "exact": Decimal holds 28 significant digits, so a
    // 29th digit is rounded rather than preserved or rejected. If this test
    // ever fails, the precision contract in the module docs needs revisiting.
    #[test]
    fn numeric_rounds_beyond_28_significant_digits() {
        let n: Numeric = serde_json::from_str("0.12345678901234567890123456789").unwrap();
        assert_eq!(n.to_string(), "0.1234567890123456789012345679");
    }

    // Values at or above 2^96 do not fit in Decimal's mantissa. This must be a
    // clean error (which aborts the whole response), never a silent wrap.
    #[test]
    fn numeric_rejects_out_of_range_magnitude() {
        assert!(serde_json::from_str::<Numeric>("1e30").is_err());
        assert!(serde_json::from_str::<Numeric>("123456789012345678901234567890").is_err());
    }
    #[test]
    fn invalid_value_is_descriptive_error() {
        let err = serde_json::from_str::<TradeDirection>("99").unwrap_err();
        assert!(
            err.to_string()
                .contains("invalid integer for TradeDirection")
        );
        let err = serde_json::from_str::<TradeDirection>(r#""Sideways""#).unwrap_err();
        assert!(
            err.to_string()
                .contains("invalid string for TradeDirection")
        );
    }
}
