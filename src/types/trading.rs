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
#[doc = "`CreateExitOrderRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"executionType\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"GTC\","]
#[doc = "        \"IOC\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"instrumentId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"positionId\": {"]
#[doc = "      \"description\": \"The ID of the position to close.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"units\": {"]
#[doc = "      \"type\": \"number\","]
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
pub struct CreateExitOrderRequest {
    #[serde(
        rename = "executionType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub execution_type: ::std::option::Option<CreateExitOrderRequestExecutionType>,
    #[serde(
        rename = "instrumentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[doc = "The ID of the position to close."]
    #[serde(
        rename = "positionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub position_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
}
impl ::std::default::Default for CreateExitOrderRequest {
    fn default() -> Self {
        Self {
            execution_type: Default::default(),
            instrument_id: Default::default(),
            position_id: Default::default(),
            units: Default::default(),
        }
    }
}
#[doc = "`CreateExitOrderRequestExecutionType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"GTC\","]
#[doc = "    \"IOC\""]
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
pub enum CreateExitOrderRequestExecutionType {
    #[serde(rename = "GTC")]
    Gtc,
    #[serde(rename = "IOC")]
    Ioc,
}
impl ::std::fmt::Display for CreateExitOrderRequestExecutionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gtc => f.write_str("GTC"),
            Self::Ioc => f.write_str("IOC"),
        }
    }
}
impl ::std::str::FromStr for CreateExitOrderRequestExecutionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "GTC" => Ok(Self::Gtc),
            "IOC" => Ok(Self::Ioc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateExitOrderRequestExecutionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CreateExitOrderRequestExecutionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateExitOrderRequestExecutionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateExitOrderResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"token\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateExitOrderResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for CreateExitOrderResponse {
    fn default() -> Self {
        Self {
            token: Default::default(),
        }
    }
}
#[doc = "`CreateOrderRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"executionType\": {"]
#[doc = "      \"description\": \"The execution type (Good-Till-Canceled or Immediate-Or-Cancel).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"GTC\","]
#[doc = "        \"IOC\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"instrumentId\": {"]
#[doc = "      \"description\": \"The ID of the instrument to trade.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"investment\": {"]
#[doc = "      \"description\": \"The amount of money invested in the trade.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"isBuy\": {"]
#[doc = "      \"description\": \"Indicates if the order is a buy (true) or sell (false).\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isTrailingStopLoss\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"leverage\": {"]
#[doc = "      \"description\": \"The leverage level for the trade.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"limitRate\": {"]
#[doc = "      \"description\": \"The limit price for limit orders.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"orderType\": {"]
#[doc = "      \"description\": \"The type of order (Market or Limit).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"MKT\","]
#[doc = "        \"LMT\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"stopLossPct\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"stopLossRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"takeProfitPct\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"takeProfitRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"units\": {"]
#[doc = "      \"description\": \"The number of units being traded.\","]
#[doc = "      \"type\": \"number\","]
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
pub struct CreateOrderRequest {
    #[doc = "The execution type (Good-Till-Canceled or Immediate-Or-Cancel)."]
    #[serde(
        rename = "executionType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub execution_type: ::std::option::Option<CreateOrderRequestExecutionType>,
    #[doc = "The ID of the instrument to trade."]
    #[serde(
        rename = "instrumentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[doc = "The amount of money invested in the trade."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub investment: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "Indicates if the order is a buy (true) or sell (false)."]
    #[serde(
        rename = "isBuy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_buy: ::std::option::Option<bool>,
    #[serde(
        rename = "isTrailingStopLoss",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_trailing_stop_loss: ::std::option::Option<bool>,
    #[doc = "The leverage level for the trade."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub leverage: ::std::option::Option<i64>,
    #[doc = "The limit price for limit orders."]
    #[serde(
        rename = "limitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub limit_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "The type of order (Market or Limit)."]
    #[serde(
        rename = "orderType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_type: ::std::option::Option<CreateOrderRequestOrderType>,
    #[serde(
        rename = "stopLossPct",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_pct: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "stopLossRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "takeProfitPct",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_pct: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "takeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "The number of units being traded."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
}
impl ::std::default::Default for CreateOrderRequest {
    fn default() -> Self {
        Self {
            execution_type: Default::default(),
            instrument_id: Default::default(),
            investment: Default::default(),
            is_buy: Default::default(),
            is_trailing_stop_loss: Default::default(),
            leverage: Default::default(),
            limit_rate: Default::default(),
            order_type: Default::default(),
            stop_loss_pct: Default::default(),
            stop_loss_rate: Default::default(),
            take_profit_pct: Default::default(),
            take_profit_rate: Default::default(),
            units: Default::default(),
        }
    }
}
#[doc = "The execution type (Good-Till-Canceled or Immediate-Or-Cancel)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The execution type (Good-Till-Canceled or Immediate-Or-Cancel).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"GTC\","]
#[doc = "    \"IOC\""]
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
pub enum CreateOrderRequestExecutionType {
    #[serde(rename = "GTC")]
    Gtc,
    #[serde(rename = "IOC")]
    Ioc,
}
impl ::std::fmt::Display for CreateOrderRequestExecutionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gtc => f.write_str("GTC"),
            Self::Ioc => f.write_str("IOC"),
        }
    }
}
impl ::std::str::FromStr for CreateOrderRequestExecutionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "GTC" => Ok(Self::Gtc),
            "IOC" => Ok(Self::Ioc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateOrderRequestExecutionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CreateOrderRequestExecutionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateOrderRequestExecutionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of order (Market or Limit)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The type of order (Market or Limit).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"MKT\","]
