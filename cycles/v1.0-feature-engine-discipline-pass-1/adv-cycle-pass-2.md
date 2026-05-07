---
document_type: adversarial-review
cycle: v1.0-feature-engine-discipline-pass-1
pass: 2
phase: F5
date: 2026-05-07
producer: adversary
prior-pass-classification: CRITICAL
prior-findings-count: 29
---

# Adversarial Review — Pass 2

## Pass-1 Fix Verification Summary

I spot-checked all 29 pass-1 findings. Most fixes are correct; several have residual gaps. Below the table I detail all findings by severity.

| Pass-1 ID | Status | Notes |
|---|---|---|
| F-CRIT-1 (step-d5 missing) | ✅ FIXED | File present at `skills/deliver-story/steps/step-d5-adversary-convergence.md`; Lobster line 79 reference resolves. |
| F-CRIT-2 (ADR-017 slug) | ✅ FIXED | All 4 anchor strings now use `-phasing.md`; verified via grep. |
| F-CRIT-3 / F-HIGH-5 / F-MED-7 (VP-071 v1.2) | ✅ FIXED | VP-071 v1.2 amended; kani harness uses `HookResult::Block { .. }`; frontmatter `bcs:` lists `BC-4.10.001, BC-5.39.001`. |
| F-CRIT-4 (emit_event missing) | ✅ FIXED | `HookCallbacks::emit_event` declared; `RealCallbacks` wires `host::emit_event`; 4 `hook.block` call sites in lib.rs. |
| F-HIGH-1 (registry comment) | ✅ FIXED | hooks-registry.toml:931-936 says canonical-form. |
| F-HIGH-2 (bats split) | ✅ FIXED | Tests at lines 162 and 182 split. |
| F-HIGH-3 (list_stories inert) | ⚠ PARTIAL — see F-P2-001 | Code-level fix in main.rs added but production wiring incomplete. |
| F-HIGH-4 (BC-4.10.002 PC3) | ✅ FIXED | PC3 amended to `host::log_info`. |
| F-HIGH-6 (placeholder semantics) | ✅ FIXED | BC-4.11.001 v1.1 invariant 6 single-segment; tests added. |
| F-HIGH-7 (relocate git mv) | ✅ FIXED | bats test at relocate-artifact.bats:283 verifies `git log --follow`. |
| F-HIGH-8 (Gate 3 narrowing) | ✅ FIXED | wave-gate/SKILL.md:88-90 carveout in place. |
| F-HIGH-9 (S-13.01 fn names) | ⚠ PARTIAL — see F-P2-002 | `parse_registry`→`load_registry` not visible in S-13.01 grep; type names still `PathDecision`/`ArtifactRegistry`. |
| F-HIGH-10 (VP-070 fn name) | ✅ FIXED | `matches_canonical` used; type names `MatchResult`/`PathRegistry`. |
| F-HIGH-11 (catch_unwind) | ✅ FIXED | Zero `catch_unwind` matches in lib.rs. |
| F-HIGH-12 (S-12.02 advisory) | ✅ FIXED | AC-002, ACR #1, Dev Notes all reference canonical block_with_fix. |
| F-HIGH-13/14 (registry paths) | ✅ FIXED | Both registry entries at lines 105-108 and 131-134 are per-story patterns. |
| F-MED-1 (relocate scope) | ✅ FIXED | `## Scope` section at SKILL.md:266. |
| F-MED-3 (vp-072 enumeration) | ✅ FIXED | bats test at line 277 enumerates programmatically. |
| F-MED-4 (EC-006 emit) | ✅ FIXED | `emit_event("hook.warn"...)` at validate-artifact-path/lib.rs:384. |
| F-MED-5 (perf benchmark) | ✅ FIXED | tests.rs:1376 EC-007 1000-call <200ms test. |
| F-MED-6 (story spec lookup) | ✅ FIXED | adversary.md:42 has explicit Glob guidance. |
| F-MED-8 (wave-gate identity) | ✅ FIXED | `starts_with("wave-gate")` at lib.rs:225; test at lib.rs:1874. |
| F-LOW-1 (F1 advisory wording) | ❌ NOT FIXED — see F-P2-003 | F1-delta-analysis.md:391 still has "advisory block stdout + hook.block event + stderr". |
| F-LOW-2/3 [process-gap] | ✅ TRACKED | E-14 stories S-14.01, S-14.02. |
| F-LOW-4 [process-gap] | ✅ ADDRESSED | D-355-AMEND added; PG-3 story tracked. |
| F-LOW-5 (input-hash) | ✅ FIXED | All 6 BCs have `input-hash: "40a6fb6"`. |

---

## Critical Findings

### FINDING [CRITICAL] — F-P2-001 Convergence hook is STILL operationally inert in production despite F-HIGH-3 "fix"

