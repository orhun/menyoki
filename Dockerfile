FROM lukemathwalker/cargo-chef:0.1.51-rust-1.67-slim-buster AS chef
WORKDIR app

# Planner
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder
FROM chef AS builder
RUN apt-get update && \
    apt-get install -y \
    --no-install-recommends \
    --allow-unauthenticated \
    pkg-config libx11-dev libxrandr-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked
RUN rm -f target/release/deps/menyoki*

# Runner
FROM debian:buster-slim as runner
WORKDIR /root/
RUN apt-get update && \
    apt-get install -y \
    --no-install-recommends \
    --allow-unauthenticated \
    libx11-dev libxrandr-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/menyoki /usr/local/bin
ENTRYPOINT ["menyoki"]
