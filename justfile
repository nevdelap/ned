set shell := ["bash", "-uc"]

# Default target shows available recipes
default: help

help:
    @just --list

# Run the test suite
test *args="":
    cargo test {{args}}

# Format code (checks only)
fmt-check:
    cargo fmt --all -- --check

# Lint with clippy and deny warnings
clippy *args="":
    cargo clippy --all-targets --all-features -- -D warnings {{args}}

# Run all linting steps
lint:
    just fmt-check
    just clippy

# Convenient CI aggregate: lint then test
ci:
    just lint
    just test

# Optional helpers
build:
    cargo build

release:
    cargo build --release

# Compare man page flags vs README (if desired)
man-compare:
    man/compare
