---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-07T00:00:00Z
phase: 8
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-brownfield-backfill
cascade: E-19-story
pass: 8
previous_review: adv-E19-pass-7.md
perimeter: E-19 epic + S-19.01..S-19.07 + STORY-INDEX
verdict: NOT-CLEAN
blocker_count: 0
high_count: 3
medium_count: 5
low_count: 3
observation_count: 6
streak: 0/3
parent_decision: D-759
---

# Adversarial Review — E-19 Pass 8 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law; rubric = policies.yaml read directly; 20 policies)
**Date:** 2026-07-07
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 3 / MEDIUM 5 / LOW 3 (11 findings + 6 observations; counts matched enumeration; every finding carried artifact-level premise greps — the pass-7 F-P7-003 evidence-rules hardening held)
**Streak:** 0/3

---

## Finding ID Convention

This review uses the in-cycle finding ID format established for the `v1.0-brownfield-backfill` cascade:

`F-P<PASS>-<SEQ>` — e.g., `F-P8-001`, `F-P8-002`, etc. Severity is stated inline in the finding header. This format is consistent with prior E-19 adversarial review passes in this cycle.

---

## Part A — Fix Verification (pass >= 2 only)

Pass-7 NOT-CLEAN B0/H2/M5/L5 (12 findings + 7 observations; 1 false-positive F-P7-003 adjudicated; closed same-burst D-758; product-owner 3-BC sequential leg + story-writer). Fresh-context adversary reads only prior Part A — findings F-P7-001..F-P7-012. All 12 findings (including F-P7-003 real defect) verified CLOSED by artifact evidence at pass-8 perimeter entry:

- **F-P7-001 CLOSED** (STORY-INDEX v4.137→v4.138 — E-19 BC-coverage summary line re-derived from live BC headers: BC-4.13.001 v1.7 / BC-2.07.001 v1.2 / BC-2.02.011 v1.4 / BC-3.08.001 v1.19 / BC-5.42.001 v1.1 / BC-1.17.001 v1.1; 0 stale version tokens confirmed by orchestrator grep; D-758 story-writer leg).
- **F-P7-002 CLOSED** (S-19.04 v1.7→v1.8 — 7 volatile `~line NNN` anchors replaced with stable behavioral pattern anchors; POLICY 20 `release_bundle_no_dev_samples` compliance row added to §Traceability; TD-VSDD-091 compliance row added; D-758 story-writer leg).
- **F-P7-003 CLOSED** (real defect: S-19.06 v1.2→v1.3 — stale "ADR uses u64/i64 — BC wins" reconciliation note replaced with correct historical note reflecting ADR-025 Decision 15 v1.9 alignment; D-758 story-writer leg; adversary false-positive reclassification stands per D-758 orchestrator adjudication).
- **F-P7-004 CLOSED** (BC-2.07.001 v1.1→v1.2 EC-007 injectable-canonicalize seam; S-19.03 v1.6→v1.7 AC-001 negative-control B per BC-2.07.001 v1.2 ruling; D-758 product-owner + story-writer legs).
- **F-P7-005 CLOSED** (S-19.01 v1.5→v1.6 AC-004 upgraded to mechanism test: while-IFS-read fragment from rc.22 failure site cited verbatim; bash-3.2-compatible execution check added; D-758 story-writer leg).
- **F-P7-006 CLOSED** (BC-4.13.001 v1.6→v1.7 Invariant 10 inclusive upper-bound; condition restated `bytes_read > 200000 AND bytes_read ≤ 262144`; boundary table added; D-758 product-owner leg).
- **F-P7-007 CLOSED** (BC-3.08.001 v1.18→v1.19 entry_index schema-level defense note; S-19.05 v1.5→v1.6 AC-002 Gate recast schema-level; EC-005 fixture-path removed; D-758 product-owner + story-writer legs).
- **F-P7-008 CLOSED** (E-19 epic v1.6→v1.7 Description item 2 amended — Phase-A+B dual-story scope enumerated; O-P7-001 phased-continuation note added; D-758 story-writer leg).
- **F-P7-009 CLOSED** (S-19.05 v1.5→v1.6 T-006 recast as grep-inspection test asserting `entry_index` key present AND value non-negative integer in raw JSONL; distinct from T-003 presence-only; D-758 story-writer leg).
- **F-P7-010 CLOSED** (S-19.04 v1.7→v1.8 AC-004 Gate amended: intrinsic non-zero exit explicitly required; `|| true` wrapping FORBIDDEN; pipefail propagation confirmed; D-758 story-writer leg).
- **F-P7-011 CLOSED** (S-19.06 v1.2→v1.3 AC-003 Gate scoped to non-comment semantic call-sites: `grep -vE '^\s*(//)|(#)'` pre-filter; D-758 story-writer leg).
- **F-P7-012 CLOSED** (S-19.02 v1.5→v1.6 T-001..T-005 and T-007..T-009 given full inline test vector content; BC-4.13.001 v1.7 boundary semantics reflected including T-NNN for `bytes_read == 262144`; D-758 story-writer leg).

