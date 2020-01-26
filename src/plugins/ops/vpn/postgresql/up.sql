CREATE TABLE vpn_users(
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password BYTEA NOT NULL,
    online BOOLEAN NOT NULL DEFAULT FALSE,
    fixed_ip VARCHAR(45),
    locked_at TIMESTAMP,
    startup DATE NOT NULL,
    shutdown DATE NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_vpn_users_name ON vpn_users(name);

CREATE UNIQUE INDEX idx_vpn_users_email ON vpn_users(email);

CREATE TABLE vpn_logs(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    trusted_ip VARCHAR(45) NOT NULL,
    trusted_port INTEGER NOT NULL,
    remote_ip VARCHAR(45) NOT NULL,
    remote_port INTEGER NOT NULL,
    received BIGINT,
    send BIGINT,
    opened_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    closed_at TIMESTAMP
);