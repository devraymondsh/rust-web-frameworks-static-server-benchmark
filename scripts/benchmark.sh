#!/bin/sh

FIBO_DESTINATION=20

echoerr() {
    printf "%s\n" "$*" >&2
}

bunchmark() {
    sleep 3

    echo "Benchmarking $1..."

    # Running the process in background
    ./binaries/"$1" &

    # Store the running background process id in a local variable
    FRAMEWORK_PID=$!

    # Wait for the framework to start up
    WRK_WAITING_ATTEMPT=1
    # shellcheck disable=SC1083
    while [ "$(curl -s -o /dev/null -w ''%{http_code}'' localhost:9852/1)" != "200" ]; do
        sleep 1

        WRK_WAITING_ATTEMPT=$((WRK_WAITING_ATTEMPT + 1))

        if [ $WRK_WAITING_ATTEMPT -gt 60 ]; then
            echoerr "The framework faild to start up at localhost:9852. Startup waiting limit reached."
            break
        fi
    done

    # Benchmarking with wrk
    wrk --latency -t4 -c200 -d8s "http://localhost:9852/$FIBO_DESTINATION"

    # Kill the background process
    KILL_ATTEMPT=0
    while [ -e /proc/$FRAMEWORK_PID/status ]; do
        kill $FRAMEWORK_PID

        sleep 1

        KILL_ATTEMPT=$((KILL_ATTEMPT + 1))

        if [ $KILL_ATTEMPT -gt 4 ]; then
            echo "Attempt $KILL_ATTEMPT to kill the $1 framework process (pid: $FRAMEWORK_PID)."
        fi

        if [ $WRK_WAITING_ATTEMPT -gt 60 ]; then
            echoerr "The attempts faild to kill the process $FRAMEWORK_PID. Killing attempt limit reached."
            break
        fi
    done

    # print a break line
    echo ""
}

compile() {
    echo "Compiling $1..."

    cargo build --release --manifest-path="/rust_web_frameworks_benchmark/frameworks/$1/Cargo.toml"
}

loop_through_frameworks() {
    frameworks_josn=$(cat /rust_web_frameworks_benchmark/scripts/frameworks.json)

    for row in $(echo "${frameworks_josn}" | jq -r '.[] | @base64'); do
        _jq() {
            echo "${row}" | base64 --decode | jq -r "${1}"
        }

        NAME=$(_jq '.name')

        $1 "$NAME"
    done
}

benchmark_all() {
    DATE=$(date)

    echo "Benchmarking started at $DATE..."
    # print a break line
    echo ""

    loop_through_frameworks bunchmark
}

gather_binaries() {
    cp -f ./frameworks/"$1"/target/release/benchmark ./binaries/"$1"
}

main() {
    if [ "$SCRIPT_ACTION" = "compile" ]; then
        loop_through_frameworks compile

        mkdir -p binaries

        loop_through_frameworks gather_binaries
    fi

    if [ "$SCRIPT_ACTION" = "benchmark" ]; then
        if [ -d "logs" ]; then
            benchmark_all | tee -a "logs/benchmark__$(date +"%y-%m-%d_%H-%M").log"
        else
            benchmark_all
        fi
    fi
}

main
