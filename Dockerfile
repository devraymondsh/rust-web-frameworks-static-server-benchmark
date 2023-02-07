# syntax=docker/dockerfile:1

FROM devraymondsh/ubuntu-docker-rust:latest AS build

RUN apt-get update && apt-get -y upgrade && apt-get -y install jq openssl pkg-config libssl-dev

WORKDIR /rust_web_frameworks_benchmark

COPY . /rust_web_frameworks_benchmark

ENV SCRIPT_ACTION=compile
RUN [ "./scripts/benchmark.sh" ]

FROM ubuntu:latest AS app

RUN apt-get update && apt-get -y upgrade && apt-get -y install  openssl pkg-config libssl-dev wrk manpages-dev curl jq

WORKDIR /rust_web_frameworks_benchmark

RUN mkdir -p /rust_web_frameworks_benchmark/logs

COPY --from=build /rust_web_frameworks_benchmark/binaries binaries
COPY --from=build /rust_web_frameworks_benchmark/scripts/frameworks.json scripts/frameworks.json
COPY --from=build /rust_web_frameworks_benchmark/scripts/benchmark.sh scripts/benchmark.sh

ENV SCRIPT_ACTION=benchmark
ENTRYPOINT [ "/rust_web_frameworks_benchmark/scripts/benchmark.sh" ]
