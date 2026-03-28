//! Registry of model providers supported by Codex.
//!
//! Providers can be defined in two places:
//!   1. Built-in defaults compiled into the binary so Codex works out-of-the-box.
//!   2. User-defined entries inside `~/.codex/config.toml` under the `model_providers`
//!      key. These override or extend the defaults at runtime.

use crate::auth::AuthMode;
use crate::error::EnvVarError;
use codex_api::Provider as ApiProvider;
use codex_api::provider::RetryConfig as ApiRetryConfig;
use http::HeaderMap;
use http::header::HeaderName;
use http::header::HeaderValue;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

const DEFAULT_STREAM_IDLE_TIMEOUT_MS: u64 = 300_000;
const DEFAULT_STREAM_MAX_RETRIES: u64 = 5;
const DEFAULT_REQUEST_MAX_RETRIES: u64 = 4;
pub(crate) const DEFAULT_WEBSOCKET_CONNECT_TIMEOUT_MS: u64 = 15_000;
/// Hard cap for user-configured `stream_max_retries`.
const MAX_STREAM_MAX_RETRIES: u64 = 100;
/// Hard cap for user-configured `request_max_retries`.
const MAX_REQUEST_MAX_RETRIES: u64 = 100;

const OPENAI_PROVIDER_NAME: &str = "OpenAI";
pub const OPENAI_PROVIDER_ID: &str = "openai";

/// Wire protocol that the provider speaks.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum WireApi {
    /// The Responses API exposed by OpenAI at `/v1/responses`.
    #[default]
    Responses,

    /// The Chat Completions API at `/v1/chat/completions`.
    /// Supported by most third-party providers (Groq, Mistral, Together, DeepSeek, xAI, OpenRouter, etc.).
    Chat,

    /// Anthropic Messages API at `/v1/messages`.
    AnthropicMessages,

    /// Google Gemini API at `/v1beta/models/{model}:streamGenerateContent`.
    GeminiGenerateContent,
}

impl fmt::Display for WireApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Responses => "responses",
            Self::Chat => "chat",
            Self::AnthropicMessages => "anthropic",
            Self::GeminiGenerateContent => "gemini",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AuthStyle {
    #[default]
    Bearer,
    XApiKey,
    QueryParam,
}

fn default_auth_style() -> AuthStyle {
    AuthStyle::Bearer
}

impl<'de> Deserialize<'de> for WireApi {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "responses" => Ok(Self::Responses),
            "chat" => Ok(Self::Chat),
            "anthropic" | "anthropic_messages" => Ok(Self::AnthropicMessages),
            "gemini" | "gemini_generate_content" => Ok(Self::GeminiGenerateContent),
            _ => Err(serde::de::Error::unknown_variant(
                &value,
                &["responses", "chat", "anthropic", "gemini"],
            )),
        }
    }
}

/// Serializable representation of a provider definition.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct ModelProviderInfo {
    /// Friendly display name.
    pub name: String,
    /// Base URL for the provider's OpenAI-compatible API.
    pub base_url: Option<String>,
    /// Environment variable that stores the user's API key for this provider.
    pub env_key: Option<String>,
    /// Optional alias environment variables to check if `env_key` is not found.
    pub env_key_aliases: Option<Vec<String>>,

    /// Optional instructions to help the user get a valid value for the
    /// variable and set it.
    pub env_key_instructions: Option<String>,

    /// Value to use with `Authorization: Bearer <token>` header. Use of this
    /// config is discouraged in favor of `env_key` for security reasons, but
    /// this may be necessary when using this programmatically.
    pub experimental_bearer_token: Option<String>,

    /// Which wire protocol this provider expects.
    #[serde(default)]
    pub wire_api: WireApi,

    /// Optional query parameters to append to the base URL.
    pub query_params: Option<HashMap<String, String>>,

    /// Additional HTTP headers to include in requests to this provider where
    /// the (key, value) pairs are the header name and value.
    pub http_headers: Option<HashMap<String, String>>,

    /// Optional HTTP headers to include in requests to this provider where the
    /// (key, value) pairs are the header name and _environment variable_ whose
    /// value should be used. If the environment variable is not set, or the
    /// value is empty, the header will not be included in the request.
    pub env_http_headers: Option<HashMap<String, String>>,

    /// Maximum number of times to retry a failed HTTP request to this provider.
    pub request_max_retries: Option<u64>,

    /// Number of times to retry reconnecting a dropped streaming response before failing.
    pub stream_max_retries: Option<u64>,

    /// Idle timeout (in milliseconds) to wait for activity on a streaming response before treating
    /// the connection as lost.
    pub stream_idle_timeout_ms: Option<u64>,

    /// Maximum time (in milliseconds) to wait for a websocket connection attempt before treating
    /// it as failed.
    pub websocket_connect_timeout_ms: Option<u64>,

    /// Does this provider require an OpenAI API Key or ChatGPT login token? If true,
    /// user is presented with login screen on first run, and login preference and token/key
    /// are stored in auth.json. If false (which is the default), login screen is skipped,
    /// and API key (if needed) comes from the "env_key" environment variable.
    #[serde(default)]
    pub requires_openai_auth: bool,

    /// How authentication works for this provider.
    #[serde(default = "default_auth_style")]
    pub auth_style: AuthStyle,

    /// Whether this provider supports the Responses API WebSocket transport.
    #[serde(default)]
    pub supports_websockets: bool,
}

