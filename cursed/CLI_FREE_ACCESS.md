Awesome so is there any providers that gives these types of daily free ai access as for 9th March 2026?? List all of them!!!

OK, I now have a very comprehensive picture. Here's the **honest, detailed breakdown** of what you can realistically reuse in your Rust CLI:

---

## ⚠️ THE HARD TRUTH FIRST

You **cannot simply call the same API these CLIs use for free** from your own custom Rust CLI without implementing their exact authentication flows. These aren't open public APIs with a shared password — they are **OAuth-based flows tied to the user's own account**. The free quota is **per-user**, not per-app. So the good news is: **you CAN do this**, but you need to implement the OAuth flows yourself.

---

## 🟢 1. GEMINI CLI — Free Model Access

### How the Auth Works
Access to Gemini CLI begins with a generous free tier, perfect for experimentation and light use. Your free usage is governed by limits that depend on your authorization type. For users who authenticate by using their Google account to access Gemini Code Assist for individuals.

The auth types supported are: `'oauth'`, `'oauth-personal'`, `'api-key'`, `'gemini-api-key'`, `'vertex-ai'`, `'google-auth-library'` — defaulting to `'oauth-personal'`.

### Two Ways to Get Free Access in Your Rust CLI:

#### **Option A: Google OAuth (Personal Account) — BEST FREE TIER**
- The free license gets you access to Gemini 2.5 Pro and its 1 million token context window. The free tier allows for 60 model requests per minute and 1,000 requests per day at no charge.
- Users want free access to Google's Gemini models without needing to manage API keys or set up billing.
- The flow: Open browser → User logs in with Google → Get OAuth tokens → Call Gemini API
- Users can authenticate via OAuth (browser opens for Google login) and send prompts to Gemini models without API key configuration, receiving streaming responses from the free tier, with OAuth tokens automatically refreshed when needed.

#### **Option B: Gemini API Key (Free Tier) — FALLBACK**
- If you don't want to authenticate using your Google account, you can use an API key from Google AI Studio. To authenticate, obtain your API key from Google AI Studio and set the GEMINI_API_KEY environment variable to your key.
- But the free API key tier has **lower limits** for Pro model:

### Free Tier Limits (as of March 2026):

| Auth Method | Model | RPM | RPD | Notes |
|---|---|---|---|---|
| **Google OAuth (personal)** | Gemini model family (CLI picks) | **60** | **1,000** | Best free option |
| **API Key (free)** | Gemini 2.5 Pro | 5 | 100 | Much lower |
| **API Key (free)** | Flash models | ~10-15 | 500-1,000 | Higher than Pro |

**Key nuances:**
- Current quota is for 1,000 model requests per day, yes, but that does not currently mean 1,000 Gemini 2.5 Pro model requests per day.
- Gemini Flash is the default for the free tier. The Gemini CLI free quota is typically 1,000 requests per day and up to 60 RPM on Flash.
- A single prompt does not necessarily equate to "one model request". A single prompt could actually trigger dozens of model requests (API calls).
- Many developers report that Gemini CLI automatically falls back to Flash when Pro quota is exhausted.
- Requests per day (RPD) quotas reset at midnight Pacific time.

### What You Need to Implement in Rust:

```
┌──────────────────────────────────────────────────┐
│  YOUR RUST CLI                                   │
│                                                  │
│  Option A: Google OAuth 2.0 Flow                 │
│  ┌────────────────────────────────────────────┐  │
│  │ 1. Open browser → accounts.google.com      │  │
│  │ 2. User logs in with personal Google acct   │  │
│  │ 3. Listen on localhost for callback         │  │
│  │ 4. Exchange auth code for access_token      │  │
│  │ 5. Cache token in ~/.your-cli/oauth.json    │  │
│  │ 6. Auto-refresh when expired                │  │
│  └────────────────────────────────────────────┘  │
│                                                  │
│  Option B: GEMINI_API_KEY env var                │
│  ┌────────────────────────────────────────────┐  │
│  │ User gets free key from aistudio.google.com │  │
│  │ export GEMINI_API_KEY="AIza..."             │  │
│  └────────────────────────────────────────────┘  │
│                                                  │
│  API Endpoint:                                   │
│  https://generativelanguage.googleapis.com/v1beta│
│                                                  │
│  Models: gemini-2.5-pro, gemini-2.5-flash,       │
│          gemini-3-flash (latest)                 │
└──────────────────────────────────────────────────┘
```

