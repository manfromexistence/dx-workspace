# Critical Linker Error - Duplicate C++ Symbols

## Problem Summary
The `dx` binary fails to link on Windows with error LNK1169 due to duplicate C++ exception handling symbols between two dependencies:
- `libv8-f642da2890a06b62.rlib` (V8 JavaScript engine)
- `libllama_cpp_sys_2-f79188df870aa9bb.rlib` (Llama.cpp bindings)

## Exact Error
```
error LNK2005: Multiple definitions of C++ exception handling symbols:
- std::exception_ptr::exception_ptr(void)
- std::exception_ptr::exception_ptr(class std::exception_ptr const &)
- std::exception_ptr::operator=(class std::exception_ptr const &)
- std::exception_ptr::~exception_ptr(void)
- std::exception_ptr::operator bool(void)const
- std::current_exception(void)
- std::rethrow_exception(class std::exception_ptr)

LINK : warning LNK4098: defaultlib 'msvcrt' conflicts with use of other libs
LINK : warning LNK4098: defaultlib 'libcmt.lib' conflicts with use of other libs
fatal error LNK1169: one or more multiply defined symbols found
```

## Root Cause
Both v8 and llama-cpp-sys-2 are C++ libraries that statically link the C++ standard library, causing duplicate symbols for exception handling. This is a common issue when mixing multiple C++ libraries in Rust FFI.

## Context
- **Platform**: Windows (x86_64-pc-windows-msvc)
- **Compiler**: MSVC 14.50.35717
- **Project**: codex-rs/dx (Terminal UI application)
- **Language**: Rust with C++ FFI dependencies
- **Build System**: Cargo

## Potential Solutions (Need Expert Evaluation)

### Option 1: Force Dynamic Linking of C++ Runtime
Add linker flags to force dynamic linking of MSVC runtime:
```toml
# In dx/Cargo.toml or .cargo/config.toml
[target.x86_64-pc-windows-msvc]
rustflags = ["/NODEFAULTLIB:libcmt.lib", "/NODEFAULTLIB:msvcrt.lib", "/DEFAULTLIB:msvcrt.lib"]
```

### Option 2: Use /FORCE:MULTIPLE Linker Flag (Dangerous)
Force the linker to accept duplicate symbols (may cause runtime issues):
```toml
[target.x86_64-pc-windows-msvc]
rustflags = ["/FORCE:MULTIPLE"]
```
**WARNING**: This may cause undefined behavior at runtime.

### Option 3: Rebuild One Library with Different C++ Runtime
Rebuild either v8 or llama-cpp-sys-2 to use dynamic C++ runtime (/MD instead of /MT).

### Option 4: Remove One Dependency
If possible, remove either v8 or llama-cpp-sys-2 from the dependency tree.

### Option 5: Use Weak Symbols (Linux-specific, won't work on Windows)
Not applicable for Windows MSVC.

### Option 6: Create Wrapper DLL
Wrap one of the libraries in a separate DLL to isolate symbols.

## Dependencies Involved

### V8 (JavaScript Engine)
- Used by: `codex_code_mode` crate
- Purpose: JavaScript execution for code mode
- Path: `codex-rs/code-mode/`

### Llama.cpp (Local LLM)
- Used by: `codex_local_llm` crate via `llama-cpp-2`
- Purpose: Local language model inference
- Path: `codex-rs/local-llm/`

## Files to Investigate
1. `codex-rs/dx/Cargo.toml` - Main binary dependencies
2. `codex-rs/code-mode/Cargo.toml` - V8 dependency
3. `codex-rs/local-llm/Cargo.toml` - Llama.cpp dependency
4. `codex-rs/.cargo/config.toml` - Cargo build configuration
5. Build scripts for v8 and llama-cpp-sys-2

## Questions for Expert AI

1. **What is the safest way to resolve C++ symbol conflicts between v8 and llama-cpp-sys-2 on Windows?**

2. **Can we configure the build to use dynamic C++ runtime (/MD) for both libraries?**

3. **Is it possible to use `#[link(kind = "dylib")]` or similar Rust attributes to isolate these libraries?**

4. **Should we consider making one of these dependencies optional/feature-gated?**

5. **Are there Cargo features or build.rs configurations that can help?**

6. **What are the runtime implications of using /FORCE:MULTIPLE?**

## Current Workaround Needed
The Codex TUI integration is complete and compiles successfully. Only the linking stage fails. We need a solution that:
- Allows both v8 and llama-cpp-sys-2 to coexist
- Works on Windows MSVC
- Doesn't cause runtime instability
- Preferably doesn't require rebuilding upstream dependencies

## Additional Context
- The dx project successfully compiled before adding Codex TUI integration
- The issue is pre-existing in the dx codebase (both v8 and llama-cpp were already dependencies)
- This is a Windows-specific issue (Linux uses different linking mechanisms)
- The Codex TUI integration code itself is correct and compiles fine

## Success Criteria
The `cargo run -j3` command in `codex-rs/dx/` should complete successfully and launch the dx TUI application with the integrated Codex ChatWidget.
