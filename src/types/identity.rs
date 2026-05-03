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
impl MeResponse {
    pub fn builder() -> builder::MeResponse {
        Default::default()
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
impl PublicAggregatedInfoResponse {
    pub fn builder() -> builder::PublicAggregatedInfoResponse {
        Default::default()
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
impl PublicAggregatedInfoUiUserAvatar {
    pub fn builder() -> builder::PublicAggregatedInfoUiUserAvatar {
        Default::default()
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
impl PublicAggregatedInfoUiUserBio {
    pub fn builder() -> builder::PublicAggregatedInfoUiUserBio {
        Default::default()
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
impl PublicAggregatedInfoUser {
    pub fn builder() -> builder::PublicAggregatedInfoUser {
        Default::default()
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
impl PublicAggregatedInfoUserCustomerRestrictionsItem {
    pub fn builder() -> builder::PublicAggregatedInfoUserCustomerRestrictionsItem {
        Default::default()
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
impl PublicAggregatedInfoUserGdprInfo {
    pub fn builder() -> builder::PublicAggregatedInfoUserGdprInfo {
        Default::default()
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
impl User {
    pub fn builder() -> builder::User {
        Default::default()
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
impl UserAvatar {
    pub fn builder() -> builder::UserAvatar {
        Default::default()
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
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct MeResponse {
        demo_cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        gcid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        real_cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for MeResponse {
        fn default() -> Self {
            Self {
                demo_cid: Ok(Default::default()),
                gcid: Ok(Default::default()),
                real_cid: Ok(Default::default()),
            }
        }
    }
    impl MeResponse {
        pub fn demo_cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.demo_cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for demo_cid: {e}"));
            self
        }
        pub fn gcid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.gcid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gcid: {e}"));
            self
        }
        pub fn real_cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.real_cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for real_cid: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MeResponse> for super::MeResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MeResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                demo_cid: value.demo_cid?,
                gcid: value.gcid?,
                real_cid: value.real_cid?,
            })
        }
    }
    impl ::std::convert::From<super::MeResponse> for MeResponse {
        fn from(value: super::MeResponse) -> Self {
            Self {
                demo_cid: Ok(value.demo_cid),
                gcid: Ok(value.gcid),
                real_cid: Ok(value.real_cid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicAggregatedInfoResponse {
        users: ::std::result::Result<
            ::std::vec::Vec<super::PublicAggregatedInfoUser>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PublicAggregatedInfoResponse {
        fn default() -> Self {
            Self {
                users: Ok(Default::default()),
            }
        }
    }
    impl PublicAggregatedInfoResponse {
        pub fn users<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PublicAggregatedInfoUser>>,
            T::Error: ::std::fmt::Display,
        {
            self.users = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for users: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PublicAggregatedInfoResponse> for super::PublicAggregatedInfoResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicAggregatedInfoResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                users: value.users?,
            })
        }
    }
    impl ::std::convert::From<super::PublicAggregatedInfoResponse> for PublicAggregatedInfoResponse {
        fn from(value: super::PublicAggregatedInfoResponse) -> Self {
            Self {
                users: Ok(value.users),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicAggregatedInfoUiUserAvatar {
        height: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::PublicAggregatedInfoUiUserAvatarType>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        width: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for PublicAggregatedInfoUiUserAvatar {
        fn default() -> Self {
            Self {
                height: Ok(Default::default()),
                type_: Ok(Default::default()),
                url: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl PublicAggregatedInfoUiUserAvatar {
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PublicAggregatedInfoUiUserAvatarType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
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
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for width: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PublicAggregatedInfoUiUserAvatar>
        for super::PublicAggregatedInfoUiUserAvatar
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicAggregatedInfoUiUserAvatar,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                height: value.height?,
                type_: value.type_?,
                url: value.url?,
                width: value.width?,
            })
        }
    }
    impl ::std::convert::From<super::PublicAggregatedInfoUiUserAvatar>
        for PublicAggregatedInfoUiUserAvatar
    {
        fn from(value: super::PublicAggregatedInfoUiUserAvatar) -> Self {
            Self {
                height: Ok(value.height),
                type_: Ok(value.type_),
                url: Ok(value.url),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicAggregatedInfoUiUserBio {
        about_me: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        about_me_short: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        gcid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        language_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        strategy_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for PublicAggregatedInfoUiUserBio {
        fn default() -> Self {
            Self {
                about_me: Ok(Default::default()),
                about_me_short: Ok(Default::default()),
                gcid: Ok(Default::default()),
                language_code: Ok(Default::default()),
                strategy_id: Ok(Default::default()),
            }
        }
    }
    impl PublicAggregatedInfoUiUserBio {
        pub fn about_me<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.about_me = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for about_me: {e}"));
            self
        }
        pub fn about_me_short<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.about_me_short = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for about_me_short: {e}"));
            self
        }
        pub fn gcid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.gcid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gcid: {e}"));
            self
        }
        pub fn language_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.language_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for language_code: {e}"));
            self
        }
        pub fn strategy_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strategy_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PublicAggregatedInfoUiUserBio>
        for super::PublicAggregatedInfoUiUserBio
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicAggregatedInfoUiUserBio,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                about_me: value.about_me?,
                about_me_short: value.about_me_short?,
                gcid: value.gcid?,
                language_code: value.language_code?,
                strategy_id: value.strategy_id?,
            })
        }
    }
    impl ::std::convert::From<super::PublicAggregatedInfoUiUserBio> for PublicAggregatedInfoUiUserBio {
        fn from(value: super::PublicAggregatedInfoUiUserBio) -> Self {
            Self {
                about_me: Ok(value.about_me),
                about_me_short: Ok(value.about_me_short),
                gcid: Ok(value.gcid),
                language_code: Ok(value.language_code),
                strategy_id: Ok(value.strategy_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicAggregatedInfoUser {
        about_me: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        about_me_short: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        account_status: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        account_type: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        allow_display_full_name:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        avatars: ::std::result::Result<
            ::std::vec::Vec<super::PublicAggregatedInfoUiUserAvatar>,
            ::std::string::String,
        >,
        country: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        customer_restrictions: ::std::result::Result<
            ::std::option::Option<
                ::std::vec::Vec<super::PublicAggregatedInfoUserCustomerRestrictionsItem>,
            >,
            ::std::string::String,
        >,
        demo_cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        first_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        fund_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        gcid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        gdpr_info: ::std::result::Result<
            ::std::option::Option<super::PublicAggregatedInfoUserGdprInfo>,
            ::std::string::String,
        >,
        homepage: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        is_pi: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_verified: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        language: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        language_iso_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        last_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        master_account_cid:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        middle_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        opt_out: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        pi_level: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        player_status: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        real_cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        user_bio: ::std::result::Result<
            ::std::option::Option<super::PublicAggregatedInfoUiUserBio>,
            ::std::string::String,
        >,
        user_flow_signature: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        username: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        verification_level:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        white_label: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for PublicAggregatedInfoUser {
        fn default() -> Self {
            Self {
                about_me: Ok(Default::default()),
                about_me_short: Ok(Default::default()),
                account_status: Ok(Default::default()),
                account_type: Ok(Default::default()),
                allow_display_full_name: Ok(Default::default()),
                avatars: Ok(Default::default()),
                country: Ok(Default::default()),
                customer_restrictions: Ok(Default::default()),
                demo_cid: Ok(Default::default()),
                first_name: Ok(Default::default()),
                fund_type: Ok(Default::default()),
                gcid: Ok(Default::default()),
                gdpr_info: Ok(Default::default()),
                homepage: Ok(Default::default()),
                is_pi: Ok(Default::default()),
                is_verified: Ok(Default::default()),
                language: Ok(Default::default()),
                language_iso_code: Ok(Default::default()),
                last_name: Ok(Default::default()),
                master_account_cid: Ok(Default::default()),
                middle_name: Ok(Default::default()),
                opt_out: Ok(Default::default()),
                pi_level: Ok(Default::default()),
                player_status: Ok(Default::default()),
                real_cid: Ok(Default::default()),
                user_bio: Ok(Default::default()),
                user_flow_signature: Ok(Default::default()),
                username: Ok(Default::default()),
                verification_level: Ok(Default::default()),
                white_label: Ok(Default::default()),
            }
        }
    }
    impl PublicAggregatedInfoUser {
        pub fn about_me<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.about_me = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for about_me: {e}"));
            self
        }
        pub fn about_me_short<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.about_me_short = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for about_me_short: {e}"));
            self
        }
        pub fn account_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.account_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account_status: {e}"));
            self
        }
        pub fn account_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.account_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account_type: {e}"));
            self
        }
        pub fn allow_display_full_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.allow_display_full_name = value.try_into().map_err(|e| {
                format!("error converting supplied value for allow_display_full_name: {e}")
            });
            self
        }
        pub fn avatars<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PublicAggregatedInfoUiUserAvatar>>,
            T::Error: ::std::fmt::Display,
        {
            self.avatars = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for avatars: {e}"));
            self
        }
        pub fn country<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.country = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for country: {e}"));
            self
        }
        pub fn customer_restrictions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    ::std::vec::Vec<super::PublicAggregatedInfoUserCustomerRestrictionsItem>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.customer_restrictions = value.try_into().map_err(|e| {
                format!("error converting supplied value for customer_restrictions: {e}")
            });
            self
        }
        pub fn demo_cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.demo_cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for demo_cid: {e}"));
            self
        }
        pub fn first_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.first_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for first_name: {e}"));
            self
        }
        pub fn fund_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.fund_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fund_type: {e}"));
            self
        }
        pub fn gcid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.gcid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gcid: {e}"));
            self
        }
        pub fn gdpr_info<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PublicAggregatedInfoUserGdprInfo>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.gdpr_info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gdpr_info: {e}"));
            self
        }
        pub fn homepage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.homepage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for homepage: {e}"));
            self
        }
        pub fn is_pi<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_pi = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_pi: {e}"));
            self
        }
        pub fn is_verified<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_verified = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_verified: {e}"));
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for language: {e}"));
            self
        }
        pub fn language_iso_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.language_iso_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for language_iso_code: {e}"));
            self
        }
        pub fn last_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_name: {e}"));
            self
        }
        pub fn master_account_cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.master_account_cid = value.try_into().map_err(|e| {
                format!("error converting supplied value for master_account_cid: {e}")
            });
            self
        }
        pub fn middle_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.middle_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for middle_name: {e}"));
            self
        }
        pub fn opt_out<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.opt_out = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for opt_out: {e}"));
            self
        }
        pub fn pi_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.pi_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pi_level: {e}"));
            self
        }
        pub fn player_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.player_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for player_status: {e}"));
            self
        }
        pub fn real_cid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.real_cid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for real_cid: {e}"));
            self
        }
        pub fn user_bio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PublicAggregatedInfoUiUserBio>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_bio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_bio: {e}"));
            self
        }
        pub fn user_flow_signature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_flow_signature = value.try_into().map_err(|e| {
                format!("error converting supplied value for user_flow_signature: {e}")
            });
            self
        }
        pub fn username<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.username = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for username: {e}"));
            self
        }
        pub fn verification_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.verification_level = value.try_into().map_err(|e| {
                format!("error converting supplied value for verification_level: {e}")
            });
            self
        }
        pub fn white_label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.white_label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for white_label: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PublicAggregatedInfoUser> for super::PublicAggregatedInfoUser {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicAggregatedInfoUser,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                about_me: value.about_me?,
                about_me_short: value.about_me_short?,
                account_status: value.account_status?,
                account_type: value.account_type?,
                allow_display_full_name: value.allow_display_full_name?,
                avatars: value.avatars?,
                country: value.country?,
                customer_restrictions: value.customer_restrictions?,
                demo_cid: value.demo_cid?,
                first_name: value.first_name?,
                fund_type: value.fund_type?,
                gcid: value.gcid?,
                gdpr_info: value.gdpr_info?,
                homepage: value.homepage?,
                is_pi: value.is_pi?,
                is_verified: value.is_verified?,
                language: value.language?,
                language_iso_code: value.language_iso_code?,
                last_name: value.last_name?,
                master_account_cid: value.master_account_cid?,
                middle_name: value.middle_name?,
                opt_out: value.opt_out?,
                pi_level: value.pi_level?,
                player_status: value.player_status?,
                real_cid: value.real_cid?,
                user_bio: value.user_bio?,
                user_flow_signature: value.user_flow_signature?,
                username: value.username?,
                verification_level: value.verification_level?,
                white_label: value.white_label?,
            })
        }
    }
    impl ::std::convert::From<super::PublicAggregatedInfoUser> for PublicAggregatedInfoUser {
        fn from(value: super::PublicAggregatedInfoUser) -> Self {
            Self {
                about_me: Ok(value.about_me),
                about_me_short: Ok(value.about_me_short),
                account_status: Ok(value.account_status),
                account_type: Ok(value.account_type),
                allow_display_full_name: Ok(value.allow_display_full_name),
                avatars: Ok(value.avatars),
                country: Ok(value.country),
                customer_restrictions: Ok(value.customer_restrictions),
                demo_cid: Ok(value.demo_cid),
                first_name: Ok(value.first_name),
                fund_type: Ok(value.fund_type),
                gcid: Ok(value.gcid),
                gdpr_info: Ok(value.gdpr_info),
                homepage: Ok(value.homepage),
                is_pi: Ok(value.is_pi),
                is_verified: Ok(value.is_verified),
                language: Ok(value.language),
                language_iso_code: Ok(value.language_iso_code),
                last_name: Ok(value.last_name),
                master_account_cid: Ok(value.master_account_cid),
                middle_name: Ok(value.middle_name),
                opt_out: Ok(value.opt_out),
                pi_level: Ok(value.pi_level),
                player_status: Ok(value.player_status),
                real_cid: Ok(value.real_cid),
                user_bio: Ok(value.user_bio),
                user_flow_signature: Ok(value.user_flow_signature),
                username: Ok(value.username),
                verification_level: Ok(value.verification_level),
                white_label: Ok(value.white_label),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicAggregatedInfoUserCustomerRestrictionsItem {
        cid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        occured: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        reason_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        restriction_type_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for PublicAggregatedInfoUserCustomerRestrictionsItem {
        fn default() -> Self {
            Self {
                cid: Ok(Default::default()),
                occured: Ok(Default::default()),
                reason_id: Ok(Default::default()),
                restriction_type_id: Ok(Default::default()),
            }
        }
    }
    impl PublicAggregatedInfoUserCustomerRestrictionsItem {
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
        pub fn occured<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.occured = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for occured: {e}"));
            self
        }
        pub fn reason_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason_id: {e}"));
            self
        }
        pub fn restriction_type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.restriction_type_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for restriction_type_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<PublicAggregatedInfoUserCustomerRestrictionsItem>
        for super::PublicAggregatedInfoUserCustomerRestrictionsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicAggregatedInfoUserCustomerRestrictionsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cid: value.cid?,
                occured: value.occured?,
                reason_id: value.reason_id?,
                restriction_type_id: value.restriction_type_id?,
            })
        }
    }
    impl ::std::convert::From<super::PublicAggregatedInfoUserCustomerRestrictionsItem>
        for PublicAggregatedInfoUserCustomerRestrictionsItem
    {
        fn from(value: super::PublicAggregatedInfoUserCustomerRestrictionsItem) -> Self {
            Self {
                cid: Ok(value.cid),
                occured: Ok(value.occured),
                reason_id: Ok(value.reason_id),
                restriction_type_id: Ok(value.restriction_type_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicAggregatedInfoUserGdprInfo {
        account_status: ::std::result::Result<
            ::std::option::Option<super::PublicAggregatedInfoAccountStatus>,
            ::std::string::String,
        >,
        player_status: ::std::result::Result<
            ::std::option::Option<super::PublicAggregatedInfoPlayerStatus>,
            ::std::string::String,
        >,
        player_status_reason: ::std::result::Result<
            ::std::option::Option<super::PublicAggregatedInfoPlayerStatusReason>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PublicAggregatedInfoUserGdprInfo {
        fn default() -> Self {
            Self {
                account_status: Ok(Default::default()),
                player_status: Ok(Default::default()),
                player_status_reason: Ok(Default::default()),
            }
        }
    }
    impl PublicAggregatedInfoUserGdprInfo {
        pub fn account_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PublicAggregatedInfoAccountStatus>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.account_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account_status: {e}"));
            self
        }
        pub fn player_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PublicAggregatedInfoPlayerStatus>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.player_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for player_status: {e}"));
            self
        }
        pub fn player_status_reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PublicAggregatedInfoPlayerStatusReason>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.player_status_reason = value.try_into().map_err(|e| {
                format!("error converting supplied value for player_status_reason: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<PublicAggregatedInfoUserGdprInfo>
        for super::PublicAggregatedInfoUserGdprInfo
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicAggregatedInfoUserGdprInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account_status: value.account_status?,
                player_status: value.player_status?,
                player_status_reason: value.player_status_reason?,
            })
        }
    }
    impl ::std::convert::From<super::PublicAggregatedInfoUserGdprInfo>
        for PublicAggregatedInfoUserGdprInfo
    {
        fn from(value: super::PublicAggregatedInfoUserGdprInfo) -> Self {
            Self {
                account_status: Ok(value.account_status),
                player_status: Ok(value.player_status),
                player_status_reason: Ok(value.player_status_reason),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct User {
        avatar:
            ::std::result::Result<::std::option::Option<super::UserAvatar>, ::std::string::String>,
        country_code: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        first_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        is_blocked: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_private: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        last_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pi_level: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        roles: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        username: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for User {
        fn default() -> Self {
            Self {
                avatar: Ok(Default::default()),
                country_code: Ok(Default::default()),
                first_name: Ok(Default::default()),
                id: Ok(Default::default()),
                is_blocked: Ok(Default::default()),
                is_private: Ok(Default::default()),
                last_name: Ok(Default::default()),
                pi_level: Ok(Default::default()),
                roles: Ok(Default::default()),
                username: Ok(Default::default()),
            }
        }
    }
    impl User {
        pub fn avatar<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UserAvatar>>,
            T::Error: ::std::fmt::Display,
        {
            self.avatar = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for avatar: {e}"));
            self
        }
        pub fn country_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.country_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for country_code: {e}"));
            self
        }
        pub fn first_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.first_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for first_name: {e}"));
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
        pub fn is_blocked<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_blocked = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_blocked: {e}"));
            self
        }
        pub fn is_private<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_private = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_private: {e}"));
            self
        }
        pub fn last_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_name: {e}"));
            self
        }
        pub fn pi_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.pi_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pi_level: {e}"));
            self
        }
        pub fn roles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.roles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for roles: {e}"));
            self
        }
        pub fn username<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.username = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for username: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<User> for super::User {
        type Error = super::error::ConversionError;
        fn try_from(value: User) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                avatar: value.avatar?,
                country_code: value.country_code?,
                first_name: value.first_name?,
                id: value.id?,
                is_blocked: value.is_blocked?,
                is_private: value.is_private?,
                last_name: value.last_name?,
                pi_level: value.pi_level?,
                roles: value.roles?,
                username: value.username?,
            })
        }
    }
    impl ::std::convert::From<super::User> for User {
        fn from(value: super::User) -> Self {
            Self {
                avatar: Ok(value.avatar),
                country_code: Ok(value.country_code),
                first_name: Ok(value.first_name),
                id: Ok(value.id),
                is_blocked: Ok(value.is_blocked),
                is_private: Ok(value.is_private),
                last_name: Ok(value.last_name),
                pi_level: Ok(value.pi_level),
                roles: Ok(value.roles),
                username: Ok(value.username),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UserAvatar {
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
    }
    impl ::std::default::Default for UserAvatar {
        fn default() -> Self {
            Self {
                large: Ok(Default::default()),
                medium: Ok(Default::default()),
                small: Ok(Default::default()),
            }
        }
    }
    impl UserAvatar {
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
    }
    impl ::std::convert::TryFrom<UserAvatar> for super::UserAvatar {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UserAvatar,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                large: value.large?,
                medium: value.medium?,
                small: value.small?,
            })
        }
    }
    impl ::std::convert::From<super::UserAvatar> for UserAvatar {
        fn from(value: super::UserAvatar) -> Self {
            Self {
                large: Ok(value.large),
                medium: Ok(value.medium),
                small: Ok(value.small),
            }
        }
    }
}
