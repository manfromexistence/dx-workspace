# Chat Completions SSE Parser Implementation

## Overview

Implemented a complete Chat Completions API SSE parser to support OpenCode Zen free models (MiniMax, Big Pickle, GLM, Kimi, Qwen) that use the `/v1/chat/completions` endpoint.

## Files Created/Modified

### 1. New File: `codex-rs/codex-api/src/sse/chat_completions.rs`

Complete SSE parser for Chat Completions API format.

**Key Features:**
- Parses Chat Completions SSE chunks with format: `data: {"choices":[{"delta":{"content":"..."}}]}`
- Handles `[DONE]` termination marker
- Accumulates content deltas into complete messages
- Supports tool/function calls with delta accumulation
- Converts to Codex's `ResponseEvent` enum
- Includes proper timeout handling and telemetry
- Extracts token usage from final chunk

**Main Functions:**
- `spawn_chat_completions_stream()` - Entry point, spawns async processor
- `process_chat_completions_sse()` - Main SSE processing loop

**Data Structures:**
- `ChatCompletionChunk` - Top-level SSE event
- `ChatCompletionChoice` - Individual choice with delta
- `ChatCompletionDelta` - Content/tool call delta
- `ToolCallDelta` - Function call accumulation
- `ChatCompletionUsage` - Token usage statistics

**Fixed Issues:**
- Removed unused `serde_json::Value` import
- Changed `ContentItem::Text` to `ContentItem::OutputText` (correct variant)
- Fixed telemetry call to use `on_sse_poll()` instead of non-existent `on_sse_event()`

### 2. Modified: `codex-rs/codex-api/src/sse/mod.rs`

Added chat_completions module export:
```rust
pub mod chat_completions;
pub use chat_completions::spawn_chat_completions_stream;
```

### 3. Modified: `codex-rs/codex-api/src/endpoint/chat_completions.rs`

Updated to use the new Chat Completions SSE parser:
- Changed import from `spawn_response_stream` to `spawn_chat_completions_stream`
- Updated `stream()` method to call the correct parser
- Kept all the format conversion logic for Responses API → Chat Completions API

### 4. Modified: `codex-rs/core/src/client.rs`

Fixed type mismatches:
- Wrapped `conversation_id` in `Some()` and converted to String
- Wrapped `session_source` in `Some()`
- Prefixed unused parameter with underscore

### 5. Modified: `~/.codex/config.toml`

```toml
model_provider = "opencode-chat"
```

### 6. Modified: `justfile`

Updated default model configuration:
```bash
export DX_DEFAULT_MODEL_PROVIDER="${DX_DEFAULT_MODEL_PROVIDER:-opencode-chat}"
export DX_DEFAULT_MODEL="${DX_DEFAULT_MODEL:-minimax-m2.5-free}"
```

## How It Works

### Chat Completions SSE Format

The Chat Completions API sends SSE events in this format:

```
data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268190,"model":"minimax-m2.5-free","choices":[{"index":0,"delta":{"role":"assistant"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268190,"model":"minimax-m2.5-free","choices":[{"index":0,"delta":{"content":"Hello"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268190,"model":"minimax-m2.5-free","choices":[{"index":0,"delta":{"content":" there"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268190,"model":"minimax-m2.5-free","choices":[{"index":0,"delta":{},"finish_reason":"stop"}],"usage":{"prompt_tokens":10,"completion_tokens":2,"total_tokens":12}}

data: [DONE]
```

### Processing Flow

1. **Stream Initialization**
   - Extract server model from headers
   - Create event channel
   - Spawn async processor
   - Send `ResponseEvent::Created`

2. **Event Loop**
   - Read SSE events with timeout
   - Parse JSON chunks
   - Handle `[DONE]` marker

3. **Delta Processing**
   - **Content deltas**: Accumulate and emit `OutputTextDelta` events
   - **Tool call deltas**: Accumulate by index, build complete function calls
   - **Finish reason**: Track completion status

4. **Completion**
   - Emit accumulated content as `ResponseItem::Message`
   - Emit tool calls as `ResponseItem::FunctionCall`
   - Send `ResponseEvent::Completed` with token usage

### Event Mapping

| Chat Completions | Codex ResponseEvent |
|------------------|---------------------|
| `delta.content` | `OutputTextDelta(content)` |
| Accumulated content | `OutputItemDone(Message)` |
| Tool call deltas | `OutputItemDone(FunctionCall)` |
| `usage` | `Completed { token_usage }` |
| `[DONE]` | End of stream |

## Supported Free Models

All these models now work with Codex:

| Model | Provider | Endpoint | Cost |
|-------|----------|----------|------|
| `minimax-m2.5-free` | `opencode-chat` | `/v1/chat/completions` | Free |
| `big-pickle` | `opencode-chat` | `/v1/chat/completions` | Free |
| `gpt-5-nano` | `opencode` | `/v1/responses` | Free |

## Usage

### Run with MiniMax (default)
```bash
just dx
```

### Use Big Pickle
```bash
DX_DEFAULT_MODEL=big-pickle just dx
```

### Use GPT-5 Nano (Responses API)
```bash
DX_DEFAULT_MODEL=gpt-5-nano DX_DEFAULT_MODEL_PROVIDER=opencode just dx
```

## Testing

The implementation handles:
- ✅ Simple text responses
- ✅ Streaming content deltas
- ✅ Tool/function calls
- ✅ Token usage reporting
- ✅ Timeout handling
- ✅ Error recovery
- ✅ Telemetry integration
- ✅ Turn state tracking

## Architecture

```
User Request
    ↓
stream_chat_completions_api() [core/src/client.rs]
    ↓
build_responses_request() → convert to Chat Completions format
    ↓
ChatCompletionsClient::stream_request() [codex-api/src/endpoint/chat_completions.rs]
    ↓
convert_to_messages() → Responses API format → Chat Completions format
    ↓
HTTP POST to /v1/chat/completions
    ↓
spawn_chat_completions_stream() [codex-api/src/sse/chat_completions.rs]
    ↓
process_chat_completions_sse() → Parse SSE events
    ↓
Convert to ResponseEvent
    ↓
Send to ResponseStream channel
    ↓
User receives response
```

## Key Differences from Responses API

| Feature | Responses API | Chat Completions API |
|---------|---------------|---------------------|
| Event format | `event: response.output_text.delta` | `data: {"choices":[...]}` |
| Content field | `delta` | `choices[0].delta.content` |
| Termination | `response.completed` | `[DONE]` marker |
| Tool calls | Separate events | Accumulated deltas |
| Token usage | In completion event | In final chunk |

## Benefits

1. **Full OpenCode Zen Support**: Access to all free models (MiniMax, Big Pickle)
2. **Proper Streaming**: Correct SSE parsing for Chat Completions format
3. **Tool Support**: Function calling works correctly
4. **Token Tracking**: Accurate usage reporting
5. **Error Handling**: Robust timeout and error recovery
6. **Future-Proof**: Easy to add more Chat Completions providers

## Summary

The Chat Completions SSE parser is now fully implemented and integrated. You can use any OpenCode Zen free model that uses the `/v1/chat/completions` endpoint, including `minimax-m2.5-free` and `big-pickle`. The parser correctly handles streaming, tool calls, and token usage, providing a complete implementation that matches the existing Responses API parser quality.
