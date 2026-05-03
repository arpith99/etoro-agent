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
#[doc = "`ArticleMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"aiSummary\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"body\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"bodyPreview\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"created\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"editStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/EditStatus\""]
#[doc = "    },"]
#[doc = "    \"featuredImage\": {"]
#[doc = "      \"$ref\": \"#/$defs/Attachment\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"languageCode\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ownerId\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"published\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"rating\": {"]
#[doc = "      \"$ref\": \"#/$defs/ArticleRating\""]
#[doc = "    },"]
#[doc = "    \"readingTimeMinutes\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/ArticleStatus\""]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Tag\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
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
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"wordCount\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ArticleMetadata {
    #[serde(
        rename = "aiSummary",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ai_summary: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub body: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "bodyPreview",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub body_preview: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "editStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub edit_status: ::std::option::Option<EditStatus>,
    #[serde(
        rename = "featuredImage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub featured_image: ::std::option::Option<Attachment>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<i32>,
    #[serde(
        rename = "languageCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ownerId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub owner_id: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub published: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rating: ::std::option::Option<ArticleRating>,
    #[serde(
        rename = "readingTimeMinutes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reading_time_minutes: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<ArticleStatus>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<::std::vec::Vec<Tag>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub updated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "wordCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub word_count: ::std::option::Option<i32>,
}
impl ::std::default::Default for ArticleMetadata {
    fn default() -> Self {
        Self {
            ai_summary: Default::default(),
            body: Default::default(),
            body_preview: Default::default(),
            created: Default::default(),
            edit_status: Default::default(),
            featured_image: Default::default(),
            id: Default::default(),
            language_code: Default::default(),
            owner_id: Default::default(),
            published: Default::default(),
            rating: Default::default(),
            reading_time_minutes: Default::default(),
            status: Default::default(),
            tags: Default::default(),
            title: Default::default(),
            updated: Default::default(),
            url: Default::default(),
            word_count: Default::default(),
        }
    }
}
impl ArticleMetadata {
    pub fn builder() -> builder::ArticleMetadata {
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
#[doc = "    \"Bearish\","]
#[doc = "    \"Bullish\""]
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
pub enum ArticleRating {
    Bearish,
    Bullish,
}
impl ::std::fmt::Display for ArticleRating {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bearish => f.write_str("Bearish"),
            Self::Bullish => f.write_str("Bullish"),
        }
    }
}
impl ::std::str::FromStr for ArticleRating {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Bearish" => Ok(Self::Bearish),
            "Bullish" => Ok(Self::Bullish),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ArticleRating {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ArticleRating {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ArticleRating {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "    \"Draft\","]
#[doc = "    \"Published\","]
#[doc = "    \"Deleted\""]
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
pub enum ArticleStatus {
    Draft,
    Published,
    Deleted,
}
impl ::std::fmt::Display for ArticleStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Draft => f.write_str("Draft"),
            Self::Published => f.write_str("Published"),
            Self::Deleted => f.write_str("Deleted"),
        }
    }
}
impl ::std::str::FromStr for ArticleStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Draft" => Ok(Self::Draft),
            "Published" => Ok(Self::Published),
            "Deleted" => Ok(Self::Deleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ArticleStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ArticleStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ArticleStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a media attachment in a post"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a media attachment in a post\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"host\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"media\": {"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"mediaType\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"None\","]
#[doc = "        \"Image\","]
#[doc = "        \"Video\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"duration\": {"]
#[doc = "          \"description\": \"Duration in seconds for video attachments\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"height\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"width\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"thumbnailUrl\": {"]
#[doc = "      \"description\": \"URL of a thumbnail image for video attachments\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"image\","]
#[doc = "        \"video\","]
#[doc = "        \"link\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Attachment {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub host: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub media: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "mediaType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub media_type: ::std::option::Option<AttachmentMediaType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<AttachmentMetadata>,
    #[doc = "URL of a thumbnail image for video attachments"]
    #[serde(
        rename = "thumbnailUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub thumbnail_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<AttachmentType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for Attachment {
    fn default() -> Self {
        Self {
            host: Default::default(),
            media: Default::default(),
            media_type: Default::default(),
            metadata: Default::default(),
            thumbnail_url: Default::default(),
            type_: Default::default(),
            url: Default::default(),
        }
    }
}
impl Attachment {
    pub fn builder() -> builder::Attachment {
        Default::default()
    }
}
#[doc = "`AttachmentMediaType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"None\","]
#[doc = "    \"Image\","]
#[doc = "    \"Video\""]
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
pub enum AttachmentMediaType {
    None,
    Image,
    Video,
}
impl ::std::fmt::Display for AttachmentMediaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::Image => f.write_str("Image"),
            Self::Video => f.write_str("Video"),
        }
    }
}
impl ::std::str::FromStr for AttachmentMediaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "Image" => Ok(Self::Image),
            "Video" => Ok(Self::Video),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AttachmentMediaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AttachmentMediaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AttachmentMediaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AttachmentMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"duration\": {"]
#[doc = "      \"description\": \"Duration in seconds for video attachments\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"height\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"width\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AttachmentMetadata {
    #[doc = "Duration in seconds for video attachments"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub duration: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<i64>,
}
impl ::std::default::Default for AttachmentMetadata {
    fn default() -> Self {
        Self {
            duration: Default::default(),
            height: Default::default(),
            width: Default::default(),
        }
    }
}
impl AttachmentMetadata {
    pub fn builder() -> builder::AttachmentMetadata {
        Default::default()
    }
}
#[doc = "`AttachmentType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"image\","]
#[doc = "    \"video\","]
#[doc = "    \"link\""]
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
pub enum AttachmentType {
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "link")]
    Link,
}
impl ::std::fmt::Display for AttachmentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Image => f.write_str("image"),
            Self::Video => f.write_str("video"),
            Self::Link => f.write_str("link"),
        }
    }
}
impl ::std::str::FromStr for AttachmentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image" => Ok(Self::Image),
            "video" => Ok(Self::Video),
            "link" => Ok(Self::Link),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AttachmentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AttachmentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AttachmentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "List of attachments for a post or comment"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"List of attachments for a post or comment\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"object\","]
#[doc = "    \"properties\": {"]
#[doc = "      \"description\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"host\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"media\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"image\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"properties\": {"]
#[doc = "              \"height\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"url\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"width\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"video\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"properties\": {"]
#[doc = "              \"image\": {"]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"properties\": {"]
#[doc = "                  \"height\": {"]
#[doc = "                    \"type\": \"integer\""]
#[doc = "                  },"]
#[doc = "                  \"url\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"width\": {"]
#[doc = "                    \"type\": \"integer\""]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"videoSource\": {"]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"None\","]
#[doc = "                  \"YouTube\","]
#[doc = "                  \"Vimeo\""]
#[doc = "                ]"]
#[doc = "              },"]
#[doc = "              \"videoSourceId\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"mediaType\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"enum\": ["]
#[doc = "          \"None\","]
#[doc = "          \"Image\","]
#[doc = "          \"Video\""]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"title\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"url\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Attachments(pub ::std::vec::Vec<AttachmentsItem>);
impl ::std::ops::Deref for Attachments {
    type Target = ::std::vec::Vec<AttachmentsItem>;
    fn deref(&self) -> &::std::vec::Vec<AttachmentsItem> {
        &self.0
    }
}
impl ::std::convert::From<Attachments> for ::std::vec::Vec<AttachmentsItem> {
    fn from(value: Attachments) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<AttachmentsItem>> for Attachments {
    fn from(value: ::std::vec::Vec<AttachmentsItem>) -> Self {
        Self(value)
    }
}
#[doc = "`AttachmentsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"host\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"media\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"image\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"height\": {"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"url\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"width\": {"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"video\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"image\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"properties\": {"]
#[doc = "                \"height\": {"]
#[doc = "                  \"type\": \"integer\""]
#[doc = "                },"]
#[doc = "                \"url\": {"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"width\": {"]
#[doc = "                  \"type\": \"integer\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"videoSource\": {"]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"None\","]
#[doc = "                \"YouTube\","]
#[doc = "                \"Vimeo\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"videoSourceId\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"mediaType\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"None\","]
#[doc = "        \"Image\","]
#[doc = "        \"Video\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
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
pub struct AttachmentsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub host: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub media: ::std::option::Option<AttachmentsItemMedia>,
    #[serde(
        rename = "mediaType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub media_type: ::std::option::Option<AttachmentsItemMediaType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for AttachmentsItem {
    fn default() -> Self {
        Self {
            description: Default::default(),
            host: Default::default(),
            media: Default::default(),
            media_type: Default::default(),
            title: Default::default(),
            url: Default::default(),
        }
    }
}
impl AttachmentsItem {
    pub fn builder() -> builder::AttachmentsItem {
        Default::default()
    }
}
#[doc = "`AttachmentsItemMedia`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"image\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"height\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"width\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"video\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"image\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"height\": {"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"url\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"width\": {"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"videoSource\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"None\","]
#[doc = "            \"YouTube\","]
#[doc = "            \"Vimeo\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"videoSourceId\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AttachmentsItemMedia {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub image: ::std::option::Option<AttachmentsItemMediaImage>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub video: ::std::option::Option<AttachmentsItemMediaVideo>,
}
impl ::std::default::Default for AttachmentsItemMedia {
    fn default() -> Self {
        Self {
            image: Default::default(),
            video: Default::default(),
        }
    }
}
impl AttachmentsItemMedia {
    pub fn builder() -> builder::AttachmentsItemMedia {
        Default::default()
    }
}
#[doc = "`AttachmentsItemMediaImage`"]
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
pub struct AttachmentsItemMediaImage {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<i64>,
}
impl ::std::default::Default for AttachmentsItemMediaImage {
    fn default() -> Self {
        Self {
            height: Default::default(),
            url: Default::default(),
            width: Default::default(),
        }
    }
}
impl AttachmentsItemMediaImage {
    pub fn builder() -> builder::AttachmentsItemMediaImage {
        Default::default()
    }
}
#[doc = "`AttachmentsItemMediaType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"None\","]
#[doc = "    \"Image\","]
#[doc = "    \"Video\""]
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
pub enum AttachmentsItemMediaType {
    None,
    Image,
    Video,
}
impl ::std::fmt::Display for AttachmentsItemMediaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::Image => f.write_str("Image"),
            Self::Video => f.write_str("Video"),
        }
    }
}
impl ::std::str::FromStr for AttachmentsItemMediaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "Image" => Ok(Self::Image),
            "Video" => Ok(Self::Video),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AttachmentsItemMediaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AttachmentsItemMediaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AttachmentsItemMediaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AttachmentsItemMediaVideo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"image\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"height\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"width\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"videoSource\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"None\","]
