CREATE TABLE users(id INTEGER PRIMARY KEY NOT NULL,
                                          real_name VARCHAR(32) NOT NULL,
                                                                nick_name VARCHAR(32) NOT NULL,
                                                                                      email VARCHAR(255) NOT NULL,
                                                                                                         password BLOB, uid VARCHAR(36) NOT NULL,
                                                                                                                                        provider_type VARCHAR(16) NOT NULL,
                                                                                                                                                                  provider_id VARCHAR(255) NOT NULL,
                                                                                                                                                                                           access_token VARCHAR(255),
                                                                                                                                                                                                        logo VARCHAR(255) NOT NULL,
                                                                                                                                                                                                                          sign_in_count BIGINT NOT NULL DEFAULT 0,
                                                                                                                                                                                                                                                                current_sign_in_at TIMESTAMP,
                                                                                                                                                                                                                                                                                   current_sign_in_ip VARCHAR(45),
                                                                                                                                                                                                                                                                                                      last_sign_in_at TIMESTAMP,
                                                                                                                                                                                                                                                                                                                      last_sign_in_ip VARCHAR(45),
                                                                                                                                                                                                                                                                                                                                      confirmed_at TIMESTAMP,
                                                                                                                                                                                                                                                                                                                                                   locked_at TIMESTAMP,
                                                                                                                                                                                                                                                                                                                                                             deleted_at TIMESTAMP,
                                                                                                                                                                                                                                                                                                                                                                        version INTEGER NOT NULL DEFAULT 0,
                                                                                                                                                                                                                                                                                                                                                                                                         created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                                                                                                                                                                                                                                                                                               updated_at TIMESTAMP NOT NULL);


CREATE INDEX idx_users_real_name ON users(real_name);


CREATE UNIQUE INDEX idx_users_nick_name ON users(nick_name);


CREATE UNIQUE INDEX idx_users_email ON users(email);


CREATE UNIQUE INDEX idx_users_uid ON users(uid);


CREATE UNIQUE INDEX idx_users_provider ON users(provider_type, provider_id);


CREATE TABLE logs(id INTEGER PRIMARY KEY NOT NULL,
                                         user_id INTEGER NOT NULL,
                                                         ip VARCHAR(45) NOT NULL,
                                                                        message VARCHAR(255) NOT NULL,
                                                                                             created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP);


CREATE TABLE policies(id INTEGER PRIMARY KEY NOT NULL,
                                             user_id INTEGER NOT NULL,
                                                             role VARCHAR(32) NOT NULL,
                                                                              resource VARCHAR(255),
                                                                                       nbf DATE NOT NULL,
                                                                                                exp DATE NOT NULL,
                                                                                                         version INTEGER NOT NULL DEFAULT 0,
                                                                                                                                          created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                                updated_at TIMESTAMP NOT NULL);


CREATE INDEX idx_policies_role ON policies(role);


CREATE TABLE attachments(id INTEGER PRIMARY KEY NOT NULL,
                                                user_id INTEGER NOT NULL,
                                                                title VARCHAR(255) NOT NULL,
                                                                                   size BIGINT NOT NULL,
                                                                                               mime_type VARCHAR(255) NOT NULL,
                                                                                                                      url VARCHAR(255) NOT NULL,
                                                                                                                                       version INTEGER NOT NULL DEFAULT 0,
                                                                                                                                                                        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                                                              updated_at TIMESTAMP NOT NULL);


CREATE INDEX idx_attachments ON attachments(title);


CREATE TABLE notifications(id INTEGER PRIMARY KEY NOT NULL,
                                                  user_id INTEGER NOT NULL,
                                                                  url VARCHAR(255) NOT NULL,
                                                                                   body TEXT NOT NULL,
                                                                                             media_type VARCHAR(8) NOT NULL,
                                                                                                                   level VARCHAR(8) NOT NULL,
                                                                                                                                    read BOOLEAN NOT NULL,
                                                                                                                                                 version INTEGER NOT NULL DEFAULT 0,
                                                                                                                                                                                  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                                                                        updated_at TIMESTAMP NOT NULL);


CREATE INDEX idx_notifications ON notifications(level);

