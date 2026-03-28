use crate::common::ResponseEvent;
use crate::common::ResponseStream;
use crate::error::ApiError;
use crate::telemetry::SseTelemetry;
use codex_client::ByteStream;
use codex_client::StreamResponse;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::protocol::TokenUsage;
use eventsource_stream::Eventsource;
use futures::StreamExt;
use serde::Deserialize;
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::Instant;
use tokio::time::timeout;
use tracing::debug;
use tracing::trace;
use tracing::warn;

/// Spawns a response stream for Chat Completions API SSE format.
pub fn spawn_chat_completions_stream(
    stream_response: StreamResponse,
    idle_timeout: Duration,
    telemetry: Option<Arc<dyn SseTelemetry>>,
    turn_state: Option<Arc<OnceLock<String>>>,
) -> ResponseStream {
    let server_model = stream_response
        .headers
        .get("openai-model")
        .and_then(|v| v.to_str().ok())
        .map(ToString::to_string);

    if let Some(turn_state) = turn_state.as_ref() {
        if let Some(header_value) = stream_response
            .headers
            .get("x-codex-turn-state")
            .and_then(|v| v.to_str().ok())
        {
            let _ = turn_state.set(header_value.to_string());
        }
    }

    let (tx_event, rx_event) = mpsc::channel::<Result<ResponseEvent, ApiError>>(1600);

    tokio::spawn(async move {
        if let Some(model) = server_model {
            let _ = tx_event.send(Ok(ResponseEvent::ServerModel(model))).await;
        }
        process_chat_completions_sse(stream_response.bytes, tx_event, idle_timeout, telemetry)
            .await;
    });

    ResponseStream { rx_event }
}

