fn main() {
    // Fix linker conflict between V8 and llama-cpp-2 on Windows
    // Both libraries define C++ standard library symbols
    // Tell the linker to allow multiple definitions and use the first one
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-arg=/FORCE:MULTIPLE");
    }
}
