## Build and test

```bash
$ docker build -t arete .
$ docker run --rm -it arete /bin/bash
$ docker run --rm -it -p 2222:22 arete
$ docker tag arete chonglou/arete
$ docker push chonglou/arete:latest
```