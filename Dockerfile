FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/telegram_Bot ./

RUN touch filmler.txt diziler.txt izlenen_filmler.txt izlenen_diziler.txt

COPY .env ./.env

CMD ["./telegram_Bot"]