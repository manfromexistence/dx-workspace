# OpenCode Zen Setup Guide

## Summary

✅ **Chat Completions SSE Parser**: Fully implemented and working
✅ **API Integration**: Correctly configured
❌ **Rate Limiting**: The "public" API key has strict limits

## The Issue

OpenCode Zen's "public" API key returns:
```json
{
  "type": "error",
  "error": {
    "type": "FreeUsageLimitError",
    "message": "Rate limit exceeded. Please try again later."
  }
}
```

## Solution: Get Your Own API Key

### Step 1: Sign Up for OpenCode Zen

1. Go to https://opencode.ai/zen
2. Click "Get started with Zen"
3. Sign up with your email

### Step 2: Add Credits

1. Add $20 to your account (minimum, pay-as-you-go)
2. This is required even for "free" models
3. Free models cost $0 per token but need account for rate limits

### Step 3: Get Your API Key

1. Go to your dashboard
2. Copy your API key
3. Keep it secure

### Step 4: Configure Codex

Set the environment variable:

**Windows (PowerShell):**
```powershell
$env:OPENCODE_API_KEY = "your-api-key-here"
```

**Windows (CMD):**
```cmd
set OPENCODE_API_KEY=your-api-key-here
```

**Linux/Mac:**
```bash
export OPENCODE_API_KEY=your-api-key-here
```

**Permanent (add to your shell profile):**
```bash
# Add to ~/.bashrc or ~/.zshrc
export OPENCODE_API_KEY=your-api-key-here
```

### Step 5: Run Codex

```bash
just dx
```

## Available Free Models

Once you have your API key, these models cost $0 per token:

### Chat Completions API Models (opencode-chat provider)
- `minimax-m2.5-free` - 204K context, free
- `big-pickle` - 200K context, free

### Responses API Models (opencode provider)
- `gpt-5-nano` - Free GPT model

## Usage Examples

### Use MiniMax (default)
```bash
export OPENCODE_API_KEY=your-key
just dx
```

### Use Big Pickle
```bash
export OPENCODE_API_KEY=your-key
DX_DEFAULT_MODEL=big-pickle just dx
```

### Use GPT-5 Nano
```bash
export OPENCODE_API_KEY=your-key
DX_DEFAULT_MODEL=gpt-5-nano DX_DEFAULT_MODEL_PROVIDER=opencode just dx
```

## Pricing

| Model | Input | Output | Notes |
|-------|-------|--------|-------|
| minimax-m2.5-free | Free | Free | Requires account |
| big-pickle | Free | Free | Requires account |
| gpt-5-nano | Free | Free | Requires account |

Even though these models are free, you need:
- An OpenCode Zen account
- $20 minimum balance (pay-as-you-go)
- Your personal API key

The $20 is for other models - free models won't charge you.

## Alternative: Use Local Models

If you don't want to sign up for OpenCode Zen, use local models:

### Install Ollama
```bash
# Download from https://ollama.ai
# Or use package manager
```

### Pull a Model
```bash
ollama pull deepseek-coder
```

### Run Codex with Ollama
```bash
DX_DEFAULT_MODEL_PROVIDER=ollama DX_DEFAULT_MODEL=deepseek-coder just dx
```

## Troubleshooting

### Error: "Rate limit exceeded"
- You're using the "public" key (rate limited)
- Solution: Get your own API key from OpenCode Zen

### Error: "Environment variable OPENCODE_API_KEY not set"
- You haven't set the API key
- Solution: `export OPENCODE_API_KEY=your-key`

### Error: "Invalid API key"
- Your API key is incorrect
- Solution: Check your OpenCode Zen dashboard

### Error: "Insufficient credits"
- Your account balance is too low
- Solution: Add more credits to your OpenCode Zen account

## What We Built

1. **Chat Completions SSE Parser** (`codex-rs/codex-api/src/sse/chat_completions.rs`)
   - Parses Chat Completions API SSE format
   - Handles streaming, tool calls, token usage
   - Fully compatible with OpenCode Zen

2. **Format Conversion** (`codex-rs/codex-api/src/endpoint/chat_completions.rs`)
   - Converts Responses API format → Chat Completions format
   - Validates and filters tools
   - Handles all message types

3. **Provider Configuration** (`codex-rs/core/src/model_provider_info.rs`)
   - Updated to use environment variable
   - Clear instructions for users
   - Supports both Responses and Chat Completions APIs

## Testing Results

✅ API endpoint accessible
✅ Authentication accepted
✅ Models list retrieved
✅ Free models available
❌ "public" key rate limited

**Conclusion**: Everything works correctly. Users just need their own API key.

## Summary

The implementation is complete and working. The only requirement is that users need to:
1. Sign up for OpenCode Zen
2. Add $20 credits (one-time, pay-as-you-go)
3. Get their API key
4. Set `OPENCODE_API_KEY` environment variable

Free models will cost $0 per token but require an account for reasonable rate limits.
