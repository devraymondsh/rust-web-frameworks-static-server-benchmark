FROM rust:1.61.0-slim

RUN apt-get update && apt-get -y upgrade && apt-get install -y openssl pkg-config libssl-dev wrk build-essential manpages-dev curl jq

WORKDIR /rust_web_frameworks_benchmark

RUN mkdir scripts

ENTRYPOINT [ "/bin/bash", "-c", "scripts/benchmark.sh" ]