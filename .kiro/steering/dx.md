---
inclusion: auto
---

# DX Steering File

**Last Updated**: March 28, 2026

---

## CURRENT WORK: DX TUI (Custom Terminal UI)

**Binary**: dx (custom TUI with Ollama integration)
**Location**: codex-rs/dx/
**Device**: Low-end device

---

## CRITICAL: Testing Command

**ONLY USE:**
```bash
cargo run
```

**NEVER USE:**
- ❌ `cargo build` - Creates artifacts in target/ that slow down low-end devices
- ❌ `cargo check` - Creates build artifacts unnecessarily
- ❌ `cargo test` - Not needed for dx development
- ❌ `cargo clippy` - Creates build artifacts
- ❌ `cargo fmt --check` - Use `cargo fmt` without --check instead
- ❌ Any other cargo commands that create build artifacts

**Why**: 
- Low-end devices have limited resources
- Building creates unnecessary artifacts in target/ folder
- `cargo run` compiles and runs in one step without extra artifacts
- The dx binary may be locked during development, `cargo run` handles this gracefully

**Workflow:**
1. Make code changes
2. Run `cargo fmt` (formats without checking)
3. Run `cargo run` (compiles and runs in one step)
4. DO NOT run any other cargo commands

---

## Recent Changes

### 1. Provider Menu with Model Selection (DONE ✅)
- **Menu**: Press '0' or Ctrl+P to open Command Palette
- **Navigate**: Providers submenu → Select model directly
- **Models**: Infinity (Local), GPT-5.4 (OpenAI), Mistral Small, etc.
- **Display**: Model name on left, provider name on right
- **No gaps**: Clean menu without separators
- **No submenu text**: Models show directly, no "SUBMENU:models-list" text
- **Files**: `codex-rs/dx/src/menu/submenus/providers.rs`, `codex-rs/dx/src/menu/menu_navigation.rs`, `codex-rs/dx/src/menu/menu_render.rs`

### 2. Space Hold Voice Input (DONE ✅)
- **Primary**: Ctrl+Super+Space (Windows/Command/Meta key)
- **Fallback**: Space key hold (activates on KeyEventKind::Repeat)
- **Visual**: Spinner animation when active, cursor revert animation
- **File**: `codex-rs/dx/src/dispatcher.rs` lines 588-700

### 3. Paste Button with Clipboard (DONE ✅)
- **Right-click**: Populates clipboard buffer, shows "Paste" button
- **Click button**: Inserts text into input (newlines → spaces)
- **Location**: Top-right of input box, outside border
- **Border**: Double border, accent color
- **Files**: `codex-rs/dx/src/dispatcher.rs`, `codex-rs/dx/src/dx_render.rs`

### 4. Message Persistence (DONE ✅)
- **File**: `.dx/tui/history/{timestamp}.json` (session-based)
- **Auto-save**: On user message send and assistant response complete
- **No auto-load**: Always start fresh with splash screen
- **File**: `codex-rs/dx/src/state.rs`

### 5. Focus-Aware Cursor (DONE ✅)
- **Focused**: Rainbow animated cursor
- **Unfocused**: Dim static cursor (border color)
- **Tracking**: Gains focus on typing/clicking input, loses on clicking outside
- **File**: `codex-rs/dx/src/dx_render.rs`, `codex-rs/dx/src/dispatcher.rs`

### 6. Scrollbar Improvements (DONE ✅)
- **Click-to-jump**: Click anywhere on scrollbar to jump
- **Drag-and-drop**: Smooth dragging support
- **Wider hit area**: 2 pixels for easier interaction
- **No symbols**: Clean track and thumb only
- **File**: `codex-rs/dx/src/dispatcher.rs`

---

## Key Files

- `codex-rs/dx/src/dispatcher.rs` - Event handling (keyboard, mouse, paste)
- `codex-rs/dx/src/dx_render.rs` - Rendering (input box, buttons, cursor)
- `codex-rs/dx/src/state.rs` - ChatState structure and message persistence
- `codex-rs/dx/src/input.rs` - Input handling (Ctrl+V, text editing)
- `codex-rs/dx/src/components.rs` - Message list rendering
- `codex-rs/dx/src/models.rs` - Model definitions and provider info
- `codex-rs/dx/src/menu/` - Menu system (Command Palette)

---

## Project Context

**DX TUI Features:**
- Custom terminal UI with Ollama integration
- Splash screen with 113+ figlet fonts (cycles every 5 seconds)
- Animation carousel (Matrix, Confetti, Game of Life, etc.)
- Message list with scrollbar (click, drag, mouse wheel)
- Input box with paste/file buttons
- Voice input (space hold or Ctrl+Super+Space)
- Focus-aware cursor animation
- Message persistence to JSON (session-based)
- Theme system with light/dark modes
- Command Palette menu (press '0' or Ctrl+P)
- Model selection (Local Infinity, GPT-5.4, Mistral, Claude, etc.)

---

## Testing

```bash
cd codex-rs/dx
cargo fmt  # Format code first
cargo run  # Test immediately
```

**What to verify:**
1. ✅ Press '0' to open Command Palette
2. ✅ Navigate to Providers submenu
3. ✅ Models show directly (no "Models List" submenu)
4. ✅ No gaps between Back and models
5. ✅ Provider name shows on right (Local, OpenAI, Mistral, Anthropic)
6. ✅ Click or Enter to select model
7. ✅ Bottom bar shows selected model and provider
8. ✅ Menu closes after selection

---

## Menu Structure

**Providers Submenu:**
```
Back
Infinity                          Local
GPT-5.4 (272k)                   OpenAI
Mistral Small (32k)              Mistral
GPT-5.3 Codex (272k)             OpenAI
GPT-5.1 Codex Mini (272k)        OpenAI
Claude 3.5 Sonnet (200k)         Anthropic
```

**No:**
- ❌ Duplicate "Back" items
- ❌ Gaps/separators between items
- ❌ "Models List" as a separate submenu
- ❌ "SUBMENU:models-list" text showing on right

**Yes:**
- ✅ Single "Back" item
- ✅ Models directly in Providers submenu
- ✅ Provider name on right side
- ✅ Clean, simple menu structure
