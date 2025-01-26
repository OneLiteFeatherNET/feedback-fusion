FROM --platform=linux/amd64 node:22-bookworm-slim AS build-ssr

RUN apt-get update && apt-get install -y wget gpg sudo lsb-release && wget -qO - 'https://proget.makedeb.org/debian-feeds/prebuilt-mpr.pub' | gpg --dearmor | tee /usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg 1> /dev/null \
  && echo "deb [arch=all,$(dpkg --print-architecture) signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://proget.makedeb.org prebuilt-mpr $(lsb_release -cs)" | sudo tee /etc/apt/sources.list.d/prebuilt-mpr.list \
  && apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev git just && npm i -g pnpm && mkdir build

WORKDIR build

COPY ./dashboard ./dashboard
COPY ./proto ./proto
COPY ./lib ./lib
COPY ./justfile .

RUN just dashboard

FROM gcr.io/distroless/nodejs22-debian12

COPY --from=build-ssr /build/dashboard/.output/ ./.output

CMD [".output/server/index.mjs"]