#[doc = "    \"LMT\""]
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
pub enum CreateOrderRequestOrderType {
    #[serde(rename = "MKT")]
    Mkt,
    #[serde(rename = "LMT")]
    Lmt,
}
impl ::std::fmt::Display for CreateOrderRequestOrderType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mkt => f.write_str("MKT"),
            Self::Lmt => f.write_str("LMT"),
        }
    }
}
impl ::std::str::FromStr for CreateOrderRequestOrderType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "MKT" => Ok(Self::Mkt),
            "LMT" => Ok(Self::Lmt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateOrderRequestOrderType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CreateOrderRequestOrderType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateOrderRequestOrderType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateOrderResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"token\": {"]
#[doc = "      \"description\": \"Unique identifier for the operation.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateOrderResponse {
    #[doc = "Unique identifier for the operation."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for CreateOrderResponse {
    fn default() -> Self {
        Self {
            token: Default::default(),
        }
    }
}
#[doc = "`DeleteExitOrderResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"token\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DeleteExitOrderResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for DeleteExitOrderResponse {
    fn default() -> Self {
        Self {
            token: Default::default(),
        }
    }
}
#[doc = "`DeleteOrderResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"token\": {"]
#[doc = "      \"description\": \"Unique identifier for the operation.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DeleteOrderResponse {
    #[doc = "Unique identifier for the operation."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for DeleteOrderResponse {
    fn default() -> Self {
        Self {
            token: Default::default(),
        }
    }
}
#[doc = "`EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"ExternalOperationData\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ExternalOperationType\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"InstrumentId\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"LotsToDeduct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ReferenceID\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"UnitsToDeduct\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ViewRateContext\": {"]
#[doc = "      \"$ref\": \"#/$defs/eToro.Trading.DistributedServices.WebApi.API.DTO.Requests.Rates.ViewRateContextDto\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest
{
    #[serde(
        rename = "ExternalOperationData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_operation_data: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ExternalOperationType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_operation_type: ::std::option::Option<i32>,
    #[serde(
        rename = "InstrumentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i32>,
    #[serde(
        rename = "LotsToDeduct",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lots_to_deduct: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "ReferenceID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reference_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "UnitsToDeduct",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub units_to_deduct: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "ViewRateContext",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub view_rate_context: ::std::option::Option<
        EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto,
    >,
}
impl ::std::default::Default
    for EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest
{
    fn default() -> Self {
        Self {
            external_operation_data: Default::default(),
            external_operation_type: Default::default(),
            instrument_id: Default::default(),
            lots_to_deduct: Default::default(),
            reference_id: Default::default(),
            units_to_deduct: Default::default(),
            view_rate_context: Default::default(),
        }
    }
}
#[doc = "`EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"AdditionalMargin\": {"]
#[doc = "      \"description\": \"Additional Margin for Futures SQF\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"Amount\": {"]
#[doc = "      \"description\": \"USD invested\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"AmountInUnits\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"CID\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"ExternalOperationData\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ExternalOperationType\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"InstrumentID\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"IsBuy\": {"]
#[doc = "      \"description\": \"true=buy, false=sell\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"IsDiscounted\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"IsNoStopLoss\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"IsNoTakeProfit\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"IsTslEnabled\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"Leverage\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"LotCount\": {"]
#[doc = "      \"description\": \"Number of contracts for Futures\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ReferenceID\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"RequestedSettlementTypeID\": {"]
#[doc = "      \"description\": \"Type of settlement\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"StopLossRate\": {"]
#[doc = "      \"description\": \"Exact rate to close for a loss\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"TakeProfitRate\": {"]
#[doc = "      \"description\": \"Exact rate to close for a profit\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"TotalExternalCosts\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ViewRateContext\": {"]
#[doc = "      \"$ref\": \"#/$defs/eToro.Trading.DistributedServices.WebApi.API.DTO.Requests.Rates.ViewRateContextDto\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest {
    #[doc = "Additional Margin for Futures SQF"]
    #[serde(
        rename = "AdditionalMargin",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub additional_margin: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "USD invested"]
    #[serde(
        rename = "Amount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub amount: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "AmountInUnits",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub amount_in_units: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "CID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cid: ::std::option::Option<i32>,
    #[serde(
        rename = "ExternalOperationData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_operation_data: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ExternalOperationType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_operation_type: ::std::option::Option<i32>,
    #[serde(
        rename = "InstrumentID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i32>,
    #[doc = "true=buy, false=sell"]
    #[serde(
        rename = "IsBuy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_buy: ::std::option::Option<bool>,
    #[serde(
        rename = "IsDiscounted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_discounted: ::std::option::Option<bool>,
    #[serde(
        rename = "IsNoStopLoss",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_no_stop_loss: ::std::option::Option<bool>,
    #[serde(
        rename = "IsNoTakeProfit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_no_take_profit: ::std::option::Option<bool>,
    #[serde(
        rename = "IsTslEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_tsl_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "Leverage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub leverage: ::std::option::Option<i32>,
    #[doc = "Number of contracts for Futures"]
    #[serde(
        rename = "LotCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lot_count: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "ReferenceID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reference_id: ::std::option::Option<::std::string::String>,
    #[doc = "Type of settlement"]
    #[serde(
        rename = "RequestedSettlementTypeID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub requested_settlement_type_id: ::std::option::Option<i32>,
    #[doc = "Exact rate to close for a loss"]
    #[serde(
        rename = "StopLossRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "Exact rate to close for a profit"]
    #[serde(
        rename = "TakeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "TotalExternalCosts",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_external_costs: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "ViewRateContext",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub view_rate_context: ::std::option::Option<
        EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto,
    >,
}
impl ::std::default::Default
    for EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest
{
    fn default() -> Self {
        Self {
            additional_margin: Default::default(),
            amount: Default::default(),
            amount_in_units: Default::default(),
            cid: Default::default(),
            external_operation_data: Default::default(),
            external_operation_type: Default::default(),
            instrument_id: Default::default(),
            is_buy: Default::default(),
            is_discounted: Default::default(),
            is_no_stop_loss: Default::default(),
            is_no_take_profit: Default::default(),
            is_tsl_enabled: Default::default(),
            leverage: Default::default(),
            lot_count: Default::default(),
            reference_id: Default::default(),
            requested_settlement_type_id: Default::default(),
            stop_loss_rate: Default::default(),
            take_profit_rate: Default::default(),
            total_external_costs: Default::default(),
            view_rate_context: Default::default(),
        }
    }
}
#[doc = "`EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"Amount\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"AmountInUnits\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"CID\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"InstrumentID\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"IsBuy\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"IsDiscounted\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"IsNoStopLoss\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"IsNoTakeProfit\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"IsTslEnabled\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"Leverage\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"Rate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"StopLossRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"TakeProfitRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest {
    #[serde(
        rename = "Amount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub amount: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "AmountInUnits",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub amount_in_units: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "CID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cid: ::std::option::Option<i32>,
    #[serde(
        rename = "InstrumentID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i32>,
    #[serde(
        rename = "IsBuy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_buy: ::std::option::Option<bool>,
    #[serde(
        rename = "IsDiscounted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_discounted: ::std::option::Option<bool>,
    #[serde(
        rename = "IsNoStopLoss",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_no_stop_loss: ::std::option::Option<bool>,
    #[serde(
        rename = "IsNoTakeProfit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_no_take_profit: ::std::option::Option<bool>,
    #[serde(
        rename = "IsTslEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_tsl_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "Leverage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub leverage: ::std::option::Option<i32>,
    #[serde(
        rename = "Rate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "StopLossRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "TakeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
}
impl ::std::default::Default
    for EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest
{
    fn default() -> Self {
        Self {
            amount: Default::default(),
            amount_in_units: Default::default(),
            cid: Default::default(),
            instrument_id: Default::default(),
            is_buy: Default::default(),
            is_discounted: Default::default(),
            is_no_stop_loss: Default::default(),
            is_no_take_profit: Default::default(),
            is_tsl_enabled: Default::default(),
            leverage: Default::default(),
            rate: Default::default(),
            stop_loss_rate: Default::default(),
            take_profit_rate: Default::default(),
        }
    }
}
#[doc = "Used to bypass view rates from client"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Used to bypass view rates from client\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"ClientRateForCalc\": {"]
#[doc = "      \"description\": \"Calculated rate passed from client\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ClientRateForCalcID\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"ClientViewRate\": {"]
#[doc = "      \"description\": \"Default client view rate\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ClientViewRateID\": {"]
#[doc = "      \"description\": \"View rate id from rates\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"PriceType\": {"]
#[doc = "      \"description\": \"Snapshot or real-time\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"SnapshotTimestamp\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto {
    #[doc = "Calculated rate passed from client"]
    #[serde(
        rename = "ClientRateForCalc",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_rate_for_calc: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "ClientRateForCalcID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_rate_for_calc_id: ::std::option::Option<i64>,
    #[doc = "Default client view rate"]
    #[serde(
        rename = "ClientViewRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_view_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "View rate id from rates"]
    #[serde(
        rename = "ClientViewRateID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_view_rate_id: ::std::option::Option<i64>,
    #[doc = "Snapshot or real-time"]
    #[serde(
        rename = "PriceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub price_type: ::std::option::Option<i32>,
    #[serde(
        rename = "SnapshotTimestamp",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub snapshot_timestamp: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
}
impl ::std::default::Default
    for EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto
{
    fn default() -> Self {
        Self {
            client_rate_for_calc: Default::default(),
            client_rate_for_calc_id: Default::default(),
            client_view_rate: Default::default(),
            client_view_rate_id: Default::default(),
            price_type: Default::default(),
            snapshot_timestamp: Default::default(),
        }
    }
}
#[doc = "`GetExitOrderResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"openDateTime\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"orderID\": {"]
#[doc = "      \"description\": \"Note CAPITAL ID — different from orderId on getOrderResponse\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"positionId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"socialTradeId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetExitOrderResponse {
    #[serde(
        rename = "openDateTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_date_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Note CAPITAL ID — different from orderId on getOrderResponse"]
    #[serde(
        rename = "orderID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_id: ::std::option::Option<i64>,
    #[serde(
        rename = "positionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub position_id: ::std::option::Option<i64>,
    #[serde(
        rename = "socialTradeId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub social_trade_id: ::std::option::Option<i64>,
}
impl ::std::default::Default for GetExitOrderResponse {
    fn default() -> Self {
        Self {
            open_date_time: Default::default(),
            order_id: Default::default(),
            position_id: Default::default(),
            social_trade_id: Default::default(),
        }
    }
}
#[doc = "`GetOrderResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"executionType\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"GTC\","]
#[doc = "        \"IOC\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"instrumentId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"investment\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"isBuy\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isEntry\": {"]
#[doc = "      \"description\": \"Indicates if the order is an entry or exit order.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"leverage\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"limitRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"openTimestamp\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"orderId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"orderType\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"MKT\","]
#[doc = "        \"LMT\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"stopLossRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"takeProfitRate\": {"]
#[doc = "      \"type\": \"number\","]
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
pub struct GetOrderResponse {
    #[serde(
        rename = "executionType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub execution_type: ::std::option::Option<GetOrderResponseExecutionType>,
    #[serde(
        rename = "instrumentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub investment: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "isBuy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_buy: ::std::option::Option<bool>,
    #[doc = "Indicates if the order is an entry or exit order."]
    #[serde(
        rename = "isEntry",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_entry: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub leverage: ::std::option::Option<i64>,
    #[serde(
        rename = "limitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub limit_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "openTimestamp",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_timestamp: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
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
    pub order_type: ::std::option::Option<GetOrderResponseOrderType>,
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
}
impl ::std::default::Default for GetOrderResponse {
    fn default() -> Self {
        Self {
            execution_type: Default::default(),
            instrument_id: Default::default(),
            investment: Default::default(),
            is_buy: Default::default(),
            is_entry: Default::default(),
            leverage: Default::default(),
            limit_rate: Default::default(),
            open_timestamp: Default::default(),
            order_id: Default::default(),
            order_type: Default::default(),
            stop_loss_rate: Default::default(),
            take_profit_rate: Default::default(),
        }
    }
}
#[doc = "`GetOrderResponseExecutionType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"GTC\","]
#[doc = "    \"IOC\""]
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
pub enum GetOrderResponseExecutionType {
    #[serde(rename = "GTC")]
    Gtc,
    #[serde(rename = "IOC")]
    Ioc,
}
impl ::std::fmt::Display for GetOrderResponseExecutionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gtc => f.write_str("GTC"),
            Self::Ioc => f.write_str("IOC"),
        }
    }
}
impl ::std::str::FromStr for GetOrderResponseExecutionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "GTC" => Ok(Self::Gtc),
            "IOC" => Ok(Self::Ioc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GetOrderResponseExecutionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GetOrderResponseExecutionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetOrderResponseExecutionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GetOrderResponseOrderType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"MKT\","]
