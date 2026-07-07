---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-07T00:00:00Z
phase: 4
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-brownfield-backfill
cascade: E-19-story
pass: 4
previous_review: adv-E19-pass-3.md
perimeter: E-19 epic + S-19.01..S-19.06 + STORY-INDEX E-19 section
verdict: NOT-CLEAN
blocker_count: 0
high_count: 1
medium_count: 6
low_count: 2
observation_count: 7
streak: 0/3
parent_decision: D-754
---

# Adversarial Review — E-19 Pass 4 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.06 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law; premise-verification prompt hardening applied — every finding carries its own ground-truth grep)
**Date:** 2026-07-07
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 1 / MEDIUM 6 / LOW 2 + 7 observations
**Streak:** 0/3

---

## Finding ID Convention

This review uses the in-cycle finding ID format established for the `v1.0-brownfield-backfill` cascade:

`F-P<PASS>-<SEQ>` — e.g., `F-P4-001`, `F-P4-002`, etc. Severity is stated inline in the finding header. This format is consistent with prior E-19 adversarial review passes in this cycle.

---

## Part A — Fix Verification (pass >= 2 only)

Pass-3 NOT-CLEAN B0/H5/M9/L6 (20 findings + 7 observations); F-P3-019 adjudicated FALSE-POSITIVE in-pass. Same-burst fix by 4 specialist legs (D-754). Fresh-context adversary reads only prior Part A — findings F-P3-001..F-P3-020. All 19 non-FALSE-POSITIVE findings verified CLOSED by artifact evidence at pass-4 perimeter entry. F-P3-019 = FALSE-POSITIVE (no fix required; already adjudicated D-754). New findings from pass-4 review below in Part B.

---

## Part B — New Findings

*Verbatim adversary output per D-448(a) source-attestation. Do not summarize or soften. Every finding carries independent ground-truth grep per premise-verification prompt hardening.*

F-P4-001 — HIGH — [process-gap] ADR-025 Decision 18(e) mandates that `verify-factory-lock` (the P0 lock guard) MUST be migrated to `read_prefix` once the host ABI is live. Ground-truth grep: `grep -n "verify-factory-lock\|read_prefix" .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md` confirms BC-4.13.001 v1.5 Precondition 3 still references `read_file` at the 262144-byte cap with no MUST-migrate clause or future-story anchor. Further, `grep -rn "verify-factory-lock" .factory/stories/S-19.01.md .factory/stories/S-19.02.md .factory/stories/S-19.06.md` shows S-19.02 declares it will NOT change the `read_file` call in `verify-factory-lock` (scope limited to raising the byte cap), and S-19.06 contains zero references to `verify-factory-lock` in its test acceptance criteria. The D18(e) MUST-obligation therefore has no story anchor in the E-19 wave. This is not an observation: the ADR specification contains a load-bearing SHALL/MUST obligation that is unscheduled across all 6 currently-drafted stories. Routing: architect (Option ruling) + product-owner (BC-4.13.001 amendment if Option B) + story-writer (new story or S-19.06 scope expansion if Option A).

F-P4-002 — MEDIUM — BC-1.17.001 version citations are stale at v1.0 across S-19.06 and the E-19 epic. Ground-truth grep: `grep -n "BC-1.17.001" .factory/stories/S-19.06.md .factory/stories/E-19-epic.md` returns 6 sites citing `BC-1.17.001 v1.0`. The actual current version is v1.1 (advanced D-754 fix burst; BC-1.17.001 v1.1 per D-754 decision-log.md Closes block and BC-INDEX v3.69). Six stale version-pin sites create drift-prone references under TD-VSDD-091. Routing: story-writer (sibling-sweep of BC-1.17.001 cites).

F-P4-003 — MEDIUM — BC-5.42.001 version citation is stale at v1.0 in S-19.01. Ground-truth grep: `grep -n "BC-5.42.001" .factory/stories/S-19.01.md` returns 3 sites citing `BC-5.42.001 v1.0`. The actual current version is v1.1 (advanced D-754 fix burst; per BC-INDEX v3.69). Three stale version-pin sites. Routing: story-writer.

