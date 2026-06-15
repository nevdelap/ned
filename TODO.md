# TODO / Improvement Roadmap

## Priority 0 — Critical/Low-Risk Wins

- [x] Resolve short-flag conflict in `src/opts.rs`:
  - Resolved: `-l` is only `--line-numbers-only`; `--follow` is long-only.
  - Updated `README.md`; all tests pass (142/142).
- [x] Added a `justfile` to run linting and tests.
- [x] Remove unused dependency `time` from `Cargo.toml`.
- [x] Remove unnecessary `extern crate` lines (edition 2021) across `src/*.rs`.
- [x] Update edition to 2024.
- [x] Avoid `process::exit` from inside library-ish flow:
  - Implemented: `ned()` writes output and returns exit codes (no direct exits); `main()` maps to process exit.

## Performance

- [x] Stream line-mode reads in `process_file`:
  - Implemented: streaming via `BufReader::read_line` with before/after windows and quiet short-circuit.
  - Verified by full test suite and `streaming_context_windows`.
- [x] Reduce allocations in coloring paths:
  - Implemented: use `Cow<str>` and `Regex::replace_all` with closures in `color_*` helpers (`color`, `color_matches_all`, `color_matches_with_number_skip_backwards`).
  - Result: avoids unnecessary allocations when colors are disabled; closure-based replace colors actual matches efficiently.
- [x] Simplify iterator types in `Files`:
  - Return `PathBuf` directly (not `Box<PathBuf>`), hold `IntoIter` directly (not boxed).
- [ ] Optional parallelism (feature-flag or new flag):
  - Add `--threads N`/`--parallel` and use `rayon` to process files concurrently.
  - Ensure deterministic output ordering or document ordering semantics; short-circuit aggressively for `--quiet`.

## Reliability

- [x] Atomic in-place writes for replacements:
  - Implemented: write to a temp file in the same directory and atomically replace via `tempfile::NamedTempFile::persist`.
  - Fallback: on persist failure (e.g., cross-filesystem/link issues), gracefully revert to in-place seek/truncate write.
- [x] Broken pipe handling:
  - Implemented: propagate `BrokenPipe` from processing and short-circuit in `main()` without flushing; exit cleanly.
- [x] Path normalization:
  - Implemented lexical normalization in `Files::normalize_relative_paths` (remove `./`, fold `../`) without resolving symlinks, and simplified return type to `PathBuf`.
  - Verified against existing tests; behavior unchanged and more explicit.

## CLI Ergonomics

- [x] Color/TTY detection:
  - Replace `atty` with `is-terminal`/`supports-color` for modern terminals and Windows handling.
- [ ] Argument parsing (future breaking change consideration):
  - Consider migrating from `getopts` to `clap` v4 derive-based parsing for richer UX, validations, and maintainability.
  - Preserve existing flags; introduce deprecations with clear help.
- [x] Naming clarity:
  - Renamed `write_newline_if_replaced_text_ends_with_newline` → `ensure_trailing_newline`.

## Code Quality

- [x] `OptionsWithDefaults`: document `NED_DEFAULTS` behavior (POSIX shell-style splitting; quotes/escapes respected; no expansion) in code and README.
- [x] `Source` enum: remove `Box<File>` and store `File` directly; borrow consistently in `process_file`.
- [x] `Files::normalize_relative_paths` return type:
  - It always `Ok`; return `PathBuf` directly (no `Result`).

## Testing

- [x] Add regression test for the `-l` short-flag conflict resolution.
- [x] Add tests for `NED_DEFAULTS` shell-style parsing (quoted values, color flags).
- [x] Add tests for streaming line-mode with context windows.
- [x] Add tests for colored context headers and match highlighting.
- [x] Add test ensuring quiet mode with context flags suppresses output and returns success on match.
- [x] Add tests for `--matches-only` combined with context flags to validate interaction:
  - [x] `--context N --matches-only` (line-mode)
  - [x] `--before N --matches-only` (line-mode)
  - [x] `--after N --matches-only` (line-mode)
  - [x] `--context N --matches-only --quiet` (suppressed output, success on match)
- [x] Add tests for atomic write path (including interruption simulation if feasible).
- [x] Add test ensuring early stop on broken pipe.
- [x] Windows symlink expectations:
  - Tests now include `test/file8.txt` in non-follow modes only when the filesystem exposes it as a regular file (`fs::symlink_metadata(...).file_type().is_file()`), matching `WalkDir` semantics.
  - This avoids brittle assumptions in CI where symlink handling varies by runner and checkout settings.
- [ ] Add optional benchmarks with Criterion:
  - Large-file scans (line-mode vs whole-file)
  - Replace-on-write (seek/truncate vs atomic temp + rename)
  - Parallel vs serial across many files

## Documentation

- [x] Update `README.md`:
  - Clarify `NED_DEFAULTS` examples and non-shell parsing rules.
  - Explain color modes and terminal support notes for Windows.
  - Note Windows symlink behavior in CI and clarify that tests adapt to runtime filesystem semantics rather than assuming symlinks are regular files.
  - Document any new flags (parallelism, atomic writes behavior) once implemented.
- [x] README badges and lint fixes:
  - Add CI status badge; remove crates.io/docs.rs badges.
  - Fix emphasis style to satisfy markdownlint MD049.

## Tooling & CI

- [x] Enforce style and lints:
  - `cargo fmt` (fail if changes)
  - `cargo clippy -- -D warnings`
- [x] GitHub Actions CI (Linux/macOS/Windows):
  - Build, test, format, clippy on stable.
  - Steps use `just` recipes for clarity.
- [x] Optional policy checks:
  - [x] Add `cargo-deny` to CI (`cargo deny check`).
  - [x] Set MSRV (`rust-version = "1.85"`) in `Cargo.toml` and add MSRV CI job.
- [x] Dockerized Markdown tooling:
  - `just md-format` (Prettier) and `just md-lint` (markdownlint) via Docker.
  - CI runs Markdown format/lint on Linux runners.

## Nice-to-Have / Future Work

- [ ] Structured error messages (machine-readable) behind a flag for editor integration.
- [ ] JSON/NDJSON output mode for matches and replacements.
- [ ] Progress reporting for large recursive scans (suppressed under `--quiet`).

## Builds

- [x] Include build artifacts in releases.

## Ongoing

- [x] Add `CHANGELOG.md` with release notes since the last version.
- [x] Update `CHANGELOG.md` with streaming context refactor and new tests.

## Notes & Decisions

- Decision implemented: keep `-l` for `--line-numbers-only`; make `--follow` long-only to avoid surprises.
- Keep behavior stable; stage breaking changes under feature flags and document clearly.
- Change: `NED_DEFAULTS` now uses POSIX shell-style parsing (quotes and escapes respected; no expansion). README updated accordingly.
