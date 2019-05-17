#!/bin/sh

echo "`date`: login $username" >> /var/log/openvpn/session
curl -v -H "Authorization: Bearer {{token}}" -X POST -d "{\"username\": \"${username}\", \"password\": \"${password}\"}" https://{{host}}/api/ops/vpn/users/sign-in
exit 0