WHY: Pass-1's F-HIGH-3 fix in `main.rs` introduced `extract_stories_from_config(&payload.plugin_config)` which reads `plugin_config.stories`. However, no mechanism populates this field at dispatch time. The dispatcher's `executor.rs:127` splices the **static** `entry_clone.config_as_json()` (from hooks-registry.toml) into `plugin_config`. The hooks-registry.toml entry for `validate-per-story-adversary-convergence` (lines 920-940) has **no `[hooks.config]` table at all** — only `[hooks.capabilities.read_file]`. With no `stories` key in the static config, `extract_stories_from_config` returns `Err`, `RealCallbacks::list_stories` returns `Err`, `hook_logic` takes the graceful-degrade branch, and the hook returns `Continue` on every wave-gate dispatch — exactly the pass-1 defect.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/plugins/vsdd-factory/hooks-registry.toml:920-940` (no `[hooks.config]` section for this hook)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/crates/factory-dispatcher/src/executor.rs:120-127` (config spliced verbatim from registry; no dynamic enrichment)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:269-272` (`extract_stories_from_config` returns Err on absent field)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:388-397` (Err → graceful-degrade Continue)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/plugins/vsdd-factory/skills/wave-gate/SKILL.md` — does NOT mention populating `plugin_config.stories`; no instruction to write the wave's story list anywhere the dispatcher will see it
- The hooks-registry.toml is statically loaded at dispatcher startup; it is not edited per-wave

IMPACT: Same as pass-1: every production wave-gate dispatch silently returns `Continue` with the misleading "graceful degrade — invoked outside wave-gate context or cycle directory absent" log, even when the agent identity IS `wave-gate-dispatch`. The whole convergence-gate mechanism is bricked. The pass-1 F-HIGH-3 "fix" added the consumer plumbing but no producer. Without either (a) the dispatcher dynamically injecting `stories` into plugin_config from `wave-state.yaml`, OR (b) the hook reading `wave-state.yaml` directly via `host::read_file`, OR (c) the orchestrator/wave-gate skill writing a manifest the hook reads, the hook is non-functional.

FIX: CODE — Pick one and execute it:
1. Have `RealCallbacks::list_stories` fall back to `host::read_file(".factory/wave-state.yaml")` when `plugin_config.stories` is absent and parse the active wave's stories. This requires no orchestrator change and no host ABI change.
2. Add an orchestrator/wave-gate skill instruction step that writes the wave's stories to a known path (e.g., `.factory/cycles/<cycle-id>/wave-stories.json`) before the SubagentStop fires, and have `list_stories` read it.
3. Extend the dispatcher to dynamically inject stories from `wave-state.yaml` into `plugin_config` before splicing.

Document the chosen mechanism in BC-4.10.001 invariant 1 (currently silent on stories source) and add a unit test that exercises the production path. POLICY: implicit — spec/code consistency, no_silent_failures.

CONFIDENCE: HIGH

---

### FINDING [CRITICAL] — F-P2-002 BC-4.10.001 line 126 and BC-5.39.001 line 121 still describe VP-071 with deprecated "advisory-block output always emitted" wording — sibling-file regression of F-CRIT-3/F-HIGH-5

WHY: Pass-1 caught the deprecated "advisory-block" pattern in VP-071 spec, S-12.02 spec, and F1-delta-analysis. The B1 fix amended VP-071 to v1.2, but the **VP traceability tables** in BC-4.10.001 and BC-5.39.001 still reference VP-071 with the original, deprecated description. Per Partial-Fix Regression Discipline, "Sibling files in the same architectural layer" — same-cycle BCs that cite the same VP — must be checked for the same pattern. They were not.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.10.001.md:126` — `| VP-071 | advisory-block output always emitted on non-cleared gate | kani (pure logic branch coverage) |`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.001.md:121` — `| VP-071 | advisory-block output always emitted when per-story gate has not been cleared | kani / adversarial review |`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-071.md:37` — actual H1 is "Block Invariant" with canonical block_with_fix form per v1.2 amendment

IMPACT: Two behavioral contracts in the cycle delta now contradict the very VP they cite. A reader of BC-4.10.001 (the primary authoritative contract for this hook) sees VP-071 described as "advisory-block" — the deprecated pattern that VP-071 v1.2 explicitly retires (lines 274-283 of VP-071.md). Per POLICY 4 (semantic_anchoring_integrity) and POLICY 9 (vp_index_is_vp_catalog_source_of_truth), VP descriptions in BC traceability tables must match the VP's authoritative description. This is the **same defect class** as pass-1's F-LOW-1 (F1-delta-analysis stale wording), now found in two BC files that pass-1 did not check. Severity escalated to CRITICAL because:
1. BC-4.10.001 is the authoritative behavioral contract (CAP-009 anchor), not a discardable upstream proposal.
2. The drift is bidirectional with VP-071 — VP-071 says "Block Invariant", BC says "advisory-block".
3. Blast radius: 2 BC files (per Partial-Fix Regression Discipline blast radius ≥ 2 → HIGH; combined with primary-spec character → CRITICAL).

