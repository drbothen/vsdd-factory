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

# BC-7.03.095: block-ai-attribution: PreToolUse `-F <path>` file-read arm

## Description

When a `git commit` command on PreToolUse includes a `-F <path>`, `--file=<path>`, or
`--file <path>` flag pointing to a commit message file, the `block-ai-attribution` WASM
plugin reads that file via `vsdd::read_file` and runs `detect_attribution` on its
contents. This arm activates only after the existing command-string `detect_attribution`
scan finds no attribution (short-circuit: the `-m` / heredoc path costs zero host calls
when attribution is already in the command string). Any file-read failure falls back to
`HookResult::Continue` — BC-7.03.094's PostToolUse safety net will catch what this arm
misses. False-positive prevention on IO error is the primary invariant.

**Hook artifact:** `crates/hook-plugins/block-ai-attribution/src/lib.rs` (PreToolUse -F arm)
**Registry entry:** `hooks-registry.toml::block-ai-attribution` (PreToolUse entry; read_file capability added with narrow path_allow per OQ-F1-002)

## Preconditions

1. **PRE-1 — Event type gate:** `hook_event_name == "PreToolUse"` AND `tool_name == "Bash"`.

2. **PRE-2 — git commit command gate:** `tool_input.command` contains the token sequence
   `git commit` where the token immediately following `git` is exactly `commit` (same
   exact-token rule as BC-7.03.002 and BC-7.03.094 PRE-2).

3. **PRE-3 — Command-string scan first (short-circuit):** The existing command-string
   `detect_attribution(command)` scan runs first. If it returns `Some(attribution)`, the
   hook blocks immediately (per existing BC-7.03.003/004 behavior) without calling
   `vsdd::read_file`. This arm activates only when the command-string scan returns `None`.

4. **PRE-4 — `-F` flag present:** The command string contains at least one of:
   - `-F <path>` (short form, space-separated)
   - `--file=<path>` (long form, equals-separated)
   - `--file <path>` (long form, space-separated)
   where `<path>` is a non-empty string that is not the stdin sentinel `-`.

5. **PRE-5 — Path is not stdin sentinel:** When the parsed path is exactly `-`, this arm
   does not fire. Stdin-sourced commit messages cannot be read by `vsdd::read_file`;
   BC-7.03.094's PostToolUse arm provides the safety net for that path.

## Postconditions

1. **PC-1 — Attribution in file → Block:** When `detect_attribution(file_contents)`
   returns `Some(attribution)`, the hook returns:
   ```
   HookResult::block_with_fix(
     plugin_name: "block-ai-attribution",
     reason:      <human-readable description of matched pattern>,
     fix:         "Edit the commit message file to remove the attribution lines, then retry.",
     event_code:  "ai_attribution_file_arm",
   )
   ```
   The `event_code` is `ai_attribution_file_arm` for all pattern types detected via this
   arm, regardless of whether the matched pattern is Co-Authored-By, Generated-with, or
   email. This distinguishes file-arm detections from command-string-arm detections in
   telemetry without duplicating event codes.

2. **PC-2 — Clean file → Continue:** When `detect_attribution(file_contents)` returns
   `None`, the hook returns `HookResult::Continue`.

3. **PC-3 — read_file failure → Continue + telemetry (CRITICAL — fail-open invariant):**
   When `vsdd::read_file` returns any error variant — including but not limited to
   `CAPABILITY_DENIED`, `NOT_FOUND`, `OUTPUT_TOO_LARGE` (file > 65536 bytes), or
   `IO_ERROR` — the hook MUST return `HookResult::Continue`. A telemetry event of type
   `internal.read_file_failed` MUST be emitted with fields: `plugin_name`, `arm`,
   `error_variant`, `path`. Under no circumstances may a read failure produce
   `HookResult::Block`. BC-7.03.094 provides the PostToolUse safety net.

4. **PC-4 — Stdin sentinel → Continue:** When the parsed path is exactly `-`, the hook
   returns `HookResult::Continue` without calling `vsdd::read_file` (per PRE-5).

## Invariants

1. **INV-1 — Fallback-to-Continue on any read failure (HIGH PRIORITY):** The hook MUST
   NOT produce `HookResult::Block` as a consequence of file IO failure. Read errors are
   handled at the call site into a telemetry + Continue path.

