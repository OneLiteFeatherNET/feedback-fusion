FROM --platform=linux/amd64 node:22-bookworm-slim AS build-ssr

RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev make && npm i -g pnpm && mkdir build

WORKDIR build

COPY ./dashboard ./dashboard
COPY ./proto ./proto
COPY ./lib ./lib
COPY ./Makefile .

RUN make dashboard_build

FROM gcr.io/distroless/nodejs22-debian12

COPY --from=build-ssr /build/dashboard/.output/ ./.output

CMD [".output/server/index.mjs"]