### Rust Crates You'll Need:
```toml
[dependencies]
oauth2 = "4"           # OAuth 2.0 flow
reqwest = "0.12"       # HTTP client
tokio = "1"            # Async runtime  
serde_json = "1"       # JSON parsing
open = "5"             # Open browser
keyring = "3"          # Secure credential storage
```

---

## 🟢 2. QWEN CODE — Free Model Access

### How the Auth Works
Qwen Code supports two authentication methods. Qwen OAuth (recommended): sign in with your qwen.ai account in a browser. API-KEY: use an API key to connect to any supported provider.

The OAuth uses Device Flow (RFC 8628): Opens your browser to chat.qwen.ai for authentication, with automatic polling to detect authorization completion, token storage saved to ~/.qwen/oauth_creds.json, and auto-refresh for renewing tokens.

### Free Tier Limits (as of March 2026):

**Official docs say:**
- Cost & quota: free, with a quota of 60 requests/minute and 1,000 requests/day.
- The CLI auth dialog shows: Qwen OAuth — Free · Up to 1,000 requests/day.

**But some third-party integrations report higher limits:**
- Free Tier: 2,000 requests/day and 60 requests/minute with no token limits, available during a promotional period.
- This plugin enables you to use Qwen models (Coder, Max, Plus and more) with 2,000 free requests per day — no API key or credit card required!

**Summary:**

| Auth | Model | RPM | RPD | Token Limits |
|---|---|---|---|---|
| **Qwen OAuth** | qwen3-coder-plus | **60** | **1,000 – 2,000** *(promotional)* | No token limits |
| **Coding Plan (paid)** | qwen3-coder-plus | — | **6,000/5hrs** | — |

- Limits reset at midnight UTC.
- A specific quota for different models is not specified; model fallback may occur to preserve shared experience quality.

### API Endpoint & OAuth Details:

- Default base URL: `https://portal.qwen.ai/v1`. Tokens auto-refresh; re-run the login command if refresh fails or access is revoked.
- Credentials are stored at `~/.qwen/oauth_creds.json`.
- The auth invokes a hook which opens a browser login page for qwen.ai. After successful login, credentials are cached locally. Subsequent sessions detect the cached token automatically — no browser step needed.

### What You Need to Implement in Rust:

```
┌──────────────────────────────────────────────────────┐
│  YOUR RUST CLI — Qwen Free Access                    │
│                                                      │
│  OAuth Device Flow (RFC 8628):                       │
│  ┌────────────────────────────────────────────────┐  │
│  │ 1. POST to Qwen device auth endpoint           │  │
│  │ 2. Get device_code + user_code + verify URL     │  │
│  │ 3. Open browser → chat.qwen.ai/device           │  │
│  │ 4. User enters code & logs in                   │  │
│  │ 5. Poll token endpoint until authorized         │  │
│  │ 6. Receive access_token + refresh_token         │  │
│  │ 7. Cache to ~/.your-cli/qwen_oauth_creds.json   │  │
│  │ 8. Auto-refresh 30s before expiration            │  │
│  └────────────────────────────────────────────────┘  │
│                                                      │
│  API Endpoint (OpenAI-compatible!):                  │
│  https://portal.qwen.ai/v1                           │
│                                                      │
│  Models: qwen3-coder-plus, qwen3-max,                │
│          qwen-plus-latest, qwen3-vl-plus (vision)    │
│                                                      │
│  Format: Standard OpenAI chat completions API        │
│  POST /v1/chat/completions                           │
│  Authorization: Bearer <access_token>                │
└──────────────────────────────────────────────────────┘
```

