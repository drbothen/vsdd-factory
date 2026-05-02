---
document_type: security-audit
audit_id: OQ-6
story_id: S-8.09
epic_id: E-8
signoff_agent_id: security-reviewer
signoff_timestamp: "2026-05-02T00:00:00Z"
total_findings: 5
critical: 0
high: 0
medium: 2
low: 3
files_reviewed: 1
confirmed_subprocesses:
  - "jq (stdin parse — line 21, 31, 36)"
  - "jq (state file write — lines 74-75)"
  - "date (UTC timestamp — line 73)"
  - "${CLAUDE_PLUGIN_ROOT}/bin/emit-event (event emission — lines 26-27)"
recommended_binary_allow: []
recommended_capabilities:
  - read_file
  - write_file
  - emit_event
---

# OQ-6 Audit: regression-gate.sh subprocess capability profile

**Source:** `plugins/vsdd-factory/hooks/regression-gate.sh`
**Reviewed by:** security-reviewer
**Date:** 2026-05-02
**Purpose:** Pre-implementation gate for S-8.09 WASM port per E-8 epic OQ-6 and S-8.09 AC-010.

---

## Source: plugins/vsdd-factory/hooks/regression-gate.sh

87-line PostToolUse telemetry hook. Fires on every PostToolUse event; body-filters to
Bash tool commands only; further filters to 9 test-runner patterns; reads/writes a
`.factory/regression-state.json` state file; emits an advisory warning on pass-to-fail
transition.

---

## Findings

### SEC-001: CMD variable interpolated into emit-event arguments without sanitization
- **Severity:** MEDIUM
- **CWE:** CWE-88 (Argument Injection into Subprocess)
- **OWASP:** A03:2021 — Injection
- **Attack Vector:** The `$CMD` variable (line 80) is extracted from the PostToolUse
  envelope's `tool_input.command` field via `jq -r` (line 31). It is then passed as
  a positional argument to `${CLAUDE_PLUGIN_ROOT}/bin/emit-event` on lines 79-80:
  ```
  _emit type=hook.block hook=regression-gate matcher=Bash \
        reason=regression_gate_pass_to_fail severity=warn command="$CMD"
  ```
  While the variable is double-quoted, the `_emit` function passes all `$@` arguments
  to `emit-event` untyped (line 27). If `emit-event` parses its arguments by splitting
  on `=` or passes them to a shell downstream, a command string like
  `cargo test; rm -rf .factory` could inject unwanted tokens into `emit-event`'s
  argument processing.
- **Impact:** Argument injection into the `emit-event` binary. Severity is limited
  because (a) `emit-event` is a first-party binary, not a shell interpreter; (b) the
  hook exits 0 unconditionally regardless of `emit-event`'s behavior; and (c) the
  attack requires the host PostToolUse envelope itself to carry a malicious command
  string, meaning the attacker already has shell execution via Claude Code's Bash tool.
  In the WASM port, this finding is **fully mitigated**: `$CMD` is passed as a typed
  string field to `host::emit_event(fields: &[(&str, &str)])`, which uses a structured
  key-value encoding, not shell interpolation. No argument injection is possible at
  the WASM host fn boundary.
- **Evidence:** Line 31: `CMD=$(echo "$INPUT" | jq -r '.tool_input.command // empty')`
  Line 79-80: `_emit ... command="$CMD"`
- **Proposed Mitigation for bash source:** Not required — the bash source is deleted
  by AC-002 when the WASM port lands. For reference: validate `$CMD` against a
  known-safe pattern (e.g., allowlist prefixes) before passing to `_emit`. The WASM
  port eliminates this attack surface entirely.

---

### SEC-002: Unvalidated state file path written via shell redirection
- **Severity:** MEDIUM
- **CWE:** CWE-73 (External Control of File Name or Path)
- **OWASP:** A01:2021 — Broken Access Control
- **Attack Vector:** `STATE_FILE` is constructed as the hardcoded string
  `"$STATE_DIR/regression-state.json"` (line 46), where `STATE_DIR=".factory"` (line 45)
  is also hardcoded. There is no user-controlled path component and no path traversal
  risk in the bash source as written. However, `STATE_DIR` is a plain shell variable
  — if a future editor appends an environment variable override (e.g.,
  `STATE_DIR="${VSDD_STATE_DIR:-.factory}"`), a malicious `VSDD_STATE_DIR` value could
  redirect the write. In the current source, this is latent/theoretical.
