CREATE TABLE vpn_users(id INTEGER PRIMARY KEY NOT NULL,
                                              name VARCHAR(32) NOT NULL,
                                                               email VARCHAR(255) NOT NULL,
                                                                                  password BLOB NOT NULL,
                                                                                                online BOOLEAN NOT NULL,
                                                                                                               locked_at TIMESTAMP,
                                                                                                                         startup DATE NOT NULL,
                                                                                                                                      shutdown DATE NOT NULL,
                                                                                                                                                    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                                          updated_at TIMESTAMP NOT NULL);


CREATE INDEX idx_vpn_users_name ON vpn_users(name);


CREATE UNIQUE INDEX idx_vpn_users_email ON vpn_users(email);


CREATE TABLE vpn_logs(id INTEGER PRIMARY KEY NOT NULL,
                                             user_id BIGINT NOT NULL,
                                                            trusted_ip VARCHAR(45) NOT NULL,
                                                                                   trusted_port INTEGER NOT NULL,
                                                                                                        remote_ip VARCHAR(45) NOT NULL,
                                                                                                                              remote_port INTEGER NOT NULL,
                                                                                                                                                  start_time TIMESTAMP NOT NULL,
                                                                                                                                                                       end_time TIMESTAMP,
                                                                                                                                                                                received NUMERIC NOT NULL,
                                                                                                                                                                                                 send NUMERIC NOT NULL);

