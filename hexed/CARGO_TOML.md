

Here is the merged `Cargo.toml`:

```toml
[workspace]
resolver = "2"
members = [
    # File browser crates
    "src/file_browser/actor",
    "src/file_browser/adapter",
    "src/file_browser/binding",
    "src/file_browser/boot",
    "src/file_browser/build",
    "src/file_browser/cli",
    "src/file_browser/codegen",
    "src/file_browser/config",
    "src/file_browser/core",
    "src/file_browser/dds",
    "src/file_browser/emulator",
    "src/file_browser/ffi",
    "src/file_browser/fs",
    "src/file_browser/macro",
    "src/file_browser/packing",
    "src/file_browser/parser",
    "src/file_browser/plugin",
    "src/file_browser/proxy",
    "src/file_browser/scheduler",
    "src/file_browser/sftp",
    "src/file_browser/shared",
    "src/file_browser/shim",
    "src/file_browser/term",
    "src/file_browser/tty",
    "src/file_browser/vfs",
    "src/file_browser/watcher",
    "src/file_browser/widgets",
    # Codex crates (adjust paths to match your project layout)
    "codex-rs/ansi-escape",
    "codex-rs/app-server-client",
    "codex-rs/app-server-protocol",
    "codex-rs/arg0",
    "codex-rs/backend-client",
    "codex-rs/chatgpt",
    "codex-rs/cli",
    "codex-rs/client",
    "codex-rs/cloud-requirements",
    "codex-rs/core",
    "codex-rs/features",
    "codex-rs/feedback",
    "codex-rs/file-search",
    "codex-rs/login",
    "codex-rs/otel",
    "codex-rs/protocol",
    "codex-rs/shell-command",
    "codex-rs/state",
    "codex-rs/terminal-detection",
    "codex-rs/tui-app-server",
    "codex-rs/windows-sandbox",
    "codex-rs/utils/absolute-path",
    "codex-rs/utils/approval-presets",
    "codex-rs/utils/cargo-bin",
    "codex-rs/utils/cli",
    "codex-rs/utils/elapsed",
    "codex-rs/utils/fuzzy-match",
    "codex-rs/utils/oss",
    "codex-rs/utils/pty",
    "codex-rs/utils/sandbox-summary",
    "codex-rs/utils/sleep-inhibitor",
    "codex-rs/utils/string",
]

[workspace.package]
version = "26.2.2"
edition = "2024"
rust-version = "1.85"
license = "MIT"
authors = ["Your Name <your.email@example.com>"]
repository = "https://github.com/yourusername/dx-tui"
homepage = "https://github.com/yourusername/dx-tui"

[workspace.dependencies]
# ── External crates (alphabetical) ──────────────────────────────────────
ansi-to-tui = "8.0.1"
anyhow = "1.0"
arboard = "3.4"
arc-swap = "1.7"
assert_matches = "1.5"
async-channel = "2.3"
async-compression = { version = "0.4", features = ["deflate", "gzip", "zstd", "futures-io"] }
async-fs = "2.1"
async-trait = "0.1"
base64 = "0.22"
better-panic = "0.3"
bitflags = "2.6"
bstr = "1.10"
byteorder = "1.5"
bytes = "1.8"
chrono = "0.4"
clap = { version = "4.5", features = ["derive", "wrap_help"] }
cli-clipboard = "0.4"
color-eyre = "0.6"
core-foundation-sys = "0.8"
cpal = "0.15"
crossterm = { version = "0.29.0", features = ["event-stream", "bracketed-paste"] }
dashmap = "6.1"
derive_more = { version = "2.0.1", features = ["display", "from", "is_variant"] }
diffy = "0.4"
dirs = "6.0.0"
dunce = "1.0.5"
dyn-clone = "1.0"
either = "1.13"
encoding_rs = "0.8"
fd-lock = "4.0"
fdlimit = "0.3.0"
figlet-rs = "1.0.0"
flume = "0.12"
foldhash = "0.2.0"
futures = "0.3"
globset = "0.4"
hashbrown = { version = "0.16.1", features = ["serde"] }
hound = "3.5"
image = { version = "0.25", default-features = false }
indexmap = "2.6"
insta = { version = "1.41", features = ["json", "redactions"] }
itertools = "0.14"
lazy_static = "1.5"
libc = "0.2"
llama-cpp-2 = "0.1"
log = "0.4"
lru = "0.16.3"
md5 = "0.8.0"
memchr = "2.7"
mlua = { version = "0.11.6", features = ["lua54", "vendored", "serialize", "macros", "async", "anyhow"] }
nix = { version = "0.31.2", default-features = false }
notify = "8.2.0"
objc2 = "0.6.4"
once_cell = "1.20"
ordered-float = { version = "5.1.0", features = ["serde"] }
palette = "0.7"
parking_lot = "0.12"
paste = "1.0"
pathdiff = "0.2.3"
percent-encoding = "2.3"
pretty_assertions = "1.4"
pulldown-cmark = "0.12"
rand = "0.8"
ratatui = { version = "0.30.0", features = ["serde"] }
ratatui-macros = "0.6.0"
rayon = "1.10"
regex = "1.11"
regex-lite = "0.1.6"
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls", "json"] }
rmcp = { version = "0.1", features = ["transport-sse", "transport-child-process"] }
russh = "0.58"
scopeguard = "1.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_with = "3.11"
serial_test = "3.2"
shell-words = "1.1"
shlex = "1.3"
signal-hook = "0.4.3"
signal-hook-tokio = { version = "0.4.0", features = ["futures-v0_3"] }
smallvec = "1.13"
strum = { version = "0.27", features = ["derive"] }
strum_macros = "0.27"
supports-color = "3.0"
syntect = "5.2"
sysinfo = "0.38.4"
tachyonfx = "0.25.0"
tempfile = "3.14"
textwrap = { version = "0.16", features = ["terminal_size"] }
thiserror = "2.0"
tiktoken-rs = "0.9"
tikv-jemallocator = "0.6.1"
tokio = { version = "1.42", features = ["full"] }
tokio-stream = "0.1"
tokio-util = "0.7"
toml = { version = "1.1.0", features = ["parse"] }
tracing = "0.1"
tracing-appender = "0.2"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
two-face = { version = "0.5", default-features = false, features = ["syntect-default-onig"] }
twox-hash = "2.0"
typed-path = "0.12.3"
unicode-segmentation = "1.12"
unicode-width = "0.2"
url = "2.5"
uzers = "0.12"
uuid = { version = "1.11", features = ["v4"] }
vergen-gitcl = { version = "9.1.0", default-features = false, features = ["build", "cargo"] }
vt100 = "0.15"
webbrowser = "1.0"
which = "8.0.2"
windows-sys = { version = "0.52", features = ["Win32_Foundation", "Win32_System_Console"] }
winsplit = "0.1"
zstd = "0.13"

# ── Internal codex crates ───────────────────────────────────────────────
codex-ansi-escape = { path = "codex-rs/ansi-escape" }
codex-app-server-client = { path = "codex-rs/app-server-client" }
codex-app-server-protocol = { path = "codex-rs/app-server-protocol" }
codex-arg0 = { path = "codex-rs/arg0" }
codex-backend-client = { path = "codex-rs/backend-client" }
codex-chatgpt = { path = "codex-rs/chatgpt" }
codex-cli = { path = "codex-rs/cli" }
codex-client = { path = "codex-rs/client" }
codex-cloud-requirements = { path = "codex-rs/cloud-requirements" }
codex-core = { path = "codex-rs/core" }
codex-features = { path = "codex-rs/features" }
codex-feedback = { path = "codex-rs/feedback" }
codex-file-search = { path = "codex-rs/file-search" }
codex-login = { path = "codex-rs/login" }
codex-otel = { path = "codex-rs/otel" }
codex-protocol = { path = "codex-rs/protocol" }
codex-shell-command = { path = "codex-rs/shell-command" }
codex-state = { path = "codex-rs/state" }
codex-terminal-detection = { path = "codex-rs/terminal-detection" }
codex-tui-app-server = { path = "codex-rs/tui-app-server" }
codex-windows-sandbox = { path = "codex-rs/windows-sandbox" }
codex-utils-absolute-path = { path = "codex-rs/utils/absolute-path" }
codex-utils-approval-presets = { path = "codex-rs/utils/approval-presets" }
codex-utils-cargo-bin = { path = "codex-rs/utils/cargo-bin" }
codex-utils-cli = { path = "codex-rs/utils/cli" }
codex-utils-elapsed = { path = "codex-rs/utils/elapsed" }
codex-utils-fuzzy-match = { path = "codex-rs/utils/fuzzy-match" }
codex-utils-oss = { path = "codex-rs/utils/oss" }
codex-utils-pty = { path = "codex-rs/utils/pty" }
codex-utils-sandbox-summary = { path = "codex-rs/utils/sandbox-summary" }
codex-utils-sleep-inhibitor = { path = "codex-rs/utils/sleep-inhibitor" }
codex-utils-string = { path = "codex-rs/utils/string" }

# ── Internal file browser crates ────────────────────────────────────────
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

# ── Workspace lints ─────────────────────────────────────────────────────
[workspace.lints.rust]
unsafe_code = "warn"
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(windows)', 'cfg(unix)'] }

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
correctness = { level = "deny", priority = -1 }
pedantic = { level = "warn", priority = -1 }

# ═════════════════════════════════════════════════════════════════════════
#  Root package
# ═════════════════════════════════════════════════════════════════════════
[package]
name = "dx-tui"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
authors.workspace = true
repository.workspace = true
homepage.workspace = true
description = "Terminal UI for Codex CLI - AI-powered coding agent"

[lib]
name = "dx_tui"
path = "src/lib.rs"

[[bin]]
name = "dx"
path = "src/main.rs"

[lints]
workspace = true

[features]
default = ["vendored-lua", "llm", "voice-input"]
vendored-lua = ["mlua/vendored"]
llm = ["dep:llama-cpp-2", "dep:tiktoken-rs", "dep:sysinfo"]
vt100-tests = []
debug-logs = []
voice-input = ["dep:cpal", "dep:hound"]

[dependencies]
# ── Internal: file browser ──────────────────────────────────────────────
fb-actor.workspace = true
fb-adapter.workspace = true
fb-binding.workspace = true
fb-boot.workspace = true
fb-config.workspace = true
fb-core.workspace = true
fb-dds.workspace = true
fb-emulator.workspace = true
fb-fs.workspace = true
fb-macro.workspace = true
fb-parser.workspace = true
fb-plugin.workspace = true
fb-proxy.workspace = true
fb-shared.workspace = true
fb-term.workspace = true
fb-tty.workspace = true
fb-vfs.workspace = true
fb-watcher.workspace = true
fb-widgets.workspace = true

# ── Internal: codex ─────────────────────────────────────────────────────
codex-ansi-escape.workspace = true
codex-app-server-client.workspace = true
codex-app-server-protocol.workspace = true
codex-arg0.workspace = true
codex-backend-client.workspace = true
codex-chatgpt.workspace = true
codex-client.workspace = true
codex-cloud-requirements.workspace = true
codex-core.workspace = true
codex-features.workspace = true
codex-feedback.workspace = true
codex-file-search.workspace = true
codex-login.workspace = true
codex-otel.workspace = true
codex-protocol.workspace = true
codex-shell-command.workspace = true
codex-state.workspace = true
codex-terminal-detection.workspace = true
codex-tui-app-server.workspace = true
codex-windows-sandbox.workspace = true
codex-utils-absolute-path.workspace = true
codex-utils-approval-presets.workspace = true
codex-utils-cli.workspace = true
codex-utils-elapsed.workspace = true
codex-utils-fuzzy-match.workspace = true
codex-utils-oss.workspace = true
codex-utils-sandbox-summary.workspace = true
codex-utils-sleep-inhibitor.workspace = true
codex-utils-string.workspace = true

# ── External (alphabetical) ────────────────────────────────────────────
ansi-to-tui.workspace = true
anyhow.workspace = true
arc-swap.workspace = true
async-channel.workspace = true
async-compression.workspace = true
async-fs.workspace = true
async-trait.workspace = true
base64.workspace = true
better-panic.workspace = true
bitflags.workspace = true
bstr.workspace = true
byteorder.workspace = true
bytes.workspace = true
chrono = { workspace = true, features = ["serde"] }
clap.workspace = true
cli-clipboard.workspace = true
color-eyre.workspace = true
core-foundation-sys.workspace = true
crossterm.workspace = true
dashmap.workspace = true
derive_more = { workspace = true, features = ["is_variant"] }
diffy.workspace = true
dirs.workspace = true
dunce.workspace = true
dyn-clone.workspace = true
either.workspace = true
encoding_rs.workspace = true
fd-lock.workspace = true
fdlimit.workspace = true
figlet-rs.workspace = true
flume.workspace = true
foldhash.workspace = true
futures.workspace = true
globset.workspace = true
hashbrown.workspace = true
image = { workspace = true, features = ["jpeg", "png", "gif", "webp"] }
indexmap.workspace = true
itertools.workspace = true
lazy_static.workspace = true
libc.workspace = true
log.workspace = true
lru.workspace = true
md5.workspace = true
memchr.workspace = true
mlua.workspace = true
notify.workspace = true
once_cell.workspace = true
ordered-float.workspace = true
palette.workspace = true
parking_lot.workspace = true
paste.workspace = true
pathdiff.workspace = true
percent-encoding.workspace = true
pulldown-cmark.workspace = true
rand.workspace = true
ratatui = { workspace = true, features = [
    "scrolling-regions",
    "unstable-backend-writer",
    "unstable-rendered-line-info",
    "unstable-widget-ref",
] }
ratatui-macros.workspace = true
rayon.workspace = true
regex.workspace = true
regex-lite.workspace = true
reqwest = { workspace = true, features = ["multipart"] }
rmcp.workspace = true
russh.workspace = true
scopeguard.workspace = true
serde.workspace = true
serde_json = { workspace = true, features = ["preserve_order"] }
serde_with.workspace = true
shell-words.workspace = true
shlex.workspace = true
signal-hook.workspace = true
smallvec.workspace = true
strum.workspace = true
strum_macros.workspace = true
supports-color.workspace = true
syntect.workspace = true
tachyonfx.workspace = true
tempfile.workspace = true
textwrap.workspace = true
thiserror.workspace = true
tokio.workspace = true
tokio-stream = { workspace = true, features = ["sync"] }
tokio-util = { workspace = true, features = ["time"] }
toml.workspace = true
tracing = { workspace = true, features = ["log"] }
tracing-appender.workspace = true
tracing-subscriber.workspace = true
two-face.workspace = true
twox-hash.workspace = true
typed-path.workspace = true
unicode-segmentation.workspace = true
unicode-width.workspace = true
url.workspace = true
uuid.workspace = true
webbrowser.workspace = true
which.workspace = true
zstd.workspace = true

# ── Optional (feature-gated) ───────────────────────────────────────────
llama-cpp-2 = { workspace = true, optional = true }
sysinfo = { workspace = true, optional = true }
tiktoken-rs = { workspace = true, optional = true }

# ── Target-specific ────────────────────────────────────────────────────
[target.'cfg(unix)'.dependencies]
libc.workspace = true
signal-hook-tokio.workspace = true

[target.'cfg(target_os = "macos")'.dependencies]
crossterm = { workspace = true, features = ["use-dev-tty", "libc"] }

[target.'cfg(all(not(target_os = "macos"), not(target_os = "windows")))'.dependencies]
tikv-jemallocator.workspace = true

[target.'cfg(not(target_os = "linux"))'.dependencies]
cpal = { workspace = true, optional = true }
hound = { workspace = true, optional = true }

[target.'cfg(not(target_os = "android"))'.dependencies]
arboard.workspace = true

[target.'cfg(windows)'.dependencies]
which.workspace = true
windows-sys.workspace = true
winsplit.workspace = true

# ── Dev dependencies ───────────────────────────────────────────────────
[dev-dependencies]
codex-cli.workspace = true
codex-core.workspace = true
codex-utils-cargo-bin.workspace = true
codex-utils-pty.workspace = true
assert_matches.workspace = true
chrono = { workspace = true, features = ["serde"] }
insta.workspace = true
pretty_assertions.workspace = true
rand.workspace = true
serial_test.workspace = true
vt100.workspace = true
uuid.workspace = true

# ── Build dependencies ─────────────────────────────────────────────────
[build-dependencies]
zstd.workspace = true

# ── Release profile ───────────────────────────────────────────────────
[profile.release]
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"
```

