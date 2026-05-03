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
#[doc = "                  \"format\": \"float\","]
#[doc = "                  \"x-rust-type\": {"]
#[doc = "                    \"crate\": \"etoro-agent\","]
#[doc = "                    \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                    \"version\": \"0.1.0\""]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                \"fromDate\": {"]
#[doc = "                  \"type\": \"string\","]
#[doc = "                  \"format\": \"date-time\""]
#[doc = "                },"]
#[doc = "                \"high\": {"]
#[doc = "                  \"type\": \"number\","]
#[doc = "                  \"format\": \"float\","]
#[doc = "                  \"x-rust-type\": {"]
#[doc = "                    \"crate\": \"etoro-agent\","]
#[doc = "                    \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                    \"version\": \"0.1.0\""]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                \"instrumentID\": {"]
#[doc = "                  \"description\": \"Capital ID — inconsistent with outer's instrumentId\","]
#[doc = "                  \"type\": \"integer\""]
#[doc = "                },"]
#[doc = "                \"low\": {"]
#[doc = "                  \"type\": \"number\","]
#[doc = "                  \"format\": \"float\","]
#[doc = "                  \"x-rust-type\": {"]
#[doc = "                    \"crate\": \"etoro-agent\","]
#[doc = "                    \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                    \"version\": \"0.1.0\""]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                \"open\": {"]
#[doc = "                  \"type\": \"number\","]
#[doc = "                  \"format\": \"float\","]
#[doc = "                  \"x-rust-type\": {"]
#[doc = "                    \"crate\": \"etoro-agent\","]
#[doc = "                    \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                    \"version\": \"0.1.0\""]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                \"volume\": {"]
#[doc = "                  \"type\": \"number\","]
#[doc = "                  \"format\": \"float\","]
#[doc = "                  \"x-rust-type\": {"]
#[doc = "                    \"crate\": \"etoro-agent\","]
#[doc = "                    \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                    \"version\": \"0.1.0\""]
#[doc = "                  }"]
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
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"rangeHigh\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"rangeLow\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"rangeOpen\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"volume\": {"]
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
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"fromDate\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"date-time\""]
#[doc = "          },"]
#[doc = "          \"high\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"instrumentID\": {"]
#[doc = "            \"description\": \"Capital ID — inconsistent with outer's instrumentId\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"low\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"open\": {"]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"volume\": {"]
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
#[doc = "    \"instrumentId\": {"]
#[doc = "      \"description\": \"Lowercase 'd'\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"rangeClose\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"rangeHigh\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"rangeLow\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"rangeOpen\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"volume\": {"]
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
    pub range_close: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "rangeHigh",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub range_high: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "rangeLow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub range_low: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "rangeOpen",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub range_open: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub volume: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fromDate\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"high\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"instrumentID\": {"]
#[doc = "      \"description\": \"Capital ID — inconsistent with outer's instrumentId\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"low\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"open\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"volume\": {"]
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
pub struct CandlesResponseCandlesItemCandlesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub close: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "fromDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub from_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub high: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "Capital ID — inconsistent with outer's instrumentId"]
    #[serde(
        rename = "instrumentID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instrument_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub low: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub open: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub volume: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
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
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
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
#[doc = "                \"format\": \"float\","]
#[doc = "                \"x-rust-type\": {"]
#[doc = "                  \"crate\": \"etoro-agent\","]
#[doc = "                  \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                  \"version\": \"0.1.0\""]
#[doc = "                }"]
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
#[doc = "        \"format\": \"float\","]
#[doc = "        \"x-rust-type\": {"]
#[doc = "          \"crate\": \"etoro-agent\","]
#[doc = "          \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "          \"version\": \"0.1.0\""]
#[doc = "        }"]
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
#[doc = "              \"format\": \"float\","]
#[doc = "              \"x-rust-type\": {"]
#[doc = "                \"crate\": \"etoro-agent\","]
#[doc = "                \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                \"version\": \"0.1.0\""]
#[doc = "              }"]
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
#[doc = "              \"format\": \"float\","]
#[doc = "              \"x-rust-type\": {"]
#[doc = "                \"crate\": \"etoro-agent\","]
#[doc = "                \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                \"version\": \"0.1.0\""]
#[doc = "              }"]
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
#[doc = "              \"format\": \"float\","]
#[doc = "              \"x-rust-type\": {"]
#[doc = "                \"crate\": \"etoro-agent\","]
#[doc = "                \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                \"version\": \"0.1.0\""]
#[doc = "              }"]
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
    pub official_closing_price: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
