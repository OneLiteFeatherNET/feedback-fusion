FROM rust:slim as build

COPY ./Cargo.toml . 
COPY ./Cargo.lock . 
COPY ./proto ./proto
COPY ./common ./common
COPY ./codegen ./codegen
COPY ./src ./src
COPY ./tests ./tests
COPY ./rust-toolchain.toml .

RUN apt-get update \ 
  && apt-get install libssl-dev protobuf-compiler libprotobuf-dev pkg-config -y --no-install-recommends \
  && apt-get clean\ 
  && cargo test --release --no-run --test integration_test \
  && rm target/release/deps/integration_test*.d

FROM gcr.io/distroless/cc-debian12

COPY --from=build ./target/release/deps/integration_test* ./integration_test

ENTRYPOINT ["./integration_test"]
