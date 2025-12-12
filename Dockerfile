# Use official Rust image as builder
FROM rust:latest AS builder

# Create a new empty project
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release
RUN cargo build --release --bin seed

# Use a minimal image for the runtime
FROM debian:bookworm-slim

# Install runtime dependencies (needed for MySQL client and SSL)
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates default-mysql-client && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/default /usr/local/bin/default
COPY --from=builder /app/target/release/seed /usr/local/bin/seed

COPY reset.sql /app/reset.sql
COPY schema.sql /app/schema.sql

# Copy the entrypoint script
COPY docker-entrypoint.sh /app/docker-entrypoint.sh
RUN chmod +x /app/docker-entrypoint.sh

# Expose the port the API runs on
EXPOSE 3000

# Run the entrypoint script
ENTRYPOINT ["/app/docker-entrypoint.sh"]
