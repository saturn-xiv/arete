#!/bin/sh

echo "`date`: connect $common_name, $trusted_ip, $trusted_port, $ifconfig_pool_remote_ip, $remote_port_1, $bytes_received, $bytes_sent" >> /var/log/openvpn/session
# curl -v -H "Authorization: Bearer {{token}}" -X POST -d "{\"username\": \"${common_name}\", \"trusted_ip\": \"${trusted_ip}\", \"trusted_port\": ${trusted_port}, \"remote_ip\": \"${ifconfig_pool_remote_ip}\", \"remote_port\": ${remote_port_1}}" https://{{host}}/api/ops/vpn/users/connect
exit 0

