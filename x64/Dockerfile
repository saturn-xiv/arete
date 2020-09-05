FROM alpine:latest

RUN apk update
RUN apk upgrade
RUN apk add clang cmake build-base boost \
  hiredis libpq zeromq rabbitmq-c sqlite liboping libsodium

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/sh"]
