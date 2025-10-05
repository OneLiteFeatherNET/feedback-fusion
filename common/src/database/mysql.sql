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
  authorization_grant  VARCHAR(32) NOT NULL,
  authorization_value VARCHAR(32) NOT NULL,
  updated_at  TIMESTAMP(3),
  created_at  TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS audit_version (
  id                  VARCHAR(32) UNIQUE NOT NULL PRIMARY KEY,
  resource_type       VARCHAR(255) NOT NULL,
  resource_id         VARCHAR(32) NOT NULL,
  data                BLOB NOT NULL,
  made_by             VARCHAR(32) NOT NULL,
  action              VARCHAR(32) NOT NULL,
  version             INT UNSIGNED NOT NULL,
  created_at          DATETIME(3),
  FOREIGN KEY (made_by) REFERENCES oidc_user(id)
);

CREATE TABLE IF NOT EXISTS index_entry (
  id                  VARCHAR(32) UNIQUE NOT NULL PRIMARY KEY,
  key_type            VARCHAR(255) NOT NULL,
  key_value           VARCHAR(255) NOT NULL,
  value_type          VARCHAR(255) NOT NULL,
  value               VARCHAR(255),
  created_at          DATETIME(3)
);
