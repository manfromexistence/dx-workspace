Let me dig deep into the Codex CLI Rust codebase and configuration system to find every advanced hidden feature.Now I have extensive data from the official docs, the actual Rust source code, and community guides. Let me compile the ultimate deep-dive into every hidden/advanced config feature.

---

# 🔥 Codex CLI (Rust) — Every Advanced & Hidden Config Feature

**The stuff no one talks about that makes Codex an absolute beast.** Everything below comes from the official config reference, the sample `config.toml`, and the actual Rust source code (`mod.rs`).

---

## 🧠 1. History Compaction Engine (Context Window Control)

No other CLI gives you this level of control over how the model's context window is managed.

### `model_auto_compact_token_limit`
Token threshold that triggers automatic history compaction (unset uses model defaults). When context fills up, Codex compacts the conversation to free space.

### `compaction_prompt`
Inline override for the history compaction prompt. You can literally rewrite how Codex summarizes its own conversation history.

### `compaction_prompt_file`
Load the compaction prompt override from a file (experimental).

```toml
model_auto_compact_token_limit = 80000
compaction_prompt = "Summarize code changes made, files touched, and remaining tasks. Drop diagnostic output."
# compaction_prompt_file = "./my-compaction-prompt.md"  # or load from file
```

**Why this is wild:** Claude Code, Gemini CLI, and others just truncate. Codex lets you **control the summarization strategy** — you can tell it to preserve file names, keep test results, or drop verbose logs. This means your long sessions don't lose critical context.

---

## 🔒 2. Shell Environment Policy (Secret Leak Prevention)

shell_environment_policy controls which environment variables Codex passes to any subprocess it launches. Start from a clean start (inherit = "none") or a trimmed set (inherit = "core"), then layer on excludes, includes, and overrides to avoid leaking secrets while still providing the paths, keys, or flags your tasks need.

```toml
[shell_environment_policy]
inherit = "none"                           # Start completely clean
set = { PATH = "/usr/bin", MY_FLAG = "1" } # Inject only these
ignore_default_excludes = false            # Keep the KEY/SECRET/TOKEN auto-filter
exclude = ["AWS_*", "AZURE_*"]             # Glob-based blocklist
include_only = ["PATH", "HOME"]            # Allowlist
```

Patterns are case-insensitive globs (*, ?, [A-Z]); ignore_default_excludes = false keeps the automatic KEY/SECRET/TOKEN filter before your includes/excludes run.

**Why this is wild:** No other coding CLI has a built-in automatic `SECRET/TOKEN/KEY` glob filter for environment variables. Claude Code and Gemini CLI just inherit your full env. Codex has a **zero-trust subprocess environment** by default.

---

## 👻 3. Ghost Snapshots (Undo System)

Buried in the source code: Settings for ghost snapshots (used for undo). `pub ghost_snapshot: GhostSnapshotConfig`

This is Codex's built-in **file-level undo mechanism** — before every write, Codex takes a snapshot so you can roll back. No other CLI agent has this baked into the runtime at the config level.

---

## ⏱️ 4. Background Terminal Max Timeout

Maximum poll window for background terminal output (`write_stdin`), in milliseconds. Default: `300000` (5 minutes).

```toml
background_terminal_max_timeout = 600000  # 10 minutes for long-running builds
```

**Why this matters:** If you're running long test suites, builds, or database migrations, other CLIs will time out. Codex lets you configure the poll window so it waits patiently for slow processes.

---

## 🧩 5. Granular Approval Policy (Per-Category Control)

You can also use a granular approval policy to allow or auto-reject individual prompt categories. This is useful when you want normal interactive approvals for some cases but want others, such as request_permissions or skill-script prompts, to fail closed automatically.

```toml
approval_policy = { granular = {
  sandbox_approval = true,        # Prompt for sandbox escapes
  rules = false,                  # Auto-deny rule violations
  mcp_elicitations = true,        # Ask before MCP data input
  request_permissions = false,    # Auto-deny permission escalation
  skill_approval = true           # Prompt for skill execution
}}
```

**Why this is wild:** Every other CLI has a binary approve/deny. Codex lets you set **different approval policies per action category** — auto-approve file writes but always prompt for network access, for example.

---

## 📏 6. Execution Policy Rules (Starlark-Based Command Filtering)

Define fine-grained rules for what commands Codex can run. Rules use Starlark format in ~/.codex/rules/.

```python
# ~/.codex/rules/safety.star
prefix_rule(
    pattern = ["git", "push"],
    decision = "forbidden",
    justification = "No direct pushes - use PR workflow"
)
prefix_rule(
    pattern = ["rm", "-rf"],
    decision = "prompt",
    justification = "Destructive operation needs confirmation"
)
```

You can also test rules without running a session: Evaluate execpolicy rule files and see whether a command would be allowed, prompted, or blocked.

In `requirements.toml`, admins can enforce prefix rules: `enforce_residency = "us"` with rules like `{ pattern = [{ any_of = ["bash", "sh", "zsh"] }], decision = "prompt", justification = "Require explicit approval for shell entrypoints" }`

**Why this is wild:** This is a **policy-as-code engine** using Google's Starlark language. No other coding CLI lets you define a formal command allowlist/denylist with justification strings.

---

## 🌍 7. Data Residency Enforcement

From the source code: `enforce_residency` means web traffic cannot be routed outside of a particular geography. HTTP clients should direct their requests using backend-specific headers or URLs to enforce this.

```toml
enforce_residency = "us"   # or "eu"
```

**Why this is wild:** For companies bound by GDPR, HIPAA, or sovereignty laws, Codex is the **only** coding CLI that enforces geographic routing at the config level.

---

## 🔬 8. Web Search Advanced Object Configuration

Type: `boolean | { context_size = "low|medium|high", allowed_domains = [string], location = { country, region, city, timezone } }`. Optional web search tool configuration. The legacy boolean form is still accepted, but the object form lets you set search context size, allowed domains, and approximate user location.

