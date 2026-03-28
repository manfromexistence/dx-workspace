# DX TUI Integration Plan: Replace Codex TUI Rendering

## 🎯 Strategy: Surgical Replacement

Instead of reimplementing features, we'll:
1. Keep Codex TUI's `ChatWidget` (has ALL the logic)
2. Remove Codex TUI's ratatui rendering code
3. Add DX's rendering methods to `ChatWidget`
4. Use DX's menu system for popups/dialogs

---

## 📋 Step-by-Step Plan

### Phase 1: Understand the Structure

**Codex TUI Architecture:**
```
codex-rs/tui/src/
├── app.rs              # Main app loop (KEEP - modify event loop)
├── chatwidget.rs       # ChatWidget with ALL logic (KEEP - replace render)
├── bottom_pane/        # Popups, menus (REPLACE with DX menu)
├── render/             # Rendering code (DELETE)
├── app_event.rs        # Event types (KEEP)
├── slash_command.rs    # Slash commands (KEEP)
└── ...
```

**What to KEEP:**
- `ChatWidget` struct and ALL its methods (event handlers, state management)
- `app_event.rs` - Event types
- `slash_command.rs` - Command definitions
- `agent.rs` - Agent spawning
- `skills.rs`, `plugins.rs` - Backend logic
- ALL event handling methods

**What to REPLACE:**
- `render()` methods - Replace with DX rendering
- `bottom_pane/` - Replace with DX menu system
- Popup rendering - Use DX menu
- Status line rendering - Use DX status bar

**What to DELETE:**
- Old ratatui widget code
- Layout calculations (DX has its own)
- Color/theme rendering (DX has better themes)

---

### Phase 2: Integration Steps

#### Step 1: Copy ChatWidget to DX
```bash
# Copy the core ChatWidget
cp codex-rs/tui/src/chatwidget.rs codex-rs/dx/src/chatwidget_full.rs
cp codex-rs/tui/src/chatwidget/*.rs codex-rs/dx/src/chatwidget/
```

#### Step 2: Remove Rendering Code
In `chatwidget_full.rs`:
- Remove `impl Widget for ChatWidget`
- Remove `render()` method
- Remove all ratatui layout code
- Keep ALL event handlers
- Keep ALL state management
- Keep ALL slash command handling

#### Step 3: Add DX Rendering
Add new methods to `ChatWidget`:
```rust
impl ChatWidget {
    // Keep all existing methods...
    
    // NEW: DX-style rendering
    pub fn render_dx(&self, area: Rect, buf: &mut Buffer, theme: &ChatTheme) {
        // Use DX's rendering style
        // Render messages with DX's message list
        // Use DX's animations
    }
    
    // NEW: Get data for DX menu
    pub fn get_menu_items(&self) -> Vec<MenuItem> {
        // Convert bottom_pane items to DX menu items
    }
}
```

#### Step 4: Replace Popups with DX Menu
When Codex TUI shows a popup:
```rust
// OLD (Codex TUI):
self.bottom_pane.show_selection_view(params);

// NEW (DX):
self.dx_menu.show_items(items);
```

#### Step 5: Wire Up Event Loop
In `codex-rs/dx/src/state.rs`:
```rust
pub struct ChatState {
    // Replace our simple state with full ChatWidget
    pub chat_widget: ChatWidget,  // Has ALL Codex TUI logic!
    
    // Keep DX-specific stuff
    pub menu: Menu,
    pub theme: ChatTheme,
    pub animations: AnimationState,
    // ...
}

impl ChatState {
    pub fn handle_codex_event(&mut self, event: Event) {
        // Just forward to ChatWidget!
        self.chat_widget.handle_codex_event(event);
    }
    
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // Use ChatWidget's data with DX's rendering
        self.chat_widget.render_dx(area, buf, &self.theme);
    }
}
```

---

### Phase 3: Specific Replacements

#### Approval Dialogs
**Codex TUI:**
```rust
// Shows approval popup
self.bottom_pane.show_approval_popup(request);
```