#[doc = "        \"YouTube\","]
#[doc = "        \"Vimeo\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"videoSourceId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AttachmentsItemMediaVideo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub image: ::std::option::Option<AttachmentsItemMediaVideoImage>,
    #[serde(
        rename = "videoSource",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub video_source: ::std::option::Option<AttachmentsItemMediaVideoVideoSource>,
    #[serde(
        rename = "videoSourceId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub video_source_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for AttachmentsItemMediaVideo {
    fn default() -> Self {
        Self {
            image: Default::default(),
            video_source: Default::default(),
            video_source_id: Default::default(),
        }
    }
}
impl AttachmentsItemMediaVideo {
    pub fn builder() -> builder::AttachmentsItemMediaVideo {
        Default::default()
    }
}
#[doc = "`AttachmentsItemMediaVideoImage`"]
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
pub struct AttachmentsItemMediaVideoImage {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<i64>,
}
impl ::std::default::Default for AttachmentsItemMediaVideoImage {
    fn default() -> Self {
        Self {
            height: Default::default(),
            url: Default::default(),
            width: Default::default(),
        }
    }
}
impl AttachmentsItemMediaVideoImage {
    pub fn builder() -> builder::AttachmentsItemMediaVideoImage {
        Default::default()
    }
}
#[doc = "`AttachmentsItemMediaVideoVideoSource`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"None\","]
#[doc = "    \"YouTube\","]
#[doc = "    \"Vimeo\""]
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
pub enum AttachmentsItemMediaVideoVideoSource {
    None,
    YouTube,
    Vimeo,
}
impl ::std::fmt::Display for AttachmentsItemMediaVideoVideoSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::YouTube => f.write_str("YouTube"),
            Self::Vimeo => f.write_str("Vimeo"),
        }
    }
}
impl ::std::str::FromStr for AttachmentsItemMediaVideoVideoSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "YouTube" => Ok(Self::YouTube),
            "Vimeo" => Ok(Self::Vimeo),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AttachmentsItemMediaVideoVideoSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AttachmentsItemMediaVideoVideoSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AttachmentsItemMediaVideoVideoSource {
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
#[doc = "`Comment`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"emotionsData\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"like\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"emotions\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"$ref\": \"#/$defs/Emotion\""]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"paging\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"properties\": {"]
#[doc = "                \"totalCount\": {"]
#[doc = "                  \"type\": \"integer\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"entity\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"attachments\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Attachment\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"created\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"date-time\""]
#[doc = "        },"]
#[doc = "        \"editStatus\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"None\","]
#[doc = "            \"Edited\","]
#[doc = "            \"Moderated\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"isSpam\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"message\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"languageCode\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"text\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"obsoleteId\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"owner\": {"]
#[doc = "          \"$ref\": \"#/$defs/User\""]
#[doc = "        },"]
#[doc = "        \"parent\": {"]
#[doc = "          \"description\": \"Parent post reference\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"id\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"obsoleteId\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"replies\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"repliesCount\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"requesterContext\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"isFlaggingAsSpam\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isFollowing\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isInteractionRestricted\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isLiking\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isPinned\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isRequesterBlocking\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isSaved\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isSubscribed\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Comment {
    #[serde(
        rename = "emotionsData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub emotions_data: ::std::option::Option<CommentEmotionsData>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub entity: ::std::option::Option<CommentEntity>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub replies: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(
        rename = "repliesCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub replies_count: ::std::option::Option<i64>,
    #[serde(
        rename = "requesterContext",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub requester_context: ::std::option::Option<CommentRequesterContext>,
}
impl ::std::default::Default for Comment {
    fn default() -> Self {
        Self {
            emotions_data: Default::default(),
            entity: Default::default(),
            replies: Default::default(),
            replies_count: Default::default(),
            requester_context: Default::default(),
        }
    }
}
impl Comment {
    pub fn builder() -> builder::Comment {
        Default::default()
    }
}
#[doc = "Request model for creating a new comment"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Request model for creating a new comment\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\","]
#[doc = "    \"owner\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"attachments\": {"]
#[doc = "      \"$ref\": \"#/$defs/Attachments\""]
#[doc = "    },"]
#[doc = "    \"mentions\": {"]
#[doc = "      \"$ref\": \"#/$defs/Mentions\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The text content of the comment\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"owner\": {"]
#[doc = "      \"description\": \"ID of the user creating the comment\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"$ref\": \"#/$defs/Tags\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommentCreateRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub attachments: ::std::option::Option<Attachments>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mentions: ::std::option::Option<Mentions>,
    #[doc = "The text content of the comment"]
    pub message: ::std::string::String,
    #[doc = "ID of the user creating the comment"]
    pub owner: i64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<Tags>,
}
impl CommentCreateRequest {
    pub fn builder() -> builder::CommentCreateRequest {
        Default::default()
    }
}
#[doc = "Recursive: replies is Vec<CommentDataResponse>. Box in Rust to break cycle."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Recursive: replies is Vec<CommentDataResponse>. Box in Rust to break cycle.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"badgedOwnerCountryId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"emotionsData\": {"]
#[doc = "      \"$ref\": \"#/$defs/EmotionsDataResponse\""]
#[doc = "    },"]
#[doc = "    \"entity\": {"]
#[doc = "      \"$ref\": \"#/$defs/Comment\""]
#[doc = "    },"]
#[doc = "    \"replies\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/CommentDataResponse\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"repliesCount\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"requesterContext\": {"]
#[doc = "      \"$ref\": \"#/$defs/RequesterContextResponse\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CommentDataResponse {
    #[serde(
        rename = "badgedOwnerCountryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub badged_owner_country_id: ::std::option::Option<i32>,
    #[serde(
        rename = "emotionsData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub emotions_data: ::std::option::Option<EmotionsDataResponse>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub entity: ::std::option::Option<Comment>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub replies: ::std::option::Option<::std::vec::Vec<CommentDataResponse>>,
    #[serde(
        rename = "repliesCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub replies_count: ::std::option::Option<i32>,
    #[serde(
        rename = "requesterContext",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub requester_context: ::std::option::Option<RequesterContextResponse>,
}
impl ::std::default::Default for CommentDataResponse {
    fn default() -> Self {
        Self {
            badged_owner_country_id: Default::default(),
            emotions_data: Default::default(),
            entity: Default::default(),
            replies: Default::default(),
            replies_count: Default::default(),
            requester_context: Default::default(),
        }
    }
}
impl CommentDataResponse {
    pub fn builder() -> builder::CommentDataResponse {
        Default::default()
    }
}
#[doc = "`CommentEmotionsData`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"like\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"emotions\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Emotion\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"paging\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"totalCount\": {"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommentEmotionsData {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub like: ::std::option::Option<CommentEmotionsDataLike>,
}
impl ::std::default::Default for CommentEmotionsData {
    fn default() -> Self {
        Self {
            like: Default::default(),
        }
    }
}
impl CommentEmotionsData {
    pub fn builder() -> builder::CommentEmotionsData {
        Default::default()
    }
}
#[doc = "`CommentEmotionsDataLike`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"emotions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Emotion\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"paging\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"totalCount\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommentEmotionsDataLike {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub emotions: ::std::vec::Vec<Emotion>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub paging: ::std::option::Option<CommentEmotionsDataLikePaging>,
}
impl ::std::default::Default for CommentEmotionsDataLike {
    fn default() -> Self {
        Self {
            emotions: Default::default(),
            paging: Default::default(),
        }
    }
}
impl CommentEmotionsDataLike {
    pub fn builder() -> builder::CommentEmotionsDataLike {
        Default::default()
    }
}
#[doc = "`CommentEmotionsDataLikePaging`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"totalCount\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommentEmotionsDataLikePaging {
    #[serde(
        rename = "totalCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_count: ::std::option::Option<i64>,
}
impl ::std::default::Default for CommentEmotionsDataLikePaging {
    fn default() -> Self {
        Self {
            total_count: Default::default(),
        }
    }
}
impl CommentEmotionsDataLikePaging {
    pub fn builder() -> builder::CommentEmotionsDataLikePaging {
        Default::default()
    }
}
#[doc = "`CommentEntity`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"attachments\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Attachment\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"created\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"editStatus\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"None\","]
#[doc = "        \"Edited\","]
#[doc = "        \"Moderated\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"isSpam\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"languageCode\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"obsoleteId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"owner\": {"]
#[doc = "      \"$ref\": \"#/$defs/User\""]
#[doc = "    },"]
#[doc = "    \"parent\": {"]
#[doc = "      \"description\": \"Parent post reference\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"obsoleteId\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommentEntity {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub attachments: ::std::vec::Vec<Attachment>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "editStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub edit_status: ::std::option::Option<CommentEntityEditStatus>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "isSpam",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_spam: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<CommentEntityMessage>,
    #[serde(
        rename = "obsoleteId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub obsolete_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<User>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub parent: ::std::option::Option<CommentEntityParent>,
}
impl ::std::default::Default for CommentEntity {
    fn default() -> Self {
        Self {
            attachments: Default::default(),
            created: Default::default(),
            edit_status: Default::default(),
            id: Default::default(),
            is_spam: Default::default(),
            message: Default::default(),
            obsolete_id: Default::default(),
            owner: Default::default(),
            parent: Default::default(),
        }
    }
}
impl CommentEntity {
    pub fn builder() -> builder::CommentEntity {
        Default::default()
    }
}
#[doc = "`CommentEntityEditStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"None\","]
#[doc = "    \"Edited\","]
#[doc = "    \"Moderated\""]
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
pub enum CommentEntityEditStatus {
    None,
    Edited,
    Moderated,
}
impl ::std::fmt::Display for CommentEntityEditStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::Edited => f.write_str("Edited"),
            Self::Moderated => f.write_str("Moderated"),
        }
    }
}
impl ::std::str::FromStr for CommentEntityEditStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "Edited" => Ok(Self::Edited),
            "Moderated" => Ok(Self::Moderated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CommentEntityEditStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CommentEntityEditStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CommentEntityEditStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CommentEntityMessage`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"languageCode\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommentEntityMessage {
    #[serde(
        rename = "languageCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for CommentEntityMessage {
    fn default() -> Self {
        Self {
            language_code: Default::default(),
            text: Default::default(),
        }
    }
}
impl CommentEntityMessage {
    pub fn builder() -> builder::CommentEntityMessage {
        Default::default()
    }
}
#[doc = "Parent post reference"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Parent post reference\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"obsoleteId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommentEntityParent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "obsoleteId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub obsolete_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for CommentEntityParent {
    fn default() -> Self {
        Self {
            id: Default::default(),
            obsolete_id: Default::default(),
            type_: Default::default(),
        }
    }
}
impl CommentEntityParent {
    pub fn builder() -> builder::CommentEntityParent {
        Default::default()
    }
}
#[doc = "`CommentRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"attachments\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Attachment\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"commentId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"mentions\": {"]
#[doc = "      \"$ref\": \"#/$defs/MentionsDataRequest\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"owner\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"postId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"replyId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"$ref\": \"#/$defs/TagsDataRequest\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CommentRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub attachments: ::std::option::Option<::std::vec::Vec<Attachment>>,
    #[serde(
        rename = "commentId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub comment_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mentions: ::std::option::Option<MentionsDataRequest>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<i32>,
    #[serde(
        rename = "postId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub post_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "replyId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reply_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<TagsDataRequest>,
}
impl ::std::default::Default for CommentRequest {
    fn default() -> Self {
        Self {
            attachments: Default::default(),
            comment_id: Default::default(),
            mentions: Default::default(),
            message: Default::default(),
            owner: Default::default(),
            post_id: Default::default(),
            reply_id: Default::default(),
            tags: Default::default(),
        }
    }
}
impl CommentRequest {
    pub fn builder() -> builder::CommentRequest {
        Default::default()
    }
}
#[doc = "`CommentRequesterContext`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"isFlaggingAsSpam\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isFollowing\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isInteractionRestricted\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isLiking\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isPinned\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isRequesterBlocking\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isSaved\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isSubscribed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommentRequesterContext {
    #[serde(
        rename = "isFlaggingAsSpam",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_flagging_as_spam: ::std::option::Option<bool>,
    #[serde(
        rename = "isFollowing",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_following: ::std::option::Option<bool>,
    #[serde(
        rename = "isInteractionRestricted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_interaction_restricted: ::std::option::Option<bool>,
    #[serde(
        rename = "isLiking",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_liking: ::std::option::Option<bool>,
    #[serde(
        rename = "isPinned",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_pinned: ::std::option::Option<bool>,
    #[serde(
        rename = "isRequesterBlocking",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_requester_blocking: ::std::option::Option<bool>,
    #[serde(
        rename = "isSaved",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_saved: ::std::option::Option<bool>,
    #[serde(
        rename = "isSubscribed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_subscribed: ::std::option::Option<bool>,
}
impl ::std::default::Default for CommentRequesterContext {
    fn default() -> Self {
        Self {
            is_flagging_as_spam: Default::default(),
            is_following: Default::default(),
            is_interaction_restricted: Default::default(),
            is_liking: Default::default(),
            is_pinned: Default::default(),
            is_requester_blocking: Default::default(),
            is_saved: Default::default(),
            is_subscribed: Default::default(),
        }
    }
}
impl CommentRequesterContext {
    pub fn builder() -> builder::CommentRequesterContext {
        Default::default()
    }
}
#[doc = "`CopyMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/CopyType\""]
#[doc = "    },"]
#[doc = "    \"user\": {"]
#[doc = "      \"$ref\": \"#/$defs/User\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CopyMetadata {
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<CopyType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<User>,
}
impl ::std::default::Default for CopyMetadata {
    fn default() -> Self {
        Self {
            type_: Default::default(),
            user: Default::default(),
        }
    }
}
impl CopyMetadata {
    pub fn builder() -> builder::CopyMetadata {
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
#[doc = "    \"Start\","]
#[doc = "    \"Stop\""]
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
pub enum CopyType {
    Start,
    Stop,
}
impl ::std::fmt::Display for CopyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Start => f.write_str("Start"),
            Self::Stop => f.write_str("Stop"),
        }
    }
}
impl ::std::str::FromStr for CopyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Start" => Ok(Self::Start),
            "Stop" => Ok(Self::Stop),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CopyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CopyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CopyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Discussion`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"commentsData\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"comments\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Comment\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"reactionPaging\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"totalCount\": {"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"emotionsData\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"like\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"emotions\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"$ref\": \"#/$defs/Emotion\""]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"paging\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"properties\": {"]
#[doc = "                \"totalCount\": {"]
#[doc = "                  \"type\": \"integer\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"post\": {"]
#[doc = "      \"$ref\": \"#/$defs/DiscussionsPost\""]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"description\": \"UNTYPED in spec - can be a string OR an object with sourceId/owner/type. Model with #[serde(untagged)] enum\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"requesterContext\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"isFlaggingAsSpam\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isFollowing\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isInteractionRestricted\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isLiking\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isPinned\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isRequesterBlocking\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isSaved\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"isSubscribed\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"sharedCount\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"totalCommentsAndReplies\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Discussion {
    #[serde(
        rename = "commentsData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub comments_data: ::std::option::Option<DiscussionCommentsData>,
    #[serde(
        rename = "emotionsData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub emotions_data: ::std::option::Option<DiscussionEmotionsData>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub post: ::std::option::Option<DiscussionsPost>,
    #[doc = "UNTYPED in spec - can be a string OR an object with sourceId/owner/type. Model with #[serde(untagged)] enum"]
    #[serde(default)]
    pub reason: (),
    #[serde(
        rename = "requesterContext",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub requester_context: ::std::option::Option<DiscussionRequesterContext>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<DiscussionSummary>,
}
impl ::std::default::Default for Discussion {
    fn default() -> Self {
        Self {
            comments_data: Default::default(),
            emotions_data: Default::default(),
            id: Default::default(),
            post: Default::default(),
            reason: Default::default(),
            requester_context: Default::default(),
            summary: Default::default(),
        }
    }
}
impl Discussion {
    pub fn builder() -> builder::Discussion {
        Default::default()
    }
}
#[doc = "`DiscussionCommentsData`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"comments\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Comment\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"reactionPaging\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"totalCount\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionCommentsData {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub comments: ::std::vec::Vec<Comment>,
    #[serde(
        rename = "reactionPaging",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reaction_paging: ::std::option::Option<DiscussionCommentsDataReactionPaging>,
}
impl ::std::default::Default for DiscussionCommentsData {
    fn default() -> Self {
        Self {
            comments: Default::default(),
            reaction_paging: Default::default(),
        }
    }
}
impl DiscussionCommentsData {
    pub fn builder() -> builder::DiscussionCommentsData {
        Default::default()
    }
}
#[doc = "`DiscussionCommentsDataReactionPaging`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"totalCount\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionCommentsDataReactionPaging {
    #[serde(
        rename = "totalCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_count: ::std::option::Option<i64>,
}
impl ::std::default::Default for DiscussionCommentsDataReactionPaging {
    fn default() -> Self {
        Self {
            total_count: Default::default(),
        }
    }
}
impl DiscussionCommentsDataReactionPaging {
    pub fn builder() -> builder::DiscussionCommentsDataReactionPaging {
        Default::default()
    }
}
#[doc = "Request model for creating a new discussion post"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Request model for creating a new discussion post\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"attachments\": {"]
#[doc = "      \"$ref\": \"#/$defs/Attachments\""]
#[doc = "    },"]
#[doc = "    \"mentions\": {"]
#[doc = "      \"$ref\": \"#/$defs/Mentions\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"owner\": {"]
#[doc = "      \"description\": \"ID of the owner creating the discussion\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"$ref\": \"#/$defs/Tags\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionCreateRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub attachments: ::std::option::Option<Attachments>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mentions: ::std::option::Option<Mentions>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    #[doc = "ID of the owner creating the discussion"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<Tags>,
}
impl ::std::default::Default for DiscussionCreateRequest {
    fn default() -> Self {
        Self {
            attachments: Default::default(),
            mentions: Default::default(),
            message: Default::default(),
            owner: Default::default(),
            tags: Default::default(),
        }
    }
}
impl DiscussionCreateRequest {
    pub fn builder() -> builder::DiscussionCreateRequest {
        Default::default()
    }
}
#[doc = "`DiscussionEmotionsData`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"like\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"emotions\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Emotion\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"paging\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"totalCount\": {"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionEmotionsData {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub like: ::std::option::Option<DiscussionEmotionsDataLike>,
}
impl ::std::default::Default for DiscussionEmotionsData {
    fn default() -> Self {
        Self {
            like: Default::default(),
        }
    }
}
impl DiscussionEmotionsData {
    pub fn builder() -> builder::DiscussionEmotionsData {
        Default::default()
    }
}
#[doc = "`DiscussionEmotionsDataLike`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"emotions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Emotion\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"paging\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"totalCount\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionEmotionsDataLike {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub emotions: ::std::vec::Vec<Emotion>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub paging: ::std::option::Option<DiscussionEmotionsDataLikePaging>,
}
impl ::std::default::Default for DiscussionEmotionsDataLike {
    fn default() -> Self {
        Self {
            emotions: Default::default(),
            paging: Default::default(),
        }
    }
}
impl DiscussionEmotionsDataLike {
    pub fn builder() -> builder::DiscussionEmotionsDataLike {
        Default::default()
    }
}
#[doc = "`DiscussionEmotionsDataLikePaging`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"totalCount\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionEmotionsDataLikePaging {
    #[serde(
        rename = "totalCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_count: ::std::option::Option<i64>,
}
impl ::std::default::Default for DiscussionEmotionsDataLikePaging {
    fn default() -> Self {
        Self {
            total_count: Default::default(),
        }
    }
}
impl DiscussionEmotionsDataLikePaging {
    pub fn builder() -> builder::DiscussionEmotionsDataLikePaging {
        Default::default()
    }
}
#[doc = "`DiscussionRequesterContext`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"isFlaggingAsSpam\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isFollowing\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isInteractionRestricted\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isLiking\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isPinned\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isRequesterBlocking\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isSaved\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isSubscribed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionRequesterContext {
    #[serde(
        rename = "isFlaggingAsSpam",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_flagging_as_spam: ::std::option::Option<bool>,
    #[serde(
        rename = "isFollowing",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_following: ::std::option::Option<bool>,
    #[serde(
        rename = "isInteractionRestricted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_interaction_restricted: ::std::option::Option<bool>,
    #[serde(
        rename = "isLiking",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_liking: ::std::option::Option<bool>,
    #[serde(
        rename = "isPinned",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_pinned: ::std::option::Option<bool>,
    #[serde(
        rename = "isRequesterBlocking",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_requester_blocking: ::std::option::Option<bool>,
    #[serde(
        rename = "isSaved",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_saved: ::std::option::Option<bool>,
    #[serde(
        rename = "isSubscribed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_subscribed: ::std::option::Option<bool>,
}
impl ::std::default::Default for DiscussionRequesterContext {
    fn default() -> Self {
        Self {
            is_flagging_as_spam: Default::default(),
            is_following: Default::default(),
            is_interaction_restricted: Default::default(),
            is_liking: Default::default(),
            is_pinned: Default::default(),
            is_requester_blocking: Default::default(),
            is_saved: Default::default(),
            is_subscribed: Default::default(),
        }
    }
}
impl DiscussionRequesterContext {
    pub fn builder() -> builder::DiscussionRequesterContext {
        Default::default()
    }
}
#[doc = "`DiscussionResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"badgedOwnerCountryId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"commentsData\": {"]
#[doc = "      \"$ref\": \"#/$defs/EntityCommentsDataResponse\""]
#[doc = "    },"]
#[doc = "    \"emotionsData\": {"]
#[doc = "      \"$ref\": \"#/$defs/EmotionsDataResponse\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"post\": {"]
#[doc = "      \"$ref\": \"#/$defs/Post\""]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"$ref\": \"#/$defs/Reason\""]
#[doc = "    },"]
#[doc = "    \"requesterContext\": {"]
#[doc = "      \"$ref\": \"#/$defs/RequesterContextResponse\""]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"$ref\": \"#/$defs/SummaryResponse\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DiscussionResponse {
    #[serde(
        rename = "badgedOwnerCountryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub badged_owner_country_id: ::std::option::Option<i32>,
    #[serde(
        rename = "commentsData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub comments_data: ::std::option::Option<EntityCommentsDataResponse>,
    #[serde(
        rename = "emotionsData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub emotions_data: ::std::option::Option<EmotionsDataResponse>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub post: ::std::option::Option<Post>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<Reason>,
    #[serde(
        rename = "requesterContext",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub requester_context: ::std::option::Option<RequesterContextResponse>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<SummaryResponse>,
}
impl ::std::default::Default for DiscussionResponse {
    fn default() -> Self {
        Self {
            badged_owner_country_id: Default::default(),
            comments_data: Default::default(),
            emotions_data: Default::default(),
            id: Default::default(),
            post: Default::default(),
            reason: Default::default(),
            requester_context: Default::default(),
            summary: Default::default(),
        }
    }
}
impl DiscussionResponse {
    pub fn builder() -> builder::DiscussionResponse {
        Default::default()
    }
}
#[doc = "`DiscussionSummary`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"sharedCount\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"totalCommentsAndReplies\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionSummary {
    #[serde(
        rename = "sharedCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub shared_count: ::std::option::Option<i64>,
    #[serde(
        rename = "totalCommentsAndReplies",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_comments_and_replies: ::std::option::Option<i64>,
}
impl ::std::default::Default for DiscussionSummary {
    fn default() -> Self {
        Self {
            shared_count: Default::default(),
            total_comments_and_replies: Default::default(),
        }
    }
}
impl DiscussionSummary {
    pub fn builder() -> builder::DiscussionSummary {
        Default::default()
    }
}
#[doc = "Older inline post shape used by Discussion (newer is Post)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Older inline post shape used by Discussion (newer is Post)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"attachments\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Attachment\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"created\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"editStatus\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"None\","]
#[doc = "        \"Edited\","]
#[doc = "        \"Moderated\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"isDeleted\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isSpam\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"mentions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"isDirect\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"user\": {"]
#[doc = "            \"$ref\": \"#/$defs/User\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"languageCode\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"poll\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"gcid\": {"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"id\": {"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"options\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"properties\": {"]
#[doc = "                  \"id\": {"]
#[doc = "                    \"type\": \"integer\""]
#[doc = "                  },"]
#[doc = "                  \"index\": {"]
#[doc = "                    \"type\": \"integer\""]
#[doc = "                  },"]
#[doc = "                  \"isUserVoted\": {"]
#[doc = "                    \"type\": \"boolean\""]
#[doc = "                  },"]
#[doc = "                  \"text\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"votesCount\": {"]
#[doc = "                    \"type\": \"integer\""]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"title\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"share\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"sharedOriginPost\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"sharedPost\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"obsoleteId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"owner\": {"]
#[doc = "      \"$ref\": \"#/$defs/User\""]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"market\": {"]
#[doc = "            \"description\": \"Inline Market shape — duplicated from standalone Market schema\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"properties\": {"]
#[doc = "              \"application\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"assetType\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"assetTypeId\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"assetTypeSubCategoryId\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"avatar\": {"]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"properties\": {"]
#[doc = "                  \"large\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"medium\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"small\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"svg\": {"]
#[doc = "                    \"type\": ["]
#[doc = "                      \"object\","]
#[doc = "                      \"null\""]
#[doc = "                    ],"]
#[doc = "                    \"properties\": {"]
#[doc = "                      \"backgroundColor\": {"]
#[doc = "                        \"type\": \"string\""]
#[doc = "                      },"]
#[doc = "                      \"textColor\": {"]
#[doc = "                        \"type\": \"string\""]
#[doc = "                      },"]
#[doc = "                      \"url\": {"]
#[doc = "                        \"type\": \"string\""]
#[doc = "                      }"]
#[doc = "                    }"]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"displayName\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"id\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"internalId\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"metadata\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"symbolName\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"updated\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"updated\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionsPost {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub attachments: ::std::vec::Vec<Attachment>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "editStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub edit_status: ::std::option::Option<DiscussionsPostEditStatus>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "isDeleted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_deleted: ::std::option::Option<bool>,
    #[serde(
        rename = "isSpam",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_spam: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub mentions: ::std::vec::Vec<DiscussionsPostMentionsItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<DiscussionsPostMessage>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<DiscussionsPostMetadata>,
    #[serde(
        rename = "obsoleteId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub obsolete_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<User>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tags: ::std::vec::Vec<DiscussionsPostTagsItem>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub updated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
}
impl ::std::default::Default for DiscussionsPost {
    fn default() -> Self {
        Self {
            attachments: Default::default(),
            created: Default::default(),
            edit_status: Default::default(),
            id: Default::default(),
            is_deleted: Default::default(),
            is_spam: Default::default(),
            mentions: Default::default(),
            message: Default::default(),
            metadata: Default::default(),
            obsolete_id: Default::default(),
            owner: Default::default(),
            tags: Default::default(),
            type_: Default::default(),
            updated: Default::default(),
        }
    }
}
impl DiscussionsPost {
    pub fn builder() -> builder::DiscussionsPost {
        Default::default()
    }
}
#[doc = "`DiscussionsPostEditStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"None\","]
#[doc = "    \"Edited\","]
#[doc = "    \"Moderated\""]
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
pub enum DiscussionsPostEditStatus {
    None,
    Edited,
    Moderated,
}
impl ::std::fmt::Display for DiscussionsPostEditStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::Edited => f.write_str("Edited"),
            Self::Moderated => f.write_str("Moderated"),
        }
    }
}
impl ::std::str::FromStr for DiscussionsPostEditStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "Edited" => Ok(Self::Edited),
            "Moderated" => Ok(Self::Moderated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DiscussionsPostEditStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DiscussionsPostEditStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DiscussionsPostEditStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DiscussionsPostMentionsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"isDirect\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"user\": {"]
