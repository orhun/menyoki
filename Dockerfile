# Planner
FROM lukemathwalker/cargo-chef:0.1.33-rust-1.58-slim-buster as planner
WORKDIR app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Cacher
FROM lukemathwalker/cargo-chef:0.1.33-rust-1.58-slim-buster as cacher
WORKDIR app
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    --allow-unauthenticated pkg-config \
    libx11-dev libxrandr-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Builder
FROM rust:1.58-slim-buster as builder
WORKDIR app
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    --allow-unauthenticated pkg-config \
    libx11-dev libxrandr-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --locked
RUN rm -f target/release/deps/menyoki*

# Runner
FROM debian:buster-slim as runner
WORKDIR /root/
RUN apt-get update && apt-get install -y \
    --no-install-recommends --allow-unauthenticated \
    libx11-dev libxrandr-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/menyoki /usr/local/bin
ENTRYPOINT ["menyoki"]
