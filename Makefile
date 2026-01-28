# Syntra Kernel Makefile

.PHONY: all build run test fmt lint clean release

all: build

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
