CREATE TABLE cards(
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    media_type VARCHAR(36) NOT NULL,
    action VARCHAR(32) NOT NULL,
    href VARCHAR(255) NOT NULL,
    logo VARCHAR(255) NOT NULL,
    loc VARCHAR(16) NOT NULL,
    lang VARCHAR(8) NOT NULL,
    position SMALLINT NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_cards_lang ON cards(lang);

CREATE INDEX idx_cards_loc ON cards(loc);

CREATE TABLE links(
    id BIGSERIAL PRIMARY KEY,
    href VARCHAR(255) NOT NULL,
    label VARCHAR(255) NOT NULL,
    loc VARCHAR(16) NOT NULL,
    lang VARCHAR(8) NOT NULL,
    x SMALLINT NOT NULL,
    y SMALLINT NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_links_lang ON links(lang);

CREATE INDEX idx_links_loc ON links(loc);

CREATE TABLE friend_links(
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    home VARCHAR(255) NOT NULL,
    logo VARCHAR(255) NOT NULL,
    position SMALLINT NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_friend_links_title ON friend_links(title);

CREATE TABLE leave_words(
    id BIGSERIAL PRIMARY KEY,
    ip VARCHAR(45) NOT NULL,
    body TEXT NOT NULL,
    media_type VARCHAR(36) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE votes(
    id BIGSERIAL PRIMARY KEY,
    point BIGINT NOT NULL,
    resource_type VARCHAR(255) NOT NULL,
    resource_id BIGINT NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_votes_resource_type ON votes(resource_type);

CREATE UNIQUE INDEX idx_votes_resource ON votes(resource_type, resource_id);

CREATE TABLE tags(
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    icon VARCHAR(16) NOT NULL,
    color VARCHAR(16) NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX idx_tags_name ON tags(name);

CREATE TABLE tag_resources(
    id BIGSERIAL PRIMARY KEY,
    tag_id BIGINT NOT NULL,
    resource_type VARCHAR(255) NOT NULL,
    resource_id BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_tag_resources ON tag_resources(tag_id, resource_type, resource_id);

CREATE TABLE categories(
    id BIGSERIAL PRIMARY KEY,
    parent_id BIGINT,
    name VARCHAR(255) NOT NULL,
    icon VARCHAR(16) NOT NULL,
    color VARCHAR(16) NOT NULL,
    position SMALLINT NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_categories_name ON categories(name);

CREATE TABLE category_resources(
    id BIGSERIAL PRIMARY KEY,
    category_id BIGINT NOT NULL,
    resource_type VARCHAR(255) NOT NULL,
    resource_id BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_category_resources ON category_resources(category_id, resource_type, resource_id);