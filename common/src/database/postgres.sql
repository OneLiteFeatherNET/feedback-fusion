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
  updated_at  timestamp,
  created_at  timestamp
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

CREATE TABLE IF NOT EXISTS oidc_user (
  id          VARCHAR(32) UNIQUE NOT NULL,
  username    VARCHAR(255) NOT NULL,
  updated_at  TIMESTAMP,
  created_at  TIMESTAMP
);

CREATE TABLE IF NOT EXISTS resource_authorization (
  id                  VARCHAR(32) UNIQUE NOT NULL,
  resource_kind       VARCHAR(255) NOT NULL,
  resource_id         VARCHAR(32) NOT NULL,
  authorization_type  VARCHAR(32) NOT NULL,
  authorization_grant  VARCHAR(32) NOT NULL,
  authorization_value VARCHAR(32) NOT NULL,
  updated_at  TIMESTAMP(3),
  created_at  TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS audit_version (
  id                  VARCHAR(32) UNIQUE NOT NULL,
  resource_type       VARCHAR(255) NOT NULL,
  resource_id         VARCHAR(32) NOT NULL,
  data                BYTEA NOT NULL, 
  made_by             VARCHAR(32) REFERENCES oidc_user(id) NOT NULL,
  action              VARCHAR(32) NOT NULL,
  version             VARCHAR(32) NOT NULL,
  created_at  TIMESTAMP(3)
);
