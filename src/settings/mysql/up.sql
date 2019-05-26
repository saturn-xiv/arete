CREATE TABLE settings(id BIGINT AUTO_INCREMENT PRIMARY KEY,
                                                       `key` VARCHAR(255) NOT NULL,
                                                                          value BLOB NOT NULL,
                                                                                     salt BLOB, version BIGINT NOT NULL DEFAULT 0,
                                                                                                                                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                     updated_at DATETIME NOT NULL);


CREATE UNIQUE INDEX idx_settings_key ON settings(`key`);

