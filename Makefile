.PHONY: fuzz

core_generate:
	pnpm i -C lib
	pnpm run -C lib protoc

docs: core_generate
	pnpm i -C docs
	pnpm run -C lib dev

lib_build: core_generate
	pnpm run -C lib build

helm_docs:
	cd charts/feedback-fusion && helm-docs
	cp charts/feedback-fusion/README.md docs/docs/deployment/helm.md

docs_build: lib_build
	pnpm i -C docs
	pnpm run -C docs docs:build

extract_translations:
	pnpm run -C lib translations:extract

build_translations:
	pnpm run -C lib translations:build

# dashboard 

dashboard_setup:
	pnpm i -C dashboard

dashboard_protoc: dashboard_setup
	pnpm run -C dashboard protoc

dashboard_build: lib_build dashboard_protoc
	pnpm run -C dashboard build
	 
dashboard: cleanup docker_network oidc-server-mock postgres_database postgres_backend lib_build dashboard_protoc
		NUXT_PUBLIC_FEEDBACK_FUSION_ENDPOINT="http://localhost:8000" \
		FEEDBACK_FUSION_OIDC_PROVIDER_AUTHORIZATION_URL="http://localhost:5151/connect/authorize" \
		FEEDBACK_FUSION_OIDC_PROVIDER_TOKEN_URL="http://localhost:5151/connect/token" \
		FEEDBACK_FUSION_OIDC_CLIENT_ID="client" \
		FEEDBACK_FUSION_OIDC_CLIENT_SECRET="secret" \
		FEEDBACK_FUSION_OIDC_REDIRECT_URL="http://localhost:3000/auth/oidc/callback" \
		FEEDBACK_FUSION_OIDC_PROVIDER_DISCOVERY_URL="http://localhost:5151/.well-known/openid-configuration" \
		pnpm run -C dashboard dev
	${MAKE} cleanup

dashboard_lint: dashboard_protoc 
	pnpm run -C dashboard lint

# linting

check:
	cargo check

clippy:
	cargo clippy -- -D warnings

unittest:
	cargo test --bin feedback-fusion

docker_network:
	docker network rm feedback-fusion; docker network create feedback-fusion

skytable:
	docker run \
		-p 2003:2003 \
		--entrypoint skyd \
		--rm --name skytable \
		--network feedback-fusion \
		-d skytable/skytable \
		--auth-root-password=passwordpassword \
		--endpoint=tcp@0.0.0.0:2003

# run oidc server for integration tests
oidc-server-mock: 
	docker compose -f tests/_common/oidc-mock/docker-compose.yaml up -d 
	sleep 5
	curl -s -o /dev/null http://localhost:5151/.well-known/openid-configuration

integration_test:
	OIDC_PROVIDER="http://localhost:5151" OIDC_CLIENT_ID="client" OIDC_CLIENT_SECRET="secret" RUST_LOG="INFO" GRPC_ENDPOINT="http://localhost:8000" \
	cargo test --no-fail-fast --test integration_test

fuzz:
	OIDC_PROVIDER="http://localhost:5151" OIDC_CLIENT_ID="client" OIDC_CLIENT_SECRET="secret" RUST_LOG="INFO" GRPC_ENDPOINT="http://localhost:8000" cargo fuzz run fuzz_create_and_export

cleanup:
	docker rm -f database;docker rm -f oidc-server-mock;docker rm -f feedback-fusion;docker rm -f skytable;docker network rm feedback-fusion; echo ""

bench: cleanup docker_network oidc-server-mock postgres_database postgres_backend 
	GRPC_ENDPOINT=http://localhost:8000 OIDC_CLIENT_ID=client OIDC_CLIENT_SECRET=secret OIDC_PROVIDER=http://localhost:5151 cargo bench
	${MAKE} cleanup

# Postgres
postgres_database:
	docker run --name database -e POSTGRES_PASSWORD=password -e POSTGRES_USERNAME=postgres --network feedback-fusion -d postgres
	sleep 1

distributed_caching_backend:
	docker build -t feedback-fusion .
	docker run --name feedback-fusion -d \
		-e FEEDBACK_FUSION_CONFIG="/etc/feedback-fusion/config.yaml" \
		-v ./tests/_common/configs/skytable.yaml:/etc/feedback-fusion/config.yaml \
		-e RUST_LOG=DEBUG \
		--network feedback-fusion -p 8000:8000 feedback-fusion
	sleep 1

distributed_caching: cleanup docker_network skytable oidc-server-mock postgres_database distributed_caching_backend integration_test 
	${MAKE} cleanup

postgres_backend:
	docker build -t feedback-fusion .
	docker run --name feedback-fusion -d \
		-e RUST_LOG=DEBUG \
		-e FEEDBACK_FUSION_CONFIG="/etc/feedback-fusion/config.yaml" \
		-v ./tests/_common/configs/postgres.yaml:/etc/feedback-fusion/config.yaml \
		--network feedback-fusion -p 8000:8000 feedback-fusion
	sleep 1

postgres: cleanup docker_network oidc-server-mock postgres_database postgres_backend integration_test 
	${MAKE} cleanup

# Mysql
mysql_database:
	docker run --name database \
		-e MYSQL_ROOT_PASSWORD=password \
		-e MYSQL_PASSWORD=password \
		-e MYSQL_USER=username \
		-e MYSQL_DATABASE=database \
		--network feedback-fusion \
		-d mysql
	sleep 30

mysql_backend:
	docker build -t feedback-fusion .
	docker run --name feedback-fusion -d \
		-e FEEDBACK_FUSION_CONFIG="/etc/feedback-fusion/config.yaml" \
		-v ./tests/_common/configs/mysql.yaml:/etc/feedback-fusion/config.yaml \
		-e RUST_LOG=DEBUG \
		--network feedback-fusion -p 8000:8000 feedback-fusion
	sleep 1

mysql: cleanup docker_network oidc-server-mock mysql_database mysql_backend integration_test
	${MAKE} cleanup

# Maria
mariadb_database:
	docker run --name database \
		-e MYSQL_ROOT_PASSWORD=password \
		-e MYSQL_PASSWORD=password \
		-e MYSQL_USER=username \
		-e MYSQL_DATABASE=database \
		--network feedback-fusion \
		-d mariadb
	sleep 10

mariadb: cleanup docker_network oidc-server-mock mariadb_database mysql_backend integration_test
	${MAKE} cleanup

# Mssql
mssql_database:
	docker run --name database \
  	-e ACCEPT_EULA=Y \
  	-e MSSQL_SA_PASSWORD=Password1 \
  	--network feedback-fusion \
  	-d mcr.microsoft.com/mssql/server:2022-latest
	sleep 10

mssql_backend:
	docker build -t feedback-fusion .
	docker run --name feedback-fusion -d \
		-e FEEDBACK_FUSION_CONFIG="/etc/feedback-fusion/config.yaml" \
		-v ./tests/_common/configs/mssql.yaml:/etc/feedback-fusion/config.yaml \
  	-e RUST_LOG=DEBUG \
  	--network feedback-fusion -p 8000:8000 feedback-fusion
	sleep 1

mssql: cleanup docker_network oidc-server-mock mssql_database mssql_backend integration_test
	${MAKE} cleanup

integration: 
	${MAKE} postgres
	${MAKE} mariadb
	${MAKE} mysql
	${MAKE} mssql
	${MAKE} distributed_caching
