# DX TUI - Codex Integration TODO

## 🎯 NEW STRATEGY: Use Codex TUI Code Directly

Instead of reimplementing features, we're taking Codex TUI's complete `ChatWidget` (which has ALL the logic) and just replacing its rendering with DX's superior UI.

**Files copied:**
- ✅ `slash_command.rs` - All slash commands
- ✅ `chatwidget_full.rs` - Complete ChatWidget with ALL event handlers
- ✅ `chatwidget/` - All ChatWidget modules (skills, plugins, agent, etc.)
- ✅ `app_event.rs` - Event types

**What we're doing:**
1. Remove ratatui rendering code from ChatWidget
2. Add DX rendering methods
3. Replace bottom_pane popups with DX menu
4. Wire up to DX's event loop

**Result:** 100% Codex TUI features + DX's superior UI

---

## 🚀 IMPLEMENTATION TASKS

### Phase 1: Core Integration (Priority 1)
- [ ] Add module declarations in `dx.rs`
- [ ] Remove ratatui rendering from `chatwidget_full.rs`
- [ ] Add DX rendering methods to ChatWidget
- [ ] Replace ChatState with ChatWidget in state.rs
- [ ] Wire up event forwarding
- [ ] Test basic chat works

### Phase 2: Menu Integration (Priority 2)
- [ ] Map approval popups to DX menu
- [ ] Map skills UI to DX menu  
- [ ] Map plugins UI to DX menu
- [ ] Map MCP UI to DX menu
- [ ] Add slash command autocomplete

### Phase 3: Polish (Priority 3)
- [ ] Add status bar integration (tokens, rate limits)
- [ ] Add session management UI
- [ ] Add reasoning display
- [ ] Add image support
- [ ] Test all features

---

## 📊 WHAT WE GET

By using Codex TUI code directly, we automatically get:

### ✅ ALL Event Handlers (~50 types)
- AgentMessageDelta, AgentReasoning, TokenCount, Warning, TurnAborted
- ExecApprovalRequest, ApplyPatchApprovalRequest, ElicitationRequest
- ExecCommandBegin/End, TerminalInteraction, PatchApplyBegin/End
- McpStartupUpdate, McpToolCallBegin/End, McpListToolsResponse
- ImageGenerationBegin/End, ViewImageToolCall, WebSearchBegin/End
- PlanDelta, PlanUpdate, CollabAgentSpawnBegin, GuardianAssessment
- And 30+ more...

### ✅ ALL Slash Commands (35 commands)
- /model, /fast, /approvals, /skills, /review, /rename, /new, /resume
- /fork, /init, /compact, /plan, /collab, /agent, /diff, /copy
- /mention, /status, /debug-config, /title, /statusline, /theme
- /mcp, /apps, /plugins, /logout, /quit, /feedback, /ps, /stop
- And more...

### ✅ ALL Features
- Approval system (Never/OnRequest/UnlessTrusted)
- Skills management (enable/disable, paths, scanning)
- Plugin marketplace (browse, install, uninstall)
- MCP integration (server startup, tool execution)
- Session management (resume, fork, history)
- Plan mode, collaboration mode, review mode
- Reasoning display (o1, DeepSeek)
- Image support, web search
- Token tracking, rate limits, credits
- And everything else Codex TUI has!

---

## 🎯 ESTIMATED TIME

- Phase 1 (Core): 8-12 hours
- Phase 2 (Menu): 6-8 hours  
- Phase 3 (Polish): 4-6 hours

**Total: 18-26 hours** (vs 200+ hours reimplementing!)

---

## ✅ COMPLETED

- [x] Copied slash_command.rs
- [x] Copied chatwidget_full.rs
- [x] Copied chatwidget/ modules
- [x] Copied app_event.rs

---

## 🎯 PRIORITY 1: Core Event Handlers (Critical)