New findings from pass-8 review below in Part B.

---

## Part B — New Findings

*Verbatim adversary output per D-448(a) source-attestation. Do not summarize or soften. Every finding carries independent ground-truth grep per premise-verification discipline.*

F-P8-001 — HIGH — S-19.07 v1.2 AC-002 gate is globally-unscoped across the entire hooks-registry.toml and therefore inert and unsatisfiable in any real implementation. Ground-truth verification: `grep -c '\[hooks\.capabilities\.read_file\]' plugins/vsdd-factory/hooks-registry.toml` returns 22 — there are 22 `[hooks.capabilities.read_file]` blocks across the full registry (all plugins combined). The AC-002 gate as written asserts this count reaches 0 after the migration fix, which is structurally impossible: there are 20 other plugin entries in the registry that legitimately use `read_file` capabilities and are not touched by S-19.07. Only 2 entries (verify-factory-lock and verify-factory-lock-bash) are in scope; the other 20 will never go to 0. The gate is globally-unscoped: it asserts a property of the entire registry rather than of the 2 in-scope entries. `grep -c` in this context returns 22 permanently regardless of fix completeness. A correctly implemented S-19.07 would still produce `grep -c` = 20, failing the gate. Fix: story-writer S-19.07 v1.2→v1.3 — AC-002 Gate rewritten with per-entry awk scoping: for each of the two named entries (`verify-factory-lock` and `verify-factory-lock-bash`), use `awk '/^name = "verify-factory-lock"$/,/^\[\[hooks\]\]/'` (and corresponding bash-variant range) to extract only that entry's stanza and assert (a) `grep -c '\[hooks\.capabilities\.read_file\]'` returns 0 within the extracted stanza AND (b) `grep -c '\[hooks\.capabilities\.read_prefix\]'` returns ≥ 1 within the extracted stanza; both entries asserted independently.

F-P8-002 — HIGH — [process-gap] S-19.02 v1.6 is in a mid-propagation mixed state: multiple normative body sites still cite BC-4.13.001 v1.6 while the story header claims v1.6 (current at D-758 pass-7 fix). Ground-truth verification: `grep -n "BC-4\.13\.001 v1\.[0-6]" .factory/stories/S-19.02-verify-factory-lock-output-too-large.md | grep -v "Changelog\|last_amended\|modified\["` returns hits at the Phase-A scope note ("BC-4.13.001 v1.6 Phase-A amendment LANDED") and at the Architecture Compliance Rules table header row (citing "BC-4.13.001 v1.6"). The pass-7 story-writer fix used `replace_all` for the pattern "BC-4.13.001 v1.6 Phase-A" → "BC-4.13.001 v1.7 Phase-A" but missed two sites where the version appeared in reverse-word-order relative to "Phase-A": (1) the scope-note sentence renders as "v1.6 Phase-A amendment" (version before the Phase qualifier) and (2) the Arch Rules table row renders as "BC-4.13.001 v1.5→v1.7" (a range cite with stale left endpoint). These two sites require targeted single-site edits rather than replace_all. This is the FOURTH recurrence of the BC-bump→partial-propagation class across this cascade (prior: F-P4-002/003/009 D-755; F-P5-003 D-756; F-P6-003 D-757). Three-or-more threshold met; MECHANICAL GATE instituted per D-759 cure-extension (see §Closure Section). Fix: story-writer S-19.02 v1.6→v1.7 — targeted replacement of both reverse-word-order stale sites; orchestrator post-fix grep confirms zero live BC-4.13.001 v1.6 or earlier cites outside changelog sections in S-19.02.

