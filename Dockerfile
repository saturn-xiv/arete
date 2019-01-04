FROM alpine:edge

RUN echo '2019-01-03' >> /version

RUN apk update
RUN apk upgrade
RUN apk add zsh git curl sudo \
  rust cargo nodejs npm make xz \
  openssl-dev libsodium-dev postgresql-dev

# deploy
RUN adduser -s /bin/zsh -D deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
USER deploy

RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)" || true

VOLUME /workspace
WORKDIR /workspace

CMD ["/bin/zsh", "-l"]
