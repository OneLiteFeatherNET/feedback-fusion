FROM --platform=$BUILDPLATFORM rust:slim AS build
ARG BUILDPLATFORM
ARG TARGETARCH

COPY ./rust-toolchain.toml ./rust-toolchain.toml

RUN apt-get update \ 
  && apt-get install build-essential libssl-dev pkg-config libprotobuf-dev protobuf-compiler gcc-aarch64-linux-gnu libc6-dev-arm64-cross -y \
  && rustup update \
  && rustup target add aarch64-unknown-linux-gnu \
  && mkdir build

WORKDIR build

COPY ./.cargo ./.cargo
COPY ./Cargo.toml . 
COPY ./Cargo.lock .
COPY ./proto ./proto
COPY ./common ./common
COPY ./codegen ./codegen
COPY ./fuzz ./fuzz
COPY ./benches ./benches
COPY ./src ./src
COPY ./indexer ./indexer

RUN if [ "$TARGETARCH" = "arm64" ]; then \
        cargo build --bin indexer --release --target aarch64-unknown-linux-gnu; \
        mv target/aarch64-unknown-linux-gnu/release/indexer target/release/indexer; \
    else \
        cargo build --bin indexer --release; \
    fi

FROM gcr.io/distroless/cc-debian12

COPY --from=build ./build/target/release/indexer .

ENTRYPOINT ["./indexer"]
