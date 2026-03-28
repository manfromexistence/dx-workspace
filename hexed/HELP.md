# Help Needed: Adding Scrollbar to Codex TUI in dx Binary

> This file is created when a task requires more expertise or a different approach.
> A more capable AI or human developer should review and resolve this issue.

**Date:** 2026-03-26

---

## Task Description

Add a custom scrollbar to the Codex TUI (Terminal User Interface) in the `dx` binary to enable scrolling through chat history. The user wants to be able to scroll up and down through the conversation history using mouse wheel and keyboard (PageUp/PageDown).

---

## Context: The Codebase

**Repository:** https://github.com/codex-rs/codex (or similar - this is the official Codex codebase)

**Working Directory:** `codex-rs/dx/`

**Binary:** `codex-tui-dx` (defined in `codex-rs/dx/Cargo.toml`)
- Entry point: `codex-rs/dx/src/codex.rs`
- Main library: `codex-rs/dx/src/` (the dx crate)

**Key Files:**
1. `codex-rs/dx/src/chatwidget.rs` - Main ChatWidget implementation (~8600+ lines)
2. `codex-rs/dx/src/render/renderable.rs` - Renderable trait for UI components
3. `codex-rs/dx/src/bottom_pane/` - Bottom pane with input composer
4. `codex-rs/dx/src/dispatcher.rs` - Event handling (keyboard/mouse)
5. `codex-rs/dx/src/state.rs` - State management (for dx.rs binary, NOT codex-tui-dx)

