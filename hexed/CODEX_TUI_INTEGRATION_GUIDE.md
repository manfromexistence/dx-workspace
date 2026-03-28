# Codex TUI Integration into DX TUI - Complete Implementation Guide

## Goal
Embed the full, working Codex TUI (from `codex-rs/tui/`) inside DX TUI (from `codex-rs/dx/`) so users can toggle between:
- **Ollama Chat** (simple local LLM chat with GGUF models)
- **Codex TUI** (full production Codex agent with all features)

## Current Status

### What Works
- DX TUI runs successfully with Yazi file browser integration
- Ollama chat works with local GGUF models
- Basic toggle mechanism exists (Ctrl+B)
- Placeholder shows where Codex TUI should render

### What's Needed
- Initialize real Codex App inside DX
- Forward events from DX to Codex App
- Render Codex App's UI in the message area
- Handle Codex's async event system

## Architecture Analysis

### DX TUI Structure
```
codex-rs/dx/src/
├── dx.rs                    # Main entry point
├── file_browser/            # Yazi file browser (fb_*)
│   └── app/App              # Yazi's App struct
├── bridge.rs                # YaziChatBridge - connects file browser & chat
├── state.rs                 # ChatState - Ollama chat state
├── dispatcher.rs            # Event routing (keyboard, mouse)
├── dx_render.rs             # Rendering logic
└── codex_integration.rs     # NEW: Codex TUI integration (started)
```

**Key Files:**
- `YaziChatBridge` - Main state container
- `ChatState` - Ollama chat state (messages, LLM, etc.)
- `dispatcher.rs` - Routes all events (keyboard, mouse, timer)
- `dx_render.rs` - Renders UI (animations, chat, file browser)

### Codex TUI Structure
```
codex-rs/tui/src/ (also in codex-rs/dx/src/ as codex_lib.rs)
├── lib.rs / codex_lib.rs    # Main entry point, run_main()
├── app.rs                   # App struct - main application
├── chatwidget.rs            # ChatWidget - handles chat UI
├── tui.rs                   # Tui struct - terminal management
├── app_event.rs             # AppEvent enum - event system
└── bottom_pane/             # Input, status, overlays
```

**Key Components:**
- `App` - Main application struct with ChatWidget, ThreadManager, etc.
- `ChatWidget` - Handles all chat UI and protocol events
- `Tui` - Manages terminal (alternate screen, raw mode, etc.)
- `AppEvent` - Event system (Key, Mouse, Timer, etc.)

## Critical Discovery: How Codex TUI Works

### 1. Initialization (from `codex_lib.rs::run_main()`)

```rust
pub async fn run_main(
    mut cli: Cli,
    arg0_paths: Arg0DispatchPaths,
    loader_overrides: LoaderOverrides,
) -> std::io::Result<AppExitInfo> {
    // 1. Load config
    let codex_home = find_codex_home()?;
    let config_toml = load_config_as_toml_with_cli_overrides(...).await?;
    
    // 2. Create AuthManager
    let auth_manager = AuthManager::shared(...);
    
    // 3. Create CloudRequirementsLoader
    let cloud_requirements = cloud_requirements_loader(...);
    
    // 4. Build Config
    let config = ConfigBuilder::new()
        .codex_home(codex_home)
        .cwd(cwd)
        .config_toml(config_toml)
        .cloud_requirements(cloud_requirements)
        .build().await?;
    
    // 5. Create SessionTelemetry
    let session_telemetry = SessionTelemetry::new(...);
    
    // 6. Create Tui (terminal manager)
    let mut tui = tui::Tui::new(...)?;
    
    // 7. Run the app
    App::run(
        &mut tui,
        auth_manager,
        config,
        ...
    ).await
}
```

### 2. App::run() - Main Event Loop (from `app.rs`)

```rust
pub async fn run(
    tui: &mut tui::Tui,
    auth_manager: Arc<AuthManager>,
    config: Config,
    ...
) -> Result<AppExitInfo> {
    // 1. Create event channel
    let (app_event_tx, mut app_event_rx) = unbounded_channel();
    
    // 2. Create ThreadManager
    let thread_manager = Arc::new(ThreadManager::new(...));
    
    // 3. Create ChatWidget
    let chat_widget = ChatWidget::new(...).await?;
    
    // 4. Create App
    let mut app = App {
        server: thread_manager,
        chat_widget,
        app_event_tx,
        ...
    };
    
    // 5. Main event loop
    loop {
        tokio::select! {
            // Handle TUI events (keyboard, mouse, etc.)
            Some(event) = tui.events.next() => {
                app.handle_tui_event(event, tui)?;
            }
            
            // Handle app events (from ChatWidget, ThreadManager, etc.)
            Some(event) = app_event_rx.recv() => {
                app.handle_app_event(event, tui).await?;
            }
            
            // Render
            _ = frame_rx.recv() => {
                tui.draw(|frame| {
                    app.draw(frame);
                })?;
            }
        }
        
        if app.should_exit {
            break;
        }
    }
    
    Ok(app.exit_info())
}
```

