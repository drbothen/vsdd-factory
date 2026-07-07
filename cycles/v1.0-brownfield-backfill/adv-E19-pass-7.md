---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-07T00:00:00Z
phase: 7
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-brownfield-backfill
cascade: E-19-story
pass: 7
previous_review: adv-E19-pass-6.md
perimeter: E-19 epic + S-19.01..S-19.07 + STORY-INDEX
verdict: NOT-CLEAN
blocker_count: 0
high_count: 2
medium_count: 5
low_count: 5
observation_count: 7
streak: 0/3
parent_decision: D-758
---

# Adversarial Review — E-19 Pass 7 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law; rubric = policies.yaml read directly; 20 policies)
**Date:** 2026-07-07
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 2 / MEDIUM 5 / LOW 5 (12 findings + 7 observations; counts matched enumeration this pass — pass-6 count-discrepancy class did not recur)
**Streak:** 0/3

> **ORCHESTRATOR ADJUDICATION NOTE (D-758 — F-P7-003 FALSE-POSITIVE):** F-P7-003 as stated by the adversary asserted that S-19.06 uses u64/i64 types while ADR-025 Decision 15 mandates u32/i32. Ground-truth grep: `grep -n "u64\|i64\|u32\|i32" .factory/specs/architecture/decisions/ADR-025*.md | grep -i "read_prefix\|Decision 15"` returns `u32/i32` at lines 1126 and 1194 — ADR-025 Decision 15 has carried u32/i32 since v1.9. The adversary's evidence grep struck only S-19.06's *narrative* description of a historical conflict note, never the ADR text itself. The adversary's premise (ADR mandates u64/i64) is FACTUALLY INCORRECT. F-P7-003 is reclassified as FALSE-POSITIVE. The REAL defect uncovered by closer inspection is narrower: S-19.06 carries a stale inline note reading "ADR uses u64/i64 — BC wins" describing a conflict that was already resolved when ADR-025 was updated to v1.9. That stale note is corrected in this pass's fix burst. This is the third premise-verification failure caught by orchestrator ground-truthing this cascade (L-BB-finding-premise class; no new lesson — existing lesson extended in applicability note). Trajectory count for this pass = 12 (all enumerated findings including F-P7-003 as initially stated, before reclassification; adversary-stated count preserved per D-448(a) source-attestation).

---

## Finding ID Convention

This review uses the in-cycle finding ID format established for the `v1.0-brownfield-backfill` cascade:

`F-P<PASS>-<SEQ>` — e.g., `F-P7-001`, `F-P7-002`, etc. Severity is stated inline in the finding header. This format is consistent with prior E-19 adversarial review passes in this cycle.

---

## Part A — Fix Verification (pass >= 2 only)

Pass-6 NOT-CLEAN B0/H5/M2/L1 (5 enumerated HIGH + stated M2/L1 with no enumerated bodies per orchestrator adjudication D-757); closed same-burst (D-757; 1 story-writer leg). Fresh-context adversary reads only prior Part A — findings F-P6-001..F-P6-005. All 5 enumerated findings verified CLOSED by artifact evidence at pass-7 perimeter entry:

- **F-P6-001 CLOSED** (S-19.04 v1.6→v1.7 POLICY 17→POLICY 20 sweep across all AC cells and §Background prose; orchestrator literal grep confirmed 0 live POLICY-17 residuals in S-19.04; D-757 story-writer leg).
- **F-P6-002 CLOSED** (S-19.04 v1.7 case-arm narrative corrected to pass-through ground truth: `) ;;` arm PASSES THROUGH to `cp`; fix moves pair to explicit skip/continue path; control-flow trace evidence cited; O-P6-002 byte-verification cited; D-757 story-writer leg).
- **F-P6-003 CLOSED** (E-19 epic v1.5→v1.6 BC-3.08.001 v1.16/v1.17 → v1.18 sweep; all three load-bearing clauses updated; orchestrator grep confirmed 0 live stale BC-3.08.001 version cites; D-757 story-writer leg).
- **F-P6-004 CLOSED** (STORY-INDEX S-19.05 row head-cite cell v1.17→v1.18; D-757 story-writer leg).
- **F-P6-005 CLOSED** (STORY-INDEX v4.136→v4.137 narrative quad VP-INDEX cell corrected from v2.52 to v2.53; re-derived from live VP-INDEX after fix burst; D-757 story-writer leg).

