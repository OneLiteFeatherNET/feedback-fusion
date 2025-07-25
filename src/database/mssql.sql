
IF NOT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = 'target')
BEGIN
    CREATE TABLE target (
        id           VARCHAR(32)    NOT NULL PRIMARY KEY,
        name         VARCHAR(255)    NOT NULL,
        description  VARCHAR(255),
        updated_at   DATETIME,
        created_at   DATETIME
    );
END;

IF NOT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = 'prompt')
BEGIN
    CREATE TABLE prompt (
        id           VARCHAR(32)    NOT NULL PRIMARY KEY,
        title        VARCHAR(255)    NOT NULL,
        description  VARCHAR(255)   NOT NULL,
        target       VARCHAR(32)    NOT NULL REFERENCES target(id),
        active       BIT            NOT NULL,
        updated_at   DATETIME,
        created_at   DATETIME
    );
END;

IF NOT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = 'field')
BEGIN
    CREATE TABLE field (
        id           VARCHAR(32)    NOT NULL PRIMARY KEY,
        title        VARCHAR(255)    NOT NULL,
        description  VARCHAR(255),
        prompt       VARCHAR(32)    NOT NULL REFERENCES prompt(id),
        field_type   VARCHAR(32)    NOT NULL,
        options      NVARCHAR(MAX)  NOT NULL,
        updated_at   DATETIME,
        created_at   DATETIME
    );
END;

IF NOT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = 'prompt_response')
BEGIN
    CREATE TABLE prompt_response (
        id           VARCHAR(32)    NOT NULL PRIMARY KEY,
        prompt       VARCHAR(32)    NOT NULL REFERENCES prompt(id),
        created_at   DATETIME
    );
END;

IF NOT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = 'field_response')
BEGIN
    CREATE TABLE field_response (
        id           VARCHAR(32)    NOT NULL PRIMARY KEY,
        response     VARCHAR(32)    NOT NULL REFERENCES prompt_response(id),
        field        VARCHAR(32)    NOT NULL REFERENCES field(id),
        data         NVARCHAR(MAX)  NOT NULL
    );
END;


IF NOT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = 'oidc_user')
BEGIN
    CREATE TABLE oidc_user (
        id          VARCHAR(32)    NOT NULL PRIMARY KEY,
        username    VARCHAR(255)   NOT NULL,
        updated_at  DATETIME,
        created_at  DATETIME
    );
END;

IF NOT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = 'resource_authorization')
BEGIN
    CREATE TABLE resource_authorization (
        id                  VARCHAR(32)    NOT NULL PRIMARY KEY,
        resource_kind       VARCHAR(255)   NOT NULL,
        resource_id         VARCHAR(32)    NOT NULL,
        authorization_type  VARCHAR(32)    NOT NULL,
        authorization_grant  VARCHAR(32)    NOT NULL,
        authorization_value VARCHAR(32)    NOT NULL,
        updated_at   DATETIME,
        created_at   DATETIME
    );
END;
