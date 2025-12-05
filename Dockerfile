# Use official Rust image as builder
FROM rust:1.75 as builder

# Create a new empty project
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a minimal image for the runtime
FROM debian:bookworm-slim

# Install runtime dependencies (needed for MySQL client and SSL)
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/beekeeper_API /usr/local/bin/beekeeper_API

# Expose the port the API runs on
EXPOSE 3000

# Run the binary
CMD ["beekeeper_API"]
