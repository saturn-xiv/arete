
## Server

* Install 
  
```bash
$ apt install -y openvpn easy-rsa dnsmasq
$ mkdir -p /etc/openvpn/easy-rsa
$ cp -r /usr/share/easy-rsa/* /etc/openvpn/easy-rsa
$ cd /etc/openvpn/easy-rsa
$ cat <<EOF > var
KEY_COUNTRY="US"
KEY_PROVINCE="CA"
KEY_CITY="Goleta"
KEY_ORG="hour.com"
KEY_EMAIL="no-reply@gmail.com"
KEY_CN="www.change-me.com"
KEY_NAME="who-am-i"
KEY_OU="ops"

export KEY_COUNTRY KEY_PROVINCE KEY_CITY KEY_ORG KEY_EMAIL KEY_CN KEY_NAME KEY_OU
EOF
```

* Generate server certs

```bash
$ source vars
$ ./clean-all
$ ./build-ca
$ ./build-key-server server
$ ./build-dh
```

* Enable port forwarding

```bash
$ echo 'net.ipv4.ip_forward=1' > /etc/sysctl.d/ip_forward.conf
$ sysctl -p
```

* Revoke user

```bash
$ source var
$ ./revoke-full client
```

## Client

* Generate client certs

```bash
$ source vars
$ ./build-key client
$ ls keys/tpl.ovpn
```