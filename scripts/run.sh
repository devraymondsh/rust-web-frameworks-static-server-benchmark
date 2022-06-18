#!/bin/bash

mkdir -p benchmarking_log

docker build -t devraymondsh/rust_web_frameworks_benchmark .

echo ""

if [ -z "$CARGO_HOME" ]; then
    CARGO_HOME="$HOME/.cargo"
fi

DOCKER_RUN_PARAMS=(
    --rm
    -it
    -e CARGO_HOME="/.cargo"
    -v="$CARGO_HOME:/.cargo"
    -v="$(pwd)/scripts:/rust_web_frameworks_benchmark/scripts"
    -v="$(pwd)/frameworks:/rust_web_frameworks_benchmark/frameworks"
    -v="$(pwd)/benchmarking_log:/rust_web_frameworks_benchmark/benchmarking_log"
    devraymondsh/rust_web_frameworks_benchmark
)

# shellcheck disable=SC2068
docker run ${DOCKER_RUN_PARAMS[@]}

# Fix ownership issues
sudo chown "$USER" -R "$CARGO_HOME"
sudo chown "$USER" -R "$(pwd)"
