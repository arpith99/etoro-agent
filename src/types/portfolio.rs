#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "`ClientPortfolio`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"accountCurrencyId\": {"]
#[doc = "      \"description\": \"Currency ID of the account (1 = USD)\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"bonusCredit\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"credit\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"entryOrders\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Order\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"exitOrders\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Order\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"mirrors\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Mirror\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"orders\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Order\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ordersForClose\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/OrderForClose\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ordersForCloseMultiple\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/OrderForCloseMultiple\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ordersForOpen\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/OrderForOpen\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"positions\": {"]
#[doc = "      \"description\": \"List of currently open trading positions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Position\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"stockOrders\": {"]
#[doc = "      \"description\": \"Stock-specific pending orders\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Order\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"unrealizedPnL\": {"]
#[doc = "      \"description\": \"Total unrealized profit and loss across all open positions in the portfolio\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClientPortfolio {
    #[doc = "Currency ID of the account (1 = USD)"]
    #[serde(
        rename = "accountCurrencyId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub account_currency_id: ::std::option::Option<i64>,
    #[serde(
        rename = "bonusCredit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bonus_credit: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub credit: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "entryOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub entry_orders: ::std::vec::Vec<::etoro_agent::types::trading::Order>,
    #[serde(
        rename = "exitOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exit_orders: ::std::vec::Vec<::etoro_agent::types::trading::Order>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub mirrors: ::std::vec::Vec<::etoro_agent::types::trading::Mirror>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub orders: ::std::vec::Vec<::etoro_agent::types::trading::Order>,
    #[serde(
        rename = "ordersForClose",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_close: ::std::vec::Vec<::etoro_agent::types::trading::OrderForClose>,
    #[serde(
        rename = "ordersForCloseMultiple",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_close_multiple:
        ::std::vec::Vec<::etoro_agent::types::trading::OrderForCloseMultiple>,
    #[serde(
        rename = "ordersForOpen",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_open: ::std::vec::Vec<::etoro_agent::types::trading::OrderForOpen>,
    #[doc = "List of currently open trading positions"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub positions: ::std::vec::Vec<::etoro_agent::types::trading::Position>,
    #[doc = "Stock-specific pending orders"]
    #[serde(
        rename = "stockOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub stock_orders: ::std::vec::Vec<::etoro_agent::types::trading::Order>,
    #[doc = "Total unrealized profit and loss across all open positions in the portfolio"]
    #[serde(
        rename = "unrealizedPnL",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub unrealized_pn_l: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
}
impl ::std::default::Default for ClientPortfolio {
    fn default() -> Self {
        Self {
            account_currency_id: Default::default(),
            bonus_credit: Default::default(),
            credit: Default::default(),
            entry_orders: Default::default(),
            exit_orders: Default::default(),
            mirrors: Default::default(),
            orders: Default::default(),
            orders_for_close: Default::default(),
            orders_for_close_multiple: Default::default(),
            orders_for_open: Default::default(),
            positions: Default::default(),
            stock_orders: Default::default(),
            unrealized_pn_l: Default::default(),
        }
    }
}
#[doc = "`GainEntry`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"gain\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GainEntry {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gain: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub timestamp: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
}
impl ::std::default::Default for GainEntry {
    fn default() -> Self {
        Self {
            gain: Default::default(),
            timestamp: Default::default(),
        }
    }
}
#[doc = "Response object containing a list of portfolio copiers."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Response object containing a list of portfolio copiers.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"copiers\": {"]
#[doc = "      \"description\": \"List of users copying your portfolio, with demographic and financial info.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"AgeCategory\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"Under 18\","]
#[doc = "              \"18-29\","]
#[doc = "              \"30-44\","]
#[doc = "              \"45-59\","]
#[doc = "              \"60+\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"AmountCategory\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"<100\","]
#[doc = "              \"100-500\","]
#[doc = "              \"500-1000\","]
#[doc = "              \"1000-5000\","]
#[doc = "              \">5000\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"AvailableCopyBalance\": {"]
#[doc = "            \"description\": \"Available copy balance (string-encoded number)\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"Club\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"example\": \"Gold\""]
#[doc = "          },"]
#[doc = "          \"CopyRealizedEquity_pnl\": {"]
#[doc = "            \"description\": \"Total realized equity PnL (string-encoded number)\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"CopyStartedAtCategory\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"less than 1 day\","]
#[doc = "              \"less than 1 week\","]
#[doc = "              \"less than 1 month\","]
#[doc = "              \"less than 1 year\","]
#[doc = "              \"more than 1 year\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"Country\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"example\": \"Germany\""]
#[doc = "          },"]
#[doc = "          \"Gender\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"example\": \"M\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GeCopiersResponse {
    #[doc = "List of users copying your portfolio, with demographic and financial info."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub copiers: ::std::option::Option<::std::vec::Vec<GeCopiersResponseCopiersItem>>,
}
impl ::std::default::Default for GeCopiersResponse {
    fn default() -> Self {
        Self {
            copiers: Default::default(),
        }
    }
}
#[doc = "`GeCopiersResponseCopiersItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"AgeCategory\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Under 18\","]
#[doc = "        \"18-29\","]
#[doc = "        \"30-44\","]
#[doc = "        \"45-59\","]
#[doc = "        \"60+\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"AmountCategory\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"<100\","]
#[doc = "        \"100-500\","]
#[doc = "        \"500-1000\","]
#[doc = "        \"1000-5000\","]
#[doc = "        \">5000\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"AvailableCopyBalance\": {"]
#[doc = "      \"description\": \"Available copy balance (string-encoded number)\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"Club\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Gold\""]
#[doc = "    },"]
#[doc = "    \"CopyRealizedEquity_pnl\": {"]
#[doc = "      \"description\": \"Total realized equity PnL (string-encoded number)\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"CopyStartedAtCategory\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"less than 1 day\","]
#[doc = "        \"less than 1 week\","]
#[doc = "        \"less than 1 month\","]
#[doc = "        \"less than 1 year\","]
#[doc = "        \"more than 1 year\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"Country\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Germany\""]
#[doc = "    },"]
#[doc = "    \"Gender\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"M\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GeCopiersResponseCopiersItem {
    #[serde(
        rename = "AgeCategory",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub age_category: ::std::option::Option<GeCopiersResponseCopiersItemAgeCategory>,
    #[serde(
        rename = "AmountCategory",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub amount_category: ::std::option::Option<GeCopiersResponseCopiersItemAmountCategory>,
    #[doc = "Available copy balance (string-encoded number)"]
    #[serde(
        rename = "AvailableCopyBalance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub available_copy_balance: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Club",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub club: ::std::option::Option<::std::string::String>,
    #[doc = "Total realized equity PnL (string-encoded number)"]
    #[serde(
        rename = "CopyRealizedEquity_pnl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub copy_realized_equity_pnl: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "CopyStartedAtCategory",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub copy_started_at_category:
        ::std::option::Option<GeCopiersResponseCopiersItemCopyStartedAtCategory>,
    #[serde(
        rename = "Country",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Gender",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub gender: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for GeCopiersResponseCopiersItem {
    fn default() -> Self {
        Self {
            age_category: Default::default(),
            amount_category: Default::default(),
            available_copy_balance: Default::default(),
            club: Default::default(),
            copy_realized_equity_pnl: Default::default(),
            copy_started_at_category: Default::default(),
            country: Default::default(),
            gender: Default::default(),
        }
    }
}
#[doc = "`GeCopiersResponseCopiersItemAgeCategory`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Under 18\","]
#[doc = "    \"18-29\","]
#[doc = "    \"30-44\","]
#[doc = "    \"45-59\","]
#[doc = "    \"60+\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GeCopiersResponseCopiersItemAgeCategory {
    #[serde(rename = "Under 18")]
    Under18,
    #[serde(rename = "18-29")]
    X1829,
    #[serde(rename = "30-44")]
    X3044,
    #[serde(rename = "45-59")]
    X4559,
    #[serde(rename = "60+")]
    X60,
}
impl ::std::fmt::Display for GeCopiersResponseCopiersItemAgeCategory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Under18 => f.write_str("Under 18"),
            Self::X1829 => f.write_str("18-29"),
            Self::X3044 => f.write_str("30-44"),
            Self::X4559 => f.write_str("45-59"),
            Self::X60 => f.write_str("60+"),
        }
    }
}
impl ::std::str::FromStr for GeCopiersResponseCopiersItemAgeCategory {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Under 18" => Ok(Self::Under18),
            "18-29" => Ok(Self::X1829),
            "30-44" => Ok(Self::X3044),
            "45-59" => Ok(Self::X4559),
            "60+" => Ok(Self::X60),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GeCopiersResponseCopiersItemAgeCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GeCopiersResponseCopiersItemAgeCategory {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GeCopiersResponseCopiersItemAgeCategory {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GeCopiersResponseCopiersItemAmountCategory`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"<100\","]
#[doc = "    \"100-500\","]
#[doc = "    \"500-1000\","]
#[doc = "    \"1000-5000\","]
#[doc = "    \">5000\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GeCopiersResponseCopiersItemAmountCategory {
    #[serde(rename = "<100")]
    X100,
    #[serde(rename = "100-500")]
    X100500,
    #[serde(rename = "500-1000")]
    X5001000,
    #[serde(rename = "1000-5000")]
    X10005000,
    #[serde(rename = ">5000")]
    X5000,
}
impl ::std::fmt::Display for GeCopiersResponseCopiersItemAmountCategory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X100 => f.write_str("<100"),
            Self::X100500 => f.write_str("100-500"),
            Self::X5001000 => f.write_str("500-1000"),
            Self::X10005000 => f.write_str("1000-5000"),
            Self::X5000 => f.write_str(">5000"),
        }
    }
}
impl ::std::str::FromStr for GeCopiersResponseCopiersItemAmountCategory {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "<100" => Ok(Self::X100),
            "100-500" => Ok(Self::X100500),
            "500-1000" => Ok(Self::X5001000),
            "1000-5000" => Ok(Self::X10005000),
            ">5000" => Ok(Self::X5000),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GeCopiersResponseCopiersItemAmountCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for GeCopiersResponseCopiersItemAmountCategory
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GeCopiersResponseCopiersItemAmountCategory {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GeCopiersResponseCopiersItemCopyStartedAtCategory`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"less than 1 day\","]
#[doc = "    \"less than 1 week\","]
#[doc = "    \"less than 1 month\","]
#[doc = "    \"less than 1 year\","]
#[doc = "    \"more than 1 year\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GeCopiersResponseCopiersItemCopyStartedAtCategory {
    #[serde(rename = "less than 1 day")]
    LessThan1Day,
    #[serde(rename = "less than 1 week")]
    LessThan1Week,
    #[serde(rename = "less than 1 month")]
    LessThan1Month,
    #[serde(rename = "less than 1 year")]
    LessThan1Year,
    #[serde(rename = "more than 1 year")]
    MoreThan1Year,
}
impl ::std::fmt::Display for GeCopiersResponseCopiersItemCopyStartedAtCategory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LessThan1Day => f.write_str("less than 1 day"),
            Self::LessThan1Week => f.write_str("less than 1 week"),
            Self::LessThan1Month => f.write_str("less than 1 month"),
            Self::LessThan1Year => f.write_str("less than 1 year"),
            Self::MoreThan1Year => f.write_str("more than 1 year"),
        }
    }
}
impl ::std::str::FromStr for GeCopiersResponseCopiersItemCopyStartedAtCategory {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "less than 1 day" => Ok(Self::LessThan1Day),
            "less than 1 week" => Ok(Self::LessThan1Week),
            "less than 1 month" => Ok(Self::LessThan1Month),
            "less than 1 year" => Ok(Self::LessThan1Year),
            "more than 1 year" => Ok(Self::MoreThan1Year),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GeCopiersResponseCopiersItemCopyStartedAtCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for GeCopiersResponseCopiersItemCopyStartedAtCategory
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for GeCopiersResponseCopiersItemCopyStartedAtCategory
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GetUserDailyGainResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/gainEntry\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"gain\": {"]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"x-rust-type\": {"]
#[doc = "            \"crate\": \"etoro-agent\","]
#[doc = "            \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "            \"version\": \"0.1.0\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum GetUserDailyGainResponse {
    Array(::std::vec::Vec<GainEntry>),
    Object {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        gain: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    },
}
impl ::std::convert::From<::std::vec::Vec<GainEntry>> for GetUserDailyGainResponse {
    fn from(value: ::std::vec::Vec<GainEntry>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`GetUserGainResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"monthly\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/gainEntry\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"yearly\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/gainEntry\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetUserGainResponse {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub monthly: ::std::vec::Vec<GainEntry>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub yearly: ::std::vec::Vec<GainEntry>,
}
impl ::std::default::Default for GetUserGainResponse {
    fn default() -> Self {
        Self {
            monthly: Default::default(),
            yearly: Default::default(),
        }
    }
}
#[doc = "Comprehensive portfolio information including positions, orders, and account status"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Comprehensive portfolio information including positions, orders, and account status\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"clientPortfolio\": {"]
#[doc = "      \"description\": \"Container for all portfolio-related information\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"bonusCredit\": {"]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"format\": \"float\","]
#[doc = "          \"x-rust-type\": {"]
#[doc = "            \"crate\": \"etoro-agent\","]
#[doc = "            \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "            \"version\": \"0.1.0\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"credit\": {"]
#[doc = "          \"description\": \"Available trading balance in USD, representing funds available for new positions\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"format\": \"float\","]
#[doc = "          \"x-rust-type\": {"]
#[doc = "            \"crate\": \"etoro-agent\","]
#[doc = "            \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "            \"version\": \"0.1.0\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"entryOrders\": {"]
#[doc = "          \"description\": \"Obsolete\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Order\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"exitOrders\": {"]
#[doc = "          \"description\": \"Obsolete\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Order\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"mirrors\": {"]
#[doc = "          \"description\": \"Copy trading configurations and positions\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"Individual mirror trading setup (inline shape; see Mirror schema in trading-schema.json for the standalone version)\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"properties\": {"]
#[doc = "              \"CID\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"availableAmount\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"closedPositionsNetProfit\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"copyExistingPositions\": {"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"delayedOrderForClose\": {"]
#[doc = "                \"description\": \"Obsolete\","]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"$ref\": \"#/$defs/OrderForClose\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"delayedOrderForOpen\": {"]
#[doc = "                \"description\": \"Obsolete\","]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"$ref\": \"#/$defs/OrderForOpen\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"depositSummary\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"entryOrders\": {"]
#[doc = "                \"description\": \"Obsolete\","]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Order\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"exitOrders\": {"]
#[doc = "                \"description\": \"Obsolete\","]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Order\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"initialInvestment\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"isPaused\": {"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"mirrorCalculationType\": {"]
#[doc = "                \"description\": \"(Obsolete) Mirror positions weights calculation methodology\","]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"mirrorID\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"mirrorStatusID\": {"]
#[doc = "                \"description\": \"0 - Active, 1 - Paused, 2 - Pending Closure, 3 - In Alignment Process\","]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"ordersForClose\": {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"$ref\": \"#/$defs/OrderForClose\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"ordersForCloseMultiple\": {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"$ref\": \"#/$defs/OrderForCloseMultiple\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"ordersForOpen\": {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"$ref\": \"#/$defs/OrderForOpen\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"parentCID\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"parentMirrors\": {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Mirror\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"parentUsername\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"pendingForClosure\": {"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"positions\": {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"$ref\": \"#/$defs/Position\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"startedCopyDate\": {"]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"format\": \"date-time\""]
#[doc = "              },"]
#[doc = "              \"stopLossAmount\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"stopLossPercentage\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"withdrawalSummary\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"orders\": {"]
#[doc = "          \"description\": \"List of pending orders\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"description\": \"Inline order shape — uses capital-ID field names (orderID, instrumentID, CID)\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"properties\": {"]
#[doc = "              \"CID\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"amount\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"executionType\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"instrumentID\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"isBuy\": {"]
#[doc = "                \"description\": \"true=Long, false=Short\","]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"isDiscounted\": {"]
#[doc = "                \"description\": \"Obsolete\","]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"isTslEnabled\": {"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"leverage\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"openDateTime\": {"]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"format\": \"date-time\""]
#[doc = "              },"]
#[doc = "              \"orderID\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"rate\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"stopLossRate\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"takeProfitRate\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"units\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"ordersForClose\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/OrderForClose\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"ordersForCloseMultiple\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/OrderForCloseMultiple\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"ordersForOpen\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/OrderForOpen\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"positions\": {"]
#[doc = "          \"description\": \"List of currently open trading positions\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Position\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"stockOrders\": {"]
#[doc = "          \"description\": \"Obsolete\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Order\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PortfolioResponse {
    #[serde(
        rename = "clientPortfolio",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_portfolio: ::std::option::Option<PortfolioResponseClientPortfolio>,
}
impl ::std::default::Default for PortfolioResponse {
    fn default() -> Self {
        Self {
            client_portfolio: Default::default(),
        }
    }
}
#[doc = "Container for all portfolio-related information"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Container for all portfolio-related information\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"bonusCredit\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"credit\": {"]
#[doc = "      \"description\": \"Available trading balance in USD, representing funds available for new positions\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"entryOrders\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Order\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"exitOrders\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Order\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"mirrors\": {"]
#[doc = "      \"description\": \"Copy trading configurations and positions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"Individual mirror trading setup (inline shape; see Mirror schema in trading-schema.json for the standalone version)\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"CID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"availableAmount\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"closedPositionsNetProfit\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"copyExistingPositions\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"delayedOrderForClose\": {"]
#[doc = "            \"description\": \"Obsolete\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"$ref\": \"#/$defs/OrderForClose\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"delayedOrderForOpen\": {"]
#[doc = "            \"description\": \"Obsolete\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"$ref\": \"#/$defs/OrderForOpen\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"depositSummary\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"entryOrders\": {"]
#[doc = "            \"description\": \"Obsolete\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"$ref\": \"#/$defs/Order\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"exitOrders\": {"]
#[doc = "            \"description\": \"Obsolete\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"$ref\": \"#/$defs/Order\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"initialInvestment\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"isPaused\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"mirrorCalculationType\": {"]
#[doc = "            \"description\": \"(Obsolete) Mirror positions weights calculation methodology\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"mirrorID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"mirrorStatusID\": {"]
#[doc = "            \"description\": \"0 - Active, 1 - Paused, 2 - Pending Closure, 3 - In Alignment Process\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"ordersForClose\": {"]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"$ref\": \"#/$defs/OrderForClose\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"ordersForCloseMultiple\": {"]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"$ref\": \"#/$defs/OrderForCloseMultiple\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"ordersForOpen\": {"]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"$ref\": \"#/$defs/OrderForOpen\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"parentCID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"parentMirrors\": {"]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"$ref\": \"#/$defs/Mirror\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"parentUsername\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"pendingForClosure\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"positions\": {"]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"$ref\": \"#/$defs/Position\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"startedCopyDate\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"date-time\""]
#[doc = "          },"]
#[doc = "          \"stopLossAmount\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"stopLossPercentage\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"withdrawalSummary\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"orders\": {"]
#[doc = "      \"description\": \"List of pending orders\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"Inline order shape — uses capital-ID field names (orderID, instrumentID, CID)\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"CID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"amount\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"executionType\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"instrumentID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"isBuy\": {"]
#[doc = "            \"description\": \"true=Long, false=Short\","]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"isDiscounted\": {"]
#[doc = "            \"description\": \"Obsolete\","]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"isTslEnabled\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"leverage\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"openDateTime\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"date-time\""]
#[doc = "          },"]
#[doc = "          \"orderID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"rate\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"stopLossRate\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"takeProfitRate\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"units\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ordersForClose\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/OrderForClose\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ordersForCloseMultiple\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/OrderForCloseMultiple\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ordersForOpen\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/OrderForOpen\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"positions\": {"]
#[doc = "      \"description\": \"List of currently open trading positions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Position\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"stockOrders\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Order\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PortfolioResponseClientPortfolio {
    #[serde(
        rename = "bonusCredit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bonus_credit: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "Available trading balance in USD, representing funds available for new positions"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub credit: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "entryOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub entry_orders: ::std::vec::Vec<::etoro_agent::types::trading::Order>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "exitOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exit_orders: ::std::vec::Vec<::etoro_agent::types::trading::Order>,
    #[doc = "Copy trading configurations and positions"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub mirrors: ::std::vec::Vec<PortfolioResponseClientPortfolioMirrorsItem>,
    #[doc = "List of pending orders"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub orders: ::std::vec::Vec<PortfolioResponseClientPortfolioOrdersItem>,
    #[serde(
        rename = "ordersForClose",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_close: ::std::vec::Vec<::etoro_agent::types::trading::OrderForClose>,
    #[serde(
        rename = "ordersForCloseMultiple",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_close_multiple:
        ::std::vec::Vec<::etoro_agent::types::trading::OrderForCloseMultiple>,
    #[serde(
        rename = "ordersForOpen",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_open: ::std::vec::Vec<::etoro_agent::types::trading::OrderForOpen>,
    #[doc = "List of currently open trading positions"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub positions: ::std::vec::Vec<::etoro_agent::types::trading::Position>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "stockOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub stock_orders: ::std::vec::Vec<::etoro_agent::types::trading::Order>,
}
impl ::std::default::Default for PortfolioResponseClientPortfolio {
    fn default() -> Self {
        Self {
            bonus_credit: Default::default(),
            credit: Default::default(),
            entry_orders: Default::default(),
            exit_orders: Default::default(),
            mirrors: Default::default(),
            orders: Default::default(),
            orders_for_close: Default::default(),
            orders_for_close_multiple: Default::default(),
            orders_for_open: Default::default(),
            positions: Default::default(),
            stock_orders: Default::default(),
        }
    }
}
#[doc = "Individual mirror trading setup (inline shape; see Mirror schema in trading-schema.json for the standalone version)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Individual mirror trading setup (inline shape; see Mirror schema in trading-schema.json for the standalone version)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"CID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"availableAmount\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"closedPositionsNetProfit\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"copyExistingPositions\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"delayedOrderForClose\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/OrderForClose\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"delayedOrderForOpen\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/OrderForOpen\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"depositSummary\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"entryOrders\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Order\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"exitOrders\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Order\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"initialInvestment\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"isPaused\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"mirrorCalculationType\": {"]
#[doc = "      \"description\": \"(Obsolete) Mirror positions weights calculation methodology\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"mirrorID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"mirrorStatusID\": {"]
#[doc = "      \"description\": \"0 - Active, 1 - Paused, 2 - Pending Closure, 3 - In Alignment Process\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"ordersForClose\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/OrderForClose\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ordersForCloseMultiple\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/OrderForCloseMultiple\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ordersForOpen\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/OrderForOpen\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"parentCID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"parentMirrors\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Mirror\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"parentUsername\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pendingForClosure\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"positions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Position\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"startedCopyDate\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"stopLossAmount\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"stopLossPercentage\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"withdrawalSummary\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PortfolioResponseClientPortfolioMirrorsItem {
    #[serde(
        rename = "availableAmount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub available_amount: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "CID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cid: ::std::option::Option<i64>,
    #[serde(
        rename = "closedPositionsNetProfit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub closed_positions_net_profit: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "copyExistingPositions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub copy_existing_positions: ::std::option::Option<bool>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "delayedOrderForClose",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub delayed_order_for_close: ::std::vec::Vec<::etoro_agent::types::trading::OrderForClose>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "delayedOrderForOpen",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub delayed_order_for_open: ::std::vec::Vec<::etoro_agent::types::trading::OrderForOpen>,
    #[serde(
        rename = "depositSummary",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub deposit_summary: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "entryOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub entry_orders: ::std::vec::Vec<::etoro_agent::types::trading::Order>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "exitOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exit_orders: ::std::vec::Vec<::etoro_agent::types::trading::Order>,
    #[serde(
        rename = "initialInvestment",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub initial_investment: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "isPaused",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_paused: ::std::option::Option<bool>,
    #[doc = "(Obsolete) Mirror positions weights calculation methodology"]
    #[serde(
        rename = "mirrorCalculationType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mirror_calculation_type: ::std::option::Option<i64>,
    #[serde(
        rename = "mirrorID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mirror_id: ::std::option::Option<i64>,
    #[doc = "0 - Active, 1 - Paused, 2 - Pending Closure, 3 - In Alignment Process"]
    #[serde(
        rename = "mirrorStatusID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mirror_status_id: ::std::option::Option<i64>,
    #[serde(
        rename = "ordersForClose",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_close: ::std::vec::Vec<::etoro_agent::types::trading::OrderForClose>,
    #[serde(
        rename = "ordersForCloseMultiple",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_close_multiple:
        ::std::vec::Vec<::etoro_agent::types::trading::OrderForCloseMultiple>,
    #[serde(
        rename = "ordersForOpen",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_open: ::std::vec::Vec<::etoro_agent::types::trading::OrderForOpen>,
    #[serde(
        rename = "parentCID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub parent_cid: ::std::option::Option<i64>,
    #[serde(
        rename = "parentMirrors",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub parent_mirrors: ::std::vec::Vec<::etoro_agent::types::trading::Mirror>,
    #[serde(
        rename = "parentUsername",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub parent_username: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "pendingForClosure",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pending_for_closure: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub positions: ::std::vec::Vec<::etoro_agent::types::trading::Position>,
    #[serde(
        rename = "startedCopyDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub started_copy_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "stopLossAmount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_amount: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "stopLossPercentage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_percentage: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "withdrawalSummary",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub withdrawal_summary: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
}
impl ::std::default::Default for PortfolioResponseClientPortfolioMirrorsItem {
    fn default() -> Self {
        Self {
            available_amount: Default::default(),
            cid: Default::default(),
            closed_positions_net_profit: Default::default(),
            copy_existing_positions: Default::default(),
            delayed_order_for_close: Default::default(),
            delayed_order_for_open: Default::default(),
            deposit_summary: Default::default(),
            entry_orders: Default::default(),
            exit_orders: Default::default(),
            initial_investment: Default::default(),
            is_paused: Default::default(),
            mirror_calculation_type: Default::default(),
            mirror_id: Default::default(),
            mirror_status_id: Default::default(),
            orders_for_close: Default::default(),
            orders_for_close_multiple: Default::default(),
            orders_for_open: Default::default(),
            parent_cid: Default::default(),
            parent_mirrors: Default::default(),
            parent_username: Default::default(),
            pending_for_closure: Default::default(),
            positions: Default::default(),
            started_copy_date: Default::default(),
            stop_loss_amount: Default::default(),
            stop_loss_percentage: Default::default(),
            withdrawal_summary: Default::default(),
        }
    }
}
#[doc = "Inline order shape — uses capital-ID field names (orderID, instrumentID, CID)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Inline order shape — uses capital-ID field names (orderID, instrumentID, CID)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"CID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"amount\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"executionType\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"instrumentID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"isBuy\": {"]
#[doc = "      \"description\": \"true=Long, false=Short\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isDiscounted\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isTslEnabled\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"leverage\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"openDateTime\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"orderID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"rate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"stopLossRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"takeProfitRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"units\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PortfolioResponseClientPortfolioOrdersItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "CID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cid: ::std::option::Option<i64>,
    #[serde(
        rename = "executionType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub execution_type: ::std::option::Option<i64>,
    #[serde(
        rename = "instrumentID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[doc = "true=Long, false=Short"]
    #[serde(
        rename = "isBuy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_buy: ::std::option::Option<bool>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "isDiscounted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_discounted: ::std::option::Option<bool>,
    #[serde(
        rename = "isTslEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_tsl_enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub leverage: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "openDateTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_date_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "orderID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "stopLossRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "takeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
}
impl ::std::default::Default for PortfolioResponseClientPortfolioOrdersItem {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            cid: Default::default(),
            execution_type: Default::default(),
            instrument_id: Default::default(),
            is_buy: Default::default(),
            is_discounted: Default::default(),
            is_tsl_enabled: Default::default(),
            leverage: Default::default(),
            open_date_time: Default::default(),
            order_id: Default::default(),
            rate: Default::default(),
            stop_loss_rate: Default::default(),
            take_profit_rate: Default::default(),
            units: Default::default(),
        }
    }
}
#[doc = "Comprehensive portfolio information including positions, orders, and account status"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Comprehensive portfolio information including positions, orders, and account status\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"clientPortfolio\": {"]
#[doc = "      \"description\": \"Container for all portfolio-related information\","]
#[doc = "      \"$ref\": \"#/$defs/ClientPortfolio\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PortfolioResponseWithPnl {
    #[doc = "Container for all portfolio-related information"]
    #[serde(
        rename = "clientPortfolio",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_portfolio: ::std::option::Option<ClientPortfolio>,
}
impl ::std::default::Default for PortfolioResponseWithPnl {
    fn default() -> Self {
        Self {
            client_portfolio: Default::default(),
        }
    }
}
