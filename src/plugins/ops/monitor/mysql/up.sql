CREATE TABLE monitor_logs(
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    code VARCHAR(255) NOT NULL,
    value TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_monitor_logs_name ON monitor_logs(name);

CREATE INDEX idx_monitor_logs_code ON monitor_logs(code);

CREATE INDEX idx_monitor_logs_name_code ON monitor_logs(name, code);