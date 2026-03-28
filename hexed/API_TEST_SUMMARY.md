# API Provider Test Results
**Date:** March 26, 2026

## Working Providers ✅

### 1. Mistral (64 models)
- **Status:** Working
- **Key Models:**
  - mistral-large-2512 (latest flagship)
  - mistral-medium-2508
  - mistral-small-2603
  - codestral-2508 (coding specialist)
  - devstral-2512 (development focused)
  - ministral-3b/8b/14b-2512 (efficient models)
  - pixtral-large-2411 (multimodal)

### 2. Cerebras (2 models)
- **Status:** Working
- **Models:**
  - llama3.1-8b
  - qwen-3-235b-a22b-instruct-2507

### 3. OpenRouter (347 models)
- **Status:** Working
- **Notable Models:**
  - openai/gpt-5.4-nano/mini/pro
  - anthropic/claude-sonnet-4.6
  - google/gemini-3.1-pro-preview
  - qwen/qwen3.5-plus-02-15
  - deepseek/deepseek-v3.2
  - meta-llama/llama-4-maverick

### 4. DeepSeek (2 models)
- **Status:** Working
- **Models:**
  - deepseek-chat
  - deepseek-reasoner

### 5. Gemini (47 models)
- **Status:** Working
- **Key Models:**
  - gemini-2.5-flash (1M context)
  - gemini-2.5-pro (2M context)
  - gemini-2.0-flash
  - gemini-3.1-flash-preview

### 6. Cohere (20 models)
- **Status:** Working
- **Key Models:**
  - command-a (latest)
  - command-a-vision-07-2025
  - command-r-plus-08-2024

### 7. SambaNova (16 models)
- **Status:** Working
- **Key Models:**
  - DeepSeek-R1-0528
  - DeepSeek-V3.1
  - DeepSeek-V3-0324
  - Llama-3.3-70B
  - Qwen2.5-Coder-32B

## Failed Providers ❌

### 1. Groq
- **Status:** HTTP 403 (Forbidden)
- **Issue:** API key may be invalid or expired
- **Action:** Need to regenerate API key at https://console.groq.com

### 2. Together AI
- **Status:** HTTP 401 (Unauthorized)
- **Issue:** API key may be invalid or expired
- **Action:** Need to regenerate API key at https://api.together.xyz

### 3. HuggingFace
- **Status:** HTTP 410 (Gone)
- **Issue:** Endpoint may have changed or API key format incorrect
- **Action:** Check HuggingFace Inference API documentation

## Models Added to models.json

Added the following new models:
1. **Mistral Large 2512** - Most capable Mistral model
2. **Codestral 2508** - Specialized coding model
3. **Devstral 2512** - Development-focused model
4. **Qwen 3 235B (Cerebras)** - Large model on fast infrastructure
5. **Gemini 2.5 Flash** - Fast multimodal with 1M context
6. **Gemini 2.5 Pro** - Most capable Gemini with 2M context
7. **Cohere Command A** - Latest Cohere model
8. **DeepSeek R1 (SambaNova)** - Reasoning model
9. **DeepSeek V3.1 (SambaNova)** - Latest DeepSeek on SambaNova

## Recommendations

1. **Regenerate failed API keys** for Groq, Together AI, and HuggingFace
2. **Test the new models** in the TUI to verify they work correctly
3. **Update default model** if desired (currently mistral-small-latest)
4. **Consider adding more Gemini models** - they have excellent context windows
5. **Explore OpenRouter** - provides access to 347 models through a single API

## Next Steps

1. Run `cargo run -j3` from `codex-rs/dx` to test the updated models
2. Try switching between different providers to verify API keys work
3. Update any failed API keys and re-test
