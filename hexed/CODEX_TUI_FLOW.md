# Codex TUI Flow - Complete Architecture Guide

## Overview

When you run `codex` (the CLI), it determines which TUI to launch based on configuration. There are TWO different TUI implementations:

1. **codex-tui** (legacy, direct) - Located at `codex-rs/tui/`
2. **codex-tui-app-server** (modern, app-server backed) - Located at `codex-rs/tui_app_server/`
3. **dx-tui** (your custom) - Located at `codex-rs/dx/`

---

## The Complete Flow

### 1. Entry Point: `codex-rs/cli/src/main.rs`

```
User runs: codex
    ↓
main() → cli_main()
    ↓
Parses CLI args into MultitoolCli struct
    ↓
No subcommand? → run_interactive_tui()
```

### 2. TUI Selection Logic: `run_interactive_tui()`

**Location**: `codex-rs/cli/src/main.rs` (lines ~1500-1550)

```rust
async fn run_interactive_tui(
    mut interactive: TuiCli,
    remote: Option<String>,
    arg0_paths: Arg0DispatchPaths,
) -> std::io::Result<AppExitInfo> {
    // ... validation code ...
    
    // KEY DECISION POINT:
    let use_app_server_tui = codex_tui::should_use_app_server_tui(&interactive).await?;
    
    if use_app_server_tui {
        // MODERN PATH: Launch codex-tui-app-server
        codex_tui_app_server::run_main(
            into_app_server_tui_cli(interactive),
            arg0_paths,
            codex_core::config_loader::LoaderOverrides::default(),
            normalized_remote,
        ).await
    } else {
        // LEGACY PATH: Launch codex-tui
        codex_tui::run_main(
            interactive,
            arg0_paths,
            codex_core::config_loader::LoaderOverrides::default(),
        ).await
    }
}
```

**The Decision Function**: `codex_tui::should_use_app_server_tui()`
- Checks feature flags in config
- Checks if `tui_app_server` feature is enabled
- Returns `true` → use modern app-server TUI
- Returns `false` → use legacy direct TUI

---

## The Two TUI Paths

### Path A: Legacy TUI (`codex-tui`)

**Location**: `codex-rs/tui/`

**Entry Point**: `codex-rs/tui/src/main.rs` → calls `codex_lib.rs`

**Architecture**:
```
codex CLI
    ↓
codex_tui::run_main()
    ↓
Creates App struct (codex-rs/tui/src/app.rs)
    ↓
Runs event loop directly
    ↓
Renders with ratatui
    ↓
Calls codex-core directly for LLM operations
```

**Key Files**:
- `codex-rs/tui/src/codex_lib.rs` - Main library entry point
- `codex-rs/tui/src/main.rs` - Binary entry point
- `codex-rs/tui/src/app.rs` - Main App struct
- `codex-rs/tui/src/tui.rs` - Terminal management
- `codex-rs/tui/src/chatwidget.rs` - Chat UI rendering

**Characteristics**:
- Direct integration with codex-core
- Single-process architecture
- Simpler but less flexible
- No client-server separation

---

### Path B: Modern TUI (`codex-tui-app-server`)

**Location**: `codex-rs/tui_app_server/`

**Entry Point**: `codex-rs/tui_app_server/src/main.rs` → calls `lib.rs`

**Architecture**:
```
codex CLI
    ↓
codex_tui_app_server::run_main()
    ↓
Spawns app-server in background (codex-rs/app-server/)
    ↓
Creates TUI client that connects to app-server
    ↓
TUI sends JSON-RPC messages to app-server
    ↓
App-server handles LLM operations
    ↓
TUI renders responses with ratatui
```

**Key Files**:
- `codex-rs/tui_app_server/src/lib.rs` - Main library entry point
- `codex-rs/tui_app_server/src/main.rs` - Binary entry point
- `codex-rs/tui_app_server/src/app.rs` - Main App struct
- `codex-rs/tui_app_server/src/tui.rs` - Terminal management
- `codex-rs/app-server/` - Backend server

**Characteristics**:
- Client-server architecture
- App-server can be remote (via `--remote` flag)
- More complex but more flexible
- Better for IDE integrations (VSCode uses this)
- Requires authentication (or `--oss` flag)

---

## Your Custom DX TUI

**Location**: `codex-rs/dx/`

**Current Status**: Standalone binary, NOT integrated into codex CLI flow

**Entry Point**: `codex-rs/dx/src/main.rs`

