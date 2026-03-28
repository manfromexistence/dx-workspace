# Codex-Rust CLI TUI Architecture Documentation

> **Last Updated:** March 25, 2026  
> **Project:** dx (Codex CLI Rust Fork)  
> **Location:** `codex-rs/tui/`

## Table of Contents

1. [Overview](#overview)
2. [Project Structure](#project-structure)
3. [Core Architecture](#core-architecture)
4. [Key Components](#key-components)
5. [Styling System](#styling-system)
6. [Event Flow](#event-flow)
7. [State Management](#state-management)
8. [Rendering Pipeline](#rendering-pipeline)
9. [Testing Strategy](#testing-strategy)
10. [Development Guidelines](#development-guidelines)

---

## 1. Overview

The Codex TUI (Terminal User Interface) is a sophisticated terminal-based chat interface built with Rust and the `ratatui` library. It provides an interactive environment for conversing with AI models, executing commands, managing files, and handling complex workflows like code reviews, approvals, and multi-agent collaboration.

### Key Features

- **Real-time streaming** of AI responses with markdown rendering
- **Multi-agent support** with thread management
- **Guardian approvals** for sensitive operations
- **MCP (Model Context Protocol)** tool integration
- **Voice input** with transcription (optional feature)
- **Image support** (paste, view, generate)
- **Syntax highlighting** for code blocks and diffs
- **Session management** (resume, fork, compact)
- **Slash commands** for quick actions
- **Customizable themes** and status lines
- **Realtime audio** conversations (experimental)

### Technology Stack

- **Language:** Rust Edition 2024
- **TUI Framework:** ratatui (with unstable features)
- **Terminal Backend:** crossterm
- **Markdown Parsing:** pulldown-cmark
- **Syntax Highlighting:** syntect + two-face
- **Image Processing:** image crate (jpeg, png, gif, webp)
- **Diff Rendering:** diffy
- **Testing:** insta (snapshot tests), vt100 (terminal emulation)

---

## 2. Project Structure

```
codex-rs/tui/
├── src/
│   ├── app.rs                    # Main application state and event loop
│   ├── lib.rs                    # Public API and entry point
│   ├── main.rs                   # Binary entry point
│   ├── tui.rs                    # Terminal abstraction layer
│   ├── chatwidget.rs             # Core chat interface component
│   │
│   ├── app/                      # App-specific modules
│   │   ├── agent_navigation.rs  # Multi-agent thread switching
│   │   └── pending_interactive_replay.rs
│   │
│   ├── bottom_pane/              # Input and footer components
│   │   ├── mod.rs               # Bottom pane orchestration
│   │   ├── chat_composer.rs     # Text input with mentions
│   │   ├── footer.rs            # Status line and hints
│   │   ├── approval_overlay.rs  # Guardian approval UI
│   │   ├── command_popup.rs     # Slash command picker
│   │   ├── file_search_popup.rs # Fuzzy file search
│   │   └── ...                  # Other overlays and views
│   │
│   ├── chatwidget/               # Chat widget submodules
│   │   ├── agent.rs             # Agent message handling
│   │   ├── plugins.rs           # Plugin integration
│   │   ├── realtime.rs          # Realtime audio
│   │   ├── session_header.rs    # Session info display
│   │   └── skills.rs            # Skills management
│   │
│   ├── history_cell.rs           # Conversation history rendering
│   ├── markdown_render.rs        # Markdown to ratatui conversion
│   ├── diff_render.rs            # Unified diff display
│   ├── status_indicator_widget.rs # Working/status display
│   │
│   ├── render/                   # Rendering utilities
│   │   ├── highlight.rs         # Syntax highlighting
│   │   ├── line_utils.rs        # Line manipulation
│   │   └── renderable.rs        # Renderable trait
│   │
│   ├── streaming/                # Streaming response handling
│   │   ├── controller.rs        # Stream state machine
│   │   ├── chunking.rs          # Chunk processing
│   │   └── commit_tick.rs       # Render commit timing
│   │
│   ├── status/                   # Status display components
│   │   ├── account.rs           # Account info
│   │   ├── rate_limits.rs       # Rate limit display
│   │   └── card.rs              # Status card rendering
│   │
│   ├── onboarding/               # First-run experience
│   │   ├── auth.rs              # Authentication flow
│   │   ├── welcome.rs           # Welcome screen
│   │   └── trust_directory.rs   # Directory trust prompt
│   │
│   └── ...                       # Additional modules
│
├── tests/                        # Integration tests
├── frames/                       # Test frame snapshots
├── Cargo.toml                    # Dependencies
├── styles.md                     # Styling guidelines
└── tooltips.txt                  # User guidance tips
```

---

## 3. Core Architecture

### 3.1 Application Structure

The TUI follows a component-based architecture with clear separation of concerns:

```
┌─────────────────────────────────────────────────────────┐
│                         App                             │
│  - Event loop orchestration                             │
│  - Thread management                                    │
│  - Configuration                                        │
└─────────────────────────────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
┌───────▼────────┐ ┌──────▼──────┐ ┌───────▼────────┐
│   ChatWidget   │ │ BottomPane  │ │      Tui       │
│  - History     │ │  - Composer │ │  - Terminal    │
│  - Streaming   │ │  - Footer   │ │  - Events      │
│  - Events      │ │  - Overlays │ │  - Drawing     │
└────────────────┘ └─────────────┘ └────────────────┘
```

### 3.2 Event-Driven Model

The TUI uses an async event-driven architecture:

1. **Input Events** (keyboard, paste, resize) → `TuiEvent`
2. **Codex Events** (AI responses, tool calls) → `Event` (from protocol)
3. **App Events** (internal state changes) → `AppEvent`

Events flow through the system:
```
Terminal Input → Tui → App → ChatWidget/BottomPane → Op (to Codex core)
                  ↑                                      ↓
                  └──────── Codex Events ←──────────────┘
```

### 3.3 Threading Model

- **Main Thread:** Event loop, rendering, user input
- **Background Tasks:** 
  - Thread event listeners (per-thread event streams)
  - Rate limit polling
  - Connector fetching
  - File search
  - Voice capture (when enabled)

---

## 4. Key Components

### 4.1 App (`app.rs`)

The `App` struct is the top-level orchestrator:

**Responsibilities:**
- Manages multiple chat threads (primary + subagents)
- Routes events to appropriate thread channels
- Handles session lifecycle (new, resume, fork)
- Coordinates configuration updates
- Manages external editor integration
- Handles exit/shutdown logic

**Key State:**
- `chat_widget: ChatWidget` - Active conversation view
- `bottom_pane: BottomPane` - Input and status UI
- `thread_channels: HashMap<ThreadId, ThreadEventChannel>` - Per-thread event streams
- `config: Config` - Current configuration
- `session_selection: SessionSelection` - Startup mode (fresh/resume/fork)

**Critical Methods:**
- `run()` - Main event loop
- `handle_event()` - Event dispatcher
- `handle_active_thread_event()` - Process Codex events
- `submit_op_to_thread()` - Send operations to Codex core
- `open_agent_picker()` - Multi-agent thread selection

### 4.2 ChatWidget (`chatwidget.rs`)

The `ChatWidget` is the core conversation interface:

**Responsibilities:**
- Renders conversation history
- Streams AI responses in real-time
- Manages active cells (user input, agent output, tool calls)
- Handles slash commands
- Coordinates approvals and reviews
- Manages collaboration modes (chat/plan/code)
- Tracks token usage and context window

**Key State:**
- `history: Vec<Box<dyn HistoryCell>>` - Conversation cells
- `active_cell: Option<Box<dyn HistoryCell>>` - Currently streaming cell
- `stream_controller: StreamController` - Streaming state machine
- `config: Config` - Widget-specific config
- `thread_id: Option<ThreadId>` - Associated thread
- `token_info: Option<TokenUsageInfo>` - Context tracking

**Event Handlers:**
- `on_agent_message_delta()` - Stream AI text
- `on_exec_command_begin/end()` - Tool execution
- `on_guardian_assessment()` - Approval requests
- `on_task_complete()` - Turn finalization
- `dispatch_command()` - Slash command execution

### 4.3 BottomPane (`bottom_pane/mod.rs`)

The `BottomPane` manages user input and status display:

**Responsibilities:**
- Text input with mention support (@file, #skill, /command)
- Status indicator (working, elapsed time)
- Footer hints and shortcuts
- Modal overlays (approvals, pickers, prompts)
- Paste burst handling
- Image attachment preview

**Key Components:**
- `ChatComposer` - Multi-line text input with history
- `StatusIndicatorWidget` - Working status with timer
- `Footer` - Dynamic hint display
- View stack for modals/overlays

**Input Handling:**
- Enter: Submit or queue message
- Tab: Autocomplete or queue when busy
- Esc: Cancel, dismiss modal, or backtrack
- Ctrl+C: Interrupt or clear
- Ctrl+L: Clear UI
- Paste: Handle text/images

### 4.4 History Cells (`history_cell.rs`)

History cells are the building blocks of conversation display:

**Cell Types:**
- `UserHistoryCell` - User messages with images
- `AgentMessageCell` - AI responses (markdown)
- `SessionHeaderHistoryCell` - Session info banner
- `McpToolCallCell` - MCP tool invocations
- `WebSearchCell` - Web search operations
- `UnifiedExecInteractionCell` - Shell command execution
- `PatchHistoryCell` - File diffs
- `PlainHistoryCell` - Generic text
- `TooltipHistoryCell` - User tips
- `ReasoningSummaryCell` - Reasoning traces

**HistoryCell Trait:**
```rust
pub trait HistoryCell {
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn desired_height(&self, width: u16) -> u16;
    fn transcript_lines(&self, width: u16) -> Vec<Line<'static>>;
    fn is_stream_continuation(&self) -> bool { false }
    fn transcript_animation_tick(&self) -> Option<u64> { None }
}
```

### 4.5 Markdown Rendering (`markdown_render.rs`)

Converts markdown to styled ratatui `Text`:

**Features:**
- Headers (bold with # prefix)
- Lists (bullet, numbered, nested)
- Code blocks (syntax highlighted)
- Inline code (cyan background)
- Links (cyan, underlined, with destination)
- Blockquotes (dim with prefix)
- Text wrapping with proper indentation
- Local file link resolution

**Styling:**
- Uses `MarkdownStyles` for consistent appearance
- Respects terminal color scheme
- Handles ANSI color codes
- Wraps long lines intelligently

### 4.6 Diff Rendering (`diff_render.rs`)

Displays unified diffs with syntax highlighting:

**Features:**
- Line-by-line diff display
- Syntax highlighting for code
- Add/delete/context line coloring
- Line number display
- File path headers
- Hunk separation
- Long line wrapping

**Colors:**
- Green: Added lines
- Red: Deleted lines
- Dim: Context lines
- Cyan: File headers

### 4.7 Status Indicator (`status_indicator_widget.rs`)

Shows working status with elapsed time:

**Display:**
```
Working... (12s)
  Details text here
  [Esc to interrupt]
```

**Features:**
- Animated header
- Elapsed time tracking
- Pause/resume timer
- Detail text wrapping
- Interrupt hint
- Inline message support

### 4.8 Streaming Controller (`streaming/controller.rs`)

Manages real-time response streaming:

**States:**
- Idle
- Streaming (agent message, plan, reasoning)
- Buffering (waiting for commit tick)
- Flushing (finalizing output)

**Commit Tick System:**
- Batches rapid deltas for smooth rendering
- Prevents frame rate issues
- Handles backpressure
- Coordinates with ratatui draw cycle

---

## 5. Styling System

### 5.1 Style Guidelines (`styles.md`)

**Headers:** Bold text, keep markdown `#` signs

**Primary Text:** Default foreground color

**Secondary Text:** Dim modifier

**Foreground Colors:**
- **Default:** Most content (use `reset` to restore)
- **Cyan:** User input tips, selection, status, links
- **Green:** Success, additions
- **Red:** Errors, failures, deletions
- **Magenta:** Codex branding

**Avoid:**
- Custom RGB colors (terminal theme compatibility)
- ANSI black/white as foreground (use default)
- ANSI blue/yellow (not in style guide)

### 5.2 Ratatui Stylize Helpers

**Preferred Patterns:**
```rust
// Simple spans
"text".into()

// Styled spans
"text".red()
"text".green()
"text".cyan()
"text".dim()
"text".bold()

// Chained styles
url.cyan().underlined()

// Lines from spans
vec!["  └ ".into(), "M".red(), " ".dim(), "file.rs".dim()]
```

**Avoid:**
```rust
// Manual style construction (unless computed)
Span::styled("text", Style::default().fg(Color::Red))

// Hardcoded white
"text".white()  // Use default instead
```

### 5.3 Text Wrapping

**Always use `textwrap::wrap`** for plain strings

**For ratatui Lines:** Use helpers in `wrapping.rs`:
- `word_wrap_lines()` - Wrap multiple lines
- `word_wrap_line()` - Wrap single line
- `prefix_lines()` - Add prefixes to wrapped lines

**Indentation:**
- Use `initial_indent` / `subsequent_indent` from `RtOptions`
- Avoid custom wrapping logic

---

## 6. Event Flow

### 6.1 User Input Flow

```
Keyboard/Mouse
    ↓
crossterm events
    ↓
Tui::event_stream()
    ↓
TuiEvent
    ↓
App::handle_tui_event()
    ↓
┌─────────────────────────────────┐
│ Key routing:                    │
│ - Ctrl+C → interrupt/cancel     │
│ - Ctrl+L → clear UI             │
│ - Esc → backtrack/dismiss       │
│ - Enter → submit                │
│ - Tab → autocomplete/queue      │
│ - Other → BottomPane/ChatWidget │
└─────────────────────────────────┘
    ↓
Op (operation to Codex core)
    ↓
Thread event stream
```

### 6.2 Codex Event Flow

```
Codex Core (SSE stream)
    ↓
Event (protocol)
    ↓
ThreadEventChannel
    ↓
App::handle_active_thread_event()
    ↓
ChatWidget::handle_codex_event()
    ↓
┌──────────────────────────────────┐
│ Event dispatch:                  │
│ - SessionConfigured → setup      │
│ - AgentMessageDelta → stream     │
│ - ExecCommandBegin → tool start  │
│ - GuardianAssessment → approval  │
│ - TaskComplete → finalize        │
│ - Error → display                │
└──────────────────────────────────┘
    ↓
Update UI state
    ↓
Request redraw
```

### 6.3 Rendering Flow

```
Frame requested
    ↓
Tui::draw()
    ↓
App::render()
    ↓
┌─────────────────────────────┐
│ Layout calculation:         │
│ - ChatWidget area           │
│ - BottomPane area           │
│ - Overlay areas             │
└─────────────────────────────┘
    ↓
Component rendering
    ↓
┌─────────────────────────────┐
│ ChatWidget::render()        │
│ - History cells             │
│ - Active streaming cell     │
│ - Scroll state              │
└─────────────────────────────┘
    ↓
┌─────────────────────────────┐
│ BottomPane::render()        │
│ - Status indicator          │
│ - Composer                  │
│ - Footer                    │
│ - Active overlay            │
└─────────────────────────────┘
    ↓
Buffer flush to terminal
```

---

## 7. State Management

### 7.1 Configuration

Configuration is hierarchical:
- User-level: `~/.codex/config.toml`
- Workspace-level: `.codex/config.toml`
- Runtime overrides: CLI flags, environment

**Key Config Sections:**
- `model` - Model selection
- `approvals` - Guardian settings
- `sandbox` - Execution policy
- `features` - Feature flags
- `tui` - TUI-specific settings
- `apps` - MCP/plugin configuration

**Config Refresh:**
- On session start
- On `/reload` command
- On CWD change (for workspace config)

### 7.2 Thread Management

**Thread Types:**
- **Primary:** Main conversation thread
- **Subagent:** Background agent threads

**Thread State:**
- `ThreadEventStore` - Event history buffer
- `ThreadEventChannel` - Event stream receiver
- Active/inactive status
- Pending approvals

**Thread Switching:**
- Agent picker UI (`Ctrl+K` or `/agent`)
- Automatic on subagent events
- Preserves input state per thread

### 7.3 Session State

**Session Selection:**
- `Fresh` - New conversation
- `Resume(thread_id)` - Continue existing
- `Fork(thread_id)` - Branch from existing

**Session Lifecycle:**
1. Load config for CWD
2. Resolve thread ID (if resume/fork)
3. Initialize ChatWidget
4. Replay events (if resume)
5. Start event listener
6. Enter main loop

**Session Persistence:**
- Thread metadata in SQLite
- Event log for replay
- Draft input saved
- Queued messages preserved

---

## 8. Rendering Pipeline

### 8.1 Layout System

The TUI uses ratatui's layout system:

```rust
let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Min(1),        // ChatWidget
        Constraint::Length(height) // BottomPane
    ])
    .split(area);
```

**Responsive Heights:**
- ChatWidget: Fills remaining space
- BottomPane: Dynamic based on content
  - Status indicator: 2-4 lines
  - Composer: 1-10 lines (auto-grow)
  - Footer: 1-2 lines
  - Overlays: Centered or full-screen

### 8.2 Scrolling

**ChatWidget Scrolling:**
- Auto-scroll to bottom during streaming
- Manual scroll with arrow keys/PgUp/PgDn
- Scroll lock when user scrolls up
- Unlock on new user message

**Overlay Scrolling:**
- Pager overlay for long content
- List views with selection
- Transcript overlay for history

### 8.3 Frame Rate Management

**Frame Requester:**
- Debounces rapid redraws
- Limits to ~60 FPS
- Batches updates during streaming

**Commit Tick:**
- Coordinates streaming with rendering
- Prevents partial updates
- Ensures smooth animation

### 8.4 Alternate Screen

**Modes:**
- Alternate screen (default): Clean slate, no history pollution
- Main screen: Leaves output in scrollback

**Transitions:**
- Enter alt screen on startup
- Leave on exit (with restore)
- Temporary leave for external editor

---

## 9. Testing Strategy

### 9.1 Snapshot Tests

Uses `insta` for visual regression testing:

**Test Pattern:**
```rust
#[test]
fn component_snapshot() {
    let cell = create_test_cell();
    let rendered = render_to_string(&cell, 80);
    insta::assert_snapshot!(rendered);
}
```

**Snapshot Locations:**
- `src/snapshots/` - Inline snapshots
- `tests/snapshots/` - Integration snapshots

**Workflow:**
1. Run tests: `cargo test -p codex-tui`
2. Review changes: `cargo insta pending-snapshots`
3. Accept: `cargo insta accept -p codex-tui`

### 9.2 VT100 Tests

Terminal emulation tests with `vt100` crate:

**Use Cases:**
- Full TUI integration tests
- Event sequence testing
- Rendering verification

**Pattern:**
```rust
#[cfg(feature = "vt100-tests")]
#[test]
fn interaction_test() {
    let mut parser = vt100::Parser::new(24, 80, 0);
    // Render to parser
    // Send key events
    // Assert screen content
}
```

### 9.3 Unit Tests

**Component Tests:**
- History cell rendering
- Markdown parsing
- Diff rendering
- Text wrapping
- State transitions

**Test Utilities:**
- `test_backend.rs` - Mock terminal
- `common/` - Shared fixtures
- `pretty_assertions` - Better diffs

---

## 10. Development Guidelines

### 10.1 Code Organization

**Module Size:**
- Target: <500 LoC (excluding tests)
- Hard limit: ~800 LoC
- Extract to new module when exceeded

**High-Touch Files:**
- `app.rs` - Avoid growing further
- `chatwidget.rs` - Extract to submodules
- `bottom_pane/mod.rs` - Keep orchestration only

**Naming:**
- Crates: `codex-*` prefix
- Modules: Snake case
- Types: PascalCase
- Functions: Snake case

### 10.2 Error Handling

**Patterns:**
- Use `anyhow::Result` for application errors
- Use `thiserror` for library errors
- Avoid `unwrap()` in production paths
- Log errors with `tracing`

**User-Facing Errors:**
- Display in chat as error cells
- Show in status indicator
- Provide actionable hints

### 10.3 Performance

**Rendering:**
- Minimize allocations in hot paths
- Cache computed layouts
- Use `Cow` for borrowed/owned strings
- Batch updates during streaming

**Memory:**
- Limit history buffer size
- Clean up closed threads
- Release resources on drop

**Async:**
- Use `tokio` for I/O
- Avoid blocking in event loop
- Cancel tasks on shutdown

### 10.4 Accessibility

**Considerations:**
- Screen reader compatibility (transcript mode)
- Keyboard-only navigation
- High contrast support
- Configurable colors

**Testing:**
- Manual testing with screen readers
- Keyboard navigation verification
- Color blindness simulation

### 10.5 Formatting

**Always run:**
```bash
just fmt
```

**Clippy:**
```bash
just fix -p codex-tui
```

**Argument Comments:**
```rust
// Good
foo(true /*enabled*/, None /*default*/)

// Bad (for bools/None)
foo(true, None)
```

### 10.6 Documentation

**Module Docs:**
- Explain purpose and responsibilities
- Document state machines
- Provide usage examples

**Function Docs:**
- Document public APIs
- Explain non-obvious behavior
- Note invariants and assumptions

**Inline Comments:**
- Explain "why", not "what"
- Mark TODOs with context
- Reference issues/PRs when relevant

---

## Appendix A: Key Dependencies

### Core TUI
- `ratatui` ^0.29 - Terminal UI framework
- `crossterm` ^0.28 - Terminal backend
- `tokio` ^1.0 - Async runtime

### Rendering
- `pulldown-cmark` ^0.12 - Markdown parsing
- `syntect` ^5.0 - Syntax highlighting
- `two-face` ^0.5 - Syntax theme management
- `diffy` ^0.4 - Diff generation
- `textwrap` ^0.16 - Text wrapping

### Media
- `image` ^0.25 - Image processing
- `cpal` ^0.15 - Audio capture (optional)
- `hound` ^3.5 - WAV encoding (optional)

### Utilities
- `anyhow` - Error handling
- `serde` + `serde_json` - Serialization
- `tracing` - Logging
- `uuid` - Unique IDs
- `chrono` - Timestamps

### Testing
- `insta` - Snapshot testing
- `vt100` - Terminal emulation
- `pretty_assertions` - Better test output

---

## Appendix B: Slash Commands

| Command | Description |
|---------|-------------|
| `/new` | Start fresh conversation |
| `/resume` | Resume previous session |
| `/fork` | Branch current conversation |
| `/compact` | Summarize history |
| `/model` | Change model/reasoning |
| `/permissions` | Configure approvals |
| `/review` | Request code review |
| `/status` | Show current status |
| `/statusline` | Configure status line |
| `/mcp` | List MCP tools |
| `/skills` | List/use skills |
| `/feedback` | Send feedback |
| `/init` | Create AGENTS.md |
| `/rename` | Rename thread |
| `/personality` | Change communication style |
| `/theme` | Select TUI theme |

---

## Appendix C: Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Enter` | Submit message (or queue if busy) |
| `Tab` | Autocomplete / Queue message |
| `Esc` | Backtrack / Dismiss modal / Cancel |
| `Ctrl+C` | Interrupt / Clear input |
| `Ctrl+L` | Clear UI |
| `Ctrl+D` | Exit (when idle) |
| `Ctrl+K` | Open agent picker |
| `Ctrl+V` | Paste (text or image) |
| `↑/↓` | Navigate history / Scroll |
| `PgUp/PgDn` | Scroll page |
| `/` | Open command popup |
| `!` | Shell command prefix |

---

## Appendix D: File Locations

### Configuration
- `~/.codex/config.toml` - User config
- `.codex/config.toml` - Workspace config
- `~/.codex/settings/mcp.json` - MCP servers

### Data
- `~/.codex/state/` - Session state
- `~/.codex/threads.db` - Thread metadata
- `~/.codex/logs/` - Debug logs

### Workspace
- `.codex/steering/` - Steering files
- `.codex/skills/` - Custom skills
- `AGENTS.md` - Project guidance

---

## Appendix E: Environment Variables

| Variable | Purpose |
|----------|---------|
| `CODEX_SANDBOX` | Sandbox mode indicator |
| `CODEX_SANDBOX_NETWORK_DISABLED` | Network disabled flag |
| `CODEX_LOG` | Log level override |
| `CODEX_CONFIG_DIR` | Config directory override |
| `NO_COLOR` | Disable colors |
| `TERM` | Terminal type detection |

---

## Appendix F: Feature Flags

### Cargo Features
- `default` - Standard features + voice-input
- `voice-input` - Voice capture support
- `vt100-tests` - Terminal emulation tests
- `debug-logs` - Verbose TUI logging

### Runtime Features (Config)
- `guardian` - AI-powered approvals
- `multi_agent` - Subagent support
- `web_search` - Web search tool
- `realtime` - Realtime audio conversations
- `collaboration_modes` - Chat/Plan/Code modes

---

## Appendix G: Common Patterns

### Adding a New History Cell

1. Define struct in `history_cell.rs`:
```rust
pub struct MyCell {
    content: String,
}
```

2. Implement `HistoryCell` trait:
```rust
impl HistoryCell for MyCell {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let lines = self.display_lines(area.width);
        // Render lines to buffer
    }
    
    fn desired_height(&self, width: u16) -> u16 {
        self.display_lines(width).len() as u16
    }
}
```

3. Add constructor:
```rust
pub(crate) fn new_my_cell(content: String) -> Box<dyn HistoryCell> {
    Box::new(MyCell { content })
}
```

4. Use in `ChatWidget`:
```rust
self.add_to_history(new_my_cell(content));
```

5. Add snapshot test:
```rust
#[test]
fn my_cell_snapshot() {
    let cell = new_my_cell("test".into());
    insta::assert_snapshot!(render_transcript(&*cell));
}
```

### Adding a New Slash Command

1. Add variant to `SlashCommand` enum in `slash_command.rs`

2. Add command metadata:
```rust
SlashCommand::MyCommand => CommandMetadata {
    name: "mycommand",
    description: "Does something",
    aliases: &["mc"],
    args: &[],
}
```

3. Implement handler in `ChatWidget::dispatch_command()`:
```rust
SlashCommand::MyCommand => {
    // Handle command
    self.add_info_message("Command executed".into(), None);
}
```

4. Add to command popup in `bottom_pane/command_popup.rs`

5. Update tooltips in `tooltips.txt`

### Adding a New Modal/Overlay

1. Create view struct implementing `BottomPaneView` trait

2. Add to `BottomPane::show_view()` or `push_view()`

3. Handle key events in view's `handle_key_event()`

4. Return `InputResult::Complete` when done

5. Process result in `BottomPane::on_active_view_complete()`

---

## Appendix H: Troubleshooting

### Common Issues

**Rendering artifacts:**
- Check terminal size handling
- Verify layout constraints
- Test with different terminal emulators

**Event loop hangs:**
- Check for blocking operations
- Verify async task cancellation
- Review channel buffer sizes

**Memory leaks:**
- Check history buffer limits
- Verify thread cleanup
- Review Arc/Rc cycles

**Snapshot test failures:**
- Review visual changes
- Check terminal width assumptions
- Verify wrapping behavior

### Debug Tools

**Logging:**
```bash
CODEX_LOG=debug codex-tui
```

**Frame inspection:**
```rust
#[cfg(feature = "debug-logs")]
tracing::debug!("Frame: {:?}", frame);
```

**State dumps:**
```rust
dbg!(&self.state);
```

---

## Conclusion

The Codex TUI is a sophisticated terminal application with rich features and careful attention to user experience. This document provides a comprehensive overview of its architecture, components, and development practices.

For updates or questions, refer to:
- Project repository
- `AGENTS.md` files in relevant directories
- Code comments and module documentation
- Snapshot tests for visual examples

**Remember:** Always run `just fmt` after changes and add snapshot tests for UI modifications.
