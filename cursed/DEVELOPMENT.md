# Fast Development Guide for Codex CLI

## TL;DR - Quick Start for Development

**For daily development work, use DEV builds, NOT release builds!**

```bash
# Run codex in development mode (FAST!)
just run

# Or directly with cargo
cargo run --bin codex -j1

# Just check if your code compiles (VERY FAST!)
cargo check --bin codex -j1
```

## Why Dev Builds Are Much Faster

| Build Type | First Build | Rebuild After Change | Optimization | Use Case |
|------------|-------------|---------------------|--------------|----------|
| **Dev** (`cargo run`) | 2-5 min | **5-30 seconds** | None | Development |
| **Release** (`cargo build --release`) | 20-40 min | 15-30 min | Full | Production |

**Key insight:** Dev builds are 5-10x faster because they skip expensive optimizations like LTO (Link-Time Optimization) and use incremental compilation.

## Development Workflow

### 1. Initial Setup (One Time)

```bash
# Clone and navigate to the project
cd codex/codex-rs

# First build will take a few minutes
just run --help
```

### 2. Daily Development Loop

```bash
# Make your code changes in your editor...

# Option A: Run and test your changes (rebuilds automatically)
just run "your test prompt"

# Option B: Just check if it compiles (even faster!)
just check

# Option C: Run specific tests
cargo test -p codex-core -j1
```

### 3. Before Committing

```bash
# Format code
just fmt

# Run tests for the package you changed
cargo test -p codex-tui -j1

# Check for lint issues
just clippy
```

## Commands Reference

### Development Commands (Use These!)

```bash
# Run codex in dev mode (FASTEST)
just run [args]
# Example: just run --help
# Example: just run "explain this code"

# Run with more parallelism (if you have 8GB+ RAM)
just run-moderate [args]

# Check code without building (VERY FAST)
just check

# Run tests for a specific package
cargo test -p codex-core -j1
cargo test -p codex-tui -j1
cargo test -p codex-cli -j1

# Format code (always do this before committing)
just fmt

# Fix clippy warnings
just fix -p codex-tui
```

### Release Build Commands (Slow, Only for Production)

```bash
# Build optimized binary (20-40 minutes on low-end devices)
just build

# Build with more parallelism (8GB+ RAM)
just build-moderate

# Build with full parallelism (16GB+ RAM)
just build-fast
```

## Understanding Your System

### Windows with Git Bash (Your Setup)

You're running on **Windows with Git Bash/WSL**. This is a great setup for Rust development!

**Recommendations:**
- Use `just run -j1` for development (limits to 1 parallel job)
- If you have 8GB+ RAM, try `just run-moderate` (2 parallel jobs)
- Dev builds work great on Windows and are much faster than release builds

### Memory Usage

| Command | RAM Usage | Time (First) | Time (Rebuild) |
|---------|-----------|--------------|----------------|
| `just run` (dev, -j1) | ~2-3GB | 2-5 min | 5-30 sec |
| `just run-moderate` (dev, -j2) | ~4-6GB | 1-3 min | 5-20 sec |
| `just build` (release, -j1) | ~2-3GB | 20-40 min | 15-30 min |

## Incremental Compilation

Rust's incremental compilation is enabled by default for dev builds. This means:

1. **First build:** Takes a few minutes (compiles everything)
2. **Subsequent builds:** Only recompiles what changed (5-30 seconds!)

This is why you should use `cargo run` for development, not `cargo build --release`.

## Common Issues and Solutions

### Issue: "Build is taking forever"

**Solution:** You're probably using `cargo build --release`. Use `cargo run` instead!

```bash
# ❌ DON'T do this for development
cargo build --release

# ✅ DO this instead
cargo run --bin codex
# or
just run
```

### Issue: "Out of memory during build"

**Solution:** Limit parallel jobs to 1:

```bash
# Use -j1 flag
cargo run --bin codex -j1
# or
just run  # already uses -j1
```

### Issue: "I want to test the actual release binary"

**Solution:** Use GitHub Actions or build once overnight:

```bash
# Start the build before going to bed
just build

# Or use GitHub Actions to build in the cloud
# (see .github/workflows/ for CI builds)
```

### Issue: "cargo check vs cargo run - which is faster?"

**Answer:**
- `cargo check`: Only type-checks, doesn't produce a binary (~30 seconds)
- `cargo run`: Compiles and runs (~1-2 minutes first time, then 5-30 seconds)

Use `cargo check` when you just want to verify your code compiles.

## Pro Tips

### 1. Use `cargo-watch` for Auto-Rebuild

```bash
# Install cargo-watch
cargo install cargo-watch

# Auto-rebuild and run on file changes
cargo watch -x 'run --bin codex -j1'
```

### 2. Use `sccache` for Faster Builds

```bash
# Install sccache (caches compilation artifacts)
cargo install sccache

# Configure in ~/.cargo/config.toml
[build]
rustc-wrapper = "sccache"
```

### 3. Build Only What You Need

```bash
# If you're only working on the TUI
cargo build -p codex-tui -j1

# If you're only working on core logic
cargo build -p codex-core -j1
```

### 4. Use GitHub Actions for Release Builds

Instead of building release binaries locally (which takes 20-40 minutes), use GitHub Actions:

1. Push your changes to a branch
2. GitHub Actions will build release binaries for you
3. Download the artifacts from the Actions tab

See `.github/workflows/` for the CI configuration.

## Comparison: Dev vs Release

### Dev Build (cargo run)
```bash
cargo run --bin codex -j1 -- --help
```
- ✅ Fast compilation (2-5 min first time, 5-30 sec rebuilds)
- ✅ Incremental compilation enabled
- ✅ Fast iteration during development
- ❌ Slower runtime performance
- ❌ Larger binary size
- ❌ Includes debug symbols

### Release Build (cargo build --release)
```bash
cargo build --bin codex --release -j1
```
- ❌ Slow compilation (20-40 min first time, 15-30 min rebuilds)
- ❌ No incremental compilation
- ❌ Very slow iteration during development
- ✅ Fast runtime performance
- ✅ Smaller binary size
- ✅ Optimized for production

## Summary

**For development:** Use `just run` or `cargo run --bin codex -j1`
- Fast rebuilds (5-30 seconds after changes)
- Works on low-end devices
- Perfect for testing and iterating

**For production:** Use GitHub Actions or `just build` overnight
- Slow builds (20-40 minutes)
- Optimized binaries
- Only needed for releases

## Questions?

- Check the main [README.md](README.md) for general information
- Check [BUILD_LOW_END.md](BUILD_LOW_END.md) for release build optimization
- See the [justfile](../justfile) for all available commands
