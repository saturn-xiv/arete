# ARETE

A web application by Rust and React.

## Development

### Install dependencies

-   For Mac

```bash
$ brew install git redis postgresql rabbitmq
$ brew services start redis
$ brew services start postgresql
$ brew services start rabbitmq
```

-   For Ubuntu

```bash
$ sudo apt-get install git build-essential redis rabbitmq-server postgresql libpq-dev
```

-   [Install nodejs](doc/NODEJS.md)
-   [Install rust](doc/RUST.md)
-   [Create database](doc/POSTGRESQL.md)
-   [Create RabbitMQ virtual host](doc/RABBITMQ.md)

```bash
$ git clone https://github.com/saturn-xiv/arete.git
$ cd arete
$ make npm
$ make check
$ ./target/debug/arete generate:config # please fix config.toml then
$ cargo run # http://localhost:8080
$ cd dashboard && npm start # http://localhost:3000/my/
```

## Deployment

### Build by docker

```bash
$ git clone https://github.com/saturn-xiv/arete.git
$ cd arete
$ docker build -t arete .
$ docker run --rm -it -v `pwd`:/workspace arete
> make clean
> make npm
> make # dist.tar.xz
```

## Documents

-   [Office UI Fabric React](https://github.com/OfficeDev/office-ui-fabric-react)
-   [Diesel: A safe, extensible ORM and Query Builder for Rust](https://github.com/diesel-rs/diesel)
-   [Actix: Rust's powerful actor system and most fun web framework](https://github.com/actix/actix-web)
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
