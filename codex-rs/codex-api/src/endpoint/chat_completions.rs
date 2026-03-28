use crate::auth::AuthProvider;
use crate::common::ResponseStream;
use crate::common::ResponsesApiRequest;
use crate::endpoint::session::EndpointSession;
use crate::error::ApiError;
use crate::provider::Provider;
use crate::requests::headers::build_conversation_headers;
use crate::requests::headers::insert_header;
use crate::requests::headers::subagent_header;
use crate::requests::responses::Compression;
use crate::sse::spawn_chat_completions_stream;
use crate::telemetry::SseTelemetry;
use codex_client::HttpTransport;
use codex_client::RequestCompression;
use codex_client::RequestTelemetry;
use codex_protocol::protocol::SessionSource;
use http::HeaderMap;
use http::HeaderValue;
use http::Method;
use serde_json::Value;
use std::sync::Arc;
use std::sync::OnceLock;

pub struct ChatCompletionsClient<T: HttpTransport, A: AuthProvider> {
    session: EndpointSession<T, A>,
    sse_telemetry: Option<Arc<dyn SseTelemetry>>,
}

#[derive(Default)]
pub struct ChatCompletionsOptions {
    pub conversation_id: Option<String>,
    pub session_source: Option<SessionSource>,
    pub extra_headers: HeaderMap,
    pub compression: Compression,
    pub turn_state: Option<Arc<OnceLock<String>>>,
}

impl<T: HttpTransport, A: AuthProvider> ChatCompletionsClient<T, A> {
    pub fn new(transport: T, provider: Provider, auth: A) -> Self {
        Self {
            session: EndpointSession::new(transport, provider, auth),
            sse_telemetry: None,
        }
    }

    pub fn with_telemetry(
        self,
        request: Option<Arc<dyn RequestTelemetry>>,
        sse: Option<Arc<dyn SseTelemetry>>,
    ) -> Self {
        Self {
            session: self.session.with_request_telemetry(request),
            sse_telemetry: sse,
        }
    }

    pub async fn stream_request(
        &self,
        request: ResponsesApiRequest,
        options: ChatCompletionsOptions,
    ) -> Result<ResponseStream, ApiError> {
        let ChatCompletionsOptions {
            conversation_id,
            session_source,
            extra_headers,
            compression,
            turn_state,
        } = options;

        // Convert ResponsesApiRequest to chat completions format
        let mut body = serde_json::json!({
            "model": request.model,
            "messages": convert_to_messages(&request.instructions, &request.input),
            "stream": request.stream,
        });

        // Add tools only if they're non-empty and properly formatted
        if !request.tools.is_empty() {
            // Validate and filter tools - MiniMax is strict about tool format
            let valid_tools: Vec<_> = request
                .tools
                .iter()
                .filter(|tool| {
                    // Must have type="function"
                    if tool.get("type").and_then(|t| t.as_str()) != Some("function") {
                        return false;
                    }

                    // Must have function.name and function.parameters
                    let function = match tool.get("function") {
                        Some(f) => f,
                        None => return false,
                    };

                    let has_name = function
                        .get("name")
                        .and_then(|n| n.as_str())
                        .is_some_and(|s| !s.is_empty());

                    let has_parameters = function.get("parameters").is_some();

                    has_name && has_parameters
                })
                .cloned()
                .collect();

            // Only include tools if we have valid ones
            if !valid_tools.is_empty() {
                body["tools"] = serde_json::json!(valid_tools);
                body["tool_choice"] = serde_json::json!(request.tool_choice);
                if request.parallel_tool_calls {
                    body["parallel_tool_calls"] = serde_json::json!(true);
                }
            }
        }

        if let Some(reasoning) = &request.reasoning {
            body["reasoning"] = serde_json::to_value(reasoning)
                .map_err(|e| ApiError::Stream(format!("failed to encode reasoning: {e}")))?;
        }

        // Only include OpenAI-specific parameters for OpenAI provider
        let provider_name = self.session.provider().name.to_lowercase();
        let is_openai = provider_name == "openai" || provider_name.contains("openai");

        if is_openai {
            if let Some(service_tier) = &request.service_tier {
                body["service_tier"] = serde_json::json!(service_tier);
            }
            if let Some(text) = &request.text {
                body["text"] = serde_json::to_value(text).map_err(|e| {
                    ApiError::Stream(format!("failed to encode text controls: {e}"))
                })?;
            }
        }

        let mut headers = extra_headers;
        headers.extend(build_conversation_headers(conversation_id));
        if let Some(subagent) = subagent_header(&session_source) {
            insert_header(&mut headers, "x-openai-subagent", &subagent);
        }

        self.stream(body, headers, compression, turn_state).await
    }

