---
document_type: behavioral-contract
level: L3
version: "1.3"
status: draft
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
last_amended: 2026-05-09
phase: F2
inputs:
  - .factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/adversary-pass-1.md
  - plugins/vsdd-factory/hooks/hooks.json.template
  - .factory/specs/architecture/decisions/ADR-019-plugin-async-semantics-at-registry-layer.md
input-hash: "[to-be-computed-by-state-manager]"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-09"
capability: "CAP-007"
lifecycle_status: active
introduced: v1.0-feature-plugin-async-semantics-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-9.01.006: hooks.json.template envelope is uniformly synchronous — every event entry has the `async` key absent (or `async: false`); no entry has `async: true`

## Description

ADR-019 §Decision 1 moves async semantics from the Claude Code hook envelope to the dispatcher-internal registry layer. As a direct consequence, `hooks.json.template` (and all per-platform variants) must not carry any `"async": true` entries. Every event registration in the template — including PostToolUse, Stop, and SubagentStop, which previously carried `"async": true` — must have the `async` key absent. This BC formalizes that post-ADR-019 invariant and assigns SS-09 as the enforcement owner. Verified at template-render time and in CI.

## Preconditions

1. `hooks.json.template` exists in the repository at `plugins/vsdd-factory/hooks/hooks.json.template`.
2. ADR-019 has been adopted (schema_version = 2 dispatcher deployed).
3. All event entries in the template file have been rendered or stamped for the current deployment.

## Postconditions

1. Every event entry in `hooks.json.template` has the `"async"` key **absent**. No entry has `"async": true`. No entry has `"async": false` (the key must be absent, not set to false — its absence is the canonical form per Claude Code hook schema; setting it false is acceptable but not required).
2. This applies to ALL event types enumerated in ADR-019 §Decision 1: PreToolUse, PostToolUse, Stop, SubagentStop, SessionStart, SessionEnd, WorktreeCreate, WorktreeRemove, PostToolUseFailure, and PermissionRequest. There are no per-event carve-outs.
3. The same absence constraint applies to all 5 per-platform variants (`hooks.json.<darwin-arm64|darwin-x64|linux-x64|linux-arm64|windows-x64>`).
4. The activate skill does not introduce `"async": true` when rendering `hooks.json` from the template at runtime.
5. A CI lint check (VP-078 or a dedicated SS-09 check) scans `hooks.json.template` and all per-platform variants for any `"async": true` occurrence and fails the build if found.

## Invariants

1. **Envelope-layer async is permanently retired**: The `"async"` key in hook envelope entries (hooks.json.template) is not a valid configuration knob post-ADR-019. Dispatcher-internal async classification lives exclusively in `hooks-registry.toml` (per BC-7.06.001). These two async control planes are completely separate; the envelope layer has no async field.
2. **Template uniformity**: All event types share the same envelope schema. There is no "some events are async at the envelope layer" exception. ADR-019 §Decision 1 applies uniformly.
3. **CI enforcement is mandatory**: Template changes that introduce `"async": true` in any event entry must be caught before merge. The lint check is a required CI gate, not optional.

## Error Paths

| Condition | Behavior |
|-----------|----------|
| `hooks.json.template` contains `"async": true` in any entry | CI lint fails; edit blocked by Claude Code PostToolUse Edit|Write hook (returning `block_intent = true`, exit code 2); human must remove the `async: true` entry |
| Per-platform variant contains `"async": true` | CI lint fails; same block as above |
| Activate skill writes `"async": true` into rendered `hooks.json` | Bug in activate skill; template compliance check detects on next CI run; validate-template-compliance plugin (which fires on PostToolUse per this BC) will catch at runtime |

## Related BCs

- BC-7.06.001 — depends on: dispatcher-internal async classification via `async` field in `hooks-registry.toml`; the two control planes are complementary and must not be confused
- BC-9.01.005 — sibling: hooks.json gitignore and template variant model; this BC governs the *content* of those template files
- BC-1.14.001 — composes with: the uniform synchronous envelope established by this BC is the precondition for BC-1.14.001's partition model (all events arrive as sync envelopes; dispatcher internally partitions plugins)

## Architecture Anchors

- `plugins/vsdd-factory/hooks/hooks.json.template` — primary artifact governed by this BC
- `plugins/vsdd-factory/hooks/hooks.json.<platform>` — per-platform variants governed equally
- ADR-019 §Decision 1 — the authoritative decision eliminating async from the envelope layer
- CI lint check — VP-078 §Harness 2 or dedicated SS-09 template scan

## Story Anchor

TBD — single story per ADR-019 §6 (no phased rollout, user decision 2026-05-07)

## VP Anchors

