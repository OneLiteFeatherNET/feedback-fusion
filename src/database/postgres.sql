CREATE TABLE IF NOT EXISTS target (
  id          VARCHAR(32) UNIQUE NOT NULL,
  name        VARCHAR(32) NOT NULL,
  description VARCHAR(255),
  updated_at  TIMESTAMP,
  created_at  TIMESTAMP
);

CREATE TABLE IF NOT EXISTS prompt (
  id          VARCHAR(32) UNIQUE NOT NULL,
  title       VARCHAR(32) NOT NULL,
  target      VARCHAR(32) REFERENCES target(id) NOT NULL,
  active      BOOLEAN NOT NULL,
  updated_at  TIMESTAMP
  created_at  TIMESTAMP
);

CREATE TABLE IF NOT EXISTS field (
  id          VARCHAR(32) UNIQUE NOT NULL,
  title       VARCHAR(255) NOT NULL,
  prompt      VARCHAR(32) REFERENCES prompt(id) NOT NULL,
  type        VARCHAR(32) NOT NULL,
  options     BPCHAR NOT NULL,
  updated_at  TIMESTAMP,
  created_at  TIMESTAMP
);

