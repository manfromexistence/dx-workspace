# DX (codex-tui-dx) Architecture Guide for Pro Rust Developers

**Date:** March 27, 2026  
**Repository:** https://github.com/codex-rs/codex-rs  
**Crate:** `codex-rs/dx/` - Terminal UI for Codex CLI

## Overview

`dx` (codex-tui-dx) is a **hybrid TUI** that combines:
1. **Yazi file manager** - Full async terminal file manager (original dx/yazi codebase)
2. **Codex AI chat interface** - AI-powered coding agent with inline message rendering

The project is merging two TUI systems:
- **Local message list rendering** (current dx implementation)
- **Codex Rust CLI inline TUI** (from codex-tui-app-server)

## Project Structure

```
codex-rs/dx/
├── src/
│   ├── bin/                    # Binary entry points
│   ├── app/                    # Main app orchestration
│   ├── chatwidget/             # Chat UI components
│   ├── bottom_pane/            # Input area, status bar
│   ├── file_browser/           # Yazi file manager (fb-*)
│   ├── menu/                   # Menu system
│   ├── render/                 # Rendering pipeline
│   │   └── renderable.rs       # Core rendering trait
│   ├── streaming/              # Stream processing
│   ├── tui/                    # TUI framework
│   ├── chatwidget.rs           # Main chat widget (8000+ lines!)
│   ├── codex.rs                # Main entry point (codex-tui-dx binary)
│   ├── codex_lib.rs            # Library interface
│   ├── scrollbar.rs            # Custom scrollbar widget
│   ├── history_cell.rs         # Message cells
│   └── ...                     # 100+ other modules
├── Cargo.toml                  # Dependencies & features
├── DX.md                       # Integration notes
└── README.md                   # Project overview
```

## Key Architecture Components

### 1. Entry Point Flow

```rust
// src/codex.rs - Main binary entry point
fn main() -> anyhow::Result<()> {
    // Parse CLI args
    let top_cli = TopCli::parse();
    
    // Decide which TUI to use
    let use_app_server_tui = should_use_app_server_tui(&inner).await?;
    
    if use_app_server_tui {
        // Use codex-tui-app-server (inline rendering)
        codex_tui_app_server::run_main(...)
    } else {
        // Use legacy dx TUI (local rendering)
        run_main(...)
    }
}
```

**Two TUI Modes:**
- `codex-tui-app-server` - Codex CLI inline message rendering
- Legacy `dx` TUI - Local message list with file browser

### 2. Core Widget: ChatWidget

**File:** `src/chatwidget.rs` (8000+ lines)

The main UI component that handles:
- Message history rendering
- Active cell (current streaming message)
- Bottom pane (input, status)
- Event processing (keyboard, protocol events)
- Scrolling (partially implemented)

```rust
pub struct ChatWidget {
    // Protocol communication
    codex_op_tx: UnboundedSender<Op>,
    
    // UI components
    bottom_pane: BottomPane,
    active_cell: Option<Box<dyn HistoryCell>>,
    
    // State
    config: Config,
    session_header: SessionHeader,
    
    // Scrolling (new)
    scroll_offset: std::cell::Cell<usize>,
    content_height: std::cell::Cell<usize>,
    viewport_height: std::cell::Cell<usize>,
    
    // ... 100+ other fields
}
```

**Key Methods:**
- `render()` - Renders the entire chat UI
- `handle_key_event()` - Processes keyboard input
- `handle_event()` - Processes protocol events from Codex core
- `as_renderable()` - Builds the renderable tree

### 3. Rendering System

**File:** `src/render/renderable.rs`

Trait-based rendering pipeline:

```rust
pub trait Renderable {
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn desired_height(&self, width: u16) -> u16;
    fn cursor_pos(&self, area: Rect) -> Option<(u16, u16)>;
}
```

**Renderable Types:**
- `ColumnRenderable` - Vertical stack of widgets
- `FlexRenderable` - Flexible layout (like CSS flexbox)
- `RowRenderable` - Horizontal layout
- `InsetRenderable` - Adds padding/margins
- `ScrollableRenderable` - Applies scroll offset (new, buggy)

**Rendering Flow:**
```
ChatWidget::render()
  └─> as_renderable()
      └─> FlexRenderable
          ├─> Active cell (message history)
          └─> Bottom pane (input area)
```

### 4. Message Cells (History)

**File:** `src/history_cell.rs`

Each message is a `HistoryCell`:

```rust
pub trait HistoryCell: Renderable + Send + Sync {
    fn handle_event(&mut self, event: &Event) -> bool;
    fn transcript_lines(&self) -> Vec<String>;
    // ... other methods
}
```

**Cell Types:**
- `AgentMessageCell` - AI responses
- `PlainHistoryCell` - User messages
- `ExecCell` - Command execution
- `WebSearchCell` - Web search results
- `McpToolCallCell` - MCP tool calls

