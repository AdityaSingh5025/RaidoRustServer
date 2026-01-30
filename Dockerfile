# Build stage
FROM rust:1.80-slim-bullseye AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

# Create dummy src/main.rs to cache dependencies
RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY . .

# Build the actual application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install OpenSSL (required for sqlx) and ca-certificates
RUN apt-get update && \
    apt-get install -y openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/rust_supabase_server /app/rust_supabase_server
COPY .env .env

EXPOSE 3000

CMD ["./rust_supabase_server"]