#[derive(Debug, Deserialize)]
struct ChatCompletionChunk {
    id: String,
    #[allow(dead_code)]
    object: String,
    #[allow(dead_code)]
    created: i64,
    #[allow(dead_code)]
    model: String,
    choices: Vec<ChatCompletionChoice>,
    #[serde(default)]
    usage: Option<ChatCompletionUsage>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionChoice {
    #[allow(dead_code)]
    index: i64,
    delta: ChatCompletionDelta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionDelta {
    #[serde(default)]
    #[allow(dead_code)]
    role: Option<String>,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<ToolCallDelta>>,
}

#[derive(Debug, Deserialize)]
struct ToolCallDelta {
    index: i64,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    r#type: Option<String>,
    #[serde(default)]
    function: Option<FunctionCallDelta>,
}

#[derive(Debug, Deserialize)]
struct FunctionCallDelta {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    arguments: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionUsage {
    prompt_tokens: i64,
    completion_tokens: i64,
    total_tokens: i64,
    #[serde(default)]
    prompt_tokens_details: Option<PromptTokensDetails>,
    #[serde(default)]
    completion_tokens_details: Option<CompletionTokensDetails>,
}

#[derive(Debug, Deserialize)]
struct PromptTokensDetails {
    #[serde(default)]
    cached_tokens: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct CompletionTokensDetails {
    #[serde(default)]
    reasoning_tokens: Option<i64>,
}

impl From<ChatCompletionUsage> for TokenUsage {
    fn from(val: ChatCompletionUsage) -> Self {
        TokenUsage {
            input_tokens: val.prompt_tokens,
            cached_input_tokens: val
                .prompt_tokens_details
                .and_then(|d| d.cached_tokens)
                .unwrap_or(0),
            output_tokens: val.completion_tokens,
            reasoning_output_tokens: val
                .completion_tokens_details
                .and_then(|d| d.reasoning_tokens)
                .unwrap_or(0),
            total_tokens: val.total_tokens,
        }
    }
}

/// State for accumulating tool calls across deltas
#[derive(Debug, Default)]
struct ToolCallAccumulator {
    id: Option<String>,
    name: String,
    arguments: String,
}

async fn process_chat_completions_sse(
    bytes: ByteStream,
    tx_event: mpsc::Sender<Result<ResponseEvent, ApiError>>,
    idle_timeout: Duration,
    telemetry: Option<Arc<dyn SseTelemetry>>,
) {
    let mut event_stream = bytes.eventsource();
    let mut response_id: Option<String> = None;
    let mut accumulated_content = String::new();
    let mut tool_calls: std::collections::HashMap<i64, ToolCallAccumulator> =
        std::collections::HashMap::new();
    let mut last_finish_reason: Option<String> = None;
    let mut message_item_added = false; // Track if we've sent OutputItemAdded for the message

    // Send Created event at the start
    let _ = tx_event.send(Ok(ResponseEvent::Created)).await;

    loop {
        let event_result = match timeout(idle_timeout, event_stream.next()).await {
            Ok(Some(result)) => result,
            Ok(None) => {
                debug!("Chat completions stream ended");
                break;
            }
            Err(_) => {
                warn!("Chat completions stream idle timeout");
                let _ = tx_event
                    .send(Err(ApiError::Stream(
                        "stream idle timeout exceeded".to_string(),
                    )))
                    .await;
                break;
            }
        };

        let event = match event_result {
            Ok(event) => event,
            Err(err) => {
                warn!(?err, "Chat completions stream error");
                let _ = tx_event
                    .send(Err(ApiError::Stream(format!("stream error: {err}"))))
                    .await;
                break;
            }
        };

        trace!(?event, "Received chat completion event");

        // Telemetry for SSE polling
        if let Some(t) = telemetry.as_ref() {
            let poll_start = Instant::now();
            t.on_sse_poll(&Ok(Some(Ok(event.clone()))), poll_start.elapsed());
        }

        // Check for [DONE] marker
        if event.data.trim() == "[DONE]" {
            debug!("Received [DONE] marker");
            break;
        }

        // Parse the chunk
        let chunk: ChatCompletionChunk = match serde_json::from_str(&event.data) {
            Ok(chunk) => chunk,
            Err(err) => {
                debug!(?err, data = ?event.data, "Failed to parse chat completion chunk");
                continue;
            }
        };

        // Store response ID
        if response_id.is_none() {
            response_id = Some(chunk.id.clone());
        }

        // Process choices
        for choice in chunk.choices {
            // Handle content delta
            if let Some(content) = choice.delta.content {
                if !content.is_empty() {
                    // Send OutputItemAdded before the first content delta
                    if !message_item_added {
                        let message_item = ResponseItem::Message {
                            id: None,
                            role: "assistant".to_string(),
                            content: vec![],
                            end_turn: None,
                            phase: None,
                        };
                        let _ = tx_event
                            .send(Ok(ResponseEvent::OutputItemAdded(message_item)))
                            .await;
                        message_item_added = true;
                    }

                    accumulated_content.push_str(&content);
                    let _ = tx_event
                        .send(Ok(ResponseEvent::OutputTextDelta(content)))
                        .await;
                }
            }

            // Handle tool calls
            if let Some(tool_call_deltas) = choice.delta.tool_calls {
                for tool_delta in tool_call_deltas {
                    let accumulator = tool_calls.entry(tool_delta.index).or_default();

                    if let Some(id) = tool_delta.id {
                        accumulator.id = Some(id);
                    }

                    if let Some(function) = tool_delta.function {
                        if let Some(name) = function.name {
                            accumulator.name.push_str(&name);
                        }
                        if let Some(args) = function.arguments {
                            accumulator.arguments.push_str(&args);
                        }
                    }
                }
            }

            // Store finish reason
            if let Some(reason) = choice.finish_reason {
                last_finish_reason = Some(reason);
            }
        }

        // Handle usage (final chunk)
        if let Some(usage) = chunk.usage {
            let token_usage: TokenUsage = usage.into();

            // Emit accumulated content as a message if we have any
            if !accumulated_content.is_empty() {
                let message_item = ResponseItem::Message {
                    id: None,
                    role: "assistant".to_string(),
                    content: vec![ContentItem::OutputText {
                        text: accumulated_content.clone(),
                    }],
                    end_turn: None,
                    phase: None,
                };
                let _ = tx_event
                    .send(Ok(ResponseEvent::OutputItemDone(message_item)))
                    .await;
            }

            // Emit tool calls if we have any
            for (_, accumulator) in tool_calls.drain() {
                if let Some(call_id) = accumulator.id {
                    let tool_call_item = ResponseItem::FunctionCall {
                        id: None,
                        name: accumulator.name,
                        namespace: None,
                        arguments: accumulator.arguments,
                        call_id,
                    };
                    let _ = tx_event
                        .send(Ok(ResponseEvent::OutputItemDone(tool_call_item)))
                        .await;
                }
            }

            // Send completion event
            let _ = tx_event
                .send(Ok(ResponseEvent::Completed {
                    response_id: response_id.clone().unwrap_or_else(|| chunk.id.clone()),
                    token_usage: Some(token_usage),
                }))
                .await;
            break;
        }
    }

    // If we didn't get a completion event with usage, send one anyway
    if last_finish_reason.is_some() && !accumulated_content.is_empty() {
        let message_item = ResponseItem::Message {
            id: None,
            role: "assistant".to_string(),
            content: vec![ContentItem::OutputText {
                text: accumulated_content,
            }],
            end_turn: None,
            phase: None,
        };
        let _ = tx_event
            .send(Ok(ResponseEvent::OutputItemDone(message_item)))
            .await;

        let _ = tx_event
            .send(Ok(ResponseEvent::Completed {
                response_id: response_id.unwrap_or_else(|| "unknown".to_string()),
                token_usage: None,
            }))
            .await;
    }
}
