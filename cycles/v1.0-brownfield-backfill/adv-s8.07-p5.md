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
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.07-p4.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.07-p3.md
  - Cargo.toml
  - crates/hook-sdk/src/host.rs
  - crates/hook-sdk/src/lib.rs
  - crates/hook-sdk/src/result.rs
  - crates/hook-plugins/capture-commit-activity/Cargo.toml
  - crates/hook-plugins/capture-commit-activity/src/main.rs
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.091.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.092.md
input-hash: "e441e99"
traces_to: prd.md
pass: p5
previous_review: adv-s8.07-p4.md
target: story
target_file: .factory/stories/S-8.07-native-port-warn-pending-wave-gate.md
story_id: "S-8.07"
pass_number: 5
story_version: "1.2"
story_input_hash: "e441e99"
verdict: NITPICK_ONLY
clock: 3_of_3
findings_critical: 0
findings_high: 0
findings_medium: 0
findings_low: 3
findings_nit: 0
---

# Adversarial Review Pass-5 — S-8.07 v1.2

## Finding ID Convention

Finding IDs use the format: `F-S807-P5-<SEQ>`
- `F`: Fixed prefix
- `S807`: Story identifier
- `P5`: Pass number
- `<SEQ>`: Three-digit sequence

## Part A — Pass-4 Fix Verification

No v1.3 fix burst was applied between p4 and p5 (per S-7.03 SKIP-FIX LOW/NIT discipline; pass-4 produced 3 LOW + 0 NIT findings, all eligible for deferral). Each pass-4 finding re-verified against current S-8.07 v1.2 (input-hash e441e99):

| Pass-4 Finding | Severity | Status | Notes |
|----------------|----------|--------|-------|
| F-S807-P4-001 — T-2 omits `__internal::run` trampoline pattern | LOW | OPEN (carried) | Story T-2 (line 342) still says "direct fn main() — per capture-commit-activity sibling pattern" without naming `vsdd_hook_sdk::__internal::run(on_hook)`. Sibling main.rs:43 confirms canonical pattern. Per S-7.03 SKIP-FIX, deferred. |
| F-S807-P4-002 — Library/binary split not specified (pending intent verification) | LOW | OPEN (carried) | File Structure (story line 450) lists only `src/main.rs`; T-2 (line 342) names only `[[bin]]`. Sibling capture-commit-activity ships both `[lib]` + `[[bin]]` (Cargo.toml:15-20). Pass-4 tagged this `(pending intent verification)`; orchestrator has not adjudicated. Carried per S-7.03 SKIP-FIX. |
| F-S807-P4-003 — Forbidden-dependency rule overstates enforcement level | LOW | OPEN (carried) | Architecture Compliance Rules (story line 425) still says "If this module gains a dependency on legacy-bash-adapter, the build MUST fail." No deny.toml entry or cargo-deny invocation specified. Per S-7.03 SKIP-FIX, deferred. |

**Pass-3 carryover audit (S-7.01 partial-fix discipline):**

| Pass-3 Finding | Severity | Status |
|----------------|----------|--------|
| F-S807-P3-001 — Trampoline pattern under-specified | LOW | OPEN — same gap as F-S807-P4-001 (duplicate, single underlying issue) |
| F-S807-P3-002 — T-3 uses `&comma_joined` without construction | LOW | OPEN — story T-3 (line 370) still references `&comma_joined` without showing construction |
| F-S807-P3-003 — Cargo.toml row undercounts | NIT | RESOLVED — story line 276 row now reads "Pure (declarative; adds serde_yaml 0.9.34 to [workspace.dependencies])" |

**Universal-patch anchor re-verification (S-7.01 propagation discipline):**