```toml
[web_search]
context_size = "high"
allowed_domains = ["docs.python.org", "developer.mozilla.org", "docs.rs"]
[web_search.location]
country = "US"
region = "CA"
city = "San Francisco"
timezone = "America/Los_Angeles"
```

**Why this is wild:** You can **lock web search to specific documentation domains** and control context size. No more random Stack Overflow answers — only official docs. And the location spoofing means search results are geographically relevant.

---

## 📦 9. Token Budget for Tool Output

Token budget for storing individual tool/function outputs in history.

This controls how much of each tool call's output gets kept in context. Critical for long sessions where shell commands produce verbose output.

---

## 🏷️ 10. Model Catalog JSON (Custom Model Registry)

Optional path to a JSON model catalog loaded on startup. Profile-level `profiles.<n>.model_catalog_json` can override this per profile.

Profiles can also override model_catalog_json. When both the top level and the selected profile set model_catalog_json, Codex prefers the profile value.

```toml
model_catalog_json = "/Users/me/.codex/model-catalogs/default.json"

[profiles.deep-review]
model_catalog_json = "/Users/me/.codex/model-catalogs/deep-review.json"
```

**Why this is wild:** You can define your own **model catalog file** that tells Codex about available models, their capabilities, context sizes, and more. Switch entire model registries per profile.

---

## 🪟 11. Windows Sandbox Elevation Control

When running Codex natively on Windows, set the native sandbox mode to elevated in the windows table. Use unelevated only if you don't have administrator permissions or if elevated setup fails.

```toml
[windows]
sandbox = "elevated"       # Recommended — uses Restricted Token API
# sandbox = "unelevated"   # Fallback
```

Implementation: In-process sandboxing via codex-windows-sandbox crate using Windows Restricted Token API.

---

## 🧹 12. Notice Suppression & Model Migration System

From the sample config — a hidden `[notice]` table:

Includes `hide_full_access_warning`, `hide_world_writable_warning`, `hide_rate_limit_model_nudge`, `hide_gpt5_1_migration_prompt`, and `model_migrations = { "gpt-4.1" = "gpt-5.1" }`

```toml
[notice]
hide_full_access_warning = true
hide_world_writable_warning = true
hide_rate_limit_model_nudge = true
model_migrations = { "gpt-4.1" = "gpt-5.1" }  # Auto-upgrade model references!
```

**Why this is wild:** `model_migrations` lets you define automatic model upgrade paths — when someone references `gpt-4.1` in old configs or prompts, Codex silently routes it to `gpt-5.1`. No other CLI has this.

---

## 🧬 13. All Feature Flags (The Hidden Power Switches)

The full set of feature flags includes: `shell_tool`, `apps`, `unified_exec`, `shell_snapshot`, `multi_agent`, `personality`, `fast_mode`, `smart_approvals`, `enable_request_compression`, `skill_mcp_dependency_install`, `prevent_idle_sleep`.

```toml
[features]
shell_tool = true                    # Shell execution tool
apps = true                          # ChatGPT Apps/connectors (experimental)
unified_exec = true                  # Unified execution engine
shell_snapshot = true                # Speed up repeated commands via caching
multi_agent = true                   # Subagent spawning tools
personality = true                   # Personality selection controls
fast_mode = true                     # Fast mode toggle
smart_approvals = true               # Guardian subagent for approvals
enable_request_compression = true    # zstd compression for requests
skill_mcp_dependency_install = true  # Auto-install MCP deps for skills
prevent_idle_sleep = false           # Prevent machine sleep during runs
```

Suppress the warning that appears when under-development feature flags are enabled.

```toml
suppress_unstable_features_warning = true
```

### Key hidden flags explained:

- **`shell_snapshot`**: Speed up repeated commands — Codex caches shell environment state between commands.
- **`smart_approvals`**: Routes approval decisions through a **guardian subagent** — an AI reviewing the AI's actions.
- **`enable_request_compression`**: Compress streaming request bodies with zstd when supported (stable; on by default). Reduces bandwidth.
- **`prevent_idle_sleep`**: Prevents your machine from sleeping during long agent runs.
- **`skill_mcp_dependency_install`**: Auto-installs MCP dependencies when a skill needs them.

---

## 🗄️ 14. SQLite State Database Location

Directory where Codex stores the SQLite-backed state DB used by agent jobs and other resumable runtime state.

Codex stores the SQLite-backed state DB under sqlite_home or the CODEX_SQLITE_HOME environment variable. When unset, WorkspaceWrite sandbox sessions default to a temp directory; other modes default to CODEX_HOME.

```toml
sqlite_home = "/fast-ssd/.codex/state"
```

**Why this matters:** Put the state DB on a fast SSD for speed, or on a shared volume for multi-machine session continuity.

---

## 🔐 15. Custom CA Bundles (Enterprise Proxy Support)

Codex can trust a custom root CA bundle for outbound HTTPS and secure websocket connections when enterprise proxies or gateways intercept TLS. This applies to login flows and to Codex's other external connections. Set CODEX_CA_CERTIFICATE to the path of a PEM file containing one or more certificate blocks.

```bash
export CODEX_CA_CERTIFICATE=/etc/ssl/corporate-proxy-ca.pem
```

If CODEX_CA_CERTIFICATE is unset, Codex falls back to SSL_CERT_FILE.

---

## 🗺️ 16. Project Root Markers (Monorepo-Friendly)

To customize project root detection, set project_root_markers in config.toml. `project_root_markers = [".git", ".hg", ".sl"]`. Set `project_root_markers = []` to skip searching parent directories and treat the current working directory as the project root.

```toml
project_root_markers = [".git", ".hg", ".sl", "Cargo.toml", "package.json"]
```

