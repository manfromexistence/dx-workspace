# Codex Integration Status for DX TUI

## ✅ COMPLETED

### 1. Codex Backend Initialization (`codex_backend.rs`)
- ✅ Created initialization function for Codex backend
- ✅ Configured Mistral as default provider
- ✅ Set up ThreadManager with proper config
- ✅ Created AuthManager for authentication
- ✅ Configured session telemetry
- ✅ Set up Op channels for communication

### 2. Codex Agent Spawning (`codex_agent.rs`)
- ✅ Extracted spawn_agent() logic from Codex TUI
- ✅ Adapted for DX TUI event architecture
- ✅ Created Op forwarding loop
- ✅ Created event listening loop
- ✅ Set up client identification (dx-tui)
- ✅ Handles SessionConfigured event
- ✅ Handles ShutdownComplete for cleanup

### 3. Model Configuration (`models.rs`)
- ✅ Set Mistral Large as default model
- ✅ Configured ModelProvider::Codex for Mistral
- ✅ Added multiple Codex models (GPT-5.4, Claude, etc.)
- ✅ Kept local-infinity for unlimited local model

### 4. Module Structure
- ✅ Added codex_agent module to dx.rs
- ✅ Organized Codex-related modules

### 5. Documentation
- ✅ Created comprehensive extraction checklist
- ✅ Documented what to extract vs ignore
- ✅ Identified key differences between Codex TUI and DX TUI
- ✅ Created implementation roadmap

---

## 🚧 IN PROGRESS / TODO

### 1. ChatState Integration
**Status**: Not started
**What to do**:
```rust
// Add to ChatState struct in state.rs
pub struct ChatState {
    // ... existing fields ...
    
    // NEW: Codex integration fields
    pub codex_thread_manager: Option<Arc<ThreadManager>>,
    pub codex_op_tx: Option<UnboundedSender<Op>>,
    pub codex_event_rx: Option<UnboundedReceiver<Event>>,
    pub codex_thread: Option<Arc<CodexThread>>,
    pub codex_session_configured: bool,
}
```

**Initialize in ChatState::new()**:
```rust
// Initialize Codex backend
let codex_backend = tokio::task::block_in_place(|| {
    tokio::runtime::Handle::current().block_on(async {
        crate::codex_backend::initialize_codex_backend().await
    })
});

let (codex_op_tx, codex_event_rx, codex_thread) = match codex_backend {
    Ok(backend) => {
        let (op_tx, event_rx, thread) = crate::codex_agent::spawn_codex_agent(
            backend.config,
            backend.thread_manager.clone(),
        );
        (Some(op_tx), Some(event_rx), Some(thread))
    }
    Err(e) => {
        tracing::error!("Failed to initialize Codex: {}", e);
        (None, None, None)
    }
};

Self {
    // ... existing fields ...
    codex_thread_manager: codex_backend.ok().map(|b| b.thread_manager),
    codex_op_tx,
    codex_event_rx,
    codex_thread,
    codex_session_configured: false,
}
```

### 2. Event Processing Loop
**Status**: Not started
**What to do**:

Add to `state.rs`:
```rust
impl ChatState {
    /// Process Codex events from the event channel
    pub fn handle_codex_event(&mut self, event: Event) {
        use codex_protocol::protocol::EventMsg;
        
        match event.msg {
            EventMsg::SessionConfigured(config) => {
                self.codex_session_configured = true;
                tracing::info!("Codex session configured: {:?}", config.model);
            }
            
            EventMsg::AssistantMessage(msg) => {
                // Add assistant message to message list
                self.messages.push(Message {
                    role: MessageRole::Assistant,
                    content: msg.text,
                    timestamp: chrono::Local::now(),
                });
            }
            
            EventMsg::ToolUse(tool) => {
                // Show tool execution
                tracing::info!("Tool use: {}", tool.name);
            }
            
            EventMsg::Error(err) => {
                // Show error message
                self.show_toast(format!("Error: {}", err.message));
            }
            
            EventMsg::TurnComplete => {
                // Mark turn as complete
                self.is_loading = false;
            }
            
            EventMsg::ShutdownComplete => {
                // Clean up
                tracing::info!("Codex shutdown complete");
            }
            
            _ => {
                // Log other events
                tracing::debug!("Codex event: {:?}", event.msg);
            }
        }
    }
    
    /// Poll Codex events in update loop
    pub fn update(&mut self) {
        // ... existing update logic ...
        
        // NEW: Process Codex events
        if let Some(rx) = &mut self.codex_event_rx {
            while let Ok(event) = rx.try_recv() {
                self.handle_codex_event(event);
            }
        }
    }
}
```

