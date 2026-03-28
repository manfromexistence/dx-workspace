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

## 🎯 REMAINING FEATURES TO IMPLEMENT

DX TUI already has file attachments and file browser. Only these 4 features from Codex TUI need integration:

### 1. External Editor Integration ❌ TODO
**What it does:** Opens external editor (vim, nano, etc.) for composing messages
**Codex TUI implementation:**
- File: `codex-rs/tui/src/external_editor.rs`
- `resolve_editor_command()` - Reads VISUAL or EDITOR env var
- `run_editor(seed, editor_cmd)` - Writes to temp file, spawns editor, reads result
- Uses `tempfile` crate for temp file creation
- Handles Windows `.cmd`/`.bat` shims via `which` crate
- Parses command with `shlex` (Unix) or `winsplit` (Windows)
**DX TUI integration:**
- Create `codex-rs/dx/src/external_editor.rs` - Copy logic from Codex TUI
- Add keyboard shortcut (e.g., Ctrl+E) to trigger editor
- When editor closes, insert returned text into chat input
- Handle terminal suspend/resume for TUI editors (vim, nano)

### 2. Approval Popup UI ❌ TODO
**What it does:** Shows approval dialog for tool execution (security feature)
**Codex TUI implementation:**
- Uses `AskForApproval` enum: `Never`, `OnRequest`, `UnlessTrusted`
- Stored in `config.permissions.approval_policy`
- When tool needs approval, shows popup with tool details
- User can approve/deny individual tool execution
**DX TUI integration:**
- Menu item "20. Approval Policy" already exists with submenu structure
- Add approval policy state to `ChatState`
- When Codex sends tool execution request, check policy
- If `OnRequest`, show DX menu-style approval dialog
- Send approval/denial back to Codex via Op channel

### 3. Skills List UI ❌ TODO
**What it does:** Lists and manages available skills (reusable agent capabilities)
**Codex TUI implementation:**
- File: `codex-rs/tui/src/chatwidget/skills.rs`
- `open_skills_menu()` - Shows skills action menu
- `open_manage_skills_popup()` - Shows enable/disable toggles
- `set_skills_from_response()` - Receives skills from backend
- `update_skill_enabled()` - Toggles skill on/off
- Skills fetched via `ListSkillsResponseEvent` from Codex backend
**DX TUI integration:**
- Menu item "5. Skills" already exists with submenu: "Per-Skill Toggle", "Skill Path", "Scan Directories"
- Add skills state to `ChatState`: `skills_all: Vec<ProtocolSkillMetadata>`
- Handle `ListSkillsResponseEvent` in `handle_codex_event()`
- Implement submenu handlers:
  - "Per-Skill Toggle" → Show list with checkboxes
  - "Skill Path" → Show skill file paths
  - "Scan Directories" → Trigger skill rescan
- Use DX menu system to render skills list

### 4. Plugin Marketplace UI ❌ TODO
**What it does:** Browse and install MCP plugins/apps
**Codex TUI implementation:**
- File: `codex-rs/tui/src/chatwidget/plugins.rs`
- `add_plugins_output()` - Opens plugin marketplace
- `on_plugins_loaded()` - Receives plugin list from backend
- `on_plugin_detail_loaded()` - Shows plugin details
- `on_plugin_install_loaded()` - Handles installation
- Fetches via `AppEvent::FetchPluginsList`, `FetchPluginDetail`, `FetchPluginInstall`
- Uses `PluginListResponse`, `PluginReadResponse`, `PluginInstallResponse`
**DX TUI integration:**
- Menu item "4. Plugins & Apps" already exists with submenu: "Plugin Management", "Marketplace Discovery", "Connector Apps", "Suggestion Allowlist"
- Add plugin state to `ChatState`: `plugins_cache: PluginsCacheState`
- Send plugin fetch requests to app-server via existing client
- Handle plugin responses and show in DX menu
- Implement submenu handlers:
  - "Marketplace Discovery" → Browse available plugins
  - "Plugin Management" → Install/uninstall plugins
  - "Connector Apps" → Manage plugin apps
- Use DX menu system to render plugin list and details

---

## 📊 COMPLETE FEATURE COMPARISON: CODEX TUI vs DX TUI

### ✅ CORE FEATURES ALREADY IN DX (100% Complete)
1. ✅ Chat interface with message history
2. ✅ File browser (Yazi integration)
3. ✅ File attachments (drag & drop)
4. ✅ Model selection UI
5. ✅ Theme system
6. ✅ Keyboard shortcuts
7. ✅ Mouse support (click, scroll, drag)
8. ✅ Clipboard integration
9. ✅ Scrolling (chat + input)
10. ✅ Custom animations
11. ✅ Audio system
12. ✅ Local GGUF model support

