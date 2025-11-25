# Deployment via Docker

## Prerequisites
- Docker installed on your target machine. [Install Docker](https://docs.docker.com/get-docker/)
- Docker Compose installed on your target machine. [Install Docker Compose](https://docs.docker.com/compose/install/)
- A running database of your choice. See [Database Configuration](/docs/configuration/server#database-configuration) for supported databases.
- For Fluvio deployments: A running Fluvio cluster. See [Broker Configuration](/docs/broker) for setup instructions.

## Docker Compose Configuration
Create a `docker-compose.yml` file with the following content:

```yaml
version: "3"

services:
  feedback-fusion:
    image: ghcr.io/onelitefeathernet/feedback-fusion:latest
    container_name: feedback-fusion
    ports:
      - "8000:8000"
    environment:
      RUST_LOG: INFO 
      FEEDBACK_FUSION_CONFIG: /path/to/container/config
    restart: unless-stopped
    volumes:
      - /path/to/config:/path/to/container/config
  
  feedback-fusion-indexer:
    image: ghcr.io/onelitefeathernet/feedback-fusion-indexer:latest
    container_name: feedback-fusion-indexer
    environment:
      RUST_LOG: INFO 
      FEEDBACK_FUSION_CONFIG: /path/to/container/config
    restart: unless-stopped
    volumes:
      - /path/to/indexer-config:/path/to/container/config
```

Refer to the [Server Configuration](/docs/configuration/server) for the main server config file details and [Indexer Configuration](/docs/configuration/indexer) for the indexer config file details.

Afterwards start the application:

```sh 
docker compose up -d
```
