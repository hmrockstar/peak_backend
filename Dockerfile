# --- Builder stage ---
FROM rust:1.82-slim AS builder
WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        pkg-config \
        libssl-dev \
        git \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo files first (for caching)
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build release binary
RUN cargo build --release

# --- Production stage ---
FROM debian:bookworm-slim
WORKDIR /app

# Install only the runtime OpenSSL library
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libssl3 \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from builder
COPY --from=builder /usr/src/app/target/release/peak_backend ./peak_backend

# Expose port
EXPOSE 8080

# Run the binary
CMD ["./peak_backend"]