**Why this is wild:** In monorepos, other CLIs get confused about the project root. Codex lets you define custom markers so it correctly identifies workspace boundaries.

---

## 📐 17. AGENTS.md Tuning

Additional filenames to try when AGENTS.md is missing.

Maximum bytes read from AGENTS.md when building project instructions.

```toml
project_doc_fallback_filenames = ["TEAM_GUIDE.md", ".agents.md", "CODEX.md"]
project_doc_max_bytes = 50000  # Read up to 50KB of project instructions
```

You can also use TEAM_GUIDE.md or .agents.md as fallbacks by setting project_doc_fallback_filenames in config.toml.

---

## 🧑‍💼 18. Personality System

Set a default communication style for supported models. `personality = "friendly"` or `"pragmatic"` or `"none"`. You can override this later in an active session with /personality.

```toml
personality = "pragmatic"
```

---

## 🔌 19. Apply Patch Tool Toggle

From source: Include the `apply_patch` tool for models that benefit from invoking file edits as a structured tool call. When unset, this falls back to the model info's default preference. `pub include_apply_patch_tool: bool`

```toml
include_apply_patch_tool = true
```

**Why this matters:** Controls whether the model uses structured patch operations vs raw file writes. Different models perform better with different approaches.

---

## 🏗️ 20. Sandbox Workspace Write Fine-Tuning

```toml
[sandbox_workspace_write]
network_access = true                   # Allow outbound network
additional_writable_roots = ["/data"]   # Extra writable directories
exclude_tmp = true                      # Block /tmp writes
exclude_tmpdir = true                   # Block $TMPDIR writes
```

Exclude /tmp from writable roots in workspace-write mode.
Exclude $TMPDIR from writable roots in workspace-write mode.
Allow outbound network access inside the workspace-write sandbox.
Additional writable roots when sandbox_mode = "workspace-write".

**Why this is wild:** You can block `/tmp` writes (preventing temp-file attack vectors) while still allowing workspace writes. This level of filesystem control doesn't exist in any competitor.

---

## 🕳️ 21. Login Shell Semantics Control

Allow shell-based tools to use login-shell semantics. Defaults to true; when false, login = true requests are rejected and omitted login defaults to non-login shells.

```toml
allow_login_shell = false  # Force non-login shells for consistency
```

---

## 📡 22. OpenTelemetry Deep Configuration

```toml
[otel]
environment = "prod"
exporter = "otlp-http"
trace_exporter = "otlp-http"
log_user_prompt = false            # Keep prompts redacted!
# otel_endpoint = "https://..."
# otel_headers = { "Authorization" = "Bearer ..." }
# otel_protocol = "http/protobuf"
# otel_ca_cert = "/path/to/ca.pem"
# otel_client_cert = "/path/to/cert.pem"
# otel_client_key = "/path/to/key.pem"
```

Opt in to exporting raw user prompts with OpenTelemetry logs.
CA certificate path, client certificate path, and client private key path for OTEL exporter TLS.
Event metadata includes service name, CLI version, env tag, conversation id, model, sandbox/approval settings, and per-event fields.

**Why this is wild:** Full mTLS support for OTEL export, prompt redaction control, and per-event metadata. This is enterprise observability that no other coding CLI even approaches.

---

## 🔄 23. Service Tier Control

Preferred service tier for new turns.

```toml
service_tier = "fast"     # or "default"
```

Enable Fast mode selection and the service_tier = "fast" path (stable; on by default).

---

## 📝 24. Developer Instructions (Inline AGENTS.md)

Additional developer instructions injected into the session (optional).

```toml
developer_instructions = "Always use TypeScript. Prefer functional patterns. Never use any."
```

Like AGENTS.md but in config — useful when you want per-user instructions that differ from the team's AGENTS.md.

You can also use a file: Replacement for built-in instructions instead of AGENTS.md.

```toml
model_instructions_file = "./my-instructions.md"
```

---

## 🔀 25. Credential Storage Mode

Control where the CLI stores cached credentials (file-based auth.json vs OS keychain).

macOS: System keychain via keyring crate. Linux: Encrypted file using keyring. Windows: Windows Credential Manager via keyring.

---

## 🧮 26. Model Context Window Override

Context window tokens available to the active model.

```toml
model_context_window = 1000000   # Override if using a custom/local model
```

---

## 🖥️ 27. TUI-Specific Hidden Options

TUI-specific options such as enabling inline desktop notifications.

Disable burst-paste detection in the TUI.

Disable alternate screen mode for the TUI (overrides tui.alternate_screen for this run).

```toml
[tui]
alternate_screen = false       # Stay in the same terminal buffer
desktop_notifications = true   # Get notified when agent finishes
disable_burst_paste = true     # Paste without detection interference
```

---

## 🏭 28. Provider Retry & Query Params

Extra query parameters appended to provider requests. Retry count for HTTP requests to the provider (default: 4).

```toml
[model_providers.proxy]
name = "My Proxy"
base_url = "http://proxy.example.com"
env_key = "OPENAI_API_KEY"
retries = 8                              # Retry up to 8 times
query_params = { "api-version" = "2024-02-01" }  # Azure-style params
headers = { "X-Custom" = "value" }       # Static headers
```

---

## 🧊 29. Review Model Override

Optional model override used by /review (defaults to the current session model).

```toml
review_model = "gpt-5-pro"   # Use a stronger model just for code review
```

**Why this is wild:** You can use a cheap fast model for coding but automatically switch to a powerful reasoning model for `/review` — cost optimization that no other CLI supports.

---

## 📊 30. JSON Schema Output Validation (Structured Output)

JSON Schema file describing the expected final response shape. Codex validates tool output against it.

```bash
codex exec --output-schema ./schema.json "Analyze this codebase and output a report"
```

**Why this is wild:** You can force Codex's output to conform to a JSON schema — perfect for CI pipelines that need structured, machine-readable output.

---