#[doc = "      \"$ref\": \"#/$defs/User\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionsPostMentionsItem {
    #[serde(
        rename = "isDirect",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_direct: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<User>,
}
impl ::std::default::Default for DiscussionsPostMentionsItem {
    fn default() -> Self {
        Self {
            is_direct: Default::default(),
            user: Default::default(),
        }
    }
}
impl DiscussionsPostMentionsItem {
    pub fn builder() -> builder::DiscussionsPostMentionsItem {
        Default::default()
    }
}
#[doc = "`DiscussionsPostMessage`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"languageCode\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionsPostMessage {
    #[serde(
        rename = "languageCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for DiscussionsPostMessage {
    fn default() -> Self {
        Self {
            language_code: Default::default(),
            text: Default::default(),
        }
    }
}
impl DiscussionsPostMessage {
    pub fn builder() -> builder::DiscussionsPostMessage {
        Default::default()
    }
}
#[doc = "`DiscussionsPostMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"poll\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"gcid\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"options\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"properties\": {"]
#[doc = "              \"id\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"index\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              },"]
#[doc = "              \"isUserVoted\": {"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"text\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"votesCount\": {"]
#[doc = "                \"type\": \"integer\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"title\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"share\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"sharedOriginPost\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"sharedPost\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionsPostMetadata {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub poll: ::std::option::Option<DiscussionsPostMetadataPoll>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub share: ::std::option::Option<DiscussionsPostMetadataShare>,
}
impl ::std::default::Default for DiscussionsPostMetadata {
    fn default() -> Self {
        Self {
            poll: Default::default(),
            share: Default::default(),
        }
    }
}
impl DiscussionsPostMetadata {
    pub fn builder() -> builder::DiscussionsPostMetadata {
        Default::default()
    }
}
#[doc = "`DiscussionsPostMetadataPoll`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"gcid\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"options\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"id\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"index\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          },"]
#[doc = "          \"isUserVoted\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"text\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"votesCount\": {"]
#[doc = "            \"type\": \"integer\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionsPostMetadataPoll {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gcid: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub options: ::std::vec::Vec<DiscussionsPostMetadataPollOptionsItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for DiscussionsPostMetadataPoll {
    fn default() -> Self {
        Self {
            gcid: Default::default(),
            id: Default::default(),
            options: Default::default(),
            title: Default::default(),
        }
    }
}
impl DiscussionsPostMetadataPoll {
    pub fn builder() -> builder::DiscussionsPostMetadataPoll {
        Default::default()
    }
}
#[doc = "`DiscussionsPostMetadataPollOptionsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"index\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"isUserVoted\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"votesCount\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionsPostMetadataPollOptionsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub index: ::std::option::Option<i64>,
    #[serde(
        rename = "isUserVoted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_user_voted: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "votesCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub votes_count: ::std::option::Option<i64>,
}
impl ::std::default::Default for DiscussionsPostMetadataPollOptionsItem {
    fn default() -> Self {
        Self {
            id: Default::default(),
            index: Default::default(),
            is_user_voted: Default::default(),
            text: Default::default(),
            votes_count: Default::default(),
        }
    }
}
impl DiscussionsPostMetadataPollOptionsItem {
    pub fn builder() -> builder::DiscussionsPostMetadataPollOptionsItem {
        Default::default()
    }
}
#[doc = "`DiscussionsPostMetadataShare`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"sharedOriginPost\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"sharedPost\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionsPostMetadataShare {
    #[serde(
        rename = "sharedOriginPost",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub shared_origin_post: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "sharedPost",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub shared_post: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for DiscussionsPostMetadataShare {
    fn default() -> Self {
        Self {
            shared_origin_post: Default::default(),
            shared_post: Default::default(),
        }
    }
}
impl DiscussionsPostMetadataShare {
    pub fn builder() -> builder::DiscussionsPostMetadataShare {
        Default::default()
    }
}
#[doc = "`DiscussionsPostTagsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"market\": {"]
#[doc = "      \"description\": \"Inline Market shape — duplicated from standalone Market schema\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"application\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"assetType\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"assetTypeId\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"assetTypeSubCategoryId\": {"]
#[doc = "          \"type\": \"integer\""]
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
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"internalId\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"metadata\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"symbolName\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"updated\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionsPostTagsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub market: ::std::option::Option<DiscussionsPostTagsItemMarket>,
}
impl ::std::default::Default for DiscussionsPostTagsItem {
    fn default() -> Self {
        Self {
            market: Default::default(),
        }
    }
}
impl DiscussionsPostTagsItem {
    pub fn builder() -> builder::DiscussionsPostTagsItem {
        Default::default()
    }
}
#[doc = "Inline Market shape — duplicated from standalone Market schema"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Inline Market shape — duplicated from standalone Market schema\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"application\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"assetType\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"assetTypeId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"assetTypeSubCategoryId\": {"]
#[doc = "      \"type\": \"integer\""]
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
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"internalId\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"symbolName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"updated\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionsPostTagsItemMarket {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub application: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "assetType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub asset_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "assetTypeId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub asset_type_id: ::std::option::Option<i64>,
    #[serde(
        rename = "assetTypeSubCategoryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub asset_type_sub_category_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<DiscussionsPostTagsItemMarketAvatar>,
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
    pub internal_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "symbolName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub symbol_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub updated: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for DiscussionsPostTagsItemMarket {
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
impl DiscussionsPostTagsItemMarket {
    pub fn builder() -> builder::DiscussionsPostTagsItemMarket {
        Default::default()
    }
}
#[doc = "`DiscussionsPostTagsItemMarketAvatar`"]
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
pub struct DiscussionsPostTagsItemMarketAvatar {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub large: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub medium: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub small: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub svg: ::std::option::Option<DiscussionsPostTagsItemMarketAvatarSvg>,
}
impl ::std::default::Default for DiscussionsPostTagsItemMarketAvatar {
    fn default() -> Self {
        Self {
            large: Default::default(),
            medium: Default::default(),
            small: Default::default(),
            svg: Default::default(),
        }
    }
}
impl DiscussionsPostTagsItemMarketAvatar {
    pub fn builder() -> builder::DiscussionsPostTagsItemMarketAvatar {
        Default::default()
    }
}
#[doc = "`DiscussionsPostTagsItemMarketAvatarSvg`"]
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
pub struct DiscussionsPostTagsItemMarketAvatarSvg {
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
impl ::std::default::Default for DiscussionsPostTagsItemMarketAvatarSvg {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            text_color: Default::default(),
            url: Default::default(),
        }
    }
}
impl DiscussionsPostTagsItemMarketAvatarSvg {
    pub fn builder() -> builder::DiscussionsPostTagsItemMarketAvatarSvg {
        Default::default()
    }
}
#[doc = "Older response shape — newer is FeedResponse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Older response shape — newer is FeedResponse\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"discussions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Discussion\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"designatedStreamType\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"experimentName\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"streamType\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"paging\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"next\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"offSet\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"take\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"version\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionsResponse {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub discussions: ::std::vec::Vec<Discussion>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<DiscussionsResponseMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub paging: ::std::option::Option<DiscussionsResponsePaging>,
}
impl ::std::default::Default for DiscussionsResponse {
    fn default() -> Self {
        Self {
            discussions: Default::default(),
            metadata: Default::default(),
            paging: Default::default(),
        }
    }
}
impl DiscussionsResponse {
    pub fn builder() -> builder::DiscussionsResponse {
        Default::default()
    }
}
#[doc = "`DiscussionsResponseMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"designatedStreamType\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"experimentName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"streamType\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionsResponseMetadata {
    #[serde(
        rename = "designatedStreamType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub designated_stream_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "experimentName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub experiment_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "streamType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stream_type: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for DiscussionsResponseMetadata {
    fn default() -> Self {
        Self {
            designated_stream_type: Default::default(),
            experiment_name: Default::default(),
            stream_type: Default::default(),
        }
    }
}
impl DiscussionsResponseMetadata {
    pub fn builder() -> builder::DiscussionsResponseMetadata {
        Default::default()
    }
}
#[doc = "`DiscussionsResponsePaging`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"next\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"offSet\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"take\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DiscussionsResponsePaging {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub next: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "offSet",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub off_set: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub take: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for DiscussionsResponsePaging {
    fn default() -> Self {
        Self {
            next: Default::default(),
            off_set: Default::default(),
            take: Default::default(),
            version: Default::default(),
        }
    }
}
impl DiscussionsResponsePaging {
    pub fn builder() -> builder::DiscussionsResponsePaging {
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
#[doc = "    \"None\","]
#[doc = "    \"Edited\","]
#[doc = "    \"Moderated\""]
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
pub enum EditStatus {
    None,
    Edited,
    Moderated,
}
impl ::std::fmt::Display for EditStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::Edited => f.write_str("Edited"),
            Self::Moderated => f.write_str("Moderated"),
        }
    }
}
impl ::std::str::FromStr for EditStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "Edited" => Ok(Self::Edited),
            "Moderated" => Ok(Self::Moderated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EditStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EditStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EditStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Emotion`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"created\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"obsoleteId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"owner\": {"]
#[doc = "      \"$ref\": \"#/$defs/User\""]
#[doc = "    },"]
#[doc = "    \"parent\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"obsoleteId\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Emotion {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "obsoleteId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub obsolete_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<User>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub parent: ::std::option::Option<EmotionParent>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for Emotion {
    fn default() -> Self {
        Self {
            created: Default::default(),
            id: Default::default(),
            obsolete_id: Default::default(),
            owner: Default::default(),
            parent: Default::default(),
            type_: Default::default(),
        }
    }
}
impl Emotion {
    pub fn builder() -> builder::Emotion {
        Default::default()
    }
}
#[doc = "`EmotionDataResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"emotions\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Emotion\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"paging\": {"]
#[doc = "      \"$ref\": \"#/$defs/ReactionPagingResponse\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EmotionDataResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub emotions: ::std::option::Option<::std::vec::Vec<Emotion>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub paging: ::std::option::Option<ReactionPagingResponse>,
}
impl ::std::default::Default for EmotionDataResponse {
    fn default() -> Self {
        Self {
            emotions: Default::default(),
            paging: Default::default(),
        }
    }
}
impl EmotionDataResponse {
    pub fn builder() -> builder::EmotionDataResponse {
        Default::default()
    }
}
#[doc = "`EmotionParent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"obsoleteId\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EmotionParent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "obsoleteId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub obsolete_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for EmotionParent {
    fn default() -> Self {
        Self {
            id: Default::default(),
            obsolete_id: Default::default(),
            type_: Default::default(),
        }
    }
}
impl EmotionParent {
    pub fn builder() -> builder::EmotionParent {
        Default::default()
    }
}
#[doc = "Integer-encoded enum (only one variant currently)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Integer-encoded enum (only one variant currently)\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"int32\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Like\""]
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
pub enum EmotionType {
    Like,
}
impl ::std::fmt::Display for EmotionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Like => f.write_str("Like"),
        }
    }
}
impl ::std::str::FromStr for EmotionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Like" => Ok(Self::Like),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EmotionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EmotionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EmotionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`EmotionsDataResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"like\": {"]
#[doc = "      \"$ref\": \"#/$defs/EmotionDataResponse\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EmotionsDataResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub like: ::std::option::Option<EmotionDataResponse>,
}
impl ::std::default::Default for EmotionsDataResponse {
    fn default() -> Self {
        Self {
            like: Default::default(),
        }
    }
}
impl EmotionsDataResponse {
    pub fn builder() -> builder::EmotionsDataResponse {
        Default::default()
    }
}
#[doc = "`EntityCommentsDataResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"comments\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/CommentDataResponse\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"reactionPaging\": {"]
#[doc = "      \"$ref\": \"#/$defs/ReactionPagingResponse\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EntityCommentsDataResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub comments: ::std::option::Option<::std::vec::Vec<CommentDataResponse>>,
    #[serde(
        rename = "reactionPaging",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reaction_paging: ::std::option::Option<ReactionPagingResponse>,
}
impl ::std::default::Default for EntityCommentsDataResponse {
    fn default() -> Self {
        Self {
            comments: Default::default(),
            reaction_paging: Default::default(),
        }
    }
}
impl EntityCommentsDataResponse {
    pub fn builder() -> builder::EntityCommentsDataResponse {
        Default::default()
    }
}
#[doc = "`FeedMetadataResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"designatedStreamType\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"experimentName\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"streamType\": {"]
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
pub struct FeedMetadataResponse {
    #[serde(
        rename = "designatedStreamType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub designated_stream_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "experimentName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub experiment_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "streamType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stream_type: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for FeedMetadataResponse {
    fn default() -> Self {
        Self {
            designated_stream_type: Default::default(),
            experiment_name: Default::default(),
            stream_type: Default::default(),
        }
    }
}
impl FeedMetadataResponse {
    pub fn builder() -> builder::FeedMetadataResponse {
        Default::default()
    }
}
#[doc = "`FeedPagingResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"next\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"offSet\": {"]
#[doc = "      \"description\": \"Note 'offSet' (camelCase capital S) — sic\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"take\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
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
pub struct FeedPagingResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub next: ::std::option::Option<::std::string::String>,
    #[doc = "Note 'offSet' (camelCase capital S) — sic"]
    #[serde(
        rename = "offSet",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub off_set: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub take: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for FeedPagingResponse {
    fn default() -> Self {
        Self {
            next: Default::default(),
            off_set: Default::default(),
            take: Default::default(),
            version: Default::default(),
        }
    }
}
impl FeedPagingResponse {
    pub fn builder() -> builder::FeedPagingResponse {
        Default::default()
    }
}
#[doc = "`FeedResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"discussions\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/DiscussionResponse\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"$ref\": \"#/$defs/FeedMetadataResponse\""]
#[doc = "    },"]
#[doc = "    \"paging\": {"]
#[doc = "      \"$ref\": \"#/$defs/FeedPagingResponse\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FeedResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub discussions: ::std::option::Option<::std::vec::Vec<DiscussionResponse>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<FeedMetadataResponse>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub paging: ::std::option::Option<FeedPagingResponse>,
}
impl ::std::default::Default for FeedResponse {
    fn default() -> Self {
        Self {
            discussions: Default::default(),
            metadata: Default::default(),
            paging: Default::default(),
        }
    }
}
impl FeedResponse {
    pub fn builder() -> builder::FeedResponse {
        Default::default()
    }
}
#[doc = "`ImageMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"height\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"width\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ImageMetadata {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<i32>,
}
impl ::std::default::Default for ImageMetadata {
    fn default() -> Self {
        Self {
            height: Default::default(),
            url: Default::default(),
            width: Default::default(),
        }
    }
}
impl ImageMetadata {
    pub fn builder() -> builder::ImageMetadata {
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
#[doc = "`Media`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"image\": {"]
#[doc = "      \"$ref\": \"#/$defs/ImageMetadata\""]
#[doc = "    },"]
#[doc = "    \"video\": {"]
#[doc = "      \"$ref\": \"#/$defs/VideoMetadata\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Media {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub image: ::std::option::Option<ImageMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub video: ::std::option::Option<VideoMetadata>,
}
impl ::std::default::Default for Media {
    fn default() -> Self {
        Self {
            image: Default::default(),
            video: Default::default(),
        }
    }
}
impl Media {
    pub fn builder() -> builder::Media {
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
#[doc = "    \"None\","]
#[doc = "    \"Link\","]
#[doc = "    \"Image\","]
#[doc = "    \"Video\""]
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
pub enum MediaType {
    None,
    Link,
    Image,
    Video,
}
impl ::std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::Link => f.write_str("Link"),
            Self::Image => f.write_str("Image"),
            Self::Video => f.write_str("Video"),
        }
    }
}
impl ::std::str::FromStr for MediaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "Link" => Ok(Self::Link),
            "Image" => Ok(Self::Image),
            "Video" => Ok(Self::Video),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MediaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MediaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MediaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Mention`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"isDirect\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"user\": {"]
#[doc = "      \"$ref\": \"#/$defs/User\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Mention {
    #[serde(
        rename = "isDirect",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_direct: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<User>,
}
impl ::std::default::Default for Mention {
    fn default() -> Self {
        Self {
            is_direct: Default::default(),
            user: Default::default(),
        }
    }
}
impl Mention {
    pub fn builder() -> builder::Mention {
        Default::default()
    }
}
#[doc = "Mentions included in a post or comment"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Mentions included in a post or comment\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"mentions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"id\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"isDirect\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"userName\": {"]
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
pub struct Mentions {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub mentions: ::std::vec::Vec<MentionsMentionsItem>,
}
impl ::std::default::Default for Mentions {
    fn default() -> Self {
        Self {
            mentions: Default::default(),
        }
    }
}
impl Mentions {
    pub fn builder() -> builder::Mentions {
        Default::default()
    }
}
#[doc = "`MentionsDataRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"mentions\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/MentionsRequest\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MentionsDataRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mentions: ::std::option::Option<::std::vec::Vec<MentionsRequest>>,
}
impl ::std::default::Default for MentionsDataRequest {
    fn default() -> Self {
        Self {
            mentions: Default::default(),
        }
    }
}
impl MentionsDataRequest {
    pub fn builder() -> builder::MentionsDataRequest {
        Default::default()
    }
}
#[doc = "`MentionsMentionsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"isDirect\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"userName\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MentionsMentionsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "isDirect",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_direct: ::std::option::Option<bool>,
    #[serde(
        rename = "userName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for MentionsMentionsItem {
    fn default() -> Self {
        Self {
            id: Default::default(),
            is_direct: Default::default(),
            user_name: Default::default(),
        }
    }
}
impl MentionsMentionsItem {
    pub fn builder() -> builder::MentionsMentionsItem {
        Default::default()
    }
}
#[doc = "`MentionsRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"isDirect\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"userName\": {"]
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
pub struct MentionsRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "isDirect",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_direct: ::std::option::Option<bool>,
    #[serde(
        rename = "userName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for MentionsRequest {
    fn default() -> Self {
        Self {
            id: Default::default(),
            is_direct: Default::default(),
            user_name: Default::default(),
        }
    }
}
impl MentionsRequest {
    pub fn builder() -> builder::MentionsRequest {
        Default::default()
    }
}
#[doc = "`Message`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"languageCode\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"text\": {"]
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
pub struct Message {
    #[serde(
        rename = "languageCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for Message {
    fn default() -> Self {
        Self {
            language_code: Default::default(),
            text: Default::default(),
        }
    }
}
impl Message {
    pub fn builder() -> builder::Message {
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
#[doc = "    \"Unknown\","]
#[doc = "    \"Post\","]
#[doc = "    \"Comment\","]
#[doc = "    \"Reply\""]
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
pub enum ParentType {
    Unknown,
    Post,
    Comment,
    Reply,
}
impl ::std::fmt::Display for ParentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("Unknown"),
            Self::Post => f.write_str("Post"),
            Self::Comment => f.write_str("Comment"),
            Self::Reply => f.write_str("Reply"),
        }
    }
}
impl ::std::str::FromStr for ParentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Unknown" => Ok(Self::Unknown),
            "Post" => Ok(Self::Post),
            "Comment" => Ok(Self::Comment),
            "Reply" => Ok(Self::Reply),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ParentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ParentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ParentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Recursive: parent is Parents (self). Box in Rust to break cycle."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Recursive: parent is Parents (self). Box in Rust to break cycle.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"obsoleteId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parent\": {"]
#[doc = "      \"$ref\": \"#/$defs/Parents\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/ParentType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Parents {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "obsoleteId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub obsolete_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub parent: ::std::option::Option<::std::boxed::Box<Parents>>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<ParentType>,
}
impl ::std::default::Default for Parents {
    fn default() -> Self {
        Self {
            id: Default::default(),
            obsolete_id: Default::default(),
            parent: Default::default(),
            type_: Default::default(),
        }
    }
}
impl Parents {
    pub fn builder() -> builder::Parents {
        Default::default()
    }
}
#[doc = "`PollMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"gcid\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"options\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/PollOption\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"totalVotes\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PollMetadata {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gcid: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub options: ::std::option::Option<::std::vec::Vec<PollOption>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "totalVotes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_votes: ::std::option::Option<i32>,
}
impl ::std::default::Default for PollMetadata {
    fn default() -> Self {
        Self {
            gcid: Default::default(),
            id: Default::default(),
            options: Default::default(),
            title: Default::default(),
            total_votes: Default::default(),
        }
    }
}
impl PollMetadata {
    pub fn builder() -> builder::PollMetadata {
        Default::default()
    }
}
#[doc = "`PollOption`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"index\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"isUserVoted\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"votesCount\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PollOption {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub index: ::std::option::Option<i32>,
    #[serde(
        rename = "isUserVoted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_user_voted: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "votesCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub votes_count: ::std::option::Option<i32>,
}
impl ::std::default::Default for PollOption {
    fn default() -> Self {
        Self {
            id: Default::default(),
            index: Default::default(),
            is_user_voted: Default::default(),
            text: Default::default(),
            votes_count: Default::default(),
        }
    }
}
impl PollOption {
    pub fn builder() -> builder::PollOption {
        Default::default()
    }
}
#[doc = "Represents a feed post with its content and metadata"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a feed post with its content and metadata\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"attachments\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Attachment\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"created\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"editStatus\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"None\","]
#[doc = "        \"Edited\","]
#[doc = "        \"Moderated\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"isDeleted\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isSpam\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"mentions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"isDirect\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"user\": {"]
#[doc = "            \"$ref\": \"#/$defs/User\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"languageCode\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"$ref\": \"#/$defs/PostMetadata\""]
#[doc = "    },"]
#[doc = "    \"obsoleteId\": {"]
#[doc = "      \"description\": \"Obsolete identifier for backward compatibility\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"owner\": {"]
#[doc = "      \"$ref\": \"#/$defs/User\""]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"market\": {"]
#[doc = "            \"$ref\": \"#/$defs/Market\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/PostType\""]
#[doc = "    },"]
#[doc = "    \"updated\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Post {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub attachments: ::std::vec::Vec<Attachment>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "editStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub edit_status: ::std::option::Option<PostEditStatus>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "isDeleted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_deleted: ::std::option::Option<bool>,
    #[serde(
        rename = "isSpam",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_spam: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub mentions: ::std::vec::Vec<PostMentionsItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<PostMessage>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<PostMetadata>,
    #[doc = "Obsolete identifier for backward compatibility"]
    #[serde(
        rename = "obsoleteId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub obsolete_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<User>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tags: ::std::vec::Vec<PostTagsItem>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<PostType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub updated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
}
impl ::std::default::Default for Post {
    fn default() -> Self {
        Self {
            attachments: Default::default(),
            created: Default::default(),
            edit_status: Default::default(),
            id: Default::default(),
            is_deleted: Default::default(),
            is_spam: Default::default(),
            mentions: Default::default(),
            message: Default::default(),
            metadata: Default::default(),
            obsolete_id: Default::default(),
            owner: Default::default(),
            tags: Default::default(),
            type_: Default::default(),
            updated: Default::default(),
        }
    }
}
impl Post {
    pub fn builder() -> builder::Post {
        Default::default()
    }
}
#[doc = "`PostEditStatus`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"None\","]
#[doc = "    \"Edited\","]
#[doc = "    \"Moderated\""]
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
pub enum PostEditStatus {
    None,
    Edited,
    Moderated,
}
impl ::std::fmt::Display for PostEditStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::Edited => f.write_str("Edited"),
            Self::Moderated => f.write_str("Moderated"),
        }
    }
}
impl ::std::str::FromStr for PostEditStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "Edited" => Ok(Self::Edited),
            "Moderated" => Ok(Self::Moderated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PostEditStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PostEditStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PostEditStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PostMentionsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"isDirect\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"user\": {"]
