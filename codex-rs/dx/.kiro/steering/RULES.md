---
inclusion: always
---

# DX-TUI Integration Rules

## CRITICAL - READ FIRST

1. **ONLY `cargo run`** - Never cargo check/build/test
2. **NO BRIDGES/WRAPPERS** - Call DX code directly
3. **BREAK DX TO MERGE** - Don't keep DX and codex separate
4. **ONE SOURCE OF TRUTH** - Put shared logic in ChatState methods
5. **TEST AFTER EVERY CHANGE** - Run `cargo run` immediately

## Integration Strategy

- DX code lives in `src/` (state.rs, dispatcher.rs, menu/, etc.)
- ChatWidget calls DX methods directly (no bridges)
- Shared logic goes in ChatState methods (like handle_menu_key)
- Both DX dispatcher and ChatWidget call the same ChatState methods

## Current Architecture

```
ChatWidget (codex-tui)
    ↓ calls
ChatState::handle_menu_key() ← SINGLE SOURCE
    ↑ calls
DX Dispatcher (dx-tui)
```

## Files

- `src/state.rs` - ChatState with all DX state + shared methods
- `src/chatwidget.rs` - Codex chat widget, calls ChatState methods
- `src/dispatcher.rs` - DX event dispatcher, calls ChatState methods
- `src/menu/` - DX menu system
- `src/splash.rs` - DX splash rendering
- `src/audio.rs` - DX audio system

## What NOT to Do

- ❌ Create bridge files (dx_dispatcher_bridge.rs)
- ❌ Duplicate code from dispatcher.rs
- ❌ Create wrapper functions
- ❌ Run cargo check/build/test
- ❌ Say "you're right" without fixing it

## What TO Do

- ✅ Add methods to ChatState for shared logic
- ✅ Call ChatState methods from both DX and codex
- ✅ Run `cargo run` after every change
- ✅ Update TODO.md, CHANGELOG.md, shame.md
- ✅ Fix issues immediately, test immediately

