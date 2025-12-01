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
  just test fluvio

  cargo llvm-cov report
  cargo llvm-cov report --lcov --output-path coverage.info

init:
  pre-commit install

#
# Linting
#

check:
  cargo check --all-features --workspace --tests

clippy:
  cargo clippy --all-features --workspace -- -D warnings

#
# Backend
#

backend TYPE=DEFAULT_TEST:
  FEEDBACK_FUSION_CONFIG="./tests/_common/configs/indexer/{{TYPE}}.hcl" RUST_LOG=DEBUG cargo llvm-cov run --bin indexer --no-report > ./target/feedback-fusion-indexer.log 2>&1 &

  while ! nc -z localhost 8080; do \
    sleep 1; \
  done
  @echo "Indexer ready"

  FEEDBACK_FUSION_CONFIG="./tests/_common/configs/{{TYPE}}.hcl" RUST_LOG=DEBUG cargo llvm-cov run --bin feedback-fusion --no-report > ./target/feedback-fusion.log 2>&1 &

  while ! nc -z localhost 8000; do \
    sleep 1; \
  done
  @echo "Application ready"

stop-backend:
  -@PID=$(lsof -t -i:8000) && if [ -n "$PID" ]; then \
    kill -2 $PID; \
  fi

  -@PID=$(lsof -t -i:8080) && if [ -n "$PID" ]; then \
    kill -2 $PID; \
  fi

bench: oidc-server-mock postgres && cleanup
  just backend postgres
  GRPC_ENDPOINT=http://localhost:8000 OIDC_CLIENT_ID=client OIDC_CLIENT_SECRET=secret OIDC_PROVIDER=http://localhost:5151 cargo bench

protoc-docs:
	docker run --rm -v ./packages/docs/docs/reference:/out -v ./proto:/protos pseudomuto/protoc-gen-doc --proto_path=/protos --doc_opt=markdown,api.md $(find ./proto -name "*.proto" | sed 's|^\./proto/||')


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

@fluvio: postgres
  fluvio cluster start

  fluvio topic create feedback-fusion 

@stop-fluvio:
  #!/usr/bin/expect -f

  spawn fluvio cluster delete
  expect "Please type the cluster name to confirm:"
  send "local\r"
  expect eof

# 
# Testing
#

@cleanup:
  -docker rm -f database > /dev/null 2>&1
  -docker rm -f oidc-server-mock > /dev/null 2>&1
  -just stop-backend
  -docker rm -f skytable > /dev/null 2>&1
  -docker network rm {{DOCKER_NETWORK}} > /dev/null 2>&1
  -just stop-fluvio

unittest:
  FEEDBACK_FUSION_CONFIG="./tests/_common/configs/postgres.hcl" cargo llvm-cov --bin feedback-fusion --no-report -- --nocapture

integration:
  OIDC_PROVIDER="http://localhost:5151" OIDC_CLIENT_ID="client" OIDC_CLIENT_SECRET="secret" RUST_LOG="INFO" GRPC_ENDPOINT="http://localhost:8000" cargo llvm-cov --no-report --no-fail-fast --test integration_test

test TYPE=DEFAULT_TEST: cleanup oidc-server-mock
  just {{TYPE}}
  @if [ "{{TYPE}}" = "mariadb" ]; then just backend mysql; else just backend {{TYPE}}; fi

  just integration

  # a short sleep to process events
  sleep 2

  just stop-backend
  -docker rm -f database > /dev/null 2>&1

fuzz:
  OIDC_PROVIDER="http://localhost:5151" OIDC_CLIENT_ID="client" OIDC_CLIENT_SECRET="secret" RUST_LOG="INFO" GRPC_ENDPOINT="http://localhost:8000" cargo fuzz run fuzz_create_and_export

#
# Lib & Docs
#

bun PACKAGE:
  bun i --cwd packages/{{PACKAGE}}

generate PACKAGE:
  bun run --cwd packages/{{PACKAGE}} protoc

lint PACKAGE:
  bun run --cwd packages/{{PACKAGE}} lint

lib-dev:
  just bun docs
  just bun lib

  just generate lib
  bun run --cwd packages/lib dev

lib:
  just bun lib
  just generate lib

  bun run --cwd packages/lib build

translations:
  bun run --cwd packages/lib translations:extract
  bun run --cwd packages/lib translations:build

docs:
  just lib
  just bun docs
  
  bun run --cwd packages/docs docs:build

#
# Dashboard
#

dashboard: lib
  just bun dashboard
  just generate dashboard

  bun run --cwd packages/dashboard build

dashboard-dev: lib cleanup oidc-server-mock postgres && cleanup
  just backend postgres
  just generate dashboard
  NUXT_PUBLIC_FEEDBACK_FUSION_ENDPOINT="http://localhost:8000" \
    NUXT_AUTH_SECRET=secret \
    NUXT_CLIENT_ID=client \
    NUXT_CLIENT_SECRET=secret \
    NUXT_OIDC_DISCOVERY=http://localhost:5151/.well-known/openid-configuration \
    NUXT_SCOPE="openid profile test email" \
    bun run --cwd packages/dashboard dev

#
# Helm
#

helm:
  cd charts/feedback-fusion && helm-docs
  cp charts/feedback-fusion/README.md packages/docs/docs/deployment/helm.md

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

  just post-prepare-release dashboard-$(bun version --cwd packages/dashboard --no-git-tag-version {{LEVEL}} | sed 's/^v//')

release-lib LEVEL:
  just prepare-release

  just post-prepare-release dashboard-$(bun version --cwd packages/lib --no-git-tag-version {{LEVEL}}| sed 's/^v//')
