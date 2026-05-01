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
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.05-p4.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - crates/hook-sdk/src/host.rs
  - crates/hook-plugins/capture-commit-activity/Cargo.toml
  - crates/hook-plugins/capture-commit-activity/src/main.rs
  - crates/hook-plugins/capture-pr-activity/Cargo.toml
  - plugins/vsdd-factory/hooks/validate-pr-review-posted.sh
  - plugins/vsdd-factory/hooks-registry.toml
input-hash: "68f3d16"
traces_to: prd.md
story_id: "S-8.05"
pass_number: 5
story_version: "1.4"
story_input_hash: "68f3d16"
pass: p5
previous_review: adv-s8.05-p4.md
target: story
target_file: .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
verdict: SUBSTANTIVE
clock: 0_of_3
findings_critical: 0
findings_high: 2
findings_medium: 1
findings_low: 1
findings_nit: 0
---

# Adversarial Review Pass-5 — S-8.05 v1.4

## Finding ID Convention

Finding IDs use the format: `F-S805-P5-<SEQ>`
- `F`: Fixed prefix
- `S805`: Story identifier
- `P5`: Pass number
- `<SEQ>`: Three-digit sequence

## Part A — Pass-4 Fix Verification

| ID | Prior Severity | Status | Notes |
|----|----------------|--------|-------|
| F-S805-P4-001 emit_event fire-and-forget reframe | HIGH | **PARTIAL** | AC-008 (lines 264-274) and EC-005 (line 307) reframed correctly to `()` semantics with bare statement form preferred. **However**, T-5 task body (lines 395-407) still uses `let _ = host::emit_event(...)` form with prose "discard result" — directly contradicting the reframed AC-008 ("There is no Result to discard"). New HIGH finding F-S805-P5-001 below. |
| F-S805-P4-002 WASI entry-point pinned | HIGH | **CLOSED** | AC-001 (lines 174-179) specifies `[lib]` + `[[bin]]` + `__internal::run` trampoline. T-2 (lines 350-372) has explicit lib+bin guidance with verbatim trampoline pattern. File Structure (line 483) lists `src/main.rs` row. Mirrors capture-commit-activity/src/main.rs:42-44 exactly. |
| F-S805-P4-003 v1.2 changelog superseded annotation | LOW | **CLOSED** | Line 495 v1.2 row contains "(superseded in v1.3 — correct path is ../../hook-sdk per pass-3 fix)". |
| F-S805-P4-004 T-8 capability-block pending intent | LOW | **CLOSED** | T-8 lines 426-429 marks "(pending intent verification)" with sibling-pattern alternative explicit. |

**Note on directive header claim:** The pass-5 invocation header asserts the pass-4 fix burst included "SS-04 anchor canonicalized." The v1.4 changelog row (line 493) does NOT list any SS-04 fix — it lists only F-S805-P4-001/002/003/004. Lines 83 and 91 still read "SS-04 Hook Plugin Layer" — contradicting ARCH-INDEX:77 (canonical "Plugin Ecosystem"). The header's SS-04 claim is unsupported by v1.4 evidence. New HIGH finding F-S805-P5-002 below.

## Part B — New Findings (Pass-5)

### HIGH

#### F-S805-P5-001: T-5 emit_event call form contradicts AC-008 reframe

- **Severity:** HIGH
- **Confidence:** HIGH
- **Category:** spec-fidelity / internal contradiction (POLICY 12 bc_tv_emitter_consistency)
- **Location:** `.factory/stories/S-8.05-native-port-validate-pr-review-posted.md` lines 395-407 (T-5)
- **Description:** Pass-4 closed F-S805-P4-001 by reframing AC-008 (line 273) and EC-005 (line 307) to state emit_event is fire-and-forget with no Result and that bare statement form is preferred per the capture-commit-activity sibling. AC-008 line 273-274 says "this story SHOULD prefer the bare statement form for parity with siblings. There is no Result to discard." However, T-5 body lines 395-407 still instructs the implementer to use `let _ = host::emit_event(...)` form with prose "discard result". This is the exact construction AC-008 now declares nonsensical. An implementer following T-5 verbatim will write code that contradicts AC-008. host.rs:53 confirms `pub fn emit_event(event_type: &str, fields: &[(&str, &str)])` returns `()`. capture-commit-activity/src/main.rs:28-37 uses bare statement form. T-5 must match.
- **Evidence:**
  - host.rs:53 `pub fn emit_event(event_type: &str, fields: &[(&str, &str)])` (no return)
  - AC-008 line 273-274: "this story SHOULD prefer the bare statement form for parity with siblings. There is no Result to discard"
  - T-5 line 396: "using the canonical Rust call form (discard result)"
  - T-5 line 398: `let _ = host::emit_event(`
  - capture-commit-activity/src/main.rs:28-37: bare `vsdd_hook_sdk::host::emit_event(...);`
- **Proposed fix:** Rewrite T-5 lines 395-407 to use bare statement form and drop the parenthetical "(discard result)".

#### F-S805-P5-002: SS-04 mis-anchor — "Hook Plugin Layer" contradicts ARCH-INDEX canonical "Plugin Ecosystem"