#[doc = "    \"LMT\""]
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
pub enum GetOrderResponseOrderType {
    #[serde(rename = "MKT")]
    Mkt,
    #[serde(rename = "LMT")]
    Lmt,
}
impl ::std::fmt::Display for GetOrderResponseOrderType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mkt => f.write_str("MKT"),
            Self::Lmt => f.write_str("LMT"),
        }
    }
}
impl ::std::str::FromStr for GetOrderResponseOrderType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "MKT" => Ok(Self::Mkt),
            "LMT" => Ok(Self::Lmt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GetOrderResponseOrderType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GetOrderResponseOrderType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetOrderResponseOrderType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"stopLossPercentage\": {"]
#[doc = "      \"description\": \"% of mirror value that StopLossAmount represented at last edit\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"withdrawalSummary\": {"]
#[doc = "      \"description\": \"Total USD withdrawn from the mirror\","]
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
pub struct Mirror {
    #[doc = "Available USD balance reserved for mirror operations"]
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
    pub deposit_summary: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub initial_investment: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub stop_loss_amount: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "% of mirror value that StopLossAmount represented at last edit"]
    #[serde(
        rename = "stopLossPercentage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_percentage: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "Total USD withdrawn from the mirror"]
    #[serde(
        rename = "withdrawalSummary",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub withdrawal_summary: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
