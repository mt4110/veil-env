default:
    @just --list

fmt:
    cargo fmt --all

lint:
    cargo fmt --all -- --check
    cargo clippy --locked --all-targets -- -D warnings

lint-md:
    markdownlint-cli2

test:
    cargo test --locked

build:
    cargo build --locked

check: lint test lint-md
