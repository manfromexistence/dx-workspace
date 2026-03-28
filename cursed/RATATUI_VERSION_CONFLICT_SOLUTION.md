# Ratatui Version Conflict Resolution

## The Problem

- **codex-rs workspace**: Uses `ratatui = "0.29.0"` with custom patches from `github.com/nornagon/ratatui` (branch: `nornagon-v0.29.0-patch`)
- **dx workspace**: Uses `ratatui = "0.30.0"` (official latest version)

**You CANNOT have both versions in the same workspace!** Cargo will fail to resolve dependencies.

---

## Solution Options

### ✅ Option 1: Downgrade dx to ratatui 0.29.0 (RECOMMENDED)

**Pros**:
- Easiest to implement
- Guaranteed compatibility with codex-tui
- Can merge workspaces immediately

**Cons**:
- Lose ratatui 0.30.0 features
- Need to update dx code if it uses 0.30.0-specific APIs

**Steps**:

1. **Update dx/Cargo.toml**:
```toml
[dependencies]
ratatui = { workspace = true, features = ["serde"] }
```

2. **Check for breaking changes**:
```bash
cd codex-rs/dx
# Search for ratatui 0.30.0-specific APIs
rg "ratatui::" src/
```

3. **Test dx with 0.29.0**:
```bash
cd codex-rs
cargo check -p dx-tui
```

4. **Fix any API incompatibilities** (see migration guide below)

---

### 🔄 Option 2: Upgrade codex-rs to ratatui 0.30.0

**Pros**:
- Use latest ratatui features
- dx works without changes

**Cons**:
- **RISKY**: Lose custom patches from nornagon's fork
- Need to update ALL codex-tui code
- May break existing functionality
- Need to verify what the custom patches do

**Steps**:

1. **Investigate custom patches**:
```bash
# Check what's in the custom fork
git clone https://github.com/nornagon/ratatui
cd ratatui
git checkout nornagon-v0.29.0-patch
git log --oneline origin/v0.29.0..HEAD
```

2. **Update workspace Cargo.toml**:
```toml
[workspace.dependencies]
ratatui = "0.30.0"
ratatui-macros = "0.7.0"  # Updated version

# Remove or update patch
[patch.crates-io]
# ratatui = { git = "https://github.com/nornagon/ratatui", branch = "nornagon-v0.29.0-patch" }
```

3. **Fix breaking changes** in codex-tui (see migration guide)

4. **Test extensively**:
```bash
cargo test --workspace
cargo run --bin codex-tui
```

---

### 🚫 Option 3: Keep Separate Workspaces (NOT RECOMMENDED)

Keep dx and codex-rs as separate workspaces with different ratatui versions.

**Cons**:
- Cannot share code between dx and codex-tui
- Defeats the purpose of merging
- Two separate build systems

---

## Ratatui 0.29.0 → 0.30.0 Migration Guide

### Breaking Changes in 0.30.0

