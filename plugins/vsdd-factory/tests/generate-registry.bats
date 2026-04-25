#!/usr/bin/env bats
# generate-registry.bats — invariants for
# scripts/generate-registry-from-hooks-json.sh (S-2.2).
#
# These tests guard the migration tool that produces
# plugins/vsdd-factory/hooks-registry.toml from the v0.79.x bash hook
# inventory. The Rust schema-side checks live at
# crates/factory-dispatcher/tests/loads_legacy_registry.rs.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  GENERATOR="$REPO_ROOT/scripts/generate-registry-from-hooks-json.sh"
  REGISTRY="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"
  HOOKS_DIR="$REPO_ROOT/plugins/vsdd-factory/hooks"
  # Locale is fixed in the generator itself; mirror it here so the
  # comm/sort calls in test bodies are byte-stable too.
  export LC_ALL=C
}

# Gate-hook allow-list — must stay in sync with the same constant in the
# generator. The duplication is deliberate: tests that read their truth
# from the file under test can't catch drift in that file.
GATE_HOOKS=(
  block-ai-attribution
  destructive-command-guard
  protect-secrets
  red-gate
  factory-branch-guard
  protect-vp
  protect-bc
  brownfield-discipline
  validate-wave-gate-prerequisite
  validate-pr-merge-prerequisites
  verify-git-push
  check-factory-commit
  validate-template-compliance
  validate-input-hash
  validate-factory-path-root
  pr-manager-completion-guard
  handoff-validator
)

@test "generator runs without errors" {
  run bash "$GENERATOR"
  [ "$status" -eq 0 ]
  [ -f "$REGISTRY" ]
}

@test "generator output is idempotent across two runs" {
  bash "$GENERATOR"
  cp "$REGISTRY" "$BATS_TEST_TMPDIR/registry-first.toml"
  bash "$GENERATOR"
  diff "$BATS_TEST_TMPDIR/registry-first.toml" "$REGISTRY"
}

@test "every script under hooks/ appears exactly once in the registry" {
  bash "$GENERATOR"

  # On-disk: every hooks/*.sh basename without extension.
  on_disk="$(find "$HOOKS_DIR" -maxdepth 1 -name '*.sh' -type f \
    | xargs -n1 basename \
    | sed 's|\.sh$||' \
    | sort -u)"

  # In registry: every distinct `name = "..."` value. (A name may map to
  # multiple [[hooks]] entries when one script handles multiple matchers,
  # e.g. protect-secrets — that's fine and counted as "appears" here.)
  in_registry="$(grep -E '^name = ' "$REGISTRY" \
    | sed 's/^name = "\(.*\)"$/\1/' \
    | sort -u)"

  missing="$(comm -23 <(printf '%s\n' "$on_disk") <(printf '%s\n' "$in_registry") || true)"
  orphan="$(comm -13 <(printf '%s\n' "$on_disk") <(printf '%s\n' "$in_registry") || true)"

  [ -z "$missing" ] || { echo "scripts on disk with no registry entry: $missing"; return 1; }
  [ -z "$orphan" ]  || { echo "registry entries with no on-disk script: $orphan"; return 1; }
}

@test "every gate hook entry has on_error = block" {
  bash "$GENERATOR"

  # Walk the registry block-by-block and check the on_error setting per
  # name. awk is the right tool here — line-grep can't tie a `name` line
  # to its `on_error` line cleanly across all 45 entries.
  fail=""
  while IFS= read -r line; do
    if [[ "$line" =~ ^name\ =\ \"(.+)\"$ ]]; then
      current_name="${BASH_REMATCH[1]}"
    elif [[ "$line" =~ ^on_error\ =\ \"(.+)\"$ ]]; then
      current_oe="${BASH_REMATCH[1]}"
      is_gate=0
      for g in "${GATE_HOOKS[@]}"; do
        if [ "$current_name" = "$g" ]; then
          is_gate=1
          break
        fi
      done
      if [ "$is_gate" = 1 ] && [ "$current_oe" != "block" ]; then
        fail+="$current_name expected on_error=block, got $current_oe"$'\n'
      fi
      if [ "$is_gate" = 0 ] && [ "$current_oe" != "continue" ]; then
        fail+="$current_name expected on_error=continue, got $current_oe"$'\n'
      fi
    fi
  done < "$REGISTRY"

  if [ -n "$fail" ]; then
    echo "$fail"
    return 1
  fi
}

@test "every entry routes through legacy-bash-adapter.wasm" {
  bash "$GENERATOR"
  count_entries="$(grep -c '^\[\[hooks\]\]' "$REGISTRY")"
  count_adapter="$(grep -c '^plugin = "hook-plugins/legacy-bash-adapter.wasm"$' "$REGISTRY")"
  [ "$count_entries" -gt 20 ]
  [ "$count_entries" = "$count_adapter" ]
}

@test "registry parses through the dispatcher's Registry::load" {
  # Cross-call into the Rust integration test; that suite owns the
  # schema validation. Re-running it here turns the bats run into a
  # full end-to-end gate without duplicating the parse logic in bash.
  bash "$GENERATOR"
  cd "$REPO_ROOT"
  run env PATH="$HOME/.cargo/bin:$PATH" \
    cargo test -p factory-dispatcher --test loads_legacy_registry --quiet
  [ "$status" -eq 0 ]
}
