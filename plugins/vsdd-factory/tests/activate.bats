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
