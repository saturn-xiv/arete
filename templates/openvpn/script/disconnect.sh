#!/bin/sh

echo "`date`: disconnect $common_name, $bytes_received, $bytes_sent, $trusted_ip, $trusted_port" >> /var/log/openvpn/session
# curl -v -H "Authorization: Bearer {{token}}" -X POST -d "{\"username\": \"${common_name}\", \"trusted_ip\": \"${trusted_ip}\", \"trusted_port\": ${trusted_port}, \"received\": ${bytes_received}, \"send\": ${bytes_sent}}" https://{{host}}/api/ops/vpn/users/disconnect
exit 0
