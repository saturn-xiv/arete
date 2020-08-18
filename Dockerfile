FROM alpine:latest
MAINTAINER Jeremy Zheng

RUN apk update
RUN apk add git curl vim bash pwgen build-base \
    rust cargo \
    nodejs npm yarn \
    openssl-dev

VOLUME /workspace
WORKDIR /workspace

ENV RUSTFLAGS="-C target-feature=+crt-static"
CMD ["/bin/bash", "-l"]