#[doc = "          \"format\": \"float\","]
#[doc = "          \"x-rust-type\": {"]
#[doc = "            \"crate\": \"etoro-agent\","]
#[doc = "            \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "            \"version\": \"0.1.0\""]
#[doc = "          }"]
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
#[doc = "          \"format\": \"float\","]
#[doc = "          \"x-rust-type\": {"]
#[doc = "            \"crate\": \"etoro-agent\","]
#[doc = "            \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "            \"version\": \"0.1.0\""]
#[doc = "          }"]
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
#[doc = "          \"format\": \"float\","]
#[doc = "          \"x-rust-type\": {"]
#[doc = "            \"crate\": \"etoro-agent\","]
#[doc = "            \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "            \"version\": \"0.1.0\""]
#[doc = "          }"]
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
pub struct ClosingPricesResponseItemClosingPricesDaily {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub price: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
}
impl ::std::default::Default for ClosingPricesResponseItemClosingPricesDaily {
    fn default() -> Self {
        Self {
            date: Default::default(),
            price: Default::default(),
        }
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
pub struct ClosingPricesResponseItemClosingPricesMonthly {
    #[doc = "0001-01-01 indicates no data available"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "-1 indicates no data available"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub price: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
}
impl ::std::default::Default for ClosingPricesResponseItemClosingPricesMonthly {
    fn default() -> Self {
        Self {
            date: Default::default(),
            price: Default::default(),
        }
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
pub struct ClosingPricesResponseItemClosingPricesWeekly {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub price: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
}
impl ::std::default::Default for ClosingPricesResponseItemClosingPricesWeekly {
    fn default() -> Self {
        Self {
            date: Default::default(),
            price: Default::default(),
        }
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
#[doc = "`Instrument`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"absBuyPctChange24Hours\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"absDailyPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"buyHoldingPct\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"buyPctChange24Hours\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"currMonthPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"currQuarterPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"currYearPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"currentRate\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"cvtAsk\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"cvtAskNoSpread\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"cvtBiNoSpread\": {"]
#[doc = "      \"description\": \"Spec spelling — likely typo for 'cvtBidNoSpread'\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"cvtBid\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"dailyPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"displayname\": {"]
#[doc = "      \"description\": \"The display name of the instrument.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"exchangeID\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"holdingPct\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"lastYearPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"oneMonthAgoPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"oneYearAgoPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"oneYearPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"sixMonthPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"sixMonthsAgoPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"symbol\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"threeMonthPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"threeMonthsAgoPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"traders14DayChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"traders30DayChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"traders7DayChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"twoMonthsAgoPriceChange\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"weeklyPriceChange\": {"]
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
pub struct Instrument {
    #[serde(
        rename = "absBuyPctChange24Hours",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub abs_buy_pct_change24_hours: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "absDailyPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub abs_daily_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "buyHoldingPct",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub buy_holding_pct: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "buyPctChange24Hours",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub buy_pct_change24_hours: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "currMonthPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub curr_month_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "currQuarterPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub curr_quarter_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "currYearPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub curr_year_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "currentRate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub current_rate: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "cvtAsk",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvt_ask: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "cvtAskNoSpread",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvt_ask_no_spread: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "Spec spelling — likely typo for 'cvtBidNoSpread'"]
    #[serde(
        rename = "cvtBiNoSpread",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvt_bi_no_spread: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "cvtBid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvt_bid: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "dailyPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub daily_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub holding_pct: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub internal_closing_price: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub last_two_years_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "lastYearPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_year_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub monthly_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "oneMonthAgoPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub one_month_ago_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "oneYearAgoPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub one_year_ago_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "oneYearPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub one_year_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub sell_holding_pct: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "sixMonthPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub six_month_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "sixMonthsAgoPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub six_months_ago_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub symbol: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "threeMonthPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub three_month_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "threeMonthsAgoPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub three_months_ago_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "traders14DayChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traders14_day_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "traders30DayChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traders30_day_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "traders7DayChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traders7_day_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "twoMonthsAgoPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub two_months_ago_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "weeklyPriceChange",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub weekly_price_change: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
#[doc = "                  \"type\": \"number\","]
#[doc = "                  \"x-rust-type\": {"]
#[doc = "                    \"crate\": \"etoro-agent\","]
#[doc = "                    \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                    \"version\": \"0.1.0\""]
#[doc = "                  }"]
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
#[doc = "                  \"type\": \"number\","]
#[doc = "                  \"x-rust-type\": {"]
#[doc = "                    \"crate\": \"etoro-agent\","]
#[doc = "                    \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "                    \"version\": \"0.1.0\""]
#[doc = "                  }"]
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
#[doc = "            \"type\": \"number\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
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
#[doc = "            \"type\": \"number\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
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
#[doc = "      \"type\": \"number\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
pub struct InstrumentsResponseInstrumentDisplayDatasItemImagesItem {
    #[serde(
        rename = "backgroundColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub background_color: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub width: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"bid\": {"]
#[doc = "            \"description\": \"Sell price\","]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"conversionRateAsk\": {"]
#[doc = "            \"description\": \"Currency → USD conversion (ask)\","]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"conversionRateBid\": {"]
#[doc = "            \"description\": \"Currency → USD conversion (bid)\","]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
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
#[doc = "            \"format\": \"float\","]
#[doc = "            \"x-rust-type\": {"]
#[doc = "              \"crate\": \"etoro-agent\","]
#[doc = "              \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "              \"version\": \"0.1.0\""]
#[doc = "            }"]
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
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"bid\": {"]
#[doc = "      \"description\": \"Sell price\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"conversionRateAsk\": {"]
#[doc = "      \"description\": \"Currency → USD conversion (ask)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"conversionRateBid\": {"]
#[doc = "      \"description\": \"Currency → USD conversion (bid)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
#[doc = "      \"format\": \"float\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
    pub ask: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "Sell price"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bid: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "Currency → USD conversion (ask)"]
    #[serde(
        rename = "conversionRateAsk",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub conversion_rate_ask: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[doc = "Currency → USD conversion (bid)"]
    #[serde(
        rename = "conversionRateBid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub conversion_rate_bid: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub last_execution: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub application: ::std::option::Option<::etoro_agent::types::manual::ApplicationSource>,
    #[serde(
        rename = "assetType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub asset_type: ::std::option::Option<::etoro_agent::types::manual::MarketAssetType>,
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
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"estimatedSales\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"isBeforeMarketOpen\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"market\": {"]
#[doc = "      \"$ref\": \"#/$defs/Market\""]
#[doc = "    },"]
#[doc = "    \"marketCap\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
    pub estimated_eps: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(
        rename = "estimatedSales",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub estimated_sales: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub market_cap: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
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
    pub tag_name: ::std::option::Option<::etoro_agent::types::manual::MarketEventTag>,
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