F-P8-003 — HIGH — [process-gap] S-19.07 v1.2 is in the same BC-version mid-propagation state as S-19.02 (F-P8-002 sibling). Ground-truth verification: `grep -n "BC-4\.13\.001 v1\.[0-6]" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md | grep -v "Changelog\|last_amended\|modified\["` returns hits for (1) all body BC-cite locations showing "BC-4.13.001 v1.6 Phase-B" — the story was authored at pass-4 citing v1.5, was partially swept at pass-7 to v1.6 in some sections but Phase-B cites were never advanced to v1.7; and (2) two stray v1.5 cites at "Architecture Mapping" and "Previous Story Intel" sections where the `Invariant 10` soft-warn context was cited with the pre-pass-7 version. `grep -c "BC-4\.13\.001 v1\.6" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md` returns ≥ 5 (Phase-B BC table row, Phase-B Precondition 3 narrative, Architecture Compliance Rules, Token Budget, AC-001 Phase-B reference). `grep -c "BC-4\.13\.001 v1\.5" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md` returns 2 (stray Architecture Mapping + Previous Story Intel). All are outside changelog sections — live normative cites. Same BC-bump→partial-propagation class as F-P8-002; FOURTH recurrence; MECHANICAL GATE applies. Fix: story-writer S-19.07 v1.2→v1.3 — replace_all BC-4.13.001 v1.6 Phase-B → v1.7 Phase-B (all body cites); targeted replacement BC-4.13.001 v1.5 Invariant 10 → v1.7 Invariant 10 (2 stray sites); BC table + Token Budget + Architecture Compliance Rules updated; orchestrator post-fix grep confirms zero live stale cites.

F-P8-004 — MEDIUM — E-19 epic v1.7 Stories-table BCs cell for S-19.03 is incomplete: it lists only "BC-2.07.001" while S-19.03 also governs BC-2.02.011 (host::read_file NOT_FOUND semantics; HostError::NotFound named variant). Ground-truth verification: `grep -n "S-19\.03" .factory/stories/epics/E-19-post-rc22-operator-hardening.md | grep "BC-"` returns the Stories-table row for S-19.03 showing BCs cell = "BC-2.07.001" only. `grep -n "BC-2\.02\.011" .factory/stories/epics/E-19-post-rc22-operator-hardening.md | grep -v "Changelog\|BC Traceability"` confirms BC-2.02.011 appears in the §BC Traceability table body but was NOT propagated back to the Stories-table BCs cell. The §BC Traceability table at epic v1.7 has a BC-2.02.011 row (added at pass-5 via O-P5-002), but the Stories table (a different location) was not updated in the same sweep — a POLICY 14 parity site gap. Fix: story-writer E-19 epic v1.7→v1.8 — Stories-table S-19.03 BCs cell updated from "BC-2.07.001" to "BC-2.07.001, BC-2.02.011"; orchestrator grep confirms BC-2.02.011 appears in both the Stories-table BCs cell AND the §BC Traceability table for S-19.03.

F-P8-005 — MEDIUM — S-19.06 v1.3 AC-003 gate strips only leading-comment lines (lines whose first non-whitespace character is `//` or `#`) but does not strip trailing inline comments, making the gate susceptible to false-positive failures. Ground-truth verification: `grep -n "AC-003\|grep.*read_prefix\|grep.*sed" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -10` returns the AC-003 gate using `grep -vE '^\s*(//\|//!\|///)' crates/factory-dispatcher/src/host/read_prefix.rs | grep -qE "host::read_file|OUTPUT_TOO_LARGE"`. A Rust source line containing both a semantic call and a trailing comment — e.g., `// call site: host::read_file(...)` rendered as `let x = something_else(); // but note: host::read_file semantics` — would survive the `grep -vE '^\s*//`' pre-filter (it doesn't START with `//`) and then match the secondary grep. The gate as written will false-positive on trailing inline comments that merely mention the forbidden symbol. Fix: story-writer S-19.06 v1.3→v1.4 — AC-003 gate updated to strip trailing inline comments before the secondary grep: `sed 's://.*::' crates/factory-dispatcher/src/host/read_prefix.rs | grep -qE "host::read_file|OUTPUT_TOO_LARGE"` (the `sed 's://.*::'` removes everything from `//` to end-of-line before grepping, covering both leading and trailing comment forms).

