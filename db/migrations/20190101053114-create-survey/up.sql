CREATE TABLE survey_forms(
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT NOT NULL,
  title VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  type JSON NOT NULL,
  nbf DATE NOT NULL,
  exp DATE NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);
CREATE INDEX idx_survey_forms_title ON survey_forms(title);

CREATE TABLE survey_fields(
  id BIGSERIAL PRIMARY KEY,
  form_id BIGINT NOT NULL,
  key VARCHAR(32) NOT NULL,
  title VARCHAR(255) NOT NULL,
  description TEXT,
  required BOOLEAN NOT NULL,
  type JSON NOT NULL,
  position SMALLINT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);
CREATE UNIQUE INDEX idx_survey_fields_form_key ON survey_fields(form_id, key);

CREATE TABLE survey_responses(
  id BIGSERIAL PRIMARY KEY,
  form_id BIGINT NOT NULL,
  email VARCHAR(255) NOT NULL,
  username VARCHAR(32) NOT NULL,
  ip VARCHAR(39),
  content JSON NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_survey_responses_ip ON survey_responses(ip);
CREATE INDEX idx_survey_responses_email ON survey_responses(email);
CREATE INDEX idx_survey_responses_username ON survey_responses(username);

CREATE TABLE survey_logs(
  id BIGSERIAL PRIMARY KEY,
  form_id BIGINT NOT NULL,
  user_id BIGINT,
  ip VARCHAR(39),
  message VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE survey_subscribers(
  id BIGSERIAL PRIMARY KEY,
  form_id BIGINT NOT NULL,
  email VARCHAR(255) NOT NULL,
  username VARCHAR(32) NOT NULL,    
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_survey_subscribers_email_form ON survey_subscribers(email, form_id);
CREATE INDEX idx_survey_subscribers_email_username ON survey_subscribers(email, username);