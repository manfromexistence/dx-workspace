# Codex Integration Analysis for DX

## Executive Summary

After analyzing the Codex TUI codebase, here's what you can extract and integrate into your DX toy without taking the problematic UI:

### What You CAN Use (Core Functionality)
1. **Markdown Rendering Engine** - Clean, well-structured markdown parser
2. **Streaming Controller** - Handles agent response streaming with animation
3. **Provider Integration** - How Codex calls AI providers (via `codex-core`)
4. **Sandboxing System** - Tool execution with safety controls
5. **Protocol Layer** - Event-driven architecture for agent communication

### What You SHOULD AVOID (UI Problems)
1. The entire `ChatWidget` and `BottomPane` UI components
2. The native terminal scrollbar implementation
3. The complex input handling system
4. The overlay and popup management

---

## Key Components to Extract

### 1. Markdown Rendering (`codex-rs/tui/src/markdown_render.rs`)

**What it does:**
- Parses markdown using `pulldown_cmark`
- Renders to `ratatui` styled `Line` and `Span` objects
- Handles code blocks with syntax highlighting
- Supports lists, blockquotes, headings, links
- Wraps text intelligently with proper indentation

**How to integrate:**
```rust
// You can use these functions directly:
use codex_tui::markdown_render::render_markdown_text;
use codex_tui::markdown_render::render_markdown_text_with_width;

// Example:
let markdown = "# Hello\n\nThis is **bold** text.";
let rendered: Text<'static> = render_markdown_text(markdown);
// Now you have ratatui Lines you can render in your DX message list
```

**Key files:**
- `codex-rs/tui/src/markdown_render.rs` - Main rendering logic
- `codex-rs/tui/src/markdown_stream.rs` - Streaming markdown collector
- `codex-rs/tui/src/markdown.rs` - Helper functions

### 2. Streaming Controller (`codex-rs/tui/src/streaming/controller.rs`)

**What it does:**
- Manages newline-gated streaming (waits for `\n` before committing lines)
- Handles animation timing for smooth text appearance
- Buffers and queues lines for display
- Supports batch commits for catch-up

**How to integrate:**
```rust
use codex_tui::streaming::controller::StreamController;

// Create controller
let mut controller = StreamController::new(Some(80), &cwd);

// Push deltas as they arrive from AI
controller.push("Hello ");
controller.push("world\n");

// On commit tick (animation frame)
let (cell, is_idle) = controller.on_commit_tick();
if let Some(cell) = cell {
    // cell contains Lines ready to display
    let lines = cell.transcript_lines(u16::MAX);
    // Add these lines to your message list
}

// Finalize when stream ends
if let Some(cell) = controller.finalize() {
    let lines = cell.transcript_lines(u16::MAX);
    // Final lines
}
```

**Key insight:** The streaming controller uses `MarkdownStreamCollector` internally, which:
- Buffers incomplete lines
- Only commits when it sees `\n`
- Renders markdown incrementally
- Prevents duplicate content

### 3. Provider Integration (`codex-rs/core/src/codex.rs`)

**What it does:**
- Manages the `Codex` struct that handles AI provider communication
- Sends `Op` (operations) and receives `Event` streams
- Handles model selection, authentication, configuration
- Manages conversation history and context

**Architecture:**
```
User Input → Op → Codex Session → Model Client → AI Provider
                                                      ↓
User Display ← Event ← Event Stream ← Response ← Provider
```

**Key types:**
- `Op` - Operations you send (UserTurn, ConfigureSession, etc.)
- `Event` - Events you receive (AgentMessageDeltaEvent, TurnCompleteEvent, etc.)
- `Codex` - Main interface with `submit()` and event receiver

**How to integrate:**
```rust
// Spawn a Codex session
let codex_spawn = Codex::spawn(CodexSpawnArgs {
    config,
    auth_manager,
    models_manager,
    // ... other args
}).await?;

let codex = codex_spawn.codex;

// Submit user message
codex.submit(Op::UserTurn {
    input: vec![UserInput::Text(TextElement {
        text: "Hello, AI!".to_string(),
    })],
    // ... other fields
}).await?;

// Receive events
while let Ok(event) = codex.rx_event.recv().await {
    match event {
        Event::AgentMessageDelta(delta) => {
            // Push delta.content to your streaming controller
        }
        Event::TurnComplete(_) => {
            // Finalize stream
        }
        // ... handle other events
    }
}
```

### 4. Sandboxing (`codex-rs/core/src/sandboxing/`)

**What it does:**
- Executes shell commands safely
- Manages file system permissions
- Controls network access
- Handles approval workflows

**Key concepts:**
- `SandboxPolicy` - Defines what's allowed
- `ExecPolicyManager` - Manages execution rules
- `ApprovalStore` - Tracks user approvals
- Platform-specific implementations (Seatbelt for macOS, Windows Sandbox, Landlock for Linux)

---

## Integration Strategy for DX

### Phase 1: Markdown Rendering
1. Add `codex-tui` as a dependency (or copy the markdown rendering files)
2. Replace your current message rendering with `render_markdown_text()`
3. Test with various markdown inputs

### Phase 2: Streaming
1. Integrate `StreamController` for agent responses
2. Set up animation timer (commit tick every 16-50ms)
3. Handle delta events from Codex

