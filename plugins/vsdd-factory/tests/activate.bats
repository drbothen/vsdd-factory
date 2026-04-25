#!/usr/bin/env bats
# activate.bats — tests for the activate skill's platform-detection helper.
#
# The activate skill itself is interpreted by Claude (it's a markdown
# procedure document), but the platform-detection step is a deterministic
# shell helper. These tests exercise that helper end-to-end with mocked
# uname output to cover every supported and unsupported platform tuple.
#
# Helper under test: plugins/vsdd-factory/skills/activate/detect-platform.sh

setup() {
  HELPER="${BATS_TEST_DIRNAME}/../skills/activate/detect-platform.sh"
  APPLY="${BATS_TEST_DIRNAME}/../skills/activate/apply-platform.sh"
}

# Build a synthetic plugin root with the hooks.json variants in place
# so apply-platform tests don't depend on the real plugin layout.
_make_synthetic_root() {
  local root
  root="$(mktemp -d)"
  mkdir -p "$root/hooks"
  mkdir -p "$root/hooks/dispatcher/bin/darwin-arm64"
  mkdir -p "$root/hooks/dispatcher/bin/darwin-x64"
  mkdir -p "$root/hooks/dispatcher/bin/linux-x64"
  mkdir -p "$root/hooks/dispatcher/bin/linux-arm64"
  mkdir -p "$root/hooks/dispatcher/bin/windows-x64"
  for p in darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64; do
    cat > "$root/hooks/hooks.json.$p" <<JSON
{ "hooks": { "platform": "$p" } }
JSON
  done
  echo "$root"
}

# Drop a real (executable) dispatcher placeholder into the synthetic
# root so the binary-existence + executable checks pass.
_install_fake_binary() {
  local root="$1" platform="$2"
  local suffix=""
  [ "$platform" = "windows-x64" ] && suffix=".exe"
  local target="$root/hooks/dispatcher/bin/$platform/factory-dispatcher$suffix"
  echo '#!/bin/sh' > "$target"
  echo 'exit 0' >> "$target"
  chmod +x "$target"
}

# Run helper with mocked uname output. Returns the stdout JSON in $output.
_detect() {
  MOCK_UNAME_S="$1" MOCK_UNAME_M="$2" "$HELPER"
}

# ---------- Structural ----------

@test "detect-platform: file is executable" {
  [ -x "$HELPER" ]
}

@test "detect-platform: passes bash syntax check" {
  bash -n "$HELPER"
}

# ---------- Supported platforms (5 canonical strings) ----------

@test "detect-platform: Darwin arm64 -> darwin-arm64" {
  run _detect Darwin arm64
  [ "$status" -eq 0 ]
  [ "$(jq -r .platform <<<"$output")" = "darwin-arm64" ]
  [ "$(jq -r .error <<<"$output")" = "null" ]
}

@test "detect-platform: Darwin x86_64 -> darwin-x64" {
  run _detect Darwin x86_64
  [ "$status" -eq 0 ]
  [ "$(jq -r .platform <<<"$output")" = "darwin-x64" ]
}

@test "detect-platform: Linux x86_64 -> linux-x64" {
  run _detect Linux x86_64
  [ "$status" -eq 0 ]
  [ "$(jq -r .platform <<<"$output")" = "linux-x64" ]
}

@test "detect-platform: Linux aarch64 -> linux-arm64" {
  run _detect Linux aarch64
  [ "$status" -eq 0 ]
  [ "$(jq -r .platform <<<"$output")" = "linux-arm64" ]
}

@test "detect-platform: Linux arm64 -> linux-arm64 (some distros)" {
  run _detect Linux arm64
  [ "$status" -eq 0 ]
  [ "$(jq -r .platform <<<"$output")" = "linux-arm64" ]
}

@test "detect-platform: MINGW64_NT-10.0 x86_64 -> windows-x64" {
  run _detect MINGW64_NT-10.0 x86_64
  [ "$status" -eq 0 ]
  [ "$(jq -r .platform <<<"$output")" = "windows-x64" ]
}

