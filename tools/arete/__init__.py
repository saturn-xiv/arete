# -*- coding: utf-8 -*-

import os
import logging

VIDEO_EXTENSIONS = ['.mp4']
MAX_RETRIES = 12


class Video:
    def __init__(self, path, playlist, title, description):
        self.title = title
        self.description = description
        self.playlist = playlist
        self.path = path

    def __repr__(self):
        return 'path="{path}" playlist="{playlist}" title="{title}" description="{description}"'.format(
            path=self.path,
            playlist=self.playlist,
            title=self.title,
            description=self.description
        )


def load_videos(root):
    items = []
    playlists = os.listdir(root)
    playlists.sort()
    for playlist in playlists:
        desc = open(os.path.join(root, playlist, 'description'),
                    'r').read().strip()
        title = open(os.path.join(root, playlist, 'title'), 'r').read().strip()
        logging.debug("find playlist %s \n %s" % (playlist, desc))

        videos = os.listdir(os.path.join(root, playlist))
        videos.sort()
        for video in videos:
            names = os.path.splitext(video)
            if names[1] in VIDEO_EXTENSIONS:
                logging.debug("find video file %s" % video)
                items.append(Video(
                    os.path.join(root, playlist, video),
                    playlist,
                    title.format(name=names[0]),
                    desc
                ))
    return items
