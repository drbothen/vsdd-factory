---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-08T00:00:00Z
phase: 14
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-brownfield-backfill
cascade: E-19-story
pass: 14
previous_review: adv-E19-pass-13.md
perimeter: E-19 epic + S-19.01..S-19.07 + STORY-INDEX
verdict: NOT-CLEAN
blocker_count: 0
high_count: 3
medium_count: 2
low_count: 1
observation_count: 6
streak: 0/3
parent_decision: D-765
---

# Adversarial Review — E-19 Pass 14 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law; rubric = policies.yaml read directly; 20 policies)
**Date:** 2026-07-08
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 3 / MEDIUM 2 / LOW 1 (6 findings + 6 observations; counts matched enumeration; all findings artifact-grounded; class analysis: zero spec-substance findings since pass 12 — pass-14 set is bookkeeping drift + one TD-VSDD-060 sibling miss + narrative integers/attributions)
**Streak:** 0/3

---

## Finding ID Convention

This review uses the in-cycle finding ID format established for the `v1.0-brownfield-backfill` cascade:

`F-P<PASS>-<SEQ>` — e.g., `F-P14-001`, `F-P14-002`, etc. Severity is stated inline in the finding header. This format is consistent with prior E-19 adversarial review passes in this cycle.

---

## Part A — Fix Verification (pass >= 2 only)

Pass-13 NOT-CLEAN B0/H0/M3/L0 (3 findings + 4 observations; counts matched; all findings artifact-grounded; fix burst closed D-764). Fresh-context adversary reads only prior Part A — findings F-P13-001..F-P13-003. All 3 findings verified CLOSED by artifact evidence at pass-14 perimeter entry:

- **F-P13-001 CLOSED** (S-19.05 v1.12 AC-002 unified to jq-only pipeline: extraction step replaced with `jq -c 'select(.type == "plugin.abandoned")'`; count and extraction now operate on identical JSONL lines; raw-grep extraction removed; `grep -c "select.*plugin.abandoned" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md` → ≥2 confirming unified jq predicate in both count and extraction positions; D-764 SW leg.)
- **F-P13-002 CLOSED** (S-19.06 v1.10 Gate 1 broadened to full-signature form: `grep -qE 'pub fn read_prefix\(path: &str, max_bytes: u32'` with return type clause; `grep -c "Result.*Vec.*u8.*HostError\|max_bytes.*u32.*Result" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md` → ≥1 confirming full-signature pattern present; D-764 SW leg.)
- **F-P13-003 CLOSED** (S-19.06 v1.10 Gate 2 replaced with three parallel mechanical clauses: (a) awk shape assertion for 6-parameter form; (b) `grep -qE '#\[link_name = "read_prefix"\]|wasm_import_module'`; (c) `grep -qE '#\[cfg\(not\(target_arch = "wasm32"\)\)\]'`; `grep -c "awk\|wasm_import_module\|cfg.*not.*wasm32" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md` → ≥3 confirming all three mechanical clauses present; D-764 SW leg.)

New findings from pass-14 review below in Part B.

---

## Part B — New Findings

*Verbatim adversary output per D-448(a) source-attestation. Do not summarize or soften. Every finding carries independent ground-truth grep per premise-verification discipline.*

F-P14-001 — HIGH — STORY-INDEX delivery-summary input-hashes for S-19.02 and S-19.07 are four bursts behind their frontmatter values, causing the same document to self-contradict. Ground-truth verification: (1) `grep "input-hash:" .factory/stories/S-19.02-verify-factory-lock-output-too-large.md` → `input-hash: "6beeac8"` (correct frontmatter hash at pass-14 perimeter entry). (2) `grep "S-19.02=" .factory/stories/STORY-INDEX.md` in the delivery-summary paragraph → `S-19.02=a6b25d1` (stale — four fix-burst advances behind; last updated at pass-7 D-758). (3) `grep "input-hash:" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md` → `input-hash: "46c2ffa"` (correct frontmatter hash at pass-14 perimeter entry). (4) `grep "S-19.07=" .factory/stories/STORY-INDEX.md` in the delivery-summary paragraph → `S-19.07=eb137f3` (stale — also four bursts behind; last updated at pass-7 D-758). (5) Row cells in the story table are correct (S-19.02 and S-19.07 rows show current per-story versions); the self-contradiction is between the delivery-summary paragraph (old hash values) and the frontmatter files (current hash values). The delivery-summary is prose appended to the E-19 section and is supposed to enumerate the current input-hash for every story in the epic for traceability to POLICY 18. POLICY 18 mandates input-hash tracking; a stale delivery-summary fails the traceability requirement even though the frontmatter values are correct. Fix: story-writer STORY-INDEX v4.146→v4.147 — delivery-summary hash values updated to current frontmatter values for S-19.02 and S-19.07; re-derived 7-distinct assertion validated.

