CREATE TABLE forum_topics(id INTEGER PRIMARY KEY NOT NULL,
                                                 user_id INTEGER NOT NULL,
                                                                 title VARCHAR(255) NOT NULL,
                                                                                    body TEXT NOT NULL,
                                                                                              media_type VARCHAR(8) NOT NULL,
                                                                                                                    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                          updated_at TIMESTAMP NOT NULL);


CREATE INDEX idx_forum_topics_title ON forum_topics(title);


CREATE TABLE forum_posts(id INTEGER PRIMARY KEY NOT NULL,
                                                user_id INTEGER NOT NULL,
                                                                topic_id INTEGER NOT NULL,
                                                                                 post_id BIGINT, body TEXT NOT NULL,
                                                                                                           media_type VARCHAR(8) NOT NULL,
                                                                                                                                 created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                       updated_at TIMESTAMP NOT NULL);