### Event Processing - Add Missing Handlers
- [ ] `AgentMessageDelta` - Streaming message deltas (CRITICAL for streaming)
- [ ] `AgentReasoning/AgentReasoningDelta` - Reasoning tokens (o1, DeepSeek)
- [ ] `AgentReasoningRawContent` - Raw reasoning content
- [ ] `AgentReasoningSectionBreak` - Reasoning section breaks
- [ ] `TokenCount` - Token usage tracking (show in status)
- [ ] `Warning` - Warning messages (show as toast)
- [ ] `TurnAborted` - Interrupted turns (show in UI)
- [ ] `TurnStarted` - Turn initialization (set loading state)
- [ ] `ThreadNameUpdated` - Thread rename events (update title)
- [ ] `StreamError` - Stream error handling (show error)

### Approval System
- [ ] `ExecApprovalRequest` - Command execution approval → Show DX menu approval dialog
- [ ] `ApplyPatchApprovalRequest` - Patch approval → Show DX menu approval dialog
- [ ] Add approval policy state to ChatState
- [ ] Create approval menu in `menu/submenus/approval_policy.rs`
- [ ] Implement approval response via Op channel

### Tool/Execution Events
- [ ] `ExecCommandBegin` - Command execution start → Show in message
- [ ] `ExecCommandEnd` - Command execution end → Update message
- [ ] `ExecCommandOutputDelta` - Command output streaming → Append to message
- [ ] `TerminalInteraction` - Terminal interaction → Show in message
- [ ] `PatchApplyBegin/End` - Patch application → Show progress

---

## 🎯 PRIORITY 2: Skills & Plugins (High Value)

### Skills System
- [ ] `ListSkillsResponseEvent` - Skills list → Store in ChatState
- [ ] Create skills state: `skills_all: Vec<ProtocolSkillMetadata>`
- [ ] Implement `menu/submenus/skills.rs`:
  - [ ] "1. Per-Skill Toggle" → Show list with checkboxes
  - [ ] "2. Skill Path" → Show skill file paths
  - [ ] "3. Scan Directories" → Trigger skill rescan
- [ ] Add skill enable/disable via Op channel
- [ ] Show skills in menu when available

### Plugin System
- [ ] `PluginListResponse` - Plugin list → Store in ChatState
- [ ] `PluginReadResponse` - Plugin details → Show in menu
- [ ] `PluginInstallResponse` - Installation result → Show toast
- [ ] `PluginUninstallResponse` - Uninstall result → Show toast
- [ ] Create plugin state: `plugins_cache: PluginsCacheState`
- [ ] Implement `menu/submenus/plugins_apps.rs`:
  - [ ] "1. Plugin Management" → Install/uninstall UI
  - [ ] "2. Marketplace Discovery" → Browse plugins
  - [ ] "3. Connector Apps" → Manage apps
  - [ ] "4. Suggestion Allowlist" → Configure allowlist
- [ ] Connect to app-server plugin API
- [ ] Show plugin marketplace in DX menu

---

## 🎯 PRIORITY 3: MCP & Advanced Features

### MCP Integration
- [ ] `McpStartupUpdate` - MCP server startup → Show progress
- [ ] `McpStartupComplete` - MCP startup done → Show toast
- [ ] `McpToolCallBegin` - MCP tool start → Show in message
- [ ] `McpToolCallEnd` - MCP tool end → Update message
- [ ] `McpListToolsResponseEvent` - MCP tools list → Show in menu
- [ ] Create MCP menu in `menu/submenus/mcp_servers.rs`

### Plan Mode
- [ ] `PlanDelta` - Plan streaming → Show in message
- [ ] `PlanUpdate` - Plan updates → Update message
- [ ] Add plan mode toggle to menu
- [ ] Show plan implementation prompt

### Collaboration Mode
- [ ] `CollabAgentSpawnBeginEvent` - Agent spawn → Show toast
- [ ] Add collaboration mode to menu
- [ ] Show agent switching UI

### Review Mode
- [ ] `ExitedReviewModeEvent` - Review exit → Update state
- [ ] Add review mode to menu
- [ ] Show review target selection

---

## 🎯 PRIORITY 4: UI Polish & Status

### Status Display
- [ ] Show token usage in status bar
- [ ] Show rate limits in status bar
- [ ] Show credits snapshot
- [ ] Show model info
- [ ] Show session info
- [ ] Add status menu item

### Image Support
- [ ] `ViewImageToolCall` - Image viewing → Show in message
- [ ] `ImageGenerationBegin` - Image gen start → Show progress
- [ ] `ImageGenerationEnd` - Image gen end → Show result