F-P4-004 — MEDIUM — E-19 epic EAC-005 is traced solely to S-19.04 AC-001. Ground-truth grep: `grep -n "EAC-005\|AC-001\|AC-007" .factory/stories/E-19-epic.md` confirms EAC-005 `satisfied_by: [S-19.04 AC-001]` with no AC-007 reference. S-19.04 AC-001 asserts the delivery gate REJECTS new hello-hook and underscore-named WASMs; however the load-bearing EAC-005 bundle-side integration obligation (established per F-P3-018 fix burst which encoded EAC-005 requirement for live-bundle path confirmation) is actually satisfied by S-19.04 AC-007 (live-bundle path gate), not AC-001 (new-introduction gate). Tracing EAC-005 exclusively to AC-001 leaves the live-bundle integration requirement unanchored in the epic-to-story traceability chain. Routing: story-writer (epic traceability table correction).

F-P4-005 — MEDIUM — S-19.02 §Background or §Rationale cites stale size figures. Ground-truth grep: `grep -n "90KB\|466\|lines" .factory/stories/S-19.02.md` returns narrative prose citing approximately `~90KB / 466 lines` as the current STATE.md size at the time the story was drafted. BC-4.13.001 v1.5 Precondition 3 specifies `193,220 bytes / 488 lines / 74% headroom` as the 2026-07-06 ground-truth measurement. The stale figures understate the headroom urgency by 2× on file size and 5% on line count. An implementer using the story rationale to evaluate cap-raise necessity would have an incorrect baseline. Routing: story-writer.

F-P4-006 — MEDIUM — S-19.03 POLICY 8 Token Budget table is missing the BC-2.02.011 row. Ground-truth grep: `grep -n "BC-2.02.011\|Token Budget\|POLICY 8" .factory/stories/S-19.03.md` confirms the Token Budget table exists but lists only the primary story BCs (BC-2.07.001, BC-2.02.002). BC-2.02.011 was added to S-19.03's `behavioral_contracts:` frontmatter array in the D-754 fix burst (per F-P3-014 closure: S-19.03 behavioral_contracts updated to include BC-2.02.011). POLICY 8 requires the Token Budget table to carry a row for every BC in the frontmatter `behavioral_contracts` array (same-burst propagation). BC-2.02.011 frontmatter entry has no corresponding Token Budget row body. Routing: story-writer (POLICY 8 propagation to body table).

F-P4-007 — MEDIUM — S-19.03 §Architecture Anchors subsystem-anchor prose omits SS-02. Ground-truth grep: `grep -n "SS-02\|dispatcher.core\|dispatcher core" .factory/stories/S-19.03.md` returns no match in the §Architecture Anchors prose narrative. The story frontmatter `subsystems: [SS-01, SS-02]` was corrected at D-754 (F-P3-012 closure), but the §Architecture Anchors narrative body still describes only the SS-01 (host ABI module) scope without noting the SS-02 (dispatcher core) path_resolution_table impact. An implementer reading the prose narrative cannot determine which dispatcher-core registration entry or dispatch-table slot is affected. Routing: story-writer.

F-P4-008 — LOW — E-19 epic story-table contains a raw frontmatter-syntax artifact in one table cell. Ground-truth grep: `grep -n "^|.*\-\-\-\|^|.*\`\`\`" .factory/stories/E-19-epic.md` returns a table row where a cell value appears to carry residual YAML list syntax (e.g., `- item`) rather than rendered inline text. This is a markdown presentation artifact that would not parse correctly in a table renderer and creates ambiguity for tooling that parses the epic's dependency or AC columns. Routing: story-writer.

F-P4-009 — LOW — S-19.05 contains residual v1.16 BC-3.08.001 version-pin cite. Ground-truth grep: `grep -n "v1\.16\|BC-3.08.001" .factory/stories/S-19.05.md` confirms at least one table cell still cites `BC-3.08.001 v1.16`. The current version is v1.17 (advanced D-754 fix burst). O-P3-004 encoded the same class of TD-VSDD-091 drift; the D-754 fix burst corrected the body-table cell in S-19.05 v1.3 but at least one residual v1.16 cite survived in a different table cell or inline reference. Routing: story-writer (TD-VSDD-091 sibling-sweep completion).

