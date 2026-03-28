# DX TUI - Codex Integration TODO

## 📊 OVERALL PROGRESS: 33% Complete

**Completed:** 20 / 60 tasks
**In Progress:** 1 / 60 tasks  
**Remaining:** 39 / 60 tasks

---

## 🎯 STRATEGY: Use Codex TUI Code Directly

Instead of reimplementing features, we're taking Codex TUI's complete `ChatWidget` (which has ALL the logic) and just replacing its rendering with DX's superior UI.

**Files copied:**
- ✅ `slash_command.rs` - All slash commands
- ✅ `chatwidget_full.rs` - Complete ChatWidget with ALL event handlers
- ✅ `chatwidget/` - All ChatWidget modules (skills, plugins, agent, etc.)
- ✅ `app_event.rs` - Event types

---

## 📋 DETAILED TASK LIST

### PHASE 1: Core Integration (Priority 1) - 33% Complete (5/15 tasks)

#### Module Setup
- [x] 1.1. Add `mod slash_command;` to dx.rs ✅
- [x] 1.2. Add `mod app_event;` to dx.rs ✅
- [x] 1.3. Add `mod chatwidget_full;` to dx.rs ✅
- [x] 1.4. Add `mod chatwidget;` to dx.rs ✅
- [x] 1.5. Update Cargo.toml dependencies if needed ✅ (All deps already present)

#### Remove Ratatui Code from ChatWidget
- [ ] 1.6. REVISED: Create minimal ChatWidget wrapper instead of modifying full file
- [ ] 1.7. Extract event handling methods we need
- [ ] 1.8. Create DX-compatible interface
- [ ] 1.9. Keep slash command processing
- [ ] 1.10. Keep approval/skills/plugins logic

#### Add DX Rendering
- [ ] 1.11. Add `render_dx()` method to ChatWidget
- [ ] 1.12. Add `get_messages_for_dx()` method
- [ ] 1.13. Add `get_menu_items_for_dx()` method
- [ ] 1.14. Add DX theme integration

#### Wire Up Event Loop
- [ ] 1.15. Integrate ChatWidget with existing ChatState

**Phase 1 Progress: 33% (5/15)** ⬜⬜⬜⬜⬜🟦⬜⬜⬜⬜⬜⬜⬜⬜⬜

---

### PHASE 2: Menu Integration (Priority 2) - 0% Complete (0/20 tasks)

#### Approval System Integration
- [ ] 2.1. Map `ExecApprovalRequest` to DX menu
- [ ] 2.2. Map `ApplyPatchApprovalRequest` to DX menu
- [ ] 2.3. Add approval response via Op channel
- [ ] 2.4. Implement approval menu in `menu/submenus/approval_policy.rs`
- [ ] 2.5. Test approval flow end-to-end

#### Skills System Integration
- [ ] 2.6. Map skills list to DX menu
- [ ] 2.7. Implement "Per-Skill Toggle" in skills submenu
- [ ] 2.8. Implement "Skill Path" display
- [ ] 2.9. Implement "Scan Directories" action
- [ ] 2.10. Test skills enable/disable

#### Plugin System Integration
- [ ] 2.11. Map plugin marketplace to DX menu
- [ ] 2.12. Implement "Plugin Management" submenu
- [ ] 2.13. Implement "Marketplace Discovery" submenu
- [ ] 2.14. Implement plugin install flow
- [ ] 2.15. Implement plugin uninstall flow
- [ ] 2.16. Test plugin installation end-to-end

#### MCP Integration
- [ ] 2.17. Map MCP tools list to DX menu
- [ ] 2.18. Implement MCP servers submenu
- [ ] 2.19. Show MCP startup status
- [ ] 2.20. Test MCP tool execution

**Phase 2 Progress: 0% (0/20)**

---

### PHASE 3: Advanced Features (Priority 3) - 0% Complete (0/15 tasks)

#### Session Management
- [ ] 3.1. Add session resume UI
- [ ] 3.2. Add session fork UI
- [ ] 3.3. Add session history browser
- [ ] 3.4. Add thread switching UI
- [ ] 3.5. Test session operations

