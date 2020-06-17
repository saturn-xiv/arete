CREATE TABLE monitor_logs(
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    uid VARCHAR(36) NOT NULL,
    code VARCHAR(255) NOT NULL,
    value TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_monitor_logs_name ON monitor_logs(name);

CREATE INDEX idx_monitor_logs_code ON monitor_logs(code);

CREATE INDEX idx_monitor_logs_uid ON monitor_logs(uid);

CREATE INDEX idx_monitor_logs_name_code ON monitor_logs(name, code);

CREATE INDEX idx_monitor_logs_name_uid ON monitor_logs(name, uid);