fn get_env_var(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.trim().to_string())
}

impl ModelProviderInfo {
    fn build_header_map(&self) -> crate::error::Result<HeaderMap> {
        let capacity = self.http_headers.as_ref().map_or(0, HashMap::len)
            + self.env_http_headers.as_ref().map_or(0, HashMap::len);
        let mut headers = HeaderMap::with_capacity(capacity);
        if let Some(extra) = &self.http_headers {
            for (k, v) in extra {
                if let (Ok(name), Ok(value)) = (HeaderName::try_from(k), HeaderValue::try_from(v)) {
                    headers.insert(name, value);
                }
            }
        }

        if let Some(env_headers) = &self.env_http_headers {
            for (header, env_var) in env_headers {
                if let Ok(val) = std::env::var(env_var)
                    && !val.trim().is_empty()
                    && let (Ok(name), Ok(value)) =
                        (HeaderName::try_from(header), HeaderValue::try_from(val))
                {
                    headers.insert(name, value);
                }
            }
        }

        Ok(headers)
    }

    pub fn to_api_provider(
        &self,
        auth_mode: Option<AuthMode>,
    ) -> crate::error::Result<ApiProvider> {
        let default_base_url = if matches!(auth_mode, Some(AuthMode::Chatgpt)) {
            "https://chatgpt.com/backend-api/codex"
        } else {
            "https://api.openai.com/v1"
        };
        let base_url = self
            .base_url
            .clone()
            .unwrap_or_else(|| default_base_url.to_string());

        let headers = self.build_header_map()?;
        let retry = ApiRetryConfig {
            max_attempts: self.request_max_retries(),
            base_delay: Duration::from_millis(200),
            retry_429: false,
            retry_5xx: true,
            retry_transport: true,
        };

        Ok(ApiProvider {
            name: self.name.clone(),
            base_url,
            query_params: self.query_params.clone(),
            headers,
            retry,
            stream_idle_timeout: self.stream_idle_timeout(),
        })
    }

    /// If `env_key` is Some, returns the API key for this provider if present
    /// (and non-empty) in the environment. If `env_key` is required but
    /// cannot be found, returns an error.
    pub fn api_key(&self) -> crate::error::Result<Option<String>> {
        // 1. Check primary key
        if let Some(env_key) = &self.env_key {
            if let Some(val) = get_env_var(env_key) {
                return Ok(Some(val));
            }
        }

        // 2. Check aliases
        if let Some(aliases) = &self.env_key_aliases {
            for alias in aliases {
                if let Some(val) = get_env_var(alias) {
                    return Ok(Some(val));
                }
            }
        }

        // 3. Fallback error if env_key was set but nothing found
        if let Some(env_key) = &self.env_key {
            return Err(crate::error::CodexErr::EnvVar(EnvVarError {
                var: env_key.clone(),
                instructions: self.env_key_instructions.clone(),
            }));
        }

        Ok(None)
    }

    /// Effective maximum number of request retries for this provider.
    pub fn request_max_retries(&self) -> u64 {
        self.request_max_retries
            .unwrap_or(DEFAULT_REQUEST_MAX_RETRIES)
            .min(MAX_REQUEST_MAX_RETRIES)
    }

