# -------- Planner
FROM rust:1-slim-bullseye AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# -------- Builder
FROM rust:1-slim-bullseye AS builder
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the cached layer
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# -------- Runtime
FROM debian:bullseye-slim
WORKDIR /app
RUN apt-get update && \
    apt-get install -y ca-certificates openssl && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust_supabase_server .
COPY .env .env
EXPOSE 3000
CMD ["./rust_supabase_server"]
