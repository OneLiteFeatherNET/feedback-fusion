# Configuration

## General Configuration 

You can set the following environment variables:

| Environment Variable    | Type              | Default Value              | Description                                                                 |
|-------------------------|-------------------|----------------------------|-----------------------------------------------------------------------------|
| `GLOBAL_RATE_LIMIT`     | `u64`             | `10`                       | The global rate limit for requests.                                         |
| `OIDC_PROVIDER`         | `String`          | N/A                        | The OIDC provider URL.                                                      |
| `OIDC_AUDIENCE`         | `String`          | `"feedback-fusion"`        | The audience for the OIDC tokens.                                           |
| `OIDC_ISSUER`           | `Option<String>`  | `None`                     | The optional issuer URL for the OIDC tokens.                                |
| `CONFIG_PATH`           | `Option<String>`  | `None`                     | The optional path to the configuration file. (Not Required using the helm chart) |
| `RUST_LOG`              | `String`          | `None`                     | The log level for the application. [Possible values](https://docs.rs/log/latest/log/enum.Level.html) | 
| `OTLP_ENDPOINT`         | `Option<String>`  | `None`                     | The gRPC OTLP endpoint to send the trace spans to                           |
| `SERVICE_NAME`          | `String`          | `"feedback-fusion"`        | Service name used in tracing context                                        |

## Database Configuration

The Backend supports mutliple database backends. The backend will choose the database based on your provided configuration values.


### PostgreSQL
| Environment Variable    | Type              | Default Value              | Description                                                                 |
|-------------------------|-------------------|----------------------------|-----------------------------------------------------------------------------|
| `POSTGRES_ENDPOINT`     | `String`          | N/A                        | The endpoint for the PostgreSQL database.                                   |
| `POSTGRES_USERNAME`     | `String`          | N/A                        | The username for the PostgreSQL database.                                   |
| `POSTGRES_PASSWORD`     | `String`          | N/A                        | The password for the PostgreSQL database.                                   |
| `POSTGRES_DATABASE`     | `String`          | N/A                        | The name of the PostgreSQL database.                                        |

### MySQL / MariaDB
| Environment Variable    | Type              | Default Value              | Description                                                                 |
|-------------------------|-------------------|----------------------------|-----------------------------------------------------------------------------|
| `MYSQL_ENDPOINT`        | `String`          | N/A                        | The endpoint for the MySQL database.                                        |
| `MYSQL_USERNAME`        | `String`          | N/A                        | The username for the MySQL database.                                        |
| `MYSQL_PASSWORD`        | `String`          | N/A                        | The password for the MySQL database.                                        |
| `MYSQL_DATABASE`        | `String`          | N/A                        | The name of the MySQL database.                                             |

### MSSQL
| Environment Variable               | Type              | Default Value | Description                                                                 |
|------------------------------------|-------------------|---------------|-----------------------------------------------------------------------------|
| `MSSQL_ENDPOINT`                   | `String`          | N/A           | The endpoint for the MSSQL database.                                        |
| `MSSQL_USERNAME`                   | `String`          | N/A           | The username for the MSSQL database.                                        |
| `MSSQL_PASSWORD`                   | `String`          | N/A           | The password for the MSSQL database.                                        |
| `MSSQL_DATABASE`                   | `String`          | N/A           | The name of the MSSQL database.                                             |
| `MSSQL_ENCRYPT`                    | `bool`            | `true`        | Whether to encrypt the connection to the MSSQL database.                    |
| `MSSQL_TRUST_SERVER_CERTIFICATE`   | `bool`            | `true`        | Whether to trust the server certificate for the MSSQL database connection.  |

## Presets 

Example: 
```yaml 
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
