#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""Setup.

install third packages:
$ sudo apt-get install build-essential libsystemd-dev python3-dev pkg-config
$ pip3 install --user toml pysnmp psycopg2 systemd

/etc/snmp/snmpd.conf:
agentAddress udp:161,udp6:[::1]:161
view systemonly included .1.3.6

"""


import os
import argparse
import logging
import threading
import time
import uuid
import fcntl
import tempfile

import toml
import pysnmp.hlapi
import psycopg2
from systemd.journal import JournalHandler

logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s',
                    level=logging.DEBUG)
logger = logging.getLogger(os.path.basename(__file__))
logger.addHandler(JournalHandler())


class Crawler(threading.Thread):
    def __init__(self, db, host, delay, ok):
        super().__init__()
        self.host = host
        self.db = db
        self.delay = delay
        self.ok = ok

    def run(self):
        logger.info("start thread %s" % self.host)
        while self.ok.is_set():
            cur = self.db.cursor()
            self.__walk(cur)
            self.db.commit()
            cur.close()

            time.sleep(self.delay)

    def __walk(self, cur):
        logger.debug("walk %s", self.host)
        uid = str(uuid.uuid4())
        for errorIndication, errorStatus, errorIndex, varBinds in pysnmp.hlapi.nextCmd(pysnmp.hlapi.SnmpEngine(),
                                                                                       pysnmp.hlapi.CommunityData(
            'public', mpModel=0),
            pysnmp.hlapi.UdpTransportTarget((host, 161)),
            pysnmp.hlapi.ContextData(),
            pysnmp.hlapi.ObjectType(pysnmp.hlapi.ObjectIdentity(".1.3"))
        ):

            if errorIndication:
                logger.error(errorIndication)
            elif errorStatus:
                logger.error('%s at %s' % (errorStatus.prettyPrint(),
                                           errorIndex and varBinds[int(errorIndex) - 1][0] or '?'))
            else:
                for key, val in varBinds:
                    # logger.debug("%s = %s" %
                    #               (key.prettyPrint(), val.prettyPrint()))
                    cur.execute("INSERT INTO monitor_logs (name, uid, code, value) VALUES (%s, %s, %s, %s)",
                                (self.host, uid, key.prettyPrint(), val.prettyPrint()))


if __name__ == '__main__':

    parser = argparse.ArgumentParser(description='Snmp agent.')
    parser.add_argument('-c', '--config', required=True,
                        type=str, help='config file')

    args = parser.parse_args()

    lock = open(os.path.join(tempfile.gettempdir(), ".snmp.lck"), "wb")
    fcntl.flock(lock.fileno(), fcntl.LOCK_EX)

    logger.info("load config from %s" % args.config)
    cfg = toml.load(args.config)

    ok = threading.Event()
    ok.set()

    for host in cfg['hosts']:
        it = Crawler(
            psycopg2.connect(cfg['postgresql']),
            host,
            cfg['sleep'],
            ok
        )
        it.daemon = True
        it.start()

    try:
        while True:
            time.sleep(.1)
    except (KeyboardInterrupt, SystemExit):
        logger.warning("exit...")
        ok.clear()

    fcntl.flock(lock.fileno(), fcntl.LOCK_UN)
