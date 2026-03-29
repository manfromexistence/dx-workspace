---
inclusion: always
---

# Agent Shame Protocol

## ABSOLUTE RULES - NO EXCEPTIONS

1. **ONLY `cargo run`** - NEVER cargo check/build/test
2. **NO BRIDGES** - Call DX code directly, don't create wrapper files
3. **NO DUPLICATION** - If code exists in DX, use it. Don't copy-paste.
4. **BREAK DX IF NEEDED** - Merge DX into codex-tui, don't keep them separate
5. **TEST IMMEDIATELY** - Run `cargo run` after EVERY change
6. **UPDATE FILES** - Update TODO.md, CHANGELOG.md, shame.md after every task

## What Worked

- Added `dx_core: Arc<Mutex<fb_core::Core>>` to ChatWidget (changed from RefCell)
- Added `dx_bridge: RefCell<YaziChatBridge>` to ChatWidget
- Created `ChatWidget::make_dx_core()` helper function
- Root widget takes direct ChatState reference from ChatWidget
- App.rs ALWAYS renders ChatWidget (which checks animation_mode)
- ChatWidget renders Root widget in transcript area when animation_mode=true
- Wrapped Root rendering with Lives::scope() and runtime_scope!() for Lua context
- Made Panic module public
- Hardcoded keys: '1' shows Matrix, '3' shows Yazi

## Current Status (2026-03-29 16:30)

- Code compiles successfully ✅
- Yazi UI renders correctly ✅
- Shows "Loading... Top 0/0" (Lua Root component working!)
- Press '1' to show Matrix animation
- Press '3' to show Yazi file browser
- Menu works with '0' key

## Known Issues

- Yazi shows "Loading..." but no files appear
  - Core is not Send, can't use tokio::spawn
  - spawn_local requires LocalSet which we don't have
  - Would need full DX actor system to load files properly
  - Yazi UI renders correctly, just shows empty directory

## Integration Points

- `src/chatwidget.rs` - Has dx_core (Arc<Mutex>), dx_bridge, make_dx_core() helper
- `src/app.rs` - Always renders ChatWidget
- `src/root.rs` - Root widget takes chat_state parameter, wrapped in Lives::scope
- `src/state.rs` - ChatState with animation state
