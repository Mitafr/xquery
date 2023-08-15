FROM rust:1.71.1-slim-buster AS backbuilder

WORKDIR /usr/src/server
COPY ./back .

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo install --path .

FROM node:lts-alpine AS frontbuilder

WORKDIR /usr/src/app

COPY ./front/package.json ./front/yarn.lock ./

RUN yarn install --frozen-lockfile

COPY ./front .

RUN chmod u+x ./build.sh
RUN ./build.sh

FROM debian:bookworm-slim

RUN mkdir -p /app

COPY --from=backbuilder /usr/src/server/target/release/w /app/server

COPY --from=frontbuilder /usr/src/app/dist /app/dist
COPY --from=backbuilder /usr/src/server/certs /app/certs

WORKDIR /app

CMD ["/app/server"]