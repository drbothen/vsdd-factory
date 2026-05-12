---
document_type: behavioral-contract
level: L3
version: "v1.0"
status: draft
producer: product-owner
timestamp: 2026-05-12T00:00:00Z
phase: F2
cycle: v1.0-feature-block-ai-attribution-message-file-arm
inputs:
  - .factory/feature-delta/F-block-ai-attribution-message-file-arm/F1-delta-analysis.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/domain-spec/invariants.md
input-hash: "[pending-recompute]"
traces_to: domain-spec/L2-INDEX.md
origin: greenfield
extracted_from: null
subsystem: "SS-07"
capability: "CAP-008"
lifecycle_status: active
introduced: v1.0.0-rc.17
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-7.03.094: block-ai-attribution: PostToolUse retroactive HEAD commit message verification

## Description

After a `git commit` bash command completes successfully, the `block-ai-attribution` WASM
plugin fires as a PostToolUse hook, calls `vsdd::exec_subprocess("git", ["log", "-1",
"--format=%B", "HEAD"], ...)` to retrieve the most recent commit message from HEAD, and
runs `detect_attribution` on the result. If attribution patterns are found, the hook
returns `HookResult::Block` with a remediation instruction pointing the agent to
`git reset --soft HEAD~1`. Any subprocess failure (timeout, capability denial, non-zero
exit, not-found) falls back to `HookResult::Continue` — false-positive prevention is
the highest-priority invariant of this contract.

**Hook artifact:** `crates/hook-plugins/block-ai-attribution/src/lib.rs` (PostToolUse arm)  
**Registry entry:** `hooks-registry.toml::block-ai-attribution` (events=[PreToolUse, PostToolUse], tool=Bash, priority=20, on_error=block)

## Preconditions

1. **PRE-1 — Event type gate:** `hook_event_name == "PostToolUse"` AND `tool_name == "Bash"`.
   Events of any other type or tool (e.g., FileWrite, Read, Edit) do not trigger this arm.

2. **PRE-2 — Precise command gate:** `tool_input.command` contains the token sequence
   `git commit` where the token immediately following `git` is exactly `commit` — not
   `commit-tree`, `commit-graph`, or any other git subcommand that begins with `commit`.
   Detection rule: split command string on whitespace, locate the token `git`, verify the
   immediately following token is exactly `commit`. If no such exact sequence is present,
   this arm does not fire.

3. **PRE-3 — Success-only gate (OQ-F1-003 resolved: success-only):** `tool_response`
   indicates the commit succeeded. Specifically: the tool response exit_code field is 0,
   OR the tool response contains a HEAD-advance signal (e.g., output matches pattern
   `[branch <hash>]` or similar git commit success output). When the commit failed (e.g.,
   pre-commit hook rejected it, git returned non-zero), this arm does not fire — the
   commit did not advance HEAD, so there is nothing to retroactively check, and
   double-blocking is avoided.

## Postconditions

1. **PC-1 — Attribution detected → Block:** When `detect_attribution(stdout)` returns
   `Some(attribution)` (where `stdout` is the output of `git log -1 --format=%B HEAD`),
   the hook returns:
   ```
   HookResult::block_with_fix(
     plugin_name: "block-ai-attribution",
     reason:      <human-readable description of the matched pattern>,
     fix:         "Run `git reset --soft HEAD~1`, edit the message to remove the attribution
                   lines, and recommit.",
     event_code:  "ai_attribution_post_commit",
   )
   ```
   The `reason` field must cite the matched attribution pattern type (e.g.,
   "Co-Authored-By attribution found in HEAD commit message").

2. **PC-2 — Clean HEAD → Continue:** When `detect_attribution(stdout)` returns `None`
   (no attribution patterns matched), the hook returns `HookResult::Continue` and
   no block event is emitted.

3. **PC-3 — Subprocess failure → Continue + telemetry (CRITICAL — fail-open invariant):**
   When `vsdd::exec_subprocess` returns any error variant — including but not limited to
   `CAPABILITY_DENIED`, `TIMEOUT`, `OUTPUT_TOO_LARGE`, `NOT_FOUND` (git binary absent),
   or a non-zero git exit code — the hook MUST return `HookResult::Continue`. A telemetry
   event of type `internal.exec_subprocess_failed` (or equivalent dispatcher-defined
   internal event) MUST be emitted with fields: `plugin_name`, `arm`, `error_variant`,
   `timeout_ms`. Under no circumstances may a subprocess failure produce
   `HookResult::Block`. This postcondition has the same severity as PC-1 because a
   false-positive block on a legitimate commit is a P0 reliability regression.

