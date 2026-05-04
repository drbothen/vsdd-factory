#!/usr/bin/env bats
# protect-secrets.bats — tests for the protect-secrets hook
#
# Covers both the Read tool path (file_path based) and the Bash tool path
# (command-string based) to ensure .env file contents don't leak.

setup() {
  HOOK="${BATS_TEST_DIRNAME}/../hooks/protect-secrets.sh"
}

_run_bash() {
  local cmd="$1"
  local input
  input=$(jq -nc --arg c "$cmd" '{tool_name: "Bash", tool_input: {command: $c}}')
  run bash -c "echo '$input' | '$HOOK' 2>&1"
}

_run_read() {
  local path="$1"
  local input
  input=$(jq -nc --arg p "$path" '{tool_name: "Read", tool_input: {file_path: $p}}')
  run bash -c "echo '$input' | '$HOOK' 2>&1"
}

# ---------- Structural ----------

@test "hook file exists and is executable" {
  [ -x "$HOOK" ]
}

@test "hook passes syntax check" {
  bash -n "$HOOK"
}

@test "registry wires protect-secrets under PreToolUse Bash matcher" {
  load "${BATS_TEST_DIRNAME}/helpers/registry.bash"
  registry_has_hook "protect-secrets" "PreToolUse" "Bash"
}

@test "registry wires protect-secrets under PreToolUse Read matcher" {
  load "${BATS_TEST_DIRNAME}/helpers/registry.bash"
  registry_has_hook "protect-secrets" "PreToolUse" "Read"
}

# ---------- Read tool: BLOCKED ----------

@test "blocks Read on .env" {
  _run_read ".env"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
}

@test "blocks Read on .env.production" {
  _run_read ".env.production"
  [ "$status" -eq 2 ]
}

@test "blocks Read on .env.local" {
  _run_read ".env.local"
  [ "$status" -eq 2 ]
}

@test "blocks Read on .envrc" {
  _run_read ".envrc"
  [ "$status" -eq 2 ]
}

@test "blocks Read on nested /path/.env" {
  _run_read "/Users/jmagady/project/.env"
  [ "$status" -eq 2 ]
}

# ---------- Read tool: ALLOWED ----------

@test "allows Read on .env.example" {
  _run_read ".env.example"
  [ "$status" -eq 0 ]
}

@test "allows Read on .env.sample" {
  _run_read ".env.sample"
  [ "$status" -eq 0 ]
}

@test "allows Read on .env.template" {
  _run_read ".env.template"
  [ "$status" -eq 0 ]
}

@test "allows Read on README.md" {
  _run_read "README.md"
  [ "$status" -eq 0 ]
}

@test "allows Read on arbitrary file" {
  _run_read "src/main.ts"
  [ "$status" -eq 0 ]
}

# ---------- Bash: content-reading tools on real .env — BLOCKED ----------

@test "blocks cat .env" {
  _run_bash "cat .env"
  [ "$status" -eq 2 ]
}

@test "blocks less .env.local" {
  _run_bash "less .env.local"
  [ "$status" -eq 2 ]
}

@test "blocks head .env" {
  _run_bash "head -5 .env"
  [ "$status" -eq 2 ]
}

@test "blocks tail .env" {
  _run_bash "tail .env"
  [ "$status" -eq 2 ]
}

@test "blocks grep on .env" {
  _run_bash "grep API_KEY .env"
  [ "$status" -eq 2 ]
}

@test "blocks sed on .env" {
  _run_bash "sed -n 1p .env"
  [ "$status" -eq 2 ]
}

@test "blocks bat .env" {
  _run_bash "bat .env"
  [ "$status" -eq 2 ]
}

@test "blocks xxd .env" {
  _run_bash "xxd .env"
  [ "$status" -eq 2 ]
}

# ---------- Bash: content-reading on safe files/templates — ALLOWED ----------

@test "allows cat .env.example" {
  _run_bash "cat .env.example"
  [ "$status" -eq 0 ]
}

@test "allows cat README.md" {
  _run_bash "cat README.md"
  [ "$status" -eq 0 ]
}

@test "allows ls .env*" {
  _run_bash "ls -la .env*"
  [ "$status" -eq 0 ]
}

