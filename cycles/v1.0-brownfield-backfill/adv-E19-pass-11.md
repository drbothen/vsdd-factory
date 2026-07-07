---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-07T00:00:00Z
phase: 11
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-brownfield-backfill
cascade: E-19-story
pass: 11
previous_review: adv-E19-pass-10.md
perimeter: E-19 epic + S-19.01..S-19.07 + STORY-INDEX
verdict: NOT-CLEAN
blocker_count: 0
high_count: 1
medium_count: 4
low_count: 1
observation_count: 9
streak: 0/3
parent_decision: D-762
---

# Adversarial Review — E-19 Pass 11 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law; rubric = policies.yaml read directly; 20 policies)
**Date:** 2026-07-07
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 1 / MEDIUM 4 / LOW 1 (6 findings + 9 observations; counts matched enumeration; all findings artifact-grounded; live-vs-history adjudication held — zero noise findings)
**Streak:** 0/3

---

## Finding ID Convention

This review uses the in-cycle finding ID format established for the `v1.0-brownfield-backfill` cascade:

`F-P<PASS>-<SEQ>` — e.g., `F-P11-001`, `F-P11-002`, etc. Severity is stated inline in the finding header. This format is consistent with prior E-19 adversarial review passes in this cycle.

---

## Part A — Fix Verification (pass >= 2 only)

Pass-10 NOT-CLEAN B0/H2/M2/L3 (7 findings + 5 observations; 0 false-positives; HUMAN DIRECTIVE strict-3-CLEAN no-cap; PO+SW legs; closed D-761). Fresh-context adversary reads only prior Part A — findings F-P10-001..F-P10-007. All 7 findings verified CLOSED by artifact evidence at pass-11 perimeter entry:

- **F-P10-001 CLOSED** (BC-4.13.001 v1.8 PC3 Phase-A + Phase-B prose corrected to `.factory/STATE.md` (file-specific); `grep -n "path_allow" .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md | head -10` returns `.factory/STATE.md` at all path_allow sites; Invariant 5 TOML shape unchanged; S-19.07 v1.5 4 path_allow sites corrected; `grep -c "path_allow.*\.factory\"" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md` → 0; S-19.02 v1.8 BC cite sweep clean; BC-INDEX v3.75; D-761 PO+SW legs.)
- **F-P10-002 CLOSED** (S-19.04 v1.10 AC-004 gate body genuinely amended — quote-tolerant `grep -vE 'tool = [...]'` filter physically present; v1.10 changelog explicitly records v1.9 false attestation; `grep -A 5 "AC-004\|quote-toler" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | head -20` confirms filter in normative AC-004 body; D-761 SW leg.)
- **F-P10-003 CLOSED** (STORY-INDEX v4.142 intro block stale live version tokens stripped; `grep -oE "S-19\.[0-9]+ v[0-9]+\.[0-9]+" .factory/stories/STORY-INDEX.md | head -5` → zero hits in intro block; D-761 SW leg.)
- **F-P10-004 CLOSED** (S-19.05 v1.9 AC-001 updated to slurp form: `jq -se 'any(.[]; .type == "plugin.completed" and ...)'`; `grep -n "jq -se\|-se " .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md` confirms slurp form in AC-001 gate body; D-761 SW leg.)
- **F-P10-005 CLOSED** (S-19.06 v1.6 deferral gate updated to Merge-PR canonical pattern: `gh pr list --search "S-19.06" --state merged --repo drbothen/vsdd-factory`; `grep -n "Merge-PR\|gh pr list" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -5` confirms Merge-PR form; D-761 SW leg.)
- **F-P10-006 CLOSED** (S-19.04 v1.10 Task 13 distinct-projection documented: 54 raw lines → 7 distinct via `sort -u`; gate assertion verifies `sort -u | wc -l` = 7; `grep -n "Task 13\|54\|distinct\|sort -u" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | head -10` confirms distinct-count derivation; D-761 SW leg.)
- **F-P10-007 CLOSED** (S-19.06 v1.6 contrastive "not ADR-025 D-15" note replaced with concurring framing: "IMPLEMENTS per ADR-025 D-15; S-19.07 MIGRATES to consume it"; `grep -n "not ADR\|IMPLEMENTS.*ADR-025 D-15" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -5` confirms concurring form and absence of contrastive form; D-761 SW leg.)

