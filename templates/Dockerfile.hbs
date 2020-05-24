# Build Stage
FROM rustlang/rust:nightly-alpine as builder
LABEL maintainer="Gary Ascuy <gary.ascuy@gmail.com>"

WORKDIR /code
RUN apk update \
    && apk add build-base openssl-dev zlib-dev \
    && rm -rf /var/cache/apk/*
COPY . .
RUN cargo build --release

# Image Stage
FROM alpine:latest
LABEL maintainer="Gary Ascuy <gary.ascuy@gmail.com>"

ENV ROCKET_ENV=production \
    ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=3666 \
    ROCKET_LOG=critical
EXPOSE 3666

COPY --from=builder /code/target/release/server /usr/local/bin/server
CMD server