    /// Effective maximum number of stream reconnection attempts for this provider.
    pub fn stream_max_retries(&self) -> u64 {
        self.stream_max_retries
            .unwrap_or(DEFAULT_STREAM_MAX_RETRIES)
            .min(MAX_STREAM_MAX_RETRIES)
    }

    /// Effective idle timeout for streaming responses.
    pub fn stream_idle_timeout(&self) -> Duration {
        self.stream_idle_timeout_ms
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_millis(DEFAULT_STREAM_IDLE_TIMEOUT_MS))
    }

    /// Effective timeout for websocket connect attempts.
    pub fn websocket_connect_timeout(&self) -> Duration {
        self.websocket_connect_timeout_ms
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_millis(DEFAULT_WEBSOCKET_CONNECT_TIMEOUT_MS))
    }

    pub fn create_openai_provider(base_url: Option<String>) -> ModelProviderInfo {
        ModelProviderInfo {
            name: OPENAI_PROVIDER_NAME.into(),
            base_url,
            env_key: Some("OPENAI_API_KEY".into()),
            env_key_aliases: None,
            env_key_instructions: None,
            experimental_bearer_token: None,
            wire_api: WireApi::Responses,
            query_params: None,
            http_headers: Some(
                [("version".to_string(), env!("CARGO_PKG_VERSION").to_string())]
                    .into_iter()
                    .collect(),
            ),
            env_http_headers: Some(
                [
                    (
                        "OpenAI-Organization".to_string(),
                        "OPENAI_ORGANIZATION".to_string(),
                    ),
                    ("OpenAI-Project".to_string(), "OPENAI_PROJECT".to_string()),
                ]
                .into_iter()
                .collect(),
            ),
            // Use global defaults for retry/timeout unless overridden in config.toml.
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: true,
            auth_style: default_auth_style(),
            supports_websockets: true,
        }
    }

    pub fn is_openai(&self) -> bool {
        self.name == OPENAI_PROVIDER_NAME
    }
}

pub const DEFAULT_LMSTUDIO_PORT: u16 = 1234;
pub const DEFAULT_OLLAMA_PORT: u16 = 11434;

pub const LMSTUDIO_OSS_PROVIDER_ID: &str = "lmstudio";
pub const OLLAMA_OSS_PROVIDER_ID: &str = "ollama";

