FROM node:22-bookworm-slim AS build-ssr

RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev make && npm i -g pnpm

COPY ./dashboard/ ./dashboard
COPY ./proto ./proto
COPY ./Makefile .

RUN make dashboard_build

FROM gcr.io/distroless/nodejs22-debian12

COPY --from=build-ssr /dashboard/.output/ ./.output

CMD [".output/server/index.mjs"]
