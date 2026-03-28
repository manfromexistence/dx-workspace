# Final Fix Needed: Show Top Messages

## Current Status

✅ **WORKING**: Bottom pane is now at the absolute bottom of the terminal
✅ **WORKING**: Status line is visible at the bottom
✅ **WORKING**: Input box is at the bottom

❌ **NOT WORKING**: Top messages are not showing (header with model info, directory, tip)

## What's Missing

The top part of the UI should show:
1. **Session header** - "Dx (v26.2.2)"
2. **Model info** - "model: mistral-large-latest low"
3. **Directory** - "directory: F:\codex\codex-rs\dx"
4. **Tip** - "Tip: New Build faster with Codex."

These are rendered by the `active_cell` which contains a `SessionHeaderHistoryCell`.

## Root Cause

The `active_cell` is likely:
- Being rendered but scrolled out of view
- Being rendered in an area with height=0
- Being clipped by the layout

## Files Involved

- `codex-rs/dx/src/chatwidget.rs` - Main rendering logic
- `codex-rs/dx/src/chatwidget/session_header.rs` - Session header component
- `codex-rs/dx/src/history_cell.rs` - History cell rendering

## Solution Needed

Ensure that when rendering the ChatWidget:
1. The `active_cell` gets enough vertical space to render
2. The `active_cell` is positioned at the top (y=0 or y=1)
3. The scroll position doesn't hide the `active_cell`

## Quick Fix Options

### Option 1: Ensure active_cell is always visible
In the render method, make sure `active_cell` is rendered in a visible area before the spacer.

### Option 2: Adjust scroll position
Make sure `scroll_position` doesn't scroll past the `active_cell` content.

### Option 3: Check FlexRenderable allocation
Verify that `flex(0)` for `active_cell` is getting its `desired_height()` allocated.

## Testing

After fix, verify:
```
┌─────────────────────────────────────┐
│ Dx (v26.2.2)                        │  ← Should be visible
│ model: mistral-large-latest low     │  ← Should be visible
│ directory: F:\codex\codex-rs\dx     │  ← Should be visible
├─────────────────────────────────────┤
│ Tip: New Build faster with Codex.  │  ← Should be visible
│                                     │
│ (transcript area)                   │
│                                     │
├─────────────────────────────────────┤
│ > input box                         │  ← Already working
├─────────────────────────────────────┤
│ status line                         │  ← Already working
└─────────────────────────────────────┘
```

## Next Steps

1. Check current `as_renderable()` or `render()` implementation
2. Verify `active_cell.desired_height()` is being respected
3. Ensure `active_cell` render area has height > 0
4. Test with `cargo run --bin codex-tui-dx`