@test "allows test -f .env" {
  _run_bash "test -f .env"
  [ "$status" -eq 0 ]
}

@test "allows [ -f .env ]" {
  _run_bash "[ -f .env ] && echo yes"
  [ "$status" -eq 0 ]
}

@test "allows source .env" {
  _run_bash "source .env"
  [ "$status" -eq 0 ]
}

@test "allows . .env (dot source)" {
  _run_bash ". .env"
  [ "$status" -eq 0 ]
}

# ---------- Bash: copy/move of .env — BLOCKED (source is real .env) ----------

@test "blocks cp .env /tmp/" {
  _run_bash "cp .env /tmp/"
  [ "$status" -eq 2 ]
}

@test "blocks mv .env backup/" {
  _run_bash "mv .env backup/"
  [ "$status" -eq 2 ]
}

@test "blocks rsync .env remote:" {
  _run_bash "rsync .env user@host:/tmp/"
  [ "$status" -eq 2 ]
}

@test "blocks scp .env remote:" {
  _run_bash "scp .env user@host:/tmp/"
  [ "$status" -eq 2 ]
}

@test "blocks tar -czf x.tgz .env" {
  _run_bash "tar -czf x.tgz .env"
  [ "$status" -eq 2 ]
}

@test "blocks zip archive .env" {
  _run_bash "zip secrets.zip .env"
  [ "$status" -eq 2 ]
}

# ---------- Bash: copy/move where source is safe — ALLOWED ----------

@test "allows cp .env.example .env (template bootstrap)" {
  _run_bash "cp .env.example .env"
  [ "$status" -eq 0 ]
}

@test "allows cp .env.sample config/.env" {
  _run_bash "cp .env.sample config/.env"
  [ "$status" -eq 0 ]
}

@test "allows cp README.md docs/" {
  _run_bash "cp README.md docs/"
  [ "$status" -eq 0 ]
}

# ---------- Bash: echo/printf of secret-shaped vars — BLOCKED ----------

@test "blocks echo \$GITHUB_TOKEN" {
  _run_bash 'echo $GITHUB_TOKEN'
  [ "$status" -eq 2 ]
}

@test "blocks echo \${API_KEY}" {
  _run_bash 'echo ${API_KEY}'
  [ "$status" -eq 2 ]
}

@test "blocks echo \$DB_PASSWORD" {
  _run_bash 'echo $DB_PASSWORD'
  [ "$status" -eq 2 ]
}

@test "blocks echo \$PRIVATE_KEY" {
  _run_bash 'echo $PRIVATE_KEY'
  [ "$status" -eq 2 ]
}

@test "blocks printf \$SECRET_TOKEN" {
  _run_bash 'printf "%s" "$SECRET_TOKEN"'
  [ "$status" -eq 2 ]
}

@test "blocks echo \$AWS_ACCESS_KEY" {
  _run_bash 'echo $AWS_ACCESS_KEY'
  [ "$status" -eq 2 ]
}

# ---------- Bash: echo of benign vars — ALLOWED ----------

@test "allows echo \$PATH" {
  _run_bash 'echo $PATH'
  [ "$status" -eq 0 ]
}

@test "allows echo \$HOME" {
  _run_bash 'echo $HOME'
  [ "$status" -eq 0 ]
}

@test "allows echo hello" {
  _run_bash "echo hello"
  [ "$status" -eq 0 ]
}

# ---------- Bash: env|grep for secrets — BLOCKED ----------

@test "blocks env | grep TOKEN" {
  _run_bash "env | grep TOKEN"
  [ "$status" -eq 2 ]
}

@test "blocks printenv | grep secret" {
  _run_bash "printenv | grep -i secret"
  [ "$status" -eq 2 ]
}

@test "blocks env | grep password" {
  _run_bash "env | grep -i password"
  [ "$status" -eq 2 ]
}

# ---------- Bash: env usage without secret grep — ALLOWED ----------

@test "allows env (bare)" {
  _run_bash "env"
  [ "$status" -eq 0 ]
}

@test "allows env | grep PATH" {
  _run_bash "env | grep PATH"
  [ "$status" -eq 0 ]
}

# ---------- Edge cases ----------

