# Syntra Kernel Justfile

default: build

build:
    cargo build

run:
    cargo run --bin syntra_kernel

test:
    cargo test

fmt:
    cargo fmt --all

lint:
    cargo clippy --all-targets --all-features -- -D warnings

clean:
    cargo clean

release:
    cargo build --release

trials:
    cargo run --bin syntra_trials

compiler-demo:
    cargo run --bin syntra_compiler_demo