### 3. Message Routing in Dispatcher
**Status**: Not started
**What to do**:

Update `dispatcher.rs` to route messages based on model provider:
```rust
// In dispatch_key() when Enter is pressed
if key.code == KeyCode::Enter && !key.modifiers.contains(KeyModifiers::SHIFT) {
    let message = self.app.state.input.text.clone();
    
    // Check which provider to use
    if self.app.state.current_model.provider == ModelProvider::Codex {
        // Use Codex backend
        self.submit_to_codex(message);
    } else {
        // Use local LLM
        self.submit_to_local_llm(message);
    }
}

fn submit_to_codex(&mut self, message: String) {
    use codex_protocol::protocol::Op;
    
    if let Some(op_tx) = &self.app.state.codex_op_tx {
        // Add user message to UI
        self.app.state.add_user_message(message.clone());
        
        // Submit to Codex
        let op = Op::UserMessage {
            text: message,
            // ... configure other fields
        };
        
        if let Err(e) = op_tx.send(op) {
            tracing::error!("Failed to send op to Codex: {}", e);
            self.app.state.show_toast("Failed to send message".to_string());
        } else {
            self.app.state.is_loading = true;
        }
    }
}

fn submit_to_local_llm(&mut self, message: String) {
    // Existing local LLM logic
    self.app.state.add_user_message(message.clone());
    self.app.state.llm_tx.send(message).ok();
    self.app.state.is_loading = true;
}
```

### 4. Message Rendering
**Status**: Not started
**What to do**:

The existing message rendering in DX should work, but we need to ensure:
- Streaming text updates are handled
- Tool execution is shown
- Errors are displayed properly

### 5. Cleanup and Shutdown
**Status**: Not started
**What to do**:

Add shutdown handling:
```rust
impl Drop for ChatState {
    fn drop(&mut self) {
        // Send shutdown op to Codex
        if let Some(op_tx) = &self.codex_op_tx {
            use codex_protocol::protocol::Op;
            op_tx.send(Op::Shutdown).ok();
        }
    }
}
```

---

## 🔧 COMPILATION FIXES NEEDED

### 1. Fix codex_agent.rs
The current implementation has a placeholder:
```rust
(codex_op_tx, event_rx, Arc::new(CodexThread::default()))
```

Should return the actual thread:
```rust
(codex_op_tx, event_rx, thread)
```

### 2. Fix codex_backend.rs
Need to return the Config along with other fields:
```rust
pub struct CodexBackend {
    pub thread_manager: Arc<ThreadManager>,
    pub auth_manager: Arc<AuthManager>,
    pub config: Config,  // ADD THIS
    pub op_tx: UnboundedSender<Op>,
    pub op_rx: UnboundedReceiver<Op>,
}
```

### 3. Add Dependencies
Ensure `Cargo.toml` has all required dependencies:
```toml
codex-core = { path = "../core" }
codex-protocol = { path = "../protocol" }
codex-cloud-requirements = { path = "../cloud-requirements" }
codex-otel = { path = "../otel" }
codex-utils-absolute-path = { path = "../utils/absolute-path" }
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
```

---

## 📊 ARCHITECTURE OVERVIEW

