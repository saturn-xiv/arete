# For Ubuntu(Bionic)

## Usage

- Verify that the iptables firewall is not blocking any of the standard mail ports (25, 465, 587, 110, 995, 143, and 993)
- Configure DNSPermalink

```
example.com A 10 12.34.56.78
example.com MX 10 example.com
mail.example.com MX 10 example.com
```

- Change hostname

```
mail.example.com
```

- Update Hosts File

```
127.0.0.1 localhost.localdomain localhost
192.0.2.0 mail.example.com mail
```

- Install dependencies

```bash
$ apt install dovecot-core dovecot-imapd dovecot-pop3d dovecot-lmtpd dovecot-pgsql \
  postfix postfix-pgsql \
  mailutils
```

- Create vmail user

```bash
$ mkdir -p /var/mail/vhosts
$ groupadd -g 5000 vmail
$ useradd -g vmail -u 5000 vmail -d /var/mail
$ chown -R vmail:vmail /var/mail

$ chown -R vmail:dovecot /etc/dovecot
$ chmod -R o-rwx /etc/dovecot
```

## Resources

- https://www.linode.com/docs/email/postfix/email-with-postfix-dovecot-and-mysql/