2. **INV-2 — Command-string scan runs first (short-circuit):** The file-read arm MUST
   NOT call `vsdd::read_file` if the command-string `detect_attribution` scan already
   returned `Some`. This preserves zero host-call cost for the common case where
   attribution is already visible in the command string (e.g., `-m "Co-Authored-By: ..."`).

3. **INV-3 — Narrow path_allow (OQ-F1-002 resolved: narrow):** The `read_file` capability
   declaration in `hooks-registry.toml` uses the narrow path_allow list:
   ```
   path_allow = [
     "**/.git/COMMIT_EDITMSG",
     "/tmp/**",
     "/var/folders/**",
     "<project-root>/**"
   ]
   ```
   Any path outside this allowlist causes `vsdd::read_file` to return `CAPABILITY_DENIED`,
   which is handled per INV-1 (Continue + telemetry). The narrow allowlist is appropriate
   because block-ai-attribution has `on_error = "block"` (security-relevant plugin) and
   has no legitimate reason to read arbitrary filesystem paths. All realistic commit
   message file locations (editor temp files, `.git/COMMIT_EDITMSG`) are covered.

4. **INV-4 — max_bytes fixed at 65536:** The `max_bytes` parameter to `vsdd::read_file`
   is fixed at 65536 (64 KiB). If the file is larger, `read_file` returns
   `OUTPUT_TOO_LARGE`, handled per INV-1.

5. **INV-5 — Single read_file call per hook invocation:** The hook issues at most one
   `vsdd::read_file` call per PreToolUse firing. Only the first `-F`/`--file` flag is
   used if multiple are present (git itself only accepts one; duplicates are a malformed
   command).

6. **INV-6 — Relative path resolution:** When the parsed path is relative (does not begin
   with `/`), it is passed to `vsdd::read_file` as-is. The dispatcher resolves relative
   paths against the `cwd` provided in the hook envelope if available; if `cwd` is absent,
   the path is passed through unchanged (the dispatcher's read_file host function owns
   the resolution semantics). This BC does not perform its own path canonicalization.

7. **INV-7 — Three flag forms handled:** Path extraction handles all three syntactic
   forms: `-F <path>` (short, space-separated), `--file=<path>` (long, equals), and
   `--file <path>` (long, space-separated). The extraction logic is order-independent
   and handles quoted paths.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `git commit -F /tmp/msg.txt`; msg.txt contains `Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>` | read_file succeeds; detect_attribution matches → PC-1 Block; event_code=`ai_attribution_file_arm` |
| EC-002 | `git commit --file=/tmp/msg.txt`; msg.txt contains attribution (equals form) | Same as EC-001; `--file=` form parsed correctly |
| EC-003 | `git commit --file /tmp/msg.txt`; msg.txt contains attribution (space form) | Same as EC-001; `--file ` (space) form parsed correctly |
| EC-004 | `git commit -F -` (stdin sentinel) | PRE-5: arm does not fire → Continue; no read_file call |
| EC-005 | `git commit -F /nonexistent/path.txt` | read_file returns NOT_FOUND → PC-3: Continue + telemetry |
| EC-006 | `git commit -F /tmp/huge_msg.txt` (file > 65536 bytes) | read_file returns OUTPUT_TOO_LARGE → PC-3: Continue + telemetry |
| EC-007 | `git commit -F /etc/passwd` (outside path_allow) | read_file returns CAPABILITY_DENIED → PC-3: Continue + telemetry |
| EC-008 | `git commit -m "Co-Authored-By: Claude..."` (attribution in command string) | PRE-3 short-circuit: command-string scan fires first, returns Some → Block immediately; read_file NOT called |
| EC-009 | `git commit -F /tmp/msg.txt`; msg.txt is empty | read_file succeeds; detect_attribution("") returns None → PC-2 Continue |
| EC-010 | `git commit -F .git/COMMIT_EDITMSG`; file contains attribution | Path matches `**/.git/COMMIT_EDITMSG` allowlist entry; read_file succeeds; detect_attribution matches → PC-1 Block |
| EC-011 | `git commit -F /tmp/msg.txt`; msg.txt is clean | read_file succeeds; detect_attribution returns None → PC-2 Continue |

## Canonical Test Vectors

