---
document_type: fix-plan
cycle: v1.0-feature-engine-discipline-pass-1
phase: F5-pass-1-triage
date: 2026-05-07
producer: architect
input-source: adv-cycle-pass-1.md
total_findings: 29
---

# F5 Pass-1 Fix Plan — v1.0-feature-engine-discipline-pass-1

---

## Section 1: Executive Summary

**Total findings:** 29 — 4 CRITICAL / 14 HIGH / 6 MEDIUM / 5 LOW (incl. 2 process-gap
stories + 3 additional process-gap observations)

**Severity note:** The adversary's return summary cites 6 MEDIUM and 5 LOW, but the body
also escalates two findings from MEDIUM→HIGH (F-MED-2: red-gate-log path, and F-MED-7:
VP-071 placeholder BCs). These are reflected below at HIGH to match the adversary's own
escalation text. The return-summary count discrepancy is a known adversary artifact (3
self-validation iterations did not re-tabulate after mid-document escalation). Triage
uses per-finding severity as authoritative.

**Revised severity breakdown (post-escalation):**
- CRITICAL: 4
- HIGH: 16 (14 original + 2 adversary self-escalated from MEDIUM)
- MEDIUM: 6 (8 original minus 2 escalated, but some adversary counts overlap)
- LOW (Observations): 5 (incl. 2 process-gap finding observations + 3 other process gaps
  mentioned in the return summary)

**Estimated total effort:**
- Small (≤15 min): 8 findings
- Medium (15-60 min): 14 findings
- Large (>60 min): 7 findings
- Estimated total: ~12-16 person-hours of focused fix work

**Recommended fix burst structure: 5 batches + 1 follow-up story track**

| Batch | Theme | Findings | Est. Hours |
|-------|-------|----------|-----------|
| B1 | Critical blockers (spec + config) | 6 | 3-4 h |
| B2 | Spec-implementation alignment (VPs, BCs, stories) | 8 | 3-4 h |
| B3 | Production correctness gaps (code) | 4 | 2-3 h |
| B4 | Test quality (Red Gate scaffold cleanup + coverage) | 5 | 2-3 h |
| B5 | Documentation drift (comments, F1, SKILL.md) | 6 | 1-2 h |
| B6 | Process-gap follow-up stories (NOT fixes, NOT this cycle) | 5 | n/a |

**Critical sequencing dependencies:**
1. B1 must merge before B2 — VP-071 v1.2 amendment in B1 defines the correct
   `HookResult::Block` shape; B2's code corrections depend on that canonical form.
2. B1 must merge before B3 — registry config fixes (convergence-state path, red-gate-log
   path) must land before production code changes that reference those paths.
3. B2 must merge before B4 — test cleanup in B4 references function names corrected in B2
   (S-13.01 spec amendments for `load_registry`/`matches_canonical`).
4. B3 can run in parallel with B2 (they touch different files), but both must merge before
   B4.
5. B5 (doc-only) is fully independent and can run in parallel with any batch.
6. B6 is never a blocking fix; it produces follow-up story specs after all other batches merge.

**Spec-amendment count:** 12 findings require spec amendments (SPEC-AMEND or combined
SPEC+CODE); 17 are CODE-FIX, DOC-FIX, or TEST-only.

---

## Section 2: Finding-by-Finding Triage Table

