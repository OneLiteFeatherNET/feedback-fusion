# Server-Configuration

## OIDC Configuration

```yaml
oidc:
  provider: "https://example.com/oidc"
  audience: "feedback-fusion"
  issuer: "https://issuer.example.com"
  group_claim: "groups"
  scopes:
    - name: "ApiAccess"
      grants:
        - endpoint: "*"
          permissions:
            - "*"
    - name: "ReadAccess"
      grants:
        - endpoint: "*"
          permissions:
            - "Read"
            - "List"
  groups:
    - name: "admin"
      grants: 
        - endpoint: "*"
          permissions:
            - "*"
```

### OIDC Configuration Reference

| Parameter   | Description                                    | Default           | Data Type |
|-------------|------------------------------------------------|-------------------|-----------|
| provider    | OIDC provider URL                              | N/A               | String    |
| audience    | Audience for OIDC tokens                       | "feedback-fusion" | String    |
| issuer      | Optional issuer URL for OIDC                   | N/A               | String    |
| group_claim | Name of the claim that contains user groups    | "groups"          | String    |
| scopes      | Access scopes and permissions                  | N/A               | List      |
| groups      | User groups and their permissions              | N/A               | List      |

### Available Endpoints and Permissions

- **Endpoints**: "Target", "Prompt", "Field", "Response", "Export"
- **Permissions**: "Read", "Write", "List"

## OTLP Configuration

```yaml
otlp:
  endpoint: "https://otlp.example.com"
  service_name: "feedback-fusion"
```

### OTLP Configuration Reference

| Parameter    | Description                        | Default           | Data Type |
|--------------|------------------------------------|-------------------|-----------|
| endpoint     | OTLP endpoint for trace spans      | N/A               | String    |
| service_name | Service name used in tracing context | "feedback-fusion" | String    |

## Database Configuration

### PostgreSQL

```yaml
database:
  postgres:
    endpoint: "localhost:5432"
    username: "postgres_user"
    password: "postgres_password"
    database: "postgres_db"
```

#### PostgreSQL Configuration Reference

| Parameter | Description                      | Default | Data Type |
|-----------|----------------------------------|---------|-----------|
| endpoint  | PostgreSQL hostname and port     | N/A     | String    |
| username  | Username for PostgreSQL          | N/A     | String    |
| password  | Password for PostgreSQL          | N/A     | String    |
| database  | Name of the PostgreSQL database  | N/A     | String    |

### MySQL / MariaDB

```yaml
database:
  mysql:
    endpoint: "localhost:3306"
    username: "mysql_user"
    password: "mysql_password"
    database: "mysql_db"
```

#### MySQL / MariaDB Configuration Reference

| Parameter | Description                      | Default | Data Type |
|-----------|----------------------------------|---------|-----------|
| endpoint  | MySQL/MariaDB hostname and port  | N/A     | String    |
| username  | Username for MySQL/MariaDB       | N/A     | String    |
| password  | Password for MySQL/MariaDB       | N/A     | String    |
| database  | Name of the MySQL/MariaDB database | N/A   | String    |

### MSSQL

```yaml
database:
  mssql:
    endpoint: "localhost:1433"
    username: "mssql_user"
    password: "mssql_password"
    database: "mssql_db"
    encrypt: true
    trust_server_certificate: true
```

#### MSSQL Configuration Reference

| Parameter                | Description                               | Default | Data Type |
|--------------------------|-------------------------------------------|---------|-----------|
| endpoint                 | MSSQL hostname and port                   | N/A     | String    |
| username                 | Username for MSSQL                        | N/A     | String    |
| password                 | Password for MSSQL                        | N/A     | String    |
| database                 | Name of the MSSQL database                | N/A     | String    |
| encrypt                  | Encrypt connection to MSSQL               | true    | Boolean   |
| trust_server_certificate | Trust server certificate for MSSQL        | true    | Boolean   |

## Presets 

Example: 
```yaml 
preset:
  targets:
  - id: target 
    name: TestTarget 
    description: A nice Target 
    prompts:
      - id: prompt 
        title: Testprompt 
        description: A nice Prompt 
        active: true
        fields:
          - id: field1 
            title: TextField
            field_type: text 
            options:
              type: text
              lines: 1 
              placeholder: test
```
