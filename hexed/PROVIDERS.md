# Multi-Provider Support in DX

DX (Codex CLI fork) supports **15+ AI providers** out of the box. All providers with API keys in your `.env` file are automatically available.

## ✅ Fully Working Providers (Chat Completions API)

These providers use the standard OpenAI-compatible Chat Completions API (`/v1/chat/completions`) and work perfectly with DX:

| Provider | ID | Base URL | API Key Env Var | Free Tier |
|----------|----|-----------|--------------------|-----------|
| **Mistral AI** | `mistral` | `https://api.mistral.ai/v1` | `MISTRAL_API_KEY` | 1B tokens/month |
| **Groq** | `groq` | `https://api.groq.com/openai/v1` | `GROQ_API_KEY` | 14,400 req/day |
| **DeepSeek** | `deepseek` | `https://api.deepseek.com/v1` | `DEEPSEEK_API_KEY` | Free credits |
| **Together AI** | `together` | `https://api.together.xyz/v1` | `TOGETHER_API_KEY` | Free tier |
| **Fireworks AI** | `fireworks` | `https://api.fireworks.ai/inference/v1` | `FIREWORKS_API_KEY` | Free tier |
| **OpenRouter** | `openrouter` | `https://openrouter.ai/api/v1` | `OPENROUTER_API_KEY` | 200 req/day |
| **xAI (Grok)** | `xai` | `https://api.x.ai/v1` | `XAI_API_KEY` | Paid |
| **GitHub Models** | `github` | `https://models.inference.ai.azure.com` | `GITHUB_MODELS_API_KEY` | Free with GitHub |
| **Cerebras** | `cerebras` | `https://api.cerebras.ai/v1` | `CEREBRAS_API_KEY` | 1M tokens/day |
| **SambaNova** | `sambanova` | `https://api.sambanova.ai/v1` | `SAMBANOVA_API_KEY` | Free tier |
| **Replicate** | `replicate` | `https://api.replicate.com/v1` | `REPLICATE_API_TOKEN` | Free credits |
| **Cohere** | `cohere` | `https://api.cohere.ai/v1` | `COHERE_API_KEY` | 1,000 req/month |
| **HuggingFace** | `huggingface` | `https://api-inference.huggingface.co/v1` | `HUGGINGFACE_API_KEY` | Free tier |

## 🚧 Partially Implemented

These providers require custom wire protocol implementations (not yet complete):

| Provider | ID | Wire API | Status |
|----------|----|-----------|--------------------|
| **Anthropic Claude** | `anthropic` | `AnthropicMessages` | Wire API stub exists, not implemented |
| **Google Gemini** | `gemini` | `GeminiGenerateContent` | Wire API stub exists, not implemented |

## 🏠 Local Providers

| Provider | ID | Description |
|----------|----|--------------------|
| **Local LLM** | `local-llm` | Direct llama.cpp integration (no HTTP) |
| **Ollama** | `ollama` | Local Ollama server (port 11434) |
| **LM Studio** | `lmstudio` | Local LM Studio server (port 1234) |

---

## Quick Start

### 1. Set API Keys in `.env`

```bash
# Add your API keys to the root .env file
MISTRAL_API_KEY="your-key-here"
GROQ_API_KEY="your-key-here"
DEEPSEEK_API_KEY="your-key-here"
# ... etc
```

### 2. Configure Default Provider

```bash
# Option 1: Set in .env
DX_DEFAULT_MODEL_PROVIDER="mistral"
DX_DEFAULT_MODEL="mistral-small-latest"

# Option 2: Use the configure command
just dx configure mistral

# Option 3: Scan all providers and choose interactively
just dx configure --scan
```

### 3. Run DX

```bash
just dx
```

---

## Provider Details

### Mistral AI (Default)
- **Best for**: General purpose, fast responses, large context (128K)
- **Models**: 
  - `mistral-small-latest` (128K, fastest, default)
  - `mistral-medium-latest` (128K, balanced)
  - `mistral-tiny-latest` (32K, simple tasks)
- **Free tier**: 1 billion tokens/month
- **Sign up**: https://console.mistral.ai

### Groq
- **Best for**: Blazing fast inference (LPU-powered)
- **Models**: 
  - `llama-3.3-70b-versatile` (128K, most capable)
  - `llama-3.1-8b-instant` (8K, fastest)
  - `meta-llama/llama-4-scout-17b-16e-instruct`
- **Free tier**: 14,400 requests/day, 30 RPM, 6,000 TPM
- **Sign up**: https://console.groq.com

### DeepSeek
- **Best for**: Coding tasks, reasoning
- **Models**: 
  - `deepseek-chat` (64K, general purpose)
  - `deepseek-reasoner` (64K, complex reasoning)
- **Free tier**: Free credits on signup
- **Sign up**: https://platform.deepseek.com

### Together AI
- **Best for**: Open-source models, fine-tuning
- **Models**: 50+ open-source models (Llama, Mixtral, Qwen, etc.)
- **Free tier**: Available
- **Sign up**: https://api.together.xyz