| ID | Adv. Severity | Title (short) | Route | Disposition | Effort | Batch | Risk | Notes |
|---|---|---|---|---|---|---|---|---|
| F-CRIT-1 | CRITICAL | Missing `step-d5-adversary-convergence.md` skill file | implementer | CODE-FIX (author new file) | medium | B1 | low — new file, no existing code changed | Evidence confirmed: `ls .../steps/` returns step-a..g + shared-context only; lobster line 79 references the missing path. Author the file by extracting the loop procedure from `per-story-delivery.md` lines 161-169 into skill format. |
| F-CRIT-2 | CRITICAL | ADR-017 slug mismatch in S-12.01, S-12.02, E-12 | story-writer | SPEC-AMEND (4 anchor strings in 3 files) | small | B1 | low — doc anchors only | Evidence confirmed: only `ADR-017-per-story-adversary-phasing.md` exists on disk; all 4 `inputs:`/`traces_to:` fields cite `ADR-017-per-story-adversary-three-perimeter-model.md`. Two fix options: rename the ADR file (higher cross-ref blast radius) or update the 4 anchor strings (lower risk). Recommend updating anchors to match actual slug — the "phasing" framing is accurate to the ADR body. No ADR rename — that would require its own cross-ref sweep. |
| F-CRIT-3 | CRITICAL | VP-071 spec declares non-existent `HookResult::BlockWithFix` variant | architect | SPEC-AMEND (VP-071 v1.1→v1.2) | medium | B1 | medium — VP versioning; VP-INDEX + arch doc propagation required | Evidence confirmed: `result.rs:18-32` has only `Continue`, `Block`, `Error`; VP-071 lines 91,112,130,147 all assert `HookResult::BlockWithFix { .. }`. The implementation silently corrected to `HookResult::Block { .. }` — code is right, spec is wrong. VP-071 amendment must propagate to VP-INDEX, `verification-architecture.md`, and `verification-coverage-matrix.md` per VP-INDEX propagation obligation. D-349 chose VP amendment over BC amendment — that decision is correct. |
| F-CRIT-4 | CRITICAL | `validate-per-story-adversary-convergence` never calls `host::emit_event` | implementer + architect | SPEC-AMEND + CODE-FIX | large | B1 | medium — touches HookCallbacks trait definition, trait implementations, and BC-4.10.001 | Evidence confirmed: only doc-comment reference to `emit_event` in lib.rs; no call sites. `HookCallbacks` trait lacks the method. Compare with `validate-artifact-path` which does call `emit_event`. Two sub-tasks: (a) amend BC-4.10.001 to clarify event-emission mandate (or confirm it and add the code), (b) add `emit_event` to `HookCallbacks` trait, wire in `main.rs`, add call sites before each block return. The adversary recommends the code fix over BC amendment — agreed. |
| F-HIGH-1 | HIGH | `hooks-registry.toml:931-933` "Advisory-block-mode" comment contradicts implementation | implementer | DOC-FIX (registry comment) | small | B5 | low — comment only | Evidence confirmed: line 931 says "hook returns Continue in all cases" while impl returns `HookResult::Block`. D-349 explicitly retired advisory-block for new hooks. |
| F-HIGH-2 | HIGH | bats test conflates `block_with_fix` format with `on_error = "continue"` policy | test-writer | CODE-FIX (split bats test) | small | B4 | low — test file only | Evidence confirmed: `per-story-adversary-convergence-hook.bats:156-168` merges two orthogonal assertions. Split into separate tests; remove "advisory-block mode" wording. |
| F-HIGH-3 | HIGH | `RealCallbacks::list_stories` always returns `Err` — hook operationally inert in WASM | implementer | CODE-FIX | large | B3 | high — production behavior change; dispatch infra may need to populate `plugin_config.stories` | Evidence confirmed: `main.rs:61-70` returns `Err(IoError(...))`. Hook silently degrades to Continue in every production wave-gate dispatch. This is a delivery defect per AC-013 / BC-4.10.001 PC1. Recommend option (1): read `plugin_config.stories` from payload. Requires orchestrator/wave-gate dispatcher change to populate the field before `SubagentStop`. This is the highest-impact code fix in the set — needs careful coordination. |
| F-HIGH-4 | HIGH | `RealCallbacks::log_debug` calls `host::log_info` contradicting BC-4.10.002 PC3 | architect | SPEC-AMEND (BC-4.10.002 PC3) | small | B2 | low — BC wording only; code is correct | Evidence confirmed: `main.rs:74-76` maps `log_debug` to `host::log_info` with an explanatory comment. HOST_ABI v1 has no `log_debug`; `log_info` is the correct floor. BC must be amended to match reality. |
| F-HIGH-5 | HIGH | VP-071 kani harness uses `HookResult::BlockWithFix` but impl uses `HookResult::Block` | architect | SPEC-AMEND (same VP-071 v1.2 as F-CRIT-3) | small | B1 | medium — subsumed by F-CRIT-3 VP amendment | This finding is addressed entirely by the VP-071 v1.2 amendment from F-CRIT-3. No separate work required beyond that amendment. Keeping as a separate row for completeness. |
| F-HIGH-6 | HIGH | `pattern_matches` may match empty-placeholder paths; behavior undefined | architect + test-writer | SPEC-AMEND + TEST | medium | B2 | medium — changes BC-4.11.001 invariant 6 semantics; test writer adds negative cases | Evidence confirmed: `lib.rs:190-252` algorithm. Adversary demonstrates `.factory/specs//y.md` could match a single-segment pattern. Decision required: is `{placeholder}` single-segment or multi-segment? Recommend single-segment (non-empty, no `/`). This is a NEEDS-CLARIFICATION on the semantic choice, but the fix direction is clear. See Notes column. **NEEDS-CLARIFICATION on semantics (single vs multi-segment)**. |
| F-HIGH-7 | HIGH | `relocate-artifact/SKILL.md` `allowed-tools: Read, Bash` with no `git mv` enforcement | test-writer | TEST (add bats test asserting `git log --follow`) | small | B4 | low — test addition only | Evidence confirmed: SKILL.md line 6. The finding is about lack of enforcement, not broken behavior. Add bats test per adversary recommendation. |
| F-HIGH-8 | HIGH | Wave-gate Gate 3 "out of scope: within-story findings" too aggressive vs bootstrap | architect | SPEC-AMEND (wave-gate/SKILL.md:88) | small | B2 | low — SKILL.md update only | Evidence confirmed: `SKILL.md:88` says "assumed converged" with no bootstrap exception carveout. D-354 documents the bootstrap exception explicitly. Amend to reference D-354 and retain blocking authority for CRITICAL/HIGH within-story findings at wave perimeter. |
| F-HIGH-9 | HIGH | S-13.01 AC-001 and T-2 cite `parse_registry` (does not exist; actual: `load_registry`) | story-writer | SPEC-AMEND (S-13.01 story spec) | small | B2 | low — story spec doc update | Evidence confirmed: `lib.rs:119` is `load_registry`; story spec says `parse_registry` throughout (lines 71, 295, 422, 470). |
| F-HIGH-10 | HIGH | VP-070 kani harness references `match_path` (does not exist; actual: `matches_canonical`) | architect | SPEC-AMEND (VP-070 spec) | small | B2 | low — VP spec doc update; kani harness in lib.rs is already correct | Evidence confirmed: `lib.rs:150` is `matches_canonical`; VP-070 lines 56, 81, 89, 90 say `match_path`. |
| F-HIGH-11 | HIGH | All 30+ convergence hook unit tests retain Red Gate `catch_unwind` scaffold | test-writer | CODE-FIX (remove catch_unwind wrapping) | large | B4 | low — test-only change; no production code affected | Evidence confirmed: `lib.rs:750-767` and 30+ subsequent tests all use `catch_unwind`. The comment at line 581-592 confirms this was intentional Red Gate scaffolding but should have been cleaned up post-implementation. Production code does not panic; wrapping now produces misleading failure messages. All 30 tests need refactoring to direct `let result = hook_logic(...)`. |
| F-HIGH-12 | HIGH | S-12.02 AC-002 still uses deprecated "advisory-block-mode" wording after OQ-9 | story-writer | SPEC-AMEND (S-12.02 story spec) | small | B2 | low — story spec wording; implementation is correct | Evidence confirmed: `S-12.02:81-93` and Architecture Compliance Rule #1 at lines 322-330 both describe advisory-block. OQ-9 (D-349) retired this pattern for new hooks. |
| F-HIGH-13 | HIGH | Registry `state-runtime-adversary` points to `.factory/adversary-convergence-state.json` (root) not per-story path | implementer | CODE-FIX (update registry YAML entry) | small | B1 | high — integration defect; the entire S-12.01/S-12.02 machinery is bricked post-bootstrap without this fix | Evidence confirmed: `artifact-path-registry.yaml:131-134` has root-level path; BC-5.39.001 PC2 mandates per-story path. This is a critical registry config defect affecting every post-bootstrap story. |
| F-HIGH-14 | HIGH | Registry `cycle-story-implementation` pattern missing `{story-id}` segment (red-gate-log path) | implementer | CODE-FIX (update registry YAML entry) | small | B1 | high — per-story red-gate-log writes will fail post-bootstrap | Evidence confirmed: `per-story-delivery.md:39` writes to `.factory/cycles/<cycle-id>/<story-id>/implementation/red-gate-log.md`; registry pattern at line 105-108 has no `{story-id}` segment. Same defect class as F-HIGH-13. |
| F-MED-1 | MEDIUM | `relocate-artifact/SKILL.md` missing explicit scope clause for non-`.factory/` paths | architect | SPEC-AMEND (SKILL.md) | small | B5 | low — doc clarification | Evidence: SKILL.md and BC-6.22.001 PC2. Low risk today; add explicit scope clause per adversary recommendation. |
| F-MED-2 | MEDIUM (adversary escalated to HIGH) | Registry `cycle-story-implementation` red-gate-log path mismatch | implementer | CODE-FIX | small | B1 | high | Identical to F-HIGH-14. The adversary addressed this as a separate finding but it is the same defect — both the convergence-state entry (F-HIGH-13) and the cycle-story-implementation entry (F-HIGH-14) are covered by the same B1 registry config fix. This row is a duplicate reference to F-HIGH-14. **TRIAGE DISPOSITION: Merged with F-HIGH-14 — single fix covers both.** |
| F-MED-3 | MEDIUM | `vp-072-sot-invariant.bats` only spot-checks named skills, not all `create-*` skills | test-writer | TEST (add programmatic enumeration test) | medium | B4 | low | Evidence: `vp-072-sot-invariant.bats:90-100`; F1 OQ-4 flagged this gap. |
| F-MED-4 | MEDIUM | `validate-artifact-path` EC-006 branch (missing `file_path`) doesn't emit `hook.warn` event | implementer | CODE-FIX | small | B3 | low — observability gap, no correctness impact | Evidence: `lib.rs:355-365` logs but doesn't emit event; `lib.rs:411-421` does emit. Add one `emit_event` call to the EC-006 branch. |
| F-MED-5 | MEDIUM | No benchmark enforcing BC-4.11.001 EC-007 ≤200ms ceiling for `matches_canonical` | test-writer | TEST (add criterion benchmark or bats perf test) | medium | B4 | low | Evidence: no `benches/` directory exists in `validate-artifact-path`. |
| F-MED-6 | MEDIUM | `agents/adversary.md` Three-Perimeter Scope Contract doesn't state story-spec path lookup | architect | SPEC-AMEND (adversary.md) | small | B5 | low | Evidence: `adversary.md:40`. Add Glob guidance for story-spec discovery. Adversary rated LOW; triage agrees but keeps MEDIUM per adversary body text. |
| F-MED-7 | MEDIUM (adversary escalated to HIGH) | VP-071 frontmatter `bcs:` contains placeholder `BC-5.NN.002` | architect | SPEC-AMEND (VP-071 frontmatter) | small | B1 | medium — subsumed by VP-071 v1.2 amendment from F-CRIT-3 | Evidence confirmed: `VP-071.md:29-31` and line 201 both cite `BC-5.NN.002`. Fix together with F-CRIT-3 in the VP-071 v1.2 amendment. Correct value: `BC-4.10.001` and `BC-5.39.001`. |
| F-MED-8 | MEDIUM | `validate-per-story-adversary-convergence` wave-gate identity match too literal | implementer | CODE-FIX (pattern match or shared constant) | small | B3 | medium — any identity drift silently disables the gate | Evidence: `lib.rs:200` matches exact string `"wave-gate-dispatch"`. Add SDK constant or `starts_with("wave-gate")` match. |
| F-MED-9 | MEDIUM [process-gap] | Policy-rubric injection into adversary dispatch is manual (orchestrator.md:243) | orchestrator skill → follow-up story | SPEC-AMEND (process-gap story) | n/a | B6 | low — process improvement | Not a code fix; a process automation story. See Section 5. |
| F-LOW-1 | LOW | F1-delta-analysis VP-071 description uses deprecated "advisory-block" wording | state-manager | DOC-FIX (F1-delta-analysis.md:342) | small | B5 | low — F1 is upstream proposal doc | Evidence: `F1-delta-analysis.md:342`. Update to VP-071 v1.1 Block Invariant wording. |
| F-LOW-2 | LOW [process-gap] | F5 adversary dispatch flow assumes adversary writes its own document | follow-up story | PROCESS (follow-up story) | n/a | B6 | low | Adversary correctly surfaced this. State-manager already handled it for this pass. Follow-up story to formalize the dispatch → state-manager persist pattern in phase-5 skill/lobster. |
| F-LOW-3 | LOW [process-gap] | Bootstrap cohort lacks `adversary-convergence-state.json` files; no F7 backfill protocol | state-manager + orchestrator | PROCESS (follow-up story + state backfill) | n/a | B6 | medium — without this, next wave-gate CONVERGENCE_STATE_MISSING for 3 stories | Per D-354, state backfill must happen at F7 cycle-close for the 3 bootstrap stories. |
| F-LOW-4 | LOW [process-gap] | D-355 overstated "no open spec contradictions" — VP-071 placeholder BC IDs were open | state-manager | DOC-FIX (decision-log amendment note + pre-F5 lint proposal) | small | B5 | low — retrospective note only | Add an amendment note to D-355 or produce follow-up story for pre-F5 placeholder lint. |
| F-LOW-5 | LOW [process-gap] | 6 new BC files have `input-hash: "[pending-recompute]"` — drift baseline never set | state-manager | CODE-FIX (run `/vsdd-factory:check-input-drift`, accept hashes) | small | B1 (as part of spec amendment burst) | low | Evidence: `BC-4.10.001.md:12` and all 5 sibling BCs. Run check-input-drift and persist the 6 correct hashes atomically. Grouped into B1 since state-manager already touches BC files for other B1 amendments. |