### 3. Rendering (from `app.rs`)

```rust
impl App {
    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.size();
        
        // Render ChatWidget
        self.chat_widget.render(area, frame.buffer_mut());
        
        // Render overlays (transcript, diff, etc.)
        if let Some(overlay) = &self.overlay {
            overlay.render(area, frame.buffer_mut());
        }
    }
}
```

**Key Insight:** Codex App doesn't have a standalone `render()` method that takes a `Rect` and `Buffer`. It's designed to be called from within `tui.draw()` which provides a `Frame`.

## The Problem

### Why Direct Embedding is Complex

1. **Terminal Management Conflict**
   - Codex TUI manages its own terminal (alternate screen, raw mode)
   - DX TUI also manages terminal (for Yazi)
   - Can't have two terminal managers fighting for control

2. **Event Loop Conflict**
   - Codex has its own `tokio::select!` event loop
   - DX has Yazi's event loop
   - Can't run two event loops simultaneously

3. **Rendering Architecture**
   - Codex expects to render via `tui.draw(|frame| ...)`
   - DX renders directly to buffer in `dx_render.rs`
   - Different rendering paradigms

4. **State Management**
   - Codex App owns ChatWidget, ThreadManager, etc.
   - These are tightly coupled and expect to run in Codex's event loop
   - Can't easily extract just the rendering part

## Solution Approaches

### Approach 1: Widget Extraction (Recommended for Quick Win)

**Idea:** Extract just ChatWidget's rendering logic, not the full App.

**Steps:**
1. Create a simplified ChatWidget that only renders (no event handling)
2. Feed it dummy data or static state
3. Render it in DX's message area
4. Gradually add more functionality

**Pros:**
- Faster to implement
- Less architectural conflict
- Can iterate incrementally

**Cons:**
- Not the "real" Codex TUI
- Missing features (agent execution, etc.)
- Requires maintaining parallel implementation

### Approach 2: Subprocess Integration (Cleanest Separation)

**Idea:** Run Codex TUI as a separate process, capture its output.

**Steps:**
1. Spawn `codex-tui` as subprocess
2. Use PTY (pseudo-terminal) to capture its output
3. Render captured output in DX's message area
4. Forward input to subprocess

**Pros:**
- Complete isolation (no conflicts)
- Get 100% real Codex TUI
- Clean architecture

**Cons:**
- More complex IPC
- Performance overhead
- Harder to debug

### Approach 3: Full Integration (Most Complex, Most Powerful)

**Idea:** Merge DX and Codex event loops, share terminal management.

**Steps:**
1. Make DX's event loop handle both Yazi and Codex events
2. Create unified terminal manager
3. Route events to appropriate handler (Yazi or Codex)
4. Coordinate rendering between both

**Pros:**
- True integration
- Full feature parity
- Best user experience

**Cons:**
- Significant refactoring required
- High complexity
- Long development time

## Recommended Implementation Plan

### Phase 1: Minimal Viable Integration (2-4 hours)

**Goal:** Show Codex ChatWidget rendering (static/dummy data)

1. **Create `CodexWidgetAdapter`** (`codex-rs/dx/src/codex_widget_adapter.rs`)
   ```rust
   pub struct CodexWidgetAdapter {
       chat_widget: ChatWidget,
       // Minimal dependencies
   }
   
   impl CodexWidgetAdapter {
       pub fn new() -> Self {
           // Create ChatWidget with dummy/minimal state
       }
       
       pub fn render(&self, area: Rect, buf: &mut Buffer) {
           // Call ChatWidget's render logic
           self.chat_widget.render(area, buf);
       }
   }
   ```

2. **Update `state.rs`**
   ```rust
   pub struct ChatState {
       // ... existing fields ...
       pub codex_widget: Option<CodexWidgetAdapter>,
   }
   ```

3. **Update `dx_render.rs`**
   ```rust
   if self.show_codex_tui {
       if let Some(widget) = &self.codex_widget {
           widget.render(chunks[0], buf);
       }
   }
   ```

**Result:** Codex UI visible (even if not functional yet)

### Phase 2: Event Forwarding (2-4 hours)

**Goal:** Make Codex UI interactive

1. **Create event adapter**
   ```rust
   fn map_dx_event_to_codex(key: KeyEvent) -> AppEvent {
       AppEvent::Key(key)
   }
   ```

2. **Forward events in `dispatcher.rs`**
   ```rust
   if self.app.bridge.chat_state.show_codex_tui {
       let codex_event = map_dx_event_to_codex(key);
       self.app.bridge.chat_state.codex_widget.handle_event(codex_event);
   }
   ```

**Result:** Can type in Codex UI, navigate, etc.

### Phase 3: Agent Integration (4-8 hours)

**Goal:** Connect to real Codex agent

1. **Initialize ThreadManager**
2. **Connect ChatWidget to ThreadManager**
3. **Handle async events from agent**
4. **Update UI based on agent responses**

