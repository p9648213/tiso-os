FROM rust:1.87.0-slim

# Install dev dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Install cargo-watch
RUN cargo install cargo-watch

# Install Node.js and Tailwind CLI
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get update && apt-get install -y nodejs && \
    npm install -g @tailwindcss/cli && \
    npm install -g nodemon && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Pre-cache Rust dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build

# Copy full app
COPY . .

# Copy and prepare entrypoint
COPY /infra/dev/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Default command
CMD ["/entrypoint.sh"]