FIX: SPEC — Update BC-4.10.001 line 126 to: `| VP-071 | Block Invariant — kani harness verifies HookResult::Block on non-converged input (canonical block_with_fix form per VP-071 v1.2) | kani |`. Update BC-5.39.001 line 121 similarly. Bump both BCs to v1.2. POLICY: 4, 7 (bc_h1_is_title_source_of_truth — affects VP description in BC body), 9.

CONFIDENCE: HIGH

---

## Important Findings

### FINDING [HIGH] — F-P2-003 F-LOW-1 not fully closed: F1-delta-analysis.md:391 still has stale "advisory block stdout + hook.block event + stderr" wording

WHY: Pass-1 identified F1-delta-analysis line 342 as containing deprecated advisory-block wording. The fix-plan's B5 was supposed to update it. The fix at line 342 was applied (now reads "Block Invariant — kani..."), but a SECOND occurrence at line 391 in the same file was not fixed. Pass-1 only cited line 342; the partial fix discipline says: when a fix changes a value, grep for ALL occurrences. The second occurrence describes the test plan: "Uncleaned gate path → advisory block stdout + hook.block event + stderr" — this is the deprecated pattern.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md:391`
- Same file line 342 was correctly amended (verified via prior reading)

IMPACT: Internal contradiction within the same scoping document. Line 342 correctly describes VP-071 as Block Invariant, line 391 describes the test plan with the deprecated advisory pattern. A reader follows the test plan → builds tests for the wrong contract. Low practical risk because the actual implementation tests are correct, but doc drift compounds.

FIX: DOC — Update F1-delta-analysis.md line 391 to: `Uncleaned gate path → HookResult::Block via block_with_fix (exit code 2) + hook.block event` per VP-071 v1.2. Add cross-reference to D-349. POLICY: 9.

CONFIDENCE: HIGH

---

### FINDING [HIGH] — F-P2-004 S-13.01 still has `PathDecision` and `ArtifactRegistry` type names — partial fix of F-HIGH-9/F-HIGH-10

WHY: D-358 declared "VP-070 v1.0→v1.1 corrects ... type names (PathDecision→MatchResult, ArtifactRegistry→PathRegistry)." The amendment was applied to VP-070.md, but the **same type names in S-13.01 prose** were NOT updated. S-13.01 line 82 still says `PathDecision`; lines 295-296 still say `load_registry(...) -> Result<ArtifactRegistry, ParseError>` and `matches_canonical(path: &str, registry: &ArtifactRegistry) -> PathDecision`. The actual production types in `validate-artifact-path/src/lib.rs:59,92` are `PathRegistry` and `MatchResult`.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-13.01-path-governance-bundle.md:82` — `pair the function always returns the same PathDecision`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-13.01-path-governance-bundle.md:295-296` — function signatures with wrong types
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/crates/hook-plugins/validate-artifact-path/src/lib.rs:59` — `pub struct PathRegistry`
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/crates/hook-plugins/validate-artifact-path/src/lib.rs:92` — `pub enum MatchResult`

IMPACT: Same finding class as pass-1's F-HIGH-9/F-HIGH-10 (story spec / VP spec drift on actual function and type names). The implementer who used S-13.01 as the source-of-truth would author the wrong type names. Per Partial-Fix Regression Discipline, "Bodies of files where frontmatter was changed" must be checked — and the fix-plan explicitly listed S-13.01 in B2 as needing terminology alignment. The B2 fix updated `parse_registry`→`load_registry` but missed the type names. POLICY: 4.

FIX: SPEC — Update S-13.01:82 from `PathDecision` to `MatchResult`; update S-13.01:295-296 to use `PathRegistry` and `MatchResult` (preserving the `load_registry(input: Option<&str>) -> Result<PathRegistry, RegistryError>` and `matches_canonical(path: &str, registry: &PathRegistry) -> MatchResult` actual signatures from lib.rs:119,150).

CONFIDENCE: HIGH

---

### FINDING [HIGH] — F-P2-005 BC-5.39.001 PC2 schema does not declare `bootstrap_annotation` field, but PG-2 backfill state files write it