- **Severity:** HIGH
- **Confidence:** HIGH
- **Category:** semantic-anchoring / POLICY 6 violation
- **Location:** `.factory/stories/S-8.05-native-port-validate-pr-review-posted.md` lines 83, 91 (Stretch-Anchor Disclosure)
- **Description:** ARCH-INDEX Subsystem Registry line 77 declares SS-04's canonical name as "Plugin Ecosystem". The story labels SS-04 twice as "Hook Plugin Layer" — line 83 ("**SS-04 Hook Plugin Layer** owns the native WASM plugin crate") and line 91 ("SS-04 Hook Plugin Layer: canonical name confirmed"). The latter is particularly egregious because it explicitly claims canonical-name verification while using the wrong name. Universal-Patch Anchors in the pass-5 directive explicitly state "SS-04 = Plugin Ecosystem (ARCH-INDEX:77)" — the name has been verified externally and is wrong in the story.

  This is a POLICY 6 HIGH violation per the project rubric (architecture_is_subsystem_name_source_of_truth HIGH) and a CRITICAL-tier mis-anchor per the engine's review axis: it would mislead an implementer searching ARCH-INDEX for "Hook Plugin Layer" — they would find no such subsystem. The pass-5 directive header claimed pass-4 closed this; verification of the v1.4 changelog (line 493) shows no SS-04 fix was applied.
- **Evidence:**
  - ARCH-INDEX line 77: `| SS-04 | Plugin Ecosystem | SS-04-plugin-ecosystem.md | crates/hook-plugins/legacy-bash-adapter/, crates/hook-plugins/capture-commit-activity/, ...`
  - S-8.05 line 83: "**SS-04 Hook Plugin Layer** owns the native WASM plugin crate"
  - S-8.05 line 91: "SS-04 Hook Plugin Layer: canonical name confirmed."
  - v1.4 changelog row (line 493): lists only F-S805-P4-001/002/003/004 fixes; no SS-04 entry
- **Proposed fix:** Replace both occurrences ("SS-04 Hook Plugin Layer") with "SS-04 Plugin Ecosystem" to match ARCH-INDEX:77 verbatim. Add a v1.5 changelog row noting the canonical-name correction.

### MEDIUM

#### F-S805-P5-003: AC-005 dual-fallback sub-cases mis-anchored to AC-005 instead of AC-003

- **Severity:** MEDIUM
- **Confidence:** HIGH
- **Category:** spec-fidelity / cross-AC anchor consistency
- **Location:** `.factory/stories/S-8.05-native-port-validate-pr-review-posted.md` lines 210-214 (AC-005) and lines 411-414 (T-5)
- **Description:** AC-005 (Check 2: gh pr comment fallback) is about gh pr comment detection, not agent identity resolution. Yet the agent dual-fallback sub-cases (g.1 / g.2) are placed inside AC-005 (lines 210-214) and again referenced from T-5 ("Bats sub-cases for agent fallback (per AC-005)" line 411). The agent identity dual-fallback chain `agent_type // subagent_name // "unknown"` is governed by AC-003 (agent scoping postcondition for BC-7.04.041), not AC-005. The mis-anchor causes traceability noise: AC-005 trace to BC-7.04.043 says "blocks `gh pr comment` fallback" — that "fallback" is the gh pr comment fallback (using comment instead of review), which is a completely different concept from the agent_type-vs-subagent_name envelope-key fallback. Implementer reading AC-005 will be confused about which "fallback" the sub-cases enumerate.
- **Evidence:**
  - AC-005 line 206: "Check 2: if result matches `gh pr comment`, accumulate error message"
  - AC-005 lines 210-214: "Agent dual-fallback sub-cases for T-5 / AC-005 bats coverage:"
  - AC-003 line 198-199: bats test for non-pr-reviewer agent (where agent extraction occurs)
  - T-5 line 411: "Bats sub-cases for agent fallback (per AC-005):"
- **Proposed fix:** Move the (g.1)/(g.2) agent fallback sub-cases from AC-005 to AC-003 (where agent identity resolution lives). Update T-5 line 411 to "(per AC-003)". The label change costs nothing structurally and corrects the mis-anchor.

### LOW

#### F-S805-P5-004: T-3 prose "RESULT" capitalized but story body uses "result"

- **Severity:** LOW
- **Confidence:** HIGH
- **Category:** consistency / nit-adjacent
- **Location:** `.factory/stories/S-8.05-native-port-validate-pr-review-posted.md` line 384 (T-3)
- **Description:** T-3 line 384 says "Extract RESULT using the dual-fallback chain". RESULT is bash-style ALLCAPS (matching the bash variable at validate-pr-review-posted.sh:22 `RESULT=$(...)`). Elsewhere the story uses lowercase `result` (envelope field name) — e.g., line 248-262 case (e), AC-004 line 203, AC-006 line 218. The bash-style allcaps name leaks into a Rust-port story. Implementer might think `RESULT` is a constant or an envelope-key spelling.
- **Proposed fix:** Lowercase to "result" in T-3 line 384, or qualify as "the `result`/`last_assistant_message` payload".
- **SKIP-FIX eligible** per S-7.03 (LOW polish).

