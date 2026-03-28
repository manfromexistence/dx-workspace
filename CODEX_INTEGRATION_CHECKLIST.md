# Codex Integration Checklist

## What We Need to Integrate from Codex TUI into DX TUI

### 1. Core Codex Protocol Communication ✅ (Already Available)
- [x] `codex_protocol` - Message protocol
- [x] `codex_core` - Core functionality
- [x] `AuthManager` - Authentication
- [x] `ThreadManager` - Conversation management
- [x] `ModelsManager` - Model selection
- [x] Agent spawning - Background AI processing

**Status:** Already implemented in `codex_integration.rs`

---

### 2. ChatWidget Component ✅ (Already Available)
**What it provides:**
- Message history rendering
- Markdown parsing and display
- Code block syntax highlighting
- Thinking/reasoning display
- Streaming message updates
- Tool execution visualization

**Files:**
- `codex_tui_dx::chatwidget::ChatWidget`
- Already imported and initialized

**Integration needed:**
- [ ] Uncomment rendering code in `dx_render.rs`
- [ ] Call `chat_widget.render()` in chat mode
- [ ] Process events with `process_events()`

---

### 3. Input Handling 🔄 (Needs Integration)
**Current DX:** Simple text input box
**Codex TUI:** Rich input composer with:
- Multi-line editing
- Syntax highlighting
- File attachments
- Command palette

**Decision:** Keep DX's simple input, send to Codex backend

**Integration needed:**
- [ ] Route Enter key to send Codex Op
- [ ] Convert DX input to Codex UserInput
- [ ] Handle Ctrl+C to cancel generation

---

### 4. Message Streaming 🔄 (Needs Integration)
**What Codex provides:**
- Token-by-token streaming
- Partial message updates
- Thinking tag parsing
- Tool execution events

**Integration needed:**
- [ ] Subscribe to AppEvent stream
- [ ] Update ChatWidget on new tokens
- [ ] Handle streaming completion
- [ ] Show typing indicator

---

### 5. Tool Execution Display 🔄 (Needs Integration)
**What Codex provides:**
- File read/write operations
- Shell command execution
- Search operations
- Code analysis

**Integration needed:**
- [ ] Show tool execution in progress
- [ ] Display tool results
- [ ] Handle tool errors
- [ ] Allow tool approval/rejection

---

### 6. File Context Management 🔄 (Needs Integration)
**What Codex provides:**
- File tree navigation
- File content display
- Context tracking
- Workspace awareness

**Integration needed:**
- [ ] Use DX's Yazi for file selection
- [ ] Send file context to Codex
- [ ] Display active files in status
- [ ] Handle file changes

---

### 7. Status Line 🔄 (Needs Integration)
**What Codex provides:**
- Connection status
- Model name
- Token count
- Active files
- Collaboration mode

**Integration needed:**
- [ ] Adapt to DX's bottom controls
- [ ] Show Codex status in DX style
- [ ] Merge with existing controls
- [ ] Keep DX's button layout

---

### 8. Syntax Highlighting ✅ (Already Available)
**What Codex provides:**
- Code block detection
- Language-specific highlighting
- Theme-aware colors

**Status:** Built into ChatWidget, works automatically

---

### 9. Markdown Rendering ✅ (Already Available)
**What Codex provides:**
- Headers, lists, tables
- Bold, italic, code
- Links and images
- Block quotes

**Status:** Built into ChatWidget, works automatically

---

### 10. Error Handling 🔄 (Needs Integration)
**What Codex provides:**
- Network errors
- Authentication errors
- Rate limiting
- Tool execution errors

**Integration needed:**
- [ ] Show errors in DX toast system
- [ ] Graceful degradation
- [ ] Retry logic
- [ ] Clear error messages

---

### 11. Authentication Flow 🔄 (Needs Integration)
**What Codex provides:**
- Login/logout
- Token management
- Account info
- Credential storage

**Integration needed:**
- [ ] Show auth status in DX
- [ ] Handle login flow
- [ ] Store credentials securely
- [ ] Show account info

---

### 12. Model Selection 🔄 (Needs Integration)
**Current DX:** Local vs Codex toggle
**Codex TUI:** Full model picker with:
- GPT-4, GPT-3.5
- Claude 3.5, Claude 3
- Mistral models
- Custom models

**Integration needed:**
- [ ] Update DX model picker
- [ ] Show Codex models
- [ ] Handle model switching
- [ ] Show model capabilities

---

### 13. Collaboration Modes ❌ (Not Needed)
**What Codex provides:**
- Auto mode (full autonomy)
- Prompt mode (ask before tools)
- Manual mode (user approval)

**Decision:** Not needed for DX TUI initially

---

