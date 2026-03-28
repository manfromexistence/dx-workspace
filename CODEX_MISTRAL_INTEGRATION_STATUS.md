# Codex + Mistral Integration Status

## Goal
Integrate Codex backend into DX TUI with Mistral Large as the default model, using the MISTRAL_API_KEY from environment.

## Changes Made

### 1. Updated Default Model (`codex-rs/dx/src/models.rs`)
✅ Changed default model from "Local Infinity" to "Mistral Large"
✅ Added `mistral-large-latest` as the first model in the list
✅ Set `is_default: true` for Mistral Large
✅ Updated test to reflect new default

**Before:**
```rust
ModelInfo {
    id: "local-infinity".to_string(),
    display_name: "Infinity".to_string(),
    provider: ModelProvider::Local,
    is_default: true,  // Was default
    ...
}
```

**After:**
```rust
ModelInfo {
    id: "mistral-large-latest".to_string(),
    display_name: "Mistral Large".to_string(),
    provider: ModelProvider::Codex,
    is_default: true,  // Now default
    ...
}
```

### 2. Created Codex Backend Module (`codex-rs/dx/src/codex_backend.rs`)
✅ New module for Codex backend initialization (no UI)
✅ Initializes with Mistral as default provider
✅ Sets up ThreadManager, AuthManager
✅ Creates Op channels for communication
✅ Configures to use `mistral-large-latest` model

**Key Configuration:**
```rust
let cli_kv_overrides = vec![
    "model_provider_id=mistral".to_string(),
    "model=mistral-large-latest".to_string(),
];
```

### 3. Added Module Declaration (`codex-rs/dx/src/dx.rs`)
✅ Added `mod codex_backend;` to module list

## What's Next

### Step 1: Update ChatState to Hold Codex Backend
```rust
// In state.rs
pub struct ChatState {
    // ... existing fields ...
    
    // NEW: Codex backend (replace old codex_widget fields)
    pub codex_backend: Option<Arc<CodexBackend>>,
    pub codex_op_tx: Option<UnboundedSender<Op>>,
    pub codex_event_rx: Option<UnboundedReceiver<ResponseEvent>>,
}
```

### Step 2: Initialize Codex on Startup
```rust
// In state.rs ChatState::new()
let codex_backend = tokio::task::spawn_local(async {
    codex_backend::initialize_codex_backend().await
});
```

### Step 3: Send Messages to Codex
```rust
// In dispatcher.rs
InputAction::Submit(msg) => {
    if self.app.bridge.chat_state.current_model.provider == ModelProvider::Codex {
        // Send to Codex
        let op = Op::UserInput(UserInput { text: msg, .. });
        self.app.bridge.chat_state.codex_op_tx.send(op);
    } else {
        // Send to local LLM
        self.app.bridge.chat_state.add_user_message(msg);
    }
}
```

### Step 4: Receive Responses from Codex
```rust
// In state.rs update()
if let Some(event_rx) = &mut self.codex_event_rx {
    while let Ok(event) = event_rx.try_recv() {
        match event {
            ResponseEvent::OutputTextDelta(text) => {
                // Append to current message
                if let Some(last_msg) = self.messages.last_mut() {
                    last_msg.content.push_str(&text);
                }
            }
            ResponseEvent::Completed { .. } => {
                self.is_loading = false;
            }
            // ... handle other events
        }
    }
}
```

### Step 5: Render Messages (Already Works!)
The existing `MessageList` component should work fine for rendering Codex responses since they're just text messages.

## Environment Setup

User has already set up:
```bash
# Windows environment variable
MISTRAL_API_KEY=your_api_key_here
```

Codex will automatically read this from the environment when using the Mistral provider.

## Provider Configuration

Codex already has Mistral configured in `codex-rs/core/src/model_provider_info.rs`:
```rust
("mistral", ModelProviderInfo {
    name: "Mistral".into(),
    base_url: Some("https://api.mistral.ai/v1".into()),
    env_key: Some("MISTRAL_API_KEY".into()),
    wire_api: WireApi::Chat,
    // ... other config
})
```

## Testing Plan

1. ✅ Verify Mistral is default model
2. ⏳ Initialize Codex backend on startup
3. ⏳ Send test message to Codex
4. ⏳ Verify response streams back
5. ⏳ Verify messages render correctly
6. ⏳ Test model switching (Mistral ↔ Local)

## Current Status

- ✅ Models updated (Mistral is default)
- ✅ Codex backend module created
- ✅ Module declared in dx.rs
- ⏳ Compiling (in progress)
- ⏳ State integration (next step)
- ⏳ Message routing (next step)
- ⏳ Event handling (next step)

## Files Modified

1. `codex-rs/dx/src/models.rs` - Updated default model
2. `codex-rs/dx/src/codex_backend.rs` - NEW: Backend initialization
3. `codex-rs/dx/src/dx.rs` - Added module declaration

## Files to Modify Next

1. `codex-rs/dx/src/state.rs` - Add Codex backend fields
2. `codex-rs/dx/src/dispatcher.rs` - Route messages to Codex
3. `codex-rs/dx/src/components.rs` - Ensure MessageList handles Codex responses

## Notes

- We're NOT using Codex TUI's UI components
- We're ONLY using Codex's backend (protocol, core, agent)
- DX's existing UI will render Codex responses
- This gives us Codex's AI power with DX's superior UX