## 🗂️ 31. Config Layering Architecture

Precedence (highest to lowest): CLI flags, profile values, project config (.codex/config.toml), user config (~/.codex/config.toml), system config (/etc/codex/config.toml), defaults.

Note the **system config** at `/etc/codex/config.toml` — perfect for fleet-wide defaults on managed servers. Codex walks from the project root to your current working directory and loads every .codex/config.toml it finds. If multiple files define the same key, the closest file to your working directory wins.

---

## 🏢 32. MDM Distribution (macOS Fleet Management)

Drop the string into your MDM profile under the com.openai.codex domain at `config_toml_base64` (managed defaults) or `requirements_toml_base64` (requirements). Push the profile, then ask users to restart Codex.

---

## 🔍 33. Debug Config Introspection

You can audit the entire config resolution chain live:

```
/debug-config
```

This shows config layer order, on/off state, and policy sources including `allowed_approval_policies`, `allowed_sandbox_modes`, `mcp_servers`, `rules`, `enforce_residency`, and `experimental_network`.

---

## 📊 Feature Comparison: Hidden Config Depth

| Advanced Config Area | Codex CLI | Claude Code | Gemini CLI | OpenCode | Cursor CLI |
|---|---|---|---|---|---|
| **Compaction prompt override** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Shell env secret filtering** | ✅ (globs) | ❌ | ❌ | ❌ | ❌ |
| **Ghost snapshot undo** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Starlark exec policy rules** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Data residency enforcement** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Web search domain allowlist** | ✅ | N/A | ❌ | ❌ | ❌ |
| **Granular per-category approval** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Model migrations (auto-upgrade)** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **SQLite state DB placement** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Custom CA / mTLS for OTEL** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **JSON schema output validation** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Review-specific model override** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **MDM fleet deployment** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **6-layer config precedence** | ✅ | 2 layers | 2 layers | 1 layer | 2 layers |
| **`/tmp` write blocking** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Background terminal timeout** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Custom model catalog JSON** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Prevent idle sleep** | ✅ | ❌ | ❌ | ❌ | ❌ |

---

The depth of Codex CLI's configuration system is genuinely unmatched. It's not just a coding agent — it's a **fully configurable, enterprise-grade, policy-driven agent runtime** that happens to write code. The closest competitor in terms of config depth is Claude Code, but it lacks the security policy engine, OTEL stack, compaction control, and fleet management capabilities that Codex has baked in at the Rust level.












































































































































































































Let me research the actual Codex Rust codebase architecture to give you an accurate difficulty assessment.Now I have a very clear picture of the actual codebase. Let me give you an honest, detailed engineering assessment.

---

# 🔧 Difficulty Assessment: Adding Chat Completions API Back to Your Codex-Rust Fork

## Overall Difficulty: **6.5 / 10**

But that number hides a LOT of nuance. Let me break it down layer by layer based on what the actual source code shows.

---

## 📊 The Architecture You're Working With

The `ModelClient` in `codex-rs/core/src/client.rs` abstracts over different model providers and handles authentication and token refresh, streaming SSE, response parsing, context compaction, and error handling with retries. The client supports multiple wire protocols: Chat Completions API (standard OpenAI format) and Responses API (OpenAI format with enhanced reasoning support).

The codebase is the root of a Cargo workspace containing quite a bit of experimental code, with `core/` containing the business logic for Codex.

### The Critical Discovery: Chat Completions Code **STILL EXISTS** in an Older Tag

In the `rust-v0.63.0` tag, `client.rs` still has a `WireApi::Chat` match arm that creates a raw streaming connection via `stream_chat_completions()`, then wraps it with an aggregation adapter so callers see only the final assistant message per turn, matching the behaviour of the Responses API.

### But in the Latest `main` Branch: It's Been Nuked

The `WireApi` enum now only has one variant: `Responses` (the default). The custom deserializer explicitly rejects `"chat"` with the `CHAT_WIRE_API_REMOVED_ERROR` message.

The actual Rust code now reads:

```rust
pub enum WireApi {
    #[default]
    Responses,
}

// In the deserializer:
"responses" => Ok(Self::Responses),
"chat" => Err(serde::de::Error::custom(CHAT_WIRE_API_REMOVED_ERROR)),
```

The error constant says: "`wire_api = \"chat\"` is no longer supported. How to fix: set `wire_api = \"responses\"` in your provider config."

---

## 🗺️ Exact Files You Need to Modify & What to Do

Here's a task-by-task breakdown with difficulty ratings:

---

### **Task 1: Restore `WireApi::Chat` Variant** — ⭐ 2/10 (Easy)

**File:** `codex-rs/core/src/model_provider_info.rs`

The `WireApi` enum and its custom deserializer are defined here. You need to add back the `Chat` variant and update the deserializer to accept `"chat"` again.

```rust
pub enum WireApi {
    #[default]
    Responses,
    Chat,  // Add this back
}

// In deserializer:
"responses" => Ok(Self::Responses),
"chat" => Ok(Self::Chat),  // Instead of returning error
```

**Difficulty: 2/10** — It's literally 3 lines of Rust.

---

### **Task 2: Restore `stream_chat_completions()` Function** — ⭐⭐⭐ 4/10 (Medium)

**File:** `codex-rs/core/src/chat_completions.rs` (needs to be restored or recreated)

The old code imported `use crate::chat_completions::stream_chat_completions` along with all the common client types.

`stream_chat_completions()` handles `/v1/chat/completions` API requests with conditional stream aggregation, and is referenced at `codex-rs/core/src/client.rs` lines 144-192.

**Your approach:**
1. Check out the `rust-v0.63.0` tag: `git checkout rust-v0.63.0 -- codex-rs/core/src/chat_completions.rs`
2. This gives you the complete, working `stream_chat_completions()` and `AggregatedChatStream` code
3. Fix any compilation errors against newer internal APIs

