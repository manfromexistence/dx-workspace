# Codex TUI vs DX TUI - What We Need to Implement

## Overview

The real Codex TUI (`codex-rs/dx/src/codex.rs` and `codex_lib.rs`) is a full-featured TUI application. Here's what it has that we need to implement in DX TUI.

## What Codex TUI Has (That We Need)

### 1. ✅ Backend Initialization (DONE)
- ThreadManager
- AuthManager  
- Config loading
- Model provider setup
- **Status**: We created `codex_backend.rs` with this

### 2. ⏳ Message Protocol Integration (TODO)
**What Codex TUI does:**
```rust
// Sends Op to agent
let op = Op::UserInput(UserInput { text, .. });
op_tx.send(op);

// Receives ResponseEvent from agent
match event {
    ResponseEvent::OutputTextDelta(text) => { /* append to message */ }
    ResponseEvent::Completed { .. } => { /* mark done */ }
    ResponseEvent::ToolExecution { .. } => { /* show tool */ }
}
```

**What we need in DX:**
- Send user input as `Op::UserInput`
- Receive and process `ResponseEvent` stream
- Update messages in real-time

### 3. ⏳ Agent Spawning (TODO)
**What Codex TUI does:**
```rust
// In chatwidget/agent.rs
pub fn spawn_agent(
    config: Config,
    app_event_sender: AppEventSender,
    thread_manager: Arc<ThreadManager>,
) -> UnboundedSender<Op> {
    let (op_tx, op_rx) = mpsc::unbounded_channel();
    
    tokio::spawn(async move {
        // Process Ops and send ResponseEvents
        while let Some(op) = op_rx.recv().await {
            // Handle Op, call Codex core, stream responses
        }
    });
    
    op_tx
}
```

**What we need in DX:**
- Spawn agent task on startup
- Create Op channel for sending messages
- Create ResponseEvent channel for receiving responses

### 4. ⏳ Response Streaming (TODO)
**What Codex TUI does:**
```rust
// Streams tokens as they arrive
ResponseEvent::OutputTextDelta(text) => {
    current_message.content.push_str(&text);
    // Trigger re-render
}
```

**What we need in DX:**
- Process `OutputTextDelta` events
- Append to current message
- Trigger UI update

### 5. ⏳ Tool Execution Display (TODO)
**What Codex TUI does:**
```rust
ResponseEvent::ToolExecution { tool_name, args, result } => {
    // Show tool execution in UI
    // Display: "Running: read_file(path='main.rs')"
    // Display: "Result: [file content]"
}
```

**What we need in DX:**
- Detect tool execution events
- Show tool name and args
- Show tool results
- Style appropriately

### 6. ⏳ Thinking/Reasoning Display (TODO)
**What Codex TUI does:**
```rust
// Detects <think>...</think> tags
// Renders in collapsed accordion
// User can expand to see reasoning
```

**What we need in DX:**
- Parse `<think>` tags from content
- Render collapsed by default
- Allow expanding/collapsing

### 7. ⏳ Code Block Rendering (TODO)
**What Codex TUI does:**
```rust
// Uses syntect for syntax highlighting
// Detects ```language\ncode\n``` blocks
// Applies theme-aware colors
```

**What we need in DX:**
- Parse code blocks from markdown
- Apply syntax highlighting with syntect
- Use DX theme colors

### 8. ⏳ Markdown Rendering (TODO)
**What Codex TUI does:**
```rust
// Renders:
// - Headers (# ## ###)
// - Bold (**text**)
// - Italic (*text*)
// - Lists (- item)
// - Links ([text](url))
// - Inline code (`code`)
```

**What we need in DX:**
- Simple markdown parser
- Render styled text
- Handle all common markdown elements

### 9. ⏳ Session Management (TODO)
**What Codex TUI does:**
```rust
// Saves conversations to ~/.codex/threads/
// Can resume previous sessions
// Can fork sessions
// Tracks thread_id
```

**What we need in DX:**
- Save messages to Codex thread format
- Load previous conversations
- Track thread_id

### 10. ⏳ Authentication (TODO)
**What Codex TUI does:**
```rust
// Handles login/logout
// Stores credentials
// Refreshes tokens
// Shows auth status
```

**What we need in DX:**
- Check auth status on startup
- Show login prompt if needed
- Handle auth errors gracefully

### 11. ❌ Onboarding (NOT NEEDED)
**What Codex TUI does:**
- First-run experience
- Trust screen
- Login screen

**What we do:**
- Skip onboarding (DX is simpler)
- Assume user has API keys set

### 12. ❌ Resume/Fork Picker (NOT NEEDED)
**What Codex TUI does:**
- UI to pick previous sessions
- Resume or fork conversations

**What we do:**
- Start fresh each time (simpler)
- Can add later if needed

### 13. ❌ Update Prompts (NOT NEEDED)
**What Codex TUI does:**
- Checks for updates
- Prompts user to update

