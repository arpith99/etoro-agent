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
#[doc = "Represents an item in a watchlist"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents an item in a watchlist\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"itemId\","]
#[doc = "    \"itemType\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"itemAddedDate\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"itemAddedReason\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Manual\""]
#[doc = "    },"]
#[doc = "    \"itemId\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\","]
#[doc = "      \"example\": 12345"]
#[doc = "    },"]
#[doc = "    \"itemRank\": {"]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\","]
#[doc = "      \"example\": 1"]
#[doc = "    },"]
#[doc = "    \"itemType\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Instrument\""]
#[doc = "    },"]
#[doc = "    \"market\": {"]
#[doc = "      \"description\": \"Market metadata for the instrument when included\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"assetTypeId\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"int32\""]
#[doc = "        },"]
#[doc = "        \"assetTypeSubCategoryId\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"integer\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"format\": \"int32\""]
#[doc = "        },"]
#[doc = "        \"avatar\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"large\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"medium\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"small\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"svg\": {"]
#[doc = "              \"type\": ["]
#[doc = "                \"object\","]
#[doc = "                \"null\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"backgroundColor\": {"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"textColor\": {"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"url\": {"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"displayName\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"exchangeId\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"int32\""]
#[doc = "        },"]
#[doc = "        \"hasExpirationDate\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"symbolName\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WatchlistItemDto {
    #[serde(
        rename = "itemAddedDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub item_added_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "itemAddedReason",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub item_added_reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "itemRank", default)]
    pub item_rank: i32,
    #[serde(rename = "itemType")]
    pub item_type: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub market: ::std::option::Option<WatchlistItemDtoMarket>,
}
#[doc = "Market metadata for the instrument when included"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Market metadata for the instrument when included\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"assetTypeId\": {"]
#[doc = "      \"type\": \"integer\","]
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
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"large\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"medium\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"small\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"svg\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"object\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"backgroundColor\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"textColor\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"url\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"displayName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"exchangeId\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"hasExpirationDate\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"symbolName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WatchlistItemDtoMarket {
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
    pub avatar: ::std::option::Option<WatchlistItemDtoMarketAvatar>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "exchangeId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exchange_id: ::std::option::Option<i32>,
    #[serde(
        rename = "hasExpirationDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_expiration_date: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "symbolName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub symbol_name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for WatchlistItemDtoMarket {
    fn default() -> Self {
        Self {
            asset_type_id: Default::default(),
            asset_type_sub_category_id: Default::default(),
            avatar: Default::default(),
            display_name: Default::default(),
            exchange_id: Default::default(),
            has_expiration_date: Default::default(),
            id: Default::default(),
            symbol_name: Default::default(),
        }
    }
}
#[doc = "`WatchlistItemDtoMarketAvatar`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"large\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"medium\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"small\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"svg\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"backgroundColor\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"textColor\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WatchlistItemDtoMarketAvatar {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub large: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub medium: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub small: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub svg: ::std::option::Option<WatchlistItemDtoMarketAvatarSvg>,
}
impl ::std::default::Default for WatchlistItemDtoMarketAvatar {
    fn default() -> Self {
        Self {
            large: Default::default(),
            medium: Default::default(),
            small: Default::default(),
            svg: Default::default(),
        }
    }
}
#[doc = "`WatchlistItemDtoMarketAvatarSvg`"]
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
#[doc = "    \"textColor\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WatchlistItemDtoMarketAvatarSvg {
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
impl ::std::default::Default for WatchlistItemDtoMarketAvatarSvg {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            text_color: Default::default(),
            url: Default::default(),
        }
    }
}
#[doc = "Represents a watchlist with its metadata and items"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a watchlist with its metadata and items\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"Gcid\": {"]
#[doc = "      \"description\": \"Global Customer ID of the watchlist owner\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\","]
#[doc = "      \"example\": 12345"]
#[doc = "    },"]
#[doc = "    \"dynamicUrl\": {"]
#[doc = "      \"description\": \"URL for dynamic watchlist queries\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"isDefault\": {"]
#[doc = "      \"type\": \"boolean\","]
#[doc = "      \"example\": true"]
#[doc = "    },"]
#[doc = "    \"isUserSelectedDefault\": {"]
#[doc = "      \"type\": \"boolean\","]
#[doc = "      \"example\": true"]
#[doc = "    },"]
#[doc = "    \"items\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/WatchlistItemDto\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Tech Watchlist\""]
#[doc = "    },"]
#[doc = "    \"relatedAssets\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\","]
#[doc = "        \"format\": \"int32\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"totalItems\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\","]
#[doc = "      \"example\": 100"]
#[doc = "    },"]
#[doc = "    \"watchlistId\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"12345\""]
#[doc = "    },"]
#[doc = "    \"watchlistRank\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\","]
#[doc = "      \"example\": 1"]
#[doc = "    },"]
#[doc = "    \"watchlistType\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Static\","]
#[doc = "        \"Dynamic\","]
#[doc = "        \"RecentlyInvested\","]
#[doc = "        \"Default\""]
#[doc = "      ],"]
#[doc = "      \"example\": \"Static\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WatchlistResponse {
    #[doc = "URL for dynamic watchlist queries"]
    #[serde(
        rename = "dynamicUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub dynamic_url: ::std::option::Option<::std::string::String>,
    #[doc = "Global Customer ID of the watchlist owner"]
    #[serde(
        rename = "Gcid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub gcid: ::std::option::Option<i32>,
    #[serde(
        rename = "isDefault",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_default: ::std::option::Option<bool>,
    #[serde(
        rename = "isUserSelectedDefault",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_user_selected_default: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub items: ::std::vec::Vec<WatchlistItemDto>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "relatedAssets",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub related_assets: ::std::option::Option<::std::vec::Vec<i32>>,
    #[serde(
        rename = "totalItems",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_items: ::std::option::Option<i32>,
    #[serde(
        rename = "watchlistId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub watchlist_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "watchlistRank",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub watchlist_rank: ::std::option::Option<i32>,
    #[serde(
        rename = "watchlistType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub watchlist_type: ::std::option::Option<WatchlistResponseWatchlistType>,
}
impl ::std::default::Default for WatchlistResponse {
    fn default() -> Self {
        Self {
            dynamic_url: Default::default(),
            gcid: Default::default(),
            is_default: Default::default(),
            is_user_selected_default: Default::default(),
            items: Default::default(),
            name: Default::default(),
            related_assets: Default::default(),
            total_items: Default::default(),
            watchlist_id: Default::default(),
            watchlist_rank: Default::default(),
            watchlist_type: Default::default(),
        }
    }
}
#[doc = "`WatchlistResponseWatchlistType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Static\","]
#[doc = "    \"Dynamic\","]
#[doc = "    \"RecentlyInvested\","]
#[doc = "    \"Default\""]
#[doc = "  ],"]
#[doc = "  \"example\": \"Static\""]
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
pub enum WatchlistResponseWatchlistType {
    Static,
    Dynamic,
    RecentlyInvested,
    Default,
}
impl ::std::fmt::Display for WatchlistResponseWatchlistType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Static => f.write_str("Static"),
            Self::Dynamic => f.write_str("Dynamic"),
            Self::RecentlyInvested => f.write_str("RecentlyInvested"),
            Self::Default => f.write_str("Default"),
        }
    }
}
impl ::std::str::FromStr for WatchlistResponseWatchlistType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Static" => Ok(Self::Static),
            "Dynamic" => Ok(Self::Dynamic),
            "RecentlyInvested" => Ok(Self::RecentlyInvested),
            "Default" => Ok(Self::Default),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WatchlistResponseWatchlistType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WatchlistResponseWatchlistType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WatchlistResponseWatchlistType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Response containing multiple watchlists with metadata"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Response containing multiple watchlists with metadata\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"exception\": {"]
