FROM rust:1.71.1-slim-buster AS backbuilder

WORKDIR /usr/src/server
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo install --path .

FROM node:19-buster AS frontbuilder

WORKDIR /usr/src/app

COPY ../front/package.json ../front/yarn.lock ./

RUN yarn install --frozen-lockfile

COPY ../front .

RUN build.sh

FROM debian:buster-slim

RUN mkdir -p /app

COPY --from=backbuilder /usr/src/server/target/release/w /app/server

COPY --from=frontbuilder /usr/src/app/dist /app/dist
COPY ./certs /app/certs

WORKDIR /app

CMD ["/app/server"]