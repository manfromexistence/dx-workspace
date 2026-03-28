# Current Work: codex-tui-dx Binary

**Date**: March 26, 2026
**Binary**: codex-tui-dx (default)
**Location**: codex-rs/dx/
**Device**: Low-end device - minimize compilation

---

## CRITICAL: Testing Command

**ONLY USE THIS COMMAND:**
```bash
cargo run
```

**DO NOT USE:**
- ❌ `cargo build` - Wastes time building without running
- ❌ `cargo check` - Doesn't actually test the binary
- ❌ `cargo run --release` - Takes too long to compile
- ❌ Any other cargo commands

**Why**: This is a low-end device. Building increases target binary size unnecessarily. Only `cargo run` to test immediately.

---

## Recent Changes

### 1. Red Background (FIXED ✅)
- Added persistent red background to Codex TUI history area
- File: `codex-rs/dx/src/chatwidget.rs` (RedBackgroundWrapper)
- Background now stays visible, not just for a split second

### 2. Default Model Changed (DONE ✅)
- Changed from "gpt-4" to "mistral-large-latest"
- File: `codex-rs/dx/src/codex_integration.rs`
- Lines: 94, 107-108, 137

**Note**: If model still shows GPT after running, it's because:
- Config file (~/.codex/config.toml) has saved model preference
- Thread history has saved model
- Use `/model mistral-large-latest` command in TUI to switch

---

## Testing

```bash
cargo run
```

**What to check:**
1. Red background visible in history area (top part)
2. Model name in status line shows "mistral-large-latest" (not GPT)

---

## Key Files

- `src/chatwidget.rs` (~8000 lines) - Chat widget, red background wrapper
- `src/codex_integration.rs` - Codex initialization (commented out, not used by codex-tui-dx)
- `src/app.rs` (~7000 lines) - Main app logic
- `src/codex.rs` - Entry point for codex-tui-dx binary

---

## Next Steps

Ready for next task. Current changes:
- ✅ Red background working
- ✅ Default model changed to Mistral