**Architecture**:
```
User runs: cargo run --bin dx
    ↓
dx/src/main.rs
    ↓
Creates DxApp struct
    ↓
Runs custom event loop
    ↓
Renders with ratatui + tachyonfx
    ↓
Has Yazi file browser integration
    ↓
Has animation carousel
```

**Key Files**:
- `codex-rs/dx/src/main.rs` - Entry point
- `codex-rs/dx/src/state.rs` - App state
- `codex-rs/dx/src/render.rs` - Rendering logic
- `codex-rs/dx/src/dispatcher.rs` - Event handling
- `codex-rs/dx/src/bridge.rs` - Mode switching
- `codex-rs/dx/src/file_browser/` - Yazi integration

---

## How to Integrate DX TUI into Codex CLI

To make `codex` launch your custom DX TUI instead of the default ones, you need to:

### Option 1: Replace the Legacy TUI (RECOMMENDED)

This is the cleanest approach - replace the legacy `codex-tui` with your DX TUI.

**Step 1: Prepare DX TUI for Integration**

Your DX TUI needs to export a public API that matches `codex_tui::run_main()`:

**File**: `codex-rs/dx/src/lib.rs` (create this file)

```rust
// Re-export the main entry point
pub use crate::codex_lib::run_main;
pub use crate::codex_lib::{Cli, AppExitInfo, ExitReason};

// Include the codex_lib module
mod codex_lib;
```

**File**: `codex-rs/dx/src/codex_lib.rs` (already exists, ensure it exports)

```rust
// At the top of the file, ensure these are public:
pub use cli::Cli;
pub use app::AppExitInfo;
pub use app::ExitReason;

// Ensure run_main is public:
pub async fn run_main(
    cli: Cli,
    arg0_paths: Arg0DispatchPaths,
    loader_overrides: LoaderOverrides,
) -> std::io::Result<AppExitInfo> {
    // ... existing implementation
}
```

**Step 2: Update Cargo.toml**

Ensure your DX crate has a library target:

```toml
[lib]
name = "dx_tui"
path = "src/lib.rs"  # New entry point that re-exports codex_lib

[[bin]]
name = "dx"
path = "src/dx.rs"
```

**Step 3: Add DX TUI as Dependency in CLI**

**File**: `codex-rs/cli/Cargo.toml`

```toml
[dependencies]
# ... existing dependencies
codex-tui = { path = "../tui" }
dx-tui = { path = "../dx" }  # Add this
```

**Step 4: Modify CLI to Use DX TUI**

**File**: `codex-rs/cli/src/main.rs`

Find the `run_interactive_tui()` function (around line 1500) and modify it:

```rust
async fn run_interactive_tui(
    mut interactive: TuiCli,
    remote: Option<String>,
    arg0_paths: Arg0DispatchPaths,
) -> std::io::Result<AppExitInfo> {
    // ... existing validation code ...
    
    let use_app_server_tui = codex_tui::should_use_app_server_tui(&interactive).await?;
    
    // Add environment variable check for DX TUI
    let use_dx_tui = std::env::var("CODEX_USE_DX_TUI").is_ok() 
        || std::env::var("DX_TUI").is_ok();
    
    if use_dx_tui {
        // OPTION 1: Use DX TUI
        dx_tui::run_main(
            interactive,
            arg0_paths,
            codex_core::config_loader::LoaderOverrides::default(),
        ).await
    } else if use_app_server_tui {
        // OPTION 2: Use modern app-server TUI
        codex_tui_app_server::run_main(
            into_app_server_tui_cli(interactive),
            arg0_paths,
            codex_core::config_loader::LoaderOverrides::default(),
            normalized_remote,
        ).await
        .map(into_legacy_app_exit_info)
    } else {
        // OPTION 3: Use legacy TUI
        codex_tui::run_main(
            interactive,
            arg0_paths,
            codex_core::config_loader::LoaderOverrides::default(),
        ).await
    }
}
```

**Step 5: Usage**

```bash
# Use DX TUI
export CODEX_USE_DX_TUI=1
codex

# Or inline
CODEX_USE_DX_TUI=1 codex

# Use default TUI
codex
```

---

### Option 2: Feature Flag (CLEANEST FOR PRODUCTION)

**Step 1: Add Feature to Root Cargo.toml**

**File**: `codex-rs/Cargo.toml`

```toml
[features]
default = []
dx-tui = ["codex-cli/dx-tui"]

[dependencies]
# ... existing
```

**Step 2: Add Feature to CLI Cargo.toml**

**File**: `codex-rs/cli/Cargo.toml`

