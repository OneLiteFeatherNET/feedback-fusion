.PHONY: pnpm core_generate docs lib_build docs_build extract_translations build_translations \
        check clippy unittest docker_network oidc-server-mock integration_test \
        postgres_database postgres_backend cleanup postgres \
        mysql_database mysql_backend mysql \
        mariadb_database mariadb \
        mssql_database mssql_backend mssql \
        integration skytable

pnpm:
	cd ./lib && pnpm i 
	cd ./docs && pnpm i

core_generate: pnpm
	cd ./lib && pnpm run protoc

docs: core_generate
	cd ./lib && pnpm run dev

lib_build: core_generate
	cd ./lib && pnpm run build

docs_build: lib_build pnpm_docs
	cd ./docs && pnpm run docs:build

extract_translations: pnpm
	cd ./lib && pnpm run translations:extract

build_translations: pnpm
	cd ./lib && pnpm run translations:build

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
		-e SKYTABLE_HOST=skytable \
		-e SKYTABLE_PORT=2003 \
		-e SKYTABLE_USERNAME=root \
		-e SKYTABLE_PASSWORD=passwordpassword \
		-e POSTGRES_USERNAME=postgres \
		-e POSTGRES_PASSWORD=password \
		-e POSTGRES_DATABASE=postgres \
		-e POSTGRES_ENDPOINT=database:5432 \
		-e OIDC_PROVIDER=http://oidc-server-mock \
		-e OIDC_ISSUER=http://localhost:5151 \
		-e RUST_LOG=DEBUG \
		--network feedback-fusion -p 8000:8000 feedback-fusion
	sleep 1

distributed_caching: cleanup docker_network skytable oidc-server-mock postgres_database distributed_caching_backend integration_test 
	${MAKE} cleanup


postgres_backend:
	docker build -t feedback-fusion .
	docker run --name feedback-fusion -d \
		-e POSTGRES_USERNAME=postgres \
		-e POSTGRES_PASSWORD=password \
		-e POSTGRES_DATABASE=postgres \
		-e POSTGRES_ENDPOINT=database:5432 \
		-e OIDC_PROVIDER=http://oidc-server-mock \
		-e OIDC_ISSUER=http://localhost:5151 \
		-e RUST_LOG=DEBUG \
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
		-e MYSQL_USERNAME=username \
		-e MYSQL_PASSWORD=password \
		-e MYSQL_DATABASE=database \
		-e MYSQL_ENDPOINT=database:3306 \
		-e OIDC_PROVIDER=http://oidc-server-mock \
		-e OIDC_ISSUER=http://localhost:5151 \
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
  	-e MSSQL_USERNAME=sa \
  	-e MSSQL_PASSWORD=Password1 \
  	-e MSSQL_DATABASE=master \
  	-e MSSQL_ENDPOINT=database:1433 \
  	-e OIDC_PROVIDER=http://oidc-server-mock \
  	-e OIDC_ISSUER=http://localhost:5151 \
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
	${MAKE} distributed_caching_backend
