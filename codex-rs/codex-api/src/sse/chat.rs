use crate::common::ResponseEvent;
use crate::protocol::TokenUsage;
use serde::Deserialize;

#[derive(Deserialize)]
struct ChatChunk {
    choices: Vec<ChatChoice>,
    usage: Option<ChatUsage>,
}

#[derive(Deserialize)]
struct ChatChoice {
    delta: ChatDelta,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct ChatDelta {
    content: Option<String>,
    reasoning_content: Option<String>,
    tool_calls: Option<Vec<ChatToolCallDelta>>,
}

#[derive(Deserialize)]
struct ChatToolCallDelta {
    id: Option<String>,
    function: Option<ChatFunctionDelta>,
}

#[derive(Deserialize)]
struct ChatFunctionDelta {
    name: Option<String>,
    arguments: Option<String>,
}

#[derive(Deserialize)]
struct ChatUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
}

/// Parse a Chat Completions SSE chunk and emit ResponseStream events
pub fn parse_chat_sse_event(data: &str) -> Vec<ResponseEvent> {
    if data == "[DONE]" {
        return vec![ResponseEvent::Done];
    }

    let chunk: ChatChunk = match serde_json::from_str(data) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let mut events = Vec::new();

    for choice in &chunk.choices {
        if let Some(content) = &choice.delta.content {
            events.push(ResponseEvent::OutputTextDelta {
                text: content.clone(),
            });
        }

        if let Some(reasoning) = &choice.delta.reasoning_content {
            events.push(ResponseEvent::ReasoningDelta {
                text: reasoning.clone(),
            });
        }

        if let Some(tool_calls) = &choice.delta.tool_calls {
            for tc in tool_calls {
                if let Some(ref func) = tc.function {
                    if tc.id.is_some() || func.name.is_some() || func.arguments.is_some() {
                        events.push(ResponseEvent::FunctionCallDelta {
                            call_id: tc.id.clone().unwrap_or_default(),
                            name: func.name.clone().unwrap_or_default(),
                            arguments_delta: func.arguments.clone().unwrap_or_default(),
                        });
                    }
                }
            }
        }

        if let Some(ref reason) = choice.finish_reason {
            match reason.as_str() {
                "stop" => events.push(ResponseEvent::OutputTextDone),
                "tool_calls" => events.push(ResponseEvent::FunctionCallDone),
                _ => {}
            }
        }
    }

    if let Some(usage) = &chunk.usage {
        events.push(ResponseEvent::UsageUpdate {
            input_tokens: usage.prompt_tokens,
            output_tokens: usage.completion_tokens,
        });
    }

    events
}
