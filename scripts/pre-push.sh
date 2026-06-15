#!/usr/bin/env bash
set -e
if [ -n "$(git status --porcelain)" ]; then
  echo $'\033[31;1mModified or untracked unignored files:\033[22m'
  git status --porcelain | cut -c4- | sed 's/^/  - /'
  echo -n $'\033[0m'
  exit 1
fi
just lint
just audit
just deny_check
just build
just test
just build --release
just test --release
git diff --exit-code
