# -*- coding: utf-8 -*-

import logging

from . import walk_upload_playlist, connect


class Playlist:
    def __init__(self, id, channel, title):
        self.id = id
        self.channel = channel
        self.title = title


class Video:
    def __init__(self, id, title, description):
        self.id = id
        self.title = title
        self.description = description


def __list_uploaded(youtube, uploads_playlist_id):
    logging.info("fetch upload playlist %s" % uploads_playlist_id)


def start(conf, root):
    logging.info("upload videos in folder %s to youtube" % root)
    youtube = connect(conf)
    walk_upload_playlist(youtube, __list_uploaded)

    # for video in load_videos(root):
    #     logging.debug('find video %s' % video)
    #     if __get_video(youtube, video):
    #         logging.info('already exists')
    #     else:
    #         logging.info('try upload...')
    #         __upload_video(youtube, video)