Check the [ratatui changelog](https://github.com/ratatui/ratatui/blob/main/CHANGELOG.md) for 0.30.0:

**Common breaking changes**:

1. **Widget trait changes**:
```rust
// Old (0.29.0)
impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) { }
}

// New (0.30.0) - may have signature changes
impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) { }
}
```

2. **Style API changes**:
```rust
// Check if Style::new() or Style::default() changed
```

3. **Layout changes**:
```rust
// Check if Layout API changed
```

4. **Feature flags**:
```toml
# Some features may have been renamed or removed
ratatui = { version = "0.30.0", features = [
    "serde",
    # Check if these still exist:
    "scrolling-regions",
    "unstable-backend-writer",
    "unstable-rendered-line-info",
    "unstable-widget-ref",
] }
```

---

## Recommended Approach: Downgrade dx to 0.29.0

### Step 1: Check dx's ratatui usage

```bash
cd codex-rs/dx
# Find all ratatui imports
rg "use ratatui" src/
rg "ratatui::" src/

# Check for 0.30.0-specific features
rg "unstable-" src/
```

### Step 2: Update dx/Cargo.toml

```toml
[dependencies]
# Change from:
# ratatui = { version = "0.30.0", features = ["serde"] }

# To:
ratatui = { workspace = true, features = ["serde"] }
```

### Step 3: Update all file_browser crates

```bash
# Update all file_browser Cargo.toml files
for dir in codex-rs/dx/src/file_browser/*/; do
    if [ -f "$dir/Cargo.toml" ]; then
        echo "Checking $dir/Cargo.toml"
        grep "ratatui" "$dir/Cargo.toml"
    fi
done
```

All should use:
```toml
ratatui = { workspace = true }
```

### Step 4: Test the downgrade

```bash
cd codex-rs
cargo clean
cargo check -p dx-tui
cargo build -p dx-tui -j3
cargo run --bin dx
```

### Step 5: Fix any compilation errors

If you get errors like:
```
error[E0599]: no method named `xyz` found for type `ratatui::...`
```

This means dx is using 0.30.0-specific APIs. You'll need to:
1. Check the ratatui 0.29.0 docs
2. Find the equivalent API
3. Update dx code

---

## Common API Differences (0.29.0 vs 0.30.0)

### 1. Widget Rendering

```rust
// Both versions should be similar, but check:
use ratatui::{
    widgets::{Block, Borders, Widget},
    layout::Rect,
    buffer::Buffer,
};

impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Implementation
    }
}
```

### 2. Style Builder

```rust
// 0.29.0
use ratatui::style::{Color, Modifier, Style};
let style = Style::default()
    .fg(Color::Red)
    .add_modifier(Modifier::BOLD);

// 0.30.0 - check if this changed
let style = Style::default()
    .fg(Color::Red)
    .add_modifier(Modifier::BOLD);
```

### 3. Layout

```rust
// Check if Layout::default() or Layout::new() changed
use ratatui::layout::{Constraint, Direction, Layout};

let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
    .split(area);
```

---

## Testing Strategy

After downgrading dx to 0.29.0:

### 1. Compilation Test
```bash
cargo check --workspace
```

### 2. Unit Tests
```bash
cargo test --workspace -j3
```

### 3. Visual Test
```bash
# Test dx TUI
cargo run --bin dx

# Test codex TUI
cargo run --bin codex-tui

# Test both work correctly
```

### 4. Integration Test
```bash
# If you plan to integrate dx with codex-tui
cargo run --bin dx -- --help
cargo run --bin codex-tui -- --help
```

---

## Investigating Custom Patches

Before deciding, check what the custom patches do:

```bash
# Clone the custom fork
git clone https://github.com/nornagon/ratatui /tmp/ratatui-custom
cd /tmp/ratatui-custom
git checkout nornagon-v0.29.0-patch

# Compare with official 0.29.0
git remote add upstream https://github.com/ratatui/ratatui
git fetch upstream
git log --oneline upstream/v0.29.0..HEAD

# See the actual changes
git diff upstream/v0.29.0..HEAD
```

**If the patches are critical** (e.g., fix bugs or add features codex needs), you have two options:
1. Stick with 0.29.0 (downgrade dx)
2. Port the patches to 0.30.0 (advanced, time-consuming)

---

## Decision Matrix

| Criterion | Downgrade dx to 0.29.0 | Upgrade codex to 0.30.0 |
|-----------|------------------------|-------------------------|
| **Effort** | Low (just update Cargo.toml) | High (update all code) |
| **Risk** | Low (known working version) | High (may break things) |
| **Custom patches** | Keep them ✅ | Lose them ❌ |
| **Latest features** | No ❌ | Yes ✅ |
| **Time to merge** | Fast (hours) | Slow (days/weeks) |
| **Recommended** | ✅ YES | ❌ NO |

---

## Final Recommendation

**Downgrade dx to ratatui 0.29.0** because:

1. ✅ Codex has custom patches that may be critical
2. ✅ Faster to implement (just update Cargo.toml)
3. ✅ Lower risk of breaking existing functionality
4. ✅ Can merge workspaces immediately
5. ✅ ratatui 0.29.0 is still very capable

You can always upgrade both to 0.30.0+ later, after the merge is complete and stable.

---

## Quick Start Script

```bash
#!/bin/bash
# Downgrade dx to ratatui 0.29.0

cd codex-rs/dx

# Backup
cp Cargo.toml Cargo.toml.backup

# Update main dx Cargo.toml
sed -i 's/ratatui = { version = "0.30.0"/ratatui = { workspace = true/' Cargo.toml

# Test
cd ..
cargo check -p dx-tui

echo "✅ If no errors, dx is now using ratatui 0.29.0!"
echo "Run: cargo run --bin dx"
```

---

## After Downgrade: Merge Checklist

Once dx uses ratatui 0.29.0:

- [ ] Update dx/Cargo.toml to use `workspace = true` for ratatui
- [ ] Update all file_browser crates to use `workspace = true`
- [ ] Run `cargo check --workspace`
- [ ] Run `cargo build --workspace -j3`
- [ ] Test dx binary: `cargo run --bin dx`
- [ ] Test codex-tui binary: `cargo run --bin codex-tui`
- [ ] Proceed with workspace merge (see WORKSPACE_MERGE_GUIDE.md)
