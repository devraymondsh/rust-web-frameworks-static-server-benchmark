#!/bin/sh

. ./scripts/common.sh

compile() {
    echo "Compiling $1..."

    cargo build --release --manifest-path="/rust_web_frameworks_benchmark/frameworks/$1/Cargo.toml"
}

gather_binaries() {
    cp -f ./frameworks/"$1"/target/release/benchmark ./binaries/"$1"
}

compile_all() {
    loop_through_frameworks compile

    mkdir -p binaries

    loop_through_frameworks gather_binaries
}

compile_all