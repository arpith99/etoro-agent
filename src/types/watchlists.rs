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
impl WatchlistItemDto {
    pub fn builder() -> builder::WatchlistItemDto {
        Default::default()
    }
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
impl WatchlistItemDtoMarket {
    pub fn builder() -> builder::WatchlistItemDtoMarket {
        Default::default()
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
impl WatchlistItemDtoMarketAvatar {
    pub fn builder() -> builder::WatchlistItemDtoMarketAvatar {
        Default::default()
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
impl WatchlistItemDtoMarketAvatarSvg {
    pub fn builder() -> builder::WatchlistItemDtoMarketAvatarSvg {
        Default::default()
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
impl WatchlistResponse {
    pub fn builder() -> builder::WatchlistResponse {
        Default::default()
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
impl WatchlistsResponse {
    pub fn builder() -> builder::WatchlistsResponse {
        Default::default()
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
impl WatchlistsResponseException {
    pub fn builder() -> builder::WatchlistsResponseException {
        Default::default()
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
impl WatchlistsResponseMeta {
    pub fn builder() -> builder::WatchlistsResponseMeta {
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
    pub struct WatchlistItemDto {
        item_added_date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        item_added_reason: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        item_id: ::std::result::Result<i32, ::std::string::String>,
        item_rank: ::std::result::Result<i32, ::std::string::String>,
        item_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        market: ::std::result::Result<
            ::std::option::Option<super::WatchlistItemDtoMarket>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WatchlistItemDto {
        fn default() -> Self {
            Self {
                item_added_date: Ok(Default::default()),
                item_added_reason: Ok(Default::default()),
                item_id: Err("no value supplied for item_id".to_string()),
                item_rank: Ok(Default::default()),
                item_type: Err("no value supplied for item_type".to_string()),
                market: Ok(Default::default()),
            }
        }
    }
    impl WatchlistItemDto {
        pub fn item_added_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.item_added_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_added_date: {e}"));
            self
        }
        pub fn item_added_reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.item_added_reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_added_reason: {e}"));
            self
        }
        pub fn item_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i32>,
            T::Error: ::std::fmt::Display,
        {
            self.item_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_id: {e}"));
            self
        }
        pub fn item_rank<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i32>,
            T::Error: ::std::fmt::Display,
        {
            self.item_rank = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_rank: {e}"));
            self
        }
        pub fn item_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.item_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_type: {e}"));
            self
        }
        pub fn market<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WatchlistItemDtoMarket>>,
            T::Error: ::std::fmt::Display,
        {
            self.market = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for market: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WatchlistItemDto> for super::WatchlistItemDto {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WatchlistItemDto,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                item_added_date: value.item_added_date?,
                item_added_reason: value.item_added_reason?,
                item_id: value.item_id?,
                item_rank: value.item_rank?,
                item_type: value.item_type?,
                market: value.market?,
            })
        }
    }
    impl ::std::convert::From<super::WatchlistItemDto> for WatchlistItemDto {
        fn from(value: super::WatchlistItemDto) -> Self {
            Self {
                item_added_date: Ok(value.item_added_date),
                item_added_reason: Ok(value.item_added_reason),
                item_id: Ok(value.item_id),
                item_rank: Ok(value.item_rank),
                item_type: Ok(value.item_type),
                market: Ok(value.market),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WatchlistItemDtoMarket {
        asset_type_id: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        asset_type_sub_category_id:
            ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        avatar: ::std::result::Result<
            ::std::option::Option<super::WatchlistItemDtoMarketAvatar>,
            ::std::string::String,
        >,
        display_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        exchange_id: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        has_expiration_date:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        symbol_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WatchlistItemDtoMarket {
        fn default() -> Self {
            Self {
                asset_type_id: Ok(Default::default()),
                asset_type_sub_category_id: Ok(Default::default()),
                avatar: Ok(Default::default()),
                display_name: Ok(Default::default()),
                exchange_id: Ok(Default::default()),
                has_expiration_date: Ok(Default::default()),
                id: Ok(Default::default()),
                symbol_name: Ok(Default::default()),
            }
        }
    }
    impl WatchlistItemDtoMarket {
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
            T: ::std::convert::TryInto<::std::option::Option<super::WatchlistItemDtoMarketAvatar>>,
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
        pub fn exchange_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
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
    }
    impl ::std::convert::TryFrom<WatchlistItemDtoMarket> for super::WatchlistItemDtoMarket {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WatchlistItemDtoMarket,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                asset_type_id: value.asset_type_id?,
                asset_type_sub_category_id: value.asset_type_sub_category_id?,
                avatar: value.avatar?,
                display_name: value.display_name?,
                exchange_id: value.exchange_id?,
                has_expiration_date: value.has_expiration_date?,
                id: value.id?,
                symbol_name: value.symbol_name?,
            })
        }
    }
    impl ::std::convert::From<super::WatchlistItemDtoMarket> for WatchlistItemDtoMarket {
        fn from(value: super::WatchlistItemDtoMarket) -> Self {
            Self {
                asset_type_id: Ok(value.asset_type_id),
                asset_type_sub_category_id: Ok(value.asset_type_sub_category_id),
                avatar: Ok(value.avatar),
                display_name: Ok(value.display_name),
                exchange_id: Ok(value.exchange_id),
                has_expiration_date: Ok(value.has_expiration_date),
                id: Ok(value.id),
                symbol_name: Ok(value.symbol_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WatchlistItemDtoMarketAvatar {
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
        svg: ::std::result::Result<
            ::std::option::Option<super::WatchlistItemDtoMarketAvatarSvg>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WatchlistItemDtoMarketAvatar {
        fn default() -> Self {
            Self {
                large: Ok(Default::default()),
                medium: Ok(Default::default()),
                small: Ok(Default::default()),
                svg: Ok(Default::default()),
            }
        }
    }
    impl WatchlistItemDtoMarketAvatar {
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::WatchlistItemDtoMarketAvatarSvg>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.svg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for svg: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WatchlistItemDtoMarketAvatar> for super::WatchlistItemDtoMarketAvatar {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WatchlistItemDtoMarketAvatar,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                large: value.large?,
                medium: value.medium?,
                small: value.small?,
                svg: value.svg?,
            })
        }
    }
    impl ::std::convert::From<super::WatchlistItemDtoMarketAvatar> for WatchlistItemDtoMarketAvatar {
        fn from(value: super::WatchlistItemDtoMarketAvatar) -> Self {
            Self {
                large: Ok(value.large),
                medium: Ok(value.medium),
                small: Ok(value.small),
                svg: Ok(value.svg),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WatchlistItemDtoMarketAvatarSvg {
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
    impl ::std::default::Default for WatchlistItemDtoMarketAvatarSvg {
        fn default() -> Self {
            Self {
                background_color: Ok(Default::default()),
                text_color: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl WatchlistItemDtoMarketAvatarSvg {
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
    impl ::std::convert::TryFrom<WatchlistItemDtoMarketAvatarSvg>
        for super::WatchlistItemDtoMarketAvatarSvg
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WatchlistItemDtoMarketAvatarSvg,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                background_color: value.background_color?,
                text_color: value.text_color?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::WatchlistItemDtoMarketAvatarSvg>
        for WatchlistItemDtoMarketAvatarSvg
    {
        fn from(value: super::WatchlistItemDtoMarketAvatarSvg) -> Self {
            Self {
                background_color: Ok(value.background_color),
                text_color: Ok(value.text_color),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WatchlistResponse {
        dynamic_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        gcid: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        is_default: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_user_selected_default:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        items:
            ::std::result::Result<::std::vec::Vec<super::WatchlistItemDto>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        related_assets: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<i32>>,
            ::std::string::String,
        >,
        total_items: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        watchlist_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        watchlist_rank: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        watchlist_type: ::std::result::Result<
            ::std::option::Option<super::WatchlistResponseWatchlistType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WatchlistResponse {
        fn default() -> Self {
            Self {
                dynamic_url: Ok(Default::default()),
                gcid: Ok(Default::default()),
                is_default: Ok(Default::default()),
                is_user_selected_default: Ok(Default::default()),
                items: Ok(Default::default()),
                name: Ok(Default::default()),
                related_assets: Ok(Default::default()),
                total_items: Ok(Default::default()),
                watchlist_id: Ok(Default::default()),
                watchlist_rank: Ok(Default::default()),
                watchlist_type: Ok(Default::default()),
            }
        }
    }
    impl WatchlistResponse {
        pub fn dynamic_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.dynamic_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dynamic_url: {e}"));
            self
        }
        pub fn gcid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.gcid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gcid: {e}"));
            self
        }
        pub fn is_default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_default: {e}"));
            self
        }
        pub fn is_user_selected_default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_user_selected_default = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_user_selected_default: {e}")
            });
            self
        }
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::WatchlistItemDto>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn related_assets<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<i32>>>,
            T::Error: ::std::fmt::Display,
        {
            self.related_assets = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for related_assets: {e}"));
            self
        }
        pub fn total_items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_items: {e}"));
            self
        }
        pub fn watchlist_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.watchlist_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for watchlist_id: {e}"));
            self
        }
        pub fn watchlist_rank<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.watchlist_rank = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for watchlist_rank: {e}"));
            self
        }
        pub fn watchlist_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::WatchlistResponseWatchlistType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.watchlist_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for watchlist_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WatchlistResponse> for super::WatchlistResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WatchlistResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                dynamic_url: value.dynamic_url?,
                gcid: value.gcid?,
                is_default: value.is_default?,
                is_user_selected_default: value.is_user_selected_default?,
                items: value.items?,
                name: value.name?,
                related_assets: value.related_assets?,
                total_items: value.total_items?,
                watchlist_id: value.watchlist_id?,
                watchlist_rank: value.watchlist_rank?,
                watchlist_type: value.watchlist_type?,
            })
        }
    }
    impl ::std::convert::From<super::WatchlistResponse> for WatchlistResponse {
        fn from(value: super::WatchlistResponse) -> Self {
            Self {
                dynamic_url: Ok(value.dynamic_url),
                gcid: Ok(value.gcid),
                is_default: Ok(value.is_default),
                is_user_selected_default: Ok(value.is_user_selected_default),
                items: Ok(value.items),
                name: Ok(value.name),
                related_assets: Ok(value.related_assets),
                total_items: Ok(value.total_items),
                watchlist_id: Ok(value.watchlist_id),
                watchlist_rank: Ok(value.watchlist_rank),
                watchlist_type: Ok(value.watchlist_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WatchlistsResponse {
        exception: ::std::result::Result<
            ::std::option::Option<super::WatchlistsResponseException>,
            ::std::string::String,
        >,
        is_succeeded: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        meta: ::std::result::Result<
            ::std::option::Option<super::WatchlistsResponseMeta>,
            ::std::string::String,
        >,
        status: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        watchlists:
            ::std::result::Result<::std::vec::Vec<super::WatchlistResponse>, ::std::string::String>,
    }
    impl ::std::default::Default for WatchlistsResponse {
        fn default() -> Self {
            Self {
                exception: Ok(Default::default()),
                is_succeeded: Ok(Default::default()),
                meta: Ok(Default::default()),
                status: Ok(Default::default()),
                watchlists: Ok(Default::default()),
            }
        }
    }
    impl WatchlistsResponse {
        pub fn exception<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WatchlistsResponseException>>,
            T::Error: ::std::fmt::Display,
        {
            self.exception = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exception: {e}"));
            self
        }
        pub fn is_succeeded<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_succeeded = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_succeeded: {e}"));
            self
        }
        pub fn meta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WatchlistsResponseMeta>>,
            T::Error: ::std::fmt::Display,
        {
            self.meta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meta: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn watchlists<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::WatchlistResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.watchlists = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for watchlists: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WatchlistsResponse> for super::WatchlistsResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WatchlistsResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                exception: value.exception?,
                is_succeeded: value.is_succeeded?,
                meta: value.meta?,
                status: value.status?,
                watchlists: value.watchlists?,
            })
        }
    }
    impl ::std::convert::From<super::WatchlistsResponse> for WatchlistsResponse {
        fn from(value: super::WatchlistsResponse) -> Self {
            Self {
                exception: Ok(value.exception),
                is_succeeded: Ok(value.is_succeeded),
                meta: Ok(value.meta),
                status: Ok(value.status),
                watchlists: Ok(value.watchlists),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WatchlistsResponseException {
        invalid_items:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        reason: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WatchlistsResponseException {
        fn default() -> Self {
            Self {
                invalid_items: Ok(Default::default()),
                message: Ok(Default::default()),
                reason: Ok(Default::default()),
            }
        }
    }
    impl WatchlistsResponseException {
        pub fn invalid_items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.invalid_items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for invalid_items: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WatchlistsResponseException> for super::WatchlistsResponseException {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WatchlistsResponseException,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                invalid_items: value.invalid_items?,
                message: value.message?,
                reason: value.reason?,
            })
        }
    }
    impl ::std::convert::From<super::WatchlistsResponseException> for WatchlistsResponseException {
        fn from(value: super::WatchlistsResponseException) -> Self {
            Self {
                invalid_items: Ok(value.invalid_items),
                message: Ok(value.message),
                reason: Ok(value.reason),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WatchlistsResponseMeta {
        items_per_page: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        max_items_in_watchlist_limit:
            ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        max_watchlists_limit:
            ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        page_number: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
    }
    impl ::std::default::Default for WatchlistsResponseMeta {
        fn default() -> Self {
            Self {
                items_per_page: Ok(Default::default()),
                max_items_in_watchlist_limit: Ok(Default::default()),
                max_watchlists_limit: Ok(Default::default()),
                page_number: Ok(Default::default()),
            }
        }
    }
    impl WatchlistsResponseMeta {
        pub fn items_per_page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.items_per_page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items_per_page: {e}"));
            self
        }
        pub fn max_items_in_watchlist_limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_items_in_watchlist_limit = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_items_in_watchlist_limit: {e}")
            });
            self
        }
        pub fn max_watchlists_limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_watchlists_limit = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_watchlists_limit: {e}")
            });
            self
        }
        pub fn page_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.page_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page_number: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WatchlistsResponseMeta> for super::WatchlistsResponseMeta {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WatchlistsResponseMeta,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                items_per_page: value.items_per_page?,
                max_items_in_watchlist_limit: value.max_items_in_watchlist_limit?,
                max_watchlists_limit: value.max_watchlists_limit?,
                page_number: value.page_number?,
            })
        }
    }
    impl ::std::convert::From<super::WatchlistsResponseMeta> for WatchlistsResponseMeta {
        fn from(value: super::WatchlistsResponseMeta) -> Self {
            Self {
                items_per_page: Ok(value.items_per_page),
                max_items_in_watchlist_limit: Ok(value.max_items_in_watchlist_limit),
                max_watchlists_limit: Ok(value.max_watchlists_limit),
                page_number: Ok(value.page_number),
            }
        }
    }
}
