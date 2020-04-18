CREATE TABLE ops_mail_domains (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    version INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX idx_ops_mail_domains_name ON ops_mail_domains(name);

CREATE TABLE ops_mail_users (
    id INTEGER PRIMARY KEY NOT NULL,
    domain_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    locked_at TIMESTAMP,
    version INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_ops_mail_users_name ON ops_mail_users(name);

CREATE UNIQUE INDEX idx_ops_mail_users_email ON ops_mail_users(email);

CREATE TABLE ops_mail_aliases (
    id INTEGER PRIMARY KEY NOT NULL,
    domain_id INTEGER NOT NULL,
    source VARCHAR(255) NOT NULL,
    destination VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_ops_mail_aliases_source ON ops_mail_aliases(source);

CREATE UNIQUE INDEX idx_ops_mail_aliases ON ops_mail_aliases(domain_id, source, destination);