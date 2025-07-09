DOCKER_NETWORK := "feedback-fusion"
LOCAL_DOCKER_IMAGE := "feedback-fusion"
LOCAL_PLATFORM := "linux/" + replace(replace(arch(), "x86_64", "amd64"), "aarch64", "arm64") 
DEFAULT_TEST := "postgres"

test-all: cleanup
  cargo llvm-cov clean --workspace

  just unittest
  just test postgres
  just test mariadb
  just test mysql
  just test mssql
  just test skytable

  cargo llvm-cov report
  cargo llvm-cov report --lcov --output-path coverage.info

init:
  pre-commit install

#
# Linting
#

check:
  cargo check --all-features --workspace

clippy:
  cargo clippy --all-features --workspace -- -D warnings

#
# Docker
#

build PLATFORM=LOCAL_PLATFORM DOCKERFILE="./Dockerfile":
  @echo "building for {{PLATFORM}}"
  docker buildx build -t {{LOCAL_DOCKER_IMAGE}} --platform {{PLATFORM}} -f {{DOCKERFILE}} --load .

build-all DOCKERFILE="./Dockerfile":
  just build linux/arm64,linux/amd64 {{DOCKERFILE}}

#
# Backend
#

backend TYPE=DEFAULT_TEST:
  FEEDBACK_FUSION_CONFIG="./tests/_common/configs/{{TYPE}}.hcl" RUST_LOG=DEBUG cargo llvm-cov run --no-report > ./target/feedback-fusion.log 2>&1 &

  while ! nc -z localhost 8000; do \
    sleep 1; \
  done
  @echo "Application ready"

stop-backend:
  @PID=$(lsof -t -i:8000) && if [ -n "$PID" ]; then \
    kill -2 $PID; \
  fi

bench: oidc-server-mock postgres && cleanup
  just backend postgres
  GRPC_ENDPOINT=http://localhost:8000 OIDC_CLIENT_ID=client OIDC_CLIENT_SECRET=secret OIDC_PROVIDER=http://localhost:5151 cargo bench

protoc-docs:
  docker run --rm -v ./docs/docs/reference:/out -v ./proto:/protos  pseudomuto/protoc-gen-doc --doc_opt=markdown,api.md

#
# Testing requirements
#

@oidc-server-mock:
  docker compose -f tests/_common/oidc-mock/docker-compose.yaml up -d 
  @sleep 5
  curl -s -o /dev/null http://localhost:5151/.well-known/openid-configuration

@postgres:
  docker run --name database -e POSTGRES_PASSWORD=password -e POSTGRES_USERNAME=postgres -p 5150:5432 -d postgres
  sleep 5

@mysql:
  docker run --name database -e MYSQL_ROOT_PASSWORD=password -e MYSQL_PASSWORD=password -e MYSQL_USER=username -e MYSQL_DATABASE=database -p 5150:3306 -d mysql
  sleep 30

@mariadb:
  docker run --name database -e MYSQL_ROOT_PASSWORD=password -e MYSQL_PASSWORD=password -e MYSQL_USER=username -e MYSQL_DATABASE=database -p 5150:3306 -d mariadb
  sleep 10

@mssql:
  docker run --name database -e ACCEPT_EULA=Y -e MSSQL_SA_PASSWORD=Password1 -p 5150:1433 -d mcr.microsoft.com/mssql/server:2022-latest
  sleep 10

@skytable: postgres
  docker run -p 2003:2003 --entrypoint skyd --rm --name skytable -d skytable/skytable --auth-root-password=passwordpassword --endpoint=tcp@0.0.0.0:2003

# 
# Testing
#

@cleanup:
  -docker rm -f database > /dev/null 2>&1
  -docker rm -f oidc-server-mock > /dev/null 2>&1
  -just stop-backend
  -docker rm -f skytable > /dev/null 2>&1
  -docker network rm {{DOCKER_NETWORK}} > /dev/null 2>&1

unittest:
  FEEDBACK_FUSION_CONFIG="./tests/_common/configs/postgres.hcl" cargo llvm-cov --bin feedback-fusion --no-report -- --nocapture

integration:
  OIDC_PROVIDER="http://localhost:5151" OIDC_CLIENT_ID="client" OIDC_CLIENT_SECRET="secret" RUST_LOG="INFO" GRPC_ENDPOINT="http://localhost:8000" cargo llvm-cov --no-report --no-fail-fast --test integration_test || (cat ./target/feedback-fusion.log; just stop-backend; cargo llvm-cov report; exit 1)

test TYPE=DEFAULT_TEST: cleanup oidc-server-mock
  just {{TYPE}}
  @if [ "{{TYPE}}" = "mariadb" ]; then just backend mysql; else just backend {{TYPE}}; fi

  just integration

  just stop-backend
  -docker rm -f database > /dev/null 2>&1

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
  just lib
  just pnpm docs
  
  pnpm run -C docs docs:build

#
# Dashboard
#

dashboard: lib
  just pnpm dashboard
  just generate dashboard

  pnpm run -C dashboard build

dashboard-dev: lib cleanup oidc-server-mock postgres && cleanup
  just backend postgres
  just generate dashboard
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

#
# Releases
#

prepare-release:
  git checkout main
  git pull

post-prepare-release TAG:
  git checkout -b release/{{TAG}}
  git add -A 
  -git commit -m "chore: release {{TAG}}"
  git push origin -u HEAD 

release-server LEVEL:
  just prepare-release

  cargo release --no-publish --no-push --no-tag --execute {{LEVEL}} 

  just post-prepare-release server-$(cargo pkgid | sed -n 's/.*#//p')
 
release-dashboard LEVEL:
  just prepare-release

  just post-prepare-release dashboard-$(pnpm version -C dashboard --no-git-tag-version {{LEVEL}} | sed 's/^v//')

release-lib LEVEL:
  just prepare-release

  just post-prepare-release dashboard-$(pnpm version -C lib --no-git-tag-version {{LEVEL}}| sed 's/^v//')