New findings from pass-11 review below in Part B.

---

## Part B — New Findings

*Verbatim adversary output per D-448(a) source-attestation. Do not summarize or soften. Every finding carries independent ground-truth grep per premise-verification discipline.*

F-P11-001 — HIGH — S-19.01 AC-001 misstates the normative stderr literal from BC-5.42.001 EC-001. The story-writer introduced an invented error message that no correct implementation would emit, rendering the AC-001 gate permanently failing against a compliant implementation. Ground-truth verification: (1) `grep -n "EC-001\|stderr\|cannot pin\|gh pr view failed" .factory/specs/behavioral-contracts/ss-05/BC-5.42.001.md | head -10` — BC-5.42.001 EC-001 defines the canonical stderr output as: `"gh pr view failed for PR #<pr_number>"` (the normative message emitted by `check-stale-verdict.sh` when the `gh pr view` call fails). (2) `grep -n "EC-001\|stderr\|cannot pin\|gh pr view failed" .factory/stories/S-19.01-pr-manager-hardening.md | head -10` — S-19.01 v1.8 AC-001 specifies a gate that checks for stderr containing `"cannot pin covered HEAD SHA"` — a message that is not present in BC-5.42.001, not emitted by `check-stale-verdict.sh`, and not present in any normative implementation of the covered-SHA check. The story-invented message and the normative BC-5.42.001 EC-001 message are mutually exclusive: a correct implementation emitting `"gh pr view failed for PR #<pr_number>"` would FAIL the S-19.01 AC-001 gate. This is a behavioral contract misquotation at the gate locus, not a cosmetic error. (3) Sibling-sweep: the EC-001 misquotation is isolated to S-19.01 AC-001; the BC body at BC-5.42.001 is correct; no other E-19 story cites this EC-001 message form. This finding is in the same category as F-P9-002 (AC-001 locus naming the wrong entity), confirming that EC-001 is a persistent misquotation risk site in S-19.01. Fix: story-writer S-19.01 v1.8→v1.9 — AC-001 gate: replace story-invented `"cannot pin covered HEAD SHA"` with normative `"gh pr view failed for PR #<pr_number>"` verbatim per BC-5.42.001 EC-001; verify all 5 sites in the AC body that reference the EC-001 message form are updated; EC-001 locus annotation corrected to name `check-stale-verdict.sh` as the emitter.

F-P11-002 — MEDIUM — [process-gap] F-P11-001 is a category sibling of F-P9-002 (same-type recurrence: S-19.01 AC-001 names the wrong entity / wrong message at the EC-001 locus) and represents a TD-VSDD-060 sibling-sweep gap. Ground-truth verification: `grep -n "F-P9-002\|sibling\|EC-001\|AC-001" .factory/cycles/v1.0-brownfield-backfill/adv-E19-pass-9.md | head -10` — F-P9-002 was "AC-001 gh-failure arm named the LLM agent as the locus rather than check-stale-verdict.sh"; the pass-9 fix (S-19.01 v1.8) corrected the locus name but did not verify the message literal against BC-5.42.001 EC-001. The sibling-sweep at D-760 closed the locus-naming aspect but left the message-literal aspect unchecked. Per TD-VSDD-060: when a finding at a named site is closed, ALL attributes of that site must be swept — locus naming AND message literal. The fix burst for F-P11-001 constitutes the complete TD-VSDD-060 sweep for the S-19.01 AC-001 EC-001 locus. (PROCESS-GAP encoded: pass-9 sweep was partial — locus corrected, message-literal not verified; complete sweep required at AC-001 EC-001 locus.)

