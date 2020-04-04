# -*- coding: utf-8 -*-

# https://github.com/etingof/pysnmp
# https://wiki.archlinux.org/index.php/Snmpd

import logging

import toml
from pysnmp.hlapi import *


def __get(host, oid):
    logging.debug("get %s@%s", oid, host)
    iterator = getCmd(SnmpEngine(),
                      CommunityData('public'),
                      UdpTransportTarget((host, 161)),
                      ContextData(),
                      ObjectType(ObjectIdentity('SNMPv2-MIB', oid, 0)))
    errorIndication, errorStatus, errorIndex, varBinds = next(iterator)

    if errorIndication:
        logging.error(errorIndication)
        return

    if errorStatus:
        logging.error('%s at %s' % (errorStatus.prettyPrint(),
                                    varBinds[int(errorIndex)-1] if errorIndex else '?'))
        return

    for varBind in varBinds:
        logging.info(' = '.join([x.prettyPrint() for x in varBind]))


def __walk(host, oid):
    logging.debug("walk %s@%s", oid, host)
    # walkCmd(SnmpEngine(),
    #         CommunityData('public'),
    #         UdpTransportTarget((host, 161)),
    #         ContextData(),
    #         ObjectType(ObjectIdentity('SNMPv2-MIB', oid, 0)))


def __fetch(host):
    for oid in ["sysDescr"]:
        __get(host, oid)
    for oid in ["hrSWRunName", "hrSWInstalledName"]:
        __walk(host, oid)


def start(cfg):
    logging.info("load config file from %s" % cfg)
    cfg = toml.load(cfg)
    for host in cfg["hosts"]:
        __fetch(host)
