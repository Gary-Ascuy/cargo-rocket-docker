# Build Stage
FROM rustlang/rust:nightly-alpine as builder
LABEL maintainer="fromfile<file@from.com>"
LABEL version="1.6.6"

WORKDIR /code
RUN apk update \
    && apk add build-base openssl-dev zlib-dev imagemagick \
    && rm -rf /var/cache/apk/*
COPY . .
RUN cargo build --release

# Image Stage
FROM alpine:latest
LABEL maintainer="fromfile<file@from.com>"
LABEL version="1.6.6"

ENV ROCKET_ENV=production \
    ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=3666 \
    ROCKET_LOG=critical

RUN apk update \
    && apk add gary \
    && rm -rf /var/cache/apk/*

EXPOSE 3666

COPY --from=builder /code/target/release/server /usr/local/bin/server
CMD server
