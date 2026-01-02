set shell := ["bash", "-uc"]

MARKDOWNLINT_IMAGE := "ghcr.io/igorshubovych/markdownlint-cli:latest"

# Default target shows available recipes.
default: help

help:
    @just --list

# Format Markdown.
format_markdown:
    uv tool run --with mdformat-gfm mdformat --number .

# Format Rust code; fail if it changed anything.
format_rust:
    cargo fmt --all

# Format code; fail if it changed anything.
format:
    just format_markdown
    just format_rust
    git diff --exit-code

# Lint Markdown via Docker with markdownlint
lint_markdown:
    docker run --rm -u "$(id -u):$(id -g)" -v "$(pwd)":/workdir {{ MARKDOWNLINT_IMAGE }} /workdir


# Lint with clippy and deny warnings, fail if it finds anything.
lint_rust *args="":
    cargo clippy --all-targets --all-features -- -D warnings {{args}}

# Run all linting steps.
lint: format
    just lint_markdown
    just lint_rust

# Run the test suite
test *args="":
    cargo test {{args}}

# Optional helpers
build:
    cargo build

release:
    cargo build --release

# Compare man page flags vs README (if desired)
man-compare:
    man/compare