@test "allows empty command" {
  local input='{"tool_name":"Bash","tool_input":{"command":""}}'
  run bash -c "echo '$input' | '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
}

@test "allows missing tool_name" {
  local input='{"tool_input":{"command":"ls"}}'
  run bash -c "echo '$input' | '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
}

@test "provides fix suggestion in block message" {
  _run_bash "cat .env"
  [[ "$output" == *"Suggestion:"* ]]
}

# ---------- Emit-event integration ----------

_run_bash_with_emit() {
  local cmd="$1"
  local input
  input=$(jq -nc --arg c "$cmd" '{tool_name: "Bash", tool_input: {command: $c}}')
  export CLAUDE_PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
  export VSDD_LOG_DIR="$EMIT_TMPDIR"
  run bash -c "echo '$input' | '$HOOK' 2>&1"
  unset CLAUDE_PLUGIN_ROOT VSDD_LOG_DIR
}

_run_read_with_emit() {
  local path="$1"
  local input
  input=$(jq -nc --arg p "$path" '{tool_name: "Read", tool_input: {file_path: $p}}')
  export CLAUDE_PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
  export VSDD_LOG_DIR="$EMIT_TMPDIR"
  run bash -c "echo '$input' | '$HOOK' 2>&1"
  unset CLAUDE_PLUGIN_ROOT VSDD_LOG_DIR
}

@test "emit: Read .env emits env_file_read_direct" {
  EMIT_TMPDIR=$(mktemp -d)
  _run_read_with_emit ".env"
  [ "$status" -eq 2 ]
  local logfile=$(ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null | head -1)
  [ -n "$logfile" ]
  [ "$(jq -r '.type' < "$logfile")" = "hook.block" ]
  [ "$(jq -r '.hook' < "$logfile")" = "protect-secrets" ]
  [ "$(jq -r '.reason' < "$logfile")" = "env_file_read_direct" ]
  [ "$(jq -r '.matcher' < "$logfile")" = "Read" ]
  rm -rf "$EMIT_TMPDIR"
}

@test "emit: cat .env emits env_file_read_shell" {
  EMIT_TMPDIR=$(mktemp -d)
  _run_bash_with_emit "cat .env"
  [ "$status" -eq 2 ]
  local logfile=$(ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null | head -1)
  [ "$(jq -r '.reason' < "$logfile")" = "env_file_read_shell" ]
  rm -rf "$EMIT_TMPDIR"
}

@test "emit: echo \$GITHUB_TOKEN emits secret_var_echo" {
  EMIT_TMPDIR=$(mktemp -d)
  _run_bash_with_emit 'echo $GITHUB_TOKEN'
  [ "$status" -eq 2 ]
  local logfile=$(ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null | head -1)
  [ "$(jq -r '.reason' < "$logfile")" = "secret_var_echo" ]
  rm -rf "$EMIT_TMPDIR"
}

@test "emit: cp .env /tmp emits env_file_copy" {
  EMIT_TMPDIR=$(mktemp -d)
  _run_bash_with_emit "cp .env /tmp/"
  [ "$status" -eq 2 ]
  local logfile=$(ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null | head -1)
  [ "$(jq -r '.reason' < "$logfile")" = "env_file_copy" ]
  rm -rf "$EMIT_TMPDIR"
}

@test "emit: allowed Read produces NO event" {
  EMIT_TMPDIR=$(mktemp -d)
  _run_read_with_emit "README.md"
  [ "$status" -eq 0 ]
  [ -z "$(ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null)" ]
  rm -rf "$EMIT_TMPDIR"
}

@test "emit: hook still blocks when CLAUDE_PLUGIN_ROOT is unset" {
  local input
  input=$(jq -nc --arg p ".env" '{tool_name: "Read", tool_input: {file_path: $p}}')
  run bash -c "unset CLAUDE_PLUGIN_ROOT; echo '$input' | '$HOOK' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
}

@test "emit: hook still blocks when emit-event path is broken" {
  local input
  input=$(jq -nc --arg p ".env" '{tool_name: "Read", tool_input: {file_path: $p}}')
  run bash -c "CLAUDE_PLUGIN_ROOT='/nonexistent' echo '$input' | '$HOOK' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BLOCKED"* ]]
}
