#!/bin/sh

. /etc/openvpn/script/config.sh

echo "${TIME_STAMP}: disconnect $common_name, $bytes_received, $bytes_sent, $trusted_ip, $trusted_port" >> ${LOG_FILE}

exit 0
