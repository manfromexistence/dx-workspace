# Codex Connection Logic Extraction Checklist

This checklist identifies what to extract from Codex TUI for DX TUI integration.

## ✅ WHAT TO EXTRACT (Connection Logic Only)

### 1. Agent Spawning (`codex-rs/tui/src/chatwidget/agent.rs`)
- [x] `spawn_agent()` function - Core connection logic
  - Creates Op channels (tx/rx)
  - Starts thread with ThreadManager
  - Spawns tokio task for event loop
  - Forwards SessionConfigured event
  - Spawns Op forwarding loop
  - Listens for events from thread.next_event()
  - Handles ShutdownComplete as terminal event
- [ ] `spawn_agent_from_existing()` - For forked threads (optional for now)
- [ ] `spawn_op_forwarder()` - For op-only forwarding (optional for now)
- [x] `initialize_app_server_client_name()` - Sets TUI client name

### 2. Event Routing (`codex-rs/tui/src/app_event.rs`)
- [x] `AppEvent::CodexEvent(Event)` - Wraps Codex events for app routing
- [x] `AppEvent::CodexOp(Op)` - Wraps Ops for submission
- [ ] Event channel pattern - Use unbounded_channel for app events

### 3. Event Processing Pattern (from `codex-rs/tui/src/app.rs`)
- [ ] Event loop that receives from multiple sources:
  - Codex events from agent
  - UI events from terminal
  - Timer events
- [ ] `handle_codex_event_now()` - Process events immediately
- [ ] Event routing to appropriate handlers based on EventMsg type

### 4. Configuration Setup (from our `codex_backend.rs`)
- [x] Already implemented in `codex-rs/dx/src/codex_backend.rs`
- [x] Mistral as default provider
- [x] ThreadManager initialization
- [x] AuthManager setup
- [x] Config loading with CLI overrides

---

## ❌ WHAT TO IGNORE (TUI-Specific Rendering)

### 1. Widget Rendering
- ❌ All `ratatui` widget code
- ❌ `ChatWidget` struct and its rendering methods
- ❌ Message list rendering
- ❌ Status line rendering
- ❌ Bottom pane rendering
- ❌ Popup rendering

### 2. TUI-Specific State
- ❌ Scroll positions
- ❌ Cursor positions
- ❌ Terminal size calculations
- ❌ Color/theme rendering
- ❌ Layout calculations

### 3. TUI Event Handling
- ❌ Keyboard event handling (we have our own)
- ❌ Mouse event handling (we have our own)
- ❌ Terminal resize handling (we have our own)

### 4. UI-Specific Features
- ❌ External editor integration
- ❌ File picker UI
- ❌ Model picker UI
- ❌ Approval popup UI
- ❌ Skills list UI
- ❌ Plugin marketplace UI

---

## 🔧 WHAT TO ADAPT FOR DX TUI

### 1. Event Bus Architecture
**Codex TUI Pattern:**
```rust
// Codex TUI uses AppEvent enum to route events
pub enum AppEvent {
    CodexEvent(Event),
    CodexOp(Op),
    // ... other UI events
}

// Events sent via AppEventSender
app_event_tx.send(AppEvent::CodexEvent(event));
```

**DX TUI Adaptation:**
```rust
// We'll add Codex event handling to our existing Event enum
// in dispatcher.rs or create a new codex_events.rs module
pub enum DxEvent {
    // Existing DX events
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    // NEW: Codex events
    CodexEvent(codex_protocol::protocol::Event),
}
```

### 2. Agent Spawning
**Codex TUI Pattern:**
```rust
pub fn spawn_agent(
    config: Config,
    app_event_tx: AppEventSender,
    server: Arc<ThreadManager>,
) -> UnboundedSender<Op>
```

**DX TUI Adaptation:**
```rust
// Create codex_agent.rs module
pub fn spawn_codex_agent(
    thread_manager: Arc<ThreadManager>,
    config: Config,
    event_tx: UnboundedSender<DxEvent>, // Our event channel
) -> UnboundedSender<Op>
```

### 3. Event Processing Loop
**Codex TUI Pattern:**
```rust
// In app.rs run() method
loop {
    tokio::select! {
        Some(event) = app_event_rx.recv() => {
            match event {
                AppEvent::CodexEvent(ev) => handle_codex_event(ev),
                // ... other events
            }
        }
        // ... other event sources
    }
}
```

**DX TUI Adaptation:**
```rust
// In state.rs update() method
// Add codex_event_rx field to ChatState
pub struct ChatState {
    // ... existing fields
    pub codex_event_rx: Option<UnboundedReceiver<Event>>,
    pub codex_op_tx: Option<UnboundedSender<Op>>,
}

// In update() method, poll codex_event_rx
pub fn update(&mut self) {
    // ... existing update logic
    
    // NEW: Process Codex events
    if let Some(rx) = &mut self.codex_event_rx {
        while let Ok(event) = rx.try_recv() {
            self.handle_codex_event(event);
        }
    }
}
```

### 4. Message Submission
**Codex TUI Pattern:**
```rust
// Submit user message via Op
let op = Op::UserMessage {
    text: message,
    // ... other fields
};
codex_op_tx.send(op);
```

**DX TUI Adaptation:**
```rust
// In dispatcher.rs when user sends message
if let Some(op_tx) = &self.app.state.codex_op_tx {
    let op = Op::UserMessage {
        text: message.clone(),
        // ... configure based on model
    };
    op_tx.send(op).ok();
}
```

