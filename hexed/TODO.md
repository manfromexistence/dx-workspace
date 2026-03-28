# Project TODO

> Auto-managed by AI. Updated after every completed or failed task.

## In Progress

- [ ] Add match arm for `WireApi::Chat` in the HTTP dispatch layer

## Pending
- [ ] Add match arm for `WireApi::Chat` in the HTTP dispatch layer
- [ ] Relax OSS provider validation in `core/src/config/mod.rs`
- [ ] Verify Chat Completions support with `just dx`
- [ ] Restore `WireApi::AnthropicMessages` and implement Anthropic-specific requests, SSE, and tools formats
- [ ] Restore `WireApi::GeminiGenerateContent` and implement Gemini-specific requests, SSE, and tools formats
- [ ] Verify Anthropic and Gemini support

## Completed

- [x] ~~Restore `WireApi::Chat` and remove deprecation/error guards in `core/src/model_provider_info.rs` and `core/src/codex.rs`~~ ✅
- [x] ~~Add built-in chat-based providers to `built_in_model_providers()` in `core/src/model_provider_info.rs`~~ ✅
- [x] ~~Add `codex-api/src/requests/chat.rs` and `codex-api/src/sse/chat.rs` for Chat Completions~~ ✅
- [x] ~~Implement `create_tools_json_for_chat_api` in `core/src/tools/spec.rs`~~ ✅

## Blocked / Failed
