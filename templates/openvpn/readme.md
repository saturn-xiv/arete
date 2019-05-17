
## Server

* Install 
  
```bash
$ apt install -y openvpn easy-rsa dnsmasq nmap curl
$ mkdir -p /etc/openvpn/easy-rsa
$ cp -r /usr/share/easy-rsa/* /etc/openvpn/easy-rsa
$ cd /etc/openvpn/easy-rsa
$ cat <<EOF >> vars
export KEY_COUNTRY="US"
export KEY_PROVINCE="CA"
export KEY_CITY="Goleta"
export KEY_ORG="Honor"
export KEY_EMAIL="no-reply@gmail.com"
export KEY_CN="www.change-me.com"
export KEY_NAME="who-am-i"
export KEY_OU="ops"
export KEY_ALTNAMES="EasyRSA"
EOF
$ openssl version
$ openvpn --version
$ ln -s openssl-1.0.0.cnf openssl.cnf
```

* Generate server certs

```bash
$ source vars
$ ./clean-all
$ ./build-ca
$ ./build-key-server server
$ ./build-dh
```

## Client

* Generate client certs

```bash
$ source vars
$ ./build-key client
```

* Revoke user

```bash
$ source var
$ ./revoke-full client
```