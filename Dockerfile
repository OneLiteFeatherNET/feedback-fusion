FROM rust:slim as build

ARG features=all-databases

RUN apt-get update 
RUN apt-get install libssl-dev protobuf-compiler -y

# set toolchain 
RUN rustup default nightly 

COPY ./Cargo.toml . 
COPY ./Cargo.lock . 
COPY ./proto ./proto
COPY ./common ./common
COPY ./codegen ./codegen
COPY ./src ./src

RUN cargo build --release --features $features

FROM gcr.io/distroless/cc-debian12

COPY --from=build ./target/release/feedback-fusion .

ENTRYPOINT ["./feedback-fusion"]