```toml
[features]
dx-tui = ["dep:dx-tui"]

[dependencies]
codex-tui = { path = "../tui" }
dx-tui = { path = "../dx", optional = true }
```

**Step 3: Conditional Compilation in CLI**

**File**: `codex-rs/cli/src/main.rs`

```rust
async fn run_interactive_tui(
    mut interactive: TuiCli,
    remote: Option<String>,
    arg0_paths: Arg0DispatchPaths,
) -> std::io::Result<AppExitInfo> {
    // ... validation ...
    
    #[cfg(feature = "dx-tui")]
    {
        // When dx-tui feature is enabled, always use DX TUI
        return dx_tui::run_main(
            interactive,
            arg0_paths,
            codex_core::config_loader::LoaderOverrides::default(),
        ).await;
    }
    
    #[cfg(not(feature = "dx-tui"))]
    {
        // Original logic
        let use_app_server_tui = codex_tui::should_use_app_server_tui(&interactive).await?;
        
        if use_app_server_tui {
            codex_tui_app_server::run_main(...)
        } else {
            codex_tui::run_main(...)
        }
    }
}
```

**Step 4: Build with Feature**

```bash
# Build with DX TUI
cargo build --features dx-tui

# Build without DX TUI (default)
cargo build

# Run with DX TUI
cargo run --features dx-tui
```

---

### Option 3: Config File Setting

**Step 1: Add Config Option**

**File**: `codex-rs/config/src/lib.rs`

Add to the config struct:

```rust
pub struct Config {
    // ... existing fields
    
    /// Which TUI implementation to use
    #[serde(default = "default_tui_implementation")]
    pub tui_implementation: TuiImplementation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TuiImplementation {
    Legacy,      // codex-tui
    AppServer,   // codex-tui-app-server
    Dx,          // dx-tui
}

fn default_tui_implementation() -> TuiImplementation {
    TuiImplementation::Legacy
}
```

**Step 2: Check Config in CLI**

**File**: `codex-rs/cli/src/main.rs`

```rust
async fn run_interactive_tui(
    mut interactive: TuiCli,
    remote: Option<String>,
    arg0_paths: Arg0DispatchPaths,
) -> std::io::Result<AppExitInfo> {
    // Load config to check TUI preference
    let config = Config::load().await.map_err(std::io::Error::other)?;
    
    match config.tui_implementation {
        TuiImplementation::Dx => {
            dx_tui::run_main(
                interactive,
                arg0_paths,
                codex_core::config_loader::LoaderOverrides::default(),
            ).await
        }
        TuiImplementation::AppServer => {
            codex_tui_app_server::run_main(...)
        }
        TuiImplementation::Legacy => {
            codex_tui::run_main(...)
        }
    }
}
```

**Step 3: User Configuration**

**File**: `~/.codex/config.toml`

```toml
# Use DX TUI
tui_implementation = "dx"

# Or use legacy
tui_implementation = "legacy"

# Or use app-server
tui_implementation = "appserver"
```

---

## Current Integration Blockers

### 1. API Compatibility

Your DX TUI needs to match the signature:

```rust
pub async fn run_main(
    cli: TuiCli,
    arg0_paths: Arg0DispatchPaths,
    loader_overrides: LoaderOverrides,
) -> std::io::Result<AppExitInfo>
```

**Current Issue**: Your DX TUI (`dx.rs`) has a different entry point:
- It's a binary with `#[tokio::main] async fn main()`
- It doesn't accept `TuiCli` arguments
- It doesn't return `AppExitInfo`

**Solution**: Create a wrapper in `codex_lib.rs` that:
1. Accepts the standard parameters
2. Converts them to your DX state
3. Runs your DX TUI
4. Returns proper exit info

### 2. Dependency Conflicts

**Current Issue**: 
- DX TUI has llama-cpp-2 (commented out due to V8 conflict)
- Codex core has V8 (for code-mode)
- Both can't coexist in the same binary

**Solutions**:
1. Keep llama-cpp disabled (current state)
2. Use a separate process for LLM (spawn external binary)
3. Use feature flags to exclude V8 when building with DX TUI

### 3. Module Structure

**Current Issue**:
- `dx.rs` is a binary entry point
- `codex_lib.rs` is the library entry point
- They have different module trees
- `dx_render.rs` is only in `dx.rs` module tree

**Solution**:
- Keep `dx.rs` for standalone binary
- Export library API from `codex_lib.rs`
- Share common modules between both

---

## Recommended Integration Path

