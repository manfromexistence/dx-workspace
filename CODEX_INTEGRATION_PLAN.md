# Codex Integration Plan for DX TUI

## Current State Analysis

### What's Already Available
1. **Codex Integration Module** (`codex_integration.rs`)
   - Full initialization code for ChatWidget
   - Event processing system
   - Already imports all necessary Codex TUI components

2. **Commented Out Code**
   - State initialization in `state.rs`
   - Rendering in `dx_render.rs`
   - Key handling in `dispatcher.rs`
   - Toggle functionality

3. **Local LLM System** (`llm.rs`)
   - Currently uses llama.cpp for local inference
   - Streaming support
   - History management
   - This will be REPLACED by Codex

## Integration Strategy

### Phase 1: Replace Local LLM with Codex
Instead of having two separate systems (Local LLM + Codex TUI), we'll:
1. Remove the local llama.cpp integration
2. Use Codex as the ONLY AI backend
3. Render Codex responses directly in DX's chat UI

### Phase 2: What to Integrate

#### A. Core Codex Functionality (KEEP)
- ✅ Codex protocol communication
- ✅ Message streaming
- ✅ Tool execution
- ✅ File context management
- ✅ Multi-turn conversations
- ✅ Thinking/reasoning display
- ✅ Code block rendering
- ✅ Error handling