---

## Section 3: Batch Plan

### B1 — Critical Blockers (spec + config corrections)

**Contents:** F-CRIT-1, F-CRIT-2, F-CRIT-3/F-HIGH-5 (VP-071 v1.2), F-CRIT-4, F-HIGH-13,
F-HIGH-14, F-MED-7, F-LOW-5

**Why together:** These are the findings that cause hard runtime failures or corrupt
traceability on the next story delivery. The registry config fixes (F-HIGH-13, F-HIGH-14)
and the VP amendment (F-CRIT-3/F-HIGH-5/F-MED-7) are individually small but must land
before anything downstream depends on correct spec shapes. The Lobster skill authoring
(F-CRIT-1) and ADR anchor corrections (F-CRIT-2) are independent but small enough to co-batch.
The input-hash recomputation (F-LOW-5) is trivially added to the same state-manager burst.

**Agents:** implementer (skill file + registry YAML), story-writer (ADR anchors in E-12,
S-12.01, S-12.02), architect (VP-071 v1.2 amendment with VP-INDEX + arch doc propagation),
state-manager (atomic commit + input-hash recomputation)

**Effort estimate:** 3-4 hours

**Rationale for SPEC-AMEND on VP-071:** The implementation's kani harness is correct
(`HookResult::Block`). The spec is wrong. VP amendment is the right direction per D-349.