    fn path() -> &'static str {
        "chat/completions"
    }

    pub async fn stream(
        &self,
        body: Value,
        extra_headers: HeaderMap,
        compression: Compression,
        turn_state: Option<Arc<OnceLock<String>>>,
    ) -> Result<ResponseStream, ApiError> {
        let request_compression = match compression {
            Compression::None => RequestCompression::None,
            Compression::Zstd => RequestCompression::Zstd,
        };

        let stream_response = self
            .session
            .stream_with(
                Method::POST,
                Self::path(),
                extra_headers,
                Some(body),
                |req| {
                    req.headers.insert(
                        http::header::ACCEPT,
                        HeaderValue::from_static("text/event-stream"),
                    );
                    req.compression = request_compression;
                },
            )
            .await?;

        Ok(spawn_chat_completions_stream(
            stream_response,
            self.session.provider().stream_idle_timeout,
            self.sse_telemetry.clone(),
            turn_state,
        ))
    }
}

/// Convert Responses API format (instructions + input) to Chat Completions format (messages)
fn convert_to_messages(
    instructions: &str,
    input: &[codex_protocol::models::ResponseItem],
) -> Vec<Value> {
    use codex_protocol::models::{ContentItem, ResponseItem};

    // Helper to extract text from ContentItem array
    let content_to_text = |content: &[ContentItem]| -> String {
        content
            .iter()
            .filter_map(|item| match item {
                ContentItem::InputText { text } | ContentItem::OutputText { text } => {
                    Some(text.as_str())
                }
                ContentItem::InputImage { .. } => None, // Skip images for now
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let mut messages = Vec::new();

    // Add system message with instructions if not empty
    if !instructions.is_empty() {
        messages.push(serde_json::json!({
            "role": "system",
            "content": instructions
        }));
    }

    // Convert input items to messages
    for item in input {
        match item {
            ResponseItem::Message { role, content, .. } => {
                // Convert "developer" role to "system" for Chat Completions API compatibility
                let chat_role = if role == "developer" {
                    "system"
                } else {
                    role.as_str()
                };
                let text_content = content_to_text(content);
                messages.push(serde_json::json!({
                    "role": chat_role,
                    "content": text_content
                }));
            }
            ResponseItem::FunctionCall {
                name,
                arguments,
                call_id,
                ..
            } => {
                // Function calls are represented as assistant messages with tool_calls
                messages.push(serde_json::json!({
                    "role": "assistant",
                    "content": null,
                    "tool_calls": [{
                        "id": call_id,
                        "type": "function",
                        "function": {
                            "name": name,
                            "arguments": arguments
                        }
                    }]
                }));
            }
            ResponseItem::FunctionCallOutput { call_id, output } => {
                // Function call outputs are tool messages
                messages.push(serde_json::json!({
                    "role": "tool",
                    "tool_call_id": call_id,
                    "content": serde_json::to_string(output).unwrap_or_default()
                }));
            }
            ResponseItem::CustomToolCall {
                name,
                input,
                call_id,
                ..
            } => {
                messages.push(serde_json::json!({
                    "role": "assistant",
                    "content": null,
                    "tool_calls": [{
                        "id": call_id,
                        "type": "function",
                        "function": {
                            "name": name,
                            "arguments": input
                        }
                    }]
                }));
            }
            ResponseItem::CustomToolCallOutput {
                call_id,
                name: _,
                output,
            } => {
                messages.push(serde_json::json!({
                    "role": "tool",
                    "tool_call_id": call_id,
                    "content": serde_json::to_string(output).unwrap_or_default()
                }));
            }
            // For other item types, we can skip them or handle them as needed
            _ => {}
        }
    }

    messages
}