/// Built-in default provider list.
pub fn built_in_model_providers(
    openai_base_url: Option<String>,
) -> HashMap<String, ModelProviderInfo> {
    use ModelProviderInfo as P;
    let openai_provider = P::create_openai_provider(openai_base_url);

    // We do not want to be in the business of adjucating which third-party
    // providers are bundled with Codex CLI, so we only include the OpenAI and
    // open source ("oss") providers by default. Users are encouraged to add to
    // `model_providers` in config.toml to add their own providers.
    //
    // DX FORK: We've restored multi-provider support by adding built-in definitions
    // for major AI providers that were locked out when OpenAI removed wire_api = "chat".
    [
        // === EXISTING ===
        (OPENAI_PROVIDER_ID, openai_provider),
        (
            OLLAMA_OSS_PROVIDER_ID,
            create_oss_provider(DEFAULT_OLLAMA_PORT, WireApi::Responses),
        ),
        (
            LMSTUDIO_OSS_PROVIDER_ID,
            create_oss_provider(DEFAULT_LMSTUDIO_PORT, WireApi::Responses),
        ),
        // === NEW: Anthropic ===
        (
            "anthropic",
            ModelProviderInfo {
                name: "Anthropic".into(),
                base_url: Some("https://api.anthropic.com/v1".into()),
                env_key: Some("ANTHROPIC_API_KEY".into()),
                env_key_aliases: None,
                env_key_instructions: Some(
                    "Get your API key at https://console.anthropic.com/settings/keys".into(),
                ),
                experimental_bearer_token: None,
                wire_api: WireApi::AnthropicMessages,
                query_params: None,
                http_headers: Some(
                    [("anthropic-version".to_string(), "2023-06-01".to_string())]
                        .into_iter()
                        .collect(),
                ),
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: AuthStyle::XApiKey,
                supports_websockets: false,
            },
        ),
        // === NEW: Google Gemini ===
        (
            "gemini",
            ModelProviderInfo {
                name: "Google Gemini".into(),
                base_url: Some("https://generativelanguage.googleapis.com".into()),
                env_key: Some("GEMINI_API_KEY".into()),
                env_key_aliases: Some(vec!["GOOGLE_GENERATIVE_AI_API_KEY".into()]),
                env_key_instructions: Some(
                    "Get your API key at https://aistudio.google.com/apikey".into(),
                ),
                experimental_bearer_token: None,
                wire_api: WireApi::GeminiGenerateContent,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: AuthStyle::QueryParam,
                supports_websockets: false,
            },
        ),
        // === NEW: OpenAI-Compatible Chat providers ===
        // These use wire_api = Chat because they implement
        // the /v1/chat/completions endpoint
        (
            "groq",
            ModelProviderInfo {
                name: "Groq".into(),
                base_url: Some("https://api.groq.com/openai/v1".into()),
                env_key: Some("GROQ_API_KEY".into()),
                env_key_aliases: None,
                env_key_instructions: Some(
                    "Get your API key at https://console.groq.com/keys".into(),
                ),
                experimental_bearer_token: None,
                wire_api: WireApi::Chat,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: default_auth_style(),
                supports_websockets: false,
            },
        ),
        (
            "mistral",
            ModelProviderInfo {
                name: "Mistral".into(),
                base_url: Some("https://api.mistral.ai/v1".into()),
                env_key: Some("MISTRAL_API_KEY".into()),
                env_key_aliases: None,
                env_key_instructions: Some(
                    "Get your API key at https://console.mistral.ai/api-keys".into(),
                ),
                experimental_bearer_token: None,
                wire_api: WireApi::Chat,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: default_auth_style(),
                supports_websockets: false,
            },
        ),
        (
            "together",
            ModelProviderInfo {
                name: "Together AI".into(),
                base_url: Some("https://api.together.xyz/v1".into()),
                env_key: Some("TOGETHER_API_KEY".into()),
                env_key_aliases: None,
                env_key_instructions: None,
                experimental_bearer_token: None,
                wire_api: WireApi::Chat,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: default_auth_style(),
                supports_websockets: false,
            },
        ),
        (
            "fireworks",
            ModelProviderInfo {
                name: "Fireworks AI".into(),
                base_url: Some("https://api.fireworks.ai/inference/v1".into()),
                env_key: Some("FIREWORKS_API_KEY".into()),
                env_key_aliases: None,
                env_key_instructions: None,
                experimental_bearer_token: None,
                wire_api: WireApi::Chat,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: default_auth_style(),
                supports_websockets: false,
            },
        ),
        (
            "deepseek",
            ModelProviderInfo {
                name: "DeepSeek".into(),
                base_url: Some("https://api.deepseek.com/v1".into()),
                env_key: Some("DEEPSEEK_API_KEY".into()),
                env_key_aliases: None,
                env_key_instructions: None,
                experimental_bearer_token: None,
                wire_api: WireApi::Chat,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: default_auth_style(),
                supports_websockets: false,
            },
        ),
        (
            "xai",
            ModelProviderInfo {
                name: "xAI (Grok)".into(),
                base_url: Some("https://api.x.ai/v1".into()),
                env_key: Some("XAI_API_KEY".into()),
                env_key_aliases: None,
                env_key_instructions: None,
                experimental_bearer_token: None,
                wire_api: WireApi::Chat,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: default_auth_style(),
                supports_websockets: false,
            },
        ),
        // === NEW: OpenRouter (access to everything) ===
        (
            "openrouter",
            ModelProviderInfo {
                name: "OpenRouter".into(),
                base_url: Some("https://openrouter.ai/api/v1".into()),
                env_key: Some("OPENROUTER_API_KEY".into()),
                env_key_aliases: None,
                env_key_instructions: Some("Get your API key at https://openrouter.ai/keys".into()),
                experimental_bearer_token: None,
                wire_api: WireApi::Chat,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: default_auth_style(),
                supports_websockets: false,
            },
        ),
        // === NEW: Additional common providers ===
        (
            "github",
            ModelProviderInfo {
                name: "GitHub Models".into(),
                base_url: Some("https://models.inference.ai.azure.com".into()),
                env_key: Some("GITHUB_MODELS_API_KEY".into()),
                env_key_aliases: Some(vec!["GITHUB_TOKEN".into()]),
                env_key_instructions: None,
                experimental_bearer_token: None,
                wire_api: WireApi::Chat,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: default_auth_style(),
                supports_websockets: false,
            },
        ),
        (
            "cerebras",
            ModelProviderInfo {
                name: "Cerebras".into(),
                base_url: Some("https://api.cerebras.ai/v1".into()),
                env_key: Some("CEREBRAS_API_KEY".into()),
                env_key_aliases: None,
                env_key_instructions: None,
                experimental_bearer_token: None,
                wire_api: WireApi::Chat,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: default_auth_style(),
                supports_websockets: false,
            },
        ),
        (
            "sambanova",
            ModelProviderInfo {
                name: "SambaNova".into(),
                base_url: Some("https://api.sambanova.ai/v1".into()),
                env_key: Some("SAMBANOVA_API_KEY".into()),
                env_key_aliases: None,
                env_key_instructions: None,
                experimental_bearer_token: None,
                wire_api: WireApi::Chat,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: default_auth_style(),
                supports_websockets: false,
            },
        ),
        (
            "replicate",
            ModelProviderInfo {
                name: "Replicate".into(),
                base_url: Some("https://api.replicate.com/v1".into()),
                env_key: Some("REPLICATE_API_TOKEN".into()),
                env_key_aliases: None,
                env_key_instructions: None,
                experimental_bearer_token: None,
                wire_api: WireApi::Chat,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: default_auth_style(),
                supports_websockets: false,
            },
        ),
        (
            "cohere",
            ModelProviderInfo {
                name: "Cohere".into(),
                base_url: Some("https://api.cohere.ai/v1".into()),
                env_key: Some("COHERE_API_KEY".into()),
                env_key_aliases: None,
                env_key_instructions: None,
                experimental_bearer_token: None,
                wire_api: WireApi::Chat,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: default_auth_style(),
                supports_websockets: false,
            },
        ),
        (
            "huggingface",
            ModelProviderInfo {
                name: "HuggingFace".into(),
                base_url: Some("https://api-inference.huggingface.co/v1".into()),
                env_key: Some("HUGGINGFACE_API_KEY".into()),
                env_key_aliases: Some(vec!["HF_TOKEN".into()]),
                env_key_instructions: Some(
                    "Get your API key at https://huggingface.co/settings/tokens".into(),
                ),
                experimental_bearer_token: None,
                wire_api: WireApi::Chat,
                query_params: None,
                http_headers: None,
                env_http_headers: None,
                request_max_retries: Some(4),
                stream_max_retries: Some(5),
                stream_idle_timeout_ms: Some(300_000),
                websocket_connect_timeout_ms: None,
                requires_openai_auth: false,
                auth_style: default_auth_style(),
                supports_websockets: false,
            },
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}

pub fn create_oss_provider(default_provider_port: u16, wire_api: WireApi) -> ModelProviderInfo {
    // These CODEX_OSS_ environment variables are experimental: we may
    // switch to reading values from config.toml instead.
    let default_codex_oss_base_url = format!(
        "http://localhost:{codex_oss_port}/v1",
        codex_oss_port = std::env::var("CODEX_OSS_PORT")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(default_provider_port)
    );

    let codex_oss_base_url = std::env::var("CODEX_OSS_BASE_URL")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or(default_codex_oss_base_url);
    create_oss_provider_with_base_url(&codex_oss_base_url, wire_api)
}

pub fn create_oss_provider_with_base_url(base_url: &str, wire_api: WireApi) -> ModelProviderInfo {
    ModelProviderInfo {
        name: "gpt-oss".into(),
        base_url: Some(base_url.into()),
        env_key: None,
        env_key_aliases: None,
        env_key_instructions: None,
        experimental_bearer_token: None,
        wire_api,
        query_params: None,
        http_headers: None,
        env_http_headers: None,
        request_max_retries: None,
        stream_max_retries: None,
        stream_idle_timeout_ms: None,
        websocket_connect_timeout_ms: None,
        requires_openai_auth: false,
        auth_style: default_auth_style(),
        supports_websockets: false,
    }
}

#[cfg(test)]
#[path = "model_provider_info_tests.rs"]
mod tests;
