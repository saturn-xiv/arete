## Build and test

```bash
$ docker pull alpine:latest
$ docker build -t arete .
$ docker run --rm -it --network host -v `pwd`:/workspace arete
```

## Documents

- [rust-lang-nursery](https://github.com/rust-lang-nursery/docker-rust-nightly/blob/master/alpine3.10/Dockerfile)