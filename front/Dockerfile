FROM node:19-buster AS builder

WORKDIR /usr/src/app

COPY package.json yarn.lock ./

RUN yarn install --frozen-lockfile

COPY . .

RUN yarn build

FROM nginx

COPY --from=builder /usr/src/app/dist /usr/share/nginx/html