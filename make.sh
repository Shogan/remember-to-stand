#!/bin/bash

DIR="$(dirname "$0")"

if cargo "$@"; then
    [ -d "$DIR/target/debug" ] && cp -a "$DIR/resources/." "$DIR/target/debug/"
    [ -d "$DIR/target/release" ] && cp -a "$DIR/resources/." "$DIR/target/release/"
fi
