---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-06-16T00:00:00Z
last_amended: "2026-06-16 (v1.0) — initial creation (E-18 scope, S-18.10 deliverable). check-state-health settings.json CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70 verification per ADR-026 §Decision 5 + §F-11. Architect adjudicated this contract is required (F3 story pass-1, F-MAJOR-003): advisory-only, non-blocking, settings.json env-var presence and value-ceiling check."
phase: F3
inputs:
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
  - plugins/vsdd-factory/skills/check-state-health/SKILL.md
input-hash: "2d42b26"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-06"
capability: "CAP-032"
lifecycle_status: draft
introduced: v1.0-feature-context-durability-E18
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-6.25.001: check-state-health verifies CLAUDE_AUTOCOMPACT_PCT_OVERRIDE in settings.json and emits advisory if absent or value exceeds 80

## Description

The `check-state-health` skill reads the active `settings.json` (project-local `.claude/settings.json` preferred; global `~/.claude/settings.json` as fallback; absent = no-file condition) and inspects the `env` block for `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`. If the key is absent or its string value is a number greater than 80, the skill emits a human-readable advisory naming the setting and providing a remediation hint. If the key is present with a numeric value ≤ 80, the skill reports GREEN with no advisory. This check is advisory-only: it never blocks the skill output and never blocks the calling session. It is consistent with the diagnostic-only contract of the `check-state-health` skill.

## Preconditions

1. The `check-state-health` skill has been invoked (human or orchestrator trigger).
2. The skill has read-access to the filesystem at the expected `settings.json` paths.
3. No other `check-state-health` checks are required to pass before this check executes — it runs as part of the standard check sequence.

## Postconditions

1. **PC1 — Key absent: emit advisory.** If `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` is absent from the `env` block of the resolved `settings.json` (or no `settings.json` exists), the skill emits an advisory row in the check table:
   - Check name: `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`
   - Status: `ADVISORY`
   - Details: `"Missing — add env: {CLAUDE_AUTOCOMPACT_PCT_OVERRIDE: \"70\"} to .claude/settings.json (ADR-026 §Decision 5: proactive compaction threshold; 70% gives PreCompact flush headroom)"`
   - The advisory does NOT block any subsequent checks or the overall skill output.

2. **PC2 — Key present, value > 80: emit advisory.** If `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` is present in the `env` block but its numeric value exceeds 80, the skill emits an advisory row:
   - Check name: `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`
   - Status: `ADVISORY`
   - Details: `"Value <N> exceeds ADR-026 §Decision 5 ceiling of 80 (MEDIUM-confidence 83% harness cap); recommend 70 for safe PreCompact flush headroom"`
   - where `<N>` is the actual configured value.
   - The advisory does NOT block.

3. **PC3 — Key present, value ≤ 80: GREEN.** If `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` is present and its numeric value is ≤ 80, the skill emits a passing row:
   - Check name: `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`
   - Status: `PASS`
   - Details: `"Present, value <N> ≤ 80 (70 is canonical per ADR-026 §Decision 5)"` where `<N>` is the actual value.
   - No advisory emitted.

4. **PC4 — settings.json path resolution.** The skill resolves the settings.json path with the following precedence:
   a. Project-local: `.claude/settings.json` relative to the project root (i.e., the directory containing `.factory/`). If this file exists and is readable, it is the authoritative source.
   b. Global: `~/.claude/settings.json`. Used only when the project-local file does not exist.
   c. Neither present: treated as key-absent condition (PC1 advisory fires); the details message additionally notes `"(no settings.json found at .claude/settings.json or ~/.claude/settings.json)"`.

5. **PC5 — Advisory-only semantics (non-blocking).** This check is a diagnostic advisory. It does not cause the skill to exit with a non-zero status, does not block the calling session, and does not block any E-18 skill or pipeline operation. The overall skill result (`HEALTHY`, `WARNINGS`, `NEEDS-COMPACT`) MAY be elevated to `WARNINGS` when this advisory fires if no other advisory has already done so — consistent with existing check-state-health advisory escalation semantics. It does NOT cause `NEEDS-COMPACT`.

## Invariants

