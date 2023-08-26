# WASM Component Model Go Filesystem testcase

## Prereqs

* Tinygo https://github.com/tinygo-org/tinygo/releases/tag/v0.28.1 download for you platform and expand in this directory
* wasm-tools 
```
cargo install wasm-tools
```
* wasi_snapshot_preview1.reactor.wasm ver 12.0.1 (https://github.com/bytecodealliance/wasmtime/releases/download/v12.0.1/wasi_snapshot_preview1.reactor.wasm)

## Building

* Build component 
```
./build.sh
```
* Run

```
cargo run
```