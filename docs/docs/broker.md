## Broker Configuration

### Background

Feedback-Fusion employs an event-driven architecture where the main server component emits events that need to be processed asynchronously.
The indexer service handles the actual processing of these events. Communication between these components flows through a broker system that ensures reliable message delivery and decouples event production from consumption.

The broker implementation offers two distinct approaches:

- **gRPC**: Designed for smaller deployments where simplicity matters more than scale. The server sends asynchronous gRPC requests to the indexer,
which processes events and returns responses after completion. This approach works well for single-instance setups but doesn't scale horizontally since each server instance needs to communicate with specific indexer instances.

- **Fluvio**: Built for high-performance scenarios requiring horizontal scalability. Events are published to a distributed streaming platform
where multiple indexer instances can consume and process them concurrently. This enables dynamic scaling based on workload and provides better fault tolerance.
Fluvio is essentially a faster, more modern alternative to Apache Kafka. See [fluvio.io](https://fluvio.io) for more details.

Choose gRPC for development environments or small production setups where operational simplicity is preferred. Select Fluvio when you need to handle high event volumes across multiple server and indexer instances.

### Configuration

#### Fluvio Driver

Configure Fluvio as your broker driver for scalable, distributed event processing:

```hcl
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
  
  batch_interval = 0
}
```

#### gRPC Driver

Configure gRPC as your broker driver for simple, direct communication:

```hcl
broker = {
  grpc = {
    endpoint = "https://grpc.example.com"
    tls = {
      key                      = "/path/to/key.pem"
      certificate              = "/path/to/certificate.pem"
      certificate_authority    = "/path/to/ca.pem"
    }
  }
  max_batch_size = 10
  batch_interval = 1000
}
```

### Configuration Reference

#### Top-Level Parameters

| Parameter | Description | Default | Data Type |
|-----------|-------------|---------|-----------|
| fluvio | Fluvio broker driver configuration | N/A | Object |
| grpc | gRPC broker driver configuration | N/A | Object |
| max_batch_size | Maximum number of messages to process in a single batch | 10 | Integer |
| batch_interval | Time interval in milliseconds to wait before processing a batch | 1000 | Integer |

#### Fluvio Configuration

| Parameter | Description | Default | Data Type |
|-----------|-------------|---------|-----------|
| fluvio | Fluvio cluster configuration object. See [FluvioClusterConfig](https://docs.rs/fluvio/latest/fluvio/config/struct.FluvioClusterConfig.html) for available options | N/A | Object |
| topic | Topic name for event streams | "feedback-fusion" | String |

#### gRPC Configuration

| Parameter | Description | Default | Data Type |
|-----------|-------------|---------|-----------|
| endpoint | gRPC server endpoint URL | N/A | String |
| tls | TLS configuration for secure communication | N/A | Object |

#### TLS Configuration

| Parameter | Description | Default | Data Type |
|-----------|-------------|---------|-----------|
| key | Path to TLS private key file | N/A | String |
| certificate | Path to TLS certificate file | N/A | String |
| certificate_authority | Path to CA certificate file | N/A | String |

**Important**: You must configure exactly one broker driver. Choose either `fluvio` or `grpc` - configuring both will result in an error.
