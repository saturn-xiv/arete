FROM ubuntu:latest

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y upgrade
RUN apt -y install apt-utils apt-transport-https wget gnupg software-properties-common \ 
  curl build-essential cmake pkg-config dh-autoreconf dh-make checkinstall \
  cpio meson intltool libtool gawk texinfo bison bc zsh moreutils tree tzdata locales \  
  net-tools lsof iputils-ping dnsutils psmisc inotify-tools logrotate \
  llvm clang \
  zip unzip bsdtar telnet git vim pwgen sudo gperf \
  dovecot-core dovecot-imapd dovecot-pop3d dovecot-lmtpd dovecot-pgsql \
  postfix postfix-pgsql \
  mailutils \
  php-fpm php-mbstring php-gettext php-xml php-recode php-pear php-mysql php-pgsql php-ldap \
  aspell aspell-en \
  openssh-server nginx supervisor

RUN apt -y autoremove
RUN apt -y clean

RUN echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
RUN locale-gen
RUN update-locale LANG=en_US.UTF-8
RUN update-alternatives --set editor /usr/bin/vim.basic

# deploy
RUN useradd -s /bin/zsh -m deploy
RUN passwd -l deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
RUN echo "deploy:hi" | chpasswd
USER deploy

# oh-my-zsh
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)" || true

USER root

RUN mkdir -p /opt/downloads

# https://github.com/postfixadmin/postfixadmin/blob/master/INSTALL.TXT
ENV POSTFIXADMIN 3.2.4
RUN wget -O /opt/downloads/postfixadmin-$POSTFIXADMIN.tar.gz https://github.com/postfixadmin/postfixadmin/archive/postfixadmin-$POSTFIXADMIN.tar.gz

# http://squirrelmail.org/download.php
ENV SQUIRREL_MAIL_WEBMAIL 1.4.22
ENV SQUIRREL_MAIL_LOCALE 1.4.18-20090526
RUN wget -O /opt/downloads/squirrelmail-webmail-$SQUIRREL_MAIL_WEBMAIL.tar.bz2 http://squirrelmail.org/countdl.php?fileurl=http%3A%2F%2Fprdownloads.sourceforge.net%2Fsquirrelmail%2Fsquirrelmail-webmail-$SQUIRREL_MAIL_WEBMAIL.tar.bz2
RUN wget -O /opt/downloads/squirrelmail-all_locales-$SQUIRREL_MAIL_LOCALE.tar.bz2 http://squirrelmail.org/countdl.php?fileurl=http%3A%2F%2Fprdownloads.sourceforge.net%2Fsquirrelmail%2Fall_locales-$SQUIRREL_MAIL_LOCALE.tar.bz2

RUN tar xf /opt/downloads/postfixadmin-$POSTFIXADMIN.tar.gz -C /var/www/html/
RUN mv /var/www/html/postfixadmin-postfixadmin-$POSTFIXADMIN /var/www/html/admin
RUN mkdir -p /var/www/html/admin/templates_c
RUN chown -R www-data:www-data /var/www/html/admin/templates_c

RUN tar xf /opt/downloads/squirrelmail-webmail-$SQUIRREL_MAIL_WEBMAIL.tar.bz2 -C /var/www/html/
RUN tar xf /opt/downloads/squirrelmail-all_locales-$SQUIRREL_MAIL_LOCALE.tar.bz2 -C /var/www/html/squirrelmail-webmail-$SQUIRREL_MAIL_WEBMAIL
RUN mv /var/www/html/squirrelmail-webmail-$SQUIRREL_MAIL_WEBMAIL /var/www/html/squirrel
RUN mkdir -p /var/www/html/squirrel/attach
RUN chown -R www-data:www-data /var/www/html/squirrel/attach
RUN chmod 730 /var/www/html/squirrel/attach

RUN echo "<?php phpinfo(); ?>" > /var/www/html/info.php
RUN echo $'<html> \n\
<?php \n\
header("Location: /squirrel/"); \n\
exit; \n\
?>' > /var/www/html/index.php

RUN mkdir /var/run/sshd
RUN echo "AllowUsers deploy" >> /etc/ssh/sshd_config

COPY nginx.conf /etc/nginx/sites-enabled/default
RUN echo "daemon off;" >> /etc/nginx/nginx.conf
RUN mkdir /run/php

RUN mkdir -p /var/mail/vhosts
RUN groupadd -g 5000 vmail
RUN useradd -g vmail -u 5000 vmail -d /var/mail
RUN chown -R vmail:vmail /var/mail

RUN chown -R vmail:dovecot /etc/dovecot
RUN chmod -R o-rwx /etc/dovecot

COPY supervisord.conf /etc/supervisord.conf

EXPOSE 22/tcp 80/tcp 25/tcp 465/tcp 587/tcp 110/tcp 995/tcp 143/tcp 993/tcp

VOLUME /workspace /home/deploy/.ssh

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
