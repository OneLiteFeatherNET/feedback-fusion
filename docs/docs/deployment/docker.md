# Deployment via Docker

## Prerequisites
- Docker installed on your target machine. [Install Docker](https://docs.docker.com/get-docker/)
- Docker Compose installed on your target machine. [Install Docker Compose](https://docs.docker.com/compose/install/)
- A running database of your choice. [Supported Databases](/docs/configuration#database-configuration)

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
      # add here your configuration
    restart: unless-stopped
```

Add in as `environment` your actual configuration values. Refer to the [configuration documentation](/docs/configuration) for the fields that need to be set.

Afterwards start the application:

```sh 
docker compose up -d
```
