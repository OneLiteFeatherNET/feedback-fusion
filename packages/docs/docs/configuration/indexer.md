## Indexer Configuration

The indexer component processes events emitted by the main Feedback-Fusion server. Since it only consumes and processes events, its configuration is limited to the essential components needed for this task.

### Configuration Structure

The indexer configuration reuses the same configuration options as the main server, but only requires two sections:

- **Database Configuration**: Where to store processed data
- **Broker Configuration**: How to receive events from the server

### Example Configuration

```hcl
# Database configuration (same as server)
database = {
  postgres = {
    endpoint = "localhost:5432"
    username = "postgres_user"
    password = "postgres_password"
    database = "postgres_db"
  }
}

# Broker configuration (event consumption only)
broker = {
  fluvio = {
    topic = "feedback-fusion"
    
    fluvio = {
      endpoint = "127.0.0.1:9003"
      use_spu_local_address = false
      
      tls = {
        tls_policy = "disabled"
      }
    }
  }
}

# OR for gRPC-based setups:

broker = {
  grpc = {
    key                      = "/path/to/key.pem"
    certificate              = "/path/to/certificate.pem"
    certificate_authority    = "/path/to/ca.pem"
  }
}
```

### Configuration Reference

#### Database Configuration

Identical to the server database configuration. See [Server Configuration](/docs/configuration/server) for available database options.

#### Broker Configuration (Indexer-Specific)

| Parameter | Description | Default | Data Type |
|-----------|-------------|---------|-----------|
| fluvio | Fluvio consumer configuration | N/A | Object |
| grpc | gRPC TLS configuration (flattened) | N/A | Object |

**Note**: Unlike the server configuration, the indexer broker configuration:
- Does not include `endpoint` for gRPC (the indexer acts as a server, not a client)
- Does not include `max_batch_size` or `batch_interval` (these are producer-specific settings)
- Must use the same broker driver type (`fluvio` or `grpc`) as configured in the server
- For gRPC, the TLS parameters are flattened directly under the `grpc` object

For detailed broker configuration options, see [Broker Configuration](/docs/broker).
