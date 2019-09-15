#!/bin/bash
RELEASE_FLAG=""
TARGET_SUBDIR="debug"
WATCH=false

for i in "$@" ; do
    if [[ $i == "--release" ]] ; then
        RELEASE_FLAG="--release"
        TARGET_SUBDIR="release"
    elif [[ $i == "--watch" ]] ; then
        WATCH=true
    fi
done

if [ $WATCH = true ] ; then
    cargo watch -x "build $RELEASE_FLAG --target wasm32-unknown-unknown" -s "wasm-bindgen --target web --out-dir ../static/dist/jwordlist/ target/wasm32-unknown-unknown/$TARGET_SUBDIR/jwordlist_front.wasm"
else
    cargo build $RELEASE_FLAG --target wasm32-unknown-unknown
    wasm-bindgen --target web --out-dir ../static/dist/jwordlist/ target/wasm32-unknown-unknown/$TARGET_SUBDIR/jwordlist_front.wasm
fi