#### B. Codex TUI Components (ADAPT)
- ✅ ChatWidget for message rendering
- ✅ Input composer for rich text input
- ✅ Status line for connection status
- ✅ Syntax highlighting for code blocks
- ✅ Markdown rendering
- ⚠️ Bottom pane (adapt to DX style)
- ⚠️ File tree (use DX's Yazi integration)

#### C. DX-Specific Features (KEEP)
- ✅ Animation carousel
- ✅ Splash screens
- ✅ Theme system
- ✅ Menu overlay
- ✅ Bottom controls (Plan, Model, Local buttons)
- ✅ Toast notifications
- ✅ Sound system

## Implementation Steps

### Step 1: Uncomment and Initialize Codex
**Files to modify:**
- `state.rs`: Uncomment Codex widget initialization
- Enable background initialization on startup

### Step 2: Replace Message Rendering
**Current:** DX renders messages using `MessageList` component
**New:** Use Codex's ChatWidget to render messages

**Files to modify:**
- `dx_render.rs`: Replace MessageList with ChatWidget rendering
- Keep DX's input box and bottom controls
- Layout: `[ChatWidget Area] [DX Input] [DX Controls]`

### Step 3: Route Input to Codex
**Current:** Input goes to local LLM via `add_user_message()`
**New:** Input goes to Codex via protocol Op

**Files to modify:**
- `dispatcher.rs`: Send input to Codex instead of local LLM
- `state.rs`: Remove local LLM message handling

### Step 4: Handle Codex Events
**Files to modify:**
- `state.rs`: Process Codex events in `update()` method
- Handle streaming responses
- Update UI when messages arrive

### Step 5: Integrate Tool Execution
Codex can execute tools (file operations, shell commands, etc.)
- Display tool execution in DX UI
- Show progress/status
- Handle errors gracefully

### Step 6: Remove Local LLM
**Files to remove/modify:**
- `llm.rs`: Remove or mark as deprecated
- `model_manager.rs`: Remove or mark as deprecated
- `state.rs`: Remove LLM channels and initialization

## UI Layout Comparison

### Current DX Layout (Local LLM)
```
┌─────────────────────────────────┐
│                                 │
│     Message History             │
│     (MessageList)               │
│                                 │
├─────────────────────────────────┤
│  Input Box (5 lines)            │
├─────────────────────────────────┤
│ Plan | Model | Local | Path    │
└─────────────────────────────────┘
```

### Proposed Codex Integration Layout
```
┌─────────────────────────────────┐
│                                 │
│     Codex ChatWidget            │
│     (includes history +         │
│      input composer +           │
│      status line)               │
│                                 │
├─────────────────────────────────┤
│ Plan | Model | Codex | Path    │
└─────────────────────────────────┘
```

### Alternative: Hybrid Layout
```
┌─────────────────────────────────┐
│                                 │
│     Codex Message History       │
│     (ChatWidget - read only)    │
│                                 │
├─────────────────────────────────┤
│  DX Input Box (5 lines)         │
├─────────────────────────────────┤
│ Plan | Model | Codex | Path    │
└─────────────────────────────────┘
```

## Key Decisions

### 1. Input Handling
**Option A:** Use Codex's input composer (rich text, multi-line)
**Option B:** Keep DX's input box, send to Codex backend
**Recommendation:** Option B - Keep DX's familiar input, use Codex for AI

### 2. Message Rendering
**Option A:** Use Codex's ChatWidget entirely
**Option B:** Use Codex's rendering but wrap in DX theme
**Recommendation:** Option A - Codex has better markdown/code rendering

### 3. Model Selection
**Current:** DX has "Local" vs "Codex" toggle
**New:** Always use Codex, but allow model selection (GPT-4, Claude, etc.)
**Recommendation:** Update model picker to show Codex models

### 4. Offline Mode
**Current:** Local LLM works offline
**New:** Codex requires internet connection
**Recommendation:** Show clear status when offline, graceful degradation

## Code Changes Summary

### Files to Modify
1. ✅ `state.rs` - Uncomment Codex initialization
2. ✅ `dx_render.rs` - Use ChatWidget for rendering
3. ✅ `dispatcher.rs` - Route input to Codex
4. ✅ `codex_integration.rs` - Already complete
5. ⚠️ `llm.rs` - Mark as deprecated or remove
6. ⚠️ `models.rs` - Update model list to Codex models

### New Features to Add
1. Connection status indicator
2. Codex authentication flow
3. Tool execution visualization
4. File context display
5. Thinking/reasoning toggle

## Testing Plan

### Phase 1: Basic Integration
- [ ] Codex initializes on startup
- [ ] Can send messages to Codex
- [ ] Responses stream back correctly
- [ ] Messages render with proper formatting

### Phase 2: Advanced Features
- [ ] Code blocks render with syntax highlighting
- [ ] Tool execution works
- [ ] File context is maintained
- [ ] Multi-turn conversations work

### Phase 3: DX Integration
- [ ] Animations still work
- [ ] Menu overlay works
- [ ] Theme system applies to Codex UI
- [ ] Sound system works
- [ ] Bottom controls work

## Timeline Estimate

- **Step 1-2:** 2 hours (Uncomment and basic rendering)
- **Step 3-4:** 3 hours (Input routing and event handling)
- **Step 5:** 2 hours (Tool execution)
- **Step 6:** 1 hour (Cleanup)
- **Testing:** 2 hours

**Total:** ~10 hours of focused development

## Risks and Mitigations

### Risk 1: Codex initialization fails
**Mitigation:** Show clear error message, fallback to animation mode

### Risk 2: Performance issues with ChatWidget
**Mitigation:** Profile and optimize, consider lazy rendering

### Risk 3: Theme conflicts
**Mitigation:** Map DX themes to Codex color scheme

### Risk 4: Input handling conflicts
**Mitigation:** Clear key binding priority, document behavior

## Success Criteria

1. ✅ User can send messages and get Codex responses
2. ✅ Messages render beautifully with code highlighting
3. ✅ Tool execution works seamlessly
4. ✅ DX features (animations, themes, sounds) still work
5. ✅ Performance is smooth (no lag)
6. ✅ Error handling is graceful
7. ✅ Documentation is clear

## Next Steps

1. Review this plan with the team
2. Start with Step 1 (uncomment initialization)
3. Test each step incrementally
4. Document any issues or deviations
5. Update this plan as needed