New findings from pass-7 review below in Part B.

---

## Part B — New Findings

*Verbatim adversary output per D-448(a) source-attestation. Do not summarize or soften. Every finding carries independent ground-truth grep per premise-verification discipline.*

F-P7-001 — HIGH — [process-gap] STORY-INDEX internal drift on BC-coverage summary line. Ground-truth verification: `grep -n "S-19.05\|BC-3.08.001\|BC-coverage" .factory/stories/STORY-INDEX.md | head -20` returns two BC-3.08.001 cites in the E-19 section — the per-story row `head_cite` cell for S-19.05 reads `BC-3.08.001 v1.18` (updated at pass-6 closure D-757; F-P6-004 fix) BUT the E-19 epic BC-coverage summary line at the bottom of the E-19 STORY-INDEX section shows `BC-3.08.001 v1.17` — the F-P6-004 sweep was scoped to the row `head_cite` cell only; the sibling summary line was missed. POLICY 14 5-leg parity mandates that all parity sites are updated in the same burst. The E-19 BC-coverage summary line is parity site 5 for the S-19.05 BC-coverage annotation. Fix: story-writer STORY-INDEX v4.137→v4.138 — exhaustive BC-coverage line re-derivation from live BC headers (BC-4.13.001 v1.6; BC-2.07.001 v1.1; BC-2.02.011 v1.4; BC-3.08.001 v1.18) replacing all stale version tokens in the summary line with current values; literal grep to confirm 0 live stale BC-3.08.001 version tokens in E-19 section after fix.

