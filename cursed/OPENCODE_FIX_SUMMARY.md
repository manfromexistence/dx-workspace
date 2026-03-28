# OpenCode Free Models Configuration Fix

## Issues Fixed

### 1. Type Mismatch Errors in `codex-rs/core/src/client.rs`

**Problem:** Lines 847-848 had type mismatches:
- `conversation_id` expected `Option<String>` but got `ThreadId`
- `session_source` expected `Option<SessionSource>` but got `SessionSource`

**Solution:** 
- Wrapped `conversation_id` in `Some()` and converted to String using `.to_string()`
- Wrapped `session_source` in `Some()`

### 2. Unused Variable Warning

**Problem:** Parameter `turn_metadata_header` in `stream_chat_completions_api` was unused.

**Solution:** Prefixed with underscore: `_turn_metadata_header`

### 3. Wrong Model Provider Configuration

**Problem:** The justfile and config were using:
- Provider: `opencode` (uses Responses API `/v1/responses`)
- Model: `trinity-large-preview-free` (doesn't exist in OpenCode Zen)

According to [OpenCode Zen documentation](https://opencode.ai/docs/zen/), different models require different API endpoints:
- **GPT models** → `/v1/responses` (Responses API)
- **MiniMax models** → `/v1/chat/completions` (Chat Completions API)
- **Claude models** → `/v1/messages` (Anthropic API)
- **GLM, Kimi, Qwen, Big Pickle** → `/v1/chat/completions`

**Solution:** Updated to use actual free models:
- Provider: `opencode-chat` (uses Chat Completions API)
- Model: `minimax-m2.5-free` (verified free model)

### 4. Chat Completions API Format Mismatch

**Problem:** The code was sending Responses API format to Chat Completions endpoint:
```json
{
  "model": "...",
  "instructions": "...",
  "input": [...]
}
```

But Chat Completions API expects:
```json
{
  "model": "...",
  "messages": [{"role": "user", "content": "..."}]
}
```

This caused the error: `Input required: specify "prompt" or "messages"`

**Solution:** Implemented proper format conversion in `codex-rs/codex-api/src/endpoint/chat_completions.rs`:
- Added `convert_to_messages()` function to transform Responses API format to Chat Completions format
- System instructions → system message
- Input items → messages array with proper roles (user, assistant, tool)
- Function calls → assistant messages with tool_calls
- Function outputs → tool messages

### 5. MiniMax Tool Validation Error

**Problem:** MiniMax returned error: `invalid params, function is empty (2013)`

This occurred because MiniMax is strict about tool definitions. Tools must be properly formatted or excluded entirely.

**Solution:** Implemented strict tool validation:
- Verify each tool has `type="function"`
- Ensure `function` object exists with non-empty `name`
- Require `function.parameters` to be present
- Filter out any malformed tools
- Only include tools array if at least one valid tool exists

This prevents sending empty or malformed tool definitions to strict providers like MiniMax.

## Files Modified

1. `codex-rs/core/src/client.rs` - Fixed type mismatches and unused variable
2. `~/.codex/config.toml` - Changed provider to `opencode-chat`
3. `justfile` - Updated default provider and model in `dx`, `run`, and `codex` recipes
4. `codex-rs/codex-api/src/endpoint/chat_completions.rs` - Implemented proper API format conversion and strict tool validation

## Available Free Models on OpenCode Zen

According to the documentation, these models are completely free:
- `minimax-m2.5-free` - Free MiniMax model (limited time) - Uses Chat Completions API
- `big-pickle` - Free stealth model (limited time) - Uses Chat Completions API
- `gpt-5-nano` - Free GPT model - Uses Responses API

## Usage

Now you can run:
```bash
just dx
```

Or to use a different free model:
```bash
# For Chat Completions API models (MiniMax, Big Pickle, etc.)
DX_DEFAULT_MODEL=big-pickle just dx

# For Responses API models (GPT)
DX_DEFAULT_MODEL=gpt-5-nano DX_DEFAULT_MODEL_PROVIDER=opencode just dx
```

## Important Notes

- MiniMax and Big Pickle models use the Chat Completions API (`opencode-chat` provider)
- GPT models use the Responses API (`opencode` provider)
- The free models are available for a limited time while teams collect feedback
- The conversion function handles all ResponseItem types including messages, function calls, and tool outputs
- Tool validation is strict to comply with MiniMax requirements - malformed tools are filtered out
