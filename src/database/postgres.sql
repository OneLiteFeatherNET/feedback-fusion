CREATE TABLE IF NOT EXISTS account(
  id            varchar(20) UNIQUE NOT NULL,
  username      varchar(20) UNIQUE NOT NULL,
  password_hash varchar(100),
  secret        varchar(100),
  salt          varchar(100),
  totp          boolean   DEFAULT FALSE,
  updated_at    timestamp,
  created_at    timestamp DEFAULT CURRENT_TIMESTAMP
);

CREATE TYPE totp_state AS ENUM ('pending', 'passed', 'failed');

CREATE TABLE IF NOT EXISTS totp_challenge(
  id            varchar(20) UNIQUE NOT NULL,
  account       varchar(20) NOT NULL,
  state         totp_state  NOT NULL DEFAULT 'pending',
  updated_at    timestamp,
  created_at    timestamp   DEFAULT CURRENT_TIMESTAMP
);