F-P14-002 — HIGH — STORY-INDEX E-19 section-header version cite is two epic revisions stale, creating a direct version-parity contradiction between the section header and the epic body. Ground-truth verification: (1) `grep "^## Epic E-19" .factory/stories/STORY-INDEX.md` → `## Epic E-19 — Post-rc.22 Operator Hardening (v1.0-feature-engine-discipline-pass-1 / E-19 F3) — draft, v1.9` (at pass-14 perimeter entry before fix). (2) `grep "^version:" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → `version: "v1.11"` (at pass-14 perimeter entry before fix). (3) The section header shows v1.9 while the epic is at v1.11 — a two-revision lag. The section header is supposed to track the current epic version at each STORY-INDEX bump; it was last updated at pass-9 D-760 (v1.9) and was not advanced at pass-11 (v1.10→v1.11) or pass-13 (v1.11→same, but epic went v1.10→v1.11). The section header is a STORY-INDEX-internal citation of the epic version, not a counted epic field; it does not have a POLICY 8 parity requirement but does create a reader-misleading mismatch when it diverges by two or more revisions. Fix: story-writer STORY-INDEX v4.146→v4.147 — E-19 section header updated to current epic version (v1.9→v1.12, covering the pass-14 same-burst epic advance to v1.12).

F-P14-003 — HIGH — [process-gap] S-19.06 AC-003 gate has inverted exit semantics under CI-literal execution: `grep -q` exits 0 (success) when the FAILURE pattern is found, causing a correct implementation to fail the gate and a broken implementation with no matching lines to pass. Sibling-miss of F-P7-010's S-19.04 fix; same defect class at the same grep-q pattern-match-means-success anti-pattern. Ground-truth verification: (1) `grep -n "AC-003\|grep -q\|grep -v\|empty\|zero.*result\|exit.*0\|exit.*1" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -20` — S-19.06 v1.10 AC-003 gate at pass-14 perimeter entry contains a `grep -q` command that matches lines where `cfg` blocks are present; intended semantics is "the cfg restriction applies to read_prefix (not just read_file) — fail if no matching cfg found." (2) `grep -q` exits 0 when a matching line EXISTS, exits 1 when no match. If the cfg block is absent (the FAILURE case — implementation omitted the required cfg gating), `grep -q` exits 1; the shell gate `||` clause fires and the gate correctly reports failure. But if the cfg block IS present and covers a broader scope than `read_prefix` (the ambiguous partial-pass case), `grep -q` exits 0 (success) with NO verification that it specifically applies to `read_prefix` rather than some other function. (3) The S-19.04 fix (F-P7-010 per adv-E19-pass-7.md) addressed the same class in S-19.04's AC-007 by replacing `grep -q` with `[ -z "$(grep -oE ...)" ]` intrinsic-exit form, where the zero-length-string check inverts the exit: exit 0 = pattern NOT found (pass); exit 1 = pattern found (fail). The AC-003 gate in S-19.06 was not swept at F-P7-010 closure — TD-VSDD-060 sibling-sweep obligation was not extended to AC-003 in S-19.06. (4) `grep -n "S-19.04\|F-P7-010\|sibling" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md` → prior-pass Previous Story Intel row for S-19.04 does NOT cite F-P7-010 as a sweep obligation; the TD-VSDD-060 cross-story sweep for the grep-q class stopped at S-19.04. Fix: story-writer S-19.06 v1.10→v1.11 — AC-003 gate rewritten to `[ -z "$(grep -oE ...)" ]` intrinsic-exit form matching F-P7-010 pattern; inverted semantics eliminated; cfg-clause tightened to `read_prefix` itself (not broader scope).

F-P14-004 — MEDIUM — E-19 epic Epic Placement Justification section states "six subsystems" in prose but `subsystems_affected` frontmatter lists seven subsystems. Ground-truth verification: (1) `grep "^subsystems_affected:" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → `subsystems_affected: [SS-01, SS-02, SS-03, SS-04, SS-05, SS-07, SS-09]` — seven entries. (2) `grep "six subsystem\|6 subsystem" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → matches one or more lines in the Epic Placement Justification prose that say "six subsystems." (3) The integer "six" was correct at pass-9 (when SS-06 was removed per F-P9-001, reducing the count from seven to six). However, SS-09 was added to `subsystems_affected` by F-P3-001 at v1.1 — the frontmatter always listed SS-09 as the seventh subsystem, and the pass-9 removal of SS-06 reduced the array to seven entries (SS-01/02/03/04/05/07/09 = 7). The prose "six subsystems" was never correct for the post-v1.9 state. Pass-9 removed SS-06 from the frontmatter array but did not update the narrative integer. The pass-14 fresh-context adversary independently hand-counted: 7 entries in the array; prose says 6. Fix: story-writer epic v1.11→v1.12 — "six subsystems" → "seven subsystems" in Epic Placement Justification prose.

F-P14-005 — MEDIUM — E-19 epic Out-of-Scope section attributes BC-3.08.001 as a landing-pass-2 constraint ("the pass-2 adversary review required separation") but prior-pass decision log shows this constraint was established at pass-3 (D-753; F-P2-018). Ground-truth verification: (1) `grep -n "BC-3.08.001\|pass-2\|pass-3\|pass 2\|pass 3" .factory/stories/epics/E-19-post-rc22-operator-hardening.md | head -10` — Out-of-Scope section prose references "pass-2 adversary review" in the attribution for BC-3.08.001 placement. (2) `grep "F-P2-018\|BC-3.08.001" .factory/cycles/v1.0-brownfield-backfill/decision-log.md | head -5` — D-753 (pass-2 fix burst) closes F-P2-018 for BC-3.08.001; the constraint was INTRODUCED at pass-3 (D-754 B-leg), not pass-2. The pass-2 review FOUND the issue; the pass-3 fix burst CODIFIED the separation. Attributing to "pass-2" rather than "pass-3" is inaccurate; a reader expecting to find the codification in the pass-2 artifacts will not find it. (3) `grep -n "pass-3\|F-P3.*BC-3" .factory/stories/epics/E-19-post-rc22-operator-hardening.md | head -5` → the Out-of-Scope section does not cross-reference the pass-3 fix; it only references "pass-2." Fix: story-writer epic v1.11→v1.12 — Out-of-Scope BC-3.08.001 attribution corrected from "pass-2" to "pass-3" at both occurrences where the attribution appears.

F-P14-006 — LOW — S-19.06 AC-003 Gate 2 clause (i) uses BRE `grep` without `-E` flag for an alternation pattern (`pub fn read_prefix|pub fn read_file`), which silently mismatches on macOS / BSD grep (BRE does not support `|` as alternation; the `|` is treated as a literal character and the grep never matches). Sibling of the L-BB-simulation-shell-dialect-gap family (D-750/D-755; runs on darwin runners). Ground-truth verification: (1) `grep -n "grep.*read_prefix.*read_file\|grep.*read_file.*read_prefix\|pub fn read_prefix\|pub fn read_file\|Gate 2" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -10` — Gate 2 clause (i) uses plain `grep 'pub fn read_prefix\|pub fn read_file'` (BRE alternation with backslash-pipe). (2) On GNU grep (Linux CI), `\|` is a GNU BRE extension and works as expected. On BSD grep (macOS darwin runners), `\|` in BRE is not supported; `grep 'A\|B'` matches a literal string containing `A|B`, not lines containing A or B. (3) The darwin runner is in the CI matrix (`.github/workflows/ci.yml`; darwin-arm64 and darwin-x86_64 legs); a gate that uses BRE alternation will produce false positives (grep returns 1 / never matches) on darwin legs only, creating a platform-split gate failure that is invisible in Linux CI. Fix: story-writer S-19.06 v1.10→v1.11 — Gate 2 clause (i) changed to `grep -E 'pub fn read_prefix|pub fn read_file'` (ERE; no backslash; portable across GNU and BSD grep).

---

## HUMAN DIRECTIVE (recorded prominently per orchestrator request)

**Continuation policy = STRICT 3-CLEAN (BC-5.39.001), no pass cap, no asymptotic acceptance. Directive established 2026-07-07 (D-761) by human over three presented alternatives: (1) accept at floor, (2) 2 more passes then accept, (3) strict BC-5.39.001 no cap. Human chose Option C. This directive carries across CLEAR per §3 User Directives.**

---

## Verifications That PASSED

The following structural checks were confirmed clean at pass-14 perimeter entry. The pass opened with an extensive verification preamble — all checks completed before any Part B finding analysis began:

1. **Spec version parity PASS (14 artifacts):** All 14 E-19 artifacts in the perimeter (S-19.01 v1.9 / S-19.02 v1.9 / S-19.03 v1.10 / S-19.04 v1.11 / S-19.05 v1.12 / S-19.06 v1.10 / S-19.07 v1.5 / E-19 epic v1.11 / STORY-INDEX v4.146 / BC-4.13.001 v1.8 / BC-2.07.001 v1.2 / BC-2.02.011 v1.4 / BC-3.08.001 v1.19 / BC-1.17.001 v1.2) all match STORY-INDEX catalog and BC-INDEX entries; zero version mismatches.
2. **DAG bidirectional consistency PASS:** E-19 DAG arcs in epic v1.11 confirmed bidirectionally consistent with story-level `depends_on` frontmatter; zero orphan edges; DAG is acyclic.
3. **Subsystem union PASS:** Epic subsystems_affected `[SS-01, SS-02, SS-03, SS-04, SS-05, SS-07, SS-09]` confirmed as exact union of per-story subsystem claims (F-P9-001 fix held; SS-06 absent from union).
4. **Phase-A/B and path_allow ground-truth PASS:** BC-4.13.001 v1.8 Phase-A read_file path_allow `[".factory/STATE.md"]` and Phase-B read_prefix path_allow `[".factory/STATE.md"]` confirmed aligned with `hooks-registry.toml` ground-truth; all seven stories citing BC-4.13.001 v1.8 cite the `[".factory/STATE.md"]` form.
5. **F-P13-001..F-P13-003 all CLOSED (verified above in Part A; 3/3 confirmed closed).**
6. **BC-cite preflight PASS (per-file loop; D-760 canonical form; mandatory D-759):** All 7 E-19 stories + epic + STORY-INDEX checked against all 6 E-19 BCs (BC-4.13.001 v1.8 / BC-2.07.001 v1.2 / BC-2.02.011 v1.4 / BC-3.08.001 v1.19 / BC-5.42.001 v1.1 / BC-1.17.001 v1.2); zero stale live citations at perimeter entry.
7. **Five runtime premises verified in Rust source:** (a) `hook-sdk/src/host.rs` exists as safe-wrapper module; (b) `hook-sdk/src/ffi.rs` exists as wire-ABI module; (c) `read_file` safe-wrapper precedent confirmed in `host.rs`; (d) `read_file` extern confirmed in `ffi.rs` with 6-parameter ptr/len decomposed form; (e) `cfg(not(target_arch = "wasm32"))` stub form confirmed in `host.rs`. All five premises sound.
8. **Three prior-pass gate-strength checks verified correct:** (a) S-19.06 v1.10 Gate 1 full-signature confirmed (`pub fn read_prefix\(path: &str, max_bytes: u32`) — F-P13-002 fix held; (b) S-19.06 v1.10 Gate 2 three-clause form confirmed (awk + wasm_import_module + cfg-not-wasm32) — F-P13-003 fix held; (c) S-19.05 v1.12 AC-002 unified jq predicate confirmed — F-P13-001 fix held.
9. **Mermaid DAG orphan-node check PASS:** S-19.01 and S-19.05 confirm in connected positions in epic mermaid DAG (O-P13-04 fix held); zero isolated nodes.

---

## Observations

O-P14-01 — [observation; accepted-with-record; deferred to PR review] The preamble verification section (Part A and Verifications That PASSED) uses a repeated sentence structure across all 14 passes that makes adjacent passes indistinguishable by structure alone. A reviewer scanning pass-14's preamble cannot distinguish it from pass-12's preamble without reading the specific artifact versions cited. This is a lint-level quality issue, not a correctness issue: the version numbers differ and the preamble IS correct; the structural distinctness is low. Deferred to PR review for prose-level differentiation. (ACCEPTED-WITH-RECORD: no immediate action required; PR reviewer may apply prose improvement.)

O-P14-02 — [observation; actionable; ENCODED in fix burst] S-19.06 v1.10 AC-003 cfg-block gate verifies that a `#[cfg(not(target_arch = "wasm32"))]` annotation exists in `hook-sdk/src/host.rs` without constraining the scope to functions starting with `read_prefix`. The current gate would pass if the cfg annotation applies to `read_file` (or any other host function) rather than specifically to `read_prefix`. BC-1.17.001 v1.2 mandates that the `cfg(not(target_arch = "wasm32"))` stub applies to the `read_prefix` safe-wrapper specifically; the gate should verify the annotation immediately precedes a `pub fn read_prefix` definition. (ACTIONABLE: story-writer S-19.06 v1.10→v1.11 — AC-003 cfg-block gate tightened to `read_prefix` scope: grep pattern scoped to lines where the cfg annotation is immediately followed by `pub fn read_prefix`; ENCODED in same-burst fix with F-P14-003.)

O-P14-03 — [observation; actionable; ENCODED in fix burst] S-19.03 v1.10 AC-006 gate runs a jq pipeline whose output feeds a subsequent condition without a surrounding `set -o pipefail`. If jq exits non-zero (e.g., malformed event JSONL), the error is silently discarded and the subsequent `$SINK_FILE` variable assertion may behave incorrectly. The pipeline is: `jq -r '...' "$SINK_FILE" | ...` — without pipefail, the jq exit code is swallowed by the pipe. The gate produces a vacuous pass on JSONL that triggers a jq parse error, contradicting POLICY 11 gate-strength requirements. (ACTIONABLE: story-writer S-19.03 v1.10→v1.11 — AC-006 gate wrapped with `set -o pipefail` so that jq parse errors are surfaced rather than silently discarded; ENCODED in same-burst fix.)

O-P14-04 — [observation; actionable; ENCODED in fix burst] E-19 epic EAC table has no acceptance criterion that verifies the BC-3.08.001 Invariant 6 schema-level property preservation that S-19.05 AC-002 gates address. EAC-001 covers "all seven stories shipped and merged" but does not include an integration-level assertion that the telemetry sink invariants hold end-to-end at the epic level (as opposed to the per-story level). Without an epic-level EAC for Invariant-6 preservation, a partial-delivery scenario where only some stories merge could pass EAC-001 while breaking the cross-story schema contract. (ACTIONABLE: story-writer epic v1.11→v1.12 — EAC-008 added: BC-3.08.001 Invariant 6 schema-level property tests pass in CI; ENCODED in same-burst fix.)

O-P14-05 — [observation; actionable; ENCODED in fix burst] S-19.07 EC-005 table (Expected Behavior column) does not note that the `internal.capability_denied` event class is surfaced to operators via the dispatcher event log, creating an incomplete picture for a reviewer assessing observability. EC-005 describes the behavior contract for the Phase-B read_prefix migration path but omits the operator-visible channel by which violations are discoverable. A story focused on Phase-B operator hardening (S-19.07 scope) should confirm that the event class provides operator visibility, not just that it is emitted. (ACTIONABLE: story-writer S-19.07 v1.5→v1.6 — EC-005 Expected Behavior column gains an operator visibility note stating that `internal.capability_denied` events are surfaced via the dispatcher event log and visible to operators monitoring `.factory/logs/events-YYYY-MM-DD.jsonl`; ENCODED in same-burst fix.)

O-P14-06 — [observation; actionable; ENCODED in fix burst] E-19 epic Maintenance Considerations section lacks a note that input-hash drift in the STORY-INDEX delivery-summary is a recurring manual maintenance burden (this is the second time in the E-19 cascade that delivery-summary hashes lagged frontmatter hashes — F-P7-002 at pass-7 and now F-P14-001 at pass-14). A tally of recurrences and an explicit drift-check reminder would equip a future state-manager or story-writer with institutional memory that this surface requires a sweep at every story-version bump. (ACTIONABLE: story-writer epic v1.11→v1.12 — Maintenance Considerations section gains a drift-check note: input-hash tally in STORY-INDEX delivery-summary requires sweep at every story input-hash change; this is the second recurrence (pass-7/pass-14); ENCODED in same-burst fix.)

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 3 |
| MEDIUM | 2 |
| LOW | 1 |
| Observations | 6 |

*Actionable findings: 6 (F-P14-001..F-P14-006). Trajectory 16→14→20→9→8→5→12→11→4→7→6→6→3→6 (6 findings; HIGH 3; MEDIUM 2; LOW 1; finding count increases 3→6; class analysis: zero spec-substance findings since pass 12 — twin STORY-INDEX sweep misses (F-P14-001/002) + one TD-VSDD-060 sibling miss of the F-P7-010 intrinsic-exit fix (F-P14-003) + narrative integers/attributions (F-P14-004/005) + dialect-gap family LOW (F-P14-006). Six observations (O-P14-01 accepted-with-record — preamble-distinctness lint deferred to PR review; O-P14-02..06 actionable — encoded in fix burst).*

**Overall Assessment:** block
**Convergence:** findings remain — iterate (strict 3-CLEAN per human directive D-761; no cap)
**Class analysis (pass 14 vs pass 13):** Pass-13 had 3 findings all in gate-strength parity class. Pass-14 has 6 findings; 5 are bookkeeping drift (twin STORY-INDEX sweep misses + narrative integer + attribution error + dialect gap) and 1 is a TD-VSDD-060 sibling miss. No spec-substance findings for 2 consecutive passes (pass 13 and 14). HIGH count rises 0→3 driven by STORY-INDEX drift findings (F-P14-001/002) and sibling-miss (F-P14-003), not by new spec design defects.

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 14 |
| **New findings** | 6 (F-P14-001..F-P14-006) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (6 / 6) |
| **Median severity** | HIGH (3 HIGH + 2 MED + 1 LOW) |
| **Trajectory (findings per pass)** | 16 → 14 → 20 → 9 → 8 → 5 → 12 → 11 → 4 → 7 → 6 → 6 → 3 → 6 |
| **Verdict** | FINDINGS_REMAIN — pass 15 dispatched under strict-3-CLEAN (no cap per human directive D-761) |

**Note on pass-14 composition:** Trajectory rises 3→6 after the lowest point in the E-19 cascade. The rise is driven entirely by bookkeeping/process-gap findings, not by new spec design defects. F-P14-001 and F-P14-002 are twin STORY-INDEX sweep misses — the delivery-summary hash and section-header version were not updated in the D-763 fix burst when the epic advanced from v1.9 to v1.10 and beyond. F-P14-003 is a TD-VSDD-060 sibling miss: F-P7-010 closed the `grep -q` inverted-exit class in S-19.04's AC-007 but did not extend the sweep to S-19.06 AC-003 which has the same structural pattern. F-P14-004 and F-P14-005 are narrative integers/attributions (integer "six" vs frontmatter "7"; pass-2 vs pass-3 attribution). F-P14-006 is a dialect-gap finding (BSD-grep BRE alternation) in the L-BB-simulation-shell-dialect-gap family. None of these findings represent new architectural or behavioral-contract gaps.

---

## Fix-Burst Closure Section (D-765)

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. Same-burst fixes do NOT advance streak.

**HUMAN DIRECTIVE (carried through fix burst):** Continuation policy = STRICT 3-CLEAN (BC-5.39.001), no pass cap, no asymptotic acceptance. Directive carried from D-761.

**All 6 findings + 5 actionable observations closed. Story-writer single leg per standing routing table (story AC gates + epic prose/EAC = story-writer; no BC amendment required). No architect or product-owner legs required — all findings are in story ACs, epic prose, and STORY-INDEX bookkeeping.**

**ORCHESTRATOR PREFLIGHT EXTENSION recorded:** the pre-pass check now includes (a) input-hash parity: verify delivery-summary hashes in STORY-INDEX match current story frontmatter input-hash values for all 7 E-19 stories; and (b) section-header version parity: verify STORY-INDEX E-19 section header version matches current epic frontmatter version. These two dimensions exposed by F-P14-001/002 are now mandatory preflight checks before each dispatch.

**Hook telemetry note:** three same-burst hook catches worked as designed — (a) `validate-table-cell-count` blocked on an unescaped jq pipe character in a draft cell (`|` inside a table cell without escaping, resolved by backslash-escaping); (b) `validate-changelog-monotonicity` blocked on a version-string appearing after a newer version in the changelog array (ordering error in epic v1.12 draft, resolved by reordering); (c) `validate-input-hash` fired on the epic after the epic inputs list was modified but input-hash was not recomputed (resolved by recomputing input-hash `f6bf703`). All three catches resolved in-burst; no write bypasses; positive control signal confirming the hook chain is functioning correctly.

### Story-writer leg (all 6 findings + 5 encoded observations)

- **STORY-INDEX v4.146→v4.147 (F-P14-001 + F-P14-002):** (a) Delivery-summary input-hashes corrected: S-19.02 hash `a6b25d1→6beeac8`; S-19.07 hash `eb137f3→46c2ffa`; all 7 hashes now current and distinct (re-derived sentence present: "All 7 distinct."). (b) E-19 section header updated: `v1.9→v1.12` (same-burst epic advance to v1.12 reflected). **Body-amendment evidence (Evidence Rule (a)):** `grep "S-19.02=6beeac8\|S-19.07=46c2ffa" .factory/stories/STORY-INDEX.md` → confirms both corrected hash values present; `grep "^## Epic E-19.*v1.12" .factory/stories/STORY-INDEX.md` → confirms section header updated. Closes F-P14-001 + F-P14-002.

- **S-19.06 v1.10→v1.11 (F-P14-003 + F-P14-006 + O-P14-02):** (a) AC-003 gate rewritten to `[ -z "$(grep -oE ...)" ]` intrinsic-exit form (mirrors F-P7-010 S-19.04 pattern; inverted semantics eliminated). (b) Gate 2 clause (i) ERE alternation: `grep -E 'pub fn read_prefix|pub fn read_file'` replaces BRE `grep 'pub fn read_prefix\|pub fn read_file'` (portable across GNU and BSD grep). (c) AC-003 cfg-block gate tightened to `read_prefix` scope: grep pattern scoped so cfg annotation is verified specifically for `pub fn read_prefix` (not broader host.rs scope). **Body-amendment evidence (Evidence Rule (a)):** `grep -n "intrinsic\|\[ -z\|grep -oE\|grep -E.*read_prefix" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -10` confirms intrinsic-exit form + ERE alternation + scoped cfg check present. Closes F-P14-003 + F-P14-006 + O-P14-02.

- **S-19.03 v1.10→v1.11 (O-P14-03):** AC-006 gate wrapped with `set -o pipefail` so that jq parse errors on malformed event JSONL are surfaced rather than silently discarded; `$SINK_FILE` variable added for shell-safety. **Body-amendment evidence (Evidence Rule (a)):** `grep -n "pipefail\|SINK_FILE" .factory/stories/S-19.03-warn-pending-wave-gate-file-not-found.md | grep -i "AC-006\|set -o\|SINK"` confirms pipefail wrapping + SINK_FILE variable present. Closes O-P14-03.

- **S-19.07 v1.5→v1.6 (O-P14-05):** EC-005 Expected Behavior column gains operator visibility note: `internal.capability_denied` events are surfaced via the dispatcher event log and visible to operators monitoring `.factory/logs/events-YYYY-MM-DD.jsonl`. **Body-amendment evidence (Evidence Rule (a)):** `grep -c "events-YYYY-MM-DD\|operator.*visibility\|capability_denied.*event.*log" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md` → ≥1 confirming operator visibility note present. Closes O-P14-05.

- **Epic v1.11→v1.12 (F-P14-004 + F-P14-005 + O-P14-04 + O-P14-06):** (a) F-P14-004: "six subsystems" → "seven subsystems" in Epic Placement Justification prose. (b) F-P14-005: Out-of-Scope BC-3.08.001 attribution corrected at both occurrences — "pass-2" → "pass-3" (the fix burst was pass-3 D-754; the review found it at pass-2 D-753). (c) O-P14-04: EAC-008 added — BC-3.08.001 Invariant 6 schema-level property tests (S-19.05 AC-002 gates a/b) pass in CI; preservation gate for the entry_index+plugin_name+type schema-level contract at epic delivery. (d) O-P14-06: Maintenance Considerations section gains drift-check note — input-hash tally in STORY-INDEX delivery-summary requires sweep at every story input-hash change; second recurrence (pass-7/pass-14). **Body-amendment evidence (Evidence Rule (a)):** `grep -c "seven subsystems\|pass-3\|EAC-008\|drift-check" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → ≥4 confirming all four changes present. Closes F-P14-004 + F-P14-005 + O-P14-04 + O-P14-06.

### Orchestrator independent verification (before declaring closure)

Orchestrator independently verified the following closure claims by reading production artifact bodies and running the mandatory BC-cite preflight per D-759/D-760:

1. **STORY-INDEX hash corrections (F-P14-001):** `grep "S-19.02=6beeac8\|S-19.07=46c2ffa" .factory/stories/STORY-INDEX.md` → both corrected hash values confirmed; "All 7 distinct." sentence confirmed.
2. **STORY-INDEX section header (F-P14-002):** `grep "^## Epic E-19" .factory/stories/STORY-INDEX.md` → `## Epic E-19 — Post-rc.22 Operator Hardening (...) — draft, v1.12` confirmed.
3. **S-19.06 AC-003 intrinsic-exit form (F-P14-003):** `sed -n '/AC-003/,/AC-004/p' .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | grep -c "\[ -z"` → ≥1 confirming intrinsic-exit form present.
4. **S-19.06 Gate 2 ERE alternation (F-P14-006):** `grep -c "grep -E.*read_prefix" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md` → ≥1 confirming ERE flag present; `grep -c "grep.*read_prefix.*\\\\|" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md` → 0 confirming BRE form absent.
5. **Epic integer correction (F-P14-004):** `grep -c "seven subsystems" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → ≥1; `grep -c "six subsystems" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → 0.
6. **Epic attribution correction (F-P14-005):** `grep -c "pass-3" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → ≥2 (two occurrences corrected); `grep -c "pass-2.*BC-3.08.001\|BC-3.08.001.*pass-2" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → 0.
7. **BC-cite preflight — ALL SIX BCs ZERO-DRIFT (mandatory post-sweep):** Per-file loop across all 9 E-19 artifacts (epic + S-19.01..S-19.07) × 6 BCs: ZERO stale live tokens.
8. **4-index at D-765 closure:** `grep "^version:" .factory/specs/behavioral-contracts/BC-INDEX.md .factory/specs/verification-properties/VP-INDEX.md .factory/stories/STORY-INDEX.md .factory/specs/architecture/ARCH-INDEX.md` → BC-INDEX: "3.76" / VP-INDEX: "2.53" / STORY-INDEX: "4.147" / ARCH-INDEX: "2.90".

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. **NEXT:** E-19 adversarial pass-15 (fresh context; 20-policy rubric; strict-3-CLEAN no-cap per human directive D-761; per-file BC-cite preflight mandatory before dispatch; Evidence Rules (a)+(b) mandatory; index-writing legs SEQUENCED per D-757; ORCHESTRATOR PREFLIGHT EXTENSION: include input-hash parity + section-header version parity checks; trajectory 16→14→20→9→8→5→12→11→4→7→6→6→3→6→pass-15).