- **Impact:** In the current source: none — paths are fully hardcoded. In a hypothetical
  patched version using an env override: write to arbitrary path under hook user
  permissions.
- **Evidence:** Line 45: `STATE_DIR=".factory"`, Line 46: `STATE_FILE="$STATE_DIR/regression-state.json"`,
  Line 75: `jq -n ... > "$STATE_FILE"`.
- **Proposed Mitigation for WASM port:** The WASM port MUST use a literal path string
  `".factory/regression-state.json"` in `host::write_file` calls. The path MUST NOT be
  derived from any environment variable or input field. The `path_allow` capability
  declaration enforces this at the sandbox boundary regardless.

---

### SEC-003: `jq` subprocess receives unsanitized stdin from hook envelope
- **Severity:** LOW
- **CWE:** CWE-20 (Improper Input Validation)
- **OWASP:** N/A (internal processing, no injection surface to jq)
- **Attack Vector:** The hook reads the entire PostToolUse JSON envelope via `cat`
  (line 20) and pipes it to `jq -r` with fixed filter expressions (lines 21, 31, 36).
  The `jq` filter strings themselves are literals — they are not constructed from
  user input. The risk is that a large or malformed envelope could cause `jq` to
  consume excessive memory or time. There is no code injection risk because `jq`
  filter expressions are compile-time literals.
- **Impact:** Denial of service (excessive jq runtime) if the hook envelope is
  pathologically large. In practice, the Claude Code dispatcher imposes `timeout_ms=5000`
  (registry line 135), limiting exposure.
- **Evidence:** Line 20: `INPUT=$(cat)`, Line 21: `TOOL=$(echo "$INPUT" | jq -r '.tool_name // empty')`.
- **Proposed Mitigation for WASM port:** Replaced entirely by `serde_json` deserialization
  in Rust (AC-009). The WASM sandbox enforces memory and time limits at the host level.
  No `jq` subprocess means this finding is fully eliminated in the port.

---

### SEC-004: `_emit` function silently no-ops if CLAUDE_PLUGIN_ROOT is unset or binary absent
- **Severity:** LOW
- **CWE:** CWE-754 (Improper Check for Unusual or Exceptional Conditions)
- **OWASP:** N/A
- **Attack Vector:** Lines 25-28 guard `bin/emit-event` with
  `[ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]`.
  If `CLAUDE_PLUGIN_ROOT` is empty or the binary is missing, the event is silently
  dropped and the hook returns 0. A hostile environment (or misconfigured install)
  could suppress regression warnings by removing or replacing `emit-event`.
- **Impact:** Silent loss of `hook.block severity=warn` events on pass-to-fail
  transitions. The state file is still written correctly (the guard only wraps
  event emission). Downstream consumers of the event (e.g., red-gate) would not
  receive the advisory signal.
- **Evidence:** Lines 25-29:
  ```bash
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
      "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
  ```
- **Proposed Mitigation for WASM port:** `host::emit_event` is a first-class host fn
  with a structured return value. The WASM port should propagate `HostError` on
  emit failure via `HookResult::Error` rather than silently suppressing. This is an
  improvement over the bash source behavior and aligns with EC-006 observability intent.

---

### SEC-005: Unguarded `date -u` subprocess
- **Severity:** LOW
- **CWE:** CWE-78 (OS Command Injection) — classified here only for completeness; actual
  risk is nil given no user input is involved
- **OWASP:** N/A
- **Attack Vector:** Line 73 invokes `date -u +"%Y-%m-%dT%H:%M:%SZ"` to generate a
  UTC timestamp. The format string is a literal. No user input reaches `date`. This is
  included for completeness per audit scope, not as a genuine risk.
- **Impact:** None. `date` is a standard POSIX utility called with a fixed format string.
  No injection surface exists. The finding is noted to confirm the subprocess was
  reviewed and dismissed.
- **Evidence:** Line 73: `TS=$(date -u +"%Y-%m-%dT%H:%M:%SZ")`.
- **Proposed Mitigation for WASM port:** Replace with `chrono` crate (or `std::time`)
  for UTC timestamp generation. Eliminates the subprocess entirely (AC-009 preferred
  profile).

---

## Subprocess Inventory (complete)