#[doc = "      \"$ref\": \"#/$defs/User\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PostMentionsItem {
    #[serde(
        rename = "isDirect",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_direct: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<User>,
}
impl ::std::default::Default for PostMentionsItem {
    fn default() -> Self {
        Self {
            is_direct: Default::default(),
            user: Default::default(),
        }
    }
}
impl PostMentionsItem {
    pub fn builder() -> builder::PostMentionsItem {
        Default::default()
    }
}
#[doc = "`PostMessage`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"languageCode\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PostMessage {
    #[serde(
        rename = "languageCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PostMessage {
    fn default() -> Self {
        Self {
            language_code: Default::default(),
            text: Default::default(),
        }
    }
}
impl PostMessage {
    pub fn builder() -> builder::PostMessage {
        Default::default()
    }
}
#[doc = "`PostMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"article\": {"]
#[doc = "      \"$ref\": \"#/$defs/ArticleMetadata\""]
#[doc = "    },"]
#[doc = "    \"copy\": {"]
#[doc = "      \"$ref\": \"#/$defs/CopyMetadata\""]
#[doc = "    },"]
#[doc = "    \"marketEvent\": {"]
#[doc = "      \"$ref\": \"#/$defs/MarketEventMetadata\""]
#[doc = "    },"]
#[doc = "    \"order\": {"]
#[doc = "      \"$ref\": \"#/$defs/OrderMetadata\""]
#[doc = "    },"]
#[doc = "    \"poll\": {"]
#[doc = "      \"$ref\": \"#/$defs/PollMetadata\""]
#[doc = "    },"]
#[doc = "    \"share\": {"]
#[doc = "      \"$ref\": \"#/$defs/ShareMetadata\""]
#[doc = "    },"]
#[doc = "    \"trade\": {"]
#[doc = "      \"$ref\": \"#/$defs/TradeMetadata\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PostMetadata {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub article: ::std::option::Option<ArticleMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub copy: ::std::option::Option<CopyMetadata>,
    #[serde(
        rename = "marketEvent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub market_event: ::std::option::Option<MarketEventMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<OrderMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub poll: ::std::option::Option<PollMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub share: ::std::option::Option<ShareMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trade: ::std::option::Option<TradeMetadata>,
}
impl ::std::default::Default for PostMetadata {
    fn default() -> Self {
        Self {
            article: Default::default(),
            copy: Default::default(),
            market_event: Default::default(),
            order: Default::default(),
            poll: Default::default(),
            share: Default::default(),
            trade: Default::default(),
        }
    }
}
impl PostMetadata {
    pub fn builder() -> builder::PostMetadata {
        Default::default()
    }
}
#[doc = "`PostTagsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"market\": {"]
#[doc = "      \"$ref\": \"#/$defs/Market\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PostTagsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub market: ::std::option::Option<Market>,
}
impl ::std::default::Default for PostTagsItem {
    fn default() -> Self {
        Self {
            market: Default::default(),
        }
    }
}
impl PostTagsItem {
    pub fn builder() -> builder::PostTagsItem {
        Default::default()
    }
}
#[doc = "Integer-encoded enum (8 variants)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Integer-encoded enum (8 variants)\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"int32\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Default\","]
#[doc = "    \"Share\","]
#[doc = "    \"MarketEvent\","]
#[doc = "    \"Trade\","]
#[doc = "    \"Order\","]
#[doc = "    \"Copy\","]
#[doc = "    \"Poll\","]
#[doc = "    \"Article\""]
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
pub enum PostType {
    Default,
    Share,
    MarketEvent,
    Trade,
    Order,
    Copy,
    Poll,
    Article,
}
impl ::std::fmt::Display for PostType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Default => f.write_str("Default"),
            Self::Share => f.write_str("Share"),
            Self::MarketEvent => f.write_str("MarketEvent"),
            Self::Trade => f.write_str("Trade"),
            Self::Order => f.write_str("Order"),
            Self::Copy => f.write_str("Copy"),
            Self::Poll => f.write_str("Poll"),
            Self::Article => f.write_str("Article"),
        }
    }
}
impl ::std::str::FromStr for PostType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Default" => Ok(Self::Default),
            "Share" => Ok(Self::Share),
            "MarketEvent" => Ok(Self::MarketEvent),
            "Trade" => Ok(Self::Trade),
            "Order" => Ok(Self::Order),
            "Copy" => Ok(Self::Copy),
            "Poll" => Ok(Self::Poll),
            "Article" => Ok(Self::Article),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PostType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PostType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PostType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ReactionPagingResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"next\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"offsetEntityId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"totalCount\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ReactionPagingResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub next: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "offsetEntityId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub offset_entity_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "totalCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_count: ::std::option::Option<i32>,
}
impl ::std::default::Default for ReactionPagingResponse {
    fn default() -> Self {
        Self {
            next: Default::default(),
            offset_entity_id: Default::default(),
            total_count: Default::default(),
        }
    }
}
impl ReactionPagingResponse {
    pub fn builder() -> builder::ReactionPagingResponse {
        Default::default()
    }
}
#[doc = "`Reason`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"owner\": {"]
#[doc = "      \"$ref\": \"#/$defs/User\""]
#[doc = "    },"]
#[doc = "    \"sourceId\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/ReasonType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Reason {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<User>,
    #[serde(
        rename = "sourceId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub source_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<ReasonType>,
}
impl ::std::default::Default for Reason {
    fn default() -> Self {
        Self {
            owner: Default::default(),
            source_id: Default::default(),
            type_: Default::default(),
        }
    }
}
impl Reason {
    pub fn builder() -> builder::Reason {
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
#[doc = "    \"None\","]
#[doc = "    \"Owner\","]
#[doc = "    \"LikedPost\","]
#[doc = "    \"LikedComment\","]
#[doc = "    \"TaggedInPost\","]
#[doc = "    \"TaggedInComment\","]
#[doc = "    \"Comment\""]
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
pub enum ReasonType {
    None,
    Owner,
    LikedPost,
    LikedComment,
    TaggedInPost,
    TaggedInComment,
    Comment,
}
impl ::std::fmt::Display for ReasonType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::Owner => f.write_str("Owner"),
            Self::LikedPost => f.write_str("LikedPost"),
            Self::LikedComment => f.write_str("LikedComment"),
            Self::TaggedInPost => f.write_str("TaggedInPost"),
            Self::TaggedInComment => f.write_str("TaggedInComment"),
            Self::Comment => f.write_str("Comment"),
        }
    }
}
impl ::std::str::FromStr for ReasonType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "Owner" => Ok(Self::Owner),
            "LikedPost" => Ok(Self::LikedPost),
            "LikedComment" => Ok(Self::LikedComment),
            "TaggedInPost" => Ok(Self::TaggedInPost),
            "TaggedInComment" => Ok(Self::TaggedInComment),
            "Comment" => Ok(Self::Comment),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReasonType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ReasonType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReasonType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Common request shape — CommentRequest and DiscussionCreateRequest extend this conceptually"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Common request shape — CommentRequest and DiscussionCreateRequest extend this conceptually\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"attachments\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Attachment\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"mentions\": {"]
