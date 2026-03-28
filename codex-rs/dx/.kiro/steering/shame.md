---
inclusion: always
---

# Agent Shame Protocol

## Core Principles

1. **ALWAYS RUN CARGO RUN** - Not cargo check, not cargo build, not cargo test. Just `cargo run --bin codex-tui-dx`
2. **NO GREP/FILTERING** - Run the full command output, don't filter with Select-String or grep
3. **TEST IMMEDIATELY** - After every meaningful change, run cargo run to verify it works
4. **USE REAL DX CODE** - Never create wrappers or duplicate code. Use the actual dx-tui functions directly
5. **NO AI SLOP** - Don't create unnecessary files, wrappers, or abstractions. Call the real code.

## What I Did Wrong

- Used Select-String to filter cargo output when user explicitly said not to
- Didn't run cargo run immediately after making changes
- Created wrapper files instead of using real DX code directly
- Made changes without verifying they work

## What I Should Do

- Run `cargo run --bin codex-tui-dx` after EVERY change
- Use actual dx-tui functions like `crate::splash::render()` directly
- Test that features actually work (like font cycling with Ctrl+.)
- Update CHANGELOG.md and TODO.md after every meaningful change
- No excuses, no "you're right" - just fix it and test it

## Current Issue

Font changing with Ctrl+. is not working. Rainbow animation works but fonts don't cycle.
Need to debug why dx_chat_state.splash_font_index isn't being used correctly.