| Line(s) | Command | Arguments | Input Control | Injection Risk |
|---------|---------|-----------|---------------|----------------|
| 21 | `jq -r` | `.tool_name // empty` (literal) | stdin from envelope (attacker data) | None — filter is a literal, not constructed from input |
| 31 | `jq -r` | `.tool_input.command // empty` (literal) | stdin from envelope | None — filter is literal; output assigned to `$CMD` |
| 36 | `jq -r` | `.tool_response.exit_code ...` (literal) | stdin from envelope | None — filter is literal |
| 69 | `jq -r` | `.status // "unknown"` (literal) | reads state file on disk | None — filter is literal |
| 73 | `date -u` | `+"%Y-%m-%dT%H:%M:%SZ"` (literal) | none | None — format is a compile-time constant |
| 74-75 | `jq -n` | `--arg s "$STATUS" --arg t "$TS" --arg c "$CMD"` | `$STATUS` (controlled), `$TS` (controlled), `$CMD` (from envelope) | LOW — jq `--arg` safely quotes all values; no injection into jq filter |
| 26-27 | `${CLAUDE_PLUGIN_ROOT}/bin/emit-event` | `type=... command="$CMD"` | `$CMD` from envelope | MEDIUM — see SEC-001 |

**Critical observation:** regression-gate does NOT invoke any test runner. It is a
pure PostToolUse OBSERVER. The commands `cargo test`, `pytest`, etc. are values read
from the `tool_input.command` field of an already-completed Bash tool invocation.
The hook never spawns those commands. OQ-6's original concern ("appears to invoke
external test runners") is definitively resolved: CONFIRMED NOT INVOKED.

---

## Required capabilities for WASM port

### CONFIRMED subprocess list (bash source)
1. `jq` — stdin JSON parsing (lines 21, 31, 36, 69, 74-75)
2. `date` — UTC timestamp (line 73)
3. `${CLAUDE_PLUGIN_ROOT}/bin/emit-event` — event emission (lines 26-27)

### RECOMMENDED capability profile for WASM crate

The preferred WASM port (AC-009) eliminates all three bash-era subprocesses:
- `jq` → replaced by `serde_json` native deserialization
- `date` → replaced by `chrono` crate or `std::time::SystemTime`
- `bin/emit-event` → replaced by `host::emit_event` host fn (AC-008)

**`exec_subprocess`:** NOT required. `binary_allow` MUST be empty (`[]`).

**`read_file`:** Required. The hook reads `.factory/regression-state.json` to
determine prior test status (lines 68-69 in bash source). Path is
`.factory/regression-state.json`.

**`write_file`:** Required. The hook writes `.factory/regression-state.json` with
the new `{status, timestamp, command}` JSON (lines 74-75). This is the D-6 Option A
blocker — `host::write_file` does not exist in vsdd-hook-sdk as of v1.0;
S-8.10 must merge before S-8.09 can implement AC-005.

**`emit_event`:** Yes. The hook emits `hook.block severity=warn` on pass-to-fail
transition (lines 79-80). Replaces `bin/emit-event` per AC-008.

---

## Security concerns

### Path traversal
**NOT present.** `STATE_FILE` is constructed from two hardcoded strings (lines 45-46).
No user-controlled path component exists anywhere in the file. The WASM port must
preserve this by using a literal string in `host::write_file` and `host::read_file`
calls. The `path_allow` sandbox declaration is a defense-in-depth backstop.

### Command injection
**NOT present for the WASM port.** The bash source has a LOW-to-MEDIUM theoretical
surface at the `_emit` boundary (SEC-001) but this is fully eliminated by the
structured `host::emit_event` host fn API. The WASM sandbox prohibits any shell
invocation by design.

### Privilege escalation
**NOT present.** The hook has no elevated privilege usage, no `sudo`, no setuid paths.
It writes a single JSON file in the project-local `.factory/` directory. The WASM
sandbox capability model further constrains file writes to the declared `path_allow`.

### Input from attacker-controlled envelope
The PostToolUse envelope is produced by Claude Code's Bash tool; the
`tool_input.command` field is the literal command string that Claude Code ran. In a
fully trusted deployment this is not attacker-controlled. In a scenario where the
hook pipeline itself is exposed to untrusted input (e.g., CI pipelines that pipe
arbitrary JSON as hook envelopes), the `$CMD` value COULD contain shell metacharacters.
The WASM port's use of `serde_json` string deserialization and `host::emit_event`
structured fields provides safe handling regardless of `$CMD` content — no
interpolation, no shell expansion.

### jq `--arg` safety (bash source, line 74-75)
The state file write uses `jq -n --arg s "$STATUS" --arg t "$TS" --arg c "$CMD"`.
`jq`'s `--arg` flag safely quotes all values — it assigns them as JSON strings
without interpreting shell metacharacters. This is correct usage. No injection risk.