**Result:** Full Codex functionality

### Phase 4: Polish (2-4 hours)

**Goal:** Smooth UX

1. **Smooth transitions between Ollama and Codex**
2. **Proper initialization/cleanup**
3. **Error handling**
4. **Performance optimization**

**Result:** Production-ready integration

## Technical Details for Implementation

### Key Files to Modify

1. **`codex-rs/dx/src/codex_widget_adapter.rs`** (NEW)
   - Wraps ChatWidget for use in DX
   - Handles initialization with minimal dependencies
   - Provides simple `render()` and `handle_event()` methods

2. **`codex-rs/dx/src/state.rs`**
   - Add `codex_widget: Option<CodexWidgetAdapter>`
   - Add initialization method
   - Add toggle method

3. **`codex-rs/dx/src/dx_render.rs`**
   - Call `codex_widget.render()` when `show_codex_tui == true`

4. **`codex-rs/dx/src/dispatcher.rs`**
   - Forward events to `codex_widget.handle_event()` when active

### Critical Code Snippets

#### ChatWidget Initialization (Minimal)

```rust
// From chatwidget.rs
pub struct ChatWidgetInit {
    pub config: Config,
    pub frame_requester: FrameRequester,
    pub app_event_tx: AppEventSender,
    pub initial_user_message: Option<UserMessage>,
    pub enhanced_keys_supported: bool,
    pub auth_manager: Arc<AuthManager>,
    pub models_manager: Arc<ModelsManager>,
    pub feedback: CodexFeedback,
    pub is_first_run: bool,
    pub feedback_audience: FeedbackAudience,
    pub model: Option<String>,
    pub startup_tooltip_override: Option<String>,
    pub status_line_invalid_items_warned: Arc<AtomicBool>,
    pub terminal_title_invalid_items_warned: Arc<AtomicBool>,
    pub session_telemetry: SessionTelemetry,
}

impl ChatWidget {
    pub async fn new(
        init: ChatWidgetInit,
        server: Arc<ThreadManager>,
    ) -> Result<Self> {
        // ... initialization logic ...
    }
}
```

#### ChatWidget Rendering

```rust
// ChatWidget doesn't have a public render() method
// It's rendered via App::draw() which is called from tui.draw()

// We need to either:
// 1. Make ChatWidget's render public
// 2. Create a wrapper that exposes rendering
// 3. Use App::draw() somehow
```

#### Event Handling

```rust
// From app.rs
impl App {
    fn handle_tui_event(&mut self, event: TuiEvent, tui: &mut Tui) -> Result<()> {
        match event {
            TuiEvent::Key(key) => {
                // Forward to ChatWidget via bottom_pane
                let result = self.chat_widget.bottom_pane.handle_key(key);
                // ... handle result ...
            }
            // ... other events ...
        }
    }
}
```

## Specific Questions for Better AI

1. **How to extract ChatWidget rendering without full App?**
   - ChatWidget's render logic is private
   - It's called from App::draw()
   - Need to either make it public or create adapter

2. **How to handle ChatWidget's dependencies?**
   - Needs ThreadManager, AuthManager, ModelsManager, etc.
   - Can we create "dummy" versions for rendering-only?
   - Or do we need full initialization?

3. **How to bridge event systems?**
   - DX uses crossterm events directly
   - Codex uses AppEvent enum
   - Need clean mapping between them

4. **How to handle async events?**
   - Codex uses tokio channels for async events
   - DX's event loop is synchronous (Yazi-based)
   - Need to poll Codex's event channel somehow

5. **How to share terminal management?**
   - Both need raw mode, alternate screen, etc.
   - Can they coexist?
   - Or does one need to "own" the terminal?

## Next Steps

1. **Decide on approach** (Widget Extraction recommended)
2. **Create `CodexWidgetAdapter`** with minimal ChatWidget
3. **Get rendering working** (even with dummy data)
4. **Add event forwarding**
5. **Connect to real agent** (if needed)

## Files to Reference

- `codex-rs/tui/src/lib.rs` - Main entry point, initialization
- `codex-rs/tui/src/app.rs` - App struct, event loop, rendering
- `codex-rs/tui/src/chatwidget.rs` - ChatWidget implementation
- `codex-rs/tui/src/tui.rs` - Terminal management
- `codex-rs/dx/src/dispatcher.rs` - DX event routing
- `codex-rs/dx/src/dx_render.rs` - DX rendering
- `codex-rs/dx/src/state.rs` - DX state management

## Conclusion

The integration is complex but achievable. The key is to start small (just rendering) and incrementally add functionality. The Widget Extraction approach is recommended for fastest results.

A better AI should focus on:
1. Creating `CodexWidgetAdapter` that wraps ChatWidget
2. Exposing ChatWidget's render logic
3. Handling minimal dependencies for rendering
4. Event forwarding from DX to Codex
5. Async event handling (if needed)

The goal is to see Codex UI rendering in DX's message area, then make it interactive, then connect to the real agent.
