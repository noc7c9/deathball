_list:
    @just --list

start:
    just watch run

watch CMD='check' +ARGS='':
    watchexec --watch src --restart --clear -- just {{CMD}} {{ARGS}}

run +ARGS='':
    cargo run -- {{ARGS}}

check +ARGS='':
    cargo check {{ARGS}}

fmt +ARGS='':
    cargo fmt {{ARGS}}

test +ARGS='':
    cargo test {{ARGS}}

build +ARGS='':
    cargo build {{ARGS}}

cargo +ARGS='':
    cargo {{ARGS}}

build-wasm:
    ./build-wasm
