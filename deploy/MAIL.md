## Server settings

1. Verify that the iptables firewall is not blocking any of the standard mail ports (25, 465, 587, 110, 995, 143, and 993)
1. Configure DNSPermalink

```
example.com A 10 12.34.56.78
example.com MX 10 example.com
mail.example.com MX 10 example.com
```

## Resources

- https://www.digitalocean.com/community/tutorials/how-to-set-up-a-postfix-email-server-with-dovecot-dynamic-maildirs-and-lmtp
- https://www.digitalocean.com/community/tutorials/how-to-configure-a-mail-server-using-postfix-dovecot-mysql-and-spamassassin
- https://www.linode.com/docs/email/postfix/email-with-postfix-dovecot-and-mysql/
- [postfixadmin](https://github.com/postfixadmin/postfixadmin)
- [SquirrelMail](http://squirrelmail.org/download.php)