**Phase 1: Make DX TUI Library-Compatible** ✅ (Partially done)
1. ✅ Create `lib.rs` that re-exports `codex_lib`
2. ✅ Ensure `codex_lib.rs` exports `run_main()`
3. ⚠️ Fix module conflicts (`dx_render` vs `render`)
4. ❌ Adapt `run_main()` to accept standard parameters

**Phase 2: Add to CLI** (Next step)
1. Add dx-tui dependency to cli/Cargo.toml
2. Add environment variable check in cli/src/main.rs
3. Test with `CODEX_USE_DX_TUI=1 codex`

**Phase 3: Polish** (Future)
1. Add feature flag support
2. Add config file option
3. Handle all CLI arguments properly
4. Ensure proper exit codes and token reporting

---

## Testing Strategy

### Test 1: Standalone Binary
```bash
cd codex-rs/dx
cargo run --bin dx
# Should work independently
```

### Test 2: Library API
```bash
cd codex-rs/dx
cargo test --lib
# Should compile and test library functions
```

### Test 3: Integration with CLI
```bash
cd codex-rs
CODEX_USE_DX_TUI=1 cargo run --bin codex
# Should launch DX TUI instead of default
```

### Test 4: Fallback
```bash
cd codex-rs
cargo run --bin codex
# Should launch default TUI (not DX)
```

---

## Summary of Required Changes

### Files to Create:
1. `codex-rs/dx/src/lib.rs` - Library entry point

### Files to Modify:
1. `codex-rs/dx/Cargo.toml` - Add [lib] section
2. `codex-rs/dx/src/codex_lib.rs` - Export public API
3. `codex-rs/cli/Cargo.toml` - Add dx-tui dependency
4. `codex-rs/cli/src/main.rs` - Add DX TUI routing logic

### Current Blockers:
1. ✅ Module conflict resolved (dx_render vs render)
2. ⚠️ Need to create lib.rs entry point
3. ❌ Need to adapt run_main() signature
4. ❌ Need to handle TuiCli arguments
5. ❌ Need to return AppExitInfo properly

**Next Step**: Create `lib.rs` and adapt the API to match codex-tui's interface.

---

## Key Integration Points

### 1. Function Signature

Your `dx_tui::run_main()` must match:

```rust
pub async fn run_main(
    cli: TuiCli,
    arg0_paths: Arg0DispatchPaths,
    loader_overrides: LoaderOverrides,
) -> std::io::Result<AppExitInfo>
```

### 2. Return Type

```rust
pub struct AppExitInfo {
    pub token_usage: TokenUsage,
    pub thread_id: Option<ThreadId>,
    pub thread_name: Option<String>,
    pub update_action: Option<UpdateAction>,
    pub exit_reason: ExitReason,
}
```

### 3. CLI Arguments

Your TUI needs to accept `TuiCli` struct:

```rust
pub struct TuiCli {
    pub prompt: Option<String>,
    pub images: Vec<PathBuf>,
    pub model: Option<String>,
    pub oss: bool,
    pub config_profile: Option<String>,
    pub sandbox_mode: Option<SandboxModeCliArg>,
    pub approval_policy: Option<ApprovalModeCliArg>,
    pub cwd: Option<PathBuf>,
    pub web_search: bool,
    pub no_alt_screen: bool,
    pub config_overrides: CliConfigOverrides,
    // ... more fields
}
```

---

## Current Justfile Commands

```bash
# Run legacy codex-tui (direct)
just dx

# Run modern codex-tui-app-server (app-server backed, OSS mode)
just dx-app-server

# Run your custom DX TUI (standalone, not integrated)
just dx-tui
```

---

## Summary

**Current State**:
- `codex` CLI → decides between `codex-tui` or `codex-tui-app-server`
- Your `dx-tui` is standalone, not integrated

**To Integrate**:
1. Export `run_main()` from `dx/src/lib.rs`
2. Match the function signature of `codex_tui::run_main()`
3. Modify `codex-rs/cli/src/main.rs` to call your TUI
4. Handle CLI arguments properly
5. Return proper exit info

**Files to Modify**:
- `codex-rs/cli/src/main.rs` - Add routing logic
- `codex-rs/dx/src/lib.rs` - Export public API
- `codex-rs/dx/Cargo.toml` - Configure as library
- `codex-rs/Cargo.toml` - Add dx-tui dependency to cli

**Next Steps**:
1. Study `codex-rs/tui/src/codex_lib.rs` to see how it exports `run_main()`
2. Create similar structure in `dx/src/lib.rs`
3. Test integration step by step
4. Keep both TUIs available during development
