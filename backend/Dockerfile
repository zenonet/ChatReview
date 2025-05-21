FROM rust:1.86.0-alpine AS builder

WORKDIR /app
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./.prod.env ./.env

RUN apk add openssl-dev musl-dev

RUN cargo install --path .

FROM alpine:latest
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/backend .
COPY --from=builder /app/.env ./.env

EXPOSE 80

CMD ["./backend"]