**DX:**
```rust
// Show in DX menu
self.menu.show_approval_dialog(request);
```

#### Skills List
**Codex TUI:**
```rust
// Shows skills in bottom pane
self.bottom_pane.show_skills_view(skills);
```

**DX:**
```rust
// Show in DX menu
self.menu.show_skills_list(skills);
```

#### Plugin Marketplace
**Codex TUI:**
```rust
// Shows plugins in bottom pane
self.bottom_pane.show_plugins_view(plugins);
```

**DX:**
```rust
// Show in DX menu
self.menu.show_plugins_marketplace(plugins);
```

---

### Phase 4: File Structure

**New DX Structure:**
```
codex-rs/dx/src/
├── dx.rs                    # Main entry
├── state.rs                 # ChatState with ChatWidget
├── chatwidget_full.rs       # Full ChatWidget (from Codex TUI)
├── chatwidget/              # ChatWidget modules
│   ├── skills.rs           # Skills logic (from Codex TUI)
│   ├── plugins.rs          # Plugins logic (from Codex TUI)
│   ├── agent.rs            # Agent logic (from Codex TUI)
│   └── ...
├── menu/                    # DX menu system (KEEP)
├── animations/              # DX animations (KEEP)
├── theme/                   # DX themes (KEEP)
└── render_dx.rs            # DX rendering for ChatWidget
```

---

### Phase 5: Benefits

**What we get:**
- ✅ ALL 50+ event types handled
- ✅ ALL slash commands working
- ✅ ALL approval logic
- ✅ ALL skills logic
- ✅ ALL plugins logic
- ✅ ALL MCP integration
- ✅ ALL session management
- ✅ Plan mode, collaboration mode, review mode
- ✅ Reasoning display, image support, web search
- ✅ Everything Codex TUI has

**What we keep:**
- ✅ DX's superior UI
- ✅ DX's animations
- ✅ DX's file browser (Yazi)
- ✅ DX's themes
- ✅ DX's menu system
- ✅ DX's audio system

**Estimated time:** 10-20 hours (vs 200+ hours reimplementing)

---

## 🚀 Implementation Order

### Week 1: Core Integration
1. Copy ChatWidget to DX
2. Remove rendering code
3. Add DX rendering methods
4. Wire up event loop
5. Test basic chat

### Week 2: Menu Integration
1. Replace approval popups with DX menu
2. Replace skills UI with DX menu
3. Replace plugins UI with DX menu
4. Replace MCP UI with DX menu

### Week 3: Polish
1. Add slash command autocomplete
2. Add status bar integration
3. Add session management UI
4. Test all features

---

## 📝 Key Files to Modify

### Copy from Codex TUI:
- `codex-rs/tui/src/chatwidget.rs` → `codex-rs/dx/src/chatwidget_full.rs`
- `codex-rs/tui/src/chatwidget/*.rs` → `codex-rs/dx/src/chatwidget/`
- `codex-rs/tui/src/app_event.rs` → `codex-rs/dx/src/app_event.rs`
- `codex-rs/tui/src/slash_command.rs` → `codex-rs/dx/src/slash_command.rs`

### Modify in DX:
- `codex-rs/dx/src/state.rs` - Replace with ChatWidget
- `codex-rs/dx/src/dispatcher.rs` - Forward events to ChatWidget
- `codex-rs/dx/src/menu/` - Add approval/skills/plugins handlers

### Delete from copied code:
- All `impl Widget` blocks
- All `render()` methods
- All ratatui layout code
- All `bottom_pane` rendering

---

## 🎯 Success Criteria

When done, DX TUI will:
- ✅ Handle ALL Codex events
- ✅ Support ALL slash commands
- ✅ Have approval dialogs (DX style)
- ✅ Have skills management (DX style)
- ✅ Have plugin marketplace (DX style)
- ✅ Have MCP integration
- ✅ Have session management
- ✅ Keep DX's superior UI/UX
- ✅ Be a complete Codex replacement

---

This is the smart way to do it! We get 100% feature parity in a fraction of the time.