F-P8-006 — MEDIUM — S-19.05 v1.6 EC-005 row retains the `[SYNTHETIC]` label in the EC-description text even though the row was recast from a synthetic-fixture-dependent test to a schema-level property/serialization test at pass-7 (F-P7-007; BC-3.08.001 v1.19 ruling). Ground-truth verification: `grep -n "\[SYNTHETIC\]\|SYNTHETIC" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | grep "EC-005"` returns the EC-005 row label or description text containing "[SYNTHETIC]" — the pass-7 story-writer rewrote the Gate column content but did not remove the "[SYNTHETIC]" label from the EC-005 row identifier or description prefix. The `[SYNTHETIC]` label was meaningful when EC-005 specified a runtime-fixture-dependent test; it is now misleading because the test is a compile-time serialization property check, not a runtime synthetic stimulus. Fix: story-writer S-19.05 v1.6→v1.7 — EC-005 row: remove `[SYNTHETIC]` label from row identifier/description prefix; update Expected Behavior column to explicitly state "verified via schema-level property/serialization tests (not runtime dispatch fixture) per BC-3.08.001 v1.19 Invariant 6."

F-P8-007 — MEDIUM — S-19.07 v1.2 AC-002 Gate (and related Architecture Mapping + Architecture Compliance Rules sections) refers to "BOTH verify-factory-lock entries" without naming them, leaving an implementer who hasn't read the full registry to guess which entries those are. Ground-truth verification: `grep -n "BOTH\|both.*entries\|both.*verify-factory-lock\|verify-factory-lock-bash" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md | head -10` returns gate text saying "BOTH verify-factory-lock entries" but not "verify-factory-lock AND verify-factory-lock-bash" — the second entry name (`verify-factory-lock-bash`) is absent from the AC-002 Gate body text, Architecture Mapping table, and Architecture Compliance Rules gate rows. An implementer updating only the entry named "verify-factory-lock" (the first match they find in the registry) would satisfy the prose and miss the bash variant. Fix: story-writer S-19.07 v1.2→v1.3 — enumerate both entry names explicitly in AC-002 Gate ("verify-factory-lock AND verify-factory-lock-bash"), Architecture Mapping section, and Architecture Compliance Rules gate rows; ensure the per-entry awk gates instituted for F-P8-001 also enumerate the bash-variant entry by name.

F-P8-008 — MEDIUM — S-19.02 v1.6 AC-006 boundary summary specifies "262145 → OUTPUT_TOO_LARGE (EC-002 fail-open)" as a normative boundary behavior but no T-NNN test row exercises the 262145-byte boundary to assert that `run_check()` returns `StateReadError` AND zero `state_md_approaching_cap` log entries. Ground-truth verification: `grep -n "T-[0-9]\{3\}\|262145" .factory/stories/S-19.02-verify-factory-lock-output-too-large.md | head -20` returns the AC-006 boundary table specifying 262145 behavior and the T-row table showing T-001..T-009 but no test row for the 262145-byte fixture case. The 262145-byte boundary is NOT exercised by any T-row: it is the single most adversarial boundary (one byte over the cap), its behavior differs from the 262144-byte case in that (a) StateReadError fires, (b) no soft-warning is emitted (the warn range is strictly (200000, 262144] — 262145 falls outside), and (c) fail-open semantics apply. All three behaviors MUST be asserted by a test. Fix: story-writer S-19.02 v1.6→v1.7 — AC-006 Gate updated to include "Unit test E: fixture STATE.md of 262145 bytes → `run_check()` returns `StateReadError` (OUTPUT_TOO_LARGE/fail-open) AND zero `state_md_approaching_cap` log entries emitted"; T-009 row added (Unit test E) to the T-row table; Task 7 implementation note updated to include Unit test E.

