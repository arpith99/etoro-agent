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
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-rust-type\": {"]
#[doc = "        \"crate\": \"etoro-agent\","]
#[doc = "        \"path\": \"etoro_agent::types::manual::Numeric\","]
#[doc = "        \"version\": \"0.1.0\""]
#[doc = "      }"]
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
    pub edit_status: ::std::option::Option<::etoro_agent::types::manual::EditStatus>,
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
    pub rating: ::std::option::Option<::etoro_agent::types::manual::ArticleRating>,
    #[serde(
        rename = "readingTimeMinutes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reading_time_minutes: ::std::option::Option<::etoro_agent::types::manual::Numeric>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::etoro_agent::types::manual::ArticleStatus>,
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
#[doc = "      \"$ref\": \"#/$defs/Media\""]
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
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub media: ::std::option::Option<Media>,
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
#[doc = "        \"$ref\": \"#/$defs/Comment\""]
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
    pub replies: ::std::vec::Vec<Comment>,
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
    pub owner: ::std::option::Option<::etoro_agent::types::identity::User>,
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
    pub type_: ::std::option::Option<::etoro_agent::types::manual::CopyType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<::etoro_agent::types::identity::User>,
}
impl ::std::default::Default for CopyMetadata {
    fn default() -> Self {
        Self {
            type_: Default::default(),
            user: Default::default(),
        }
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
    pub owner: ::std::option::Option<::etoro_agent::types::identity::User>,
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
    pub user: ::std::option::Option<::etoro_agent::types::identity::User>,
}
impl ::std::default::Default for DiscussionsPostMentionsItem {
    fn default() -> Self {
        Self {
            is_direct: Default::default(),
            user: Default::default(),
        }
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
    pub owner: ::std::option::Option<::etoro_agent::types::identity::User>,
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
    pub user: ::std::option::Option<::etoro_agent::types::identity::User>,
}
impl ::std::default::Default for Mention {
    fn default() -> Self {
        Self {
            is_direct: Default::default(),
            user: Default::default(),
        }
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
    pub type_: ::std::option::Option<::etoro_agent::types::manual::ParentType>,
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
    pub owner: ::std::option::Option<::etoro_agent::types::identity::User>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tags: ::std::vec::Vec<PostTagsItem>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::etoro_agent::types::manual::PostType>,
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
    pub user: ::std::option::Option<::etoro_agent::types::identity::User>,
}
impl ::std::default::Default for PostMentionsItem {
    fn default() -> Self {
        Self {
            is_direct: Default::default(),
            user: Default::default(),
        }
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
    pub market_event: ::std::option::Option<::etoro_agent::types::market_data::MarketEventMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<::etoro_agent::types::trading::OrderMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub poll: ::std::option::Option<PollMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub share: ::std::option::Option<ShareMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trade: ::std::option::Option<::etoro_agent::types::trading::TradeMetadata>,
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
    pub market: ::std::option::Option<::etoro_agent::types::market_data::Market>,
}
impl ::std::default::Default for PostTagsItem {
    fn default() -> Self {
        Self {
            market: Default::default(),
        }
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
    pub owner: ::std::option::Option<::etoro_agent::types::identity::User>,
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
    pub type_: ::std::option::Option<::etoro_agent::types::manual::ReasonType>,
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
    pub market: ::std::option::Option<::etoro_agent::types::market_data::Market>,
}
impl ::std::default::Default for Tag {
    fn default() -> Self {
        Self {
            market: Default::default(),
        }
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
    pub video_source: ::std::option::Option<::etoro_agent::types::manual::VideoSource>,
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
