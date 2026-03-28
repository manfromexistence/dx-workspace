# Unified TUI Plan - The REAL Architecture

**Date**: March 25, 2026  
**Goal**: Merge dx + Codex into ONE unified TUI with shared components

---

## 🎯 WHAT YOU'RE ACTUALLY DOING

**NOT**: Two separate TUIs running side-by-side  
**YES**: ONE TUI with components from both codebases

```
Unified TUI
├── main.rs (single entry point)
├── event_loop (single loop)
├── terminal (single ownership)
└── components/
    ├── dx_file_browser (30% screen)
    ├── dx_animations
    ├── codex_chat (70% screen)
    ├── codex_markdown_render
    └── codex_history_cells
```

---

## ✅ WHY THIS ACTUALLY WORKS

### You're Right About:

1. **"We can change Codex code"** - YES! You're not using it as a crate, you're merging source
2. **"Render like components"** - YES! Codex's ChatWidget becomes just another component
3. **"No way to distinguish"** - YES! It's all one codebase after merge
4. **"Not two separate things"** - YES! One app, one event loop, one terminal

### The Architecture:

```rust
// ONE unified main.rs
fn main() {
    let mut terminal = Terminal::new(...)?;  // ONE terminal
    let mut app = UnifiedApp::new();         // ONE app
    
    loop {
        let event = crossterm::event::read()?;  // ONE event loop
        app.handle_event(event);                // Routes to components
        
        terminal.draw(|f| {
            let layout = Layout::horizontal([
                Constraint::Percentage(30),  // dx components
                Constraint::Percentage(70),  // codex components
            ]);
            
            app.render_dx(layout[0], f.buffer_mut());
            app.render_codex(layout[1], f.buffer_mut());
        })?;
    }
}
```

---

## 📋 THE ACTUAL MERGE PLAN

### Step 1: Choose Host Directory
**Option A**: Merge Codex into dx
```
codex-rs/dx/
├── src/
│   ├── main.rs (dx entry)
│   ├── dx/ (your code)
│   └── codex/ (copied from tui/src/)
└── Cargo.toml (merged deps)
```

**Option B**: Merge dx into Codex
```
codex-rs/tui/
├── src/
│   ├── main.rs (codex entry, modified)
│   ├── dx/ (copied from dx/src/)
│   └── ... (existing codex code)
└── Cargo.toml (merged deps)
```

**Recommendation**: Option B (merge into Codex) because:
- Codex has more infrastructure (auth, config, etc.)
- Easier to add dx components than rebuild Codex infrastructure

---

### Step 2: Copy Source Files

```bash
# Copy dx code into Codex TUI
cp -r codex-rs/dx/src/* codex-rs/tui/src/dx/

# Files copied:
# - animations.rs
# - chat.rs (your local LLM)
# - chat_components.rs
# - file_browser/ (Yazi)
# - llm.rs
# - menu/
# - theme.rs
# - ... all dx files
```

---

### Step 3: Merge Cargo.toml

```toml
# codex-rs/tui/Cargo.toml

[dependencies]
# Existing Codex deps (50+)
codex-core = { workspace = true }
codex-protocol = { workspace = true }
# ... all existing

# NEW: dx-specific deps
llama-cpp-2 = "0.1"
tiktoken-rs = "0.9"
palette = "0.7"
cli-clipboard = "0.4"
figlet-rs = "1.0.0"
mlua = { workspace = true }
tachyonfx = { workspace = true }
```

---

### Step 4: Refactor Codex Components

**Current Codex structure:**
```rust
// tui/src/app.rs
pub struct App {
    // Owns terminal, event loop, everything
}

impl App {
    pub async fn run(...) {
        // Takes over completely
    }
}
```

**Refactored structure:**
```rust
// tui/src/codex/mod.rs
pub struct CodexChatComponent {
    // Just the chat rendering, no terminal ownership
    widget: ChatWidget,
    state: ChatState,
}

impl CodexChatComponent {
    pub fn new(...) -> Self { ... }
    
    pub fn handle_event(&mut self, event: Event) { ... }
    
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // Render in provided area
        self.widget.render(area, buf);
    }
}
```

---

### Step 5: Create Unified App

