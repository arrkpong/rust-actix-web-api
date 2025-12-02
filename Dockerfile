# syntax=docker/dockerfile:1

FROM rust:1.91-slim-bookworm AS builder

# Install build dependencies for sea-orm Postgres + OpenSSL
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src
COPY src ./src
RUN cargo build --release --locked

# Build migration binary
COPY migration/Cargo.toml migration/Cargo.lock ./migration/
RUN mkdir migration/src
COPY migration/src ./migration/src
RUN cargo build --release --locked --manifest-path migration/Cargo.toml

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates openssl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/rust-actix-web-api /usr/local/bin/rust-actix-web-api
COPY --from=builder /app/migration/target/release/migration /usr/local/bin/migration

# Ensure the server listens on all interfaces inside the container
ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE 8080

CMD ["rust-actix-web-api"]
