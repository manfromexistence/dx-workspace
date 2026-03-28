# Help File: Two Critical Issues in codex-tui-dx Binary

## Project Context

**Working Directory**: `F:\codex\codex-rs\dx\`
**Binary**: `codex-tui-dx` (default binary in dx folder)
**Entry Point**: `codex-rs/dx/src/codex.rs`
**Device**: Low-end device - ONLY use `cargo run` command, NEVER use `cargo build`, `cargo check`, or `cargo run --release`
**UI Framework**: ratatui (Rust TUI library)

## Project Structure

The `codex-rs/dx/` folder contains TWO binaries:
1. **codex-tui-dx** (`src/codex.rs`) - Full working Codex TUI - THIS IS WHAT WE'RE MODIFYING
2. **dx** (`src/dx.rs`) - Custom DX TUI with Codex integration (currently commented out, not being used)

**Strategy**: Add DX features to codex-tui-dx, not the other way around.

## Issue 1: Red Background Disappearing in History Area

### Problem Description
A red background was added to the history area (top part) of the Codex TUI. The red background shows for a split second when rendering, but then disappears/gets overwritten by the actual content rendering.

### Current Implementation Location
**File**: `codex-rs/dx/src/chatwidget.rs`
**Lines**: 8545-8670

### Current Code Structure

```rust
// Around line 8610 - RedBackgroundWrapper struct
struct RedBackgroundWrapper<'a, T: Renderable> {
    inner: &'a T,
}

impl<'a, T: Renderable> Renderable for RedBackgroundWrapper<'a, T> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        // FIRST: Apply red background to all cells in the area
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let cell = &mut buf[(x, y)];
                cell.set_bg(ratatui::style::Color::Red);
            }
        }
        
        // THEN: Render the inner content on top
        self.inner.render(area, buf);
        
        // FINALLY: Re-apply red background to ensure it's visible
        // This is needed because some widgets reset the background
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let cell = &mut buf[(x, y)];
                cell.set_bg(ratatui::style::Color::Red);
            }
        }
    }

    fn desired_height(&self, width: u16) -> u16 {
        self.inner.desired_height(width)
    }

    fn cursor_pos(&self, area: Rect) -> Option<(u16, u16)> {
        self.inner.cursor_pos(area)
    }
}

// Around line 8650 - EmptyRedBackground struct
struct EmptyRedBackground;

impl Renderable for EmptyRedBackground {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let cell = &mut buf[(x, y)];
                cell.set_bg(ratatui::style::Color::Red);
                cell.set_char(' ');
            }
        }
    }

    fn desired_height(&self, _width: u16) -> u16 {
        0
    }

    fn cursor_pos(&self, _area: Rect) -> Option<(u16, u16)> {
        None
    }
}

