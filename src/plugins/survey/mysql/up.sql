CREATE TABLE survey_forms(
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    type TEXT NOT NULL,
    nbf DATE NOT NULL,
    exp DATE NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL
);

CREATE INDEX idx_survey_forms_title ON survey_forms(title);

CREATE TABLE survey_fields(
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    form_id BIGINT NOT NULL,
    `key` VARCHAR(32) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    required BOOLEAN NOT NULL,
    type TEXT NOT NULL,
    position SMALLINT NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL
);

CREATE UNIQUE INDEX idx_survey_fields_form_key ON survey_fields(form_id, `key`);

CREATE TABLE survey_responses(
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    form_id BIGINT NOT NULL,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(32) NOT NULL,
    ip VARCHAR(45) NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_survey_responses_ip ON survey_responses(ip);

CREATE INDEX idx_survey_responses_email ON survey_responses(email);

CREATE INDEX idx_survey_responses_username ON survey_responses(username);

CREATE TABLE survey_logs(
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    form_id BIGINT NOT NULL,
    user_id BIGINT,
    ip VARCHAR(45) NOT NULL,
    message VARCHAR(255) NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE survey_subscribers(
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    form_id BIGINT NOT NULL,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(32) NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_survey_subscribers_email_form ON survey_subscribers(email, form_id);

CREATE INDEX idx_survey_subscribers_email_username ON survey_subscribers(email, username);