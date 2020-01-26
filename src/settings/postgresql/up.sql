CREATE TABLE settings(
    id BIGSERIAL PRIMARY KEY,
    key VARCHAR(255) NOT NULL,
    value BYTEA NOT NULL,
    salt BYTEA,
    version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX idx_settings_key ON settings(key);