#!/usr/bin/env bash
# Git stub for demo recordings: fetch succeeds; user.email returns dev@x.com
if [[ "$1" == "fetch" ]]; then
  exit 0
elif [[ "$1" == "config" && "${2:-}" == "user.email" ]]; then
  echo "dev@x.com"
  exit 0
else
  exec /usr/bin/git "$@"
fi