@test "detect-platform: MSYS_NT-10.0 x86_64 -> windows-x64" {
  run _detect MSYS_NT-10.0 x86_64
  [ "$status" -eq 0 ]
  [ "$(jq -r .platform <<<"$output")" = "windows-x64" ]
}

@test "detect-platform: CYGWIN_NT-10.0 amd64 -> windows-x64" {
  run _detect CYGWIN_NT-10.0 amd64
  [ "$status" -eq 0 ]
  [ "$(jq -r .platform <<<"$output")" = "windows-x64" ]
}

# ---------- Unsupported platforms ----------

@test "detect-platform: FreeBSD x86_64 -> unsupported (rc=1)" {
  run _detect FreeBSD x86_64
  [ "$status" -eq 1 ]
  [ "$(jq -r .platform <<<"$output")" = "null" ]
  [ "$(jq -r .error <<<"$output")" = "unsupported-platform" ]
}

@test "detect-platform: Darwin i386 -> unsupported (no 32-bit)" {
  run _detect Darwin i386
  [ "$status" -eq 1 ]
  [ "$(jq -r .error <<<"$output")" = "unsupported-platform" ]
}

@test "detect-platform: Linux mips -> unsupported" {
  run _detect Linux mips
  [ "$status" -eq 1 ]
  [ "$(jq -r .error <<<"$output")" = "unsupported-platform" ]
}

# ---------- Detected-from accuracy ----------

@test "detect-platform: detected_from.os reflects mocked value" {
  run _detect Linux x86_64
  [ "$status" -eq 0 ]
  [ "$(jq -r .detected_from.os <<<"$output")" = "Linux" ]
  [ "$(jq -r .detected_from.arch <<<"$output")" = "x86_64" ]
  [ "$(jq -r .detected_from.raw_uname <<<"$output")" = "Linux x86_64" ]
}

@test "detect-platform: detected_from preserved on unsupported" {
  run _detect FreeBSD x86_64
  [ "$status" -eq 1 ]
  [ "$(jq -r .detected_from.os <<<"$output")" = "FreeBSD" ]
  [ "$(jq -r .detected_from.arch <<<"$output")" = "x86_64" ]
}

# ---------- Argument validation ----------

@test "detect-platform: --help prints usage" {
  run "$HELPER" --help
  [ "$status" -eq 0 ]
  [[ "$output" == *"detect host platform"* ]]
}

@test "detect-platform: rejects unknown flag" {
  run "$HELPER" --bogus
  [ "$status" -eq 2 ]
  [[ "$output" == *"unknown argument"* ]]
}

# ---------- Real-host smoke ----------

@test "detect-platform: real-host invocation produces a known canonical platform" {
  # We don't know what host this runs on, but it must be one of the 5
  # canonical strings (darwin-arm64 in dev, linux-x64/arm64 in CI).
  run "$HELPER"
  [ "$status" -eq 0 ]
  p=$(jq -r .platform <<<"$output")
  case "$p" in
    darwin-arm64|darwin-x64|linux-x64|linux-arm64|windows-x64) : ;;
    *) echo "unexpected platform: $p" >&2; false ;;
  esac
}

# ---------- apply-platform structural ----------

@test "apply-platform: file is executable" {
  [ -x "$APPLY" ]
}

@test "apply-platform: passes bash syntax check" {
  bash -n "$APPLY"
}

@test "apply-platform: --help prints usage" {
  run "$APPLY" --help
  [ "$status" -eq 0 ]
  [[ "$output" == *"copy hooks.json.<platform> into place"* ]]
}

@test "apply-platform: rejects unknown platform" {
  run "$APPLY" freebsd-x64
  [ "$status" -eq 4 ]
  [[ "$output" == *"unsupported platform"* ]]
}

@test "apply-platform: rejects missing argument" {
  run "$APPLY"
  [ "$status" -eq 4 ]
  [[ "$output" == *"usage:"* ]]
}

# ---------- apply-platform success path ----------