Observations: O-P4-001 (Red-Gate staging discipline: stories S-19.01/S-19.02/S-19.03 each define Red Gate test entries but do not make explicit which bats file anchors the Red Gate — the `bats_file:` field is absent or implicit; a test-writer dispatched without that field must infer the bats file from context, risking mis-placement). O-P4-002 (intent-comment matching tolerance: S-19.01 AC-002 specifies a grep pattern for `enforce-merge-strategy.sh` content that matches on partial string; an intent comment with a different prefix but the same keyword would pass; tighter anchoring via exact function-name or line-start anchor would prevent false-pass). O-P4-003 (--merge default-injection control: S-19.01 enforce-merge-strategy.sh approach injects `--merge` into the gh pr merge invocation; if the implementer uses `gh pr merge` version that re-orders CLI flags, the injection position matters; story should specify the exact flag insertion point relative to PR-number argument). O-P4-004 (DAG restatement: E-19 epic's dependency section states the intra-epic ordering constraint as narrative prose; a machine-parseable `depends_on` matrix table listing all intra-epic constraints would reduce ambiguity for the story-writer wave-scheduler). O-P4-005 (D18(a)-(e) mapping sweep: reviewing ADR-025 Decision 18 sub-clauses (a) through (e) against S-19.06 test acceptance criteria reveals Decision 18(d) — the `timeout_expired` FFI return path — has only a PARTIAL coverage marker in S-19.06. The story's T-010 slot is reserved but has no explicit `timeout_expired` bats assertion stub; this is a test-completeness gap, not a missing spec). O-P4-006 (ci.yml job-key convention: S-19.01 EC-003 references `bats-darwin-leg-macos` as the CI job key for the macOS darwin bats leg; the actual ci.yml job key registered for darwin is `bats-full-suite`; the naming convention drift is non-blocking since the story's intent is clear, but the job-key mismatch would cause confusion for a new contributor correlating story EC cells with CI output). O-P4-007 (EC-005 synthetic-fixture note: S-19.04 AC-001's baseline-comparison test uses a synthetic release.yml fixture, not the live release.yml; a comment or annotation in the story test table noting the synthetic-fixture rationale would prevent a future test-writer from replacing the fixture with a live-file read, which would introduce test-environment coupling).

Verdict: NOT-CLEAN. BLOCKER 0 / HIGH 1 / MEDIUM 6 / LOW 2.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 1 |
| MEDIUM | 6 |
| LOW | 2 |
| Observations | 7 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Severity decay from pass 3:** B0/H5/M9/L6 → B0/H1/M6/L2 (finding volume 20→9; decay resumed; HIGH reduced from 5 to 1; dominant class: version-pin drift + POLICY 8 propagation gap)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 9 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (9 / 9) |
| **Median severity** | MEDIUM |
| **Trajectory** | 16 → 13 → 20 → 9 (decay resumed; pass-3 outlier due to POLICY 9 gaps self-introduced by pass-2 fix burst) |
| **Verdict** | FINDINGS_REMAIN — pass 5 dispatched with fresh context |

**Note:** Zero false-positives this pass. Premise-verification prompt hardening (every finding carries its own ground-truth grep) confirmed all 9 findings via independent grep evidence before reporting.

---

## Fix-Burst Closure Section (D-755)

**Human-authorized cascade. All 9 findings (F-P4-001..F-P4-009) closed across specialist legs. S-19.07 NEW added per architect Option B ruling + HUMAN APPROVAL 2026-07-07.**

**Streak: 0/3** — pass-4 verdict is NOT-CLEAN per D-628; same-burst fixes do not advance streak; pass-5 NEXT with fresh context.

### Architect leg

- **F-P4-001 ADR-025 D18(e) MUST-obligation routing:** Options evaluated: (A) Expand S-19.06 scope to include verify-factory-lock migration — REJECTED (S-19.06 scope purity; combining a new ABI implementation story with an existing-consumer migration story violates single-responsibility; S-19.06 WIP risk at migration time); (B) NEW dedicated story S-19.07 for verify-factory-lock read_prefix migration — SELECTED; (C) Defer to post-E-19 wave — REJECTED (defer-pattern violation per Canonical Principle Rule 3; D18(e) is a MUST obligation in the active ADR; no concrete future dependency makes deferral necessary). **Option B rationale:** (i) P0 lock guard must not be first consumer of an unproven host fn — S-19.06 delivers the ABI and proves correctness; S-19.07 completes the migration with a battle-tested ABI; (ii) S-19.06 scope purity preserved; (iii) W1 cap-raise (S-19.02) unblocked since BC-4.13.001 Phase-A explicitly anchors it as the interim state; (iv) migration yields structural improvement — max_bytes 262144→8192 (sufficient for verify-factory-lock STATE.md read pattern), OutputTooLarge error class eliminated from the hot path. **HUMAN APPROVED 2026-07-07: S-19.07 added to E-19 (7 stories, ~45pts).** Closes F-P4-001.

### Product-owner leg