F-P11-003 — MEDIUM — S-19.05 v1.9 AC-001 pre-filter (`entry_index` grep) causes vacuous pass on fixtures lacking `entry_index` fields — erasing the missing-entry_index defect class. Ground-truth verification: (1) `grep -n "entry_index\|pre-filter\|jq -se" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -15` — AC-001 at v1.9 applies a pre-filter that greps the JSONL sink file for `entry_index` before passing records to the slurp form. (2) Demonstration with captured jq stdout: `printf '{"type":"plugin.completed","timestamp":"t","hook_id":"h","tool":"Edit","exit_code":0,"duration_ms":1,"source":"dispatcher","plugin_version":"1.0"}\n' | jq -se 'any(.[]; .type == "plugin.completed" and .entry_index != null)'` → `false` with exit code 1 (correct gate failure — record lacks entry_index). However: `printf '{"type":"plugin.completed","timestamp":"t","hook_id":"h","tool":"Edit","exit_code":0,"duration_ms":1,"source":"dispatcher","plugin_version":"1.0"}\n' | grep -q "entry_index" || echo "NO_ENTRY_INDEX"` → `NO_ENTRY_INDEX` — the pre-filter fails on this fixture because `entry_index` is absent, causing the gate to exit BEFORE the jq slurp assertion is reached. An empty slurp (zero records pass the pre-filter) causes `any([]; ...)` to evaluate as `false` → exit 1 → gate FAILS but for the WRONG reason: the gate is failing because no records passed the pre-filter, not because the condition was evaluated and failed. More critically: if the pre-filter greps for `entry_index` and finds ZERO matches, the gate exits early with an error message that OBSCURES the true defect (missing `entry_index` field on dispatcher events). The `entry_index` pre-filter is erasing the missing-entry_index defect class by short-circuiting before the field-presence assertion can report its finding. Fix: story-writer S-19.05 v1.9→v1.10 — remove the `entry_index` pre-filter; add a non-empty guard that fails with a clear message if the slurped records set is empty; let the jq slurp assertion report `.entry_index != null` failures directly (the slurp form handles missing fields correctly without a pre-filter). The fixture design for AC-001 must include at least one record with all 8 required fields to test affirmatively.

F-P11-004 — MEDIUM — S-19.05 v1.9 AC-002 exhibits the same vacuous-pass class on an empty abandoned set. Ground-truth verification: `grep -n "AC-002\|abandoned\|jq -se\|empty\|vacuous" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -15` — AC-002 at v1.9 applies a similar structural pattern to the `plugin.abandoned` (Event 5) field loop but with a pre-filter that uses an empty-set test. When the dispatcher emits no abandoned events (empty abandoned set in the JSONL sink), the AC-002 gate either passes vacuously (all 0 records satisfy all conditions — vacuous truth) or is gated on a pre-filter that silently exits early. In either case the gate cannot distinguish between (a) "no abandoned events were emitted but the gate condition held for the zero records present" and (b) "the dispatcher correctly emitted at least one abandoned event with all required fields." The POLICY 11 positive-control requirement mandates that AC-002 include at least one fixture record with all Event 5 fields to test the affirmative case. Fix: story-writer S-19.05 v1.9→v1.10 — AC-002 must include a positive-control assertion: at least one `plugin.abandoned` record with all BC-3.08.001 Event 5 required fields must be present in the fixture; the gate must fail if the abandoned-events set is empty (use a non-empty guard analogous to the AC-001 fix).

F-P11-005 — MEDIUM — ADR-025 D18 test bullet (e) cites a stale 262144-byte read bound that contradicts BC-4.13.001 v1.8 Phase-B max_bytes=8192. Ground-truth verification: (1) `grep -n "D18\|262144\|8192\|max_bytes" .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md | head -15` — ADR-025 Decision 18 test bullet (e) at v1.9 states the verify-factory-lock plugin must correctly parse a STATE.md file that approaches the 262144-byte Phase-A cap (the fixture body is padded to force the test). However, per BC-4.13.001 v1.8 Phase-B, the plugin invokes `host::read_prefix` with `max_bytes=8192` — meaning the plugin READS ONLY THE FIRST 8192 BYTES regardless of the total file size. The 262144-byte reference in the test bullet describes the fixture size (total file length), not the read window. The test specification is ambiguous: it does not state that the plugin reads only a 8192-byte prefix and must correctly parse frontmatter from that prefix even when the full file is larger. A reader of the test bullet cannot distinguish "test that the full 262144-byte file is correctly processed" from "test that a 8192-byte prefix correctly captures frontmatter when the file is large." Fix: architect ADR-025 v1.9→v1.10 — D18 test bullet (e) reworded: the fixture body is padded past 8192 bytes (approaching 262144-byte Phase-A cap) to test that the plugin correctly parses frontmatter from the 8192-byte prefix read via `host::read_prefix`; the 262144 value describes the FIXTURE SIZE (upper bound for Phase-A file cap), NOT the read bound; the read bound is `max_bytes=8192` per BC-4.13.001 v1.8 Phase-B.

