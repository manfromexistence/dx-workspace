# Codex Integration into DX TUI - REVISED PLAN

## Key Decision: NO Codex TUI Wrapping

**We are NOT using Codex TUI's UI components inside DX TUI.**

Why? Codex TUI has fundamental TUI problems. Instead, we'll:
1. Use Codex's **backend** (protocol, core, agent system)
2. Build our own **UI** in DX TUI to display Codex responses
3. Keep DX's superior TUI architecture

## What We're Actually Doing

### Use from Codex (Backend Only)
- ✅ `codex-protocol` - Message protocol
- ✅ `codex-core` - Core functionality, ThreadManager, AuthManager
- ✅ Agent system - Background AI processing
- ✅ Tool execution - File operations, shell commands
- ✅ Streaming responses - Token-by-token updates
- ✅ Multi-provider support - OpenAI, Anthropic, Mistral, etc.

### Build in DX TUI (Our Own UI)
- ✅ Message rendering - Use DX's existing MessageList or build better
- ✅ Input handling - Keep DX's input box
- ✅ Code highlighting - Implement our own with syntect
- ✅ Markdown rendering - Build simple markdown parser
- ✅ Tool execution display - Show in DX style
- ✅ Status display - Use DX's bottom controls
- ✅ Theme integration - Use DX's theme system

## Architecture

```
┌─────────────────────────────────────────┐
│           DX TUI (Our UI)               │
│  ┌───────────────────────────────────┐  │
│  │  Message Display (DX Style)       │  │
│  │  - Markdown rendering             │  │
│  │  - Code blocks with highlighting  │  │
│  │  - Tool execution display         │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  Input Box (DX Style)             │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  Bottom Controls (DX Style)       │  │
│  │  Plan | Model | Codex | Path      │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
                  ↓ ↑
         (Send/Receive Messages)
                  ↓ ↑
┌─────────────────────────────────────────┐
│        Codex Backend (No UI)            │
│  ┌───────────────────────────────────┐  │
│  │  codex-core                       │  │
│  │  - ThreadManager                  │  │
│  │  - AuthManager                    │  │
│  │  - ModelsManager                  │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  codex-protocol                   │  │
│  │  - Op (operations)                │  │
│  │  - ResponseEvent                  │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │  Agent System                     │  │
│  │  - Background processing          │  │
│  │  - Tool execution                 │  │
│  │  - Streaming                      │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

## Implementation Steps

### Step 1: Initialize Codex Backend (No UI)
```rust
// In state.rs
use codex_core::{AuthManager, ThreadManager, Config};
use codex_protocol::protocol::Op;

pub struct ChatState {
    // ... existing DX fields ...
    
