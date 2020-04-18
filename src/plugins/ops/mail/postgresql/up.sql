CREATE TABLE ops_mail_domains (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    locked_at TIMESTAMP,
    version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX idx_ops_mail_domains ON ops_mail_domains(name);

CREATE TABLE ops_mail_users (
    id BIGSERIAL PRIMARY KEY,
    domain_id BIGINT NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX idx_ops_mail_users ON ops_mail_users(email);

CREATE TABLE ops_mail_aliases (
    id BIGSERIAL PRIMARY KEY,
    domain_id BIGINT NOT NULL,
    source VARCHAR(255) NOT NULL,
    destination VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);