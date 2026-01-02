set shell := ["bash", "-uc"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

MARKDOWNLINT_IMAGE := "ghcr.io/igorshubovych/markdownlint-cli:latest"
SHELLCHECK_IMAGE := "koalaman/shellcheck:latest"
SHFMT_IMAGE := "mvdan/shfmt:latest"

# Default target shows available recipes.
default: help

help:
    @just --list

# Format Markdown.
format_markdown:
    mapfile -t changed_files < <(git diff --name-only --diff-filter=AMR origin/master -- '*.md'); \
    if [ ${#changed_files[@]} -gt 0 ]; then \
        uv tool run --with mdformat-gfm mdformat --number "${changed_files[@]}"; \
    fi

# Format TOML files (e.g., Cargo.toml, rust-toolchain.toml).
format_toml:
    mapfile -t changed_files < <(git diff --name-only --diff-filter=AMR origin/master -- '*.toml'); \
    if [ ${#changed_files[@]} -gt 0 ]; then \
        taplo format "${changed_files[@]}"; \
    fi

# Format shell scripts in `scripts/` and `man/` using shfmt.
format_shell:
    mapfile -t changed_files < <(git diff --name-only --diff-filter=AMR origin/master -- '*.sh'); \
    if [ ${#changed_files[@]} -gt 0 ]; then \
        docker run --rm -u "$(id -u):$(id -g)" -v "$(pwd)":/work -w /work {{ SHFMT_IMAGE }} -w -i 2 "${changed_files[@]}"; \
    fi

# Format Rust code; fail if it changed anything.
format_rust:
    cargo fmt --all

# Format code; fail if it changed anything.
format:
    just format_markdown
    just format_toml
    just format_shell
    just format_rust
    git diff --exit-code

# Lint Markdown via Docker with markdownlint
lint_markdown:
    mapfile -t changed_files < <(git diff --name-only --diff-filter=AMR origin/master -- '*.md'); \
    if [ ${#changed_files[@]} -gt 0 ]; then \
        prefixed=(); \
        for f in "${changed_files[@]}"; do prefixed+=( "/workdir/$f" ); done; \
        docker run --rm -u "$(id -u):$(id -g)" -v "$(pwd)":/workdir {{ MARKDOWNLINT_IMAGE }} "${prefixed[@]}"; \
    fi


# Lint TOML files.
lint_toml:
    mapfile -t changed_files < <(git diff --name-only --diff-filter=AMR origin/master -- '*.toml'); \
    if [ ${#changed_files[@]} -gt 0 ]; then \
        taplo check "${changed_files[@]}"; \
    fi

# Lint shell scripts in `scripts/` and `man/` with ShellCheck.
lint_shell:
    mapfile -t changed_files < <(git diff --name-only --diff-filter=AMR origin/master -- '*.sh'); \
    if [ ${#changed_files[@]} -gt 0 ]; then \
        prefixed=(); \
        for f in "${changed_files[@]}"; do prefixed+=( "/code/$f" ); done; \
        docker run --rm -u "$(id -u):$(id -g)" -v "$(pwd)":/code {{ SHELLCHECK_IMAGE }} "${prefixed[@]}"; \
    fi

# Lint with clippy and deny warnings, fail if it finds anything.
lint_rust *args="":
    cargo clippy --all-targets --all-features -- -D warnings {{args}}

# Run all linting steps.
lint: format
    just lint_markdown
    just lint_toml
    just lint_shell
    just lint_rust

# Run the test suite
test *args="":
    cargo test {{args}}

# Do a debug build.
build *args="":
    cargo build {{args}}

# Do a release build.
release *args="":
    cargo build {{args}}

# Compare man page flags vs README (if desired)
man-compare:
    man/compare.sh
