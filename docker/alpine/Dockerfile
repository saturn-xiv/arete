FROM alpine:latest

ENV HOSTNAME armbian

RUN apk update
RUN apk add sudo zsh pwgen vim screen \
  git pkgconf openssl-dev \
  python3 perl \
  make cmake curl \
  ca-certificates build-base \
  musl-dev zeromq-dev postgresql-dev \
  nodejs npm 

# deploy
RUN adduser -D -s /bin/zsh deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
RUN mkdir /opt/saturn-xiv
RUN chown deploy:deploy /opt/saturn-xiv
USER deploy

# oh-my-zsh
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)" || true
RUN echo 'export TERM=screen-256color' >> ~/.zshrc

# rust
# https://rust-lang.github.io/rustup-components-history/index.html
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.zshrc
RUN zsh -c "source $HOME/.zshrc \  
  && rustup component add rls rust-analysis rust-src"

RUN git clone https://github.com/saturn-xiv/arete.git /opt/saturn-xiv/arete  

WORKDIR /workspace 
VOLUME /workspace

CMD ["/bin/zsh", "-l"]
