# -*- coding: utf-8 -*-

import logging
import httplib2

import random
import time

import googleapiclient.http

from . import connect, list_channels, PUBLIC_PRIVACY
from .. import load_videos

# httplib.NotConnected,
#                         httplib.IncompleteRead, httplib.ImproperConnectionState,
#                         httplib.CannotSendRequest, httplib.CannotSendHeader,
#                         httplib.ResponseNotReady, httplib.BadStatusLine
RETRIABLE_EXCEPTIONS = (httplib2.HttpLib2Error, IOError)
RETRIABLE_STATUS_CODES = [500, 502, 503, 504]


class Playlist:
    def __init__(self, id, channel, title, description, videos):
        self.id = id
        self.channel = channel
        self.title = title
        self.videos = videos
        self.description = description

    def __repr__(self):
        return 'id="{id}" channel="{channel}" title="{title}" videos="{videos}" description="{description}"'.format(
            id=self.id,
            channel=self.channel,
            title=self.title,
            description=self.description,
            videos=self.videos
        )


class Video:
    def __init__(self, id, title, description):
        self.id = id
        self.title = title
        self.description = description

    def __repr__(self):
        return 'id="{id}" title="{title}" description="{description}"'.format(
            id=self.id,
            title=self.title,
            description=self.description
        )


def __load_playlists(youtube):
    playlists = []

    for channel in list_channels(youtube):
        if channel['contentDetails']['relatedPlaylists']['uploads']:
            playlists.extend(__list_uploaded_playlist(youtube, channel['id']))

    return playlists


def __list_uploaded_playlist_items(youtube, playlist_id):
    videos = []
    logging.info("fetch upload playlist items for %s" % playlist_id)
    request = youtube.playlistItems().list(
        playlistId=playlist_id,
        part='snippet,contentDetails,status',
        maxResults=50
    )

    while request:
        response = request.execute()
        for item in response['items']:
            logging.debug("find playlist item %s" % item)
            title = item['snippet']['title']
            description = item['snippet']['description']
            video_id = item['snippet']['resourceId']['videoId']
            videos.append(Video(video_id, title, description))

        request = youtube.playlistItems().list_next(request, response)
    return videos


def __list_uploaded_playlist(youtube, channel_id):

    logging.info("fetch upload playlist for  %s" % channel_id)
    playlists = []

    request = youtube.playlists().list(
        channelId=channel_id,
        part='snippet,contentDetails,status',
        maxResults=50
    )

    while request:
        response = request.execute()
        for item in response['items']:
            logging.debug("find playlist %s" % item)
            title = item['snippet']['title']
            desc = item['snippet']['description']
            playlist_id = item['id']
            videos = __list_uploaded_playlist_items(youtube, playlist_id)

            playlists.append(
                Playlist(playlist_id, channel_id, title, desc, videos)
            )

        request = youtube.playlists().list_next(request, response)

    return playlists


def __upload_playlist(youtube, playlists, title, description):
    for it in playlists:
        if it.title == title:
            logging.info('playlist %s already exists' % title)
            return it

    logging.warn("can't find playlist %s" % title)
    response = youtube.playlists().insert(
        part='snippet,status',
        body=dict(
            snippet=dict(
                title=title,
                description=description
            ),
            status=dict(
                privacyStatus=PUBLIC_PRIVACY
            )
        )
    ).execute()

    logging.info("create playlist %s", title)
    it = Playlist(
        response['id'],
        response['snippet']['channelId'],
        title,
        description,
        []
    )
    playlists.append(it)
    return it


def __upload_video(youtube, playlists, item):
    pl = __upload_playlist(
        youtube, playlists, item.playlist, item.description)
    for it in pl.videos:
        if it.title == item.title:
            logging.info("%s already exists" % item.path)
            return
    logging.info("upload video %s => %s" % (item.path, item.title))

    insert_request = youtube.videos().insert(
        part='snippet,status'
        body=dict(
            snippet=dict(
                title=item.title,
                description=item.description,
            ),
            status=dict(
                privacyStatus=PUBLIC_PRIVACY
            )
        )
        media_body=googleapiclient.http.MediaFileUpload(
            item.file, chunksize=-1, resumable=True
        )
    )

    __resumable_upload(insert_request)


def __resumable_upload(request):
    response = None
    error = None
    retry = 0
    while response is None:
        try:
            print('Uploading file...')
            status, response = request.next_chunk()
            if response is not None:
                if 'id' in response:
                    print('Video id "%s" was successfully uploaded.' %
                          response['id'])
                else:
                    os.exit(
                        'The upload failed with an unexpected response: %s' % response)
        except HttpError as e:
            if e.resp.status in RETRIABLE_STATUS_CODES:
                error = 'A retriable HTTP error %d occurred:\n%s' % (e.resp.status,
                                                                     e.content)
            else:
                raise
        except RETRIABLE_EXCEPTIONS as e:
            error = 'A retriable error occurred: %s' % e

        if error is not None:
            print(error)
            retry += 1
            if retry > MAX_RETRIES:
                os.exit('No longer attempting to retry.')

            max_sleep = 2 ** retry
            sleep_seconds = random.random() * max_sleep
            print('Sleeping %f seconds and then retrying...' % sleep_seconds)
            time.sleep(sleep_seconds)


def start(conf, root):
    logging.info("upload videos in folder %s to youtube" % root)
    youtube = connect(conf)
    playlists = __load_playlists(youtube)

    for video in load_videos(root):
        __upload_video(youtube, playlists, video)