F-P8-009 — LOW — S-19.07 v1.2 AC-002 Gate (as written at v1.2 before F-P8-001 fix) uses a shell pattern with mixed quoting that may be brittle across shell implementations. Ground-truth verification: `grep -n "grep.*\\\\[hooks" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md | head -5` returns the gate pattern using a mix of single-quoted outer shell and escaped-backslash inner regex which, when expanded by a posix sh vs bash vs zsh interpreter, may produce different regex interpretations for `\[`. The standard form should use single quotes throughout with explicit bracket escaping. Non-blocking if the awk-scoped rewrite for F-P8-001 uses standard single-quoted awk programs; the quote-style issue is subsumed if that rewrite is done correctly. Fix: same pass — story-writer gate rewrite for F-P8-001 MUST use consistent single-quoted awk and grep patterns; verify the gate text as written is syntactically correct in POSIX sh.

F-P8-010 — LOW — S-19.05 v1.6 AC-001 gate asserts that the dispatcher emits a `plugin.completed` event but verifies only that the event is present, not that all 9 mandatory fields (per BC-3.08.001 v1.19 Event 6: type, timestamp, hook_id, tool, exit_code, duration_ms, source, plugin_version, entry_index) are present in the emitted JSON. Ground-truth verification: `grep -n "AC-001\|9.*field\|jq\|per-field" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -10` returns AC-001 gate text asserting event emission but using a single `jq 'select(.type == "plugin.completed")'` check — no per-field enumeration or per-field assertion. The BC mandates all 9 fields; a partial implementation that emits only type + timestamp would satisfy the current gate while violating BC-3.08.001 v1.19 Event 6. Fix: story-writer S-19.05 v1.6→v1.7 — AC-001 Gate updated to include a jq per-field loop asserting all 9 mandatory fields are present and non-null in the captured JSONL output: `jq -e '.type == "plugin.completed" and .timestamp != null and .hook_id != null and .tool != null and .exit_code != null and .duration_ms != null and .source != null and .plugin_version != null and .entry_index != null'`.

F-P8-011 — LOW — E-19 epic v1.7 §Wave Sequencing section lists W2 stories (S-19.04, S-19.05, S-19.06) as executable in parallel, but S-19.06 has `depends_on: [S-19.03, S-19.04]` — specifically depends on S-19.04 which is also W2. This creates an intra-wave serial dependency that contradicts the wave-model parallelism assumption. Ground-truth verification: `grep -n "W2\|wave.*2\|parallel\|depends_on" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -5` returns `depends_on: [S-19.03, S-19.04]`; `grep -n "W2.*parallel\|S-19\.04.*S-19\.05.*S-19\.06" .factory/stories/epics/E-19-post-rc22-operator-hardening.md | head -5` returns wave-sequencing text presenting W2 as a parallel batch. The epic does not note the S-19.04→S-19.06 intra-wave ordering constraint. An orchestrator dispatching W2 in parallel would attempt to run S-19.06 implementation before S-19.04 completes its host::read_prefix delivery, which S-19.06 depends on. Fix: story-writer E-19 epic v1.7→v1.8 — §Wave Sequencing or §Dependency Graph updated to note the W2 intra-wave ordering constraint (S-19.04 BEFORE S-19.06; S-19.04 and S-19.05 may run in parallel; S-19.06 runs after S-19.04 completes).

Observations:

O-P8-B-1 — [ADOPTED AS PREFLIGHT] BC-cite impact matrix should be produced by every fix-burst leg that bumps a BC, and independently verified by the orchestrator before dispatching each adversarial pass. The current cascade has experienced four recurrences (F-P4-002/003/009, F-P5-003, F-P6-003, F-P8-002/003) of the BC-bump→partial-propagation class, all attributable to replace_all missing non-canonical cite forms. A structured preflight — mapping each of the 6 E-19 BCs to every artifact that cites it, classifying live vs changelog-history cites — would have caught F-P8-002/003 before the pass-8 adversary run. ADOPTED: orchestrator to run the BC-cite preflight independently before dispatching every subsequent adversarial pass (see D-759 MECHANICAL GATE codification).

O-P8-B-2 — S-19.07 §O-P3-001 deferral gate (merge-commit-pattern) currently uses a broad pattern (`Merge.*branch\|Merge.*pull`) that would match any merge commit message, including non-release-merge commits in the factory-artifacts worktree history. A tighter pattern (`Merge pull request`) scoped to GitHub-style PR merge commits would reduce false-positive risk at the deferral gate. (Actioned in S-19.07 v1.3 fix burst; O-P8-B-2 encoded.)

