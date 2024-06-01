CREATE TABLE IF NOT EXISTS target (
  id          VARCHAR(32) UNIQUE NOT NULL PRIMARY KEY,
  name        VARCHAR(32) NOT NULL,
  description VARCHAR(255),
  updated_at  TIMESTAMP(3),
  created_at  TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS prompt (
  id          VARCHAR(32) UNIQUE NOT NULL PRIMARY KEY,
  title       VARCHAR(32) NOT NULL,
  description VARCHAR(255) NOT NULL,
  target      VARCHAR(32) NOT NULL,
  active      BOOLEAN NOT NULL,
  updated_at  TIMESTAMP(3),
  created_at  TIMESTAMP(3),
  FOREIGN KEY (target) REFERENCES target(id)
);

CREATE TABLE IF NOT EXISTS field (
  id          VARCHAR(32) UNIQUE NOT NULL PRIMARY KEY,
  title       VARCHAR(32) NOT NULL,
  description VARCHAR(255),
  prompt      VARCHAR(32) NOT NULL,
  type        VARCHAR(32) NOT NULL,
  options     TEXT NOT NULL,
  updated_at  TIMESTAMP(3),
  created_at  TIMESTAMP(3),
  FOREIGN KEY (prompt) REFERENCES prompt(id)
);

CREATE TABLE IF NOT EXISTS prompt_response (
  id          VARCHAR(32) UNIQUE NOT NULL PRIMARY KEY,
  prompt      VARCHAR(32) NOT NULL,
  created_at  TIMESTAMP(3),
  FOREIGN KEY (prompt) REFERENCES prompt(id)
);

CREATE TABLE IF NOT EXISTS field_response (
  id          VARCHAR(32) UNIQUE NOT NULL PRIMARY KEY,
  response    VARCHAR(32) NOT NULL,
  field       VARCHAR(32) NOT NULL,
  data        TEXT NOT NULL,
  FOREIGN KEY (response) REFERENCES prompt_response(id),
  FOREIGN KEY (field) REFERENCES field(id)
);

