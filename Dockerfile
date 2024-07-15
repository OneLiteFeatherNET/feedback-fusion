FROM rust:slim as build

ARG features=all-databases,otlp

COPY ./Cargo.toml . 
COPY ./Cargo.lock . 
COPY ./proto ./proto
COPY ./common ./common
COPY ./codegen ./codegen
COPY ./src ./src
COPY ./benches ./benches
COPY ./rust-toolchain.toml .

RUN apt-get update \ 
  && apt-get install libssl-dev protobuf-compiler libprotobuf-dev -y --no-install-recommends \
  && apt-get clean \
  && cargo build --release --features $features

FROM gcr.io/distroless/cc-debian12

COPY --from=build ./target/release/feedback-fusion .

ENTRYPOINT ["./feedback-fusion"]
