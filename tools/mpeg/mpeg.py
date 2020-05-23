#!/usr/bin/env python
# -*- coding: utf-8 -*-

# https://www.ffmpeg.org/download.html

"""Setup.

install third packages:
$ sudo apt-get install ffmpeg

"""

import sys
import os
import argparse
import subprocess
import logging
import csv
import tempfile
import datetime
from urllib.request import pathname2url

logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s',
                    level=logging.DEBUG)


def _concat(in_):
    out = datetime.datetime.now().strftime("%Y%m%d%H:%M:%S.mp4")
    logging.debug("generate file %s" % out)
    job = subprocess.Popen(
        ["ffmpeg", "-f", "concat", "-i", in_,  "-c", "copy", out],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT
    )
    stdout, stderr = job.communicate()
    if stderr:
        logging.error(stderr)
        return
    logging.debug(stdout)
    return out


def _split(_in, begin, end):
    if not os.path.exists(_in):
        logging.error("can't find file %s" % _in)
        return

    out = pathname2url(_in, begin, end)
    if os.path.exists(out):
        logging.warning("%s already exists, ignore" % out)
        return out

    logging.debug("generate file %s" % out)
    job = subprocess.Popen(
        ["ffmpeg", "-i", _in, "-ss", begin, "-to", end, "-c", "copy", out],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT
    )
    stdout, stderr = job.communicate()
    if stderr:
        logging.error(stderr)
        return
    logging.debug(stdout)
    return pathname2url(_in, begin, end)


def _run(name):
    items = []
    with open("%s.csv" % name, 'r') as file:
        rdr = csv.reader(file)
        for row in rdr:
            if len(row) == 3:
                logging.info("find task file(%s) from(%s) to(%s)" % (
                    row[0], row[1], row[2]))
                out = _split(row[0], row[1], row[2])
                if out:
                    items.append(out)
            else:
                logging.warning("ignore line %s" % row)
    if len(items) > 0:
        logging.info("concat video files %s" % items)
        in_ = "%s.txt" % name
        with open(in_, 'w') as file:
            for it in items:
                file.write(it)
                file.write("\n")
        out = _concat(in_, "%s.mp4")
        if out:
            logging.info("Done!")


if __name__ == '__main__':

    parser = argparse.ArgumentParser(description='MP4 cutter & linker.')
    parser.add_argument('-n', '--name', required=True,
                        type=str, help='Job name')

    args = parser.parse_args()

    if args.name:
        _run(args.name)
