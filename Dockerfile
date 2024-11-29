# Build stage
FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Final run stage
FROM debian:bookworm-slim AS runner

# Install libsqlite3
RUN apt-get update && apt-get install -y libsqlite3-0 && rm -rf /var/lib/apt/lists/*
RUN sqlx database create --database-url "sqlite:./database.db"
RUN sqlx migrate run

WORKDIR /app
COPY --from=builder /app/target/release/simple-books-api /app/simple-books-api
CMD ["/app/simple-books-api"]