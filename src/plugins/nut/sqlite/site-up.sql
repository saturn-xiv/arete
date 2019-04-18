CREATE TABLE cards(id INTEGER PRIMARY KEY NOT NULL,
                                          title VARCHAR(255) NOT NULL,
                                                             body TEXT NOT NULL,
                                                                       media_type VARCHAR(8) NOT NULL,
                                                                                             action VARCHAR(32) NOT NULL,
                                                                                                                href VARCHAR(255) NOT NULL,
                                                                                                                                  logo VARCHAR(255) NOT NULL,
                                                                                                                                                    loc VARCHAR(16) NOT NULL,
                                                                                                                                                                    lang VARCHAR(8) NOT NULL,
                                                                                                                                                                                    position SMALLINT NOT NULL,
                                                                                                                                                                                                      created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                                                                                            updated_at TIMESTAMP NOT NULL);


CREATE INDEX idx_cards_lang ON cards(lang);


CREATE INDEX idx_cards_loc ON cards(loc);


CREATE TABLE links(id INTEGER PRIMARY KEY NOT NULL,
                                          href VARCHAR(255) NOT NULL,
                                                            label VARCHAR(255) NOT NULL,
                                                                               loc VARCHAR(16) NOT NULL,
                                                                                               lang VARCHAR(8) NOT NULL,
                                                                                                               x SMALLINT NOT NULL,
                                                                                                                          y SMALLINT NOT NULL,
                                                                                                                                     created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                           updated_at TIMESTAMP NOT NULL);


CREATE INDEX idx_links_lang ON links(lang);


CREATE INDEX idx_links_loc ON links(loc);


CREATE TABLE friend_links(id INTEGER PRIMARY KEY NOT NULL,
                                                 title VARCHAR(255) NOT NULL,
                                                                    home VARCHAR(255) NOT NULL,
                                                                                      logo VARCHAR(255) NOT NULL,
                                                                                                        position SMALLINT NOT NULL,
                                                                                                                          created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                updated_at TIMESTAMP NOT NULL);


CREATE INDEX idx_friend_links_title ON friend_links(title);


CREATE TABLE leave_words(id INTEGER PRIMARY KEY NOT NULL,
                                                ip VARCHAR(45) NOT NULL,
                                                               body TEXT NOT NULL,
                                                                         media_type VARCHAR(8) NOT NULL,
                                                                                               created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP);


CREATE TABLE votes(id INTEGER PRIMARY KEY NOT NULL,
                                          point BIGINT NOT NULL,
                                                       resource_type VARCHAR(255) NOT NULL,
                                                                                  resource_id INTEGER NOT NULL,
                                                                                                     created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                           updated_at TIMESTAMP NOT NULL);


CREATE INDEX idx_votes_resource_type ON votes(resource_type);


CREATE UNIQUE INDEX idx_votes_resource ON votes(resource_type, resource_id);


CREATE TABLE tags(id INTEGER PRIMARY KEY NOT NULL,
                                         name VARCHAR(255) NOT NULL,
                                                           icon VARCHAR(16) NOT NULL,
                                                                            color VARCHAR(16) NOT NULL,
                                                                                              created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                    updated_at TIMESTAMP NOT NULL);


CREATE UNIQUE INDEX idx_tags_name ON tags(name);


CREATE TABLE tag_resources(id INTEGER PRIMARY KEY NOT NULL,
                                                  tag_id INTEGER NOT NULL,
                                                                resource_type VARCHAR(255) NOT NULL,
                                                                                           resource_id INTEGER NOT NULL,
                                                                                                              created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP);


CREATE UNIQUE INDEX idx_tag_resources ON tag_resources(tag_id, resource_type, resource_id);


CREATE TABLE categories(id INTEGER PRIMARY KEY NOT NULL,
                                               parent_id BIGINT, name VARCHAR(255) NOT NULL,
                                                                                   icon VARCHAR(16) NOT NULL,
                                                                                                    color VARCHAR(16) NOT NULL,
                                                                                                                      position SMALLINT NOT NULL,
                                                                                                                                        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                              updated_at TIMESTAMP NOT NULL);


CREATE INDEX idx_categories_name ON categories(name);


CREATE TABLE category_resources(id INTEGER PRIMARY KEY NOT NULL,
                                                       category_id INTEGER NOT NULL,
                                                                          resource_type VARCHAR(255) NOT NULL,
                                                                                                     resource_id INTEGER NOT NULL,
                                                                                                                        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP);


CREATE UNIQUE INDEX idx_category_resources ON category_resources(category_id, resource_type, resource_id);

