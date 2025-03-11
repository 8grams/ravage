# base image
FROM node:22.14.0-bookworm AS basenode
RUN apt update && apt install curl
RUN corepack enable
RUN corepack prepare pnpm@latest-8 --activate
RUN pnpm config set store-dir .pnpm-store
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:$PATH"

FROM rust:1.85.0-bookworm AS baserust
RUN update-ca-certificates
RUN apt update -y
RUN apt install build-essential -y
RUN apt install lld clang -y

# Node
FROM basenode AS nodebase
WORKDIR /app
COPY ./templates ./templates
COPY ./tailwind.config.js ./tailwind.config.js
COPY ./package.json ./package.json
RUN pnpm install
RUN pnpm build:css

# builder
FROM baserust AS builder
ENV USER=app
ENV UID=10001
RUN adduser --disabled-password --gecos "" --home "/nonexistent" --shell "/sbin/nologin" --no-create-home --uid "${UID}" "${USER}"
WORKDIR /app
COPY . .
COPY --from=nodebase /app/static/* ./static/

ARG BUILD_TIMESTAMP
ENV BUILD_TIMESTAMP=${BUILD_TIMESTAMP}
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release --locked
RUN strip -s target/release/ravage
CMD ["/app/target/release/ravage"]

# final outcome
FROM debian:12.9-slim
RUN apt update && apt install -y nginx supervisor
WORKDIR /app

COPY --from=builder /app/target/release/ravage ./
COPY ./docker/supervisord.conf /etc/supervisord.conf
COPY ./docker/default.conf /etc/nginx/conf.d/default.conf

RUN mkdir -p /var/log/supervisor \
    && mkdir -p /var/run/supervisor \
    && chmod -R 777 /var/run/supervisor \
    && chmod -R 777 /var/log/supervisor

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
