#!/usr/bin/env bash
# update-cargo-audit-cache.sh — Pre-commit bash data provisioner for cargo-audit advisory check.
# Runs 'cargo audit --json' and writes cache to .factory/hooks/cargo-audit-cache.json.
# NOT a hook plugin — never registered in hooks-registry.toml.
# Per ADR-021 Option b: bash script provisions cache; WASM hook reads cache via host::read_file.
set -euo pipefail
OUTPUT_FILE=".factory/hooks/cargo-audit-cache.json"
mkdir -p "$(dirname "$OUTPUT_FILE")"
cargo audit --json > "$OUTPUT_FILE"
echo "cargo-audit-cache.json updated at $OUTPUT_FILE"
