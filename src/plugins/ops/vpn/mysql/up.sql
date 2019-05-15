CREATE TABLE vpn_users(id BIGINT AUTO_INCREMENT PRIMARY KEY,
                                                        name VARCHAR(32) NOT NULL,
                                                                         email VARCHAR(255) NOT NULL,
                                                                                            password BLOB NOT NULL,
                                                                                                          online BOOLEAN NOT NULL DEFAULT FALSE,
                                                                                                                                          locked_at DATETIME,
                                                                                                                                          startup DATE NOT NULL,
                                                                                                                                                       shutdown DATE NOT NULL,
                                                                                                                                                                     created_at DATETIME NOT NULL DEFAULT NOW(),
                                                                                                                                                                                                          updated_at DATETIME NOT NULL);


CREATE INDEX idx_vpn_users_name ON vpn_users(name);


CREATE UNIQUE INDEX idx_vpn_users_email ON vpn_users(email);


CREATE TABLE vpn_logs(id BIGINT AUTO_INCREMENT PRIMARY KEY,
                                                       user_id BIGINT NOT NULL,
                                                                      type VARCHAR(16) NOT NULL,
                                                                                       trusted_ip VARCHAR(45) NOT NULL,
                                                                                                              trusted_port INTEGER NOT NULL,
                                                                                                                                   remote_ip VARCHAR(45) NOT NULL,
                                                                                                                                                         remote_port INTEGER NOT NULL,
                                                                                                                                                                             received DOUBLE PRECISION NOT NULL,
                                                                                                                                                                                                       send DOUBLE PRECISION NOT NULL,
                                                                                                                                                                                                                             created_at DATETIME NOT NULL DEFAULT NOW());