### B2 — Spec-Implementation Alignment

**Contents:** F-HIGH-4 (BC-4.10.002 PC3 log_debug→log_info), F-HIGH-6 (pattern_matches
empty-placeholder semantics, pending clarification), F-HIGH-8 (Gate 3 bootstrap carveout),
F-HIGH-9 (S-13.01 parse_registry→load_registry), F-HIGH-10 (VP-070 match_path→matches_canonical),
F-HIGH-12 (S-12.02 advisory-block wording)

**Why together:** All are spec documents with wording that disagrees with the
implementation. No code changes required. Safe to batch as a single spec-amendment PR.
The pattern_matches semantic decision (F-HIGH-6) may need human input first — see
NEEDS-CLARIFICATION note.

**Agents:** architect (BC-4.10.002, VP-070, wave-gate/SKILL.md), story-writer (S-13.01,
S-12.02), state-manager

**Effort estimate:** 3-4 hours

**Dependency:** VP-071 v1.2 (B1) should be merged first so B2's BC-4.10.002 amendment
references a consistent VP suite. B2 does not hard-depend on B1 for its individual fixes,
but the VP-INDEX must be consistent when B2 lands.

### B3 — Production Correctness Gaps (code)

**Contents:** F-HIGH-3 (list_stories always Err — production hook inert), F-MED-4
(EC-006 missing hook.warn event), F-MED-8 (wave-gate identity literal match)

