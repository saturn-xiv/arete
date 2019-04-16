CREATE TABLE vip_members(id BIGINT AUTO_INCREMENT PRIMARY KEY,
                                                          nick_name VARCHAR(64) NOT NULL,
                                                                                real_name VARCHAR(64) NOT NULL,
                                                                                                      gender VARCHAR(16) NOT NULL,
                                                                                                                         birthday DATE NOT NULL,
                                                                                                                                       contact TEXT NOT NULL,
                                                                                                                                                    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                                         updated_at DATETIME NOT NULL);


CREATE UNIQUE INDEX idx_vip_members_nick_name ON vip_members(nick_name);

