# DX-TUI Integration Plan for Codex-TUI-DX

## Current Status
- ✅ DX splash screen showing with rainbow animation
- ❌ Font cycling not working (trying to duplicate logic instead of using DX dispatcher)
- ❌ Not using DX's core architecture (bridge, dispatcher, state)

## DX-TUI Architecture (What We Need to Integrate)

### 1. **Core Components**
```
DX-TUI Structure:
├── bridge.rs          - YaziChatBridge (connects ChatState with file browser)
├── state.rs           - ChatState (all DX state: animations, theme, input, etc.)
├── dispatcher.rs      - Event handling & update logic (font cycling, animations, etc.)
├── root.rs            - Root widget (renders everything)
├── dx_render.rs       - ChatState render methods
└── splash.rs          - Splash screen rendering
```

### 2. **What Each Component Does**

#### **ChatState (state.rs)**
- Holds ALL DX state:
  - Theme, animations, effects (rainbow, shimmer, typing)
  - Input state, messages, LLM
  - Font cycling (splash_font_index, last_font_change)
  - Animation mode, current animation
  - Menu, autocomplete, model picker
  - Scroll offsets, mouse tracking
  - Audio player, performance monitor

#### **YaziChatBridge (bridge.rs)**
- Connects ChatState with file browser (yazi)
- Manages AppMode (Chat vs FilePicker)
- Provides methods to switch modes

#### **Dispatcher (dispatcher.rs)**
- Handles ALL events: Key, Mouse, Timer, Resize, Focus, Paste
- Contains UPDATE LOGIC:
  - Font cycling every 5 seconds
  - Menu updates
  - Animation updates
  - Input handling
  - Scroll adjustments
- Calls `chat_state.update()` on timer

#### **Root (root.rs)**
- Top-level widget that renders everything
- Decides what to show based on state
- Renders animations, file picker, or chat

#### **dx_render.rs**
- ChatState render methods
- Handles all rendering logic for DX screens

## Integration Strategy for Codex-TUI-DX

### Phase 1: Use DX Dispatcher for Updates ✅ NEXT
**Goal**: Stop duplicating update logic, use DX dispatcher

**Changes Needed**:
1. In `src/codex.rs` (codex-tui-dx binary):
   - Import DX dispatcher
   - Call dispatcher.dispatch_timer() periodically
   - This handles font cycling, menu updates, etc.

2. Remove duplicate logic from chatwidget.rs:
   - Remove manual font cycling code
   - Let dispatcher handle it

**Files to Modify**:
- `src/codex.rs` - Add timer event dispatch
- `src/chatwidget.rs` - Remove duplicate font cycling

### Phase 2: Integrate DX Event Handling
**Goal**: Use DX dispatcher for key/mouse events on splash screen

**Changes Needed**:
1. When showing DX splash in ChatWidget:
   - Pass key events to DX dispatcher
   - Let DX handle Ctrl+. and other shortcuts
   
2. Add event routing:
   - If showing splash → route to DX dispatcher
   - Otherwise → route to codex handlers

**Files to Modify**:
- `src/chatwidget.rs` - Route events to DX dispatcher when showing splash

### Phase 3: Full DX Integration (Optional)
**Goal**: Use DX Root widget for complete DX experience

**Changes Needed**:
1. Replace splash rendering with DX Root widget
2. This gives access to:
   - Animation carousel
   - File picker integration
   - Full DX menu system
   - All DX effects and animations

**Files to Modify**:
- `src/chatwidget.rs` - Use Root widget instead of just splash::render

## Key Principles

1. **Use Real DX Code**: Never duplicate DX logic
2. **Dispatcher is Key**: All update logic lives in dispatcher
3. **ChatState is Source of Truth**: All DX state in one place
4. **Event Routing**: Route events to DX when showing DX screens

## Current Problem

We're calling `crate::splash::render()` directly (✅ good) but:
- ❌ Duplicating font cycling logic in chatwidget.rs
- ❌ Not using dispatcher for updates
- ❌ Not routing events to DX

## Solution

Use DX dispatcher! It already has:
- Font cycling every 5 seconds
- Menu updates
- Animation updates
- All event handling

Just need to:
1. Call `dispatcher.dispatch_timer()` periodically
2. Route key events to `dispatcher.dispatch_key()` when showing splash
3. Remove duplicate logic

## Files That Need Changes

### Immediate (Phase 1):
- [ ] `src/codex.rs` - Add timer dispatch
- [ ] `src/chatwidget.rs` - Remove duplicate font cycling, use dispatcher

### Later (Phase 2):
- [ ] `src/chatwidget.rs` - Route key events to dispatcher
- [ ] `src/dispatcher.rs` - Maybe adapt for codex-tui context

### Optional (Phase 3):
- [ ] `src/chatwidget.rs` - Use Root widget for full DX experience
