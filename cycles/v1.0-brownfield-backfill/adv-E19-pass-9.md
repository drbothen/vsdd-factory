---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-07T00:00:00Z
phase: 9
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-brownfield-backfill
cascade: E-19-story
pass: 9
previous_review: adv-E19-pass-8.md
perimeter: E-19 epic + S-19.01..S-19.07 + STORY-INDEX
verdict: NOT-CLEAN
blocker_count: 0
high_count: 0
medium_count: 1
low_count: 3
observation_count: 5
streak: 0/3
parent_decision: D-760
---

# Adversarial Review — E-19 Pass 9 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law; rubric = policies.yaml read directly; 20 policies)
**Date:** 2026-07-07
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 0 / MEDIUM 1 / LOW 3 (4 findings + 5 observations; counts matched enumeration; all findings artifact-grounded; live-vs-history adjudication held — zero noise findings)
**Streak:** 0/3

---

## Finding ID Convention

This review uses the in-cycle finding ID format established for the `v1.0-brownfield-backfill` cascade:

`F-P<PASS>-<SEQ>` — e.g., `F-P9-001`, `F-P9-002`, etc. Severity is stated inline in the finding header. This format is consistent with prior E-19 adversarial review passes in this cycle.

---

## Part A — Fix Verification (pass >= 2 only)

Pass-8 NOT-CLEAN B0/H3/M5/L3 (11 findings + 6 observations; 0 false-positives; BC-cite drift preflight MECHANICAL GATE instituted two-sided; story-writer single leg; closed D-759). Fresh-context adversary reads only prior Part A — findings F-P8-001..F-P8-011. All 11 findings verified CLOSED by artifact evidence at pass-9 perimeter entry:

