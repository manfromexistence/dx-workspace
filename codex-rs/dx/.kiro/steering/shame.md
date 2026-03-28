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
- Initialize DX subsystems in async context (src/codex.rs)
- App.rs checks `animation_mode` and uses Root widget when true
- Root widget handles ALL DX rendering (animations + Yazi)
- Made Panic module public
- Hardcoded keys: '1' shows Matrix, '3' shows Yazi

## Current Status

- DX initialization works (in async context)
- Root widget renders animations and Yazi
- Press '1' to show Matrix animation
- Press '3' to show Yazi file browser
- Menu works with '0' key

## Integration Points

- `src/codex.rs` - DX initialization in async block
- `src/app.rs` - Conditionally uses Root widget for animations
- `src/chatwidget.rs` - Has dx_core and dx_bridge fields
- `src/root.rs` - DX Root widget (unchanged, direct DX code)
- `src/state.rs` - ChatState with animation state

