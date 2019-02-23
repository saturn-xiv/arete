## DOCKER

-   [Document](https://docs.docker.com/install/)

-   Install(for archlinux)

```bash
sudo pacman -S docker
sudo systemctl enable docker
sudo gpasswd -a who-am-i docker
```

```bash
docker system prune # clear, DON RUN IT ON SERVER
docker image prune # remove all dangling images
docker ps
```
