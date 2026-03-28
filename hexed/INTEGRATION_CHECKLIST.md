# Codex TUI Integration Checklist

## What's Implemented ✅

### Initialization
- ✅ `spawn_agent()` creates the agent thread
- ✅ `codex_op_tx` channel connects ChatWidget to agent
- ✅ `app_event_rx` channel receives events from agent
- ✅ ChatWidget created with `new_with_op_sender()`

### Event Flow
- ✅ Keys forwarded to `chat_widget.handle_key_event()`
- ✅ Events processed in `process_events()` → `handle_codex_event()`
- ✅ Timer calls `update()` every 50ms
- ✅ `update()` calls `process_events()`

### Rendering
- ✅ `pre_draw_tick()` called before render
- ✅ `chat_widget.render()` called with proper area
- ✅ DX bottom controls rendered (1 line)
- ✅ Codex gets rest of screen space

## What Should Happen

### When User Types
1. Key event → `dispatch_key()`
2. → `chat_widget.handle_key_event(key)`
3. → ChatWidget updates its internal input state
4. → `process_events()` called
5. → Render triggered

### When User Presses Enter
1. Key event (Enter) → `dispatch_key()`
2. → `chat_widget.handle_key_event(Enter)`
3. → ChatWidget generates `Op::UserTurn` internally
4. → Op sent through `codex_op_tx` to agent
5. → Agent processes and generates `Event`
6. → Event comes back through `app_event_rx` as `AppEvent::CodexEvent`
7. → `process_events()` receives it
8. → `chat_widget.handle_codex_event(event)` updates UI
9. → Render shows the response

## Potential Issues

### Issue #1: Events Not Being Processed
**Check**: Are events actually coming through `app_event_rx`?
**Debug**: Add logging in `process_events()` to see if events arrive

### Issue #2: ChatWidget Not Sending Ops
**Check**: Does ChatWidget actually send Ops when Enter is pressed?
**Debug**: The Op should be sent automatically by ChatWidget's internal logic

### Issue #3: Render Not Updating
**Check**: Is `NEED_RENDER` being set after events?
**Debug**: We set it in dispatcher after `process_events()`

### Issue #4: Area Too Small
**Check**: Does ChatWidget have enough space?
**Current**: We give it `area - 1 line` for DX controls
**Fix**: Try giving it full area temporarily to test

## Quick Tests

### Test 1: Verify Initialization
Run and check if "Codex TUI ready!" toast appears
- ✅ Yes → Initialization worked
- ❌ No → Check `initialize_codex_widget()` errors

### Test 2: Verify Key Forwarding
Type characters and see if they appear in Codex input
- ✅ Yes → Keys are being forwarded
- ❌ No → Check dispatcher key forwarding

### Test 3: Verify Enter Key
Press Enter and watch for any change
- ✅ Input clears → Enter reached ChatWidget
- ❌ Nothing → Enter not being processed

### Test 4: Verify Events
After Enter, wait a few seconds
- ✅ Response appears → Full flow works!
- ❌ Nothing → Events not coming back from agent

## Next Steps

If everything above checks out but it still doesn't work:

1. **Add Debug Logging**
   - Log in `process_events()` when events arrive
   - Log in `handle_key_event()` when Enter is pressed
   - Log in `update()` to verify it's being called

2. **Check Agent Status**
   - Is the agent thread actually running?
   - Are there any errors in the agent?
   - Is the model configured correctly?

3. **Simplify**
   - Try giving Codex the full screen (no DX controls)
   - Try removing all DX key interception
   - Try running codex-tui-dx binary to verify it works standalone

## The Real Question

**Why isn't it working when the code looks correct?**

Possible answers:
- Agent thread crashed/not running
- Model not configured (using wrong model name)
- Auth issues (no API key)
- Events being dropped somewhere
- Render not being called frequently enough
- Some async timing issue

The code structure is correct. The issue is likely environmental or timing-related.