**Huge advantage:** The Qwen API is multi-protocol and OpenAI-compatible, so you sign in with Qwen OAuth for 1,000 free requests/day through standard OpenAI `chat/completions` format!

---

## 📊 SIDE-BY-SIDE COMPARISON

| Feature | **Gemini CLI Free** | **Qwen Code Free** |
|---|---|---|
| **Free RPD** | 1,000 (combined models) | 1,000–2,000 (promotional) |
| **Free RPM** | 60 | 60 |
| **Top Model** | Gemini 2.5 Pro (limited) + Flash | qwen3-coder-plus |
| **Context Window** | 1M tokens | 1M tokens |
| **OAuth Type** | Authorization Code Flow | Device Flow (RFC 8628) |
| **API Format** | Google Gemini API (custom) | **OpenAI-compatible** ✅ |
| **Token Storage** | `~/.gemini/` | `~/.qwen/oauth_creds.json` |
| **API Endpoint** | `generativelanguage.googleapis.com` | `portal.qwen.ai/v1` |
| **Quota Reset** | Midnight Pacific Time | Midnight UTC |
| **Token Limits** | Varies by model | No token limits (promotional) |

---

## 🏗️ RECOMMENDED ARCHITECTURE FOR YOUR RUST CLI

```
your-rust-cli/
├── src/
│   ├── main.rs
│   ├── auth/
│   │   ├── mod.rs           # Auth trait/enum
│   │   ├── gemini_oauth.rs  # Google OAuth 2.0 (Auth Code flow)
│   │   ├── qwen_oauth.rs    # Qwen OAuth (Device Code flow) 
│   │   └── api_key.rs       # BYOK fallback (env vars)
│   ├── providers/
│   │   ├── mod.rs           # Provider trait  
│   │   ├── gemini.rs        # Gemini API client
│   │   └── openai_compat.rs # OpenAI-compat (covers Qwen, DeepSeek, etc.)
│   ├── config/
│   │   ├── mod.rs
│   │   └── credentials.rs   # Token caching & refresh
│   └── cli/
│       └── mod.rs           # TUI / interactive prompt
```

### Priority implementation order:
1. **Qwen OAuth (Device Flow)** — Easiest to implement, OpenAI-compatible API, generous limits
2. **Gemini API Key** — Simple env var, works immediately
3. **Gemini Google OAuth** — Most complex but best free tier
4. **BYOK (any OpenAI-compatible)** — For users who have their own keys

Good luck building your Rust CLI! The key takeaway is: **the free access is per-user via OAuth, not a shared secret** — so you implement the OAuth flow, the user logs in with *their* account, and they get their own free quota. 🚀

Great question! Let me do a thorough search for ALL providers offering free daily AI access as of March 9, 2026.Here is the **COMPLETE list of every provider offering free daily AI API access** as of March 9, 2026, with verified limits from the latest sources:

---

# 🏆 TIER 1 — PERMANENT FREE TIERS (Best for Your Rust CLI)

## 1. 🟢 Google Gemini (Google AI Studio)
The free tier includes Gemini 2.5 Pro, Flash, and Flash-Lite models with rate limits ranging from 5-15 requests per minute and 100-1,000 requests per day depending on the model.

| Model | RPM | RPD |
|---|---|---|
| Gemini 2.5 Pro | 5 | 100 |
| Gemini 2.5 Flash | 10 | 250 |
| Gemini 2.0 Flash-Lite | 30 | 1,000+ |

- No credit card requirement, 1 million token context window, and access to three capable models makes it one of the most generous free offerings in the AI API landscape.
- The daily limits reset at midnight Pacific Time.
- **Auth:** API Key from `aistudio.google.com` (free) OR Google OAuth (1,000 RPD via Gemini CLI-style auth)
- **API:** `https://generativelanguage.googleapis.com/v1beta`
- ✅ **OpenAI-compatible endpoint available**

---

## 2. 🟢 Qwen Code (Alibaba/Qwen)
Multiple free authentication options including Qwen OAuth, 2000 requests per day.

