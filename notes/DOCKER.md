## DOCKER

-   [Document](https://docs.docker.com/install/)

-   Install(for archlinux)

```bash
sudo pacman -S docker
sudo systemctl enable docker
sudo gpasswd -a who-am-i docker
```

- Install for disco, add `edge test` after stable in sources.list's docker line.

- Clean, [see](https://docs.docker.com/config/pruning/)


```bash
docker system prune --volumes # clear, DON RUN IT ON SERVER
docker image prune -a # remove all dangling images
docker ps
```

- Build & push

```bash
docker build -t NAME .
docker build --no-cache -t NAME . # without cache
docker tag NAME REMOTE
docker push REMOTE
```
