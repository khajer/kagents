# syntax=docker/dockerfile:1

# ---- Build stage ----
FROM rust:1-slim-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Build dependencies first so they're cached across source-only changes.
COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src/server src/kcli \
    && echo "fn main() {}" > src/server/kserve.rs \
    && echo "fn main() {}" > src/kcli/kcli.rs \
    && cargo build --release --bin kserve --bin kcli \
    && rm -rf src

COPY src ./src
RUN touch src/server/kserve.rs src/kcli/kcli.rs \
    && cargo build --release --bin kserve --bin kcli

# ---- Runtime stage ----
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/kserve /usr/local/bin/kserve
COPY --from=builder /app/target/release/kcli /usr/local/bin/kcli

# kserve creates agents.sqlite and workspace/ relative to its working
# directory — mount a volume at /app to persist both.
EXPOSE 6411

CMD ["kserve"]
