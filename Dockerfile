FROM alpine:latest
MAINTAINER Jeremy Zheng

RUN apk update
RUN apk add git curl vim zsh pwgen build-base \
    # nodejs-current npm yarn \
    # cargo rust \
    openssl-dev

# https://github.com/ohmyzsh/ohmyzsh
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# https://www.rust-lang.org/tools/install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# https://github.com/nvm-sh/nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | sh
RUN sh -c ". $HOME/.profile \
    && nvm install node"
RUN sh -c ". $HOME/.nvm/nvm.sh \
    && npm install -g yarn"

VOLUME /workspace
WORKDIR /workspace

ENV RUSTFLAGS="-C target-feature=+crt-static"
CMD ["/bin/zsh", "-l"]
