set positional-arguments
set dotenv-load := true

# Display help
help:
    just -l

# Run codex-tui (legacy, direct TUI)
# Usage: just dx --help
# On Windows, this uses pre-built V8 binaries to avoid symlink privilege issues
# ENHANCED: 3 parallel jobs for Ryzen 5 5600 + 8GB RAM (3x faster builds)
dx *args:
    #!/usr/bin/env bash
    export V8_FROM_SOURCE=0
    # Load .env
    if [ -f .env ]; then
        export $(grep -v '^#' .env | xargs)
    fi
    C_FLAGS=()
    if [ ! -z "$DX_DEFAULT_MODEL_PROVIDER" ]; then C_FLAGS+=("-c" "model_provider=$DX_DEFAULT_MODEL_PROVIDER"); fi
    if [ ! -z "$DX_DEFAULT_MODEL" ]; then C_FLAGS+=("-c" "model=$DX_DEFAULT_MODEL"); fi
    cd codex-rs
    cargo run -p codex-tui --bin codex-tui -j3 -- "${C_FLAGS[@]}" "$@"

# Run codex-tui-app-server (modern, app-server backed TUI in OSS mode)
# Usage: just dx-app-server --help
# Usage: just dx-app-server --no-alt-screen
# Runs in OSS mode (no authentication required)
# ENHANCED: 3 parallel jobs for Ryzen 5 5600 + 8GB RAM (3x faster builds)
dx-app-server *args:
    #!/usr/bin/env bash
    export V8_FROM_SOURCE=0
    # Load .env
    if [ -f .env ]; then
        export $(grep -v '^#' .env | xargs)
    fi
    cd codex-rs
    cargo run --bin codex-tui-app-server -j3 -- --oss "$@"

# DEVELOPMENT COMMANDS (Fast iteration, use these for development!)
# ================================================================

# Run codex in dev mode with ENHANCED settings (3 parallel jobs)
# Optimized for Ryzen 5 5600 + 8GB RAM (3x faster than old -j1 config)
run *args:
    #!/usr/bin/env bash
    export V8_FROM_SOURCE=0
    if [ -f .env ]; then
        export $(grep -v '^#' .env | xargs)
    fi
    C_FLAGS=()
    if [ ! -z "$DX_DEFAULT_MODEL_PROVIDER" ]; then C_FLAGS+=("-c" "model_provider=$DX_DEFAULT_MODEL_PROVIDER"); fi
    if [ ! -z "$DX_DEFAULT_MODEL" ]; then C_FLAGS+=("-c" "model=$DX_DEFAULT_MODEL"); fi
    cd codex-rs
    cargo run --bin codex -j3 -- "${C_FLAGS[@]}" "$@"

# Run with 2 parallel jobs (ONLY if you have 8GB+ RAM)
run-moderate *args:
    #!/usr/bin/env bash
    export CARGO_BUILD_JOBS=2
    cd codex-rs && cargo run --bin codex -j2 -- "$@"

# Run custom DX TUI (your custom implementation with Yazi integration)
dx-tui *args:
    #!/usr/bin/env bash
    export V8_FROM_SOURCE=0
    cd codex-rs/dx
    cargo run --bin dx -j3 -- "$@"

# Run codex-tui app server (modern, app-server backed TUI)
# Useful flags:
#   --no-alt-screen: Keep output in terminal after exit
#   --model <MODEL>: Specify model (e.g., gpt-4o-mini)
#   --oss: Use OSS mode
codex-tui *args:
    #!/usr/bin/env bash
    export V8_FROM_SOURCE=0
    if [ -f .env ]; then
        export $(grep -v '^#' .env | xargs)
    fi
    C_FLAGS=()
    if [ ! -z "$DX_DEFAULT_MODEL_PROVIDER" ]; then C_FLAGS+=("-c" "model_provider=$DX_DEFAULT_MODEL_PROVIDER"); fi
    if [ ! -z "$DX_DEFAULT_MODEL" ]; then C_FLAGS+=("-c" "model=$DX_DEFAULT_MODEL"); fi
    cd codex-rs
    cargo run --bin codex -j3 -- "${C_FLAGS[@]}" "$@"

# Check code without building (VERY FAST - just type checking)
# Use this to quickly verify your changes compile
check:
    cd codex-rs && cargo check --bin codex -j3

# Verify local-llm integration without building (INSTANT)
check-local-llm:
    powershell -ExecutionPolicy Bypass -File check-local-llm.ps1

# Test local LLM model loading and inference (requires model file)
test-local-llm prompt="Hello, who are you?":
    #!powershell
    $env:V8_FROM_SOURCE = "0"
    cd codex-rs
    cargo run --bin codex -j3 -- test-local-llm "{{prompt}}"

# NEW: Test with cargo-nextest (3x faster than cargo test)
# Install: cargo install cargo-nextest --locked
test *args:
    cd codex-rs && cargo nextest run -j3 {{args}}

# NEW: Test specific crate with nextest
test-crate crate:
    cd codex-rs && cargo nextest run -p {{crate}} -j3

# NEW: Show sccache compilation cache statistics
# Install: cargo install sccache
cache-stats:
    sccache --show-stats

# NEW: Clear sccache cache (if needed)
cache-clear:
    sccache --stop-server

# HOT-RELOAD COMMANDS (Subsecond iteration for CLI changes!)
# ===========================================================

