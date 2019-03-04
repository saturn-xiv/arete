## Build and test

```bash
$ docker pull alpine:latest
$ docker build -t arete .
$ docker run --rm -it -p 2222:22 -v `pwd`:/workspace arete
```