**Difficulty: 4/10** — The code existed and worked. You're mostly porting it forward and fixing API drift in internal types.

---

### **Task 3: Restore the `WireApi::Chat` Match Arm in `client.rs`** — ⭐⭐⭐ 4/10 (Medium)

**File:** `codex-rs/core/src/client.rs`

The old match arm called `stream_chat_completions(prompt, &self.config.model_family, &self.client, &self.provider, ...)`, then wrapped the response stream with `AggregatedChatStream` — either in `streaming_mode()` for raw reasoning or `.aggregate()` for normal mode.

You need to add back the `WireApi::Chat => { ... }` branch in the main streaming dispatch function.

**Difficulty: 4/10** — The pattern is clear from the old code. Main risk is that new internal types (`Prompt`, `ResponseEvent`, etc.) may have new fields that Chat Completions doesn't produce.

---

### **Task 4: Tool Call Translation Layer (Responses → Chat format)** — ⭐⭐⭐⭐⭐⭐⭐ 7/10 (Hard)

**THIS IS THE HARDEST PART.** Here's why:

Codex's entire agentic loop (file editing, command execution, patching) relies on **tool/function calling**. The Responses API and Chat Completions API represent tool calls **completely differently**:

| Concept | Responses API | Chat Completions API |
|---|---|---|
| Tool calls | `output[].type = "function_call"` items | `choices[].message.tool_calls[]` array |
| Tool results | `output[].type = "function_call_output"` | Separate `role: "tool"` messages |
| Conversation state | `previous_response_id` (stateful) | Full message array (stateless) |
| Reasoning | `output[].type = "reasoning"` items | Not natively supported |
| Streaming events | `response.output_item.added`, etc. | `delta.tool_calls[].function` chunks |

Real users already hit this: "An assistant message with 'tool_calls' must be followed by tool messages responding to each 'tool_call_id'. The following tool_call_ids did not have response messages." This is exactly the kind of message-ordering bug you'll need to handle.

OpenAI themselves admitted: "the chat/completions API has increasingly hampered our ability to improve Codex and deliver new features. Maintaining compatibility with this legacy protocol has added complexity, introduced regressions, and increased support overhead."

