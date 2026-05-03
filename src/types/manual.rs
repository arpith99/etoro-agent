//! Hand-written types that override typify-generated ones via `x-rust-type`.
//!
//! These exist because cargo typify can't emit what we need:
//!
//! - **Integer-encoded enums.** The eToro spec declares enum types as
//!   `type: integer` but lists string-valued variants. The wire format is
//!   actually the integer index. Some endpoints (e.g. watchlists) are observed
//!   to return the string name instead. We deserialize either form and always
//!   serialize as the integer.
//!
//! Wired in by adding `x-rust-type` to the corresponding schema in
//! `docs/*-schema.json`; the regen pipeline passes the matching `--crate` flag
//! to cargo typify so the generated code references these types directly
//! instead of emitting its own.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
    fn invalid_value_is_descriptive_error() {
        let err = serde_json::from_str::<TradeDirection>("99").unwrap_err();
        assert!(err.to_string().contains("invalid integer for TradeDirection"));
        let err = serde_json::from_str::<TradeDirection>(r#""Sideways""#).unwrap_err();
        assert!(err.to_string().contains("invalid string for TradeDirection"));
    }
}
