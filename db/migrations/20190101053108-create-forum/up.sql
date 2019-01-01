CREATE TABLE forum_topics(
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT NOT NULL,
  title VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  media_type VARCHAR(8) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);
CREATE INDEX idx_forum_topics_title ON forum_topics(title);

CREATE TABLE forum_posts(
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT NOT NULL,
  topic_id BIGINT NOT NULL,
  post_id BIGINT,
  body TEXT NOT NULL,
  media_type VARCHAR(8) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL
);

CREATE TABLE forum_topics_tags(
  id BIGSERIAL PRIMARY KEY,
  topic_id BIGINT NOT NULL,
  tag_id BIGINT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_forum_topics_tags ON forum_topics_tags(topic_id, tag_id);

CREATE TABLE forum_topics_categories(
  id BIGSERIAL PRIMARY KEY,
  topic_id BIGINT NOT NULL,
  category_id BIGINT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_forum_topics_categories ON forum_topics_categories(topic_id, category_id);