#[doc = "      \"description\": \"Units to open. If > 0, position opens on units, not amount\","]
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
pub struct Order {
    #[doc = "USD amount"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    #[doc = "Units to open. If > 0, position opens on units, not amount"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub lots_to_deduct: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub units_to_deduct: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub lots_to_deduct: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub units_to_deduct: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"amountInUnits\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
#[doc = "    \"totalExternalCosts\": {"]
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
pub struct OrderForOpen {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "amountInUnits",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub amount_in_units: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub frozen_amount: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub lot_count: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub stop_loss_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "takeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "totalExternalCosts",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_external_costs: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
#[doc = "Comprehensive order info containing details and all positions opened from this order"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Comprehensive order info containing details and all positions opened from this order\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"CID\","]
#[doc = "    \"instrumentID\","]
#[doc = "    \"orderID\","]
#[doc = "    \"orderType\","]
#[doc = "    \"requestOccurred\","]
#[doc = "    \"statusID\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"CID\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"amount\": {"]
#[doc = "      \"description\": \"USD amount requested\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"errorCode\": {"]
#[doc = "      \"description\": \"Null on success\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"errorMessage\": {"]
#[doc = "      \"description\": \"Null on success\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"instrumentID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"openActionType\": {"]
#[doc = "      \"description\": \"Reason/context for opening (manual, copy, automated, etc.)\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"orderID\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"orderType\": {"]
#[doc = "      \"description\": \"1=Market, 2=Limit, 3=Stop (system-specific)\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"positions\": {"]
#[doc = "      \"description\": \"Empty if order not yet executed or execution failed\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/OrderForOpenPositionInfo\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"referenceID\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"requestOccurred\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"statusID\": {"]
#[doc = "      \"description\": \"0=Pending, 1=Executed, 2=Cancelled, 3=Rejected, 4=Partially Executed (system-specific)\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"token\": {"]
#[doc = "      \"description\": \"Tracking token for the request, used for correlation\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"units\": {"]
#[doc = "      \"type\": \"number\","]
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
pub struct OrderForOpenInfoResponse {
    #[doc = "USD amount requested"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(rename = "CID")]
    pub cid: i64,
    #[doc = "Null on success"]
    #[serde(
        rename = "errorCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub error_code: ::std::option::Option<i64>,
    #[doc = "Null on success"]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instrumentID")]
    pub instrument_id: i64,
    #[doc = "Reason/context for opening (manual, copy, automated, etc.)"]
    #[serde(
        rename = "openActionType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_action_type: ::std::option::Option<i64>,
    #[serde(rename = "orderID")]
    pub order_id: i64,
    #[doc = "1=Market, 2=Limit, 3=Stop (system-specific)"]
    #[serde(rename = "orderType")]
    pub order_type: i64,
    #[doc = "Empty if order not yet executed or execution failed"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub positions: ::std::vec::Vec<OrderForOpenPositionInfo>,
    #[serde(
        rename = "referenceID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reference_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestOccurred")]
    pub request_occurred: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "0=Pending, 1=Executed, 2=Cancelled, 3=Rejected, 4=Partially Executed (system-specific)"]
    #[serde(rename = "statusID")]
    pub status_id: i64,
    #[doc = "Tracking token for the request, used for correlation"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
}
#[doc = "Detailed information about a position opened from an order"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Detailed information about a position opened from an order\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"amount\","]
#[doc = "    \"isOpen\","]
#[doc = "    \"occurred\","]
#[doc = "    \"orderType\","]
#[doc = "    \"positionID\","]
#[doc = "    \"rate\","]
#[doc = "    \"units\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"amount\": {"]
#[doc = "      \"description\": \"USD amount invested in this position\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"conversionRate\": {"]
#[doc = "      \"description\": \"Currency conversion rate at execution (instrument's base → account currency, typically USD)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"isOpen\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"occurred\": {"]
#[doc = "      \"description\": \"Exact timestamp when this position was opened (ISO 8601 UTC)\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"orderType\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"positionID\": {"]
#[doc = "      \"description\": \"Primary identifier for the position in the trading system\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"rate\": {"]
#[doc = "      \"description\": \"Execution rate (price) — may differ from requested rate\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"units\": {"]
#[doc = "      \"type\": \"number\","]
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
pub struct OrderForOpenPositionInfo {
    #[doc = "USD amount invested in this position"]
    pub amount: ::etoro_agent::types::manual::Numeric,
    #[doc = "Currency conversion rate at execution (instrument's base → account currency, typically USD)"]
    #[serde(
        rename = "conversionRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub conversion_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
    #[doc = "Exact timestamp when this position was opened (ISO 8601 UTC)"]
    pub occurred: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "orderType")]
    pub order_type: i64,
    #[doc = "Primary identifier for the position in the trading system"]
    #[serde(rename = "positionID")]
    pub position_id: i64,
    #[doc = "Execution rate (price) — may differ from requested rate"]
    pub rate: ::etoro_agent::types::manual::Numeric,
    pub units: ::etoro_agent::types::manual::Numeric,
}
#[doc = "`OrderMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"direction\": {"]
#[doc = "      \"$ref\": \"#/$defs/TradeDirection\""]
#[doc = "    },"]
#[doc = "    \"market\": {"]
#[doc = "      \"$ref\": \"#/$defs/Market\""]
#[doc = "    },"]
#[doc = "    \"orderId\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
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
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/TradeType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OrderMetadata {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub direction: ::std::option::Option<::etoro_agent::types::manual::TradeDirection>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub market: ::std::option::Option<::etoro_agent::types::market_data::Market>,
    #[serde(
        rename = "orderId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::etoro_agent::types::manual::TradeType>,
}
impl ::std::default::Default for OrderMetadata {
    fn default() -> Self {
        Self {
            direction: Default::default(),
            market: Default::default(),
            order_id: Default::default(),
            rate: Default::default(),
            type_: Default::default(),
        }
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
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"initialAmountInDollars\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"initialUnits\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"lotCount\": {"]
#[doc = "      \"description\": \"For FutureContracts = number of contracts acquired\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"mirrorID\": {"]
#[doc = "      \"description\": \"Mirror ID if part of copy trading, 0 otherwise\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"openConversionRate\": {"]
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
#[doc = "    \"openPositionActionType\": {"]
#[doc = "      \"description\": \"Position open reason\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"openRate\": {"]
#[doc = "      \"description\": \"Entry price in instrument's currency\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"stopLossVersion\": {"]
#[doc = "      \"description\": \"Increments each time StopLossRate is manually updated\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"takeProfitRate\": {"]
#[doc = "      \"description\": \"Trigger price; must be better than current price\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"totalExternalFees\": {"]
#[doc = "      \"description\": \"USD fees (e.g. TicketFee). Excludes overnight fees and dividends\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"totalExternalTaxes\": {"]
#[doc = "      \"description\": \"USD taxes (e.g. SDRT)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"totalFees\": {"]
#[doc = "      \"description\": \"Total overnight fees and dividends in USD. Negative = refund\","]
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
#[doc = "    },"]
#[doc = "    \"unitsBaseValueDollars\": {"]
#[doc = "      \"description\": \"Current units invested value in USD\","]
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
pub struct Position {
    #[doc = "USD allocated (initial investment + additional margin collateral)"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub initial_amount_in_dollars: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "initialUnits",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub initial_units: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub leverage: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "For FutureContracts = number of contracts acquired"]
    #[serde(
        rename = "lotCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lot_count: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub open_conversion_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub open_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub stop_loss_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub take_profit_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "USD fees (e.g. TicketFee). Excludes overnight fees and dividends"]
    #[serde(
        rename = "totalExternalFees",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_external_fees: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "USD taxes (e.g. SDRT)"]
    #[serde(
        rename = "totalExternalTaxes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_external_taxes: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "Total overnight fees and dividends in USD. Negative = refund"]
    #[serde(
        rename = "totalFees",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_fees: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "Current units invested value in USD"]
    #[serde(
        rename = "unitsBaseValueDollars",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub units_base_value_dollars: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