F-P7-002 — HIGH — [process-gap] S-19.04 carries 7 volatile line-number pins in normative AC prose, violating TD-VSDD-091 anti-volatile-pin policy, and the story is missing a POLICY 20 compliance-rule row. Ground-truth verification: `grep -n "~line\|line [0-9]\|:[0-9]\{3\}" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | grep -v "^---\|Changelog\|last_amended\|modified"` returns 7 occurrences of `~line NNN` patterns in the AC-001, AC-002, AC-003, AC-004, AC-006, and AC-007 prose (specific forms: `~line 47`, `~line 51`, `~line 53`, `~line 58`, `~line 62`, `~line 68`, `~line 71`). These are volatile line-number anchors that will drift on the next diff to `release.yml`. TD-VSDD-091 mandates stable pattern anchors (function names, behavioral anchors), not file:NNN forms. Additionally, `grep -n "POLICY 20\|release_bundle_no_dev_samples\|compliance-rule\|compliance row" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | grep -i "compliance"` returns zero rows in the §Traceability or §Compliance table — POLICY 20 `release_bundle_no_dev_samples` is the governing policy for this story's bundle-hygiene scope, and it must appear as a compliance row. Fix: story-writer S-19.04 v1.7→v1.8 — (a) 7 volatile `~line NNN` anchors replaced with stable behavioral pattern descriptions (e.g., `the artifact-staging loop's case-statement open arm`, `the hello-hook binary-copy step`); (b) POLICY 20 `release_bundle_no_dev_samples` compliance row added to §Traceability; orchestrator literal grep confirms 0 live `~line` residuals in normative S-19.04 prose after fix.

F-P7-003 — MEDIUM — [premise-verification-failure — ORCHESTRATOR ADJUDICATION: FALSE-POSITIVE on the ADR premise] S-19.06 §Traceability note reads "ADR-025 Decision 15 specifies u64/i64 types for offset/length; BC-1.17.001 specifies u32/i32; BC wins per BC-SPEC-WINS standing rule." Adversary premise: this implies a live conflict between ADR and BC. Ground-truth verification by adversary: `grep -n "u64\|i64\|u32\|i32" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -10` returns the note text. Adversary did NOT grep the actual ADR text. **ORCHESTRATOR ADJUDICATION:** `grep -n "u64\|i64\|u32\|i32" .factory/specs/architecture/decisions/ADR-025*.md | grep -i "read_prefix\|Decision 15"` returns u32/i32 at lines 1126 and 1194 — ADR-025 Decision 15 has carried u32/i32 since v1.9. The adversary's premise is FACTUALLY INCORRECT; the ADR does NOT specify u64/i64. F-P7-003 is FALSE-POSITIVE — there is no live ADR vs BC conflict. The REAL (narrower) defect: the S-19.06 inline note is describing a historical conflict that was already resolved at ADR-025 v1.9 — the note is stale and misleading. Fix: story-writer S-19.06 v1.2→v1.3 — stale "ADR uses u64/i64 — BC wins" reconciliation note replaced with a correct historical note (e.g., "ADR-025 Decision 15 v1.9 was updated to u32/i32 matching BC-1.17.001; no conflict as of ADR-025 v1.9").

F-P7-004 — MEDIUM — BC-2.07.001 EC-007 negative-control is structurally unimplementable as a test on real POSIX filesystems. Ground-truth verification: `grep -n "EC-007\|no existing ancestor\|/ always" .factory/specs/behavioral-contracts/ss-02/BC-2.07.001.md | head -10` returns the EC-007 precondition text specifying "a path where `canonicalize()` returns an error for every ancestor attempted by the walk." On any portable Unix filesystem (Linux, macOS), the root `/` always exists and always canonicalizes successfully — the "no existing ancestor" precondition is a dead branch in practice; a bats integration test that tries to exercise it by pointing at e.g. `/nonexistent/path/file` would still reach `/` as an ancestor and return the synthesized path rather than exhausting all ancestors. The dead-branch nature means the EC-007 integration test would either (a) silently pass without exercising the code branch, or (b) require an artificial in-process filesystem mock. The BC does not currently specify the testability mechanism. Fix: product-owner BC-2.07.001 v1.1→v1.2 — EC-007 reformulated to acknowledge the dead-branch nature on real filesystems and prescribe an injectable `canonicalize` function parameter in `path_util::resolve_path_for_allowlist` enabling unit tests to inject a mock that returns `Err` for every path; testability note added stating S-19.03 AC-001 negative-control B MUST inject such a mock and assert `path_allowed() == false` + `reason = path_resolution_failed`.

F-P7-005 — MEDIUM — [process-gap] S-19.01 AC-004 claims to close `L-BB-simulation-shell-dialect-gap` but the mechanism is asserted by prose only; no release.yml script fragment is actually tested under `/bin/bash 3.2` within the story's acceptance criteria. Ground-truth verification: `grep -n "bash.*3\.2\|/bin/bash\|while.*IFS\|mapfile" .factory/stories/S-19.01-pr-manager-hardening.md | head -15` returns the AC-004 gate text referring to "the while-IFS-read loop pattern validated under /bin/bash 3.2" as a closing mechanism — but the gate as written asserts the presence of the fix (the `while IFS= read -r` pattern) rather than specifying a mechanically-gated test that executes the script under Apple `/bin/bash 3.2.57` (or equivalent 3.2-compatible shell). `L-BB-simulation-shell-dialect-gap` was codified specifically because dev-host bash-5 validation gives a false-green for bash-3.2 incompatibilities (mapfile). The gate closes the lesson by detection-of-the-pattern, not by execution-under-3.2. Fix: story-writer S-19.01 v1.5→v1.6 — AC-004 amended to require: (a) the specific while-IFS-read fragment from the actual rc.22 failure site (`while IFS= read -r line; do ... done < <(...)`) cited verbatim, AND (b) a mechanism test confirming this fragment is syntactically valid under a bash-3.2 invocation (`bash --version 2>&1 | grep "version 3\.2"` or equivalent guard + `bash -n` check under the relevant script path), not just presence-of-pattern.

F-P7-006 — MEDIUM — BC-4.13.001 Invariant 10 upper-boundary blind spot: the soft-warn condition `bytes_read > soft_warn_threshold` (200000) does not have an explicit upper-bound qualifier, meaning an implementation reading it as `> 200000 AND < 262144` (exclusive upper) would silently omit the `state_md_approaching_cap` event when `bytes_read == 262144` — the most alarming observable state (exactly at the readable cap, 1 byte from failure). Ground-truth verification: `grep -n "Invariant 10\|soft_warn\|262144\|> 200000\|> soft_warn" .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md | head -15` returns Invariant 10 text with `bytes_read > soft_warn_threshold` condition and no explicit upper-bound qualifier that forces inclusive behavior at 262144. An implementation could emit the warn for 200001..262143 and silently skip 262144 — conforming to the stated condition while leaving the most alarming state unwarned. Fix: product-owner BC-4.13.001 v1.6→v1.7 — Invariant 10 amended: (a) condition restated as `bytes_read > 200000 AND bytes_read ≤ cap_bytes (262144)` to make the cap-boundary inclusive; (b) boundary table added: 200000 → no warn; 200001 → warn; 262144 → warn+readable (most alarming, MUST warn); 262145 → OUTPUT_TOO_LARGE (unreachable since host caps at 262144); (c) soft-warn range explicitly stated as `bytes_read ∈ (200000, 262144]` inclusive at cap.

F-P7-007 — MEDIUM — S-19.05 EC-005 test fixture specifies a synthetic `plugin.completed` JSON payload without a concrete runtime path and references a fixture directory that does not exist in the story perimeter. Ground-truth verification: `grep -n "EC-005\|fixture\|fixture_dir\|synthetic\|plugin\.completed" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -15` returns EC-005 test vector row referencing a fixture path like `tests/fixtures/event-payloads/plugin-completed-async.json` or similar. `ls .factory/stories/` confirms no such fixture directory exists within the story's documented scope. The EC-005 fixture is a forward-reference to a path that will only exist after the story's tests are written — not a currently-verifiable gate. Additionally, the entry_index field in the EC-005 synthetic payload cannot be verified without a runtime dispatch. Fix: product-owner BC-3.08.001 v1.18→v1.19 — EC-005/Invariant 6 clarified: `entry_index` is a schema-level defense (0-based ordinal from `enumerate()` of the async partition at dispatch time), not a runtime dispatch gate; its correctness is verified by serialization/property tests over the event struct (asserting the ordinal is correctly marshalled), NOT by a runtime concurrent-dispatch fixture; S-19.05 AC-002 Gate amended accordingly (schema-level serialization/property test, not EC-005 runtime fixture). Story-writer S-19.05 v1.5→v1.6 — EC-005 fixture path cite removed from AC-002 Gate; test recast as serialization/property assertion on `entry_index` field type and derivation; T-006 recast as grep-inspection of the emitted JSON for the `entry_index` key presence.

F-P7-008 — LOW — E-19 epic Description item 2 lists "factory-lock guard OUTPUT_TOO_LARGE silent-pass" as a Phase-A item only, when this item actually spans Phase-A (S-19.02 implementation) and Phase-B (S-19.07 read_prefix migration which removes the OUTPUT_TOO_LARGE class entirely). The epic Description does not enumerate the Phase-B closure of this item. Fix: story-writer E-19 epic v1.6→v1.7 — Description item 2 amended to note Phase-A+B dual-story scope (S-19.02 implements the guard fix; S-19.07 migrates to read_prefix eliminating the TooLarge class structurally). Observation: non-blocking at draft stage; noted for implementer clarity.

F-P7-009 — LOW — S-19.05 T-006 test row is structurally redundant with T-003: both assert that `entry_index` is present in the emitted event. T-006 adds no additional stimulus or observable distinction from T-003. Ground-truth verification: `grep -n "T-006\|T-003\|entry_index" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -10` returns T-003 and T-006 both asserting `entry_index` presence in the `plugin.completed` event with identical stimulus conditions. Fix: story-writer S-19.05 v1.5→v1.6 — T-006 recast as a grep-inspection test (asserting `entry_index` key is present AND its value is a non-negative integer in the raw JSONL output) rather than a duplicate presence assertion. This distinguishes T-006 from T-003 and adds value.

F-P7-010 — LOW — S-19.04 AC-004 Gate states "assert non-zero exit from `grep` when no matching artifacts found" but does not specify whether this means pipeline exit-code propagation through the `set -euo pipefail` frame or an explicit `exit 1` call. An implementation that uses `grep ... || true` (silencing grep's non-zero exit) would pass the behavioral criterion while defeating the gate. Fix: story-writer S-19.04 v1.7→v1.8 — AC-004 Gate amended to explicitly specify that the gate asserts an intrinsic non-zero exit from grep (i.e., the absence pattern MUST NOT be wrapped with `|| true`; the pipeline's pipefail semantics MUST propagate the non-zero); test vector updated to confirm the grep non-zero propagates to the script's exit code.

F-P7-011 — LOW — S-19.06 AC-003 Gate "assert no doc-comment call-sites remain after cleanup" is fragile: a grep on `read_file` or `read_prefix` that does not exclude comment lines would flag doc-comment examples in test files or example code as false positives. Ground-truth verification: `grep -n "AC-003\|doc.comment\|#.*read_file" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -10` returns AC-003 gate text that asserts call-site removal without a `grep -v '#'` or `--include` scope qualifier. Fix: story-writer S-19.06 v1.2→v1.3 — AC-003 Gate scoped to non-comment semantic call-sites: grep for `read_file(` excluding lines where the first non-whitespace character is `//` or `#` (Rust doc-comment and attribute lines); ensures the gate is not tripped by doc-comment examples or test scaffolding.

F-P7-012 — LOW — S-19.02 T-001..T-005 and T-007..T-009 test rows are not present as inline unit test specifications in the story body; they appear only in the red-gate test plan table header row with no content. Separately, T-006 (an explicitly inline test) shows the pattern that other rows should follow. Ground-truth verification: `grep -n "T-00[1-9]\|T-0[1-9]" .factory/stories/S-19.02-verify-factory-lock-output-too-large.md | head -15` returns T-001..T-009 headers in the test plan table but all rows except T-006 lack inline test vector content. This creates implementation ambiguity: a test-writer must infer test content from the AC descriptions rather than the normative T-row content. Fix: story-writer S-19.02 v1.5→v1.6 — T-001..T-005 and T-007..T-009 given full inline test vector content consistent with BC-4.13.001 v1.7 boundary semantics including the inclusive-upper-bound case (T-NNN for `bytes_read == 262144` MUST emit warn per Invariant 10 amended).

Observations:

O-P7-001 — E-19 epic §Description could explicitly note the phased-continuation nature of the Phase-A/Phase-B split for factory-lock guard items (item 2) — currently the Phase-A/Phase-B distinction is in §BC Traceability but not summarized in the Description overview. Non-blocking.

O-P7-002 — S-19.07 §Verification Properties is empty (no VP entries). This is consistent with its scope (guard migration, not new behavioral property) but a rationale note would be helpful: "No new VPs; Phase-B migration satisfies existing VP-098 path-resolution coverage via the shared path_util module (unchanged); VP-094..VP-096 cover the migrated guard behavior under read_prefix." Non-blocking.

O-P7-003 — BC-3.08.001 v1.18 §Canonical Test Vectors row for EC-008 (async plugin completes within drain window) could enumerate the mandatory `entry_index` field in the Expected Output column alongside `plugin_version` — the current EC-008 row notes `entry_index` in the §Edge Cases prose but not in the test vector. (ACCEPTED-WITH-RECORD per D-758 orchestrator review; non-blocking at draft stage.)

O-P7-004 — S-19.04 §Traceability does not cite the dual-registry cross-check (hooks-registry.toml + resolvers-registry.toml) as a mandatory prerequisite for any future orphan-detection gate AC. (ACCEPTED-WITH-RECORD per D-758 orchestrator review; not actionable in this story's scope; anchor: S-19.04 AC-006 bats gate — already has dual-registry note in AC-006 body.)

O-P7-005 — Task 1 in all S-19.01..S-19.07 stories references PR-description checklist item "E-19 wave and story acceptance" but does not enumerate the specific items that define done for the wave-level acceptance condition. Suggests adding a cross-reference to E-19 epic EAC-001..EAC-005 in Task 1 prose. Non-blocking. (Not actioned this pass; epic EAC list is already self-contained.)

O-P7-006 — BC-2.07.001 §Related BCs cites BC-1.17.001 (read_prefix BC) as a sibling that inherits the same path_allow + rejoin resolution model. It would be useful to add a forward-note that BC-1.17.001 does NOT use the `resolve_path_for_allowlist` function (read_prefix has a fixed max_bytes semantics; it handles absent-path differently). (ACCEPTED-WITH-RECORD per D-758 orchestrator review; non-blocking; BC-1.17.001 is out of scope for this pass.)

O-P7-007 — S-19.05 §Background could note that the `VSDD_SINK_FILE` environment variable is currently the only runtime switch for enabling the telemetry sink and that a future story might introduce a compile-time feature gate or registry-level config for the sink. Non-blocking. (ACCEPTED-WITH-RECORD per D-758 orchestrator review; out of scope.)

---

## Verifications That PASSED

The following 13 structural checks were confirmed clean at pass-7 perimeter entry:

1. Bidirectional DAG parity PASS: all `depends_on` / `blocks` reciprocals verified for S-19.01..S-19.07 at pass-7 perimeter entry.
2. F-P6-002 control-flow ground truth PASS: S-19.04 v1.7 now correctly describes `) ;;` as pass-through and specifies explicit skip/continue path as the fix; no residual inversion.
3. POLICY 20 sweep PASS: S-19.04 v1.7 cites POLICY 20 `release_bundle_no_dev_samples` in all normative AC cells and §Background; zero POLICY-17 residuals (orchestrator grep D-757 confirmed).
4. ADR/BC version matrix PASS: ADR-025 v1.9 / ADR-030 v1.0 / BC-5.42.001 v1.1 / BC-2.07.001 v1.1 / BC-1.17.001 v1.1 / BC-4.13.001 v1.6 / BC-3.08.001 v1.18 consistent with D-757 state.
5. S-19.03 Red Gate discipline PASS: stub constant -1000 (non-zero, out-of-band; guaranteed failing Red Gate).
6. S-19.02 drift-tolerant rationale PASS: range-based size expression in place; no per-pass stale point-in-time citation.
7. BC-INDEX total_bcs PASS: 1,977 consistent; no orphan BCs in E-19 frontmatter arrays.
8. VP-INDEX completeness PASS: VP-094..VP-101 + VP-079 all registered.
9. STORY-INDEX totals PASS: story_count 130, E-19 45pts correctly reflect 7 stories post-S-19.07 addition.
10. ARCH-INDEX v2.89 unchanged PASS: ADR-030 v1.0 and ADR-025 v1.9 registered; no architectural changes since D-754.
11. BC-2.02.011 epic traceability PASS: BC-2.02.011 row in epic §BC Traceability table; S-19.03 ↔ epic bidirectional trace confirmed.
12. S-19.06 dependency chain PASS: depends_on [S-19.03] correctly anchored; BC-1.17.001 v1.1 cited; SS-02 present in subsystems[].
13. S-19.07 Phase-B anchor PASS: S-19.07 v1.1 depends_on [S-19.02, S-19.06]; W3 placement sound; BC-4.13.001 Phase-B anchor correctly cites S-19.07.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 2 |
| MEDIUM | 5 |
| LOW | 5 |
| Observations | 7 |

*Actionable findings: 12 (all enumerated; F-P7-001..F-P7-012). F-P7-003 reclassified as FALSE-POSITIVE per orchestrator adjudication; the narrow real defect (stale reconciliation note) is fixed in scope.*

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Severity decay from pass 6 (enumerated):** B0/H5/M0/L0 (5 actionable) → B0/H2/M5/L5 (12 total; volume tick-up reflects deeper probe layers: testability semantics, boundary tables, fixture realizability — NOT re-found classes from prior passes; novelty score 1.0)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 7 |
| **New findings** | 12 (F-P7-001..F-P7-012; F-P7-003 reclassified FALSE-POSITIVE) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (12 / 12) |
| **Median severity** | MEDIUM |
| **Trajectory (findings per pass)** | 16 → 14 → 20 → 9 → 8 → 5 → 12 |
| **Verdict** | FINDINGS_REMAIN — pass 8 dispatched with fresh context |

**Note on volume tick-up at pass 7:** The increase from 5 (pass-6 enumerated) to 12 reflects the adversary reaching deeper probe layers not previously examined — testability semantics (EC-007 dead branch, EC-005 fixture realizability), boundary table completeness (Invariant 10 inclusive upper), and pin-stability (TD-VSDD-091 volatile `~line` patterns). These are distinct classes from the POLICY-ID, control-flow-inversion, and BC-version-cite classes found in passes 5-6. The count increase is therefore a diagnostic signal of scope deepening, not regression or duplicate recurrence.

---

## Fix-Burst Closure Section (D-758)

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. Same-burst fixes do NOT advance streak.

**All 12 findings (including F-P7-003 real defect) closed. Orchestrator verified zero residuals via body-scoped greps before commit.**

### Product-owner leg

- **BC-2.07.001 v1.1→v1.2 (F-P7-004):** EC-007 reformulated — precondition changed from "no existing ancestor on real filesystem" (dead branch on portable Unix; `/` always canonicalizes) to "mock-injectable canonicalize returns Err for every ancestor." `path_util::resolve_path_for_allowlist` MUST accept injectable `canonicalize` fn parameter (signature `fn(&Path) -> std::io::Result<PathBuf>`) enabling unit tests to inject a mock returning `Err(...)` for every path. EC-007 expected-behavior updated with testability note; S-19.03 AC-001 negative-control B ruling for story-writer: inject mock canonicalize returning `Err(...)` for every ancestor; assert `path_allowed() == false` + reason `path_resolution_failed`. BC-INDEX v3.71→v3.72. Closes F-P7-004.

- **BC-4.13.001 v1.6→v1.7 (F-P7-006):** Invariant 10 upper-boundary blind spot closed — soft-warn range made explicitly inclusive at cap: `bytes_read ∈ (200000, 262144]`. Condition restated as `bytes_read > 200000 AND bytes_read ≤ cap_bytes (262144)`. Boundary table added: 200000 → no warn; 200001 → warn; 262144 → warn+readable (MUST warn — most alarming state); 262145 → OUTPUT_TOO_LARGE. BC-INDEX v3.72→v3.73. Closes F-P7-006.

- **BC-3.08.001 v1.18→v1.19 (F-P7-007):** EC-005 / Invariant 6 clarification — `entry_index` ruled schema-level defense: its correctness verified by serialization/property tests over the event struct (asserting 0-based ordinal from `enumerate()` is correctly marshalled), NOT by a runtime concurrent-dispatch fixture; schema-level note added to both Event 5 and Event 6 `entry_index` semantics paragraphs and Invariant 6; S-19.05 AC-002 Gate story-writer ruling: property/serialization test, not EC-005 runtime fixture. BC-INDEX v3.73→v3.74. Closes F-P7-007.

- **BC-INDEX v3.71→v3.74:** Three sequential increments (one per BC amendment above); total_bcs UNCHANGED 1,977; POLICY 7 H1 titles UNCHANGED; sequenced legs per D-757 NEW RULE. Closes BC index leg.

### Story-writer leg

- **S-19.01 v1.5→v1.6 (F-P7-005):** AC-004 amended with concrete darwin-leg mechanism test — the while-IFS-read loop from the actual rc.22 failure site cited verbatim; mechanism test confirms fragment syntactically valid under bash-3.2 via `bash -n` under Apple `/bin/bash 3.2`-compatible check; presence-of-pattern gate upgraded to execution-under-3.2 gate. O-P7-005 Task 1 PR-description checklist: EAC cross-reference note added. Closes F-P7-005.

- **S-19.02 v1.5→v1.6 (F-P7-012 + F-P7-006 boundary conformance):** T-001..T-005 and T-007..T-009 given full inline test vector content; BC-4.13.001 v1.7 Invariant 10 inclusive-upper boundary reflected (T-NNN for `bytes_read == 262144` asserts warn fired); BC Trace version updated to v1.7; Task 12 (boundary test) added. O-P7-005 present. Closes F-P7-012 (test row completeness); reflects F-P7-006 fix.

- **S-19.03 v1.6→v1.7 (F-P7-004 BC conformance):** AC-001 negative-control B updated per BC-2.07.001 v1.2 ruling: inject mock canonicalize function returning `Err(std::io::Error::from(std::io::ErrorKind::NotFound))` for every ancestor; assert `path_allowed() == false` + dispatcher emits `internal.capability_denied reason=path_resolution_failed`. O-P7-005 present. Closes F-P7-004 story leg.

- **S-19.04 v1.7→v1.8 (F-P7-002 + F-P7-010):** (a) 7 volatile `~line NNN` anchors replaced with stable behavioral pattern anchors (e.g., `the artifact-staging loop's case-statement open arm for the underscore-pair pattern`, `the hello-hook binary-copy step after the case-esac block`); POLICY 20 `release_bundle_no_dev_samples` compliance row added to §Traceability. (b) AC-004 Gate amended: intrinsic non-zero exit explicitly required (grep non-zero MUST propagate; `|| true` wrapping FORBIDDEN); test vector confirms non-zero propagation through `set -euo pipefail`. TD-VSDD-091 compliance row added per POLICY gap. Orchestrator grep confirms 0 live `~line` residuals after fix. O-P7-005 present. Closes F-P7-002 + F-P7-010.

- **S-19.05 v1.5→v1.6 (F-P7-007 + F-P7-009):** (a) AC-002 Gate recast from EC-005 runtime-dispatch fixture to serialization/property test per BC-3.08.001 v1.19 ruling; EC-005 fixture-path cite removed; BC table updated to v1.19. (b) T-006 recast as grep-inspection test: asserts `entry_index` key is present AND value is a non-negative integer in raw JSONL output (distinct from T-003 presence-only assertion). (c) RG-003 (Red Gate row) updated to reflect schema-level test scope. O-P7-005 present. Closes F-P7-007 + F-P7-009.

- **S-19.06 v1.2→v1.3 (F-P7-003 real defect + F-P7-011):** (a) Stale "ADR uses u64/i64 — BC wins" reconciliation note replaced with correct historical note: "ADR-025 Decision 15 was updated to u32/i32 at v1.9, aligning with BC-1.17.001; no conflict as of ADR-025 v1.9." (b) AC-003 Gate scoped to non-comment semantic call-sites: grep for `read_file(` with `grep -vE '^\s*(//)|(#)'` pre-filter to exclude Rust doc-comment and attribute lines; prevents false-positive on doc-comment examples. O-P7-005 present. Closes F-P7-003 real defect + F-P7-011.

- **S-19.07 v1.1→v1.2 (O-P7-002):** §Verification Properties rationale note added: "No new VPs authored for Phase-B migration; existing VP-098 path-resolution coverage extends via the shared path_util module (unchanged); VP-094..VP-096 cover the migrated guard behavior under read_prefix semantics." O-P7-005 present. Closes O-P7-002.

- **E-19 epic v1.6→v1.7 (F-P7-008 + O-P7-001):** Description item 2 amended to enumerate Phase-A+B dual-story scope (S-19.02 implements guard fix in Phase-A; S-19.07 migrates to read_prefix in Phase-B, eliminating OUTPUT_TOO_LARGE class structurally). O-P7-001 phased-continuation note added to §Description overview. BC-3.08.001 v1.18→v1.19 sweep in §PRD/Out-of-Scope/BC Traceability prose (sibling-sweep per F-P7-001 root cause). Closes F-P7-008 + O-P7-001.

- **STORY-INDEX v4.137→v4.138 (F-P7-001):** Exhaustive BC-coverage line re-derivation from live BC headers — all six BC versions re-derived via literal grep at Commit time: BC-4.13.001 v1.7; BC-2.07.001 v1.2; BC-2.02.011 v1.4; BC-3.08.001 v1.19; BC-5.42.001 v1.1; BC-1.17.001 v1.1 — replacing all stale version tokens in the E-19 section BC-coverage summary line. Orchestrator grep confirmed 0 live stale BC-3.08.001 version tokens in E-19 section after sweep. Sequential leg ordering observed per D-757 NEW RULE (BC-INDEX legs sequenced before STORY-INDEX leg). Closes F-P7-001.

### Artifact versions at pass-7 closure

| Artifact | Version |
|----------|---------|
| BC-INDEX | v3.74 |
| VP-INDEX | v2.53 (UNCHANGED) |
| ARCH-INDEX | v2.89 (UNCHANGED) |
| BC-2.07.001 | v1.2 |
| BC-4.13.001 | v1.7 |
| BC-3.08.001 | v1.19 |
| S-19.01 | v1.6 |
| S-19.02 | v1.6 |
| S-19.03 | v1.7 |
| S-19.04 | v1.8 |
| S-19.05 | v1.6 |
| S-19.06 | v1.3 |
| S-19.07 | v1.2 |
| E-19 epic | v1.7 |
| STORY-INDEX | v4.138 |

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. **NEXT:** E-19 adversarial pass-8 (fresh context; 20-policy rubric; trajectory 16→14→20→9→8→5→12→pass-8).
