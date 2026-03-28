# Building Codex CLI on Low-End Devices

This guide helps you build the Codex CLI on systems with limited resources (4GB RAM or less).

## Quick Start

For low-end devices (4GB RAM or less):
```bash
just build
```

This uses `-j1` flag to limit parallel compilation to a single job, significantly reducing memory usage at the cost of longer build times.

## Build Options by System Specs

### Low-End (4GB RAM or less)
```bash
just build
```
- Uses 1 parallel job (`-j1`)
- Estimated time: 20-40 minutes depending on CPU
- Peak memory usage: ~2-3GB

### Moderate (8GB RAM)
```bash
just build-moderate
```
- Uses 2 parallel jobs (`-j2`)
- Estimated time: 15-25 minutes
- Peak memory usage: ~4-6GB

### High-End (16GB+ RAM)
```bash
just build-fast
```
- Uses all available CPU cores
- Estimated time: 5-15 minutes
- Peak memory usage: 8GB+

## Additional Memory-Saving Tips

### 1. Increase Swap Space (Linux/WSL)

If you're still running out of memory, increase swap:

```bash
# Check current swap
free -h

# Create 4GB swap file
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Make it permanent
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab
```

### 2. Close Other Applications

Before building:
- Close web browsers (they use significant RAM)
- Close IDEs and editors
- Stop any development servers
- Close unnecessary background applications

### 3. Use Incremental Compilation

The project already has incremental compilation enabled in dev builds. For release builds, you can temporarily enable it:

```bash
# Set environment variable for this build only
CARGO_INCREMENTAL=1 cargo build --bin codex --release -j1
```

### 4. Monitor Memory Usage

Keep an eye on memory during the build:

**Linux/WSL:**
```bash
watch -n 1 free -h
```

**Windows (PowerShell):**
```powershell
while ($true) { Get-Process | Sort-Object WS -Descending | Select-Object -First 10; Start-Sleep 2; Clear-Host }
```

### 5. Clean Before Building

If you've attempted builds before:
```bash
cargo clean
just build
```

## Troubleshooting

### Build Killed by OOM (Out of Memory)

If the build process is killed:

1. **Increase swap space** (see above)
2. **Use `-j1`** if you haven't already
3. **Try building specific crates** one at a time:
   ```bash
   cargo build -p codex-core --release -j1
   cargo build -p codex-tui --release -j1
   cargo build -p codex-cli --release -j1
   ```

### Very Slow Build

This is expected on low-end devices. The build can take 30-60 minutes. Be patient!

### Disk Space Issues

The build requires significant disk space (~10GB for target directory). Ensure you have at least 15GB free.

## Build Configuration Details

The project's `Cargo.toml` already includes optimizations for release builds:

- **LTO (Link-Time Optimization)**: `lto = "fat"` - Optimizes across all crates
- **Codegen Units**: `codegen-units = 1` - Better optimization, less parallelism
- **Strip Symbols**: `strip = "symbols"` - Reduces binary size
- **Split Debug Info**: `split-debuginfo = "off"` - Reduces memory during linking

These settings prioritize binary size and optimization over compilation speed, which is appropriate for release builds.

## After Building

The compiled binary will be at:
```
codex-rs/target/release/codex.exe  (Windows)
codex-rs/target/release/codex      (Linux/macOS)
```

You can run it directly:
```bash
./target/release/codex --version
```

Or install it globally:
```bash
cargo install --path cli
```

## References

Based on best practices from:
- [Alibaba Cloud: Optimization on Memory Usage During Rust Cargo Code Compiling](https://www.alibabacloud.com/blog/optimization-on-memory-usage-during-rust-cargo-code-compiling_601189)
- [Rust Compiler Performance Improvements](https://markaicode.com/rust-compiler-performance-2025/)
- Rust community discussions on low-memory compilation strategies
