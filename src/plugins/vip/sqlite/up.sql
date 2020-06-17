CREATE TABLE vip_members(
    id INTEGER PRIMARY KEY NOT NULL,
    nick_name VARCHAR(64) NOT NULL,
    real_name VARCHAR(64) NOT NULL,
    gender VARCHAR(16) NOT NULL,
    birthday DATE NOT NULL,
    contact TEXT NOT NULL,
    point BIGINT NOT NULL DEFAULT 0,
    version INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX idx_vip_members_nick_name ON vip_members(nick_name);