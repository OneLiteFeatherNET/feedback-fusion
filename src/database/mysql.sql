CREATE TABLE IF NOT EXISTS target (
  id          VARCHAR(32) UNIQUE NOT NULL PRIMARY KEY,
  name        VARCHAR(255) NOT NULL,
  description VARCHAR(255),
  updated_at  TIMESTAMP(3),
  created_at  TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS prompt (
  id          VARCHAR(32) UNIQUE NOT NULL PRIMARY KEY,
  title       VARCHAR(255) NOT NULL,
  description VARCHAR(255) NOT NULL,
  target      VARCHAR(32) NOT NULL,
  active      BOOLEAN NOT NULL,
  updated_at  TIMESTAMP(3),
  created_at  TIMESTAMP(3),
  FOREIGN KEY (target) REFERENCES target(id)
);

CREATE TABLE IF NOT EXISTS field (
  id          VARCHAR(32) UNIQUE NOT NULL PRIMARY KEY,
  title       VARCHAR(255) NOT NULL,
  description VARCHAR(255),
  prompt      VARCHAR(32) NOT NULL,
  field_type  VARCHAR(32) NOT NULL,
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

CREATE TABLE IF NOT EXISTS oidc_user (
  id          VARCHAR(32) UNIQUE NOT NULL PRIMARY KEY,
  username    VARCHAR(255) NOT NULL,
  updated_at  TIMESTAMP(3),
  created_at  TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS resource_authorization (
  id                  VARCHAR(32) UNIQUE NOT NULL PRIMARY KEY,
  resource_kind       VARCHAR(255) NOT NULL,
  resource_id         VARCHAR(32) NOT NULL,
  authorization_type  VARCHAR(32) NOT NULL,
  authoriztion_grant  VARCHAR(32) NOT NULL,
  authorization_value VARCHAR(32) NOT NULL,
  updated_at          TIMESTAMP(3),
  created_at          TIMESTAMP(3)     
);
