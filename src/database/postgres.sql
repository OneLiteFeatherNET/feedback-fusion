CREATE TABLE IF NOT EXISTS feedback_target (
  id          VARCHAR(32) UNIQUE NOT NULL,
  name        VARCHAR(32) NOT NULL,
  description VARCHAR(255),
  updated_at  TIMESTAMP,
  created_at  TIMESTAMP
);

CREATE TABLE IF NOT EXISTS feedback_prompt (
  id          VARCHAR(32) UNIQUE NOT NULL,
  title       VARCHAR(32) NOT NULL,
  target      VARCHAR(32) REFERENCES feedback_target(id) NOT NULL,
  active      BOOLEAN NOT NULL,
  updated_at  TIMESTAMP,
  created_at  TIMESTAMP
);

CREATE TABLE IF NOT EXISTS feedback_prompt_field (
  id          VARCHAR(32) UNIQUE NOT NULL,
  title       VARCHAR(255) NOT NULL,
  prompt      VARCHAR(32) REFERENCES feedback_prompt(id) NOT NULL,
  type        VARCHAR(32) NOT NULL,
  options     JSON NOT NULL,
  updated_at  TIMESTAMP,
  created_at  TIMESTAMP
);

CREATE TABLE IF NOT EXISTS response (
  id          VARCHAR(32) UNIQUE NOT NULL,
  prompt      VARCHAR(32) REFERENCES feedback_prompt(id) NOT NULL,
  created_at  TIMESTAMP
);

CREATE TABLE IF NOT EXISTS feedback_prompt_field_response (
  id          VARCHAR(32) UNIQUE NOT NULL,
  response    VARCHAR(32) REFERENCES response(id) NOT NULL,
  field       VARCHAR(32) REFERENCES feedback_prompt_field(id) NOT NULL,
  data        JSON NOT NULL
);


