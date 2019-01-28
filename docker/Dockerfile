FROM ubuntu:latest

RUN echo '2019-01-28' >> /version

ENV DEBIAN_FRONTEND noninteractive
ENV UBUNTU_VERSION bionic

# packages
RUN apt-get update
RUN apt-get -y upgrade
RUN apt-get -y install apt-utils apt-transport-https wget gnupg software-properties-common curl

RUN add-apt-repository -y ppa:webupd8team/java
RUN echo "oracle-java8-installer shared/accepted-oracle-license-v1-1 select true" | debconf-set-selections

RUN wget -O - "https://artifacts.elastic.co/GPG-KEY-elasticsearch" | apt-key add -
RUN echo "deb https://artifacts.elastic.co/packages/6.x/apt stable main" > /etc/apt/sources.list.d/elasticsearch.list

# http://www.rabbitmq.com/which-erlang.html
RUN curl -s "https://packagecloud.io/install/repositories/rabbitmq/rabbitmq-server/script.deb.sh" | bash

RUN apt-get -y install build-essential clang cmake pkg-config dh-autoreconf dh-make checkinstall \
  cpio meson intltool libtool gawk texinfo bison bc zsh moreutils tree tzdata \
  libncurses5-dev \
  ansible ansible-lint \
  net-tools lsof iputils-ping dnsutils psmisc inotify-tools logrotate \
  musl musl-dev musl-tools llvm \
  zip unzip bsdtar telnet git vim pwgen sudo gperf \
  libsodium-dev libpq-dev libmysqlclient-dev libsqlite3-dev libudev-dev liboping-dev libzmq3-dev \
  libssl-dev libreadline-dev zlib1g-dev oracle-java8-installer \
  sqlite3 postgresql redis rabbitmq-server openssh-server nginx supervisor
RUN apt-get -y install elasticsearch
RUN apt-get -y autoremove
RUN apt-get -y clean

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

RUN /usr/sbin/rabbitmq-plugins enable rabbitmq_management

# deploy
RUN useradd -s /bin/zsh -m deploy
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
RUN echo "deploy:hi" | chpasswd
RUN mkdir /opt/saturn-xiv
RUN chown deploy:deploy /opt/saturn-xiv
USER deploy

RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)" || true
RUN mkdir $HOME/downloads

ENV ANDROID_SDK_VERSION 4333796
RUN wget -P $HOME/downloads https://dl.google.com/android/repository/sdk-tools-linux-$ANDROID_SDK_VERSION.zip
RUN mkdir -pv $HOME/local/android-sdk
RUN unzip $HOME/downloads/sdk-tools-linux-$ANDROID_SDK_VERSION.zip -d $HOME/local/android-sdk
RUN yes | $HOME/local/android-sdk/tools/bin/sdkmanager --licenses

RUN curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.34.0/install.sh | zsh
RUN zsh -c "source $HOME/.zshrc \
  && nvm install node"

RUN curl -s "https://get.sdkman.io" | zsh
RUN sed -i -e 's/sdkman_auto_answer=false/sdkman_auto_answer=true/g' $HOME/.sdkman/etc/config
RUN zsh -c "source $HOME/.zshrc \
  && sdk install java 8.0.201-oracle \
  && sdk install maven \
  && sdk install gradle"

# rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.zshrc
# https://mexus.github.io/rustup-components-history/
RUN zsh -c "source $HOME/.zshrc \
  && rustup default nightly-2019-01-26 \
  && rustup component add rls rust-analysis rust-src rustfmt clippy"

# saturn-xiv
RUN zsh -c "source $HOME/.zshrc \
  && git clone https://github.com/saturn-xiv/arete.git /opt/saturn-xiv/arete \
  && cargo install diesel_cli"

# setup
USER root

RUN mkdir /var/run/sshd
RUN echo "AllowUsers deploy" >> /etc/ssh/sshd_config

RUN mkdir -p /var/run/postgresql/10-main.pg_stat_tmp
RUN chown -R postgres:postgres /var/run/postgresql
RUN echo "local   all             all                                     trust" > /etc/postgresql/10/main/pg_hba.conf
RUN echo "host    all             all             127.0.0.1/32            trust" >> /etc/postgresql/10/main/pg_hba.conf
RUN echo "host    all             all             ::1/128                 trust" >> /etc/postgresql/10/main/pg_hba.conf
RUN echo "log_statement = 'all'" >> /etc/postgresql/10/main/postgresql.conf

COPY supervisord.conf /etc/supervisord.conf

EXPOSE 22/tcp 80/tcp 15672/tcp 3000/tcp 8080/tcp 

VOLUME /workspace /home/deploy/.ssh

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