### ✅ CODEX BACKEND INTEGRATION (100% Complete)
13. ✅ ThreadManager connection
14. ✅ Event-driven architecture
15. ✅ Op channel for message submission
16. ✅ Streaming responses
17. ✅ Tool execution tracking with visual indicators
18. ✅ Session management
19. ✅ Graceful shutdown
20. ✅ Multi-provider support (Mistral default)
21. ✅ Error handling and toasts

### ❌ MISSING CODEX TUI FEATURES (4 features)
22. ❌ External editor integration (Ctrl+E to open vim/nano)
23. ❌ Approval policy UI (tool execution permissions)
24. ❌ Skills management UI (enable/disable skills)
25. ❌ Plugin marketplace UI (browse/install plugins)

### 🚫 CODEX TUI FEATURES WE DON'T NEED (Already have better alternatives)
- ❌ Codex TUI's file picker → DX has Yazi file browser ✅
- ❌ Codex TUI's attachment UI → DX has drag & drop ✅
- ❌ Codex TUI's theme picker → DX has theme menu ✅
- ❌ Codex TUI's model picker → DX has model menu ✅
- ❌ Codex TUI's scrolling → DX has custom scrolling ✅
- ❌ Codex TUI's status line → DX has custom status ✅

### 🎯 ADVANCED CODEX FEATURES (Optional - Not Critical)
These exist in Codex TUI but are advanced features we can add later:
- Voice/Realtime API integration
- Session resume/fork
- Conversation history search
- Git diff integration
- Branch detection
- Reasoning effort selection
- Collaboration mode
- Plan tool integration
- Windows sandbox integration
- Web search integration
- MCP server management UI
- Memory/RAG integration
- Multi-agent orchestration
- Network proxy settings
- Project trust levels
- Developer instructions
- Feature flags UI
- Hooks & events UI

---

## 🎯 PROGRESS CALCULATION

### Critical Features for "Codex Fully Integrated"
**Total: 25 features**
- Core DX features: 12 ✅
- Codex backend: 9 ✅
- Missing UI features: 4 ❌

**Current Progress: 84% (21/25 features complete)**

### If We Include Optional Advanced Features
**Total: ~45 features**
- Critical features: 25 (21 done, 4 todo)
- Optional advanced: ~20 (0 done)

**Current Progress: 47% (21/45 features complete)**

---

## 📈 REALISTIC ASSESSMENT

### For "Codex Fully Integrated in DX" - We are at **84%**

The 4 missing features are UI-only integrations:
1. External editor - 2-3 hours work
2. Approval policy - 3-4 hours work
3. Skills management - 4-5 hours work
4. Plugin marketplace - 5-6 hours work

**Estimated time to 100%: 14-18 hours of focused development**

### What "100%" Means
- ✅ All Codex backend functionality working
- ✅ Streaming, tool execution, session management
- ✅ Multi-provider support
- ✅ DX's superior UI (file browser, animations, themes)
- ✅ 4 remaining UI features (editor, approvals, skills, plugins)

### What We're NOT Implementing (By Design)
- Codex TUI's inferior UI widgets (we have better ones)
- Advanced features that aren't core to chat functionality
- Features that require significant backend changes

---

## 🎬 NEXT STEPS (In Priority Order)

### Step 1: External Editor Integration (Easiest - 2-3 hours)
- Copy `external_editor.rs` from Codex TUI
- Add keyboard shortcut (Ctrl+E)
- Handle terminal suspend/resume
- Insert editor result into chat input

### Step 2: Approval Policy UI (Medium - 3-4 hours)
- Add approval state to ChatState
- Intercept tool execution events
- Show DX menu-style approval dialog
- Send approval/denial via Op channel

### Step 3: Skills Management (Medium - 4-5 hours)
- Add skills state to ChatState
- Handle ListSkillsResponseEvent
- Implement submenu handlers (toggle, path, scan)
- Render skills list with DX menu

### Step 4: Plugin Marketplace (Hardest - 5-6 hours)
- Add plugin cache state
- Connect to app-server plugin API
- Handle plugin list/detail/install/uninstall
- Render marketplace with DX menu

---

## 🏆 SUMMARY

**We are 84% complete** for full Codex integration in DX TUI.

The remaining 16% is purely UI work - connecting existing Codex backend logic to DX's menu system. No complex backend integration needed, just wiring up 4 features that already have menu items in place.

**DX TUI is already superior to Codex TUI** in many ways:
- Better file browser (Yazi vs basic picker)
- Better animations and visual effects
- Better theme system
- Better mouse support
- Audio system
- Local GGUF support alongside Codex

Once we add the 4 missing UI features, DX will be a complete replacement for Codex TUI with significant improvements.

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
