# Changelog - DX Fork

## [Unreleased] - 2026-03-28

### 🚀 DX TUI: Codex Backend Integration

DX TUI now uses the professional Codex backend for AI interactions! This brings enterprise-grade features while keeping DX's custom UI.

**What's New:**
- **Codex Backend Integration** - Full connection to Codex's ThreadManager and event system
- **Streaming Responses** - Real-time message streaming from Mistral and other providers
- **Tool Execution Tracking** - Visual indicators (⚙️ Running, ✓ Complete, ✗ Failed) for tool calls
- **Multi-Provider Support** - Seamless switching between Codex (Mistral) and local GGUF models
- **Professional Architecture** - Event-driven design with proper async handling and graceful shutdown

**Technical Implementation:**
- Created `codex_agent.rs` - Extracted spawn logic from Codex TUI's `agent.rs`
- Updated `ChatState` - Added Codex channels (`codex_op_tx`, `codex_event_rx`) and session tracking
- Enhanced `dispatcher.rs` - Routes messages to Codex or local LLM based on `ModelProvider`
- Implemented event handling - Processes `SessionConfigured`, `AssistantMessage`, `ToolUse`, `ToolResult`, `Error`, `TurnComplete`, `ShutdownComplete`
- Added tool tracking UI - Shows tool execution status with icons in chat messages
- Graceful shutdown - Drop implementation stops Codex thread cleanly

**Default Configuration:**
- Provider: Mistral AI
- Model: `mistral-large-latest`
- Requires: `MISTRAL_API_KEY` in environment

### Audio System for Animations

Added immersive audio experience to DX TUI's animation mode:

- **Embedded Audio Files** - Using `rodio` library with `include_bytes!` macro
- **Animation-Specific Sounds**:
  - Matrix → matrix.mp3
  - Rain → rain.mp3
  - Waves → wave.mp3
  - Fireworks → fireworks.mp3
  - Starfield → space.mp3
  - Plasma → plasma.mp3
- **Exit Animation Sounds** - Train running and whistle sounds
- **Smart Audio Management** - Stops on mode exit, app close, or file browser navigation
- **Volume Control** - Set to 10% for comfortable listening

### Removed Broken Local LLM

Cleaned up non-functional local LLM stub from Codex core:

- Removed `stream_local_llm()` function that always failed with C++ runtime error
- Removed `codex-local-llm` dependency from core and CLI
- Removed `test-local-llm` CLI command
- Removed `LOCAL_LLM_PROVIDER_ID` and `create_local_llm_provider()`
- DX TUI's working GGUF implementation in `dx/src/llm.rs` remains untouched

## [Unreleased] - 2026-03-24

### 🎉 Multi-Provider Support Fully Restored!

DX now supports **15+ AI providers** out of the box! All providers with API keys in your `.env` file work automatically.

**IMPORTANT**: Models are provider-specific. Always specify both `model_provider` and `model` together when switching providers:
```bash
just dx -c model_provider=groq -c model=llama-3.3-70b-versatile
```

Or use the configure command to set both at once:
```bash
just dx configure groq
```

See `PROVIDERS.md` for complete documentation.

### Added - Multi-Provider Support Restored

- **Restored `wire_api = "chat"`** - Re-added Chat Completions API support that was removed by OpenAI
- **Added new WireApi variants**: `Chat`, `AnthropicMessages`, `GeminiGenerateContent` for different provider protocols
- **Built-in provider definitions** for major AI providers:
  - Mistral AI (`mistral`) - **DEFAULT PROVIDER**
  - Groq (`groq`)
  - DeepSeek (`deepseek`)
  - xAI Grok (`xai`)
  - Together AI (`together`)
  - Fireworks AI (`fireworks`)
  - OpenRouter (`openrouter`)
  - GitHub Models (`github`)
  - Cerebras (`cerebras`)
  - SambaNova (`sambanova`)
  - Replicate (`replicate`)
  - Cohere (`cohere`)
  - HuggingFace (`huggingface`)
  - Anthropic Claude (`anthropic`) - wire API not yet implemented
  - Google Gemini (`gemini`) - wire API not yet implemented
