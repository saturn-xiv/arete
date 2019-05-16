#!/bin/sh


. /etc/openvpn/script/config.sh

echo "${TIMESTAMP}: connect $common_name, $trusted_ip, $trusted_port, $ifconfig_pool_remote_ip, $remote_port_1, $bytes_received, $bytes_sent" >> ${LOG_FILE}

exit 0