F-P11-006 — LOW — S-19.06 deferral-gate is asymmetrically worded relative to its sibling S-19.03. Ground-truth verification: `grep -n "deferral\|depends_on\|S-19.03\|S-19.04\|hard\|soft" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -10` AND `grep -n "deferral\|depends_on\|hard\|soft" .factory/stories/S-19.03-warn-pending-wave-gate-file-not-found.md | head -10` — S-19.03 AC for its dependency gate uses bolded hard-gate language ("MUST NOT be merged until S-19.02 merges") while S-19.06 v1.6 uses soft language for its dependency gate on S-19.04 ("should not deploy until S-19.04 merges") despite both stories having identical `depends_on:` frontmatter relationships. The asymmetry is arbitrary: both stories have exactly one upstream dependency and the same wave-gating semantics per the E-19 dependency graph. A reader could conclude S-19.06's deployment dependency is optional. Fix: story-writer S-19.06 v1.6→v1.7 — deferral gate for S-19.04 dependency updated to bolded hard-gate language matching S-19.03's form ("MUST NOT be merged until S-19.04 merges; attempting to deploy S-19.06 without S-19.04 would register `host::read_prefix` against a dispatcher that has not yet exposed the host function").

---

## HUMAN DIRECTIVE (recorded prominently per orchestrator request)

**Continuation policy = STRICT 3-CLEAN (BC-5.39.001), no pass cap, no asymptotic acceptance. Directive established 2026-07-07 (D-761) by human over three presented alternatives: (1) accept at floor, (2) 2 more passes then accept, (3) strict BC-5.39.001 no cap. Human chose Option C. This directive carries across CLEAR per §3 User Directives.**

---

## Verifications That PASSED

The following structural checks were confirmed clean at pass-11 perimeter entry:

1. BC-cite preflight PASS (per-file loop; D-760 canonical form): all 9 E-19 artifacts across all 6 E-19 BCs (BC-4.13.001 v1.8 / BC-2.07.001 v1.2 / BC-2.02.011 v1.4 / BC-3.08.001 v1.19 / BC-5.42.001 v1.1 / BC-1.17.001 v1.1); zero stale live cites confirmed.
2. F-P10-001..F-P10-007 all CLOSED (verified above in Part A; 7/7 confirmed closed).
3. E-19 epic subsystems_affected PASS: `[SS-01, SS-02, SS-03, SS-04, SS-05, SS-07, SS-09]`; SS-06 absent (F-P9-001 fix held).
4. STORY-INDEX intro block PASS: `grep -oE "S-19\.[0-9]+ v[0-9]+\.[0-9]+" .factory/stories/STORY-INDEX.md | head -5` → zero hits in intro block (F-P10-003 fix held).
5. S-19.04 AC-004 quote-tolerant filter PASS: filter physically present in normative gate body (F-P10-002 fix held; paper-fix not re-introduced).
6. 4-index at perimeter entry PASS: BC v3.75 / VP v2.53 / STORY v4.142 / ARCH v2.90 consistent with D-761 state (ARCH-INDEX bumped D-761 fix burst ADR-025 v1.9→v1.10).

---

## Observations