### 14. Session Management 🔄 (Needs Integration)
**What Codex provides:**
- Save/load conversations
- Thread history
- Session resume
- Export conversations

**Integration needed:**
- [ ] Save Codex threads to DX history
- [ ] Load previous conversations
- [ ] Export to markdown
- [ ] Clear history

---

### 15. Telemetry and Analytics ❌ (Optional)
**What Codex provides:**
- Usage tracking
- Error reporting
- Performance metrics

**Decision:** Optional, can be disabled

---

## Implementation Priority

### Phase 1: Core Functionality (Must Have)
1. ✅ Codex initialization
2. 🔄 Message sending
3. 🔄 Message streaming
4. 🔄 ChatWidget rendering
5. 🔄 Basic error handling

### Phase 2: Enhanced Features (Should Have)
6. 🔄 Tool execution
7. 🔄 File context
8. 🔄 Model selection
9. 🔄 Status display
10. 🔄 Authentication

### Phase 3: Advanced Features (Nice to Have)
11. ❌ Session management
12. ❌ Collaboration modes
13. ❌ Telemetry
14. ❌ Advanced tool approval

---

## What We DON'T Need from Codex TUI

### 1. Full TUI Framework ❌
Codex TUI has its own app loop, event handling, etc.
**DX has:** Its own TUI framework (Yazi-based)
**Decision:** Only use ChatWidget component, not full app

### 2. File Browser ❌
Codex TUI has a file tree widget
**DX has:** Yazi file manager integration
**Decision:** Use DX's Yazi, send selections to Codex

### 3. Settings UI ❌
Codex TUI has a settings panel
**DX has:** Menu overlay system
**Decision:** Use DX's menu for settings

### 4. Splash Screen ❌
Codex TUI has its own splash
**DX has:** Animation carousel
**Decision:** Keep DX's animations

### 5. Theme System ❌
Codex TUI has its own themes
**DX has:** JSON-based theme system
**Decision:** Map DX themes to Codex colors

### 6. Bottom Pane ❌
Codex TUI has a bottom status pane
**DX has:** Custom bottom controls
**Decision:** Keep DX's controls, add Codex status

---

## Integration Approach

### Minimal Integration (Recommended)
```rust
// In DX TUI:
1. Initialize Codex ChatWidget on startup
2. Render ChatWidget in chat area
3. Send user input to Codex via Op
4. Process Codex events in update loop
5. Keep all DX features (animations, themes, sounds)
```

### Full Integration (Not Recommended)
```rust
// Would require:
1. Replacing DX's entire UI with Codex TUI
2. Losing DX's unique features
3. Major refactoring
4. Compatibility issues
```

---

## Code Structure

### Current DX Structure
```
dx/
├── src/
│   ├── dx.rs              # Main entry
│   ├── state.rs           # App state
│   ├── dx_render.rs       # Rendering
│   ├── dispatcher.rs      # Input handling
│   ├── llm.rs            # Local LLM (TO REMOVE)
│   ├── codex_integration.rs  # Codex setup ✅
│   └── ...
```

### After Integration
```
dx/
├── src/
│   ├── dx.rs              # Main entry
│   ├── state.rs           # App state + Codex
│   ├── dx_render.rs       # Rendering + ChatWidget
│   ├── dispatcher.rs      # Input → Codex
│   ├── codex_integration.rs  # Codex setup ✅
│   ├── codex_handler.rs   # NEW: Event processing
│   └── ...
```

---

## Summary

### What's Already Done ✅
- Codex initialization code
- ChatWidget import
- Event system setup
- Protocol communication

### What Needs to Be Done 🔄
1. Uncomment initialization in `state.rs`
2. Uncomment rendering in `dx_render.rs`
3. Route input to Codex in `dispatcher.rs`
4. Process Codex events in `state.rs`
5. Handle tool execution
6. Update model picker
7. Add authentication flow
8. Integrate file context

### What We're NOT Doing ❌
- Replacing DX's TUI framework
- Using Codex's file browser
- Using Codex's settings UI
- Using Codex's theme system
- Full collaboration mode support

---

## Estimated Effort

- **Uncomment and test:** 1 hour
- **Input routing:** 2 hours
- **Event handling:** 2 hours
- **Tool execution:** 3 hours
- **Model picker:** 1 hour
- **Authentication:** 2 hours
- **Testing and polish:** 3 hours

**Total:** ~14 hours

---

## Success Metrics

1. ✅ Can send message to Codex
2. ✅ Response streams back in real-time
3. ✅ Code blocks render with highlighting
4. ✅ Tools execute successfully
5. ✅ DX features still work (animations, themes, sounds)
6. ✅ Performance is smooth
7. ✅ Error handling is graceful
