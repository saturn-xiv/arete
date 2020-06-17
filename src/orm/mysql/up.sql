CREATE TABLE IF NOT EXISTS schema_migrations(id BIGINT AUTO_INCREMENT PRIMARY KEY,
                                                                              version CHAR(14) NOT NULL,
                                                                                               name VARCHAR(255) NOT NULL,
                                                                                                                 up TEXT NOT NULL,
                                                                                                                         down TEXT NOT NULL,
                                                                                                                                   run_at DATETIME);

