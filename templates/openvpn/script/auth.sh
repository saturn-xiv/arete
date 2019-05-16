#!/bin/sh

. /etc/openvpn/script/config.sh

echo "${TIME_STAMP}: auth user ${username} ${password}" >> ${LOG_FILE}

exit 0
