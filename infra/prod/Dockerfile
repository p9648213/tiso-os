# ---- Build Stage ----
FROM rust:1.87.0-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Pre-copy for caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Copy actual source
COPY . .

# Build your actual app
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bookworm-slim AS runtime

# Install OpenSSL 3 and ca-certificates
RUN apt-get update && apt-get install -y libssl3 ca-certificates && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Set non-root user for safety
RUN adduser --system --group appuser

# Create app directory
WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/tiso-os /app/tiso-os

# Use non-root user
USER appuser

# Run the binary
CMD ["./tiso-os"]