#[doc = "`PutTradeRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"isTrailingStopLoss\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"positionId\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"stopLossRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"takeProfitRate\": {"]
#[doc = "      \"type\": \"number\","]
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
pub struct PutTradeRequest {
    #[serde(
        rename = "isTrailingStopLoss",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_trailing_stop_loss: ::std::option::Option<bool>,
    #[serde(
        rename = "positionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub position_id: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
}
impl ::std::default::Default for PutTradeRequest {
    fn default() -> Self {
        Self {
            is_trailing_stop_loss: Default::default(),
            position_id: Default::default(),
            stop_loss_rate: Default::default(),
            take_profit_rate: Default::default(),
        }
    }
}
#[doc = "`TradeMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"direction\": {"]
#[doc = "      \"$ref\": \"#/$defs/TradeDirection\""]
#[doc = "    },"]
#[doc = "    \"gain\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"market\": {"]
#[doc = "      \"$ref\": \"#/$defs/Market\""]
#[doc = "    },"]
#[doc = "    \"positionId\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
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
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/TradeType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TradeMetadata {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub direction: ::std::option::Option<::etoro_agent::types::manual::TradeDirection>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gain: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub market: ::std::option::Option<::etoro_agent::types::market_data::Market>,
    #[serde(
        rename = "positionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub position_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::etoro_agent::types::manual::TradeType>,
}
impl ::std::default::Default for TradeMetadata {
    fn default() -> Self {
        Self {
            direction: Default::default(),
            gain: Default::default(),
            market: Default::default(),
            position_id: Default::default(),
            rate: Default::default(),
            type_: Default::default(),
        }
    }
}