#[doc = "      \"$ref\": \"#/$defs/MentionsDataRequest\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"owner\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"$ref\": \"#/$defs/TagsDataRequest\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RequestBase {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub attachments: ::std::option::Option<::std::vec::Vec<Attachment>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mentions: ::std::option::Option<MentionsDataRequest>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<TagsDataRequest>,
}
impl ::std::default::Default for RequestBase {
    fn default() -> Self {
        Self {
            attachments: Default::default(),
            mentions: Default::default(),
            message: Default::default(),
            owner: Default::default(),
            tags: Default::default(),
        }
    }
}
impl RequestBase {
    pub fn builder() -> builder::RequestBase {
        Default::default()
    }
}
#[doc = "`RequesterContextResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"isFlaggingAsSpam\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isFollowing\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isInteractionRestricted\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isLiking\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isOwner\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isPinned\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isRequesterBlocking\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isSaved\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"isSubscribed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RequesterContextResponse {
    #[serde(
        rename = "isFlaggingAsSpam",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_flagging_as_spam: ::std::option::Option<bool>,
    #[serde(
        rename = "isFollowing",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_following: ::std::option::Option<bool>,
    #[serde(
        rename = "isInteractionRestricted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_interaction_restricted: ::std::option::Option<bool>,
    #[serde(
        rename = "isLiking",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_liking: ::std::option::Option<bool>,
    #[serde(
        rename = "isOwner",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_owner: ::std::option::Option<bool>,
    #[serde(
        rename = "isPinned",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_pinned: ::std::option::Option<bool>,
    #[serde(
        rename = "isRequesterBlocking",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_requester_blocking: ::std::option::Option<bool>,
    #[serde(
        rename = "isSaved",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_saved: ::std::option::Option<bool>,
    #[serde(
        rename = "isSubscribed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_subscribed: ::std::option::Option<bool>,
}
impl ::std::default::Default for RequesterContextResponse {
    fn default() -> Self {
        Self {
            is_flagging_as_spam: Default::default(),
            is_following: Default::default(),
            is_interaction_restricted: Default::default(),
            is_liking: Default::default(),
            is_owner: Default::default(),
            is_pinned: Default::default(),
            is_requester_blocking: Default::default(),
            is_saved: Default::default(),
            is_subscribed: Default::default(),
        }
    }
}
impl RequesterContextResponse {
    pub fn builder() -> builder::RequesterContextResponse {
        Default::default()
    }
}
#[doc = "`ShareMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"sharedOriginPost\": {"]
#[doc = "      \"$ref\": \"#/$defs/Post\""]
#[doc = "    },"]
#[doc = "    \"sharedPost\": {"]
#[doc = "      \"$ref\": \"#/$defs/Post\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ShareMetadata {
    #[serde(rename = "sharedOriginPost", default)]
    pub shared_origin_post: ::std::boxed::Box<::std::option::Option<Post>>,
    #[serde(rename = "sharedPost", default)]
    pub shared_post: ::std::boxed::Box<::std::option::Option<Post>>,
}
impl ::std::default::Default for ShareMetadata {
    fn default() -> Self {
        Self {
            shared_origin_post: Default::default(),
            shared_post: Default::default(),
        }
    }
}
impl ShareMetadata {
    pub fn builder() -> builder::ShareMetadata {
        Default::default()
    }
}
#[doc = "`SummaryResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"sharedCount\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    },"]
#[doc = "    \"totalCommentsAndReplies\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int32\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SummaryResponse {
    #[serde(
        rename = "sharedCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub shared_count: ::std::option::Option<i32>,
    #[serde(
        rename = "totalCommentsAndReplies",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_comments_and_replies: ::std::option::Option<i32>,
}
impl ::std::default::Default for SummaryResponse {
    fn default() -> Self {
        Self {
            shared_count: Default::default(),
            total_comments_and_replies: Default::default(),
        }
    }
}
impl SummaryResponse {
    pub fn builder() -> builder::SummaryResponse {
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
#[doc = "`Tag`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"market\": {"]
#[doc = "      \"$ref\": \"#/$defs/Market\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Tag {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub market: ::std::option::Option<Market>,
}
impl ::std::default::Default for Tag {
    fn default() -> Self {
        Self {
            market: Default::default(),
        }
    }
}
impl Tag {
    pub fn builder() -> builder::Tag {
        Default::default()
    }
}
#[doc = "`TagRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
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
pub struct TagRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for TagRequest {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
        }
    }
}
impl TagRequest {
    pub fn builder() -> builder::TagRequest {
        Default::default()
    }
}
#[doc = "Tags associated with a post or comment"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Tags associated with a post or comment\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"tags\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"id\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"name\": {"]
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
pub struct Tags {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tags: ::std::vec::Vec<TagsTagsItem>,
}
impl ::std::default::Default for Tags {
    fn default() -> Self {
        Self {
            tags: Default::default(),
        }
    }
}
impl Tags {
    pub fn builder() -> builder::Tags {
        Default::default()
    }
}
#[doc = "`TagsDataRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"tags\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/TagRequest\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TagsDataRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<::std::vec::Vec<TagRequest>>,
}
impl ::std::default::Default for TagsDataRequest {
    fn default() -> Self {
        Self {
            tags: Default::default(),
        }
    }
}
impl TagsDataRequest {
    pub fn builder() -> builder::TagsDataRequest {
        Default::default()
    }
}
#[doc = "`TagsTagsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TagsTagsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for TagsTagsItem {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
        }
    }
}
impl TagsTagsItem {
    pub fn builder() -> builder::TagsTagsItem {
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
#[doc = "`VideoMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"image\": {"]
#[doc = "      \"$ref\": \"#/$defs/ImageMetadata\""]
#[doc = "    },"]
#[doc = "    \"videoSource\": {"]
#[doc = "      \"$ref\": \"#/$defs/VideoSource\""]
#[doc = "    },"]
#[doc = "    \"videoSourceId\": {"]
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
pub struct VideoMetadata {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub image: ::std::option::Option<ImageMetadata>,
    #[serde(
        rename = "videoSource",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub video_source: ::std::option::Option<VideoSource>,
    #[serde(
        rename = "videoSourceId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub video_source_id: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for VideoMetadata {
    fn default() -> Self {
        Self {
            image: Default::default(),
            video_source: Default::default(),
            video_source_id: Default::default(),
        }
    }
}
impl VideoMetadata {
    pub fn builder() -> builder::VideoMetadata {
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
#[doc = "    \"None\","]
#[doc = "    \"YouTube\","]
#[doc = "    \"Vimeo\""]
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
pub enum VideoSource {
    None,
    YouTube,
    Vimeo,
}
impl ::std::fmt::Display for VideoSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::YouTube => f.write_str("YouTube"),
            Self::Vimeo => f.write_str("Vimeo"),
        }
    }
}
impl ::std::str::FromStr for VideoSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "YouTube" => Ok(Self::YouTube),
            "Vimeo" => Ok(Self::Vimeo),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VideoSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VideoSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VideoSource {
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
    pub struct ArticleMetadata {
        ai_summary: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        body: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        body_preview: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        created: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        edit_status:
            ::std::result::Result<::std::option::Option<super::EditStatus>, ::std::string::String>,
        featured_image:
            ::std::result::Result<::std::option::Option<super::Attachment>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        language_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        owner_id: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        published: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        rating: ::std::result::Result<
            ::std::option::Option<super::ArticleRating>,
            ::std::string::String,
        >,
        reading_time_minutes:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        status: ::std::result::Result<
            ::std::option::Option<super::ArticleStatus>,
            ::std::string::String,
        >,
        tags: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Tag>>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        updated: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        word_count: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
    }
    impl ::std::default::Default for ArticleMetadata {
        fn default() -> Self {
            Self {
                ai_summary: Ok(Default::default()),
                body: Ok(Default::default()),
                body_preview: Ok(Default::default()),
                created: Ok(Default::default()),
                edit_status: Ok(Default::default()),
                featured_image: Ok(Default::default()),
                id: Ok(Default::default()),
                language_code: Ok(Default::default()),
                owner_id: Ok(Default::default()),
                published: Ok(Default::default()),
                rating: Ok(Default::default()),
                reading_time_minutes: Ok(Default::default()),
                status: Ok(Default::default()),
                tags: Ok(Default::default()),
                title: Ok(Default::default()),
                updated: Ok(Default::default()),
                url: Ok(Default::default()),
                word_count: Ok(Default::default()),
            }
        }
    }
    impl ArticleMetadata {
        pub fn ai_summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ai_summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ai_summary: {e}"));
            self
        }
        pub fn body<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for body: {e}"));
            self
        }
        pub fn body_preview<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.body_preview = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for body_preview: {e}"));
            self
        }
        pub fn created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created: {e}"));
            self
        }
        pub fn edit_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EditStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.edit_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for edit_status: {e}"));
            self
        }
        pub fn featured_image<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Attachment>>,
            T::Error: ::std::fmt::Display,
        {
            self.featured_image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for featured_image: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
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
        pub fn owner_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.owner_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner_id: {e}"));
            self
        }
        pub fn published<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.published = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for published: {e}"));
            self
        }
        pub fn rating<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ArticleRating>>,
            T::Error: ::std::fmt::Display,
        {
            self.rating = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rating: {e}"));
            self
        }
        pub fn reading_time_minutes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.reading_time_minutes = value.try_into().map_err(|e| {
                format!("error converting supplied value for reading_time_minutes: {e}")
            });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ArticleStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Tag>>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
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
        pub fn word_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.word_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for word_count: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ArticleMetadata> for super::ArticleMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ArticleMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ai_summary: value.ai_summary?,
                body: value.body?,
                body_preview: value.body_preview?,
                created: value.created?,
                edit_status: value.edit_status?,
                featured_image: value.featured_image?,
                id: value.id?,
                language_code: value.language_code?,
                owner_id: value.owner_id?,
                published: value.published?,
                rating: value.rating?,
                reading_time_minutes: value.reading_time_minutes?,
                status: value.status?,
                tags: value.tags?,
                title: value.title?,
                updated: value.updated?,
                url: value.url?,
                word_count: value.word_count?,
            })
        }
    }
    impl ::std::convert::From<super::ArticleMetadata> for ArticleMetadata {
        fn from(value: super::ArticleMetadata) -> Self {
            Self {
                ai_summary: Ok(value.ai_summary),
                body: Ok(value.body),
                body_preview: Ok(value.body_preview),
                created: Ok(value.created),
                edit_status: Ok(value.edit_status),
                featured_image: Ok(value.featured_image),
                id: Ok(value.id),
                language_code: Ok(value.language_code),
                owner_id: Ok(value.owner_id),
                published: Ok(value.published),
                rating: Ok(value.rating),
                reading_time_minutes: Ok(value.reading_time_minutes),
                status: Ok(value.status),
                tags: Ok(value.tags),
                title: Ok(value.title),
                updated: Ok(value.updated),
                url: Ok(value.url),
                word_count: Ok(value.word_count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Attachment {
        host: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        media: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        media_type: ::std::result::Result<
            ::std::option::Option<super::AttachmentMediaType>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            ::std::option::Option<super::AttachmentMetadata>,
            ::std::string::String,
        >,
        thumbnail_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::AttachmentType>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Attachment {
        fn default() -> Self {
            Self {
                host: Ok(Default::default()),
                media: Ok(Default::default()),
                media_type: Ok(Default::default()),
                metadata: Ok(Default::default()),
                thumbnail_url: Ok(Default::default()),
                type_: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl Attachment {
        pub fn host<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.host = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for host: {e}"));
            self
        }
        pub fn media<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.media = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media: {e}"));
            self
        }
        pub fn media_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AttachmentMediaType>>,
            T::Error: ::std::fmt::Display,
        {
            self.media_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_type: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AttachmentMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn thumbnail_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.thumbnail_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thumbnail_url: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AttachmentType>>,
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
    }
    impl ::std::convert::TryFrom<Attachment> for super::Attachment {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Attachment,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                host: value.host?,
                media: value.media?,
                media_type: value.media_type?,
                metadata: value.metadata?,
                thumbnail_url: value.thumbnail_url?,
                type_: value.type_?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::Attachment> for Attachment {
        fn from(value: super::Attachment) -> Self {
            Self {
                host: Ok(value.host),
                media: Ok(value.media),
                media_type: Ok(value.media_type),
                metadata: Ok(value.metadata),
                thumbnail_url: Ok(value.thumbnail_url),
                type_: Ok(value.type_),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AttachmentMetadata {
        duration: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        height: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        width: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for AttachmentMetadata {
        fn default() -> Self {
            Self {
                duration: Ok(Default::default()),
                height: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl AttachmentMetadata {
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {e}"));
            self
        }
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
    impl ::std::convert::TryFrom<AttachmentMetadata> for super::AttachmentMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AttachmentMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                duration: value.duration?,
                height: value.height?,
                width: value.width?,
            })
        }
    }
    impl ::std::convert::From<super::AttachmentMetadata> for AttachmentMetadata {
        fn from(value: super::AttachmentMetadata) -> Self {
            Self {
                duration: Ok(value.duration),
                height: Ok(value.height),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AttachmentsItem {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        host: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        media: ::std::result::Result<
            ::std::option::Option<super::AttachmentsItemMedia>,
            ::std::string::String,
        >,
        media_type: ::std::result::Result<
            ::std::option::Option<super::AttachmentsItemMediaType>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AttachmentsItem {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                host: Ok(Default::default()),
                media: Ok(Default::default()),
                media_type: Ok(Default::default()),
                title: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl AttachmentsItem {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn host<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.host = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for host: {e}"));
            self
        }
        pub fn media<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AttachmentsItemMedia>>,
            T::Error: ::std::fmt::Display,
        {
            self.media = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media: {e}"));
            self
        }
        pub fn media_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AttachmentsItemMediaType>>,
            T::Error: ::std::fmt::Display,
        {
            self.media_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_type: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
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
    impl ::std::convert::TryFrom<AttachmentsItem> for super::AttachmentsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AttachmentsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                host: value.host?,
                media: value.media?,
                media_type: value.media_type?,
                title: value.title?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::AttachmentsItem> for AttachmentsItem {
        fn from(value: super::AttachmentsItem) -> Self {
            Self {
                description: Ok(value.description),
                host: Ok(value.host),
                media: Ok(value.media),
                media_type: Ok(value.media_type),
                title: Ok(value.title),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AttachmentsItemMedia {
        image: ::std::result::Result<
            ::std::option::Option<super::AttachmentsItemMediaImage>,
            ::std::string::String,
        >,
        video: ::std::result::Result<
            ::std::option::Option<super::AttachmentsItemMediaVideo>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AttachmentsItemMedia {
        fn default() -> Self {
            Self {
                image: Ok(Default::default()),
                video: Ok(Default::default()),
            }
        }
    }
    impl AttachmentsItemMedia {
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AttachmentsItemMediaImage>>,
            T::Error: ::std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {e}"));
            self
        }
        pub fn video<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AttachmentsItemMediaVideo>>,
            T::Error: ::std::fmt::Display,
        {
            self.video = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AttachmentsItemMedia> for super::AttachmentsItemMedia {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AttachmentsItemMedia,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                image: value.image?,
                video: value.video?,
            })
        }
    }
    impl ::std::convert::From<super::AttachmentsItemMedia> for AttachmentsItemMedia {
        fn from(value: super::AttachmentsItemMedia) -> Self {
            Self {
                image: Ok(value.image),
                video: Ok(value.video),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AttachmentsItemMediaImage {
        height: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        width: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for AttachmentsItemMediaImage {
        fn default() -> Self {
            Self {
                height: Ok(Default::default()),
                url: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl AttachmentsItemMediaImage {
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
    impl ::std::convert::TryFrom<AttachmentsItemMediaImage> for super::AttachmentsItemMediaImage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AttachmentsItemMediaImage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                height: value.height?,
                url: value.url?,
                width: value.width?,
            })
        }
    }
    impl ::std::convert::From<super::AttachmentsItemMediaImage> for AttachmentsItemMediaImage {
        fn from(value: super::AttachmentsItemMediaImage) -> Self {
            Self {
                height: Ok(value.height),
                url: Ok(value.url),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AttachmentsItemMediaVideo {
        image: ::std::result::Result<
            ::std::option::Option<super::AttachmentsItemMediaVideoImage>,
            ::std::string::String,
        >,
        video_source: ::std::result::Result<
            ::std::option::Option<super::AttachmentsItemMediaVideoVideoSource>,
            ::std::string::String,
        >,
        video_source_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AttachmentsItemMediaVideo {
        fn default() -> Self {
            Self {
                image: Ok(Default::default()),
                video_source: Ok(Default::default()),
                video_source_id: Ok(Default::default()),
            }
        }
    }
    impl AttachmentsItemMediaVideo {
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AttachmentsItemMediaVideoImage>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {e}"));
            self
        }
        pub fn video_source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AttachmentsItemMediaVideoVideoSource>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.video_source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_source: {e}"));
            self
        }
        pub fn video_source_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.video_source_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_source_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AttachmentsItemMediaVideo> for super::AttachmentsItemMediaVideo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AttachmentsItemMediaVideo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                image: value.image?,
                video_source: value.video_source?,
                video_source_id: value.video_source_id?,
            })
        }
    }
    impl ::std::convert::From<super::AttachmentsItemMediaVideo> for AttachmentsItemMediaVideo {
        fn from(value: super::AttachmentsItemMediaVideo) -> Self {
            Self {
                image: Ok(value.image),
                video_source: Ok(value.video_source),
                video_source_id: Ok(value.video_source_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AttachmentsItemMediaVideoImage {
        height: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        width: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for AttachmentsItemMediaVideoImage {
        fn default() -> Self {
            Self {
                height: Ok(Default::default()),
                url: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl AttachmentsItemMediaVideoImage {
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
    impl ::std::convert::TryFrom<AttachmentsItemMediaVideoImage>
        for super::AttachmentsItemMediaVideoImage
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AttachmentsItemMediaVideoImage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                height: value.height?,
                url: value.url?,
                width: value.width?,
            })
        }
    }
    impl ::std::convert::From<super::AttachmentsItemMediaVideoImage>
        for AttachmentsItemMediaVideoImage
    {
        fn from(value: super::AttachmentsItemMediaVideoImage) -> Self {
            Self {
                height: Ok(value.height),
                url: Ok(value.url),
                width: Ok(value.width),
            }
        }
    }
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
    pub struct Comment {
        emotions_data: ::std::result::Result<
            ::std::option::Option<super::CommentEmotionsData>,
            ::std::string::String,
        >,
        entity: ::std::result::Result<
            ::std::option::Option<super::CommentEntity>,
            ::std::string::String,
        >,
        replies: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        replies_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        requester_context: ::std::result::Result<
            ::std::option::Option<super::CommentRequesterContext>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Comment {
        fn default() -> Self {
            Self {
                emotions_data: Ok(Default::default()),
                entity: Ok(Default::default()),
                replies: Ok(Default::default()),
                replies_count: Ok(Default::default()),
                requester_context: Ok(Default::default()),
            }
        }
    }
    impl Comment {
        pub fn emotions_data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CommentEmotionsData>>,
            T::Error: ::std::fmt::Display,
        {
            self.emotions_data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for emotions_data: {e}"));
            self
        }
        pub fn entity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CommentEntity>>,
            T::Error: ::std::fmt::Display,
        {
            self.entity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entity: {e}"));
            self
        }
        pub fn replies<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.replies = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for replies: {e}"));
            self
        }
        pub fn replies_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.replies_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for replies_count: {e}"));
            self
        }
        pub fn requester_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CommentRequesterContext>>,
            T::Error: ::std::fmt::Display,
        {
            self.requester_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for requester_context: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Comment> for super::Comment {
        type Error = super::error::ConversionError;
        fn try_from(value: Comment) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                emotions_data: value.emotions_data?,
                entity: value.entity?,
                replies: value.replies?,
                replies_count: value.replies_count?,
                requester_context: value.requester_context?,
            })
        }
    }
    impl ::std::convert::From<super::Comment> for Comment {
        fn from(value: super::Comment) -> Self {
            Self {
                emotions_data: Ok(value.emotions_data),
                entity: Ok(value.entity),
                replies: Ok(value.replies),
                replies_count: Ok(value.replies_count),
                requester_context: Ok(value.requester_context),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommentCreateRequest {
        attachments:
            ::std::result::Result<::std::option::Option<super::Attachments>, ::std::string::String>,
        mentions:
            ::std::result::Result<::std::option::Option<super::Mentions>, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        owner: ::std::result::Result<i64, ::std::string::String>,
        tags: ::std::result::Result<::std::option::Option<super::Tags>, ::std::string::String>,
    }
    impl ::std::default::Default for CommentCreateRequest {
        fn default() -> Self {
            Self {
                attachments: Ok(Default::default()),
                mentions: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
                owner: Err("no value supplied for owner".to_string()),
                tags: Ok(Default::default()),
            }
        }
    }
    impl CommentCreateRequest {
        pub fn attachments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Attachments>>,
            T::Error: ::std::fmt::Display,
        {
            self.attachments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attachments: {e}"));
            self
        }
        pub fn mentions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Mentions>>,
            T::Error: ::std::fmt::Display,
        {
            self.mentions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mentions: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Tags>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CommentCreateRequest> for super::CommentCreateRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommentCreateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attachments: value.attachments?,
                mentions: value.mentions?,
                message: value.message?,
                owner: value.owner?,
                tags: value.tags?,
            })
        }
    }
    impl ::std::convert::From<super::CommentCreateRequest> for CommentCreateRequest {
        fn from(value: super::CommentCreateRequest) -> Self {
            Self {
                attachments: Ok(value.attachments),
                mentions: Ok(value.mentions),
                message: Ok(value.message),
                owner: Ok(value.owner),
                tags: Ok(value.tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommentDataResponse {
        badged_owner_country_id:
            ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        emotions_data: ::std::result::Result<
            ::std::option::Option<super::EmotionsDataResponse>,
            ::std::string::String,
        >,
        entity: ::std::result::Result<::std::option::Option<super::Comment>, ::std::string::String>,
        replies: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::CommentDataResponse>>,
            ::std::string::String,
        >,
        replies_count: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        requester_context: ::std::result::Result<
            ::std::option::Option<super::RequesterContextResponse>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CommentDataResponse {
        fn default() -> Self {
            Self {
                badged_owner_country_id: Ok(Default::default()),
                emotions_data: Ok(Default::default()),
                entity: Ok(Default::default()),
                replies: Ok(Default::default()),
                replies_count: Ok(Default::default()),
                requester_context: Ok(Default::default()),
            }
        }
    }
    impl CommentDataResponse {
        pub fn badged_owner_country_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.badged_owner_country_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for badged_owner_country_id: {e}")
            });
            self
        }
        pub fn emotions_data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EmotionsDataResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.emotions_data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for emotions_data: {e}"));
            self
        }
        pub fn entity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Comment>>,
            T::Error: ::std::fmt::Display,
        {
            self.entity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entity: {e}"));
            self
        }
        pub fn replies<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::CommentDataResponse>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.replies = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for replies: {e}"));
            self
        }
        pub fn replies_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.replies_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for replies_count: {e}"));
            self
        }
        pub fn requester_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RequesterContextResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.requester_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for requester_context: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CommentDataResponse> for super::CommentDataResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommentDataResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                badged_owner_country_id: value.badged_owner_country_id?,
                emotions_data: value.emotions_data?,
                entity: value.entity?,
                replies: value.replies?,
                replies_count: value.replies_count?,
                requester_context: value.requester_context?,
            })
        }
    }
    impl ::std::convert::From<super::CommentDataResponse> for CommentDataResponse {
        fn from(value: super::CommentDataResponse) -> Self {
            Self {
                badged_owner_country_id: Ok(value.badged_owner_country_id),
                emotions_data: Ok(value.emotions_data),
                entity: Ok(value.entity),
                replies: Ok(value.replies),
                replies_count: Ok(value.replies_count),
                requester_context: Ok(value.requester_context),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommentEmotionsData {
        like: ::std::result::Result<
            ::std::option::Option<super::CommentEmotionsDataLike>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CommentEmotionsData {
        fn default() -> Self {
            Self {
                like: Ok(Default::default()),
            }
        }
    }
    impl CommentEmotionsData {
        pub fn like<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CommentEmotionsDataLike>>,
            T::Error: ::std::fmt::Display,
        {
            self.like = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for like: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CommentEmotionsData> for super::CommentEmotionsData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommentEmotionsData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { like: value.like? })
        }
    }
    impl ::std::convert::From<super::CommentEmotionsData> for CommentEmotionsData {
        fn from(value: super::CommentEmotionsData) -> Self {
            Self {
                like: Ok(value.like),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommentEmotionsDataLike {
        emotions: ::std::result::Result<::std::vec::Vec<super::Emotion>, ::std::string::String>,
        paging: ::std::result::Result<
            ::std::option::Option<super::CommentEmotionsDataLikePaging>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CommentEmotionsDataLike {
        fn default() -> Self {
            Self {
                emotions: Ok(Default::default()),
                paging: Ok(Default::default()),
            }
        }
    }
    impl CommentEmotionsDataLike {
        pub fn emotions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Emotion>>,
            T::Error: ::std::fmt::Display,
        {
            self.emotions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for emotions: {e}"));
            self
        }
        pub fn paging<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CommentEmotionsDataLikePaging>>,
            T::Error: ::std::fmt::Display,
        {
            self.paging = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for paging: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CommentEmotionsDataLike> for super::CommentEmotionsDataLike {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommentEmotionsDataLike,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                emotions: value.emotions?,
                paging: value.paging?,
            })
        }
    }
    impl ::std::convert::From<super::CommentEmotionsDataLike> for CommentEmotionsDataLike {
        fn from(value: super::CommentEmotionsDataLike) -> Self {
            Self {
                emotions: Ok(value.emotions),
                paging: Ok(value.paging),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommentEmotionsDataLikePaging {
        total_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for CommentEmotionsDataLikePaging {
        fn default() -> Self {
            Self {
                total_count: Ok(Default::default()),
            }
        }
    }
    impl CommentEmotionsDataLikePaging {
        pub fn total_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_count: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CommentEmotionsDataLikePaging>
        for super::CommentEmotionsDataLikePaging
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommentEmotionsDataLikePaging,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                total_count: value.total_count?,
            })
        }
    }
    impl ::std::convert::From<super::CommentEmotionsDataLikePaging> for CommentEmotionsDataLikePaging {
        fn from(value: super::CommentEmotionsDataLikePaging) -> Self {
            Self {
                total_count: Ok(value.total_count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommentEntity {
        attachments:
            ::std::result::Result<::std::vec::Vec<super::Attachment>, ::std::string::String>,
        created: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        edit_status: ::std::result::Result<
            ::std::option::Option<super::CommentEntityEditStatus>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        is_spam: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        message: ::std::result::Result<
            ::std::option::Option<super::CommentEntityMessage>,
            ::std::string::String,
        >,
        obsolete_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        owner: ::std::result::Result<::std::option::Option<super::User>, ::std::string::String>,
        parent: ::std::result::Result<
            ::std::option::Option<super::CommentEntityParent>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CommentEntity {
        fn default() -> Self {
            Self {
                attachments: Ok(Default::default()),
                created: Ok(Default::default()),
                edit_status: Ok(Default::default()),
                id: Ok(Default::default()),
                is_spam: Ok(Default::default()),
                message: Ok(Default::default()),
                obsolete_id: Ok(Default::default()),
                owner: Ok(Default::default()),
                parent: Ok(Default::default()),
            }
        }
    }
    impl CommentEntity {
        pub fn attachments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Attachment>>,
            T::Error: ::std::fmt::Display,
        {
            self.attachments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attachments: {e}"));
            self
        }
        pub fn created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created: {e}"));
            self
        }
        pub fn edit_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CommentEntityEditStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.edit_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for edit_status: {e}"));
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
        pub fn is_spam<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_spam = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_spam: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CommentEntityMessage>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn obsolete_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.obsolete_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for obsolete_id: {e}"));
            self
        }
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::User>>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
            self
        }
        pub fn parent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CommentEntityParent>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CommentEntity> for super::CommentEntity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommentEntity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attachments: value.attachments?,
                created: value.created?,
                edit_status: value.edit_status?,
                id: value.id?,
                is_spam: value.is_spam?,
                message: value.message?,
                obsolete_id: value.obsolete_id?,
                owner: value.owner?,
                parent: value.parent?,
            })
        }
    }
    impl ::std::convert::From<super::CommentEntity> for CommentEntity {
        fn from(value: super::CommentEntity) -> Self {
            Self {
                attachments: Ok(value.attachments),
                created: Ok(value.created),
                edit_status: Ok(value.edit_status),
                id: Ok(value.id),
                is_spam: Ok(value.is_spam),
                message: Ok(value.message),
                obsolete_id: Ok(value.obsolete_id),
                owner: Ok(value.owner),
                parent: Ok(value.parent),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommentEntityMessage {
        language_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CommentEntityMessage {
        fn default() -> Self {
            Self {
                language_code: Ok(Default::default()),
                text: Ok(Default::default()),
            }
        }
    }
    impl CommentEntityMessage {
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
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CommentEntityMessage> for super::CommentEntityMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommentEntityMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                language_code: value.language_code?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::CommentEntityMessage> for CommentEntityMessage {
        fn from(value: super::CommentEntityMessage) -> Self {
            Self {
                language_code: Ok(value.language_code),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommentEntityParent {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        obsolete_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CommentEntityParent {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                obsolete_id: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl CommentEntityParent {
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
        pub fn obsolete_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.obsolete_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for obsolete_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CommentEntityParent> for super::CommentEntityParent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommentEntityParent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                obsolete_id: value.obsolete_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CommentEntityParent> for CommentEntityParent {
        fn from(value: super::CommentEntityParent) -> Self {
            Self {
                id: Ok(value.id),
                obsolete_id: Ok(value.obsolete_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommentRequest {
        attachments: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Attachment>>,
            ::std::string::String,
        >,
        comment_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        mentions: ::std::result::Result<
            ::std::option::Option<super::MentionsDataRequest>,
            ::std::string::String,
        >,
        message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        owner: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        post_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        reply_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tags: ::std::result::Result<
            ::std::option::Option<super::TagsDataRequest>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CommentRequest {
        fn default() -> Self {
            Self {
                attachments: Ok(Default::default()),
                comment_id: Ok(Default::default()),
                mentions: Ok(Default::default()),
                message: Ok(Default::default()),
                owner: Ok(Default::default()),
                post_id: Ok(Default::default()),
                reply_id: Ok(Default::default()),
                tags: Ok(Default::default()),
            }
        }
    }
    impl CommentRequest {
        pub fn attachments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Attachment>>>,
            T::Error: ::std::fmt::Display,
        {
            self.attachments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attachments: {e}"));
            self
        }
        pub fn comment_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.comment_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for comment_id: {e}"));
            self
        }
        pub fn mentions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MentionsDataRequest>>,
            T::Error: ::std::fmt::Display,
        {
            self.mentions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mentions: {e}"));
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
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
            self
        }
        pub fn post_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.post_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for post_id: {e}"));
            self
        }
        pub fn reply_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reply_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reply_id: {e}"));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TagsDataRequest>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CommentRequest> for super::CommentRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommentRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attachments: value.attachments?,
                comment_id: value.comment_id?,
                mentions: value.mentions?,
                message: value.message?,
                owner: value.owner?,
                post_id: value.post_id?,
                reply_id: value.reply_id?,
                tags: value.tags?,
            })
        }
    }
    impl ::std::convert::From<super::CommentRequest> for CommentRequest {
        fn from(value: super::CommentRequest) -> Self {
            Self {
                attachments: Ok(value.attachments),
                comment_id: Ok(value.comment_id),
                mentions: Ok(value.mentions),
                message: Ok(value.message),
                owner: Ok(value.owner),
                post_id: Ok(value.post_id),
                reply_id: Ok(value.reply_id),
                tags: Ok(value.tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommentRequesterContext {
        is_flagging_as_spam:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_following: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_interaction_restricted:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_liking: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_pinned: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_requester_blocking:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_saved: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_subscribed: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for CommentRequesterContext {
        fn default() -> Self {
            Self {
                is_flagging_as_spam: Ok(Default::default()),
                is_following: Ok(Default::default()),
                is_interaction_restricted: Ok(Default::default()),
                is_liking: Ok(Default::default()),
                is_pinned: Ok(Default::default()),
                is_requester_blocking: Ok(Default::default()),
                is_saved: Ok(Default::default()),
                is_subscribed: Ok(Default::default()),
            }
        }
    }
    impl CommentRequesterContext {
        pub fn is_flagging_as_spam<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_flagging_as_spam = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_flagging_as_spam: {e}")
            });
            self
        }
        pub fn is_following<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_following = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_following: {e}"));
            self
        }
        pub fn is_interaction_restricted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_interaction_restricted = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_interaction_restricted: {e}")
            });
            self
        }
        pub fn is_liking<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_liking = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_liking: {e}"));
            self
        }
        pub fn is_pinned<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_pinned = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_pinned: {e}"));
            self
        }
        pub fn is_requester_blocking<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_requester_blocking = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_requester_blocking: {e}")
            });
            self
        }
        pub fn is_saved<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_saved = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_saved: {e}"));
            self
        }
        pub fn is_subscribed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_subscribed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_subscribed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CommentRequesterContext> for super::CommentRequesterContext {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommentRequesterContext,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                is_flagging_as_spam: value.is_flagging_as_spam?,
                is_following: value.is_following?,
                is_interaction_restricted: value.is_interaction_restricted?,
                is_liking: value.is_liking?,
                is_pinned: value.is_pinned?,
                is_requester_blocking: value.is_requester_blocking?,
                is_saved: value.is_saved?,
                is_subscribed: value.is_subscribed?,
            })
        }
    }
    impl ::std::convert::From<super::CommentRequesterContext> for CommentRequesterContext {
        fn from(value: super::CommentRequesterContext) -> Self {
            Self {
                is_flagging_as_spam: Ok(value.is_flagging_as_spam),
                is_following: Ok(value.is_following),
                is_interaction_restricted: Ok(value.is_interaction_restricted),
                is_liking: Ok(value.is_liking),
                is_pinned: Ok(value.is_pinned),
                is_requester_blocking: Ok(value.is_requester_blocking),
                is_saved: Ok(value.is_saved),
                is_subscribed: Ok(value.is_subscribed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CopyMetadata {
        type_: ::std::result::Result<::std::option::Option<super::CopyType>, ::std::string::String>,
        user: ::std::result::Result<::std::option::Option<super::User>, ::std::string::String>,
    }
    impl ::std::default::Default for CopyMetadata {
        fn default() -> Self {
            Self {
                type_: Ok(Default::default()),
                user: Ok(Default::default()),
            }
        }
    }
    impl CopyMetadata {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CopyType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::User>>,
            T::Error: ::std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CopyMetadata> for super::CopyMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CopyMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
                user: value.user?,
            })
        }
    }
    impl ::std::convert::From<super::CopyMetadata> for CopyMetadata {
        fn from(value: super::CopyMetadata) -> Self {
            Self {
                type_: Ok(value.type_),
                user: Ok(value.user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Discussion {
        comments_data: ::std::result::Result<
            ::std::option::Option<super::DiscussionCommentsData>,
            ::std::string::String,
        >,
        emotions_data: ::std::result::Result<
            ::std::option::Option<super::DiscussionEmotionsData>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        post: ::std::result::Result<
            ::std::option::Option<super::DiscussionsPost>,
            ::std::string::String,
        >,
        reason: ::std::result::Result<(), ::std::string::String>,
        requester_context: ::std::result::Result<
            ::std::option::Option<super::DiscussionRequesterContext>,
            ::std::string::String,
        >,
        summary: ::std::result::Result<
            ::std::option::Option<super::DiscussionSummary>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Discussion {
        fn default() -> Self {
            Self {
                comments_data: Ok(Default::default()),
                emotions_data: Ok(Default::default()),
                id: Ok(Default::default()),
                post: Ok(Default::default()),
                reason: Ok(Default::default()),
                requester_context: Ok(Default::default()),
                summary: Ok(Default::default()),
            }
        }
    }
    impl Discussion {
        pub fn comments_data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionCommentsData>>,
            T::Error: ::std::fmt::Display,
        {
            self.comments_data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for comments_data: {e}"));
            self
        }
        pub fn emotions_data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionEmotionsData>>,
            T::Error: ::std::fmt::Display,
        {
            self.emotions_data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for emotions_data: {e}"));
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
        pub fn post<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionsPost>>,
            T::Error: ::std::fmt::Display,
        {
            self.post = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for post: {e}"));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<()>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
        pub fn requester_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionRequesterContext>>,
            T::Error: ::std::fmt::Display,
        {
            self.requester_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for requester_context: {e}"));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionSummary>>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Discussion> for super::Discussion {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Discussion,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                comments_data: value.comments_data?,
                emotions_data: value.emotions_data?,
                id: value.id?,
                post: value.post?,
                reason: value.reason?,
                requester_context: value.requester_context?,
                summary: value.summary?,
            })
        }
    }
    impl ::std::convert::From<super::Discussion> for Discussion {
        fn from(value: super::Discussion) -> Self {
            Self {
                comments_data: Ok(value.comments_data),
                emotions_data: Ok(value.emotions_data),
                id: Ok(value.id),
                post: Ok(value.post),
                reason: Ok(value.reason),
                requester_context: Ok(value.requester_context),
                summary: Ok(value.summary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionCommentsData {
        comments: ::std::result::Result<::std::vec::Vec<super::Comment>, ::std::string::String>,
        reaction_paging: ::std::result::Result<
            ::std::option::Option<super::DiscussionCommentsDataReactionPaging>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionCommentsData {
        fn default() -> Self {
            Self {
                comments: Ok(Default::default()),
                reaction_paging: Ok(Default::default()),
            }
        }
    }
    impl DiscussionCommentsData {
        pub fn comments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Comment>>,
            T::Error: ::std::fmt::Display,
        {
            self.comments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for comments: {e}"));
            self
        }
        pub fn reaction_paging<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::DiscussionCommentsDataReactionPaging>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.reaction_paging = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reaction_paging: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionCommentsData> for super::DiscussionCommentsData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionCommentsData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                comments: value.comments?,
                reaction_paging: value.reaction_paging?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionCommentsData> for DiscussionCommentsData {
        fn from(value: super::DiscussionCommentsData) -> Self {
            Self {
                comments: Ok(value.comments),
                reaction_paging: Ok(value.reaction_paging),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionCommentsDataReactionPaging {
        total_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for DiscussionCommentsDataReactionPaging {
        fn default() -> Self {
            Self {
                total_count: Ok(Default::default()),
            }
        }
    }
    impl DiscussionCommentsDataReactionPaging {
        pub fn total_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_count: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionCommentsDataReactionPaging>
        for super::DiscussionCommentsDataReactionPaging
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionCommentsDataReactionPaging,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                total_count: value.total_count?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionCommentsDataReactionPaging>
        for DiscussionCommentsDataReactionPaging
    {
        fn from(value: super::DiscussionCommentsDataReactionPaging) -> Self {
            Self {
                total_count: Ok(value.total_count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionCreateRequest {
        attachments:
            ::std::result::Result<::std::option::Option<super::Attachments>, ::std::string::String>,
        mentions:
            ::std::result::Result<::std::option::Option<super::Mentions>, ::std::string::String>,
        message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        owner: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        tags: ::std::result::Result<::std::option::Option<super::Tags>, ::std::string::String>,
    }
    impl ::std::default::Default for DiscussionCreateRequest {
        fn default() -> Self {
            Self {
                attachments: Ok(Default::default()),
                mentions: Ok(Default::default()),
                message: Ok(Default::default()),
                owner: Ok(Default::default()),
                tags: Ok(Default::default()),
            }
        }
    }
    impl DiscussionCreateRequest {
        pub fn attachments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Attachments>>,
            T::Error: ::std::fmt::Display,
        {
            self.attachments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attachments: {e}"));
            self
        }
        pub fn mentions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Mentions>>,
            T::Error: ::std::fmt::Display,
        {
            self.mentions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mentions: {e}"));
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
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Tags>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionCreateRequest> for super::DiscussionCreateRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionCreateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attachments: value.attachments?,
                mentions: value.mentions?,
                message: value.message?,
                owner: value.owner?,
                tags: value.tags?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionCreateRequest> for DiscussionCreateRequest {
        fn from(value: super::DiscussionCreateRequest) -> Self {
            Self {
                attachments: Ok(value.attachments),
                mentions: Ok(value.mentions),
                message: Ok(value.message),
                owner: Ok(value.owner),
                tags: Ok(value.tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionEmotionsData {
        like: ::std::result::Result<
            ::std::option::Option<super::DiscussionEmotionsDataLike>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionEmotionsData {
        fn default() -> Self {
            Self {
                like: Ok(Default::default()),
            }
        }
    }
    impl DiscussionEmotionsData {
        pub fn like<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionEmotionsDataLike>>,
            T::Error: ::std::fmt::Display,
        {
            self.like = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for like: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionEmotionsData> for super::DiscussionEmotionsData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionEmotionsData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { like: value.like? })
        }
    }
    impl ::std::convert::From<super::DiscussionEmotionsData> for DiscussionEmotionsData {
        fn from(value: super::DiscussionEmotionsData) -> Self {
            Self {
                like: Ok(value.like),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionEmotionsDataLike {
        emotions: ::std::result::Result<::std::vec::Vec<super::Emotion>, ::std::string::String>,
        paging: ::std::result::Result<
            ::std::option::Option<super::DiscussionEmotionsDataLikePaging>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionEmotionsDataLike {
        fn default() -> Self {
            Self {
                emotions: Ok(Default::default()),
                paging: Ok(Default::default()),
            }
        }
    }
    impl DiscussionEmotionsDataLike {
        pub fn emotions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Emotion>>,
            T::Error: ::std::fmt::Display,
        {
            self.emotions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for emotions: {e}"));
            self
        }
        pub fn paging<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::DiscussionEmotionsDataLikePaging>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.paging = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for paging: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionEmotionsDataLike> for super::DiscussionEmotionsDataLike {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionEmotionsDataLike,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                emotions: value.emotions?,
                paging: value.paging?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionEmotionsDataLike> for DiscussionEmotionsDataLike {
        fn from(value: super::DiscussionEmotionsDataLike) -> Self {
            Self {
                emotions: Ok(value.emotions),
                paging: Ok(value.paging),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionEmotionsDataLikePaging {
        total_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for DiscussionEmotionsDataLikePaging {
        fn default() -> Self {
            Self {
                total_count: Ok(Default::default()),
            }
        }
    }
    impl DiscussionEmotionsDataLikePaging {
        pub fn total_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_count: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionEmotionsDataLikePaging>
        for super::DiscussionEmotionsDataLikePaging
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionEmotionsDataLikePaging,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                total_count: value.total_count?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionEmotionsDataLikePaging>
        for DiscussionEmotionsDataLikePaging
    {
        fn from(value: super::DiscussionEmotionsDataLikePaging) -> Self {
            Self {
                total_count: Ok(value.total_count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionRequesterContext {
        is_flagging_as_spam:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_following: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_interaction_restricted:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_liking: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_pinned: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_requester_blocking:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_saved: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_subscribed: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for DiscussionRequesterContext {
        fn default() -> Self {
            Self {
                is_flagging_as_spam: Ok(Default::default()),
                is_following: Ok(Default::default()),
                is_interaction_restricted: Ok(Default::default()),
                is_liking: Ok(Default::default()),
                is_pinned: Ok(Default::default()),
                is_requester_blocking: Ok(Default::default()),
                is_saved: Ok(Default::default()),
                is_subscribed: Ok(Default::default()),
            }
        }
    }
    impl DiscussionRequesterContext {
        pub fn is_flagging_as_spam<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_flagging_as_spam = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_flagging_as_spam: {e}")
            });
            self
        }
        pub fn is_following<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_following = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_following: {e}"));
            self
        }
        pub fn is_interaction_restricted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_interaction_restricted = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_interaction_restricted: {e}")
            });
            self
        }
        pub fn is_liking<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_liking = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_liking: {e}"));
            self
        }
        pub fn is_pinned<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_pinned = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_pinned: {e}"));
            self
        }
        pub fn is_requester_blocking<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_requester_blocking = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_requester_blocking: {e}")
            });
            self
        }
        pub fn is_saved<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_saved = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_saved: {e}"));
            self
        }
        pub fn is_subscribed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_subscribed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_subscribed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionRequesterContext> for super::DiscussionRequesterContext {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionRequesterContext,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                is_flagging_as_spam: value.is_flagging_as_spam?,
                is_following: value.is_following?,
                is_interaction_restricted: value.is_interaction_restricted?,
                is_liking: value.is_liking?,
                is_pinned: value.is_pinned?,
                is_requester_blocking: value.is_requester_blocking?,
                is_saved: value.is_saved?,
                is_subscribed: value.is_subscribed?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionRequesterContext> for DiscussionRequesterContext {
        fn from(value: super::DiscussionRequesterContext) -> Self {
            Self {
                is_flagging_as_spam: Ok(value.is_flagging_as_spam),
                is_following: Ok(value.is_following),
                is_interaction_restricted: Ok(value.is_interaction_restricted),
                is_liking: Ok(value.is_liking),
                is_pinned: Ok(value.is_pinned),
                is_requester_blocking: Ok(value.is_requester_blocking),
                is_saved: Ok(value.is_saved),
                is_subscribed: Ok(value.is_subscribed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionResponse {
        badged_owner_country_id:
            ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        comments_data: ::std::result::Result<
            ::std::option::Option<super::EntityCommentsDataResponse>,
            ::std::string::String,
        >,
        emotions_data: ::std::result::Result<
            ::std::option::Option<super::EmotionsDataResponse>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        post: ::std::result::Result<::std::option::Option<super::Post>, ::std::string::String>,
        reason: ::std::result::Result<::std::option::Option<super::Reason>, ::std::string::String>,
        requester_context: ::std::result::Result<
            ::std::option::Option<super::RequesterContextResponse>,
            ::std::string::String,
        >,
        summary: ::std::result::Result<
            ::std::option::Option<super::SummaryResponse>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionResponse {
        fn default() -> Self {
            Self {
                badged_owner_country_id: Ok(Default::default()),
                comments_data: Ok(Default::default()),
                emotions_data: Ok(Default::default()),
                id: Ok(Default::default()),
                post: Ok(Default::default()),
                reason: Ok(Default::default()),
                requester_context: Ok(Default::default()),
                summary: Ok(Default::default()),
            }
        }
    }
    impl DiscussionResponse {
        pub fn badged_owner_country_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.badged_owner_country_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for badged_owner_country_id: {e}")
            });
            self
        }
        pub fn comments_data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EntityCommentsDataResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.comments_data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for comments_data: {e}"));
            self
        }
        pub fn emotions_data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EmotionsDataResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.emotions_data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for emotions_data: {e}"));
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
        pub fn post<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Post>>,
            T::Error: ::std::fmt::Display,
        {
            self.post = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for post: {e}"));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Reason>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
        pub fn requester_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RequesterContextResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.requester_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for requester_context: {e}"));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SummaryResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionResponse> for super::DiscussionResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                badged_owner_country_id: value.badged_owner_country_id?,
                comments_data: value.comments_data?,
                emotions_data: value.emotions_data?,
                id: value.id?,
                post: value.post?,
                reason: value.reason?,
                requester_context: value.requester_context?,
                summary: value.summary?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionResponse> for DiscussionResponse {
        fn from(value: super::DiscussionResponse) -> Self {
            Self {
                badged_owner_country_id: Ok(value.badged_owner_country_id),
                comments_data: Ok(value.comments_data),
                emotions_data: Ok(value.emotions_data),
                id: Ok(value.id),
                post: Ok(value.post),
                reason: Ok(value.reason),
                requester_context: Ok(value.requester_context),
                summary: Ok(value.summary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionSummary {
        shared_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        total_comments_and_replies:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for DiscussionSummary {
        fn default() -> Self {
            Self {
                shared_count: Ok(Default::default()),
                total_comments_and_replies: Ok(Default::default()),
            }
        }
    }
    impl DiscussionSummary {
        pub fn shared_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.shared_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shared_count: {e}"));
            self
        }
        pub fn total_comments_and_replies<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_comments_and_replies = value.try_into().map_err(|e| {
                format!("error converting supplied value for total_comments_and_replies: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionSummary> for super::DiscussionSummary {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionSummary,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                shared_count: value.shared_count?,
                total_comments_and_replies: value.total_comments_and_replies?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionSummary> for DiscussionSummary {
        fn from(value: super::DiscussionSummary) -> Self {
            Self {
                shared_count: Ok(value.shared_count),
                total_comments_and_replies: Ok(value.total_comments_and_replies),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionsPost {
        attachments:
            ::std::result::Result<::std::vec::Vec<super::Attachment>, ::std::string::String>,
        created: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        edit_status: ::std::result::Result<
            ::std::option::Option<super::DiscussionsPostEditStatus>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        is_deleted: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_spam: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        mentions: ::std::result::Result<
            ::std::vec::Vec<super::DiscussionsPostMentionsItem>,
            ::std::string::String,
        >,
        message: ::std::result::Result<
            ::std::option::Option<super::DiscussionsPostMessage>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            ::std::option::Option<super::DiscussionsPostMetadata>,
            ::std::string::String,
        >,
        obsolete_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        owner: ::std::result::Result<::std::option::Option<super::User>, ::std::string::String>,
        tags: ::std::result::Result<
            ::std::vec::Vec<super::DiscussionsPostTagsItem>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        updated: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionsPost {
        fn default() -> Self {
            Self {
                attachments: Ok(Default::default()),
                created: Ok(Default::default()),
                edit_status: Ok(Default::default()),
                id: Ok(Default::default()),
                is_deleted: Ok(Default::default()),
                is_spam: Ok(Default::default()),
                mentions: Ok(Default::default()),
                message: Ok(Default::default()),
                metadata: Ok(Default::default()),
                obsolete_id: Ok(Default::default()),
                owner: Ok(Default::default()),
                tags: Ok(Default::default()),
                type_: Ok(Default::default()),
                updated: Ok(Default::default()),
            }
        }
    }
    impl DiscussionsPost {
        pub fn attachments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Attachment>>,
            T::Error: ::std::fmt::Display,
        {
            self.attachments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attachments: {e}"));
            self
        }
        pub fn created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created: {e}"));
            self
        }
        pub fn edit_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionsPostEditStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.edit_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for edit_status: {e}"));
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
        pub fn is_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_deleted: {e}"));
            self
        }
        pub fn is_spam<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_spam = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_spam: {e}"));
            self
        }
        pub fn mentions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::DiscussionsPostMentionsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.mentions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mentions: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionsPostMessage>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionsPostMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn obsolete_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.obsolete_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for obsolete_id: {e}"));
            self
        }
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::User>>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::DiscussionsPostTagsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
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
    impl ::std::convert::TryFrom<DiscussionsPost> for super::DiscussionsPost {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsPost,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attachments: value.attachments?,
                created: value.created?,
                edit_status: value.edit_status?,
                id: value.id?,
                is_deleted: value.is_deleted?,
                is_spam: value.is_spam?,
                mentions: value.mentions?,
                message: value.message?,
                metadata: value.metadata?,
                obsolete_id: value.obsolete_id?,
                owner: value.owner?,
                tags: value.tags?,
                type_: value.type_?,
                updated: value.updated?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionsPost> for DiscussionsPost {
        fn from(value: super::DiscussionsPost) -> Self {
            Self {
                attachments: Ok(value.attachments),
                created: Ok(value.created),
                edit_status: Ok(value.edit_status),
                id: Ok(value.id),
                is_deleted: Ok(value.is_deleted),
                is_spam: Ok(value.is_spam),
                mentions: Ok(value.mentions),
                message: Ok(value.message),
                metadata: Ok(value.metadata),
                obsolete_id: Ok(value.obsolete_id),
                owner: Ok(value.owner),
                tags: Ok(value.tags),
                type_: Ok(value.type_),
                updated: Ok(value.updated),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionsPostMentionsItem {
        is_direct: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        user: ::std::result::Result<::std::option::Option<super::User>, ::std::string::String>,
    }
    impl ::std::default::Default for DiscussionsPostMentionsItem {
        fn default() -> Self {
            Self {
                is_direct: Ok(Default::default()),
                user: Ok(Default::default()),
            }
        }
    }
    impl DiscussionsPostMentionsItem {
        pub fn is_direct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_direct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_direct: {e}"));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::User>>,
            T::Error: ::std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionsPostMentionsItem> for super::DiscussionsPostMentionsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsPostMentionsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                is_direct: value.is_direct?,
                user: value.user?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionsPostMentionsItem> for DiscussionsPostMentionsItem {
        fn from(value: super::DiscussionsPostMentionsItem) -> Self {
            Self {
                is_direct: Ok(value.is_direct),
                user: Ok(value.user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionsPostMessage {
        language_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionsPostMessage {
        fn default() -> Self {
            Self {
                language_code: Ok(Default::default()),
                text: Ok(Default::default()),
            }
        }
    }
    impl DiscussionsPostMessage {
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
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionsPostMessage> for super::DiscussionsPostMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsPostMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                language_code: value.language_code?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionsPostMessage> for DiscussionsPostMessage {
        fn from(value: super::DiscussionsPostMessage) -> Self {
            Self {
                language_code: Ok(value.language_code),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionsPostMetadata {
        poll: ::std::result::Result<
            ::std::option::Option<super::DiscussionsPostMetadataPoll>,
            ::std::string::String,
        >,
        share: ::std::result::Result<
            ::std::option::Option<super::DiscussionsPostMetadataShare>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionsPostMetadata {
        fn default() -> Self {
            Self {
                poll: Ok(Default::default()),
                share: Ok(Default::default()),
            }
        }
    }
    impl DiscussionsPostMetadata {
        pub fn poll<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionsPostMetadataPoll>>,
            T::Error: ::std::fmt::Display,
        {
            self.poll = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for poll: {e}"));
            self
        }
        pub fn share<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionsPostMetadataShare>>,
            T::Error: ::std::fmt::Display,
        {
            self.share = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for share: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionsPostMetadata> for super::DiscussionsPostMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsPostMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                poll: value.poll?,
                share: value.share?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionsPostMetadata> for DiscussionsPostMetadata {
        fn from(value: super::DiscussionsPostMetadata) -> Self {
            Self {
                poll: Ok(value.poll),
                share: Ok(value.share),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionsPostMetadataPoll {
        gcid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        options: ::std::result::Result<
            ::std::vec::Vec<super::DiscussionsPostMetadataPollOptionsItem>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionsPostMetadataPoll {
        fn default() -> Self {
            Self {
                gcid: Ok(Default::default()),
                id: Ok(Default::default()),
                options: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl DiscussionsPostMetadataPoll {
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
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::DiscussionsPostMetadataPollOptionsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.options = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for options: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionsPostMetadataPoll> for super::DiscussionsPostMetadataPoll {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsPostMetadataPoll,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                gcid: value.gcid?,
                id: value.id?,
                options: value.options?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionsPostMetadataPoll> for DiscussionsPostMetadataPoll {
        fn from(value: super::DiscussionsPostMetadataPoll) -> Self {
            Self {
                gcid: Ok(value.gcid),
                id: Ok(value.id),
                options: Ok(value.options),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionsPostMetadataPollOptionsItem {
        id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        index: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        is_user_voted: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        votes_count: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for DiscussionsPostMetadataPollOptionsItem {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                index: Ok(Default::default()),
                is_user_voted: Ok(Default::default()),
                text: Ok(Default::default()),
                votes_count: Ok(Default::default()),
            }
        }
    }
    impl DiscussionsPostMetadataPollOptionsItem {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for index: {e}"));
            self
        }
        pub fn is_user_voted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_user_voted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_user_voted: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn votes_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.votes_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for votes_count: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionsPostMetadataPollOptionsItem>
        for super::DiscussionsPostMetadataPollOptionsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsPostMetadataPollOptionsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                index: value.index?,
                is_user_voted: value.is_user_voted?,
                text: value.text?,
                votes_count: value.votes_count?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionsPostMetadataPollOptionsItem>
        for DiscussionsPostMetadataPollOptionsItem
    {
        fn from(value: super::DiscussionsPostMetadataPollOptionsItem) -> Self {
            Self {
                id: Ok(value.id),
                index: Ok(value.index),
                is_user_voted: Ok(value.is_user_voted),
                text: Ok(value.text),
                votes_count: Ok(value.votes_count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionsPostMetadataShare {
        shared_origin_post: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        shared_post: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionsPostMetadataShare {
        fn default() -> Self {
            Self {
                shared_origin_post: Ok(Default::default()),
                shared_post: Ok(Default::default()),
            }
        }
    }
    impl DiscussionsPostMetadataShare {
        pub fn shared_origin_post<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.shared_origin_post = value.try_into().map_err(|e| {
                format!("error converting supplied value for shared_origin_post: {e}")
            });
            self
        }
        pub fn shared_post<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.shared_post = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shared_post: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionsPostMetadataShare> for super::DiscussionsPostMetadataShare {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsPostMetadataShare,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                shared_origin_post: value.shared_origin_post?,
                shared_post: value.shared_post?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionsPostMetadataShare> for DiscussionsPostMetadataShare {
        fn from(value: super::DiscussionsPostMetadataShare) -> Self {
            Self {
                shared_origin_post: Ok(value.shared_origin_post),
                shared_post: Ok(value.shared_post),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionsPostTagsItem {
        market: ::std::result::Result<
            ::std::option::Option<super::DiscussionsPostTagsItemMarket>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionsPostTagsItem {
        fn default() -> Self {
            Self {
                market: Ok(Default::default()),
            }
        }
    }
    impl DiscussionsPostTagsItem {
        pub fn market<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionsPostTagsItemMarket>>,
            T::Error: ::std::fmt::Display,
        {
            self.market = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for market: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionsPostTagsItem> for super::DiscussionsPostTagsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsPostTagsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                market: value.market?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionsPostTagsItem> for DiscussionsPostTagsItem {
        fn from(value: super::DiscussionsPostTagsItem) -> Self {
            Self {
                market: Ok(value.market),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionsPostTagsItemMarket {
        application: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        asset_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        asset_type_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        asset_type_sub_category_id:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        avatar: ::std::result::Result<
            ::std::option::Option<super::DiscussionsPostTagsItemMarketAvatar>,
            ::std::string::String,
        >,
        display_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        internal_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        symbol_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        updated: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionsPostTagsItemMarket {
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
    impl DiscussionsPostTagsItemMarket {
        pub fn application<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.application = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for application: {e}"));
            self
        }
        pub fn asset_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.asset_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for asset_type: {e}"));
            self
        }
        pub fn asset_type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.asset_type_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for asset_type_id: {e}"));
            self
        }
        pub fn asset_type_sub_category_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.asset_type_sub_category_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for asset_type_sub_category_id: {e}")
            });
            self
        }
        pub fn avatar<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::DiscussionsPostTagsItemMarketAvatar>,
            >,
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
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.updated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for updated: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionsPostTagsItemMarket>
        for super::DiscussionsPostTagsItemMarket
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsPostTagsItemMarket,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
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
    impl ::std::convert::From<super::DiscussionsPostTagsItemMarket> for DiscussionsPostTagsItemMarket {
        fn from(value: super::DiscussionsPostTagsItemMarket) -> Self {
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
    pub struct DiscussionsPostTagsItemMarketAvatar {
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
            ::std::option::Option<super::DiscussionsPostTagsItemMarketAvatarSvg>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionsPostTagsItemMarketAvatar {
        fn default() -> Self {
            Self {
                large: Ok(Default::default()),
                medium: Ok(Default::default()),
                small: Ok(Default::default()),
                svg: Ok(Default::default()),
            }
        }
    }
    impl DiscussionsPostTagsItemMarketAvatar {
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
                ::std::option::Option<super::DiscussionsPostTagsItemMarketAvatarSvg>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.svg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for svg: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionsPostTagsItemMarketAvatar>
        for super::DiscussionsPostTagsItemMarketAvatar
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsPostTagsItemMarketAvatar,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                large: value.large?,
                medium: value.medium?,
                small: value.small?,
                svg: value.svg?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionsPostTagsItemMarketAvatar>
        for DiscussionsPostTagsItemMarketAvatar
    {
        fn from(value: super::DiscussionsPostTagsItemMarketAvatar) -> Self {
            Self {
                large: Ok(value.large),
                medium: Ok(value.medium),
                small: Ok(value.small),
                svg: Ok(value.svg),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionsPostTagsItemMarketAvatarSvg {
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
    impl ::std::default::Default for DiscussionsPostTagsItemMarketAvatarSvg {
        fn default() -> Self {
            Self {
                background_color: Ok(Default::default()),
                text_color: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl DiscussionsPostTagsItemMarketAvatarSvg {
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
    impl ::std::convert::TryFrom<DiscussionsPostTagsItemMarketAvatarSvg>
        for super::DiscussionsPostTagsItemMarketAvatarSvg
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsPostTagsItemMarketAvatarSvg,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                background_color: value.background_color?,
                text_color: value.text_color?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionsPostTagsItemMarketAvatarSvg>
        for DiscussionsPostTagsItemMarketAvatarSvg
    {
        fn from(value: super::DiscussionsPostTagsItemMarketAvatarSvg) -> Self {
            Self {
                background_color: Ok(value.background_color),
                text_color: Ok(value.text_color),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionsResponse {
        discussions:
            ::std::result::Result<::std::vec::Vec<super::Discussion>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<super::DiscussionsResponseMetadata>,
            ::std::string::String,
        >,
        paging: ::std::result::Result<
            ::std::option::Option<super::DiscussionsResponsePaging>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionsResponse {
        fn default() -> Self {
            Self {
                discussions: Ok(Default::default()),
                metadata: Ok(Default::default()),
                paging: Ok(Default::default()),
            }
        }
    }
    impl DiscussionsResponse {
        pub fn discussions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Discussion>>,
            T::Error: ::std::fmt::Display,
        {
            self.discussions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for discussions: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionsResponseMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn paging<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiscussionsResponsePaging>>,
            T::Error: ::std::fmt::Display,
        {
            self.paging = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for paging: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionsResponse> for super::DiscussionsResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                discussions: value.discussions?,
                metadata: value.metadata?,
                paging: value.paging?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionsResponse> for DiscussionsResponse {
        fn from(value: super::DiscussionsResponse) -> Self {
            Self {
                discussions: Ok(value.discussions),
                metadata: Ok(value.metadata),
                paging: Ok(value.paging),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionsResponseMetadata {
        designated_stream_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        experiment_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        stream_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionsResponseMetadata {
        fn default() -> Self {
            Self {
                designated_stream_type: Ok(Default::default()),
                experiment_name: Ok(Default::default()),
                stream_type: Ok(Default::default()),
            }
        }
    }
    impl DiscussionsResponseMetadata {
        pub fn designated_stream_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.designated_stream_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for designated_stream_type: {e}")
            });
            self
        }
        pub fn experiment_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.experiment_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for experiment_name: {e}"));
            self
        }
        pub fn stream_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.stream_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stream_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionsResponseMetadata> for super::DiscussionsResponseMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsResponseMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                designated_stream_type: value.designated_stream_type?,
                experiment_name: value.experiment_name?,
                stream_type: value.stream_type?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionsResponseMetadata> for DiscussionsResponseMetadata {
        fn from(value: super::DiscussionsResponseMetadata) -> Self {
            Self {
                designated_stream_type: Ok(value.designated_stream_type),
                experiment_name: Ok(value.experiment_name),
                stream_type: Ok(value.stream_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscussionsResponsePaging {
        next: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        off_set: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        take: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscussionsResponsePaging {
        fn default() -> Self {
            Self {
                next: Ok(Default::default()),
                off_set: Ok(Default::default()),
                take: Ok(Default::default()),
                version: Ok(Default::default()),
            }
        }
    }
    impl DiscussionsResponsePaging {
        pub fn next<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next: {e}"));
            self
        }
        pub fn off_set<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.off_set = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for off_set: {e}"));
            self
        }
        pub fn take<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.take = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for take: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscussionsResponsePaging> for super::DiscussionsResponsePaging {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscussionsResponsePaging,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                next: value.next?,
                off_set: value.off_set?,
                take: value.take?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::DiscussionsResponsePaging> for DiscussionsResponsePaging {
        fn from(value: super::DiscussionsResponsePaging) -> Self {
            Self {
                next: Ok(value.next),
                off_set: Ok(value.off_set),
                take: Ok(value.take),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Emotion {
        created: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        obsolete_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        owner: ::std::result::Result<::std::option::Option<super::User>, ::std::string::String>,
        parent: ::std::result::Result<
            ::std::option::Option<super::EmotionParent>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Emotion {
        fn default() -> Self {
            Self {
                created: Ok(Default::default()),
                id: Ok(Default::default()),
                obsolete_id: Ok(Default::default()),
                owner: Ok(Default::default()),
                parent: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl Emotion {
        pub fn created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created: {e}"));
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
        pub fn obsolete_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.obsolete_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for obsolete_id: {e}"));
            self
        }
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::User>>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
            self
        }
        pub fn parent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EmotionParent>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Emotion> for super::Emotion {
        type Error = super::error::ConversionError;
        fn try_from(value: Emotion) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created: value.created?,
                id: value.id?,
                obsolete_id: value.obsolete_id?,
                owner: value.owner?,
                parent: value.parent?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Emotion> for Emotion {
        fn from(value: super::Emotion) -> Self {
            Self {
                created: Ok(value.created),
                id: Ok(value.id),
                obsolete_id: Ok(value.obsolete_id),
                owner: Ok(value.owner),
                parent: Ok(value.parent),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EmotionDataResponse {
        emotions: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Emotion>>,
            ::std::string::String,
        >,
        paging: ::std::result::Result<
            ::std::option::Option<super::ReactionPagingResponse>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EmotionDataResponse {
        fn default() -> Self {
            Self {
                emotions: Ok(Default::default()),
                paging: Ok(Default::default()),
            }
        }
    }
    impl EmotionDataResponse {
        pub fn emotions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Emotion>>>,
            T::Error: ::std::fmt::Display,
        {
            self.emotions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for emotions: {e}"));
            self
        }
        pub fn paging<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReactionPagingResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.paging = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for paging: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EmotionDataResponse> for super::EmotionDataResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EmotionDataResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                emotions: value.emotions?,
                paging: value.paging?,
            })
        }
    }
    impl ::std::convert::From<super::EmotionDataResponse> for EmotionDataResponse {
        fn from(value: super::EmotionDataResponse) -> Self {
            Self {
                emotions: Ok(value.emotions),
                paging: Ok(value.paging),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EmotionParent {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        obsolete_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EmotionParent {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                obsolete_id: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl EmotionParent {
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
        pub fn obsolete_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.obsolete_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for obsolete_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EmotionParent> for super::EmotionParent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EmotionParent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                obsolete_id: value.obsolete_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EmotionParent> for EmotionParent {
        fn from(value: super::EmotionParent) -> Self {
            Self {
                id: Ok(value.id),
                obsolete_id: Ok(value.obsolete_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EmotionsDataResponse {
        like: ::std::result::Result<
            ::std::option::Option<super::EmotionDataResponse>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EmotionsDataResponse {
        fn default() -> Self {
            Self {
                like: Ok(Default::default()),
            }
        }
    }
    impl EmotionsDataResponse {
        pub fn like<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EmotionDataResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.like = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for like: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EmotionsDataResponse> for super::EmotionsDataResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EmotionsDataResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { like: value.like? })
        }
    }
    impl ::std::convert::From<super::EmotionsDataResponse> for EmotionsDataResponse {
        fn from(value: super::EmotionsDataResponse) -> Self {
            Self {
                like: Ok(value.like),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EntityCommentsDataResponse {
        comments: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::CommentDataResponse>>,
            ::std::string::String,
        >,
        reaction_paging: ::std::result::Result<
            ::std::option::Option<super::ReactionPagingResponse>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EntityCommentsDataResponse {
        fn default() -> Self {
            Self {
                comments: Ok(Default::default()),
                reaction_paging: Ok(Default::default()),
            }
        }
    }
    impl EntityCommentsDataResponse {
        pub fn comments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::CommentDataResponse>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.comments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for comments: {e}"));
            self
        }
        pub fn reaction_paging<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReactionPagingResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.reaction_paging = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reaction_paging: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EntityCommentsDataResponse> for super::EntityCommentsDataResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EntityCommentsDataResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                comments: value.comments?,
                reaction_paging: value.reaction_paging?,
            })
        }
    }
    impl ::std::convert::From<super::EntityCommentsDataResponse> for EntityCommentsDataResponse {
        fn from(value: super::EntityCommentsDataResponse) -> Self {
            Self {
                comments: Ok(value.comments),
                reaction_paging: Ok(value.reaction_paging),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FeedMetadataResponse {
        designated_stream_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        experiment_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        stream_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FeedMetadataResponse {
        fn default() -> Self {
            Self {
                designated_stream_type: Ok(Default::default()),
                experiment_name: Ok(Default::default()),
                stream_type: Ok(Default::default()),
            }
        }
    }
    impl FeedMetadataResponse {
        pub fn designated_stream_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.designated_stream_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for designated_stream_type: {e}")
            });
            self
        }
        pub fn experiment_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.experiment_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for experiment_name: {e}"));
            self
        }
        pub fn stream_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.stream_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stream_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FeedMetadataResponse> for super::FeedMetadataResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FeedMetadataResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                designated_stream_type: value.designated_stream_type?,
                experiment_name: value.experiment_name?,
                stream_type: value.stream_type?,
            })
        }
    }
    impl ::std::convert::From<super::FeedMetadataResponse> for FeedMetadataResponse {
        fn from(value: super::FeedMetadataResponse) -> Self {
            Self {
                designated_stream_type: Ok(value.designated_stream_type),
                experiment_name: Ok(value.experiment_name),
                stream_type: Ok(value.stream_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FeedPagingResponse {
        next: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        off_set: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        take: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FeedPagingResponse {
        fn default() -> Self {
            Self {
                next: Ok(Default::default()),
                off_set: Ok(Default::default()),
                take: Ok(Default::default()),
                version: Ok(Default::default()),
            }
        }
    }
    impl FeedPagingResponse {
        pub fn next<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next: {e}"));
            self
        }
        pub fn off_set<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.off_set = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for off_set: {e}"));
            self
        }
        pub fn take<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.take = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for take: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FeedPagingResponse> for super::FeedPagingResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FeedPagingResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                next: value.next?,
                off_set: value.off_set?,
                take: value.take?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::FeedPagingResponse> for FeedPagingResponse {
        fn from(value: super::FeedPagingResponse) -> Self {
            Self {
                next: Ok(value.next),
                off_set: Ok(value.off_set),
                take: Ok(value.take),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FeedResponse {
        discussions: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::DiscussionResponse>>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            ::std::option::Option<super::FeedMetadataResponse>,
            ::std::string::String,
        >,
        paging: ::std::result::Result<
            ::std::option::Option<super::FeedPagingResponse>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FeedResponse {
        fn default() -> Self {
            Self {
                discussions: Ok(Default::default()),
                metadata: Ok(Default::default()),
                paging: Ok(Default::default()),
            }
        }
    }
    impl FeedResponse {
        pub fn discussions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::DiscussionResponse>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.discussions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for discussions: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FeedMetadataResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn paging<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FeedPagingResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.paging = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for paging: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FeedResponse> for super::FeedResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FeedResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                discussions: value.discussions?,
                metadata: value.metadata?,
                paging: value.paging?,
            })
        }
    }
    impl ::std::convert::From<super::FeedResponse> for FeedResponse {
        fn from(value: super::FeedResponse) -> Self {
            Self {
                discussions: Ok(value.discussions),
                metadata: Ok(value.metadata),
                paging: Ok(value.paging),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ImageMetadata {
        height: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        width: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
    }
    impl ::std::default::Default for ImageMetadata {
        fn default() -> Self {
            Self {
                height: Ok(Default::default()),
                url: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl ImageMetadata {
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height: {e}"));
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
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for width: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ImageMetadata> for super::ImageMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ImageMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                height: value.height?,
                url: value.url?,
                width: value.width?,
            })
        }
    }
    impl ::std::convert::From<super::ImageMetadata> for ImageMetadata {
        fn from(value: super::ImageMetadata) -> Self {
            Self {
                height: Ok(value.height),
                url: Ok(value.url),
                width: Ok(value.width),
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
    pub struct Media {
        image: ::std::result::Result<
            ::std::option::Option<super::ImageMetadata>,
            ::std::string::String,
        >,
        video: ::std::result::Result<
            ::std::option::Option<super::VideoMetadata>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Media {
        fn default() -> Self {
            Self {
                image: Ok(Default::default()),
                video: Ok(Default::default()),
            }
        }
    }
    impl Media {
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ImageMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {e}"));
            self
        }
        pub fn video<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VideoMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.video = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Media> for super::Media {
        type Error = super::error::ConversionError;
        fn try_from(value: Media) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                image: value.image?,
                video: value.video?,
            })
        }
    }
    impl ::std::convert::From<super::Media> for Media {
        fn from(value: super::Media) -> Self {
            Self {
                image: Ok(value.image),
                video: Ok(value.video),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Mention {
        is_direct: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        user: ::std::result::Result<::std::option::Option<super::User>, ::std::string::String>,
    }
    impl ::std::default::Default for Mention {
        fn default() -> Self {
            Self {
                is_direct: Ok(Default::default()),
                user: Ok(Default::default()),
            }
        }
    }
    impl Mention {
        pub fn is_direct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_direct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_direct: {e}"));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::User>>,
            T::Error: ::std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Mention> for super::Mention {
        type Error = super::error::ConversionError;
        fn try_from(value: Mention) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                is_direct: value.is_direct?,
                user: value.user?,
            })
        }
    }
    impl ::std::convert::From<super::Mention> for Mention {
        fn from(value: super::Mention) -> Self {
            Self {
                is_direct: Ok(value.is_direct),
                user: Ok(value.user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Mentions {
        mentions: ::std::result::Result<
            ::std::vec::Vec<super::MentionsMentionsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Mentions {
        fn default() -> Self {
            Self {
                mentions: Ok(Default::default()),
            }
        }
    }
    impl Mentions {
        pub fn mentions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::MentionsMentionsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.mentions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mentions: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Mentions> for super::Mentions {
        type Error = super::error::ConversionError;
        fn try_from(value: Mentions) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mentions: value.mentions?,
            })
        }
    }
    impl ::std::convert::From<super::Mentions> for Mentions {
        fn from(value: super::Mentions) -> Self {
            Self {
                mentions: Ok(value.mentions),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MentionsDataRequest {
        mentions: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::MentionsRequest>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MentionsDataRequest {
        fn default() -> Self {
            Self {
                mentions: Ok(Default::default()),
            }
        }
    }
    impl MentionsDataRequest {
        pub fn mentions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::MentionsRequest>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.mentions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mentions: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MentionsDataRequest> for super::MentionsDataRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MentionsDataRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mentions: value.mentions?,
            })
        }
    }
    impl ::std::convert::From<super::MentionsDataRequest> for MentionsDataRequest {
        fn from(value: super::MentionsDataRequest) -> Self {
            Self {
                mentions: Ok(value.mentions),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MentionsMentionsItem {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        is_direct: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        user_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MentionsMentionsItem {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                is_direct: Ok(Default::default()),
                user_name: Ok(Default::default()),
            }
        }
    }
    impl MentionsMentionsItem {
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
        pub fn is_direct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_direct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_direct: {e}"));
            self
        }
        pub fn user_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MentionsMentionsItem> for super::MentionsMentionsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MentionsMentionsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                is_direct: value.is_direct?,
                user_name: value.user_name?,
            })
        }
    }
    impl ::std::convert::From<super::MentionsMentionsItem> for MentionsMentionsItem {
        fn from(value: super::MentionsMentionsItem) -> Self {
            Self {
                id: Ok(value.id),
                is_direct: Ok(value.is_direct),
                user_name: Ok(value.user_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MentionsRequest {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        is_direct: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        user_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MentionsRequest {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                is_direct: Ok(Default::default()),
                user_name: Ok(Default::default()),
            }
        }
    }
    impl MentionsRequest {
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
        pub fn is_direct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_direct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_direct: {e}"));
            self
        }
        pub fn user_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MentionsRequest> for super::MentionsRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MentionsRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                is_direct: value.is_direct?,
                user_name: value.user_name?,
            })
        }
    }
    impl ::std::convert::From<super::MentionsRequest> for MentionsRequest {
        fn from(value: super::MentionsRequest) -> Self {
            Self {
                id: Ok(value.id),
                is_direct: Ok(value.is_direct),
                user_name: Ok(value.user_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Message {
        language_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Message {
        fn default() -> Self {
            Self {
                language_code: Ok(Default::default()),
                text: Ok(Default::default()),
            }
        }
    }
    impl Message {
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
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Message> for super::Message {
        type Error = super::error::ConversionError;
        fn try_from(value: Message) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                language_code: value.language_code?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::Message> for Message {
        fn from(value: super::Message) -> Self {
            Self {
                language_code: Ok(value.language_code),
                text: Ok(value.text),
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
    pub struct Parents {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        obsolete_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        parent: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::Parents>>,
            ::std::string::String,
        >,
        type_:
            ::std::result::Result<::std::option::Option<super::ParentType>, ::std::string::String>,
    }
    impl ::std::default::Default for Parents {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                obsolete_id: Ok(Default::default()),
                parent: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl Parents {
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
        pub fn obsolete_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.obsolete_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for obsolete_id: {e}"));
            self
        }
        pub fn parent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::boxed::Box<super::Parents>>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ParentType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Parents> for super::Parents {
        type Error = super::error::ConversionError;
        fn try_from(value: Parents) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                obsolete_id: value.obsolete_id?,
                parent: value.parent?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Parents> for Parents {
        fn from(value: super::Parents) -> Self {
            Self {
                id: Ok(value.id),
                obsolete_id: Ok(value.obsolete_id),
                parent: Ok(value.parent),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PollMetadata {
        gcid: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        options: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::PollOption>>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        total_votes: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
    }
    impl ::std::default::Default for PollMetadata {
        fn default() -> Self {
            Self {
                gcid: Ok(Default::default()),
                id: Ok(Default::default()),
                options: Ok(Default::default()),
                title: Ok(Default::default()),
                total_votes: Ok(Default::default()),
            }
        }
    }
    impl PollMetadata {
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
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::PollOption>>>,
            T::Error: ::std::fmt::Display,
        {
            self.options = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for options: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn total_votes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_votes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_votes: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PollMetadata> for super::PollMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PollMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                gcid: value.gcid?,
                id: value.id?,
                options: value.options?,
                title: value.title?,
                total_votes: value.total_votes?,
            })
        }
    }
    impl ::std::convert::From<super::PollMetadata> for PollMetadata {
        fn from(value: super::PollMetadata) -> Self {
            Self {
                gcid: Ok(value.gcid),
                id: Ok(value.id),
                options: Ok(value.options),
                title: Ok(value.title),
                total_votes: Ok(value.total_votes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PollOption {
        id: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        index: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        is_user_voted: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        votes_count: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
    }
    impl ::std::default::Default for PollOption {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                index: Ok(Default::default()),
                is_user_voted: Ok(Default::default()),
                text: Ok(Default::default()),
                votes_count: Ok(Default::default()),
            }
        }
    }
    impl PollOption {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for index: {e}"));
            self
        }
        pub fn is_user_voted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_user_voted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_user_voted: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn votes_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.votes_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for votes_count: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PollOption> for super::PollOption {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PollOption,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                index: value.index?,
                is_user_voted: value.is_user_voted?,
                text: value.text?,
                votes_count: value.votes_count?,
            })
        }
    }
    impl ::std::convert::From<super::PollOption> for PollOption {
        fn from(value: super::PollOption) -> Self {
            Self {
                id: Ok(value.id),
                index: Ok(value.index),
                is_user_voted: Ok(value.is_user_voted),
                text: Ok(value.text),
                votes_count: Ok(value.votes_count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Post {
        attachments:
            ::std::result::Result<::std::vec::Vec<super::Attachment>, ::std::string::String>,
        created: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        edit_status: ::std::result::Result<
            ::std::option::Option<super::PostEditStatus>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        is_deleted: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_spam: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        mentions:
            ::std::result::Result<::std::vec::Vec<super::PostMentionsItem>, ::std::string::String>,
        message:
            ::std::result::Result<::std::option::Option<super::PostMessage>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<super::PostMetadata>,
            ::std::string::String,
        >,
        obsolete_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        owner: ::std::result::Result<::std::option::Option<super::User>, ::std::string::String>,
        tags: ::std::result::Result<::std::vec::Vec<super::PostTagsItem>, ::std::string::String>,
        type_: ::std::result::Result<::std::option::Option<super::PostType>, ::std::string::String>,
        updated: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Post {
        fn default() -> Self {
            Self {
                attachments: Ok(Default::default()),
                created: Ok(Default::default()),
                edit_status: Ok(Default::default()),
                id: Ok(Default::default()),
                is_deleted: Ok(Default::default()),
                is_spam: Ok(Default::default()),
                mentions: Ok(Default::default()),
                message: Ok(Default::default()),
                metadata: Ok(Default::default()),
                obsolete_id: Ok(Default::default()),
                owner: Ok(Default::default()),
                tags: Ok(Default::default()),
                type_: Ok(Default::default()),
                updated: Ok(Default::default()),
            }
        }
    }
    impl Post {
        pub fn attachments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Attachment>>,
            T::Error: ::std::fmt::Display,
        {
            self.attachments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attachments: {e}"));
            self
        }
        pub fn created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created: {e}"));
            self
        }
        pub fn edit_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PostEditStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.edit_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for edit_status: {e}"));
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
        pub fn is_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_deleted: {e}"));
            self
        }
        pub fn is_spam<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_spam = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_spam: {e}"));
            self
        }
        pub fn mentions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PostMentionsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.mentions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mentions: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PostMessage>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PostMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn obsolete_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.obsolete_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for obsolete_id: {e}"));
            self
        }
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::User>>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PostTagsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PostType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
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
    impl ::std::convert::TryFrom<Post> for super::Post {
        type Error = super::error::ConversionError;
        fn try_from(value: Post) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attachments: value.attachments?,
                created: value.created?,
                edit_status: value.edit_status?,
                id: value.id?,
                is_deleted: value.is_deleted?,
                is_spam: value.is_spam?,
                mentions: value.mentions?,
                message: value.message?,
                metadata: value.metadata?,
                obsolete_id: value.obsolete_id?,
                owner: value.owner?,
                tags: value.tags?,
                type_: value.type_?,
                updated: value.updated?,
            })
        }
    }
    impl ::std::convert::From<super::Post> for Post {
        fn from(value: super::Post) -> Self {
            Self {
                attachments: Ok(value.attachments),
                created: Ok(value.created),
                edit_status: Ok(value.edit_status),
                id: Ok(value.id),
                is_deleted: Ok(value.is_deleted),
                is_spam: Ok(value.is_spam),
                mentions: Ok(value.mentions),
                message: Ok(value.message),
                metadata: Ok(value.metadata),
                obsolete_id: Ok(value.obsolete_id),
                owner: Ok(value.owner),
                tags: Ok(value.tags),
                type_: Ok(value.type_),
                updated: Ok(value.updated),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PostMentionsItem {
        is_direct: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        user: ::std::result::Result<::std::option::Option<super::User>, ::std::string::String>,
    }
    impl ::std::default::Default for PostMentionsItem {
        fn default() -> Self {
            Self {
                is_direct: Ok(Default::default()),
                user: Ok(Default::default()),
            }
        }
    }
    impl PostMentionsItem {
        pub fn is_direct<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_direct = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_direct: {e}"));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::User>>,
            T::Error: ::std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PostMentionsItem> for super::PostMentionsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PostMentionsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                is_direct: value.is_direct?,
                user: value.user?,
            })
        }
    }
    impl ::std::convert::From<super::PostMentionsItem> for PostMentionsItem {
        fn from(value: super::PostMentionsItem) -> Self {
            Self {
                is_direct: Ok(value.is_direct),
                user: Ok(value.user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PostMessage {
        language_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PostMessage {
        fn default() -> Self {
            Self {
                language_code: Ok(Default::default()),
                text: Ok(Default::default()),
            }
        }
    }
    impl PostMessage {
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
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PostMessage> for super::PostMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PostMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                language_code: value.language_code?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::PostMessage> for PostMessage {
        fn from(value: super::PostMessage) -> Self {
            Self {
                language_code: Ok(value.language_code),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PostMetadata {
        article: ::std::result::Result<
            ::std::option::Option<super::ArticleMetadata>,
            ::std::string::String,
        >,
        copy: ::std::result::Result<
            ::std::option::Option<super::CopyMetadata>,
            ::std::string::String,
        >,
        market_event: ::std::result::Result<
            ::std::option::Option<super::MarketEventMetadata>,
            ::std::string::String,
        >,
        order: ::std::result::Result<
            ::std::option::Option<super::OrderMetadata>,
            ::std::string::String,
        >,
        poll: ::std::result::Result<
            ::std::option::Option<super::PollMetadata>,
            ::std::string::String,
        >,
        share: ::std::result::Result<
            ::std::option::Option<super::ShareMetadata>,
            ::std::string::String,
        >,
        trade: ::std::result::Result<
            ::std::option::Option<super::TradeMetadata>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PostMetadata {
        fn default() -> Self {
            Self {
                article: Ok(Default::default()),
                copy: Ok(Default::default()),
                market_event: Ok(Default::default()),
                order: Ok(Default::default()),
                poll: Ok(Default::default()),
                share: Ok(Default::default()),
                trade: Ok(Default::default()),
            }
        }
    }
    impl PostMetadata {
        pub fn article<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ArticleMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.article = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for article: {e}"));
            self
        }
        pub fn copy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CopyMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.copy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for copy: {e}"));
            self
        }
        pub fn market_event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MarketEventMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.market_event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for market_event: {e}"));
            self
        }
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OrderMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn poll<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PollMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.poll = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for poll: {e}"));
            self
        }
        pub fn share<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ShareMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.share = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for share: {e}"));
            self
        }
        pub fn trade<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TradeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.trade = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trade: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PostMetadata> for super::PostMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PostMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                article: value.article?,
                copy: value.copy?,
                market_event: value.market_event?,
                order: value.order?,
                poll: value.poll?,
                share: value.share?,
                trade: value.trade?,
            })
        }
    }
    impl ::std::convert::From<super::PostMetadata> for PostMetadata {
        fn from(value: super::PostMetadata) -> Self {
            Self {
                article: Ok(value.article),
                copy: Ok(value.copy),
                market_event: Ok(value.market_event),
                order: Ok(value.order),
                poll: Ok(value.poll),
                share: Ok(value.share),
                trade: Ok(value.trade),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PostTagsItem {
        market: ::std::result::Result<::std::option::Option<super::Market>, ::std::string::String>,
    }
    impl ::std::default::Default for PostTagsItem {
        fn default() -> Self {
            Self {
                market: Ok(Default::default()),
            }
        }
    }
    impl PostTagsItem {
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
    }
    impl ::std::convert::TryFrom<PostTagsItem> for super::PostTagsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PostTagsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                market: value.market?,
            })
        }
    }
    impl ::std::convert::From<super::PostTagsItem> for PostTagsItem {
        fn from(value: super::PostTagsItem) -> Self {
            Self {
                market: Ok(value.market),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReactionPagingResponse {
        next: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        offset_entity_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        total_count: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
    }
    impl ::std::default::Default for ReactionPagingResponse {
        fn default() -> Self {
            Self {
                next: Ok(Default::default()),
                offset_entity_id: Ok(Default::default()),
                total_count: Ok(Default::default()),
            }
        }
    }
    impl ReactionPagingResponse {
        pub fn next<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next: {e}"));
            self
        }
        pub fn offset_entity_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.offset_entity_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for offset_entity_id: {e}"));
            self
        }
        pub fn total_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_count: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ReactionPagingResponse> for super::ReactionPagingResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReactionPagingResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                next: value.next?,
                offset_entity_id: value.offset_entity_id?,
                total_count: value.total_count?,
            })
        }
    }
    impl ::std::convert::From<super::ReactionPagingResponse> for ReactionPagingResponse {
        fn from(value: super::ReactionPagingResponse) -> Self {
            Self {
                next: Ok(value.next),
                offset_entity_id: Ok(value.offset_entity_id),
                total_count: Ok(value.total_count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Reason {
        owner: ::std::result::Result<::std::option::Option<super::User>, ::std::string::String>,
        source_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_:
            ::std::result::Result<::std::option::Option<super::ReasonType>, ::std::string::String>,
    }
    impl ::std::default::Default for Reason {
        fn default() -> Self {
            Self {
                owner: Ok(Default::default()),
                source_id: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl Reason {
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::User>>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
            self
        }
        pub fn source_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.source_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReasonType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Reason> for super::Reason {
        type Error = super::error::ConversionError;
        fn try_from(value: Reason) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                owner: value.owner?,
                source_id: value.source_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Reason> for Reason {
        fn from(value: super::Reason) -> Self {
            Self {
                owner: Ok(value.owner),
                source_id: Ok(value.source_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RequestBase {
        attachments: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Attachment>>,
            ::std::string::String,
        >,
        mentions: ::std::result::Result<
            ::std::option::Option<super::MentionsDataRequest>,
            ::std::string::String,
        >,
        message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        owner: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        tags: ::std::result::Result<
            ::std::option::Option<super::TagsDataRequest>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RequestBase {
        fn default() -> Self {
            Self {
                attachments: Ok(Default::default()),
                mentions: Ok(Default::default()),
                message: Ok(Default::default()),
                owner: Ok(Default::default()),
                tags: Ok(Default::default()),
            }
        }
    }
    impl RequestBase {
        pub fn attachments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Attachment>>>,
            T::Error: ::std::fmt::Display,
        {
            self.attachments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attachments: {e}"));
            self
        }
        pub fn mentions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MentionsDataRequest>>,
            T::Error: ::std::fmt::Display,
        {
            self.mentions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mentions: {e}"));
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
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TagsDataRequest>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RequestBase> for super::RequestBase {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RequestBase,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attachments: value.attachments?,
                mentions: value.mentions?,
                message: value.message?,
                owner: value.owner?,
                tags: value.tags?,
            })
        }
    }
    impl ::std::convert::From<super::RequestBase> for RequestBase {
        fn from(value: super::RequestBase) -> Self {
            Self {
                attachments: Ok(value.attachments),
                mentions: Ok(value.mentions),
                message: Ok(value.message),
                owner: Ok(value.owner),
                tags: Ok(value.tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RequesterContextResponse {
        is_flagging_as_spam:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_following: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_interaction_restricted:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_liking: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_owner: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_pinned: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_requester_blocking:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_saved: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        is_subscribed: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for RequesterContextResponse {
        fn default() -> Self {
            Self {
                is_flagging_as_spam: Ok(Default::default()),
                is_following: Ok(Default::default()),
                is_interaction_restricted: Ok(Default::default()),
                is_liking: Ok(Default::default()),
                is_owner: Ok(Default::default()),
                is_pinned: Ok(Default::default()),
                is_requester_blocking: Ok(Default::default()),
                is_saved: Ok(Default::default()),
                is_subscribed: Ok(Default::default()),
            }
        }
    }
    impl RequesterContextResponse {
        pub fn is_flagging_as_spam<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_flagging_as_spam = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_flagging_as_spam: {e}")
            });
            self
        }
        pub fn is_following<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_following = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_following: {e}"));
            self
        }
        pub fn is_interaction_restricted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_interaction_restricted = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_interaction_restricted: {e}")
            });
            self
        }
        pub fn is_liking<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_liking = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_liking: {e}"));
            self
        }
        pub fn is_owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_owner: {e}"));
            self
        }
        pub fn is_pinned<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_pinned = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_pinned: {e}"));
            self
        }
        pub fn is_requester_blocking<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_requester_blocking = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_requester_blocking: {e}")
            });
            self
        }
        pub fn is_saved<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_saved = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_saved: {e}"));
            self
        }
        pub fn is_subscribed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_subscribed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_subscribed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RequesterContextResponse> for super::RequesterContextResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RequesterContextResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                is_flagging_as_spam: value.is_flagging_as_spam?,
                is_following: value.is_following?,
                is_interaction_restricted: value.is_interaction_restricted?,
                is_liking: value.is_liking?,
                is_owner: value.is_owner?,
                is_pinned: value.is_pinned?,
                is_requester_blocking: value.is_requester_blocking?,
                is_saved: value.is_saved?,
                is_subscribed: value.is_subscribed?,
            })
        }
    }
    impl ::std::convert::From<super::RequesterContextResponse> for RequesterContextResponse {
        fn from(value: super::RequesterContextResponse) -> Self {
            Self {
                is_flagging_as_spam: Ok(value.is_flagging_as_spam),
                is_following: Ok(value.is_following),
                is_interaction_restricted: Ok(value.is_interaction_restricted),
                is_liking: Ok(value.is_liking),
                is_owner: Ok(value.is_owner),
                is_pinned: Ok(value.is_pinned),
                is_requester_blocking: Ok(value.is_requester_blocking),
                is_saved: Ok(value.is_saved),
                is_subscribed: Ok(value.is_subscribed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ShareMetadata {
        shared_origin_post: ::std::result::Result<
            ::std::boxed::Box<::std::option::Option<super::Post>>,
            ::std::string::String,
        >,
        shared_post: ::std::result::Result<
            ::std::boxed::Box<::std::option::Option<super::Post>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ShareMetadata {
        fn default() -> Self {
            Self {
                shared_origin_post: Ok(Default::default()),
                shared_post: Ok(Default::default()),
            }
        }
    }
    impl ShareMetadata {
        pub fn shared_origin_post<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<::std::option::Option<super::Post>>>,
            T::Error: ::std::fmt::Display,
        {
            self.shared_origin_post = value.try_into().map_err(|e| {
                format!("error converting supplied value for shared_origin_post: {e}")
            });
            self
        }
        pub fn shared_post<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::boxed::Box<::std::option::Option<super::Post>>>,
            T::Error: ::std::fmt::Display,
        {
            self.shared_post = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shared_post: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ShareMetadata> for super::ShareMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ShareMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                shared_origin_post: value.shared_origin_post?,
                shared_post: value.shared_post?,
            })
        }
    }
    impl ::std::convert::From<super::ShareMetadata> for ShareMetadata {
        fn from(value: super::ShareMetadata) -> Self {
            Self {
                shared_origin_post: Ok(value.shared_origin_post),
                shared_post: Ok(value.shared_post),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SummaryResponse {
        shared_count: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        total_comments_and_replies:
            ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
    }
    impl ::std::default::Default for SummaryResponse {
        fn default() -> Self {
            Self {
                shared_count: Ok(Default::default()),
                total_comments_and_replies: Ok(Default::default()),
            }
        }
    }
    impl SummaryResponse {
        pub fn shared_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.shared_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shared_count: {e}"));
            self
        }
        pub fn total_comments_and_replies<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_comments_and_replies = value.try_into().map_err(|e| {
                format!("error converting supplied value for total_comments_and_replies: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<SummaryResponse> for super::SummaryResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SummaryResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                shared_count: value.shared_count?,
                total_comments_and_replies: value.total_comments_and_replies?,
            })
        }
    }
    impl ::std::convert::From<super::SummaryResponse> for SummaryResponse {
        fn from(value: super::SummaryResponse) -> Self {
            Self {
                shared_count: Ok(value.shared_count),
                total_comments_and_replies: Ok(value.total_comments_and_replies),
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
    pub struct Tag {
        market: ::std::result::Result<::std::option::Option<super::Market>, ::std::string::String>,
    }
    impl ::std::default::Default for Tag {
        fn default() -> Self {
            Self {
                market: Ok(Default::default()),
            }
        }
    }
    impl Tag {
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
    }
    impl ::std::convert::TryFrom<Tag> for super::Tag {
        type Error = super::error::ConversionError;
        fn try_from(value: Tag) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                market: value.market?,
            })
        }
    }
    impl ::std::convert::From<super::Tag> for Tag {
        fn from(value: super::Tag) -> Self {
            Self {
                market: Ok(value.market),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TagRequest {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TagRequest {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl TagRequest {
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
    }
    impl ::std::convert::TryFrom<TagRequest> for super::TagRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TagRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::TagRequest> for TagRequest {
        fn from(value: super::TagRequest) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Tags {
        tags: ::std::result::Result<::std::vec::Vec<super::TagsTagsItem>, ::std::string::String>,
    }
    impl ::std::default::Default for Tags {
        fn default() -> Self {
            Self {
                tags: Ok(Default::default()),
            }
        }
    }
    impl Tags {
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TagsTagsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Tags> for super::Tags {
        type Error = super::error::ConversionError;
        fn try_from(value: Tags) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { tags: value.tags? })
        }
    }
    impl ::std::convert::From<super::Tags> for Tags {
        fn from(value: super::Tags) -> Self {
            Self {
                tags: Ok(value.tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TagsDataRequest {
        tags: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::TagRequest>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TagsDataRequest {
        fn default() -> Self {
            Self {
                tags: Ok(Default::default()),
            }
        }
    }
    impl TagsDataRequest {
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::TagRequest>>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TagsDataRequest> for super::TagsDataRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TagsDataRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { tags: value.tags? })
        }
    }
    impl ::std::convert::From<super::TagsDataRequest> for TagsDataRequest {
        fn from(value: super::TagsDataRequest) -> Self {
            Self {
                tags: Ok(value.tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TagsTagsItem {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TagsTagsItem {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl TagsTagsItem {
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
    }
    impl ::std::convert::TryFrom<TagsTagsItem> for super::TagsTagsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TagsTagsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::TagsTagsItem> for TagsTagsItem {
        fn from(value: super::TagsTagsItem) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
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
    #[derive(Clone, Debug)]
    pub struct VideoMetadata {
        image: ::std::result::Result<
            ::std::option::Option<super::ImageMetadata>,
            ::std::string::String,
        >,
        video_source:
            ::std::result::Result<::std::option::Option<super::VideoSource>, ::std::string::String>,
        video_source_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VideoMetadata {
        fn default() -> Self {
            Self {
                image: Ok(Default::default()),
                video_source: Ok(Default::default()),
                video_source_id: Ok(Default::default()),
            }
        }
    }
    impl VideoMetadata {
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ImageMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {e}"));
            self
        }
        pub fn video_source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VideoSource>>,
            T::Error: ::std::fmt::Display,
        {
            self.video_source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_source: {e}"));
            self
        }
        pub fn video_source_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.video_source_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_source_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VideoMetadata> for super::VideoMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VideoMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                image: value.image?,
                video_source: value.video_source?,
                video_source_id: value.video_source_id?,
            })
        }
    }
    impl ::std::convert::From<super::VideoMetadata> for VideoMetadata {
        fn from(value: super::VideoMetadata) -> Self {
            Self {
                image: Ok(value.image),
                video_source: Ok(value.video_source),
                video_source_id: Ok(value.video_source_id),
            }
        }
    }
}