```rust
// tui/src/unified_app.rs
pub struct UnifiedApp {
    // dx components
    dx_file_browser: DxFileBrowser,
    dx_animations: DxAnimations,
    dx_local_llm: LocalLlm,
    
    // Codex components
    codex_chat: CodexChatComponent,
    codex_config: Config,
    codex_auth: AuthManager,
    
    // Shared state
    focus: Focus,
    layout: LayoutMode,
}

enum Focus {
    DxFileBrowser,
    CodexChat,
}

impl UnifiedApp {
    pub fn handle_event(&mut self, event: Event) {
        match self.focus {
            Focus::DxFileBrowser => {
                self.dx_file_browser.handle_event(event);
            }
            Focus::CodexChat => {
                self.codex_chat.handle_event(event);
            }
        }
    }
    
    pub fn render(&self, frame: &mut Frame) {
        let layout = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ]).split(frame.size());
        
        // Render dx side
        self.dx_file_browser.render(layout[0], frame.buffer_mut());
        
        // Render codex side
        self.codex_chat.render(layout[1], frame.buffer_mut());
    }
}
```

---

### Step 6: Update Main Entry Point

```rust
// tui/src/main.rs
mod dx;
mod codex;
mod unified_app;

use unified_app::UnifiedApp;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize terminal (ONE terminal)
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    
    // Create unified app
    let mut app = UnifiedApp::new().await?;
    
    // Single event loop
    loop {
        terminal.draw(|f| app.render(f))?;
        
        if let Event::Key(key) = crossterm::event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
            app.handle_event(Event::Key(key));
        }
    }
    
    Ok(())
}
```

---

## 🔥 BRUTAL TRUTH: WHAT'S ACTUALLY HARD

### Easy Parts (30%):
- ✅ Copying files
- ✅ Merging Cargo.toml
- ✅ Creating layout split

### Hard Parts (70%):
- ⚠️ **Refactoring Codex's App** - It's designed to own everything
- ⚠️ **State management** - Who owns config, auth, threads?
- ⚠️ **Event routing** - Complex logic for focus switching
- ⚠️ **Initialization** - Codex has complex startup (auth, config, etc.)
- ⚠️ **Async coordination** - Codex uses tokio heavily

---

## ⏱️ REALISTIC TIME ESTIMATE

### Phase 1: File Merge (1 day)
- Copy dx files to tui/src/dx/
- Merge Cargo.toml
- Fix import paths
- Get it compiling

### Phase 2: Component Extraction (2-3 days)
- Extract Codex ChatWidget as component
- Remove terminal ownership from Codex
- Make Codex rendering passive
- Create component interfaces

### Phase 3: Unified App (2-3 days)
- Create UnifiedApp struct
- Implement event routing
- Implement focus management
- Wire up both sides

### Phase 4: Integration (2-3 days)
- Handle state sharing
- Fix async issues
- Polish transitions
- Test everything

**Total: 1-2 weeks**

---

## 💡 WHAT MAKES THIS FEASIBLE

You're right that this IS doable because:

1. **Same framework** - Both use Ratatui
2. **Source access** - You can modify Codex code
3. **Component-based** - Ratatui is naturally component-based
4. **One codebase** - No inter-process communication needed

The key insight: **Codex's ChatWidget is already a component**, it just needs to be extracted from the App wrapper.

---

## 🎯 RECOMMENDED APPROACH

### Week 1: Proof of Concept
1. Copy files
2. Get basic split-screen working
3. dx on left (static), Codex on right (static)
4. Prove the layout works

### Week 2: Make It Interactive
1. Event routing
2. Focus switching
3. Full integration
4. Polish

---

## ✅ MY HONEST ASSESSMENT

**Difficulty**: 6/10 (Medium)  
**Time**: 1-2 weeks  
**Feasibility**: HIGH (this actually works!)

**Why it works:**
- You're not fighting two separate apps
- You're building ONE app with components from both
- Ratatui is designed for this
- You have source access to modify anything

**The hard part:**
- Extracting Codex's components from its App wrapper
- Managing shared state (config, auth, etc.)
- Event routing logic

**But it's totally doable.**

---

## 🚀 READY TO START?

If you want to proceed, here's the first step:

```bash
# 1. Create dx module in tui
mkdir -p codex-rs/tui/src/dx

# 2. Copy all dx source files
cp -r codex-rs/dx/src/*.rs codex-rs/tui/src/dx/
cp -r codex-rs/dx/src/file_browser codex-rs/tui/src/dx/
cp -r codex-rs/dx/src/menu codex-rs/tui/src/dx/

# 3. Create module file
# (I'll generate this)
```

**Should I execute this plan?**

This time I actually understand what you're doing, and it DOES make sense.
