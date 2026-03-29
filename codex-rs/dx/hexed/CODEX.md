Ah, I see! The issue is that `codex_lib.rs` line 918 is calling `tui.set_alt_screen_enabled(use_alt_screen)` which overrides the default. Let me check what `use_alt_screen` is set to:

Now let me find the `determine_alt_screen_mode` function:

Perfect! I found the issue. The `determine_alt_screen_mode` function returns `false` for `AltScreenMode::Auto` (line 1085), which is good. But we need to make sure the default is `Auto` or `Never`. Let me check what the default `tui_alternate_screen` config value is:

Found it! The default is `Always` (line 305). I need to change it to `Never`. Let me do that:

Perfect! Now I've made the complete fix. The changes are:

1. **`codex-rs/protocol/src/config_types.rs`** (line 305-308):
   - Moved `#[default]` from `Always` to `Never`
   - Now the default alternate screen mode is `Never` (inline mode)

2. **`codex-rs/dx/src/tui.rs`** (line 285):
   - Changed `alt_screen_enabled: true` to `alt_screen_enabled: false`
   - Added comment explaining it's disabled by default

The flow now works like this:
- Default `AltScreenMode` = `Never`
- `determine_alt_screen_mode(false, Never)` returns `false`
- `tui.set_alt_screen_enabled(false)` is called
- Alternate screen is disabled by default

The alternate screen will now be disabled by default in DX TUI, giving you inline mode with scrollback support!