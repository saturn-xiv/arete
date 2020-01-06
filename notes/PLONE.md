## Install 5.2

```bash
$ sudo apt-get install zlib1g-dev libxml2-dev libxslt-dev libexpat-dev libjpeg-dev libssl-dev
$ ./install.sh --build-python=3 standalone
$ cd ~/Plone/zinstance
$ bin/plonectl start
$ more ~/Plone/zinstance/adminPassword.txt
$ bin/plonectl stop
```

- [nginx](https://docs.plone.org/manage/deploying/front-end/nginx.html)