**Key decisions made during the merge:**

1. **Package name kept as `dx-tui`** with lib name `dx_tui` and bin name `dx`. Adjust if you prefer the `codex-tui` naming.

2. **All file-browser (`fb-*`) and codex (`codex-*`) path dependencies** were promoted into `[workspace.dependencies]` for consistency.

3. **Codex crate paths** assume a `codex-rs/` directory layout (e.g. `codex-rs/core`, `codex-rs/utils/fuzzy-match`). Adjust these paths to match your actual project structure.

4. **Features were merged**: `default` now includes `vendored-lua`, `llm`, and `voice-input`. The `vt100-tests` and `debug-logs` feature flags from codex-tui are preserved.

5. **Dependency features were unioned**: e.g. `crossterm` gets both `event-stream` and `bracketed-paste` in the workspace definition; `ratatui` gets the extra unstable features in the package dep; `serde_json` gains `preserve_order`; `tokio-stream` gains `sync`; `tracing` gains `log`.

6. **Target-specific deps** from both files are merged — `signal-hook-tokio` (unix), `crossterm` extra features (macOS), `tikv-jemallocator` (non-macOS/non-Windows), `cpal`/`hound` (non-Linux, optional), `arboard` (non-Android), and `windows-sys`/`winsplit`/`which` (Windows).

7. **Versions for newly-added crates** (e.g. `derive_more`, `diffy`, `image`, `itertools`, etc.) use versions consistent with the codex-cli project. Verify these match your lockfile.