// Around line 8545 - as_renderable() method
fn as_renderable(&self) -> RenderableItem<'_> {
    let active_cell_renderable = match &self.active_cell {
        Some(cell) => {
            let red_bg_cell = RedBackgroundWrapper { inner: cell };
            RenderableItem::Owned(Box::new(red_bg_cell))
                .inset(Insets::tlbr(/*top*/ 1, /*left*/ 0, /*bottom*/ 0, /*right*/ 0))
        }
        None => {
            RenderableItem::Owned(Box::new(EmptyRedBackground))
        }
    };
    let mut flex = FlexRenderable::new();
    flex.push(/*flex*/ 1, active_cell_renderable);
    flex.push(
        /*flex*/ 0,
        RenderableItem::Borrowed(&self.bottom_pane)
            .inset(Insets::tlbr(/*top*/ 1, /*left*/ 0, /*bottom*/ 0, /*right*/ 0)),
    );
    RenderableItem::Owned(Box::new(flex))
}
```

### What's Been Tried
1. Applied red background BEFORE rendering content - didn't work
2. Applied red background AFTER rendering content - didn't work
3. Applied red background BOTH before AND after rendering - still disappears

### The Problem - RESEARCH FINDINGS
The inner content rendering (`self.inner.render(area, buf)`) is somehow resetting or overwriting the background color. Even applying the red background after rendering doesn't persist - something in the rendering pipeline is clearing it.

**Key Research Findings:**
1. The `RedBackgroundWrapper` correctly re-applies background AFTER inner render (lines 8605-8660)
2. The rendering pipeline uses `FlexRenderable` + `InsetRenderable` for layout (see `codex-rs/dx/src/render/renderable.rs`)
3. The `as_renderable()` method wraps only the `active_cell_renderable` (history), then `FlexRenderable` renders `bottom_pane` as a sibling
4. **CRITICAL**: Ratatui's `Terminal::draw()` does a **diff** against the previous frame buffer - if previous frame had same cells without red background, ratatui emits the delta
5. The post-render loop runs BEFORE `FlexRenderable` renders sibling children (`bottom_pane`)
6. The background should work in theory, but something in the full widget render pipeline is overriding it after

**The Real Issue:**
The red background is being applied correctly, but either:
- The terminal diff algorithm is not detecting the background change
- Something in the rendering pipeline after `RedBackgroundWrapper` is clearing it
- The `FlexRenderable` or parent rendering is overwriting the cells

### Possible Solutions to Investigate
1. **Use a Block widget with red background** - Wrap the entire active_cell area in a ratatui Block with red bg
2. **Modify at a higher level** - Apply red background in the parent render, not in the wrapper
3. **Force style on every cell during render** - Instead of post-render loop, modify the actual content cells as they're rendered
4. **Check if InsetRenderable is clearing** - The `.inset()` call might be creating a new buffer that loses the background
5. **Apply background at the FlexRenderable level** - Modify FlexRenderable to support background colors for its children

### Key Files to Investigate
- `codex-rs/dx/src/chatwidget.rs` (lines 8545-8670) - Current implementation
- `codex-rs/dx/src/render/renderable.rs` (lines 208-406) - FlexRenderable and InsetRenderable implementation
- Look for where `active_cell` is created/populated
- Check how ratatui's Buffer and terminal diff works

---

## Issue 2: Default Model Still Shows as gpt-5.3-codex Instead of mistral-large-latest

### Problem Description
The user wants to change the default model from "gpt-5.3-codex-high" to "mistral-large-latest". Several hardcoded references have been changed, but when running `cargo run`, the model still shows as "gpt-5.3-codex".

### Changes Already Made

#### 1. codex-rs/core/src/codex.rs (Line 3360)
**Changed**: Error message from "gpt-5.2"/"gpt-5.3-codex" to "mistral-small"/"mistral-large-latest"

#### 2. codex-rs/app-server/src/outgoing_message.rs (Lines 816, 828)
**Changed**: Test cases from "gpt-5.3-codex"/"gpt-5.2" to "mistral-large-latest"/"mistral-small"

#### 3. codex-rs/dx/src/codex_integration.rs (Lines 94, 107-108, 137)
**Changed**: Default model from "gpt-4" to "mistral-large-latest"
**Note**: This file is only used by the `dx` binary, NOT `codex-tui-dx`

### Remaining gpt-5.3-codex References (Test Files Only)
**File**: `codex-rs/dx/src/chatwidget/tests.rs`
**Lines**: 5329, 5343, 8399, 8428, 8460, 10769
**Status**: These are test cases and likely don't affect the runtime default

### The Real Problem - RESEARCH FINDINGS
The model is loaded from the config file (`~/.codex/config.toml`) which persists the last selected model. Code defaults are only used when no config exists. The actual default model initialization happens somewhere in the app startup code.

**Key Research Findings:**

1. **models.json** (`codex-rs/core/models.json`) contains the model catalog
   - `mistral-small-latest` is at position 0 with priority 0
   - Models are sorted by priority ascending in `build_available_models()` (manager.rs:518-529)

2. **get_default_model()** (`codex-rs/core/src/models_manager/manager.rs:276-308`)
   - Line 305: Uses `.find(|model| model.is_default)` to find the default
   - Falls back to first model if none marked as default

3. **mark_default_by_picker_visibility()** (`codex-rs/protocol/src/openai_models.rs:461-473`)
   - Resets all models to `is_default = false`
   - Marks the FIRST picker-visible model as default
   - `mistral-small-latest` has `visibility: list` so it's picker-visible

4. **The Issue**: The dx binary may not be going through the `mark_default_by_picker_visibility` path, OR the config file already has `gpt-5.3-codex` saved and is overriding the default

### Key Code Locations

#### app.rs Line 2248-2251
```rust
let mut model = thread_manager
    .get_models_manager()
    .get_default_model(&config.model, RefreshStrategy::Offline)
    .await;
