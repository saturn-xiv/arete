CREATE TABLE settings(id INTEGER PRIMARY KEY NOT NULL,
                                             key VARCHAR(255) NOT NULL,
                                                              value BLOB NOT NULL,
                                                                         salt BLOB, created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                          updated_at TIMESTAMP NOT NULL);


CREATE UNIQUE INDEX idx_settings_key ON settings(key);