    // NEW: Codex backend (no UI components)
    pub codex_thread_manager: Option<Arc<ThreadManager>>,
    pub codex_auth_manager: Option<Arc<AuthManager>>,
    pub codex_op_tx: Option<UnboundedSender<Op>>,
    pub codex_event_rx: Option<UnboundedReceiver<ResponseEvent>>,
}
```

### Step 2: Send Messages to Codex
```rust
// In dispatcher.rs - when user presses Enter
InputAction::Submit(msg) => {
    // Send to Codex backend
    if let Some(op_tx) = &self.app.bridge.chat_state.codex_op_tx {
        let op = Op::UserInput(UserInput {
            text: msg,
            // ... other fields
        });
        let _ = op_tx.send(op);
    }
}
```

### Step 3: Receive Responses from Codex
```rust
// In state.rs update() method
pub fn update(&mut self) {
    // Process Codex events
    if let Some(event_rx) = &mut self.codex_event_rx {
        while let Ok(event) = event_rx.try_recv() {
            match event {
                ResponseEvent::OutputTextDelta(text) => {
                    // Add to current message
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
}
```

### Step 4: Render Messages in DX Style
```rust
// In dx_render.rs - use existing MessageList or build better
pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
    // ... existing DX rendering ...
    
    // Render messages with DX's style
    let message_list = MessageList::new(&self.messages, &self.theme);
    message_list.render(chat_area, buf);
}
```

### Step 5: Add Code Highlighting
```rust
// NEW: code_highlighter.rs
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

pub fn highlight_code(code: &str, language: &str) -> Vec<StyledLine> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    
    let syntax = ss.find_syntax_by_token(language)
        .unwrap_or_else(|| ss.find_syntax_plain_text());
    
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    
    // Highlight and return styled lines
    // ...
}
```

### Step 6: Add Markdown Rendering
```rust
// NEW: markdown_renderer.rs
pub fn render_markdown(text: &str) -> Vec<RenderedLine> {
    // Simple markdown parser
    // - Headers: # ## ###
    // - Bold: **text**
    // - Italic: *text*
    // - Code: `code`
    // - Code blocks: ```language\ncode\n```
    // - Lists: - item
    
    // Return styled lines for rendering
}
```

## What We DON'T Use from Codex TUI

❌ `codex-tui-dx` crate - Has TUI problems
❌ `ChatWidget` - We build our own message display
❌ `InputComposer` - We use DX's input box
❌ `BottomPane` - We use DX's bottom controls
❌ Any Codex TUI rendering code

## What We DO Use from Codex

✅ `codex-core` - Backend logic
✅ `codex-protocol` - Message protocol
✅ `codex-api` - API clients
✅ Agent spawning - Background processing
✅ Tool execution - File operations
✅ Authentication - Login/logout
✅ Model management - Provider selection

## File Structure

```
codex-rs/dx/src/
├── dx.rs                    # Main entry
├── state.rs                 # App state + Codex backend
├── dx_render.rs             # DX rendering (our UI)
├── dispatcher.rs            # Input → Codex backend
├── codex_backend.rs         # NEW: Codex initialization
├── codex_handler.rs         # NEW: Event processing
├── markdown_renderer.rs     # NEW: Markdown parsing
├── code_highlighter.rs      # NEW: Syntax highlighting
├── message_display.rs       # NEW: Better message rendering
└── ...
```

## Dependencies to Add

```toml
[dependencies]
# Codex backend (no UI)
codex-core = { workspace = true }
codex-protocol = { workspace = true }
codex-api = { workspace = true }

# For code highlighting
syntect = "5.0"

# For markdown parsing (simple)
pulldown-cmark = "0.9"  # Optional, or build our own
```

## Message Flow

```
User types message
    ↓
DX Input Box captures it
    ↓
dispatcher.rs sends Op::UserInput to Codex
    ↓
Codex Agent processes in background
    ↓
Codex sends ResponseEvent::OutputTextDelta
    ↓
state.rs update() receives events
    ↓
Appends to messages Vec
    ↓
dx_render.rs renders with DX style
    ↓
User sees response in DX UI
```

## Advantages of This Approach

1. ✅ **No TUI conflicts** - We control all rendering
2. ✅ **DX's superior UX** - Keep animations, themes, sounds
3. ✅ **Codex's AI power** - Full backend functionality
4. ✅ **Clean separation** - UI vs Backend
5. ✅ **Easy to maintain** - No complex wrapping
6. ✅ **Better performance** - No double rendering
7. ✅ **Full control** - Customize everything

## Comparison

### ❌ Old Plan (Wrapping Codex TUI)
```rust
// BAD: Trying to render Codex TUI inside DX
codex_widget.chat_widget.render(area, buf);
// Problems: TUI conflicts, double rendering, loss of control
```

### ✅ New Plan (Codex Backend Only)
```rust
// GOOD: Use Codex backend, render with DX UI
let op = Op::UserInput(user_message);
codex_op_tx.send(op);

// Later, in update():
match codex_event_rx.recv() {
    ResponseEvent::OutputTextDelta(text) => {
        // Add to our messages and render with DX style
        self.messages.last_mut().content.push_str(&text);
    }
}
```

## Timeline

- **Step 1-2**: Initialize Codex backend (2 hours)
- **Step 3**: Message sending/receiving (2 hours)
- **Step 4**: Basic message rendering (1 hour)
- **Step 5**: Code highlighting (2 hours)
- **Step 6**: Markdown rendering (2 hours)
- **Testing**: (2 hours)

**Total**: ~11 hours

## Success Criteria

1. ✅ Can send messages to Codex from DX input
2. ✅ Responses stream back in real-time
3. ✅ Code blocks render with syntax highlighting
4. ✅ Markdown renders correctly (bold, italic, lists)
5. ✅ Tool execution shows in DX style
6. ✅ All DX features work (animations, themes, sounds)
7. ✅ No TUI conflicts or rendering issues
8. ✅ Performance is smooth

## Next Steps

1. Remove old Codex TUI integration code (commented out sections)
2. Create `codex_backend.rs` for initialization
3. Create `codex_handler.rs` for event processing
4. Update `state.rs` to hold Codex backend references
5. Update `dispatcher.rs` to send to Codex
6. Build message rendering in DX style
7. Add code highlighting
8. Add markdown rendering
9. Test and polish

## Conclusion

We're building a **hybrid system**:
- **Backend**: Codex's powerful AI engine
- **Frontend**: DX's superior TUI

This gives us the best of both worlds without the problems of wrapping Codex TUI inside DX TUI.
