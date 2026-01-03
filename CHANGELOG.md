# Change Log

**This is a work in progress.**

## Breaking Changes

- Short-flag duplication fixed: `-l` now maps exclusively to
  `--line-numbers-only`; `--follow` is long-only (no short flag). Scripts
  relying on prior short-flag behavior may need updates.
- Environment defaults parsing: `NED_DEFAULTS` now respects quotes and escapes
  (POSIX shell-style splitting). Unquoted values that previously split on
  spaces may behave differently; quote values containing spaces.

## Changed

- Release now includes artifacts for Linux (GNU & musl), macOS, and Windows
  (Microsoft Visual C++ (MSVC) Toolchain & GNU/MinGW-w64 + GCC Toolchain).

- Dependencies' versions updated. Most importantly for the `regex` crate. See
  the [latest syntax documentation for supported regular
  expressions](https://docs.rs/regex/1.12.2/regex/#syntax).

- Line-mode context with `-A/-B/-C` options streams more efficiently, reducing
  memory usage and improving performance on large files; quiet modes
  short-circuit earlier for faster runs.

- Environment defaults (`NED_DEFAULTS`) now support quoted/escaped values via
  POSIX shell-style splitting; shell expansion and globbing is not performed.
  Quote patterns or values with spaces inside `NED_DEFAULTS`.

- Color handling is more reliable across modern terminals (including Windows).
  `--colors=auto|always|never` behavior is unchanged but detection is improved.

- Option parsing now adheres to standard `getopts` semantics. Free arguments
  and the `-p/--pattern` option can be placed after files as documented,
  making command composition more predictable.

- Path normalization is simplified for more consistent printed paths across
  platforms; matching behavior is unchanged.

- Quiet matching short-circuits earlier and avoids reading unnecessary files,
  improving performance on large trees.

## Documentation

- `README.md` and man page updated:
  - Clarified `NED_DEFAULTS` parsing (quoted/escaped values; no expansion) with
    examples.
  - Color modes and platform notes (Windows), `less -R`, and symlink behavior.
