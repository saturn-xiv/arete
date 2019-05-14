#!/bin/bash
. /etc/openvpn/script/config.sh
##insert data connection to table log
mysql -h$HOST -P$PORT -u$USER -p$PASS $DB -e "INSERT INTO log (log_id,user_id,log_trusted_ip,log_trusted_port,log_remote_ip,log_remote_port,log_start_time,log_end_time,log_received,log_send) VALUES(NULL,'$common_name','$trusted_ip','$trusted_port','$ifconfig_pool_remote_ip','$remote_port_1',now(),'0000-00-00 00:00:00','$bytes_received','$bytes_sent')"
##set status online to user connected
mysql -h$HOST -P$PORT -u$USER -p$PASS $DB -e "UPDATE user SET user_online=1 WHERE user_id='$common_name'"