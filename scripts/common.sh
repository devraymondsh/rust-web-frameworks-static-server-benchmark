#!/bin/sh

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

echoerr() {
    printf "%s\n" "$*" >&2
}