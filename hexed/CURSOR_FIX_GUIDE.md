# Cursor Fix Guide for codex-tui-dx

**Problem:** The cursor is hidden in the text input box of the codex-tui-dx binary.

**Root Cause:** The cursor is hidden when `cursor_pos()` returns `None`. The rendering flow is:

```
App::draw()
  └─> chat_widget.cursor_pos(area)
      └─> as_renderable().cursor_pos(area)
          └─> FlexRenderable
              └─> BottomPane.cursor_pos(area)
                  └─> as_renderable().cursor_pos(area)
                      └─> TextArea.cursor_pos_with_state(area, state)
```

When `cursor_pos()` returns `None`, the terminal calls `hide_cursor()` (see `custom_terminal.rs:370`).

## Solution

The issue is in the `TextArea::cursor_pos_with_state()` method in `src/bottom_pane/textarea.rs`.

### Current Code (line 221-231):

```rust
pub fn cursor_pos_with_state(&self, area: Rect, state: TextAreaState) -> Option<(u16, u16)> {
    if self.text.is_empty() {
        return None;  // <-- THIS HIDES THE CURSOR!
    }
    let lines = self.wrapped_lines(area.width);
    let effective_scroll = self.effective_scroll(area.height, &lines, state.scroll);
    let i = Self::wrapped_line_index_by_start(&lines, self.cursor_pos)?;
    let ls = &lines[i];
    let col = self.text[ls.start..self.cursor_pos].width() as u16;
    let screen_row = i.saturating_sub(effective_scroll as usize).try_into().unwrap_or(0);
    // ... rest of calculation
}
```

### Fix:

Change line 223-225 to always return a cursor position, even when text is empty:

```rust
pub fn cursor_pos_with_state(&self, area: Rect, state: TextAreaState) -> Option<(u16, u16)> {
    if self.text.is_empty() {
        // Return cursor at the start of the input area instead of None
        return Some((area.x, area.y));
    }
    // ... rest of the method
}
```

## Files to Modify

**File:** `codex-rs/dx/src/bottom_pane/textarea.rs`

**Line:** 223-225

**Change:**
```rust
// BEFORE:
if self.text.is_empty() {
    return None;
}

// AFTER:
if self.text.is_empty() {
    return Some((area.x, area.y));
}
```

## Testing

After making the change:

```bash
cd codex-rs/dx
cargo run
```

The cursor should now be visible in the text input box, even when it's empty.

## Alternative Solution (if the above doesn't work)

If the cursor is still hidden, you can force it to always be shown by modifying the draw call in `app.rs`:

**File:** `codex-rs/dx/src/app.rs`

**Line:** 2639-2642

**Change:**
```rust
// BEFORE:
if let Some((x, y)) = self.chat_widget.cursor_pos(frame.area()) {
    frame.set_cursor_position((x, y));
}

// AFTER:
let cursor_pos = self.chat_widget.cursor_pos(frame.area())
    .unwrap_or_else(|| {
        // Default to bottom-left of the frame if no cursor position is provided
        let area = frame.area();
        (area.x, area.bottom().saturating_sub(1))
    });
frame.set_cursor_position(cursor_pos);
```

This ensures the cursor is always shown, even if `cursor_pos()` returns `None`.

## Why This Happens

The cursor is hidden when:
1. The text input is empty (`self.text.is_empty()`)
2. `cursor_pos()` returns `None`
3. The terminal's `draw()` method sees `cursor_position = None`
4. It calls `hide_cursor()` (see `custom_terminal.rs:370`)

By ensuring `cursor_pos()` always returns `Some((x, y))`, the cursor will always be visible.

---

**Date:** March 27, 2026  
**Repository:** https://github.com/codex-rs/codex-rs  
**Binary:** codex-tui-dx
