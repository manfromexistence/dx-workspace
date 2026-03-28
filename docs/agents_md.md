# AGENTS.md

For information about AGENTS.md, see [this documentation](https://developers.openai.com/codex/guides/agents-md).

## Hierarchical agents message

When the `child_agents_md` feature flag is enabled (via `[features]` in `config.toml`), Codex appends additional guidance about AGENTS.md scope and precedence to the user instructions message and emits that message even when no AGENTS.md is present.

## Repository-specific resource constraints

Use `dx` as the standard local command workflow in this repository. Do not run full build or test commands in this environment due to RAM limits. Use incremental `cargo run` instead of `cargo build`, and run one job at a time (for example, `-j 1`) so tasks succeed on low-end devices.
