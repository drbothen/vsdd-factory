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
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.07-p3.md
  - crates/hook-sdk/src/host.rs
  - Cargo.toml
  - crates/hook-plugins/capture-commit-activity/src/main.rs
  - .factory/specs/architecture/ARCH-INDEX.md
input-hash: "e441e99"
traces_to: prd.md
pass: p4
previous_review: adv-s8.07-p3.md
target: story
target_file: .factory/stories/S-8.07-native-port-warn-pending-wave-gate.md
verdict: NITPICK_ONLY
clock: 2_of_3
findings_critical: 0
findings_high: 0
findings_medium: 0
findings_low: 3
findings_nit: 0
---

# Adversarial Review: S-8.07 v1.2 (Pass 4)

## Finding ID Convention

Finding IDs use the format: `F-S807-P4-<SEQ>`

- `F`: Fixed prefix
- `S807`: Story identifier
- `P4`: Pass number
- `<SEQ>`: Three-digit sequence

## Part A — Fix Verification

Pass-3 NITPICK_ONLY findings (3 LOW + 1 NIT) carried open via clock advancement — no v1.3 fix burst was required or applied. All P0 build-breaking patches from pass-2 (vsdd-hook-sdk path `../../hook-sdk`, workspace members T-1.6) verified still present and clean. No regressions on any universal-patch anchor.

## Part B — New Findings (3)

### LOW

#### F-S807-P4-001: T-2 omits `__internal::run` trampoline pattern

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.07 T-2 (fn main implementation task)
- **Description:** T-2 describes the main function as "direct fn main()" without naming the `vsdd_hook_sdk::__internal::run` trampoline. Sibling capture-commit-activity/src/main.rs:43 demonstrates the canonical pattern: `vsdd_hook_sdk::__internal::run(on_hook)`. Without naming the trampoline in T-2, an implementer may write a raw `fn main()` that does not invoke the SDK dispatch loop, producing a WASM binary that compiles but does not handle hook events correctly.
- **Proposed Fix:** Add the trampoline call to T-2: "fn main() { vsdd_hook_sdk::__internal::run(on_hook); }" with a note that this is the required SDK dispatch pattern (see capture-commit-activity/src/main.rs:43 for reference).

#### F-S807-P4-002: Library/binary split not specified (pending intent verification)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.07 T-2 and File Structure
- **Description:** T-2 and the File Structure list only `src/main.rs`. Sibling capture-commit-activity has both `src/lib.rs` and `src/main.rs` (the lib contains the on_hook handler; the bin contains the trampoline). A single `src/main.rs` without a lib may be intentional for simpler hooks, but this is not stated explicitly. If the intent is lib+bin, the File Structure needs updating.
- **Proposed Fix:** Pending orchestrator intent verification. If single-file: add prose note "Single-file main.rs is intentional for this hook's complexity level; on_hook handler is defined inline." If lib+bin: add `src/lib.rs` to File Structure.

#### F-S807-P4-003: Forbidden-dependency rule overstates enforcement level

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.07 Library table or dependency constraints section
- **Description:** A constraint states "build MUST fail if forbidden dependency X is added." No concrete cargo-deny or cargo-vet configuration is provided to enforce this at build time. The constraint is aspirational without a mechanism. The "MUST" language implies automated enforcement that does not exist.
- **Proposed Fix:** Replace "build MUST fail" with "cargo-deny check will flag (see deny.toml deny.crates list); enforcement requires deny.toml update." Or add deny.toml entry as part of the implementation task.

## Sniff Verifications

| Check | Result |
|-------|--------|
| SS-04 "Plugin Ecosystem" canonical | PASS — line 86 verbatim match |
| vsdd-hook-sdk path `../../hook-sdk` | PASS |
| Workspace members T-1.6 present | PASS |
| BC trace verbatim POLICY 7 (BC-7.03.091/092) | PASS — both titles match verbatim |
| serde_yaml 0.9.34 pin | PASS — still present |
| emit_event slice-of-tuples form | PASS |
| Wave 15 [process-gap] disclosure | PASS |

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 3 |
| NIT | 0 |

**Overall Assessment:** nitpick only — advance clock
**Convergence:** convergence continuing (clock 1/3 → 2/3)
**Readiness:** spec stable; 3 LOW deferred per S-7.03 skip-fix discipline

## Verdict

**NITPICK_ONLY** — 0 CRITICAL, 0 HIGH, 0 MED, 3 LOW, 0 NIT. Clock advances 1/3 → 2/3 per ADR-013. One more clean pass closes the clock.

## Trajectory

| Pass | H | M | L | NIT | Total |
|------|---|---|---|-----|-------|
| p1 | 6 | 5 | 2 | 1 | 14 |
| p2 | 4 | 4 | 2 | 1 | 11 |
| p3 | 0 | 0 | 3 | 0 | 3 |
| p4 | 0 | 0 | 3 | 0 | 3 |

Stable LOW-only residue across p3 and p4.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 3 |
| **Closures** | 0 (no fix burst between p3 and p4) |
| **Novelty score** | 1.0 (3/3 novel) |
| **Median severity** | LOW |
| **Trajectory** | 14→11→3→3 |
| **Verdict** | CONVERGENCE_CONTINUING — clock 1/3 → 2/3 |