| Auth | Model | RPM | RPD |
|---|---|---|---|
| Qwen OAuth | qwen3-coder-plus | 60 | 1,000–2,000 |

- Sign in via Qwen OAuth to use it directly, or get an API key from Alibaba Cloud ModelStudio to access it through the OpenAI-compatible API.
- **API:** `https://portal.qwen.ai/v1` (OpenAI-compatible!)
- **Auth:** OAuth Device Flow or API key
- ✅ **OpenAI-compatible**

---

## 3. 🟢 Groq
Groq provides the industry's fastest inference speed at 800+ tokens/sec free API! Free service with typical quota ~14,400/day, supporting Llama 3.3, Mixtral, Gemma 2 and more.

| Metric | Limit |
|---|---|
| RPD | ~14,400 |
| RPM | 30 |
| TPM | 60,000+ |

- Fully compatible with OpenAI API format, allowing you to switch existing code to Groq by simply changing the base_url.
- **Endpoint:** `https://api.groq.com/openai/v1`
- **Models:** Llama 3.3 70B, DeepSeek R1 Distill, Mixtral, Gemma 2
- ✅ **OpenAI-compatible**, no credit card needed

---

## 4. 🟢 Cerebras
The free tier includes Llama 3.3 70B, Qwen3 32B, Qwen3 235B, and OpenAI's open-source GPT-OSS 120B - with 30 requests per minute and 1 million tokens per day.

| Metric | Limit |
|---|---|
| RPM | 30 |
| TPM | 60,000 |
| Tokens/Day | 1,000,000 |

- No waitlist, no credit card.
- Cerebras Inference powers the world's top coding models at 2,000 tokens/sec.
- **Endpoint:** `https://api.cerebras.ai/v1` (OpenAI-compatible)
- ✅ **OpenAI-compatible**

---

## 5. 🟢 Mistral (La Plateforme)
Limits (per-model): 1 request/second, 500,000 tokens/minute, 1,000,000,000 tokens/month

| Metric | Limit |
|---|---|
| RPS | 1 |
| TPM | 500,000 |
| Tokens/Month | **1 BILLION** |

- Mistral offers free API access to their smaller models.
- 1 billion tokens per month is hard to beat.
- **Models:** Mistral 7B, Mistral Small, Codestral (for code)
- ✅ **OpenAI-compatible**

---

## 6. 🟢 OpenRouter (Unified Gateway)
Limits: 20 RPM on free models, 50 requests per day without a paid balance (increased to 1,000/day if you have $10+ account balance). Free models are marked with a :free suffix in the model list.

| Metric | Limit |
|---|---|
| RPM | 20 |
| RPD (no balance) | 50 |
| RPD ($10+ balance) | 1,000 |

- Free models include DeepSeek R1 and V3, Llama 4 Maverick and Scout, Qwen3 235B, and others.
- **Endpoint:** `https://openrouter.ai/api/v1`
- ✅ **OpenAI-compatible**

---

## 7. 🟢 Cloudflare Workers AI
Cloudflare gives 10,000 free inference requests per day across multiple open-source models, including Llama, Mistral, and Stable Diffusion.

| Metric | Limit |
|---|---|
| RPD | 10,000 |
| Neurons/Day | 10,000 |

- The edge deployment means low latency for users worldwide.
- ⚠️ Cloudflare uses its own REST API format, not OpenAI-compatible. That means significantly more configuration work.

---

## 8. 🟢 NVIDIA NIM
NVIDIA NIM — Free tier (provider quota by model).

- Free API key at `build.nvidia.com`
- Models vary (Llama, Mistral, Qwen, code models)
- Free-coding-models configures tools to use NVIDIA NIM's remote API, so models run on NVIDIA's infrastructure. No GPU or local setup required.
- ✅ **OpenAI-compatible**

---

## 9. 🟢 SambaNova Cloud
SambaNova offers a genuinely persistent free tier, not just credits, with access to Llama 3.3 70B, Llama 3.1 (up to 405B), Qwen 2.5 72B, and other models on their custom RDU hardware. You also get $5 in initial credits (valid 30 days) on top of the free tier.