@test "apply-platform: applies darwin-arm64 with binary present" {
  root=$(_make_synthetic_root)
  _install_fake_binary "$root" darwin-arm64
  run env VSDD_PLUGIN_ROOT_OVERRIDE="$root" "$APPLY" darwin-arm64
  [ "$status" -eq 0 ]
  [[ "$output" == *"applied darwin-arm64"* ]]
  # hooks.json materialized
  [ -f "$root/hooks/hooks.json" ]
  grep -q '"platform": "darwin-arm64"' "$root/hooks/hooks.json"
  rm -rf "$root"
}

@test "apply-platform: applies windows-x64 with .exe binary" {
  root=$(_make_synthetic_root)
  _install_fake_binary "$root" windows-x64
  run env VSDD_PLUGIN_ROOT_OVERRIDE="$root" "$APPLY" windows-x64
  [ "$status" -eq 0 ]
  [ -f "$root/hooks/hooks.json" ]
  grep -q '"platform": "windows-x64"' "$root/hooks/hooks.json"
  rm -rf "$root"
}

# ---------- apply-platform failure paths ----------

@test "apply-platform: missing variant returns code 1 with diagnostic" {
  root=$(mktemp -d)
  mkdir -p "$root/hooks/dispatcher/bin/linux-x64"
  _install_fake_binary "$root" linux-x64
  run env VSDD_PLUGIN_ROOT_OVERRIDE="$root" "$APPLY" linux-x64
  [ "$status" -eq 1 ]
  [[ "$output" == *"missing variant"* ]]
  rm -rf "$root"
}

@test "apply-platform: missing binary returns code 2 with restoration hints" {
  root=$(_make_synthetic_root)
  # No binary installed.
  run env VSDD_PLUGIN_ROOT_OVERRIDE="$root" "$APPLY" linux-x64
  [ "$status" -eq 2 ]
  [[ "$output" == *"dispatcher binary missing"* ]]
  [[ "$output" == *"v0.79.4"* ]]
  [[ "$output" == *"Build the dispatcher locally"* ]]
  # hooks.json must NOT have been written — fail early, fail clean.
  [ ! -f "$root/hooks/hooks.json" ]
  rm -rf "$root"
}

@test "apply-platform: non-executable binary returns code 3" {
  root=$(_make_synthetic_root)
  local target="$root/hooks/dispatcher/bin/linux-x64/factory-dispatcher"
  echo '#!/bin/sh' > "$target"
  echo 'exit 0' >> "$target"
  chmod -x "$target"
  run env VSDD_PLUGIN_ROOT_OVERRIDE="$root" "$APPLY" linux-x64
  [ "$status" -eq 3 ]
  [[ "$output" == *"not executable"* ]]
  rm -rf "$root"
}

# ---------- apply-platform --check (verify-only) ----------

@test "apply-platform --check: success does NOT write hooks.json" {
  root=$(_make_synthetic_root)
  _install_fake_binary "$root" linux-x64
  run env VSDD_PLUGIN_ROOT_OVERRIDE="$root" "$APPLY" --check linux-x64
  [ "$status" -eq 0 ]
  [[ "$output" == *"variant + binary present"* ]]
  [ ! -f "$root/hooks/hooks.json" ]
  rm -rf "$root"
}

@test "apply-platform --check: missing binary still returns code 2" {
  root=$(_make_synthetic_root)
  run env VSDD_PLUGIN_ROOT_OVERRIDE="$root" "$APPLY" --check linux-x64
  [ "$status" -eq 2 ]
}

# ---------- regression: cross-platform combinations ----------

@test "apply-platform: every canonical platform applies cleanly" {
  for p in darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64; do
    root=$(_make_synthetic_root)
    _install_fake_binary "$root" "$p"
    run env VSDD_PLUGIN_ROOT_OVERRIDE="$root" "$APPLY" "$p"
    [ "$status" -eq 0 ] || { echo "FAIL on $p: $output" >&2; rm -rf "$root"; false; }
    rm -rf "$root"
  done
}
