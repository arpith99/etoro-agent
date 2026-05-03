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
#[doc = "`PublicAggregatedInfoAccountStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ],"]
#[doc = "  \"enum\": ["]
#[doc = "    1,"]
#[doc = "    2,"]
#[doc = "    null"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PublicAggregatedInfoAccountStatus(
    pub ::std::option::Option<PublicAggregatedInfoAccountStatusInner>,
);
impl ::std::ops::Deref for PublicAggregatedInfoAccountStatus {
    type Target = ::std::option::Option<PublicAggregatedInfoAccountStatusInner>;
    fn deref(&self) -> &::std::option::Option<PublicAggregatedInfoAccountStatusInner> {
        &self.0
    }
}
impl ::std::convert::From<PublicAggregatedInfoAccountStatus>
    for ::std::option::Option<PublicAggregatedInfoAccountStatusInner>
{
    fn from(value: PublicAggregatedInfoAccountStatus) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<PublicAggregatedInfoAccountStatusInner>>
    for PublicAggregatedInfoAccountStatus
{
    fn from(value: ::std::option::Option<PublicAggregatedInfoAccountStatusInner>) -> Self {
        Self(value)
    }
}
#[doc = "`PublicAggregatedInfoAccountStatusInner`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"enum\": ["]
#[doc = "    1,"]
#[doc = "    2"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PublicAggregatedInfoAccountStatusInner(i64);
impl ::std::ops::Deref for PublicAggregatedInfoAccountStatusInner {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<PublicAggregatedInfoAccountStatusInner> for i64 {
    fn from(value: PublicAggregatedInfoAccountStatusInner) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i64> for PublicAggregatedInfoAccountStatusInner {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![1_i64, 2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for PublicAggregatedInfoAccountStatusInner {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "`PublicAggregatedInfoPlayerStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ],"]
#[doc = "  \"enum\": ["]
#[doc = "    1,"]
#[doc = "    2,"]
#[doc = "    3,"]
#[doc = "    4,"]
#[doc = "    5,"]
#[doc = "    6,"]
#[doc = "    7,"]
#[doc = "    8,"]
#[doc = "    9,"]
#[doc = "    10,"]
#[doc = "    11,"]
#[doc = "    12,"]
#[doc = "    13,"]
#[doc = "    14,"]
#[doc = "    15,"]
#[doc = "    null"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PublicAggregatedInfoPlayerStatus(
    pub ::std::option::Option<PublicAggregatedInfoPlayerStatusInner>,
);
impl ::std::ops::Deref for PublicAggregatedInfoPlayerStatus {
    type Target = ::std::option::Option<PublicAggregatedInfoPlayerStatusInner>;
    fn deref(&self) -> &::std::option::Option<PublicAggregatedInfoPlayerStatusInner> {
        &self.0
    }
}
impl ::std::convert::From<PublicAggregatedInfoPlayerStatus>
    for ::std::option::Option<PublicAggregatedInfoPlayerStatusInner>
{
    fn from(value: PublicAggregatedInfoPlayerStatus) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<PublicAggregatedInfoPlayerStatusInner>>
    for PublicAggregatedInfoPlayerStatus
{
    fn from(value: ::std::option::Option<PublicAggregatedInfoPlayerStatusInner>) -> Self {
        Self(value)
    }
}
#[doc = "`PublicAggregatedInfoPlayerStatusInner`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"enum\": ["]
#[doc = "    1,"]
#[doc = "    2,"]
#[doc = "    3,"]
#[doc = "    4,"]
#[doc = "    5,"]
#[doc = "    6,"]
#[doc = "    7,"]
#[doc = "    8,"]
#[doc = "    9,"]
#[doc = "    10,"]
#[doc = "    11,"]
#[doc = "    12,"]
#[doc = "    13,"]
#[doc = "    14,"]
#[doc = "    15"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PublicAggregatedInfoPlayerStatusInner(i64);
impl ::std::ops::Deref for PublicAggregatedInfoPlayerStatusInner {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<PublicAggregatedInfoPlayerStatusInner> for i64 {
    fn from(value: PublicAggregatedInfoPlayerStatusInner) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i64> for PublicAggregatedInfoPlayerStatusInner {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![
            1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64, 8_i64, 9_i64, 10_i64, 11_i64, 12_i64,
            13_i64, 14_i64, 15_i64,
        ]
        .contains(&value)
        {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for PublicAggregatedInfoPlayerStatusInner {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "`PublicAggregatedInfoPlayerStatusReason`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ],"]
#[doc = "  \"enum\": ["]
#[doc = "    0,"]
#[doc = "    1,"]
#[doc = "    2,"]
#[doc = "    3,"]
#[doc = "    4,"]
#[doc = "    5,"]
#[doc = "    6,"]
#[doc = "    7,"]
#[doc = "    8,"]
#[doc = "    9,"]
#[doc = "    10,"]
#[doc = "    11,"]
#[doc = "    12,"]
#[doc = "    13,"]
#[doc = "    14,"]
#[doc = "    15,"]
#[doc = "    16,"]
#[doc = "    17,"]
#[doc = "    18,"]
#[doc = "    19,"]
#[doc = "    20,"]
#[doc = "    21,"]
#[doc = "    22,"]
#[doc = "    23,"]
#[doc = "    24,"]
#[doc = "    25,"]
#[doc = "    26,"]
#[doc = "    27,"]
#[doc = "    28,"]
#[doc = "    29,"]
#[doc = "    30,"]
#[doc = "    31,"]
#[doc = "    32,"]
#[doc = "    33,"]
#[doc = "    34,"]
#[doc = "    35,"]
#[doc = "    36,"]
#[doc = "    37,"]
#[doc = "    38,"]
#[doc = "    39,"]
#[doc = "    40,"]
#[doc = "    41,"]
#[doc = "    42,"]
#[doc = "    null"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PublicAggregatedInfoPlayerStatusReason(
    pub ::std::option::Option<PublicAggregatedInfoPlayerStatusReasonInner>,
);
impl ::std::ops::Deref for PublicAggregatedInfoPlayerStatusReason {
    type Target = ::std::option::Option<PublicAggregatedInfoPlayerStatusReasonInner>;
    fn deref(&self) -> &::std::option::Option<PublicAggregatedInfoPlayerStatusReasonInner> {
        &self.0
    }
}
impl ::std::convert::From<PublicAggregatedInfoPlayerStatusReason>
    for ::std::option::Option<PublicAggregatedInfoPlayerStatusReasonInner>
{
    fn from(value: PublicAggregatedInfoPlayerStatusReason) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<PublicAggregatedInfoPlayerStatusReasonInner>>
    for PublicAggregatedInfoPlayerStatusReason
{
    fn from(value: ::std::option::Option<PublicAggregatedInfoPlayerStatusReasonInner>) -> Self {
        Self(value)
    }
}
#[doc = "`PublicAggregatedInfoPlayerStatusReasonInner`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"enum\": ["]
#[doc = "    0,"]
#[doc = "    1,"]
#[doc = "    2,"]
#[doc = "    3,"]
#[doc = "    4,"]
#[doc = "    5,"]
#[doc = "    6,"]
#[doc = "    7,"]
#[doc = "    8,"]
#[doc = "    9,"]
#[doc = "    10,"]
#[doc = "    11,"]
#[doc = "    12,"]
#[doc = "    13,"]
#[doc = "    14,"]
#[doc = "    15,"]
#[doc = "    16,"]
#[doc = "    17,"]
#[doc = "    18,"]
#[doc = "    19,"]
#[doc = "    20,"]
#[doc = "    21,"]
#[doc = "    22,"]
#[doc = "    23,"]
#[doc = "    24,"]
#[doc = "    25,"]
#[doc = "    26,"]
#[doc = "    27,"]
#[doc = "    28,"]
#[doc = "    29,"]
#[doc = "    30,"]
#[doc = "    31,"]
#[doc = "    32,"]
#[doc = "    33,"]
#[doc = "    34,"]
#[doc = "    35,"]
#[doc = "    36,"]
#[doc = "    37,"]
#[doc = "    38,"]
#[doc = "    39,"]
#[doc = "    40,"]
#[doc = "    41,"]
#[doc = "    42"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PublicAggregatedInfoPlayerStatusReasonInner(i64);
impl ::std::ops::Deref for PublicAggregatedInfoPlayerStatusReasonInner {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<PublicAggregatedInfoPlayerStatusReasonInner> for i64 {
    fn from(value: PublicAggregatedInfoPlayerStatusReasonInner) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i64> for PublicAggregatedInfoPlayerStatusReasonInner {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![
            0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64, 8_i64, 9_i64, 10_i64, 11_i64,
            12_i64, 13_i64, 14_i64, 15_i64, 16_i64, 17_i64, 18_i64, 19_i64, 20_i64, 21_i64, 22_i64,
            23_i64, 24_i64, 25_i64, 26_i64, 27_i64, 28_i64, 29_i64, 30_i64, 31_i64, 32_i64, 33_i64,
            34_i64, 35_i64, 36_i64, 37_i64, 38_i64, 39_i64, 40_i64, 41_i64, 42_i64,
        ]
        .contains(&value)
        {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for PublicAggregatedInfoPlayerStatusReasonInner {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
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
    pub account_status: ::std::option::Option<PublicAggregatedInfoAccountStatus>,
    #[serde(
        rename = "playerStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub player_status: ::std::option::Option<PublicAggregatedInfoPlayerStatus>,
    #[serde(
        rename = "playerStatusReason",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub player_status_reason: ::std::option::Option<PublicAggregatedInfoPlayerStatusReason>,
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
#[doc = "`UserRole`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"int32\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Regular\","]
#[doc = "    \"PI\","]
#[doc = "    \"Moderator\","]
#[doc = "    \"Anonymous\","]
#[doc = "    \"eToroTeam\","]
#[doc = "    \"eTorian\","]
#[doc = "    \"CopyPortfolio\","]
#[doc = "    \"Depositor\","]
#[doc = "    \"Admin\","]
#[doc = "    \"Verified\","]
#[doc = "    \"Analyst\""]
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
pub enum UserRole {
    Regular,
    #[serde(rename = "PI")]
    Pi,
    Moderator,
    Anonymous,
    #[serde(rename = "eToroTeam")]
    EToroTeam,
    #[serde(rename = "eTorian")]
    ETorian,
    CopyPortfolio,
    Depositor,
    Admin,
    Verified,
    Analyst,
}
impl ::std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Regular => f.write_str("Regular"),
            Self::Pi => f.write_str("PI"),
            Self::Moderator => f.write_str("Moderator"),
            Self::Anonymous => f.write_str("Anonymous"),
            Self::EToroTeam => f.write_str("eToroTeam"),
            Self::ETorian => f.write_str("eTorian"),
            Self::CopyPortfolio => f.write_str("CopyPortfolio"),
            Self::Depositor => f.write_str("Depositor"),
            Self::Admin => f.write_str("Admin"),
            Self::Verified => f.write_str("Verified"),
            Self::Analyst => f.write_str("Analyst"),
        }
    }
}
impl ::std::str::FromStr for UserRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Regular" => Ok(Self::Regular),
            "PI" => Ok(Self::Pi),
            "Moderator" => Ok(Self::Moderator),
            "Anonymous" => Ok(Self::Anonymous),
            "eToroTeam" => Ok(Self::EToroTeam),
            "eTorian" => Ok(Self::ETorian),
            "CopyPortfolio" => Ok(Self::CopyPortfolio),
            "Depositor" => Ok(Self::Depositor),
            "Admin" => Ok(Self::Admin),
            "Verified" => Ok(Self::Verified),
            "Analyst" => Ok(Self::Analyst),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UserRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UserRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UserRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