| Metric | Limit |
|---|---|
| RPM | 10–30 (model-dependent) |

- Free tier persists indefinitely beyond the credits.
- ✅ **OpenAI-compatible**

---

## 10. 🟢 HuggingFace Inference
HuggingFace offers free inference for thousands of models hosted on their platform. The free tier is rate-limited but sufficient for development.

- Some popular models are supported even if they exceed 10GB.
- Thousands of models available (Llama, Mistral, specialized models)
- ✅ **OpenAI-compatible endpoint available**

---

## 11. 🟢 Cohere
Cohere's free tier targets developers building with retrieval-augmented generation (RAG).

- Free access to Command R+, Embed 4, Rerank 3.5
- 30 requests/minute, 60,000 tokens/minute, 900 requests/hour, 1,000,000 tokens/hour, 14,400 requests/day, 1,000,000 tokens/day
- ✅ **OpenAI-compatible**

---

## 12. 🟢 GitHub Models
GitHub Models gives you playground and API access to a curated selection of high-quality models including GPT-4o, GPT-4.1, o3, xAI Grok-3, DeepSeek-R1, and others. It is aimed squarely at developers who want to experiment with models before integrating them.

- Tight rate limits, split into tiers
- Free with GitHub account

---

## 13. 🟢 GitHub Copilot Free Tier
Free Tier -- Available to everyone, 2,000 completions + 50 chat messages/month.

- In late 2025, GitHub launched a free tier for Copilot that is available to everyone with a GitHub account. This is the easiest way to get started.

---

## 14. 🟢 Fireworks AI
Fireworks offers free API access at 10 RPM without a payment method, enough for light prototyping. Adding a payment method unlocks up to 6,000 RPM.

- Models: Llama 3.1 405B, DeepSeek R1, and hundreds of others
- ✅ **OpenAI-compatible**

---

## 15. 🟢 Ollama (Local/Self-hosted)
Ollama lets you run open-source LLMs on your own machine. It is completely free, works offline, and keeps all data private.

- **Endpoint:** `http://localhost:11434/v1`
- Unlimited (limited only by hardware)
- ✅ **OpenAI-compatible**

---

# 🟡 TIER 2 — SIGNUP CREDITS (Expire After 30-90 Days)

DeepSeek and Together AI don't have permanent free tiers. Those are signup credits that expire after 30 to 90 days.

| Provider | Free Credits | Validity | Notes |
|---|---|---|---|
| **DeepSeek** | 5 million free tokens (~$8.40 value) | 30 days | No credit card required; OpenAI-compatible |
| **Together AI** | $25 free credits | ~30 days | 200+ open-source models |
| **SambaNova** | $5 initial credits | 30 days | On top of persistent free tier |
| **Fireworks** | Free credits | Varies | 10 RPM free without payment method |

---

# 🟠 TIER 3 — AGGREGATORS / GATEWAYS

| Provider | Free Offer | API Format |
|---|---|---|
| **Puter.js** | Access hundreds of LLMs completely free, without any API keys. Access GPT-5, Claude, Gemini, Llama, DeepSeek, Mistral, and 500+ other models. | OpenAI-compatible |
| **OpenRouter** | 50 RPD free (24+ models) | OpenAI-compatible |
| **AIML API** | Free tier with limited models | OpenAI-compatible |
| **mlvoca.com** | Currently works without any kind of rate limit or API key. Can be used for free without token/call limits. DeepSeek R1 & TinyLlama | Ollama-compatible |
| **ApiFreeLLM** | 200B+ models accessible to everyone - no token costs, no limits, forever free. Sign in with Google to get your free API key instantly. | OpenAI-compatible |

---

# 📊 MASTER COMPARISON TABLE