#[doc = "      \"description\": \"Exception details when the request partially failed\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"invalidItems\": {"]
#[doc = "          \"description\": \"List of invalid item identifiers\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"message\": {"]
#[doc = "          \"description\": \"Human-readable exception message\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"reason\": {"]
#[doc = "          \"description\": \"Reason for the exception\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"isSucceeded\": {"]
#[doc = "      \"description\": \"Whether the request succeeded\","]
#[doc = "      \"type\": \"boolean\","]
#[doc = "      \"example\": true"]
#[doc = "    },"]
#[doc = "    \"meta\": {"]
#[doc = "      \"description\": \"Response metadata including pagination info\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"itemsPerPage\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"int32\","]
#[doc = "          \"example\": 100"]
#[doc = "        },"]
#[doc = "        \"maxItemsInWatchlistLimit\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"int32\","]
#[doc = "          \"example\": 1000"]
#[doc = "        },"]
#[doc = "        \"maxWatchlistsLimit\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"int32\","]
#[doc = "          \"example\": 10"]
#[doc = "        },"]
#[doc = "        \"pageNumber\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"format\": \"int32\","]
#[doc = "          \"example\": 0"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"HTTP status code of the response\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\","]
#[doc = "      \"example\": 200"]
#[doc = "    },"]
#[doc = "    \"watchlists\": {"]
#[doc = "      \"description\": \"List of user watchlists\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/WatchlistResponse\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WatchlistsResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub exception: ::std::option::Option<WatchlistsResponseException>,
    #[doc = "Whether the request succeeded"]
    #[serde(
        rename = "isSucceeded",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_succeeded: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub meta: ::std::option::Option<WatchlistsResponseMeta>,
    #[doc = "HTTP status code of the response"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<i32>,
    #[doc = "List of user watchlists"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub watchlists: ::std::vec::Vec<WatchlistResponse>,
}
impl ::std::default::Default for WatchlistsResponse {
    fn default() -> Self {
        Self {
            exception: Default::default(),
            is_succeeded: Default::default(),
            meta: Default::default(),
            status: Default::default(),
            watchlists: Default::default(),
        }
    }
}
#[doc = "Exception details when the request partially failed"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Exception details when the request partially failed\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"invalidItems\": {"]
#[doc = "      \"description\": \"List of invalid item identifiers\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Human-readable exception message\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"description\": \"Reason for the exception\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WatchlistsResponseException {
    #[doc = "List of invalid item identifiers"]
    #[serde(
        rename = "invalidItems",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub invalid_items: ::std::vec::Vec<::std::string::String>,
    #[doc = "Human-readable exception message"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    #[doc = "Reason for the exception"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for WatchlistsResponseException {
    fn default() -> Self {
        Self {
            invalid_items: Default::default(),
            message: Default::default(),
            reason: Default::default(),
        }
    }
}
#[doc = "Response metadata including pagination info"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Response metadata including pagination info\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"itemsPerPage\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\","]
#[doc = "      \"example\": 100"]
#[doc = "    },"]
#[doc = "    \"maxItemsInWatchlistLimit\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\","]
#[doc = "      \"example\": 1000"]
#[doc = "    },"]
#[doc = "    \"maxWatchlistsLimit\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\","]
#[doc = "      \"example\": 10"]
#[doc = "    },"]
#[doc = "    \"pageNumber\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\","]
#[doc = "      \"example\": 0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WatchlistsResponseMeta {
    #[serde(
        rename = "itemsPerPage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub items_per_page: ::std::option::Option<i32>,
    #[serde(
        rename = "maxItemsInWatchlistLimit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_items_in_watchlist_limit: ::std::option::Option<i32>,
    #[serde(
        rename = "maxWatchlistsLimit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_watchlists_limit: ::std::option::Option<i32>,
    #[serde(
        rename = "pageNumber",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub page_number: ::std::option::Option<i32>,
}
impl ::std::default::Default for WatchlistsResponseMeta {
    fn default() -> Self {
        Self {
            items_per_page: Default::default(),
            max_items_in_watchlist_limit: Default::default(),
            max_watchlists_limit: Default::default(),
            page_number: Default::default(),
        }
    }
}
