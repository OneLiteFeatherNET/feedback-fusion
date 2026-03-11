FROM --platform=linux/amd64 node:22-trixie-slim AS build-ssr

RUN apt-get update \
  && apt-get install -y wget gpg sudo lsb-release just git protobuf-compiler libprotobuf-dev \
  && npm i -g bun \
  && mkdir build

WORKDIR build

COPY ./packages ./packages
COPY ./proto ./proto
COPY ./justfile .
Copy ./package.json .

RUN just dashboard

FROM gcr.io/distroless/nodejs22-debian13

COPY --from=build-ssr /build/packages/dashboard/.output/ ./.output

CMD [".output/server/index.mjs"]
