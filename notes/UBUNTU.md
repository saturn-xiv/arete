- Check block device

```bash
$ blkid
$ lsblk
```

- Login without password

```bash
$ ssh-copy-id -i ~/.ssh/id_rsa deploy@xxx.xxx.xxx.xxx
```

- Sudo without password

```bash
$ echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
```

- Change default editor

```bash
$ sudo update-alternatives --config editor
```

- swap file

```bash
$ dd if=/dev/zero of=/mnt/swap.fs bs=1M count=2048
$ chmod 600 /mnt/swap.fs
$ mkswap /mnt/swap.fs
$ swapon /mnt/swap.fs
$ echo '/mnt/swap.fs 		none 			swap 	defaults 	0 0' >> /etc/fstab
```
