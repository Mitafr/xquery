FROM rust:1.66-slim-buster AS builder

WORKDIR /usr/src/server
COPY . .

RUN cargo install --path .

FROM debian:buster-slim

RUN mkdir -p /app

COPY --from=builder /usr/src/server/target/release/w /app/server

COPY ./dist /app/dist
COPY ./certs /app/certs

WORKDIR /app

CMD ["/app/server"]