- **Hardcoded Mistral models in models.json**:
  - `mistral-small-latest` (128K context, fastest, default)
  - `mistral-medium-latest` (128K context, balanced)
  - `mistral-tiny-latest` (32K context, simple tasks)
- **Added models for all providers in models.json** (March 24, 2026):
  - Groq: `llama-3.3-70b-versatile`, `llama-3.1-8b-instant`
  - DeepSeek: `deepseek-chat`, `deepseek-reasoner`
  - Cerebras: `llama3.1-8b`
  - OpenRouter: `openrouter/free`
- **Chat Completions streaming implementation** - Full SSE parsing and request building via existing `ApiChatCompletionsClient`
- **Removed provider restrictions** - Relaxed OSS provider validation to allow any provider
- **Interactive provider configuration CLI** - New `codex configure` command:
  - `codex configure <provider>` - Configure a specific provider with interactive model selection
  - `codex configure --scan` - Scan all providers with API keys and show available models
  - `codex configure --auto` - Auto-select first model without interaction
  - Concurrent model fetching from all providers with API keys
  - Displays default model from each provider in scan list

### Changed

- Removed `CHAT_WIRE_API_REMOVED_ERROR` and `OLLAMA_CHAT_PROVIDER_REMOVED_ERROR` constants
- Removed `LEGACY_OLLAMA_CHAT_PROVIDER_ID` error handling
- Updated `set_default_oss_provider()` to accept any provider in built-in list

### Fixed

- Removed incomplete stub implementations for Anthropic and Gemini wire APIs
- Removed duplicate `stream_chat_completions` method
- Fixed `WireApi::Chat` match arm to call correct method with all required parameters
- Cleaned up module declarations and imports
- Fixed SSE error handling to use `CodexErr::Stream` instead of incompatible `ResponseStreamFailed`
- Removed unused imports from `client_common.rs`
- **Fixed 422 Unprocessable Entity errors with Mistral API**:
  - Added role conversion in `convert_to_messages()` to map `developer` role to `system` role (Mistral and other providers don't support the `developer` role)
  - Added content extraction to convert `Vec<ContentItem>` to plain string (Mistral expects string content, not structured arrays)
  - Created `content_to_text()` helper that extracts text from InputText/OutputText items and joins with newlines
  - Added provider detection to only send OpenAI-specific parameters (`text`, `service_tier`) when the provider is actually OpenAI (other providers reject these extra fields)
- **Fixed "OutputTextDelta without active item" panic**:
  - Chat Completions SSE parser now sends `OutputItemAdded` event before the first `OutputTextDelta` to create an active item
  - This matches the expected event ordering that Codex core requires for streaming responses

### Technical Details

This fork restores multi-provider support that was removed when OpenAI deprecated the Chat Completions API in favor of their Responses API. The Responses API is OpenAI-exclusive, locking out all other providers (Mistral, Groq, Anthropic, Google, etc.).

**What was restored:**
- `WireApi::Chat` enum variant and deserializer
- `ChatCompletionsClient` for HTTP streaming
- `spawn_chat_completions_stream()` for SSE parsing
- Tool call format translation between Responses API and Chat Completions API

**What's working:**
- Chat Completions API streaming (Mistral, Groq, DeepSeek, xAI, Together, Fireworks, OpenRouter)
- Full conversation history management
- Tool calls and function calling
- Reasoning tokens (DeepSeek, QwQ)
- Token usage tracking

**What's not yet implemented:**
- Anthropic Messages API (`wire_api = "anthropic"`)
- Google Gemini API (`wire_api = "gemini"`)

### Migration Guide

To use a non-OpenAI provider, set in your `~/.codex/config.toml`:

```toml
model = "mistral-large-latest"
model_provider = "mistral"
```

Or via environment variables:
```bash
export MISTRAL_API_KEY="your-key-here"
```

Available providers: `mistral`, `groq`, `deepseek`, `xai`, `together`, `fireworks`, `openrouter`, `anthropic`, `gemini`
