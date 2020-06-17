#!/bin/sh

echo "`date`: disconnect $common_name, $bytes_received, $bytes_sent, $trusted_ip, $trusted_port" >> /var/log/openvpn/session
curl -v -H "Authorization: Bearer {{token}}" -d "{\"email\": \"${common_name}\", \"trustedIp\": \"${trusted_ip}\", \"trustedPort\": ${trusted_port}, \"received\": ${bytes_received}, \"send\": ${bytes_sent}}" https://{{host}}/api/ops/vpn/users/disconnect
exit 0
