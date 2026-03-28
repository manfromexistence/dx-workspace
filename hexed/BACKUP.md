version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
authors.workspace = true
repository.workspace = true
homepage.workspace = true

[workspace]
resolver = "2"
members = [
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
ansi-to-tui = "7.0.0"
anyhow = "1.0"
arc-swap = "1.7"
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
color-eyre = "0.6"
core-foundation-sys = "0.8"
crossterm = { version = "0.28.1", features = ["event-stream"] }
dashmap = "6.1"
dirs = "6.0.0"
dyn-clone = "1.0"
either = "1.13"
encoding_rs = "0.8"
fd-lock = "4.0"
fdlimit = "0.3.0"
flume = "0.12"
foldhash = "0.2.0"
futures = "0.3"
globset = "0.4"
hashbrown = { version = "0.16.1", features = ["serde"] }
indexmap = "2.6"
libc = "0.2"
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
parking_lot = "0.12"
paste = "1.0"
percent-encoding = "2.3"
rand = "0.8"
ratatui = "0.29.0"
ratatui-macros = "0.6.0"
rayon = "1.10"
regex = "1.11"
russh = "0.58"
scopeguard = "1.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_with = "3.11"
shell-words = "1.1"
signal-hook = "0.4.3"
smallvec = "1.13"
syntect = "5.2"
tachyonfx = "0.25.0"
tempfile = "3.14"
thiserror = "2.0"
tokio = { version = "1.42", features = ["full"] }
tokio-stream = "0.1"
tokio-util = "0.7"
toml = { version = "1.1.0", features = ["parse"] }
tracing = "0.1"
tracing-subscriber = "0.3"
twox-hash = "2.0"
typed-path = "0.12.3"
unicode-width = "0.2"
url = "2.5"
uzers = "0.12"
uuid = { version = "1.11", features = ["v4"] }
vergen-gitcl = { version = "9.1.0", default-features = false, features = ["build", "cargo"] }
which = "8.0.2"

[workspace.lints.rust]
unsafe_code = "warn"  # Changed from "deny" to "warn" for file_browser code
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(windows)', 'cfg(unix)'] }

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
correctness = { level = "deny", priority = -1 }
pedantic = { level = "warn", priority = -1 }



























































[workspace.lints]
rust = {}

[workspace.lints.clippy]
expect_used = "deny"
identity_op = "deny"
manual_clamp = "deny"
manual_filter = "deny"
manual_find = "deny"
manual_flatten = "deny"
manual_map = "deny"
manual_memcpy = "deny"
manual_non_exhaustive = "deny"
manual_ok_or = "deny"
manual_range_contains = "deny"
manual_retain = "deny"
manual_strip = "deny"
manual_try_fold = "deny"
manual_unwrap_or = "deny"
needless_borrow = "deny"
needless_borrowed_reference = "deny"
needless_collect = "deny"
needless_late_init = "deny"
needless_option_as_deref = "deny"
needless_question_mark = "deny"
needless_update = "deny"
redundant_clone = "deny"
redundant_closure = "deny"
redundant_closure_for_method_calls = "deny"
redundant_static_lifetimes = "deny"
trivially_copy_pass_by_ref = "deny"
uninlined_format_args = "deny"
unnecessary_filter_map = "deny"
unnecessary_lazy_evaluations = "deny"
unnecessary_sort_by = "deny"
unnecessary_to_owned = "deny"
unwrap_used = "deny"