| # | Provider | RPD (Free) | RPM | Token Limits | Auth | OpenAI-Compat |
|---|---|---|---|---|---|---|
| 1 | **Google Gemini OAuth** | **1,000** | 60 | 1M context | OAuth / API Key | ✅ |
| 2 | **Qwen Code OAuth** | **1,000–2,000** | 60 | No token limit | OAuth Device Flow | ✅ |
| 3 | **Groq** | **~14,400** | 30 | 60K TPM | API Key | ✅ |
| 4 | **Cerebras** | **~33K** (1M tok/day) | 30 | 1M tokens/day | API Key | ✅ |
| 5 | **Mistral** | Unlimited (1 RPS) | ~60 | **1B tokens/month** | API Key | ✅ |
| 6 | **Cohere** | **14,400** | 30 | 1M tokens/day | API Key | ✅ |
| 7 | **Cloudflare** | **10,000** | — | 10K neurons/day | API Key | ❌ (Custom) |
| 8 | **OpenRouter** | 50 (or 1K w/$10) | 20 | Varies | API Key | ✅ |
| 9 | **NVIDIA NIM** | Model-dependent | Varies | Varies | API Key | ✅ |
| 10 | **SambaNova** | Persistent free | 10–30 | Varies | API Key | ✅ |
| 11 | **HuggingFace** | Rate-limited | Varies | Varies | API Token | ✅ |
| 12 | **GitHub Models** | Tight limits | Varies | Varies | GitHub account | ✅ |
| 13 | **Fireworks** | 10 RPM free | 10 | Varies | API Key | ✅ |
| 14 | **DeepSeek** | 5M tokens (30d credit) | Varies | 128K context | API Key | ✅ |
| 15 | **Ollama** | **∞ (local)** | ∞ | Hardware-limited | None | ✅ |
| 16 | **Puter.js** | Undocumented limits | — | — | OAuth / None | ✅ |
| 17 | **mlvoca.com** | Unlimited (shared) | — | — | None | Ollama-compat |
| 18 | **ApiFreeLLM** | "Unlimited" | — | — | Google Sign-in | ✅ |

---

# 🏗️ RECOMMENDED PRIORITY FOR YOUR RUST CLI

Since almost all providers above use **OpenAI-compatible APIs**, here's the best implementation strategy:

```
┌─────────────────────────────────────────────────────────────┐
│  YOUR RUST CLI — Universal Free AI Access                    │
│                                                              │
│  Priority 1: OAuth-based FREE tiers (highest daily limits)   │
│  ├── Google Gemini OAuth → 1,000 RPD (custom API)           │
│  └── Qwen OAuth          → 1,000-2,000 RPD (OpenAI-compat) │
│                                                              │
│  Priority 2: API-key-based FREE tiers (OpenAI-compatible)    │
│  ├── Groq          → ~14,400 RPD  ⚡ Fastest                │
│  ├── Cerebras      → 1M tokens/day ⚡ Ultra-fast             │
│  ├── Mistral       → 1B tokens/month  📦 Highest volume     │
│  ├── Cohere        → 14,400 RPD                              │
│  ├── NVIDIA NIM    → Model-dependent                         │
│  ├── SambaNova     → Persistent free tier                    │
│  ├── HuggingFace   → Thousands of models                    │
│  ├── OpenRouter    → 50-1,000 RPD, 24+ free models          │
│  └── Fireworks     → 10 RPM free                             │
│                                                              │
│  Priority 3: Local (unlimited, no internet)                   │
│  └── Ollama        → ∞ (your hardware)                       │
│                                                              │
│  All use: POST /v1/chat/completions                          │
│  All use: Authorization: Bearer <key>                        │
│  Only diff: base_url per provider                            │
└─────────────────────────────────────────────────────────────┘
```

The beauty is that since **~15 of these 18 providers are OpenAI-compatible**, your Rust CLI only needs **ONE generic `chat/completions` client** + a provider config that swaps the `base_url`. The only special implementations needed are:

1. **Google Gemini OAuth** (custom auth + custom API format)
2. **Qwen OAuth** (Device Flow auth, but then standard OpenAI format)
3. **Cloudflare Workers AI** (custom REST format)

Everything else is just `base_url` + `api_key` + `model_name`. Good luck! 🚀