**What we do:**
- Skip update checks (DX is standalone)

## Implementation Priority

### Phase 1: Core Functionality (Must Have)
1. ✅ Backend initialization (`codex_backend.rs`)
2. ⏳ Agent spawning
3. ⏳ Op sending (user input → Codex)
4. ⏳ ResponseEvent receiving (Codex → UI)
5. ⏳ Basic message rendering

### Phase 2: Enhanced Display (Should Have)
6. ⏳ Code block syntax highlighting
7. ⏳ Markdown rendering
8. ⏳ Tool execution display
9. ⏳ Thinking/reasoning display

### Phase 3: Advanced Features (Nice to Have)
10. ⏳ Session management
11. ⏳ Authentication handling
12. ❌ Resume/fork (skip for now)
13. ❌ Onboarding (skip)

## Key Files in Codex TUI

### Core Agent Logic
- `src/chatwidget/agent.rs` - Agent spawning and Op processing
- `src/app.rs` - Main app loop and event handling
- `src/chatwidget/mod.rs` - ChatWidget component

### Rendering
- `src/render/mod.rs` - Rendering system
- `src/markdown_render.rs` - Markdown parsing
- `src/render/highlight.rs` - Syntax highlighting
- `src/exec_cell.rs` - Tool execution display

### Protocol
- Uses `codex-protocol` crate for Op and ResponseEvent
- Uses `codex-core` for ThreadManager, AuthManager
- Uses `codex-api` for API clients

## What We're Building in DX

```rust
// DX Architecture
┌─────────────────────────────────────┐
│  DX TUI (Our UI)                    │
│  - state.rs (holds Codex backend)   │
│  - dispatcher.rs (sends Ops)        │
│  - dx_render.rs (renders messages)  │
│  - components.rs (MessageList)      │
└─────────────────────────────────────┘
              ↓ ↑
         (Op / ResponseEvent)
              ↓ ↑
┌─────────────────────────────────────┐
│  Codex Backend (No UI)              │
│  - codex_backend.rs (init)          │
│  - Agent task (spawned)             │
│  - ThreadManager                    │
│  - AuthManager                      │
└─────────────────────────────────────┘
              ↓ ↑
         (HTTP/WebSocket)
              ↓ ↑
┌─────────────────────────────────────┐
│  Mistral API                        │
│  (or any other provider)            │
└─────────────────────────────────────┘
```

## Next Steps

### Step 1: Agent Spawning
Create `codex-rs/dx/src/codex_agent.rs`:
```rust
pub fn spawn_codex_agent(
    config: Config,
    thread_manager: Arc<ThreadManager>,
) -> (UnboundedSender<Op>, UnboundedReceiver<ResponseEvent>) {
    let (op_tx, op_rx) = mpsc::unbounded_channel();
    let (event_tx, event_rx) = mpsc::unbounded_channel();
    
    tokio::spawn(async move {
        // Process Ops and send ResponseEvents
    });
    
    (op_tx, event_rx)
}
```

### Step 2: Update ChatState
```rust
pub struct ChatState {
    // ... existing fields ...
    
    // Codex backend
    pub codex_thread_manager: Option<Arc<ThreadManager>>,
    pub codex_op_tx: Option<UnboundedSender<Op>>,
    pub codex_event_rx: Option<UnboundedReceiver<ResponseEvent>>,
}
```

### Step 3: Send Messages
```rust
// In dispatcher.rs
InputAction::Submit(msg) => {
    if current_model.provider == ModelProvider::Codex {
        let op = Op::UserInput(UserInput {
            text: msg,
            images: vec![],
        });
        self.app.bridge.chat_state.codex_op_tx.send(op);
    }
}
```

### Step 4: Receive Responses
```rust
// In state.rs update()
if let Some(event_rx) = &mut self.codex_event_rx {
    while let Ok(event) = event_rx.try_recv() {
        match event {
            ResponseEvent::OutputTextDelta(text) => {
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

### Step 5: Render Messages
The existing `MessageList` should work, but we may need to add:
- Code block highlighting
- Markdown parsing
- Tool execution display

## Summary

**What Codex TUI has that we need:**
1. ✅ Backend initialization (done)
2. ⏳ Agent spawning (next)
3. ⏳ Op/ResponseEvent protocol (next)
4. ⏳ Streaming responses (next)
5. ⏳ Code highlighting (later)
6. ⏳ Markdown rendering (later)
7. ⏳ Tool execution display (later)

**What we're skipping:**
- Onboarding screens
- Resume/fork pickers
- Update prompts
- Complex session management (for now)

**Our advantage:**
- Simpler architecture
- DX's superior UI
- Keep animations, themes, sounds
- No TUI conflicts

The key is to implement the agent spawning and Op/ResponseEvent protocol next, then we'll have a working Codex integration!
