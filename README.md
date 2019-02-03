# ARETE

A web application by Rust and React.

## Clone source code

```bash
$ git clone https://github.com/saturn-xiv/arete.git ~/workspace/arete
```

## Start docker

```bash
$ docker pull chonglou/arete:latest 
$ docker create --name arete -p 2222:22 -p 8080:8080 -p 3000:3000 -p 15672:15672 -v $HOME/.ssh:/home/deploy/.ssh -v $HOME/workspace:/workspace chonglou/arete:latest 
$ docker start arete 
```

## Log into docker

```bash
$ ssh -p 2222 deploy@localhost # default password is 'hi'
> cd /workspace/arete
> make npm # install frontend dependencies
> make check # build backend and documents
```

## Development

- Create database

```bash
> psql -U postgres
  CREATE DATABASE arete;
```

- Create rabbitmq virtual host

```bash
> rabbitmqctl add_vhost /arete
```

- Usage
 
```bash
> ./target/debug/arete generate:config # please fix config.toml then
> ./target/debug/arete database:migrate # migrate database
> ./target/debug/arete i18n:sync -d locales # import locale items
> cargo run # http://localhost:8080
> cd dashboard && npm start # http://localhost:3000/my/
```
**Create the admin in http://localhost:3000/my/install** 

## Deployment

### Build by docker

```bash
> make clean
> make npm
> make # dist.tar.xz
```

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
