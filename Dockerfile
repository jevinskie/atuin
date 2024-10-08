FROM lukemathwalker/cargo-chef:latest-rust-1.80.1-slim-bookworm AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

# Ensure working C compile setup (not installed by default in arm64 images)
RUN apt update && apt install build-essential -y

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

COPY . .
RUN cargo build --bin atuin

FROM debian:bookworm-slim AS runtime

RUN groupadd -g 1002 atuin-server && useradd -c 'atuin-server user' -g 1002 -u 1002 -N atuin-server && mkdir /config && chown atuin-server:atuin-server /config && mkdir /certs && chown atuin-server:atuin-server /certs
# Install ca-certificates for webhooks to work
RUN apt update && apt install ca-certificates gdbserver libc6-dbg -y && rm -rf /var/lib/apt/lists/*
WORKDIR app

USER atuin-server

ENV TZ=Etc/UTC
ENV RUST_LOG=atuin::api=info
ENV ATUIN_CONFIG_DIR=/config

COPY --from=builder /app/target/debug/atuin /usr/local/bin
ENTRYPOINT ["/usr/bin/gdbserver", "127.0.0.1:2222", "--disable-randomization", "/usr/local/bin/atuin"]
