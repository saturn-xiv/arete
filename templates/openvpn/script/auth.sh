#!/bin/sh

echo "`date`: login $username" >> /var/log/openvpn/session
curl -v -H "Authorization: Bearer {{token}}" -d "{\"email\": \"${username}\", \"password\": \"${password}\"}" https://{{host}}/api/ops/vpn/users/sign-in
exit 0