WHY: The PG-2 inline backfill (D-359) wrote 3 convergence-state files for the bootstrap cohort. Each file contains a `bootstrap_annotation` object with sub-fields (`exception_type`, `rationale`, `f5_pass_1_review`, etc.). However, BC-5.39.001 PC2 (lines 56-71) declares the schema with exactly 5 top-level fields: `passes_clean`, `last_finding_count`, `last_classification`, `last_timestamp`, `deferred_findings`. There is no `bootstrap_annotation` field in the BC schema.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.01/adversary-convergence-state.json:7-15`
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.02/adversary-convergence-state.json:7-15`
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-13.01/adversary-convergence-state.json:7-15`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.001.md:56-71` (schema declaration without bootstrap_annotation)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:72-80` (`ConvergenceState` struct does not declare `bootstrap_annotation`; serde tolerates unknown fields by default)

IMPACT: Schema-data drift. Today, the WASM hook tolerates unknown fields silently (serde default behavior). But: (a) any future schema validator will reject these files as non-conformant; (b) a future maintainer who adds `#[serde(deny_unknown_fields)]` to `ConvergenceState` will break parsing; (c) the bootstrap_annotation is an extension point that should be governed by the BC, not invented ad-hoc. POLICY: 4 (semantic_anchoring_integrity — schema doc must match data).

FIX: SPEC — Amend BC-5.39.001 v1.0→v1.1 to declare an optional `bootstrap_annotation: Object` field with the sub-fields used by the inline backfill (`exception_type`, `rationale`, `f5_pass_1_review`, `fix_batches_completed`, `fix_batches_pending`, `authorized_by`, `authorized_date`). Mark it optional (only present for bootstrap-exception cohort). Update PC2 schema example. Cross-reference D-354 and D-359. Update `ConvergenceState` struct to add this optional field for forward consistency (or document explicitly in BC that the struct ignores unknown fields).

CONFIDENCE: HIGH

---

### FINDING [HIGH] — F-P2-006 BC-4.10.001 lacks observability mandate codifying the F-CRIT-4 fix (`emit_event("hook.block")`) — SPEC remained unchanged while CODE added the requirement

WHY: Pass-1's F-CRIT-4 fix-plan said "Two sub-tasks: (a) amend BC-4.10.001 to clarify event-emission mandate (or confirm it and add the code), (b) add `emit_event` to `HookCallbacks` trait." The fix burst added `emit_event` to the trait + 4 call sites in lib.rs, but BC-4.10.001 itself was NOT amended to mandate `hook.block` event emission. The current BC postconditions describe what the block message contains (PC2/3/4) but never say "the hook MUST emit a `hook.block` event before returning Block." The code now does this; the BC is silent.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.10.001.md:54-99` (postconditions and invariants — no mention of hook.block event emission)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:446-454, 463-472, 502-511, 577-586` (4 call sites emitting hook.block)
- BC-4.10.001 invariant 2 (line 91-92) only mentions emit_event as an ABI surface, not as a behavioral requirement

IMPACT: Spec-implementation gap of the inverse polarity from pass-1: code is "ahead" of spec. A future implementer reading BC-4.10.001 alone would not know they must emit `hook.block`; they would treat it as an optional debugging affordance. A future code rewrite could remove the call sites without violating the BC. The pass-1 finding F-CRIT-4 noted: "Add an emit_event method to the HookCallbacks trait... Alternatively, amend BC-4.10.001 to drop the hook.block event requirement." The fix-plan chose option 1 but did not codify the mandate in spec. POLICY: implicit — spec/code consistency.

FIX: SPEC — Add a new postcondition to BC-4.10.001: `PC9. Before returning HookResult::Block, the hook MUST emit a "hook.block" event via host::emit_event with fields: hook (HOOK_NAME), code (telemetry code), story (story_id), reason (block reason string).` Bump BC-4.10.001 to v1.1. Update VP-071 cross-references if needed. Reference: BC-7.03.075 hook.block event pattern.

CONFIDENCE: HIGH

---

### FINDING [HIGH] — F-P2-007 BC-4.10.002 PC3 amendment introduced a contradiction with BC-4.10.001 invariant 2

WHY: BC-4.10.002 v1.1 PC3 (just amended in B2) now says: "Logs a single advisory message via `host::log_info(...)`... Note: HOST_ABI v1 does not expose a `log_debug` endpoint; `log_info` is the lowest-severity level available. The `log_debug` symbol referenced in early drafts is absent from the SDK." But BC-4.10.001 invariant 2 (line 91-92) still says: "HOST_ABI_VERSION = 1 — no new host functions are required. The hook uses only `host::read_file`, `host::log_*`, and `host::emit_event` (already present in ABI v1)." The wildcard `log_*` would lead a reader to assume `log_debug` exists. BC-4.10.002 v1.1 explicitly says it does not. Within the same hook, two sibling BCs disagree on the SDK surface area.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.10.001.md:91-92`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.10.002.md:56-60`

IMPACT: Reader confusion about SDK surface. The trait at lib.rs:311-341 declares `log_debug` (which actually maps to `host::log_info`); a reader trying to align this with BC-4.10.001's "log_*" (which implies log_debug exists) would be misled. POLICY: 4.

FIX: SPEC — Amend BC-4.10.001 invariant 2 to clarify: "The hook uses only `host::read_file`, `host::log_info`, `host::log_warn`, `host::log_error`, and `host::emit_event` (HOST_ABI v1; no `host::log_debug` exists — see BC-4.10.002 PC3)." Bump BC-4.10.001 to v1.1.