**Important Note:** There are TWO binaries in `codex-rs/dx/`:
- `dx` - Uses `codex-rs/dx/src/dx.rs` (different TUI, uses state.rs)
- `codex-tui-dx` - Uses `codex-rs/dx/src/codex.rs` (the one we're working on)

The user is working with `codex-tui-dx`, which uses the standard Codex TUI implementation.

---

## Current UI Structure

The Codex TUI is structured as follows:

```
┌─────────────────────────────────────┐
│                                     │
│  History Area (active_cell)         │  <- This area needs scrollbar
│  - Chat messages                    │
│  - Agent responses                  │
│  - Tool outputs                     │
│  (This grows as conversation grows) │
│                                     │
├─────────────────────────────────────┤
│  Input Box (bottom_pane)            │  <- Fixed at bottom
│  > User types here...               │
├─────────────────────────────────────┤
│  Status Line                        │  <- Fixed at bottom
│  model | tokens | status            │
└─────────────────────────────────────┘
```

**Key Implementation Details:**

In `chatwidget.rs`, the `as_renderable()` method (around line 8548) builds the UI:

```rust
fn as_renderable(&self) -> RenderableItem<'_> {
    let active_cell_renderable = match &self.active_cell {
        Some(cell) => RenderableItem::Borrowed(cell)
            .inset(Insets::tlbr(1, 0, 0, 0)),
        None => RenderableItem::Owned(Box::new(())),
    };
    let mut flex = FlexRenderable::new();
    flex.push(/*flex*/ 1, active_cell_renderable);  // History area (grows)
    flex.push(/*flex*/ 0, RenderableItem::Borrowed(&self.bottom_pane)
        .inset(Insets::tlbr(1, 0, 0, 0)));  // Input area (fixed)
    RenderableItem::Owned(Box::new(flex))
}
```

The `active_cell` contains the chat history and grows as the conversation progresses.

---

## What Has Been Attempted

### Attempt 1: Created Custom Scrollbar Widget
**Files Modified:**
- Created `codex-rs/dx/src/scrollbar.rs` - Custom scrollbar implementation
- Modified `codex-rs/dx/src/dx.rs` - Added `mod scrollbar;`
- Modified `codex-rs/dx/src/state.rs` - Added `ScrollbarState` field

**Problem:** These changes were for the WRONG binary (`dx` instead of `codex-tui-dx`). The `state.rs` file is only used by the `dx` binary, not `codex-tui-dx`.

### Attempt 2: Added Red Background to Identify History Area
**Files Modified:**
- `codex-rs/dx/src/chatwidget.rs` - Added `RedBackgroundWrapper` and `EmptyRedBackground` structs

**Code Added (around line 8600):**
```rust
// TEMPORARY: Wrapper to add red background to the history area
struct RedBackgroundWrapper<'a, T: Renderable> {
    inner: &'a T,
}

impl<'a, T: Renderable> Renderable for RedBackgroundWrapper<'a, T> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.inner.render(area, buf);
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let cell = &mut buf[(x, y)];
                cell.set_bg(ratatui::style::Color::Red);
            }
        }
    }
    // ... other trait methods
}
```

**Modified `as_renderable()` to use wrapper:**
```rust
let active_cell_renderable = match &self.active_cell {
    Some(cell) => {
        let red_bg_cell = RedBackgroundWrapper { inner: cell };
        RenderableItem::Owned(Box::new(red_bg_cell))
            .inset(Insets::tlbr(1, 0, 0, 0))
    }
    None => RenderableItem::Owned(Box::new(EmptyRedBackground)),
};
```

**Problem:** The red background is not showing up. The content seems to be rendering over it or the background is not being applied correctly. This suggests the rendering pipeline might be more complex than expected.

### Attempt 3: Mouse Scroll Event Handling
**Files Modified:**
- `codex-rs/dx/src/dispatcher.rs` - Added scroll event handling

**Code Added (around line 800):**
```rust
if !self.app.bridge.chat_state.show_tachyon_menu
    && self.app.bridge.chat_state.show_codex_tui
    && self.app.bridge.chat_state.codex_widget.is_some()
{
    match mouse.kind {
        MouseEventKind::ScrollUp => {
            self.app.bridge.chat_state.codex_scrollbar_state.scroll_up(3);
            self.app.bridge.chat_state.codex_scroll_offset =
                self.app.bridge.chat_state.codex_scrollbar_state.position;
            NEED_RENDER.store(1, Ordering::Relaxed);
            succ!()
        }
        MouseEventKind::ScrollDown => {
            self.app.bridge.chat_state.codex_scrollbar_state.scroll_down(3);
            self.app.bridge.chat_state.codex_scroll_offset =
                self.app.bridge.chat_state.codex_scrollbar_state.position;
            NEED_RENDER.store(1, Ordering::Relaxed);
            succ!()
        }
        _ => {}
    }
}
```

**Problem:** This code references `chat_state` which is from the `dx` binary, not `codex-tui-dx`. The event handling needs to be done differently for the actual Codex TUI.

---

## Root Cause Analysis

1. **Wrong Binary Confusion:** Initial attempts modified code for the `dx` binary (which has its own TUI with `ChatState` in `state.rs`), but the user is running `codex-tui-dx` which uses the standard Codex TUI (`ChatWidget` in `chatwidget.rs`).

2. **Rendering Pipeline Complexity:** The Codex TUI uses a custom `Renderable` trait system with `FlexRenderable`, `ColumnRenderable`, etc. Simply wrapping with a background doesn't work as expected.

3. **Active Cell Structure:** The `active_cell` field in `ChatWidget` is of type `Option<ActiveCell>` and contains the entire chat history. Understanding how this cell renders and scrolls internally is crucial.

4. **Missing Scroll State:** The `ChatWidget` doesn't currently have scroll state management. We need to:
   - Track scroll position
   - Modify rendering to respect scroll offset
   - Handle scroll events (mouse wheel, PageUp/PageDown)

---

## Suggested Solutions

### Solution 1: Investigate Active Cell Rendering
1. Find where `ActiveCell` is defined and how it renders
2. Check if it already has internal scrolling capability
3. Look for existing scroll offset or viewport management

### Solution 2: Add Scroll State to ChatWidget
1. Add scroll state fields to `ChatWidget` struct:
   ```rust
   pub(crate) scroll_offset: usize,
   pub(crate) viewport_height: usize,
   ```

2. Modify the rendering to apply scroll offset when rendering `active_cell`

3. Add scroll event handlers in the appropriate event handling code (not in `dispatcher.rs` for `dx` binary)

### Solution 3: Use Ratatui's Built-in Scrollbar
Ratatui has a built-in `Scrollbar` widget. Instead of creating a custom one:
1. Import `ratatui::widgets::{Scrollbar, ScrollbarState}`
2. Add scrollbar state to `ChatWidget`
3. Render the scrollbar alongside the history area

### Solution 4: Find Existing Scroll Implementation
The Codex TUI likely already has scrolling for overlays (like the transcript overlay mentioned in comments). Search for:
- `scroll` in `chatwidget.rs`
- `pager_overlay.rs` or similar files
- Existing scroll state management

---

## Key Questions to Answer

1. **Where is the event handling for `codex-tui-dx`?**
   - The `dispatcher.rs` seems to be for the `dx` binary
   - Where does `codex-tui-dx` handle keyboard/mouse events?

2. **How does `ActiveCell` render?**
   - What is the `ActiveCell` type?
   - Does it have internal scrolling?
   - How can we control its viewport?

3. **Is there existing scroll functionality?**
   - Check for transcript overlay scrolling
   - Look for pager implementations
   - Search for existing scroll state

4. **Why doesn't the red background show?**
   - Is the rendering order correct?
   - Are styles being overridden?
   - Is the area calculation correct?

---

## Files to Investigate

1. `codex-rs/dx/src/chatwidget.rs` - Main widget, search for:
   - `ActiveCell` definition
   - Existing scroll implementations
   - Event handling methods

2. `codex-rs/dx/src/pager_overlay.rs` - Might have scroll implementation

3. `codex-rs/dx/src/app.rs` - Application event loop for `codex-tui-dx`

4. `codex-rs/dx/src/exec_cell/` - Might contain `ActiveCell` implementation

5. `codex-rs/tui/` - Check if there's a separate TUI crate with shared code

---

## Environment Info

- **Language/Runtime:** Rust 1.85 (edition 2024)
- **OS:** Windows
- **Key Dependencies:**
  - `ratatui` - Terminal UI framework
  - `crossterm` - Terminal manipulation
  - `tokio` - Async runtime

---

## Next Steps for a More Capable AI

1. **Understand the Architecture:**
   - Map out the relationship between `dx` binary and `codex-tui-dx` binary
   - Identify where `codex-tui-dx` handles events
   - Find the `ActiveCell` implementation

2. **Find Existing Scroll Code:**
   - Search the codebase for existing scroll implementations
   - Look at how overlays handle scrolling
   - Check if there's already a scroll offset mechanism

3. **Implement Proper Scrolling:**
   - Add scroll state to the correct structure (likely `ChatWidget`)
   - Hook up scroll events in the correct event handler
   - Modify rendering to respect scroll offset
   - Add visual scrollbar indicator

4. **Test the Red Background:**
   - Debug why the red background isn't showing
   - This will confirm we can access and modify the history area
   - Once confirmed, replace with actual scrollbar

---

## References

- Ratatui Documentation: https://docs.rs/ratatui/
- Crossterm Events: https://docs.rs/crossterm/latest/crossterm/event/
- Original Issue: User wants to scroll through Codex TUI chat history using mouse wheel and keyboard

---

## Contact

If you need clarification on any of the attempts made or the current state of the code, please review:
- `codex-rs/dx/src/chatwidget.rs` (lines 8600-8650 for red background wrapper)
- `codex-rs/dx/src/scrollbar.rs` (custom scrollbar widget - may not be needed)
- `codex-rs/dx/src/dispatcher.rs` (lines 800-820 for scroll event handling - wrong binary)

The goal is to enable smooth scrolling through chat history in the `codex-tui-dx` binary with a visible scrollbar indicator.
