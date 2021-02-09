#!/bin/bash

WABT_BIN=$HOME/Code/wabt/bin
BINARYEN_BIN=$HOME/Code/binaryen/bin
TARGET=wasm32-unknown-unknown
NAME=do-addition
BINARY=target/$TARGET/release/$NAME.wasm32

cargo build --target $TARGET --release
#WABT_BIN/wasm-strip $BINARY
mkdir -p www
$BINARYEN_BIN/wasm-opt -o www/$NAME.wasm -Oz $BINARY