- VP-078 — CI lint scanning hooks.json.template for `async: true` entries (Harness 2 or dedicated check per architect's VP-078 scope clarification, F-P1-007)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All event entries in template have no `async` key (happy path, post-ADR-019 state) | CI lint passes; all platform variants pass |
| EC-002 | A future engineer adds `"async": true` to a PostToolUse entry in template | Claude Code PostToolUse Edit|Write hook catches; tool call blocked before edit lands; CI also catches |
| EC-003 | Template has `"async": false` (explicit false, not absent) | Acceptable; CI lint checks for `async: true` only; false is benign |
| EC-004 | New event type added to template with `"async": true` | CI lint catches; same block path as EC-002 |
| EC-005 | Activate skill generates `hooks.json` from template correctly | No `async` key present; Claude Code treats hooks as synchronous |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `hooks.json.template` with all entries lacking `async` key | CI lint passes; no findings | happy-path |
| `hooks.json.template` with `"async": true` in PostToolUse entry | CI lint fails with finding citing the offending entry name and line | lint-violation |
| `hooks.json.template` with `"async": true` in Stop entry | CI lint fails (Stop was the classic async-true event per pre-ADR-019 schema) | lint-violation |
| `hooks.json.template` with `"async": true` in SubagentStop entry | CI lint fails | lint-violation |
| Per-platform variant `hooks.json.darwin-arm64` with `"async": true` | CI lint fails on the platform variant file | lint-violation (variant) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-078 | No entry in `hooks.json.template` or per-platform variants has `"async": true` | integration / CI lint |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-007 ("Activate the dispatcher for any project via a one-command skill") per capabilities.md §CAP-007 |
| Capability Anchor Justification | CAP-007 ("Activate the dispatcher for any project via a one-command skill") per capabilities.md §CAP-007 — the activate skill writes hooks.json from hooks.json.template; this BC ensures the template it reads has no async envelope entries, so the activated hooks.json is always schema-correct for the v2 dispatcher |
| L2 Domain Invariants | DI-015 — Per-project activation is required before the dispatcher can run; the template is the source artifact for activation |
| Architecture Module | SS-09 — `plugins/vsdd-factory/hooks/hooks.json.template`; per-platform variants |
| ADR | ADR-019 §Decision 1 — Async Semantics at Registry Layer, Not Envelope Layer; eliminates `async: true` from all envelope entries |
| Stories | S-15.01 (single story per ADR-019 §6) |
| Cycle | v1.0-feature-plugin-async-semantics-pass-1 (F2) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | Adversary pass-1 F-P1-001: PostToolUse, Stop, SubagentStop still carry `"async": true` in hooks.json.template; ADR-019 §Decision 1 requires all entries to be synchronous at the envelope layer |
| **Confidence** | HIGH — adversary confirmed this is the exact bug ADR-019 was created to fix; three events were missing coverage |
| **Extraction Date** | 2026-05-07 |

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | CI lint: reads template file from filesystem. Template-render (activate skill): reads template, writes hooks.json. Both are filesystem I/O. |
| **Global state access** | None during lint check (pure read + parse). |
| **Deterministic** | YES — given same template content, lint result is always the same. |
| **Thread safety** | YES — lint is a read-only scan. |
| **Overall classification** | Pure scan with filesystem I/O; lint result is deterministic. |

## Amendment 2026-05-09 (v1.2 → v1.3 — F5 fix-burst-35 F-P36-001: Traceability Stories TBD→S-15.01)

**F-P36-001 (BC body vs BC-INDEX Stories drift):** Traceability `Stories` row updated from `TBD — single story per ADR-019 §6 (no phased rollout, user decision 2026-05-07)` to `S-15.01 (single story per ADR-019 §6)`. BC-INDEX row (v1.28) already listed S-15.01; source body was pre-F3. F3 story decomposition (PR #106 merged 2026-05-07) is canonical.

## Amendment 2026-05-07 (v1.1 → v1.2 — WASM-rule audit follow-up)

PO + architect parallel update aligning Layer 1 wording with the actual enforcement mechanism. Architect's WASM-rule audit (`.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/wasm-rule-audit.md`) found that Error Paths row 1 and EC-002 still described a "pre-commit hook" / "commit blocked" mechanism implying a Git `.git/hooks/pre-commit` file. The actual mechanism is the Claude Code PostToolUse Edit|Write hook (identical to the mechanism codified in BC-7.06.001 v1.3).

**Mirror of BC-7.06.001 v1.3 fix:** BC-7.06.001 v1.3 received this correction in its Postcondition 7 and Invariant 5 Layer 1. BC-9.01.006 did not get the parallel update. This amendment closes that gap.

**ADR-019 verified clean:** ADR-019 §Decision 4 uses "pre-commit hook" as a generic/conceptual label meaning "fires before any commit attempt is finalized." That usage is intentionally generic and does not require Git-native pre-commit hooks. No ADR-019 amendment is needed.

**Conceptual intent preserved:** The intent — "violation is caught before it can land in the working tree or reach CI" — is unchanged. The Claude Code PostToolUse Edit|Write hook fires even earlier than git pre-commit, blocking the edit before git ever sees the file.

**Changes:** Error Paths row 1 rewritten from "commit blocked by pre-commit hook" to "edit blocked by Claude Code PostToolUse Edit|Write hook (returning `block_intent = true`, exit code 2)". EC-002 rewritten from "Pre-commit hook catches; commit blocked" to "Claude Code PostToolUse Edit|Write hook catches; tool call blocked before edit lands".

## Amendment 2026-05-07 (v1.0 → v1.1 — F2 pass-7 F-P7-002: stale inputs path corrected)

The `inputs:` frontmatter listed `.factory/specs/architecture/ADR-019.md` — a path that never existed. The actual file is at `.factory/specs/architecture/decisions/ADR-019-plugin-async-semantics-at-registry-layer.md`. Corrected the path in frontmatter. No substantive content changed.