4. **PC-4 — Empty stdout → Continue:** When `vsdd::exec_subprocess` succeeds (exit 0)
   but returns zero bytes of stdout (e.g., empty repository, no commits), the hook
   treats this as a clean result and returns `HookResult::Continue`. An empty string
   fed to `detect_attribution` must return `None`.

## Invariants

1. **INV-1 — Fallback-to-Continue on any subprocess error (HIGH PRIORITY):** The hook
   MUST NOT produce `HookResult::Block` as a consequence of subprocess infrastructure
   failure. This invariant is the primary safety constraint of the PostToolUse arm.
   Subprocess errors are handled at the call site via `match` or `?`-operator into a
   telemetry + Continue path, never propagated to the detection logic.

2. **INV-2 — Lookalike command non-trigger:** The hook MUST NOT call
   `vsdd::exec_subprocess` when the command matches `git commit-tree`, `git commit-graph`,
   `git commit-msg`, or any other git subcommand that is not exactly `git commit`.
   The exact-token check in PRE-2 enforces this. There must be no shell-out for any
   command that does not satisfy PRE-2.

3. **INV-3 — Subprocess timeout fixed at 1000ms (OQ-F1-001 resolved: 1000ms):** The
   `timeout_ms` parameter to `vsdd::exec_subprocess` is fixed at `1000`. This value
   complies with ADR-020 Class A (p95 ≤ 1500ms for binary-spawn model). The value is
   not configurable at runtime; it is a compile-time constant in the plugin.

4. **INV-4 — max_output_bytes fixed at 65536:** The `max_output_bytes` parameter to
   `vsdd::exec_subprocess` is fixed at 65536 (64 KiB). A single commit message will
   always fit within this budget. If the output exceeds this limit (pathological case),
   the call returns `OUTPUT_TOO_LARGE`, which is handled per INV-1 (Continue + telemetry).

5. **INV-5 — Single exec_subprocess call per PostToolUse invocation:** The hook MUST
   NOT issue more than one `exec_subprocess` call per hook firing. There is no retry
   logic. The single call either succeeds and drives the detection, or fails and
   falls back to Continue.

6. **INV-6 — detect_attribution is pure:** The `detect_attribution` function has no I/O
   side effects. It is called on the stdout string after the subprocess returns. Its
   behavior is identical regardless of whether the string originated from a `-m`
   command argument, a `-F` file, or a `git log` capture.

7. **INV-7 — Known residual gap (advisory):** An agent could embed AI attribution in
   `git config user.name` or `user.email` rather than the commit message. This BC does
   not cover that vector. It is out of scope for this feature. Documented for future
   consideration.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `git commit --allow-empty` via editor flow; HEAD body contains `Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>` | `detect_attribution` matches Co-Authored-By pattern in HEAD body → PC-1 Block |
| EC-002 | `git commit -F /tmp/msg.txt`; msg.txt contained attribution; commit succeeded | HEAD contains the attribution → PostToolUse arm detects via `git log` stdout → PC-1 Block (safety-net for BC-7.03.095 faster-feedback path) |
| EC-003 | `git commit -m "Fix login bug"` — no attribution | `detect_attribution` returns None → PC-2 Continue |
| EC-004 | `git commit-tree <tree> -p <parent> -m "msg"` | Token after `git` is `commit-tree` → PRE-2 fails → no exec_subprocess call → Continue |
| EC-005 | `git commit-graph write` | Token after `git` is `commit-graph` → PRE-2 fails → no exec_subprocess call |
| EC-006 | Failed commit: `git commit -m "..."` exits non-zero (pre-commit hook rejection) | PRE-3 fails → arm does not fire → Continue without subprocess |
| EC-007 | `vsdd::exec_subprocess` returns `TIMEOUT` after 1000ms | INV-1 + PC-3: HookResult::Continue, telemetry event emitted with `error_variant: TIMEOUT` |
| EC-008 | `vsdd::exec_subprocess` returns `CAPABILITY_DENIED` | PC-3: HookResult::Continue, telemetry emitted |
| EC-009 | `git log -1 --format=%B HEAD` returns exit 128 (orphan branch, no commits) | PC-3 (non-zero exit): HookResult::Continue, telemetry emitted |
| EC-010 | `git log -1 --format=%B HEAD` succeeds, stdout is empty (0 bytes) | PC-4: `detect_attribution("")` returns None → Continue |
| EC-011 | git binary not in PATH; exec_subprocess returns `NOT_FOUND` | PC-3: HookResult::Continue, telemetry emitted |

## Canonical Test Vectors

