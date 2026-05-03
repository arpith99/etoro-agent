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
#[doc = "`ApplicationSource`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"int32\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"eToro\","]
#[doc = "    \"Delta\","]
#[doc = "    \"Gatsby\""]
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
pub enum ApplicationSource {
    #[serde(rename = "eToro")]
    EToro,
    Delta,
    Gatsby,
}
impl ::std::fmt::Display for ApplicationSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::EToro => f.write_str("eToro"),
            Self::Delta => f.write_str("Delta"),
            Self::Gatsby => f.write_str("Gatsby"),
        }
    }
}
impl ::std::str::FromStr for ApplicationSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "eToro" => Ok(Self::EToro),
            "Delta" => Ok(Self::Delta),
            "Gatsby" => Ok(Self::Gatsby),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplicationSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ApplicationSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplicationSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Avatar`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"large\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"medium\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"small\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"svg\": {"]
#[doc = "      \"$ref\": \"#/$defs/Svg\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Avatar {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub large: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub medium: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub small: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub svg: ::std::option::Option<Svg>,
}
impl ::std::default::Default for Avatar {
    fn default() -> Self {
        Self {
            large: Default::default(),
            medium: Default::default(),
            small: Default::default(),
            svg: Default::default(),
        }
    }
}
impl Avatar {
    pub fn builder() -> builder::Avatar {
        Default::default()
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
#[doc = "      \"type\": \"number\""]
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
    pub units: ::std::option::Option<f64>,
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
impl CreateExitOrderRequest {
    pub fn builder() -> builder::CreateExitOrderRequest {
        Default::default()
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
impl CreateExitOrderResponse {
    pub fn builder() -> builder::CreateExitOrderResponse {
        Default::default()
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
#[doc = "      \"type\": \"number\""]
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
#[doc = "      \"type\": \"number\""]
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
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"stopLossRate\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"takeProfitPct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"takeProfitRate\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"units\": {"]
#[doc = "      \"description\": \"The number of units being traded.\","]
#[doc = "      \"type\": \"number\""]
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
    pub investment: ::std::option::Option<f64>,
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
    pub limit_rate: ::std::option::Option<f64>,
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
    pub stop_loss_pct: ::std::option::Option<f64>,
    #[serde(
        rename = "stopLossRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_rate: ::std::option::Option<f64>,
    #[serde(
        rename = "takeProfitPct",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_pct: ::std::option::Option<f64>,
    #[serde(
        rename = "takeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<f64>,
    #[doc = "The number of units being traded."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<f64>,
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
impl CreateOrderRequest {
    pub fn builder() -> builder::CreateOrderRequest {
        Default::default()
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
impl CreateOrderResponse {
    pub fn builder() -> builder::CreateOrderResponse {
        Default::default()
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
impl DeleteExitOrderResponse {
    pub fn builder() -> builder::DeleteExitOrderResponse {
        Default::default()
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
impl DeleteOrderResponse {
    pub fn builder() -> builder::DeleteOrderResponse {
        Default::default()
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
#[doc = "      \"format\": \"double\""]
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
#[doc = "      \"format\": \"double\""]
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
    pub lots_to_deduct: ::std::option::Option<f64>,
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
    pub units_to_deduct: ::std::option::Option<f64>,
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
impl EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest {
    pub fn builder () -> builder :: EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest{
        Default::default()
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
#[doc = "      \"format\": \"double\""]
#[doc = "    },"]
#[doc = "    \"Amount\": {"]
#[doc = "      \"description\": \"USD invested\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"double\""]
#[doc = "    },"]
#[doc = "    \"AmountInUnits\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"double\""]
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
#[doc = "      \"format\": \"double\""]
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
#[doc = "      \"format\": \"double\""]
#[doc = "    },"]
#[doc = "    \"TakeProfitRate\": {"]
#[doc = "      \"description\": \"Exact rate to close for a profit\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\""]
#[doc = "    },"]
#[doc = "    \"TotalExternalCosts\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\""]
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
    pub additional_margin: ::std::option::Option<f64>,
    #[doc = "USD invested"]
    #[serde(
        rename = "Amount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub amount: ::std::option::Option<f64>,
    #[serde(
        rename = "AmountInUnits",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub amount_in_units: ::std::option::Option<f64>,
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
    pub lot_count: ::std::option::Option<f64>,
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
    pub stop_loss_rate: ::std::option::Option<f64>,
    #[doc = "Exact rate to close for a profit"]
    #[serde(
        rename = "TakeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<f64>,
    #[serde(
        rename = "TotalExternalCosts",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_external_costs: ::std::option::Option<f64>,
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
impl EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest {
    pub fn builder(
    ) -> builder::EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest
    {
        Default::default()
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
#[doc = "      \"format\": \"double\""]
#[doc = "    },"]
#[doc = "    \"AmountInUnits\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"double\""]
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
#[doc = "      \"format\": \"double\""]
#[doc = "    },"]
#[doc = "    \"StopLossRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\""]
#[doc = "    },"]
#[doc = "    \"TakeProfitRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\""]
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
    pub amount: ::std::option::Option<f64>,
    #[serde(
        rename = "AmountInUnits",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub amount_in_units: ::std::option::Option<f64>,
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
    pub rate: ::std::option::Option<f64>,
    #[serde(
        rename = "StopLossRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_rate: ::std::option::Option<f64>,
    #[serde(
        rename = "TakeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<f64>,
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
impl EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest {
    pub fn builder(
    ) -> builder::EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest {
        Default::default()
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
#[doc = "      \"format\": \"double\""]
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
#[doc = "      \"format\": \"double\""]
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
    pub client_rate_for_calc: ::std::option::Option<f64>,
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
    pub client_view_rate: ::std::option::Option<f64>,
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
impl EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto {
    pub fn builder(
    ) -> builder::EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto {
        Default::default()
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
impl GetExitOrderResponse {
    pub fn builder() -> builder::GetExitOrderResponse {
        Default::default()
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
#[doc = "      \"type\": \"number\""]
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
#[doc = "      \"type\": \"number\""]
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
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"takeProfitRate\": {"]
#[doc = "      \"type\": \"number\""]
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
    pub investment: ::std::option::Option<f64>,
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
    pub limit_rate: ::std::option::Option<f64>,
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
    pub stop_loss_rate: ::std::option::Option<f64>,
    #[serde(
        rename = "takeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<f64>,
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
impl GetOrderResponse {
    pub fn builder() -> builder::GetOrderResponse {
        Default::default()
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
#[doc = "`Market`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"application\": {"]
#[doc = "      \"$ref\": \"#/$defs/ApplicationSource\""]
#[doc = "    },"]
#[doc = "    \"assetType\": {"]
#[doc = "      \"$ref\": \"#/$defs/MarketAssetType\""]
#[doc = "    },"]
#[doc = "    \"assetTypeId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"assetTypeSubCategoryId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"avatar\": {"]
#[doc = "      \"$ref\": \"#/$defs/Avatar\""]
#[doc = "    },"]
#[doc = "    \"displayName\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"internalId\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"symbolName\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"updated\": {"]
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
pub struct Market {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub application: ::std::option::Option<ApplicationSource>,
    #[serde(
        rename = "assetType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub asset_type: ::std::option::Option<MarketAssetType>,
    #[serde(
        rename = "assetTypeId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub asset_type_id: ::std::option::Option<i32>,
    #[serde(
        rename = "assetTypeSubCategoryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub asset_type_sub_category_id: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<Avatar>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "internalId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internal_id: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "symbolName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub symbol_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub updated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
}
impl ::std::default::Default for Market {
    fn default() -> Self {
        Self {
            application: Default::default(),
            asset_type: Default::default(),
            asset_type_id: Default::default(),
            asset_type_sub_category_id: Default::default(),
            avatar: Default::default(),
            display_name: Default::default(),
            id: Default::default(),
            internal_id: Default::default(),
            metadata: Default::default(),
            symbol_name: Default::default(),
            updated: Default::default(),
        }
    }
}
impl Market {
    pub fn builder() -> builder::Market {
        Default::default()
    }
}
#[doc = "Enum encoded as integer index; values listed below"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Enum encoded as integer index; values listed below\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"int32\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Stocks\","]
#[doc = "    \"Bonds\","]
#[doc = "    \"ETF\","]
#[doc = "    \"Index\","]
#[doc = "    \"Warrants\","]
#[doc = "    \"Options\","]
#[doc = "    \"Futures\","]
#[doc = "    \"CFD\","]
#[doc = "    \"TRS\","]
#[doc = "    \"FOREX\","]
#[doc = "    \"CommodityMetals\","]
#[doc = "    \"CommodityEnergyAgriculture\","]
#[doc = "    \"CryptoCoin\","]
#[doc = "    \"NFT\""]
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
pub enum MarketAssetType {
    Stocks,
    Bonds,
    #[serde(rename = "ETF")]
    Etf,
    Index,
    Warrants,
    Options,
    Futures,
    #[serde(rename = "CFD")]
    Cfd,
    #[serde(rename = "TRS")]
    Trs,
    #[serde(rename = "FOREX")]
    Forex,
    CommodityMetals,
    CommodityEnergyAgriculture,
    CryptoCoin,
    #[serde(rename = "NFT")]
    Nft,
}
impl ::std::fmt::Display for MarketAssetType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Stocks => f.write_str("Stocks"),
            Self::Bonds => f.write_str("Bonds"),
            Self::Etf => f.write_str("ETF"),
            Self::Index => f.write_str("Index"),
            Self::Warrants => f.write_str("Warrants"),
            Self::Options => f.write_str("Options"),
            Self::Futures => f.write_str("Futures"),
            Self::Cfd => f.write_str("CFD"),
            Self::Trs => f.write_str("TRS"),
            Self::Forex => f.write_str("FOREX"),
            Self::CommodityMetals => f.write_str("CommodityMetals"),
            Self::CommodityEnergyAgriculture => f.write_str("CommodityEnergyAgriculture"),
            Self::CryptoCoin => f.write_str("CryptoCoin"),
            Self::Nft => f.write_str("NFT"),
        }
    }
}
impl ::std::str::FromStr for MarketAssetType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Stocks" => Ok(Self::Stocks),
            "Bonds" => Ok(Self::Bonds),
            "ETF" => Ok(Self::Etf),
            "Index" => Ok(Self::Index),
            "Warrants" => Ok(Self::Warrants),
            "Options" => Ok(Self::Options),
            "Futures" => Ok(Self::Futures),
            "CFD" => Ok(Self::Cfd),
            "TRS" => Ok(Self::Trs),
            "FOREX" => Ok(Self::Forex),
            "CommodityMetals" => Ok(Self::CommodityMetals),
            "CommodityEnergyAgriculture" => Ok(Self::CommodityEnergyAgriculture),
            "CryptoCoin" => Ok(Self::CryptoCoin),
            "NFT" => Ok(Self::Nft),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MarketAssetType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MarketAssetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MarketAssetType {
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
#[doc = "      \"type\": \"number\""]
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
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OrderForOpenInfoResponse {
    #[doc = "USD amount requested"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<f64>,
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
    pub units: ::std::option::Option<f64>,
}
impl OrderForOpenInfoResponse {
    pub fn builder() -> builder::OrderForOpenInfoResponse {
        Default::default()
    }
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
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"conversionRate\": {"]
#[doc = "      \"description\": \"Currency conversion rate at execution (instrument's base → account currency, typically USD)\","]
#[doc = "      \"type\": \"number\""]
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
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"units\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OrderForOpenPositionInfo {
    #[doc = "USD amount invested in this position"]
    pub amount: f64,
    #[doc = "Currency conversion rate at execution (instrument's base → account currency, typically USD)"]
    #[serde(
        rename = "conversionRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub conversion_rate: ::std::option::Option<f64>,
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
    pub rate: f64,
    pub units: f64,
}
impl OrderForOpenPositionInfo {
    pub fn builder() -> builder::OrderForOpenPositionInfo {
        Default::default()
    }
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
#[doc = "      \"format\": \"float\""]
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
    pub direction: ::std::option::Option<TradeDirection>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub market: ::std::option::Option<Market>,
    #[serde(
        rename = "orderId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub order_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rate: ::std::option::Option<f32>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<TradeType>,
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
impl OrderMetadata {
    pub fn builder() -> builder::OrderMetadata {
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
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"stopLossRate\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"takeProfitRate\": {"]
#[doc = "      \"type\": \"number\""]
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
    pub position_id: ::std::option::Option<f64>,
    #[serde(
        rename = "stopLossRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_loss_rate: ::std::option::Option<f64>,
    #[serde(
        rename = "takeProfitRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub take_profit_rate: ::std::option::Option<f64>,
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
impl PutTradeRequest {
    pub fn builder() -> builder::PutTradeRequest {
        Default::default()
    }
}
#[doc = "`Svg`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"backgroundColor\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"textColor\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Svg {
    #[serde(
        rename = "backgroundColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub background_color: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "textColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub text_color: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for Svg {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            text_color: Default::default(),
            url: Default::default(),
        }
    }
}
impl Svg {
    pub fn builder() -> builder::Svg {
        Default::default()
    }
}
#[doc = "Integer-encoded enum"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Integer-encoded enum\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"int32\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Long\","]
#[doc = "    \"Short\""]
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
pub enum TradeDirection {
    Long,
    Short,
}
impl ::std::fmt::Display for TradeDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Long => f.write_str("Long"),
            Self::Short => f.write_str("Short"),
        }
    }
}
impl ::std::str::FromStr for TradeDirection {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Long" => Ok(Self::Long),
            "Short" => Ok(Self::Short),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TradeDirection {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TradeDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TradeDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "      \"format\": \"float\""]
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
#[doc = "      \"format\": \"float\""]
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
    pub direction: ::std::option::Option<TradeDirection>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gain: ::std::option::Option<f32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub market: ::std::option::Option<Market>,
    #[serde(
        rename = "positionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub position_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rate: ::std::option::Option<f32>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<TradeType>,
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
impl TradeMetadata {
    pub fn builder() -> builder::TradeMetadata {
        Default::default()
    }
}
#[doc = "Integer-encoded enum"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Integer-encoded enum\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"int32\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Open\","]
#[doc = "    \"Close\""]
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
pub enum TradeType {
    Open,
    Close,
}
impl ::std::fmt::Display for TradeType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Open => f.write_str("Open"),
            Self::Close => f.write_str("Close"),
        }
    }
}
impl ::std::str::FromStr for TradeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Open" => Ok(Self::Open),
            "Close" => Ok(Self::Close),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TradeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TradeType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TradeType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Avatar {
        large: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        medium: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        small: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        svg: ::std::result::Result<::std::option::Option<super::Svg>, ::std::string::String>,
    }
    impl ::std::default::Default for Avatar {
        fn default() -> Self {
            Self {
                large: Ok(Default::default()),
                medium: Ok(Default::default()),
                small: Ok(Default::default()),
                svg: Ok(Default::default()),
            }
        }
    }
    impl Avatar {
        pub fn large<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.large = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for large: {e}"));
            self
        }
        pub fn medium<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.medium = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for medium: {e}"));
            self
        }
        pub fn small<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.small = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for small: {e}"));
            self
        }
        pub fn svg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Svg>>,
            T::Error: ::std::fmt::Display,
        {
            self.svg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for svg: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Avatar> for super::Avatar {
        type Error = super::error::ConversionError;
        fn try_from(value: Avatar) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                large: value.large?,
                medium: value.medium?,
                small: value.small?,
                svg: value.svg?,
            })
        }
    }
    impl ::std::convert::From<super::Avatar> for Avatar {
        fn from(value: super::Avatar) -> Self {
            Self {
                large: Ok(value.large),
                medium: Ok(value.medium),
                small: Ok(value.small),
                svg: Ok(value.svg),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateExitOrderRequest {
        execution_type: ::std::result::Result<
            ::std::option::Option<super::CreateExitOrderRequestExecutionType>,
            ::std::string::String,
        >,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        position_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        units: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateExitOrderRequest {
        fn default() -> Self {
            Self {
                execution_type: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                position_id: Ok(Default::default()),
                units: Ok(Default::default()),
            }
        }
    }
    impl CreateExitOrderRequest {
        pub fn execution_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateExitOrderRequestExecutionType>,
            >,
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
        pub fn units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for units: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateExitOrderRequest> for super::CreateExitOrderRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateExitOrderRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                execution_type: value.execution_type?,
                instrument_id: value.instrument_id?,
                position_id: value.position_id?,
                units: value.units?,
            })
        }
    }
    impl ::std::convert::From<super::CreateExitOrderRequest> for CreateExitOrderRequest {
        fn from(value: super::CreateExitOrderRequest) -> Self {
            Self {
                execution_type: Ok(value.execution_type),
                instrument_id: Ok(value.instrument_id),
                position_id: Ok(value.position_id),
                units: Ok(value.units),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateExitOrderResponse {
        token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateExitOrderResponse {
        fn default() -> Self {
            Self {
                token: Ok(Default::default()),
            }
        }
    }
    impl CreateExitOrderResponse {
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateExitOrderResponse> for super::CreateExitOrderResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateExitOrderResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                token: value.token?,
            })
        }
    }
    impl ::std::convert::From<super::CreateExitOrderResponse> for CreateExitOrderResponse {
        fn from(value: super::CreateExitOrderResponse) -> Self {
            Self {
                token: Ok(value.token),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateOrderRequest {
        execution_type: ::std::result::Result<
            ::std::option::Option<super::CreateOrderRequestExecutionType>,
            ::std::string::String,
        >,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        investment: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        is_buy: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_trailing_stop_loss:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        leverage: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        limit_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        order_type: ::std::result::Result<
            ::std::option::Option<super::CreateOrderRequestOrderType>,
            ::std::string::String,
        >,
        stop_loss_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        stop_loss_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        take_profit_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        take_profit_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        units: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateOrderRequest {
        fn default() -> Self {
            Self {
                execution_type: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                investment: Ok(Default::default()),
                is_buy: Ok(Default::default()),
                is_trailing_stop_loss: Ok(Default::default()),
                leverage: Ok(Default::default()),
                limit_rate: Ok(Default::default()),
                order_type: Ok(Default::default()),
                stop_loss_pct: Ok(Default::default()),
                stop_loss_rate: Ok(Default::default()),
                take_profit_pct: Ok(Default::default()),
                take_profit_rate: Ok(Default::default()),
                units: Ok(Default::default()),
            }
        }
    }
    impl CreateOrderRequest {
        pub fn execution_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateOrderRequestExecutionType>,
            >,
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
        pub fn investment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.investment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for investment: {e}"));
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
        pub fn is_trailing_stop_loss<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_trailing_stop_loss = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_trailing_stop_loss: {e}")
            });
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
        pub fn limit_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit_rate: {e}"));
            self
        }
        pub fn order_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CreateOrderRequestOrderType>>,
            T::Error: ::std::fmt::Display,
        {
            self.order_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_type: {e}"));
            self
        }
        pub fn stop_loss_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_loss_pct: {e}"));
            self
        }
        pub fn stop_loss_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_loss_rate: {e}"));
            self
        }
        pub fn take_profit_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.take_profit_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for take_profit_pct: {e}"));
            self
        }
        pub fn take_profit_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.take_profit_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for take_profit_rate: {e}"));
            self
        }
        pub fn units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for units: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateOrderRequest> for super::CreateOrderRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateOrderRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                execution_type: value.execution_type?,
                instrument_id: value.instrument_id?,
                investment: value.investment?,
                is_buy: value.is_buy?,
                is_trailing_stop_loss: value.is_trailing_stop_loss?,
                leverage: value.leverage?,
                limit_rate: value.limit_rate?,
                order_type: value.order_type?,
                stop_loss_pct: value.stop_loss_pct?,
                stop_loss_rate: value.stop_loss_rate?,
                take_profit_pct: value.take_profit_pct?,
                take_profit_rate: value.take_profit_rate?,
                units: value.units?,
            })
        }
    }
    impl ::std::convert::From<super::CreateOrderRequest> for CreateOrderRequest {
        fn from(value: super::CreateOrderRequest) -> Self {
            Self {
                execution_type: Ok(value.execution_type),
                instrument_id: Ok(value.instrument_id),
                investment: Ok(value.investment),
                is_buy: Ok(value.is_buy),
                is_trailing_stop_loss: Ok(value.is_trailing_stop_loss),
                leverage: Ok(value.leverage),
                limit_rate: Ok(value.limit_rate),
                order_type: Ok(value.order_type),
                stop_loss_pct: Ok(value.stop_loss_pct),
                stop_loss_rate: Ok(value.stop_loss_rate),
                take_profit_pct: Ok(value.take_profit_pct),
                take_profit_rate: Ok(value.take_profit_rate),
                units: Ok(value.units),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateOrderResponse {
        token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateOrderResponse {
        fn default() -> Self {
            Self {
                token: Ok(Default::default()),
            }
        }
    }
    impl CreateOrderResponse {
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateOrderResponse> for super::CreateOrderResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateOrderResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                token: value.token?,
            })
        }
    }
    impl ::std::convert::From<super::CreateOrderResponse> for CreateOrderResponse {
        fn from(value: super::CreateOrderResponse) -> Self {
            Self {
                token: Ok(value.token),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteExitOrderResponse {
        token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DeleteExitOrderResponse {
        fn default() -> Self {
            Self {
                token: Ok(Default::default()),
            }
        }
    }
    impl DeleteExitOrderResponse {
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteExitOrderResponse> for super::DeleteExitOrderResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteExitOrderResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                token: value.token?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteExitOrderResponse> for DeleteExitOrderResponse {
        fn from(value: super::DeleteExitOrderResponse) -> Self {
            Self {
                token: Ok(value.token),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteOrderResponse {
        token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DeleteOrderResponse {
        fn default() -> Self {
            Self {
                token: Ok(Default::default()),
            }
        }
    }
    impl DeleteOrderResponse {
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteOrderResponse> for super::DeleteOrderResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteOrderResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                token: value.token?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteOrderResponse> for DeleteOrderResponse {
        fn from(value: super::DeleteOrderResponse) -> Self {
            Self {
                token: Ok(value.token),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest
    {
        external_operation_data: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        external_operation_type:
            ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        instrument_id: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        lots_to_deduct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        reference_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        units_to_deduct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        view_rate_context: ::std::result::Result<
            ::std::option::Option<
                super::EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto,
            >,
            ::std::string::String,
        >,
    }
    impl :: std :: default :: Default for EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest { fn default () -> Self { Self { external_operation_data : Ok (Default :: default ()) , external_operation_type : Ok (Default :: default ()) , instrument_id : Ok (Default :: default ()) , lots_to_deduct : Ok (Default :: default ()) , reference_id : Ok (Default :: default ()) , units_to_deduct : Ok (Default :: default ()) , view_rate_context : Ok (Default :: default ()) , } } }
    impl EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest {
        pub fn external_operation_data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_operation_data = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_operation_data: {e}")
            });
            self
        }
        pub fn external_operation_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_operation_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_operation_type: {e}")
            });
            self
        }
        pub fn instrument_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
            self
        }
        pub fn lots_to_deduct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.lots_to_deduct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lots_to_deduct: {e}"));
            self
        }
        pub fn reference_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reference_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reference_id: {e}"));
            self
        }
        pub fn units_to_deduct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.units_to_deduct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for units_to_deduct: {e}"));
            self
        }        pub fn view_rate_context < T > (mut self , value : T) -> Self where T : :: std :: convert :: TryInto < :: std :: option :: Option < super :: EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto > > , T :: Error : :: std :: fmt :: Display ,{
            self.view_rate_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for view_rate_context: {e}"));
            self
        }
    }
    impl :: std :: convert :: TryFrom < EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest > for super :: EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest { type Error = super :: error :: ConversionError ; fn try_from (value : EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest) -> :: std :: result :: Result < Self , super :: error :: ConversionError > { Ok (Self { external_operation_data : value . external_operation_data ? , external_operation_type : value . external_operation_type ? , instrument_id : value . instrument_id ? , lots_to_deduct : value . lots_to_deduct ? , reference_id : value . reference_id ? , units_to_deduct : value . units_to_deduct ? , view_rate_context : value . view_rate_context ? , }) } }
    impl :: std :: convert :: From < super :: EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest > for EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest { fn from (value : super :: EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForCloseOrderForCloseDetailsRequest) -> Self { Self { external_operation_data : Ok (value . external_operation_data) , external_operation_type : Ok (value . external_operation_type) , instrument_id : Ok (value . instrument_id) , lots_to_deduct : Ok (value . lots_to_deduct) , reference_id : Ok (value . reference_id) , units_to_deduct : Ok (value . units_to_deduct) , view_rate_context : Ok (value . view_rate_context) , } } }
    #[derive(Clone, Debug)]
    pub struct EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest {
        additional_margin: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        amount_in_units: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        cid: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        external_operation_data: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        external_operation_type:
            ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        instrument_id: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        is_buy: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_discounted: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_no_stop_loss: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_no_take_profit:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_tsl_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        leverage: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        lot_count: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        reference_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        requested_settlement_type_id:
            ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        stop_loss_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        take_profit_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        total_external_costs:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        view_rate_context: ::std::result::Result<
            ::std::option::Option<
                super::EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default
        for EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest
    {
        fn default() -> Self {
            Self {
                additional_margin: Ok(Default::default()),
                amount: Ok(Default::default()),
                amount_in_units: Ok(Default::default()),
                cid: Ok(Default::default()),
                external_operation_data: Ok(Default::default()),
                external_operation_type: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                is_buy: Ok(Default::default()),
                is_discounted: Ok(Default::default()),
                is_no_stop_loss: Ok(Default::default()),
                is_no_take_profit: Ok(Default::default()),
                is_tsl_enabled: Ok(Default::default()),
                leverage: Ok(Default::default()),
                lot_count: Ok(Default::default()),
                reference_id: Ok(Default::default()),
                requested_settlement_type_id: Ok(Default::default()),
                stop_loss_rate: Ok(Default::default()),
                take_profit_rate: Ok(Default::default()),
                total_external_costs: Ok(Default::default()),
                view_rate_context: Ok(Default::default()),
            }
        }
    }
    impl EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest {
        pub fn additional_margin<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.additional_margin = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for additional_margin: {e}"));
            self
        }
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn amount_in_units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_in_units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_in_units: {e}"));
            self
        }
        pub fn cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cid: {e}"));
            self
        }
        pub fn external_operation_data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_operation_data = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_operation_data: {e}")
            });
            self
        }
        pub fn external_operation_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_operation_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_operation_type: {e}")
            });
            self
        }
        pub fn instrument_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
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
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.leverage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for leverage: {e}"));
            self
        }
        pub fn lot_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.lot_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lot_count: {e}"));
            self
        }
        pub fn reference_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reference_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reference_id: {e}"));
            self
        }
        pub fn requested_settlement_type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.requested_settlement_type_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for requested_settlement_type_id: {e}")
            });
            self
        }
        pub fn stop_loss_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_loss_rate: {e}"));
            self
        }
        pub fn take_profit_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.take_profit_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for take_profit_rate: {e}"));
            self
        }
        pub fn total_external_costs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_external_costs = value.try_into().map_err(|e| {
                format!("error converting supplied value for total_external_costs: {e}")
            });
            self
        }        pub fn view_rate_context < T > (mut self , value : T) -> Self where T : :: std :: convert :: TryInto < :: std :: option :: Option < super :: EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto > > , T :: Error : :: std :: fmt :: Display ,{
            self.view_rate_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for view_rate_context: {e}"));
            self
        }
    }
    impl :: std :: convert :: TryFrom < EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest > for super :: EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest { type Error = super :: error :: ConversionError ; fn try_from (value : EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest) -> :: std :: result :: Result < Self , super :: error :: ConversionError > { Ok (Self { additional_margin : value . additional_margin ? , amount : value . amount ? , amount_in_units : value . amount_in_units ? , cid : value . cid ? , external_operation_data : value . external_operation_data ? , external_operation_type : value . external_operation_type ? , instrument_id : value . instrument_id ? , is_buy : value . is_buy ? , is_discounted : value . is_discounted ? , is_no_stop_loss : value . is_no_stop_loss ? , is_no_take_profit : value . is_no_take_profit ? , is_tsl_enabled : value . is_tsl_enabled ? , leverage : value . leverage ? , lot_count : value . lot_count ? , reference_id : value . reference_id ? , requested_settlement_type_id : value . requested_settlement_type_id ? , stop_loss_rate : value . stop_loss_rate ? , take_profit_rate : value . take_profit_rate ? , total_external_costs : value . total_external_costs ? , view_rate_context : value . view_rate_context ? , }) } }
    impl :: std :: convert :: From < super :: EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest > for EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest { fn from (value : super :: EToroTradingDistributedServicesWebApiApiDtoRequestsOrderForOpenOrderForOpenRequest) -> Self { Self { additional_margin : Ok (value . additional_margin) , amount : Ok (value . amount) , amount_in_units : Ok (value . amount_in_units) , cid : Ok (value . cid) , external_operation_data : Ok (value . external_operation_data) , external_operation_type : Ok (value . external_operation_type) , instrument_id : Ok (value . instrument_id) , is_buy : Ok (value . is_buy) , is_discounted : Ok (value . is_discounted) , is_no_stop_loss : Ok (value . is_no_stop_loss) , is_no_take_profit : Ok (value . is_no_take_profit) , is_tsl_enabled : Ok (value . is_tsl_enabled) , leverage : Ok (value . leverage) , lot_count : Ok (value . lot_count) , reference_id : Ok (value . reference_id) , requested_settlement_type_id : Ok (value . requested_settlement_type_id) , stop_loss_rate : Ok (value . stop_loss_rate) , take_profit_rate : Ok (value . take_profit_rate) , total_external_costs : Ok (value . total_external_costs) , view_rate_context : Ok (value . view_rate_context) , } } }
    #[derive(Clone, Debug)]
    pub struct EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest {
        amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        amount_in_units: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        cid: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        instrument_id: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        is_buy: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_discounted: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_no_stop_loss: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_no_take_profit:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_tsl_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        leverage: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        stop_loss_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        take_profit_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default
        for EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest
    {
        fn default() -> Self {
            Self {
                amount: Ok(Default::default()),
                amount_in_units: Ok(Default::default()),
                cid: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                is_buy: Ok(Default::default()),
                is_discounted: Ok(Default::default()),
                is_no_stop_loss: Ok(Default::default()),
                is_no_take_profit: Ok(Default::default()),
                is_tsl_enabled: Ok(Default::default()),
                leverage: Ok(Default::default()),
                rate: Ok(Default::default()),
                stop_loss_rate: Ok(Default::default()),
                take_profit_rate: Ok(Default::default()),
            }
        }
    }
    impl EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn amount_in_units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount_in_units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount_in_units: {e}"));
            self
        }
        pub fn cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cid: {e}"));
            self
        }
        pub fn instrument_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
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
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.leverage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for leverage: {e}"));
            self
        }
        pub fn rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rate: {e}"));
            self
        }
        pub fn stop_loss_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_loss_rate: {e}"));
            self
        }
        pub fn take_profit_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.take_profit_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for take_profit_rate: {e}"));
            self
        }
    }
    impl
        ::std::convert::TryFrom<
            EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest,
        > for super::EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                amount_in_units: value.amount_in_units?,
                cid: value.cid?,
                instrument_id: value.instrument_id?,
                is_buy: value.is_buy?,
                is_discounted: value.is_discounted?,
                is_no_stop_loss: value.is_no_stop_loss?,
                is_no_take_profit: value.is_no_take_profit?,
                is_tsl_enabled: value.is_tsl_enabled?,
                leverage: value.leverage?,
                rate: value.rate?,
                stop_loss_rate: value.stop_loss_rate?,
                take_profit_rate: value.take_profit_rate?,
            })
        }
    }
    impl
        ::std::convert::From<
            super::EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest,
        > for EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest
    {
        fn from(
            value: super::EToroTradingDistributedServicesWebApiApiDtoRequestsOrdersOrderOpenRequest,
        ) -> Self {
            Self {
                amount: Ok(value.amount),
                amount_in_units: Ok(value.amount_in_units),
                cid: Ok(value.cid),
                instrument_id: Ok(value.instrument_id),
                is_buy: Ok(value.is_buy),
                is_discounted: Ok(value.is_discounted),
                is_no_stop_loss: Ok(value.is_no_stop_loss),
                is_no_take_profit: Ok(value.is_no_take_profit),
                is_tsl_enabled: Ok(value.is_tsl_enabled),
                leverage: Ok(value.leverage),
                rate: Ok(value.rate),
                stop_loss_rate: Ok(value.stop_loss_rate),
                take_profit_rate: Ok(value.take_profit_rate),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto {
        client_rate_for_calc:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        client_rate_for_calc_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        client_view_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        client_view_rate_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        price_type: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        snapshot_timestamp: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default
        for EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto
    {
        fn default() -> Self {
            Self {
                client_rate_for_calc: Ok(Default::default()),
                client_rate_for_calc_id: Ok(Default::default()),
                client_view_rate: Ok(Default::default()),
                client_view_rate_id: Ok(Default::default()),
                price_type: Ok(Default::default()),
                snapshot_timestamp: Ok(Default::default()),
            }
        }
    }
    impl EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto {
        pub fn client_rate_for_calc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_rate_for_calc = value.try_into().map_err(|e| {
                format!("error converting supplied value for client_rate_for_calc: {e}")
            });
            self
        }
        pub fn client_rate_for_calc_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_rate_for_calc_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for client_rate_for_calc_id: {e}")
            });
            self
        }
        pub fn client_view_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_view_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_view_rate: {e}"));
            self
        }
        pub fn client_view_rate_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_view_rate_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for client_view_rate_id: {e}")
            });
            self
        }
        pub fn price_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.price_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for price_type: {e}"));
            self
        }
        pub fn snapshot_timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.snapshot_timestamp = value.try_into().map_err(|e| {
                format!("error converting supplied value for snapshot_timestamp: {e}")
            });
            self
        }
    }
    impl
        ::std::convert::TryFrom<
            EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto,
        > for super::EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_rate_for_calc: value.client_rate_for_calc?,
                client_rate_for_calc_id: value.client_rate_for_calc_id?,
                client_view_rate: value.client_view_rate?,
                client_view_rate_id: value.client_view_rate_id?,
                price_type: value.price_type?,
                snapshot_timestamp: value.snapshot_timestamp?,
            })
        }
    }
    impl
        ::std::convert::From<
            super::EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto,
        > for EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto
    {
        fn from(
            value : super :: EToroTradingDistributedServicesWebApiApiDtoRequestsRatesViewRateContextDto,
        ) -> Self {
            Self {
                client_rate_for_calc: Ok(value.client_rate_for_calc),
                client_rate_for_calc_id: Ok(value.client_rate_for_calc_id),
                client_view_rate: Ok(value.client_view_rate),
                client_view_rate_id: Ok(value.client_view_rate_id),
                price_type: Ok(value.price_type),
                snapshot_timestamp: Ok(value.snapshot_timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetExitOrderResponse {
        open_date_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        order_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        position_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        social_trade_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for GetExitOrderResponse {
        fn default() -> Self {
            Self {
                open_date_time: Ok(Default::default()),
                order_id: Ok(Default::default()),
                position_id: Ok(Default::default()),
                social_trade_id: Ok(Default::default()),
            }
        }
    }
    impl GetExitOrderResponse {
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
        pub fn social_trade_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.social_trade_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for social_trade_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetExitOrderResponse> for super::GetExitOrderResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetExitOrderResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                open_date_time: value.open_date_time?,
                order_id: value.order_id?,
                position_id: value.position_id?,
                social_trade_id: value.social_trade_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetExitOrderResponse> for GetExitOrderResponse {
        fn from(value: super::GetExitOrderResponse) -> Self {
            Self {
                open_date_time: Ok(value.open_date_time),
                order_id: Ok(value.order_id),
                position_id: Ok(value.position_id),
                social_trade_id: Ok(value.social_trade_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetOrderResponse {
        execution_type: ::std::result::Result<
            ::std::option::Option<super::GetOrderResponseExecutionType>,
            ::std::string::String,
        >,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        investment: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        is_buy: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_entry: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        leverage: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        limit_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        open_timestamp: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        order_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order_type: ::std::result::Result<
            ::std::option::Option<super::GetOrderResponseOrderType>,
            ::std::string::String,
        >,
        stop_loss_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        take_profit_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for GetOrderResponse {
        fn default() -> Self {
            Self {
                execution_type: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                investment: Ok(Default::default()),
                is_buy: Ok(Default::default()),
                is_entry: Ok(Default::default()),
                leverage: Ok(Default::default()),
                limit_rate: Ok(Default::default()),
                open_timestamp: Ok(Default::default()),
                order_id: Ok(Default::default()),
                order_type: Ok(Default::default()),
                stop_loss_rate: Ok(Default::default()),
                take_profit_rate: Ok(Default::default()),
            }
        }
    }
    impl GetOrderResponse {
        pub fn execution_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::GetOrderResponseExecutionType>>,
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
        pub fn investment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.investment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for investment: {e}"));
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
        pub fn is_entry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_entry = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_entry: {e}"));
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
        pub fn limit_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit_rate: {e}"));
            self
        }
        pub fn open_timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.open_timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open_timestamp: {e}"));
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
            T: ::std::convert::TryInto<::std::option::Option<super::GetOrderResponseOrderType>>,
            T::Error: ::std::fmt::Display,
        {
            self.order_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_type: {e}"));
            self
        }
        pub fn stop_loss_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_loss_rate: {e}"));
            self
        }
        pub fn take_profit_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.take_profit_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for take_profit_rate: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetOrderResponse> for super::GetOrderResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetOrderResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                execution_type: value.execution_type?,
                instrument_id: value.instrument_id?,
                investment: value.investment?,
                is_buy: value.is_buy?,
                is_entry: value.is_entry?,
                leverage: value.leverage?,
                limit_rate: value.limit_rate?,
                open_timestamp: value.open_timestamp?,
                order_id: value.order_id?,
                order_type: value.order_type?,
                stop_loss_rate: value.stop_loss_rate?,
                take_profit_rate: value.take_profit_rate?,
            })
        }
    }
    impl ::std::convert::From<super::GetOrderResponse> for GetOrderResponse {
        fn from(value: super::GetOrderResponse) -> Self {
            Self {
                execution_type: Ok(value.execution_type),
                instrument_id: Ok(value.instrument_id),
                investment: Ok(value.investment),
                is_buy: Ok(value.is_buy),
                is_entry: Ok(value.is_entry),
                leverage: Ok(value.leverage),
                limit_rate: Ok(value.limit_rate),
                open_timestamp: Ok(value.open_timestamp),
                order_id: Ok(value.order_id),
                order_type: Ok(value.order_type),
                stop_loss_rate: Ok(value.stop_loss_rate),
                take_profit_rate: Ok(value.take_profit_rate),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Market {
        application: ::std::result::Result<
            ::std::option::Option<super::ApplicationSource>,
            ::std::string::String,
        >,
        asset_type: ::std::result::Result<
            ::std::option::Option<super::MarketAssetType>,
            ::std::string::String,
        >,
        asset_type_id: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        asset_type_sub_category_id:
            ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        avatar: ::std::result::Result<::std::option::Option<super::Avatar>, ::std::string::String>,
        display_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        internal_id: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        symbol_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        updated: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Market {
        fn default() -> Self {
            Self {
                application: Ok(Default::default()),
                asset_type: Ok(Default::default()),
                asset_type_id: Ok(Default::default()),
                asset_type_sub_category_id: Ok(Default::default()),
                avatar: Ok(Default::default()),
                display_name: Ok(Default::default()),
                id: Ok(Default::default()),
                internal_id: Ok(Default::default()),
                metadata: Ok(Default::default()),
                symbol_name: Ok(Default::default()),
                updated: Ok(Default::default()),
            }
        }
    }
    impl Market {
        pub fn application<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ApplicationSource>>,
            T::Error: ::std::fmt::Display,
        {
            self.application = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for application: {e}"));
            self
        }
        pub fn asset_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MarketAssetType>>,
            T::Error: ::std::fmt::Display,
        {
            self.asset_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for asset_type: {e}"));
            self
        }
        pub fn asset_type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.asset_type_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for asset_type_id: {e}"));
            self
        }
        pub fn asset_type_sub_category_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.asset_type_sub_category_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for asset_type_sub_category_id: {e}")
            });
            self
        }
        pub fn avatar<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Avatar>>,
            T::Error: ::std::fmt::Display,
        {
            self.avatar = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for avatar: {e}"));
            self
        }
        pub fn display_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.display_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display_name: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn internal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.internal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for internal_id: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn symbol_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.symbol_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbol_name: {e}"));
            self
        }
        pub fn updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.updated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for updated: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Market> for super::Market {
        type Error = super::error::ConversionError;
        fn try_from(value: Market) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                application: value.application?,
                asset_type: value.asset_type?,
                asset_type_id: value.asset_type_id?,
                asset_type_sub_category_id: value.asset_type_sub_category_id?,
                avatar: value.avatar?,
                display_name: value.display_name?,
                id: value.id?,
                internal_id: value.internal_id?,
                metadata: value.metadata?,
                symbol_name: value.symbol_name?,
                updated: value.updated?,
            })
        }
    }
    impl ::std::convert::From<super::Market> for Market {
        fn from(value: super::Market) -> Self {
            Self {
                application: Ok(value.application),
                asset_type: Ok(value.asset_type),
                asset_type_id: Ok(value.asset_type_id),
                asset_type_sub_category_id: Ok(value.asset_type_sub_category_id),
                avatar: Ok(value.avatar),
                display_name: Ok(value.display_name),
                id: Ok(value.id),
                internal_id: Ok(value.internal_id),
                metadata: Ok(value.metadata),
                symbol_name: Ok(value.symbol_name),
                updated: Ok(value.updated),
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
    pub struct OrderForOpenInfoResponse {
        amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        cid: ::std::result::Result<i64, ::std::string::String>,
        error_code: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        error_message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        instrument_id: ::std::result::Result<i64, ::std::string::String>,
        open_action_type: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order_id: ::std::result::Result<i64, ::std::string::String>,
        order_type: ::std::result::Result<i64, ::std::string::String>,
        positions: ::std::result::Result<
            ::std::vec::Vec<super::OrderForOpenPositionInfo>,
            ::std::string::String,
        >,
        reference_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        request_occurred:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        status_id: ::std::result::Result<i64, ::std::string::String>,
        token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        units: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for OrderForOpenInfoResponse {
        fn default() -> Self {
            Self {
                amount: Ok(Default::default()),
                cid: Err("no value supplied for cid".to_string()),
                error_code: Ok(Default::default()),
                error_message: Ok(Default::default()),
                instrument_id: Err("no value supplied for instrument_id".to_string()),
                open_action_type: Ok(Default::default()),
                order_id: Err("no value supplied for order_id".to_string()),
                order_type: Err("no value supplied for order_type".to_string()),
                positions: Ok(Default::default()),
                reference_id: Ok(Default::default()),
                request_occurred: Err("no value supplied for request_occurred".to_string()),
                status_id: Err("no value supplied for status_id".to_string()),
                token: Ok(Default::default()),
                units: Ok(Default::default()),
            }
        }
    }
    impl OrderForOpenInfoResponse {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cid: {e}"));
            self
        }
        pub fn error_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_code: {e}"));
            self
        }
        pub fn error_message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_message: {e}"));
            self
        }
        pub fn instrument_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
            self
        }
        pub fn open_action_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_action_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open_action_type: {e}"));
            self
        }
        pub fn order_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.order_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_id: {e}"));
            self
        }
        pub fn order_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.order_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_type: {e}"));
            self
        }
        pub fn positions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrderForOpenPositionInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.positions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for positions: {e}"));
            self
        }
        pub fn reference_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reference_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reference_id: {e}"));
            self
        }
        pub fn request_occurred<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.request_occurred = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for request_occurred: {e}"));
            self
        }
        pub fn status_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.status_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status_id: {e}"));
            self
        }
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {e}"));
            self
        }
        pub fn units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for units: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrderForOpenInfoResponse> for super::OrderForOpenInfoResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrderForOpenInfoResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                cid: value.cid?,
                error_code: value.error_code?,
                error_message: value.error_message?,
                instrument_id: value.instrument_id?,
                open_action_type: value.open_action_type?,
                order_id: value.order_id?,
                order_type: value.order_type?,
                positions: value.positions?,
                reference_id: value.reference_id?,
                request_occurred: value.request_occurred?,
                status_id: value.status_id?,
                token: value.token?,
                units: value.units?,
            })
        }
    }
    impl ::std::convert::From<super::OrderForOpenInfoResponse> for OrderForOpenInfoResponse {
        fn from(value: super::OrderForOpenInfoResponse) -> Self {
            Self {
                amount: Ok(value.amount),
                cid: Ok(value.cid),
                error_code: Ok(value.error_code),
                error_message: Ok(value.error_message),
                instrument_id: Ok(value.instrument_id),
                open_action_type: Ok(value.open_action_type),
                order_id: Ok(value.order_id),
                order_type: Ok(value.order_type),
                positions: Ok(value.positions),
                reference_id: Ok(value.reference_id),
                request_occurred: Ok(value.request_occurred),
                status_id: Ok(value.status_id),
                token: Ok(value.token),
                units: Ok(value.units),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrderForOpenPositionInfo {
        amount: ::std::result::Result<f64, ::std::string::String>,
        conversion_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        is_open: ::std::result::Result<bool, ::std::string::String>,
        occurred:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        order_type: ::std::result::Result<i64, ::std::string::String>,
        position_id: ::std::result::Result<i64, ::std::string::String>,
        rate: ::std::result::Result<f64, ::std::string::String>,
        units: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for OrderForOpenPositionInfo {
        fn default() -> Self {
            Self {
                amount: Err("no value supplied for amount".to_string()),
                conversion_rate: Ok(Default::default()),
                is_open: Err("no value supplied for is_open".to_string()),
                occurred: Err("no value supplied for occurred".to_string()),
                order_type: Err("no value supplied for order_type".to_string()),
                position_id: Err("no value supplied for position_id".to_string()),
                rate: Err("no value supplied for rate".to_string()),
                units: Err("no value supplied for units".to_string()),
            }
        }
    }
    impl OrderForOpenPositionInfo {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn conversion_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.conversion_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conversion_rate: {e}"));
            self
        }
        pub fn is_open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_open = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_open: {e}"));
            self
        }
        pub fn occurred<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.occurred = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for occurred: {e}"));
            self
        }
        pub fn order_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.order_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_type: {e}"));
            self
        }
        pub fn position_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.position_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position_id: {e}"));
            self
        }
        pub fn rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rate: {e}"));
            self
        }
        pub fn units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.units = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for units: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrderForOpenPositionInfo> for super::OrderForOpenPositionInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrderForOpenPositionInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                conversion_rate: value.conversion_rate?,
                is_open: value.is_open?,
                occurred: value.occurred?,
                order_type: value.order_type?,
                position_id: value.position_id?,
                rate: value.rate?,
                units: value.units?,
            })
        }
    }
    impl ::std::convert::From<super::OrderForOpenPositionInfo> for OrderForOpenPositionInfo {
        fn from(value: super::OrderForOpenPositionInfo) -> Self {
            Self {
                amount: Ok(value.amount),
                conversion_rate: Ok(value.conversion_rate),
                is_open: Ok(value.is_open),
                occurred: Ok(value.occurred),
                order_type: Ok(value.order_type),
                position_id: Ok(value.position_id),
                rate: Ok(value.rate),
                units: Ok(value.units),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrderMetadata {
        direction: ::std::result::Result<
            ::std::option::Option<super::TradeDirection>,
            ::std::string::String,
        >,
        market: ::std::result::Result<::std::option::Option<super::Market>, ::std::string::String>,
        order_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        rate: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        type_:
            ::std::result::Result<::std::option::Option<super::TradeType>, ::std::string::String>,
    }
    impl ::std::default::Default for OrderMetadata {
        fn default() -> Self {
            Self {
                direction: Ok(Default::default()),
                market: Ok(Default::default()),
                order_id: Ok(Default::default()),
                rate: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl OrderMetadata {
        pub fn direction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TradeDirection>>,
            T::Error: ::std::fmt::Display,
        {
            self.direction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for direction: {e}"));
            self
        }
        pub fn market<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Market>>,
            T::Error: ::std::fmt::Display,
        {
            self.market = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for market: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TradeType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrderMetadata> for super::OrderMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrderMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                direction: value.direction?,
                market: value.market?,
                order_id: value.order_id?,
                rate: value.rate?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::OrderMetadata> for OrderMetadata {
        fn from(value: super::OrderMetadata) -> Self {
            Self {
                direction: Ok(value.direction),
                market: Ok(value.market),
                order_id: Ok(value.order_id),
                rate: Ok(value.rate),
                type_: Ok(value.type_),
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
    #[derive(Clone, Debug)]
    pub struct PutTradeRequest {
        is_trailing_stop_loss:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        position_id: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        stop_loss_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        take_profit_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for PutTradeRequest {
        fn default() -> Self {
            Self {
                is_trailing_stop_loss: Ok(Default::default()),
                position_id: Ok(Default::default()),
                stop_loss_rate: Ok(Default::default()),
                take_profit_rate: Ok(Default::default()),
            }
        }
    }
    impl PutTradeRequest {
        pub fn is_trailing_stop_loss<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_trailing_stop_loss = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_trailing_stop_loss: {e}")
            });
            self
        }
        pub fn position_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.position_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position_id: {e}"));
            self
        }
        pub fn stop_loss_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop_loss_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_loss_rate: {e}"));
            self
        }
        pub fn take_profit_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.take_profit_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for take_profit_rate: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PutTradeRequest> for super::PutTradeRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PutTradeRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                is_trailing_stop_loss: value.is_trailing_stop_loss?,
                position_id: value.position_id?,
                stop_loss_rate: value.stop_loss_rate?,
                take_profit_rate: value.take_profit_rate?,
            })
        }
    }
    impl ::std::convert::From<super::PutTradeRequest> for PutTradeRequest {
        fn from(value: super::PutTradeRequest) -> Self {
            Self {
                is_trailing_stop_loss: Ok(value.is_trailing_stop_loss),
                position_id: Ok(value.position_id),
                stop_loss_rate: Ok(value.stop_loss_rate),
                take_profit_rate: Ok(value.take_profit_rate),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Svg {
        background_color: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text_color: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Svg {
        fn default() -> Self {
            Self {
                background_color: Ok(Default::default()),
                text_color: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl Svg {
        pub fn background_color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.background_color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for background_color: {e}"));
            self
        }
        pub fn text_color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text_color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_color: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Svg> for super::Svg {
        type Error = super::error::ConversionError;
        fn try_from(value: Svg) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                background_color: value.background_color?,
                text_color: value.text_color?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::Svg> for Svg {
        fn from(value: super::Svg) -> Self {
            Self {
                background_color: Ok(value.background_color),
                text_color: Ok(value.text_color),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TradeMetadata {
        direction: ::std::result::Result<
            ::std::option::Option<super::TradeDirection>,
            ::std::string::String,
        >,
        gain: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        market: ::std::result::Result<::std::option::Option<super::Market>, ::std::string::String>,
        position_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        rate: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        type_:
            ::std::result::Result<::std::option::Option<super::TradeType>, ::std::string::String>,
    }
    impl ::std::default::Default for TradeMetadata {
        fn default() -> Self {
            Self {
                direction: Ok(Default::default()),
                gain: Ok(Default::default()),
                market: Ok(Default::default()),
                position_id: Ok(Default::default()),
                rate: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl TradeMetadata {
        pub fn direction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TradeDirection>>,
            T::Error: ::std::fmt::Display,
        {
            self.direction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for direction: {e}"));
            self
        }
        pub fn gain<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.gain = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gain: {e}"));
            self
        }
        pub fn market<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Market>>,
            T::Error: ::std::fmt::Display,
        {
            self.market = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for market: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TradeType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TradeMetadata> for super::TradeMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TradeMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                direction: value.direction?,
                gain: value.gain?,
                market: value.market?,
                position_id: value.position_id?,
                rate: value.rate?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TradeMetadata> for TradeMetadata {
        fn from(value: super::TradeMetadata) -> Self {
            Self {
                direction: Ok(value.direction),
                gain: Ok(value.gain),
                market: Ok(value.market),
                position_id: Ok(value.position_id),
                rate: Ok(value.rate),
                type_: Ok(value.type_),
            }
        }
    }
}
