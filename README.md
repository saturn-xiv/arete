# ARETE

A web application by Rust and React.

## Usage

- Create deploy user on server

```bash
$ useradd -s /bin/bash -m deploy
$ echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
```

- Setup non-password login to servers

```bash
$ ssh-keygen -t rsa -b 4096 -C "your_email@example.com" # generate ssh key
$ ssh-copy-id deploy@xxx.xxx.xxx.xxx # upload public ssh key to server
```

- Install ansible

```bash
$ pip install --user ansible
$ echo 'export $PATH=$HOME.local/bin' >> ~/.zshrc # then re-login
```

- Build on localhost & upload to servers

```bash
$ git clone https://github.com/saturn-xiv/arete.git
$ cd arete/deploy
$ openssl rand -base64 32 # generate random secrets key
# FIX settings in inventories/production
$ ansible production build
$ ansible production deploy
```

Create the administrator in **http://xxx.xxx.xxx.xxx/my/install**

## Development

### Generate logo

```bash
$ convert -background transparent -resize 512x512 from.svg to.png
$ convert -background transparent -resize 192x192 from.svg to.png
$ # favicon.ico: 16x16 32x32 64x64
```

### Vscode

- [ESLint](https://github.com/Microsoft/vscode-eslint)
- [Rust(rls)](https://github.com/rust-lang/rls-vscode)
- [Icons](https://github.com/vscode-icons/vscode-icons)
- [OneDark Pro](https://github.com/Binaryify/OneDark-Pro)
- [Ansible](https://github.com/VSChina/vscode-ansible)
- [Better Toml](https://github.com/bungcip/better-toml)
- [Prettier - Code formatter](https://github.com/prettier/prettier-vscode)
- [SQL Formatter](https://github.com/kufii/vscode-sql-formatter)

### Mock test

- [MinIO is a high performance object storage server compatible with Amazon S3 APIs](https://github.com/minio/minio)
- [Message queueing system with an actor-based Scala and Amazon SQS-compatible interfaces. Runs stand-alone or embedded.](https://github.com/softwaremill/elasticmq)

## Documents

- [Create React App](https://create-react-app.dev/docs/adding-typescript)
- [Fabric React](https://developer.microsoft.com/en-us/fabric#/get-started)
- [Ionic: Build amazing Native and Progressive Web Apps with web technologies.](https://ionicframework.com/docs/components)
- [Pluggable enterprise-level react application framework](https://umijs.org/)
- [Ant Desigh Pro](https://pro.ant.design/docs/getting-started)
- [Validate.js](https://validatejs.org/#validators)
- [Third-Party Libraries](https://ant.design/docs/react/recommendation)
- [A Fast and Flexible Static Site Generator](https://github.com/gohugoio/hugo)
- [Ansible Documentation](https://docs.ansible.com/ansible/latest/user_guide/playbooks_best_practices.html)
- [Docker](https://docs.docker.com/install/linux/docker-ce/ubuntu/)
- [Diesel: A safe, extensible ORM and Query Builder for Rust](https://github.com/diesel-rs/diesel)
- [Rocket: A web framework for Rust](https://rocket.rs/)
- [favicon.ico](http://icoconvert.com/)
- [smver](http://semver.org/)
- [keep a changelog](https://keepachangelog.com/en/1.0.0/)
- [banner.txt](http://patorjk.com/software/taag/)
- [jwt](https://jwt.io/)
- [GraphQL](https://graphql.org/learn/)
- [Alibaba Java Coding Guidelines](https://github.com/alibaba/p3c)
- [An emoji guide for your commit messages](https://gitmoji.carloscuesta.me/)
- [Let’s Encrypt](https://letsencrypt.org/)
- [Certbot](https://certbot.eff.org/)
- [SSL Server Test](https://www.ssllabs.com/ssltest/index.html)
- [LINE Developers](https://developers.line.me/en/)
- [UTF-8 Miscellaneous Symbols](https://www.w3schools.com/charsets/ref_utf_misc_symbols.asp)
- [msmtp](https://wiki.archlinux.org/index.php/msmtp)
- [For gmail smtp](http://stackoverflow.com/questions/20337040/gmail-smtp-debug-error-please-log-in-via-your-web-browser)
- [W3C Feed Validation Service](https://validator.w3.org/feed/)
- [XML Sitemap Validator](https://www.xml-sitemaps.com/validate-xml-sitemap.html)
- [robots.txt Tester](https://support.google.com/webmasters/answer/6062598?hl=en)
- [Evernote robust API.](https://dev.evernote.com/)
- [Actix web is a small, pragmatic, and extremely fast rust web framework](https://github.com/actix/actix-web)
- [Juniper](https://graphql-rust.github.io/juniper/current/)
- [1 million+ Stunning Free Images to Use Anywhere](https://pixabay.com/)
- [famfamfam icons](http://www.famfamfam.com/lab/icons/)
- [Python Youtube V3 Quickstart](https://developers.google.com/youtube/v3/quickstart/python)
- [Code samples for YouTube APIs](https://github.com/youtube/api-samples)
- [Youtube data api](https://developers.google.com/youtube/v3/docs)
- [The official Python client library for Google's discovery based APIs.](https://github.com/googleapis/google-api-python-client)