O-P8-B-3 — S-19.05 EC-006 (negative control: zero abandoned events when all complete before drain) was referenced in pass-7 observations but not explicitly present as a test vector in S-19.05 v1.6. A dedicated EC-006 row with explicit expected behavior (zero `plugin.abandoned`; assert via `jq 'select(.type == "plugin.abandoned")' | wc -l` returns 0) would strengthen the AC-002 boundary coverage. (Actioned in S-19.05 v1.7 fix burst as O-P8-B-3 EC-006 negative-control row.)

O-P8-B-4 — S-19.01 §Wave and §Dependencies are consistent with W1 placement and no intra-wave ordering constraint. The AC-004 bash-3.2 mechanism test (closed F-P7-005) is the most mechanically precise gate in E-19 as of this pass — it serves as a model for the shell-dialect simulation discipline being adopted cascade-wide. (ACCEPTED-WITH-RECORD; non-blocking; no action required this pass.)

O-P8-B-5 — STORY-INDEX v4.138 BC-coverage summary was re-derived at pass-7 close but uses a static enumeration that will drift again on the next BC version bump. The orchestrator's BC-cite preflight (O-P8-B-1 adoption) substitutes for re-derivation automation at this stage. (Encoded in the preflight gate as preflight step (b); no additional action beyond the preflight institution.)

O-P8-B-6 — BC-4.13.001 v1.7 Invariant 10 boundary table (added at pass-7) is thorough for Phase-A semantics but does not cross-reference the Phase-B story (S-19.07) which structurally eliminates the soft-warn range (read_prefix never emits OutputTooLarge; the approaching-cap concept doesn't apply). A cross-reference note in BC-4.13.001 would prevent future adversaries from looking for Invariant 10 compliance in Phase-B code. (ACCEPTED-WITH-RECORD; out of scope for this cascade's story spec work; attach to S-19.07 implementer guidance if desired.)

---

## Verifications That PASSED

The following 12 structural checks were confirmed clean at pass-8 perimeter entry:

1. Bidirectional DAG parity PASS: all `depends_on` / `blocks` reciprocals verified for S-19.01..S-19.07 at pass-8 perimeter entry (F-P8-011 is a wave-model prose gap, not a DAG inconsistency).
2. BC-4.13.001 v1.7 boundary table completeness PASS: 200000/200001/262144 boundary behaviors correctly specified in S-19.02 v1.6 AC-006 (F-P8-008 fills the 262145 gap; the 3 implemented cases are correct).
3. POLICY 20 compliance PASS: S-19.04 v1.8 `release_bundle_no_dev_samples` row present in §Traceability; POLICY 17 residuals confirmed zero (F-P7-002 closure STANDS).
4. ADR/BC version matrix PASS: ADR-025 v1.9 / ADR-030 v1.0 / BC-5.42.001 v1.1 / BC-2.07.001 v1.2 / BC-1.17.001 v1.1 / BC-4.13.001 v1.7 / BC-3.08.001 v1.19 consistent with D-758 state.
5. S-19.03 Red Gate discipline PASS: stub constant -1000 (non-zero, out-of-band; guaranteed failing Red Gate).
6. STORY-INDEX totals PASS: story_count 130, E-19 45pts correctly reflect 7 stories.
7. VP-INDEX completeness PASS: VP-094..VP-101 + VP-079 all registered; ARCH-INDEX v2.89 unchanged.
8. BC-INDEX total_bcs PASS: 1,977 consistent; no orphan BCs.
9. S-19.06 dependency chain PASS: depends_on [S-19.03, S-19.04] correctly anchored; BC-1.17.001 v1.1 cited; SS-02 present (F-P8-011 is epic-prose gap only, not story-level defect).
10. BC-2.07.001 v1.2 EC-007 injectable-canonicalize seam PASS: testability seam present in BC; S-19.03 AC-001 negative-control B updated per ruling.
11. S-19.02 v1.6 T-rows completeness PASS: T-001..T-009 present with full inline content (F-P7-012 closure STANDS); F-P8-008 adds T-009 Unit test E gap only.
12. BC-3.08.001 v1.19 schema-level entry_index PASS: S-19.05 v1.6 AC-002 Gate recast to property/serialization test; EC-005 fixture-path cite removed (F-P7-007 closure STANDS; F-P8-006 is residual [SYNTHETIC] label only).

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 3 |
| MEDIUM | 5 |
| LOW | 3 |
| Observations | 6 |

