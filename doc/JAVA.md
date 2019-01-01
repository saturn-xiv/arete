## Java

```bash
curl -s "https://get.sdkman.io" | zsh
# please re-loging at first
sdk install java
sdk install maven
sdk install gradle
```

### [Inotify Watches Limit](https://confluence.jetbrains.com/display/IDEADEV/Inotify+Watches+Limit)

-   Add the following line to either /etc/sysctl.conf file or a new \*.conf file (e.g. idea.conf) under /etc/sysctl.d/ directory:

```text
fs.inotify.max_user_watches = 524288
```

-   Then run this command to apply the change:

```bash
sudo sysctl -p --system
```
