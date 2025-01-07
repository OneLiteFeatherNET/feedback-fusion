FROM rust:slim AS build

RUN apt-get update \ 
  && apt-get install libssl-dev protobuf-compiler libprotobuf-dev pkg-config -y --no-install-recommends \
  && apt-get clean \
  && rustup toolchain install stable \
  && rustup default stable

ARG features=all-databases,otlp

COPY ./Cargo.toml . 
COPY ./Cargo.lock . 
COPY ./proto ./proto
COPY ./common ./common
COPY ./codegen ./codegen
COPY ./src ./src
COPY ./benches ./benches

RUN cargo build --release --features $features

FROM gcr.io/distroless/cc-debian12

COPY --from=build ./target/release/feedback-fusion .

ENTRYPOINT ["./feedback-fusion"]
