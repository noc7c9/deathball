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
    cargo build --release --target wasm32-unknown-unknown
    mkdir -p ./target/build-wasm
    cp ./src/index.html \
       ./target/wasm32-unknown-unknown/release/deathball.wasm \
       ./target/build-wasm
    curl https://raw.githubusercontent.com/not-fl3/miniquad/v0.3.0-alpha.37/native/sapp-wasm/js/gl.js \
        --output ./target/build-wasm/gl.js
