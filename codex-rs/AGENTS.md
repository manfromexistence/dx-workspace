# codex-rs low-end device workflow

These instructions apply to all files under `codex-rs/`.

- Use `dx` as the primary local command workflow.
- Do not run full build or full test commands by default.
  - Avoid commands like `cargo build`, `cargo test`, and `just test` unless the user explicitly asks.
- Prefer incremental execution with `cargo run` instead of `cargo build`.
- Run one job at a time for Rust commands (for example, use `-j 1`).
- Avoid running multiple heavy Rust jobs in parallel on this machine.