## Open Questions

1. **Q-P5-1 (T-8 capability-block intent):** F-S805-P4-004 closed as "pending intent verification" with sibling-pattern alternative documented (line 428-429). Pass-5 does not adjudicate this — orchestrator/human still owes a decision. Sibling capture-commit-activity registry block at hooks-registry.toml:69-70 uses explicit empty `env_allow = []`, not removal — so the sibling pattern actually FAVORS the alternative.
2. **Q-P5-2 (regex case-sensitivity gh pr comment):** AC-005 line 206 says "if result matches `gh pr comment`" — the bash uses `grep -qE "gh pr comment"` which is case-sensitive. T-4 line 392 says "test result for gh pr comment". The story does not anywhere explicitly say "case-sensitive" for this pattern.

## Pass-6 Priors

- Verify v1.5 fix burst applies F-S805-P5-001 (T-5 → bare statement form) and F-S805-P5-002 (SS-04 → "Plugin Ecosystem" at lines 83 and 91) — both HIGH must close to advance clock.
- Check whether F-S805-P5-003 (AC-005→AC-003 sub-case re-anchor) was applied; if not, demote to LOW per S-7.03.
- Re-verify no regression on F-S805-P4-001 (emit_event fire-and-forget reframe in AC-008/EC-005) and F-S805-P4-002 (WASI entry-point [[bin]] + trampoline) — these must remain stable.
- Re-verify T-5 prose "(discard result)" parenthetical removed.
- Sample the host fn surface against host.rs:53 to confirm signature still `pub fn emit_event(event_type: &str, fields: &[(&str, &str)])` (no Result, no `_ =` needed).

## Verdict

**SUBSTANTIVE** — 2 HIGH defects discovered:
1. F-S805-P5-001 — T-5 internal contradiction with AC-008 reframe (partial-fix regression per S-7.01 axis (a): AC-008 reframed but T-5 prose unchanged).
2. F-S805-P5-002 — SS-04 mis-anchor "Hook Plugin Layer" vs ARCH-INDEX canonical "Plugin Ecosystem"; POLICY 6 HIGH violation; pass-5 directive header claim that pass-4 closed this is contradicted by v1.4 changelog and current line text.

Plus 1 MEDIUM (F-S805-P5-003 AC-005 sub-case mis-anchor) and 1 LOW (F-S805-P5-004 T-3 RESULT casing).

**Clock state: 0_of_3 (held).** Pass-4 fix burst was incomplete (F-S805-P4-001 closed only in AC-008/EC-005, not propagated to T-5; SS-04 claim in directive header not actually applied in story v1.4). v1.5 fix burst required.

## Trajectory

| Pass | H | M | L | NIT | Total |
|------|---|---|---|-----|-------|
| p1   | 4 | 5 | 2 | 1   | 12 |
| p2   | 0 | 3 | 1 | 0   | 4  |
| p3   | 2 | 1 | 2 | 0   | 5  |
| p4   | 2 | 0 | 2 | 0   | 4  |
| p5   | 2 | 1 | 1 | 0   | 4  |

Severity profile holds 2 HIGH for the third consecutive pass (p3, p4, p5). Pattern: each pass closes 1-2 HIGHs but introduces or surfaces new HIGHs because partial fixes don't propagate (S-7.01 axis (a)/(c) violations). One HIGH this pass (F-S805-P5-001) is a direct partial-fix regression of pass-4's F-S805-P4-001.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 5 |
| **New findings** | 4 |
| **Closures (vs pass-4)** | 3/4 (F-S805-P4-002, F-S805-P4-003, F-S805-P4-004 all CLOSED; F-S805-P4-001 PARTIAL — propagation gap to T-5) |
| **Novelty score** | 0.75 (3 of 4 are genuinely new vs pass-4) |
| **Median severity** | MED |
| **Trajectory** | 4→5→4→4 |
| **Verdict** | FINDINGS_REMAIN |

Convergence: NOT REACHED. Pass-5 surfaces a NEW HIGH (SS-04 mis-anchor) that prior passes (p1-p4) did not catch — strong evidence for fresh-context compounding value.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0     |
| HIGH     | 2     |
| MEDIUM   | 1     |
| LOW      | 1     |
| NIT      | 0     |

**Overall Assessment:** BLOCK — 2 HIGH structural defects (T-5 contradicts AC-008 emit_event reframe; SS-04 anchor "Hook Plugin Layer" violates ARCH-INDEX:77 canonical "Plugin Ecosystem").

**Convergence:** Not reached. Severity profile holds 2 HIGH across p3/p4/p5. Pattern of partial-fix propagation gaps (S-7.01 axis (a) violations) is recurring. v1.5 burst should propagate F-S805-P4-001 fully and correct SS-04 anchor; with both closed, p6 likely converges to 1_of_3 if no new defects surface.

**Readiness:** Requires v1.5 fix burst. Two HIGH content defects — both content-fix in place; no process-gap tag warranted.