### Web Search
- [ ] `WebSearchBegin` - Search start → Show progress
- [ ] `WebSearchEnd` - Search end → Show results

### User Input Requests
- [ ] `RequestUserInput` - User input request → Show input dialog
- [ ] `RequestPermissions` - Permission request → Show permission dialog
- [ ] `ElicitationRequest` - Elicitation → Show dialog

### Guardian/Moderation
- [ ] `GuardianAssessment` - Content moderation → Show warning if needed
- [ ] Handle moderation status

### Deprecation & Warnings
- [ ] `DeprecationNotice` - Deprecation → Show warning toast
- [ ] `BackgroundEvent` - Background events → Log or show

### Undo System
- [ ] `UndoStarted` - Undo start → Show progress
- [ ] `UndoCompleted` - Undo done → Show toast

### Diff Display
- [ ] `TurnDiff` - Turn diff → Show in message or popup

---

## 🎯 PRIORITY 5: Session Management

### Session Operations
- [ ] Session resume → Use existing Codex resume picker
- [ ] Session fork → Send Fork op
- [ ] Session history → Show in menu
- [ ] Thread switching → Show thread list
- [ ] Thread naming → Show rename dialog

---

## 🎯 PRIORITY 6: Slash Commands (Optional - Backend handles these)

Note: Slash commands are processed by Codex backend. We just need to:
- [ ] Add slash command autocomplete in input
- [ ] Show slash command help menu
- [ ] Forward slash commands to Codex (already works via Op::UserMessage)

Most useful commands to highlight in UI:
- [ ] /model - Model selection (we have menu for this)
- [ ] /skills - Skills (we'll have menu for this)
- [ ] /plugins - Plugins (we'll have menu for this)
- [ ] /status - Status (we'll have menu for this)
- [ ] /review - Code review
- [ ] /diff - Git diff
- [ ] /new - New session
- [ ] /resume - Resume session

---

## 🎯 PRIORITY 7: External Editor (80% Done)

- [x] Editor command resolution
- [x] Ctrl+E keyboard shortcut
- [ ] Terminal suspend/resume
- [ ] Editor spawning
- [ ] Result insertion

---

## 📊 IMPLEMENTATION STRATEGY

### Phase 1: Core Events (Week 1)
1. Add all missing event handlers to `state.rs`
2. Update message rendering for new event types
3. Add token/status display

### Phase 2: Approval & Skills (Week 2)
1. Implement approval menu and dialogs
2. Implement skills menu and toggle UI
3. Test approval flow end-to-end

### Phase 3: Plugins & MCP (Week 3)
1. Implement plugin marketplace menu
2. Implement MCP server menu
3. Test plugin installation flow

### Phase 4: Polish & Advanced (Week 4)
1. Add plan mode, collaboration mode, review mode
2. Add image support, web search
3. Add session management UI
4. Complete external editor

---

## 🎯 QUICK WINS (Do These First)

These are easy to implement and high value:

1. **AgentMessageDelta** - Fix streaming (5 min)
2. **TokenCount** - Show token usage (10 min)
3. **Warning** - Show warnings as toasts (5 min)
4. **TurnAborted** - Show interruption (5 min)
5. **ExecCommandBegin/End** - Show command execution (15 min)
6. **McpStartupUpdate** - Show MCP startup (10 min)
7. **ListSkillsResponseEvent** - Store skills list (10 min)
8. **PluginListResponse** - Store plugins list (10 min)

Total: ~70 minutes to add 8 critical features!

---

## 📝 NOTES

- We're NOT reimplementing backend logic
- We're just adding UI for events we're already receiving
- Most events just need: match arm + UI update + toast/message
- Menu integration is straightforward - we have the menu system
- Estimated total work: 40-60 hours (not 200-300!)

---

## ✅ COMPLETED

- [x] Basic Codex backend connection
- [x] SessionConfigured event
- [x] AssistantMessage event (basic)
- [x] ToolUse event (basic)
- [x] ToolResult event
- [x] Error event
- [x] TurnComplete event
- [x] ShutdownComplete event
- [x] Streaming responses (basic)
- [x] Tool execution tracking (basic)
- [x] External editor (80%)