| Anchor | Story Location | Status |
|--------|----------------|--------|
| WASI target `wasm32-wasip1` | T-2 line 341, AC-001 line 165, T-4 line 374, Library table line 434, Compliance row line 426 | PASS |
| vsdd-hook-sdk path `../../hook-sdk` | T-2 line 341, Library table line 435 | PASS — verbatim match against sibling capture-commit-activity/Cargo.toml:23 |
| Workspace `[workspace] members` registration | T-1.6 line 327, File Structure line 448 | PASS |
| `host::emit_event` slice-of-tuples form | AC-003 lines 181-191, T-3 lines 367-370 | PASS — matches host.rs:53 signature |
| `host::read_file` (path, 65536, 1000) | T-3 line 346 | PASS — matches host.rs:187 signature |
| HOST_ABI_VERSION = 1 | Compliance row line 418 | PASS — matches lib.rs:58 |
| WASI entry-point `[lib]` + `[[bin]]` + `__internal::run` | T-2 line 342, File Structure line 450 | PARTIAL — `[[bin]]` named; `[lib]` and `__internal::run` not named |
| SS-04 = "Plugin Ecosystem" | story line 88 | PASS — verbatim match against ARCH-INDEX:77 |
| SS-02 = "Hook SDK and Plugin ABI" | story lines 78, 87 | PASS — verbatim match against ARCH-INDEX:75 |
| serde_yaml pinned 0.9.34 | T-1.5 line 332, Library table line 436, Goal line 100 | PASS |
| BC-7.03.091 title verbatim | story BC table line 112 | PASS — matches BC file H1 line 27 |
| BC-7.03.092 title verbatim | story BC table line 113 | PASS — matches BC file H1 line 27 |

All build-breaking issues from pass-2 (SDK path, workspace members) remain correctly resolved; no regression.

## Part B — New Findings (Pass-5)

### CRITICAL

_None._

### HIGH

_None._

### MEDIUM

_None._

### LOW

#### F-S807-P5-001: Trampoline pattern under-specified (re-flag of F-S807-P4-001 / F-S807-P3-001)

- **Severity:** LOW
- **Confidence:** HIGH
- **Category:** spec-fidelity
- **Location:** `.factory/stories/S-8.07-native-port-warn-pending-wave-gate.md:342` (T-2 task)
- **Evidence:** Story T-2 prose: "src/main.rs with hook entry point (`[[bin]]` target, direct `fn main()`, no `#[hook]` macro — per capture-commit-activity sibling pattern)." Sibling reference at `crates/hook-plugins/capture-commit-activity/src/main.rs:42-44`: `fn main() { vsdd_hook_sdk::__internal::run(on_hook); }`. The trampoline call is `#[doc(hidden)]` (lib.rs:42-43) so an implementer reading public SDK docs alone would not discover it.
- **Description:** T-2 still does not explicitly name the `vsdd_hook_sdk::__internal::run(on_hook)` trampoline call. An implementer who lands on the SDK's documented `#[hook]` macro path (lib.rs:10-21) would miss the doc-hidden `__internal::run` route used by the sibling. Two prior passes flagged the same gap.
- **Proposed Fix:** Add to T-2: `fn main() { vsdd_hook_sdk::__internal::run(on_hook); }` literal, with cite to capture-commit-activity/src/main.rs:43.
- **Disposition:** SKIP-FIX per S-7.03 (LOW). Carried.

#### F-S807-P5-002: Library/binary split intent still unverified (re-flag of F-S807-P4-002)

- **Severity:** LOW (pending intent verification)
- **Confidence:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `.factory/stories/S-8.07-native-port-warn-pending-wave-gate.md:342, 450`
- **Description:** Story does not declare whether warn-pending-wave-gate intentionally diverges from the sibling lib+bin pattern. Pass-4 raised; orchestrator has not adjudicated.
- **Disposition:** SKIP-FIX per S-7.03 (LOW pending-intent). Carried.

#### F-S807-P5-003: `comma_joined` construction still elided in T-3 (re-flag of F-S807-P3-002)

