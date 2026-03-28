# .idx/dev.nix
# Firebase Studio environment for building OpenAI Codex-RS CLI from source
# Updated: March 23, 2026

{ pkgs, ... }: {

  # ============================================
  # 📦 Nix Channel
  # Use "unstable" for latest Rust toolchain
  # or "stable-24.11" for stability
  # ============================================
  channel = "unstable";

  # ============================================
  # 🔧 System Packages
  # Everything needed to build codex-rs
  # ============================================
  packages = [
    # ---- Rust Toolchain ----
    pkgs.rustup           # Rust installer & version manager
    pkgs.rustc             # Rust compiler (fallback)
    pkgs.cargo             # Rust package manager (fallback)

    # ---- Build Essentials ----
    pkgs.gcc               # C/C++ compiler (needed for linking)
    pkgs.pkg-config        # Finds installed libraries
    pkgs.gnumake           # Make build tool
    pkgs.cmake             # CMake (some crates need it)
    pkgs.clang             # Clang compiler (LLVM)
    pkgs.llvmPackages.bintools  # LLVM linker tools (lld)

    # ---- Required Libraries for Codex-RS ----
    pkgs.openssl           # OpenSSL (TLS/crypto - required by many Rust crates)
    pkgs.openssl.dev       # OpenSSL development headers
    pkgs.zlib              # Compression library
    pkgs.zstd              # Zstandard compression (codex uses .jsonl.zst)
    pkgs.libcap            # Linux capabilities (needed for sandbox)
    pkgs.musl              # musl libc (codex-rs Linux builds use musl)
    pkgs.musl.dev          # musl development headers

    # ---- Git & Version Control ----
    pkgs.git               # Git (required to clone repo)
    pkgs.git-lfs           # Git Large File Storage

    # ---- Codex Build Tools ----
    pkgs.just              # Just command runner (codex uses justfile)
    pkgs.cargo-nextest     # Fast Rust test runner (optional but recommended)

    # ---- Useful CLI Tools ----
    pkgs.ripgrep           # Fast search tool (rg)
    pkgs.fd                # Fast find alternative
    pkgs.jq                # JSON processor
    pkgs.curl              # HTTP client
    pkgs.wget              # File downloader
    pkgs.htop              # System monitor (check RAM/CPU usage)
    pkgs.tree              # Directory tree viewer

    # ---- Zig (needed for cross-compilation in CI) ----
    pkgs.zig               # Zig compiler (used as cross-compilation linker)

    # ---- Node.js (optional - for npm install method) ----
    pkgs.nodejs_22         # Node.js 22 (if you also want npm-based install)
  ];

  # ============================================
  # 🧩 IDE Extensions
  # ============================================
  idx.extensions = [
    # Rust development
    "rust-lang.rust-analyzer"          # Official Rust language server
    "vadimcn.vscode-lldb"              # LLDB debugger for Rust
    "serayuzgur.crates"                # Crate version management
    "tamasfe.even-better-toml"         # TOML file support (Cargo.toml)

    # General development
    "usernamehw.errorlens"             # Inline error display
    "eamodio.gitlens"                  # Git integration
    "streetsidesoftware.code-spell-checker"  # Spell checker
    "fill-labs.dependi"                # Dependency management
  ];

  # ============================================
  # 🌍 Environment Variables
  # ============================================
  idx.env = {
    # OpenSSL configuration for Rust builds
    OPENSSL_DIR = "${pkgs.openssl.dev}";
    OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
    OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

    # Rust configuration
    RUST_BACKTRACE = "1";
    CARGO_NET_GIT_FETCH_WITH_CLI = "true";

    # Codex logging (optional - uncomment for debugging)
    # RUST_LOG = "codex_core=info,codex_tui=info";
  };

  # ============================================
  # 🚀 Workspace Lifecycle Hooks
  # ============================================

  # Run ONCE when workspace is first created
  idx.workspace.onCreate = {
    install-rust-toolchain = ''
      # Install the latest stable Rust toolchain via rustup
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
      source "$HOME/.cargo/env"

      # Add required Rust components
      rustup component add rustfmt
      rustup component add clippy
      rustup target add x86_64-unknown-linux-musl

      # Install cargo helper tools
      cargo install just
      cargo install --locked cargo-nextest
      cargo install cargo-insta

      echo "✅ Rust toolchain installed successfully!"
    '';

    clone-codex = ''
      # Clone the OpenAI Codex repository
      cd /home/user
      if [ ! -d "codex" ]; then
        git clone https://github.com/openai/codex.git
        echo "✅ Codex repo cloned!"
      else
        echo "ℹ️ Codex repo already exists, skipping clone."
      fi
    '';
  };

  # Run EVERY TIME the workspace starts
  idx.workspace.onStart = {
    ensure-rust-env = ''
      # Ensure Rust environment is sourced
      if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
      fi
      echo "🦀 Rust environment ready!"
      echo "📂 Codex source at: /home/user/codex/codex-rs"
      echo ""
      echo "Quick commands:"
      echo "  cd /home/user/codex/codex-rs"
      echo "  cargo build                    # Build Codex"
      echo "  cargo build --release          # Release build"
      echo "  cargo test -p codex-tui        # Test specific crate"
      echo "  just fmt                       # Format code"
      echo "  just fix -p codex-tui          # Fix lint issues"
      echo ""
    '';
  };

  # ============================================
  # 🖥️ Preview (disabled for CLI projects)
  # ============================================
  idx.previews = {
    enable = false;
  };
}