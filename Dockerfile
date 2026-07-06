# Stage 1: Build the application
FROM ubuntu:24.04 AS builder

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libpq-dev \
    git \
    unzip \
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Add the wasm32 compilation target
RUN rustup target add wasm32-unknown-unknown

# Install Dioxus CLI
RUN curl -sSL https://dioxus.dev/install.sh | bash

# Set the working directory
WORKDIR /app

# Copy the project files
COPY . .

# Enable SQLx offline mode so compilation doesn't require a live database connection
ENV SQLX_OFFLINE=true

# Build the project (includes both Axum server and client WASM assets)
RUN dx build --release

# Find the server binary in the target directory and copy it to a predictable path
RUN find target -type f -name "krynon" -path "*/server-release/*" -exec cp {} /app/krynon-server \;

# Stage 2: Create the runtime image
FROM ubuntu:24.04

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the server binary and assets from the builder stage
COPY --from=builder /app/krynon-server /app/krynon
COPY --from=builder /app/target/dx/krynon/release/web/public /app/public

# Default environment configuration
ENV IP=0.0.0.0
ENV PORT=8080

# Expose the application port
EXPOSE 8080

# Run the server binary
CMD ["./krynon"]