**Why together:** These are runtime-behavior code fixes. F-HIGH-3 is the most impactful —
the hook is completely inert in production. F-MED-8 is a guard-logic fix. F-MED-4 is
an observability addition. All three touch `validate-per-story-adversary-convergence`
source (lib.rs, main.rs, or shared SDK constants).

**Agents:** implementer, test-writer (add tests for new code paths), state-manager

**Effort estimate:** 2-3 hours (F-HIGH-3 is the bulk — requires orchestrator coordination
to understand how `plugin_config.stories` gets populated before SubagentStop; see risk note)

**Sequencing:** B1 (registry fixes) should merge first, but B3 can proceed in parallel
with B2 since they touch different files.

### B4 — Test Quality

**Contents:** F-HIGH-2 (bats conflation test split), F-HIGH-7 (relocate-artifact git mv
enforcement bats test), F-HIGH-11 (remove catch_unwind Red Gate scaffold from 30+ tests),
F-MED-3 (vp-072 enumeration coverage), F-MED-5 (perf benchmark)

**Why together:** All are test-file-only changes. F-HIGH-11 is the largest effort (30+
test refactors). F-MED-5 requires a new benchmark setup (could be a criterion crate add
or a bats perf test — either way, small scope).

**Agents:** test-writer, state-manager

**Effort estimate:** 2-3 hours (F-HIGH-11 bulk)

**Dependency:** B2 must merge first (function name corrections) so the refactored tests
cite the correct function names.

### B5 — Documentation Drift

**Contents:** F-HIGH-1 (hooks-registry.toml advisory-block comment), F-MED-1
(relocate-artifact scope clause), F-MED-6 (adversary.md story-spec lookup), F-LOW-1
(F1-delta-analysis VP-071 wording), F-LOW-4 (D-355 overstated claim retrospective note)

**Why together:** Pure documentation fixes (comments, SKILL.md prose, agent docs, cycle
docs). Zero production behavior change. Safe to batch and run in parallel with any other
batch.

**Agents:** implementer (hooks-registry.toml comment), architect (SKILL.md scope clause,
adversary.md, F1-delta-analysis, decision-log note), state-manager

**Effort estimate:** 1-2 hours

**Sequencing:** No dependencies. Can run in parallel with B1 or any other batch.

### B6 — Process-Gap Follow-Up Stories

**Contents:** F-MED-9 [process-gap], F-LOW-2 [process-gap], F-LOW-3 [process-gap],
F-LOW-4 (partial: pre-F5 lint story), F-LOW-5 (partial: if not handled in B1)

See Section 5 for full story proposals.

---

## Section 4: Sequencing and Dependencies

```
B5 (doc-only) ─────────────────────────────────────────────────────────────────────────┐
                                                                                        │
B1 (CRITICALs + registry + VP-071 v1.2)                                               │
  │                                                                                     │
  ├──► B2 (spec-impl alignment)                                                        │
  │      │                                                                              │
  │      ├──► B4 (test quality)  ◄──────────────────────────────────────────────────── │
  │      │                                                                              │
  │      └──► (convergence check)                                                       │
  │                                                                                     │
  └──► B3 (production code) ───────────► B4 (test quality)                             │
                                                                                        │
B6 (process-gap stories) ───────────► separate cycle or follow-up sprint               │
                                                                                        │
All B1..B5 merged ──────────────────► F5 pass-2 adversarial re-dispatch ◄──────────────┘
```

**Notes on cross-batch dependencies:**

- F-CRIT-3/F-HIGH-5/F-MED-7 (VP-071) is a SINGLE VP amendment (v1.2). These three
  findings all fix in one SPEC-AMEND operation in B1. They must not be split.
- F-HIGH-13 and F-HIGH-14 (registry config) both modify `artifact-path-registry.yaml`.
  They must be applied in the same commit to avoid any window where one path is
  registered and the other is not.
- F-HIGH-3 (list_stories inert) in B3 requires the wave-gate dispatcher to populate
  `plugin_config.stories`. This may require a small orchestrator skill update or
  wave-gate SKILL.md change. If that turns out to span a separate story, B3 may need
  to be split — architect should flag this during execution.
- F-MED-2 is a duplicate of F-HIGH-14. No separate row needed in execution.

---

## Section 5: Process-Gap Findings — Follow-Up Stories

### PG-1: F5 Adversary Dispatch — State-Manager Persist Pattern

**Source finding:** F-LOW-2 (adversary returns chat text; state-manager must persist)

**Proposed story title:** `S-next-auto-persist-adversary-findings`

**Target epic:** E-12 (Engine Governance — per-story adversary infrastructure)

**Acceptance criteria sketch:** The F5 phase skill (or lobster workflow) dispatches
adversary, captures chat output, and dispatches state-manager with the captured text
to write to the canonical adversary-review path. No adversary dispatch prompt instructs
the adversary to write directly.

**Recommended timing:** Author during this cycle's F7 close-out; deliver in the
next engine-discipline pass-2 cycle.

---

### PG-2: Bootstrap Cohort State-File Backfill at F7 Close

**Source finding:** F-LOW-3 (S-12.01, S-12.02, S-13.01 lack convergence-state files;
F7 must backfill them)

**Proposed story title:** `S-next-bootstrap-convergence-state-backfill`

**Target epic:** E-12 (Engine Governance)

