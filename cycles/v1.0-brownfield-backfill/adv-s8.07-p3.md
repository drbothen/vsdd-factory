---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.07-native-port-warn-pending-wave-gate.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.07-p2.md
  - Cargo.toml
  - crates/hook-sdk/src/host.rs
  - crates/hook-sdk/src/lib.rs
  - crates/hook-plugins/capture-commit-activity/Cargo.toml
  - crates/hook-plugins/capture-commit-activity/src/main.rs
  - .factory/specs/architecture/ARCH-INDEX.md
input-hash: "e441e99"
traces_to: prd.md
pass: p3
previous_review: adv-s8.07-p2.md
target: story
target_file: .factory/stories/S-8.07-native-port-warn-pending-wave-gate.md
verdict: NITPICK_ONLY
clock: 1_of_3
findings_critical: 0
findings_high: 0
findings_medium: 0
findings_low: 2
findings_nit: 1
---

# Adversarial Review: S-8.07 v1.2 (Pass 3)

## Finding ID Convention

Finding IDs use the format: `F-S807-P3-<SEQ>` per project short-form convention.

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| P0-001 vsdd-hook-sdk path = `../../hook-sdk` | HIGH | RESOLVED | Matches sibling pattern empirically |
| P0-002 Workspace members entry T-1.6 | HIGH | RESOLVED | Entry confirmed |
| P0-003 SS-02 added subsystems frontmatter | MED | RESOLVED | SS-02 present |
| P0-004 emit_event slice-of-tuples | MED | RESOLVED | Slice form applied |
| P0-005 read_file mandatory args | MED | RESOLVED | (path, 65536, 1000) form applied |
| P0-006 S-8.28 → S-8.29 | LOW | RESOLVED | Renumber confirmed |
| P2-001 | HIGH | RESOLVED | Resolved |
| P2-002 | HIGH | RESOLVED | Resolved |
| P2-003 | MED | RESOLVED | Resolved |
| P2-004 | MED | RESOLVED | Resolved |
| P2-005 | LOW | RESOLVED | Resolved |

## Part B — New Findings (or all findings for pass 1)

Pass-3 review of S-8.07 v1.2 (hash e441e99). All P0 build-breaking patches verified landed. All hard sniffs PASS. 11/11 P2 findings closed. 3 findings: 0H / 0M / 2L / 1NIT. Verdict NITPICK_ONLY. Clock 0/3 -> 1/3. Trajectory P1 14 -> P2 11 -> P3 3 (steep decay).

### HIGH

_None._

### MEDIUM

_None._

### LOW

#### F-S807-P3-001: Trampoline pattern under-specified
- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** S-8.07 T-2
- **Description:** `[[bin]]`/`__internal::run` trampoline pattern under-specified. Sibling capture-commit-activity/main.rs:42-44 calls `vsdd_hook_sdk::__internal::run(on_hook)`. Story says "direct fn main() — per capture-commit-activity sibling pattern" but never names the trampoline. SDK lib.rs only documents `#[hook]` macro path; `__internal::run` is doc(hidden).
- **Evidence:** `capture-commit-activity/main.rs:42-44`: `fn main() { vsdd_hook_sdk::__internal::run(on_hook); }`. Story T-2 silent on trampoline name.
- **Proposed Fix:** T-2 add explicit `fn main() { vsdd_hook_sdk::__internal::run(on_hook); }` snippet.

#### F-S807-P3-002: T-3 uses &comma_joined without construction
- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** S-8.07 T-3
- **Description:** T-3 uses `&comma_joined` without pinning where it is constructed. EC-004 documents expected form.
- **Evidence:** T-3 references `&comma_joined` but does not show `let comma_joined = pending_wave_names.join(",");`.
- **Proposed Fix:** Add `let comma_joined = pending_wave_names.join(",");` before emit_event.

### NIT

#### F-S807-P3-003: Architecture Mapping Cargo.toml row undercounts
- **Severity:** NIT
- **Category:** spec-fidelity
- **Location:** S-8.07 Architecture Mapping
- **Description:** Architecture Mapping Cargo.toml row description undercounts (omits members array mutation).
- **Proposed Fix:** Reword to include members array mutation in description.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 2 |
| NIT | 1 |

**Overall Assessment:** pass-with-findings
**Convergence:** CONVERGENCE_REACHED (clock 1/3)
**Readiness:** ready for next phase

## P0 Build-Breaking Patches: ALL VERIFIED

- vsdd-hook-sdk path = `../../hook-sdk` — PASS (matches sibling pattern empirically)
- Workspace members entry T-1.6 — PASS
- SS-02 added subsystems frontmatter — PASS
- emit_event slice-of-tuples — PASS
- read_file (path, 65536, 1000) mandatory args — PASS
- S-8.28 → S-8.29 — PASS

## Hard Sniffs: ALL PASS

- HookResult decision table all branches Continue
- [[bin]] entry point + src/main.rs + direct fn main() (caveat F-S807-P3-001)
- Newline rendering 0x0A; bats $'...\n...'
- bats invocation dispatcher path
- Stdin JSON envelope concretely specified
- Workspace members entry concretely specified

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 3 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (3/3) — all low/nit severity |
| **Median severity** | 1.0 |
| **Trajectory** | 14→11→3 |
| **Verdict** | CONVERGENCE_REACHED |