O-P11-A — [pass-attestation] BC-4.13.001 Phase-A/B path_allow corrected to `.factory/STATE.md` (F-P10-001) remains closed at pass-11; all 4 S-19.07 path_allow sites verified; S-19.02 cite sweep clean. Fixes introduced at D-761 PO+SW legs ALL HELD at pass-11 perimeter entry.

O-P11-B — [confirmation of O-P10-D] BC-4.13.001 PC-6 UTF-8 clause: `grep -n "PC-6\|UTF-8\|non-ASCII\|bytes" .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md | head -10` confirms BC-4.13.001 PC-6 explicitly codifies that `host::read_prefix` does not perform UTF-8 normalization or trimming — the returned bytes are the raw prefix regardless of encoding. Additionally, S-19.06 `grep -n "UTF-8\|non-ASCII\|encoding" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -5` shows S-19.06 traces resolve cleanly to PC-6 (the story's compliance rules reference PC-6 as the authority for encoding behavior). O-P10-D queued question fully resolved — no additional fix required. (ACCEPTED: confirms O-P10-D resolution; BC-1.17.001 PC-6 codifies no-UTF-8-trim; S-19.06 traces resolve cleanly; non-blocking.)

O-P11-C — [observation] Several S-19.01 and S-19.05 gate bodies contain multi-line block comments (`/* ... */`) for inline documentation of gate logic. The vsdd-factory codebase convention (Bash hook scripts use inline `#`-prefixed comments; WASM Rust source uses doc-attributes `///`) does not include multi-line C-style block comments in Bash gate scripts. The block comments are non-idiomatic for the target execution environment and may cause parsing issues if a gate script is executed under strict `set -euo pipefail`. (ACCEPTED-WITH-RECORD: multi-line block comments in Bash gate scripts are non-idiomatic; next story-writer touch on affected gates should migrate to `#`-prefixed inline comments; non-blocking this pass.)

O-P11-D — [observation] S-19.05 AC-001 at v1.9 references "T-006" as a test ID for the Mutex-gate assertion on concurrent dispatcher events. Ground-truth verification: `grep -n "T-006\|Mutex\|mutex\|concurrent" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -10` — the T-006 test ID is cited in the AC-001 narrative as the literal `Mutex`-gate test that verifies the dispatcher's concurrency control. The T-006 reference is a normative test ID that should be traceable to a failing test stub and a corresponding Red Gate. The test ID is a literal gate per ADR-025 D-15 semantics. (ENCODED: T-006 literal Mutex gate is a normative test ID in S-19.05 AC-001; the Red Gate for T-006 must be implemented before the story can enter TDD phase; traceability is correctly anchored.)

O-P11-E — [pass-attestation] S-19.04 v1.10 quote-tolerant AC-004 gate (F-P10-002; D-761 fix) held at pass-11 perimeter entry: `grep -A 3 "AC-004\|quote-toler" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | head -10` confirms filter present; no paper-fix re-introduction detected.

O-P11-F — [pass-attestation] S-19.05 v1.9 jq slurp form (F-P10-004; D-761 fix) held at pass-11 perimeter entry: slurp (`-se`) form present in AC-001; non-slurp `-e` form absent outside changelog sections. (The vacuous-pass issues in F-P11-003/F-P11-004 are new structural issues not present at pass-10 because the pre-filter and empty-set patterns were introduced as part of the pass-10 fix itself — they constitute a different defect class than the order-dependency found at pass-10.)

O-P11-G — [pass-attestation] S-19.06 v1.6 Merge-PR deferral pattern (F-P10-005; D-761 fix) held at pass-11 perimeter entry: `grep -n "gh pr list\|Merge-PR" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -5` confirms Merge-PR form; ID-substring form absent.

O-P11-H — [pass-attestation] S-19.07 v1.5 path_allow four-site correction (F-P10-001; D-761 fix) held at pass-11 perimeter entry: `grep -c "path_allow.*\.factory\"" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md` → 0; `grep -c "path_allow.*STATE.md" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md` → 4 (all four sites corrected).

O-P11-I — [pass-attestation] E-19 epic v1.9 dependency graph (F-P9-004; D-760 fix) held at pass-11: mermaid block with exactly 4 edges; W1→S-19.07 visual artifact absent; `grep -n "mermaid\|graph LR" .factory/stories/epics/E-19-post-rc22-operator-hardening.md | head -5` confirms graph form.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 1 |
| MEDIUM | 4 |
| LOW | 1 |
| Observations | 9 |

*Actionable findings: 6 (F-P11-001..F-P11-006). Trajectory 16→14→20→9→8→5→12→11→4→7→6 (6 findings; lower than pass-10's 7; HIGH count reduced to 1 from 2; MEDIUM count increased from 2 to 4 reflecting new structural vacuous-pass category). Nine PASSED-verification observations (O-P11-A/E/F/G/H/I attest passes 7-10 fixes ALL HELD; O-P11-B resolves O-P10-D queued question; O-P11-C/D advisory with record).*

**Overall Assessment:** block
**Convergence:** findings remain — iterate (strict 3-CLEAN per human directive D-761; no cap)
**Severity decay from pass 10 (enumerated):** B0/H2/M2/L3 (7 total) → B0/H1/M4/L1 (6 total; HIGH reduced by 1; MEDIUM increased by 2 from new vacuous-pass class; LOW reduced by 2; net improvement −1 finding)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 11 |
| **New findings** | 6 (F-P11-001..F-P11-006) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (6 / 6) |
| **Median severity** | MEDIUM |
| **Trajectory (findings per pass)** | 16 → 14 → 20 → 9 → 8 → 5 → 12 → 11 → 4 → 7 → 6 |
| **Verdict** | FINDINGS_REMAIN — pass 12 dispatched under strict-3-CLEAN (no cap per human directive D-761) |

**Note on pass-11 composition:** The trajectory continues asymptotic descent (6 < 7). F-P11-001 (HIGH) is the third recurrence class at the S-19.01 AC-001 EC-001 locus — locus-naming corrected at pass-9; message-literal not swept (F-P11-002 process-gap). F-P11-003/004 (MEDIUM vacuous-pass) is a new defect class introduced by the pass-10 fix: the pre-filter and empty-set patterns that were added to fix the order-dependency created a new structural vacuous-pass risk. This is a characteristic fix-introduced defect: fixing the order-dependency (F-P10-004) by adding a pre-filter introduced a pre-filter-erasure risk. F-P11-005 (MEDIUM) is an ADR-025 D18(e) specification ambiguity at the fixture-vs-read-bound boundary. F-P11-006 (LOW) is a sibling-gating asymmetry.

---

## Fix-Burst Closure Section (D-762)

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. Same-burst fixes do NOT advance streak.

**HUMAN DIRECTIVE (carried through fix burst):** Continuation policy = STRICT 3-CLEAN (BC-5.39.001), no pass cap, no asymptotic acceptance. Directive carried from D-761.

**All 6 findings closed. Orchestrator self-attributed deviation: D-757 sequencing rule violated (two index-writing legs dispatched in parallel; recorded as process deviation). Quad-race repair applied in-burst (no version bump). Orchestrator independently verified gate bodies per Evidence Rules (a)+(b).**

### Orchestrator self-attributed process deviation (D-757 sequencing rule violation)

The orchestrator dispatched two index-writing legs in parallel during the pass-11 fix burst, violating the D-757 sequencing rule that requires index-writing legs to be dispatched SEQUENTIALLY. As a direct consequence, the architect leg completed its work citing STORY-INDEX v4.142 (the live version at the time of the architect leg's dispatch), while the story-writer leg subsequently bumped STORY-INDEX to v4.143. The ARCH-INDEX v2.90 changelog entries authored by the architect leg therefore cite a stale STORY-INDEX version — a quad-race (parallel-leg index-citation race condition). This deviation is self-attributed to the orchestrator. Recovery: in-burst quad-race repair applied to ARCH-INDEX v2.90 entries (STORY-INDEX v4.142 → v4.143 in both last_amended and changelog; no ARCH-INDEX version bump per in-burst repair policy). Root cause: orchestrator dispatched architect and story-writer legs in parallel. D-757 correction: index-writing legs MUST be dispatched sequentially in pass-12 and all subsequent bursts.

**Quad-race repair verification:** `grep -c "v4\.142" .factory/specs/architecture/ARCH-INDEX.md` → 0 (zero v4.142 cites remaining); `grep -o "STORY-INDEX v4\.14[0-9]" .factory/specs/architecture/ARCH-INDEX.md | head -4` → all returns `STORY-INDEX v4.143` (v2.90 entries corrected).

### Architect leg

- **ADR-025 v1.9→v1.10 (F-P11-005):** D18 test bullet (e) reworded — fixture body padded past 8192 bytes (approaching 262144-byte Phase-A cap) to test that verify-factory-lock plugin correctly parses frontmatter from the 8192-byte prefix read via `host::read_prefix`; the 262144 value describes the FIXTURE SIZE (total file length upper bound, approaching Phase-A cap), NOT the read bound; the read bound is `max_bytes=8192` per BC-4.13.001 v1.8 Phase-B. The ambiguous "approaches the 262144-byte Phase-A cap" phrasing replaced with explicit fixture-vs-read-bound distinction. **Body-amendment evidence (Evidence Rule (a)):** `grep -n "8192\|262144\|max_bytes\|fixture" .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md | grep "D18"` confirms D18(e) now cites `max_bytes=8192` as the read bound and 262144 as the fixture size bound. Closes F-P11-005.

- **ARCH-INDEX v2.89→v2.90 (architect leg + quad-race repair):** ADR-025 row bumped v1.9→v1.10. Quad-race repair applied to v2.90 changelog entries: STORY-INDEX v4.142 → STORY-INDEX v4.143 in both last_amended and changelog body. No additional version bump (in-burst repair of same-day rows).

### Story-writer leg

- **S-19.01 v1.8→v1.9 (F-P11-001 + F-P11-002):** AC-001 gate updated — story-invented `"cannot pin covered HEAD SHA"` replaced with normative BC-5.42.001 EC-001 literal: `"gh pr view failed for PR #<pr_number>"` at all 5 sites in the AC body; EC-001 locus annotation confirmed to name `check-stale-verdict.sh` as the emitter (TD-VSDD-060 full sweep at EC-001 locus: locus name + message literal both verified). **Body-amendment evidence (Evidence Rule (a)):** `grep -n "cannot pin\|gh pr view failed" .factory/stories/S-19.01-pr-manager-hardening.md | head -10` → zero "cannot pin covered HEAD SHA" hits; all 5 EC-001 message sites return `"gh pr view failed for PR #<pr_number>"`. **Changelog-claim-vs-body parity evidence (Evidence Rule (b)):** v1.9 changelog records "BC-5.42.001 EC-001 stderr literal corrected from story-invented 'cannot pin covered HEAD SHA' to normative 'gh pr view failed for PR #<pr_number>' (F-P11-001; TD-VSDD-060 full sweep — locus name and message literal both verified at 5 AC body sites; F-P11-002 process-gap acknowledged)"; body grep above confirms change. Closes F-P11-001 + F-P11-002.

- **S-19.05 v1.9→v1.10 (F-P11-003 + F-P11-004):** (a) AC-001 pre-filter removed — `entry_index` grep pre-filter that erased the missing-entry_index defect class dropped from gate; non-empty guard added: gate fails with `ASYNC_SINK_EMPTY` if the JSONL sink file contains zero records after the initial pre-filter step; jq slurp assertion now evaluates `.entry_index != null` directly without pre-filter; fixture design mandates at least one record with all 8 required fields for affirmative pass. (b) AC-002: non-empty guard added before the `plugin.abandoned` field loop — gate fails with `ABANDONED_SET_EMPTY` if the abandoned events set is empty; positive-control assertion: at least one `plugin.abandoned` record with all BC-3.08.001 Event 5 required fields must be present in the fixture for AC-002 to pass. T-006 literal Mutex gate encoded as a normative test ID per O-P11-D (test must be implemented before TDD phase). **Body-amendment evidence (Evidence Rule (a)):** `grep -n "entry_index\|pre-filter\|ASYNC_SINK_EMPTY\|ABANDONED_SET_EMPTY\|T-006" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -15` confirms: pre-filter absent; ASYNC_SINK_EMPTY guard present in AC-001; ABANDONED_SET_EMPTY guard present in AC-002; T-006 Mutex gate reference present. Closes F-P11-003 + F-P11-004.

- **S-19.06 v1.6→v1.7 (F-P11-006):** Deferral gate for S-19.04 dependency updated to bolded hard-gate language matching S-19.03's form: "**MUST NOT be merged until S-19.04 merges**; attempting to deploy S-19.06 without S-19.04 would register `host::read_prefix` against a dispatcher that has not yet exposed the host function." Previous soft-worded "should not deploy" form replaced. Symmetry with S-19.03's bolded deferral gate confirmed. **Body-amendment evidence (Evidence Rule (a)):** `grep -n "MUST NOT.*S-19.04\|should not.*S-19.04" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -5` → one hit "MUST NOT be merged until S-19.04 merges"; zero hits "should not deploy". Closes F-P11-006.

- **STORY-INDEX v4.142→v4.143 (story cell updates):** S-19.01 cell updated to v1.9; S-19.05 cell updated to v1.10; S-19.06 cell updated to v1.7. All other E-19 story cells UNCHANGED. **Body-amendment evidence (Evidence Rule (a)):** `grep "^version:" .factory/stories/STORY-INDEX.md` → `"4.143"`. `grep -E "S-19\.01|S-19\.05|S-19\.06" .factory/stories/STORY-INDEX.md | head -10` confirms v1.9, v1.10, v1.7 in catalog rows.

### Orchestrator independent verification (before declaring closure)

Orchestrator independently verified the following closure claims by reading production artifact bodies:

1. **BC-5.42.001 EC-001 literal in S-19.01 AC-001 (F-P11-001):** `grep -c "cannot pin covered HEAD SHA" .factory/stories/S-19.01-pr-manager-hardening.md` → 0 (story-invented message absent); `grep -c "gh pr view failed" .factory/stories/S-19.01-pr-manager-hardening.md` → 5 (normative literal present at all 5 AC body sites).
2. **ADR-025 D18(e) read-bound disambiguation (F-P11-005):** `grep -n "8192\|max_bytes" .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md | grep "D18"` → hit at D18(e) citing max_bytes=8192 as the read bound.
3. **S-19.05 pre-filter removal (F-P11-003/004):** `grep -c "entry_index.*grep\|grep.*entry_index" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md` → 0 (pre-filter absent); `grep -c "ASYNC_SINK_EMPTY\|ABANDONED_SET_EMPTY" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md` → 2 (non-empty guards present).
4. **S-19.06 hard-gate language (F-P11-006):** `grep -c "MUST NOT.*S-19\.04" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md` → 1; `grep -c "should not.*deploy\|should not.*S-19\.04" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md` → 0.
5. **4-index at D-762 closure:** `grep "^version:" .factory/specs/behavioral-contracts/BC-INDEX.md .factory/specs/verification-properties/VP-INDEX.md .factory/stories/STORY-INDEX.md .factory/specs/architecture/ARCH-INDEX.md` → BC-INDEX: "3.75" / VP-INDEX: "2.53" / STORY-INDEX: "4.143" / ARCH-INDEX: "2.90".
6. **Quad-race repair:** `grep -c "v4\.142" .factory/specs/architecture/ARCH-INDEX.md` → 0; `grep -o "STORY-INDEX v4\.143" .factory/specs/architecture/ARCH-INDEX.md | wc -l` → ≥2 (v2.90 entries repaired).

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. **NEXT:** E-19 adversarial pass-12 (fresh context; 20-policy rubric; strict-3-CLEAN no-cap per human directive D-761; per-file BC-cite preflight mandatory before dispatch; Evidence Rules (a)+(b) mandatory; index-writing legs SEQUENCED per D-757; trajectory 16→14→20→9→8→5→12→11→4→7→6→pass-12).