**Acceptance criteria sketch:** State files exist at
`.factory/cycles/v1.0-feature-engine-discipline-pass-1/{S-12.01,S-12.02,S-13.01}/adversary-convergence-state.json`
with `passes_clean: 3`, `last_classification: NITPICK_ONLY`, and an annotation field
recording that F5 pass-1 (2026-05-07) was the source of convergence evidence. Hook's
wave-gate check on future wave dispatches no longer returns `CONVERGENCE_STATE_MISSING`
for these stories.

**Recommended timing:** Must complete as part of F7 cycle close-out (blocking for
correct wave-gate behavior). Do not defer to a future cycle — this will brick the next
wave-gate dispatch for any new story in E-12/E-13.

---

### PG-3: Pre-F5 Placeholder Lint Check

**Source findings:** F-LOW-4 (D-355 overclaim), F-LOW-5 (input-hash pending-recompute)

**Proposed story title:** `S-next-pre-f5-artifact-lint`

**Target epic:** New engine-discipline-pass-2 cycle or E-12 continuation

**Acceptance criteria sketch:** A lint check (bats or hook) runs before F5 dispatch and
fails if any new VP/BC/ADR frontmatter contains placeholder strings (`BC-S.NN`, `VP-NNN`,
`[pending-recompute]`, `see PO output`, `TBD`). The check outputs a finding list and
blocks F5 entry.

**Recommended timing:** Author in next engine-discipline pass-2 story decomposition.

---

### PG-4: Policy-Rubric Auto-Injection for Adversary Dispatch

**Source finding:** F-MED-9 (orchestrator.md:243 policy rubric injection is manual)

**Proposed story title:** `S-next-adversary-policy-rubric-injection`

**Target epic:** E-12 (Engine Governance)

**Acceptance criteria sketch:** The F5 adversary dispatch skill (or lobster step) reads
`.factory/policies.yaml` and injects all policies under a `## Project Policy Rubric`
heading before dispatching the adversary subagent. Manual per-dispatch inclusion of
policy instructions is retired.

**Recommended timing:** Author in next engine-discipline pass-2; can be authored
concurrently with PG-1.

---

### PG-5: Convergence-State Backfill Coverage (wave-gate exemption registry)

This is a sub-requirement of PG-2. If the hook's `list_stories` is fixed (F-HIGH-3/B3),
the wave-gate will start checking convergence state. The bootstrap cohort must be
backfilled (PG-2) OR explicitly exempted in the registry before the next wave-gate
dispatch. Recommend backfill (PG-2) as the cleaner solution — no code required, just
state file authoring.

---

## Section 6: Risk Assessment

### Worst-case wrong-order scenario

Fixing B3 (list_stories implementation) before B1 (registry config) would make the
convergence hook active in production while the per-story convergence-state path is still
registered at the wrong root path. The first non-bootstrap story's wave-gate dispatch
would immediately fail with `CONVERGENCE_STATE_MISSING` or `ARTIFACT_PATH_UNREGISTERED`
because the state file write goes to a per-story path that the registry blocks. This is
a hard developer-experience regression.

**Mitigation:** B1 must merge and be confirmed in production before B3 is dispatched.

### Parallel-safe fix pairs

The following finding pairs are safe to work in parallel (separate worktrees, no shared files):

- B5 (doc-only) with any other batch — all doc fixes are in distinct files
- F-HIGH-9 (S-13.01 story spec) with F-HIGH-12 (S-12.02 story spec) — different files
- F-HIGH-3 (lib.rs/main.rs in validate-per-story-adversary-convergence) with F-CRIT-2
  (story spec anchor strings)
- F-MED-5 (benchmark addition) with any B2 spec amendment

### Sequential-only fixes (shared state)

- `artifact-path-registry.yaml` — F-HIGH-13 and F-HIGH-14 must be in the same commit
- `VP-071.md` — F-CRIT-3, F-HIGH-5, and F-MED-7 must be one atomic amendment
- `VP-INDEX.md`, `verification-architecture.md`, `verification-coverage-matrix.md` —
  must update together when VP-071 is amended (VP-INDEX propagation obligation)
- `validate-per-story-adversary-convergence/src/lib.rs` — F-CRIT-4 (add emit_event
  to trait), F-HIGH-3 (implement list_stories), F-HIGH-11 (remove catch_unwind), and
  F-MED-8 (wave-gate identity) all touch the same file. If split across B1/B3/B4, the
  batches must merge in strict order to avoid conflicts. Recommend consolidating all
  lib.rs changes into B1+B3 with a careful merge plan.

### Fixes that might introduce new pass-2 findings

- **F-CRIT-4 (add emit_event to HookCallbacks):** Adding a new trait method requires
  updating all trait implementations (RealCallbacks, FakeCallbacks, and any test
  implementations). A future adversary pass will check that the new emit_event
  implementations are correct and that the event is emitted in all block paths.
- **F-HIGH-3 (implement list_stories):** The implementation will read `plugin_config.stories`.
  A pass-2 adversary might find that `plugin_config.stories` is not populated by the
  dispatcher, or that the implementation fails on empty/malformed story lists.
- **F-HIGH-6 (pattern_matches semantics):** Whatever semantic decision is made
  (single-segment vs multi-segment `{placeholder}`), pass-2 will verify the implementation
  matches. If the decision is single-segment, the adversary will likely check for a test
  asserting `a/b/c` does NOT match `{placeholder}`.

