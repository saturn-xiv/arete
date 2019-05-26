# ARETE

A web application by Rust and Vue.js.

## Build

Please install [docker](https://docs.docker.com/install/linux/docker-ce/ubuntu/) at first.

```bash
$ git clone https://github.com/saturn-xiv/arete.git ~/workspace/arete # clone source code
$ cd arete
$ docker run --name arete -d -p 2222:22 -p 3000:3000 -v $HOME/.ssh:/home/deploy/.ssh -v `pwd`:/workspace chonglou/arete:latest # ONLY for start docker container first time
$ docker start arete # start docker container NEXT time
$ ssh -p 2222 deploy@localhost # default password is 'hi'
> make deb # build for debian/ubuntu package
> dpkg -c target/debian/arete_0.1.0_amd64.deb
```

## How to use

* Generate a 256-bit base64 encoded string by `openssl rand -base64 32`
* Generate **config.toml** `arete generate:config`, then change it if you need
* Migrate database `arete database:migrate`
* Import locale records `arete i18n:sync`
* Generate nginx config file `arete generate:nginx`, then mv it into **/etc/nginx/sites-enabled/** and restart nginx
* Generate systemd config file `arete generate:systemd`, then mv it into **/lib/systemd/system/** and start it
* Create the admin in **http://YOUR-HOST/my/install** 

## Documents

-   [Ant Design](https://ant.design/docs/react/introduce)
-   [Ant Design Pro](https://pro.ant.design/components/AvatarList)
-   [Font Awesome](https://fontawesome.com/how-to-use/on-the-web/setup/hosting-font-awesome-yourself)
-   [Diesel: A safe, extensible ORM and Query Builder for Rust](https://github.com/diesel-rs/diesel)
-   [Rocket: A web framework for Rust](https://rocket.rs/)
-   [favicon.ico](http://icoconvert.com/)
-   [smver](http://semver.org/)
-   [keep a changelog](https://keepachangelog.com/en/1.0.0/)
-   [banner.txt](http://patorjk.com/software/taag/)
-   [jwt](https://jwt.io/)
-   [GraphQL](https://graphql.org/learn/)
-   [Alibaba Java Coding Guidelines](https://github.com/alibaba/p3c)
-   [An emoji guide for your commit messages](https://gitmoji.carloscuesta.me/)
-   [Let’s Encrypt](https://letsencrypt.org/)
-   [Certbot](https://certbot.eff.org/)
-   [SSL Server Test](https://www.ssllabs.com/ssltest/index.html)
-   [LINE Developers](https://developers.line.me/en/)
-   [UTF-8 Miscellaneous Symbols](https://www.w3schools.com/charsets/ref_utf_misc_symbols.asp)
-   [msmtp](https://wiki.archlinux.org/index.php/msmtp)
-   [For gmail smtp](http://stackoverflow.com/questions/20337040/gmail-smtp-debug-error-please-log-in-via-your-web-browser)
-   [W3C Feed Validation Service](https://validator.w3.org/feed/)
-   [XML Sitemap Validator](https://www.xml-sitemaps.com/validate-xml-sitemap.html)
-   [robots.txt Tester](https://support.google.com/webmasters/answer/6062598?hl=en)
