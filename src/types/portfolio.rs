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
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"credit\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"entryOrders\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"exitOrders\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
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
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"unrealizedPnL\": {"]
#[doc = "      \"description\": \"Total unrealized profit and loss across all open positions in the portfolio\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
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
    pub bonus_credit: ::std::option::Option<f32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub credit: ::std::option::Option<f32>,
    #[serde(
        rename = "entryOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub entry_orders:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(
        rename = "exitOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exit_orders: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub mirrors: ::std::vec::Vec<Mirror>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub orders: ::std::vec::Vec<Order>,
    #[serde(
        rename = "ordersForClose",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_close: ::std::vec::Vec<OrderForClose>,
    #[serde(
        rename = "ordersForCloseMultiple",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_close_multiple: ::std::vec::Vec<OrderForCloseMultiple>,
    #[serde(
        rename = "ordersForOpen",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_open: ::std::vec::Vec<OrderForOpen>,
    #[doc = "List of currently open trading positions"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub positions: ::std::vec::Vec<Position>,
    #[doc = "Stock-specific pending orders"]
    #[serde(
        rename = "stockOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub stock_orders:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "Total unrealized profit and loss across all open positions in the portfolio"]
    #[serde(
        rename = "unrealizedPnL",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub unrealized_pn_l: ::std::option::Option<f32>,
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
impl ClientPortfolio {
    pub fn builder() -> builder::ClientPortfolio {
        Default::default()
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
#[doc = "      \"type\": \"number\""]
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
    pub gain: ::std::option::Option<f64>,
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
impl GainEntry {
    pub fn builder() -> builder::GainEntry {
        Default::default()
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
impl GeCopiersResponse {
    pub fn builder() -> builder::GeCopiersResponse {
        Default::default()
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
impl GeCopiersResponseCopiersItem {
    pub fn builder() -> builder::GeCopiersResponseCopiersItem {
        Default::default()
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
#[doc = "          \"type\": \"number\""]
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
        gain: ::std::option::Option<f64>,
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
impl GetUserGainResponse {
    pub fn builder() -> builder::GetUserGainResponse {
        Default::default()
    }
}
#[doc = "Copy-trading configuration. Many fields obsolete or internal — see descriptions."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Copy-trading configuration. Many fields obsolete or internal — see descriptions.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"CID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"availableAmount\": {"]
#[doc = "      \"description\": \"Available USD balance reserved for mirror operations\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"closedPositionsNetProfit\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"copyExistingPositions\": {"]
#[doc = "      \"description\": \"Whether mirror copied parent's existing positions on registration\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"delayedOrderForClose\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"delayedOrderForOpen\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"depositSummary\": {"]
#[doc = "      \"description\": \"Total USD deposited after initial investment\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"entryOrders\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"exitOrders\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"initialInvestment\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"isPaused\": {"]
#[doc = "      \"description\": \"If true, restricts opening additional positions in mirror\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"mirrorCalculationType\": {"]
#[doc = "      \"description\": \"(Obsolete) Position weights calculation methodology\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"mirrorID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"mirrorStatusID\": {"]
#[doc = "      \"description\": \"0=Active, 1=Paused, 2=Pending Closure, 3=In Alignment Process\","]
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
#[doc = "      \"description\": \"Customer ID of the trader being copied\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"parentMirrors\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"parentUsername\": {"]
#[doc = "      \"description\": \"Username of the trader being copied\","]
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
#[doc = "      \"description\": \"USD value at which MirrorStopLoss triggers liquidation\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"stopLossPercentage\": {"]
#[doc = "      \"description\": \"% of mirror value that StopLossAmount represented at last edit\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"withdrawalSummary\": {"]
#[doc = "      \"description\": \"Total USD withdrawn from the mirror\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Mirror {
    #[doc = "Available USD balance reserved for mirror operations"]
    #[serde(
        rename = "availableAmount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub available_amount: ::std::option::Option<f32>,
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
    pub closed_positions_net_profit: ::std::option::Option<f32>,
    #[doc = "Whether mirror copied parent's existing positions on registration"]
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
    pub delayed_order_for_close:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "delayedOrderForOpen",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub delayed_order_for_open:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "Total USD deposited after initial investment"]
    #[serde(
        rename = "depositSummary",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub deposit_summary: ::std::option::Option<f32>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "entryOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub entry_orders:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "exitOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exit_orders: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(
        rename = "initialInvestment",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub initial_investment: ::std::option::Option<f32>,
    #[doc = "If true, restricts opening additional positions in mirror"]
    #[serde(
        rename = "isPaused",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_paused: ::std::option::Option<bool>,
    #[doc = "(Obsolete) Position weights calculation methodology"]
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
    #[doc = "0=Active, 1=Paused, 2=Pending Closure, 3=In Alignment Process"]
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
    pub orders_for_close: ::std::vec::Vec<OrderForClose>,
    #[serde(
        rename = "ordersForCloseMultiple",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_close_multiple: ::std::vec::Vec<OrderForCloseMultiple>,
    #[serde(
        rename = "ordersForOpen",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_open: ::std::vec::Vec<OrderForOpen>,
    #[doc = "Customer ID of the trader being copied"]
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
    pub parent_mirrors:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "Username of the trader being copied"]
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
    pub positions: ::std::vec::Vec<Position>,
    #[serde(
        rename = "startedCopyDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub started_copy_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "USD value at which MirrorStopLoss triggers liquidation"]
    #[serde(
        rename = "stopLossAmount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_amount: ::std::option::Option<f32>,
    #[doc = "% of mirror value that StopLossAmount represented at last edit"]
    #[serde(
        rename = "stopLossPercentage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_percentage: ::std::option::Option<f32>,
    #[doc = "Total USD withdrawn from the mirror"]
    #[serde(
        rename = "withdrawalSummary",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub withdrawal_summary: ::std::option::Option<f32>,
}
impl ::std::default::Default for Mirror {
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
impl Mirror {
    pub fn builder() -> builder::Mirror {
        Default::default()
    }
}
#[doc = "`Order`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"amount\": {"]
#[doc = "      \"description\": \"USD amount\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"cid\": {"]
#[doc = "      \"description\": \"Customer ID — note lowercase here, but Position uses CID (capital)\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"executionType\": {"]
#[doc = "      \"description\": \"Integer-encoded here (vs string enum on createOrderRequest)\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"instrumentId\": {"]
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
#[doc = "    \"isNoStopLoss\": {"]
#[doc = "      \"description\": \"false = enabled, true = disabled\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isNoTakeProfit\": {"]
#[doc = "      \"description\": \"false = enabled, true = disabled\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isTslEnabled\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"leverage\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"openDateTime\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"orderId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"rate\": {"]
#[doc = "      \"description\": \"Asset rate at which to send market order\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"stopLossRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"takeProfitRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"units\": {"]
#[doc = "      \"description\": \"Units to open. If > 0, position opens on units, not amount\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Order {
    #[doc = "USD amount"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<f32>,
    #[doc = "Customer ID — note lowercase here, but Position uses CID (capital)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cid: ::std::option::Option<i64>,
    #[doc = "Integer-encoded here (vs string enum on createOrderRequest)"]
    #[serde(
        rename = "executionType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub execution_type: ::std::option::Option<i64>,
    #[serde(
        rename = "instrumentId",
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
    #[doc = "false = enabled, true = disabled"]
    #[serde(
        rename = "isNoStopLoss",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_no_stop_loss: ::std::option::Option<bool>,
    #[doc = "false = enabled, true = disabled"]
    #[serde(
        rename = "isNoTakeProfit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_no_take_profit: ::std::option::Option<bool>,
    #[serde(
        rename = "isTslEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_tsl_enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub leverage: ::std::option::Option<i64>,
    #[serde(
        rename = "openDateTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_date_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "orderId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_id: ::std::option::Option<i64>,
    #[doc = "Asset rate at which to send market order"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rate: ::std::option::Option<f32>,
    #[serde(
        rename = "stopLossRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_rate: ::std::option::Option<f32>,
    #[serde(
        rename = "takeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<f32>,
    #[doc = "Units to open. If > 0, position opens on units, not amount"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<f32>,
}
impl ::std::default::Default for Order {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            cid: Default::default(),
            execution_type: Default::default(),
            instrument_id: Default::default(),
            is_buy: Default::default(),
            is_discounted: Default::default(),
            is_no_stop_loss: Default::default(),
            is_no_take_profit: Default::default(),
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
impl Order {
    pub fn builder() -> builder::Order {
        Default::default()
    }
}
#[doc = "`OrderForClose`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cid\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"instrumentId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"lastUpdate\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"lotsToDeduct\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"openDateTime\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"orderId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"orderType\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"positionId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"statusId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"unitsToDeduct\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OrderForClose {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cid: ::std::option::Option<i64>,
    #[serde(
        rename = "instrumentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[serde(
        rename = "lastUpdate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_update: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "lotsToDeduct",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lots_to_deduct: ::std::option::Option<f32>,
    #[serde(
        rename = "openDateTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_date_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "orderId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_id: ::std::option::Option<i64>,
    #[serde(
        rename = "orderType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_type: ::std::option::Option<i64>,
    #[serde(
        rename = "positionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub position_id: ::std::option::Option<i64>,
    #[serde(
        rename = "statusId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_id: ::std::option::Option<i64>,
    #[serde(
        rename = "unitsToDeduct",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub units_to_deduct: ::std::option::Option<f32>,
}
impl ::std::default::Default for OrderForClose {
    fn default() -> Self {
        Self {
            cid: Default::default(),
            instrument_id: Default::default(),
            last_update: Default::default(),
            lots_to_deduct: Default::default(),
            open_date_time: Default::default(),
            order_id: Default::default(),
            order_type: Default::default(),
            position_id: Default::default(),
            status_id: Default::default(),
            units_to_deduct: Default::default(),
        }
    }
}
impl OrderForClose {
    pub fn builder() -> builder::OrderForClose {
        Default::default()
    }
}
#[doc = "`OrderForCloseMultiple`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cid\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"instrumentId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"lastUpdate\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"lotsToDeduct\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"openDateTime\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"orderId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"orderType\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"pendingClosePositionIds\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"statusId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"unitsToDeduct\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OrderForCloseMultiple {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cid: ::std::option::Option<i64>,
    #[serde(
        rename = "instrumentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[serde(
        rename = "lastUpdate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_update: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "lotsToDeduct",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lots_to_deduct: ::std::option::Option<f32>,
    #[serde(
        rename = "openDateTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_date_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "orderId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_id: ::std::option::Option<i64>,
    #[serde(
        rename = "orderType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_type: ::std::option::Option<i64>,
    #[serde(
        rename = "pendingClosePositionIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub pending_close_position_ids: ::std::vec::Vec<i64>,
    #[serde(
        rename = "statusId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_id: ::std::option::Option<i64>,
    #[serde(
        rename = "unitsToDeduct",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub units_to_deduct: ::std::option::Option<f32>,
}
impl ::std::default::Default for OrderForCloseMultiple {
    fn default() -> Self {
        Self {
            cid: Default::default(),
            instrument_id: Default::default(),
            last_update: Default::default(),
            lots_to_deduct: Default::default(),
            open_date_time: Default::default(),
            order_id: Default::default(),
            order_type: Default::default(),
            pending_close_position_ids: Default::default(),
            status_id: Default::default(),
            units_to_deduct: Default::default(),
        }
    }
}
impl OrderForCloseMultiple {
    pub fn builder() -> builder::OrderForCloseMultiple {
        Default::default()
    }
}
#[doc = "`OrderForOpen`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"amount\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"amountInUnits\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"cid\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"externalOperation\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"frozenAmount\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"instrumentId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"isBuy\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isDiscounted\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isNoStopLoss\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isNoTakeProfit\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isTslEnabled\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"lastUpdate\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"leverage\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"lotCount\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"mirrorId\": {"]
#[doc = "      \"description\": \"ID for mirrored trades, if applicable\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"openDateTime\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"openPositionActionType\": {"]
#[doc = "      \"description\": \"Position open reason\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"orderId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"orderType\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"statusId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"stopLossRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"takeProfitRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"totalExternalCosts\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OrderForOpen {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<f32>,
    #[serde(
        rename = "amountInUnits",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub amount_in_units: ::std::option::Option<f32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cid: ::std::option::Option<i64>,
    #[serde(
        rename = "externalOperation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_operation:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(
        rename = "frozenAmount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub frozen_amount: ::std::option::Option<f32>,
    #[serde(
        rename = "instrumentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[serde(
        rename = "isBuy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_buy: ::std::option::Option<bool>,
    #[serde(
        rename = "isDiscounted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_discounted: ::std::option::Option<bool>,
    #[serde(
        rename = "isNoStopLoss",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_no_stop_loss: ::std::option::Option<bool>,
    #[serde(
        rename = "isNoTakeProfit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_no_take_profit: ::std::option::Option<bool>,
    #[serde(
        rename = "isTslEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_tsl_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "lastUpdate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_update: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub leverage: ::std::option::Option<i64>,
    #[serde(
        rename = "lotCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lot_count: ::std::option::Option<f32>,
    #[doc = "ID for mirrored trades, if applicable"]
    #[serde(
        rename = "mirrorId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mirror_id: ::std::option::Option<i64>,
    #[serde(
        rename = "openDateTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_date_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Position open reason"]
    #[serde(
        rename = "openPositionActionType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_position_action_type: ::std::option::Option<i64>,
    #[serde(
        rename = "orderId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_id: ::std::option::Option<i64>,
    #[serde(
        rename = "orderType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_type: ::std::option::Option<i64>,
    #[serde(
        rename = "statusId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_id: ::std::option::Option<i64>,
    #[serde(
        rename = "stopLossRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_rate: ::std::option::Option<f32>,
    #[serde(
        rename = "takeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<f32>,
    #[serde(
        rename = "totalExternalCosts",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_external_costs: ::std::option::Option<f32>,
}
impl ::std::default::Default for OrderForOpen {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            amount_in_units: Default::default(),
            cid: Default::default(),
            external_operation: Default::default(),
            frozen_amount: Default::default(),
            instrument_id: Default::default(),
            is_buy: Default::default(),
            is_discounted: Default::default(),
            is_no_stop_loss: Default::default(),
            is_no_take_profit: Default::default(),
            is_tsl_enabled: Default::default(),
            last_update: Default::default(),
            leverage: Default::default(),
            lot_count: Default::default(),
            mirror_id: Default::default(),
            open_date_time: Default::default(),
            open_position_action_type: Default::default(),
            order_id: Default::default(),
            order_type: Default::default(),
            status_id: Default::default(),
            stop_loss_rate: Default::default(),
            take_profit_rate: Default::default(),
            total_external_costs: Default::default(),
        }
    }
}
impl OrderForOpen {
    pub fn builder() -> builder::OrderForOpen {
        Default::default()
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
#[doc = "          \"format\": \"float\""]
#[doc = "        },"]
#[doc = "        \"credit\": {"]
#[doc = "          \"description\": \"Available trading balance in USD, representing funds available for new positions\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"format\": \"float\""]
#[doc = "        },"]
#[doc = "        \"entryOrders\": {"]
#[doc = "          \"description\": \"Obsolete\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"object\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"exitOrders\": {"]
#[doc = "          \"description\": \"Obsolete\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"object\""]
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
#[doc = "                \"format\": \"float\""]
#[doc = "              },"]
#[doc = "              \"closedPositionsNetProfit\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\""]
#[doc = "              },"]
#[doc = "              \"copyExistingPositions\": {"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"delayedOrderForClose\": {"]
#[doc = "                \"description\": \"Obsolete\","]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"type\": \"object\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"delayedOrderForOpen\": {"]
#[doc = "                \"description\": \"Obsolete\","]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"type\": \"object\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"depositSummary\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\""]
#[doc = "              },"]
#[doc = "              \"entryOrders\": {"]
#[doc = "                \"description\": \"Obsolete\","]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"type\": \"object\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"exitOrders\": {"]
#[doc = "                \"description\": \"Obsolete\","]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"type\": \"object\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"initialInvestment\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\""]
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
#[doc = "                  \"type\": \"object\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"ordersForCloseMultiple\": {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"type\": \"object\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"ordersForOpen\": {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"type\": \"object\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"parentCID\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"parentMirrors\": {"]
#[doc = "                \"type\": \"array\","]
#[doc = "                \"items\": {"]
#[doc = "                  \"type\": \"object\""]
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
#[doc = "                \"format\": \"float\""]
#[doc = "              },"]
#[doc = "              \"stopLossPercentage\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\""]
#[doc = "              },"]
#[doc = "              \"withdrawalSummary\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\""]
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
#[doc = "                \"format\": \"float\""]
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
#[doc = "                \"format\": \"float\""]
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
#[doc = "                \"format\": \"float\""]
#[doc = "              },"]
#[doc = "              \"stopLossRate\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\""]
#[doc = "              },"]
#[doc = "              \"takeProfitRate\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\""]
#[doc = "              },"]
#[doc = "              \"units\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"ordersForClose\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"object\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"ordersForCloseMultiple\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"object\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"ordersForOpen\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"object\""]
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
#[doc = "            \"type\": \"object\""]
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
impl PortfolioResponse {
    pub fn builder() -> builder::PortfolioResponse {
        Default::default()
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
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"credit\": {"]
#[doc = "      \"description\": \"Available trading balance in USD, representing funds available for new positions\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"entryOrders\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"exitOrders\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
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
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"closedPositionsNetProfit\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"copyExistingPositions\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"delayedOrderForClose\": {"]
#[doc = "            \"description\": \"Obsolete\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"object\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"delayedOrderForOpen\": {"]
#[doc = "            \"description\": \"Obsolete\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"object\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"depositSummary\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"entryOrders\": {"]
#[doc = "            \"description\": \"Obsolete\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"object\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"exitOrders\": {"]
#[doc = "            \"description\": \"Obsolete\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"object\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"initialInvestment\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
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
#[doc = "              \"type\": \"object\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"ordersForCloseMultiple\": {"]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"object\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"ordersForOpen\": {"]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"object\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"parentCID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"parentMirrors\": {"]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"object\""]
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
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"stopLossPercentage\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"withdrawalSummary\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
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
#[doc = "            \"format\": \"float\""]
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
#[doc = "            \"format\": \"float\""]
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
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"stopLossRate\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"takeProfitRate\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"units\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ordersForClose\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ordersForCloseMultiple\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ordersForOpen\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
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
#[doc = "        \"type\": \"object\""]
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
    pub bonus_credit: ::std::option::Option<f32>,
    #[doc = "Available trading balance in USD, representing funds available for new positions"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub credit: ::std::option::Option<f32>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "entryOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub entry_orders:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "exitOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exit_orders: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
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
    pub orders_for_close:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(
        rename = "ordersForCloseMultiple",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_close_multiple:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(
        rename = "ordersForOpen",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_open:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "List of currently open trading positions"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub positions: ::std::vec::Vec<Position>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "stockOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub stock_orders:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
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
impl PortfolioResponseClientPortfolio {
    pub fn builder() -> builder::PortfolioResponseClientPortfolio {
        Default::default()
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
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"closedPositionsNetProfit\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"copyExistingPositions\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"delayedOrderForClose\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"delayedOrderForOpen\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"depositSummary\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"entryOrders\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"exitOrders\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"initialInvestment\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
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
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ordersForCloseMultiple\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ordersForOpen\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"parentCID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"parentMirrors\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
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
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"stopLossPercentage\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"withdrawalSummary\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
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
    pub available_amount: ::std::option::Option<f32>,
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
    pub closed_positions_net_profit: ::std::option::Option<f32>,
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
    pub delayed_order_for_close:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "delayedOrderForOpen",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub delayed_order_for_open:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(
        rename = "depositSummary",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub deposit_summary: ::std::option::Option<f32>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "entryOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub entry_orders:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "exitOrders",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exit_orders: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(
        rename = "initialInvestment",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub initial_investment: ::std::option::Option<f32>,
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
    pub orders_for_close:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(
        rename = "ordersForCloseMultiple",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_close_multiple:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(
        rename = "ordersForOpen",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub orders_for_open:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
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
    pub parent_mirrors:
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
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
    pub positions: ::std::vec::Vec<Position>,
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
    pub stop_loss_amount: ::std::option::Option<f32>,
    #[serde(
        rename = "stopLossPercentage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_percentage: ::std::option::Option<f32>,
    #[serde(
        rename = "withdrawalSummary",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub withdrawal_summary: ::std::option::Option<f32>,
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
impl PortfolioResponseClientPortfolioMirrorsItem {
    pub fn builder() -> builder::PortfolioResponseClientPortfolioMirrorsItem {
        Default::default()
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
#[doc = "      \"format\": \"float\""]
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
#[doc = "      \"format\": \"float\""]
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
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"stopLossRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"takeProfitRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"units\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PortfolioResponseClientPortfolioOrdersItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<f32>,
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
    pub leverage: ::std::option::Option<f32>,
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
    pub rate: ::std::option::Option<f32>,
    #[serde(
        rename = "stopLossRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_rate: ::std::option::Option<f32>,
    #[serde(
        rename = "takeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<f32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<f32>,
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
impl PortfolioResponseClientPortfolioOrdersItem {
    pub fn builder() -> builder::PortfolioResponseClientPortfolioOrdersItem {
        Default::default()
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
impl PortfolioResponseWithPnl {
    pub fn builder() -> builder::PortfolioResponseWithPnl {
        Default::default()
    }
}
#[doc = "Individual position details"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Individual position details\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"CID\": {"]
#[doc = "      \"description\": \"Customer ID — note CAPITAL CID (vs lowercase cid in Order schema)\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"amount\": {"]
#[doc = "      \"description\": \"USD allocated (initial investment + additional margin collateral)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"initialAmountInDollars\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"initialUnits\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"instrumentID\": {"]
#[doc = "      \"description\": \"CAPITAL ID — different from instrumentId in other schemas\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"isBuy\": {"]
#[doc = "      \"description\": \"true = long (buy), false = short (sell)\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isDetached\": {"]
#[doc = "      \"description\": \"True if originally opened in a mirror and detached from it\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isDiscounted\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isNoStopLoss\": {"]
#[doc = "      \"description\": \"false=enabled, true=disabled\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isNoTakeProfit\": {"]
#[doc = "      \"description\": \"false=enabled, true=disabled\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isPartiallyAltered\": {"]
#[doc = "      \"description\": \"Whether this position was partially closed\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isSettled\": {"]
#[doc = "      \"description\": \"Obsolete\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isTslEnabled\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"leverage\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"lotCount\": {"]
#[doc = "      \"description\": \"For FutureContracts = number of contracts acquired\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"mirrorID\": {"]
#[doc = "      \"description\": \"Mirror ID if part of copy trading, 0 otherwise\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"openConversionRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"openDateTime\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"openPositionActionType\": {"]
#[doc = "      \"description\": \"Position open reason\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"openRate\": {"]
#[doc = "      \"description\": \"Entry price in instrument's currency\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"orderID\": {"]
#[doc = "      \"description\": \"Original orderID. Match together with orderType\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"orderType\": {"]
#[doc = "      \"description\": \"Original orderType. Match together with orderID\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"parentPositionID\": {"]
#[doc = "      \"description\": \"Parent position ID for mirrored positions, 0 otherwise\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"pnlVersion\": {"]
#[doc = "      \"description\": \"PnL formula used for calculating profit and loss\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"positionID\": {"]
#[doc = "      \"description\": \"Unique identifier for the position\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"redeemStatusID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"settlementTypeID\": {"]
#[doc = "      \"description\": \"0=CFD, 1=Real Asset, 2=SWAP, 3=Crypto MarginTrade, 4=Future Contract\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"stopLossRate\": {"]
#[doc = "      \"description\": \"Trigger price; must be worse than current price\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"stopLossVersion\": {"]
#[doc = "      \"description\": \"Increments each time StopLossRate is manually updated\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"takeProfitRate\": {"]
#[doc = "      \"description\": \"Trigger price; must be better than current price\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"totalExternalFees\": {"]
#[doc = "      \"description\": \"USD fees (e.g. TicketFee). Excludes overnight fees and dividends\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"totalExternalTaxes\": {"]
#[doc = "      \"description\": \"USD taxes (e.g. SDRT)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"totalFees\": {"]
#[doc = "      \"description\": \"Total overnight fees and dividends in USD. Negative = refund\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"units\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"unitsBaseValueDollars\": {"]
#[doc = "      \"description\": \"Current units invested value in USD\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Position {
    #[doc = "USD allocated (initial investment + additional margin collateral)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<f32>,
    #[doc = "Customer ID — note CAPITAL CID (vs lowercase cid in Order schema)"]
    #[serde(
        rename = "CID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cid: ::std::option::Option<i64>,
    #[serde(
        rename = "initialAmountInDollars",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub initial_amount_in_dollars: ::std::option::Option<f32>,
    #[serde(
        rename = "initialUnits",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub initial_units: ::std::option::Option<f32>,
    #[doc = "CAPITAL ID — different from instrumentId in other schemas"]
    #[serde(
        rename = "instrumentID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[doc = "true = long (buy), false = short (sell)"]
    #[serde(
        rename = "isBuy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_buy: ::std::option::Option<bool>,
    #[doc = "True if originally opened in a mirror and detached from it"]
    #[serde(
        rename = "isDetached",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_detached: ::std::option::Option<bool>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "isDiscounted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_discounted: ::std::option::Option<bool>,
    #[doc = "false=enabled, true=disabled"]
    #[serde(
        rename = "isNoStopLoss",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_no_stop_loss: ::std::option::Option<bool>,
    #[doc = "false=enabled, true=disabled"]
    #[serde(
        rename = "isNoTakeProfit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_no_take_profit: ::std::option::Option<bool>,
    #[doc = "Whether this position was partially closed"]
    #[serde(
        rename = "isPartiallyAltered",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_partially_altered: ::std::option::Option<bool>,
    #[doc = "Obsolete"]
    #[serde(
        rename = "isSettled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_settled: ::std::option::Option<bool>,
    #[serde(
        rename = "isTslEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_tsl_enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub leverage: ::std::option::Option<f32>,
    #[doc = "For FutureContracts = number of contracts acquired"]
    #[serde(
        rename = "lotCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lot_count: ::std::option::Option<f32>,
    #[doc = "Mirror ID if part of copy trading, 0 otherwise"]
    #[serde(
        rename = "mirrorID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mirror_id: ::std::option::Option<i64>,
    #[serde(
        rename = "openConversionRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_conversion_rate: ::std::option::Option<f32>,
    #[serde(
        rename = "openDateTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_date_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Position open reason"]
    #[serde(
        rename = "openPositionActionType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_position_action_type: ::std::option::Option<i64>,
    #[doc = "Entry price in instrument's currency"]
    #[serde(
        rename = "openRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_rate: ::std::option::Option<f32>,
    #[doc = "Original orderID. Match together with orderType"]
    #[serde(
        rename = "orderID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_id: ::std::option::Option<i64>,
    #[doc = "Original orderType. Match together with orderID"]
    #[serde(
        rename = "orderType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_type: ::std::option::Option<i64>,
    #[doc = "Parent position ID for mirrored positions, 0 otherwise"]
    #[serde(
        rename = "parentPositionID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub parent_position_id: ::std::option::Option<i64>,
    #[doc = "PnL formula used for calculating profit and loss"]
    #[serde(
        rename = "pnlVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pnl_version: ::std::option::Option<i64>,
    #[doc = "Unique identifier for the position"]
    #[serde(
        rename = "positionID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub position_id: ::std::option::Option<i64>,
    #[serde(
        rename = "redeemStatusID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub redeem_status_id: ::std::option::Option<i64>,
    #[doc = "0=CFD, 1=Real Asset, 2=SWAP, 3=Crypto MarginTrade, 4=Future Contract"]
    #[serde(
        rename = "settlementTypeID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub settlement_type_id: ::std::option::Option<i64>,
    #[doc = "Trigger price; must be worse than current price"]
    #[serde(
        rename = "stopLossRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_rate: ::std::option::Option<f32>,
    #[doc = "Increments each time StopLossRate is manually updated"]
    #[serde(
        rename = "stopLossVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_version: ::std::option::Option<i64>,
    #[doc = "Trigger price; must be better than current price"]
    #[serde(
        rename = "takeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<f32>,
    #[doc = "USD fees (e.g. TicketFee). Excludes overnight fees and dividends"]
    #[serde(
        rename = "totalExternalFees",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_external_fees: ::std::option::Option<f32>,
    #[doc = "USD taxes (e.g. SDRT)"]
    #[serde(
        rename = "totalExternalTaxes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_external_taxes: ::std::option::Option<f32>,
    #[doc = "Total overnight fees and dividends in USD. Negative = refund"]
    #[serde(
        rename = "totalFees",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_fees: ::std::option::Option<f32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<f32>,
    #[doc = "Current units invested value in USD"]
    #[serde(
        rename = "unitsBaseValueDollars",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub units_base_value_dollars: ::std::option::Option<f32>,
}
impl ::std::default::Default for Position {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            cid: Default::default(),
            initial_amount_in_dollars: Default::default(),
            initial_units: Default::default(),
            instrument_id: Default::default(),
            is_buy: Default::default(),
            is_detached: Default::default(),
            is_discounted: Default::default(),
            is_no_stop_loss: Default::default(),
            is_no_take_profit: Default::default(),
            is_partially_altered: Default::default(),
            is_settled: Default::default(),
            is_tsl_enabled: Default::default(),
            leverage: Default::default(),
            lot_count: Default::default(),
            mirror_id: Default::default(),
            open_conversion_rate: Default::default(),
            open_date_time: Default::default(),
            open_position_action_type: Default::default(),
            open_rate: Default::default(),
            order_id: Default::default(),
            order_type: Default::default(),
            parent_position_id: Default::default(),
            pnl_version: Default::default(),
            position_id: Default::default(),
            redeem_status_id: Default::default(),
            settlement_type_id: Default::default(),
            stop_loss_rate: Default::default(),
            stop_loss_version: Default::default(),
            take_profit_rate: Default::default(),
            total_external_fees: Default::default(),
            total_external_taxes: Default::default(),
            total_fees: Default::default(),
            units: Default::default(),
            units_base_value_dollars: Default::default(),
        }
    }
}
impl Position {
    pub fn builder() -> builder::Position {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ClientPortfolio {
        account_currency_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        bonus_credit: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        credit: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        entry_orders: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        exit_orders: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        mirrors: ::std::result::Result<::std::vec::Vec<super::Mirror>, ::std::string::String>,
        orders: ::std::result::Result<::std::vec::Vec<super::Order>, ::std::string::String>,
        orders_for_close:
            ::std::result::Result<::std::vec::Vec<super::OrderForClose>, ::std::string::String>,
        orders_for_close_multiple: ::std::result::Result<
            ::std::vec::Vec<super::OrderForCloseMultiple>,
            ::std::string::String,
        >,
        orders_for_open:
            ::std::result::Result<::std::vec::Vec<super::OrderForOpen>, ::std::string::String>,
        positions: ::std::result::Result<::std::vec::Vec<super::Position>, ::std::string::String>,
        stock_orders: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        unrealized_pn_l: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for ClientPortfolio {
        fn default() -> Self {
            Self {
                account_currency_id: Ok(Default::default()),
                bonus_credit: Ok(Default::default()),
                credit: Ok(Default::default()),
                entry_orders: Ok(Default::default()),
                exit_orders: Ok(Default::default()),
                mirrors: Ok(Default::default()),
                orders: Ok(Default::default()),
                orders_for_close: Ok(Default::default()),
                orders_for_close_multiple: Ok(Default::default()),
                orders_for_open: Ok(Default::default()),
                positions: Ok(Default::default()),
                stock_orders: Ok(Default::default()),
                unrealized_pn_l: Ok(Default::default()),
            }
        }
    }
    impl ClientPortfolio {
        pub fn account_currency_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.account_currency_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for account_currency_id: {e}")
            });
            self
        }
        pub fn bonus_credit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.bonus_credit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bonus_credit: {e}"));
            self
        }
        pub fn credit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.credit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for credit: {e}"));
            self
        }
        pub fn entry_orders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.entry_orders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entry_orders: {e}"));
            self
        }
        pub fn exit_orders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.exit_orders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exit_orders: {e}"));
            self
        }
        pub fn mirrors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Mirror>>,
            T::Error: ::std::fmt::Display,
        {
            self.mirrors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mirrors: {e}"));
            self
        }
        pub fn orders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Order>>,
            T::Error: ::std::fmt::Display,
        {
            self.orders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orders: {e}"));
            self
        }
        pub fn orders_for_close<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrderForClose>>,
            T::Error: ::std::fmt::Display,
        {
            self.orders_for_close = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orders_for_close: {e}"));
            self
        }
        pub fn orders_for_close_multiple<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrderForCloseMultiple>>,
            T::Error: ::std::fmt::Display,
        {
            self.orders_for_close_multiple = value.try_into().map_err(|e| {
                format!("error converting supplied value for orders_for_close_multiple: {e}")
            });
            self
        }
        pub fn orders_for_open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrderForOpen>>,
            T::Error: ::std::fmt::Display,
        {
            self.orders_for_open = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orders_for_open: {e}"));
            self
        }
        pub fn positions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Position>>,
            T::Error: ::std::fmt::Display,
        {
            self.positions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for positions: {e}"));
            self
        }
        pub fn stock_orders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.stock_orders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stock_orders: {e}"));
            self
        }
        pub fn unrealized_pn_l<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.unrealized_pn_l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unrealized_pn_l: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClientPortfolio> for super::ClientPortfolio {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClientPortfolio,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account_currency_id: value.account_currency_id?,
                bonus_credit: value.bonus_credit?,
                credit: value.credit?,
                entry_orders: value.entry_orders?,
                exit_orders: value.exit_orders?,
                mirrors: value.mirrors?,
                orders: value.orders?,
                orders_for_close: value.orders_for_close?,
                orders_for_close_multiple: value.orders_for_close_multiple?,
                orders_for_open: value.orders_for_open?,
                positions: value.positions?,
                stock_orders: value.stock_orders?,
                unrealized_pn_l: value.unrealized_pn_l?,
            })
        }
    }
    impl ::std::convert::From<super::ClientPortfolio> for ClientPortfolio {
        fn from(value: super::ClientPortfolio) -> Self {
            Self {
                account_currency_id: Ok(value.account_currency_id),
                bonus_credit: Ok(value.bonus_credit),
                credit: Ok(value.credit),
                entry_orders: Ok(value.entry_orders),
                exit_orders: Ok(value.exit_orders),
                mirrors: Ok(value.mirrors),
                orders: Ok(value.orders),
                orders_for_close: Ok(value.orders_for_close),
                orders_for_close_multiple: Ok(value.orders_for_close_multiple),
                orders_for_open: Ok(value.orders_for_open),
                positions: Ok(value.positions),
                stock_orders: Ok(value.stock_orders),
                unrealized_pn_l: Ok(value.unrealized_pn_l),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GainEntry {
        gain: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        timestamp: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GainEntry {
        fn default() -> Self {
            Self {
                gain: Ok(Default::default()),
                timestamp: Ok(Default::default()),
            }
        }
    }
    impl GainEntry {
        pub fn gain<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.gain = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gain: {e}"));
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GainEntry> for super::GainEntry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GainEntry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                gain: value.gain?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::GainEntry> for GainEntry {
        fn from(value: super::GainEntry) -> Self {
            Self {
                gain: Ok(value.gain),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GeCopiersResponse {
        copiers: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::GeCopiersResponseCopiersItem>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GeCopiersResponse {
        fn default() -> Self {
            Self {
                copiers: Ok(Default::default()),
            }
        }
    }
    impl GeCopiersResponse {
        pub fn copiers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::GeCopiersResponseCopiersItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.copiers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for copiers: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GeCopiersResponse> for super::GeCopiersResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GeCopiersResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                copiers: value.copiers?,
            })
        }
    }
    impl ::std::convert::From<super::GeCopiersResponse> for GeCopiersResponse {
        fn from(value: super::GeCopiersResponse) -> Self {
            Self {
                copiers: Ok(value.copiers),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GeCopiersResponseCopiersItem {
        age_category: ::std::result::Result<
            ::std::option::Option<super::GeCopiersResponseCopiersItemAgeCategory>,
            ::std::string::String,
        >,
        amount_category: ::std::result::Result<
            ::std::option::Option<super::GeCopiersResponseCopiersItemAmountCategory>,
            ::std::string::String,
        >,
        available_copy_balance: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        club: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        copy_realized_equity_pnl: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        copy_started_at_category: ::std::result::Result<
            ::std::option::Option<super::GeCopiersResponseCopiersItemCopyStartedAtCategory>,
            ::std::string::String,
        >,
        country: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        gender: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GeCopiersResponseCopiersItem {
        fn default() -> Self {
            Self {
                age_category: Ok(Default::default()),
                amount_category: Ok(Default::default()),
                available_copy_balance: Ok(Default::default()),
                club: Ok(Default::default()),
                copy_realized_equity_pnl: Ok(Default::default()),
                copy_started_at_category: Ok(Default::default()),
                country: Ok(Default::default()),
                gender: Ok(Default::default()),
            }
        }
    }
    impl GeCopiersResponseCopiersItem {
        pub fn age_category<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::GeCopiersResponseCopiersItemAgeCategory>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.age_category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for age_category: {e}"));
            self
        }
        pub fn amount_category<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::GeCopiersResponseCopiersItemAmountCategory>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.amount_category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_category: {e}"));
            self
        }
        pub fn available_copy_balance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.available_copy_balance = value.try_into().map_err(|e| {
                format!("error converting supplied value for available_copy_balance: {e}")
            });
            self
        }
        pub fn club<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.club = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for club: {e}"));
            self
        }
        pub fn copy_realized_equity_pnl<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.copy_realized_equity_pnl = value.try_into().map_err(|e| {
                format!("error converting supplied value for copy_realized_equity_pnl: {e}")
            });
            self
        }
        pub fn copy_started_at_category<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::GeCopiersResponseCopiersItemCopyStartedAtCategory>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.copy_started_at_category = value.try_into().map_err(|e| {
                format!("error converting supplied value for copy_started_at_category: {e}")
            });
            self
        }
        pub fn country<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.country = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for country: {e}"));
            self
        }
        pub fn gender<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.gender = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gender: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GeCopiersResponseCopiersItem> for super::GeCopiersResponseCopiersItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GeCopiersResponseCopiersItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                age_category: value.age_category?,
                amount_category: value.amount_category?,
                available_copy_balance: value.available_copy_balance?,
                club: value.club?,
                copy_realized_equity_pnl: value.copy_realized_equity_pnl?,
                copy_started_at_category: value.copy_started_at_category?,
                country: value.country?,
                gender: value.gender?,
            })
        }
    }
    impl ::std::convert::From<super::GeCopiersResponseCopiersItem> for GeCopiersResponseCopiersItem {
        fn from(value: super::GeCopiersResponseCopiersItem) -> Self {
            Self {
                age_category: Ok(value.age_category),
                amount_category: Ok(value.amount_category),
                available_copy_balance: Ok(value.available_copy_balance),
                club: Ok(value.club),
                copy_realized_equity_pnl: Ok(value.copy_realized_equity_pnl),
                copy_started_at_category: Ok(value.copy_started_at_category),
                country: Ok(value.country),
                gender: Ok(value.gender),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetUserGainResponse {
        monthly: ::std::result::Result<::std::vec::Vec<super::GainEntry>, ::std::string::String>,
        yearly: ::std::result::Result<::std::vec::Vec<super::GainEntry>, ::std::string::String>,
    }
    impl ::std::default::Default for GetUserGainResponse {
        fn default() -> Self {
            Self {
                monthly: Ok(Default::default()),
                yearly: Ok(Default::default()),
            }
        }
    }
    impl GetUserGainResponse {
        pub fn monthly<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::GainEntry>>,
            T::Error: ::std::fmt::Display,
        {
            self.monthly = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for monthly: {e}"));
            self
        }
        pub fn yearly<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::GainEntry>>,
            T::Error: ::std::fmt::Display,
        {
            self.yearly = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for yearly: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetUserGainResponse> for super::GetUserGainResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetUserGainResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                monthly: value.monthly?,
                yearly: value.yearly?,
            })
        }
    }
    impl ::std::convert::From<super::GetUserGainResponse> for GetUserGainResponse {
        fn from(value: super::GetUserGainResponse) -> Self {
            Self {
                monthly: Ok(value.monthly),
                yearly: Ok(value.yearly),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Mirror {
        available_amount: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        closed_positions_net_profit:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        copy_existing_positions:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        delayed_order_for_close: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        delayed_order_for_open: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        deposit_summary: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        entry_orders: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        exit_orders: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        initial_investment:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        is_paused: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        mirror_calculation_type:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        mirror_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        mirror_status_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        orders_for_close:
            ::std::result::Result<::std::vec::Vec<super::OrderForClose>, ::std::string::String>,
        orders_for_close_multiple: ::std::result::Result<
            ::std::vec::Vec<super::OrderForCloseMultiple>,
            ::std::string::String,
        >,
        orders_for_open:
            ::std::result::Result<::std::vec::Vec<super::OrderForOpen>, ::std::string::String>,
        parent_cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        parent_mirrors: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        parent_username: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pending_for_closure:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        positions: ::std::result::Result<::std::vec::Vec<super::Position>, ::std::string::String>,
        started_copy_date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        stop_loss_amount: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        stop_loss_percentage:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        withdrawal_summary:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for Mirror {
        fn default() -> Self {
            Self {
                available_amount: Ok(Default::default()),
                cid: Ok(Default::default()),
                closed_positions_net_profit: Ok(Default::default()),
                copy_existing_positions: Ok(Default::default()),
                delayed_order_for_close: Ok(Default::default()),
                delayed_order_for_open: Ok(Default::default()),
                deposit_summary: Ok(Default::default()),
                entry_orders: Ok(Default::default()),
                exit_orders: Ok(Default::default()),
                initial_investment: Ok(Default::default()),
                is_paused: Ok(Default::default()),
                mirror_calculation_type: Ok(Default::default()),
                mirror_id: Ok(Default::default()),
                mirror_status_id: Ok(Default::default()),
                orders_for_close: Ok(Default::default()),
                orders_for_close_multiple: Ok(Default::default()),
                orders_for_open: Ok(Default::default()),
                parent_cid: Ok(Default::default()),
                parent_mirrors: Ok(Default::default()),
                parent_username: Ok(Default::default()),
                pending_for_closure: Ok(Default::default()),
                positions: Ok(Default::default()),
                started_copy_date: Ok(Default::default()),
                stop_loss_amount: Ok(Default::default()),
                stop_loss_percentage: Ok(Default::default()),
                withdrawal_summary: Ok(Default::default()),
            }
        }
    }
    impl Mirror {
        pub fn available_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.available_amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for available_amount: {e}"));
            self
        }
        pub fn cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cid: {e}"));
            self
        }
        pub fn closed_positions_net_profit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.closed_positions_net_profit = value.try_into().map_err(|e| {
                format!("error converting supplied value for closed_positions_net_profit: {e}")
            });
            self
        }
        pub fn copy_existing_positions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.copy_existing_positions = value.try_into().map_err(|e| {
                format!("error converting supplied value for copy_existing_positions: {e}")
            });
            self
        }
        pub fn delayed_order_for_close<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.delayed_order_for_close = value.try_into().map_err(|e| {
                format!("error converting supplied value for delayed_order_for_close: {e}")
            });
            self
        }
        pub fn delayed_order_for_open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.delayed_order_for_open = value.try_into().map_err(|e| {
                format!("error converting supplied value for delayed_order_for_open: {e}")
            });
            self
        }
        pub fn deposit_summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.deposit_summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for deposit_summary: {e}"));
            self
        }
        pub fn entry_orders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.entry_orders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entry_orders: {e}"));
            self
        }
        pub fn exit_orders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.exit_orders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exit_orders: {e}"));
            self
        }
        pub fn initial_investment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.initial_investment = value.try_into().map_err(|e| {
                format!("error converting supplied value for initial_investment: {e}")
            });
            self
        }
        pub fn is_paused<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_paused = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_paused: {e}"));
            self
        }
        pub fn mirror_calculation_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mirror_calculation_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for mirror_calculation_type: {e}")
            });
            self
        }
        pub fn mirror_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mirror_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mirror_id: {e}"));
            self
        }
        pub fn mirror_status_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mirror_status_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mirror_status_id: {e}"));
            self
        }
        pub fn orders_for_close<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrderForClose>>,
            T::Error: ::std::fmt::Display,
        {
            self.orders_for_close = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orders_for_close: {e}"));
            self
        }
        pub fn orders_for_close_multiple<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrderForCloseMultiple>>,
            T::Error: ::std::fmt::Display,
        {
            self.orders_for_close_multiple = value.try_into().map_err(|e| {
                format!("error converting supplied value for orders_for_close_multiple: {e}")
            });
            self
        }
        pub fn orders_for_open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrderForOpen>>,
            T::Error: ::std::fmt::Display,
        {
            self.orders_for_open = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orders_for_open: {e}"));
            self
        }
        pub fn parent_cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent_cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent_cid: {e}"));
            self
        }
        pub fn parent_mirrors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.parent_mirrors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent_mirrors: {e}"));
            self
        }
        pub fn parent_username<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent_username = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent_username: {e}"));
            self
        }
        pub fn pending_for_closure<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.pending_for_closure = value.try_into().map_err(|e| {
                format!("error converting supplied value for pending_for_closure: {e}")
            });
            self
        }
        pub fn positions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Position>>,
            T::Error: ::std::fmt::Display,
        {
            self.positions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for positions: {e}"));
            self
        }
        pub fn started_copy_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.started_copy_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for started_copy_date: {e}"));
            self
        }
        pub fn stop_loss_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_loss_amount: {e}"));
            self
        }
        pub fn stop_loss_percentage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_percentage = value.try_into().map_err(|e| {
                format!("error converting supplied value for stop_loss_percentage: {e}")
            });
            self
        }
        pub fn withdrawal_summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.withdrawal_summary = value.try_into().map_err(|e| {
                format!("error converting supplied value for withdrawal_summary: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<Mirror> for super::Mirror {
        type Error = super::error::ConversionError;
        fn try_from(value: Mirror) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                available_amount: value.available_amount?,
                cid: value.cid?,
                closed_positions_net_profit: value.closed_positions_net_profit?,
                copy_existing_positions: value.copy_existing_positions?,
                delayed_order_for_close: value.delayed_order_for_close?,
                delayed_order_for_open: value.delayed_order_for_open?,
                deposit_summary: value.deposit_summary?,
                entry_orders: value.entry_orders?,
                exit_orders: value.exit_orders?,
                initial_investment: value.initial_investment?,
                is_paused: value.is_paused?,
                mirror_calculation_type: value.mirror_calculation_type?,
                mirror_id: value.mirror_id?,
                mirror_status_id: value.mirror_status_id?,
                orders_for_close: value.orders_for_close?,
                orders_for_close_multiple: value.orders_for_close_multiple?,
                orders_for_open: value.orders_for_open?,
                parent_cid: value.parent_cid?,
                parent_mirrors: value.parent_mirrors?,
                parent_username: value.parent_username?,
                pending_for_closure: value.pending_for_closure?,
                positions: value.positions?,
                started_copy_date: value.started_copy_date?,
                stop_loss_amount: value.stop_loss_amount?,
                stop_loss_percentage: value.stop_loss_percentage?,
                withdrawal_summary: value.withdrawal_summary?,
            })
        }
    }
    impl ::std::convert::From<super::Mirror> for Mirror {
        fn from(value: super::Mirror) -> Self {
            Self {
                available_amount: Ok(value.available_amount),
                cid: Ok(value.cid),
                closed_positions_net_profit: Ok(value.closed_positions_net_profit),
                copy_existing_positions: Ok(value.copy_existing_positions),
                delayed_order_for_close: Ok(value.delayed_order_for_close),
                delayed_order_for_open: Ok(value.delayed_order_for_open),
                deposit_summary: Ok(value.deposit_summary),
                entry_orders: Ok(value.entry_orders),
                exit_orders: Ok(value.exit_orders),
                initial_investment: Ok(value.initial_investment),
                is_paused: Ok(value.is_paused),
                mirror_calculation_type: Ok(value.mirror_calculation_type),
                mirror_id: Ok(value.mirror_id),
                mirror_status_id: Ok(value.mirror_status_id),
                orders_for_close: Ok(value.orders_for_close),
                orders_for_close_multiple: Ok(value.orders_for_close_multiple),
                orders_for_open: Ok(value.orders_for_open),
                parent_cid: Ok(value.parent_cid),
                parent_mirrors: Ok(value.parent_mirrors),
                parent_username: Ok(value.parent_username),
                pending_for_closure: Ok(value.pending_for_closure),
                positions: Ok(value.positions),
                started_copy_date: Ok(value.started_copy_date),
                stop_loss_amount: Ok(value.stop_loss_amount),
                stop_loss_percentage: Ok(value.stop_loss_percentage),
                withdrawal_summary: Ok(value.withdrawal_summary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Order {
        amount: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        execution_type: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        is_buy: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_discounted: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_no_stop_loss: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_no_take_profit:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_tsl_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        leverage: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        open_date_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        order_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        rate: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        stop_loss_rate: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        take_profit_rate: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        units: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for Order {
        fn default() -> Self {
            Self {
                amount: Ok(Default::default()),
                cid: Ok(Default::default()),
                execution_type: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                is_buy: Ok(Default::default()),
                is_discounted: Ok(Default::default()),
                is_no_stop_loss: Ok(Default::default()),
                is_no_take_profit: Ok(Default::default()),
                is_tsl_enabled: Ok(Default::default()),
                leverage: Ok(Default::default()),
                open_date_time: Ok(Default::default()),
                order_id: Ok(Default::default()),
                rate: Ok(Default::default()),
                stop_loss_rate: Ok(Default::default()),
                take_profit_rate: Ok(Default::default()),
                units: Ok(Default::default()),
            }
        }
    }
    impl Order {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cid: {e}"));
            self
        }
        pub fn execution_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.execution_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for execution_type: {e}"));
            self
        }
        pub fn instrument_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
            self
        }
        pub fn is_buy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_buy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_buy: {e}"));
            self
        }
        pub fn is_discounted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_discounted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_discounted: {e}"));
            self
        }
        pub fn is_no_stop_loss<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_no_stop_loss = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_no_stop_loss: {e}"));
            self
        }
        pub fn is_no_take_profit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_no_take_profit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_no_take_profit: {e}"));
            self
        }
        pub fn is_tsl_enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_tsl_enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_tsl_enabled: {e}"));
            self
        }
        pub fn leverage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.leverage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for leverage: {e}"));
            self
        }
        pub fn open_date_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.open_date_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open_date_time: {e}"));
            self
        }
        pub fn order_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.order_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_id: {e}"));
            self
        }
        pub fn rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rate: {e}"));
            self
        }
        pub fn stop_loss_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_loss_rate: {e}"));
            self
        }
        pub fn take_profit_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.take_profit_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for take_profit_rate: {e}"));
            self
        }
        pub fn units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for units: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Order> for super::Order {
        type Error = super::error::ConversionError;
        fn try_from(value: Order) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                cid: value.cid?,
                execution_type: value.execution_type?,
                instrument_id: value.instrument_id?,
                is_buy: value.is_buy?,
                is_discounted: value.is_discounted?,
                is_no_stop_loss: value.is_no_stop_loss?,
                is_no_take_profit: value.is_no_take_profit?,
                is_tsl_enabled: value.is_tsl_enabled?,
                leverage: value.leverage?,
                open_date_time: value.open_date_time?,
                order_id: value.order_id?,
                rate: value.rate?,
                stop_loss_rate: value.stop_loss_rate?,
                take_profit_rate: value.take_profit_rate?,
                units: value.units?,
            })
        }
    }
    impl ::std::convert::From<super::Order> for Order {
        fn from(value: super::Order) -> Self {
            Self {
                amount: Ok(value.amount),
                cid: Ok(value.cid),
                execution_type: Ok(value.execution_type),
                instrument_id: Ok(value.instrument_id),
                is_buy: Ok(value.is_buy),
                is_discounted: Ok(value.is_discounted),
                is_no_stop_loss: Ok(value.is_no_stop_loss),
                is_no_take_profit: Ok(value.is_no_take_profit),
                is_tsl_enabled: Ok(value.is_tsl_enabled),
                leverage: Ok(value.leverage),
                open_date_time: Ok(value.open_date_time),
                order_id: Ok(value.order_id),
                rate: Ok(value.rate),
                stop_loss_rate: Ok(value.stop_loss_rate),
                take_profit_rate: Ok(value.take_profit_rate),
                units: Ok(value.units),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrderForClose {
        cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        last_update: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        lots_to_deduct: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        open_date_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        order_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order_type: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        position_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        status_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        units_to_deduct: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for OrderForClose {
        fn default() -> Self {
            Self {
                cid: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                last_update: Ok(Default::default()),
                lots_to_deduct: Ok(Default::default()),
                open_date_time: Ok(Default::default()),
                order_id: Ok(Default::default()),
                order_type: Ok(Default::default()),
                position_id: Ok(Default::default()),
                status_id: Ok(Default::default()),
                units_to_deduct: Ok(Default::default()),
            }
        }
    }
    impl OrderForClose {
        pub fn cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cid: {e}"));
            self
        }
        pub fn instrument_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
            self
        }
        pub fn last_update<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.last_update = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_update: {e}"));
            self
        }
        pub fn lots_to_deduct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.lots_to_deduct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lots_to_deduct: {e}"));
            self
        }
        pub fn open_date_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.open_date_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open_date_time: {e}"));
            self
        }
        pub fn order_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.order_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_id: {e}"));
            self
        }
        pub fn order_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.order_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_type: {e}"));
            self
        }
        pub fn position_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.position_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position_id: {e}"));
            self
        }
        pub fn status_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status_id: {e}"));
            self
        }
        pub fn units_to_deduct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.units_to_deduct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for units_to_deduct: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrderForClose> for super::OrderForClose {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrderForClose,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cid: value.cid?,
                instrument_id: value.instrument_id?,
                last_update: value.last_update?,
                lots_to_deduct: value.lots_to_deduct?,
                open_date_time: value.open_date_time?,
                order_id: value.order_id?,
                order_type: value.order_type?,
                position_id: value.position_id?,
                status_id: value.status_id?,
                units_to_deduct: value.units_to_deduct?,
            })
        }
    }
    impl ::std::convert::From<super::OrderForClose> for OrderForClose {
        fn from(value: super::OrderForClose) -> Self {
            Self {
                cid: Ok(value.cid),
                instrument_id: Ok(value.instrument_id),
                last_update: Ok(value.last_update),
                lots_to_deduct: Ok(value.lots_to_deduct),
                open_date_time: Ok(value.open_date_time),
                order_id: Ok(value.order_id),
                order_type: Ok(value.order_type),
                position_id: Ok(value.position_id),
                status_id: Ok(value.status_id),
                units_to_deduct: Ok(value.units_to_deduct),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrderForCloseMultiple {
        cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        last_update: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        lots_to_deduct: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        open_date_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        order_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order_type: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        pending_close_position_ids:
            ::std::result::Result<::std::vec::Vec<i64>, ::std::string::String>,
        status_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        units_to_deduct: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for OrderForCloseMultiple {
        fn default() -> Self {
            Self {
                cid: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                last_update: Ok(Default::default()),
                lots_to_deduct: Ok(Default::default()),
                open_date_time: Ok(Default::default()),
                order_id: Ok(Default::default()),
                order_type: Ok(Default::default()),
                pending_close_position_ids: Ok(Default::default()),
                status_id: Ok(Default::default()),
                units_to_deduct: Ok(Default::default()),
            }
        }
    }
    impl OrderForCloseMultiple {
        pub fn cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cid: {e}"));
            self
        }
        pub fn instrument_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
            self
        }
        pub fn last_update<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.last_update = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_update: {e}"));
            self
        }
        pub fn lots_to_deduct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.lots_to_deduct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lots_to_deduct: {e}"));
            self
        }
        pub fn open_date_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.open_date_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open_date_time: {e}"));
            self
        }
        pub fn order_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.order_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_id: {e}"));
            self
        }
        pub fn order_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.order_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_type: {e}"));
            self
        }
        pub fn pending_close_position_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.pending_close_position_ids = value.try_into().map_err(|e| {
                format!("error converting supplied value for pending_close_position_ids: {e}")
            });
            self
        }
        pub fn status_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status_id: {e}"));
            self
        }
        pub fn units_to_deduct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.units_to_deduct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for units_to_deduct: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrderForCloseMultiple> for super::OrderForCloseMultiple {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrderForCloseMultiple,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cid: value.cid?,
                instrument_id: value.instrument_id?,
                last_update: value.last_update?,
                lots_to_deduct: value.lots_to_deduct?,
                open_date_time: value.open_date_time?,
                order_id: value.order_id?,
                order_type: value.order_type?,
                pending_close_position_ids: value.pending_close_position_ids?,
                status_id: value.status_id?,
                units_to_deduct: value.units_to_deduct?,
            })
        }
    }
    impl ::std::convert::From<super::OrderForCloseMultiple> for OrderForCloseMultiple {
        fn from(value: super::OrderForCloseMultiple) -> Self {
            Self {
                cid: Ok(value.cid),
                instrument_id: Ok(value.instrument_id),
                last_update: Ok(value.last_update),
                lots_to_deduct: Ok(value.lots_to_deduct),
                open_date_time: Ok(value.open_date_time),
                order_id: Ok(value.order_id),
                order_type: Ok(value.order_type),
                pending_close_position_ids: Ok(value.pending_close_position_ids),
                status_id: Ok(value.status_id),
                units_to_deduct: Ok(value.units_to_deduct),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrderForOpen {
        amount: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        amount_in_units: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        external_operation: ::std::result::Result<
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        frozen_amount: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        is_buy: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_discounted: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_no_stop_loss: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_no_take_profit:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_tsl_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        last_update: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        leverage: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        lot_count: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        mirror_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        open_date_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        open_position_action_type:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order_type: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        status_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        stop_loss_rate: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        take_profit_rate: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        total_external_costs:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for OrderForOpen {
        fn default() -> Self {
            Self {
                amount: Ok(Default::default()),
                amount_in_units: Ok(Default::default()),
                cid: Ok(Default::default()),
                external_operation: Ok(Default::default()),
                frozen_amount: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                is_buy: Ok(Default::default()),
                is_discounted: Ok(Default::default()),
                is_no_stop_loss: Ok(Default::default()),
                is_no_take_profit: Ok(Default::default()),
                is_tsl_enabled: Ok(Default::default()),
                last_update: Ok(Default::default()),
                leverage: Ok(Default::default()),
                lot_count: Ok(Default::default()),
                mirror_id: Ok(Default::default()),
                open_date_time: Ok(Default::default()),
                open_position_action_type: Ok(Default::default()),
                order_id: Ok(Default::default()),
                order_type: Ok(Default::default()),
                status_id: Ok(Default::default()),
                stop_loss_rate: Ok(Default::default()),
                take_profit_rate: Ok(Default::default()),
                total_external_costs: Ok(Default::default()),
            }
        }
    }
    impl OrderForOpen {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn amount_in_units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_in_units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_in_units: {e}"));
            self
        }
        pub fn cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cid: {e}"));
            self
        }
        pub fn external_operation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.external_operation = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_operation: {e}")
            });
            self
        }
        pub fn frozen_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.frozen_amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for frozen_amount: {e}"));
            self
        }
        pub fn instrument_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
            self
        }
        pub fn is_buy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_buy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_buy: {e}"));
            self
        }
        pub fn is_discounted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_discounted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_discounted: {e}"));
            self
        }
        pub fn is_no_stop_loss<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_no_stop_loss = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_no_stop_loss: {e}"));
            self
        }
        pub fn is_no_take_profit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_no_take_profit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_no_take_profit: {e}"));
            self
        }
        pub fn is_tsl_enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_tsl_enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_tsl_enabled: {e}"));
            self
        }
        pub fn last_update<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.last_update = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_update: {e}"));
            self
        }
        pub fn leverage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.leverage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for leverage: {e}"));
            self
        }
        pub fn lot_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.lot_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lot_count: {e}"));
            self
        }
        pub fn mirror_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mirror_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mirror_id: {e}"));
            self
        }
        pub fn open_date_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.open_date_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open_date_time: {e}"));
            self
        }
        pub fn open_position_action_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_position_action_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for open_position_action_type: {e}")
            });
            self
        }
        pub fn order_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.order_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_id: {e}"));
            self
        }
        pub fn order_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.order_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_type: {e}"));
            self
        }
        pub fn status_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status_id: {e}"));
            self
        }
        pub fn stop_loss_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_loss_rate: {e}"));
            self
        }
        pub fn take_profit_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.take_profit_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for take_profit_rate: {e}"));
            self
        }
        pub fn total_external_costs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_external_costs = value.try_into().map_err(|e| {
                format!("error converting supplied value for total_external_costs: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<OrderForOpen> for super::OrderForOpen {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrderForOpen,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                amount_in_units: value.amount_in_units?,
                cid: value.cid?,
                external_operation: value.external_operation?,
                frozen_amount: value.frozen_amount?,
                instrument_id: value.instrument_id?,
                is_buy: value.is_buy?,
                is_discounted: value.is_discounted?,
                is_no_stop_loss: value.is_no_stop_loss?,
                is_no_take_profit: value.is_no_take_profit?,
                is_tsl_enabled: value.is_tsl_enabled?,
                last_update: value.last_update?,
                leverage: value.leverage?,
                lot_count: value.lot_count?,
                mirror_id: value.mirror_id?,
                open_date_time: value.open_date_time?,
                open_position_action_type: value.open_position_action_type?,
                order_id: value.order_id?,
                order_type: value.order_type?,
                status_id: value.status_id?,
                stop_loss_rate: value.stop_loss_rate?,
                take_profit_rate: value.take_profit_rate?,
                total_external_costs: value.total_external_costs?,
            })
        }
    }
    impl ::std::convert::From<super::OrderForOpen> for OrderForOpen {
        fn from(value: super::OrderForOpen) -> Self {
            Self {
                amount: Ok(value.amount),
                amount_in_units: Ok(value.amount_in_units),
                cid: Ok(value.cid),
                external_operation: Ok(value.external_operation),
                frozen_amount: Ok(value.frozen_amount),
                instrument_id: Ok(value.instrument_id),
                is_buy: Ok(value.is_buy),
                is_discounted: Ok(value.is_discounted),
                is_no_stop_loss: Ok(value.is_no_stop_loss),
                is_no_take_profit: Ok(value.is_no_take_profit),
                is_tsl_enabled: Ok(value.is_tsl_enabled),
                last_update: Ok(value.last_update),
                leverage: Ok(value.leverage),
                lot_count: Ok(value.lot_count),
                mirror_id: Ok(value.mirror_id),
                open_date_time: Ok(value.open_date_time),
                open_position_action_type: Ok(value.open_position_action_type),
                order_id: Ok(value.order_id),
                order_type: Ok(value.order_type),
                status_id: Ok(value.status_id),
                stop_loss_rate: Ok(value.stop_loss_rate),
                take_profit_rate: Ok(value.take_profit_rate),
                total_external_costs: Ok(value.total_external_costs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PortfolioResponse {
        client_portfolio: ::std::result::Result<
            ::std::option::Option<super::PortfolioResponseClientPortfolio>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PortfolioResponse {
        fn default() -> Self {
            Self {
                client_portfolio: Ok(Default::default()),
            }
        }
    }
    impl PortfolioResponse {
        pub fn client_portfolio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PortfolioResponseClientPortfolio>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.client_portfolio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_portfolio: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PortfolioResponse> for super::PortfolioResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PortfolioResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_portfolio: value.client_portfolio?,
            })
        }
    }
    impl ::std::convert::From<super::PortfolioResponse> for PortfolioResponse {
        fn from(value: super::PortfolioResponse) -> Self {
            Self {
                client_portfolio: Ok(value.client_portfolio),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PortfolioResponseClientPortfolio {
        bonus_credit: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        credit: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        entry_orders: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        exit_orders: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        mirrors: ::std::result::Result<
            ::std::vec::Vec<super::PortfolioResponseClientPortfolioMirrorsItem>,
            ::std::string::String,
        >,
        orders: ::std::result::Result<
            ::std::vec::Vec<super::PortfolioResponseClientPortfolioOrdersItem>,
            ::std::string::String,
        >,
        orders_for_close: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        orders_for_close_multiple: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        orders_for_open: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        positions: ::std::result::Result<::std::vec::Vec<super::Position>, ::std::string::String>,
        stock_orders: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PortfolioResponseClientPortfolio {
        fn default() -> Self {
            Self {
                bonus_credit: Ok(Default::default()),
                credit: Ok(Default::default()),
                entry_orders: Ok(Default::default()),
                exit_orders: Ok(Default::default()),
                mirrors: Ok(Default::default()),
                orders: Ok(Default::default()),
                orders_for_close: Ok(Default::default()),
                orders_for_close_multiple: Ok(Default::default()),
                orders_for_open: Ok(Default::default()),
                positions: Ok(Default::default()),
                stock_orders: Ok(Default::default()),
            }
        }
    }
    impl PortfolioResponseClientPortfolio {
        pub fn bonus_credit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.bonus_credit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bonus_credit: {e}"));
            self
        }
        pub fn credit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.credit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for credit: {e}"));
            self
        }
        pub fn entry_orders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.entry_orders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entry_orders: {e}"));
            self
        }
        pub fn exit_orders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.exit_orders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exit_orders: {e}"));
            self
        }
        pub fn mirrors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::PortfolioResponseClientPortfolioMirrorsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.mirrors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mirrors: {e}"));
            self
        }
        pub fn orders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::PortfolioResponseClientPortfolioOrdersItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.orders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orders: {e}"));
            self
        }
        pub fn orders_for_close<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.orders_for_close = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orders_for_close: {e}"));
            self
        }
        pub fn orders_for_close_multiple<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.orders_for_close_multiple = value.try_into().map_err(|e| {
                format!("error converting supplied value for orders_for_close_multiple: {e}")
            });
            self
        }
        pub fn orders_for_open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.orders_for_open = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orders_for_open: {e}"));
            self
        }
        pub fn positions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Position>>,
            T::Error: ::std::fmt::Display,
        {
            self.positions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for positions: {e}"));
            self
        }
        pub fn stock_orders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.stock_orders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stock_orders: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PortfolioResponseClientPortfolio>
        for super::PortfolioResponseClientPortfolio
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PortfolioResponseClientPortfolio,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bonus_credit: value.bonus_credit?,
                credit: value.credit?,
                entry_orders: value.entry_orders?,
                exit_orders: value.exit_orders?,
                mirrors: value.mirrors?,
                orders: value.orders?,
                orders_for_close: value.orders_for_close?,
                orders_for_close_multiple: value.orders_for_close_multiple?,
                orders_for_open: value.orders_for_open?,
                positions: value.positions?,
                stock_orders: value.stock_orders?,
            })
        }
    }
    impl ::std::convert::From<super::PortfolioResponseClientPortfolio>
        for PortfolioResponseClientPortfolio
    {
        fn from(value: super::PortfolioResponseClientPortfolio) -> Self {
            Self {
                bonus_credit: Ok(value.bonus_credit),
                credit: Ok(value.credit),
                entry_orders: Ok(value.entry_orders),
                exit_orders: Ok(value.exit_orders),
                mirrors: Ok(value.mirrors),
                orders: Ok(value.orders),
                orders_for_close: Ok(value.orders_for_close),
                orders_for_close_multiple: Ok(value.orders_for_close_multiple),
                orders_for_open: Ok(value.orders_for_open),
                positions: Ok(value.positions),
                stock_orders: Ok(value.stock_orders),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PortfolioResponseClientPortfolioMirrorsItem {
        available_amount: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        closed_positions_net_profit:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        copy_existing_positions:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        delayed_order_for_close: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        delayed_order_for_open: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        deposit_summary: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        entry_orders: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        exit_orders: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        initial_investment:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        is_paused: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        mirror_calculation_type:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        mirror_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        mirror_status_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        orders_for_close: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        orders_for_close_multiple: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        orders_for_open: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        parent_cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        parent_mirrors: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        parent_username: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pending_for_closure:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        positions: ::std::result::Result<::std::vec::Vec<super::Position>, ::std::string::String>,
        started_copy_date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        stop_loss_amount: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        stop_loss_percentage:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        withdrawal_summary:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for PortfolioResponseClientPortfolioMirrorsItem {
        fn default() -> Self {
            Self {
                available_amount: Ok(Default::default()),
                cid: Ok(Default::default()),
                closed_positions_net_profit: Ok(Default::default()),
                copy_existing_positions: Ok(Default::default()),
                delayed_order_for_close: Ok(Default::default()),
                delayed_order_for_open: Ok(Default::default()),
                deposit_summary: Ok(Default::default()),
                entry_orders: Ok(Default::default()),
                exit_orders: Ok(Default::default()),
                initial_investment: Ok(Default::default()),
                is_paused: Ok(Default::default()),
                mirror_calculation_type: Ok(Default::default()),
                mirror_id: Ok(Default::default()),
                mirror_status_id: Ok(Default::default()),
                orders_for_close: Ok(Default::default()),
                orders_for_close_multiple: Ok(Default::default()),
                orders_for_open: Ok(Default::default()),
                parent_cid: Ok(Default::default()),
                parent_mirrors: Ok(Default::default()),
                parent_username: Ok(Default::default()),
                pending_for_closure: Ok(Default::default()),
                positions: Ok(Default::default()),
                started_copy_date: Ok(Default::default()),
                stop_loss_amount: Ok(Default::default()),
                stop_loss_percentage: Ok(Default::default()),
                withdrawal_summary: Ok(Default::default()),
            }
        }
    }
    impl PortfolioResponseClientPortfolioMirrorsItem {
        pub fn available_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.available_amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for available_amount: {e}"));
            self
        }
        pub fn cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cid: {e}"));
            self
        }
        pub fn closed_positions_net_profit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.closed_positions_net_profit = value.try_into().map_err(|e| {
                format!("error converting supplied value for closed_positions_net_profit: {e}")
            });
            self
        }
        pub fn copy_existing_positions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.copy_existing_positions = value.try_into().map_err(|e| {
                format!("error converting supplied value for copy_existing_positions: {e}")
            });
            self
        }
        pub fn delayed_order_for_close<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.delayed_order_for_close = value.try_into().map_err(|e| {
                format!("error converting supplied value for delayed_order_for_close: {e}")
            });
            self
        }
        pub fn delayed_order_for_open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.delayed_order_for_open = value.try_into().map_err(|e| {
                format!("error converting supplied value for delayed_order_for_open: {e}")
            });
            self
        }
        pub fn deposit_summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.deposit_summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for deposit_summary: {e}"));
            self
        }
        pub fn entry_orders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.entry_orders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entry_orders: {e}"));
            self
        }
        pub fn exit_orders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.exit_orders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exit_orders: {e}"));
            self
        }
        pub fn initial_investment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.initial_investment = value.try_into().map_err(|e| {
                format!("error converting supplied value for initial_investment: {e}")
            });
            self
        }
        pub fn is_paused<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_paused = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_paused: {e}"));
            self
        }
        pub fn mirror_calculation_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mirror_calculation_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for mirror_calculation_type: {e}")
            });
            self
        }
        pub fn mirror_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mirror_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mirror_id: {e}"));
            self
        }
        pub fn mirror_status_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mirror_status_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mirror_status_id: {e}"));
            self
        }
        pub fn orders_for_close<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.orders_for_close = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orders_for_close: {e}"));
            self
        }
        pub fn orders_for_close_multiple<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.orders_for_close_multiple = value.try_into().map_err(|e| {
                format!("error converting supplied value for orders_for_close_multiple: {e}")
            });
            self
        }
        pub fn orders_for_open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.orders_for_open = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orders_for_open: {e}"));
            self
        }
        pub fn parent_cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent_cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent_cid: {e}"));
            self
        }
        pub fn parent_mirrors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.parent_mirrors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent_mirrors: {e}"));
            self
        }
        pub fn parent_username<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent_username = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent_username: {e}"));
            self
        }
        pub fn pending_for_closure<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.pending_for_closure = value.try_into().map_err(|e| {
                format!("error converting supplied value for pending_for_closure: {e}")
            });
            self
        }
        pub fn positions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Position>>,
            T::Error: ::std::fmt::Display,
        {
            self.positions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for positions: {e}"));
            self
        }
        pub fn started_copy_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.started_copy_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for started_copy_date: {e}"));
            self
        }
        pub fn stop_loss_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_loss_amount: {e}"));
            self
        }
        pub fn stop_loss_percentage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_percentage = value.try_into().map_err(|e| {
                format!("error converting supplied value for stop_loss_percentage: {e}")
            });
            self
        }
        pub fn withdrawal_summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.withdrawal_summary = value.try_into().map_err(|e| {
                format!("error converting supplied value for withdrawal_summary: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<PortfolioResponseClientPortfolioMirrorsItem>
        for super::PortfolioResponseClientPortfolioMirrorsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PortfolioResponseClientPortfolioMirrorsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                available_amount: value.available_amount?,
                cid: value.cid?,
                closed_positions_net_profit: value.closed_positions_net_profit?,
                copy_existing_positions: value.copy_existing_positions?,
                delayed_order_for_close: value.delayed_order_for_close?,
                delayed_order_for_open: value.delayed_order_for_open?,
                deposit_summary: value.deposit_summary?,
                entry_orders: value.entry_orders?,
                exit_orders: value.exit_orders?,
                initial_investment: value.initial_investment?,
                is_paused: value.is_paused?,
                mirror_calculation_type: value.mirror_calculation_type?,
                mirror_id: value.mirror_id?,
                mirror_status_id: value.mirror_status_id?,
                orders_for_close: value.orders_for_close?,
                orders_for_close_multiple: value.orders_for_close_multiple?,
                orders_for_open: value.orders_for_open?,
                parent_cid: value.parent_cid?,
                parent_mirrors: value.parent_mirrors?,
                parent_username: value.parent_username?,
                pending_for_closure: value.pending_for_closure?,
                positions: value.positions?,
                started_copy_date: value.started_copy_date?,
                stop_loss_amount: value.stop_loss_amount?,
                stop_loss_percentage: value.stop_loss_percentage?,
                withdrawal_summary: value.withdrawal_summary?,
            })
        }
    }
    impl ::std::convert::From<super::PortfolioResponseClientPortfolioMirrorsItem>
        for PortfolioResponseClientPortfolioMirrorsItem
    {
        fn from(value: super::PortfolioResponseClientPortfolioMirrorsItem) -> Self {
            Self {
                available_amount: Ok(value.available_amount),
                cid: Ok(value.cid),
                closed_positions_net_profit: Ok(value.closed_positions_net_profit),
                copy_existing_positions: Ok(value.copy_existing_positions),
                delayed_order_for_close: Ok(value.delayed_order_for_close),
                delayed_order_for_open: Ok(value.delayed_order_for_open),
                deposit_summary: Ok(value.deposit_summary),
                entry_orders: Ok(value.entry_orders),
                exit_orders: Ok(value.exit_orders),
                initial_investment: Ok(value.initial_investment),
                is_paused: Ok(value.is_paused),
                mirror_calculation_type: Ok(value.mirror_calculation_type),
                mirror_id: Ok(value.mirror_id),
                mirror_status_id: Ok(value.mirror_status_id),
                orders_for_close: Ok(value.orders_for_close),
                orders_for_close_multiple: Ok(value.orders_for_close_multiple),
                orders_for_open: Ok(value.orders_for_open),
                parent_cid: Ok(value.parent_cid),
                parent_mirrors: Ok(value.parent_mirrors),
                parent_username: Ok(value.parent_username),
                pending_for_closure: Ok(value.pending_for_closure),
                positions: Ok(value.positions),
                started_copy_date: Ok(value.started_copy_date),
                stop_loss_amount: Ok(value.stop_loss_amount),
                stop_loss_percentage: Ok(value.stop_loss_percentage),
                withdrawal_summary: Ok(value.withdrawal_summary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PortfolioResponseClientPortfolioOrdersItem {
        amount: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        execution_type: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        is_buy: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_discounted: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_tsl_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        leverage: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        open_date_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        order_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        rate: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        stop_loss_rate: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        take_profit_rate: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        units: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for PortfolioResponseClientPortfolioOrdersItem {
        fn default() -> Self {
            Self {
                amount: Ok(Default::default()),
                cid: Ok(Default::default()),
                execution_type: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                is_buy: Ok(Default::default()),
                is_discounted: Ok(Default::default()),
                is_tsl_enabled: Ok(Default::default()),
                leverage: Ok(Default::default()),
                open_date_time: Ok(Default::default()),
                order_id: Ok(Default::default()),
                rate: Ok(Default::default()),
                stop_loss_rate: Ok(Default::default()),
                take_profit_rate: Ok(Default::default()),
                units: Ok(Default::default()),
            }
        }
    }
    impl PortfolioResponseClientPortfolioOrdersItem {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cid: {e}"));
            self
        }
        pub fn execution_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.execution_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for execution_type: {e}"));
            self
        }
        pub fn instrument_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
            self
        }
        pub fn is_buy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_buy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_buy: {e}"));
            self
        }
        pub fn is_discounted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_discounted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_discounted: {e}"));
            self
        }
        pub fn is_tsl_enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_tsl_enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_tsl_enabled: {e}"));
            self
        }
        pub fn leverage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.leverage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for leverage: {e}"));
            self
        }
        pub fn open_date_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.open_date_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open_date_time: {e}"));
            self
        }
        pub fn order_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.order_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_id: {e}"));
            self
        }
        pub fn rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rate: {e}"));
            self
        }
        pub fn stop_loss_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_loss_rate: {e}"));
            self
        }
        pub fn take_profit_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.take_profit_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for take_profit_rate: {e}"));
            self
        }
        pub fn units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for units: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PortfolioResponseClientPortfolioOrdersItem>
        for super::PortfolioResponseClientPortfolioOrdersItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PortfolioResponseClientPortfolioOrdersItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                cid: value.cid?,
                execution_type: value.execution_type?,
                instrument_id: value.instrument_id?,
                is_buy: value.is_buy?,
                is_discounted: value.is_discounted?,
                is_tsl_enabled: value.is_tsl_enabled?,
                leverage: value.leverage?,
                open_date_time: value.open_date_time?,
                order_id: value.order_id?,
                rate: value.rate?,
                stop_loss_rate: value.stop_loss_rate?,
                take_profit_rate: value.take_profit_rate?,
                units: value.units?,
            })
        }
    }
    impl ::std::convert::From<super::PortfolioResponseClientPortfolioOrdersItem>
        for PortfolioResponseClientPortfolioOrdersItem
    {
        fn from(value: super::PortfolioResponseClientPortfolioOrdersItem) -> Self {
            Self {
                amount: Ok(value.amount),
                cid: Ok(value.cid),
                execution_type: Ok(value.execution_type),
                instrument_id: Ok(value.instrument_id),
                is_buy: Ok(value.is_buy),
                is_discounted: Ok(value.is_discounted),
                is_tsl_enabled: Ok(value.is_tsl_enabled),
                leverage: Ok(value.leverage),
                open_date_time: Ok(value.open_date_time),
                order_id: Ok(value.order_id),
                rate: Ok(value.rate),
                stop_loss_rate: Ok(value.stop_loss_rate),
                take_profit_rate: Ok(value.take_profit_rate),
                units: Ok(value.units),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PortfolioResponseWithPnl {
        client_portfolio: ::std::result::Result<
            ::std::option::Option<super::ClientPortfolio>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PortfolioResponseWithPnl {
        fn default() -> Self {
            Self {
                client_portfolio: Ok(Default::default()),
            }
        }
    }
    impl PortfolioResponseWithPnl {
        pub fn client_portfolio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ClientPortfolio>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_portfolio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_portfolio: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PortfolioResponseWithPnl> for super::PortfolioResponseWithPnl {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PortfolioResponseWithPnl,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_portfolio: value.client_portfolio?,
            })
        }
    }
    impl ::std::convert::From<super::PortfolioResponseWithPnl> for PortfolioResponseWithPnl {
        fn from(value: super::PortfolioResponseWithPnl) -> Self {
            Self {
                client_portfolio: Ok(value.client_portfolio),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Position {
        amount: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        initial_amount_in_dollars:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        initial_units: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        is_buy: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_detached: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_discounted: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_no_stop_loss: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_no_take_profit:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_partially_altered:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_settled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_tsl_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        leverage: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        lot_count: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        mirror_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        open_conversion_rate:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        open_date_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        open_position_action_type:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        open_rate: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        order_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order_type: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        parent_position_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        pnl_version: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        position_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        redeem_status_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        settlement_type_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        stop_loss_rate: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        stop_loss_version: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        take_profit_rate: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        total_external_fees:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        total_external_taxes:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        total_fees: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        units: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        units_base_value_dollars:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for Position {
        fn default() -> Self {
            Self {
                amount: Ok(Default::default()),
                cid: Ok(Default::default()),
                initial_amount_in_dollars: Ok(Default::default()),
                initial_units: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                is_buy: Ok(Default::default()),
                is_detached: Ok(Default::default()),
                is_discounted: Ok(Default::default()),
                is_no_stop_loss: Ok(Default::default()),
                is_no_take_profit: Ok(Default::default()),
                is_partially_altered: Ok(Default::default()),
                is_settled: Ok(Default::default()),
                is_tsl_enabled: Ok(Default::default()),
                leverage: Ok(Default::default()),
                lot_count: Ok(Default::default()),
                mirror_id: Ok(Default::default()),
                open_conversion_rate: Ok(Default::default()),
                open_date_time: Ok(Default::default()),
                open_position_action_type: Ok(Default::default()),
                open_rate: Ok(Default::default()),
                order_id: Ok(Default::default()),
                order_type: Ok(Default::default()),
                parent_position_id: Ok(Default::default()),
                pnl_version: Ok(Default::default()),
                position_id: Ok(Default::default()),
                redeem_status_id: Ok(Default::default()),
                settlement_type_id: Ok(Default::default()),
                stop_loss_rate: Ok(Default::default()),
                stop_loss_version: Ok(Default::default()),
                take_profit_rate: Ok(Default::default()),
                total_external_fees: Ok(Default::default()),
                total_external_taxes: Ok(Default::default()),
                total_fees: Ok(Default::default()),
                units: Ok(Default::default()),
                units_base_value_dollars: Ok(Default::default()),
            }
        }
    }
    impl Position {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cid: {e}"));
            self
        }
        pub fn initial_amount_in_dollars<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.initial_amount_in_dollars = value.try_into().map_err(|e| {
                format!("error converting supplied value for initial_amount_in_dollars: {e}")
            });
            self
        }
        pub fn initial_units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.initial_units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for initial_units: {e}"));
            self
        }
        pub fn instrument_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
            self
        }
        pub fn is_buy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_buy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_buy: {e}"));
            self
        }
        pub fn is_detached<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_detached = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_detached: {e}"));
            self
        }
        pub fn is_discounted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_discounted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_discounted: {e}"));
            self
        }
        pub fn is_no_stop_loss<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_no_stop_loss = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_no_stop_loss: {e}"));
            self
        }
        pub fn is_no_take_profit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_no_take_profit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_no_take_profit: {e}"));
            self
        }
        pub fn is_partially_altered<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_partially_altered = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_partially_altered: {e}")
            });
            self
        }
        pub fn is_settled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_settled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_settled: {e}"));
            self
        }
        pub fn is_tsl_enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_tsl_enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_tsl_enabled: {e}"));
            self
        }
        pub fn leverage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.leverage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for leverage: {e}"));
            self
        }
        pub fn lot_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.lot_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lot_count: {e}"));
            self
        }
        pub fn mirror_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mirror_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mirror_id: {e}"));
            self
        }
        pub fn open_conversion_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_conversion_rate = value.try_into().map_err(|e| {
                format!("error converting supplied value for open_conversion_rate: {e}")
            });
            self
        }
        pub fn open_date_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.open_date_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open_date_time: {e}"));
            self
        }
        pub fn open_position_action_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_position_action_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for open_position_action_type: {e}")
            });
            self
        }
        pub fn open_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open_rate: {e}"));
            self
        }
        pub fn order_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.order_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_id: {e}"));
            self
        }
        pub fn order_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.order_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_type: {e}"));
            self
        }
        pub fn parent_position_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent_position_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for parent_position_id: {e}")
            });
            self
        }
        pub fn pnl_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.pnl_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pnl_version: {e}"));
            self
        }
        pub fn position_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.position_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position_id: {e}"));
            self
        }
        pub fn redeem_status_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.redeem_status_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for redeem_status_id: {e}"));
            self
        }
        pub fn settlement_type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.settlement_type_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for settlement_type_id: {e}")
            });
            self
        }
        pub fn stop_loss_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_loss_rate: {e}"));
            self
        }
        pub fn stop_loss_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_loss_version: {e}"));
            self
        }
        pub fn take_profit_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.take_profit_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for take_profit_rate: {e}"));
            self
        }
        pub fn total_external_fees<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_external_fees = value.try_into().map_err(|e| {
                format!("error converting supplied value for total_external_fees: {e}")
            });
            self
        }
        pub fn total_external_taxes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_external_taxes = value.try_into().map_err(|e| {
                format!("error converting supplied value for total_external_taxes: {e}")
            });
            self
        }
        pub fn total_fees<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_fees = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_fees: {e}"));
            self
        }
        pub fn units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for units: {e}"));
            self
        }
        pub fn units_base_value_dollars<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.units_base_value_dollars = value.try_into().map_err(|e| {
                format!("error converting supplied value for units_base_value_dollars: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<Position> for super::Position {
        type Error = super::error::ConversionError;
        fn try_from(value: Position) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                cid: value.cid?,
                initial_amount_in_dollars: value.initial_amount_in_dollars?,
                initial_units: value.initial_units?,
                instrument_id: value.instrument_id?,
                is_buy: value.is_buy?,
                is_detached: value.is_detached?,
                is_discounted: value.is_discounted?,
                is_no_stop_loss: value.is_no_stop_loss?,
                is_no_take_profit: value.is_no_take_profit?,
                is_partially_altered: value.is_partially_altered?,
                is_settled: value.is_settled?,
                is_tsl_enabled: value.is_tsl_enabled?,
                leverage: value.leverage?,
                lot_count: value.lot_count?,
                mirror_id: value.mirror_id?,
                open_conversion_rate: value.open_conversion_rate?,
                open_date_time: value.open_date_time?,
                open_position_action_type: value.open_position_action_type?,
                open_rate: value.open_rate?,
                order_id: value.order_id?,
                order_type: value.order_type?,
                parent_position_id: value.parent_position_id?,
                pnl_version: value.pnl_version?,
                position_id: value.position_id?,
                redeem_status_id: value.redeem_status_id?,
                settlement_type_id: value.settlement_type_id?,
                stop_loss_rate: value.stop_loss_rate?,
                stop_loss_version: value.stop_loss_version?,
                take_profit_rate: value.take_profit_rate?,
                total_external_fees: value.total_external_fees?,
                total_external_taxes: value.total_external_taxes?,
                total_fees: value.total_fees?,
                units: value.units?,
                units_base_value_dollars: value.units_base_value_dollars?,
            })
        }
    }
    impl ::std::convert::From<super::Position> for Position {
        fn from(value: super::Position) -> Self {
            Self {
                amount: Ok(value.amount),
                cid: Ok(value.cid),
                initial_amount_in_dollars: Ok(value.initial_amount_in_dollars),
                initial_units: Ok(value.initial_units),
                instrument_id: Ok(value.instrument_id),
                is_buy: Ok(value.is_buy),
                is_detached: Ok(value.is_detached),
                is_discounted: Ok(value.is_discounted),
                is_no_stop_loss: Ok(value.is_no_stop_loss),
                is_no_take_profit: Ok(value.is_no_take_profit),
                is_partially_altered: Ok(value.is_partially_altered),
                is_settled: Ok(value.is_settled),
                is_tsl_enabled: Ok(value.is_tsl_enabled),
                leverage: Ok(value.leverage),
                lot_count: Ok(value.lot_count),
                mirror_id: Ok(value.mirror_id),
                open_conversion_rate: Ok(value.open_conversion_rate),
                open_date_time: Ok(value.open_date_time),
                open_position_action_type: Ok(value.open_position_action_type),
                open_rate: Ok(value.open_rate),
                order_id: Ok(value.order_id),
                order_type: Ok(value.order_type),
                parent_position_id: Ok(value.parent_position_id),
                pnl_version: Ok(value.pnl_version),
                position_id: Ok(value.position_id),
                redeem_status_id: Ok(value.redeem_status_id),
                settlement_type_id: Ok(value.settlement_type_id),
                stop_loss_rate: Ok(value.stop_loss_rate),
                stop_loss_version: Ok(value.stop_loss_version),
                take_profit_rate: Ok(value.take_profit_rate),
                total_external_fees: Ok(value.total_external_fees),
                total_external_taxes: Ok(value.total_external_taxes),
                total_fees: Ok(value.total_fees),
                units: Ok(value.units),
                units_base_value_dollars: Ok(value.units_base_value_dollars),
            }
        }
    }
}