---

## Section 7: Recommended Execution Plan

### Pre-execution gate

- Obtain human sign-off on F-HIGH-6 NEEDS-CLARIFICATION (single-segment vs multi-segment
  `{placeholder}` semantics). This decision must precede B2 spec amendment.
- Confirm the PG-2 bootstrap backfill timing: F7 close-out (this cycle) or next cycle?
  The answer affects whether wave-gate is safe to use before cycle close.

### B1 Execution (critical blockers, ~3-4 hours)

1. Architect: Produce VP-071 v1.2 amendment (F-CRIT-3/F-HIGH-5/F-MED-7). Propagate to
   VP-INDEX, verification-architecture.md, verification-coverage-matrix.md in same burst.
2. Story-writer: Update E-12 `traces_to:` + `inputs:`, S-12.01 `inputs:`, S-12.02
   `inputs:` — change `ADR-017-per-story-adversary-three-perimeter-model.md` to
   `ADR-017-per-story-adversary-phasing.md` (F-CRIT-2).
3. Implementer: Author
   `plugins/vsdd-factory/skills/deliver-story/steps/step-d5-adversary-convergence.md`
   from `per-story-delivery.md` lines 161-169 (F-CRIT-1).
4. Implementer: Add `emit_event` method to `HookCallbacks` trait and wire in
   `RealCallbacks` (`main.rs`). Add `emit_event` call sites before each block return in
   `hook_logic` (F-CRIT-4). Coordinate with BC-4.10.001 spec wording to confirm the
   event mandate.
5. Implementer: Update `artifact-path-registry.yaml` — fix `state-runtime-adversary`
   path pattern and `cycle-story-implementation` path pattern in one atomic change
   (F-HIGH-13 + F-HIGH-14).
6. State-manager: Run `check-input-drift` on 6 BC files; accept recomputed hashes (F-LOW-5).
7. State-manager: Produce single-commit burst per TD-VSDD-053.
8. Dispatch security-review + pr-reviewer on B1 fix-PR.
9. Merge B1.

### B2 Execution (spec alignment, ~3-4 hours, after B1 merges and F-HIGH-6 clarified)

1. Architect: Amend BC-4.10.002 PC3 (log_debug→log_info) (F-HIGH-4).
2. Architect: Update BC-4.11.001 invariant 6 with `{placeholder}` semantics decision;
   add test stubs for negative cases (F-HIGH-6).
3. Architect: Amend `wave-gate/SKILL.md:88` to add bootstrap exception carveout (F-HIGH-8).
4. Story-writer: Update S-13.01 AC-001 and T-2 (`parse_registry`→`load_registry`,
   `match_path`→`matches_canonical`) (F-HIGH-9).
5. Architect: Update VP-070 to use `matches_canonical` (F-HIGH-10).
6. Story-writer: Amend S-12.02 AC-002 and Architecture Compliance Rule #1 (F-HIGH-12).
7. State-manager: Commit burst.
8. Dispatch security-review + pr-reviewer. Merge B2.

### B3 Execution (production code, ~2-3 hours, after B1 merges)

1. Implementer: Implement `RealCallbacks::list_stories` to read `plugin_config.stories`.
   Coordinate with wave-gate SKILL.md on how stories list is populated before SubagentStop
   (may require a wave-gate orchestration change). (F-HIGH-3)
2. Implementer: Add `emit_event("hook.warn", ...)` in the EC-006 branch of
   `validate-artifact-path/src/lib.rs:355-365`. (F-MED-4)
3. Implementer: Change `graceful_degrade_outside_wave_gate` to match
   `identity.starts_with("wave-gate")` or a shared SDK constant. Document canonical
   identity string in BC-4.10.001 invariant 1. (F-MED-8)
4. Test-writer: Add unit tests for the new list_stories path.
5. State-manager: Commit burst. Dispatch security-review + pr-reviewer. Merge B3.

### B4 Execution (test quality, ~2-3 hours, after B2 and B3 merge)

1. Test-writer: Split `per-story-adversary-convergence-hook.bats:156-168` into two tests.
   Remove "advisory-block mode" wording. (F-HIGH-2)
2. Test-writer: Add bats test in `relocate-artifact.bats` asserting `git log --follow`
   on moved artifact. (F-HIGH-7)
3. Test-writer: Remove `catch_unwind` wrapping from all 30+ convergence hook unit tests.
   Replace with direct `let hook_result = hook_logic(...)` pattern. Update inline
   comments. (F-HIGH-11)
4. Test-writer: Add programmatic enumeration test in `vp-072-sot-invariant.bats` (F-MED-3).
5. Test-writer: Add criterion benchmark or bats perf assertion for `matches_canonical`
   ≤200ms ceiling (F-MED-5).
6. State-manager: Commit burst. Dispatch security-review + pr-reviewer. Merge B4.

### B5 Execution (doc-only, ~1-2 hours, parallel with any batch)

1. Implementer: Update `hooks-registry.toml:931-933` comment from advisory-block to
   canonical block_with_fix description. (F-HIGH-1)
2. Architect: Add explicit scope clause to `relocate-artifact/SKILL.md`. (F-MED-1)
3. Architect: Amend `agents/adversary.md` Three-Perimeter Scope Contract with story-spec
   lookup guidance. (F-MED-6)