### Phase 3: Provider Integration
1. Add `codex-core` and `codex-protocol` dependencies
2. Set up `Codex` session on startup
3. Wire up your input to `Op::UserTurn`
4. Wire up events to your message list

### Phase 4: Tool Execution (Optional)
1. Integrate sandboxing if you want tool execution
2. Handle `ExecCommandBeginEvent`, `ExecCommandOutputDeltaEvent`, `ExecCommandEndEvent`
3. Display tool execution in your message list

---

## Recommended File Structure for DX

```
codex-rs/dx/src/
├── codex_integration/
│   ├── mod.rs              # Main integration module
│   ├── markdown.rs         # Markdown rendering wrapper
│   ├── streaming.rs        # Streaming controller wrapper
│   ├── provider.rs         # Codex provider interface
│   └── events.rs           # Event handling
├── state.rs                # Your existing state
├── dx_render.rs            # Your existing rendering
└── dispatcher.rs           # Your existing dispatcher
```

---

## Code Example: Complete Integration

```rust
// codex-rs/dx/src/codex_integration/mod.rs

use codex_core::{Codex, CodexSpawnArgs};
use codex_protocol::protocol::{Event, Op};
use codex_tui::streaming::controller::StreamController;
use ratatui::text::Line;

pub struct CodexIntegration {
    codex: Codex,
    stream_controller: Option<StreamController>,
    current_message_lines: Vec<Line<'static>>,
}

impl CodexIntegration {
    pub async fn new(config: Config) -> Result<Self> {
        let codex_spawn = Codex::spawn(CodexSpawnArgs {
            config,
            // ... setup args
        }).await?;

        Ok(Self {
            codex: codex_spawn.codex,
            stream_controller: None,
            current_message_lines: Vec::new(),
        })
    }

    pub async fn send_message(&mut self, text: String) -> Result<()> {
        self.codex.submit(Op::UserTurn {
            input: vec![UserInput::Text(TextElement { text })],
            // ... other fields
        }).await?;
        Ok(())
    }

    pub async fn poll_events(&mut self) -> Option<Vec<Line<'static>>> {
        match self.codex.rx_event.try_recv() {
            Ok(Event::AgentMessageDelta(delta)) => {
                // Initialize stream controller if needed
                if self.stream_controller.is_none() {
                    let cwd = std::env::current_dir().ok()?;
                    self.stream_controller = Some(
                        StreamController::new(Some(80), &cwd)
                    );
                }

                // Push delta
                if let Some(controller) = &mut self.stream_controller {
                    controller.push(&delta.content);
                }
                None
            }
            Ok(Event::TurnComplete(_)) => {
                // Finalize stream
                if let Some(mut controller) = self.stream_controller.take() {
                    if let Some(cell) = controller.finalize() {
                        return Some(cell.transcript_lines(u16::MAX));
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub fn on_animation_tick(&mut self) -> Option<Vec<Line<'static>>> {
        if let Some(controller) = &mut self.stream_controller {
            let (cell, _is_idle) = controller.on_commit_tick();
            if let Some(cell) = cell {
                return Some(cell.transcript_lines(u16::MAX));
            }
        }
        None
    }
}
```

---

## Dependencies You'll Need

Add to `codex-rs/dx/Cargo.toml`:

```toml
[dependencies]
# Core Codex functionality
codex-core = { path = "../core" }
codex-protocol = { path = "../protocol" }
codex-tui = { path = "../tui" }  # For markdown rendering

# Markdown parsing
pulldown-cmark = "0.9"

# Already have ratatui
ratatui = { version = "0.28", features = ["all-widgets"] }
```

---

## Key Differences from Codex TUI

| Aspect | Codex TUI | Your DX Approach |
|--------|-----------|------------------|
| Scrollbar | Native terminal | Custom rendered |
| Input | Complex BottomPane | Simple input box |
| Layout | Multiple panes | Single message list |
| Animations | Limited | Full custom animations |
| Message Display | HistoryCell abstraction | Direct Line rendering |

---

## Next Steps

1. **Start Small**: Begin with just markdown rendering
2. **Test Incrementally**: Verify each component works before moving to the next
3. **Keep It Minimal**: Don't import unnecessary Codex UI code
4. **Use Codex Core**: Let `codex-core` handle provider communication
5. **Custom UI**: Keep your beautiful DX UI and animations

---

## Questions to Consider

1. **Do you want tool execution?** (shell commands, file operations)
   - If yes: Integrate sandboxing
   - If no: Just handle text responses

2. **Do you want streaming?** (text appears gradually)
   - If yes: Use `StreamController`
   - If no: Just use `render_markdown_text()` on complete responses

3. **Do you want multiple providers?** (OpenAI, Anthropic, etc.)
   - Codex core handles this via `ModelsManager`

4. **Do you want conversation history?**
   - Codex core manages this via `ThreadManager`

---

## Conclusion

You can absolutely integrate Codex's AI functionality without its UI. The key is:

1. Use `codex-core` for provider communication
2. Use `codex-tui`'s markdown rendering (it's clean and separate)
3. Use `StreamController` for smooth streaming
4. Keep your custom DX UI, scrollbar, and animations
5. Wire events from Codex into your message list

The Codex codebase is well-structured with clear separation between core logic and UI, making this integration very feasible.
