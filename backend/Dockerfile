FROM rust:1.86.0-alpine AS builder

WORKDIR /app
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./.env.production ./.env

RUN apk add pkgconfig openssl openssl-dev musl-dev libgcc libstdc++ ncurses-libs

RUN RUSTFLAGS="-C target-feature=-crt-static" cargo install --path .

FROM alpine:latest
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apk add openssl openssl-dev musl-dev libgcc libstdc++ ncurses-libs
WORKDIR /app
COPY --from=builder /app/target/release/backend .
COPY --from=builder /app/.env ./.env

EXPOSE 80

CMD ["./backend"]