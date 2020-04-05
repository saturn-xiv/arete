#!/usr/bin/env python
# -*- coding: utf-8 -*-

# pip install toml pysnmp psycopg2

# import os
# import datetime
import argparse
# import sys
# import subprocess
# import urllib.request
# import json
import logging
import threading
import time

import toml
import pysnmp.hlapi
import psycopg2


class Crawler(threading.Thread):
    def __init__(self, db, host, get, walk, delay, ok):
        super().__init__()
        self.host = host
        self.get = get
        self.walk = walk
        self.db = db
        self.delay = delay
        self.ok = ok

    def run(self):
        logging.info("start thread %s" % self.host)
        while self.ok.is_set():
            cur = self.db.cursor()

            for it in self.get:
                self.__get(it, cur)
            for it in self.walk:
                self.__walk(it, cur)

            self.db.commit()
            cur.close()

            time.sleep(self.delay)

    def __get(self, oid, cur):
        logging.debug("get %s from %s", oid, self.host)
        errorIndication, errorStatus, errorIndex, varBinds = next(
            pysnmp.hlapi.getCmd(pysnmp.hlapi.SnmpEngine(),
                                pysnmp.hlapi.CommunityData(
                                    'public', mpModel=0),
                                pysnmp.hlapi.UdpTransportTarget((host, 161)),
                                pysnmp.hlapi.ContextData(),
                                pysnmp.hlapi.ObjectType(pysnmp.hlapi.ObjectIdentity('SNMPv2-MIB', oid, 0)))
        )

        if errorIndication:
            logging.error(errorIndication)
        elif errorStatus:
            logging.error('%s at %s' % (errorStatus.prettyPrint(),
                                        errorIndex and varBinds[int(errorIndex) - 1][0] or '?'))
        else:
            for key, val in varBinds:
                logging.debug("%s = %s" %
                              (key.prettyPrint(), val.prettyPrint()))
                cur.execute("INSERT INTO monitor_logs (name, code, value) VALUES (%s, %s, %s)",
                            (self.host, key.prettyPrint(), val.prettyPrint()))

    def __walk(self, oid, cur):
        logging.debug("walk %s from %s", oid, self.host)


if __name__ == '__main__':
    logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s',
                        level=logging.DEBUG)

    parser = argparse.ArgumentParser(description='Snmp agent.')
    parser.add_argument('-c', '--config', required=True,
                        type=str, help='config file')

    args = parser.parse_args()
    logging.info("load config from %s" % args.config)
    cfg = toml.load(args.config)

    ok = threading.Event()
    ok.set()

    for host in cfg['hosts']:
        it = Crawler(
            psycopg2.connect(cfg['postgresql']),
            host,
            cfg['oids']['get'],
            cfg['oids']['walk'],
            cfg['sleep'],
            ok
        )
        it.daemon = True
        it.start()

    try:
        while True:
            time.sleep(.1)
    except (KeyboardInterrupt, SystemExit):
        logging.warning("exit...")
        ok.clear()