---

## Recommendation

**VERDICT: WASM port can use existing host fns (read_file, write_file, emit_event).
No new host fn is required. exec_subprocess is NOT needed.**

**Blocker:** `host::write_file` does not exist in vsdd-hook-sdk v1.0. S-8.09 is
blocked on S-8.10 (SDK extension) per E-8 D-6 Option A. This is an ABI gap, not a
security finding.

**Block-mode:** Advisory (HookResult::Continue + emit hook.block severity=warn).
The hook never blocks execution — it records state and warns. This is explicitly
specified in BC-7.03.071 (exit 0 always) and BC-7.03.075 (does NOT block, exit 0).
The WASM port must preserve this advisory-only mode.

**Preliminary profile reconciliation:** The S-8.09 story's preliminary profile
(story lines 117-123) proposed `binary_allow = ["jq"]` (or `[]` if serde_json is
used). This audit CONFIRMS the `[]` (empty) profile — `jq` is not needed in the
WASM port. The implementer should proceed with AC-009's preferred serde_json profile.
No deviation from AC-009 is warranted by this audit.

---

## Capability declaration block (suggested for hooks-registry.toml)

```toml
[[hooks]]
name = "regression-gate"
event = "PostToolUse"
plugin = "hook-plugins/regression-gate.wasm"
priority = 230
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [".factory/regression-state.json"]
max_bytes_per_call = 4096

[hooks.capabilities.write_file]
path_allow = [".factory/regression-state.json"]
max_bytes_per_call = 512

[hooks.capabilities]
emit_event = true
```

**Notes on the declaration:**

1. `exec_subprocess` section is intentionally ABSENT. No binary invocations are
   needed in the WASM port. Do not add `binary_allow`.
2. `read_file.max_bytes_per_call = 4096`: The state file is a 3-field JSON object
   (`{status, timestamp, command}`). Even with a 4096-character command string it
   will not exceed 4 KiB. This bound prevents unbounded reads from a corrupted or
   replaced state file.
3. `write_file.max_bytes_per_call = 512`: The written JSON is `{"status":"pass"|"fail",
   "timestamp":"2026-05-02T00:00:00Z","command":"<cmd>"}`. The command field from a
   typical test runner invocation is well under 400 bytes. 512 bytes is a safe upper
   bound.
4. `emit_event = true`: Required for BC-7.03.075 pass-to-fail warning emission.
5. `env_allow` is intentionally ABSENT. The WASM port has no environment variable
   dependencies — `CLAUDE_PLUGIN_ROOT` is used only to locate `bin/emit-event` in
   the bash source, and that dependency is eliminated by `host::emit_event`.

---

## AC-010 Falsifiable Check Satisfaction

This document satisfies all three conditions of S-8.09 AC-010:

1. File exists at documented path (`.worktrees/S-8.09/docs/oq-6-regression-gate-security-audit.md`
   — to be moved to `.factory/cycles/v1.0-brownfield-backfill/E-8-oq6-capability-profile.md`
   by the implementer per S-8.09 task sequencing).
2. `signoff_agent_id: security-reviewer` is non-empty (frontmatter above).
3. `recommended_binary_allow: []` is explicitly present (frontmatter above).

**Implementing agent instruction:** Before writing any Rust code for the regression-gate
crate (T-3), load this document and confirm:
- `recommended_binary_allow` is `[]` — do NOT add `binary_allow` to the registry entry.
- `recommended_capabilities` are `read_file`, `write_file`, `emit_event` only.
- Proceed with serde_json + chrono native approach (AC-009 preferred profile confirmed).
- Do NOT implement T-4 write path until S-8.10 (host::write_file SDK extension) merges.

---

## Risk Register Dispositions (E-8 D-11)

The E-8 Risk Register (D-11) contains 10 entries (R-8.01 through R-8.10). None are
classified as Category=security. All are operational, performance, or quality risks.
Relevant dispositions for this security audit scope:

| Risk | Description | Disposition |
|------|-------------|-------------|
| R-8.04 | Behavior-change drift during port | **mitigated** — capability profile confirms no subprocess capability is needed; implementer cannot accidentally add `exec_subprocess` without violating this audit's recommended profile |
| R-8.07 | bin/emit-event interaction | **mitigated** — `bin/emit-event` call site is fully replaced by `host::emit_event`; no security implication from deferral since the bash source's `_emit` guard already handles absence gracefully |

No security-category R-NNN entries exist in the E-8 Risk Register. No unmitigated
security risks block this story's progression.