### 5. Bottom Pane (Input Area)

**Directory:** `src/bottom_pane/`

Handles:
- Text input with autocomplete
- Status line (model, tokens, directory)
- Approval requests
- Selection views (model picker, etc.)
- Collaboration mode indicator

### 6. File Browser Integration

**Directories:** `src/file_browser/fb-*/`

The original Yazi file manager components:
- `fb-core` - Core file operations
- `fb-adapter` - Terminal adapters
- `fb-plugin` - Lua plugin system
- `fb-widgets` - UI widgets
- `fb-vfs` - Virtual filesystem

## How to Change the Codex TUI

### A. Modify Message Rendering

**Goal:** Change how messages are displayed

**Files to edit:**
- `src/chatwidget.rs` - Main rendering logic
- `src/history_cell.rs` - Individual message cells
- `src/markdown_render.rs` - Markdown formatting

**Example: Change message styling**
```rust
// In src/history_cell.rs
impl Renderable for AgentMessageCell {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Modify rendering here
        let style = Style::default().fg(Color::Cyan); // Change color
        // ... render with new style
    }
}
```

### B. Add New UI Components

**Goal:** Add a new widget to the UI

**Steps:**
1. Create widget struct implementing `Renderable`
2. Add field to `ChatWidget`
3. Update `as_renderable()` to include new widget
4. Handle events in `handle_key_event()`

**Example: Add a sidebar**
```rust
// 1. Create widget
pub struct Sidebar {
    items: Vec<String>,
}

impl Renderable for Sidebar {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Render sidebar
    }
    fn desired_height(&self, width: u16) -> u16 { area.height }
}

// 2. Add to ChatWidget
pub struct ChatWidget {
    sidebar: Sidebar,
    // ... other fields
}

// 3. Update as_renderable()
fn as_renderable(&self) -> RenderableItem<'_> {
    let mut row = RowRenderable::new();
    row.push(20, &self.sidebar);  // 20 columns wide
    row.push(area.width - 20, main_content);
    RenderableItem::Owned(Box::new(row))
}
```

### C. Modify Keyboard Shortcuts

**File:** `src/chatwidget.rs` - `handle_key_event()` method

**Example: Add Ctrl+K shortcut**
```rust
fn handle_key_event(&mut self, key_event: KeyEvent) {
    match key_event {
        KeyEvent { 
            code: KeyCode::Char('k'), 
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            ..
        } => {
            // Your custom action
            self.do_something();
            return;
        }
        // ... other keys
    }
}
```

### D. Integrate Codex CLI Inline Rendering

**Goal:** Merge codex-tui-app-server rendering into dx

**Current State:**
- Two separate TUI implementations
- `codex.rs` switches between them based on `should_use_app_server_tui()`
- Goal: Merge into single unified TUI

**Files involved:**
- `src/codex.rs` - Entry point with TUI selection
- `src/app_server_tui_dispatch.rs` - App server integration
- `codex-tui-app-server` crate - Inline rendering implementation

**Integration Strategy:**

1. **Study codex-tui-app-server:**
```bash
# Find the app-server TUI implementation
cd codex-rs
find . -name "codex-tui-app-server" -type d
```

2. **Extract message rendering:**
   - Locate message list rendering in app-server
   - Remove branding/menus
   - Keep core message display logic

3. **Create unified renderer:**
```rust
// In src/chatwidget.rs
enum RenderMode {
    Local,      // Current dx rendering
    AppServer,  // Codex CLI inline rendering
}

impl ChatWidget {
    fn render_messages(&self, mode: RenderMode) {
        match mode {
            RenderMode::Local => self.render_local_messages(),
            RenderMode::AppServer => self.render_app_server_messages(),
        }
    }
}
```

4. **Remove app-server UI chrome:**
   - Strip out menus
   - Remove branding
   - Keep only message list

### E. Fix Scrollbar Implementation

**Current Issue:** Scrollbar renders but content doesn't scroll

**Files:**
- `src/scrollbar.rs` - Scrollbar widget (working)
- `src/chatwidget.rs` - Integration (broken)
- `src/render/renderable.rs` - ScrollableRenderable (buggy)

**Problem:** Compilation errors in struct initialization

**Fix locations:**
```rust
// Around line 3523, 3715, 3901 in chatwidget.rs
// WRONG:
last_rendered_user_message_event: None,
}  // <-- Extra closing brace
    scroll_offset: std::cell::Cell::new(0),

// CORRECT:
last_rendered_user_message_event: None,
scroll_offset: std::cell::Cell::new(0),
```

**Test scrolling:**
```bash
cd codex-rs/dx
cargo run
# Press PageUp/PageDown to test
```

### F. Modify Configuration

**File:** `Cargo.toml`

