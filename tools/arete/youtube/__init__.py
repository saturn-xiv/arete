# -*- coding: utf-8 -*-

import os
import json
import logging


import google_auth_oauthlib
import googleapiclient
import google.oauth2.credentials
import google_auth_oauthlib.flow
import googleapiclient.discovery

PUBLIC_PRIVACY = 'public'


def walk_upload_playlist(youtube, func):
    response = youtube.channels().list(
        mine=True,
        part='snippet,contentDetails,statistics',
    ).execute()

    for channel in response['items']:
        logging.debug("find channel %s" % channel)
        func(youtube, channel['contentDetails']['relatedPlaylists']['uploads'])


def __auth(conf):
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


def connect(conf):
    cred = __auth(conf)
    return googleapiclient.discovery.build(
        'youtube',
        'v3',
        credentials=cred,
        cache_discovery=False
    )
