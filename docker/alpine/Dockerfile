FROM alpine:latest

RUN apk update
RUN apk add sudo \
  git rust cargo pkgconf openssl-dev \
  python make cmake curl \
  nodejs npm 

# deploy
RUN adduser -D deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
RUN mkdir /opt/saturn-xiv
RUN chown deploy:deploy /opt/saturn-xiv
USER deploy

# arete
RUN git clone https://github.com/saturn-xiv/arete.git /opt/saturn-xiv/arete  
RUN cargo install cargo-deb
RUN echo 'export PATH=$HOME/.cargo/bin:$PATH' >> $HOME/.profile

WORKDIR /workspace 
VOLUME /workspace

CMD ["/bin/sh"]