### OpenRouter
- **Best for**: Access to 30+ providers through one API
- **Models**: 
  - `openrouter/free` (auto-routes to free models)
  - `nvidia/nemotron-3-super-120b-a12b:free`
  - `minimax/minimax-m2.5:free`
  - 200+ other models
- **Free tier**: 200 requests/day on free models
- **Sign up**: https://openrouter.ai

### Cerebras
- **Best for**: Fastest inference (RDU-powered)
- **Models**: 
  - `llama3.1-8b` (8K, ultra-fast)
  - `qwen-3-235b-a22b-instruct-2507` (128K, large)
- **Free tier**: 1,000,000 tokens/day
- **Sign up**: https://cloud.cerebras.ai

### GitHub Models
- **Best for**: Free access with GitHub account
- **Models**: GPT-4o, Llama, Mistral, Phi, and more
- **Free tier**: Rate limited (RPM/RPD/TPR/concurrent)
- **Sign up**: https://github.com/marketplace/models

---

## Usage Examples

### Switch Provider on the Fly

```bash
# IMPORTANT: Always specify BOTH provider and model together
# Models are provider-specific and cannot be mixed

# Use Groq for fast responses
just dx -c model_provider=groq -c model=llama-3.3-70b-versatile

# Use DeepSeek for coding
just dx -c model_provider=deepseek -c model=deepseek-chat

# Use Cerebras for maximum speed
just dx -c model_provider=cerebras -c model=llama3.1-8b
```

### Configure Provider Permanently

```bash
# Interactive configuration (recommended)
just dx configure groq

# Auto-select first model
just dx configure groq --auto

# Scan all providers with API keys
just dx configure --scan
```

### Important Notes

- **Models are provider-specific**: You cannot use a Groq model with the Mistral provider
- **Switching models in TUI**: If you use `/model` in the TUI to switch models, make sure you're using a model from the current provider
- **Best practice**: Use `just dx configure <provider>` to properly set both provider and model together

### Check Available Models

```bash
# List models from a specific provider
curl -H "Authorization: Bearer $MISTRAL_API_KEY" https://api.mistral.ai/v1/models

# Or use the configure command to see available models
just dx configure mistral
```

---

## Architecture Notes

### Wire API Types

DX supports multiple wire protocols:

1. **Responses API** (`wire_api = "responses"`)
   - OpenAI's proprietary stateful API
   - Uses `previous_response_id` for conversation continuity
   - Endpoint: `/v1/responses`

2. **Chat Completions API** (`wire_api = "chat"`)
   - Standard OpenAI-compatible API
   - Stateless (full message history each turn)
   - Endpoint: `/v1/chat/completions`
   - **Most providers use this**

3. **Anthropic Messages API** (`wire_api = "anthropic_messages"`)
   - Anthropic's custom protocol
   - Endpoint: `/v1/messages`
   - Not yet implemented

4. **Gemini Generate Content API** (`wire_api = "gemini_generate_content"`)
   - Google's custom protocol
   - Endpoint: `/v1beta/models/{model}:streamGenerateContent`
   - Not yet implemented

### Protocol Translation

DX automatically translates between the Responses API format (used internally) and the Chat Completions format (used by most providers):

- **Role mapping**: `developer` → `system`
- **Content extraction**: `Vec<ContentItem>` → plain string
- **Tool call format**: Responses API format ↔ Chat Completions format
- **Conversation history**: Managed locally for stateless providers

### Provider-Specific Quirks

- **OpenAI-specific parameters** (`text`, `service_tier`) are only sent to OpenAI
- **Mistral** requires string content (not arrays)
- **Groq** has strict rate limits but blazing fast inference
- **Cerebras** offers the highest free tier (1M tokens/day)
- **OpenRouter** provides access to all providers through one API

---

## Adding Custom Providers

You can add any OpenAI-compatible provider by editing `~/.codex/config.toml`:

```toml
[model_providers.my_custom_provider]
name = "My Custom Provider"
base_url = "https://api.example.com/v1"
env_key = "MY_PROVIDER_API_KEY"
wire_api = "chat"
```

Then set the API key in your `.env`:

```bash
MY_PROVIDER_API_KEY="your-key-here"
```

---

## Troubleshooting

### Provider Not Working

1. **Check API key**: Ensure the environment variable is set in `.env`
2. **Verify base URL**: Some providers have different endpoints
3. **Check rate limits**: Free tiers have strict limits
4. **Test with curl**: Verify the provider's API works directly

### 422 Unprocessable Entity

This usually means:
- Provider doesn't support a parameter (e.g., `text`, `service_tier`)
- Content format is wrong (array vs string)
- Role is invalid (e.g., `developer` not supported)

DX handles these automatically for known providers.

### "OutputTextDelta without active item" Panic

This means the SSE parser isn't sending `OutputItemAdded` before `OutputTextDelta`. This is fixed for Chat Completions providers.

---

## Contributing

To add support for a new provider:

1. Add provider definition to `codex-rs/core/src/model_provider_info.rs`
2. If it uses a custom wire protocol, implement it in `codex-rs/codex-api/src/endpoint/`
3. Add SSE parsing in `codex-rs/codex-api/src/sse/`
4. Test with `just dx`

---

## License

DX is a fork of OpenAI's Codex CLI (Rust). See LICENSE for details.
