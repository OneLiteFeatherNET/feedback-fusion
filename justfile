DOCKER_NETWORK := "feedback-fusion"
LOCAL_DOCKER_IMAGE := "feedback-fusion"
LOCAL_PLATFORM := "linux/" + replace(replace(arch(), "x86_64", "amd64"), "aarch64", "arm64") 
DEFAULT_TEST := "postgres"

test-all:
  just test postgres
  just test mariadb
  just test mysql
  just test mssql
  just test skytable

#
# Linting
#

check:
  cargo check --all-features

clippy:
  cargo clippy --all-features -- -D warnings

#
# Docker
#

build PLATFORM=LOCAL_PLATFORM DOCKERFILE="./Dockerfile":
  @echo "Building for {{PLATFORM}}"
  docker buildx build -t {{LOCAL_DOCKER_IMAGE}} --platform {{PLATFORM}} -f {{DOCKERFILE}} .

build-all DOCKERFILE="./Dockerfile":
  just build linux/arm64,linux/amd64 {{DOCKERFILE}}

#
# Backend
#

backend TYPE=DEFAULT_TEST:
  docker run --name {{LOCAL_DOCKER_IMAGE}} -d -e FEEDBACK_FUSION_CONFIG="/etc/feedback-fusion/config.yaml" -v ./tests/_common/configs/{{TYPE}}.yaml:/etc/feedback-fusion/config.yaml -e RUST_LOG=DEBUG --network {{DOCKER_NETWORK}} -p 8000:8000 {{LOCAL_DOCKER_IMAGE}}

bench: docker oidc-server-mock postgres && cleanup
  just backend postgres
  GRPC_ENDPOINT=http://localhost:8000 OIDC_CLIENT_ID=client OIDC_CLIENT_SECRET=secret OIDC_PROVIDER=http://localhost:5151 cargo bench

#
# Testing requirements
#

@docker: cleanup
  docker network create {{DOCKER_NETWORK}} > /dev/null

@oidc-server-mock:
  docker compose -f tests/_common/oidc-mock/docker-compose.yaml up -d 
  sleep 5
  curl -s -o /dev/null http://localhost:5151/.well-known/openid-configuration

@postgres:
  docker run --name database -e POSTGRES_PASSWORD=password -e POSTGRES_USERNAME=postgres --network {{DOCKER_NETWORK}} -d postgres
  sleep 5

@mysql:
  docker run --name database -e MYSQL_ROOT_PASSWORD=password -e MYSQL_PASSWORD=password -e MYSQL_USER=username -e MYSQL_DATABASE=database --network {{DOCKER_NETWORK}} -d mysql
  sleep 30

@mariadb:
  docker run --name database -e MYSQL_ROOT_PASSWORD=password -e MYSQL_PASSWORD=password -e MYSQL_USER=username -e MYSQL_DATABASE=database --network {{DOCKER_NETWORK}} -d mariadb
  sleep 10

@mssql:
  docker run --name database -e ACCEPT_EULA=Y -e MSSQL_SA_PASSWORD=Password1 --network {{DOCKER_NETWORK}} -d mcr.microsoft.com/mssql/server:2022-latest
  sleep 10

@skytable: postgres
  docker run -p 2003:2003 --entrypoint skyd --rm --name skytable --network {{DOCKER_NETWORK}} -d skytable/skytable --auth-root-password=passwordpassword --endpoint=tcp@0.0.0.0:2003

# 
# Testing
#

@cleanup:
  -docker rm -f database > /dev/null 2>&1
  -docker rm -f oidc-server-mock > /dev/null 2>&1
  -docker rm -f feedback-fusion > /dev/null 2>&1
  -docker rm -f skytable > /dev/null 2>&1
  -docker network rm {{DOCKER_NETWORK}} > /dev/null 2>&1

unittest:
  cargo test --bin feedback-fusion

integration:
  OIDC_PROVIDER="http://localhost:5151" OIDC_CLIENT_ID="client" OIDC_CLIENT_SECRET="secret" RUST_LOG="INFO" GRPC_ENDPOINT="http://localhost:8000" cargo test --no-fail-fast --test integration_test

test TYPE=DEFAULT_TEST: docker oidc-server-mock
  just {{TYPE}}
  if [ "{{TYPE}}" = "mariadb" ]; then just backend mysql; else just backend {{TYPE}}; fi
  sleep 1
  just integration

  -docker rm -f database > /dev/null 2>&1
  -docker rm -f feedback-fusion > /dev/null 2>&1

fuzz:
  OIDC_PROVIDER="http://localhost:5151" OIDC_CLIENT_ID="client" OIDC_CLIENT_SECRET="secret" RUST_LOG="INFO" GRPC_ENDPOINT="http://localhost:8000" cargo fuzz run fuzz_create_and_export

#
# Lib & Docs
#

pnpm PACKAGE:
  pnpm i -C {{PACKAGE}}

generate PACKAGE:
  pnpm run -C {{PACKAGE}} protoc

lint PACKAGE:
  pnpm run -C {{PACKAGE}} lint

lib-dev:
  just pnpm docs
  just pnpm lib

  just generate lib
  pnpm run -C lib dev

lib:
  just pnpm lib
  just generate lib

  pnpm run -C lib build

translations:
  pnpm run -C lib translations:extract
  pnpm run -C lib translations:build

docs:
  just pnpm lib
  just pnpm docs
  
  just generate lib

  pnpm run -C docs docs:build

#
# Dashboard
#

dashboard: lib
  just pnpm dashboard
  just generate dashboard

  pnpm run -C dashboard build

dashboard-dev: docker oidc-server-mock postgres && cleanup
  just backend postgres
  NUXT_PUBLIC_FEEDBACK_FUSION_ENDPOINT="http://localhost:8000" \
    FEEDBACK_FUSION_OIDC_PROVIDER_AUTHORIZATION_URL="http://localhost:5151/connect/authorize" \
    FEEDBACK_FUSION_OIDC_PROVIDER_TOKEN_URL="http://localhost:5151/connect/token" \
    FEEDBACK_FUSION_OIDC_CLIENT_ID="client" \
    FEEDBACK_FUSION_OIDC_CLIENT_SECRET="secret" \
    FEEDBACK_FUSION_OIDC_REDIRECT_URL="http://localhost:3000/auth/oidc/callback" \
    FEEDBACK_FUSION_OIDC_PROVIDER_DISCOVERY_URL="http://localhost:5151/.well-known/openid-configuration" \
    pnpm run -C dashboard dev

#
# Helm
#

helm:
  cd charts/feedback-fusion && helm-docs
  cp charts/feedback-fusion/README.md docs/docs/deployment/helm.md
