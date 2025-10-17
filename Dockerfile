# To test locally, use: docker buildx build --platform linux/arm64 -t peak_backend --load . && docker run -p 8080:8080 peak_backend

# --- Builder stage ---
FROM rust:1.81-slim AS builder
WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config build-essential libssl-dev git && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo files first (for caching)
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src

# Build release binary
RUN cargo build --release

# --- Production stage ---
FROM debian:bookworm-slim
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /usr/src/app/target/release/peak_backend ./peak_backend

# Expose port
EXPOSE 8080

# Run the binary
CMD ["./peak_backend"]