- **Severity:** LOW
- **Confidence:** HIGH
- **Category:** ambiguous-language
- **Location:** `.factory/stories/S-8.07-native-port-warn-pending-wave-gate.md:368-370` (T-3 emit_event call)
- **Evidence:** T-3 prose at line 370: `("pending_waves", &comma_joined)` — references the variable `&comma_joined` but never shows its construction.
- **Disposition:** SKIP-FIX per S-7.03 (LOW). Re-flagging for visibility only.

### NIT

_None._

## Open Questions

1. **F-S807-P5-002 intent adjudication** — orchestrator has not yet stated whether warn-pending-wave-gate should follow the sibling lib+bin pattern or stay single-file. This question has been open since p4. Recommend codifying either way before S-8.09 unblocks.

## Pass-6 Priors

If a v1.3 fix burst lands before p6, the three LOW findings above (P5-001, P5-002, P5-003) are the targets. If no fix burst is applied (clock-only advancement scenario), p5 produces clock 3/3 = CONVERGENCE_REACHED and p6 should not run. Pass-6 only triggers if a SUBSTANTIVE finding is introduced or if a fix burst regresses universal-patch anchors.

## Verdict

**NITPICK_ONLY** — 0 CRITICAL, 0 HIGH, 0 MEDIUM, 3 LOW, 0 NIT. All three LOW findings are re-flags of prior-pass deferrable issues (no SUBSTANTIVE regression). Per ADR-013, clock advances **2/3 → 3/3 = CONVERGENCE_REACHED**.

## Trajectory

| Pass | H | M | L | NIT | Total | Verdict |
|------|---|---|---|-----|-------|---------|
| p1 | 6 | 5 | 2 | 1 | 14 | SUBSTANTIVE |
| p2 | 4 | 4 | 2 | 1 | 11 | SUBSTANTIVE |
| p3 | 0 | 0 | 2 | 1 | 3 | NITPICK_ONLY (clock 0→1) |
| p4 | 0 | 0 | 3 | 0 | 3 | NITPICK_ONLY (clock 1→2) |
| p5 | 0 | 0 | 3 | 0 | 3 | NITPICK_ONLY (clock 2→3) |

Decay profile: 14 → 11 → 3 → 3 → 3. Stable LOW-only residue across three consecutive passes; all three pass-5 LOW findings are re-flags (zero novel findings). Indicates spec has converged on substantive content.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 5 |
| **New findings** | 3 |
| **Truly novel findings** | 0 (all three are re-flags of P3/P4 LOW residue) |
| **Closures** | 1 (F-S807-P3-003 NIT — Cargo.toml row description corrected) |
| **Novelty score** | 0.0 (0/3 novel) |
| **Median severity** | LOW |
| **Trajectory** | 14 → 11 → 3 → 3 → 3 |
| **Verdict** | CONVERGENCE_REACHED — clock 2/3 → 3/3 per ADR-013 |

**Novelty: LOW — findings are refinements/re-flags, not new gaps. Spec has converged.**

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 3 |
| NIT | 0 |

**Overall Assessment:** Nitpick-only — zero substantive issues. All three LOW findings are re-flags from prior passes (P3/P4 carryover); no novel gaps surfaced. Universal-patch anchors all PASS; BC titles verbatim; SDK path correct; workspace members task explicit; serde_yaml pin + TD entry intact; emit_event/read_file signatures match host.rs; SS-02 / SS-04 canonical names verified against ARCH-INDEX.

**Convergence:** **CONVERGENCE_REACHED** — clock 2/3 → 3/3 per ADR-013. Three consecutive NITPICK_ONLY passes (p3 clock 0→1, p4 clock 1→2, p5 clock 2→3) satisfy the ADR-013 requirement.

**Readiness:** Spec is **READY** for downstream consumption. Three open LOW findings are SKIP-FIX deferrals per S-7.03; orchestrator may close them in a single drive-by edit during implementation or carry them as known implementer-discoverable nits. None block S-8.07 advancing to status=ready, and none block the W-15 unblock chain (S-8.07 → S-8.09).
