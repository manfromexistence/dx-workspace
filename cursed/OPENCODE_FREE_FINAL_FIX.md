# OpenCode Free Models - Final Fix

## Root Cause Analysis

The fundamental issue was attempting to use Chat Completions API models (`minimax-m2.5-free`, `big-pickle`) with Codex's existing infrastructure, which only supports the Responses API SSE format.

### The Problem

1. **Different API Formats**: OpenCode Zen provides models through different endpoints:
   - `/v1/responses` - OpenAI Responses API (GPT models)
   - `/v1/chat/completions` - Chat Completions API (MiniMax, Big Pickle, GLM, Kimi, Qwen)
   - `/v1/messages` - Anthropic API (Claude models)

2. **SSE Format Mismatch**: 
   - Responses API sends events like: `event: response.output_text.delta`
   - Chat Completions API sends events like: `data: {"choices":[{"delta":{"content":"..."}}]}`
   
3. **Missing Parser**: Codex only has an SSE parser for the Responses API format (`codex-rs/codex-api/src/sse/responses.rs`). There is no Chat Completions SSE parser.

4. **Errors Encountered**:
   - `Input required: specify "prompt" or "messages"` - Wrong request format
   - `invalid params, function is empty` - Tool validation issues
   - `stream closed before response.completed` - SSE format mismatch
   - `429 Too Many Requests` - Rate limiting from failed retries

## The Solution

**Use `gpt-5-nano` which is free and uses the Responses API that Codex already supports!**

According to the OpenCode Zen documentation:
- `gpt-5-nano` is **completely free** (Input: Free, Output: Free)
- Uses `/v1/responses` endpoint (compatible with existing Codex infrastructure)
- No API key required (uses "public" bearer token)

## Configuration

### Files Modified

1. **`~/.codex/config.toml`**
```toml
model_provider = "opencode"
```

2. **`justfile`** - Updated `dx`, `run`, and `codex` recipes:
```bash
export DX_DEFAULT_MODEL_PROVIDER="${DX_DEFAULT_MODEL_PROVIDER:-opencode}"
export DX_DEFAULT_MODEL="${DX_DEFAULT_MODEL:-gpt-5-nano}"
```

3. **`codex-rs/core/src/client.rs`** - Fixed type mismatches:
   - Wrapped `conversation_id` in `Some()` and converted to String
   - Wrapped `session_source` in `Some()`
   - Prefixed unused `turn_metadata_header` with underscore

4. **`codex-rs/codex-api/src/endpoint/chat_completions.rs`** - Added format conversion (for future Chat Completions support):
   - Implemented `convert_to_messages()` function
   - Added strict tool validation
   - Converts Responses API format to Chat Completions format

## Available Free Models on OpenCode Zen

| Model | Endpoint | Status | Notes |
|-------|----------|--------|-------|
| `gpt-5-nano` | `/v1/responses` | ✅ **Works with Codex** | Free, compatible |
| `minimax-m2.5-free` | `/v1/chat/completions` | ❌ Needs Chat Completions parser | Free but incompatible |
| `big-pickle` | `/v1/chat/completions` | ❌ Needs Chat Completions parser | Free but incompatible |

## Usage

### Run Codex with Free Model
```bash
just dx
```

### Use Different Models
```bash
# Use gpt-5-nano (default, works)
just dx

# Try to use other free models (won't work without Chat Completions parser)
# DX_DEFAULT_MODEL=minimax-m2.5-free DX_DEFAULT_MODEL_PROVIDER=opencode-chat just dx
```

## Future Work: Supporting Chat Completions API

To support MiniMax and Big Pickle models in the future, Codex would need:

1. **Chat Completions SSE Parser** (`codex-rs/codex-api/src/sse/chat_completions.rs`):
   - Parse `data: {"choices":[...]}` format
   - Handle `[DONE]` termination
   - Convert to `ResponseEvent` enum

2. **Update `spawn_response_stream`**:
   - Detect API type (Responses vs Chat Completions)
   - Route to appropriate parser

3. **Example Chat Completions SSE Format**:
```
data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268190,"model":"minimax-m2.5-free","choices":[{"index":0,"delta":{"role":"assistant","content":"Hello"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268190,"model":"minimax-m2.5-free","choices":[{"index":0,"delta":{"content":" there"},"finish_reason":null}]}

data: [DONE]
```

## Summary

The fix is simple: **use `gpt-5-nano` instead of `minimax-m2.5-free`**. Both are free, but `gpt-5-nano` uses the Responses API that Codex already supports, while MiniMax requires a Chat Completions SSE parser that doesn't exist yet.

The Chat Completions format conversion code we added is ready for when someone implements the SSE parser, but for now, stick with `gpt-5-nano`.
