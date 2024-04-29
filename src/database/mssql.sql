
IF NOT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = 'target')
BEGIN
    CREATE TABLE target (
        id           VARCHAR(32)    NOT NULL PRIMARY KEY,
        name         VARCHAR(32)    NOT NULL,
        description  VARCHAR(255),
        updated_at   DATETIME,
        created_at   DATETIME
    );
END;

IF NOT EXISTS (SELECT * FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = 'prompt')
BEGIN
    CREATE TABLE prompt (
        id           VARCHAR(32)    NOT NULL PRIMARY KEY,
        title        VARCHAR(32)    NOT NULL,
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
        title        VARCHAR(32)    NOT NULL,
        description  VARCHAR(255),
        prompt       VARCHAR(32)    NOT NULL REFERENCES prompt(id),
        type         VARCHAR(32)    NOT NULL,
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
