#!/bin/sh

echo "`date`: connect $common_name, $trusted_ip, $trusted_port, $ifconfig_pool_remote_ip, $remote_port_1, $bytes_received, $bytes_sent" >> /var/log/openvpn/session
curl -v -H "Authorization: Bearer {{token}}" -d "{\"email\": \"${common_name}\", \"trustedIp\": \"${trusted_ip}\", \"trustedPort\": ${trusted_port}, \"remoteIp\": \"${ifconfig_pool_remote_ip}\", \"remotePort\": ${remote_port_1}}" https://{{host}}/api/ops/vpn/users/connect
exit 0

