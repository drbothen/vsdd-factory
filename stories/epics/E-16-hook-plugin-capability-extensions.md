---
document_type: epic
epic_id: "E-16"
version: "v1.0"
status: draft
title: "Hook Plugin Capability Extensions — block-ai-attribution message-file arm"
prd_capabilities: [CAP-008]
subsystems_affected: [SS-04, SS-07]
target_release: "v1.0.0-rc.17"
story_count: 2
producer: product-owner
timestamp: 2026-05-12T00:00:00Z
phase: F2
cycle: v1.0-feature-block-ai-attribution-message-file-arm
depends_on: []
inputs:
  - .factory/feature-delta/F-block-ai-attribution-message-file-arm/F1-delta-analysis.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.094.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.095.md
  - .factory/specs/verification-properties/VP-080.md
input-hash: "[pending-recompute]"
---

# Epic E-16: Hook Plugin Capability Extensions — block-ai-attribution message-file arm

## Description

Extend the `block-ai-attribution` WASM plugin to close two commit-path bypass vectors
that allow AI attribution text to land in commits despite the existing PreToolUse
command-string gate. The feature adds two new behavioral arms — a PostToolUse retroactive
HEAD verification (Option C) and a PreToolUse `-F <path>` file-read arm (Option B) —
plus the host-function capability declarations (`exec_subprocess`, `read_file`) that make
them possible.

## Trigger / Motivation

**Concrete bypass demonstrated, 2026-05-12.**

The existing `block-ai-attribution` WASM plugin (shipped in S-3.03 at rc.1) inspects
only `tool_input.command` on PreToolUse. A real commit with attribution text in a
separate message file bypassed the gate entirely:

```
git commit -F /tmp/agent-commit-msg.txt
# /tmp/agent-commit-msg.txt contained:
# Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
```

The PreToolUse hook saw `git commit -F /tmp/...` in the command string, ran
`detect_attribution` on the command string (no attribution there), and returned
`HookResult::Continue`. The commit landed with the attribution line intact.

The same bypass is possible via editor-flow (`git commit` without `-m`, editor opens,
agent injects attribution into the editor buffer), heredoc commit messages that embed
attribution after the initial command-string scan, and the `-F -` stdin path.

Per F1 §3 and §5, the gap is confirmed. Two arms close it:

- **Option C (PostToolUse retroactive HEAD verification — S-16.01):** After every
  successful `git commit`, read HEAD via `git log -1 --format=%B HEAD` and run
  `detect_attribution` on the result. Path-agnostic: catches all bypass vectors because
  every commit path advances HEAD. Higher risk-reduction per point.

- **Option B (PreToolUse -F file-read arm — S-16.02):** Before `git commit -F <path>`
  executes, read the file at `<path>` via `vsdd::read_file` and run `detect_attribution`
  on its contents. Faster feedback than Option C — blocks before the commit lands.
  Option C remains the safety net for paths Option B cannot reach (stdin `-F -`,
  capability-denied reads, files outside `path_allow`).

## Epic Placement Justification

Per F1 §6:

**E-3 ("WASM Port — High-Value Hooks") is closed.** E-3 shipped four stories at
milestone rc.1. The project is now at rc.16. Reopening E-3 would violate POLICY 1
(append-only numbering / forward-only epic lifecycle) and is semantically wrong — E-3's
framing is "porting bash hooks to WASM." The `block-ai-attribution` plugin is already
native WASM; this feature extends its capability surface, not its runtime platform.

**E-11 ("Tier-3 native WASM migration") and E-14 ("Engine Discipline pass-2") are also
rejected** as homes for this work. E-11 is about migrating additional bash hooks;
E-14 is about governance process gaps. Neither describes plugin behavioral capability
extensions.

**E-16 is the next available ID** after E-15 (current ceiling) under POLICY 1
(append-only numbering). The conceptual scope — extending shipped WASM plugins with new
host-function capabilities — is coherent and likely to attract additional stories from
other plugins in future cycles. E-16 is a better long-term home than a one-off addendum
to a closed epic.

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-008 | Gate tool calls with pre-execution behavioral checks (PreToolUse hooks) | P0 |

## Capability Anchor Justification

