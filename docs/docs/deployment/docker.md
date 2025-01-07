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
      FEEDBACK_FUSION_CONFIG: /path/to/container/config
    restart: unless-stopped
    volumes:
      - /path/to/config:/path/to/container/config
```

Refer to the [configuration documentation](/docs/configuration) for the config file details. 

Afterwards start the application:

```sh 
docker compose up -d
```

## Verifying Deployment

To verify your deployment you can now run the dockerized integration tests using `ghcr.io/onelitefeathernet/feedback-fusion-integrtion:<version>`.
The image requires the following environment variables to be set:

| Key             | Description                                         |
|-----------------|-----------------------------------------------------|
| OIDC_PROVIDER   | URL of the OIDC provider                           |
| OIDC_CLIENT_ID  | The client ID                                       |
| OIDC_CLIENT_SECRET | The client secret                               |
| GRPC_ENDPOINT   | The endpoint of the deployed application            |

### Run the tests 

```sh 
docker run --network <network> \
    -e OIDC_PROVIDER=<oidc_provider> \
    -e OIDC_CLIENT_ID=<oidc_client_id> \
    -e OIDC_CLIENT_SECRET=<oidc_client_secret> \
    -e GRPC_ENDPOINT=<grpc_endpoint> 
    --name feedback-fusion-integration-test \
    --rm \
    ghcr.io/onelitefeathernet/feedback-fusion-integrtion:<version>
```

### On finish

You should now reset your database as the integration test does not delete everything it created.
