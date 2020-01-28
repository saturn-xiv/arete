#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
https://github.com/youtube/api-samples/tree/master/python

$ virtualenv -p /usr/bin/python3 tmp/local/python
$ source tmp/local/python/bin/activate
$ pip install pyzmq bottle
$ pip install google-api-python-client google-auth google-auth-oauthlib google-auth-httplib2

"""

import os
import datetime
import argparse
import sys
import subprocess
import urllib.request
import json
import logging
import time

import arete.server
import arete.youtube
import arete.youku


CHOICE_UPLOAD = "upload"
CHOICE_LIST = "list"


# ------------------------ youtube -----------------


# def youtube_authenticated_service(conf):


# def youtube_list(conf):


# def youtube_upload(conf, folder):
#     youtube = youtube_authenticated_service(conf)
#     logging.info("load videos from %s", folder)

# -------------------------------------


if __name__ == '__main__':
    logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s',
                        level=logging.DEBUG)

    parser = argparse.ArgumentParser(description='Arete background tools.')
    parser.add_argument('--youtube', choices=[CHOICE_LIST, CHOICE_UPLOAD],
                        help='youtube api v3')
    parser.add_argument('--youku', choices=[CHOICE_LIST, CHOICE_UPLOAD],
                        help='youku api')
    parser.add_argument('-w', '--work-dir', help='working folder')
    parser.add_argument('--google-client-secrets',
                        help='google client secrets file')
    parser.add_argument('-s', '--server', type=int, help='start server')

    args = parser.parse_args()
    if args.youtube == CHOICE_LIST:
        arete.youtube.list(args.google_client_secrets)
    elif args.youtube == CHOICE_UPLOAD:
        arete.youtube.upload(args.google_client_secrets, args.work_dir)
    elif args.youku == CHOICE_LIST:
        arete.youku.list(args.credentials)
    elif args.youku == CHOICE_UPLOAD:
        arete.youku.upload(args.credentials, args.work_dir)
    elif args.server:
        arete.server.start(args.server)
    else:
        parser.print_help(sys.stderr)