1. **Advisory-only, never blocking.** The `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` check MUST NOT emit `exit 2`, set `block_intent=true`, or prevent any downstream operation. It is a passive diagnostic, not a guard. Implementations that block are a specification violation.

2. **Precedence-stable path resolution.** Project-local `.claude/settings.json` always takes precedence over global `~/.claude/settings.json`. If both exist, the global file is never read for this check. This mirrors Claude Code's own settings resolution order.

3. **Numeric value comparison only.** The `env` block value for `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` in `settings.json` is a string (JSON strings for env values). The skill parses the string as a decimal integer for comparison. Non-numeric or empty values (e.g., `""`, `"auto"`) are treated as absent (PC1 fires) with a note in the advisory: `"Value '<raw>' is not a valid integer; treating as absent"`.

4. **No side effects.** The check reads `settings.json` only; it does NOT write, modify, or create any file. It does not write to `factory-artifacts`. It does not emit events into the dispatcher log.

5. **Row always emitted.** A check table row for `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` MUST always appear in the `check-state-health` output table — PASS, ADVISORY, or error-notation. Silent omission of the row is a specification violation.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `.claude/settings.json` exists; `env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE: "70"` | PASS row; value 70 ≤ 80; no advisory |
| EC-002 | `.claude/settings.json` exists; `env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` absent from `env` block | ADVISORY row PC1; remediation hint shown |
| EC-003 | `.claude/settings.json` exists; `env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE: "85"` | ADVISORY row PC2; value 85 > 80; recommend 70 |
| EC-004 | `.claude/settings.json` exists; `env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE: "80"` | PASS row; value 80 ≤ 80 (boundary — 80 is acceptable per ceiling definition) |
| EC-005 | `.claude/settings.json` absent; `~/.claude/settings.json` exists with `env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE: "70"` | PASS row; global fallback used; value 70 ≤ 80 |
| EC-006 | Both `.claude/settings.json` and `~/.claude/settings.json` absent | ADVISORY row PC1 + PC4(c); note "no settings.json found at .claude/settings.json or ~/.claude/settings.json" |
| EC-007 | `.claude/settings.json` exists; `env` block is absent entirely (no `env:` key in settings.json) | ADVISORY row PC1; key absent from env block |
| EC-008 | `.claude/settings.json` exists; `env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE: ""` (empty string) | ADVISORY row PC1 (non-numeric empty value treated as absent per Invariant 3); note in advisory |
| EC-009 | `.claude/settings.json` exists; `env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE: "auto"` (non-integer) | ADVISORY row PC1 (non-numeric value treated as absent per Invariant 3); note "Value 'auto' is not a valid integer; treating as absent" |
| EC-010 | Both project-local and global settings.json exist; project-local has value 85 (advisory); global has value 70 (PASS) | Project-local takes precedence; ADVISORY row PC2 fires for value 85 — global file not consulted |
| EC-011 | `.claude/settings.json` exists but is malformed JSON (parse error) | ADVISORY row; details note "settings.json parse error: <error>; cannot verify CLAUDE_AUTOCOMPACT_PCT_OVERRIDE" |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `.claude/settings.json`: `{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "70"}}` | Row: `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE \| PASS \| Present, value 70 ≤ 80 (70 is canonical per ADR-026 §Decision 5)` | happy-path (canonical value) |
| `.claude/settings.json`: `{"env": {}}` (env block present but key absent) | Row: `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE \| ADVISORY \| Missing — add env: {CLAUDE_AUTOCOMPACT_PCT_OVERRIDE: "70"} to .claude/settings.json (ADR-026 §Decision 5: proactive compaction threshold; 70% gives PreCompact flush headroom)` | absent-key |
| `.claude/settings.json`: `{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "85"}}` | Row: `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE \| ADVISORY \| Value 85 exceeds ADR-026 §Decision 5 ceiling of 80 (MEDIUM-confidence 83% harness cap); recommend 70 for safe PreCompact flush headroom` | value-exceeds-ceiling |
| `.claude/settings.json`: `{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "80"}}` | Row: `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE \| PASS \| Present, value 80 ≤ 80 (70 is canonical per ADR-026 §Decision 5)` | boundary-inclusive |
| Neither settings.json present | Row: `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE \| ADVISORY \| Missing — add env: {CLAUDE_AUTOCOMPACT_PCT_OVERRIDE: "70"} to .claude/settings.json (ADR-026 §Decision 5: proactive compaction threshold; 70% gives PreCompact flush headroom) (no settings.json found at .claude/settings.json or ~/.claude/settings.json)` | no-settings-file |
| `.claude/settings.json` absent; `~/.claude/settings.json`: `{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "70"}}` | Row: `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE \| PASS \| Present, value 70 ≤ 80 (70 is canonical per ADR-026 §Decision 5)` — global fallback used | global-fallback |
| `.claude/settings.json`: malformed JSON | Row: `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE \| ADVISORY \| settings.json parse error: <error>; cannot verify CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` | parse-error |