CONFIDENCE: MEDIUM

---

### FINDING [HIGH] — F-P2-008 No production-path integration test verifies the convergence hook actually fires `hook.block` events when invoked through the real WASM dispatch

WHY: The unit tests in lib.rs use `FakeCallbacks` which counts emit_event calls (`block_events_emitted`), and tests at lines 1695-1717 verify that "F-CRIT-4: emit_event(\"hook.block\") called before each block return." But: (a) the FakeCallbacks `list_stories` method always succeeds for tests (returning the injected `stories` vec) — production never reaches this path because of F-P2-001 (no stories source). (b) No bats integration test invokes the WASM binary against a real hook payload to verify hook.block events are emitted. The pass-1 fix burst added unit tests but the production code path is dead per F-P2-001, so the unit tests prove correctness of dead code.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:1695-1717` (unit test using FakeCallbacks)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/plugins/vsdd-factory/tests/per-story-adversary-convergence-hook.bats` (bats tests; verified earlier — they check structural source patterns, not behavioral integration)

IMPACT: Tests pass for the wrong reason — the tested code paths cannot be reached in production until F-P2-001 is fixed. Per POLICY 11 (no_test_tautologies), the test calls a production fn but the test asserts on data the test itself constructed (the `stories` vec injected into FakeCallbacks). Without an end-to-end integration test, there is no signal that hook.block events actually fire when the dispatcher runs the real WASM.

FIX: TEST — After F-P2-001 is resolved, add a bats integration test that: (a) writes a fixture cycle directory with one story whose state file has `passes_clean: 0`; (b) invokes the WASM binary via the dispatcher with `agent_type: "wave-gate-dispatch"` and the cycle context; (c) asserts the hook returns exit code 2 AND emits a `hook.block` event observable via `.factory/logs/`. POLICY: 11.

CONFIDENCE: MEDIUM

---

## Medium Findings

### FINDING [MEDIUM] — F-P2-009 BC-5.39.001 PC2 schema example is inconsistent with the actual hook's `ConvergenceState` struct treatment of optional fields

WHY: BC-5.39.001 PC2 (line 56-71) shows the schema with 5 required-looking fields. The actual `ConvergenceState` struct at lib.rs:73-80 has `last_finding_count: Option<u32>`, `last_classification: Option<String>`, `last_timestamp: Option<String>` (3 fields are Option) and `deferred_findings: Vec<...>` with `#[serde(default)]`. The BC example shows `last_classification` as an enum-like string and `last_finding_count` as an int — not as null-safe. A test fixture with `last_finding_count: null` would parse correctly per the code (Some(None)) but contradicts the BC example.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.001.md:56-71`
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:72-80`

IMPACT: Reader confusion about which fields are optional vs required. This is a low-severity drift; behavior is correct (the hook handles missing/null fields safely per VP-071 Part D).

FIX: SPEC — Annotate the BC-5.39.001 PC2 schema example with explicit required/optional markers, e.g., add `(optional, may be null)` to last_finding_count, last_classification, last_timestamp. Bump BC-5.39.001 to v1.1 (will combine with F-P2-005 fix).

CONFIDENCE: MEDIUM

---

### FINDING [MEDIUM] — F-P2-010 `extract_code_from_reason` parser is fragile and untested for edge cases

