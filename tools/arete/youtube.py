# -*- coding: utf-8 -*-

import logging
import os
import json


import google_auth_oauthlib
import googleapiclient
import google.oauth2.credentials
import google_auth_oauthlib.flow
import googleapiclient.discovery

PUBLIC_PRIVACY = 'public'


def __sign_in(conf):
    cred = os.path.splitext(conf)[0]+'.token'
    if os.path.exists(cred):
        logging.info("load credentials from %s" % cred)
        with open(cred, encoding='utf-8') as cred:
            buf = json.load(cred)
            return google.oauth2.credentials.Credentials(**buf)
    else:
        logging.info("load client secrets from %s", conf)
        flow = google_auth_oauthlib.flow.InstalledAppFlow.from_client_secrets_file(
            conf,
            [
                'https://www.googleapis.com/auth/youtube.force-ssl',
                'https://www.googleapis.com/auth/youtube.upload',
                'https://www.googleapis.com/auth/youtube.readonly',
                "https://www.googleapis.com/auth/youtube"
            ], state='arete')
        credentials = flow.run_console()
        logging.info("generate credentials file %s" % cred)
        with open(cred, 'w', encoding='utf-8') as cred:
            json.dump({
                'token': credentials.token,
                'refresh_token': credentials.refresh_token,
                'token_uri': credentials.token_uri,
                'client_id': credentials.client_id,
                'client_secret': credentials.client_secret,
                'scopes': credentials.scopes
            }, cred)
        return credentials


def __connect(credentials):
    return googleapiclient.discovery.build(
        'youtube',
        'v3',
        credentials=credentials,
        cache_discovery=False
    )


def __list_uploaded(youtube, uploads_playlist_id):
    request = youtube.playlistItems().list(
        playlistId=uploads_playlist_id,
        part='snippet,contentDetails',
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


def upload(conf, target):
    logging.info("upload videos in folder %s to youtube" % target)


def list(conf):
    logging.info("fetch all videos in youtube")
    cred = __sign_in(conf)
    youtube = __connect(cred)
    response = youtube.channels().list(
        mine=True,
        part='snippet,contentDetails,statistics',
    ).execute()

    for channel in response['items']:
        logging.debug("find channel %s" % channel)
        __list_uploaded(
            youtube,
            channel['contentDetails']['relatedPlaylists']['uploads']
        )
