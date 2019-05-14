#!/bin/sh

#!/bin/bash
. /etc/openvpn/script/config.sh
##Authentication
user_id=$(mysql -h$HOST -P$PORT -u$USER -p$PASS $DB -sN -e "select user_id from user where user_id = '$username' AND user_pass = '$password' AND user_enable=1 AND user_start_date != user_end_date AND TO_DAYS(now()) >= TO_DAYS(user_start_date) AND (TO_DAYS(now()) <= TO_DAYS(user_end_date) OR user_end_date='0000-00-00')")
##Check user
[ "$user_id" != '' ] && [ "$user_id" = "$username" ] && echo "user : $username" && echo 'authentication ok.' && exit 0 || echo 'authentication failed.'; exit 1

PASSFILE="/etc/openvpn/users"

if [ ! -r "${PASSFILE}" ]; then
  echo "${TIME_STAMP}: Could not open password file \"${PASSFILE}\" for reading." >> ${LOG_FILE}
  exit 1
fi

CORRECT_PASSWORD=`awk '!/^;/&&!/^#/&&$1=="'${username}'"{print $2;exit}' ${PASSFILE}`

if [ "${CORRECT_PASSWORD}" = "" ]; then 
  echo "${TIME_STAMP}: User does not exist: username=\"${username}\", password=\"${password}\"." >> ${LOG_FILE}
  exit 1
fi

if [ "${password}" = "${CORRECT_PASSWORD}" ]; then 
  echo "${TIME_STAMP}: Successful authentication: username=\"${username}\"." >> ${LOG_FILE}
  exit 0
fi

echo "${TIME_STAMP}: Incorrect password: username=\"${username}\", password=\"${password}\"." >> ${LOG_FILE}
exit 1