| ID | Input | Expected Output | Category |
|----|-------|-----------------|----------|
| TV-095-001 | PreToolUse; command=`git commit -F /tmp/msg.txt`; file contains `Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>` | `HookResult::Block`; event_code=`ai_attribution_file_arm` | happy-path (short -F form) |
| TV-095-002 | PreToolUse; command=`git commit --file=/tmp/msg.txt`; file contains attribution | `HookResult::Block`; event_code=`ai_attribution_file_arm` | happy-path (--file= form) |
| TV-095-003 | PreToolUse; command=`git commit --file /tmp/msg.txt`; file contains attribution | `HookResult::Block`; event_code=`ai_attribution_file_arm` | happy-path (--file space form) |
| TV-095-004 | PreToolUse; command=`git commit -F -` (stdin) | `HookResult::Continue`; read_file NOT called | edge-case (stdin sentinel) |
| TV-095-005 | PreToolUse; command=`git commit -F /nonexistent` | `HookResult::Continue`; telemetry emitted | error (file not found) |
| TV-095-006 | PreToolUse; command=`git commit -F /tmp/oversized`; file > 65536 bytes | `HookResult::Continue`; telemetry emitted | error (output too large) |
| TV-095-007 | PreToolUse; command=`git commit -m "Co-Authored-By: Claude..."` | `HookResult::Block` via command-string arm; read_file NOT called | edge-case (short-circuit) |
| TV-095-008 | PreToolUse; command=`git commit -F /tmp/clean_msg.txt`; file is clean | `HookResult::Continue` | happy-path (clean file) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|--------------|
| VP-080 | `detect_attribution(s: &str)` correctly identifies all TV-001..011 patterns when `s` is file contents; same pure function reused from PostToolUse arm | proptest (1024 cases/run, pure-core) — S-16.02 reuses the harness from S-16.01; no new VP required |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-008 ("Gate tool calls with pre-execution behavioral checks (PreToolUse hooks)") per capabilities.md §CAP-008 |
| Capability Anchor Justification | CAP-008 ("Gate tool calls with pre-execution behavioral checks (PreToolUse hooks)") per capabilities.md §CAP-008 — this BC describes a PreToolUse behavioral gate that blocks `git commit` before execution when the commit message file contains AI attribution injection, which is explicitly listed as a covered use case in CAP-008's prose. |
| L2 Domain Invariants | DI-004 (capability denial produces return code + audit event), DI-005 (read_file requires path_allow declaration; shell_bypass_acknowledged not applicable) |
| Verification Properties | VP-080 (same proptest harness reused from S-16.01; no new VP; covers detect_attribution on file-arm inputs across all TV-001..011 patterns) |
| Architecture Module | SS-07 (Hook Bash Layer) per ARCH-INDEX Subsystem Registry |
| Stories | S-16.02 (block-ai-attribution: PreToolUse -F file-read arm) |
| introduced | v1.0.0-rc.17 |

## Related BCs

- BC-7.03.001 (composes with): identity and registry binding; this BC adds read_file capability to the PreToolUse registry entry
- BC-7.03.002 (depends on): git commit command-string substring gate; PRE-2 uses same exact-token rule
- BC-7.03.003 (sibling): Co-Authored-By detection; detect_attribution called here covers same patterns
- BC-7.03.004 (sibling): Generated-with/email detection; detect_attribution called here covers same patterns
- BC-7.03.094 (sibling — safety net): PostToolUse retroactive HEAD verification; catches -F commits that bypass this arm (e.g., -F - stdin, capability denied, file outside path_allow)

## Architecture Anchors

- `.factory/specs/architecture/SS-07-hook-bash.md` — block-ai-attribution module table; PreToolUse read_file capability addition
- `.factory/specs/architecture/SS-04-plugin-ecosystem.md` — block-ai-attribution crate; read_file capability surface

## Story Anchor

S-16.02 — block-ai-attribution: PreToolUse -F file-read arm

## VP Anchors

- VP-080 — proptest coverage of detect_attribution; S-16.02 reuses VP-080 harness (no new VP)

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.0 | 2026-05-12 | product-owner | Initial authoring. F2 spec evolution for F-block-ai-attribution-message-file-arm. OQ-F1-002 baked: narrow path_allow. OQ-F1-003 baked: PreToolUse arm fires before commit (no success gate needed). |