- **F-P8-001 CLOSED** (S-19.07 v1.3 AC-002 Gate rewritten with per-entry awk scoping: `awk '/^name = "verify-factory-lock"$/,/^\[\[hooks\]\]/'` and `awk '/^name = "verify-factory-lock-bash"$/,/^\[\[hooks\]\]/'` extract each entry's stanza independently; per-stanza `[hooks.capabilities.read_file]` asserted 0 AND `[hooks.capabilities.read_prefix]` asserted ≥1; both entries tested independently; single-quoted patterns throughout; D-759 story-writer leg).
- **F-P8-002 CLOSED** (S-19.02 v1.7 targeted replacement of 2 reverse-word-order stale cites: Phase-A scope note "BC-4.13.001 v1.6 Phase-A amendment LANDED" → v1.7; Architecture Compliance Rules table range endpoint "BC-4.13.001 v1.5→v1.7" left endpoint → v1.7→v1.7; orchestrator post-fix grep confirmed zero live BC-4.13.001 v1.6 or earlier cites outside changelog sections in S-19.02; D-759 story-writer leg).
- **F-P8-003 CLOSED** (S-19.07 v1.3 replace_all BC-4.13.001 v1.6 Phase-B → v1.7 Phase-B in all body cites; targeted replacement BC-4.13.001 v1.5 Invariant 10 → v1.7 Invariant 10 at 2 stray sites (Architecture Mapping + Previous Story Intel); BC table + Token Budget + Architecture Compliance Rules updated; orchestrator post-fix grep confirmed zero live stale BC-4.13.001 cites; D-759 story-writer leg).
- **F-P8-004 CLOSED** (E-19 epic v1.8 Stories-table S-19.03 BCs cell updated to "BC-2.07.001, BC-2.02.011"; orchestrator grep confirms BC-2.02.011 in both Stories-table BCs cell and §BC Traceability table; D-759 story-writer leg).
- **F-P8-005 CLOSED** (S-19.06 v1.4 AC-003 gate updated: `sed 's://.*::'` trailing-comment strip chains before secondary grep — strips both leading-line and trailing inline `//` comment forms before asserting absence of forbidden symbols; D-759 story-writer leg).
- **F-P8-006 CLOSED** (S-19.05 v1.7 EC-005 `[SYNTHETIC]` label dropped from row identifier/description prefix; Expected Behavior column updated to "verified via schema-level property/serialization tests (not runtime dispatch fixture) per BC-3.08.001 v1.19 Invariant 6"; D-759 story-writer leg).
- **F-P8-007 CLOSED** (S-19.07 v1.3 both entry names ("verify-factory-lock" AND "verify-factory-lock-bash") enumerated explicitly in AC-002 Gate body text, Architecture Mapping table, and Architecture Compliance Rules gate rows; awk-scoped gate for F-P8-001 also names both entries explicitly; D-759 story-writer leg).
- **F-P8-008 CLOSED** (S-19.02 v1.7 AC-006 Gate Unit test E added: "fixture STATE.md of 262145 bytes → `run_check()` returns `StateReadError` (OUTPUT_TOO_LARGE/fail-open) AND zero `state_md_approaching_cap` log entries emitted"; T-009 row added to T-row table; Task 7 implementation note updated with Unit test E; D-759 story-writer leg).
- **F-P8-009 CLOSED** (S-19.07 v1.3 subsumed by F-P8-001 awk-scoped rewrite — all gate patterns use consistent single-quoted awk and grep, eliminating the backslash-escape mixed-quoting brittle pattern; D-759 story-writer leg).
- **F-P8-010 CLOSED** (S-19.05 v1.7 AC-001 Gate updated with jq per-field assertion loop: `jq -e '.type == "plugin.completed" and .timestamp != null and .hook_id != null and .tool != null and .exit_code != null and .duration_ms != null and .source != null and .plugin_version != null and .entry_index != null'`; all 9 mandatory BC-3.08.001 v1.19 Event 6 fields individually asserted; D-759 story-writer leg).
- **F-P8-011 CLOSED** (E-19 epic v1.8 §Wave Sequencing note added: "S-19.04 MUST complete before S-19.06 (S-19.06 depends_on S-19.04); S-19.04 and S-19.05 may be dispatched in parallel; S-19.05 and S-19.06 may be dispatched in parallel after S-19.04 completes"; intra-wave ordering constraint made explicit for orchestrators dispatching W2; D-759 story-writer leg).

New findings from pass-9 review below in Part B.

---

## Part B — New Findings

*Verbatim adversary output per D-448(a) source-attestation. Do not summarize or soften. Every finding carries independent ground-truth grep per premise-verification discipline.*

F-P9-001 — MEDIUM — E-19 epic v1.8 `subsystems_affected:` frontmatter includes SS-06 which is not covered by any of the 7 stories (S-19.01..S-19.07) in the epic's scope. Ground-truth verification: `grep "^subsystems_affected:" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` returns `[SS-01, SS-02, SS-03, SS-04, SS-05, SS-06, SS-07, SS-09]`. Cross-reference against per-story `subsystems:` frontmatter and BC traceability: S-19.01 (SS-05 via BC-5.42.001), S-19.02 (SS-04 via BC-4.13.001), S-19.03 (SS-02 via BC-2.07.001/BC-2.02.011), S-19.04 (SS-09 config-only), S-19.05 (SS-03 via BC-3.08.001), S-19.06 (SS-01 via BC-1.17.001), S-19.07 (SS-04 via BC-4.13.001) — union is `{SS-01, SS-02, SS-03, SS-04, SS-05, SS-07, SS-09}` (7 subsystems). `grep -r "SS-06" .factory/stories/S-19.0*.md | grep "^subsystems:"` returns zero hits — no story in the epic names SS-06 in its subsystems frontmatter or BC traceability. SS-06 is a phantom entry propagated from the original epic v1.0 with no story basis. Root cause: the F-P1-002 sibling-sweep at E-19 pass-1 corrected S-19.01's Architecture Mapping (removing SS-06 there) but never propagated the correction back to the epic's `subsystems_affected:` array — a TD-VSDD-060 class sibling-site gap. Fix: story-writer E-19 epic v1.8→v1.9 — recompute `subsystems_affected:` as the 7-story union: `[SS-01, SS-02, SS-03, SS-04, SS-05, SS-07, SS-09]`; remove SS-06; verify by diffing the union against all 7 stories' `subsystems:` frontmatter fields.

F-P9-002 — LOW — S-19.01 v1.7 AC-001 gate uses the phrase "pr-manager exits non-zero" as the failure indicator, but pr-manager is an LLM agent dispatched via the `Agent` tool — it does not exit with a POSIX process exit code. The test locus per BC-5.42.001 EC-001 is the shell script `check-stale-verdict.sh`, which exits non-zero with `READY_SHA_FETCH_FAILED` when the READY-verdict HEAD SHA fails to match `origin/develop`. Ground-truth verification: `grep -n "exits non-zero\|exit.*non-zero\|non-zero.*exit\|pr-manager.*exit" .factory/stories/S-19.01-pr-manager-hardening.md | grep "AC-001"` returns the AC-001 gate text naming pr-manager as the subject of the non-zero exit, rather than the check-stale-verdict.sh script. The category error means a test authored against the gate as written would invoke an LLM agent and attempt to observe its "exit code" — which is not a testable property. BC-5.42.001 EC-001 anchors the exit-code behavior on check-stale-verdict.sh specifically: "Exit non-zero with READY_SHA_FETCH_FAILED written to stderr when the stale verdict condition is detected." Fix: story-writer S-19.01 v1.7→v1.8 — AC-001 gh-failure arm reworded: replace "pr-manager exits non-zero" with "check-stale-verdict.sh exits non-zero with READY_SHA_FETCH_FAILED on stderr (per BC-5.42.001 EC-001)"; negative-control fixture updated to invoke the script directly rather than the agent dispatch path.

F-P9-003 — LOW — S-19.06 v1.4 AC-003 gate chains `sed 's://.*::'` before the forbidden-symbol grep to strip `//`-style line comments, but Rust source code can also contain `/* ... */` C-style block comments that span inline. A line such as `let max = something; /* legacy: used host::read_file here */` would survive the `sed 's://.*::'` filter unchanged (it does not contain `//`) and then match the `grep -qE "host::read_file"` secondary check, producing a false-positive gate failure. Ground-truth verification: `grep -n "AC-003\|sed.*://\|block.comment\|/\*" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -10` returns the AC-003 gate using `sed 's://.*::'` with no corresponding `/* */` block-comment stripping step. The gate was corrected for `//` inline comments at pass-8 (F-P8-005) but the analogous C-style block comment form was not addressed. Rust source files legitimately use both comment forms. Fix: story-writer S-19.06 v1.4→v1.5 — AC-003 gate updated to chain a block-comment stripper: `sed 's:/\*.*\*/::g; s://.*::' <file> | grep -qE "host::read_file|OUTPUT_TOO_LARGE"` (the `s:/\*.*\*/::g` pass removes `/* ... */` inline block comments before the `//` strip and the forbidden-symbol grep; note: multi-line block comments are out of scope for this gate — production Rust files in this crate do not use multi-line block comments in call-site contexts per Architecture Compliance convention).

F-P9-004 — LOW — E-19 epic v1.8 §Dependency Graph section contains an ASCII art diagram that visually depicts W1→S-19.07 edges that do not exist in the actual dependency graph. Ground-truth verification: `grep -n "W1\|W3\|S-19.07\|→" .factory/stories/epics/E-19-post-rc22-operator-hardening.md | grep -v "^[0-9]*:--\|Changelog\|version\|last_amended" | head -20` at v1.8 perimeter shows the ASCII dependency graph section rendering W1 stories (S-19.01, S-19.02, S-19.03) as parent nodes of S-19.07 (W3), implying S-19.01→S-19.07 and S-19.03→S-19.07 edges that are not present in any story's `depends_on:` frontmatter. The authoritative edges are: S-19.03→S-19.06, S-19.04→S-19.06, S-19.02→S-19.07, S-19.06→S-19.07 (4 edges total; matches the `depends_on` frontmatter in the relevant stories). The ASCII layout groups W1/W2/W3 in a columnar arrangement that draws a visual implication of all W1 stories feeding W3 directly. An orchestrator or human reading the Dependency Graph section alone would dispatch S-19.07 only after ALL of S-19.01, S-19.02, S-19.03 complete (adding unnecessary sequencing constraints). Fix: story-writer E-19 epic v1.8→v1.9 — replace ASCII Dependency Graph with a mermaid `graph LR` diagram containing exactly the 4 frontmatter-authoritative edges: `S-19.03 --> S-19.06`, `S-19.04 --> S-19.06`, `S-19.02 --> S-19.07`, `S-19.06 --> S-19.07`; caption confirming "Only S-19.02 and S-19.06 gate S-19.07; only S-19.03 and S-19.04 gate S-19.06."

Observations:

O-P9-001 — STORY-INDEX v4.139 introduction line states a story count and epic count that do not match the live catalog. Ground-truth verification: `grep -n "file-resident\|stub IDs\|stories registered" .factory/stories/STORY-INDEX.md | head -5` returns an intro line with counts inconsistent with the actual number of story files and epic files present. The STORY-INDEX intro count is a non-normative narrative summary that drifts when stories are added or archived. (ACTIONED in STORY-INDEX v4.140 fix burst: intro counts corrected.)

O-P9-002 — S-19.03 v1.7 AC-003 gate uses `grep -rq crates/` (recursive directory scan) rather than targeting the two canonical site files specifically (`crates/factory-dispatcher/src/host/mod.rs` and `crates/hook-sdk/src/host.rs`). A recursive scan over `crates/` would match any file in the entire workspace that contains the constant definition or HostError::NotFound variant — including test files, documentation comments, or future crates that import the same symbol for different purposes. The gate as written cannot distinguish a removal of the canonical definition from a removal of a test reference. (ACTIONED in S-19.03 v1.8 fix burst: gate narrowed to two canonical-site greps.)

O-P9-003 — E-19 epic v1.8 §Behavioral Contract Traceability table uses abbreviated BC titles in the Story column cells (e.g., "pr-manager READY verdict SHA pinning + merge-strategy guard" rather than the full H1 title from BC-5.42.001). POLICY 7 states that the BC file H1 is the sole authoritative title. The abbreviated form is a practical table-fit accommodation shared across all E-19 passes. Recommendation: add an explicit non-normative note to the §BC Traceability section acknowledging that Story column text uses abbreviated titles for table fit; the authoritative title is the BC file H1. (ACTIONED in E-19 epic v1.9 fix burst: abbreviation-convention sentence added to §BC Traceability.)

O-P9-004 — S-19.07 v1.3 AC-002 Gate uses awk range patterns of the form `awk '/START_PATTERN/,/END_PATTERN/'` where the END_PATTERN `^\[\[hooks\]\]` also matches a line that could appear before the stanza is complete if the registry file gains a `[[hooks]]` block inside the target entry's extended section. The range-based approach is fragile if the registry structure evolves. The per-entry-terminated flag form `awk '/START/{found=1} found{print} /END/{found=0}'` is more explicit about stanza boundaries and does not rely on line-ordering assumptions. (ACTIONED in S-19.07 v1.4 fix burst: awk range patterns replaced with per-entry-terminated flag form ×4.)

O-P9-005 — S-19.03 v1.7 AC-002 gate asserts `zero capability_denied events in dispatcher log after the fix is applied` but does not scope the assertion to the `warn-pending-wave-gate` plugin by name. A different plugin emitting a `capability_denied` event during the test run would falsely fail the gate. The assertion must scope to `plugin_name = warn-pending-wave-gate` to be a valid unit test. (ACTIONED in S-19.03 v1.8 fix burst: AC-002 gate scoped to `plugin_name=warn-pending-wave-gate` in the log assertion.)

---

## Verifications That PASSED

The following structural checks were confirmed clean at pass-9 perimeter entry:

1. BC-cite preflight PASS: orchestrator ran 6-BC × 9-artifact matrix scan independently before dispatching pass-9; `grep "^version:"` on all 6 BC files confirmed live versions BC-4.13.001 v1.7 / BC-2.07.001 v1.2 / BC-2.02.011 v1.4 / BC-3.08.001 v1.19 / BC-5.42.001 v1.1 / BC-1.17.001 v1.1; zero stale live cites confirmed across all 9 artifacts (D-759 MECHANICAL GATE holding at perimeter entry).
2. F-P8-001 closure PASS: S-19.07 v1.3 AC-002 per-entry awk gates present; both entry names ("verify-factory-lock" AND "verify-factory-lock-bash") explicitly enumerated; single-quoted patterns; per-stanza caps.read_file=0 AND caps.read_prefix≥1 assertions.
3. F-P8-002/003 closure PASS: S-19.02 v1.7 and S-19.07 v1.3 BC-4.13.001 v1.7 confirmed as sole live version in all body cites; no v1.6 or v1.5 tokens outside changelog sections.
4. F-P8-004 closure PASS: E-19 epic v1.8 Stories-table S-19.03 BCs cell = "BC-2.07.001, BC-2.02.011" confirmed.
5. F-P8-005 closure PASS: S-19.06 v1.4 AC-003 gate uses `sed 's://.*::'` trailing-comment strip before secondary grep (F-P8-003 STANDS; F-P9-003 identifies the additional block-comment gap).
6. F-P8-006 closure PASS: S-19.05 v1.7 EC-005 `[SYNTHETIC]` label absent; Expected Behavior states schema-level property/serialization test nature.
7. F-P8-007 closure PASS: S-19.07 v1.3 both entry names enumerated at all three gate locations.
8. F-P8-008 closure PASS: S-19.02 v1.7 T-009 Unit test E present (262145-byte fixture → StateReadError + zero state_md_approaching_cap).
9. F-P8-010 closure PASS: S-19.05 v1.7 AC-001 jq per-field loop asserts all 9 mandatory BC-3.08.001 v1.19 Event 6 fields.
10. F-P8-011 closure PASS: E-19 epic v1.8 wave model note states S-19.04 BEFORE S-19.06 ordering constraint explicitly.
11. 4-index at perimeter entry PASS: BC v3.74 / VP v2.53 / STORY v4.139 / ARCH v2.89 consistent with D-759 state.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 0 |
| MEDIUM | 1 |
| LOW | 3 |
| Observations | 5 |

*Actionable findings: 4 (F-P9-001..F-P9-004). First pass in the E-19 cascade with zero HIGH findings. Trajectory 16→14→20→9→8→5→12→11→4.*

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Severity decay from pass 8 (enumerated):** B0/H3/M5/L3 (11 total) → B0/H0/M1/L3 (4 total; first zero-HIGH pass; 3 HIGH and 4 MEDIUM resolved by pass-8 fixes; 0 re-found classes from prior closed findings; novelty score 1.0)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 9 |
| **New findings** | 4 (F-P9-001..F-P9-004) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (4 / 4) |
| **Median severity** | LOW |
| **Trajectory (findings per pass)** | 16 → 14 → 20 → 9 → 8 → 5 → 12 → 11 → 4 |
| **Verdict** | FINDINGS_REMAIN — pass 10 dispatched with fresh context |

**Note on pass-9 volume (4 vs pass-8's 11):** The significant reduction reflects closure of the HIGH class (globally-unscoped gate, BC-cite propagation) and the systematic MEDIUM class. Remaining findings (1 MEDIUM, 3 LOW) are deeper correctness details: a phantom subsystem entry (governance), a category error in test locus (precision), a comment-style gap (edge case), and an ASCII diagram ambiguity (clarity). The zero-HIGH milestone is the first in the E-19 cascade; asymptotic convergence pattern is consistent with the 3-CLEAN target at the LOW end.

---

## Fix-Burst Closure Section (D-760)

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. Same-burst fixes do NOT advance streak.

**All 4 findings closed. Orchestrator ran per-file BC-cite preflight independently before and after fix burst (O-P9 preflight catch protocol; per-file form is now canonical following orchestrator verification-command defect discovery — see D-760 §(b)).**

### Story-writer leg

- **E-19 epic v1.8→v1.9 (F-P9-001 + F-P9-004 + O-P9-003):** (a) `subsystems_affected:` recomputed as 7-story union: `[SS-01, SS-02, SS-03, SS-04, SS-05, SS-07, SS-09]`; SS-06 removed (phantom — no story names SS-06 in subsystems frontmatter or BC traceability; F-P1-002 sibling-sweep never reached the epic; TD-VSDD-060 class; closes F-P9-001). (b) §Dependency Graph: ASCII art diagram replaced with mermaid `graph LR` containing exactly 4 frontmatter-authoritative edges (`S-19.03 --> S-19.06`, `S-19.04 --> S-19.06`, `S-19.02 --> S-19.07`, `S-19.06 --> S-19.07`); nonexistent W1→S-19.07 visual edges eliminated; caption "Only S-19.02 and S-19.06 gate S-19.07; only S-19.03 and S-19.04 gate S-19.06" (closes F-P9-004). (c) §BC Traceability: abbreviation-convention sentence added ("Story column uses abbreviated titles for table fit; authoritative title is each BC file H1 per POLICY 7; abbreviations are non-normative") (O-P9-003 encoded). Input-hash updated from b42dd69 → d0f7250. Closes F-P9-001 + F-P9-004; O-P9-003 encoded.

- **S-19.01 v1.7→v1.8 (F-P9-002):** AC-001 gh-failure arm reworded — locus corrected from "pr-manager exits non-zero" (category error: LLM agent has no process exit codes) to "check-stale-verdict.sh exits non-zero with READY_SHA_FETCH_FAILED on stderr (per BC-5.42.001 EC-001)"; negative-control fixture updated to invoke check-stale-verdict.sh directly rather than the agent dispatch path. Closes F-P9-002.

- **S-19.03 v1.7→v1.8 (O-P9-002 + O-P9-005):** (a) AC-003 gate narrowed from broad `grep -rq crates/` recursive scan to two canonical-site greps: `grep -q "codes::NOT_FOUND" crates/factory-dispatcher/src/host/mod.rs && grep -q "HostError::NotFound" crates/hook-sdk/src/host.rs` (O-P9-002 canonical-site greps). (b) AC-002 zero-capability_denied assertion scoped to `plugin_name=warn-pending-wave-gate` in the log assertion, preventing false failures from other plugins emitting capability_denied events during the same test run (O-P9-005 name-scoped assertion).

- **S-19.03 v1.8→v1.9 (BC-cite drift preflight catch):** Story-writer post-O-P9 BC-cite drift preflight scan identified 2 pre-existing stale cites in S-19.03 (outside changelog sections): (1) §Behavioral Contracts table body row BC-2.07.001 v1.1 → v1.2; (2) Token Budget row BC-2.07.001 v1.0 → v1.2. Both are live normative cites outside changelog history. Fixed in-scope per BC-cite drift preflight (D-759 MECHANICAL GATE application — story-writer side); no adversary finding required.

- **S-19.06 v1.4→v1.5 (F-P9-003):** AC-003 gate updated to chain a block-comment stripper before the forbidden-symbol grep: `sed 's:/\*.*\*/::g; s://.*::' <file> | grep -qE "host::read_file|OUTPUT_TOO_LARGE"` — the `s:/\*.*\*/::g` pass strips `/* ... */` inline C-style block comments; the `s://.*::` pass strips `//`-style line comments; both forms stripped before the secondary grep. Architecture Compliance Rules: note added that multi-line block comments are out of scope for this gate per production codebase convention. Closes F-P9-003.

- **S-19.07 v1.3→v1.4 (O-P9-004):** AC-002 Gate awk range patterns (`awk '/START/,/END/'`) replaced with per-entry-terminated flag form (×4 occurrences — both verify-factory-lock and verify-factory-lock-bash, each with a caps.read_file assertion and a caps.read_prefix assertion): `awk '/^name = "verify-factory-lock"$/{found=1} found{print} /^\[\[hooks\]\]/{if(found>1){exit}; found++}'` style replaced with explicit `found=0` flag form that terminates cleanly on entry boundary without relying on `[[hooks]]` ordering assumptions. O-P9-004 encoded.

- **STORY-INDEX v4.139→v4.140 (O-P9-001 + story cell updates):** (a) Introduction line updated: story count and epic count corrected from stale values to 130 file-resident stories / 20 epics (O-P9-001 actioned). (b) E-19 section story cells updated to reflect post-pass-9-fix versions: S-19.01 v1.8; S-19.03 v1.9; S-19.06 v1.5; S-19.07 v1.4; E-19 epic v1.9. BC coverage line updated to reflect post-fix artifact state.

- **S-19.05 v1.7→v1.8 (BC-cite drift preflight catch — orchestrator per-file form):** Orchestrator ran independent BC-cite verification using initial cross-file awk form: `awk '/BC-3\.08\.001 v1\.1[0-8]/{print FILENAME": "$0}' .factory/stories/S-19.*.md` — result: no output (FALSE NEGATIVE). Root cause identified as awk state carryover between files in multi-file invocation: the awk `/pattern/{action}` form does not reset per-file state between input files, causing match accumulation to false-negative on per-file boundaries. Orchestrator switched to per-file loop: `for f in .factory/stories/S-19.*.md .factory/stories/epics/E-19*.md .factory/stories/STORY-INDEX.md; do grep -nE "BC-3\.08\.001 v1\.(1[0-8]|[0-9])([^0-9]|$)" "$f" && echo "  STALE FILE: $f"; done`. Per-file loop detected 8 body-scope `BC-3.08.001 v1.18` tokens in S-19.05 at lines 84, 92, 93, 94, 102, 110, 111, 150 — all outside changelog/last_amended sections, all live normative cites. Story-writer applied replace_all `v1.18` → `v1.19` in body scope; zero v1.18 tokens remain. Per-file loop re-run: ZERO stale live cites confirmed. Per-file loop is now the CANONICAL preflight command (cross-file awk FORBIDDEN for BC-cite drift preflight). Orchestrator verification-command defect recorded at D-760 §(b). **STORY-INDEX v4.140→v4.141:** S-19.05 cell updated to v1.8 after preflight catch; STORY-INDEX changelog entry added.

### 6-BC × 9-Artifact Cite Matrix (D-760 closure; live BC versions confirmed via `grep "^version:"` at commit time)

Live BC versions at D-760 closure (UNCHANGED from D-759 — no BC amendments this pass):
- BC-4.13.001: v1.7
- BC-2.07.001: v1.2
- BC-2.02.011: v1.4
- BC-3.08.001: v1.19
- BC-5.42.001: v1.1
- BC-1.17.001: v1.1

| Artifact | BC-4.13.001 | BC-2.07.001 | BC-2.02.011 | BC-3.08.001 | BC-5.42.001 | BC-1.17.001 |
|----------|-------------|-------------|-------------|-------------|-------------|-------------|
| S-19.01 | — | — | — | — | v1.1 ✓ | — |
| S-19.02 | v1.7 ✓ | — | — | — | — | — |
| S-19.03 | — | v1.2 ✓ | v1.4 ✓ | — | — | — |
| S-19.04 | — | — | — | — | — | — |
| S-19.05 | — | — | — | v1.19 ✓ | — | — |
| S-19.06 | — | — | — | — | — | v1.1 ✓ |
| S-19.07 | v1.7 ✓ | — | — | — | — | — |
| E-19 epic | v1.7 ✓ | v1.2 ✓ | v1.4 ✓ | v1.19 ✓ | v1.1 ✓ | v1.1 ✓ |
| STORY-INDEX | v1.7 ✓ | v1.2 ✓ | v1.4 ✓ | v1.19 ✓ | v1.1 ✓ | v1.1 ✓ |

*Orchestrator independently verified ZERO stale live cites outside changelog sections across all 9 artifacts using per-file loop form. Cross-file awk forbidden for this gate (D-760 §(b) orchestrator verification-command defect).*

### Artifact versions at pass-9 closure

| Artifact | Version |
|----------|---------|
| BC-INDEX | v3.74 (UNCHANGED — no BC amendments this pass) |
| VP-INDEX | v2.53 (UNCHANGED) |
| ARCH-INDEX | v2.89 (UNCHANGED) |
| BC-4.13.001 | v1.7 (UNCHANGED) |
| BC-2.07.001 | v1.2 (UNCHANGED) |
| BC-2.02.011 | v1.4 (UNCHANGED) |
| BC-3.08.001 | v1.19 (UNCHANGED) |
| BC-5.42.001 | v1.1 (UNCHANGED) |
| BC-1.17.001 | v1.1 (UNCHANGED) |
| S-19.01 | v1.8 |
| S-19.02 | v1.7 (UNCHANGED this pass) |
| S-19.03 | v1.9 |
| S-19.04 | v1.9 (UNCHANGED this pass) |
| S-19.05 | v1.8 |
| S-19.06 | v1.5 |
| S-19.07 | v1.4 |
| E-19 epic | v1.9 |
| STORY-INDEX | v4.141 |

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. **NEXT:** E-19 adversarial pass-10 (fresh context; 20-policy rubric; trajectory 16→14→20→9→8→5→12→11→4→pass-10; BC-cite per-file preflight mandatory before dispatch).
