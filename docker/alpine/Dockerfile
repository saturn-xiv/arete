FROM alpine:latest

RUN apk update
RUN apk add sudo bash pwgen \
  git pkgconf openssl-dev \
  python perl make cmake curl \
  ca-certificates build-base \
  musl-dev zeromq-dev postgresql-dev \
  nodejs npm 

# deploy
RUN adduser -D -s /bin/bash deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
RUN mkdir /opt/saturn-xiv
RUN chown deploy:deploy /opt/saturn-xiv
USER deploy

RUN wget https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-musl/rustup-init -O $HOME/rustup-init
RUN chmod +x $HOME/rustup-init
RUN bash -c "$HOME/rustup-init -y --default-toolchain nightly"

RUN git clone https://github.com/saturn-xiv/arete.git /opt/saturn-xiv/arete  

WORKDIR /workspace 
VOLUME /workspace

CMD ["/bin/bash", "-l"]
