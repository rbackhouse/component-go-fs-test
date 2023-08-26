#!/bin/bash

export PATH=$(pwd)/tinygo/bin:$PATH
rm testfs*.wasm
tinygo build -target=wasi -o testfs.wasm gosrc/main/main.go
wasm-tools component embed --world testfs ./wit testfs.wasm -o testfs.embed.wasm
wasm-tools component new testfs.embed.wasm --adapt wasi_snapshot_preview1=./wasi_snapshot_preview1.reactor.wasm -o testfs.component.wasm 
