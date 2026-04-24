#!/usr/bin/env bats
# factory-obs-registry.bats — tests for the multi-factory registry + override
# generation that landed in v0.78.0.
#
# Covers:
#   - register (explicit path, cwd autoresolve, dedup, bad input)
#   - unregister (success, no-op when absent, cwd autoresolve)
#   - list (empty, populated, status labels for missing dirs)
#   - regenerate (override file shape, per-factory bind mounts, glob target)
#   - fallbacks (empty registry + VSDD_FACTORY_LOGS, empty registry + cwd)
#
# Isolation: each test gets its own VSDD_OBS_REGISTRY pointed at a tmp
# file, so real user ~/.config is never touched.

setup() {
  TOOL="${BATS_TEST_DIRNAME}/../bin/factory-obs"
  OBS_DIR="${BATS_TEST_DIRNAME}/../tools/observability"
  OVERRIDE_FILE="$OBS_DIR/docker-compose.override.yml"

  TEST_HOME="$(mktemp -d)"
  export VSDD_OBS_REGISTRY="$TEST_HOME/watched-factories"

  # Build a fake factory project inside the tmp HOME so tests can
  # register paths without touching the real repo layout.
  FAKE_FACTORY_A="$TEST_HOME/proj-alpha"
  FAKE_FACTORY_B="$TEST_HOME/subdir/proj-beta"
  mkdir -p "$FAKE_FACTORY_A/.factory/logs"
  mkdir -p "$FAKE_FACTORY_B/.factory/logs"
}

teardown() {
  rm -f "$OVERRIDE_FILE"
  rm -rf "$TEST_HOME"
  unset VSDD_OBS_REGISTRY
}

# ---------- register ----------

@test "register: explicit absolute path is added" {
  run "$TOOL" register "$FAKE_FACTORY_A"
  [ "$status" -eq 0 ]
  [[ "$output" == *"registered $FAKE_FACTORY_A"* ]]
  grep -Fxq "$FAKE_FACTORY_A" "$VSDD_OBS_REGISTRY"
}

@test "register: cwd autoresolves to nearest .factory root" {
  # Running from a subdir of a factory project should still register
  # the project root (one ancestor up).
  mkdir -p "$FAKE_FACTORY_A/src/deep"
  run bash -c "cd '$FAKE_FACTORY_A/src/deep' && '$TOOL' register"
  [ "$status" -eq 0 ]
  [[ "$output" == *"registered $FAKE_FACTORY_A"* ]]
  grep -Fxq "$FAKE_FACTORY_A" "$VSDD_OBS_REGISTRY"
}

@test "register: refuses a path without .factory/ subdir" {
  local notafactory="$TEST_HOME/notafactory"
  mkdir -p "$notafactory"
  run "$TOOL" register "$notafactory"
  [ "$status" -ne 0 ]
  [[ "$output" == *"no .factory"* ]]
}

@test "register: refuses a relative path (needs absolute)" {
  # Pass a relative path that clearly doesn't start with /.
  run "$TOOL" register "relative/path"
  [ "$status" -ne 0 ]
}

@test "register: dedups when the same path is registered twice" {
  "$TOOL" register "$FAKE_FACTORY_A" >/dev/null
  run "$TOOL" register "$FAKE_FACTORY_A"
  [ "$status" -eq 0 ]
  [[ "$output" == *"already registered"* ]]
  # Only one line for this path.
  [ "$(grep -Fxc -- "$FAKE_FACTORY_A" "$VSDD_OBS_REGISTRY")" -eq 1 ]
}

@test "register: two different factories coexist in registry" {
  "$TOOL" register "$FAKE_FACTORY_A" >/dev/null
  "$TOOL" register "$FAKE_FACTORY_B" >/dev/null
  grep -Fxq "$FAKE_FACTORY_A" "$VSDD_OBS_REGISTRY"
  grep -Fxq "$FAKE_FACTORY_B" "$VSDD_OBS_REGISTRY"
}

# ---------- unregister ----------

@test "unregister: removes a registered path" {
  "$TOOL" register "$FAKE_FACTORY_A" >/dev/null
  run "$TOOL" unregister "$FAKE_FACTORY_A"
  [ "$status" -eq 0 ]
  [[ "$output" == *"unregistered $FAKE_FACTORY_A"* ]]
  ! grep -Fxq "$FAKE_FACTORY_A" "$VSDD_OBS_REGISTRY"
}

@test "unregister: no-op when path is not registered" {
  run "$TOOL" unregister "$FAKE_FACTORY_A"
  [ "$status" -eq 0 ]
  [[ "$output" == *"not registered"* ]]
}

@test "unregister: preserves other entries" {
  "$TOOL" register "$FAKE_FACTORY_A" >/dev/null
  "$TOOL" register "$FAKE_FACTORY_B" >/dev/null
  "$TOOL" unregister "$FAKE_FACTORY_A" >/dev/null
  ! grep -Fxq "$FAKE_FACTORY_A" "$VSDD_OBS_REGISTRY"
  grep -Fxq "$FAKE_FACTORY_B" "$VSDD_OBS_REGISTRY"
}

