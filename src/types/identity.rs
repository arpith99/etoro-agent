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
#[doc = "`MeResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"demoCid\": {"]
#[doc = "      \"description\": \"Demo account Customer ID - the identifier for the user's virtual/demo trading account.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"gcid\": {"]
#[doc = "      \"description\": \"Global Customer ID - the unique identifier for the user across all eToro systems.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"realCid\": {"]
#[doc = "      \"description\": \"Real account Customer ID - the identifier for the user's real trading account.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MeResponse {
    #[doc = "Demo account Customer ID - the identifier for the user's virtual/demo trading account."]
    #[serde(
        rename = "demoCid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub demo_cid: ::std::option::Option<i64>,
    #[doc = "Global Customer ID - the unique identifier for the user across all eToro systems."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gcid: ::std::option::Option<i64>,
    #[doc = "Real account Customer ID - the identifier for the user's real trading account."]
    #[serde(
        rename = "realCid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub real_cid: ::std::option::Option<i64>,
}
impl ::std::default::Default for MeResponse {
    fn default() -> Self {
        Self {
            demo_cid: Default::default(),
            gcid: Default::default(),
            real_cid: Default::default(),
        }
    }
}
#[doc = "Container for the aggregated user information response"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Container for the aggregated user information response\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"users\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PublicAggregatedInfoUser\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PublicAggregatedInfoResponse {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub users: ::std::vec::Vec<PublicAggregatedInfoUser>,
}
impl ::std::default::Default for PublicAggregatedInfoResponse {
    fn default() -> Self {
        Self {
            users: Default::default(),
        }
    }
}
#[doc = "`PublicAggregatedInfoUiUserAvatar`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"height\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"Type of avatar image\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Original\","]
#[doc = "        \"OriginalCropped\","]
#[doc = "        \"Resized\","]
#[doc = "        \"Retouched\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"width\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PublicAggregatedInfoUiUserAvatar {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<i64>,
    #[doc = "Type of avatar image"]
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<PublicAggregatedInfoUiUserAvatarType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<i64>,
}
impl ::std::default::Default for PublicAggregatedInfoUiUserAvatar {
    fn default() -> Self {
        Self {
            height: Default::default(),
            type_: Default::default(),
            url: Default::default(),
            width: Default::default(),
        }
    }
}
#[doc = "Type of avatar image"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Type of avatar image\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Original\","]
#[doc = "    \"OriginalCropped\","]
#[doc = "    \"Resized\","]
#[doc = "    \"Retouched\""]
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
pub enum PublicAggregatedInfoUiUserAvatarType {
    Original,
    OriginalCropped,
    Resized,
    Retouched,
}
impl ::std::fmt::Display for PublicAggregatedInfoUiUserAvatarType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Original => f.write_str("Original"),
            Self::OriginalCropped => f.write_str("OriginalCropped"),
            Self::Resized => f.write_str("Resized"),
            Self::Retouched => f.write_str("Retouched"),
        }
    }
}
impl ::std::str::FromStr for PublicAggregatedInfoUiUserAvatarType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Original" => Ok(Self::Original),
            "OriginalCropped" => Ok(Self::OriginalCropped),
            "Resized" => Ok(Self::Resized),
            "Retouched" => Ok(Self::Retouched),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PublicAggregatedInfoUiUserAvatarType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PublicAggregatedInfoUiUserAvatarType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PublicAggregatedInfoUiUserAvatarType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PublicAggregatedInfoUiUserBio`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"aboutMe\": {"]
#[doc = "      \"description\": \"User's full about me text\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"aboutMeShort\": {"]
#[doc = "      \"description\": \"Short summary of user's about me text\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"gcid\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"languageCode\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"strategyID\": {"]
#[doc = "      \"description\": \"ID of the user's trading strategy\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PublicAggregatedInfoUiUserBio {
    #[doc = "User's full about me text"]
    #[serde(
        rename = "aboutMe",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub about_me: ::std::option::Option<::std::string::String>,
    #[doc = "Short summary of user's about me text"]
    #[serde(
        rename = "aboutMeShort",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub about_me_short: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gcid: ::std::option::Option<i64>,
    #[serde(
        rename = "languageCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[doc = "ID of the user's trading strategy"]
    #[serde(
        rename = "strategyID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub strategy_id: ::std::option::Option<i64>,
}
impl ::std::default::Default for PublicAggregatedInfoUiUserBio {
    fn default() -> Self {
        Self {
            about_me: Default::default(),
            about_me_short: Default::default(),
            gcid: Default::default(),
            language_code: Default::default(),
            strategy_id: Default::default(),
        }
    }
}
#[doc = "Comprehensive user profile information including account details, verification status, and preferences"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Comprehensive user profile information including account details, verification status, and preferences\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"aboutMe\": {"]
#[doc = "      \"description\": \"User's full about me text\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"aboutMeShort\": {"]
#[doc = "      \"description\": \"Short summary of user's about me text\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"accountStatus\": {"]
#[doc = "      \"description\": \"Current account status code indicating active, suspended, or other states\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"accountType\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"allowDisplayFullName\": {"]
#[doc = "      \"description\": \"Indicates whether the user has consented to displaying their full name publicly\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"avatars\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PublicAggregatedInfoUiUserAvatar\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"country\": {"]
#[doc = "      \"description\": \"User's registered country ID based on system country codes\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"customerRestrictions\": {"]
#[doc = "      \"description\": \"List of customer restrictions applied to the account\","]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"CID\": {"]
#[doc = "            \"description\": \"Customer ID\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"occured\": {"]
#[doc = "            \"description\": \"When the restriction occurred (note: spec field name is 'occured', sic — typo for 'occurred')\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"date-time\""]
#[doc = "          },"]
#[doc = "          \"reasonID\": {"]
#[doc = "            \"description\": \"Reason for restriction\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"restrictionTypeID\": {"]
#[doc = "            \"description\": \"Type of restriction\","]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"demoCID\": {"]
#[doc = "      \"description\": \"Customer ID for demo/practice account if available\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"firstName\": {"]
#[doc = "      \"description\": \"User's first name (visible if allowDisplayFullName is true)\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"fundType\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"gcid\": {"]
#[doc = "      \"description\": \"Global Customer ID - Unique identifier across all systems\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"gdprInfo\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"object\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"accountStatus\": {"]
#[doc = "          \"$ref\": \"#/$defs/PublicAggregatedInfoAccountStatus\""]
#[doc = "        },"]
#[doc = "        \"playerStatus\": {"]
#[doc = "          \"$ref\": \"#/$defs/PublicAggregatedInfoPlayerStatus\""]
#[doc = "        },"]
#[doc = "        \"playerStatusReason\": {"]
#[doc = "          \"$ref\": \"#/$defs/PublicAggregatedInfoPlayerStatusReason\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"homepage\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"isPi\": {"]
#[doc = "      \"description\": \"Indicates if user is a Professional Investor with special privileges\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isVerified\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"language\": {"]
#[doc = "      \"description\": \"User's preferred language ID based on system language codes\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"languageIsoCode\": {"]
#[doc = "      \"description\": \"ISO 639-1 language code for user's preferred language\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"lastName\": {"]
#[doc = "      \"description\": \"User's last name (visible if allowDisplayFullName is true)\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"masterAccountCid\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"middleName\": {"]
#[doc = "      \"description\": \"User's middle name\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"optOut\": {"]
#[doc = "      \"description\": \"Indicates if user has opted out of public profile features\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"piLevel\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"playerStatus\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"realCID\": {"]
#[doc = "      \"description\": \"Customer ID for real trading account\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"userBio\": {"]
#[doc = "      \"description\": \"Structured biographical information including trading strategy\","]
#[doc = "      \"$ref\": \"#/$defs/PublicAggregatedInfoUiUserBio\""]
#[doc = "    },"]
#[doc = "    \"userFlowSignature\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"username\": {"]
#[doc = "      \"description\": \"Unique username identifier for the user\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"verificationLevel\": {"]
#[doc = "      \"description\": \"User's current verification level (0-3, where 3 is fully verified)\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"whiteLabel\": {"]
#[doc = "      \"description\": \"White label partner identifier if user belongs to a partner program\","]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PublicAggregatedInfoUser {
    #[doc = "User's full about me text"]
    #[serde(
        rename = "aboutMe",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub about_me: ::std::option::Option<::std::string::String>,
    #[doc = "Short summary of user's about me text"]
    #[serde(
        rename = "aboutMeShort",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub about_me_short: ::std::option::Option<::std::string::String>,
    #[doc = "Current account status code indicating active, suspended, or other states"]
    #[serde(
        rename = "accountStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub account_status: ::std::option::Option<i64>,
    #[serde(
        rename = "accountType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub account_type: ::std::option::Option<i64>,
    #[doc = "Indicates whether the user has consented to displaying their full name publicly"]
    #[serde(
        rename = "allowDisplayFullName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_display_full_name: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub avatars: ::std::vec::Vec<PublicAggregatedInfoUiUserAvatar>,
    #[doc = "User's registered country ID based on system country codes"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<i64>,
    #[doc = "List of customer restrictions applied to the account"]
    #[serde(
        rename = "customerRestrictions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub customer_restrictions:
        ::std::option::Option<::std::vec::Vec<PublicAggregatedInfoUserCustomerRestrictionsItem>>,
    #[doc = "Customer ID for demo/practice account if available"]
    #[serde(
        rename = "demoCID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub demo_cid: ::std::option::Option<i64>,
    #[doc = "User's first name (visible if allowDisplayFullName is true)"]
    #[serde(
        rename = "firstName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "fundType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fund_type: ::std::option::Option<::std::string::String>,
    #[doc = "Global Customer ID - Unique identifier across all systems"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gcid: ::std::option::Option<i64>,
    #[serde(
        rename = "gdprInfo",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub gdpr_info: ::std::option::Option<PublicAggregatedInfoUserGdprInfo>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub homepage: ::std::option::Option<i64>,
    #[doc = "Indicates if user is a Professional Investor with special privileges"]
    #[serde(
        rename = "isPi",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_pi: ::std::option::Option<bool>,
    #[serde(
        rename = "isVerified",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_verified: ::std::option::Option<bool>,
    #[doc = "User's preferred language ID based on system language codes"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<i64>,
    #[doc = "ISO 639-1 language code for user's preferred language"]
    #[serde(
        rename = "languageIsoCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub language_iso_code: ::std::option::Option<::std::string::String>,
    #[doc = "User's last name (visible if allowDisplayFullName is true)"]
    #[serde(
        rename = "lastName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "masterAccountCid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub master_account_cid: ::std::option::Option<i64>,
    #[doc = "User's middle name"]
    #[serde(
        rename = "middleName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub middle_name: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates if user has opted out of public profile features"]
    #[serde(
        rename = "optOut",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub opt_out: ::std::option::Option<bool>,
    #[serde(
        rename = "piLevel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pi_level: ::std::option::Option<i64>,
    #[serde(
        rename = "playerStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub player_status: ::std::option::Option<i64>,
    #[doc = "Customer ID for real trading account"]
    #[serde(
        rename = "realCID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub real_cid: ::std::option::Option<i64>,
    #[doc = "Structured biographical information including trading strategy"]
    #[serde(
        rename = "userBio",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_bio: ::std::option::Option<PublicAggregatedInfoUiUserBio>,
    #[serde(
        rename = "userFlowSignature",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_flow_signature: ::std::option::Option<::std::string::String>,
    #[doc = "Unique username identifier for the user"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub username: ::std::option::Option<::std::string::String>,
    #[doc = "User's current verification level (0-3, where 3 is fully verified)"]
    #[serde(
        rename = "verificationLevel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub verification_level: ::std::option::Option<i64>,
    #[doc = "White label partner identifier if user belongs to a partner program"]
    #[serde(
        rename = "whiteLabel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub white_label: ::std::option::Option<i64>,
}
impl ::std::default::Default for PublicAggregatedInfoUser {
    fn default() -> Self {
        Self {
            about_me: Default::default(),
            about_me_short: Default::default(),
            account_status: Default::default(),
            account_type: Default::default(),
            allow_display_full_name: Default::default(),
            avatars: Default::default(),
            country: Default::default(),
            customer_restrictions: Default::default(),
            demo_cid: Default::default(),
            first_name: Default::default(),
            fund_type: Default::default(),
            gcid: Default::default(),
            gdpr_info: Default::default(),
            homepage: Default::default(),
            is_pi: Default::default(),
            is_verified: Default::default(),
            language: Default::default(),
            language_iso_code: Default::default(),
            last_name: Default::default(),
            master_account_cid: Default::default(),
            middle_name: Default::default(),
            opt_out: Default::default(),
            pi_level: Default::default(),
            player_status: Default::default(),
            real_cid: Default::default(),
            user_bio: Default::default(),
            user_flow_signature: Default::default(),
            username: Default::default(),
            verification_level: Default::default(),
            white_label: Default::default(),
        }
    }
}
#[doc = "`PublicAggregatedInfoUserCustomerRestrictionsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"CID\": {"]
#[doc = "      \"description\": \"Customer ID\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"occured\": {"]
#[doc = "      \"description\": \"When the restriction occurred (note: spec field name is 'occured', sic — typo for 'occurred')\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"reasonID\": {"]
#[doc = "      \"description\": \"Reason for restriction\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"restrictionTypeID\": {"]
#[doc = "      \"description\": \"Type of restriction\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PublicAggregatedInfoUserCustomerRestrictionsItem {
    #[doc = "Customer ID"]
    #[serde(
        rename = "CID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cid: ::std::option::Option<i64>,
    #[doc = "When the restriction occurred (note: spec field name is 'occured', sic — typo for 'occurred')"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub occured: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Reason for restriction"]
    #[serde(
        rename = "reasonID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reason_id: ::std::option::Option<i64>,
    #[doc = "Type of restriction"]
    #[serde(
        rename = "restrictionTypeID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub restriction_type_id: ::std::option::Option<i64>,
}
impl ::std::default::Default for PublicAggregatedInfoUserCustomerRestrictionsItem {
    fn default() -> Self {
        Self {
            cid: Default::default(),
            occured: Default::default(),
            reason_id: Default::default(),
            restriction_type_id: Default::default(),
        }
    }
}
#[doc = "`PublicAggregatedInfoUserGdprInfo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"accountStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/PublicAggregatedInfoAccountStatus\""]
#[doc = "    },"]
#[doc = "    \"playerStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/PublicAggregatedInfoPlayerStatus\""]
#[doc = "    },"]
#[doc = "    \"playerStatusReason\": {"]
#[doc = "      \"$ref\": \"#/$defs/PublicAggregatedInfoPlayerStatusReason\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PublicAggregatedInfoUserGdprInfo {
    #[serde(
        rename = "accountStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub account_status:
        ::std::option::Option<::etoro_agent::types::manual::PublicAggregatedInfoAccountStatus>,
    #[serde(
        rename = "playerStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub player_status:
        ::std::option::Option<::etoro_agent::types::manual::PublicAggregatedInfoPlayerStatus>,
    #[serde(
        rename = "playerStatusReason",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub player_status_reason:
        ::std::option::Option<::etoro_agent::types::manual::PublicAggregatedInfoPlayerStatusReason>,
}
impl ::std::default::Default for PublicAggregatedInfoUserGdprInfo {
    fn default() -> Self {
        Self {
            account_status: Default::default(),
            player_status: Default::default(),
            player_status_reason: Default::default(),
        }
    }
}
#[doc = "`User`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"avatar\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"large\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        },"]
#[doc = "        \"medium\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        },"]
#[doc = "        \"small\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"countryCode\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"firstName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"isBlocked\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isPrivate\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"lastName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"piLevel\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"roles\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"username\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct User {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<UserAvatar>,
    #[serde(
        rename = "countryCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub country_code: ::std::option::Option<i64>,
    #[serde(
        rename = "firstName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "isBlocked",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_blocked: ::std::option::Option<bool>,
    #[serde(
        rename = "isPrivate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_private: ::std::option::Option<bool>,
    #[serde(
        rename = "lastName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "piLevel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pi_level: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub roles: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub username: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for User {
    fn default() -> Self {
        Self {
            avatar: Default::default(),
            country_code: Default::default(),
            first_name: Default::default(),
            id: Default::default(),
            is_blocked: Default::default(),
            is_private: Default::default(),
            last_name: Default::default(),
            pi_level: Default::default(),
            roles: Default::default(),
            username: Default::default(),
        }
    }
}
#[doc = "`UserAvatar`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"large\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"medium\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"small\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UserAvatar {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub large: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub medium: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub small: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for UserAvatar {
    fn default() -> Self {
        Self {
            large: Default::default(),
            medium: Default::default(),
            small: Default::default(),
        }
    }
}
