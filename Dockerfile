FROM devraymondsh/ubuntu-docker-rust:latest AS build

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get -y upgrade
RUN apt-get -y --no-install-recommends install jq openssl pkg-config libssl-dev

WORKDIR /rust_web_frameworks_benchmark

COPY static /rust_web_frameworks_benchmark/static
COPY frameworks /rust_web_frameworks_benchmark/frameworks
COPY scripts/frameworks.json /rust_web_frameworks_benchmark/scripts/frameworks.json
COPY scripts/common.sh /rust_web_frameworks_benchmark/scripts/common.sh
COPY scripts/compile.sh /rust_web_frameworks_benchmark/scripts/compile.sh

RUN chmod +x scripts/*.sh
RUN [ "./scripts/compile.sh" ]

FROM ubuntu:latest AS app

ENV DEBIAN_FRONTEND=noninteractive

RUN echo "deb http://security.ubuntu.com/ubuntu focal-security main" | tee /etc/apt/sources.list.d/focal-security.list
RUN apt-get update && apt-get -y upgrade
RUN apt-get -y --no-install-recommends install openssl ca-certificates pkg-config manpages-dev curl jq libssl1.1
RUN update-ca-certificates

RUN curl -L -O https://github.com/hnakamur/wrk2-deb/releases/download/debian%2F0.20190924.git44a94c1-1ppa1_bionic/wrk2_0.20190924.git44a94c1-1ppa1.bionic_amd64.deb
RUN chmod +x *.deb && dpkg -i *.deb

WORKDIR /rust_web_frameworks_benchmark

RUN mkdir -p /rust_web_frameworks_benchmark/logs

COPY --from=build /rust_web_frameworks_benchmark/binaries binaries
COPY scripts/frameworks.json /rust_web_frameworks_benchmark/scripts/frameworks.json
COPY scripts/common.sh /rust_web_frameworks_benchmark/scripts/common.sh
COPY scripts/benchmark.sh /rust_web_frameworks_benchmark/scripts/benchmark.sh

RUN chmod +x scripts/*.sh

ENV THREADS=8
ENV RATE=50000
ENV DURATION=10
ENV CLIENTS=200
ENV FILENAME="index.html"
ENTRYPOINT [ "./scripts/benchmark.sh" ]
