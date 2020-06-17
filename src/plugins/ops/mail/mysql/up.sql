CREATE TABLE ops_mail_domains (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT NOW(),
    updated_at DATETIME NOT NULL
);

CREATE UNIQUE INDEX idx_ops_mail_domains_name ON ops_mail_domains(name);

CREATE TABLE ops_mail_users (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    domain_id BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    locked_at DATETIME,
    version BIGINT NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT NOW(),
    updated_at DATETIME NOT NULL
);

CREATE INDEX idx_ops_mail_users_name ON ops_mail_users(name);

CREATE UNIQUE INDEX idx_ops_mail_users_email ON ops_mail_users(email);

CREATE TABLE ops_mail_aliases (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    domain_id BIGINT NOT NULL,
    source VARCHAR(255) NOT NULL,
    destination VARCHAR(255) NOT NULL,
    created_at DATETIME NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ops_mail_aliases_source ON ops_mail_aliases(source);

CREATE UNIQUE INDEX idx_ops_mail_aliases ON ops_mail_aliases(domain_id, source, destination);