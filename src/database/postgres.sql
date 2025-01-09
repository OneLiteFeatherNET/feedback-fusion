CREATE TABLE IF NOT EXISTS target (
  id          VARCHAR(32) UNIQUE NOT NULL,
  name        VARCHAR(255) NOT NULL,
  description VARCHAR(255),
  updated_at  TIMESTAMP,
  created_at  TIMESTAMP
);

CREATE TABLE IF NOT EXISTS prompt (
  id          VARCHAR(32) UNIQUE NOT NULL,
  title       VARCHAR(255) NOT NULL,
  description VARCHAR(255) NOT NULL,
  target      VARCHAR(32) REFERENCES target(id) NOT NULL,
  active      BOOLEAN NOT NULL,
  updated_at  TIMESTAMP,
  created_at  TIMESTAMP
);

CREATE TABLE IF NOT EXISTS field (
  id          VARCHAR(32) UNIQUE NOT NULL,
  title       VARCHAR(255) NOT NULL,
  description VARCHAR(255),
  prompt      VARCHAR(32) REFERENCES prompt(id) NOT NULL,
  field_type  VARCHAR(32) NOT NULL,
  options     TEXT NOT NULL,
  updated_at  TIMESTAMP,
  created_at  TIMESTAMP
);

CREATE TABLE IF NOT EXISTS prompt_response (
  id          VARCHAR(32) UNIQUE NOT NULL,
  prompt      VARCHAR(32) REFERENCES prompt(id) NOT NULL,
  created_at  TIMESTAMP
);

CREATE TABLE IF NOT EXISTS field_response (
  id          VARCHAR(32) UNIQUE NOT NULL,
  response    VARCHAR(32) REFERENCES prompt_response(id) NOT NULL,
  field       VARCHAR(32) REFERENCES field(id) NOT NULL,
  data        TEXT NOT NULL
);


