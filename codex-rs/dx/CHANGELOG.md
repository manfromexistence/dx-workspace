# Changelog - DX-TUI Integration into Codex-TUI-DX

All notable changes to the dx-tui integration will be documented in this file.

## [2026-03-29] - DX-TUI Integration Phase 1

### Added - Font Cycling with Ctrl+. (Latest)
- **Ctrl+. Font Cycling** (`src/chatwidget.rs` handle_key_event)
  - Added Ctrl+. key handler at the beginning of handle_key_event()
  - Only triggers when showing welcome screen (transcript_cells.is_empty())
  - Cycles through 113 valid DX splash fonts
  - Uses dx_chat_state.splash_font_index directly (no duplicate code)
  - Updates dx_chat_state.last_font_change timestamp
  - Schedules frame for immediate re-render
  - Follows same pattern as onboarding welcome screen

### Changed - Use DX ChatState Directly
- **Replaced Custom Fields with DX ChatState** (`src/chatwidget.rs`)
  - Removed custom fields: `dx_rainbow_effect`, `dx_splash_font_index`, `dx_last_animation_update`
  - Added `dx_chat_state: RefCell<ChatState>` to ChatWidget struct
  - Updated all 3 constructors (lines ~3545, ~3747, ~3944) to initialize `dx_chat_state`
  - Now using the real DX ChatState implementation instead of duplicate code
  - ChatState contains: rainbow_effect, splash_font_index, last_animation_update, and more
  - This follows the user's directive: "use dx-tui code directly, don't create any slop"

### Added - Animation Support
- **Rainbow Animation State** (`src/chatwidget.rs`)
  - Added `dx_rainbow_effect: RefCell<RainbowEffect>` to ChatWidget struct
  - Added `dx_splash_font_index: Cell<usize>` for font cycling
  - Added `dx_last_animation_update: Cell<Instant>` for frame timing
  - Rainbow colors now animate smoothly (updates every 50ms)
  - Frame scheduling triggers continuous re-renders for animation

- **Animated Splash Rendering** (`src/chatwidget.rs` render method)
  - Rainbow effect now uses persistent state instead of creating new instance
  - Calls `frame_requester.schedule_frame()` on EVERY render when showing welcome
  - RainbowEffect auto-updates based on elapsed time (no manual update needed)
  - Font index ready for Ctrl+. cycling (to be implemented)
  - Removed 50ms throttling - now schedules frames continuously for smooth animation

### Added
- **DX-TUI Module Integration** (`src/codex_lib.rs`)
  - Exposed all dx-tui modules as public: `animations`, `audio`, `autocomplete`, `bridge`, `chat`, `chat_components`, `chat_input`, `components`, `dispatcher`, `dx_render`, `effects`, `exit_animation`, `external_editor`, `font`, `input`, `llm`, `logs`, `menu`, `modal`, `model_manager`, `models`, `panic`, `perf`, `root`, `scrollbar`, `signals`, `splash`, `state`, `theme`
  - Added `file_browser` module for yazi integration
  - Removed duplicate module declarations for `external_editor` and `scrollbar`

- **DX Splash Screen in Onboarding** (`src/onboarding/welcome.rs`)
  - Replaced codex welcome screen with DX splash screen
  - Added `RainbowEffect` for animated rainbow colors
  - Added `splash_font_index` to cycle through different figlet fonts
  - Ctrl+. now cycles through DX splash fonts instead of codex animations
  - Shows "DX" in rainbow figlet art with "Enhanced Development Experience" subtitle

- **DX Splash Screen in ChatWidget** (`src/chatwidget.rs`)
  - Replaced "Welcome to Codex" message with DX splash screen
  - DX splash renders directly to buffer when no transcript cells exist
  - Added imports: `RainbowEffect`, `splash`, `ChatTheme`, `ThemeVariant`
  - Wrapped transcript paragraph rendering in `if !show_welcome` condition
  - Made `transcript_content_height` an expression that returns 0 when showing welcome

### Changed
- **Cursor Visibility** (`src/chatwidget.rs`)
  - Modified `cursor_pos()` to return `None` when showing welcome screen (no transcript cells)
  - Cursor now hidden until user starts interacting with the app

- **Import Organization** (`src/dispatcher.rs`)
  - Added missing imports: `AnimationType`, `AppMode`, `InputAction`, `MenuAction`, `Root`
  - Replaced all `crate::AnimationType::` references with direct `AnimationType::`
  - Replaced all `crate::AppMode::` references with direct `AppMode::`
  - Removed duplicate `AnimationType` import

- **File Browser Integration** (`src/file_browser/executor.rs`)
  - Added `Root` import
  - Replaced `crate::Root::reflow` with direct `Root::reflow`

### Commented Out
- **DX Input Handling** (`src/dispatcher.rs`)
  - Commented out dx input handling (lines 788-860) to use codex-tui bottom pane
  - Kept codex bottom pane for input while integrating dx screens

### Technical Details

#### Files Modified
1. `src/codex_lib.rs` - Module declarations and exports
2. `src/onboarding/welcome.rs` - Welcome screen rendering
3. `src/chatwidget.rs` - Main chat widget welcome screen and cursor
4. `src/dispatcher.rs` - Input handling and imports
5. `src/file_browser/executor.rs` - Root import
6. `TODO.md` - Progress tracking

#### Revert Instructions
If issues arise, revert these changes in order:

1. **Revert ChatWidget splash** (`src/chatwidget.rs` lines ~8910-8945):
   - Replace DX splash rendering with original "Welcome to Codex" message
   - Remove `RainbowEffect`, `splash`, `ChatTheme` imports
   - Unwrap `transcript_content_height` from if-expression

2. **Revert cursor hiding** (`src/chatwidget.rs` line ~9043):
   - Remove the `if self.transcript_cells.is_empty()` check in `cursor_pos()`

3. **Revert onboarding splash** (`src/onboarding/welcome.rs`):
   - Replace `splash::render()` with original codex welcome text
   - Remove `RainbowEffect` and `splash` imports
   - Remove `splash_font_index` field

4. **Revert module exports** (`src/codex_lib.rs`):
   - Remove `pub mod` declarations for dx-tui modules
   - Keep only codex-tui modules

### Next Steps
- [ ] Add animation carousel (Ctrl+. to cycle through DX animations)
- [ ] Wire up theme system
- [ ] Test each screen individually
- [ ] Integrate dx menu system
- [ ] Add dx status line