**Features:**
- `llm` - Local LLM support
- `voice-input` - Voice transcription
- `vt100-tests` - Terminal emulator tests
- `debug-logs` - Verbose logging

**Enable feature:**
```bash
cargo run --features debug-logs
```

### G. Add New Protocol Events

**Goal:** Handle new events from Codex core

**File:** `src/chatwidget.rs` - `handle_event()` method

```rust
pub fn handle_event(&mut self, event: &Event) {
    match event {
        Event::YourNewEvent(data) => {
            // Handle new event
            self.process_new_event(data);
        }
        // ... other events
    }
}
```

## Development Workflow

### Build & Run
```bash
cd codex-rs/dx

# Run (low-end device - use this only!)
cargo run

# Run with features
cargo run --features debug-logs

# Run specific binary
cargo run --bin codex-tui-dx
```

### Testing
```bash
# Test specific package
cargo test -p dx

# Test with features
cargo test --features vt100-tests

# Run snapshot tests
cargo insta test
cargo insta review  # Review changes
cargo insta accept  # Accept all
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Fix lints
cargo clippy --fix --allow-dirty
```

## Key Dependencies

### UI Framework
- `ratatui` - Terminal UI framework (like React for terminals)
- `crossterm` - Cross-platform terminal manipulation
- `tachyonfx` - Terminal effects/animations

### Codex Integration
- `codex-core` - Core Codex logic
- `codex-protocol` - Protocol definitions
- `codex-tui-app-server` - App server TUI (inline rendering)

### Async Runtime
- `tokio` - Async runtime
- `async-channel` - Async channels
- `flume` - Fast MPSC channels

### Rendering
- `syntect` - Syntax highlighting
- `pulldown-cmark` - Markdown parsing
- `two-face` - Font rendering

## Common Patterns

### 1. Adding State to ChatWidget
```rust
// Add field
pub struct ChatWidget {
    my_new_state: MyState,
}

// Initialize in constructors (3 places!)
// - new_with_op_sender()
// - new_from_existing()
// - new() (if exists)
```

### 2. Rendering Pattern
```rust
impl Renderable for MyWidget {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        // 1. Calculate layout
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);
        
        // 2. Render children
        child1.render(layout[0], buf);
        child2.render(layout[1], buf);
    }
    
    fn desired_height(&self, width: u16) -> u16 {
        // Return total height needed
        child1.desired_height(width) + child2.desired_height(width)
    }
}
```

### 3. Event Handling Pattern
```rust
fn handle_key_event(&mut self, key: KeyEvent) {
    // 1. Check if child handles it
    if self.bottom_pane.handle_key_event(key) {
        return;
    }
    
    // 2. Handle at this level
    match key {
        KeyEvent { code: KeyCode::Char('q'), .. } => {
            self.request_quit();
        }
        _ => {}
    }
}
```

## Performance Tips

1. **Avoid allocations in render()** - Use `Cell<T>` for mutable state
2. **Cache computed values** - Store in fields, invalidate on change
3. **Use `desired_height()` wisely** - Called frequently, keep it fast
4. **Batch redraws** - Use `request_redraw()` instead of immediate render

## Debugging

### Enable Debug Logs
```rust
// In Cargo.toml
[features]
debug-logs = []

// In code
#[cfg(feature = "debug-logs")]
tracing::debug!("My debug message: {}", value);
```

### View Logs
```bash
# Run with logging
RUST_LOG=debug cargo run 2> debug.log

# View logs
tail -f debug.log
```

### Test in Isolation
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_widget() {
        let widget = MyWidget::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 24));
        widget.render(Rect::new(0, 0, 80, 24), &mut buf);
        // Assert on buffer content
    }
}
```

## Next Steps for Integration

1. **Study codex-tui-app-server:**
   - Find message rendering code
   - Understand inline rendering approach
   - Identify reusable components

2. **Extract message list:**
   - Copy message rendering logic
   - Remove UI chrome (menus, branding)
   - Adapt to dx's Renderable trait

3. **Create unified interface:**
   - Add RenderMode enum
   - Implement both rendering paths
   - Add config flag to switch modes

4. **Test integration:**
   - Verify both modes work
   - Ensure smooth switching
   - Fix any layout issues

5. **Clean up:**
   - Remove duplicate code
   - Update documentation
   - Add tests

## Resources

- **Ratatui docs:** https://ratatui.rs/
- **Crossterm docs:** https://docs.rs/crossterm/
- **Codex protocol:** `codex-rs/protocol/`
- **TUI examples:** `codex-rs/dx/tests/`

---

**Pro tip:** The codebase is large (8000+ line files!). Use `ripgrep` to navigate:
```bash
rg "fn render" --type rust
rg "impl Renderable" --type rust
rg "KeyCode::" src/chatwidget.rs
```

Good luck with your integration! 🚀
