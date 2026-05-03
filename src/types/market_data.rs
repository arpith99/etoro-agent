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
#[doc = "Response containing historical price data in candlestick format"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Response containing historical price data in candlestick format\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"candles\": {"]
#[doc = "      \"description\": \"Outer: per-instrument grouping\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"candles\": {"]
#[doc = "            \"description\": \"Inner: per-time-period candles\","]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"properties\": {"]
#[doc = "                \"close\": {"]
#[doc = "                  \"type\": \"number\","]
#[doc = "                  \"format\": \"float\""]
#[doc = "                },"]
#[doc = "                \"fromDate\": {"]
#[doc = "                  \"type\": \"string\","]
#[doc = "                  \"format\": \"date-time\""]
#[doc = "                },"]
#[doc = "                \"high\": {"]
#[doc = "                  \"type\": \"number\","]
#[doc = "                  \"format\": \"float\""]
#[doc = "                },"]
#[doc = "                \"instrumentID\": {"]
#[doc = "                  \"description\": \"Capital ID — inconsistent with outer's instrumentId\","]
#[doc = "                  \"type\": \"integer\""]
#[doc = "                },"]
#[doc = "                \"low\": {"]
#[doc = "                  \"type\": \"number\","]
#[doc = "                  \"format\": \"float\""]
#[doc = "                },"]
#[doc = "                \"open\": {"]
#[doc = "                  \"type\": \"number\","]
#[doc = "                  \"format\": \"float\""]
#[doc = "                },"]
#[doc = "                \"volume\": {"]
#[doc = "                  \"type\": \"number\","]
#[doc = "                  \"format\": \"float\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"instrumentId\": {"]
#[doc = "            \"description\": \"Lowercase 'd'\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"rangeClose\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"rangeHigh\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"rangeLow\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"rangeOpen\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"volume\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"interval\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"OneMinute\","]
#[doc = "        \"FiveMinutes\","]
#[doc = "        \"TenMinutes\","]
#[doc = "        \"FifteenMinutes\","]
#[doc = "        \"ThirtyMinutes\","]
#[doc = "        \"OneHour\","]
#[doc = "        \"FourHours\","]
#[doc = "        \"OneDay\","]
#[doc = "        \"OneWeek\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CandlesResponse {
    #[doc = "Outer: per-instrument grouping"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub candles: ::std::vec::Vec<CandlesResponseCandlesItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub interval: ::std::option::Option<CandlesResponseInterval>,
}
impl ::std::default::Default for CandlesResponse {
    fn default() -> Self {
        Self {
            candles: Default::default(),
            interval: Default::default(),
        }
    }
}
impl CandlesResponse {
    pub fn builder() -> builder::CandlesResponse {
        Default::default()
    }
}
#[doc = "`CandlesResponseCandlesItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"candles\": {"]
#[doc = "      \"description\": \"Inner: per-time-period candles\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"close\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"fromDate\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"date-time\""]
#[doc = "          },"]
#[doc = "          \"high\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"instrumentID\": {"]
#[doc = "            \"description\": \"Capital ID — inconsistent with outer's instrumentId\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"low\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"open\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"volume\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"instrumentId\": {"]
#[doc = "      \"description\": \"Lowercase 'd'\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"rangeClose\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"rangeHigh\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"rangeLow\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"rangeOpen\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"volume\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CandlesResponseCandlesItem {
    #[doc = "Inner: per-time-period candles"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub candles: ::std::vec::Vec<CandlesResponseCandlesItemCandlesItem>,
    #[doc = "Lowercase 'd'"]
    #[serde(
        rename = "instrumentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[serde(
        rename = "rangeClose",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub range_close: ::std::option::Option<f32>,
    #[serde(
        rename = "rangeHigh",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub range_high: ::std::option::Option<f32>,
    #[serde(
        rename = "rangeLow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub range_low: ::std::option::Option<f32>,
    #[serde(
        rename = "rangeOpen",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub range_open: ::std::option::Option<f32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub volume: ::std::option::Option<f32>,
}
impl ::std::default::Default for CandlesResponseCandlesItem {
    fn default() -> Self {
        Self {
            candles: Default::default(),
            instrument_id: Default::default(),
            range_close: Default::default(),
            range_high: Default::default(),
            range_low: Default::default(),
            range_open: Default::default(),
            volume: Default::default(),
        }
    }
}
impl CandlesResponseCandlesItem {
    pub fn builder() -> builder::CandlesResponseCandlesItem {
        Default::default()
    }
}
#[doc = "`CandlesResponseCandlesItemCandlesItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"close\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"fromDate\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"high\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"instrumentID\": {"]
#[doc = "      \"description\": \"Capital ID — inconsistent with outer's instrumentId\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"low\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"open\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"volume\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CandlesResponseCandlesItemCandlesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub close: ::std::option::Option<f32>,
    #[serde(
        rename = "fromDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub from_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub high: ::std::option::Option<f32>,
    #[doc = "Capital ID — inconsistent with outer's instrumentId"]
    #[serde(
        rename = "instrumentID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub low: ::std::option::Option<f32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub open: ::std::option::Option<f32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub volume: ::std::option::Option<f32>,
}
impl ::std::default::Default for CandlesResponseCandlesItemCandlesItem {
    fn default() -> Self {
        Self {
            close: Default::default(),
            from_date: Default::default(),
            high: Default::default(),
            instrument_id: Default::default(),
            low: Default::default(),
            open: Default::default(),
            volume: Default::default(),
        }
    }
}
impl CandlesResponseCandlesItemCandlesItem {
    pub fn builder() -> builder::CandlesResponseCandlesItemCandlesItem {
        Default::default()
    }
}
#[doc = "`CandlesResponseInterval`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"OneMinute\","]
#[doc = "    \"FiveMinutes\","]
#[doc = "    \"TenMinutes\","]
#[doc = "    \"FifteenMinutes\","]
#[doc = "    \"ThirtyMinutes\","]
#[doc = "    \"OneHour\","]
#[doc = "    \"FourHours\","]
#[doc = "    \"OneDay\","]
#[doc = "    \"OneWeek\""]
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
pub enum CandlesResponseInterval {
    OneMinute,
    FiveMinutes,
    TenMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    FourHours,
    OneDay,
    OneWeek,
}
impl ::std::fmt::Display for CandlesResponseInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OneMinute => f.write_str("OneMinute"),
            Self::FiveMinutes => f.write_str("FiveMinutes"),
            Self::TenMinutes => f.write_str("TenMinutes"),
            Self::FifteenMinutes => f.write_str("FifteenMinutes"),
            Self::ThirtyMinutes => f.write_str("ThirtyMinutes"),
            Self::OneHour => f.write_str("OneHour"),
            Self::FourHours => f.write_str("FourHours"),
            Self::OneDay => f.write_str("OneDay"),
            Self::OneWeek => f.write_str("OneWeek"),
        }
    }
}
impl ::std::str::FromStr for CandlesResponseInterval {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "OneMinute" => Ok(Self::OneMinute),
            "FiveMinutes" => Ok(Self::FiveMinutes),
            "TenMinutes" => Ok(Self::TenMinutes),
            "FifteenMinutes" => Ok(Self::FifteenMinutes),
            "ThirtyMinutes" => Ok(Self::ThirtyMinutes),
            "OneHour" => Ok(Self::OneHour),
            "FourHours" => Ok(Self::FourHours),
            "OneDay" => Ok(Self::OneDay),
            "OneWeek" => Ok(Self::OneWeek),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CandlesResponseInterval {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CandlesResponseInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CandlesResponseInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ClosingPricesResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"object\","]
#[doc = "    \"properties\": {"]
#[doc = "      \"closingPrices\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"daily\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"properties\": {"]
#[doc = "              \"date\": {"]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"format\": \"date-time\""]
#[doc = "              },"]
#[doc = "              \"price\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"monthly\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"properties\": {"]
#[doc = "              \"date\": {"]
#[doc = "                \"description\": \"0001-01-01 indicates no data available\","]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"format\": \"date-time\""]
#[doc = "              },"]
#[doc = "              \"price\": {"]
#[doc = "                \"description\": \"-1 indicates no data available\","]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"weekly\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"properties\": {"]
#[doc = "              \"date\": {"]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"format\": \"date-time\""]
#[doc = "              },"]
#[doc = "              \"price\": {"]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"format\": \"float\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"instrumentId\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      },"]
#[doc = "      \"isMarketOpen\": {"]
#[doc = "        \"description\": \"Obsolete - Do not use\","]
#[doc = "        \"type\": \"boolean\""]
#[doc = "      },"]
#[doc = "      \"officialClosingPrice\": {"]
#[doc = "        \"type\": \"number\","]
#[doc = "        \"format\": \"float\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ClosingPricesResponse(pub ::std::vec::Vec<ClosingPricesResponseItem>);
impl ::std::ops::Deref for ClosingPricesResponse {
    type Target = ::std::vec::Vec<ClosingPricesResponseItem>;
    fn deref(&self) -> &::std::vec::Vec<ClosingPricesResponseItem> {
        &self.0
    }
}
impl ::std::convert::From<ClosingPricesResponse> for ::std::vec::Vec<ClosingPricesResponseItem> {
    fn from(value: ClosingPricesResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<ClosingPricesResponseItem>> for ClosingPricesResponse {
    fn from(value: ::std::vec::Vec<ClosingPricesResponseItem>) -> Self {
        Self(value)
    }
}
#[doc = "`ClosingPricesResponseItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"closingPrices\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"daily\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"date\": {"]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"format\": \"date-time\""]
#[doc = "            },"]
#[doc = "            \"price\": {"]
#[doc = "              \"type\": \"number\","]
#[doc = "              \"format\": \"float\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"monthly\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"date\": {"]
#[doc = "              \"description\": \"0001-01-01 indicates no data available\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"format\": \"date-time\""]
#[doc = "            },"]
#[doc = "            \"price\": {"]
#[doc = "              \"description\": \"-1 indicates no data available\","]
#[doc = "              \"type\": \"number\","]
#[doc = "              \"format\": \"float\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"weekly\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"date\": {"]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"format\": \"date-time\""]
#[doc = "            },"]
#[doc = "            \"price\": {"]
#[doc = "              \"type\": \"number\","]
#[doc = "              \"format\": \"float\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"instrumentId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"isMarketOpen\": {"]
#[doc = "      \"description\": \"Obsolete - Do not use\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"officialClosingPrice\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClosingPricesResponseItem {
    #[serde(
        rename = "closingPrices",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub closing_prices: ::std::option::Option<ClosingPricesResponseItemClosingPrices>,
    #[serde(
        rename = "instrumentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[doc = "Obsolete - Do not use"]
    #[serde(
        rename = "isMarketOpen",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_market_open: ::std::option::Option<bool>,
    #[serde(
        rename = "officialClosingPrice",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub official_closing_price: ::std::option::Option<f32>,
}
impl ::std::default::Default for ClosingPricesResponseItem {
    fn default() -> Self {
        Self {
            closing_prices: Default::default(),
            instrument_id: Default::default(),
            is_market_open: Default::default(),
            official_closing_price: Default::default(),
        }
    }
}
impl ClosingPricesResponseItem {
    pub fn builder() -> builder::ClosingPricesResponseItem {
        Default::default()
    }
}
#[doc = "`ClosingPricesResponseItemClosingPrices`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"daily\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"date\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"date-time\""]
#[doc = "        },"]
#[doc = "        \"price\": {"]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"format\": \"float\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"monthly\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"date\": {"]
#[doc = "          \"description\": \"0001-01-01 indicates no data available\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"date-time\""]
#[doc = "        },"]
#[doc = "        \"price\": {"]
#[doc = "          \"description\": \"-1 indicates no data available\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"format\": \"float\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"weekly\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"date\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"date-time\""]
#[doc = "        },"]
#[doc = "        \"price\": {"]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"format\": \"float\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClosingPricesResponseItemClosingPrices {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub daily: ::std::option::Option<ClosingPricesResponseItemClosingPricesDaily>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub monthly: ::std::option::Option<ClosingPricesResponseItemClosingPricesMonthly>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub weekly: ::std::option::Option<ClosingPricesResponseItemClosingPricesWeekly>,
}
impl ::std::default::Default for ClosingPricesResponseItemClosingPrices {
    fn default() -> Self {
        Self {
            daily: Default::default(),
            monthly: Default::default(),
            weekly: Default::default(),
        }
    }
}
impl ClosingPricesResponseItemClosingPrices {
    pub fn builder() -> builder::ClosingPricesResponseItemClosingPrices {
        Default::default()
    }
}
#[doc = "`ClosingPricesResponseItemClosingPricesDaily`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"date\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"price\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClosingPricesResponseItemClosingPricesDaily {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub price: ::std::option::Option<f32>,
}
impl ::std::default::Default for ClosingPricesResponseItemClosingPricesDaily {
    fn default() -> Self {
        Self {
            date: Default::default(),
            price: Default::default(),
        }
    }
}
impl ClosingPricesResponseItemClosingPricesDaily {
    pub fn builder() -> builder::ClosingPricesResponseItemClosingPricesDaily {
        Default::default()
    }
}
#[doc = "`ClosingPricesResponseItemClosingPricesMonthly`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"date\": {"]
#[doc = "      \"description\": \"0001-01-01 indicates no data available\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"price\": {"]
#[doc = "      \"description\": \"-1 indicates no data available\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClosingPricesResponseItemClosingPricesMonthly {
    #[doc = "0001-01-01 indicates no data available"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "-1 indicates no data available"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub price: ::std::option::Option<f32>,
}
impl ::std::default::Default for ClosingPricesResponseItemClosingPricesMonthly {
    fn default() -> Self {
        Self {
            date: Default::default(),
            price: Default::default(),
        }
    }
}
impl ClosingPricesResponseItemClosingPricesMonthly {
    pub fn builder() -> builder::ClosingPricesResponseItemClosingPricesMonthly {
        Default::default()
    }
}
#[doc = "`ClosingPricesResponseItemClosingPricesWeekly`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"date\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"price\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClosingPricesResponseItemClosingPricesWeekly {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub price: ::std::option::Option<f32>,
}
impl ::std::default::Default for ClosingPricesResponseItemClosingPricesWeekly {
    fn default() -> Self {
        Self {
            date: Default::default(),
            price: Default::default(),
        }
    }
}
impl ClosingPricesResponseItemClosingPricesWeekly {
    pub fn builder() -> builder::ClosingPricesResponseItemClosingPricesWeekly {
        Default::default()
    }
}
#[doc = "`ExchangesResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"exchangeInfo\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"exchangeDescription\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"exchangeID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExchangesResponse {
    #[serde(
        rename = "exchangeInfo",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exchange_info: ::std::vec::Vec<ExchangesResponseExchangeInfoItem>,
}
impl ::std::default::Default for ExchangesResponse {
    fn default() -> Self {
        Self {
            exchange_info: Default::default(),
        }
    }
}
impl ExchangesResponse {
    pub fn builder() -> builder::ExchangesResponse {
        Default::default()
    }
}
#[doc = "`ExchangesResponseExchangeInfoItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"exchangeDescription\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"exchangeID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ExchangesResponseExchangeInfoItem {
    #[serde(
        rename = "exchangeDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exchange_description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "exchangeID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exchange_id: ::std::option::Option<i64>,
}
impl ::std::default::Default for ExchangesResponseExchangeInfoItem {
    fn default() -> Self {
        Self {
            exchange_description: Default::default(),
            exchange_id: Default::default(),
        }
    }
}
impl ExchangesResponseExchangeInfoItem {
    pub fn builder() -> builder::ExchangesResponseExchangeInfoItem {
        Default::default()
    }
}
#[doc = "`Instrument`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"absBuyPctChange24Hours\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"absDailyPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"buyHoldingPct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"buyPctChange24Hours\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"currMonthPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"currQuarterPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"currYearPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"currentRate\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"cvtAsk\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"cvtAskNoSpread\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"cvtBiNoSpread\": {"]
#[doc = "      \"description\": \"Spec spelling — likely typo for 'cvtBidNoSpread'\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"cvtBid\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"dailyPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"displayname\": {"]
#[doc = "      \"description\": \"The display name of the instrument.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"exchangeID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"holdingPct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"industryNameId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"instrumentId\": {"]
#[doc = "      \"description\": \"A unique identifier for the instrument.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"instrumentType\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"instrumentTypeID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"internalAssetClassId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"internalAssetClassName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"internalClosingPrice\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"internalCryptoTypeId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"internalExchangeId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"internalExchangeName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"internalIndustryId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"internalInstrumentDisplayName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"internalInstrumentId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"internalStockIndustryName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"internalSymbolFull\": {"]
#[doc = "      \"description\": \"The full internal symbol — use this for exact symbol matching\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"isActiveInPlatform\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isBuyEnabled\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isCurrentlyTradable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isDelisted\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isExchangeOpen\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isHiddenFromClient\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isInternalInstrument\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isOpen\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"lastTwoYearsPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"lastYearPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"logo150x150\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"logo35x35\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"logo50x50\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"monthlyPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"oneMonthAgoPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"oneYearAgoPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"oneYearPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"popularityUniques\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"popularityUniques14Day\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"popularityUniques30Day\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"popularityUniques7Day\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"sectorNameId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"sellHoldingPct\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"sixMonthPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"sixMonthsAgoPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"symbol\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threeMonthPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"threeMonthsAgoPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"traders14DayChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"traders30DayChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"traders7DayChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"twoMonthsAgoPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"weeklyPriceChange\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Instrument {
    #[serde(
        rename = "absBuyPctChange24Hours",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub abs_buy_pct_change24_hours: ::std::option::Option<f64>,
    #[serde(
        rename = "absDailyPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub abs_daily_price_change: ::std::option::Option<f64>,
    #[serde(
        rename = "buyHoldingPct",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub buy_holding_pct: ::std::option::Option<f64>,
    #[serde(
        rename = "buyPctChange24Hours",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub buy_pct_change24_hours: ::std::option::Option<f64>,
    #[serde(
        rename = "currMonthPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub curr_month_price_change: ::std::option::Option<f64>,
    #[serde(
        rename = "currQuarterPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub curr_quarter_price_change: ::std::option::Option<f64>,
    #[serde(
        rename = "currYearPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub curr_year_price_change: ::std::option::Option<f64>,
    #[serde(
        rename = "currentRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub current_rate: ::std::option::Option<f64>,
    #[serde(
        rename = "cvtAsk",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvt_ask: ::std::option::Option<f64>,
    #[serde(
        rename = "cvtAskNoSpread",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvt_ask_no_spread: ::std::option::Option<f64>,
    #[doc = "Spec spelling — likely typo for 'cvtBidNoSpread'"]
    #[serde(
        rename = "cvtBiNoSpread",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvt_bi_no_spread: ::std::option::Option<f64>,
    #[serde(
        rename = "cvtBid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvt_bid: ::std::option::Option<f64>,
    #[serde(
        rename = "dailyPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub daily_price_change: ::std::option::Option<f64>,
    #[doc = "The display name of the instrument."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub displayname: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "exchangeID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exchange_id: ::std::option::Option<i64>,
    #[serde(
        rename = "holdingPct",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub holding_pct: ::std::option::Option<f64>,
    #[serde(
        rename = "industryNameId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub industry_name_id: ::std::option::Option<i64>,
    #[doc = "A unique identifier for the instrument."]
    #[serde(
        rename = "instrumentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[serde(
        rename = "instrumentType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "instrumentTypeID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_type_id: ::std::option::Option<i64>,
    #[serde(
        rename = "internalAssetClassId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internal_asset_class_id: ::std::option::Option<i64>,
    #[serde(
        rename = "internalAssetClassName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internal_asset_class_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "internalClosingPrice",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internal_closing_price: ::std::option::Option<f64>,
    #[serde(
        rename = "internalCryptoTypeId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internal_crypto_type_id: ::std::option::Option<i64>,
    #[serde(
        rename = "internalExchangeId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internal_exchange_id: ::std::option::Option<i64>,
    #[serde(
        rename = "internalExchangeName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internal_exchange_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "internalIndustryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internal_industry_id: ::std::option::Option<i64>,
    #[serde(
        rename = "internalInstrumentDisplayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internal_instrument_display_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "internalInstrumentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internal_instrument_id: ::std::option::Option<i64>,
    #[serde(
        rename = "internalStockIndustryName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internal_stock_industry_name: ::std::option::Option<::std::string::String>,
    #[doc = "The full internal symbol — use this for exact symbol matching"]
    #[serde(
        rename = "internalSymbolFull",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub internal_symbol_full: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "isActiveInPlatform",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_active_in_platform: ::std::option::Option<bool>,
    #[serde(
        rename = "isBuyEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_buy_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "isCurrentlyTradable",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_currently_tradable: ::std::option::Option<bool>,
    #[serde(
        rename = "isDelisted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_delisted: ::std::option::Option<bool>,
    #[serde(
        rename = "isExchangeOpen",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_exchange_open: ::std::option::Option<bool>,
    #[serde(
        rename = "isHiddenFromClient",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_hidden_from_client: ::std::option::Option<bool>,
    #[serde(
        rename = "isInternalInstrument",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_internal_instrument: ::std::option::Option<bool>,
    #[serde(
        rename = "isOpen",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_open: ::std::option::Option<bool>,
    #[serde(
        rename = "lastTwoYearsPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_two_years_price_change: ::std::option::Option<f64>,
    #[serde(
        rename = "lastYearPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_year_price_change: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logo150x150: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logo35x35: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logo50x50: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "monthlyPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub monthly_price_change: ::std::option::Option<f64>,
    #[serde(
        rename = "oneMonthAgoPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub one_month_ago_price_change: ::std::option::Option<f64>,
    #[serde(
        rename = "oneYearAgoPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub one_year_ago_price_change: ::std::option::Option<f64>,
    #[serde(
        rename = "oneYearPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub one_year_price_change: ::std::option::Option<f64>,
    #[serde(
        rename = "popularityUniques",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub popularity_uniques: ::std::option::Option<i64>,
    #[serde(
        rename = "popularityUniques14Day",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub popularity_uniques14_day: ::std::option::Option<i64>,
    #[serde(
        rename = "popularityUniques30Day",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub popularity_uniques30_day: ::std::option::Option<i64>,
    #[serde(
        rename = "popularityUniques7Day",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub popularity_uniques7_day: ::std::option::Option<i64>,
    #[serde(
        rename = "sectorNameId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sector_name_id: ::std::option::Option<i64>,
    #[serde(
        rename = "sellHoldingPct",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sell_holding_pct: ::std::option::Option<f64>,
    #[serde(
        rename = "sixMonthPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub six_month_price_change: ::std::option::Option<f64>,
    #[serde(
        rename = "sixMonthsAgoPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub six_months_ago_price_change: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub symbol: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "threeMonthPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub three_month_price_change: ::std::option::Option<f64>,
    #[serde(
        rename = "threeMonthsAgoPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub three_months_ago_price_change: ::std::option::Option<f64>,
    #[serde(
        rename = "traders14DayChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traders14_day_change: ::std::option::Option<f64>,
    #[serde(
        rename = "traders30DayChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traders30_day_change: ::std::option::Option<f64>,
    #[serde(
        rename = "traders7DayChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traders7_day_change: ::std::option::Option<f64>,
    #[serde(
        rename = "twoMonthsAgoPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub two_months_ago_price_change: ::std::option::Option<f64>,
    #[serde(
        rename = "weeklyPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub weekly_price_change: ::std::option::Option<f64>,
}
impl ::std::default::Default for Instrument {
    fn default() -> Self {
        Self {
            abs_buy_pct_change24_hours: Default::default(),
            abs_daily_price_change: Default::default(),
            buy_holding_pct: Default::default(),
            buy_pct_change24_hours: Default::default(),
            curr_month_price_change: Default::default(),
            curr_quarter_price_change: Default::default(),
            curr_year_price_change: Default::default(),
            current_rate: Default::default(),
            cvt_ask: Default::default(),
            cvt_ask_no_spread: Default::default(),
            cvt_bi_no_spread: Default::default(),
            cvt_bid: Default::default(),
            daily_price_change: Default::default(),
            displayname: Default::default(),
            exchange_id: Default::default(),
            holding_pct: Default::default(),
            industry_name_id: Default::default(),
            instrument_id: Default::default(),
            instrument_type: Default::default(),
            instrument_type_id: Default::default(),
            internal_asset_class_id: Default::default(),
            internal_asset_class_name: Default::default(),
            internal_closing_price: Default::default(),
            internal_crypto_type_id: Default::default(),
            internal_exchange_id: Default::default(),
            internal_exchange_name: Default::default(),
            internal_industry_id: Default::default(),
            internal_instrument_display_name: Default::default(),
            internal_instrument_id: Default::default(),
            internal_stock_industry_name: Default::default(),
            internal_symbol_full: Default::default(),
            is_active_in_platform: Default::default(),
            is_buy_enabled: Default::default(),
            is_currently_tradable: Default::default(),
            is_delisted: Default::default(),
            is_exchange_open: Default::default(),
            is_hidden_from_client: Default::default(),
            is_internal_instrument: Default::default(),
            is_open: Default::default(),
            last_two_years_price_change: Default::default(),
            last_year_price_change: Default::default(),
            logo150x150: Default::default(),
            logo35x35: Default::default(),
            logo50x50: Default::default(),
            monthly_price_change: Default::default(),
            one_month_ago_price_change: Default::default(),
            one_year_ago_price_change: Default::default(),
            one_year_price_change: Default::default(),
            popularity_uniques: Default::default(),
            popularity_uniques14_day: Default::default(),
            popularity_uniques30_day: Default::default(),
            popularity_uniques7_day: Default::default(),
            sector_name_id: Default::default(),
            sell_holding_pct: Default::default(),
            six_month_price_change: Default::default(),
            six_months_ago_price_change: Default::default(),
            symbol: Default::default(),
            three_month_price_change: Default::default(),
            three_months_ago_price_change: Default::default(),
            traders14_day_change: Default::default(),
            traders30_day_change: Default::default(),
            traders7_day_change: Default::default(),
            two_months_ago_price_change: Default::default(),
            weekly_price_change: Default::default(),
        }
    }
}
impl Instrument {
    pub fn builder() -> builder::Instrument {
        Default::default()
    }
}
#[doc = "`InstrumentSearchResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"items\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Instrument\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"page\": {"]
#[doc = "      \"description\": \"The current page number.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"pageSize\": {"]
#[doc = "      \"description\": \"The number of items per page.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"totalItems\": {"]
#[doc = "      \"description\": \"The total number of instruments matching the search criteria.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InstrumentSearchResponse {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub items: ::std::vec::Vec<Instrument>,
    #[doc = "The current page number."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<i64>,
    #[doc = "The number of items per page."]
    #[serde(
        rename = "pageSize",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub page_size: ::std::option::Option<i64>,
    #[doc = "The total number of instruments matching the search criteria."]
    #[serde(
        rename = "totalItems",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_items: ::std::option::Option<i64>,
}
impl ::std::default::Default for InstrumentSearchResponse {
    fn default() -> Self {
        Self {
            items: Default::default(),
            page: Default::default(),
            page_size: Default::default(),
            total_items: Default::default(),
        }
    }
}
impl InstrumentSearchResponse {
    pub fn builder() -> builder::InstrumentSearchResponse {
        Default::default()
    }
}
#[doc = "`InstrumentTypesResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"instrumentTypes\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"instrumentTypeDescription\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"instrumentTypeID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InstrumentTypesResponse {
    #[serde(
        rename = "instrumentTypes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub instrument_types: ::std::vec::Vec<InstrumentTypesResponseInstrumentTypesItem>,
}
impl ::std::default::Default for InstrumentTypesResponse {
    fn default() -> Self {
        Self {
            instrument_types: Default::default(),
        }
    }
}
impl InstrumentTypesResponse {
    pub fn builder() -> builder::InstrumentTypesResponse {
        Default::default()
    }
}
#[doc = "`InstrumentTypesResponseInstrumentTypesItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"instrumentTypeDescription\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"instrumentTypeID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InstrumentTypesResponseInstrumentTypesItem {
    #[serde(
        rename = "instrumentTypeDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_type_description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "instrumentTypeID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_type_id: ::std::option::Option<i64>,
}
impl ::std::default::Default for InstrumentTypesResponseInstrumentTypesItem {
    fn default() -> Self {
        Self {
            instrument_type_description: Default::default(),
            instrument_type_id: Default::default(),
        }
    }
}
impl InstrumentTypesResponseInstrumentTypesItem {
    pub fn builder() -> builder::InstrumentTypesResponseInstrumentTypesItem {
        Default::default()
    }
}
#[doc = "`InstrumentsResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"instrumentDisplayDatas\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"exchangeID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"hasExpirationDate\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"images\": {"]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"properties\": {"]
#[doc = "                \"backgroundColor\": {"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"height\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                },"]
#[doc = "                \"instrumentID\": {"]
#[doc = "                  \"type\": \"integer\""]
#[doc = "                },"]
#[doc = "                \"textColor\": {"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"uri\": {"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"width\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"instrumentDisplayName\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"instrumentID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"instrumentTypeID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"isInternalInstrument\": {"]
#[doc = "            \"description\": \"If true, the instrument is restricted from public access.\","]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"priceSource\": {"]
#[doc = "            \"description\": \"Data provider or market source (Nasdaq, LSE, CME, ...)\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"stocksIndustryId\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"symbolFull\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InstrumentsResponse {
    #[serde(
        rename = "instrumentDisplayDatas",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub instrument_display_datas: ::std::vec::Vec<InstrumentsResponseInstrumentDisplayDatasItem>,
}
impl ::std::default::Default for InstrumentsResponse {
    fn default() -> Self {
        Self {
            instrument_display_datas: Default::default(),
        }
    }
}
impl InstrumentsResponse {
    pub fn builder() -> builder::InstrumentsResponse {
        Default::default()
    }
}
#[doc = "`InstrumentsResponseInstrumentDisplayDatasItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"exchangeID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"hasExpirationDate\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"images\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"backgroundColor\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"height\": {"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"instrumentID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"textColor\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"uri\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"width\": {"]
#[doc = "            \"type\": \"number\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"instrumentDisplayName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"instrumentID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"instrumentTypeID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"isInternalInstrument\": {"]
#[doc = "      \"description\": \"If true, the instrument is restricted from public access.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"priceSource\": {"]
#[doc = "      \"description\": \"Data provider or market source (Nasdaq, LSE, CME, ...)\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"stocksIndustryId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"symbolFull\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InstrumentsResponseInstrumentDisplayDatasItem {
    #[serde(
        rename = "exchangeID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exchange_id: ::std::option::Option<i64>,
    #[serde(
        rename = "hasExpirationDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_expiration_date: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub images: ::std::vec::Vec<InstrumentsResponseInstrumentDisplayDatasItemImagesItem>,
    #[serde(
        rename = "instrumentDisplayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_display_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "instrumentID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[serde(
        rename = "instrumentTypeID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_type_id: ::std::option::Option<i64>,
    #[doc = "If true, the instrument is restricted from public access."]
    #[serde(
        rename = "isInternalInstrument",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_internal_instrument: ::std::option::Option<bool>,
    #[doc = "Data provider or market source (Nasdaq, LSE, CME, ...)"]
    #[serde(
        rename = "priceSource",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub price_source: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "stocksIndustryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stocks_industry_id: ::std::option::Option<i64>,
    #[serde(
        rename = "symbolFull",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub symbol_full: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for InstrumentsResponseInstrumentDisplayDatasItem {
    fn default() -> Self {
        Self {
            exchange_id: Default::default(),
            has_expiration_date: Default::default(),
            images: Default::default(),
            instrument_display_name: Default::default(),
            instrument_id: Default::default(),
            instrument_type_id: Default::default(),
            is_internal_instrument: Default::default(),
            price_source: Default::default(),
            stocks_industry_id: Default::default(),
            symbol_full: Default::default(),
        }
    }
}
impl InstrumentsResponseInstrumentDisplayDatasItem {
    pub fn builder() -> builder::InstrumentsResponseInstrumentDisplayDatasItem {
        Default::default()
    }
}
#[doc = "`InstrumentsResponseInstrumentDisplayDatasItemImagesItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"backgroundColor\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"height\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"instrumentID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"textColor\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"width\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InstrumentsResponseInstrumentDisplayDatasItemImagesItem {
    #[serde(
        rename = "backgroundColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub background_color: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<f64>,
    #[serde(
        rename = "instrumentID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[serde(
        rename = "textColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub text_color: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub uri: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<f64>,
}
impl ::std::default::Default for InstrumentsResponseInstrumentDisplayDatasItemImagesItem {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            height: Default::default(),
            instrument_id: Default::default(),
            text_color: Default::default(),
            uri: Default::default(),
            width: Default::default(),
        }
    }
}
impl InstrumentsResponseInstrumentDisplayDatasItemImagesItem {
    pub fn builder() -> builder::InstrumentsResponseInstrumentDisplayDatasItemImagesItem {
        Default::default()
    }
}
#[doc = "Container for real-time market rates data"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Container for real-time market rates data\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"rates\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"Individual instrument rate information\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"ask\": {"]
#[doc = "            \"description\": \"Buy price\","]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"bid\": {"]
#[doc = "            \"description\": \"Sell price\","]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"conversionRateAsk\": {"]
#[doc = "            \"description\": \"Currency → USD conversion (ask)\","]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"conversionRateBid\": {"]
#[doc = "            \"description\": \"Currency → USD conversion (bid)\","]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"date\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"date-time\""]
#[doc = "          },"]
#[doc = "          \"instrumentID\": {"]
#[doc = "            \"description\": \"CAPITAL ID — naming differs from search response\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"lastExecution\": {"]
#[doc = "            \"description\": \"Most recent trade execution price\","]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\""]
#[doc = "          },"]
#[doc = "          \"priceRateID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LiveRatesResponse {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub rates: ::std::vec::Vec<LiveRatesResponseRatesItem>,
}
impl ::std::default::Default for LiveRatesResponse {
    fn default() -> Self {
        Self {
            rates: Default::default(),
        }
    }
}
impl LiveRatesResponse {
    pub fn builder() -> builder::LiveRatesResponse {
        Default::default()
    }
}
#[doc = "Individual instrument rate information"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Individual instrument rate information\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"ask\": {"]
#[doc = "      \"description\": \"Buy price\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"bid\": {"]
#[doc = "      \"description\": \"Sell price\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"conversionRateAsk\": {"]
#[doc = "      \"description\": \"Currency → USD conversion (ask)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"conversionRateBid\": {"]
#[doc = "      \"description\": \"Currency → USD conversion (bid)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"date\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"instrumentID\": {"]
#[doc = "      \"description\": \"CAPITAL ID — naming differs from search response\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"lastExecution\": {"]
#[doc = "      \"description\": \"Most recent trade execution price\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    },"]
#[doc = "    \"priceRateID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LiveRatesResponseRatesItem {
    #[doc = "Buy price"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ask: ::std::option::Option<f32>,
    #[doc = "Sell price"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bid: ::std::option::Option<f32>,
    #[doc = "Currency → USD conversion (ask)"]
    #[serde(
        rename = "conversionRateAsk",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub conversion_rate_ask: ::std::option::Option<f32>,
    #[doc = "Currency → USD conversion (bid)"]
    #[serde(
        rename = "conversionRateBid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub conversion_rate_bid: ::std::option::Option<f32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "CAPITAL ID — naming differs from search response"]
    #[serde(
        rename = "instrumentID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[doc = "Most recent trade execution price"]
    #[serde(
        rename = "lastExecution",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_execution: ::std::option::Option<f32>,
    #[serde(
        rename = "priceRateID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub price_rate_id: ::std::option::Option<i64>,
}
impl ::std::default::Default for LiveRatesResponseRatesItem {
    fn default() -> Self {
        Self {
            ask: Default::default(),
            bid: Default::default(),
            conversion_rate_ask: Default::default(),
            conversion_rate_bid: Default::default(),
            date: Default::default(),
            instrument_id: Default::default(),
            last_execution: Default::default(),
            price_rate_id: Default::default(),
        }
    }
}
impl LiveRatesResponseRatesItem {
    pub fn builder() -> builder::LiveRatesResponseRatesItem {
        Default::default()
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
#[doc = "`MarketEventMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"earningReportId\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"earningsDate\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"earningsQuarter\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"earningsYear\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"estimatedEps\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\""]
#[doc = "    },"]
#[doc = "    \"estimatedSales\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\""]
#[doc = "    },"]
#[doc = "    \"isBeforeMarketOpen\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"market\": {"]
#[doc = "      \"$ref\": \"#/$defs/Market\""]
#[doc = "    },"]
#[doc = "    \"marketCap\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\""]
#[doc = "    },"]
#[doc = "    \"stocksIndustryId\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"tagName\": {"]
#[doc = "      \"$ref\": \"#/$defs/MarketEventTag\""]
#[doc = "    },"]
#[doc = "    \"textKey\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"verified\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MarketEventMetadata {
    #[serde(
        rename = "earningReportId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub earning_report_id: ::std::option::Option<i32>,
    #[serde(
        rename = "earningsDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub earnings_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "earningsQuarter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub earnings_quarter: ::std::option::Option<i32>,
    #[serde(
        rename = "earningsYear",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub earnings_year: ::std::option::Option<i32>,
    #[serde(
        rename = "estimatedEps",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub estimated_eps: ::std::option::Option<f64>,
    #[serde(
        rename = "estimatedSales",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub estimated_sales: ::std::option::Option<f64>,
    #[serde(
        rename = "isBeforeMarketOpen",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_before_market_open: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub market: ::std::option::Option<Market>,
    #[serde(
        rename = "marketCap",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub market_cap: ::std::option::Option<f64>,
    #[serde(
        rename = "stocksIndustryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stocks_industry_id: ::std::option::Option<i32>,
    #[serde(
        rename = "tagName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tag_name: ::std::option::Option<MarketEventTag>,
    #[serde(
        rename = "textKey",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub text_key: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub verified: ::std::option::Option<bool>,
}
impl ::std::default::Default for MarketEventMetadata {
    fn default() -> Self {
        Self {
            earning_report_id: Default::default(),
            earnings_date: Default::default(),
            earnings_quarter: Default::default(),
            earnings_year: Default::default(),
            estimated_eps: Default::default(),
            estimated_sales: Default::default(),
            is_before_market_open: Default::default(),
            market: Default::default(),
            market_cap: Default::default(),
            stocks_industry_id: Default::default(),
            tag_name: Default::default(),
            text_key: Default::default(),
            verified: Default::default(),
        }
    }
}
impl MarketEventMetadata {
    pub fn builder() -> builder::MarketEventMetadata {
        Default::default()
    }
}
#[doc = "Enum encoded as integer index"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Enum encoded as integer index\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"int32\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Reports\","]
#[doc = "    \"Dividends\","]
#[doc = "    \"Split\","]
#[doc = "    \"ReverseSplit\""]
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
pub enum MarketEventTag {
    Reports,
    Dividends,
    Split,
    ReverseSplit,
}
impl ::std::fmt::Display for MarketEventTag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Reports => f.write_str("Reports"),
            Self::Dividends => f.write_str("Dividends"),
            Self::Split => f.write_str("Split"),
            Self::ReverseSplit => f.write_str("ReverseSplit"),
        }
    }
}
impl ::std::str::FromStr for MarketEventTag {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Reports" => Ok(Self::Reports),
            "Dividends" => Ok(Self::Dividends),
            "Split" => Ok(Self::Split),
            "ReverseSplit" => Ok(Self::ReverseSplit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MarketEventTag {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MarketEventTag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MarketEventTag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Response containing market recommendations"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Response containing market recommendations\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"Recommendations\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\","]
#[doc = "        \"format\": \"int32\""]
#[doc = "      },"]
#[doc = "      \"example\": ["]
#[doc = "        12345,"]
#[doc = "        67890"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ResponseType\": {"]
#[doc = "      \"description\": \"Type of recommendation response\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Instrument\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MarketRecommendationsResponse {
    #[serde(
        rename = "Recommendations",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub recommendations: ::std::vec::Vec<i32>,
    #[doc = "Type of recommendation response"]
    #[serde(
        rename = "ResponseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub response_type: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for MarketRecommendationsResponse {
    fn default() -> Self {
        Self {
            recommendations: Default::default(),
            response_type: Default::default(),
        }
    }
}
impl MarketRecommendationsResponse {
    pub fn builder() -> builder::MarketRecommendationsResponse {
        Default::default()
    }
}
#[doc = "`StocksIndustriesResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"stocksIndustries\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"industryID\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"industryName\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StocksIndustriesResponse {
    #[serde(
        rename = "stocksIndustries",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub stocks_industries: ::std::vec::Vec<StocksIndustriesResponseStocksIndustriesItem>,
}
impl ::std::default::Default for StocksIndustriesResponse {
    fn default() -> Self {
        Self {
            stocks_industries: Default::default(),
        }
    }
}
impl StocksIndustriesResponse {
    pub fn builder() -> builder::StocksIndustriesResponse {
        Default::default()
    }
}
#[doc = "`StocksIndustriesResponseStocksIndustriesItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"industryID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"industryName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StocksIndustriesResponseStocksIndustriesItem {
    #[serde(
        rename = "industryID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub industry_id: ::std::option::Option<i64>,
    #[serde(
        rename = "industryName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub industry_name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for StocksIndustriesResponseStocksIndustriesItem {
    fn default() -> Self {
        Self {
            industry_id: Default::default(),
            industry_name: Default::default(),
        }
    }
}
impl StocksIndustriesResponseStocksIndustriesItem {
    pub fn builder() -> builder::StocksIndustriesResponseStocksIndustriesItem {
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
    pub struct CandlesResponse {
        candles: ::std::result::Result<
            ::std::vec::Vec<super::CandlesResponseCandlesItem>,
            ::std::string::String,
        >,
        interval: ::std::result::Result<
            ::std::option::Option<super::CandlesResponseInterval>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CandlesResponse {
        fn default() -> Self {
            Self {
                candles: Ok(Default::default()),
                interval: Ok(Default::default()),
            }
        }
    }
    impl CandlesResponse {
        pub fn candles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CandlesResponseCandlesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.candles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for candles: {e}"));
            self
        }
        pub fn interval<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CandlesResponseInterval>>,
            T::Error: ::std::fmt::Display,
        {
            self.interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interval: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CandlesResponse> for super::CandlesResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CandlesResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                candles: value.candles?,
                interval: value.interval?,
            })
        }
    }
    impl ::std::convert::From<super::CandlesResponse> for CandlesResponse {
        fn from(value: super::CandlesResponse) -> Self {
            Self {
                candles: Ok(value.candles),
                interval: Ok(value.interval),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CandlesResponseCandlesItem {
        candles: ::std::result::Result<
            ::std::vec::Vec<super::CandlesResponseCandlesItemCandlesItem>,
            ::std::string::String,
        >,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        range_close: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        range_high: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        range_low: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        range_open: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        volume: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for CandlesResponseCandlesItem {
        fn default() -> Self {
            Self {
                candles: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                range_close: Ok(Default::default()),
                range_high: Ok(Default::default()),
                range_low: Ok(Default::default()),
                range_open: Ok(Default::default()),
                volume: Ok(Default::default()),
            }
        }
    }
    impl CandlesResponseCandlesItem {
        pub fn candles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::CandlesResponseCandlesItemCandlesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.candles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for candles: {e}"));
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
        pub fn range_close<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.range_close = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for range_close: {e}"));
            self
        }
        pub fn range_high<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.range_high = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for range_high: {e}"));
            self
        }
        pub fn range_low<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.range_low = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for range_low: {e}"));
            self
        }
        pub fn range_open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.range_open = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for range_open: {e}"));
            self
        }
        pub fn volume<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.volume = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for volume: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CandlesResponseCandlesItem> for super::CandlesResponseCandlesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CandlesResponseCandlesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                candles: value.candles?,
                instrument_id: value.instrument_id?,
                range_close: value.range_close?,
                range_high: value.range_high?,
                range_low: value.range_low?,
                range_open: value.range_open?,
                volume: value.volume?,
            })
        }
    }
    impl ::std::convert::From<super::CandlesResponseCandlesItem> for CandlesResponseCandlesItem {
        fn from(value: super::CandlesResponseCandlesItem) -> Self {
            Self {
                candles: Ok(value.candles),
                instrument_id: Ok(value.instrument_id),
                range_close: Ok(value.range_close),
                range_high: Ok(value.range_high),
                range_low: Ok(value.range_low),
                range_open: Ok(value.range_open),
                volume: Ok(value.volume),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CandlesResponseCandlesItemCandlesItem {
        close: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        from_date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        high: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        low: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        open: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        volume: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for CandlesResponseCandlesItemCandlesItem {
        fn default() -> Self {
            Self {
                close: Ok(Default::default()),
                from_date: Ok(Default::default()),
                high: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                low: Ok(Default::default()),
                open: Ok(Default::default()),
                volume: Ok(Default::default()),
            }
        }
    }
    impl CandlesResponseCandlesItemCandlesItem {
        pub fn close<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.close = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for close: {e}"));
            self
        }
        pub fn from_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.from_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for from_date: {e}"));
            self
        }
        pub fn high<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.high = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for high: {e}"));
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
        pub fn low<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.low = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for low: {e}"));
            self
        }
        pub fn open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.open = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open: {e}"));
            self
        }
        pub fn volume<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.volume = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for volume: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CandlesResponseCandlesItemCandlesItem>
        for super::CandlesResponseCandlesItemCandlesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CandlesResponseCandlesItemCandlesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                close: value.close?,
                from_date: value.from_date?,
                high: value.high?,
                instrument_id: value.instrument_id?,
                low: value.low?,
                open: value.open?,
                volume: value.volume?,
            })
        }
    }
    impl ::std::convert::From<super::CandlesResponseCandlesItemCandlesItem>
        for CandlesResponseCandlesItemCandlesItem
    {
        fn from(value: super::CandlesResponseCandlesItemCandlesItem) -> Self {
            Self {
                close: Ok(value.close),
                from_date: Ok(value.from_date),
                high: Ok(value.high),
                instrument_id: Ok(value.instrument_id),
                low: Ok(value.low),
                open: Ok(value.open),
                volume: Ok(value.volume),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClosingPricesResponseItem {
        closing_prices: ::std::result::Result<
            ::std::option::Option<super::ClosingPricesResponseItemClosingPrices>,
            ::std::string::String,
        >,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        is_market_open: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        official_closing_price:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for ClosingPricesResponseItem {
        fn default() -> Self {
            Self {
                closing_prices: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                is_market_open: Ok(Default::default()),
                official_closing_price: Ok(Default::default()),
            }
        }
    }
    impl ClosingPricesResponseItem {
        pub fn closing_prices<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ClosingPricesResponseItemClosingPrices>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.closing_prices = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for closing_prices: {e}"));
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
        pub fn is_market_open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_market_open = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_market_open: {e}"));
            self
        }
        pub fn official_closing_price<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.official_closing_price = value.try_into().map_err(|e| {
                format!("error converting supplied value for official_closing_price: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ClosingPricesResponseItem> for super::ClosingPricesResponseItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClosingPricesResponseItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                closing_prices: value.closing_prices?,
                instrument_id: value.instrument_id?,
                is_market_open: value.is_market_open?,
                official_closing_price: value.official_closing_price?,
            })
        }
    }
    impl ::std::convert::From<super::ClosingPricesResponseItem> for ClosingPricesResponseItem {
        fn from(value: super::ClosingPricesResponseItem) -> Self {
            Self {
                closing_prices: Ok(value.closing_prices),
                instrument_id: Ok(value.instrument_id),
                is_market_open: Ok(value.is_market_open),
                official_closing_price: Ok(value.official_closing_price),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClosingPricesResponseItemClosingPrices {
        daily: ::std::result::Result<
            ::std::option::Option<super::ClosingPricesResponseItemClosingPricesDaily>,
            ::std::string::String,
        >,
        monthly: ::std::result::Result<
            ::std::option::Option<super::ClosingPricesResponseItemClosingPricesMonthly>,
            ::std::string::String,
        >,
        weekly: ::std::result::Result<
            ::std::option::Option<super::ClosingPricesResponseItemClosingPricesWeekly>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ClosingPricesResponseItemClosingPrices {
        fn default() -> Self {
            Self {
                daily: Ok(Default::default()),
                monthly: Ok(Default::default()),
                weekly: Ok(Default::default()),
            }
        }
    }
    impl ClosingPricesResponseItemClosingPrices {
        pub fn daily<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ClosingPricesResponseItemClosingPricesDaily>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.daily = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for daily: {e}"));
            self
        }
        pub fn monthly<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ClosingPricesResponseItemClosingPricesMonthly>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.monthly = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for monthly: {e}"));
            self
        }
        pub fn weekly<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ClosingPricesResponseItemClosingPricesWeekly>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.weekly = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weekly: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClosingPricesResponseItemClosingPrices>
        for super::ClosingPricesResponseItemClosingPrices
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClosingPricesResponseItemClosingPrices,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                daily: value.daily?,
                monthly: value.monthly?,
                weekly: value.weekly?,
            })
        }
    }
    impl ::std::convert::From<super::ClosingPricesResponseItemClosingPrices>
        for ClosingPricesResponseItemClosingPrices
    {
        fn from(value: super::ClosingPricesResponseItemClosingPrices) -> Self {
            Self {
                daily: Ok(value.daily),
                monthly: Ok(value.monthly),
                weekly: Ok(value.weekly),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClosingPricesResponseItemClosingPricesDaily {
        date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        price: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for ClosingPricesResponseItemClosingPricesDaily {
        fn default() -> Self {
            Self {
                date: Ok(Default::default()),
                price: Ok(Default::default()),
            }
        }
    }
    impl ClosingPricesResponseItemClosingPricesDaily {
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {e}"));
            self
        }
        pub fn price<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.price = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for price: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClosingPricesResponseItemClosingPricesDaily>
        for super::ClosingPricesResponseItemClosingPricesDaily
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClosingPricesResponseItemClosingPricesDaily,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                date: value.date?,
                price: value.price?,
            })
        }
    }
    impl ::std::convert::From<super::ClosingPricesResponseItemClosingPricesDaily>
        for ClosingPricesResponseItemClosingPricesDaily
    {
        fn from(value: super::ClosingPricesResponseItemClosingPricesDaily) -> Self {
            Self {
                date: Ok(value.date),
                price: Ok(value.price),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClosingPricesResponseItemClosingPricesMonthly {
        date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        price: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for ClosingPricesResponseItemClosingPricesMonthly {
        fn default() -> Self {
            Self {
                date: Ok(Default::default()),
                price: Ok(Default::default()),
            }
        }
    }
    impl ClosingPricesResponseItemClosingPricesMonthly {
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {e}"));
            self
        }
        pub fn price<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.price = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for price: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClosingPricesResponseItemClosingPricesMonthly>
        for super::ClosingPricesResponseItemClosingPricesMonthly
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClosingPricesResponseItemClosingPricesMonthly,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                date: value.date?,
                price: value.price?,
            })
        }
    }
    impl ::std::convert::From<super::ClosingPricesResponseItemClosingPricesMonthly>
        for ClosingPricesResponseItemClosingPricesMonthly
    {
        fn from(value: super::ClosingPricesResponseItemClosingPricesMonthly) -> Self {
            Self {
                date: Ok(value.date),
                price: Ok(value.price),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClosingPricesResponseItemClosingPricesWeekly {
        date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        price: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for ClosingPricesResponseItemClosingPricesWeekly {
        fn default() -> Self {
            Self {
                date: Ok(Default::default()),
                price: Ok(Default::default()),
            }
        }
    }
    impl ClosingPricesResponseItemClosingPricesWeekly {
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {e}"));
            self
        }
        pub fn price<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.price = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for price: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClosingPricesResponseItemClosingPricesWeekly>
        for super::ClosingPricesResponseItemClosingPricesWeekly
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClosingPricesResponseItemClosingPricesWeekly,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                date: value.date?,
                price: value.price?,
            })
        }
    }
    impl ::std::convert::From<super::ClosingPricesResponseItemClosingPricesWeekly>
        for ClosingPricesResponseItemClosingPricesWeekly
    {
        fn from(value: super::ClosingPricesResponseItemClosingPricesWeekly) -> Self {
            Self {
                date: Ok(value.date),
                price: Ok(value.price),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExchangesResponse {
        exchange_info: ::std::result::Result<
            ::std::vec::Vec<super::ExchangesResponseExchangeInfoItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ExchangesResponse {
        fn default() -> Self {
            Self {
                exchange_info: Ok(Default::default()),
            }
        }
    }
    impl ExchangesResponse {
        pub fn exchange_info<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ExchangesResponseExchangeInfoItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.exchange_info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exchange_info: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ExchangesResponse> for super::ExchangesResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExchangesResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                exchange_info: value.exchange_info?,
            })
        }
    }
    impl ::std::convert::From<super::ExchangesResponse> for ExchangesResponse {
        fn from(value: super::ExchangesResponse) -> Self {
            Self {
                exchange_info: Ok(value.exchange_info),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExchangesResponseExchangeInfoItem {
        exchange_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        exchange_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ExchangesResponseExchangeInfoItem {
        fn default() -> Self {
            Self {
                exchange_description: Ok(Default::default()),
                exchange_id: Ok(Default::default()),
            }
        }
    }
    impl ExchangesResponseExchangeInfoItem {
        pub fn exchange_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.exchange_description = value.try_into().map_err(|e| {
                format!("error converting supplied value for exchange_description: {e}")
            });
            self
        }
        pub fn exchange_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.exchange_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exchange_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ExchangesResponseExchangeInfoItem>
        for super::ExchangesResponseExchangeInfoItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExchangesResponseExchangeInfoItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                exchange_description: value.exchange_description?,
                exchange_id: value.exchange_id?,
            })
        }
    }
    impl ::std::convert::From<super::ExchangesResponseExchangeInfoItem>
        for ExchangesResponseExchangeInfoItem
    {
        fn from(value: super::ExchangesResponseExchangeInfoItem) -> Self {
            Self {
                exchange_description: Ok(value.exchange_description),
                exchange_id: Ok(value.exchange_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Instrument {
        abs_buy_pct_change24_hours:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        abs_daily_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        buy_holding_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        buy_pct_change24_hours:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        curr_month_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        curr_quarter_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        curr_year_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        current_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        cvt_ask: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        cvt_ask_no_spread: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        cvt_bi_no_spread: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        cvt_bid: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        daily_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        displayname: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        exchange_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        holding_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        industry_name_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        instrument_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        instrument_type_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        internal_asset_class_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        internal_asset_class_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        internal_closing_price:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        internal_crypto_type_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        internal_exchange_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        internal_exchange_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        internal_industry_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        internal_instrument_display_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        internal_instrument_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        internal_stock_industry_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        internal_symbol_full: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        is_active_in_platform:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_buy_enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_currently_tradable:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_delisted: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_exchange_open: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_hidden_from_client:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_internal_instrument:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_open: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        last_two_years_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        last_year_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        logo150x150: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        logo35x35: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        logo50x50: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        monthly_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        one_month_ago_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        one_year_ago_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        one_year_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        popularity_uniques:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        popularity_uniques14_day:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        popularity_uniques30_day:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        popularity_uniques7_day:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        sector_name_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        sell_holding_pct: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        six_month_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        six_months_ago_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        symbol: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        three_month_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        three_months_ago_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        traders14_day_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        traders30_day_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        traders7_day_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        two_months_ago_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        weekly_price_change:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for Instrument {
        fn default() -> Self {
            Self {
                abs_buy_pct_change24_hours: Ok(Default::default()),
                abs_daily_price_change: Ok(Default::default()),
                buy_holding_pct: Ok(Default::default()),
                buy_pct_change24_hours: Ok(Default::default()),
                curr_month_price_change: Ok(Default::default()),
                curr_quarter_price_change: Ok(Default::default()),
                curr_year_price_change: Ok(Default::default()),
                current_rate: Ok(Default::default()),
                cvt_ask: Ok(Default::default()),
                cvt_ask_no_spread: Ok(Default::default()),
                cvt_bi_no_spread: Ok(Default::default()),
                cvt_bid: Ok(Default::default()),
                daily_price_change: Ok(Default::default()),
                displayname: Ok(Default::default()),
                exchange_id: Ok(Default::default()),
                holding_pct: Ok(Default::default()),
                industry_name_id: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                instrument_type: Ok(Default::default()),
                instrument_type_id: Ok(Default::default()),
                internal_asset_class_id: Ok(Default::default()),
                internal_asset_class_name: Ok(Default::default()),
                internal_closing_price: Ok(Default::default()),
                internal_crypto_type_id: Ok(Default::default()),
                internal_exchange_id: Ok(Default::default()),
                internal_exchange_name: Ok(Default::default()),
                internal_industry_id: Ok(Default::default()),
                internal_instrument_display_name: Ok(Default::default()),
                internal_instrument_id: Ok(Default::default()),
                internal_stock_industry_name: Ok(Default::default()),
                internal_symbol_full: Ok(Default::default()),
                is_active_in_platform: Ok(Default::default()),
                is_buy_enabled: Ok(Default::default()),
                is_currently_tradable: Ok(Default::default()),
                is_delisted: Ok(Default::default()),
                is_exchange_open: Ok(Default::default()),
                is_hidden_from_client: Ok(Default::default()),
                is_internal_instrument: Ok(Default::default()),
                is_open: Ok(Default::default()),
                last_two_years_price_change: Ok(Default::default()),
                last_year_price_change: Ok(Default::default()),
                logo150x150: Ok(Default::default()),
                logo35x35: Ok(Default::default()),
                logo50x50: Ok(Default::default()),
                monthly_price_change: Ok(Default::default()),
                one_month_ago_price_change: Ok(Default::default()),
                one_year_ago_price_change: Ok(Default::default()),
                one_year_price_change: Ok(Default::default()),
                popularity_uniques: Ok(Default::default()),
                popularity_uniques14_day: Ok(Default::default()),
                popularity_uniques30_day: Ok(Default::default()),
                popularity_uniques7_day: Ok(Default::default()),
                sector_name_id: Ok(Default::default()),
                sell_holding_pct: Ok(Default::default()),
                six_month_price_change: Ok(Default::default()),
                six_months_ago_price_change: Ok(Default::default()),
                symbol: Ok(Default::default()),
                three_month_price_change: Ok(Default::default()),
                three_months_ago_price_change: Ok(Default::default()),
                traders14_day_change: Ok(Default::default()),
                traders30_day_change: Ok(Default::default()),
                traders7_day_change: Ok(Default::default()),
                two_months_ago_price_change: Ok(Default::default()),
                weekly_price_change: Ok(Default::default()),
            }
        }
    }
    impl Instrument {
        pub fn abs_buy_pct_change24_hours<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.abs_buy_pct_change24_hours = value.try_into().map_err(|e| {
                format!("error converting supplied value for abs_buy_pct_change24_hours: {e}")
            });
            self
        }
        pub fn abs_daily_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.abs_daily_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for abs_daily_price_change: {e}")
            });
            self
        }
        pub fn buy_holding_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.buy_holding_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for buy_holding_pct: {e}"));
            self
        }
        pub fn buy_pct_change24_hours<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.buy_pct_change24_hours = value.try_into().map_err(|e| {
                format!("error converting supplied value for buy_pct_change24_hours: {e}")
            });
            self
        }
        pub fn curr_month_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.curr_month_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for curr_month_price_change: {e}")
            });
            self
        }
        pub fn curr_quarter_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.curr_quarter_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for curr_quarter_price_change: {e}")
            });
            self
        }
        pub fn curr_year_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.curr_year_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for curr_year_price_change: {e}")
            });
            self
        }
        pub fn current_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.current_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for current_rate: {e}"));
            self
        }
        pub fn cvt_ask<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cvt_ask = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cvt_ask: {e}"));
            self
        }
        pub fn cvt_ask_no_spread<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cvt_ask_no_spread = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cvt_ask_no_spread: {e}"));
            self
        }
        pub fn cvt_bi_no_spread<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cvt_bi_no_spread = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cvt_bi_no_spread: {e}"));
            self
        }
        pub fn cvt_bid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cvt_bid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cvt_bid: {e}"));
            self
        }
        pub fn daily_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.daily_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for daily_price_change: {e}")
            });
            self
        }
        pub fn displayname<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.displayname = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for displayname: {e}"));
            self
        }
        pub fn exchange_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.exchange_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exchange_id: {e}"));
            self
        }
        pub fn holding_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.holding_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for holding_pct: {e}"));
            self
        }
        pub fn industry_name_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.industry_name_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for industry_name_id: {e}"));
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
        pub fn instrument_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instrument_type: {e}"));
            self
        }
        pub fn instrument_type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_type_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for instrument_type_id: {e}")
            });
            self
        }
        pub fn internal_asset_class_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.internal_asset_class_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for internal_asset_class_id: {e}")
            });
            self
        }
        pub fn internal_asset_class_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.internal_asset_class_name = value.try_into().map_err(|e| {
                format!("error converting supplied value for internal_asset_class_name: {e}")
            });
            self
        }
        pub fn internal_closing_price<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.internal_closing_price = value.try_into().map_err(|e| {
                format!("error converting supplied value for internal_closing_price: {e}")
            });
            self
        }
        pub fn internal_crypto_type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.internal_crypto_type_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for internal_crypto_type_id: {e}")
            });
            self
        }
        pub fn internal_exchange_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.internal_exchange_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for internal_exchange_id: {e}")
            });
            self
        }
        pub fn internal_exchange_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.internal_exchange_name = value.try_into().map_err(|e| {
                format!("error converting supplied value for internal_exchange_name: {e}")
            });
            self
        }
        pub fn internal_industry_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.internal_industry_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for internal_industry_id: {e}")
            });
            self
        }
        pub fn internal_instrument_display_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.internal_instrument_display_name = value.try_into().map_err(|e| {
                format!("error converting supplied value for internal_instrument_display_name: {e}")
            });
            self
        }
        pub fn internal_instrument_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.internal_instrument_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for internal_instrument_id: {e}")
            });
            self
        }
        pub fn internal_stock_industry_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.internal_stock_industry_name = value.try_into().map_err(|e| {
                format!("error converting supplied value for internal_stock_industry_name: {e}")
            });
            self
        }
        pub fn internal_symbol_full<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.internal_symbol_full = value.try_into().map_err(|e| {
                format!("error converting supplied value for internal_symbol_full: {e}")
            });
            self
        }
        pub fn is_active_in_platform<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_active_in_platform = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_active_in_platform: {e}")
            });
            self
        }
        pub fn is_buy_enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_buy_enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_buy_enabled: {e}"));
            self
        }
        pub fn is_currently_tradable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_currently_tradable = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_currently_tradable: {e}")
            });
            self
        }
        pub fn is_delisted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_delisted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_delisted: {e}"));
            self
        }
        pub fn is_exchange_open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_exchange_open = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_exchange_open: {e}"));
            self
        }
        pub fn is_hidden_from_client<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_hidden_from_client = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_hidden_from_client: {e}")
            });
            self
        }
        pub fn is_internal_instrument<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_internal_instrument = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_internal_instrument: {e}")
            });
            self
        }
        pub fn is_open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_open = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_open: {e}"));
            self
        }
        pub fn last_two_years_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_two_years_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for last_two_years_price_change: {e}")
            });
            self
        }
        pub fn last_year_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_year_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for last_year_price_change: {e}")
            });
            self
        }
        pub fn logo150x150<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.logo150x150 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logo150x150: {e}"));
            self
        }
        pub fn logo35x35<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.logo35x35 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logo35x35: {e}"));
            self
        }
        pub fn logo50x50<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.logo50x50 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logo50x50: {e}"));
            self
        }
        pub fn monthly_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.monthly_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for monthly_price_change: {e}")
            });
            self
        }
        pub fn one_month_ago_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.one_month_ago_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for one_month_ago_price_change: {e}")
            });
            self
        }
        pub fn one_year_ago_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.one_year_ago_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for one_year_ago_price_change: {e}")
            });
            self
        }
        pub fn one_year_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.one_year_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for one_year_price_change: {e}")
            });
            self
        }
        pub fn popularity_uniques<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.popularity_uniques = value.try_into().map_err(|e| {
                format!("error converting supplied value for popularity_uniques: {e}")
            });
            self
        }
        pub fn popularity_uniques14_day<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.popularity_uniques14_day = value.try_into().map_err(|e| {
                format!("error converting supplied value for popularity_uniques14_day: {e}")
            });
            self
        }
        pub fn popularity_uniques30_day<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.popularity_uniques30_day = value.try_into().map_err(|e| {
                format!("error converting supplied value for popularity_uniques30_day: {e}")
            });
            self
        }
        pub fn popularity_uniques7_day<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.popularity_uniques7_day = value.try_into().map_err(|e| {
                format!("error converting supplied value for popularity_uniques7_day: {e}")
            });
            self
        }
        pub fn sector_name_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.sector_name_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sector_name_id: {e}"));
            self
        }
        pub fn sell_holding_pct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.sell_holding_pct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sell_holding_pct: {e}"));
            self
        }
        pub fn six_month_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.six_month_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for six_month_price_change: {e}")
            });
            self
        }
        pub fn six_months_ago_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.six_months_ago_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for six_months_ago_price_change: {e}")
            });
            self
        }
        pub fn symbol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.symbol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbol: {e}"));
            self
        }
        pub fn three_month_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.three_month_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for three_month_price_change: {e}")
            });
            self
        }
        pub fn three_months_ago_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.three_months_ago_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for three_months_ago_price_change: {e}")
            });
            self
        }
        pub fn traders14_day_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.traders14_day_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for traders14_day_change: {e}")
            });
            self
        }
        pub fn traders30_day_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.traders30_day_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for traders30_day_change: {e}")
            });
            self
        }
        pub fn traders7_day_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.traders7_day_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for traders7_day_change: {e}")
            });
            self
        }
        pub fn two_months_ago_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.two_months_ago_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for two_months_ago_price_change: {e}")
            });
            self
        }
        pub fn weekly_price_change<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.weekly_price_change = value.try_into().map_err(|e| {
                format!("error converting supplied value for weekly_price_change: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<Instrument> for super::Instrument {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Instrument,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                abs_buy_pct_change24_hours: value.abs_buy_pct_change24_hours?,
                abs_daily_price_change: value.abs_daily_price_change?,
                buy_holding_pct: value.buy_holding_pct?,
                buy_pct_change24_hours: value.buy_pct_change24_hours?,
                curr_month_price_change: value.curr_month_price_change?,
                curr_quarter_price_change: value.curr_quarter_price_change?,
                curr_year_price_change: value.curr_year_price_change?,
                current_rate: value.current_rate?,
                cvt_ask: value.cvt_ask?,
                cvt_ask_no_spread: value.cvt_ask_no_spread?,
                cvt_bi_no_spread: value.cvt_bi_no_spread?,
                cvt_bid: value.cvt_bid?,
                daily_price_change: value.daily_price_change?,
                displayname: value.displayname?,
                exchange_id: value.exchange_id?,
                holding_pct: value.holding_pct?,
                industry_name_id: value.industry_name_id?,
                instrument_id: value.instrument_id?,
                instrument_type: value.instrument_type?,
                instrument_type_id: value.instrument_type_id?,
                internal_asset_class_id: value.internal_asset_class_id?,
                internal_asset_class_name: value.internal_asset_class_name?,
                internal_closing_price: value.internal_closing_price?,
                internal_crypto_type_id: value.internal_crypto_type_id?,
                internal_exchange_id: value.internal_exchange_id?,
                internal_exchange_name: value.internal_exchange_name?,
                internal_industry_id: value.internal_industry_id?,
                internal_instrument_display_name: value.internal_instrument_display_name?,
                internal_instrument_id: value.internal_instrument_id?,
                internal_stock_industry_name: value.internal_stock_industry_name?,
                internal_symbol_full: value.internal_symbol_full?,
                is_active_in_platform: value.is_active_in_platform?,
                is_buy_enabled: value.is_buy_enabled?,
                is_currently_tradable: value.is_currently_tradable?,
                is_delisted: value.is_delisted?,
                is_exchange_open: value.is_exchange_open?,
                is_hidden_from_client: value.is_hidden_from_client?,
                is_internal_instrument: value.is_internal_instrument?,
                is_open: value.is_open?,
                last_two_years_price_change: value.last_two_years_price_change?,
                last_year_price_change: value.last_year_price_change?,
                logo150x150: value.logo150x150?,
                logo35x35: value.logo35x35?,
                logo50x50: value.logo50x50?,
                monthly_price_change: value.monthly_price_change?,
                one_month_ago_price_change: value.one_month_ago_price_change?,
                one_year_ago_price_change: value.one_year_ago_price_change?,
                one_year_price_change: value.one_year_price_change?,
                popularity_uniques: value.popularity_uniques?,
                popularity_uniques14_day: value.popularity_uniques14_day?,
                popularity_uniques30_day: value.popularity_uniques30_day?,
                popularity_uniques7_day: value.popularity_uniques7_day?,
                sector_name_id: value.sector_name_id?,
                sell_holding_pct: value.sell_holding_pct?,
                six_month_price_change: value.six_month_price_change?,
                six_months_ago_price_change: value.six_months_ago_price_change?,
                symbol: value.symbol?,
                three_month_price_change: value.three_month_price_change?,
                three_months_ago_price_change: value.three_months_ago_price_change?,
                traders14_day_change: value.traders14_day_change?,
                traders30_day_change: value.traders30_day_change?,
                traders7_day_change: value.traders7_day_change?,
                two_months_ago_price_change: value.two_months_ago_price_change?,
                weekly_price_change: value.weekly_price_change?,
            })
        }
    }
    impl ::std::convert::From<super::Instrument> for Instrument {
        fn from(value: super::Instrument) -> Self {
            Self {
                abs_buy_pct_change24_hours: Ok(value.abs_buy_pct_change24_hours),
                abs_daily_price_change: Ok(value.abs_daily_price_change),
                buy_holding_pct: Ok(value.buy_holding_pct),
                buy_pct_change24_hours: Ok(value.buy_pct_change24_hours),
                curr_month_price_change: Ok(value.curr_month_price_change),
                curr_quarter_price_change: Ok(value.curr_quarter_price_change),
                curr_year_price_change: Ok(value.curr_year_price_change),
                current_rate: Ok(value.current_rate),
                cvt_ask: Ok(value.cvt_ask),
                cvt_ask_no_spread: Ok(value.cvt_ask_no_spread),
                cvt_bi_no_spread: Ok(value.cvt_bi_no_spread),
                cvt_bid: Ok(value.cvt_bid),
                daily_price_change: Ok(value.daily_price_change),
                displayname: Ok(value.displayname),
                exchange_id: Ok(value.exchange_id),
                holding_pct: Ok(value.holding_pct),
                industry_name_id: Ok(value.industry_name_id),
                instrument_id: Ok(value.instrument_id),
                instrument_type: Ok(value.instrument_type),
                instrument_type_id: Ok(value.instrument_type_id),
                internal_asset_class_id: Ok(value.internal_asset_class_id),
                internal_asset_class_name: Ok(value.internal_asset_class_name),
                internal_closing_price: Ok(value.internal_closing_price),
                internal_crypto_type_id: Ok(value.internal_crypto_type_id),
                internal_exchange_id: Ok(value.internal_exchange_id),
                internal_exchange_name: Ok(value.internal_exchange_name),
                internal_industry_id: Ok(value.internal_industry_id),
                internal_instrument_display_name: Ok(value.internal_instrument_display_name),
                internal_instrument_id: Ok(value.internal_instrument_id),
                internal_stock_industry_name: Ok(value.internal_stock_industry_name),
                internal_symbol_full: Ok(value.internal_symbol_full),
                is_active_in_platform: Ok(value.is_active_in_platform),
                is_buy_enabled: Ok(value.is_buy_enabled),
                is_currently_tradable: Ok(value.is_currently_tradable),
                is_delisted: Ok(value.is_delisted),
                is_exchange_open: Ok(value.is_exchange_open),
                is_hidden_from_client: Ok(value.is_hidden_from_client),
                is_internal_instrument: Ok(value.is_internal_instrument),
                is_open: Ok(value.is_open),
                last_two_years_price_change: Ok(value.last_two_years_price_change),
                last_year_price_change: Ok(value.last_year_price_change),
                logo150x150: Ok(value.logo150x150),
                logo35x35: Ok(value.logo35x35),
                logo50x50: Ok(value.logo50x50),
                monthly_price_change: Ok(value.monthly_price_change),
                one_month_ago_price_change: Ok(value.one_month_ago_price_change),
                one_year_ago_price_change: Ok(value.one_year_ago_price_change),
                one_year_price_change: Ok(value.one_year_price_change),
                popularity_uniques: Ok(value.popularity_uniques),
                popularity_uniques14_day: Ok(value.popularity_uniques14_day),
                popularity_uniques30_day: Ok(value.popularity_uniques30_day),
                popularity_uniques7_day: Ok(value.popularity_uniques7_day),
                sector_name_id: Ok(value.sector_name_id),
                sell_holding_pct: Ok(value.sell_holding_pct),
                six_month_price_change: Ok(value.six_month_price_change),
                six_months_ago_price_change: Ok(value.six_months_ago_price_change),
                symbol: Ok(value.symbol),
                three_month_price_change: Ok(value.three_month_price_change),
                three_months_ago_price_change: Ok(value.three_months_ago_price_change),
                traders14_day_change: Ok(value.traders14_day_change),
                traders30_day_change: Ok(value.traders30_day_change),
                traders7_day_change: Ok(value.traders7_day_change),
                two_months_ago_price_change: Ok(value.two_months_ago_price_change),
                weekly_price_change: Ok(value.weekly_price_change),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InstrumentSearchResponse {
        items: ::std::result::Result<::std::vec::Vec<super::Instrument>, ::std::string::String>,
        page: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        page_size: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        total_items: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for InstrumentSearchResponse {
        fn default() -> Self {
            Self {
                items: Ok(Default::default()),
                page: Ok(Default::default()),
                page_size: Ok(Default::default()),
                total_items: Ok(Default::default()),
            }
        }
    }
    impl InstrumentSearchResponse {
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Instrument>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {e}"));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {e}"));
            self
        }
        pub fn page_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.page_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page_size: {e}"));
            self
        }
        pub fn total_items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_items: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InstrumentSearchResponse> for super::InstrumentSearchResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InstrumentSearchResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                items: value.items?,
                page: value.page?,
                page_size: value.page_size?,
                total_items: value.total_items?,
            })
        }
    }
    impl ::std::convert::From<super::InstrumentSearchResponse> for InstrumentSearchResponse {
        fn from(value: super::InstrumentSearchResponse) -> Self {
            Self {
                items: Ok(value.items),
                page: Ok(value.page),
                page_size: Ok(value.page_size),
                total_items: Ok(value.total_items),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InstrumentTypesResponse {
        instrument_types: ::std::result::Result<
            ::std::vec::Vec<super::InstrumentTypesResponseInstrumentTypesItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for InstrumentTypesResponse {
        fn default() -> Self {
            Self {
                instrument_types: Ok(Default::default()),
            }
        }
    }
    impl InstrumentTypesResponse {
        pub fn instrument_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::InstrumentTypesResponseInstrumentTypesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instrument_types: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InstrumentTypesResponse> for super::InstrumentTypesResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InstrumentTypesResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                instrument_types: value.instrument_types?,
            })
        }
    }
    impl ::std::convert::From<super::InstrumentTypesResponse> for InstrumentTypesResponse {
        fn from(value: super::InstrumentTypesResponse) -> Self {
            Self {
                instrument_types: Ok(value.instrument_types),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InstrumentTypesResponseInstrumentTypesItem {
        instrument_type_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        instrument_type_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for InstrumentTypesResponseInstrumentTypesItem {
        fn default() -> Self {
            Self {
                instrument_type_description: Ok(Default::default()),
                instrument_type_id: Ok(Default::default()),
            }
        }
    }
    impl InstrumentTypesResponseInstrumentTypesItem {
        pub fn instrument_type_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_type_description = value.try_into().map_err(|e| {
                format!("error converting supplied value for instrument_type_description: {e}")
            });
            self
        }
        pub fn instrument_type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_type_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for instrument_type_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<InstrumentTypesResponseInstrumentTypesItem>
        for super::InstrumentTypesResponseInstrumentTypesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InstrumentTypesResponseInstrumentTypesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                instrument_type_description: value.instrument_type_description?,
                instrument_type_id: value.instrument_type_id?,
            })
        }
    }
    impl ::std::convert::From<super::InstrumentTypesResponseInstrumentTypesItem>
        for InstrumentTypesResponseInstrumentTypesItem
    {
        fn from(value: super::InstrumentTypesResponseInstrumentTypesItem) -> Self {
            Self {
                instrument_type_description: Ok(value.instrument_type_description),
                instrument_type_id: Ok(value.instrument_type_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InstrumentsResponse {
        instrument_display_datas: ::std::result::Result<
            ::std::vec::Vec<super::InstrumentsResponseInstrumentDisplayDatasItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for InstrumentsResponse {
        fn default() -> Self {
            Self {
                instrument_display_datas: Ok(Default::default()),
            }
        }
    }
    impl InstrumentsResponse {
        pub fn instrument_display_datas<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::InstrumentsResponseInstrumentDisplayDatasItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_display_datas = value.try_into().map_err(|e| {
                format!("error converting supplied value for instrument_display_datas: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<InstrumentsResponse> for super::InstrumentsResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InstrumentsResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                instrument_display_datas: value.instrument_display_datas?,
            })
        }
    }
    impl ::std::convert::From<super::InstrumentsResponse> for InstrumentsResponse {
        fn from(value: super::InstrumentsResponse) -> Self {
            Self {
                instrument_display_datas: Ok(value.instrument_display_datas),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InstrumentsResponseInstrumentDisplayDatasItem {
        exchange_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        has_expiration_date:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        images: ::std::result::Result<
            ::std::vec::Vec<super::InstrumentsResponseInstrumentDisplayDatasItemImagesItem>,
            ::std::string::String,
        >,
        instrument_display_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        instrument_type_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        is_internal_instrument:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        price_source: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        stocks_industry_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        symbol_full: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for InstrumentsResponseInstrumentDisplayDatasItem {
        fn default() -> Self {
            Self {
                exchange_id: Ok(Default::default()),
                has_expiration_date: Ok(Default::default()),
                images: Ok(Default::default()),
                instrument_display_name: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                instrument_type_id: Ok(Default::default()),
                is_internal_instrument: Ok(Default::default()),
                price_source: Ok(Default::default()),
                stocks_industry_id: Ok(Default::default()),
                symbol_full: Ok(Default::default()),
            }
        }
    }
    impl InstrumentsResponseInstrumentDisplayDatasItem {
        pub fn exchange_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.exchange_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exchange_id: {e}"));
            self
        }
        pub fn has_expiration_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.has_expiration_date = value.try_into().map_err(|e| {
                format!("error converting supplied value for has_expiration_date: {e}")
            });
            self
        }
        pub fn images<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::InstrumentsResponseInstrumentDisplayDatasItemImagesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.images = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for images: {e}"));
            self
        }
        pub fn instrument_display_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_display_name = value.try_into().map_err(|e| {
                format!("error converting supplied value for instrument_display_name: {e}")
            });
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
        pub fn instrument_type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.instrument_type_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for instrument_type_id: {e}")
            });
            self
        }
        pub fn is_internal_instrument<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_internal_instrument = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_internal_instrument: {e}")
            });
            self
        }
        pub fn price_source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.price_source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for price_source: {e}"));
            self
        }
        pub fn stocks_industry_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.stocks_industry_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for stocks_industry_id: {e}")
            });
            self
        }
        pub fn symbol_full<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.symbol_full = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbol_full: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InstrumentsResponseInstrumentDisplayDatasItem>
        for super::InstrumentsResponseInstrumentDisplayDatasItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InstrumentsResponseInstrumentDisplayDatasItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                exchange_id: value.exchange_id?,
                has_expiration_date: value.has_expiration_date?,
                images: value.images?,
                instrument_display_name: value.instrument_display_name?,
                instrument_id: value.instrument_id?,
                instrument_type_id: value.instrument_type_id?,
                is_internal_instrument: value.is_internal_instrument?,
                price_source: value.price_source?,
                stocks_industry_id: value.stocks_industry_id?,
                symbol_full: value.symbol_full?,
            })
        }
    }
    impl ::std::convert::From<super::InstrumentsResponseInstrumentDisplayDatasItem>
        for InstrumentsResponseInstrumentDisplayDatasItem
    {
        fn from(value: super::InstrumentsResponseInstrumentDisplayDatasItem) -> Self {
            Self {
                exchange_id: Ok(value.exchange_id),
                has_expiration_date: Ok(value.has_expiration_date),
                images: Ok(value.images),
                instrument_display_name: Ok(value.instrument_display_name),
                instrument_id: Ok(value.instrument_id),
                instrument_type_id: Ok(value.instrument_type_id),
                is_internal_instrument: Ok(value.is_internal_instrument),
                price_source: Ok(value.price_source),
                stocks_industry_id: Ok(value.stocks_industry_id),
                symbol_full: Ok(value.symbol_full),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InstrumentsResponseInstrumentDisplayDatasItemImagesItem {
        background_color: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        height: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        text_color: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uri: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        width: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for InstrumentsResponseInstrumentDisplayDatasItemImagesItem {
        fn default() -> Self {
            Self {
                background_color: Ok(Default::default()),
                height: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                text_color: Ok(Default::default()),
                uri: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl InstrumentsResponseInstrumentDisplayDatasItemImagesItem {
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
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height: {e}"));
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
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
            self
        }
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for width: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InstrumentsResponseInstrumentDisplayDatasItemImagesItem>
        for super::InstrumentsResponseInstrumentDisplayDatasItemImagesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InstrumentsResponseInstrumentDisplayDatasItemImagesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                background_color: value.background_color?,
                height: value.height?,
                instrument_id: value.instrument_id?,
                text_color: value.text_color?,
                uri: value.uri?,
                width: value.width?,
            })
        }
    }
    impl ::std::convert::From<super::InstrumentsResponseInstrumentDisplayDatasItemImagesItem>
        for InstrumentsResponseInstrumentDisplayDatasItemImagesItem
    {
        fn from(value: super::InstrumentsResponseInstrumentDisplayDatasItemImagesItem) -> Self {
            Self {
                background_color: Ok(value.background_color),
                height: Ok(value.height),
                instrument_id: Ok(value.instrument_id),
                text_color: Ok(value.text_color),
                uri: Ok(value.uri),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveRatesResponse {
        rates: ::std::result::Result<
            ::std::vec::Vec<super::LiveRatesResponseRatesItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveRatesResponse {
        fn default() -> Self {
            Self {
                rates: Ok(Default::default()),
            }
        }
    }
    impl LiveRatesResponse {
        pub fn rates<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::LiveRatesResponseRatesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.rates = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rates: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveRatesResponse> for super::LiveRatesResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveRatesResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                rates: value.rates?,
            })
        }
    }
    impl ::std::convert::From<super::LiveRatesResponse> for LiveRatesResponse {
        fn from(value: super::LiveRatesResponse) -> Self {
            Self {
                rates: Ok(value.rates),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveRatesResponseRatesItem {
        ask: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        bid: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        conversion_rate_ask:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        conversion_rate_bid:
            ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        last_execution: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
        price_rate_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for LiveRatesResponseRatesItem {
        fn default() -> Self {
            Self {
                ask: Ok(Default::default()),
                bid: Ok(Default::default()),
                conversion_rate_ask: Ok(Default::default()),
                conversion_rate_bid: Ok(Default::default()),
                date: Ok(Default::default()),
                instrument_id: Ok(Default::default()),
                last_execution: Ok(Default::default()),
                price_rate_id: Ok(Default::default()),
            }
        }
    }
    impl LiveRatesResponseRatesItem {
        pub fn ask<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.ask = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ask: {e}"));
            self
        }
        pub fn bid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.bid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bid: {e}"));
            self
        }
        pub fn conversion_rate_ask<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.conversion_rate_ask = value.try_into().map_err(|e| {
                format!("error converting supplied value for conversion_rate_ask: {e}")
            });
            self
        }
        pub fn conversion_rate_bid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.conversion_rate_bid = value.try_into().map_err(|e| {
                format!("error converting supplied value for conversion_rate_bid: {e}")
            });
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {e}"));
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
        pub fn last_execution<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_execution = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_execution: {e}"));
            self
        }
        pub fn price_rate_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.price_rate_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for price_rate_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveRatesResponseRatesItem> for super::LiveRatesResponseRatesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveRatesResponseRatesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ask: value.ask?,
                bid: value.bid?,
                conversion_rate_ask: value.conversion_rate_ask?,
                conversion_rate_bid: value.conversion_rate_bid?,
                date: value.date?,
                instrument_id: value.instrument_id?,
                last_execution: value.last_execution?,
                price_rate_id: value.price_rate_id?,
            })
        }
    }
    impl ::std::convert::From<super::LiveRatesResponseRatesItem> for LiveRatesResponseRatesItem {
        fn from(value: super::LiveRatesResponseRatesItem) -> Self {
            Self {
                ask: Ok(value.ask),
                bid: Ok(value.bid),
                conversion_rate_ask: Ok(value.conversion_rate_ask),
                conversion_rate_bid: Ok(value.conversion_rate_bid),
                date: Ok(value.date),
                instrument_id: Ok(value.instrument_id),
                last_execution: Ok(value.last_execution),
                price_rate_id: Ok(value.price_rate_id),
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
    pub struct MarketEventMetadata {
        earning_report_id: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        earnings_date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        earnings_quarter: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        earnings_year: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        estimated_eps: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        estimated_sales: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        is_before_market_open:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        market: ::std::result::Result<::std::option::Option<super::Market>, ::std::string::String>,
        market_cap: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        stocks_industry_id:
            ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        tag_name: ::std::result::Result<
            ::std::option::Option<super::MarketEventTag>,
            ::std::string::String,
        >,
        text_key: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        verified: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for MarketEventMetadata {
        fn default() -> Self {
            Self {
                earning_report_id: Ok(Default::default()),
                earnings_date: Ok(Default::default()),
                earnings_quarter: Ok(Default::default()),
                earnings_year: Ok(Default::default()),
                estimated_eps: Ok(Default::default()),
                estimated_sales: Ok(Default::default()),
                is_before_market_open: Ok(Default::default()),
                market: Ok(Default::default()),
                market_cap: Ok(Default::default()),
                stocks_industry_id: Ok(Default::default()),
                tag_name: Ok(Default::default()),
                text_key: Ok(Default::default()),
                verified: Ok(Default::default()),
            }
        }
    }
    impl MarketEventMetadata {
        pub fn earning_report_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.earning_report_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for earning_report_id: {e}"));
            self
        }
        pub fn earnings_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.earnings_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for earnings_date: {e}"));
            self
        }
        pub fn earnings_quarter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.earnings_quarter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for earnings_quarter: {e}"));
            self
        }
        pub fn earnings_year<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.earnings_year = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for earnings_year: {e}"));
            self
        }
        pub fn estimated_eps<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.estimated_eps = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for estimated_eps: {e}"));
            self
        }
        pub fn estimated_sales<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.estimated_sales = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for estimated_sales: {e}"));
            self
        }
        pub fn is_before_market_open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_before_market_open = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_before_market_open: {e}")
            });
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
        pub fn market_cap<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.market_cap = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for market_cap: {e}"));
            self
        }
        pub fn stocks_industry_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.stocks_industry_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for stocks_industry_id: {e}")
            });
            self
        }
        pub fn tag_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MarketEventTag>>,
            T::Error: ::std::fmt::Display,
        {
            self.tag_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tag_name: {e}"));
            self
        }
        pub fn text_key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.text_key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_key: {e}"));
            self
        }
        pub fn verified<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.verified = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for verified: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MarketEventMetadata> for super::MarketEventMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MarketEventMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                earning_report_id: value.earning_report_id?,
                earnings_date: value.earnings_date?,
                earnings_quarter: value.earnings_quarter?,
                earnings_year: value.earnings_year?,
                estimated_eps: value.estimated_eps?,
                estimated_sales: value.estimated_sales?,
                is_before_market_open: value.is_before_market_open?,
                market: value.market?,
                market_cap: value.market_cap?,
                stocks_industry_id: value.stocks_industry_id?,
                tag_name: value.tag_name?,
                text_key: value.text_key?,
                verified: value.verified?,
            })
        }
    }
    impl ::std::convert::From<super::MarketEventMetadata> for MarketEventMetadata {
        fn from(value: super::MarketEventMetadata) -> Self {
            Self {
                earning_report_id: Ok(value.earning_report_id),
                earnings_date: Ok(value.earnings_date),
                earnings_quarter: Ok(value.earnings_quarter),
                earnings_year: Ok(value.earnings_year),
                estimated_eps: Ok(value.estimated_eps),
                estimated_sales: Ok(value.estimated_sales),
                is_before_market_open: Ok(value.is_before_market_open),
                market: Ok(value.market),
                market_cap: Ok(value.market_cap),
                stocks_industry_id: Ok(value.stocks_industry_id),
                tag_name: Ok(value.tag_name),
                text_key: Ok(value.text_key),
                verified: Ok(value.verified),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MarketRecommendationsResponse {
        recommendations: ::std::result::Result<::std::vec::Vec<i32>, ::std::string::String>,
        response_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MarketRecommendationsResponse {
        fn default() -> Self {
            Self {
                recommendations: Ok(Default::default()),
                response_type: Ok(Default::default()),
            }
        }
    }
    impl MarketRecommendationsResponse {
        pub fn recommendations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.recommendations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recommendations: {e}"));
            self
        }
        pub fn response_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.response_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MarketRecommendationsResponse>
        for super::MarketRecommendationsResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MarketRecommendationsResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                recommendations: value.recommendations?,
                response_type: value.response_type?,
            })
        }
    }
    impl ::std::convert::From<super::MarketRecommendationsResponse> for MarketRecommendationsResponse {
        fn from(value: super::MarketRecommendationsResponse) -> Self {
            Self {
                recommendations: Ok(value.recommendations),
                response_type: Ok(value.response_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StocksIndustriesResponse {
        stocks_industries: ::std::result::Result<
            ::std::vec::Vec<super::StocksIndustriesResponseStocksIndustriesItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StocksIndustriesResponse {
        fn default() -> Self {
            Self {
                stocks_industries: Ok(Default::default()),
            }
        }
    }
    impl StocksIndustriesResponse {
        pub fn stocks_industries<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::StocksIndustriesResponseStocksIndustriesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.stocks_industries = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stocks_industries: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StocksIndustriesResponse> for super::StocksIndustriesResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StocksIndustriesResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                stocks_industries: value.stocks_industries?,
            })
        }
    }
    impl ::std::convert::From<super::StocksIndustriesResponse> for StocksIndustriesResponse {
        fn from(value: super::StocksIndustriesResponse) -> Self {
            Self {
                stocks_industries: Ok(value.stocks_industries),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StocksIndustriesResponseStocksIndustriesItem {
        industry_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        industry_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StocksIndustriesResponseStocksIndustriesItem {
        fn default() -> Self {
            Self {
                industry_id: Ok(Default::default()),
                industry_name: Ok(Default::default()),
            }
        }
    }
    impl StocksIndustriesResponseStocksIndustriesItem {
        pub fn industry_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.industry_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for industry_id: {e}"));
            self
        }
        pub fn industry_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.industry_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for industry_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StocksIndustriesResponseStocksIndustriesItem>
        for super::StocksIndustriesResponseStocksIndustriesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StocksIndustriesResponseStocksIndustriesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                industry_id: value.industry_id?,
                industry_name: value.industry_name?,
            })
        }
    }
    impl ::std::convert::From<super::StocksIndustriesResponseStocksIndustriesItem>
        for StocksIndustriesResponseStocksIndustriesItem
    {
        fn from(value: super::StocksIndustriesResponseStocksIndustriesItem) -> Self {
            Self {
                industry_id: Ok(value.industry_id),
                industry_name: Ok(value.industry_name),
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
}
