FROM ubuntu:latest

RUN echo '2018-12-30' >> /version

ENV DEBIAN_FRONTEND noninteractive
ENV TERM linux

# packages
RUN apt-get update
RUN apt-get -y install apt-utils
RUN apt-get -y upgrade
RUN apt-get -y install build-essential pkg-config curl git sudo zsh pwgen \
  libpq-dev
RUN apt-get -y autoremove
RUN apt-get -y clean

# deploy
RUN useradd -s /bin/zsh -m deploy
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
USER deploy

RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)" || true
RUN mkdir $HOME/downloads

RUN curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.11/install.sh | zsh
RUN zsh -c "source $HOME/.zshrc \
  && nvm install node \
  && npm install -g grunt-cli"

# rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.zshrc
# https://github.com/rust-lang/rust/issues/50504#issuecomment-410550021
RUN export RUSTFLAGS="-Aproc-macro-derive-resolution-fallback" 

VOLUME /workspace
WORKDIR /workspace

CMD ["/usr/bin/zsh", "-l"]