# ---------- list ----------

@test "list: empty registry shows '(no factories registered)'" {
  run "$TOOL" list
  [ "$status" -eq 0 ]
  [[ "$output" == *"no factories registered"* ]]
}

@test "list: populated registry shows each factory with a safe-name" {
  "$TOOL" register "$FAKE_FACTORY_A" >/dev/null
  "$TOOL" register "$FAKE_FACTORY_B" >/dev/null
  run "$TOOL" list
  [ "$status" -eq 0 ]
  [[ "$output" == *"$FAKE_FACTORY_A"* ]]
  [[ "$output" == *"$FAKE_FACTORY_B"* ]]
  # basename-hash format
  [[ "$output" == *"name: proj-alpha-"* ]]
  [[ "$output" == *"name: proj-beta-"* ]]
  [[ "$output" == *"total: 2"* ]]
}

@test "list: flags a registered path whose .factory/ is missing" {
  "$TOOL" register "$FAKE_FACTORY_A" >/dev/null
  rm -rf "$FAKE_FACTORY_A/.factory"
  run "$TOOL" list
  [ "$status" -eq 0 ]
  [[ "$output" == *"MISSING .factory"* ]]
}

# ---------- regenerate ----------

@test "regenerate: produces an override with one mount per registered factory" {
  "$TOOL" register "$FAKE_FACTORY_A" >/dev/null
  "$TOOL" register "$FAKE_FACTORY_B" >/dev/null
  run "$TOOL" regenerate
  [ "$status" -eq 0 ]
  [ -f "$OVERRIDE_FILE" ]

  # Each registered factory should show up in the override as a bind
  # mount pointing at a per-factory subdir of /var/log/factory/.
  grep -q "$FAKE_FACTORY_A/.factory/logs:/var/log/factory/proj-alpha-" "$OVERRIDE_FILE"
  grep -q "$FAKE_FACTORY_B/.factory/logs:/var/log/factory/proj-beta-" "$OVERRIDE_FILE"
  # All mounts are read-only.
  [ "$(grep -c ':ro"' "$OVERRIDE_FILE")" -eq 2 ]
}

@test "regenerate: mount targets end up under /var/log/factory/<subdir>/ (matches collector glob)" {
  "$TOOL" register "$FAKE_FACTORY_A" >/dev/null
  "$TOOL" regenerate >/dev/null
  # The collector globs /var/log/factory/*/events-*.jsonl. Verify the
  # override mount target is one level deep under /var/log/factory/.
  # Filter to actual bind-mount lines (quoted source:target:ro) so the
  # header comment's example paths don't confuse the extraction.
  local target
  target=$(grep -E '^\s+- "' "$OVERRIDE_FILE" | grep -oE '/var/log/factory/[^:]+' | head -1)
  [[ "$target" =~ ^/var/log/factory/[^/]+$ ]]
}

@test "regenerate: override yaml parses cleanly" {
  "$TOOL" register "$FAKE_FACTORY_A" >/dev/null
  "$TOOL" regenerate >/dev/null
  yq eval . "$OVERRIDE_FILE" >/dev/null
}

# ---------- fallbacks (empty registry) ----------

@test "regenerate: empty registry + VSDD_FACTORY_LOGS uses that path as fallback" {
  VSDD_FACTORY_LOGS="$FAKE_FACTORY_A/.factory/logs" run "$TOOL" regenerate
  [ "$status" -eq 0 ]
  grep -q "$FAKE_FACTORY_A/.factory/logs:/var/log/factory/proj-alpha-" "$OVERRIDE_FILE"
}

@test "regenerate: empty registry + cwd-in-factory uses cwd as fallback" {
  run bash -c "cd '$FAKE_FACTORY_A' && '$TOOL' regenerate"
  [ "$status" -eq 0 ]
  [ -f "$OVERRIDE_FILE" ]
  grep -q "$FAKE_FACTORY_A/.factory/logs:/var/log/factory/proj-alpha-" "$OVERRIDE_FILE"
}

@test "regenerate: empty registry + cwd-not-in-factory is an error" {
  local nonfactory="$TEST_HOME/justadir"
  mkdir -p "$nonfactory"
  run bash -c "cd '$nonfactory' && '$TOOL' regenerate"
  [ "$status" -ne 0 ]
  [[ "$output" == *"no factories registered"* ]]
}

# ---------- safe-name disambiguation ----------

@test "regenerate: two projects sharing a basename get distinct safe-names via path-hash" {
  # Both end in "api" — hash suffix must disambiguate.
  local a="$TEST_HOME/parent1/api"
  local b="$TEST_HOME/parent2/api"
  mkdir -p "$a/.factory/logs" "$b/.factory/logs"
  "$TOOL" register "$a" >/dev/null
  "$TOOL" register "$b" >/dev/null
  "$TOOL" regenerate >/dev/null
  # Two distinct subdir targets under /var/log/factory/, both starting
  # with "api-".
  local count
  count=$(grep -oE '/var/log/factory/api-[0-9a-f]+' "$OVERRIDE_FILE" | sort -u | wc -l | tr -d ' ')
  [ "$count" -eq 2 ]
}