| ID | Input | Expected Output | Category |
|----|-------|-----------------|----------|
| TV-094-001 | PostToolUse; command=`git commit --allow-empty -m ""`; exit=0; HEAD body=`Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>` | `HookResult::Block`; event_code=`ai_attribution_post_commit` | happy-path (detection) |
| TV-094-002 | PostToolUse; command=`git commit -F /tmp/msg.txt`; exit=0; HEAD body=`Generated with [Claude Code](https://claude.com/claude-code)` | `HookResult::Block`; event_code=`ai_attribution_post_commit` | happy-path (safety-net) |
| TV-094-003 | PostToolUse; command=`git commit -m "Fix login bug"`; exit=0; HEAD body=`Fix login bug` | `HookResult::Continue` | happy-path (clean) |
| TV-094-004 | PostToolUse; command=`git commit-tree abc123 -p def456 -m "msg"`; exit=0 | `HookResult::Continue`; exec_subprocess NOT called | edge-case (lookalike non-trigger) |
| TV-094-005 | PostToolUse; command=`git commit -m "msg"`; exit=1 (failed commit) | `HookResult::Continue`; exec_subprocess NOT called | edge-case (failed commit non-trigger) |
| TV-094-006 | PostToolUse; command=`git commit -m "msg"`; exit=0; exec_subprocess returns TIMEOUT | `HookResult::Continue`; telemetry event emitted | error (subprocess timeout) |
| TV-094-007 | PostToolUse; command=`git commit -m "msg"`; exit=0; exec_subprocess returns CAPABILITY_DENIED | `HookResult::Continue`; telemetry emitted | error (capability denied) |
| TV-094-008 | PostToolUse; command=`git commit -m "msg"`; exit=0; exec_subprocess returns NOT_FOUND | `HookResult::Continue`; telemetry emitted | error (binary not found) |
| TV-094-009 | PostToolUse; command=`git commit -m "msg"`; exit=0; exec_subprocess stdout=`` (empty) | `HookResult::Continue` | edge-case (empty stdout) |
| TV-094-010 | PostToolUse; command=`git commit-graph write`; exit=0 | `HookResult::Continue`; exec_subprocess NOT called | edge-case (commit-graph non-trigger) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|--------------|
| VP-080 | `detect_attribution(s: &str)` correctly identifies all TV-001..011 patterns regardless of whether `s` came from `-m` argument, `-F` file body, or `git log -1 HEAD` stdout | proptest (1024 cases/run, pure-core) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-008 ("Gate tool calls with pre-execution behavioral checks (PreToolUse hooks)") per capabilities.md §CAP-008 |
| Capability Anchor Justification | CAP-008 ("Gate tool calls with pre-execution behavioral checks (PreToolUse hooks)") per capabilities.md §CAP-008 — this BC describes a behavioral gate on `git commit` results that detects AI attribution injection, which is explicitly listed as a covered use case in CAP-008's prose. |
| L2 Domain Invariants | DI-004 (capability denial produces return code + audit event), DI-005 (exec_subprocess requires binary_allow; shell_bypass_acknowledged not set) |
| Verification Properties | VP-080 (proptest coverage of detect_attribution pure function across all TV-001..011 patterns; 1024 cases/run; primary source BC for this VP) |
| Architecture Module | SS-07 (Hook Bash Layer) per ARCH-INDEX Subsystem Registry |
| Stories | S-16.01 (block-ai-attribution: PostToolUse retroactive HEAD verification) |
| introduced | v1.0.0-rc.17 |

## Related BCs

- BC-7.03.001 (composes with): identity and registry binding contract for block-ai-attribution; this BC adds a PostToolUse event to that registry entry
- BC-7.03.002 (depends on): git commit command-string substring gate; PRE-2 of this BC uses the same exact-token logic
- BC-7.03.003 (sibling): Co-Authored-By detection pattern; detect_attribution called by this BC covers same patterns
- BC-7.03.004 (sibling): Generated-with/email detection pattern; detect_attribution called by this BC covers same patterns
- BC-7.03.095 (sibling — complementary arm): PreToolUse -F file-read arm; this BC is the safety net that catches what BC-7.03.095 catches earlier

## Architecture Anchors

- `.factory/specs/architecture/SS-07-hook-bash.md` — block-ai-attribution module table row; PostToolUse event registration
- `.factory/specs/architecture/SS-04-plugin-ecosystem.md` — block-ai-attribution crate description; exec_subprocess capability surface

## Story Anchor

S-16.01 — block-ai-attribution: PostToolUse retroactive HEAD verification

## VP Anchors

- VP-080 — proptest coverage of detect_attribution across TV-001..011 patterns (primary anchor)

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.0 | 2026-05-12 | product-owner | Initial authoring. F2 spec evolution for F-block-ai-attribution-message-file-arm. OQ-F1-001 baked: timeout_ms=1000. OQ-F1-003 baked: success-only gate. |
