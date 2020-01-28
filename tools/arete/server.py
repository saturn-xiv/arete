# -*- coding: utf-8 -*-

from bottle import get, run


@get('/dict/<keyword>')
def dict(keyword):
    return []


def start(port):
    run(host='127.0.0.1', port=port)
