# -*- coding: utf-8 -*-

import logging

from .. import VIDEO_EXTENSIONS, load_videos
from . import connect, walk_upload_playlist


def __list_uploaded(youtube, uploads_playlist_id):
    logging.info("fetch upload playlist %s" % uploads_playlist_id)
    request = youtube.playlistItems().list(
        playlistId=uploads_playlist_id,
        part='snippet,contentDetails,status',
        maxResults=50
    )

    while request:
        response = request.execute()
        for item in response['items']:
            logging.debug("find playlist %s" % item)
            title = item['snippet']['title']
            video_id = item['snippet']['resourceId']['videoId']
            logging.info('%s (%s)' % (title, video_id))

        request = youtube.playlistItems().list_next(request, response)


def start(conf):
    logging.info("fetch all videos in youtube")
    youtube = connect(conf)
    walk_upload_playlist(youtube, __list_uploaded)
