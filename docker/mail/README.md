## Build and test

```bash
$ docker pull ubuntu:latest
$ docker build -t mail .
$ docker run --rm -it -p 2222:22 -p 8080:80 mail
```

## Run from Docker Hub

```bash
$ docker pull chonglou/mail:latest
$ docker run --rm -it --network host -d -p 2222:22 -v `pwd`:/var/mail chonglou/mail:latest
```

## Usage

```bash
$ ssh -p 2222 deploy@localhost # password is 'hi'
```

## Server settings

1. Verify that the iptables firewall is not blocking any of the standard mail ports (25, 465, 587, 110, 995, 143, and 993)
1. Configure DNSPermalink

```
example.com A 10 12.34.56.78
example.com MX 10 example.com
mail.example.com MX 10 example.com
```

1. Update Hosts File

```
127.0.0.1 localhost.localdomain localhost
192.0.2.0 mail.example.com mail
```

## Resources

- https://www.digitalocean.com/community/tutorials/how-to-set-up-a-postfix-email-server-with-dovecot-dynamic-maildirs-and-lmtp
- https://www.digitalocean.com/community/tutorials/how-to-configure-a-mail-server-using-postfix-dovecot-mysql-and-spamassassin
- https://www.linode.com/docs/email/postfix/email-with-postfix-dovecot-and-mysql/
