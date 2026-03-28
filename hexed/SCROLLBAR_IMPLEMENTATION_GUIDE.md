# Scrollbar Implementation Guide for Codex TUI Fullscreen Mode

**Date:** March 27, 2026  
**Repository:** https://github.com/codex-rs/codex-rs  
**Target:** `codex-tui-dx` binary in `codex-rs/dx/` folder

## Problem Statement

The Codex TUI has two display modes:
1. **Inline mode** - Has native terminal scrollbar (works fine)
2. **Fullscreen/all-screen mode** - Auto-scrolls to latest content with NO manual scroll capability

Currently, in fullscreen mode, users cannot scroll back to view previous messages. The UI automatically jumps to the latest content with no way to manually navigate the history.

## Goal

Implement a custom scrollbar on the right side of the Codex UI in fullscreen mode that allows users to:
- Scroll up/down through message history
- See their current position in the content
- Use keyboard controls (PageUp/PageDown, Ctrl+Home/End, Ctrl+Up/Down)

## Current State (March 27, 2026)

### What Exists
- Custom scrollbar widget already implemented in `codex-rs/dx/src/scrollbar.rs`
- Scrollbar state tracking with `ScrollbarState` struct
- Visual rendering of scrollbar track and thumb

### What Was Attempted
1. Added scroll state fields to `ChatWidget` struct:
   - `scroll_offset: std::cell::Cell<usize>`
   - `content_height: std::cell::Cell<usize>`
   - `viewport_height: std::cell::Cell<usize>`

2. Added keyboard controls in `handle_key_event()`:
   - PageUp/PageDown: Scroll by viewport height
   - Ctrl+Home: Jump to top
   - Ctrl+End: Jump to bottom
   - Ctrl+Up/Down: Scroll one line at a time

3. Created `ScrollableRenderable` wrapper in `codex-rs/dx/src/render/renderable.rs` to apply scroll offset

4. Modified `ChatWidget::render()` to render scrollbar when content exceeds viewport

### Current Issues
- **Compilation errors** in `chatwidget.rs` around line 3715-3739
- Scroll fields were added incorrectly to struct initialization (misplaced closing braces)
- The scroll fields appear in THREE constructor functions that need fixing:
  - `new_with_op_sender()` (around line 3523)
  - `new_from_existing()` (around line 3715)
  - Another constructor (around line 3901)

### The Core Problem
The scroll offset is tracked but NOT applied to content rendering. The scrollbar thumb moves, but the actual content doesn't scroll. The `ScrollableRenderable` wrapper was created but may not be working correctly.

## Technical Architecture

### Key Files
- `codex-rs/dx/src/chatwidget.rs` - Main chat widget (8000+ lines)
- `codex-rs/dx/src/scrollbar.rs` - Custom scrollbar widget
- `codex-rs/dx/src/render/renderable.rs` - Rendering system
- `codex-rs/dx/src/history_cell.rs` - Individual message cells

### Rendering Pipeline
1. `ChatWidget::as_renderable()` builds content as `FlexRenderable`
2. `FlexRenderable` contains:
   - Active cell (message history)
   - Bottom pane (input area)
3. `ChatWidget::render()` renders the content and scrollbar
4. Content needs to be offset by `scroll_offset` during rendering

### The Renderable Trait
```rust
pub trait Renderable {
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn desired_height(&self, width: u16) -> u16;
    fn cursor_pos(&self, _area: Rect) -> Option<(u16, u16)>;
}
```

## Implementation Requirements

### 1. Fix Compilation Errors
The scroll fields need to be properly added to the `ChatWidget` struct initialization in all three constructors. Look for:
```rust
last_rendered_user_message_event: None,
}  // <-- This closing brace is wrong
    scroll_offset: std::cell::Cell::new(0),
    content_height: std::cell::Cell::new(0),
    viewport_height: std::cell::Cell::new(0),
};
```

Should be:
```rust
last_rendered_user_message_event: None,
scroll_offset: std::cell::Cell::new(0),
content_height: std::cell::Cell::new(0),
viewport_height: std::cell::Cell::new(0),
};
```

### 2. Verify ScrollableRenderable Implementation
The `ScrollableRenderable` wrapper in `render/renderable.rs` needs to:
- Accept a `scroll_offset` parameter
- Create a virtual rendering area offset by the scroll amount
- Properly clip content to the visible viewport
- Handle cursor position adjustments

### 3. Apply Scroll Offset to Content
In `ChatWidget::render()`, the content must be wrapped in `ScrollableRenderable` so the offset is actually applied during rendering.

### 4. Handle Auto-Scroll Behavior
When new messages arrive, the UI should:
- Auto-scroll to bottom if user is already at bottom
- Stay at current position if user has scrolled up
- Provide visual indication when new content arrives while scrolled up

### 5. Test Scrolling
After implementation:
```bash
cd codex-rs/dx
cargo run
```

Verify:
- Scrollbar appears when content exceeds viewport
- PageUp/PageDown scrolls content
- Ctrl+Home/End jumps to top/bottom
- Ctrl+Up/Down scrolls one line
- Content actually moves (not just scrollbar thumb)
- New messages don't force scroll to bottom when viewing history

## Device Constraints
- **Low-end device** - Only use `cargo run`, NEVER `cargo build`, `cargo check`, or `cargo run --release`
- Test incrementally to avoid long compilation times

## Code Style Guidelines (from AGENTS.md)
- Use `format!` with inline variables: `format!("{variable}")` not `format!("{}", variable)`
- Collapse if statements per clippy rules
- Avoid small helper methods referenced only once
- Keep modules under 500 LoC (chatwidget.rs is already huge at 8000+ lines)
- Run `just fmt` after changes
- Run `cargo test -p codex-dx` after changes

## Success Criteria
1. ✅ Code compiles without errors
2. ✅ Scrollbar appears in fullscreen mode when content > viewport
3. ✅ Keyboard controls work (PageUp/Down, Ctrl+Home/End, Ctrl+Up/Down)
4. ✅ Content actually scrolls (not just scrollbar thumb)
5. ✅ Scrollbar thumb position accurately reflects scroll position
6. ✅ Auto-scroll to bottom works for new messages when at bottom
7. ✅ Manual scroll position is preserved when new messages arrive

## Additional Context
- The codebase uses `ratatui` for TUI rendering
- The project follows Rust best practices and clippy lints
- Custom scrollbar already exists and renders correctly
- The main challenge is applying the scroll offset to content rendering
- The rendering system uses a trait-based approach with `Renderable`

## Next Steps for Implementation
1. Fix the compilation errors in all three constructors
2. Verify `ScrollableRenderable` correctly offsets content
3. Test with `cargo run` in `codex-rs/dx`
4. Debug why content doesn't scroll if scrollbar thumb moves but content stays static
5. Add auto-scroll logic for new messages
6. Consider adding mouse wheel support (optional)

## Questions to Investigate
- Does `ScrollableRenderable::render()` actually offset the buffer correctly?
- Is the virtual area calculation correct for clipping?
- Should the scroll offset be applied to the entire `FlexRenderable` or just the active cell?
- How does ratatui's buffer clipping work with negative Y coordinates?

---

Good luck! The foundation is there, it just needs the final pieces to connect properly.
