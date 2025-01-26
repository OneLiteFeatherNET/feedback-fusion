FROM --platform=linux/amd64 rust:slim AS build

COPY ./rust-toolchain.toml ./rust-toolchain.toml

RUN apt-get update \ 
  && apt-get install build-essential libssl-dev pkg-config libprotobuf-dev protobuf-compiler gcc-aarch64-linux-gnu libc6-dev-arm64-cross -y \
  && rustup update \
  && rustup target add aarch64-unknown-linux-gnu \
  && mkdir build

WORKDIR build

ARG features=all-databases,otlp,caching-skytable

RUN USER=root cargo init --bin --name feedback-fusion
RUN USER=root cargo init --lib --name feedback_fusion_common common
RUN USER=root cargo init --lib --name feedback_fusion_codegen codegen

COPY ./.cargo ./.cargo
COPY ./Cargo.toml . 
COPY ./Cargo.lock .
COPY ./proto ./proto
COPY ./common ./common
COPY ./codegen ./codegen
COPY ./fuzz ./fuzz
COPY ./benches ./benches

ARG TARGETARCH

RUN if [ "$TARGETARCH" = "arm64" ]; then \
        cargo build --release --target aarch64-unknown-linux-gnu --features $features; \
    else \
        cargo build --release --features $features; \
    fi

RUN rm -Rf ./src
COPY ./src ./src

# for some reason cargo does not detect the file change
RUN touch src/main.rs

RUN if [ "$TARGETARCH" = "arm64" ]; then \
        cargo build --release --target aarch64-unknown-linux-gnu --features $features; \
        mv target/aarch64-unknown-linux-gnu/release/feedback-fusion target/release/feedback-fusion; \
    else \
        cargo build --release --features $features; \
    fi

FROM gcr.io/distroless/cc-debian12

COPY --from=build ./build/target/release/feedback-fusion .

ENTRYPOINT ["./feedback-fusion"]
