#!/bin/sh

. ./scripts/common.sh

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
    while [ "$(curl -s -o /dev/null -w ''%{http_code}'' localhost:9852/index.html)" != "200" ]; do
        sleep 1

        WRK_WAITING_ATTEMPT=$((WRK_WAITING_ATTEMPT + 1))

        if [ $WRK_WAITING_ATTEMPT -gt 120 ]; then
            echoerr "The framework faild to start up at localhost:9852. Startup waiting limit reached."
            break
        fi
    done

    # Benchmarking with wrk
    wrk2 -t"$THREADS" -c"$CLIENTS" -d"$DURATION"s -R"$RATE" "http://localhost:9852/$FILENAME"

    # Kill the background process
    KILL_ATTEMPT=0
    while [ -e /proc/$FRAMEWORK_PID/status ]; do
        kill $FRAMEWORK_PID

        sleep 1

        KILL_ATTEMPT=$((KILL_ATTEMPT + 1))

        if [ $KILL_ATTEMPT -gt 4 ]; then
            echo "Attempt $KILL_ATTEMPT to kill the $1 framework process (pid: $FRAMEWORK_PID)."
        fi

        if [ $WRK_WAITING_ATTEMPT -gt 120 ]; then
            echoerr "The attempts faild to kill the process $FRAMEWORK_PID. Killing attempt limit reached."
            break
        fi
    done

    # print a break line
    echo ""
}

benchmark_all() {
    DATE=$(date)

    echo "Benchmarking started at $DATE..."

    # print a break line
    echo ""

    loop_through_frameworks bunchmark
}

if [ -d "logs" ]; then
    benchmark_all | tee -a "logs/benchmark__$(date +"%y-%m-%d_%H-%M").log"
else
    benchmark_all
fi