**Primary anchor: CAP-008** ("Gate tool calls with pre-execution behavioral checks
(PreToolUse hooks)") per `domain-spec/capabilities.md` §CAP-008. CAP-008's prose
explicitly names "AI attribution injection" as a covered pattern for behavioral gating
hooks. Both stories in this epic extend the gating surface of the already-shipped
`block-ai-attribution` plugin: S-16.01 adds PostToolUse retroactive verification,
S-16.02 adds PreToolUse file-read inspection. The PostToolUse arm is an extension of
the same gating capability surface — it confirms what the PreToolUse arm should have
caught. No other capability better describes this behavior.

## Stories

| Story ID | Title | Option | Points | Depends On |
|----------|-------|--------|--------|-----------|
| S-16.01 | block-ai-attribution: PostToolUse retroactive HEAD verification | C | 5 | S-3.03 (merged) |
| S-16.02 | block-ai-attribution: PreToolUse -F file-read arm | B | 3 | S-16.01 |

**Sequencing rationale (C before B):** Option C is path-agnostic and closes the largest
residual risk surface in a single story. Option B is a faster-feedback optimization
layered on top. C-before-B also ensures S-16.01's proptest infrastructure for
`detect_attribution` (VP-080) is available when S-16.02's bats tests are written.

## Out of Scope

- **Lefthook / git-hooks approach:** Explicitly rejected per F1 analysis. The dispatcher
  architecture is the correct enforcement layer; a `commit-msg` git hook would be a
  parallel mechanism outside the VSDD pipeline and would require separate installation
  and maintenance per project.

- **Non-Claude AI commit paths:** Attribution from non-Claude AI tools (e.g., Copilot)
  is a separate concern. The `block-ai-attribution` detection patterns are Claude-specific.
  Extending detection to other AI tools is out of scope for this epic.

- **git config user.name/user.email attribution:** An agent could set `user.name` or
  `user.email` to an AI identity in `git config` rather than embedding attribution in the
  commit message. Neither arm of this epic catches that vector. Documented as a known
  residual gap in BC-7.03.094 INV-7. Out of scope for rc.17.

- **Retroactive audit of existing commits:** This epic only adds real-time gating on
  new commits. Scanning history for existing attribution is out of scope.

## Acceptance Criteria

1. `git commit -F /tmp/msg.txt` with any TV-001..011 attribution pattern in the file is
   blocked by S-16.02 (PreToolUse, before commit lands).
2. `git commit` via editor flow with attribution in the editor buffer is blocked by
   S-16.01 (PostToolUse, after commit lands, with `git reset --soft HEAD~1` remediation).
3. Any `vsdd::exec_subprocess` or `vsdd::read_file` failure during either arm produces
   `HookResult::Continue` — zero false-positive blocks on legitimate commits.
4. All existing BC-7.03.001..005 bats vectors pass unchanged (regression baseline).
5. VP-080 proptest suite passes: 1024 cases, all TV-001..011 patterns detected,
   no false positives on clean strings.

## Regression Risk Summary

Per F1 §8: **Risk level MEDIUM.**

Primary regression risk: the PostToolUse arm (S-16.01) adds a new code path that fires
after every successful `git commit`. If the fallback-to-Continue guard has a bug, the
arm could produce a spurious Block on a legitimate commit — a P0 reliability regression.
The `-F` arm (S-16.02) regression risk is LOW because fallback semantics are strong
(any read failure → Continue).

Critical regression vectors (must all pass after both stories merge):

- `git commit -m "clean message"` → Continue (PreToolUse, existing BC-7.03.002..004)
- `git commit -m "Co-Authored-By: Claude..."` → Block (existing)
- PostToolUse on clean HEAD → Continue (BC-7.03.094 PC-2)
- PostToolUse subprocess failure (timeout, NOT_FOUND, CAPABILITY_DENIED) → Continue (BC-7.03.094 INV-1)
- `git commit-tree` PostToolUse → Continue, no exec_subprocess called (BC-7.03.094 INV-2)

See F1 §8 for the full 16-entry regression checklist.

## Behavioral Contract Traceability

| BC ID | Title | Story |
|-------|-------|-------|
| BC-7.03.094 | block-ai-attribution: PostToolUse retroactive HEAD commit message verification | S-16.01 |
| BC-7.03.095 | block-ai-attribution: PreToolUse -F file-read arm | S-16.02 |
| BC-7.03.001 | block-ai-attribution: identity and registry binding (v1.3 — registry shape extended) | S-16.01 (registry change) |

## Verification Property Traceability

| VP ID | Title | Anchor Story |
|-------|-------|--------------|
| VP-080 | detect_attribution proptest coverage of TV-001..011 across all call sites | S-16.01 |

## Dependencies (External)

None. Both stories depend only on S-3.03 (block-ai-attribution WASM plugin, merged at
rc.1) which is already in the develop branch. No external service dependencies, no
third-party API contracts, no infrastructure provisioning required before implementation
begins.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.0 | 2026-05-12 | product-owner | Initial authoring. F2 spec evolution for F-block-ai-attribution-message-file-arm. |