---

## 📋 IMPLEMENTATION STEPS

### Step 1: Create Agent Module ✅ DONE
- [x] Created `codex_backend.rs` with initialization
- [x] Create `codex_agent.rs` with spawn function
- [x] Extract spawn_agent() logic from Codex TUI
- [x] Adapt to use DX event channels

### Step 2: Update ChatState ✅ DONE
- [x] Add fields to `ChatState`:
  ```rust
  pub codex_op_tx: Option<UnboundedSender<Op>>,
  pub codex_event_rx: Option<UnboundedReceiver<Event>>,
  pub codex_session_configured: bool,
  pub codex_current_turn_id: Option<String>,
  ```
- [x] Initialize Codex backend in `ChatState::new()`
- [x] Call spawn_codex_agent() and store channels

### Step 3: Event Processing ✅ DONE
- [x] Add `handle_codex_event()` method to `ChatState`
- [x] Process different EventMsg types:
  - `SessionConfigured` - Store session info
  - `AssistantMessage` - Add to message list
  - `ToolUse` - Show tool execution
  - `Error` - Show error message
  - `TurnComplete` - Mark turn as done
  - `ShutdownComplete` - Clean up
- [x] Poll `codex_event_rx` in `update()` method

### Step 4: Message Routing in Dispatcher ✅ DONE
- [x] Detect when using Codex model (not local-infinity)
- [x] Route to Codex via Op submission:
  ```rust
  if model.provider == ModelProvider::Codex {
      // Use Codex
      self.submit_to_codex(message);
  } else {
      // Use local LLM
      self.submit_to_local_llm(message);
  }
  ```

### Step 5: Message Rendering ✅ DONE
- [x] Update MessageList to render Codex responses (basic)
- [x] Handle streaming text updates
- [x] Show tool execution status (visual indicator with icons)
- [x] Display errors appropriately

### Step 6: Cleanup ✅ DONE
- [x] Handle shutdown properly
- [x] Stop Codex thread when closing app (Drop impl)
- [x] Clean up resources

---

## 🎯 KEY DIFFERENCES: Codex TUI vs DX TUI

| Aspect | Codex TUI | DX TUI |
|--------|-----------|--------|
| **Event Bus** | `AppEvent` enum with `AppEventSender` | Our own `Event` enum in dispatcher |
| **Rendering** | `ratatui` widgets | Our custom rendering |
| **State** | `App` struct with `ChatWidget` | `ChatState` struct |
| **Event Loop** | `tokio::select!` in `app.rs` | `update()` method in `state.rs` |
| **Message List** | Complex widget with scrolling | Our simple message list |
| **Op Submission** | Via `AppEvent::CodexOp` | Direct via `codex_op_tx` |

---

## 🚀 MINIMAL VIABLE INTEGRATION ✅ COMPLETE

To get Codex working in DX TUI with minimal code:

1. ✅ **Extract spawn_agent()** from `agent.rs` → Create `codex_agent.rs`
2. ✅ **Add 4 fields to ChatState**: `codex_op_tx`, `codex_event_rx`, `codex_session_configured`, `codex_current_turn_id`
3. ✅ **Initialize in ChatState::new()**: Call `initialize_codex_backend()` and `spawn_codex_agent()`
4. ✅ **Poll events in update()**: Check `codex_event_rx` and process events
5. ✅ **Route messages in dispatcher**: Submit to Codex when using Codex models
6. ✅ **Render responses**: Add Codex messages to message list

## 🎯 NEXT ENHANCEMENTS

Now that basic integration is complete, we can add:

1. **Tool Execution UI** - Show visual indicators when tools are running
2. **File Attachments** - Support sending files with messages
3. **Advanced Features**:
   - Reasoning effort selection
   - Collaboration mode
   - Model-specific settings
4. **Better Error Handling** - More detailed error messages
5. **Conversation Management** - Save/load Codex conversations

---

## 📝 CODE FILES TO CREATE/MODIFY

### New Files:
- [ ] `codex-rs/dx/src/codex_agent.rs` - Agent spawning logic

### Files to Modify:
- [ ] `codex-rs/dx/src/state.rs` - Add Codex fields, event processing
- [ ] `codex-rs/dx/src/dispatcher.rs` - Route messages to Codex
- [ ] `codex-rs/dx/src/dx.rs` - Add module declaration
- [ ] `codex-rs/dx/Cargo.toml` - Ensure dependencies are present

---

## ✨ PROFESSIONAL PATTERNS TO KEEP

From Codex TUI's professional implementation:

1. **Unbounded channels** for Op/Event communication
2. **Tokio spawn** for async agent loop
3. **Arc<ThreadManager>** for shared thread access
4. **Event-driven architecture** with clear separation
5. **Error handling** with tracing::error! for failures
6. **Graceful shutdown** by detecting ShutdownComplete
7. **Client identification** via set_app_server_client_name()

---

## 🎬 NEXT ACTIONS

1. Read and extract `spawn_agent()` from `codex-rs/tui/src/chatwidget/agent.rs`
2. Create `codex-rs/dx/src/codex_agent.rs` with adapted spawn function
3. Update `ChatState` with Codex fields
4. Implement event processing loop
5. Route messages through dispatcher
6. Test with Mistral model

This approach uses the professional Codex TUI connection code while keeping DX's own UI rendering!
