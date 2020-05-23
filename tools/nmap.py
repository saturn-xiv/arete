#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""Setup.

install third packages:
$ sudo apt-get install nmap

"""

import sys
import os
import argparse
import subprocess
import logging
from xml.etree import ElementTree

logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s',
                    level=logging.DEBUG)


def list_network(network):
    out = subprocess.Popen(
        ["sudo", "nmap", "-n", "-sn", "-oX", "-", network],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT
    )
    stdout, stderr = out.communicate()
    if stderr:
        logging.error(stderr)
        return
    logging.debug(stdout)
    doc = ElementTree.fromstring(stdout)
    for item in doc.iterfind('host'):
        logging.debug(item)


if __name__ == '__main__':

    parser = argparse.ArgumentParser(description='Nmap scan tools.')
    parser.add_argument('-n', '--network',
                        type=str, help='Network')

    args = parser.parse_args()

    if args.network:
        list_network(args.network)
