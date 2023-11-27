CREATE TABLE IF NOT EXISTS account(
  id            varchar(32) UNIQUE NOT NULL,
  username      varchar(20) UNIQUE,
  type          varchar(16) NOT NULL,
  password_hash varchar(100),
  secret        varchar(100),
  salt          varchar(100),
  totp          boolean   DEFAULT FALSE,
  updated_at    timestamp,
  created_at    timestamp DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS totp_challenge(
  id            varchar(32) UNIQUE NOT NULL,
  account       varchar(20) NOT NULL,
  state         varchar(20)  NOT NULL DEFAULT 'pending',
  updated_at    timestamp,
  created_at    timestamp   DEFAULT CURRENT_TIMESTAMP
);