#### Status Display
- [ ] 3.6. Add token usage to status bar
- [ ] 3.7. Add rate limits to status bar
- [ ] 3.8. Add credits snapshot display
- [ ] 3.9. Add model info display
- [ ] 3.10. Test status updates

#### Advanced Modes
- [ ] 3.11. Add plan mode UI
- [ ] 3.12. Add collaboration mode UI
- [ ] 3.13. Add review mode UI
- [ ] 3.14. Add reasoning display (o1, DeepSeek)
- [ ] 3.15. Test advanced modes

**Phase 3 Progress: 0% (0/15)**

---

### PHASE 4: Polish & Testing (Priority 4) - 0% Complete (0/10 tasks)

#### UI Polish
- [ ] 4.1. Add slash command autocomplete
- [ ] 4.2. Add command help menu
- [ ] 4.3. Improve error messages
- [ ] 4.4. Add loading indicators
- [ ] 4.5. Polish animations

#### Testing
- [ ] 4.6. Test all slash commands
- [ ] 4.7. Test all event types
- [ ] 4.8. Test approval flow
- [ ] 4.9. Test skills/plugins
- [ ] 4.10. End-to-end testing

**Phase 4 Progress: 0% (0/10)**

---

## ✅ COMPLETED TASKS (15 tasks)

### Setup & Preparation
- [x] 0.1. Created TODO.md with task breakdown
- [x] 0.2. Created INTEGRATION_PLAN.md
- [x] 0.3. Analyzed Codex TUI architecture
- [x] 0.4. Identified files to copy

### File Copying
- [x] 0.5. Copied slash_command.rs from Codex TUI
- [x] 0.6. Copied chatwidget_full.rs from Codex TUI
- [x] 0.7. Copied chatwidget/ directory from Codex TUI
- [x] 0.8. Copied app_event.rs from Codex TUI

### Previous Work
- [x] 0.9. Basic Codex backend connection
- [x] 0.10. SessionConfigured event handler
- [x] 0.11. Basic streaming responses
- [x] 0.12. Tool execution tracking
- [x] 0.13. External editor (80% - needs terminal handling)
- [x] 0.14. Audio system for animations
- [x] 0.15. Local LLM removal

---

## 📈 PROGRESS BY PHASE

| Phase | Tasks | Complete | In Progress | Remaining | Progress |
|-------|-------|----------|-------------|-----------|----------|
| Setup | 15 | 15 | 0 | 0 | 100% ✅ |
| Phase 1 | 15 | 4 | 0 | 11 | 27% 🟦 |
| Phase 2 | 20 | 0 | 0 | 20 | 0% ⬜ |
| Phase 3 | 15 | 0 | 0 | 15 | 0% ⬜ |
| Phase 4 | 10 | 0 | 0 | 10 | 0% ⬜ |
| **TOTAL** | **75** | **19** | **0** | **56** | **25%** |

---

## 🎯 NEXT IMMEDIATE TASKS

Starting Phase 1 - Core Integration:

1. **Task 1.1** - Add module declarations (5 min)
2. **Task 1.6-1.10** - Remove ratatui code (30 min)
3. **Task 1.11-1.14** - Add DX rendering (60 min)
4. **Task 1.15** - Wire up event loop (30 min)

**Estimated time for Phase 1:** 2-3 hours

---

## 🔥 CRITICAL PATH

To get DX TUI working with full Codex features:

**Week 1:** Phase 1 (Core Integration) → 25% → 40%
**Week 2:** Phase 2 (Menu Integration) → 40% → 70%
**Week 3:** Phase 3 (Advanced Features) → 70% → 90%
**Week 4:** Phase 4 (Polish & Testing) → 90% → 100%

---

## 📝 NOTES

- We're NOT reimplementing backend logic
- We're just swapping UI rendering
- ChatWidget has ALL the features already
- Estimated total: 18-26 hours to 100%

---

**Last Updated:** Now
**Current Task:** Starting Phase 1 - Module Setup

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