```
┌─────────────────────────────────────────────────────────────┐
│                         DX TUI                              │
│                                                             │
│  ┌──────────────┐         ┌──────────────┐                │
│  │  ChatState   │         │  Dispatcher  │                │
│  │              │         │              │                │
│  │ - messages   │◄────────┤ - key events │                │
│  │ - input      │         │ - routing    │                │
│  │ - codex_*    │         └──────────────┘                │
│  └──────┬───────┘                                          │
│         │                                                   │
│         │ codex_op_tx                                      │
│         │ codex_event_rx                                   │
│         │                                                   │
│  ┌──────▼───────────────────────────────────────────────┐ │
│  │           Codex Agent (codex_agent.rs)              │ │
│  │                                                      │ │
│  │  ┌────────────┐         ┌────────────┐             │ │
│  │  │ Op Forward │         │ Event Loop │             │ │
│  │  │   Loop     │         │            │             │ │
│  │  └─────┬──────┘         └──────▲─────┘             │ │
│  │        │                       │                    │ │
│  └────────┼───────────────────────┼────────────────────┘ │
│           │                       │                      │
└───────────┼───────────────────────┼──────────────────────┘
            │                       │
            │ Op::UserMessage       │ Event::AssistantMessage
            │                       │
┌───────────▼───────────────────────┴──────────────────────┐
│                  Codex Core                               │
│                                                           │
│  ┌──────────────┐    ┌──────────────┐                   │
│  │ThreadManager │    │ CodexThread  │                   │
│  └──────────────┘    └──────────────┘                   │
│                                                           │
│  ┌──────────────────────────────────┐                   │
│  │      Model Providers              │                   │
│  │  - Mistral (default)              │                   │
│  │  - GPT-5.4                        │                   │
│  │  - Claude 3.5                     │                   │
│  └──────────────────────────────────┘                   │
└───────────────────────────────────────────────────────────┘
```

---

## 🎯 NEXT STEPS (Priority Order)

1. **Fix compilation issues** in codex_agent.rs and codex_backend.rs
2. **Add Codex fields to ChatState** struct
3. **Initialize Codex in ChatState::new()**
4. **Add event processing** in ChatState::update()
5. **Route messages** in dispatcher based on model provider
6. **Test with Mistral** model
7. **Handle streaming responses** properly
8. **Add shutdown handling**

---

## 🧪 TESTING PLAN

### Phase 1: Basic Connection
- [ ] Codex backend initializes without errors
- [ ] Agent spawns successfully
- [ ] SessionConfigured event is received
- [ ] Can send a simple message

### Phase 2: Message Flow
- [ ] User message appears in UI
- [ ] Message is sent to Codex
- [ ] Assistant response is received
- [ ] Response appears in message list

### Phase 3: Advanced Features
- [ ] Tool execution is shown
- [ ] Errors are displayed
- [ ] Streaming works properly
- [ ] Can switch between models
- [ ] Shutdown is clean

---

## 📝 KEY INSIGHTS

### What We're Using from Codex TUI:
1. **spawn_agent()** - Professional connection logic
2. **Event/Op protocol** - Clean async communication
3. **ThreadManager** - Thread lifecycle management
4. **Event loop pattern** - Robust event processing

### What We're NOT Using:
1. **ratatui widgets** - We have our own rendering
2. **ChatWidget** - Too TUI-specific
3. **UI state management** - We have ChatState
4. **Keyboard/mouse handling** - We have dispatcher

### Why This Approach Works:
- Uses professional, battle-tested connection code
- Keeps DX's clean, simple UI
- Avoids Codex TUI's rendering problems
- Maintains separation of concerns
- Easy to debug and extend

---

## 🚀 ESTIMATED COMPLETION

- **Phase 1** (Basic Connection): 1-2 hours
- **Phase 2** (Message Flow): 2-3 hours
- **Phase 3** (Advanced Features): 3-4 hours

**Total**: 6-9 hours of focused work

---

## 💡 TIPS FOR IMPLEMENTATION

1. **Start small**: Get basic message sending working first
2. **Use tracing**: Log everything during development
3. **Test incrementally**: Don't try to implement everything at once
4. **Handle errors gracefully**: Codex might fail to initialize
5. **Keep UI responsive**: Don't block on Codex operations
6. **Follow the checklist**: Use CODEX_CONNECTION_EXTRACTION_CHECKLIST.md

---

## 📚 REFERENCE FILES

- `CODEX_CONNECTION_EXTRACTION_CHECKLIST.md` - Detailed extraction guide
- `codex-rs/tui/src/chatwidget/agent.rs` - Original spawn_agent()
- `codex-rs/tui/src/app_event.rs` - Event routing pattern
- `codex-rs/dx/src/codex_agent.rs` - Our adapted agent
- `codex-rs/dx/src/codex_backend.rs` - Backend initialization
- `codex-rs/dx/src/models.rs` - Model configuration

---

This integration uses the professional Codex TUI connection code while keeping DX's own UI!