WHY: lib.rs:50-59 implements `extract_code_from_reason(reason: &str) -> Option<&str>` to extract the telemetry code from a canonical block_with_fix reason string by searching for `"Code: "` and reading until the next `.`. The hook uses this at lib.rs:574-576 to compute the `code` field for the hook.block event. However: (a) if the reason string contains a literal `Code: foo. bar` where `foo` is the code and `bar` is incidental, the parser correctly stops at the first `.`. But if the canonical format ever embeds a `.` within the code (which it shouldn't but could via a typo), the parser silently truncates. (b) No unit test verifies `extract_code_from_reason` handles the exact format produced by `block_with_fix(...)`. (c) The fallback at line 576 (`unwrap_or(HOOK_CODE_BASE)`) silently degrades to the hook prefix if parsing fails — a silent failure mode that hides parser bugs.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:50-59`
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:574-576` (silent fallback)
- Grep finds no `test_extract_code_from_reason` in lib.rs

IMPACT: If `block_with_fix` ever changes format, the code-extraction silently degrades to HOOK_CODE_BASE, all hook.block events emit the same generic code, and telemetry analytics lose the ability to distinguish CONVERGENCE_STATE_MISSING vs CONVERGENCE_PASSES_INSUFFICIENT vs CONVERGENCE_CLASSIFICATION_INSUFFICIENT. This is a silent observability degradation — exactly the class of failure SOUL.md #4 prohibits.

FIX: TEST + CODE — Add unit tests for `extract_code_from_reason` covering: (a) canonical happy path; (b) missing "Code: " prefix; (c) embedded `.` within the code segment. Replace the silent `unwrap_or(HOOK_CODE_BASE)` fallback with a `log_error` invocation when extraction fails (so the failure is observable). POLICY: implicit — observability consistency, no_silent_failures.

CONFIDENCE: MEDIUM

---

### FINDING [MEDIUM] — F-P2-011 BC-5.39.001 PC2 deferred_findings sub-schema does not match what S-12.01 / step-d5 / adversary.md tell adversary to write

WHY: BC-5.39.001 PC2 schema declares each deferred finding as `{finding_id, category, target, note}`. But step-d5-adversary-convergence.md prose describes deferred findings only by category and target (no example). And adversary.md:54 says: "The `deferred_findings` JSON field in the convergence state file records each deferred finding with fields: `finding_id`, `category`, `target` (`wave-gate` or `phase-5`), and `note`." So adversary.md and BC agree. However, the actual bootstrap_annotation backfill files have `"deferred_findings": []` (empty), so the schema isn't tested via real data. A future agent populating deferred_findings could omit `finding_id` and the system has no validator that checks this.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.001.md:62-69`
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:79` (`deferred_findings: Vec<serde_json::Value>` — accepts any JSON shape)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/plugins/vsdd-factory/skills/deliver-story/steps/step-d5-adversary-convergence.md:48-57` — describes categories and targets but not the per-entry schema

IMPACT: The hook accepts arbitrary JSON in `deferred_findings`. A typo'd `finding-id` (hyphen instead of underscore) would be silently accepted. POLICY: implicit — schema consistency.

FIX: SPEC + CODE — Either tighten the `ConvergenceState.deferred_findings` to `Vec<DeferredFinding>` with a typed struct matching the BC schema, OR add a programmatic validator that checks each deferred finding against the BC schema. Document the choice in BC-5.39.001 invariants.

CONFIDENCE: LOW

---

### FINDING [MEDIUM] — F-P2-012 hooks-registry.toml `validate-per-story-adversary-convergence` capability table only allows reading `.factory/cycles` — but the hook (post-fix) may need to read `.factory/wave-state.yaml` to enumerate stories

WHY: hooks-registry.toml:938-939 declares `[hooks.capabilities.read_file] path_allow = [".factory/cycles"]`. If F-P2-001 is fixed by reading `.factory/wave-state.yaml`, the path_allow set must be expanded. Currently a `host::read_file(".factory/wave-state.yaml")` call would return `CapabilityDenied` and the main.rs error mapping at line 70 maps this to `Ok(None)`, which would silently not produce an error — but also would not produce a story list.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/plugins/vsdd-factory/hooks-registry.toml:938-939`
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review-p2/crates/hook-plugins/validate-per-story-adversary-convergence/src/main.rs:69-70` (CapabilityDenied → Ok(None))

IMPACT: Latent risk if F-P2-001 is fixed via the wave-state.yaml route. Currently inert (paired with F-P2-001).

FIX: CODE — When fixing F-P2-001, also expand path_allow to include the chosen manifest path. Add a unit test that confirms CapabilityDenied does not silently map to "no stories" without surfacing.

CONFIDENCE: MEDIUM

---

## Observations

### OBSERVATION [LOW] [process-gap] — F-P2-013 Bootstrap-annotation field invented in PG-2 backfill without spec amendment first — process inversion

WHY: D-359 records the inline backfill of 3 convergence-state files with `bootstrap_annotation` field. The annotation was added without first amending BC-5.39.001 to declare the field. Per the order-of-operations cycle discipline, schema changes should be specified in the BC first, then the data conforms to the spec. This is a process gap: the cycle's own engine-discipline norms were violated by the fix burst that responded to the cycle's own findings. F-P2-005 is the substantive content defect; this is the underlying process gap.

EVIDENCE:
- D-359 in decision-log.md describes the inline backfill
- BC-5.39.001 v1.0 was unchanged through fix burst (verified — no v1.1 amendment)

IMPACT: A pattern that recurs: state-manager creates ad-hoc data shapes during inline operations, never propagating to the BC spec. Over time, the BC schema drifts further from reality. `[process-gap]`

FIX: PROCESS — In future cycles, any inline state file authoring that introduces a new field MUST first amend the governing BC. Add a pre-write hook or state-manager checklist step: "Before writing JSON state files with new fields, confirm the field is declared in the governing BC's PC2 schema." This could be a follow-up story under E-14.

CONFIDENCE: HIGH

---

### OBSERVATION [LOW] [process-gap] — F-P2-014 No automated check that VP descriptions in BC traceability tables match the VP's authoritative description

WHY: F-P2-002 found two BCs whose VP-071 traceability rows have stale descriptions. There is no automated check that, for every `| VP-NNN | <description> | <method> |` row in a BC's Verification Properties table, the `<description>` matches the VP's H1 or property statement. A bats lint or hook could grep BCs for VP-NNN references, fetch the actual VP file, and assert the description string is consistent. Pass-1 missed this; the fix burst missed it; pass-2 found it. The same defect class will recur.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.10.001.md:126`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.001.md:121`
- No corresponding lint hook in `plugins/vsdd-factory/hooks/` or `tests/`

IMPACT: The VP-INDEX-to-BC-traceability propagation gap is a known class (POLICY 9) but currently checked only at the VP-INDEX/architecture-doc layer, not at the BC-traceability-row layer. `[process-gap]`

FIX: PROCESS — Author a follow-up story under E-14 (or extend PG-3 placeholder lint) to add a bats test that, for each BC's VP table, fetches the VP description from the VP file and compares.

CONFIDENCE: MEDIUM

---

### OBSERVATION [LOW] — F-P2-015 The fix-plan's risk-assessment Section 6 predicted F-CRIT-4 would "introduce new pass-2 findings" because the trait change requires correct emit_event implementations — and indeed, the BC was not amended with the corresponding observability mandate

WHY: F5-pass-1-fix-plan.md Section 6 (line 374-375) explicitly predicted: "F-CRIT-4 (add emit_event to HookCallbacks): Adding a new trait method requires updating all trait implementations... A future adversary pass will check that the new emit_event implementations are correct and that the event is emitted in all block paths." Pass-2 verified the implementations are correct — but the fix-plan did not anticipate that the BC itself would need amending to codify the new mandate (F-P2-006).

EVIDENCE: F5-pass-1-fix-plan.md:374-375; this review's F-P2-006

IMPACT: The fix-plan's risk-assessment was directionally correct but incomplete — it caught the code-side risk and missed the spec-side risk. This is informational; not a defect.

FIX: None — observation only.

CONFIDENCE: LOW

---

## Novelty Assessment

Pass-2 surfaces **8 new findings** (F-P2-001 through F-P2-008) plus 4 medium and 3 observations. Of these:

- **F-P2-001 (CRITICAL):** A new finding that re-opens the supposedly-closed F-HIGH-3. The pass-1 fix added the consumer side but no producer plumbing exists. This is a **regression** in the sense of "pass-1 closed a finding that wasn't actually closed."
- **F-P2-002 (CRITICAL):** A sibling-file regression — same defect class as pass-1 F-CRIT-3/F-HIGH-5 in a different scope (BC traceability rows vs VP body). Per Partial-Fix Regression Discipline, this is a fix-propagation gap.
- **F-P2-003 (HIGH):** Direct continuation of F-LOW-1 (F1 advisory wording) — pass-1 cited only one line, but two occurrences existed in the same file. Partial fix.
- **F-P2-004 (HIGH):** Direct partial-fix gap — D-358 declared type-name corrections in VP-070 but did not propagate to S-13.01.
- **F-P2-005, F-P2-006, F-P2-007, F-P2-008 (HIGH):** Genuinely new findings exposed by examining the post-fix state. These are NOT rewords of pass-1 findings — they emerge from the fix burst itself.
- **F-P2-009 through F-P2-012 (MEDIUM):** Some related to spec-implementation completeness; F-P2-010 (extract_code_from_reason silent fallback) is genuinely new.
- **F-P2-013, F-P2-014 (process-gap):** Fresh process gaps revealed by the fix burst's own approach.

Novelty: HIGH — pass-2 surfaces 2 CRITICAL findings, including one (F-P2-001) that essentially proves the convergence hook is still inert in production. The cycle delivered the CODE for F-CRIT-4 emit_event but no SPEC update; delivered consumer-side stories for F-HIGH-3 but no producer-side wiring. The pass-2 findings are not rewordings — they are real gaps that pass-1 either missed or did not consider closed-loop.

---

## Self-Validation Loop (3 iterations)

**Iteration 1 (evidence check):** All findings have file:line evidence. F-P2-001 cross-checked via three sources: hooks-registry.toml has no [hooks.config], executor.rs:127 splices verbatim, lib.rs:269-272 returns Err on absent field. F-P2-002 cross-checked via two grep results (BC-4.10.001:126 and BC-5.39.001:121). F-P2-006 confirmed by reading BC-4.10.001 postconditions in full — no PC mentions hook.block emit.

**Iteration 2 (actionability check):** Every finding has a specific fix recommendation with route hint. F-P2-001 has three fix options ranked by effort. F-P2-006 names the exact PC number to add and the BC to bump.

**Iteration 3 (duplication check):** F-P2-001 (production hook inert) and F-P2-008 (no integration test for the inert path) are related but distinct: the first is a production wiring defect; the second is a test-coverage defect that masks the first. F-P2-002 (BC VP-row drift) and F-P2-014 (process gap: no automated lint for VP-row drift) are content + process pair. Kept separate.

After 3 iterations, all findings stand.

---

PASS_CLASSIFICATION: CRITICAL

---

# Return Summary

## (a) Findings count by severity

- CRITICAL: 2 (F-P2-001 production hook inert; F-P2-002 BC VP-row stale wording)
- HIGH: 6 (F-P2-003 F1 line 391; F-P2-004 S-13.01 type names; F-P2-005 bootstrap_annotation schema gap; F-P2-006 BC-4.10.001 missing hook.block PC; F-P2-007 BC-4.10.001 vs BC-4.10.002 SDK contradiction; F-P2-008 no integration test for hook.block production path)
- MEDIUM: 4 (F-P2-009 schema optional markers; F-P2-010 extract_code_from_reason silent fallback; F-P2-011 deferred_findings sub-schema; F-P2-012 capability path_allow latent gap)
- LOW (Observations): 3 (F-P2-013 process inversion on bootstrap_annotation; F-P2-014 missing VP-to-BC lint; F-P2-015 fix-plan blind spot)

## (b) PASS_CLASSIFICATION: CRITICAL

## (c) Top 3 findings

1. **CRITICAL F-P2-001** — `validate-per-story-adversary-convergence` is STILL operationally inert in production. Pass-1 F-HIGH-3 fix added the `extract_stories_from_config` consumer in main.rs, but the hooks-registry.toml has no `[hooks.config]` table populating `stories`, the dispatcher only splices static config, and no skill/orchestrator step writes the wave's stories anywhere the hook can read. The hook gracefully degrades on every wave-gate dispatch — exactly the pass-1 defect.
2. **CRITICAL F-P2-002** — BC-4.10.001 line 126 and BC-5.39.001 line 121 both reference VP-071 with deprecated "advisory-block" wording in their VP traceability tables, contradicting VP-071 v1.2 (which retired that pattern). Sibling-file regression of F-CRIT-3/F-HIGH-5 — pass-1 caught the wording in VP-071 itself, S-12.02, and F1-delta-analysis but missed the two BC files that anchor to the VP.
3. **HIGH F-P2-006** — BC-4.10.001 has no postcondition codifying the `hook.block` event-emission mandate that the F-CRIT-4 code fix added. Code is now ahead of spec; a future BC-only reader would not know the hook must emit `hook.block` events.

## (d) Pass-1 findings that DID NOT properly close (regressions)

- **F-HIGH-3 (list_stories)** — Code-level fix incomplete; production path still inert (F-P2-001).
- **F-HIGH-9 (S-13.01 fn names) / F-HIGH-10 (VP-070 fn names)** — Type-name portion of D-358 amendment did not propagate to S-13.01 body (F-P2-004).
- **F-LOW-1 (F1 advisory wording)** — Only line 342 fixed; line 391 in same file still has deprecated wording (F-P2-003).
- **F-CRIT-3/F-HIGH-5/F-MED-7 (VP-071 v1.2 amendment)** — VP file fully amended but VP-traceability rows in BC-4.10.001 and BC-5.39.001 still cite the deprecated description (F-P2-002).
- **F-CRIT-4 (emit_event)** — Code added but BC-4.10.001 not amended to mandate the event (F-P2-006).

## (e) NEW findings novel to this pass

- F-P2-005 — bootstrap_annotation field invented without BC schema update
- F-P2-007 — BC-4.10.001 vs BC-4.10.002 inconsistency on `host::log_*` SDK surface
- F-P2-008 — No production-path integration test for hook.block emission
- F-P2-010 — `extract_code_from_reason` silent fallback to HOOK_CODE_BASE on parse failure
- F-P2-011 — `deferred_findings` field accepts arbitrary JSON; no schema validation
- F-P2-012 — Capability `path_allow` gap latent on F-P2-001 fix path

## (f) Process-gap findings

- F-P2-013 — Inline state file backfill introduced unspecified field; spec-after-data inversion
- F-P2-014 — No automated lint for VP-description drift in BC traceability rows
- F-P2-015 — Fix-plan risk-assessment was code-side biased; missed spec-side risk for F-CRIT-4

## (g) Anything I couldn't review

- I did not run `cargo test`, `cargo kani`, or any bats suites. Test verdicts are derived from source-code reading. The fix-plan claim that B1+B2+B5 are merged with green tests is inherited.
- I did not exhaustively read all of `validate-artifact-path/src/tests.rs` (it has 1700+ lines); I covered the new BC-4.11.001 invariant 6 tests and the EC-007 perf benchmark. Other test areas may have residual catch_unwind scaffolding that would mirror F-HIGH-11.
- I did not audit each of the 9 modified creation skills' registry-read preambles word-for-word; I confirmed the bats programmatic enumeration test exists and walks the skills directory.
- I did not verify that the wasmtime build of the WASM hooks succeeds on develop HEAD (no Bash access).
- I did not read every E-14 follow-up story spec end-to-end; I confirmed the 5 stories exist via D-359 attestation but did not audit story content.