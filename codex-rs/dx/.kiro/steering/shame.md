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

- Added `dx_core: RefCell<fb_core::Core>` to ChatWidget
- Added `dx_bridge: RefCell<YaziChatBridge>` to ChatWidget
- Created `ChatWidget::make_dx_core()` helper function
- Initialize Core with empty files (Yazi shows empty dir initially)
- Root widget takes direct ChatState reference from ChatWidget
- App.rs ALWAYS renders ChatWidget (which checks animation_mode)
- ChatWidget renders Root widget in transcript area when animation_mode=true
- Made Panic module public
- Hardcoded keys: '1' shows Matrix, '3' shows Yazi

## Current Status (2026-03-29 16:00)

- Code compiles successfully ✅
- Core initialization simplified (no async file loading)
- Root widget accesses correct ChatState
- Ready for testing with `cargo run`
- Press '1' to show Matrix animation
- Press '3' to show Yazi file browser (will show empty initially)
- Menu works with '0' key

## Known Issues

- Yazi shows empty directory (no files loaded)
  - Files::from_dir_bulk() is async, can't call in sync constructor
  - Would need DX actor system running to load files properly
  - For now, Yazi renders but shows no files

## Integration Points

- `src/chatwidget.rs` - Has dx_core, dx_bridge, make_dx_core() helper
- `src/app.rs` - Always renders ChatWidget
- `src/root.rs` - Root widget takes chat_state parameter
- `src/state.rs` - ChatState with animation state
