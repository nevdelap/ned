# Changelog

## 1.3.4 — 2026-01-02

### Changed

- Environment defaults (`NED_DEFAULTS`) now support quoted/escaped values via
  POSIX shell-style splitting; shell expansion is not performed. Quote patterns
  or values with spaces inside `NED_DEFAULTS`.
- Color handling is more reliable across modern terminals (including Windows).
  `--colors=auto|always|never` behavior is unchanged but detection is improved.

### Fixed

- Short-flag conflict: `-l` is only `--line-numbers-only`; `--follow` is
  long-only.

### Documentation

- README and man page updated:
  - Clarified `NED_DEFAULTS` parsing (quoted/escaped values; no expansion) with
    examples.
  - Color modes and platform notes (Windows), `less -R`, and symlink behavior.

### Upgrade Notes

- If you used `NED_DEFAULTS` previously without quoting, quote any values
  containing spaces or glob characters to keep them as a single argument, e.g.
  `NED_DEFAULTS="--replace 'out standing' --include '*.txt'"`.
- `--follow` remains long-only. Use `--line-numbers-only` via `-l` for listing
  matched line numbers.
