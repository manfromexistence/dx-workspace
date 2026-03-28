# Changing Default Model from GPT to Mistral

## Changes Made

### 1. codex-rs/dx/src/codex_integration.rs
Changed hardcoded "gpt-4" defaults to "mistral-large-latest":
- Line 94: `or_else(|| Some("mistral-large-latest".to_string()))`
- Line 107-108: Session telemetry model names
- Line 137: ChatWidget initialization default model

### 2. Red Background Fix
Fixed `RedBackgroundWrapper` in `codex-rs/dx/src/chatwidget.rs`:
- Apply red background BEFORE rendering content
- Re-apply AFTER rendering to ensure it persists

## Why Model Might Still Show GPT

The model is **persisted to config** when you select it. Even if we change the code default, if you have:

1. **~/.codex/config.toml** with `model = "gpt-5.3-codex-high"`
2. **Thread history** with saved model preference
3. **Profile config** with model setting

The persisted value will override the code default.

## Solution

### Option A: Delete Config (Nuclear)
```bash
rm ~/.codex/config.toml
```
Then run `cargo run` - it will use the new default.

### Option B: Edit Config Manually
Edit `~/.codex/config.toml`:
```toml
model = "mistral-large-latest"
```

### Option C: Use /model Command
In the TUI, type:
```
/model mistral-large-latest
```
This will switch and persist the model.

### Option D: Clear Thread History
The thread might have a saved model. Starting a new session should use the default.

## Testing

```bash
cargo run
```

Look at the model name in the status line (bottom of screen). It should show "mistral-large-latest" instead of "gpt-5.3-codex-high".

If it still shows GPT, the config file is overriding the code default.
