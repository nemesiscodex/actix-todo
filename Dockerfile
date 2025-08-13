FROM rust:1.87-slim AS builder
WORKDIR /app

# Build-time dependencies for compiling the app and diesel_cli (Postgres)
RUN apt-get update -qq \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libpq-dev \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies first
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -f src/main.rs

# Copy source and build the release binary
COPY . .
RUN cargo build --release

# Install diesel_cli to run DB migrations (uses libpq)
RUN cargo install diesel_cli --no-default-features --features postgres

FROM debian:bookworm-slim
WORKDIR /app

# Runtime dependencies for diesel_cli (libpq)
RUN apt-get update -qq \
    && apt-get install -y --no-install-recommends \
        libpq5 \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary and diesel cli
COPY --from=builder /app/target/release/actix-todo /app/actix-todo
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copy migrations metadata for diesel
COPY migrations ./migrations
COPY diesel.toml ./diesel.toml

# Default DATABASE_URL matches the docker compose setup
ENV DATABASE_URL=postgres://actix:actix@postgres:5432/actix

EXPOSE 8080

# Create startup script that runs migrations first
RUN echo '#!/bin/bash\n\
echo "Waiting for database..."\n\
until diesel setup --database-url "$DATABASE_URL"; do\n\
  echo "Database not ready, retrying in 5 seconds..."\n\
  sleep 5\n\
done\n\
echo "Database ready, starting application..."\n\
exec /app/actix-todo' > /app/start.sh && chmod +x /app/start.sh

CMD ["/app/start.sh"]