```
**This is the critical line** - `get_default_model(&config.model, ...)` passes the config's model as a preference

#### app.rs Line 2264-2267
```rust
if let Some(updated_model) = config.model.clone() {
    model = updated_model;
}
```
**This overrides the default** - If config.model exists, it replaces the default

#### manager.rs Line 280-320 (get_default_model implementation)
```rust
pub async fn get_default_model(
    &self,
    preferred_model: &Option<String>,
    refresh_strategy: RefreshStrategy,
) -> String {
    // ... loads catalog ...
    
    // Line 305: Find model marked as default
    let default_model = models.iter().find(|model| model.is_default);
    
    // Returns preferred_model if valid, else default, else first model
}
```

#### config.rs (codex-rs/core/src/config/mod.rs)
- Config is loaded from `~/.codex/config.toml`
- `pub model: Option<String>` field stores the last used model
- This persists across sessions

### What Needs to Be Done
1. **Check if config file exists** - The user's `~/.codex/config.toml` likely has `model = "gpt-5.3-codex"` saved
2. **Either**: Delete the config file to force default, OR
3. **Modify the default in models.json** - Change which model has `is_default = true`, OR
4. **Modify mark_default_by_picker_visibility** - Ensure it selects `mistral-large-latest` instead of `mistral-small-latest`, OR
5. **Override in app.rs** - Force `config.model = Some("mistral-large-latest".to_string())` if it's currently `gpt-5.3-codex`

### Files to Investigate
- `codex-rs/dx/src/app.rs` - Main app initialization (lines 2200-2400)
- `codex-rs/core/src/config.rs` - Config loading and defaults
- `codex-rs/core/src/models.rs` or similar - Where `get_default_model()` is implemented
- Look for `Config::load()` or similar config initialization
- Look for where `ModelManager` or `ModelsManager` sets defaults

### Search Queries to Run
```bash
# Find where config.model gets its initial value
rg "config\.model\s*=" codex-rs/dx/src/
rg "config\.model\s*=" codex-rs/core/src/

# Find get_default_model implementation
rg "fn get_default_model" codex-rs/

# Find config loading
rg "Config::load" codex-rs/dx/src/
rg "Config::new" codex-rs/dx/src/

