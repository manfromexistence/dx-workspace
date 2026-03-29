
[package]
name = "dx-tui"
version = "26.2.2"
edition = "2024"
rust-version = "1.85"
license = "MIT"
authors = ["DX Contributors"]
repository = "https://github.com/sxyazi/yazi"
homepage = "https://github.com/sxyazi/yazi"
description = "Terminal UI for Codex CLI - AI-powered coding agent"

[features]
default = ["vendored-lua", "llm"]
vendored-lua = ["mlua/vendored"]
llm = ["llama-cpp-2", "tiktoken-rs", "sysinfo", "reqwest", "futures"]

[dependencies]
# File browser dependencies
fb-actor = { path = "src/file_browser/actor" }
fb-adapter = { path = "src/file_browser/adapter" }
fb-binding = { path = "src/file_browser/binding" }
fb-boot = { path = "src/file_browser/boot" }
fb-config = { path = "src/file_browser/config" }
fb-core = { path = "src/file_browser/core" }
fb-dds = { path = "src/file_browser/dds" }
fb-emulator = { path = "src/file_browser/emulator" }
fb-fs = { path = "src/file_browser/fs" }
fb-macro = { path = "src/file_browser/macro" }
fb-parser = { path = "src/file_browser/parser" }
fb-plugin = { path = "src/file_browser/plugin" }
fb-proxy = { path = "src/file_browser/proxy" }
fb-shared = { path = "src/file_browser/shared" }
fb-term = { path = "src/file_browser/term" }
fb-tty = { path = "src/file_browser/tty" }
fb-vfs = { path = "src/file_browser/vfs" }
fb-watcher = { path = "src/file_browser/watcher" }
fb-widgets = { path = "src/file_browser/widgets" }

# LLM dependencies (optional)
llama-cpp-2 = { version = "0.1", optional = true }
sysinfo = { version = "0.38.4", optional = true }
tiktoken-rs = { version = "0.9", optional = true }
reqwest = { version = "0.12", features = ["stream"], optional = true }
futures = { workspace = true, optional = true }

# Font compression
zstd = "0.13"

# Animation
palette = "0.7"
rand.workspace = true

# Clipboard
cli-clipboard = "0.4"

# FIGlet rendering
figlet-rs = "1.0.0"
anyhow.workspace = true
better-panic.workspace = true
chrono.workspace = true
crossterm.workspace = true
fdlimit.workspace = true
mlua.workspace = true
once_cell.workspace = true
paste.workspace = true
ratatui = { workspace = true, features = ["serde"] }
scopeguard.workspace = true
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
tachyonfx.workspace = true
thiserror.workspace = true
tokio = { workspace = true, features = ["full"] }
tokio-stream.workspace = true
tracing.workspace = true
tracing-appender = "0.2.4"
tracing-subscriber = { version = "0.3.23", features = ["env-filter"] }
ansi-to-tui.workspace = true
arc-swap.workspace = true
async-channel.workspace = true
async-fs.workspace = true
async-trait.workspace = true
base64.workspace = true
bitflags.workspace = true
bstr.workspace = true
byteorder.workspace = true
bytes.workspace = true
dirs.workspace = true
dyn-clone.workspace = true
either.workspace = true
encoding_rs.workspace = true
fd-lock.workspace = true
flume.workspace = true
foldhash.workspace = true
globset.workspace = true
libc.workspace = true
log.workspace = true
lru.workspace = true
md5.workspace = true
memchr.workspace = true
notify.workspace = true
parking_lot.workspace = true
percent-encoding.workspace = true
rayon.workspace = true
regex.workspace = true
shell-words.workspace = true
signal-hook.workspace = true
smallvec.workspace = true
syntect.workspace = true
tempfile.workspace = true
twox-hash.workspace = true
typed-path.workspace = true
unicode-width.workspace = true
url.workspace = true
which.workspace = true
clap = { workspace = true, features = ["derive", "wrap_help"] }
color-eyre.workspace = true
core-foundation-sys.workspace = true
dashmap.workspace = true
hashbrown = { workspace = true, features = ["serde"] }
indexmap.workspace = true
ordered-float = { workspace = true, features = ["serde"] }
serde_with.workspace = true
tokio-util.workspace = true
toml = { workspace = true, features = ["parse"] }
uuid = { workspace = true, features = ["v4"] }
async-compression = { workspace = true, features = ["deflate", "gzip", "zstd", "futures-io"] }
russh.workspace = true

[target."cfg(unix)".dependencies]
libc.workspace = true
signal-hook-tokio = { version = "0.4.0", features = ["futures-v0_3"] }

[target.'cfg(target_os = "macos")'.dependencies]
crossterm = { workspace = true, features = ["use-dev-tty", "libc"] }

[target.'cfg(all(not(target_os = "macos"), not(target_os = "windows")))'.dependencies]
tikv-jemallocator = "0.6.1"

[build-dependencies]
zstd = "0.13"

[[bin]]
name = "dx"
path = "src/main.rs"
