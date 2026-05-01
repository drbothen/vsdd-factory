---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.05-p3.md
  - crates/hook-sdk/src/host.rs
  - crates/hook-plugins/capture-commit-activity/Cargo.toml
  - crates/hook-plugins/capture-commit-activity/src/main.rs
  - plugins/vsdd-factory/hooks/validate-pr-review-posted.sh
input-hash: "68f3d16"
traces_to: prd.md
pass: p4
previous_review: adv-s8.05-p3.md
target: story
target_file: .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
verdict: SUBSTANTIVE
clock: 0_of_3
findings_critical: 0
findings_high: 2
findings_medium: 0
findings_low: 2
findings_nit: 0
---

# Adversarial Review: S-8.05 v1.3 (Pass 4)

## Finding ID Convention

Finding IDs use the format: `F-S805-P4-<SEQ>`

- `F`: Fixed prefix
- `S805`: Story identifier
- `P4`: Pass number
- `<SEQ>`: Three-digit sequence

## Part A — Fix Verification (5/5 closed)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S805-P3-001 vsdd-hook-sdk path BUILD-BREAKING | HIGH | VERIFIED | `../../hook-sdk` confirmed at 4 sites in Cargo.toml and task prose |
| F-S805-P3-002 agent dual-fallback chain | HIGH | VERIFIED | `.agent_type // .subagent_name // "unknown"` at T-3 line 352 |
| F-S805-P3-003 SS-02 frontmatter + stretch | MEDIUM | VERIFIED | SS-02 added to subsystems frontmatter and cross-CAP stretch prose |
| F-S805-P3-004 emit_event canonical snippet | MEDIUM | VERIFIED | Slice-of-tuples form confirmed at T-5 lines 369-381 |
| F-S805-P3-005 case (e) prose | LOW | VERIFIED | EC-005 case (e) prose updated |

All 5 pass-3 findings confirmed CLOSED. No regressions on universal-patch anchors.

## Part B — New Findings (4)

### HIGH

#### F-S805-P4-001: AC-008 emit_event error prohibitions semantically void — SDK fire-and-forget

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** S-8.05 AC-008 and EC-005
- **Description:** AC-008 prohibits use of `?`, `unwrap()`, and `expect()` on `host::emit_event` calls, and EC-005 warns against silently swallowing a `Result::Err` from emit_event. However, the actual SDK host.rs:53 shows `emit_event` returns `()` (unit), not `Result<(), HostError>`. The function is fire-and-forget — it cannot fail at the Rust type level. The sibling crate capture-commit-activity/src/main.rs:28-37 correctly calls emit_event as a bare statement with no error handling. AC-008 prohibitions are semantically meaningless for a `()` return type and will confuse implementers. This is a content defect, not a minor wording issue.
- **Evidence:** host.rs:53: `pub fn emit_event(event_type: &str, fields: &[(&str, &str)]) -> ()`. capture-commit-activity/src/main.rs:28-37: bare `emit_event(...)` call without Result handling.
- **Proposed Fix:** Rewrite AC-008 to acknowledge fire-and-forget semantics: "emit_event is fire-and-forget (returns unit). No error handling is required or possible. Do not wrap in Result operators." Drop the `?`/`unwrap()`/`expect()` prohibition language from AC-008 and EC-005.

#### F-S805-P4-002: WASI command entry point unspecified — implementer may produce no .wasm artifact

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** S-8.05 T-2 and File Structure section
- **Description:** T-2 and the File Structure requirements list only `src/lib.rs`. There are two valid sibling patterns for WASM plugin crates: (a) `[lib]` + `[[bin]]` with `src/lib.rs` + `src/main.rs` (pattern used by capture-commit-activity); (b) `[lib]` with `crate-type = ["cdylib", "rlib"]` (pattern used by block-ai-attribution). Without explicit guidance, an implementer who follows only T-2 may produce a `[lib]` crate that compiles to `.rlib` only — cargo would produce no `.wasm` artifact, breaking the hooks-registry binding. The story must specify which pattern applies or explain why a single `src/lib.rs` is sufficient.
- **Evidence:** capture-commit-activity/Cargo.toml uses `[[bin]]` + `src/main.rs` with `vsdd_hook_sdk::__internal::run(on_hook)` trampoline. block-ai-attribution uses `crate-type = ["cdylib", "rlib"]` with no main. Both produce `.wasm`; lib-only without crate-type spec does not.
- **Proposed Fix:** Mirror capture-commit-activity pattern: add `[[bin]]` section to Cargo.toml task and add `src/main.rs` to File Structure with the trampoline call `vsdd_hook_sdk::__internal::run(on_hook)`.

### LOW

#### F-S805-P4-003: v1.2 changelog row description misleading about path fix

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.05 Change Log v1.2 row
- **Description:** v1.2 changelog row claims `../hook-sdk` was the v1.2 fix. v1.3 then corrected this to `../../hook-sdk`. The v1.2 row is actively misleading to anyone reading the changelog to understand the fix history (the v1.2 fix was itself wrong and superseded in v1.3).
- **Proposed Fix:** Optional cosmetic: append "(superseded in v1.3; correct path is ../../hook-sdk)" to the v1.2 changelog path-fix description.

#### F-S805-P4-004: T-8 capability-block removal vs sibling retention (pending intent verification)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.05 T-8
- **Description:** T-8 specifies removal of `[hooks.capabilities]` blocks from the Cargo.toml. Sibling crate block-ai-attribution retains `[hooks.capabilities] env_allow = []` as an explicit empty declaration. The story does not clarify whether the removal is universal or specific to hooks that declare no capabilities at all. Pending orchestrator intent verification.
- **Proposed Fix:** Pending orchestrator adjudication. If removal is universal: document rationale. If sibling pattern is correct: update T-8 to "omit [hooks.capabilities] only if no capabilities are declared; an explicit empty declaration is also acceptable per sibling pattern."

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 2 |
| MEDIUM | 0 |
| LOW | 2 |
| NIT | 0 |

**Overall Assessment:** block — 2 HIGH structural defects
**Convergence:** findings remain; severity profile holds 2 HIGH
**Readiness:** requires revision (v1.4 fix burst required)

## Verdict

**SUBSTANTIVE** — 2 HIGH structural defects (emit_event semantics contradiction; WASM entry-point missing). Both are content defects that will directly cause implementation errors. Clock HELD at 0_of_3.

## Trajectory

| Pass | H | M | L | NIT | Total |
|------|---|---|---|-----|-------|
| p1 | 4 | 5 | 2 | 1 | 12 |
| p2 | 0 | 3 | 1 | 0 | 4 |
| p3 | 2 | 1 | 2 | 0 | 5 |
| p4 | 2 | 0 | 2 | 0 | 4 |

Severity profile holds 2 HIGH across p3 and p4.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 4 |
| **Closures** | 5 |
| **Novelty score** | 1.0 (4/4 novel, not regressions of p3) |
| **Median severity** | MED |
| **Trajectory** | 4→5→4 (severity profile holds 2H) |
| **Verdict** | FINDINGS_REMAIN |