# Find gpt-5.3-codex-high specifically
rg "gpt-5\.3-codex-high" codex-rs/
```

### Possible Solutions
1. **Modify config loading** to use "mistral-large-latest" as fallback when config.model is None
2. **Modify get_default_model()** to return "mistral-large-latest" instead of "gpt-5.3-codex-high"
3. **Set config.model explicitly** during app initialization before ChatWidget is created
4. **User workaround** (temporary): Delete `~/.codex/config.toml` or use `/model mistral-large-latest` command in TUI

---

## Testing Instructions

### CRITICAL: Only use `cargo run` command
```bash
cd codex-rs/dx
cargo run
```

**DO NOT USE**:
- `cargo build`
- `cargo check`
- `cargo run --release`
- `just build`
- Any other build commands

**Reason**: Low-end device, need to minimize compilation time and binary size.

### After Making Changes
1. Run `just fmt` in `codex-rs` directory (auto-format code)
2. Run `cargo test -p codex-tui` (test the specific project)
3. Run `just argument-comment-lint` (check comment lint)
4. DO NOT run `just fix` unless specifically needed

---

## Additional Context

### Steering File Location
`.kiro/steering/dx.md` - This should be updated with progress, NOT a separate file

### TUI Style Conventions
See `codex-rs/tui/styles.md` for styling guidelines

### Ratatui Styling Helpers
- Prefer: `"text".red()`, `"text".dim()`, `"text".cyan()` over manual Style construction
- Use `"text".into()` for simple spans
- Use `vec![...].into()` for lines

---

## Summary

Two issues need to be solved:

1. **Red background disappearing** - The wrapper approach isn't working because something in the rendering pipeline is clearing the background. Need to either modify the history cell's style directly or find a different approach.

2. **Default model wrong** - Need to find where the actual default model is set when no config exists, or where `get_default_model()` gets its fallback value, and change it from "gpt-5.3-codex-high" to "mistral-large-latest".

Both issues are in the `codex-tui-dx` binary in the `codex-rs/dx/` folder.


---

## COMPLETE RESEARCH FINDINGS SUMMARY

### Issue 1: Red Background - Root Cause Analysis
After extensive investigation, the red background wrapper is correctly implemented but fails because:
1. The `RedBackgroundWrapper` applies red bg before AND after inner render
2. However, the rendering happens in a buffer that gets diffed by ratatui's terminal
3. The `FlexRenderable` may be creating a new buffer or the terminal diff is not detecting the change
4. **Most likely**: The `InsetRenderable` wrapper (from `.inset()` call) is creating a sub-buffer that loses the background

**Recommended Solution**: Instead of wrapping with `RedBackgroundWrapper`, modify the rendering at the `FlexRenderable` level or use a ratatui `Block` widget with red background that wraps the entire area.

### Issue 2: Default Model - Root Cause Analysis
After complete investigation:
1. `models.json` has `mistral-small-latest` at priority 0 (should be default)
2. `get_default_model()` correctly finds the default model from catalog
3. **BUT**: The user's `~/.codex/config.toml` file has `model = "gpt-5.3-codex"` saved from previous session
4. In `app.rs` line 2264-2267, if `config.model` exists, it OVERRIDES the default
5. This is why the model still shows as `gpt-5.3-codex` - it's loaded from config, not from code defaults

**Recommended Solution**: Add migration logic in `app.rs` around line 2264 to replace `gpt-5.3-codex*` with `mistral-large-latest` when loading from config.

---

## IMPLEMENTATION PLAN FOR BETTER AI

### Fix Issue 2 First (Easier)
In `codex-rs/dx/src/app.rs` around line 2264, add this logic:
```rust
if let Some(updated_model) = config.model.clone() {
    // Migrate old default to new default
    if updated_model.starts_with("gpt-5.3-codex") {
        model = "mistral-large-latest".to_string();
        config.model = Some(model.clone());
        // Optionally save config here to persist the change
    } else {
        model = updated_model;
    }
}
```

### Fix Issue 1 (More Complex)
Try one of these approaches:

**Approach A: Use Block widget**
```rust
// In as_renderable(), wrap active_cell with a Block
use ratatui::widgets::{Block, Borders};

let active_cell_renderable = match &self.active_cell {
    Some(cell) => {
        // Create a block with red background
        let block = Block::default()
            .style(Style::default().bg(Color::Red));
        // Wrap cell with block
        RenderableItem::Owned(Box::new(BlockWrapper { 
            inner: cell, 
            block 
        }))
    }
    // ...
};
```

**Approach B: Modify FlexRenderable**
Add a background color parameter to `FlexRenderable` and apply it before rendering children.

**Approach C: Apply at higher level**
In the parent widget that calls `as_renderable()`, apply red background to the entire area before rendering.

---

## FILES THAT NEED MODIFICATION

### For Issue 2 (Default Model):
- `codex-rs/dx/src/app.rs` (line ~2264) - Add migration logic

### For Issue 1 (Red Background):
- `codex-rs/dx/src/chatwidget.rs` (lines 8545-8670) - Modify rendering approach
- Possibly `codex-rs/dx/src/render/renderable.rs` - If modifying FlexRenderable

---

## TESTING CHECKLIST

After implementing fixes:
1. Delete `~/.codex/config.toml` to test fresh install
2. Run `cargo run` and verify model shows as `mistral-large-latest`
3. Verify red background persists in history area
4. Run `just fmt` to format code
5. Run `cargo test -p codex-tui` to ensure tests pass
6. Update `.kiro/steering/dx.md` with results