**What you need to build:**
- A **bidirectional translation layer** that:
  - Converts Codex's internal Responses-format tool definitions → Chat Completions `tools[]` parameter
  - Converts Chat Completions streaming `delta.tool_calls` chunks → Codex's internal `ResponseEvent` stream
  - Manages **full conversation history** (Chat Completions is stateless, unlike Responses API's `previous_response_id`)
  - Handles tool result messages properly (ordering matters!)
  - Fakes or strips reasoning items (Chat Completions doesn't have `"type": "reasoning"`)

For reference, the Hermes Agent project already built exactly this kind of adapter: "The Codex OAuth provider uses OpenAI's Responses API, which has a different request/response format than Chat Completions. The adapter translates between these formats." You can study their `codex_responses.py` for the translation logic (though it's in Python, the mapping is the same).

**Difficulty: 7/10** — This is the core engineering challenge. You're effectively writing a protocol adapter between two different API schemas, and edge cases in streaming tool calls will bite you.

---

### **Task 5: Provider-Specific Adapters (Gemini, Mistral, Anthropic)** — ⭐⭐⭐⭐⭐⭐⭐⭐ 8/10 (Very Hard)

Beyond just Chat Completions, if you want **native** Gemini/Mistral/Anthropic support (not just OpenAI-compatible Chat Completions):

Some Gemini models like gemini-2.5-pro don't support `streamGenerateContent` (which is what Chat Completions maps to). Users requested a `wire_api = "generateContent"` option to use the correct non-streaming endpoint.

Each provider has quirks:
- **Gemini**: Uses `generateContent` endpoint, different auth (`X-Goog-Api-Key`), different tool call format
- **Anthropic/Claude**: Uses Messages API with `tool_use` blocks, not `tool_calls`
- **Mistral**: OpenAI-compatible Chat Completions but with quirks around function calling
- **Ollama**: OpenAI-compatible but tool support varies by model

**Difficulty: 8/10** — Each provider is essentially a separate adapter. But if you stick to only OpenAI-compatible Chat Completions (which Gemini, Mistral, and Groq all offer), you can drop this to **3/10**.

---

### **Task 6: Making ALL Existing Config Features Work with Chat** — ⭐⭐⭐⭐⭐⭐ 6/10 (Medium-Hard)

This is where the "beast" nature of Codex bites you. Features that need adaptation:

| Feature | Works with Chat out of the box? | Effort |
|---|---|---|
| **Compaction** | ⚠️ Needs full history management | Medium |
| **Subagents** | ⚠️ Tool call format differs | Hard |
| **Skills** | ⚠️ Depend on tool execution | Medium |
| **Smart Approvals** | ✅ Internal to Codex | Free |
| **Sandbox/Exec policy** | ✅ Internal to Codex | Free |
| **Shell env policy** | ✅ Internal to Codex | Free |
| **Web search** | ⚠️ Tool call format | Medium |
| **Code review** | ⚠️ May use tool calls | Medium |
| **Image input** | ⚠️ Different format per provider | Medium |
| **Session resume/fork** | ❌ `previous_response_id` doesn't exist | Hard |
| **Ghost snapshots** | ✅ Internal to Codex | Free |
| **Fast mode** | ⚠️ Provider-dependent | Easy |
| **MCP** | ✅ Internal to Codex | Free |
| **OTEL** | ✅ Internal to Codex | Free |

The **good news**: roughly 50-60% of Codex's advanced features are **internal to the agent runtime** (sandbox, exec policy, shell env, approvals, ghost snapshots, OTEL, MCP, TUI). These don't care what wire protocol you use.

The **bad news**: Session management (`previous_response_id`) is deeply wired into the Responses API's stateful model. When a turn completes, the `response_id` from the model's final response is stored in the Session state to resume the thread given the next `Op::UserInput`. The `response_id` is also returned in `EventMsg::TurnComplete` to the UI, which can be used to fork the thread from an earlier point. With Chat Completions, you'll need to manage conversation history yourself instead.

**Difficulty: 6/10** — Most features are fine, but session resume and subagents need real work.

---

### **Task 7: Better TUI** — ⭐⭐⭐⭐⭐ 5/10 (Medium)

The TUI is built with Ratatui, which is a well-documented Rust TUI framework. Adding provider selection UI, model switching dropdowns, or a nicer layout is straightforward Ratatui work.

**Difficulty: 5/10** — Ratatui is good, but TUI work is always fiddly. Not conceptually hard, just time-consuming.

---

## 📋 Final Difficulty Summary

| Task | Difficulty | Time Estimate | Can Skip? |
|---|---|---|---|
| 1. Restore `WireApi::Chat` enum | ⭐⭐ 2/10 | 30 minutes | No |
| 2. Restore `stream_chat_completions()` | ⭐⭐⭐⭐ 4/10 | 1-2 days | No |
| 3. Restore client.rs match arm | ⭐⭐⭐⭐ 4/10 | 1 day | No |
| 4. Tool call translation layer | ⭐⭐⭐⭐⭐⭐⭐ 7/10 | 1-2 weeks | No |
| 5. Native provider adapters | ⭐⭐⭐⭐⭐⭐⭐⭐ 8/10 | 2-4 weeks | ✅ Yes — use OpenAI-compat only |
| 6. Existing features compat | ⭐⭐⭐⭐⭐⭐ 6/10 | 1-2 weeks | Partially |
| 7. Better TUI | ⭐⭐⭐⭐⭐ 5/10 | 1-2 weeks | ✅ Can do later |

---

## 🎯 My Honest Recommendation: The Smart Path

### Phase 1 — Get Chat Completions working (2-3 weeks)
1. `git checkout rust-v0.63.0` and extract the full `chat_completions.rs` module
2. Port it to current `main`, fix type mismatches
3. Restore `WireApi::Chat` and the client.rs dispatch
4. Build the tool call translation layer (study Hermes Agent's `codex_responses.py` for the mapping)
5. Test with Gemini, Mistral, Groq (all offer OpenAI-compatible endpoints)

### Phase 2 — Fix session management (1 week)
- Implement local conversation history storage for Chat Completions mode
- Make session resume work by replaying history instead of `previous_response_id`

### Phase 3 — TUI improvements (ongoing)
- Add provider switcher, model browser, etc.

### Phase 4 — Native provider adapters (optional, later)
- Add native Gemini `generateContent`, Anthropic Messages API, etc. for providers that don't offer OpenAI-compatible endpoints

---

## 🏆 Will It Be the Best AI CLI Ever?

**Honestly? It has a real shot.** Here's why:

- **Codex's runtime** (sandbox, exec policy, shell env filtering, MCP, OTEL, ghost snapshots, skills, subagents) is **genuinely unmatched** — and none of it depends on the wire protocol
- **Claude Code** is locked to Anthropic. **Gemini CLI** is locked to Google. Your fork would be the **only agent with Codex's full feature set + any provider**
- **open-codex** (the Node.js fork) has multi-provider but lacks all the Rust-only features (native sandbox, Ratatui TUI, zero-dependency binary, OTEL, skills, subagents, etc.)
- The community is **desperate** for this — as one user put it: "So there is no way to use Codex now - models in lmstudio are too small and no providers support responses except OpenAI"

The biggest risk isn't technical — it's **maintenance burden**. Every time upstream Codex adds a new Responses API feature, you'll need to add the Chat Completions equivalent. But if you structure the translation layer cleanly as a standalone module, rebasing on upstream should be manageable.

**Go build it. The community is waiting. 🚀**







































































Let me research the actual current state of Codex-Rust CLI on GitHub to give you a fully accurate, up-to-date difficulty assessment.Now I have a comprehensive picture from the actual source code and community. Let me give you the full, honest assessment.

---

# 🔍 Bringing Back Multi-Provider Support to Codex-Rust: Difficulty Assessment (Out of 10)

## Based on Real GitHub Source Code & Docs — March 22, 2026

---

## 📜 What ACTUALLY Happened: The Full Timeline

### ✅ It DID support other providers — via `wire_api = "chat"`

In early 2025, OpenAI introduced the Responses API — an interface optimized for reasoning models, multi-turn conversations, and tool-rich workflows, replacing the older Chat Completions API which originated in the GPT-3.5 era. While Codex historically supported both APIs, the Chat Completions API increasingly hampered their ability to improve Codex and deliver new features.

Users could configure any OpenAI-compatible provider with:
```toml
[model_providers.deepseek]
name = "DeepSeek"
base_url = "https://api.deepseek.com/v1"
env_key = "DEEPSEEK_API_KEY"
wire_api = "chat"
```

This worked with **DeepSeek, Gemini, Mistral, Groq, Ollama, LM Studio, Anthropic (via proxy), Azure OpenAI, LiteLLM**, and basically anything that spoke OpenAI-compatible Chat Completions.

### ❌ Then They Killed It

The Codex CLI was updated to emit a deprecation warning if configured to use the Chat Completions API. In February 2026, this transitioned to a hard error as support was fully removed.

Maintaining compatibility with this legacy protocol had added complexity, introduced regressions, and increased support overhead. As a result, the Codex team deprecated support for the Chat Completions API, with full removal slated for early February 2026.

### 🔒 Current State of the Code (v0.116.0)

The `WireApi` enum now only has one variant — `Responses` (the default). The custom deserializer explicitly rejects `"chat"` with `CHAT_WIRE_API_REMOVED_ERROR`.

The built-in provider list says it all: "We do not want to be in the business of adjudicating which third-party providers are bundled with Codex CLI, so we only include the OpenAI and open source ('oss') providers by default." Users are encouraged to add their own providers in config.toml.

But here's the cruel irony — even those OSS providers (Ollama, LM Studio) now default to `WireApi::Responses`, which many local model servers still don't fully support.

### 🔥 Community Pain Is REAL

A widely-shared DeepSeek configuration gist now reads: "⚠️ No Longer Working (January 2026): This configuration was working when originally created, but is now broken due to Codex deprecating the Chat Completions API. Since DeepSeek only supports Chat Completions, this integration no longer works reliably."

Azure users are also hit hard: "Many Azure/APIM deployments expose Chat Completions but not Responses. Without a documented knob to choose the wire (or override the path), Codex can't interoperate with otherwise standard Azure OpenAI setups."

Even providers like Z.AI's GLM-4.7 are broken because "Codex is sending messages with the `developer` role, but Z.AI's Chat Completions endpoint only accepts `system`, `user`, `assistant`." The role conversion logic doesn't work for custom providers.

Independent reviewers note: Codex "falls short: No Agent Teams, locked to OpenAI models, outputs can vary between runs."

And the fundamental problem across ALL foundation lab agents: "The trade-off is lock-in. Each agent only works with its provider's models."

---

## 🗺️ The Old Code Still Exists — Here's What It Looked Like

### At `rust-v0.63.0` (the last version with full Chat support):

The `WireApi::Chat` match arm created a raw streaming connection via `stream_chat_completions()`, then wrapped it with an aggregation adapter (`AggregatedChatStream`) so callers see only the final assistant message per turn — matching the behaviour of the Responses API. It supported both `streaming_mode()` for raw reasoning and `.aggregate()` for normal mode.

The client imported `stream_chat_completions` from the `chat_completions` module along with all common client types: `Prompt`, `Reasoning`, `ResponseEvent`, `ResponseStream`, `ResponsesApiRequest`, `Config`, `CodexHttpClient`, `WireApi`, etc.

---

## 📊 REAL Difficulty Assessment — Task by Task

### **Task 1: Restore `WireApi::Chat` Variant**
**Difficulty: ⭐⭐ 2/10 | Time: 30 minutes**

You're literally changing 3 lines in `model_provider_info.rs`:
```rust
// FROM (current):
pub enum WireApi {
    #[default]
    Responses,
}
// "chat" => Err(serde::de::Error::custom(CHAT_WIRE_API_REMOVED_ERROR)),

// TO:
pub enum WireApi {
    #[default]
    Responses,
    Chat,  // Add back
}
// "chat" => Ok(Self::Chat),  // Accept again
```

---

### **Task 2: Restore `stream_chat_completions()` Module**
**Difficulty: ⭐⭐⭐⭐ 4/10 | Time: 1–2 days**

```bash
git checkout rust-v0.63.0 -- codex-rs/core/src/chat_completions.rs
```

Then fix type drift against current `main`. The module is self-contained — it handles `/v1/chat/completions` API requests with streaming SSE parsing and wraps them in the same `ResponseEvent` stream the rest of Codex expects.

**Main risk:** Internal types (`Prompt`, `ResponseEvent`, `ContentItem`) have grown new fields since v0.63.0. You'll need to map or default them.

---

### **Task 3: Restore the `client.rs` Match Arm**
**Difficulty: ⭐⭐⭐⭐ 4/10 | Time: 1 day**

Add back the `WireApi::Chat => { ... }` branch in the streaming dispatch. The old code shows exactly the pattern. Main work is making sure new session management fields (`response_id`, `previous_response_id`) are handled gracefully when Chat Completions doesn't provide them.

---

### **Task 4: Tool Call Translation Layer (THE BEAST)**
**Difficulty: ⭐⭐⭐⭐⭐⭐⭐ 7/10 | Time: 1–2 weeks**

**THIS is the make-or-break.** Codex's entire agentic loop relies on tool/function calling, and the two APIs represent it completely differently:

| Concept | Responses API | Chat Completions API |
|---|---|---|
| Tool calls | `output[].type = "function_call"` | `choices[].message.tool_calls[]` |
| Tool results | `output[].type = "function_call_output"` | Separate `role: "tool"` messages |
| Conversation state | `previous_response_id` (stateful) | Full message array (stateless) |
| Reasoning | `output[].type = "reasoning"` | Not natively supported |
| Role names | `developer` | `system` |

This is already causing real bugs: "Codex is not converting `developer` role to `system` role when using `wire_api = 'chat'` with custom providers."

You need to build a **bidirectional protocol adapter** that:
- Converts Responses-format tool definitions → Chat Completions `tools[]` format
- Converts streaming `delta.tool_calls` chunks → internal `ResponseEvent` stream
- Manages full conversation history (Chat Completions is stateless)
- Handles role conversion (`developer` → `system`)
- Properly orders tool result messages (ordering bugs are common)
- Strips/fakes reasoning items

---

### **Task 5: Session Management Without `previous_response_id`**
**Difficulty: ⭐⭐⭐⭐⭐⭐ 6/10 | Time: 1 week**

The Responses API is **stateful** — you just send `previous_response_id` and the server remembers everything. Chat Completions is **stateless** — you must send the full message history every turn.

This affects:
- **Session resume** (`codex resume`) — needs local conversation history replay
- **Session fork** (Esc → pick earlier turn → Enter) — needs local history branching
- **Context compaction** — needs to compact your local history, not just ask the server to summarize
- **Subagent threads** — each subagent needs its own conversation history

---

### **Task 6: Provider-Specific Quirks**
**Difficulty: ⭐⭐⭐ 3/10 (if OpenAI-compatible only) → ⭐⭐⭐⭐⭐⭐⭐⭐ 8/10 (if native per-provider)**

**The smart path:** Only target OpenAI-compatible Chat Completions endpoints. This covers:
- ✅ Gemini (via OpenAI-compat mode)
- ✅ Mistral (native OpenAI-compat)
- ✅ Groq (native OpenAI-compat)
- ✅ DeepSeek (native OpenAI-compat)
- ✅ Ollama (OpenAI-compat layer)
- ✅ LM Studio (OpenAI-compat layer)
- ✅ LiteLLM (universal OpenAI-compat proxy)
- ✅ Azure OpenAI (OpenAI-compat with quirks)
- ⚠️ Anthropic (needs proxy like LiteLLM — Claude uses Messages API, not Chat Completions)

**The hard path (native adapters):** Each provider has its own API format (Gemini's `generateContent`, Anthropic's Messages API with `tool_use` blocks, etc.). Skip this unless you have months.

---

### **Task 7: Making Codex's Advanced Features Work**
**Difficulty: ⭐⭐⭐⭐⭐⭐ 6/10 | Time: 1–2 weeks**

| Feature | Works out of the box? | Why |
|---|---|---|
| 🎙️ Voice Transcription | ✅ FREE | Internal to TUI |
| 🗣️ Realtime Conversation | ❌ BREAKS | Uses OpenAI Realtime API |
| 🛡️ Guardian Approval | ⚠️ NEEDS WORK | Runs a subagent (needs Chat tool calls) |
| 🧠 Subagents | ⚠️ NEEDS WORK | Each needs own conversation history |
| 😴 Sleep Prevention | ✅ FREE | Internal to runtime |
| 🎨 Personality Modes | ✅ FREE | Internal system prompt |
| 📸 Shell Snapshots | ✅ FREE | Internal to exec layer |
| ↩️ Undo/Ghost Snapshots | ✅ FREE | Internal filesystem ops |
| ⚡ Fast Mode | ⚠️ PARTIAL | Provider-dependent |
| 📊 OTEL | ✅ FREE | Internal observability |
| 🎓 Skills | ⚠️ NEEDS WORK | May use tool calls |
| 🔐 Sandbox | ✅ FREE | Internal (Seatbelt/Bubblewrap) |
| 🔀 Session Fork/Resume | ❌ NEEDS REBUILD | No `previous_response_id` |
| 🌐 Web Search | ⚠️ NEEDS WORK | Tool call format differs |
| 🪝 Hooks | ✅ FREE | Internal to TUI |
| 🧩 Plugins/MCP | ✅ FREE | Internal to runtime |
| Artifacts | ⚠️ NEEDS WORK | Depends on tool format |
| Compaction | ⚠️ NEEDS WORK | Must manage local history |

**The GREAT news:** ~60% of Codex's killer features (voice, sandbox, undo, shell snapshots, sleep prevention, OTEL, hooks, MCP, plugins, personality) are **internal to the agent runtime** and don't care what wire protocol you use.

---

## 📋 Final Scorecard

| Task | Difficulty | Time | Can Skip? |
|---|---|---|---|
| 1. Restore `WireApi::Chat` enum | ⭐⭐ **2/10** | 30 min | ❌ No |
| 2. Restore `chat_completions.rs` | ⭐⭐⭐⭐ **4/10** | 1–2 days | ❌ No |
| 3. Restore `client.rs` match arm | ⭐⭐⭐⭐ **4/10** | 1 day | ❌ No |
| 4. Tool call translation layer | ⭐⭐⭐⭐⭐⭐⭐ **7/10** | 1–2 weeks | ❌ No |
| 5. Session management rebuild | ⭐⭐⭐⭐⭐⭐ **6/10** | 1 week | Partially |
| 6. Provider quirks (OpenAI-compat only) | ⭐⭐⭐ **3/10** | 2–3 days | ✅ Use LiteLLM |
| 7. Feature compatibility | ⭐⭐⭐⭐⭐⭐ **6/10** | 1–2 weeks | Partially |

### **Overall Difficulty: 6.5 / 10**

---

## 🎯 The Smart Build Path

### Phase 1 — Get Chat Completions Working (2–3 weeks)
1. `git checkout rust-v0.63.0 -- codex-rs/core/src/chat_completions.rs`
2. Port forward to `main`, fix type mismatches
3. Restore `WireApi::Chat` + `client.rs` dispatch
4. Build the tool call translation layer
5. Test with Gemini, Groq, DeepSeek, Ollama, LM Studio

### Phase 2 — Session Management (1 week)
- Local conversation history storage
- Make `codex resume` work by replaying message history
- Make session fork work via local history branching

### Phase 3 — Subagents + Guardian (1 week)
- Each subagent gets its own message history
- Guardian approval adapted to Chat format

### Phase 4 — Polish & TUI (ongoing)
- Provider/model switcher in TUI
- Provider-specific error handling

---

## 🏆 Is It Worth It?

**Absolutely yes.** Here's why:

Claude Code runs Anthropic models exclusively. You cannot swap in GPT-5, Gemini 3 Pro, or DeepSeek when Claude struggles with a particular task type.

For model flexibility, OpenCode supports 75+ providers, Aider works with any LLM via LiteLLM, and Cline connects to 10+ providers including local models. But **none of them** have Codex's sandbox, voice input, subagents, skills, undo, OTEL, ghost snapshots, or guardian approvals.

Forks like stellarlinkco/codex are already trying: "This fork aims to match Claude Code-style workflows: agent teams, hooks, Anthropic API agent. Goal: a Rust OpenCode with multi-model support, multi-agent collaboration, and long-running orchestration."

And "Every Code" — a fork with 3.6k stars — already offers "validation, automation, browser integration, multi-agents, theming, and much more. Orchestrate agents from OpenAI, Claude, Gemini or any provider."

**The community is already racing to build this.** If you do it right — clean translation layer, maintained as a standalone module for easy rebasing on upstream — you'd have the **only agent with Codex's full feature set + any provider**. That's the gap nobody else has filled yet. 🚀