## Related BCs

- BC-6.24.001 — sibling (same E-18 feature, CAP-032): rehydrate-wave loads wave-state.yaml for session rehydration; this BC covers the pre-session settings verification that ensures the compaction threshold prerequisite is met
- BC-1.15.001 — sibling (CAP-032, SS-01): dispatcher PreCompact hook routing; this BC covers the operator-side env-var prerequisite that enables the PreCompact hook to fire at the intended 70% threshold

## Architecture Anchors

- `plugins/vsdd-factory/skills/check-state-health/SKILL.md` — the skill being extended by S-18.10; this check is added as Check 8 (settings.json env-var verification)
- ADR-026 §Decision 5 ("Proactive compaction threshold: 70% via CLAUDE_AUTOCOMPACT_PCT_OVERRIDE (D5 LOCKED)") — normative source of the 70% canonical value and the advisory requirement
- ADR-026 §F-11 ("Settings.json env-verification (F-11): The `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` setting requires verification in the active `settings.json`. The `check-state-health` skill must verify this env var is present with value `70` in `settings.json` and emit an advisory if absent.") — the explicit mandate that created this story

## Story Anchor

S-18.10 (check-state-health settings.json CLAUDE_AUTOCOMPACT_PCT_OVERRIDE verification; E-18 scope per architect adjudication F3 story pass-1 F-MAJOR-003)

## VP Anchors

- VP-092 — to be authored by architect (check-state-health CLAUDE_AUTOCOMPACT_PCT_OVERRIDE settings.json verification; advisory-only; value-ceiling check; path-resolution precedence)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-092 | check-state-health reads CLAUDE_AUTOCOMPACT_PCT_OVERRIDE from project-local settings.json (with global fallback), emits ADVISORY when absent or value > 80, emits PASS when value ≤ 80, never blocks, always emits a check table row | integration (bats: unit test each of PC1/PC2/PC3/PC4/EC-006/EC-010/EC-011 with fixture settings.json files) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC specifies the operator-facing prerequisite verification step that ensures `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` is present in settings.json. Without this setting, the proactive auto-compaction that gives the PreCompact flush sufficient headroom to complete (ADR-026 §Decision 5) is not configured, degrading the mid-wave compaction losslessness guarantee that CAP-032 mandates. |
| L2 Domain Invariants | DI-020 ("Wave/phase boundary transitions must not lose load-bearing pipeline state") — this check verifies that the operator configuration prerequisite for the PreCompact flush safety net is in place; an unconfigured or misconfigured compaction threshold undermines the Part B (mid-wave compaction) arm of DI-020 |
| Architecture Module | SS-06 (Skill Catalog) — check-state-health skill extension in `plugins/vsdd-factory/skills/check-state-health/` |
| ADR | ADR-026 §Decision 5 (CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70; value ceiling 80; proactive compaction threshold LOCKED); ADR-026 §F-11 (settings.json env-verification requirement; check-state-health mandate) |
| Stories | S-18.10 |
| Cycle | v1.0-feature-context-durability-E18 (F3) |
| Feature | issue #173 / E-18 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.0 | 2026-06-16 | product-owner | Initial creation. check-state-health settings.json CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70 verification per ADR-026 §Decision 5 + §F-11. Architect adjudicated as required BC (F3 story pass-1, F-MAJOR-003). 4 postconditions (PC1–PC4) plus PC5 advisory-only semantics; 5 invariants; 11 edge cases; 7 canonical test vectors. |