4. Architect: Update `F1-delta-analysis.md:342` VP-071 description. (F-LOW-1)
5. Architect: Add amendment note to D-355 in decision-log. (F-LOW-4)
6. State-manager: Commit burst. Merge B5 (no security review needed for doc-only changes,
   but pr-reviewer SHOULD review for wording accuracy).

### B6 Execution (follow-up story authoring, after all other batches merge)

1. Story-writer: Author the 4 process-gap follow-up story specs (PG-1 through PG-4).
2. Product-owner: Assign to E-12 or new engine-discipline-pass-2 epic as appropriate.
3. State-manager: Update STORY-INDEX with new story IDs.
4. PG-2 backfill: State-manager writes 3 convergence-state JSON files for bootstrap
   cohort as part of F7 close-out (not a new story — direct state-manager action).

### Pass-2 Dispatch

After all B1..B5 batches merge and PG-2 bootstrap backfill is complete:

1. Verify all 29 findings have confirmed fixes (check fix-PR commit log).
2. Dispatch adversary for F5 pass-2 with the worktree at current HEAD on develop.
3. State-manager persists pass-2 review output (per PG-1 improvement, or manually as with pass-1).

**Estimated total end-to-end time (excluding human review delays and NEEDS-CLARIFICATION
resolution):** 12-16 person-hours of agent work. With reasonable parallelism (B5 + B1
in parallel, then B2/B3 in parallel, then B4), wall-clock time is approximately 6-8 hours.

---

## Appendix A: NEEDS-CLARIFICATION Items

### NC-1: F-HIGH-6 — `{placeholder}` Single-Segment vs Multi-Segment Semantics

**Question:** Should a `{placeholder}` in an `artifact-path-registry.yaml` pattern match:
- (A) A single path segment only (no `/` in matched content), e.g., `{cycle-id}` matches
  `v1.0-foo` but NOT `v1.0-foo/sub`, or
- (B) One or more path segments (may span `/`), e.g., `{cycle-id}` matches both
  `v1.0-foo` and `v1.0-foo/sub`.

**Why it matters:** BC-4.11.001 invariant 6 says "any non-empty path segment or sequence
of segments" — Option B. But Option B means `.factory/cycles/foo/bar/decision-log.md`
could match `.factory/cycles/{cycle-id}/decision-log.md`, which would incorrectly classify
a double-nested path as a cycle decision-log. Option A is safer but contradicts the current
BC wording.

**Evidence:** `lib.rs:190-252` adversary analysis; `tests.rs:540-568` (no negative test).

**Recommended decision:** Option A (single-segment, non-empty). The pattern semantics
should be tightened. Current registry patterns do not require multi-segment matching,
and single-segment semantics are more predictable. BC-4.11.001 invariant 6 should be
amended to read: "`{placeholder}` matches any non-empty sequence of characters that does
not contain `/` (single path segment)."

**Who decides:** Human or architect-level decision. This is a semantic API contract change.

---

## Appendix B: Adversary Self-Escalations

The adversary body text escalated two findings mid-document:

1. **F-MED-2 → HIGH:** "CONFIDENCE: HIGH (escalating to HIGH given the same pattern
   as the convergence-state file finding)" — this is F-HIGH-14 in this plan; the
   adversary's return summary counts it separately as a MEDIUM.
2. **F-MED-7 → HIGH:** "CONFIDENCE: HIGH (escalating to HIGH)" — VP-071 placeholder
   BCs. Triage aligns with the escalated severity.

The adversary's final return summary count (6 MEDIUM, 5 LOW) does not reflect these
escalations. The per-finding severity in the body is authoritative.

---

## Appendix C: Anomalies Surfaced During Triage

1. **F-MED-2 is a duplicate of F-HIGH-14.** The adversary surfaced the red-gate-log
   path mismatch as both F-MED-2 (cycle-story-implementation registry) and F-HIGH-14
   (same finding framed differently). Both are addressed by updating the registry YAML
   entry to include `{story-id}`. The fix-plan consolidates these into a single B1
   registry edit.

2. **F-CRIT-4 (missing emit_event) may interact with F-HIGH-3 (list_stories inert).**
   Both require changes to `lib.rs` (HookCallbacks trait) and `main.rs` (RealCallbacks).
   In B1 (emit_event), the trait gets a new method. In B3 (list_stories), the production
   implementation changes. These must not conflict at merge time. Recommend B1 lands and
   is confirmed before B3 begins work on the same files.

3. **The adversary's self-identified tooling constraint is itself a finding.** The
   adversary correctly surfaced its own process gap (F-LOW-2) — the dispatch flow
   contradicts its read-only profile. The adversary performed exactly as designed (read
   only, return findings as text). The gap is in the orchestration layer, not the agent.

4. **CRITICAL severity on F-CRIT-4 is debatable.** The adversary calls missing
   `emit_event` CRITICAL because it violates a mandate in BC-4.10.001. However, the
   hook's core blocking behavior (returning `HookResult::Block`) is intact; only the
   observability event emission is absent. Triage agrees with CRITICAL classification
   because the `hook.block` event is cited in the registry comment and BC-4.10.001
   postconditions as part of the observable contract. A monitoring dashboard built on
   `hook.block` events would never see any convergence-gate firings. Severity stands.
