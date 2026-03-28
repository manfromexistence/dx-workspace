---
inclusion: always
---

# DX-TUI Integration Rules & Guidelines

## 🎯 Core Principles

### 1. Use Real DX Code - NO AI SLOP
- **ALWAYS** use existing DX-TUI code directly
- **NEVER** create wrapper functions or duplicate logic
- **EDIT** existing DX files to fit codex-tui context
- **CALL** real DX functions like `crate::splash::render()`, `ChatState::play_animation_sound()`

### 2. One File at a Time
- Integrate **COMPLETE FILES**, not individual functions
- Finish one file fully before moving to next
- Test after each file integration

### 3. Testing Protocol
- **ONLY** run `cargo run --bin codex-tui-dx`
- **NEVER** run `cargo check`, `cargo build`, or `cargo test`
- Test immediately after every change
- Fix errors before moving forward

### 4. Documentation
- Update `TODO.md` after every task
- Update `CHANGELOG.md` with changes
- Mark completed tasks with ✅ and timestamp
- Track blocked/failed tasks

---

## 📁 File Integration Strategy

### Current Integration Points

#### ChatWidget (`src/chatwidget.rs`)
- Main integration point for DX features
- Uses `dx_chat_state: RefCell<ChatState>` for all DX state
- Calls `DxDispatcherBridge::dispatch_timer()` for updates
- Renders DX splash with `crate::splash::render()`

#### DX Dispatcher Bridge (`src/dx_dispatcher_bridge.rs`)
- Wraps DX dispatcher timer logic
- Provides `dispatch_timer()` method
- NO duplication - uses real dispatcher code

#### DX Modules (Already Exposed)
- `src/state.rs` - ChatState
- `src/dispatcher.rs` - Event handling
- `src/splash.rs` - Splash rendering
- `src/audio.rs` - Audio system
- `src/menu/` - Menu system
- `src/animations.rs` - Animations
- `src/theme.rs` - Theme system

---

## 🚫 What NOT to Do

### ❌ Don't Create Wrappers
```rust
// BAD - Creating wrapper
fn my_splash_render() {
    // duplicate DX logic
}

// GOOD - Use real DX code
crate::splash::render(area, buf, &theme, font_index, &rainbow);
```

### ❌ Don't Duplicate Logic
```rust
// BAD - Duplicating font cycling
if elapsed >= 5 seconds {
    font_index = (font_index + 1) % 113;
}

// GOOD - Use DX dispatcher
DxDispatcherBridge::dispatch_timer(&mut dx_chat_state);
```

### ❌ Don't Run Wrong Commands
```bash
# BAD
cargo check
cargo build
cargo test

# GOOD
cargo run --bin codex-tui-dx
```

---

## ✅ What TO Do

### ✅ Use Real DX Functions
```rust
// Call real DX methods
dx_chat_state.play_animation_sound();
dx_chat_state.play_ui_sound("assets/click.mp3");
dx_chat_state.stop_animation_sound();
```

### ✅ Edit DX Files When Needed
```rust
// If DX code needs adaptation, edit the DX file directly
// Example: Comment out codex-specific code in dispatcher.rs
// COMMENTED OUT: Codex TUI integration
// if self.app.bridge.chat_state.show_codex_tui { ... }
```

### ✅ Test Immediately
```bash
# After every change
cargo run --bin codex-tui-dx
```

---

## 📋 Integration Checklist

Before integrating a new feature:
- [ ] Read the complete DX file (don't truncate)
- [ ] Understand what it does
- [ ] Identify integration points in codex-tui
- [ ] Plan how to call DX code (don't duplicate)
- [ ] Make changes
- [ ] Update TODO.md
- [ ] Update CHANGELOG.md
- [ ] Test with `cargo run --bin codex-tui-dx`
- [ ] Fix any errors
- [ ] Mark task as complete

---

## 🎯 Current Focus

**Audio Integration**
- Play splash screen sound when showing DX splash
- Use `dx_chat_state.play_animation_sound()`
- Stop sound when leaving splash
- Test sound playback

---

## 📝 File Modification Guidelines

### When to Edit DX Files
- To comment out codex-specific integration code
- To adapt file paths or module references
- To fix compilation errors

### When to Edit Codex Files
- To call DX functions
- To integrate DX state
- To route events to DX

### Never
- Create new files that duplicate DX functionality
- Write wrapper functions around DX code
- Copy-paste DX code into codex files

---

## 🔄 Update Protocol

After every change:
1. Test with `cargo run --bin codex-tui-dx`
2. Update TODO.md (mark completed, update in-progress)
3. Update CHANGELOG.md (document what changed)
4. Commit changes with clear message

---

## 🎨 Code Style

- Use existing DX code style
- Keep DX architecture intact
- Maintain separation: DX handles DX features, Codex handles Codex features
- Bridge between them cleanly (like `dx_dispatcher_bridge.rs`)

---

## 🚀 Integration Priority

1. **Audio** - Sounds for splash, animations, UI
2. **Menu** - Press '0' to open, all submenus
3. **Animations** - Carousel with Left/Right navigation
4. **File Browser** - Yazi integration
5. **Event Routing** - Full keyboard/mouse handling
6. **Voice Mode** - Space key hold
7. **Chat Features** - Complete DX chat rendering

---

## 📞 When to Ask for Clarification

- When DX code conflicts with Codex architecture
- When multiple integration approaches are possible
- When unsure which DX file to integrate next
- When encountering complex dependencies

---

## ✨ Success Criteria

A feature is successfully integrated when:
- ✅ Uses real DX code (no duplication)
- ✅ Compiles without errors
- ✅ Works correctly in `cargo run --bin codex-tui-dx`
- ✅ TODO.md and CHANGELOG.md are updated
- ✅ No AI slop created
