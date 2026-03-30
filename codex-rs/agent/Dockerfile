# syntax=docker/dockerfile:1.7

# ── Stage 0: Frontend build ─────────────────────────────────────
FROM node:22-alpine AS web-builder
WORKDIR /web
COPY web/package.json web/package-lock.json* ./
RUN npm ci --ignore-scripts 2>/dev/null || npm install --ignore-scripts
COPY web/ .
RUN npm run build

# ── Stage 1: Build ────────────────────────────────────────────
FROM rust:1.94-slim@sha256:da9dab7a6b8dd428e71718402e97207bb3e54167d37b5708616050b1e8f60ed6 AS builder

WORKDIR /app
ARG AGENT_CARGO_FEATURES=""

# Install build dependencies
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && apt-get install -y \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

# 1. Copy manifests to cache dependencies
COPY Cargo.toml Cargo.lock ./
# Remove robot-kit from workspace members — it is excluded by .dockerignore
# and is not needed for the Docker build (hardware-only crate).
RUN sed -i 's/members = \[".", "crates\/robot-kit"\]/members = ["."]/' Cargo.toml
# Create dummy targets declared in Cargo.toml so manifest parsing succeeds.
RUN mkdir -p src benches \
    && echo "fn main() {}" > src/main.rs \
    && echo "" > src/lib.rs \
    && echo "fn main() {}" > benches/agent_benchmarks.rs
RUN --mount=type=cache,id=agent-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=agent-cargo-git,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,id=agent-target,target=/app/target,sharing=locked \
    if [ -n "$AGENT_CARGO_FEATURES" ]; then \
      cargo build --release --locked --features "$AGENT_CARGO_FEATURES"; \
    else \
      cargo build --release --locked; \
    fi
RUN rm -rf src benches

# 2. Copy only build-relevant source paths (avoid cache-busting on docs/tests/scripts)
COPY src/ src/
COPY benches/ benches/
COPY --from=web-builder /web/dist web/dist
COPY *.rs .
RUN touch src/main.rs
RUN --mount=type=cache,id=agent-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=agent-cargo-git,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,id=agent-target,target=/app/target,sharing=locked \
    rm -rf target/release/.fingerprint/agentlabs-* \
           target/release/deps/agentlabs-* \
           target/release/incremental/agentlabs-* && \
    if [ -n "$AGENT_CARGO_FEATURES" ]; then \
      cargo build --release --locked --features "$AGENT_CARGO_FEATURES"; \
    else \
      cargo build --release --locked; \
    fi && \
    cp target/release/agent /app/agent && \
    strip /app/agent
RUN size=$(stat -c%s /app/agent 2>/dev/null || stat -f%z /app/agent) && \
    if [ "$size" -lt 1000000 ]; then echo "ERROR: binary too small (${size} bytes), likely dummy build artifact" && exit 1; fi

# Prepare runtime directory structure and default config inline (no extra stage)
RUN mkdir -p /agent-data/.agent /agent-data/workspace && \
    printf '%s\n' \
        'workspace_dir = "/agent-data/workspace"' \
        'config_path = "/agent-data/.agent/config.toml"' \
        'api_key = ""' \
        'default_provider = "openrouter"' \
        'default_model = "anthropic/claude-sonnet-4-20250514"' \
        'default_temperature = 0.7' \
        '' \
        '[gateway]' \
        'port = 42617' \
        'host = "[::]"' \
        'allow_public_bind = true' \
        > /agent-data/.agent/config.toml && \
    chown -R 65534:65534 /agent-data

# ── Stage 2: Development Runtime (Debian) ────────────────────
FROM debian:trixie-slim@sha256:f6e2cfac5cf956ea044b4bd75e6397b4372ad88fe00908045e9a0d21712ae3ba AS dev

# Install essential runtime dependencies only (use docker-compose.override.yml for dev tools)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /agent-data /agent-data
COPY --from=builder /app/agent /usr/local/bin/agent

# Overwrite minimal config with DEV template (Ollama defaults)
COPY dev/config.template.toml /agent-data/.agent/config.toml
RUN chown 65534:65534 /agent-data/.agent/config.toml

# Environment setup
# Ensure UTF-8 locale so CJK / multibyte input is handled correctly
ENV LANG=C.UTF-8
# Use consistent workspace path
ENV AGENT_WORKSPACE=/agent-data/workspace
ENV HOME=/agent-data
# Defaults for local dev (Ollama) - matches config.template.toml
ENV PROVIDER="ollama"
ENV AGENT_MODEL="llama3.2"
ENV AGENT_GATEWAY_PORT=42617

# Note: API_KEY is intentionally NOT set here to avoid confusion.
# It is set in config.toml as the Ollama URL.

WORKDIR /agent-data
USER 65534:65534
EXPOSE 42617
HEALTHCHECK --interval=60s --timeout=10s --retries=3 --start-period=10s \
    CMD ["agent", "status", "--format=exit-code"]
ENTRYPOINT ["agent"]
CMD ["gateway"]

# ── Stage 3: Production Runtime (Distroless) ─────────────────
FROM gcr.io/distroless/cc-debian13:nonroot@sha256:84fcd3c223b144b0cb6edc5ecc75641819842a9679a3a58fd6294bec47532bf7 AS release

COPY --from=builder /app/agent /usr/local/bin/agent
COPY --from=builder /agent-data /agent-data

# Environment setup
# Ensure UTF-8 locale so CJK / multibyte input is handled correctly
ENV LANG=C.UTF-8
ENV AGENT_WORKSPACE=/agent-data/workspace
ENV HOME=/agent-data
# Default provider and model are set in config.toml, not here,
# so config file edits are not silently overridden
#ENV PROVIDER=
ENV AGENT_GATEWAY_PORT=42617

# API_KEY must be provided at runtime!

WORKDIR /agent-data
USER 65534:65534
EXPOSE 42617
HEALTHCHECK --interval=60s --timeout=10s --retries=3 --start-period=10s \
    CMD ["agent", "status", "--format=exit-code"]
ENTRYPOINT ["agent"]
CMD ["gateway"]
