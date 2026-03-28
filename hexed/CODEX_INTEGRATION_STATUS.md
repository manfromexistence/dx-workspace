# Codex TUI Integration Status

## Current State (March 26, 2026)

### What Works ✅
- Codex TUI initializes successfully
- ChatWidget renders in dx TUI
- Keys are forwarded to ChatWidget (including Enter)
- DX bottom controls render (1 line at bottom)

### What Doesn't Work ❌

#### 1. Enter Key Not Submitting Messages
**Symptom**: Pressing Enter doesn't submit the message to Codex agent

**Root Cause**: We're calling `chat_widget.handle_key_event(key)` but the ChatWidget needs to:
1. Process the key event
2. Generate an Op (operation) to send to the agent
3. The Op needs to be sent through `codex_op_tx`

**The Real Flow** (from codex-tui-dx):
```
User presses Enter
→ ChatWidget.handle_key_event(Enter)
→ ChatWidget generates Op::UserTurn
→ Op sent to agent via codex_op_tx
→ Agent processes and sends back Event
→ Event comes through app_event_rx as AppEvent::CodexEvent
→ ChatWidget.handle_codex_event(event) updates UI
```

**What We're Missing**: ChatWidget doesn't automatically send Ops when Enter is pressed. It expects the app to handle this.

#### 2. AI Responses Not Showing
**Symptom**: After submitting (if Enter worked), no AI response appears

**Root Cause**: We ARE handling `AppEvent::CodexEvent` in `process_events()`, but:
- Events might not be coming through
- Or ChatWidget isn't updating its display properly
- Or we're not calling the right methods

#### 3. Status Line Not Showing
**Symptom**: Codex TUI status line (bottom of Codex area) not visible

**Root Cause**: ChatWidget expects more vertical space. We're giving it `area - 1 line` for DX controls, but ChatWidget's internal layout might need adjustment.

## The Real Problem

Looking at the real codex-tui-dx (`src/app.rs`), it does this:

```rust
// In the event loop:
match event {
    TuiEvent::Key(key_event) => {
        self.handle_key_event(tui, key_event).await;
    }
}

// handle_key_event forwards to:
self.chat_widget.handle_key_event(key_event);

// Then in the render loop:
self.chat_widget.pre_draw_tick();
tui.draw(self.chat_widget.desired_height(tui.terminal.size()?.width), |frame| {
    self.chat_widget.render(frame.area(), frame.buffer);
    if let Some((x, y)) = self.chat_widget.cursor_pos(frame.area()) {
        frame.set_cursor_position((x, y));
    }
})?;
```

**Key differences from our implementation:**
1. Real app uses `desired_height()` - we don't
2. Real app sets cursor position - we don't
3. Real app has a proper async event loop - we're in a sync context

## What We Need to Fix

### Fix #1: Proper Event Handling
The ChatWidget generates Ops internally when Enter is pressed, but we need to ensure:
- The Op channel is working
- Events are flowing back from the agent
- We're processing them correctly

### Fix #2: Cursor Position
We need to set the cursor position after rendering. This might be why Enter doesn't work - the cursor isn't in the right place.

### Fix #3: Desired Height
ChatWidget has a `desired_height()` method that calculates how much space it needs. We should use this.

## Recommended Approach

### Option A: Minimal Fix (1-2 hours)
1. Add cursor position setting in the render loop
2. Verify event channel is working (add debug logs)
3. Ensure `pre_draw_tick()` is called every frame

### Option B: Proper Integration (1 day)
1. Create a proper async context for Codex widget
2. Run Codex event loop in background task
3. Use channels to communicate between dx and Codex
4. This is what the real app does

### Option C: Subprocess (30 minutes)
1. Run `codex-tui-dx` as a subprocess
2. Capture its output and render in dx
3. Forward input to it
4. This is hacky but would work immediately

## My Recommendation

**Start with Option A** - it's the quickest path to a working demo. If that doesn't work, we know we need Option B or C.

The key insight: ChatWidget is designed to work in a full app context with proper event loops. We're trying to use it as a simple widget, which is why things break.
