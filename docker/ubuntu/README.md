## Build and test

```bash
$ docker pull ubuntu:latest
$ docker build -t arete .
$ docker run --rm -it -p 2222:22 -v `pwd`:/workspace arete # just test
```

## Run from Docker Hub

```bash
$ docker rm arete # remove exists container if need
$ docker run --name arete -d -p 2222:22 -v `pwd`:/workspace chonglou/arete:latest # first time to run
$ docker start arete # next time
```