*Actionable findings: 11 (F-P8-001..F-P8-011). F-P8-002 and F-P8-003 are the FOURTH recurrence of the BC-bump→partial-propagation class; MECHANICAL GATE codified at D-759.*

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Severity decay from pass 7 (enumerated):** B0/H2/M5/L5 (12 total) → B0/H3/M5/L3 (11 total; 3 HIGH reflect persistent process-gap class plus globally-unscoped gate; LOW reduction from 5 to 3 reflects consolidation; no re-found classes from closed findings; novelty score 1.0)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 8 |
| **New findings** | 11 (F-P8-001..F-P8-011) |
| **Duplicate/variant findings** | 0 (F-P8-002/003 are same class as prior recurrences but at different sites; counted as new findings per cascade convention) |
| **Novelty score** | 1.0 (11 / 11) |
| **Median severity** | MEDIUM |
| **Trajectory (findings per pass)** | 16 → 14 → 20 → 9 → 8 → 5 → 12 → 11 |
| **Verdict** | FINDINGS_REMAIN — pass 9 dispatched with fresh context |

**Note on pass-8 volume (11 vs pass-7's 12):** The slight reduction reflects consolidation at the MEDIUM-HIGH boundary — the globally-unscoped gate (F-P8-001) and the two process-gap BC-cite sites (F-P8-002/003) are HIGH individually but stem from one root class that the MECHANICAL GATE now addresses structurally. The 11-finding count is consistent with the asymptotic trajectory pattern (trajectory is not monotonically decreasing; volume fluctuates while finding classes shift deeper into correctness details).

---

## Fix-Burst Closure Section (D-759)

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. Same-burst fixes do NOT advance streak.

**All 11 findings closed. Orchestrator ran BC-cite drift preflight independently (O-P8-B-1 adoption) before declaring closure; ZERO stale live cites confirmed outside changelog sections — first fully cite-coherent state of the E-19 cascade.**

### Story-writer leg

- **S-19.01 v1.6→v1.7 (O-P8-B-4 carry):** No normative changes; version bump records pass-8 closure audit (O-P8-B-4 accepted-with-record; input-hash refreshed). Changelog entry added.

- **S-19.02 v1.6→v1.7 (F-P8-002 + F-P8-008):** (a) Targeted replacement of 2 reverse-word-order stale BC-4.13.001 v1.6 cites: Phase-A scope note → v1.7; Arch Rules table v1.5→v1.7 range cite endpoint → v1.7. (b) AC-006 Gate Unit test E added (262145-byte fixture → StateReadError + zero `state_md_approaching_cap` log entries); T-009 Unit test E row added; Task 7 Unit test E implementation note added. Orchestrator post-fix grep confirmed zero live BC-4.13.001 v1.6 or earlier cites outside changelog sections. Closes F-P8-002 + F-P8-008.

- **S-19.04 v1.8→v1.9 (audit carry):** No normative gap found at pass-8 for S-19.04; version bump records pass-8 citation-coherence audit. Changelog entry added.

- **S-19.05 v1.6→v1.7 (F-P8-006 + F-P8-010 + O-P8-B-3):** (a) EC-005 `[SYNTHETIC]` label dropped; Expected Behavior column updated to "verified via schema-level property/serialization tests per BC-3.08.001 v1.19 Invariant 6 (not runtime dispatch fixture)." (b) AC-001 Gate updated with jq per-field loop asserting all 9 mandatory fields present and non-null in JSONL output. (c) EC-006 negative-control row added: "Drain-timer fires with zero async plugins in-flight (all completed before timer fires) → zero `plugin.abandoned` events; assert `jq 'select(.type == "plugin.abandoned")' | wc -l` returns 0." Closes F-P8-006 + F-P8-010; O-P8-B-3 encoded.

- **S-19.06 v1.3→v1.4 (F-P8-005):** AC-003 gate updated: `sed 's://.*::'` trailing-comment strip added before secondary grep, replacing `grep -vE '^\s*(//|//!|///)` leading-only comment filter — now correctly strips both leading and trailing inline comment forms before asserting absence of forbidden symbols. Closes F-P8-005.

- **S-19.07 v1.2→v1.3 (F-P8-001 + F-P8-002/003 + F-P8-007 + F-P8-009 + O-P8-B-2):** (a) AC-002 Gate rewritten with per-entry awk scoping for each of `verify-factory-lock` and `verify-factory-lock-bash`; single-quoted patterns throughout (F-P8-001 + F-P8-009). (b) Both entry names enumerated explicitly in AC-002 Gate, Architecture Mapping, and Architecture Compliance Rules (F-P8-007). (c) replace_all BC-4.13.001 v1.6 Phase-B → v1.7 Phase-B (all body cites); targeted replacement BC-4.13.001 v1.5 Invariant 10 → v1.7 Invariant 10 (2 stray sites in Architecture Mapping + Previous Story Intel) (F-P8-002/003). (d) Deferral gate grep tightened to `Merge pull request` pattern at 3 sites: O-P3-001 box, Previous Story Intel, Architecture Compliance Rules (O-P8-B-2 encoded). Closes F-P8-001 + F-P8-002/003 story leg + F-P8-007 + F-P8-009; O-P8-B-2 encoded.

- **E-19 epic v1.7→v1.8 (F-P8-004 + F-P8-011):** (a) Stories-table S-19.03 BCs cell updated from "BC-2.07.001" to "BC-2.07.001, BC-2.02.011" (F-P8-004). (b) §Wave Sequencing / §Dependency Graph updated to note W2 intra-wave ordering constraint: "S-19.04 MUST complete before S-19.06 (S-19.06 depends_on S-19.04); S-19.04 and S-19.05 may be dispatched in parallel; S-19.05 and S-19.06 may be dispatched in parallel after S-19.04 completes" (F-P8-011). Orchestrator grep confirms BC-2.02.011 appears in both Stories-table BCs cell and §BC Traceability table for S-19.03. Closes F-P8-004 + F-P8-011.

- **STORY-INDEX v4.138→v4.139 (cite coherence):** BC-cite drift preflight run at STORY-INDEX level — post-fix grep confirmed all E-19 epic + story BC version cites are coherent. STORY-INDEX narrative quad updated to reflect v4.139 per POLICY 14 parity. Changelog entry added.

### 6-BC × 9-Artifact Cite Matrix (D-759 closure; live BC versions confirmed via `grep "^version:"` at commit time)

Live BC versions at D-759 closure:
- BC-4.13.001: v1.7 (`grep "^version:" .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md`)
- BC-2.07.001: v1.2 (`grep "^version:" .factory/specs/behavioral-contracts/ss-02/BC-2.07.001.md`)
- BC-2.02.011: v1.4 (`grep "^version:" .factory/specs/behavioral-contracts/ss-02/BC-2.02.011.md`)
- BC-3.08.001: v1.19 (`grep "^version:" .factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md`)
- BC-5.42.001: v1.1 (`grep "^version:" .factory/specs/behavioral-contracts/ss-05/BC-5.42.001.md`)
- BC-1.17.001: v1.1 (`grep "^version:" .factory/specs/behavioral-contracts/ss-01/BC-1.17.001.md`)

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

*Orchestrator independently verified ZERO stale live cites outside changelog sections across all 9 artifacts. First fully cite-coherent state of the E-19 cascade.*

### Artifact versions at pass-8 closure

| Artifact | Version |
|----------|---------|
| BC-INDEX | v3.74 (UNCHANGED — no BC amendments this pass) |
| VP-INDEX | v2.53 (UNCHANGED) |
| ARCH-INDEX | v2.89 (UNCHANGED) |
| BC-4.13.001 | v1.7 (UNCHANGED) |
| BC-2.07.001 | v1.2 (UNCHANGED) |
| BC-3.08.001 | v1.19 (UNCHANGED) |
| S-19.01 | v1.7 |
| S-19.02 | v1.7 |
| S-19.03 | v1.7 (UNCHANGED this pass) |
| S-19.04 | v1.9 |
| S-19.05 | v1.7 |
| S-19.06 | v1.4 |
| S-19.07 | v1.3 |
| E-19 epic | v1.8 |
| STORY-INDEX | v4.139 |

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. **NEXT:** E-19 adversarial pass-9 (fresh context; 20-policy rubric; trajectory 16→14→20→9→8→5→12→11→pass-9).
