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
#[doc = "`AgentPortfolioItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"agentPortfolioGcid\": {"]
#[doc = "      \"description\": \"The GCID associated with the agent-portfolio.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"example\": 12345678"]
#[doc = "    },"]
#[doc = "    \"agentPortfolioId\": {"]
#[doc = "      \"description\": \"The unique identifier of the agent-portfolio.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uuid\","]
#[doc = "      \"example\": \"a1b2c3d4-e5f6-7890-abcd-ef1234567890\""]
#[doc = "    },"]
#[doc = "    \"agentPortfolioName\": {"]
#[doc = "      \"description\": \"The display name of the agent-portfolio.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"MyPort1\""]
#[doc = "    },"]
#[doc = "    \"agentPortfolioVirtualBalance\": {"]
#[doc = "      \"description\": \"The fixed virtual balance (in USD) that the agent-portfolio was funded with. The investmentAmountInUsd used to copy is proportional to this balance.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 10000"]
#[doc = "    },"]
#[doc = "    \"createdAt\": {"]
#[doc = "      \"description\": \"When this agent-portfolio was created.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\","]
#[doc = "      \"example\": \"2026-03-01T10:30:00+00:00\""]
#[doc = "    },"]
#[doc = "    \"mirrorId\": {"]
#[doc = "      \"description\": \"The Trading API mirror ID for this agent-portfolio's copy trade.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"example\": 12345"]
#[doc = "    },"]
#[doc = "    \"userTokens\": {"]
#[doc = "      \"description\": \"The user tokens associated with this agent-portfolio.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/AgentPortfolioUserTokenItem\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AgentPortfolioItem {
    #[doc = "The GCID associated with the agent-portfolio."]
    #[serde(
        rename = "agentPortfolioGcid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_portfolio_gcid: ::std::option::Option<i64>,
    #[doc = "The unique identifier of the agent-portfolio."]
    #[serde(
        rename = "agentPortfolioId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_portfolio_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "The display name of the agent-portfolio."]
    #[serde(
        rename = "agentPortfolioName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_portfolio_name: ::std::option::Option<::std::string::String>,
    #[doc = "The fixed virtual balance (in USD) that the agent-portfolio was funded with. The investmentAmountInUsd used to copy is proportional to this balance."]
    #[serde(
        rename = "agentPortfolioVirtualBalance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_portfolio_virtual_balance: ::std::option::Option<f64>,
    #[doc = "When this agent-portfolio was created."]
    #[serde(
        rename = "createdAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The Trading API mirror ID for this agent-portfolio's copy trade."]
    #[serde(
        rename = "mirrorId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mirror_id: ::std::option::Option<i64>,
    #[doc = "The user tokens associated with this agent-portfolio."]
    #[serde(
        rename = "userTokens",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub user_tokens: ::std::vec::Vec<AgentPortfolioUserTokenItem>,
}
impl ::std::default::Default for AgentPortfolioItem {
    fn default() -> Self {
        Self {
            agent_portfolio_gcid: Default::default(),
            agent_portfolio_id: Default::default(),
            agent_portfolio_name: Default::default(),
            agent_portfolio_virtual_balance: Default::default(),
            created_at: Default::default(),
            mirror_id: Default::default(),
            user_tokens: Default::default(),
        }
    }
}
impl AgentPortfolioItem {
    pub fn builder() -> builder::AgentPortfolioItem {
        Default::default()
    }
}
#[doc = "`AgentPortfolioUserTokenItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"clientId\": {"]
#[doc = "      \"description\": \"The OAuth client identifier associated with the user token.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uuid\","]
#[doc = "      \"example\": \"c1d2e3f4-a5b6-7890-cdef-123456789abc\""]
#[doc = "    },"]
#[doc = "    \"createdAt\": {"]
#[doc = "      \"description\": \"When this user token was created.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\","]
#[doc = "      \"example\": \"2026-03-01T10:30:00Z\""]
#[doc = "    },"]
#[doc = "    \"expiresAt\": {"]
#[doc = "      \"description\": \"The expiration date and time of the user token in UTC.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\","]
#[doc = "      \"example\": \"2026-12-31T23:59:59Z\""]
#[doc = "    },"]
#[doc = "    \"externalApplicationName\": {"]
#[doc = "      \"description\": \"The name of the external application registered for this token.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"Trading Bot v2\""]
#[doc = "    },"]
#[doc = "    \"ipsWhitelist\": {"]
#[doc = "      \"description\": \"The set of whitelisted IP addresses authorized to use this token.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"example\": ["]
#[doc = "        \"192.168.1.1\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scopeIds\": {"]
#[doc = "      \"description\": \"The set of permission scope identifiers granted to this token. Available scopes: 200 = etoro-public:real:read, 201 = etoro-public:demo:read, 202 = etoro-public:real:write, 203 = etoro-public:demo:write.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      },"]
#[doc = "      \"example\": ["]
#[doc = "        200,"]
#[doc = "        202"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"userTokenId\": {"]
#[doc = "      \"description\": \"The unique identifier of the user token.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uuid\","]
#[doc = "      \"example\": \"f9e8d7c6-b5a4-3210-fedc-ba9876543210\""]
#[doc = "    },"]
#[doc = "    \"userTokenName\": {"]
#[doc = "      \"description\": \"The user-defined name for the user token.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"my-trading-token\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AgentPortfolioUserTokenItem {
    #[doc = "The OAuth client identifier associated with the user token."]
    #[serde(
        rename = "clientId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "When this user token was created."]
    #[serde(
        rename = "createdAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The expiration date and time of the user token in UTC."]
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The name of the external application registered for this token."]
    #[serde(
        rename = "externalApplicationName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_application_name: ::std::option::Option<::std::string::String>,
    #[doc = "The set of whitelisted IP addresses authorized to use this token."]
    #[serde(
        rename = "ipsWhitelist",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub ips_whitelist: ::std::vec::Vec<::std::string::String>,
    #[doc = "The set of permission scope identifiers granted to this token. Available scopes: 200 = etoro-public:real:read, 201 = etoro-public:demo:read, 202 = etoro-public:real:write, 203 = etoro-public:demo:write."]
    #[serde(
        rename = "scopeIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub scope_ids: ::std::vec::Vec<i64>,
    #[doc = "The unique identifier of the user token."]
    #[serde(
        rename = "userTokenId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_token_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "The user-defined name for the user token."]
    #[serde(
        rename = "userTokenName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_token_name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for AgentPortfolioUserTokenItem {
    fn default() -> Self {
        Self {
            client_id: Default::default(),
            created_at: Default::default(),
            expires_at: Default::default(),
            external_application_name: Default::default(),
            ips_whitelist: Default::default(),
            scope_ids: Default::default(),
            user_token_id: Default::default(),
            user_token_name: Default::default(),
        }
    }
}
impl AgentPortfolioUserTokenItem {
    pub fn builder() -> builder::AgentPortfolioUserTokenItem {
        Default::default()
    }
}
#[doc = "`CreateAgentPortfolioPartialResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"agentPortfolioGcid\": {"]
#[doc = "      \"description\": \"The GCID associated with the agent-portfolio.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"example\": 12345678"]
#[doc = "    },"]
#[doc = "    \"agentPortfolioId\": {"]
#[doc = "      \"description\": \"The unique identifier of the newly created agent-portfolio.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uuid\","]
#[doc = "      \"example\": \"a1b2c3d4-e5f6-7890-abcd-ef1234567890\""]
#[doc = "    },"]
#[doc = "    \"agentPortfolioName\": {"]
#[doc = "      \"description\": \"The display name assigned to the agent-portfolio.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"MyPort1\""]
#[doc = "    },"]
#[doc = "    \"agentPortfolioVirtualBalance\": {"]
#[doc = "      \"description\": \"The fixed virtual balance (in USD) that the agent-portfolio was funded with. The investmentAmountInUsd used to copy is proportional to this balance.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"example\": 10000"]
#[doc = "    },"]
#[doc = "    \"mirrorId\": {"]
#[doc = "      \"description\": \"The Trading API mirror ID for this agent-portfolio's copy trade.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"example\": 12345"]
#[doc = "    },"]
#[doc = "    \"userTokenCreated\": {"]
#[doc = "      \"description\": \"Always false — indicates that the user token was not created.\","]
#[doc = "      \"type\": \"boolean\","]
#[doc = "      \"example\": false"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateAgentPortfolioPartialResponse {
    #[doc = "The GCID associated with the agent-portfolio."]
    #[serde(
        rename = "agentPortfolioGcid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_portfolio_gcid: ::std::option::Option<i64>,
    #[doc = "The unique identifier of the newly created agent-portfolio."]
    #[serde(
        rename = "agentPortfolioId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_portfolio_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "The display name assigned to the agent-portfolio."]
    #[serde(
        rename = "agentPortfolioName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_portfolio_name: ::std::option::Option<::std::string::String>,
    #[doc = "The fixed virtual balance (in USD) that the agent-portfolio was funded with. The investmentAmountInUsd used to copy is proportional to this balance."]
    #[serde(
        rename = "agentPortfolioVirtualBalance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_portfolio_virtual_balance: ::std::option::Option<i64>,
    #[doc = "The Trading API mirror ID for this agent-portfolio's copy trade."]
    #[serde(
        rename = "mirrorId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mirror_id: ::std::option::Option<i64>,
    #[doc = "Always false — indicates that the user token was not created."]
    #[serde(
        rename = "userTokenCreated",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_token_created: ::std::option::Option<bool>,
}
impl ::std::default::Default for CreateAgentPortfolioPartialResponse {
    fn default() -> Self {
        Self {
            agent_portfolio_gcid: Default::default(),
            agent_portfolio_id: Default::default(),
            agent_portfolio_name: Default::default(),
            agent_portfolio_virtual_balance: Default::default(),
            mirror_id: Default::default(),
            user_token_created: Default::default(),
        }
    }
}
impl CreateAgentPortfolioPartialResponse {
    pub fn builder() -> builder::CreateAgentPortfolioPartialResponse {
        Default::default()
    }
}
#[doc = "`CreateAgentPortfolioRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"agentPortfolioName\","]
#[doc = "    \"investmentAmountInUsd\","]
#[doc = "    \"scopeIds\","]
#[doc = "    \"userTokenName\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agentPortfolioDescription\": {"]
#[doc = "      \"description\": \"An optional description of the agent-portfolio's purpose or strategy.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"My trading portfolio\""]
#[doc = "    },"]
#[doc = "    \"agentPortfolioName\": {"]
#[doc = "      \"description\": \"A unique display name for the agent-portfolio (6-10 characters).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"MyPort1\""]
#[doc = "    },"]
#[doc = "    \"expiresAt\": {"]
#[doc = "      \"description\": \"An optional expiration date and time (UTC) for the provisioned user token.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\","]
#[doc = "      \"example\": \"2026-12-31T23:59:59Z\""]
#[doc = "    },"]
#[doc = "    \"investmentAmountInUsd\": {"]
#[doc = "      \"description\": \"The amount in USD deducted from the CALLER's account balance to copy-trade this agent-portfolio. This is NOT the agent-portfolio's own balance — the agent-portfolio receives a separate fixed virtual balance (returned as agentPortfolioVirtualBalance). Positions are mirrored proportionally: e.g. $2,000 with a $10,000 virtual balance = 20% position sizing.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 2000"]
#[doc = "    },"]
#[doc = "    \"ipsWhitelist\": {"]
#[doc = "      \"description\": \"An optional set of IPv4 addresses allowed to use the provisioned user token.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"example\": ["]
#[doc = "        \"192.168.1.1\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scopeIds\": {"]
#[doc = "      \"description\": \"The set of permission scope identifiers to grant to the provisioned user token. Available scopes: 200 = etoro-public:real:read, 201 = etoro-public:demo:read, 202 = etoro-public:real:write, 203 = etoro-public:demo:write.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      },"]
#[doc = "      \"example\": ["]
#[doc = "        200,"]
#[doc = "        202"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"userTokenName\": {"]
#[doc = "      \"description\": \"A human-readable name for the user token provisioned with the agent-portfolio.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"my-trading-token\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateAgentPortfolioRequest {
    #[doc = "An optional description of the agent-portfolio's purpose or strategy."]
    #[serde(
        rename = "agentPortfolioDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_portfolio_description: ::std::option::Option<::std::string::String>,
    #[doc = "A unique display name for the agent-portfolio (6-10 characters)."]
    #[serde(rename = "agentPortfolioName")]
    pub agent_portfolio_name: ::std::string::String,
    #[doc = "An optional expiration date and time (UTC) for the provisioned user token."]
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The amount in USD deducted from the CALLER's account balance to copy-trade this agent-portfolio. This is NOT the agent-portfolio's own balance — the agent-portfolio receives a separate fixed virtual balance (returned as agentPortfolioVirtualBalance). Positions are mirrored proportionally: e.g. $2,000 with a $10,000 virtual balance = 20% position sizing."]
    #[serde(rename = "investmentAmountInUsd")]
    pub investment_amount_in_usd: f64,
    #[doc = "An optional set of IPv4 addresses allowed to use the provisioned user token."]
    #[serde(
        rename = "ipsWhitelist",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub ips_whitelist: ::std::vec::Vec<::std::string::String>,
    #[doc = "The set of permission scope identifiers to grant to the provisioned user token. Available scopes: 200 = etoro-public:real:read, 201 = etoro-public:demo:read, 202 = etoro-public:real:write, 203 = etoro-public:demo:write."]
    #[serde(rename = "scopeIds")]
    pub scope_ids: ::std::vec::Vec<i64>,
    #[doc = "A human-readable name for the user token provisioned with the agent-portfolio."]
    #[serde(rename = "userTokenName")]
    pub user_token_name: ::std::string::String,
}
impl CreateAgentPortfolioRequest {
    pub fn builder() -> builder::CreateAgentPortfolioRequest {
        Default::default()
    }
}
#[doc = "`CreateAgentPortfolioResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"agentPortfolioGcid\": {"]
#[doc = "      \"description\": \"The GCID associated with the agent-portfolio.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"example\": 12345678"]
#[doc = "    },"]
#[doc = "    \"agentPortfolioId\": {"]
#[doc = "      \"description\": \"The unique identifier of the newly created agent-portfolio.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uuid\","]
#[doc = "      \"example\": \"a1b2c3d4-e5f6-7890-abcd-ef1234567890\""]
#[doc = "    },"]
#[doc = "    \"agentPortfolioName\": {"]
#[doc = "      \"description\": \"The display name assigned to the agent-portfolio.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"MyPort1\""]
#[doc = "    },"]
#[doc = "    \"agentPortfolioVirtualBalance\": {"]
#[doc = "      \"description\": \"The fixed virtual balance (in USD) that the agent-portfolio was funded with. The investmentAmountInUsd used to copy is proportional to this balance.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"example\": 10000"]
#[doc = "    },"]
#[doc = "    \"mirrorId\": {"]
#[doc = "      \"description\": \"The Trading API mirror ID for this agent-portfolio's copy trade.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"example\": 12345"]
#[doc = "    },"]
#[doc = "    \"userTokens\": {"]
#[doc = "      \"description\": \"The user tokens generated during agent-portfolio creation.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/CreateAgentPortfolioUserTokenItem\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateAgentPortfolioResponse {
    #[doc = "The GCID associated with the agent-portfolio."]
    #[serde(
        rename = "agentPortfolioGcid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_portfolio_gcid: ::std::option::Option<i64>,
    #[doc = "The unique identifier of the newly created agent-portfolio."]
    #[serde(
        rename = "agentPortfolioId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_portfolio_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "The display name assigned to the agent-portfolio."]
    #[serde(
        rename = "agentPortfolioName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_portfolio_name: ::std::option::Option<::std::string::String>,
    #[doc = "The fixed virtual balance (in USD) that the agent-portfolio was funded with. The investmentAmountInUsd used to copy is proportional to this balance."]
    #[serde(
        rename = "agentPortfolioVirtualBalance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agent_portfolio_virtual_balance: ::std::option::Option<f64>,
    #[doc = "The Trading API mirror ID for this agent-portfolio's copy trade."]
    #[serde(
        rename = "mirrorId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mirror_id: ::std::option::Option<i64>,
    #[doc = "The user tokens generated during agent-portfolio creation."]
    #[serde(
        rename = "userTokens",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub user_tokens: ::std::vec::Vec<CreateAgentPortfolioUserTokenItem>,
}
impl ::std::default::Default for CreateAgentPortfolioResponse {
    fn default() -> Self {
        Self {
            agent_portfolio_gcid: Default::default(),
            agent_portfolio_id: Default::default(),
            agent_portfolio_name: Default::default(),
            agent_portfolio_virtual_balance: Default::default(),
            mirror_id: Default::default(),
            user_tokens: Default::default(),
        }
    }
}
impl CreateAgentPortfolioResponse {
    pub fn builder() -> builder::CreateAgentPortfolioResponse {
        Default::default()
    }
}
#[doc = "`CreateAgentPortfolioUserTokenItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"clientId\": {"]
#[doc = "      \"description\": \"The OAuth client identifier associated with the user token.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uuid\","]
#[doc = "      \"example\": \"c1d2e3f4-a5b6-7890-cdef-123456789abc\""]
#[doc = "    },"]
#[doc = "    \"expiresAt\": {"]
#[doc = "      \"description\": \"The expiration date and time of the user token in UTC.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\","]
#[doc = "      \"example\": \"2026-12-31T23:59:59Z\""]
#[doc = "    },"]
#[doc = "    \"ipsWhitelist\": {"]
#[doc = "      \"description\": \"The set of whitelisted IP addresses authorized to use this token.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"example\": ["]
#[doc = "        \"192.168.1.1\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scopeIds\": {"]
#[doc = "      \"description\": \"The set of permission scope identifiers granted to this token. Available scopes: 200 = etoro-public:real:read, 201 = etoro-public:demo:read, 202 = etoro-public:real:write, 203 = etoro-public:demo:write.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      },"]
#[doc = "      \"example\": ["]
#[doc = "        200,"]
#[doc = "        202"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"userToken\": {"]
#[doc = "      \"description\": \"The generated user token secret. Only available at creation time.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"sk_live_a1b2c3d4e5f6...\""]
#[doc = "    },"]
#[doc = "    \"userTokenId\": {"]
#[doc = "      \"description\": \"The unique identifier of the newly created user token.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uuid\","]
#[doc = "      \"example\": \"f9e8d7c6-b5a4-3210-fedc-ba9876543210\""]
#[doc = "    },"]
#[doc = "    \"userTokenName\": {"]
#[doc = "      \"description\": \"The user-defined name for the user token.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"my-trading-token\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateAgentPortfolioUserTokenItem {
    #[doc = "The OAuth client identifier associated with the user token."]
    #[serde(
        rename = "clientId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "The expiration date and time of the user token in UTC."]
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The set of whitelisted IP addresses authorized to use this token."]
    #[serde(
        rename = "ipsWhitelist",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub ips_whitelist: ::std::vec::Vec<::std::string::String>,
    #[doc = "The set of permission scope identifiers granted to this token. Available scopes: 200 = etoro-public:real:read, 201 = etoro-public:demo:read, 202 = etoro-public:real:write, 203 = etoro-public:demo:write."]
    #[serde(
        rename = "scopeIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub scope_ids: ::std::vec::Vec<i64>,
    #[doc = "The generated user token secret. Only available at creation time."]
    #[serde(
        rename = "userToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_token: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identifier of the newly created user token."]
    #[serde(
        rename = "userTokenId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_token_id: ::std::option::Option<::uuid::Uuid>,
    #[doc = "The user-defined name for the user token."]
    #[serde(
        rename = "userTokenName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_token_name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for CreateAgentPortfolioUserTokenItem {
    fn default() -> Self {
        Self {
            client_id: Default::default(),
            expires_at: Default::default(),
            ips_whitelist: Default::default(),
            scope_ids: Default::default(),
            user_token: Default::default(),
            user_token_id: Default::default(),
            user_token_name: Default::default(),
        }
    }
}
impl CreateAgentPortfolioUserTokenItem {
    pub fn builder() -> builder::CreateAgentPortfolioUserTokenItem {
        Default::default()
    }
}
#[doc = "`CreateUserTokenRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"scopeIds\","]
#[doc = "    \"userTokenName\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expiresAt\": {"]
#[doc = "      \"description\": \"An optional expiration date and time for the token in UTC.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\","]
#[doc = "      \"example\": \"2026-12-31T23:59:59Z\""]
#[doc = "    },"]
#[doc = "    \"ipsWhitelist\": {"]
#[doc = "      \"description\": \"An optional set of IPv4 addresses allowed to use this token.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"example\": ["]
#[doc = "        \"192.168.1.1\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scopeIds\": {"]
#[doc = "      \"description\": \"The set of permission scope identifiers to grant to this token. Available scopes: 200 = etoro-public:real:read, 201 = etoro-public:demo:read, 202 = etoro-public:real:write, 203 = etoro-public:demo:write.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      },"]
#[doc = "      \"example\": ["]
#[doc = "        200,"]
#[doc = "        202"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"userTokenName\": {"]
#[doc = "      \"description\": \"A human-readable name to identify the user token.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"my-trading-token\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateUserTokenRequest {
    #[doc = "An optional expiration date and time for the token in UTC."]
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "An optional set of IPv4 addresses allowed to use this token."]
    #[serde(
        rename = "ipsWhitelist",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub ips_whitelist: ::std::vec::Vec<::std::string::String>,
    #[doc = "The set of permission scope identifiers to grant to this token. Available scopes: 200 = etoro-public:real:read, 201 = etoro-public:demo:read, 202 = etoro-public:real:write, 203 = etoro-public:demo:write."]
    #[serde(rename = "scopeIds")]
    pub scope_ids: ::std::vec::Vec<i64>,
    #[doc = "A human-readable name to identify the user token."]
    #[serde(rename = "userTokenName")]
    pub user_token_name: ::std::string::String,
}
impl CreateUserTokenRequest {
    pub fn builder() -> builder::CreateUserTokenRequest {
        Default::default()
    }
}
#[doc = "`CreateUserTokenResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"userToken\": {"]
#[doc = "      \"description\": \"The generated user token secret. Only available at creation time.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"example\": \"sk_live_a1b2c3d4e5f6...\""]
#[doc = "    },"]
#[doc = "    \"userTokenId\": {"]
#[doc = "      \"description\": \"The unique identifier of the newly created user token.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uuid\","]
#[doc = "      \"example\": \"f9e8d7c6-b5a4-3210-fedc-ba9876543210\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateUserTokenResponse {
    #[doc = "The generated user token secret. Only available at creation time."]
    #[serde(
        rename = "userToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_token: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identifier of the newly created user token."]
    #[serde(
        rename = "userTokenId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_token_id: ::std::option::Option<::uuid::Uuid>,
}
impl ::std::default::Default for CreateUserTokenResponse {
    fn default() -> Self {
        Self {
            user_token: Default::default(),
            user_token_id: Default::default(),
        }
    }
}
impl CreateUserTokenResponse {
    pub fn builder() -> builder::CreateUserTokenResponse {
        Default::default()
    }
}
#[doc = "`GetAgentPortfoliosResponse`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"agentPortfolios\": {"]
#[doc = "      \"description\": \"The collection of agent-portfolios owned by the user.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/AgentPortfolioItem\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetAgentPortfoliosResponse {
    #[doc = "The collection of agent-portfolios owned by the user."]
    #[serde(
        rename = "agentPortfolios",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub agent_portfolios: ::std::vec::Vec<AgentPortfolioItem>,
}
impl ::std::default::Default for GetAgentPortfoliosResponse {
    fn default() -> Self {
        Self {
            agent_portfolios: Default::default(),
        }
    }
}
impl GetAgentPortfoliosResponse {
    pub fn builder() -> builder::GetAgentPortfoliosResponse {
        Default::default()
    }
}
#[doc = "`UpdateUserTokenRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"expiresAt\": {"]
#[doc = "      \"description\": \"An updated expiration date and time (UTC) for the token.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\","]
#[doc = "      \"example\": \"2026-12-31T23:59:59Z\""]
#[doc = "    },"]
#[doc = "    \"ipsWhitelist\": {"]
#[doc = "      \"description\": \"An updated set of IPv4 addresses allowed to use this token.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"example\": ["]
#[doc = "        \"192.168.1.1\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scopeIds\": {"]
#[doc = "      \"description\": \"An updated set of permission scope identifiers for the token. Available scopes: 200 = etoro-public:real:read, 201 = etoro-public:demo:read, 202 = etoro-public:real:write, 203 = etoro-public:demo:write.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      },"]
#[doc = "      \"example\": ["]
#[doc = "        200,"]
#[doc = "        202"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UpdateUserTokenRequest {
    #[doc = "An updated expiration date and time (UTC) for the token."]
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "An updated set of IPv4 addresses allowed to use this token."]
    #[serde(
        rename = "ipsWhitelist",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub ips_whitelist: ::std::vec::Vec<::std::string::String>,
    #[doc = "An updated set of permission scope identifiers for the token. Available scopes: 200 = etoro-public:real:read, 201 = etoro-public:demo:read, 202 = etoro-public:real:write, 203 = etoro-public:demo:write."]
    #[serde(
        rename = "scopeIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub scope_ids: ::std::vec::Vec<i64>,
}
impl ::std::default::Default for UpdateUserTokenRequest {
    fn default() -> Self {
        Self {
            expires_at: Default::default(),
            ips_whitelist: Default::default(),
            scope_ids: Default::default(),
        }
    }
}
impl UpdateUserTokenRequest {
    pub fn builder() -> builder::UpdateUserTokenRequest {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AgentPortfolioItem {
        agent_portfolio_gcid:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        agent_portfolio_id:
            ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
        agent_portfolio_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        agent_portfolio_virtual_balance:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        created_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        mirror_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        user_tokens: ::std::result::Result<
            ::std::vec::Vec<super::AgentPortfolioUserTokenItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgentPortfolioItem {
        fn default() -> Self {
            Self {
                agent_portfolio_gcid: Ok(Default::default()),
                agent_portfolio_id: Ok(Default::default()),
                agent_portfolio_name: Ok(Default::default()),
                agent_portfolio_virtual_balance: Ok(Default::default()),
                created_at: Ok(Default::default()),
                mirror_id: Ok(Default::default()),
                user_tokens: Ok(Default::default()),
            }
        }
    }
    impl AgentPortfolioItem {
        pub fn agent_portfolio_gcid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_gcid = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_gcid: {e}")
            });
            self
        }
        pub fn agent_portfolio_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_id: {e}")
            });
            self
        }
        pub fn agent_portfolio_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_name = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_name: {e}")
            });
            self
        }
        pub fn agent_portfolio_virtual_balance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_virtual_balance = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_virtual_balance: {e}")
            });
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {e}"));
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
        pub fn user_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgentPortfolioUserTokenItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_tokens: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentPortfolioItem> for super::AgentPortfolioItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentPortfolioItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent_portfolio_gcid: value.agent_portfolio_gcid?,
                agent_portfolio_id: value.agent_portfolio_id?,
                agent_portfolio_name: value.agent_portfolio_name?,
                agent_portfolio_virtual_balance: value.agent_portfolio_virtual_balance?,
                created_at: value.created_at?,
                mirror_id: value.mirror_id?,
                user_tokens: value.user_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::AgentPortfolioItem> for AgentPortfolioItem {
        fn from(value: super::AgentPortfolioItem) -> Self {
            Self {
                agent_portfolio_gcid: Ok(value.agent_portfolio_gcid),
                agent_portfolio_id: Ok(value.agent_portfolio_id),
                agent_portfolio_name: Ok(value.agent_portfolio_name),
                agent_portfolio_virtual_balance: Ok(value.agent_portfolio_virtual_balance),
                created_at: Ok(value.created_at),
                mirror_id: Ok(value.mirror_id),
                user_tokens: Ok(value.user_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentPortfolioUserTokenItem {
        client_id:
            ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
        created_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        expires_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        external_application_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ips_whitelist:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        scope_ids: ::std::result::Result<::std::vec::Vec<i64>, ::std::string::String>,
        user_token_id:
            ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
        user_token_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgentPortfolioUserTokenItem {
        fn default() -> Self {
            Self {
                client_id: Ok(Default::default()),
                created_at: Ok(Default::default()),
                expires_at: Ok(Default::default()),
                external_application_name: Ok(Default::default()),
                ips_whitelist: Ok(Default::default()),
                scope_ids: Ok(Default::default()),
                user_token_id: Ok(Default::default()),
                user_token_name: Ok(Default::default()),
            }
        }
    }
    impl AgentPortfolioUserTokenItem {
        pub fn client_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_id: {e}"));
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {e}"));
            self
        }
        pub fn expires_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.expires_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
            self
        }
        pub fn external_application_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_application_name = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_application_name: {e}")
            });
            self
        }
        pub fn ips_whitelist<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ips_whitelist = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ips_whitelist: {e}"));
            self
        }
        pub fn scope_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.scope_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scope_ids: {e}"));
            self
        }
        pub fn user_token_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_token_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_token_id: {e}"));
            self
        }
        pub fn user_token_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_token_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_token_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentPortfolioUserTokenItem> for super::AgentPortfolioUserTokenItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentPortfolioUserTokenItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_id: value.client_id?,
                created_at: value.created_at?,
                expires_at: value.expires_at?,
                external_application_name: value.external_application_name?,
                ips_whitelist: value.ips_whitelist?,
                scope_ids: value.scope_ids?,
                user_token_id: value.user_token_id?,
                user_token_name: value.user_token_name?,
            })
        }
    }
    impl ::std::convert::From<super::AgentPortfolioUserTokenItem> for AgentPortfolioUserTokenItem {
        fn from(value: super::AgentPortfolioUserTokenItem) -> Self {
            Self {
                client_id: Ok(value.client_id),
                created_at: Ok(value.created_at),
                expires_at: Ok(value.expires_at),
                external_application_name: Ok(value.external_application_name),
                ips_whitelist: Ok(value.ips_whitelist),
                scope_ids: Ok(value.scope_ids),
                user_token_id: Ok(value.user_token_id),
                user_token_name: Ok(value.user_token_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateAgentPortfolioPartialResponse {
        agent_portfolio_gcid:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        agent_portfolio_id:
            ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
        agent_portfolio_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        agent_portfolio_virtual_balance:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        mirror_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        user_token_created:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateAgentPortfolioPartialResponse {
        fn default() -> Self {
            Self {
                agent_portfolio_gcid: Ok(Default::default()),
                agent_portfolio_id: Ok(Default::default()),
                agent_portfolio_name: Ok(Default::default()),
                agent_portfolio_virtual_balance: Ok(Default::default()),
                mirror_id: Ok(Default::default()),
                user_token_created: Ok(Default::default()),
            }
        }
    }
    impl CreateAgentPortfolioPartialResponse {
        pub fn agent_portfolio_gcid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_gcid = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_gcid: {e}")
            });
            self
        }
        pub fn agent_portfolio_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_id: {e}")
            });
            self
        }
        pub fn agent_portfolio_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_name = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_name: {e}")
            });
            self
        }
        pub fn agent_portfolio_virtual_balance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_virtual_balance = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_virtual_balance: {e}")
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
        pub fn user_token_created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_token_created = value.try_into().map_err(|e| {
                format!("error converting supplied value for user_token_created: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CreateAgentPortfolioPartialResponse>
        for super::CreateAgentPortfolioPartialResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateAgentPortfolioPartialResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent_portfolio_gcid: value.agent_portfolio_gcid?,
                agent_portfolio_id: value.agent_portfolio_id?,
                agent_portfolio_name: value.agent_portfolio_name?,
                agent_portfolio_virtual_balance: value.agent_portfolio_virtual_balance?,
                mirror_id: value.mirror_id?,
                user_token_created: value.user_token_created?,
            })
        }
    }
    impl ::std::convert::From<super::CreateAgentPortfolioPartialResponse>
        for CreateAgentPortfolioPartialResponse
    {
        fn from(value: super::CreateAgentPortfolioPartialResponse) -> Self {
            Self {
                agent_portfolio_gcid: Ok(value.agent_portfolio_gcid),
                agent_portfolio_id: Ok(value.agent_portfolio_id),
                agent_portfolio_name: Ok(value.agent_portfolio_name),
                agent_portfolio_virtual_balance: Ok(value.agent_portfolio_virtual_balance),
                mirror_id: Ok(value.mirror_id),
                user_token_created: Ok(value.user_token_created),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateAgentPortfolioRequest {
        agent_portfolio_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        agent_portfolio_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        expires_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        investment_amount_in_usd: ::std::result::Result<f64, ::std::string::String>,
        ips_whitelist:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        scope_ids: ::std::result::Result<::std::vec::Vec<i64>, ::std::string::String>,
        user_token_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateAgentPortfolioRequest {
        fn default() -> Self {
            Self {
                agent_portfolio_description: Ok(Default::default()),
                agent_portfolio_name: Err("no value supplied for agent_portfolio_name".to_string()),
                expires_at: Ok(Default::default()),
                investment_amount_in_usd: Err(
                    "no value supplied for investment_amount_in_usd".to_string()
                ),
                ips_whitelist: Ok(Default::default()),
                scope_ids: Err("no value supplied for scope_ids".to_string()),
                user_token_name: Err("no value supplied for user_token_name".to_string()),
            }
        }
    }
    impl CreateAgentPortfolioRequest {
        pub fn agent_portfolio_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_description = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_description: {e}")
            });
            self
        }
        pub fn agent_portfolio_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_name = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_name: {e}")
            });
            self
        }
        pub fn expires_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.expires_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
            self
        }
        pub fn investment_amount_in_usd<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.investment_amount_in_usd = value.try_into().map_err(|e| {
                format!("error converting supplied value for investment_amount_in_usd: {e}")
            });
            self
        }
        pub fn ips_whitelist<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ips_whitelist = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ips_whitelist: {e}"));
            self
        }
        pub fn scope_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.scope_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scope_ids: {e}"));
            self
        }
        pub fn user_token_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.user_token_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_token_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateAgentPortfolioRequest> for super::CreateAgentPortfolioRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateAgentPortfolioRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent_portfolio_description: value.agent_portfolio_description?,
                agent_portfolio_name: value.agent_portfolio_name?,
                expires_at: value.expires_at?,
                investment_amount_in_usd: value.investment_amount_in_usd?,
                ips_whitelist: value.ips_whitelist?,
                scope_ids: value.scope_ids?,
                user_token_name: value.user_token_name?,
            })
        }
    }
    impl ::std::convert::From<super::CreateAgentPortfolioRequest> for CreateAgentPortfolioRequest {
        fn from(value: super::CreateAgentPortfolioRequest) -> Self {
            Self {
                agent_portfolio_description: Ok(value.agent_portfolio_description),
                agent_portfolio_name: Ok(value.agent_portfolio_name),
                expires_at: Ok(value.expires_at),
                investment_amount_in_usd: Ok(value.investment_amount_in_usd),
                ips_whitelist: Ok(value.ips_whitelist),
                scope_ids: Ok(value.scope_ids),
                user_token_name: Ok(value.user_token_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateAgentPortfolioResponse {
        agent_portfolio_gcid:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        agent_portfolio_id:
            ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
        agent_portfolio_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        agent_portfolio_virtual_balance:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        mirror_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        user_tokens: ::std::result::Result<
            ::std::vec::Vec<super::CreateAgentPortfolioUserTokenItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateAgentPortfolioResponse {
        fn default() -> Self {
            Self {
                agent_portfolio_gcid: Ok(Default::default()),
                agent_portfolio_id: Ok(Default::default()),
                agent_portfolio_name: Ok(Default::default()),
                agent_portfolio_virtual_balance: Ok(Default::default()),
                mirror_id: Ok(Default::default()),
                user_tokens: Ok(Default::default()),
            }
        }
    }
    impl CreateAgentPortfolioResponse {
        pub fn agent_portfolio_gcid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_gcid = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_gcid: {e}")
            });
            self
        }
        pub fn agent_portfolio_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_id: {e}")
            });
            self
        }
        pub fn agent_portfolio_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_name = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_name: {e}")
            });
            self
        }
        pub fn agent_portfolio_virtual_balance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolio_virtual_balance = value.try_into().map_err(|e| {
                format!("error converting supplied value for agent_portfolio_virtual_balance: {e}")
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
        pub fn user_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CreateAgentPortfolioUserTokenItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_tokens: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateAgentPortfolioResponse> for super::CreateAgentPortfolioResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateAgentPortfolioResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent_portfolio_gcid: value.agent_portfolio_gcid?,
                agent_portfolio_id: value.agent_portfolio_id?,
                agent_portfolio_name: value.agent_portfolio_name?,
                agent_portfolio_virtual_balance: value.agent_portfolio_virtual_balance?,
                mirror_id: value.mirror_id?,
                user_tokens: value.user_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::CreateAgentPortfolioResponse> for CreateAgentPortfolioResponse {
        fn from(value: super::CreateAgentPortfolioResponse) -> Self {
            Self {
                agent_portfolio_gcid: Ok(value.agent_portfolio_gcid),
                agent_portfolio_id: Ok(value.agent_portfolio_id),
                agent_portfolio_name: Ok(value.agent_portfolio_name),
                agent_portfolio_virtual_balance: Ok(value.agent_portfolio_virtual_balance),
                mirror_id: Ok(value.mirror_id),
                user_tokens: Ok(value.user_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateAgentPortfolioUserTokenItem {
        client_id:
            ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
        expires_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        ips_whitelist:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        scope_ids: ::std::result::Result<::std::vec::Vec<i64>, ::std::string::String>,
        user_token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_token_id:
            ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
        user_token_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateAgentPortfolioUserTokenItem {
        fn default() -> Self {
            Self {
                client_id: Ok(Default::default()),
                expires_at: Ok(Default::default()),
                ips_whitelist: Ok(Default::default()),
                scope_ids: Ok(Default::default()),
                user_token: Ok(Default::default()),
                user_token_id: Ok(Default::default()),
                user_token_name: Ok(Default::default()),
            }
        }
    }
    impl CreateAgentPortfolioUserTokenItem {
        pub fn client_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_id: {e}"));
            self
        }
        pub fn expires_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.expires_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
            self
        }
        pub fn ips_whitelist<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ips_whitelist = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ips_whitelist: {e}"));
            self
        }
        pub fn scope_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.scope_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scope_ids: {e}"));
            self
        }
        pub fn user_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_token: {e}"));
            self
        }
        pub fn user_token_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_token_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_token_id: {e}"));
            self
        }
        pub fn user_token_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_token_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_token_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateAgentPortfolioUserTokenItem>
        for super::CreateAgentPortfolioUserTokenItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateAgentPortfolioUserTokenItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_id: value.client_id?,
                expires_at: value.expires_at?,
                ips_whitelist: value.ips_whitelist?,
                scope_ids: value.scope_ids?,
                user_token: value.user_token?,
                user_token_id: value.user_token_id?,
                user_token_name: value.user_token_name?,
            })
        }
    }
    impl ::std::convert::From<super::CreateAgentPortfolioUserTokenItem>
        for CreateAgentPortfolioUserTokenItem
    {
        fn from(value: super::CreateAgentPortfolioUserTokenItem) -> Self {
            Self {
                client_id: Ok(value.client_id),
                expires_at: Ok(value.expires_at),
                ips_whitelist: Ok(value.ips_whitelist),
                scope_ids: Ok(value.scope_ids),
                user_token: Ok(value.user_token),
                user_token_id: Ok(value.user_token_id),
                user_token_name: Ok(value.user_token_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateUserTokenRequest {
        expires_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        ips_whitelist:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        scope_ids: ::std::result::Result<::std::vec::Vec<i64>, ::std::string::String>,
        user_token_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateUserTokenRequest {
        fn default() -> Self {
            Self {
                expires_at: Ok(Default::default()),
                ips_whitelist: Ok(Default::default()),
                scope_ids: Err("no value supplied for scope_ids".to_string()),
                user_token_name: Err("no value supplied for user_token_name".to_string()),
            }
        }
    }
    impl CreateUserTokenRequest {
        pub fn expires_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.expires_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
            self
        }
        pub fn ips_whitelist<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ips_whitelist = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ips_whitelist: {e}"));
            self
        }
        pub fn scope_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.scope_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scope_ids: {e}"));
            self
        }
        pub fn user_token_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.user_token_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_token_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateUserTokenRequest> for super::CreateUserTokenRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateUserTokenRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expires_at: value.expires_at?,
                ips_whitelist: value.ips_whitelist?,
                scope_ids: value.scope_ids?,
                user_token_name: value.user_token_name?,
            })
        }
    }
    impl ::std::convert::From<super::CreateUserTokenRequest> for CreateUserTokenRequest {
        fn from(value: super::CreateUserTokenRequest) -> Self {
            Self {
                expires_at: Ok(value.expires_at),
                ips_whitelist: Ok(value.ips_whitelist),
                scope_ids: Ok(value.scope_ids),
                user_token_name: Ok(value.user_token_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateUserTokenResponse {
        user_token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_token_id:
            ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateUserTokenResponse {
        fn default() -> Self {
            Self {
                user_token: Ok(Default::default()),
                user_token_id: Ok(Default::default()),
            }
        }
    }
    impl CreateUserTokenResponse {
        pub fn user_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_token: {e}"));
            self
        }
        pub fn user_token_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_token_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_token_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateUserTokenResponse> for super::CreateUserTokenResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateUserTokenResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                user_token: value.user_token?,
                user_token_id: value.user_token_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateUserTokenResponse> for CreateUserTokenResponse {
        fn from(value: super::CreateUserTokenResponse) -> Self {
            Self {
                user_token: Ok(value.user_token),
                user_token_id: Ok(value.user_token_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetAgentPortfoliosResponse {
        agent_portfolios: ::std::result::Result<
            ::std::vec::Vec<super::AgentPortfolioItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetAgentPortfoliosResponse {
        fn default() -> Self {
            Self {
                agent_portfolios: Ok(Default::default()),
            }
        }
    }
    impl GetAgentPortfoliosResponse {
        pub fn agent_portfolios<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgentPortfolioItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_portfolios = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent_portfolios: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetAgentPortfoliosResponse> for super::GetAgentPortfoliosResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetAgentPortfoliosResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent_portfolios: value.agent_portfolios?,
            })
        }
    }
    impl ::std::convert::From<super::GetAgentPortfoliosResponse> for GetAgentPortfoliosResponse {
        fn from(value: super::GetAgentPortfoliosResponse) -> Self {
            Self {
                agent_portfolios: Ok(value.agent_portfolios),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateUserTokenRequest {
        expires_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        ips_whitelist:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        scope_ids: ::std::result::Result<::std::vec::Vec<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateUserTokenRequest {
        fn default() -> Self {
            Self {
                expires_at: Ok(Default::default()),
                ips_whitelist: Ok(Default::default()),
                scope_ids: Ok(Default::default()),
            }
        }
    }
    impl UpdateUserTokenRequest {
        pub fn expires_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.expires_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
            self
        }
        pub fn ips_whitelist<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ips_whitelist = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ips_whitelist: {e}"));
            self
        }
        pub fn scope_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.scope_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scope_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateUserTokenRequest> for super::UpdateUserTokenRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateUserTokenRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expires_at: value.expires_at?,
                ips_whitelist: value.ips_whitelist?,
                scope_ids: value.scope_ids?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateUserTokenRequest> for UpdateUserTokenRequest {
        fn from(value: super::UpdateUserTokenRequest) -> Self {
            Self {
                expires_at: Ok(value.expires_at),
                ips_whitelist: Ok(value.ips_whitelist),
                scope_ids: Ok(value.scope_ids),
            }
        }
    }
}
