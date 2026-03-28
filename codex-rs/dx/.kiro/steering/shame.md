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

- Added `handle_menu_key()` method to ChatState (single source of truth)
- Both DX dispatcher and ChatWidget call the same method
- Menu renders with `render_menu_in_area()` for animations
- Removed dx_dispatcher_bridge.rs (useless wrapper)

## Current Status

- Menu shows with '0' key
- Navigation works: Up/Down, j/k, PageUp/PageDown, Home/End, Enter, Esc
- Opening animation needs fixing (only closing animation shows)

## Integration Points

- `src/state.rs` - ChatState::handle_menu_key() handles all menu keys
- `src/chatwidget.rs` - Calls handle_menu_key(), renders with render_menu_in_area()
- `src/dispatcher.rs` - Original DX code unchanged, calls handle_menu_key()