- **BC-4.13.001 v1.5→v1.6:** Precondition 3 restructured as PHASED dual-story anchor: Phase-A `read_file@262144` active per S-19.02 (interim cap-raise; active on S-19.02 merge); Phase-B `read_prefix@8192` migration per S-19.07 (durable fix; active on S-19.07 merge). Dual-story anchor explicitly documents that Phase-A is not a permanent state. Closes F-P4-001 BC leg.
- **BC-INDEX v3.69→v3.70** (BC-4.13.001 v1.6 advance).

### Story-writer leg

- **S-19.07 v1.0 NEW:** W3, 3pts, `depends_on: [S-19.02, S-19.06]`, `behavioral_contracts: [BC-4.13.001]`, `verification_properties: []`. Acceptance criteria per architect Option B ruling: AC-001 (verify-factory-lock calls `host::read_prefix` with `max_bytes=8192` — not `host::read_file`); AC-002 (OutputTooLarge error path removed from verify-factory-lock code path); AC-003 (`verify-factory-lock-read-prefix.bats` integration test: RED under S-19.06-only baseline, GREEN after S-19.07 implementation). Blocks reciprocals set: S-19.02 `blocks: [S-19.07]` added; S-19.06 `blocks: [S-19.07]` added. Closes F-P4-001 story leg.
- **S-19.06 v1.2:** T-010 slot filled — `timeout_expired` FFI return path bats assertion stub added per O-P4-005 D18(d) gap. BC-1.17.001 version-pin updated to v1.1 (6 sites) per F-P4-002. Closes F-P4-002 (S-19.06 leg) + O-P4-005 encoding.
- **S-19.01 v1.5:** BC-5.42.001 v1.0→v1.1 version-pin updated (3 sites) per F-P4-003. Closes F-P4-003.
- **S-19.02 v1.4:** Stale size-figure prose updated: `~90KB / 466 lines` replaced with `193,220 bytes / 488 lines / 74% headroom / 35% growth` per BC-4.13.001 v1.5 Precondition 3 ground-truth. `blocks: [S-19.07]` added per S-19.07 reciprocal DAG. Closes F-P4-005.
- **S-19.03 v1.5:** POLICY 8 Token Budget table: BC-2.02.011 row added per F-P4-006. §Architecture Anchors prose extended to include SS-02 dispatcher-core dispatch-table impact per F-P4-007. Closes F-P4-006 + F-P4-007.
- **S-19.04 v1.5:** EAC-005 traceability correction: `satisfied_by` extended to include AC-007 (live-bundle path gate) alongside AC-001 (new-introduction gate) per F-P4-004. Closes F-P4-004 (story leg; epic leg also corrected in epic v1.4 below).
- **S-19.05 v1.4:** Residual `BC-3.08.001 v1.16` version-pin cite corrected to bare `BC-3.08.001` per F-P4-009 TD-VSDD-091 sweep. Orchestrator independent grep (`grep -n "v1\.16" .factory/stories/S-19.05.md`) caught a single additional cell that survived the D-754 burst correction; repaired in-burst before commit. Closes F-P4-009.
- **E-19 epic v1.4:** EAC-005 `satisfied_by` corrected to `[S-19.04 AC-001, S-19.04 AC-007]` dual-trace per F-P4-004. Raw frontmatter-syntax artifact removed from story-table cell per F-P4-008. DAG intra-epic constraint table added per O-P4-004 observation (encoded as enhancement; non-blocking). 7 stories; wave allocation: W1 S-19.01/02/03 (parallel), W2 S-19.04/05/06, W3 S-19.07. ~45pts total. Closes F-P4-004 (epic leg) + F-P4-008.
- **STORY-INDEX v4.134→v4.135** (S-19.07 row added; E-19 epic v1.4 + all story version bumps).

### Artifact versions at pass-4 closure

| Artifact | Version |
|----------|---------|
| BC-4.13.001 | v1.6 |
| BC-INDEX | v3.70 |
| VP-INDEX | v2.52 (UNCHANGED) |
| ARCH-INDEX | v2.89 (UNCHANGED) |
| S-19.01 | v1.5 |
| S-19.02 | v1.4 |
| S-19.03 | v1.5 |
| S-19.04 | v1.5 |
| S-19.05 | v1.4 |
| S-19.06 | v1.2 |
| S-19.07 | v1.0 (NEW) |
| E-19 epic | v1.4 |
| STORY-INDEX | v4.135 |

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. **NEXT:** E-19 adversarial pass-5 (fresh context).
