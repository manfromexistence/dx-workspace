use codex_protocol::models::{ContentItem, ResponseItem};
use serde_json::Value;

fn role_to_chat_role(role: &str) -> &str {
    if role == "developer" {
        "system"
    } else {
        role
    }
}

fn content_to_chat_content(content: &[ContentItem]) -> Value {
    if content.len() == 1 {
        match &content[0] {
            ContentItem::InputText { text } => return Value::String(text.clone()),
            ContentItem::OutputText { text } => return Value::String(text.clone()),
            _ => {}
        }
    }

    let parts: Vec<Value> = content
        .iter()
        .map(|c| match c {
            ContentItem::InputText { text } | ContentItem::OutputText { text } => {
                serde_json::json!({
                    "type": "text",
                    "text": text
                })
            }
            ContentItem::InputImage { image_url } => {
                serde_json::json!({
                    "type": "image_url",
                    "image_url": { "url": image_url }
                })
            }
        })
        .collect();

    Value::Array(parts)
}

fn append_tool_call(messages: &mut Vec<Value>, call_id: &str, name: &str, arguments: &str) {
    let tool_call = serde_json::json!({
        "id": call_id,
        "type": "function",
        "function": {
            "name": name,
            "arguments": arguments
        }
    });

    if let Some(last) = messages.last_mut() {
        if last.get("role").and_then(|r| r.as_str()) == Some("assistant") {
            if let Some(tool_calls) = last.get_mut("tool_calls") {
                if let Some(array) = tool_calls.as_array_mut() {
                    array.push(tool_call);
                    return;
                }
            } else if let Some(obj) = last.as_object_mut() {
                obj.insert("tool_calls".to_string(), Value::Array(vec![tool_call]));
                return;
            }
        }
    }

    messages.push(serde_json::json!({
        "role": "assistant",
        "content": null,
        "tool_calls": [tool_call]
    }));
}

pub fn items_to_chat_messages(items: &[ResponseItem]) -> Vec<Value> {
    let mut messages = Vec::new();
    for item in items {
        match item {
            ResponseItem::Message { role, content, .. } => {
                messages.push(serde_json::json!({
                    "role": role_to_chat_role(role),
                    "content": content_to_chat_content(content),
                }));
            }
            ResponseItem::FunctionCall {
                name,
                arguments,
                call_id,
                ..
            } => {
                append_tool_call(&mut messages, call_id, name, arguments);
            }
            ResponseItem::FunctionCallOutput {
                call_id, output, ..
            } => {
                messages.push(serde_json::json!({
                    "role": "tool",
                    "tool_call_id": call_id,
                    "content": output.text_content().unwrap_or_default(),
                }));
            }
            ResponseItem::CustomToolCall {
                call_id,
                name,
                input,
                ..
            } => {
                append_tool_call(&mut messages, call_id, name, input);
            }
            ResponseItem::CustomToolCallOutput {
                call_id, output, ..
            } => {
                messages.push(serde_json::json!({
                    "role": "tool",
                    "tool_call_id": call_id,
                    "content": output.text_content().unwrap_or_default(),
                }));
            }
            ResponseItem::LocalShellCall {
                call_id, action, ..
            } => {
                let name = "local_shell";
                let args = serde_json::to_string(&action).unwrap_or_default();
                append_tool_call(
                    &mut messages,
                    &call_id.clone().unwrap_or_default(),
                    name,
                    &args,
                );
            }
            ResponseItem::ToolSearchCall {
                call_id, arguments, ..
            } => {
                let name = "tool_search";
                let args = serde_json::to_string(&arguments).unwrap_or_default();
                append_tool_call(
                    &mut messages,
                    &call_id.clone().unwrap_or_default(),
                    name,
                    &args,
                );
            }
            ResponseItem::ToolSearchOutput { call_id, tools, .. } => {
                let content = serde_json::to_string(&tools).unwrap_or_default();
                messages.push(serde_json::json!({
                    "role": "tool",
                    "tool_call_id": call_id.clone().unwrap_or_default(),
                    "content": content,
                }));
            }
            ResponseItem::WebSearchCall { id, action, .. } => {
                let name = "web_search";
                let args = serde_json::to_string(&action).unwrap_or_default();
                append_tool_call(&mut messages, &id.clone().unwrap_or_default(), name, &args);
            }
            ResponseItem::ImageGenerationCall {
                id, revised_prompt, ..
            } => {
                let name = "image_generation";
                let args = serde_json::json!({ "prompt": revised_prompt }).to_string();
                append_tool_call(&mut messages, id, name, &args);
            }
            ResponseItem::Reasoning { .. }
            | ResponseItem::GhostSnapshot { .. }
            | ResponseItem::Compaction { .. }
            | ResponseItem::Other => {}
        }
    }
    messages
}

pub fn build_chat_request(
    model: &str,
    instructions: &str,
    items: &[ResponseItem],
    tools: &[Value],
) -> Value {
    let mut messages = Vec::new();
    if !instructions.is_empty() {
        messages.push(serde_json::json!({
            "role": "system",
            "content": instructions
        }));
    }
    messages.extend(items_to_chat_messages(items));

    let mut body = serde_json::json!({
        "model": model,
        "messages": messages,
        "stream": true,
        "stream_options": { "include_usage": true },
    });
    if !tools.is_empty() {
        body["tools"] = serde_json::json!(tools);
    }
    body
}
