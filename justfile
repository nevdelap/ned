set shell := ["bash", "-uc"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

ACTIONLINT_IMAGE := "docker.io/rhysd/actionlint:latest"
MARKDOWNLINT_IMAGE := "ghcr.io/igorshubovych/markdownlint-cli:latest"
SHELLCHECK_IMAGE := "koalaman/shellcheck:latest"
SHFMT_IMAGE := "mvdan/shfmt:latest"

# Default target shows available recipes.
default: help

# Show help.
help:
    @just --list

# Install the pre-push git hook.
install:
    @if [ ! -f .git/hooks/pre-push ]; then mkdir -p .git/hooks && cd .git/hooks/ && ln -f -s ../../scripts/pre-push.sh pre-push; fi

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

# Format shell scripts in `scripts/` using shfmt.
format_shell:
    mapfile -t changed_files < <(git diff --name-only --diff-filter=AMR origin/master -- '*.sh'); \
    if [ ${#changed_files[@]} -gt 0 ]; then \
        docker run --rm -u "$(id -u):$(id -g)" -v "$(pwd)":/workdir -w /workdir {{ SHFMT_IMAGE }} -w -i 2 "${changed_files[@]}"; \
    fi

# Format Rust code; fail if it changed anything.
format_rust:
    cargo fmt --all

# Format code; fail if it changed anything.
format: install
    just format_markdown
    just format_toml
    just format_shell
    just format_rust
    git diff --exit-code

# Lint Markdown via Docker with markdownlint
lint_markdown:
    mapfile -t changed_files < <(git diff --name-only --diff-filter=AMR origin/master -- '*.md'); \
    if [ ${#changed_files[@]} -gt 0 ]; then \
        docker run --rm -u "$(id -u):$(id -g)" -v "$(pwd)":/workdir -w /workdir {{ MARKDOWNLINT_IMAGE }} "${changed_files[@]}"; \
    fi

# Lint TOML files.
lint_toml:
    mapfile -t changed_files < <(git diff --name-only --diff-filter=AMR origin/master -- '*.toml'); \
    if [ ${#changed_files[@]} -gt 0 ]; then \
        taplo check "${changed_files[@]}"; \
    fi

# Lint shell scripts in `scripts/` with ShellCheck.
lint_shell:
    mapfile -t changed_files < <(git diff --name-only --diff-filter=AMR origin/master -- '*.sh'); \
    if [ ${#changed_files[@]} -gt 0 ]; then \
        docker run --rm -u "$(id -u):$(id -g)" -v "$(pwd)":/workdir -w /workdir {{ SHELLCHECK_IMAGE }} "${changed_files[@]}"; \
    fi

# Lint GitHub Actions workflows with actionlint.
lint_actions:
    mapfile -t changed_files < <(git diff --name-only --diff-filter=AMR origin/master -- '.github/workflows/*.yml' '.github/workflows/*.yaml'); \
    if [ ${#changed_files[@]} -gt 0 ]; then \
        docker run --rm -u "$(id -u):$(id -g)" -v "$(pwd)":/workdir -w /workdir {{ ACTIONLINT_IMAGE }} "${changed_files[@]}"; \
    fi

# Lint with clippy and deny warnings, fail if it finds anything.
lint_rust *args="":
    cargo clippy --all-targets --all-features -- -D warnings {{args}}

# Run all linting steps.
lint: format
    just lint_markdown
    just lint_toml
    just lint_shell
    just lint_actions
    just lint_rust

# Run the test suite.
test *args="":
    cargo test {{args}}

# Build the project.
build *args="":
    cargo build {{args}}

# Get version from Cargo.toml
get_version:
    @docker run --rm -v "$(pwd)":/workdir -w /workdir docker.io/mikefarah/yq:4 '.package.version' Cargo.toml

# After tests, verify `ned --version` matches Cargo.toml (debug)
version_check_debug:
    cargo run -- --version | grep -q "ned $(just get_version)"

# After tests, verify `ned --version` matches Cargo.toml (release)
version_check_release:
    cargo run --release -- --version | grep -q "ned $(just get_version)"

# Run cargo deny check.
deny_check:
    cargo install cargo-deny
    cargo deny check

# Create and push a git tag based on Cargo.toml version.
tag_release:
    set -euo pipefail
    if [ -n "$(git status --porcelain)" ]; then \
        echo $'\033[31;1mModified or untracked unignored files:\033[22m'; \
        git status --porcelain | cut -c4- | sed 's/^/  - /'; \
        echo -n $'\033[0m'; \
        exit 1; \
    fi
    version=$(just get_version); \
    if [ -z "$version" ]; then \
        echo "Version not found in Cargo.toml"; exit 1; \
    fi; \
    tag="release.$version"; \
    git tag --force "$tag"; \
    git push --force origin "$tag"

# Show help.
ned_help:
    cargo run -- --help