# Run with hot-reloading enabled (~130-500ms hot-patches for CLI changes)
# IMPORTANT: Only works for codex-cli/src/*.rs changes (tip crate)
# Does NOT work for codex-core, codex-tui, or struct layout changes
# Install: cargo install dioxus-cli
dx-hot *args:
    #!powershell
    $env:V8_FROM_SOURCE = "0"
    if (-not $env:DX_DEFAULT_MODEL_PROVIDER) { $env:DX_DEFAULT_MODEL_PROVIDER = "local-llm" }
    if (-not $env:DX_DEFAULT_MODEL) { $env:DX_DEFAULT_MODEL = "Qwen-3-0.6B-Q4_K_M" }
    cd codex-rs/cli
    dx serve --hotpatch --features hotpatch -- -c "model_provider=`"$env:DX_DEFAULT_MODEL_PROVIDER`"" -c "model=`"$env:DX_DEFAULT_MODEL`"" {{args}}

# Run with hot-reloading (OpenAI provider)
run-hot *args:
    #!powershell
    $env:V8_FROM_SOURCE = "0"
    if (-not $env:DX_DEFAULT_MODEL_PROVIDER) { $env:DX_DEFAULT_MODEL_PROVIDER = "openai" }
    if (-not $env:DX_DEFAULT_MODEL) { $env:DX_DEFAULT_MODEL = "gpt-4o-mini" }
    cd codex-rs/cli
    dx serve --hotpatch --features hotpatch -- -c "model_provider=`"$env:DX_DEFAULT_MODEL_PROVIDER`"" -c "model=`"$env:DX_DEFAULT_MODEL`"" {{args}}

# RELEASE BUILD COMMANDS (Slow, optimized binaries)
# ===================================================

# Build the CLI with ENHANCED parallelism (2 jobs for release builds)
# Builds dependencies with 2 parallel jobs (safe for 8GB RAM)
# Takes 10-20 minutes (was 30-60 min with -j1)
build:
    cd codex-rs && cargo clean && cargo build -p codex-core -j2 --lib --release && cargo build -p codex-config -j2 --lib --release && cargo build -p codex-exec -j2 --lib --release && cargo build -p codex-tui -j2 --lib --release && cargo build -p codex-cli -j2 --bin codex --release && echo "✓ Build complete! Binary at: target/release/codex.exe"

# Build with moderate parallelism (for 8GB+ RAM systems)
build-moderate:
    @echo "Building codex with moderate parallelism (2 jobs)..."
    cd codex-rs && cargo build --bin codex --release -j2

# Build with full parallelism (for 16GB+ RAM systems)
build-fast:
    @echo "Building codex with full parallelism..."
    cd codex-rs && cargo build --bin codex --release

# LEGACY ALIASES
# ==============

# `codex` - runs in dev mode with enhanced settings (same as `just run`)
alias c := codex
codex *args:
    #!powershell
    $env:V8_FROM_SOURCE = "0"
    if (-not $env:DX_DEFAULT_MODEL_PROVIDER) { $env:DX_DEFAULT_MODEL_PROVIDER = "openai" }
    if (-not $env:DX_DEFAULT_MODEL) { $env:DX_DEFAULT_MODEL = "gpt-4o-mini" }
    cd codex-rs
    cargo run --bin codex -j3 -- -c "model_provider=`"$env:DX_DEFAULT_MODEL_PROVIDER`"" -c "model=`"$env:DX_DEFAULT_MODEL`"" {{args}}

# `codex exec`
exec *args:
    cargo run --bin codex -- exec "$@"

# Run the CLI version of the file-search crate.
file-search *args:
    cargo run --bin codex-file-search -- "$@"

# Build the CLI and run the app-server test client
app-server-test-client *args:
    cargo build -p codex-cli
    cargo run -p codex-app-server-test-client -- --codex-bin ./target/debug/codex "$@"

# format code
fmt:
    cargo fmt -- --config imports_granularity=Item 2>/dev/null

fix *args:
    cargo clippy --fix --tests --allow-dirty "$@"

clippy *args:
    cargo clippy --tests "$@"

install:
    rustup show active-toolchain
    cargo fetch

# Build and run Codex from source using Bazel.
# Note we have to use the combination of `[no-cd]` and `--run_under="cd $PWD &&"`
# to ensure that Bazel runs the command in the current working directory.
[no-cd]
bazel-codex *args:
    bazel run //codex-rs/cli:codex --run_under="cd $PWD &&" -- "$@"

[no-cd]
bazel-lock-update:
    bazel mod deps --lockfile_mode=update

[no-cd]
bazel-lock-check:
    ./scripts/check-module-bazel-lock.sh

bazel-test:
    bazel test //... --keep_going

bazel-remote-test:
    bazel test //... --config=remote --platforms=//:rbe --keep_going

build-for-release:
    bazel build //codex-rs/cli:release_binaries --config=remote

# Run the MCP server
mcp-server-run *args:
    cargo run -p codex-mcp-server -- "$@"

# Regenerate the json schema for config.toml from the current config types.
write-config-schema:
    cargo run -p codex-core --bin codex-write-config-schema

# Regenerate vendored app-server protocol schema artifacts.
write-app-server-schema *args:
    cargo run -p codex-app-server-protocol --bin write_schema_fixtures -- "$@"

[no-cd]
write-hooks-schema:
    cargo run --manifest-path ./codex-rs/Cargo.toml -p codex-hooks --bin write_hooks_schema_fixtures

# Run the argument-comment Dylint checks across codex-rs.
[no-cd]
argument-comment-lint *args:
    ./tools/argument-comment-lint/run-prebuilt-linter.sh "$@"

[no-cd]
argument-comment-lint-from-source *args:
    ./tools/argument-comment-lint/run.sh "$@"

# Tail logs from the state SQLite database
log *args:
    if [ "${1:-}" = "--" ]; then shift; fi; cargo run -p codex-state --bin logs_client -- "$@"
