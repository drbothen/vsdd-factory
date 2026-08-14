---
document_type: behavioral-contract
level: L3
version: "1.22"
status: draft
producer: product-owner
timestamp: 2026-07-30T00:00:00Z
phase: v1.0-feature-engine-discipline-pass-1
cycle: v1.0-feature-engine-discipline-pass-1
inputs:
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md
  - .factory/policies.yaml
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-28.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-29.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-30.md
  - .factory/cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-1.md
  - .factory/cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-10.md
input-hash: "3dc02af"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
extracted_from: null
origin: brownfield
subsystem: "SS-05"
capability: "E-12"
lifecycle_status: draft
introduced: v1.0-feature-engine-discipline-pass-1
modified:
  - "2026-07-30"
  - "2026-07-30 (v1.1)"
  - "2026-07-30 (v1.2)"
  - "2026-07-30 (v1.3)"
  - "2026-08-03 (v1.4)"
  - "2026-08-04 (v1.5)"
  - "2026-08-04 (v1.6)"
  - "2026-08-04 (v1.7)"
  - "2026-08-04 (v1.8)"
  - "2026-08-04 (v1.9)"
  - "2026-08-05 (v1.10)"
  - "2026-08-05 (v1.11)"
  - "2026-08-05 (v1.12)"
  - "2026-08-06 (v1.13)"
  - "2026-08-07 (v1.14)"
  - "2026-08-08 (v1.15)"
  - "2026-08-08 (v1.16)"
  - "2026-08-08 (v1.17)"
  - "2026-08-13 (v1.18)"
  - "2026-08-13 (v1.19)"
  - "2026-08-14 (v1.20)"
  - "2026-08-14 (v1.21)"
  - "2026-08-14 (v1.22)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.010
section: "5.39"
last_amended: |-
  2026-08-14 (v1.22) — ADV-RECON11-001 (LOW, near-zero-reachability) architect-adjudicated
  amendment: "Secondary Index-File UTF-8 Decode Failure" clause added, DISTINCT from precondition
  15a / postcondition 25 (which govern PRIMARY-target decode failure only). Gap: BC-INDEX.md
  (Arm A1 secondary read) and STORY-INDEX.md (Arm B1 secondary read only — NOT Arm B2, where
  STORY-INDEX.md is the primary target already covered by 15a/25) had no specified UTF-8 decode
  disposition. A non-UTF-8 BC-INDEX silently degraded to `RowAbsent`, which — for any BC with
  frontmatter `version:` > "1.0" — triggers postcondition 4's BLOCK with a MISLEADING message
  ("dropped registration"; actual root cause is index-file corruption, not a missing row). A
  non-UTF-8 STORY-INDEX, read at Arm B1 (not Arm B2), silently degraded to `(None, None)`,
  triggering postcondition 12's fail-open "not yet registered" advisory — silently disabling
  three-way hash checking with no disclosure of the decode failure. Same diagnostic-accuracy
  defect class as F-S2107-P1B-016 (fixed for Arm A2's secondary BC-FILE read), applied
  asymmetrically to Arm A1/B1's secondary INDEX-FILE reads. New precondition 15b ("Secondary
  Index-File UTF-8 Decode Failure") added in a new subsection immediately following precondition
  15a, scoped to Arm A1's BC-INDEX.md secondary read and Arm B1's STORY-INDEX.md secondary read
  (Arm B2 explicitly excluded — already primary-target-governed). New postcondition 26 added:
  MUST emit a distinct advisory naming the index-file path and stating the row/hash state is
  INDETERMINATE (not confirmed-absent) — prescribed verbatim message: "validate-cross-site-
  correspondence: <index-file> failed UTF-8 decode — row/hash state for '<id>' is INDETERMINATE,
  not confirmed-absent. Fix: verify the index file's encoding and re-save as UTF-8." — and MUST
  NOT fall through into `RowAbsent` (Arm A1) or `(None, None)`/"not yet registered" (Arm B1).
  Disposition is ADVISORY (Continue), not block — preserves the low-disruption posture
  appropriate to this LOW/near-zero-reachability gap, deliberately lower severity than
  precondition 15a / postcondition 25's primary-target BLOCK. New EC-040 added (combined Arm A1
  + Arm B1 scenario). New Canonical Test Vector row added ("Secondary Index-File UTF-8 Decode
  Failure", covering both arms' correct advisory dispositions against the two false-fallthrough
  mutants). Invariant 5 ("Selective fail-open for secondary targets on NotFound only") extended
  with a parenthetical clarifying its title no longer fully describes secondary-target
  disposition scope as of v1.22 — UTF-8 decode failure on a secondary target is a distinct,
  non-`HostError`, non-NotFound case now governed by precondition 15b / postcondition 26,
  disposed identically (advisory + Continue) but for a different reason (indeterminate state,
  not legitimate bootstrap absence). No BC H1 title change (POLICY 7): the title's promise is
  unaffected; this is read-failure-disposition hardening within the existing scope, mirroring
  the v1.20 primary-target amendment's framing. No renumbering of any existing ID (POLICY 1
  append-only): precondition 15b, postcondition 26, EC-040, and the new test-vector row are all
  net-new appended IDs. BC-INDEX, the S-21.07 story, and the implementation are NOT amended in
  this burst — BC-INDEX version-cell sync is owed to state-manager; S-21.07 story
  BC-table/Token Budget cite + new AC are owed to story-writer; the `extract_bc_index_version_state`
  function's `std::str::from_utf8(index_content).unwrap_or("")` fallback (arm_a1.rs) and the
  `parse_story_index_catalog_hash` / `parse_story_index_blockquote_hash` functions' `.ok()?`
  fallbacks (arm_b.rs) — all of which currently silently discard UTF-8 decode errors — are owed
  to implementer for the new IndexUnreadable-disposition wiring; a RED-gate unit test exercising
  both the Arm A1 non-UTF-8-BC-INDEX and Arm B1 non-UTF-8-STORY-INDEX scenarios is owed to
  test-writer. (product-owner; architect ruling on ADV-RECON11-001; closes ADV-RECON11-001.)
  [Prior: 2026-08-14 (v1.21) — ADV-RECON5-003 (MEDIUM) product-owner adjudication amendment, human-approved
  "Amend now (v1.21)": PC13 Phase 2 same-field scan-stop added. Corpus-confirmed reachable
  (2026-08-14, not hypothetical): `.factory/stories/S-4.08-rc1-release-gate.md`'s `## Behavioral
  Contracts` row for BC-9.01.002 — Trace cell mentions BC-9.01.001 (present in this story's
  `behavioral_contracts:` frontmatter array) then, later in the SAME field, an unrelated "v1.1 BC
  candidate" phrase — the v1.20 field-scoped-only Phase 2 algorithm still extracted "v1.1" as
  BC-9.01.001's citation despite its actual frontmatter version being "1.0", a live false BLOCK,
  not hypothetical. Phase 2 now MUST terminate the forward v-token scan (without producing a
  version) the moment it encounters a DIFFERENT `BC-S.SS.NNN` token before any qualifying v-token,
  then fall through to the next pipe-delimited field per the existing per-field fallback.
  §Preconditions PC13 Phase 2 correction paragraph added, clarifying that field-scoping alone
  (v1.19 / ADR-038 §Decision 5) did not actually resolve the S-4.08 case its own text cited as the
  motivating example. New EC-039 + new Canonical Test Vector row added (Phase 2 same-field
  scan-stop). No BC H1 title change (POLICY 7): the title's promise — blocking on stale cross-site
  version-cites — is unaffected; this is extraction-correctness hardening within existing scope.
  No renumbering of any existing ID (POLICY 1 append-only): PC13 Phase 2 text amended in place
  (correction + scan-stop paragraphs appended after the existing reverse-field-NON-CONFORMING
  paragraph); EC-039 is a net-new appended ID. BC-INDEX, the S-21.07 story, and the
  `find_phase2_version` function in
  `crates/hook-plugins/validate-cross-site-correspondence/src/arm_a2.rs` are NOT amended in this
  burst — BC-INDEX version-cell sync is owed to state-manager; S-21.07 story BC-table/Token Budget
  cite is owed to story-writer; the `find_phase2_version` scan-stop implementation is owed to
  implementer; a RED-gate unit test exercising the S-4.08 BC-9.01.002 row shape is owed to
  test-writer. (product-owner; closes ADV-RECON5-003.) [Prior: 2026-08-14 (v1.20) — Two spec-correctness amendments from fresh-context adversarial review
  ADV-RECON-003 (MEDIUM) and ADV-RECON-007 (LOW); human approved "fix both now" (production-grade
  default; no tech-debt-register deferral — neither item has a genuine blocking dependency, unlike
  S-21.08's Class D, which explicitly depends on Closes/Refs convention standardization; S-21.08 is
  therefore the WRONG anchor for either item, and both are amended in place here instead). Amendment
  1 (ADV-RECON-003): §Preconditions PC13 Phase 1 (pure-version-field extraction) rearchitected from
  an unanchored full-row scan to BC-ID-anchored: a row is now eligible for Phase 1 extraction for BC
  ID X only if the row's FIRST non-empty pipe-delimited field contains X at a word boundary (same
  locator-predicate test already normative for Part A Arm1 and for Phase 2) — closing a residual
  hazard where a multi-BC `## Behavioral Contracts` table row (a supersession/refinement/VP-mapping
  note naming a second BC in the same row) could cause Phase 1 to resolve the WRONG BC's Version
  cell, producing a false BLOCK (versions differ) or a false PASS (coincidental match). Corpus-
  confirmed reachable, not hypothetical (2026-08-14, `.factory/stories/*.md`): 14 of 480 examined
  `## Behavioral Contracts` body-table rows name more than one distinct BC ID within a single row
  (S-4.06, S-4.08, S-2.01, S-21.11, S-8.04, S-5.01, S-6.01, S-5.02). New EC-037 + new Canonical Test
  Vector row added (cross-BC-row Phase 1 resolution). Gate Spec `run_part_a_arm2` Phase 1 comment
  updated to match, per TD-VSDD-060 sibling-site sweep — the pseudocode would otherwise contradict
  the amended normative text one paragraph away in the same document. Amendment 2 (ADV-RECON-007):
  invariant 4 (fail-closed for primary targets) extended to cover primary-target reads that succeed
  as bytes but fail UTF-8 decoding — previously undefined by this BC. The shipped implementation's
  primary-target UTF-8 decode step (post-primary-read, pre-arm-dispatch, in the hook's `run`
  entrypoint) filled that silence by citing invariant 9 ("is_char_boundary() slicing safety") as
  authorization for `HookResult::Continue` on decode failure; invariant 9 does not actually grant
  this — it governs slicing of already-decoded strings only. New precondition 15a ("Primary Target
  UTF-8 Decode Failure — applies to all arms") and new postcondition 25 normatively require
  `HookResult::block_with_fix(...)` on primary-target decode failure, under the same fail-closed
  posture as invariant 4, and explicitly name the invariant-9 misattribution as NON-CONFORMING to
  prevent recurrence. New EC-038 added. No BC H1 title change (POLICY 7): the title's promise —
  blocking on stale cross-site version-cites — is unaffected by either amendment; both are
  extraction-correctness and read-failure-disposition hardening within the existing scope. No
  renumbering of any existing ID (POLICY 1 append-only): PC13 Phase 1 text amended in place;
  precondition 15a and postcondition 25 are net-new appended IDs; EC-037/EC-038 are net-new appended
  IDs. VP files, BC-INDEX, and the S-21.07 story are NOT amended in this burst — BC-INDEX version
  cell sync is owed to state-manager; story BC-table/Token Budget cite + new ACs are owed to
  story-writer. (product-owner; closes ADV-RECON-003, ADV-RECON-007.) [Prior: 2026-08-13 (v1.19) — §VP Anchors count-reconciliation (S-21.07 pass-18, F-S2107-P18-001,
  MEDIUM — POLICY 5 category-(i) + POLICY 4): §VP Anchors stated "VP-102 through VP-118
  (17 VPs) are planned for this story per D-945" while §Verification Properties enumerates 19
  rows (Class A: 8; Class B: 4; Class D: 3 DEFERRED; Class E: 4) — the count grew 17→19 at
  v1.12, when two advisory-path property rows ("A Arm1 Primary-Newer-than-Index Advisory
  (PC2a)" and "B Arm1 STORY-INDEX-Consistent Advisory (PC13a)") were added per F-P6-001
  Option 1, but §VP Anchors was never swept (TD-VSDD-060 sibling-site gap). Cross-checked
  against S-21.07's own Token Budget, which already correctly cites "19 VPs" — 19 is
  canonical; §VP Anchors was the sole stale site. §VP Anchors corrected to "VP-102 through
  VP-120 (19 VPs)": all 19 enumerated rows, including the two v1.12 advisory-path additions,
  are first-class verification properties eligible for the same state-manager post-merge
  VP-INDEX allocation as the original 17 — neither advisory row is excluded from allocation,
  so the reserved range is expanded rather than split into an allocatable/advisory-only
  accounting. D-945 basis reference preserved. Full BC count-parity sweep performed per
  TD-VSDD-060 (production-grade default; no sibling defer): §Edge Cases table confirmed at
  36 rows (EC-001 through EC-036), matching S-21.07's Token Budget "36 ECs" cite — no change
  needed. This BC carries no Acceptance Criteria (ACs) section of its own — ACs are
  story-scoped per VSDD convention, not BC-scoped — so no BC-side "N ACs" count claim exists
  to reconcile against S-21.07's "24 ACs" cite; the sole AC-related text in this BC
  (§Preconditions Part A Arm2, the corpus-example citation "the Story ACs column contains
  `AC-001 through AC-021 (AC-012/013/014 DEFERRED v1.6 — Class D)`") is a fixed historical
  illustration of the PC13 ACs-column-collision hazard, not a current total-count assertion,
  and is left unchanged. §Verification Properties Class sub-counts (A:8 / B:4 / D:3 / E:4,
  re-derived by direct table enumeration) are asserted nowhere else in this BC and required
  no correction. No behavioral or normative change: §Verification Properties table rows,
  §Postconditions, §Preconditions, and §Edge Cases entries are all unchanged — this is a
  count/reservation-statement correction only. (product-owner; closes F-S2107-P18-001.)
  [Prior: 2026-08-13 (v1.18) — §Gate Spec fuel-cap present-perfect claim corrected (S-21.07 pass-10,
  F-S2107-P10-004, HIGH): v1.16/v1.17 asserted in present perfect that the cap "has been raised
  to 20,000,000" and the `≤ 12,000,000` margin gate was "satisfiable at HEAD." Verified FALSE at
  the review's frozen snapshot (`feature/S-21.07` @ `5370db80`): literal-shell grep confirmed
  both `RegistryDefaults::default()` and `InvokeLimits::default()` still returned
  `fuel_cap: 10_000_000` there. Because `fuel_consumed` is clamped to the operative cap and every
  observed exhaustion trap reports `fuel_consumed == cap`, a 10,000,000 cap made
  `fuel_consumed ≤ 12,000,000` true on every reachable path (completion or exhaustion) purely
  because `10,000,000 < 12,000,000` — the gate was tautological, not merely early, at that
  snapshot. Stale-baseline re-verification (production-grade default: verify before fixing) found
  PR #774 (commit `62fbcf1a`, "raise WASM fuel cap 10M→20M + fuel-vs-epoch block_reason
  disambiguation"; ADR-042 §Decision 1) has since merged to `develop` and is confirmed (via
  `git merge-base --is-ancestor`) an ancestor of both `origin/develop` and this repository's
  working HEAD: `DEFAULT_FUEL_CAP = 20_000_000` and both defaults source it (no duplicated
  literal), pinned by a `fuel_cap_defaults_stay_in_sync` regression test. The present-perfect
  claim is now TRUE at source-HEAD. Source-HEAD-vs-operator-effective distinction added: the
  marketplace-cached dispatcher binary this project's own hooks actually run remains pinned to
  the pre-raise 10,000,000 cap through v1.0.0-rc.23; the 20,000,000 cap is not operator-effective
  for this hook's own PostToolUse enforcement until v1.0.0-rc.24 is consumed — until then the
  margin gate remains tautological in live use despite being fixed at source. Margin-gate framing
  rewritten to state the precise, non-tautological failure condition at the 20,000,000 cap
  (`fuel_consumed` in `(12,000,000, 20,000,000)`: hook completes successfully but assertion
  fails — a reachable, 8,000,000-fuel-wide region, contrasted with the empty failing region at
  the former 10,000,000 cap) rather than merely asserting the gate is meaningful. No BC-table
  row content change (title/priority/status unaffected); Gate Specifications §fuel-cap
  verification, §Margin gate, and §Operational consequence prose amended only. (product-owner;
  closes F-S2107-P10-004.) [Prior: 2026-08-08 (v1.17) — §Gate Spec placement-sensitivity correction (ADR-042): retracted
  v1.15-erratum "falsified" characterization and v1.16 "two compounding errors / ~22× overestimate"
  claim — the ~110-row figure was derived from an append-scenario measurement against the former
  10M cap and is correct for that scenario. Two distinct runway scenarios documented: append
  (~146 rows at 10M cap, ~541 fuel/row; ~18,600 rows at 20M cap) and insert-before (~4–5 rows at
  10M cap, ~17,114 fuel/row avg; ~589 rows at 20M cap); ratio ~29×. Margin gate `≤ 12,000,000`
  rationale restated with explicit governing scenario (insert-before, adversarial). STORY-INDEX
  cite corrected from v4.290 to v4.291 (stale from parallel dispatch). Characterization errors
  originated with orchestrator analysis, not product-owner authoring; v1.17 supersedes both the
  v1.15-erratum and v1.16 characterizations. 4-index: BC-INDEX v4.55 / VP-INDEX v2.76 /
  STORY-INDEX v4.291 / ARCH-INDEX v3.51. (product-owner.) [Prior: 2026-08-08 (v1.16) — §Gate Spec fuel corrections (ADR-042, F-S2107-P9-002): global cap raised
  10M→20M (ADR-042 §Decision 1); margin gate updated fuel_consumed ≤ 12,000,000 (60% of 20M cap;
  satisfiable at HEAD: current consumption 9,920,913 < 12,000,000; ~121 SS-05 rows before gate
  fires, ~468 rows before exhaustion at 20M); ~110-row runway figure corrected — true runway 4
  rows safe, 5th exhausts for SS-05-sized entries (~486 bytes/row); early-return scope qualifier
  documented (extract_bc_index_version_state returns at row 921; only rows 1–920 scanned; rows
  beyond cost nothing); two compounding errors documented (scan-region mismatch: used total 1,985
  rows instead of 921 pre-BC-5.39.010 rows; row-size mismatch: used ~155 bytes/row average instead
  of ~486 bytes/row for SS-05 entries; combined ~22× overestimate); ERRATUM block removed and
  measurement integrated into normative text; fuel_cap prohibition lifted per ADR-042 §Decision 2
  — SHOULD set per-plugin cap once ADR-039 Phase 1 ships (p99×1.5, min 50M for Phase 3 fail-closed
  annotation); exhaustion visibility requirement re-scoped to ADR-042 §Decision 3 class (a):
  observable signaling (fuel_exhausted=true in dispatcher stderr + advisories[] entry), not on_error
  escalation; pending-implementation obligation stated (not a spec gap); Changelog v1.15-erratum row
  preserved; 4-index: BC-INDEX v4.54 / VP-INDEX v2.76 / STORY-INDEX v4.291 / ARCH-INDEX v3.51.
  (product-owner; closes F-S2107-P9-002.) [Prior: 2026-08-08 (v1.15) — §Gate Spec fuel-exhaustion section corrected (F-S2107-P9-002): false calibration claim retracted — "calibrated to bound reads inside the fuel budget at current artifact sizes" is false at HEAD (1 MiB PC4 cap is ~76% above the ~594 KB exhaustion point); measured fuel 9,920,913/10,000,000 (99.21% consumed) at BC-INDEX 576,396 bytes/1,985 rows (adversary-pass-9 at 0a6c8fda; verbatim stdout in §SDK Grounding Evidence); headroom 79,087 fuel (0.79%); exhaustion threshold 593,525–601,548 bytes (~110 additional BC-INDEX rows, ~3–4% above current corpus); normative requirement added: production-scale bats gate MUST assert fuel_consumed ≤ 60% of cap (margin gate, fires on approach) AND fixture BC-INDEX sha256 or row-count MUST track live file (drift gate); fuel_cap paragraph updated: evidentiary precondition now confirmed — prohibition retained per human ruling 2026-08-08, operator chose margin-gate over cap increase, fuel_cap remains unauthorized; operational consequence stated: exhaustion expected within ~110 rows of ordinary registration traffic, now surfaces loudly via margin gate. (product-owner; closes F-S2107-P9-002.) [Prior: 2026-08-07 (v1.14) — PC5 corpus figures corrected (three stale claims falsified by same burst's leg D, which registered two BCs and escaped BC-4.13.001 annotation pipes before leg C's spec was re-measured): RowPresentNoVersion 1,943/1,983 → 1,945/1,985 (structural invariant: RowPresentNoVersion count = total_candidates − n≥6_count; verbatim re-derivation stdout in §SDK Grounding Evidence); old-algorithm non-conformance two-of-four (not three): BC-3.08.001 oldalg 1.23 ≠ first-of-last 1.24 [differs]; BC-7.03.079 oldalg 1.4 ≠ first-of-last 1.5 [differs]; BC-4.13.001 oldalg 1.18 = first-of-last 1.18 [same, pipe-escaping in leg D eliminated the diff]; BC-5.24.006 1.3 = 1.3 [same]. fields[5..].join("|") reassembly arm adjudicated DEFENSIVE: zero live n>6 rows at 2026-08-07; step retained for future bare-pipe annotation rows; pinning test needed (reported to test-writer). PC40 77→76 at two sites; ADR-037 v1.2 version pins replaced with ADR-037 §Context anchor form per TD-VSDD-091. (product-owner; closes F-S2107-P8-005, F-S2107-P8-010.) [Prior: 2026-08-06 (v1.13) — PC5/PC6 version-chain extraction algorithm replaced: rightmost-of-field[5] → first-token-of-last-chain-entry + fields[5..].join("|") reassembly per ADR-038 §Decisions 1/4; corpus figures re-verified at 2026-08-06 (5-field: 1943 / 6-field: 39 / 9-field: 1 (BC-4.13.001) / total: 1983 / n≥6: 40; five-field and total confirmed unchanged from v1.8 measurement; six-field refined to 39 six-field + 1 nine-field); "The 6th field is the version-chain cell" removed (inaccurate for bare-pipe rows per ADR-038 §Empirical Measurement); rightmost-of-field[5] marked NON-CONFORMING in PC5 and PC6. PC13c added (ADR-038 §Decision 3): half-present case — exactly one of {B2, B3} present and differing from B1; other absent — dispositioned as advisory + Continue per PC12 inclusive-or; implementation MUST NOT block for (Some(b2), None) or (None, Some(b3)); live instances S-18.11/S-18.12. Gate Spec run_part_b_arm1 extended with PC13c bullet. Architecture Anchor for extract_bc_index_version_state updated with first-token-of-last-chain-entry algorithm note. EC-036 added (half-present advisory + Continue). Test vector added (PC13c advisory + Continue). (product-owner; ADR-038 §Decision 4 routing.) PC13 Phase 2 algorithm replaced (ADR-038 §Decision 5 / §Decision 4 Change 5): reverse-field (rightmost-first) → BC-ID-anchored first-v-token (locate anchor field by word-boundary BC ID match; first v-token after BC ID in that field); corpus count 30 rows (2026-08-04, stale) → 67 Phase 2 rows / 44 with BC IDs (2026-08-06); reverse-field NON-CONFORMING per ADR-038 §Decision 5; Architecture Anchor for extract_story_bc_version_citations and Gate Spec pseudocode updated; Phase 1 corpus date updated (2026-08-04 → 2026-08-06; count 58 unchanged per ADR-038 v1.1). Refs: ADR-038 §Decision 5. (product-owner; ADR-038 §Decision 4 Change 5.) [Prior: 2026-08-05 (v1.12) — EC table aligned with v1.11 normative postconditions (F-P6-001 Option 1 propagation; story-writer routing): EC-002 corrected (primary-newer-than-index direction → advisory + Continue per PC2a; prior text erroneously prescribed Block for this direction); EC-005 description updated (block trigger is B2≠B3 per PC13b, not B2≠B1 — parenthetical was semantically incorrect under v1.11); EC-019 updated (Class A Arm1 primary-newer direction is advisory per PC2a, not a block violation; cross-arm output is advisory + E1 block, not combined-block); EC-034 added (PC13a advisory path: B2==B3 AND B1≠B2 → advisory + Continue; state-manager STORY-INDEX update pending; no internal inconsistency); EC-035 added (PC2b block path: index-newer-than-primary → Block; anomalous direction). Canonical Test Vector "A Arm1 — stale" corrected (advisory + Continue per PC2a; mutant extended with PC2b block case). Gate Spec §Parts B, D, E run_part_b_arm1 prose extended: v1.11 PC13a/13b split described inline (prior prose described only the PC40/v1.5 volatile-check change, leaving the v1.11 behavioral change undescribed). VP table updated: "A Arm1 Stale-Index Block" renamed to "A Arm1 Index-Newer-than-Primary Block (PC2b)" to match the actual PC2b condition; "A Arm1 Primary-Newer-than-Index Advisory (PC2a)" and "B Arm1 STORY-INDEX-Consistent Advisory (PC13a)" added as new pending VP entries. (product-owner; F-P6-001 Option 1 propagation.) [Prior: 2026-08-05 (v1.11) — PC2 directional carve-out (F-P6-001 human-approved Option 1): primary-newer-than-index direction downgraded to advisory ("primary newer than index; state-manager index update pending; Class A BLOCK suspended"); index-newer-than-primary retains BLOCK (anomalous; no POLICY 3 ordering explanation). PC13 two sub-cases (F-P6-001): B2==B3 AND B1≠B2 → advisory (POLICY 3 ordering artefact; STORY-INDEX internally consistent); B2≠B3 → retains BLOCK (STORY-INDEX internal inconsistency). Both carve-outs mirror PC3/PC12 POLICY 3 rationale. PC4a verbatim assertion strengthened (F-P6-002): test-writer MUST assert COMPLETE formatted string by equality check; .contains()-only on substrings is NON-CONFORMING. PC5 ≥6-field no-v-token state defined normatively (F-P6-018): ≥6 non-empty fields AND no \bv([0-9]+\.[0-9]+)\b in field 6 → RowPresentNoVersion; non-empty counting confirmed canonical; empty cell → counts as 5 non-empty → RowPresentNoVersion, field 6 unread. PC36 corpus updated 2→3 (F-P6-012): ADR-037 adopted last_amended: |- in v1.1 amendment; grep confirmed 3 files. PC40 transitional clause corrected (F-P6-007): both prior preconditions satisfied in pass-5 burst (S-21.07 added to ADR-037 §Context; ARCH-INDEX.md removed from S-21.07 inputs:); PC40 vacuous for S-21.07; "no permanent weakening" guarantee holds; "widening" characterization of ARCH-INDEX.md in is_volatile_path WITHDRAWN — pattern 6 is normative conformance per ADR-037 §Decision 2; 77 stories remain in scope. Architecture Anchors corrected (F-P6-004): extract_bc_index_version_state replaces extract_bc_index_version (wrapper deleted by F-P4-016 in pass-5 burst; spec named the deleted symbol); is_volatile_path(path: &str) -> bool + parse_story_volatile_inputs(content: &str) -> Vec<String> replace check_volatile_inputs (never existed; actual shape is two public fns + inline check in run_arm_b1). §Traceability Stories + §Story Anchor: v1.4 in flight → v1.5 in flight (F-P6-005). §VP Anchors: v1.1 Changelog erratum added — "VP table extended to 17 entries" was a planning annotation; VP-102..VP-118 anchored to S-21.07 post-merge per D-945; state-manager allocates at post-merge burst; no VP table was written into BC body. Gate Spec pseudocode: PC2a/PC2b split in run_part_a_arm1 Version arm; PC13a/PC13b split in B1 check; check_volatile_inputs reference replaced with is_volatile_path + parse_story_volatile_inputs. (product-owner; S-21.07 pass-6 fix burst.) [Prior: 2026-08-05 (v1.10) — PC5 self-contradiction fixed (F-S2107-P4-022): split into two-level locator/body-table predicates; RowMalformed redefined as "locator-matched line (conditions (1)+(2) satisfied) with <5 fields after escape-aware split" — eliminating the contradiction where the normative three-condition candidacy predicate made RowMalformed empty by construction. PC5 candidate-selection order added (F-S2107-P4-005): full-file scan must prefer first (1)+(2)+(3)-satisfying line; RowMalformed only when ALL locator-matched lines fail (3); first-match-wins implementation is NON-CONFORMING. Postcondition 4a pinned normatively (F-S2107-P4-025): prescribed advisory text is MUST-verbatim with <id>/<N> as only substitutions; omitting "Registration status cannot be determined" and "Verify BC-INDEX body-table registration manually" is NON-CONFORMING. Postcondition 13 expanded with three-category enumeration (F-S2107-P4-006 ruling): hook MUST enumerate stale/fabricated/algorithm-divergent as possible explanations without asserting which applies; classify_provenance heuristic picking one label is NON-CONFORMING; invariant 11 governs; AC-009 stops requiring classification. Invariant 11 SHOULD→MUST. Postcondition 22 prescribed message added citing POLICY 14 leg 3 (F-S2107-P4-008 sibling sweep: postconditions 2 and 7 both cite leg 5 correctly; postcondition 20 cites leg 4 correctly; gap was postcondition 22 only). PC36 block-scalar normative requirement added (F-S2107-P4-004 coupling): extract_frontmatter_field MUST handle |- block scalars; returning "|-" is NON-CONFORMING; corpus 2 occurrences (this BC + S-21.07 story); load-bearing for Class E1 enforcement on governing artifacts. PC40 transitional clause updated (F-S2107-P4-013 BC-side ruling): "no permanent weakening" guarantee requires exhaustive ADR-037 §Context enumeration; S-21.07 absent from 19-story table; correction routed to architect; story-writer removes volatile ARCH-INDEX.md input. Gate Spec pseudocode updated for two-level predicates. Architecture Anchors: extract_bc_index_version updated v1.9→v1.10 terminology; extract_frontmatter_field block-scalar requirement added. Story Anchor and §Traceability Stories TBD→S-21.07. (product-owner; pass-5 fix burst.) [Prior: 2026-08-04 (v1.9) — PC5 fourth state `RowMalformed`: a candidate line matching the locator pattern (`^\| \[<id>\]` or `^\| <id> \|`) was found but has <5 non-empty fields after escape-aware splitting — it is NOT a valid body-table row (likely a Changelog entry, subsystem-section row, or notes table that incidentally carries the BC ID link). `RowMalformed` disposition: advisory + Continue; NEVER reaches postcondition 4 blocking path. This state is distinct from `RowAbsent`: a candidate line WAS found; the found-but-malformed case cannot be collapsed into RowAbsent without triggering false BLOCKs. Narrows `RowAbsent` to exclusively mean "no candidate line found at all." Normative body-table row recognition predicate specified: condition (1) starts with `|`; condition (2) first non-empty field matches `^\[X\]` link form or equals `X` plain form; condition (3) total non-empty field count ≥5. First-cell link form alone is insufficient — a 4-field line `| [BC-5.39.010](path) | title | draft | v1.6 |` satisfies condition (2) but fails condition (3) and is NOT a body-table row. Corpus-validated 2026-08-04: 0 RowMalformed lines in real BC-INDEX (all 1,983 BC-ID-matching lines have ≥5 fields); forward-looking protection. Postcondition 4a added: advisory message prescribing manual verification. Gate Spec `run_part_a_arm1` match extended to four arms. (product-owner; resolves internal contradiction discovered by implementer during v1.8 implementation.) [Prior: 2026-08-04 (v1.8) — PC5 column-anchored locator: state classification now uses escape-aware column count (5 fields → RowPresentNoVersion unconditionally; 6+ fields → Version(v) from 6th column) — token-search approach was a spec gap because story IDs like `S-15.01` in the Stories column match bare-form `\bv?([0-9]+\.[0-9]+)\b`, producing Version("15.01") instead of RowPresentNoVersion; 194 of 1,943 canonical rows carry such story IDs (load-bearing count). Escape-aware splitting required: `\|` within version-chain cells is non-splitting literal; naive `|` split inflates field count. PC13 two-phase algorithm: Phase 1 pure-version field (`^v?[0-9]+\.[0-9]+$`) covers 58 BC-section rows; Phase 2 mandatory-v inline (`\bv([0-9]+\.[0-9]+)\b`) covers 30 Token Budget rows; prior optional-v bare form excluded — produces story-ID collision (29 rows / 6 stories), BC-section-number collision (Token Budget `BC-5.39.010 v1.7` → `5.39` extracted before `1.7`), and ACs-column collision (S-21.07 `DEFERRED v1.6` in rightmost ACs field). Corpus-validated 2026-08-04: 1983 total rows; 1943 five-field (RowPresentNoVersion); 40 six-field (Version(v)); 194 story-ID hazard rows; 1 ACs-column hazard row (S-21.07/BC-5.39.010). PC5 also corrects: version-chain extraction algorithm — latest (rightmost) `\bv([0-9]+\.[0-9]+)\b` match in 6th field. (product-owner; closes F-S2107-P3-001 + PC13 two-phase. Prior v1.7 fixes retained.) [Prior: 2026-08-04 (v1.7) — PC5 corrected: BC-INDEX canonical shape is 5-column (`| BC ID | Title | Status | Capability | Stories |`); version-chain cell is ad-hoc 6th column present on only 40 of 1983 body-table rows (corpus 2026-08-04, adversary pass-3 verified); `extract_bc_index_version` rearchitected from two-state `Option<String>` to three-state `RowAbsent` / `RowPresentNoVersion` / `Version(v)` — two-state `None` conflating the first two misdiagnosed ≥1,712 correct registrations as structural faults per F-S2107-P3-001. Postcondition 4 expanded: `RowAbsent` + version > "1.0" → block (unchanged; genuine structural fault); `RowPresentNoVersion` → silent-continue (5-column canonical shape is standard for ~98% of rows; advisory would be unactionable noise). Part B postconditions note and invariant 11 corrected: `1acf3c6` reclassified from "fabricated" to ALGORITHM-DIVERGENT per ADR-036 §Decision 4 — produced by rc.23 CACHE binary trailing-newline-stripping algorithm, not fabricated; no PROVENANCE-BREAK annotation was warranted; Pass-30 M02 POLICY 18 violation claim for `1acf3c6` retracted. Invariant 11 title updated to three-category taxonomy (stale / fabricated / algorithm-divergent). Gate Spec `run_part_a_arm1` pseudocode updated to reflect three-state match. PC40 confirmed as-written conformant — F-S2107-P3-002 is implementation non-conformance to existing spec, not a spec defect; no PC40 amendment warranted. (product-owner; closes F-S2107-P3-001 spec-side; closes ADR-036 §Decision 4 annotation corrections routed at D-952.) [Prior: 2026-08-04 (v1.6) — Class D (finding-ID namespace advisory in Closes/Refs lines) descoped entirely; active gated classes now A, B, E only. `is_cycle_artifact` dispatch branch marked DEFERRED; `.factory/cycles/` removed from registry path_allow. Premise unsound against unstandardized Closes/Refs convention: six shapes measured across both cycle burst-logs (`**Closes:**`=70, `**Closes (per …):**`=13, no-colon bold=13, non-bold=12, hyphen-form=8); PC31 failed three iterations (v1.2 plain-colon→0 matches; v1.3 bold-bare-colon→20/34; v1.5 bold-word-boundary-colon→86/96 bold but 0/20 non-bold); v1.5 measurement taken against wrong cycle. PC28-PC33 DEFERRED; postconditions 16-18/24 DEFERRED; invariant 6 DEFERRED — all IDs preserved per POLICY 1 append-only. Knowledge preserved in §Deferred Scope with follow-up story target S-21.08 (E-21 epic). PC34 VP-path correction, PC40 volatile-input precondition, invariant-6 I/O-vs-content adjudication, and all Class A/B/E amendments from v1.4-v1.5 survive intact. (product-owner; human-approved scope decision 2026-08-04.) [Prior: 2026-08-04 (v1.5) — Amendment 1 (PC31): Closes/Refs regex corrected to `^\*\*Closes\b[^:]*:\*\*`/`^\*\*Refs\b[^:]*:\*\*` — bare-colon form `^\*\*Closes:\*\*` matched only 20 of 34 burst-log Closes lines (corpus check 2026-08-04, full-file grep); 14 missed (parenthetical `**Closes (per ...):**` x11 + bare-word `**Closes per ...:**` x3); Refs = 0 corpus instances, retained forward-looking; PC31a scope-count advisory added (postcondition 24). Amendment 2 (PC34): VP path `ss-*/VP-*.md` → flat `^VP-[0-9]+\.md$` with VP-INDEX.md exclusion (corpus 2026-08-04: zero ss-* subdirs; 102 VPs flat); epics clause added (dispatch.rs carried arm without PC34 counterpart). Amendment 3 (invariant 6 adjudication): CapabilityDenied/Timeout on cycle artifact is BLOCKING per PC33/postcondition 18/invariant 5; invariant 6 scopes to finding-content verdicts only; postcondition 18 expanded to include Timeout. Amendment 4 (PC40): volatile-input precondition for Class B Arm B1 per ADR-037 §Decision 4; scan story inputs: for volatile patterns; emit prescribed advisory + Continue if found; transitional (vacuous post-remediation); EC-032 added. (product-owner.) [Prior: 2026-08-03 (v1.4) — PC13: bounding-section heading-match predicates changed from exact equality to prefix-with-word-boundary (^## Behavioral Contracts\b, ^## Token Budget\b); 133 of 144 production stories use ## Token Budget Estimate or ## Token Budget Estimate (MANDATORY), which exact equality skipped, causing stale Token Budget citations to go undetected; corpus check (2026-08-03) confirmed zero false positives on .factory/stories/*.md; ## Edge Cases (148 occurrences) remains excluded. Architecture Anchor for extract_story_bc_version_citations updated. Exact-equality non-conformance note added. (product-owner.) [Prior: 2026-07-30 (v1.3) — PC13: bounding section added (scan confined to ##Behavioral Contracts + ##Token Budget sections; ≥9 spurious blocks from Edge Cases rows eliminated); dual version-token format (\bv?([0-9]+\.[0-9]+)\b covers both bare 1.2 and v-prefixed v1.2); LAST rightmost pipe-field algorithm stated. PC31: bold-markdown form (**Closes:**/**Refs:**) required to match D-444(c) real burst-log format; union scan not else-if. PC38 + postcondition 21: non-decreasing relation (∀i: date[i] ≤ date[i+1]); equal same-day dates PERMITTED; EC-030/031 + test vectors added. Amendment 4: no spec change — PC29 (2 MiB) and PC33 (NotFound advisory+Continue on cycle artifact) already unambiguous; fault is purely implementational. PC32: O- deliberately non-excluded per D-449(d)(i); ruling made explicit. POLICY 14 five-leg parity; v1.1 modified[] entry restored (missing since initial authoring — irony: this hook checks modified[] monotonicity but not modified[]↔Changelog row correspondence, so it structurally cannot catch this defect in its own governing BC). (product-owner.) [Prior: 2026-07-30 (v1.2) — Registry entry corrected: tools = [...] array replaced with tool = "^(Edit|Write|MultiEdit)$" regex string (field name singular + MultiEdit added; POLICY 13 ESCAPE-SCOPE-PARITY). Fuel-exhaustion note added to Gate Specifications per ADR-035 §Decision 5. BC-version-pin datum-copy ruling added to Postconditions §Part A Arm2. (product-owner.) [Prior: 2026-07-30 (v1.1) — Part A Arm2 (story-file-side trigger) added; advisory rationales made explicit for every advisory arm; Class D tokenizer namespace-exclusion list added (D-, S-, BC-, VP-, R-, L-, ADR-, EC-, NFR-, ASM-, FM-); EC-024 rationale corrected; Class A coverage-gap routing replaced with correctly-sized latency-gap explanation; Invariant 11 (fabricated vs stale hash provenance) added; EC-026/027/028/029 added; Gate Spec updated with run_part_a_arm2; VP table extended to 17 entries. (product-owner; coordinator review.) [Prior: 2026-07-30 (v1.0) — Initial authoring (product-owner; pre-pass-30 fix-burst). BC-5.39.010 allocated after BC-5.39.009. input-hash d248fc3 per hook-authoritative marketplace binary. lifecycle_status: draft.]]]]]]]]]]]]]
---

# BC-5.39.010: validate-cross-site-correspondence WASM hook MUST block on stale BC-INDEX version-cite after a BC frontmatter bump (Class A Arm1), stale story body BC-table and Token Budget citations after a story edit (Class A Arm2), STORY-INDEX three-way input-hash inequality (Class B), and frontmatter version↔last_amended text-prefix mismatch and modified[] date-decrease (Class E); Class C count/enumeration parity is not mechanically checkable in WASM; Class D (finding-ID namespace advisory in Closes/Refs lines) deferred pending Closes/Refs convention standardization

## Description

The `validate-cross-site-correspondence` WASM hook enforces value-equivalence invariants across
multiple artifact sites that hold the same semantic datum. Six consecutive adversarial review
passes (passes 28-30) produced the same class of finding: a BC or story was correctly updated
at its primary site, but one or more secondary sites holding the same datum were left stale.
In all observed failures **the secondary site was present — its value was stale**. A
presence-only gate would have passed every one of the six failures. Every invariant in this BC
compares values across sites, not the existence of sites.

This BC specifies three mechanically-gateable classes (A, B, E) via a single WASM hook named
`validate-cross-site-correspondence`, and one honest gap (Class C). All three gated arms fire
PostToolUse (Edit/Write); none prevent a write; all signal after the write has completed. Class A
has two arms: Arm1 fires on a BC file write and checks BC-INDEX.md; Arm2 fires on a story file
write and checks the story's own BC version citations against each cited BC's current frontmatter
version. Both arms close the same recurring failure pattern from complementary directions with no
directory enumeration required. Part B covers Class B (three-way input-hash equality per POLICY 18
D-923). Part E covers Class E (frontmatter version↔last_amended text-prefix parity + modified[] monotonicity per
POLICY 14 leg 4 / POLICY 17). Class D (finding-ID namespace advisory in Closes/Refs lines) is
deferred pending Closes/Refs convention standardization; see §Deferred Scope.

Class C is documented in §Honest Gap. It is NOT deferred arbitrarily — it is declared ungatable
by a generic WASM hook for specific structural reasons, and routed to a Rust workspace integration
test per POLICY 21.

## Preconditions

### Part A — Arm1 (BC-File Trigger): BC-INDEX Version-Cite Correspondence (Class A)

1. A PostToolUse Edit/Write event has fired on a file satisfying ALL of:
   - `Path::new(file_path).file_name()` matches `^BC-[0-9]+\.[0-9]+\.[0-9]+\.md$` (regexp on
     basename — path-component-strict per BC-5.39.008 precedent)
   - The file is under `.factory/specs/behavioral-contracts/` (checked via
     `components().any(|c| c.as_os_str() == "behavioral-contracts")`)
   - The file is NOT `BC-INDEX.md` itself (basename guard)
2. The hook reads the BC file via `host::read_file` and extracts the YAML frontmatter `version:`
   field. Strip surrounding quotes. Result: decimal string like `1.18`.
3. The BC identifier is derived from the basename by stripping `.md` (e.g., `BC-5.39.010`).
4. The hook reads `.factory/specs/behavioral-contracts/BC-INDEX.md` via `host::read_file` with
   `max_bytes = 1048576` (1 MiB) and `timeout_ms = 3000`. The 1 MiB cap prevents the
   META-LEVEL-24 silent-truncation false-green class.
5. The hook locates the body-table row for the BC ID post-frontmatter (pattern `^\| \[<id>\]` or
   `^\| <id> \|`). **BC-INDEX canonical table shape is 5-column: `| BC ID | Title | Status | Capability | Stories |`.**
   A version-chain cell is an **ad-hoc 6th column** present on only **40 of 1,985** body-table rows
   (corpus 2026-08-07, escape-aware count: total 1,985; 5-field 1,945; 40 six-field = 40 rows with n≥6).

   **Escape-aware splitting is required.** The version-chain cell uses `\|` (backslash+pipe) as
   an internal separator between version tokens (e.g., `v1.3 \| v1.4 \| v1.5`). The row splitter
   MUST substitute `\|` → placeholder before splitting on `|`, then restore — so that `\|` within
   the version-chain cell does NOT create additional column boundaries. A naive `|` split on a
   version-chain row produces 15+ fields instead of 6 for rows with
   escaped-pipe separators. The escape-aware split yields **6 non-empty fields for all current
   Version(v) rows** (all 40 n≥6 rows have exactly 6 non-empty fields at 2026-08-07). The split
   **MAY yield more** for any future row whose version-chain annotation contains unescaped `|`
   characters (e.g., a regex annotation recorded as `^(Edit\|Write\|MultiEdit\|Agent)$` with bare
   `|`). The `fields[5..].join("|")` reassembly step is REQUIRED and is **retained defensively**:
   zero live n>6 rows exist at 2026-08-07 (the triggering condition was eliminated when the same
   burst's leg D escaped BC-4.13.001's annotation pipes, reducing that row from 9 to 6 non-empty
   fields), but the step is essential to correctness for any future bare-pipe annotation row.
   A pinning test exercising the n>6 arm is needed and has been reported to test-writer.
   Corpus measurement (2026-08-07): `5-field rows: 1,945 /
   6-field rows: 40 / 9-field rows: 0 / total: 1,985 / rows with n≥6: 40`.

   **Column-count-anchored state classification.** After escape-aware splitting, count non-empty
   (whitespace-stripped) fields. The `extract_bc_index_version_state` function MUST return one of
   **four** normative states — **NOT** a two-state `Option<String>` that conflates the first two:
   - **`RowAbsent`**: **no candidate line found at all** for this BC ID post-frontmatter. No line
     matching the locator pattern (`^\| \[<id>\]` or `^\| <id> \|`) exists in BC-INDEX.md. This
     is the genuine structural fault: a previously registered BC whose INDEX row was dropped.
     `RowAbsent` means **exclusively** "no candidate line found" — it does NOT cover found-but-
     malformed lines.
   - **`RowPresentNoVersion`**: row found AND non-empty field count is exactly **5** (the 5-column
     canonical shape). This state is determined **structurally by column count alone** — **no
     token search is performed on any field of the row**. This is the canonical normal state for
     exactly **1,945 of 1,985** rows (corpus 2026-08-07; structural invariant:
     RowPresentNoVersion count = total_candidates − n≥6_count = 1,985 − 40 = 1,945;
     re-derivable via escape-aware split over BC-INDEX post-frontmatter lines matching the
     locator predicate). It is NOT a defect.
   - **`Version(v)`**: row found AND non-empty field count is **≥6** AND the 6th non-empty field
     contains at least one `\bv([0-9]+\.[0-9]+)\b` token.
     Extract the current version using the **first-token-of-last-chain-entry algorithm** (see
     §Decision 4 of ADR-038 for the rationale and four-row proof): (1) join all non-empty fields
     from index 5 onward with `|` to reconstruct the complete version-chain cell, accounting for
     bare `|` characters in annotation text; (2) split the reconstructed cell on `\x00` (the
     escape sentinel substituted from `\|` by the escape-aware split) to isolate chain entries;
     (3) take the LAST non-empty entry — the most-recent entry in the chain; (4) extract the FIRST
     `\bv([0-9]+\.[0-9]+)\b` token from that entry. The first v-token in a chain entry is the
     authoritative current version; subsequent v-tokens in the same entry are annotation prose
     (backward references like `(promoted v1.23)` or `[prior: v1.4]`). The **rightmost-token-of-
     field[5]** algorithm is **NON-CONFORMING** — it produces spurious PC2a advisories for **two of
     four** corpus rows with annotation prose (re-derived 2026-08-07: BC-3.08.001 oldalg 1.23 ≠
     first-of-last 1.24 [differs]; BC-7.03.079 oldalg 1.4 ≠ first-of-last 1.5 [differs];
     BC-4.13.001 oldalg 1.18 = first-of-last 1.18 [same — same burst's leg D escaped the
     annotation pipes that had produced a 9-field row at v1.13 authoring time]; BC-5.24.006
     1.3 = 1.3 [same]; verbatim stdout in §SDK Grounding Evidence). 40 rows reach the `n≥6` arm
     in the current corpus (corpus 2026-08-07).

   **≥6-field / no-v-token case (normative; F-P6-018)**: row found AND non-empty field count is
   **≥6** AND the 6th non-empty field contains **no** `\bv([0-9]+\.[0-9]+)\b` token → classified
   as **`RowPresentNoVersion`**. Rationale: the version-chain cell exists structurally (≥6 fields)
   but carries no recognizable v-prefixed version token. This is a forward-looking state for future
   BC-INDEX annotations in the 6th column that are not version chains. Disposition is identical to
   the 5-field `RowPresentNoVersion` case: `HookResult::Continue` silently with no version
   comparison.

   **Empty-cell counting (normative)**: field count uses **non-empty** (whitespace-stripped) fields.
   A 6-column row (6 pipe-delimited cells) with any one cell blank has 5 non-empty fields →
   classified as `RowPresentNoVersion` regardless of structural column count. The 6th positional
   cell is never read in the 5-count case. Both escape paths (≥6 fields / no v-token, and ≤5
   non-empty fields from a blank cell) produce the same safe outcome: `Continue` silently.

   - **`RowMalformed`**: a **locator-matched line was found** — a line satisfying the locator
     predicate (conditions (1)+(2); see Normative Recognition Predicates below) — but after
     escape-aware splitting the total non-empty field count is **<5** (1–4 fields). The line
     satisfies the locator predicate but fails the body-table row predicate. It is a structurally
     different table (Changelog entry, subsystem-section row, notes table, or other Markdown table)
     that incidentally carries the BC ID link or plain-ID pattern in its first cell. Disposition:
     `host::log_warn` advisory + `HookResult::Continue` — see postcondition 4a. **`RowMalformed`
     is distinct from `RowAbsent`**: a locator-matched line WAS found; the found-but-malformed case
     MUST NOT be collapsed into `RowAbsent` (which would trigger postcondition 4's blocking path
     and produce false BLOCKs). Corpus count (2026-08-04): **0 RowMalformed lines** in real
     BC-INDEX — all 1,983 locator-matched lines have ≥5 fields. This state is forward-looking
     protection.

   **Normative recognition predicates — two-level (F-S2107-P4-022 resolution).** Row classification
   uses two separate predicates, eliminating the apparent contradiction between RowMalformed's
   "locator-matched line found with <5 fields" and the prior three-condition candidacy wording:

   **Locator predicate**: a line satisfies the locator predicate for BC ID X if and only if BOTH
   conditions hold: (1) the line starts with `|`; AND (2) after escape-aware splitting, the first
   non-empty field's stripped content matches `^\[X\]` (link form: `[BC-5.39.010](path)`) OR
   equals `X` exactly (plain form: `BC-5.39.010`). A line satisfying (1)+(2) is called a
   **locator-matched line**. The locator predicate governs which lines are examined; it does NOT
   guarantee the line is a valid body-table row.

   **Body-table row predicate**: a locator-matched line is a **valid BC-INDEX body-table row** if
   and only if condition (3) also holds: after escape-aware splitting, the total non-empty field
   count is **≥5**. Condition (3) is required because first-cell link form alone cannot
   discriminate body-table rows from other tables: a 4-field line
   `| [BC-5.39.010](ss-05/BC-5.39.010.md) | title | draft | v1.6 |` satisfies the locator
   predicate but fails condition (3) — it is `RowMalformed`, not a body-table row.

   **Four-state classification summary**: `RowAbsent` (no locator-matched line anywhere in the
   file); `RowMalformed` (locator-matched line found; field count <5; fails body-table row
   predicate); `RowPresentNoVersion` (locator-matched AND exactly 5 fields); `Version(v)`
   (locator-matched AND ≥6 fields).

   **Candidate-selection order — prefer valid over malformed (F-S2107-P4-005 resolution)**:
   when multiple locator-matched lines exist in BC-INDEX.md, the hook MUST scan the **full file**
   and return the state of the **first locator-matched line that also satisfies condition (3)**.
   `RowMalformed` is returned only when ALL locator-matched lines in the file fail condition (3);
   `RowAbsent` only when no locator-matched line exists at all. A first-match-wins implementation
   that stops on the first locator-matched line without checking whether a subsequent valid line
   exists is **NON-CONFORMING**: a single malformed line appearing before the real body-table row
   permanently silences postcondition 2 (stale-version block) and postcondition 4 (dropped-
   registration block) for that BC — the mirror-image false-negative to the false-positive class
   this state was added to prevent. See Gate Spec pseudocode for the conforming scan algorithm.

   Corpus verification (2026-08-04, escape-aware split):
   `python3 -c "..."` → **0 locator-matched lines** fail condition (3) among BC-INDEX's 1,983
   BC-ID-candidate lines; all are currently valid body-table rows. `RowMalformed` is forward-
   looking protection for anomalous lines that future BC-INDEX maintenance might introduce.

   **Why column-anchored, not token-search.** A token-search implementation (scanning all pipe
   fields for any `\bv?([0-9]+\.[0-9]+)\b` match) is **NON-CONFORMING** for two reasons:

   (a) **RowAbsent/RowPresentNoVersion conflation (F-S2107-P3-001 blast radius ≥1,712)**: a
   two-state `Option<String>` mapping both states to `None` misdiagnoses ≥1,712 correct
   registrations as dropped rows (`RowAbsent` false positives). Per F-S2107-P3-001 corpus
   analysis: 1,983 − 40 version-cell rows − at most 231 v1.0 advisory rows = ≥1,712 false BLOCKs.

   (b) **Story-ID column collision (194 rows)**: the Stories column (field 5) of 194 of the 1,943
   canonical rows contains a story ID of the form `S-NNN.NN` (e.g., `S-15.01`) whose digits
   `NNN.NN` match the bare-form pattern `\bv?([0-9]+\.[0-9]+)\b`. A token-search extractor returns
   `Version("15.01")` for such rows instead of `RowPresentNoVersion`, then compares `15.01`
   against the BC's actual version (e.g., `1.0`) and blocks falsely. The 194 count is the
   load-bearing corpus figure — it is why column-anchoring is necessary rather than merely tidier.
   Corpus verification (2026-08-04):
   `grep -E '^\| \[BC-[0-9]+\.[0-9]+\.[0-9]+\]' BC-INDEX.md | grep -vE '\| v[0-9]+\.[0-9]+' | grep -cE '\| S-[0-9]+\.[0-9]+'`
   → **194**.

   An implementation returning `Option<String>` that maps both `RowAbsent` and `RowPresentNoVersion`
   to `None` is **NON-CONFORMING**: it misdiagnoses ≥1,712 correct registrations as structural
   faults (blast radius confirmed, F-S2107-P3-001). **Additionally**: the test `corpus_arm_a1`
   MUST NOT select only from the 40 version-cell rows — it MUST include at least one BC from the
   1,943 `RowPresentNoVersion` majority; selecting only version-chain rows cannot detect this
   failure class (the corpus test picked BC-1.17.001, one of the 40, allowing F-S2107-P3-001 to
   survive three passes undetected).
6. `RowPresentNoVersion` route: the hook proceeds directly to postcondition 4 (silent-continue).
   No version comparison is performed. `Version(v)` route: the version token is extracted by the
   first-token-of-last-chain-entry algorithm (ADR-038 §Decision 1): join non-empty fields[5..] with
   `|`, split on `\x00` (escape sentinel from `\|`), take the last non-empty entry, extract the
   FIRST `\bv([0-9]+\.[0-9]+)\b` token. The result is normalized by stripping the leading `v`; both
   values (frontmatter `version:` and extracted token) compared as case-sensitive decimal strings
   after normalization (postconditions 1-2). Note: the 6th column (and beyond, for rows with
   bare-pipe annotation fragmentation) contains version chain entries separated by `\|`
   (rendered as `\x00` after escape-aware split); the FIRST v-token of the LAST entry is
   always the current. The rightmost-of-cell algorithm is NON-CONFORMING per ADR-038.

### Part A — Arm1 read failure semantics

7. `host::read_file` for the BC file returns any HostError: `HookResult::block_with_fix(...)`
   naming the error class and path. Primary target, PostToolUse — no fail-open path
   (BC-5.39.008 v1.6 fail-closed ruling).
8. `host::read_file` for BC-INDEX.md returns `HostError::NotFound`: `host::log_warn` advisory +
   `HookResult::Continue` (legitimate bootstrap).
   `HostError::CapabilityDenied` or any capability-class error on BC-INDEX.md:
   `HookResult::block_with_fix(...)` naming the error. Sandbox misconfiguration on a secondary
   target is blocking.

### Part A — Arm2 (Story-File Trigger): Story BC-Citation Currency (Class A)

9. A PostToolUse Edit/Write event has fired on a file satisfying ALL of:
   - `Path::new(file_path).file_name()` matches `^S-[0-9]+\.[0-9]+.*\.md$` (basename)
   - The file is under directory component `stories` (path-component-strict)
   - The file is NOT `STORY-INDEX.md` itself (basename guard)
10. The hook reads the story file via `host::read_file` with `max_bytes = 524288` and
    `timeout_ms = 3000`. Extracts the YAML frontmatter `behavioral_contracts:` sequence.
    If `behavioral_contracts:` is absent or empty: Arm A2 skips entirely (`HookResult::Continue`).
11. For each BC ID in `behavioral_contracts:`, the hook derives the BC file path deterministically:
    - Strip `BC-` prefix from the ID; split on `.`; take first component as the major section
      integer S (e.g., `BC-5.39.010` → S=5)
    - Path: `.factory/specs/behavioral-contracts/ss-<S zero-padded to 2 digits>/<BC-ID>.md`
    - Example: `BC-5.39.010` → S=5 → `ss-05` → `.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md`
    - Example: `BC-6.26.001` → S=6 → `ss-06` → `.factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md`
    - No directory enumeration required; path is mechanically derived from the BC ID.
12. For each BC, the hook reads the BC file via `host::read_file` with `max_bytes = 524288` and
    `timeout_ms = 3000`. Extracts the BC's `version:` from frontmatter.
13. Within the story file content, **the scan is CONFINED to the two BC-citation sections**,
    identified by the following heading-match predicates applied to each `^## ` boundary line:

    - **Behavioral Contracts section start predicate**: a heading line H matches iff H satisfies
      `^## Behavioral Contracts\b` — the text after `## ` begins with the byte sequence
      `Behavioral Contracts` immediately followed by a word boundary (`\b`): a space, `(`, or
      end-of-line. This covers all corpus variants measured 2026-08-03 against
      `.factory/stories/*.md` (139 total sections): `## Behavioral Contracts` (131),
      `## Behavioral Contracts Table` (6), `## Behavioral Contracts Referenced` (1),
      `## Behavioral Contracts (BC Count: NNN)` (1).

    - **Token Budget section start predicate**: a heading line H matches iff H satisfies
      `^## Token Budget\b` — the text after `## ` begins with `Token Budget` immediately
      followed by a word boundary. This covers all corpus variants measured 2026-08-03 (144
      total sections): `## Token Budget` (11), `## Token Budget Estimate` (39),
      `## Token Budget Estimate (MANDATORY)` (94).

    Both predicates MUST be evaluated as regex prefix-with-word-boundary matches, NOT as exact
    equality. An implementation using `heading == "Behavioral Contracts"` or
    `heading == "Token Budget"` (string equality) is **non-conforming**: it silently skips 133
    of 144 production Token Budget sections (`## Token Budget Estimate` and
    `## Token Budget Estimate (MANDATORY)`) and leaves stale Token Budget citations undetected —
    which is one of the two failure modes the BC's H1 title explicitly promises to catch.
    The prior v1.3 text named exact heading text and was misread as exact equality by the
    implementer; this clause corrects that by stating the predicate form normatively.

    **False-positive safety**: the corpus check found zero other `^## ` headings in
    `.factory/stories/*.md` that match either predicate. `## Edge Cases` (148 occurrences)
    begins with `Edge` — it does NOT match `^## Behavioral Contracts\b` or `^## Token Budget\b`,
    so the ≥9-spurious-block regression introduced by an unbounded scan remains prevented.

    Each matched section spans from the matching H2 line to the next `^## ` heading (or EOF for
    the final section). Rows outside these two sections MUST NOT be scanned. Rationale: the Edge
    Cases section, Out-of-Scope section, and other narrative sections in a story file contain
    references of the form `BC-S.SS.NNN EC-0NN` and descriptions that incidentally carry
    version-like tokens (e.g., `v1.0`, `v1.17`); an unbounded scan generates ≥9 spurious
    blocking violations on any story that documents BC edge cases in a table, making the gate
    unwritable for its own governing story.

    Within each BC-citation section, a **version citation** is a table row (contains `|`) where
    the BC ID is present as an **exact word-boundary token**: the text `BC-S.SS.NNN` must be
    bounded by `\b` on both sides. Implementations MUST NOT use a plain `line.contains(bc_id)`
    test. Version extraction uses a **two-phase algorithm**:

    **Phase 1 — Pure-version field (BC-ID-anchored, v1.20 / ADV-RECON-003)**: a row is eligible
    for Phase 1 extraction for BC ID X only if the row's FIRST non-empty pipe-delimited field
    contains X as a word-boundary token — the same locator-predicate test already normative for
    Part A Arm1 (`^\[X\]` link form or `X` plain form; see Normative Recognition Predicates
    above). This excludes rows whose BC-ID column names a **different** BC and which merely
    mention X elsewhere in the row (e.g., a Notes/Trace/Related-BC cell). Within an eligible row,
    scan all pipe-delimited fields; if any field's stripped content matches
    `^v?([0-9]+\.[0-9]+)$` exactly (the entire field is exclusively a version number, optionally
    v-prefixed), use it. Strip the leading `v` if present. This covers the
    `## Behavioral Contracts` body table where the Version column is an isolated field like
    `1.7` or `v1.6`, in the row whose own BC-ID column names the target BC. Corpus count
    (2026-08-06): **58 rows** across all story files (row-first-field anchoring is not expected
    to change this count for legitimately single-subject rows; it only excludes cross-BC-row
    false matches — re-verification against the anchored predicate is an implementer task, see
    Traceability).

    **Why anchoring is required (ADV-RECON-003)**: unlike Phase 2, which anchors within a single
    field to the BC ID's own occurrence, un-anchored Phase 1 scanned the ENTIRE row for the
    rightmost pure-version field with no requirement that the row's own subject is the target BC.
    For a multi-BC story whose `## Behavioral Contracts` table cross-references BCs in
    Notes/Trace/VP columns (corpus-confirmed 2026-08-14: 14 of 480 examined body-table rows
    across `.factory/stories/*.md` name more than one distinct BC ID within a single row — e.g.
    S-4.06's `BC-3.04.001`/`BC-3.04.004`/`BC-3.02.009` supersession rows, S-5.01's session-start
    BC rows, S-6.01's VP-to-BC mapping rows), an unanchored scan resolving BC-Y could pick up
    BC-X's Version cell from the same row, producing (a) a FALSE BLOCK when BC-X's cited version
    differs from BC-Y's current frontmatter version, or (b) a FALSE PASS when a stale citation for
    BC-Y coincidentally matches a rightmost pure-version field belonging to BC-X. Row-first-field
    anchoring closes both directions using the same locator-predicate pattern already normative
    for Part A Arm1 and Phase 2. An implementation that scans the full row without first
    confirming the row's own first field names the target BC is **NON-CONFORMING**.

    **Phase 2 — BC-ID-anchored inline v-prefixed token (fallback)**: if Phase 1 finds no
    pure-version field, locate the field in the row that contains the BC ID (same word-boundary
    test as `line_contains_bc_id_at_boundary`); within that field, return the FIRST
    `\bv([0-9]+\.[0-9]+)\b` token appearing AFTER the BC ID position. Mandatory `v` prefix.
    Return None if no field contains both the BC ID and a subsequent v-prefixed token. This
    covers `## Token Budget` rows where the BC ID and version appear inline in a single field
    (e.g., `BC-5.39.010 v1.7 (full text, 33 ECs...)`).
    The **reverse-field (rightmost-first) algorithm** is **NON-CONFORMING** per ADR-038 §Decision 5:
    (a) annotation prose in the anchor field can carry older version-like tokens after the
    authoritative citation (S-15.17 BC-5.39.009: rightmost returns v1.3 from `POLICY 5 v1.3.6`,
    correct is v1.9); (b) the scan is not scoped to the BC ID's anchor field, enabling cross-field
    and cross-BC contamination (S-4.08: returns v1.1 from a field about a different BC). The
    first-v-token-after-BC-ID algorithm is the direct analog of the first-token-of-last-chain-entry
    ruling in §Decision 1: citation leads; annotation follows. Corpus count (2026-08-06):
    **67 Phase 2 rows / 44 containing BC IDs** (prior figure 30 rows, 2026-08-04, stale — same
    defect class as PC5 count corrected by this ADR's §Decision 4 Change 1).

    **Correction (v1.21 / ADV-RECON5-003):** field-scoping alone (v1.19 / ADR-038 §Decision 5) does
    NOT fully resolve the S-4.08 case cited in (b) above. That citation named the SAME row this
    amendment now closes — `.factory/stories/S-4.08-rc1-release-gate.md`'s `## Behavioral
    Contracts` row for `BC-9.01.002` (Trace cell: "...AC-13 traces ONLY to BC-9.01.001 PC2
    (CHANGELOG monotonicity). BC-9.01.002 covers the orthogonal ... rather than by an AC. v1.1 BC
    candidate `BC-9.01.NNN-...` proposed for explicit gate.") — and field-scoping does not help it,
    because the row's own first field names `BC-9.01.002` (Phase 1 ineligible for target
    `BC-9.01.001`, which IS in this story's `behavioral_contracts:` frontmatter array), and the
    spurious `v1.1` token and the `BC-9.01.001` mention BOTH reside in the SAME (Trace) field — so
    scoping the scan to that one field does not exclude the false match. Corpus-confirmed reachable
    (2026-08-14): the field-scoped algorithm as it stood at v1.20 still extracts `v1.1` as the
    citation for `BC-9.01.001`, whose actual frontmatter `version:` is `"1.0"` — a live false
    BLOCK, not a hypothetical one. The **same-field scan-stop** clause immediately below closes
    this residual.

    **Phase 2 same-field scan-stop (v1.21 / ADV-RECON5-003):** within the anchor field, the forward
    scan for `\bv([0-9]+\.[0-9]+)\b` after the BC ID's position MUST terminate — without producing
    a version — the moment it encounters a DIFFERENT `BC-S.SS.NNN` token (word-boundary match, same
    test as `line_contains_bc_id_at_boundary`) before any qualifying v-token is found. When the scan
    terminates this way, the anchor field yields NO citation, and the caller MUST proceed to the
    next pipe-delimited field per the existing per-field fallback (the "try next field" behavior
    already normative for `find_phase2_version`) rather than returning the first v-token found
    anywhere later in the field regardless of intervening different-BC-ID mentions. Rationale: a
    field that mentions the anchor BC ID and, later in the same cell, ALSO mentions a DIFFERENT BC
    ID before any version token appears is evidence that the eventual v-token is more plausibly
    associated with the later-mentioned different BC (or with neither), not with the anchor BC —
    "citation leads; annotation follows" (§Decision 1) does not license attributing a version token
    past an intervening different-BC-ID mention. Applied to the S-4.08 row above: the anchor field
    for `BC-9.01.001` is the Trace cell; scanning forward from the `BC-9.01.001` occurrence, the
    scan encounters `BC-9.01.002` (a different BC ID) BEFORE reaching `v1.1` — the scan MUST stop
    there and yield no citation from this field; no other field in the row contains `BC-9.01.001`,
    so the row correctly produces zero citations, closing the false BLOCK. An implementation that
    returns the first v-token in the field without this stop-check is **NON-CONFORMING**.

    If neither phase produces a match: the row is NOT a version citation; skip it
    (`HookResult::Continue` for that BC). Absence is not a violation.

    **Why the prior `\bv?([0-9]+\.[0-9]+)\b` bare form (v1.3–v1.7) is NON-CONFORMING:**

    The optional-v bare form creates three collision classes confirmed by corpus analysis
    (2026-08-04, 197 story files):

    (1) **Story-ID collision** (29 rows / 6 stories): story files use tables like `| BC ID | Title
    | Trace |` where the Trace column carries the implementing story ID (`S-1.03 implements
    HookResult...`). The bare form `1.03` matches `\bv?([0-9]+\.[0-9]+)\b`, extracting `1.03` as
    the cited version. The referenced BCs (BC-2.01.001 at `v1.0`, BC-2.02.001 at `v1.1`, etc.)
    are not at version `1.03` → false BLOCK on 29 rows across `S-0.03`, `S-1.03`, `S-2.06`,
    `S-3.01`, `S-4.07`, `S-8.09`. Corpus command:
    `python3 -c "...BC rows where S-N.NN in rightmost Trace field and no Version col..." → 29`.

    (2) **BC-section-number collision** (Token Budget rows): Token Budget rows embed BC ID inline
    (`BC-5.39.010 v1.7 (full text...)`). The bare-form rightmost algorithm extracts `5.39` (from
    `BC-5.39.010`) before `v1.7` (the actual version), because `5.39` is a word-boundary match
    appearing before `v1.7` in the string. All 30 v-prefix-inline Token Budget rows and all 82
    bare-only-or-none Token Budget rows are mishandled. The two-phase algorithm's Phase 2
    (mandatory `v`) correctly extracts `v1.7` from `BC-5.39.010 v1.7 (...)`.

    (3) **ACs-column collision** (1 row — S-21.07, this BC): the Story ACs column contains
    `AC-001 through AC-021 (AC-012/013/014 DEFERRED v1.6 — Class D)`. The rightmost algorithm
    returns `v1.6` from this column instead of `1.7` from the Version column (which is to the
    LEFT of the ACs column). BC-5.39.010 is at `v1.7` → false BLOCK on its own governing story.
    Phase 1 resolves this by finding the isolated `1.7` pure-version field before the ACs column.

    **Corpus summary (2026-08-04):**
    `## Behavioral Contracts` rows: 58 pure-v-col + 10 v-prefix-inline + 375 bare/none = 443 total.
    `## Token Budget` rows: 30 v-prefix-inline + 82 bare/none = 112 total.
    Story-ID hazard (correctly excluded by mandatory-v Phase 2): 29 rows.
    ACs-column hazard (correctly resolved by Phase 1): 1 row.

### Part A — Arm2 read failure semantics

14. `host::read_file` for the story file returns any HostError: `HookResult::block_with_fix(...)`
    — primary target, fail-closed.
15. `host::read_file` for a BC file returns `HostError::NotFound`: `host::log_warn` advisory for
    that specific BC ID + continue checking remaining BCs.
    `HostError::CapabilityDenied` on any BC file: `HookResult::block_with_fix(...)` naming the BC
    path — sandbox misconfiguration is blocking regardless of whether the target is primary.

### Primary Target UTF-8 Decode Failure (applies to all arms; v1.20 / ADV-RECON-007)

15a. After a successful `host::read_file` on the PostToolUse **primary target** (the BC file for
     Arm1/Part E, the story file for Arm2/Arm B1/Part E, or STORY-INDEX.md for Arm B2), if the
     returned bytes fail UTF-8 decoding: this is NOT eligible for silent `HookResult::Continue`.
     The hook MUST emit `HookResult::block_with_fix(...)` naming the file path and the decode
     error, under the **same fail-closed posture as invariant 4** (HostError-class primary-target
     faults). **Rationale**: a primary target that reads successfully as bytes but is not valid
     UTF-8 is itself a data-quality defect on a governing artifact — this repository's BC, story,
     STORY-INDEX, VP, and epic files are authored as UTF-8 Markdown by convention. Silently
     `Continue`-ing on decode failure provides ZERO cross-site correspondence checking for exactly
     the malformed-artifact case most in need of it — the inverse of this BC's entire purpose.
     **Invariant 9 governs `is_char_boundary()` slicing safety on already-decoded strings ONLY —
     it does NOT authorize fail-open disposition of a primary-target decode failure.** An
     implementation citing invariant 9 to justify `HookResult::Continue` on a UTF-8 decode
     failure is **NON-CONFORMING** (misattribution — invariant 9's scope is slicing, not
     read-decode disposition). See postcondition 25 and EC-038.

### Secondary Index-File UTF-8 Decode Failure (Arm A1 BC-INDEX.md secondary read; Arm B1 STORY-INDEX.md secondary read only; v1.22 / ADV-RECON11-001)

15b. This clause is **DISTINCT from precondition 15a** — 15a governs PRIMARY-target decode
     failure (BLOCK); this clause governs **secondary index-file** decode failure (ADVISORY).
     Scope: (i) the BC-INDEX.md read at precondition 4 (**Arm A1**), and (ii) the STORY-INDEX.md
     read at precondition 19 (**Arm B1 only**). It does **NOT** apply to Arm B2, where
     STORY-INDEX.md IS the PostToolUse primary target and is already governed by precondition
     15a / postcondition 25 (fail-closed BLOCK).

     After `host::read_file` on one of these two secondary index-file targets succeeds as bytes
     but the returned bytes **fail UTF-8 decoding**, the row (Arm A1) or hash (Arm B1) state for
     the queried ID is genuinely **INDETERMINATE** — the row-location scan (Arm A1) or hash-token
     extraction (Arm B1) cannot run against undecodable bytes, so the hook cannot distinguish
     "row/hash absent" from "row/hash present but unreadable due to corrupted encoding." This
     state is NOT eligible for silent fallthrough into `RowAbsent` (Arm A1) or `(None, None)`
     (Arm B1) — those states remain reserved **exclusively** for genuinely-absent rows/hashes in a
     **decodable** index file.

     **Rationale (ADV-RECON11-001, architect-adjudicated, LOW/near-zero-reachability)**: prior to
     this amendment, a non-UTF-8 BC-INDEX.md silently degraded to `RowAbsent` on the Arm A1 read
     path, which — for any BC with frontmatter `version:` > `"1.0"` — triggers postcondition 4's
     BLOCK with a MISLEADING message ("dropped registration"; wrong root cause: the true fault is
     index-file corruption, not a missing row). A non-UTF-8 STORY-INDEX.md, read at Arm B1 (not
     Arm B2), silently degraded to `(None, None)` for sites B2/B3, which triggers postcondition
     12's fail-open "not yet registered" advisory — silently disabling three-way hash checking
     for that story with no disclosure that the reason is an undecodable index file rather than
     legitimate pre-registration bootstrap. This is the **same diagnostic-accuracy defect class**
     as F-S2107-P1B-016 (fixed for Arm A2's secondary BC-FILE read at precondition 15/
     postcondition 10), applied **asymmetrically** — F-S2107-P1B-016 closed the gap for secondary
     BC-FILE reads but left secondary INDEX-FILE reads (BC-INDEX.md at Arm A1, STORY-INDEX.md at
     Arm B1) ungoverned for the decode-failure case.

     **Disposition — ADVISORY (Continue), not block**: this is deliberately **lower severity**
     than precondition 15a / postcondition 25's primary-target BLOCK. A secondary target's
     undecodable bytes leave the queried datum's state indeterminate, not confirmably faulty —
     consistent with invariant 5's selective fail-open posture for secondary targets, extended
     here to a second non-NotFound case. See postcondition 26 and EC-040.

### Part B — Three-Way Input-Hash Equality (Class B)

Three sites that must hold identical values for each story S-NNN:

- **Site B1**: story frontmatter `input-hash:` field
- **Site B2**: STORY-INDEX.md body-table catalog row, `input-hash` token
- **Site B3**: STORY-INDEX.md aggregation blockquote `S-NNN=HHHHHHH`

### Arm B1 Preconditions (story file write)

16. PostToolUse on a story file (basename `S-[0-9]+\.[0-9]+.*\.md`, component `stories`, NOT
    `STORY-INDEX.md`).
17. Story ID extracted from basename: `^(S-[0-9]+\.[0-9]+)`.
18. `host::read_file` reads the story file; extracts `input-hash:` from frontmatter.
    If `input-hash:` is absent or null: Arm B1 skips entirely.
19. `host::read_file` reads `.factory/stories/STORY-INDEX.md` with `max_bytes = 1048576` and
    `timeout_ms = 3000`.
20. Catalog row located (post-frontmatter). `input-hash` token extracted:
    `\binput-hash\s+([0-9a-f]{7,40})\b`. If absent from row, Site B2 is absent.
21. Aggregation blockquote (`^> ` lines) scanned. Pattern `\b<id>=([0-9a-f]{7,40})\b` extracts
    Site B3. If story ID absent from blockquote, Site B3 is absent.

### Arm B2 Preconditions (STORY-INDEX.md write)

22. PostToolUse on `STORY-INDEX.md` (basename guard + component `stories`).
23. `host::read_file` reads STORY-INDEX.md with `max_bytes = 2097152` and `timeout_ms = 5000`.
24. All `S-NNN.MM=HHHHHHH` pairs extracted from aggregation blockquote region.
25. For each story ID in the blockquote set, the catalog row is located and `input-hash` token
    extracted. No individual story file reads are performed in Arm B2.

### Part B — read failure semantics

26. Arm B1: story file HostError → block (primary). STORY-INDEX.md `HostError::NotFound` →
    advisory + Continue. `HostError::CapabilityDenied` → block.
27. Arm B2: STORY-INDEX.md IS the primary target. Any HostError → block.

### Part D — Finding-ID Namespace Format (Class D) — **[DEFERRED v1.6; see §Deferred Scope]**

> **Class D is deferred as of v1.6 (human-approved scope decision 2026-08-04). All precondition**
> **IDs below (PC28–PC33) are preserved for POLICY 1 append-only traceability. The**
> **`is_cycle_artifact` dispatch branch MUST NOT be compiled into the hook in v1.6. The**
> **`.factory/cycles/` path_allow entry is removed from the registry until convention**
> **standardization is complete. See §Deferred Scope for the full knowledge base and the**
> **follow-up story brief (target: S-21.08, E-21 epic).**

28. PostToolUse on a file satisfying ANY of:
    - Basename `burst-log.md` AND component `cycles`
    - Basename `lessons.md` AND component `cycles`
    - Basename `INDEX.md` AND component `cycles`
29. `host::read_file` with `max_bytes = 2097152` and `timeout_ms = 5000`.
30. **Scope-limited extraction** (frozen-provenance exclusion by structural position):
    - `burst-log.md`: last H2 section (text from last `^## ` heading through end-of-file)
    - `lessons.md`: last `^L-EDP1-[0-9]+-[0-9]+:` anchor block; if absent, last 200 lines
    - `INDEX.md`: `## Adversarial Reviews` section (between that heading and the next `^## `)
31. Extract all Closes and Refs lines from the scoped region using the **bold-markdown form**
    that D-444(c) mandatory blocks actually use in `burst-log.md`:
    - Closes lines: `^\*\*Closes\b[^:]*:\*\*\s*(.+)$`
    - Refs lines: `^\*\*Refs\b[^:]*:\*\*\s*(.+)$`
    **Both patterns are applied independently (UNION, not `else if`)** to every line.
    Rationale: (i) plain-colon forms `^Closes:\s*` and `^Refs:\s*` match zero real burst-log
    lines — D-444(c) blocks always use bold-markdown; a plain-colon implementation would make
    Class D entirely inert, undetectably. (ii) `^\*\*Closes:\*\*` (bare-colon form) matches
    only **20 of 34** `^\*\*Closes`-class lines in `burst-log.md` (corpus check 2026-08-04,
    full-file grep); the remaining 14 use parenthetical or bare-word qualifiers:
    `**Closes (per D-413(b)):**` (11 instances) and `**Closes per D-413(b) completeness
    mandate:**` (3 instances) — both covered by `\b[^:]*:`. An implementation using the
    bare-colon form misses 41% of live Closes lines in the most recent bursts, making Class D
    exit 0 indistinguishable from "nothing scanned" for those entries. (iii) `^\*\*Refs` = **0**
    corpus instances in `burst-log.md` as of 2026-08-04 (full-file grep). The Refs pattern is
    retained **forward-looking**: D-444(c) reserves `**Refs:**` lines for reference-only
    citations in mandatory burst-log blocks; the predicate uses the word-boundary form for
    consistency and will be ready when that form appears. (iv) a compound line carrying both
    markers must have BOTH segments tokenized — an `else if` implementation drops the Refs
    segment from compound lines. Each line is tested against both patterns independently.

    **Positive-coverage assertion (PC31a)**: after line extraction from the scoped region
    (PC30), the hook MUST emit a `host::log_warn` advisory with the count of Closes/Refs lines
    matched:
    `"validate-cross-site-correspondence [Class D] scope: <N> Closes/Refs line(s) scanned in
    <section> of <file>"`. This fires before the namespace-format check loop (postconditions
    16-17). When N = 0, this makes the inert-scope condition observable to the operator — an
    inert scope (no Closes/Refs lines in the scoped region) cannot be distinguished from a
    clean scope without this advisory; exit 0 becomes vacuously passing rather than
    meaningfully passing. See postcondition 24.
32. For each Closes/Refs line: tokenize by comma and whitespace. A token is classified as
    **finding-like** if and only if BOTH conditions hold:
    - It matches shape `[A-Za-z][A-Za-z0-9-]*[0-9]+` (starts with letter, ends with digit)
    - It does NOT start with any known-safe namespace prefix:
      `D-`, `S-`, `BC-`, `VP-`, `R-`, `L-`, `ADR-`, `EC-`, `NFR-`, `ASM-`, `FM-`
    Finding-like tokens that do NOT start with `F-` are flagged for advisory.
    **`O-` observation IDs are DELIBERATELY non-excluded.** Rationale: D-449(d)(i) explicitly
    scopes 4-index changelog `Refs:` cells to findings (`F-`), policy gates (`PG`), and
    D-NNN decisions only — observation IDs (`O-P30-001`) are declared out of scope for those
    cells. Flagging `O-` tokens as advisory (postcondition 17) correctly surfaces misuse of
    observation IDs in `Closes:`/`Refs:` lines and enforces D-449(d)(i). If a future cycle
    legitimately needs to reference observations in burst-log Closes/Refs, add `O-` to the
    exclusion list with new D-NNN authorization. No such authorization exists as of v1.3.

### Part D — read failure semantics — **[DEFERRED v1.6]**

33. `HostError::CapabilityDenied` on the cycle artifact: block. `HostError::NotFound`:
    advisory + Continue. `HostError::Timeout` or other: block.

### Part E — Frontmatter Internal Parity (Class E)

34. PostToolUse on a file satisfying ANY of:
    - Under `.factory/specs/behavioral-contracts/ss-*/` with basename `BC-*.md` (not
      `BC-INDEX.md`)
    - Under `.factory/specs/verification-properties/` (component-strict: checked via
      `components().any(|c| c.as_os_str() == "verification-properties")`) with basename
      matching `^VP-[0-9]+\.md$`. The `[0-9]+` digit requirement naturally excludes
      `VP-INDEX.md`; an explicit basename guard `file_name() != "VP-INDEX.md"` is REQUIRED for
      defence-in-depth, because `starts_with("VP-") && ends_with(".md")` admits VP-INDEX.md.
      **Corpus verification (2026-08-04):** zero `ss-*` subdirectories exist under
      `.factory/specs/verification-properties/`; all 102 VP files sit flat at
      `.factory/specs/verification-properties/VP-NNN.md`. An implementation scoping to
      `verification-properties/ss-*/VP-*.md` is inert for all 102 VPs in the repository —
      the same defect class as `is_bc_file` admitting `BC-INDEX.md` (pass-1 BLOCKER). The
      prior v1.4 `ss-*/` clause is corrected to the flat path by this amendment.
    - Under `.factory/stories/` with basename `S-*.md` (not `STORY-INDEX.md`)
    - Under `.factory/stories/epics/` (both `stories` and `epics` path components present,
      each verified via `components().any(…)`, component-strict) with basename matching
      `^E-[0-9]+-.*\.md$`. **Rationale:** the shipped `is_frontmatter_parity_target` function
      includes an epics arm (`dispatch.rs`); the prior PC34 had no counterpart, leaving the
      spec incomplete relative to the implementation. Epic files at `.factory/stories/epics/`
      follow POLICY 14 frontmatter conventions (version/last_amended/modified[]) identical to
      story files and must be subject to Class E parity enforcement. This clause closes the
      spec gap.
35. `host::read_file` with `max_bytes = 524288` and `timeout_ms = 3000`. If content does not begin
    with `---`: `HookResult::Continue` immediately.
36. **version: extraction**: extract `version:` YAML field; strip quotes. Result: e.g., `1.6`.
    **Block-scalar parsing REQUIRED** (F-S2107-P4-004 normative coupling): the field extraction
    function `extract_frontmatter_field` MUST handle YAML block scalar indicators (`|`, `|-`,
    `>`, `>-`). When a field line is `<field>: |-` (or any block scalar form), the function MUST
    collect the block body from subsequent indented lines rather than returning the literal
    indicator string `"|-"`. Returning `"|-"` for a block scalar field is **NON-CONFORMING**:
    PC37 feeds the extracted `last_amended:` value directly into a date-prefix regex; a `"|-"`
    return causes `extract_last_amended_outer_version("|-")` to fail (length 2 < 14) → advisory
    branch → Class E1 structurally inert on any artifact using block scalar form. **BC-5.39.010
    itself uses `last_amended: |-` (per D-953 block-scalar convention for long narrative fields),
    as does S-21.07's own story file and ADR-037 — these are the three files this hook most needs
    to gate.**
    Corpus (2026-08-05): `grep -rl '^last_amended: |-' .factory/` → **3 occurrences**:
    `BC-5.39.010.md`, `S-21.07-validate-cross-site-correspondence.md`, and
    `ADR-037-input-hash-stable-input-constraint-volatile-artifacts-excluded.md` (ADR-037 adopted
    the `|-` convention in its v1.1 amendment, 2026-08-05). Block-scalar support is a load-bearing
    normative requirement to prevent silent regression of Class E1 enforcement on the BC's own
    governing artifacts.
37. **last_amended: outermost version extraction**: apply regex
    `^\d{4}-\d{2}-\d{2}\s+\(v([0-9]+(?:\.[0-9]+)*)\)` at CHARACTER POSITION 0 of the field value.
    Captures the outermost (active) version. `[Prior:` chains appear later in the string and are
    excluded structurally by the positional anchor. If regex fails to match: `host::log_warn`
    advisory + `HookResult::Continue` (do NOT block on unparseable format).
38. **modified: extraction**: extract YAML sequence under `modified:`. Strip annotation suffixes
    (e.g., `" (v1.3)"`); compare date strings lexicographically. The required ordering relation
    is **non-decreasing (weak-ascending)**: `∀i: date[i] ≤ date[i+1]`. Equal consecutive dates
    are PERMITTED — a date[i] == date[i+1] pair does NOT violate E2. Only a strict decrease
    (date[i] > date[i+1] after suffix-strip) is a violation. Rationale: same-day multi-burst
    authoring is normal factory operation; this BC's own `modified[]` array at v1.0/v1.1
    authoring was `["2026-07-30", "2026-07-30 (v1.1)"]`, an equal-date pair that would
    self-violate under strict-ascending comparison — the spec must not prohibit its own
    authoring pattern. If absent or empty: skip E2.
39. Any HostError on the primary target file: `HookResult::block_with_fix(...)` — fail-closed.

### Part B — Volatile-Input Precondition (ADR-037 §Decision 4)

40. **Volatile-input scan (Arm B1 only; invoked after PC18, before PC19)**: after extracting
    the story's `input-hash:` field (PC18) and IF the field was present and non-null, the hook
    MUST scan the story's `inputs:` YAML sequence for paths matching the volatile patterns
    defined in **ADR-037 §Decision 2**:

    | Pattern | Matches when |
    |---------|-------------|
    | `.factory/STATE.md` | path equals `.factory/STATE.md` exactly |
    | `.factory/cycles/**/STATE.md` | path contains `.factory/cycles/` AND ends with `/STATE.md` |
    | `.factory/cycles/**/{decision-log,lessons,burst-log}.md` | path contains `.factory/cycles/` AND ends with `/decision-log.md`, `/lessons.md`, or `/burst-log.md` |
    | `.factory/stories/STORY-INDEX.md` | path equals `.factory/stories/STORY-INDEX.md` |
    | `.factory/specs/behavioral-contracts/BC-INDEX.md` | path equals `.factory/specs/behavioral-contracts/BC-INDEX.md` |
    | `.factory/specs/architecture/ARCH-INDEX.md` | path equals `.factory/specs/architecture/ARCH-INDEX.md` |

    Matching uses string equality and suffix checks only — no glob evaluator required. Collect
    all matching paths as `volatile_found`.

    If `volatile_found` is **non-empty**: emit `host::log_warn` advisory with the prescribed
    message from ADR-037 §Decision 4 and return `HookResult::Continue` for this story WITHOUT
    proceeding to PC19-21 or performing the three-way comparison:

    `"validate-cross-site-correspondence [Class B] advisory: Story <id> has volatile inputs per
    ADR-037 §Decision 2 — three-way equality is unsatisfiable until story-writer removes volatile
    inputs and state-manager recomputes the hash; Class B BLOCK suspended. Volatile path(s): <list>"`

    If `volatile_found` is **empty**: proceed to PC19 normally (full three-way check, BLOCKING).

    **Implementation note**: the volatile-pattern list MUST be kept in sync with ADR-037
    §Decision 2. Implement as a compile-time constant slice.

    **Transitional clause**: this precondition is vacuous once **all** stories with volatile inputs
    have had their `inputs:` arrays corrected per ADR-037 §Decision 5. After exhaustive
    remediation, no story matches the volatile patterns and Class B enforces full BLOCKING severity
    for all stories with no carve-outs. **The "imposes no permanent weakening" guarantee holds only
    if ADR-037 §Context enumerates every story with a volatile input exhaustively.**

    **Status as of v1.11 (F-P6-007 correction)**: both preconditions named in the v1.10
    transitional clause have been satisfied. ADR-037 v1.1 §Context added S-21.07 to the
    volatile-story table (corrected to 76 stories in ADR-037 §Context after S-21.07 remediation), and
    S-21.07's `inputs:` array no longer contains `ARCH-INDEX.md` (removed in the pass-5 fix burst,
    2026-08-05). **PC40 is therefore vacuous for S-21.07** — `volatile_found` is empty, and Class B
    proceeds to the full three-way check. **The "no permanent weakening" guarantee holds for
    S-21.07.**

    **Remaining remediation scope**: ADR-037 §Context records **76 stories** with volatile inputs.
    PC40 remains non-vacuous for those 76 stories until story-writer completes the §Decision 5
    remediation sweep.

    **Note on `.factory/specs/architecture/ARCH-INDEX.md` in the volatile pattern table** (row 6):
    including `ARCH-INDEX.md` in `is_volatile_path` is **mandated conformance** to ADR-037
    §Decision 2 — it is one of the six canonical volatile patterns listed there. The v1.10
    characterization of this addition as "widening the suppression rather than closing it" was
    **incorrect and is withdrawn**. Pattern 6 is normative per ADR-037 §Decision 2.

    This clause exists solely to prevent the self-locking failure mode described in ADR-037
    §Rationale during the transition window.

    **Arm B2 not affected**: Arm B2 (PC22-25) fires on STORY-INDEX.md writes and checks
    catalog-vs-blockquote consistency only; it does not read individual story `inputs:` arrays.
    PC40 applies exclusively to Arm B1.

## Postconditions

### Part A Arm1 postconditions

1. BC ID found in BC-INDEX body table with matching version cell (normalized): `HookResult::Continue`.
2. BC ID found with DIFFERENT version cell. Two sub-cases (directional carve-out per F-P6-001
   Option 1; rationale mirrors PC3/PC12 — POLICY 3 (`state_manager_runs_last`) guarantees the BC
   file is always written before the BC-INDEX row in a conforming burst; at the PostToolUse
   instant after a BC write, "index behind primary" is the expected intermediate state of a correct
   in-progress burst, not a defect):

   **2a. Primary newer than index** (strip-v-prefix(`fm_version`) parses as numerically greater
   than strip-v-prefix(`index_version`), comparing `major.minor` as decimal integers):
   `host::log_warn` advisory + `HookResult::Continue`:
   `"validate-cross-site-correspondence [Class A Arm1] advisory: BC-INDEX.md body-table row for
   <id> cites v<index_version> but frontmatter version: is \"<fm_version>\" — primary newer than
   index; state-manager index update pending; Class A BLOCK suspended."`.
   **Advisory rationale**: POLICY 3 forces state-manager to update BC-INDEX (secondary site) AFTER
   the product-owner writes the BC (primary site). "Index behind primary" at the primary-write
   instant is therefore guaranteed in every conforming burst — it is a burst-ordering artefact, not
   a defect. Blocking here produces a spurious `exit 2` for every correct BC authoring burst. This
   is the only direction for which the POLICY 3 ordering argument applies; sub-case (b) has no such
   explanation and retains BLOCK.

   **2b. Index newer than primary** (strip-v-prefix(`index_version`) parses as numerically greater
   than strip-v-prefix(`fm_version`)):
   `HookResult::block_with_fix(...)`:
   `"validate-cross-site-correspondence [Class A Arm1]: BC-INDEX.md body-table row for <id> cites
   v<index_version> but frontmatter version: is \"<fm_version>\" — index is newer than primary.
   This is anomalous: the index cannot legitimately advance ahead of the BC it cites. Verify no
   index row was updated out-of-burst or under the wrong BC path. Update per POLICY 14 leg 5."`.
   **Rationale**: no burst-ordering argument explains the index carrying a higher version than the
   BC file itself. This direction is genuinely anomalous.

   **Version comparison**: strip the leading `v` from both tokens; split on `.`; compare `major`
   then `minor` as non-negative integers. If either token fails to parse as `major.minor` decimal:
   treat as anomalous and emit the 2b block — an unparseable version cell is itself an anomaly.

3. `RowAbsent` (BC ID not in BC-INDEX body table) AND frontmatter `version:` is `"1.0"`:
   `host::log_warn` advisory + `HookResult::Continue`.
   **Advisory rationale**: a v1.0 BC not yet in BC-INDEX is the expected state immediately after
   writing a new BC, before the INDEX update tool call completes in the same burst. Blocking would
   make correct BC authoring impossible — the BC file is always written before the INDEX row. This
   is NOT a "partial check = advisory" rationale; it is a "blocking causes systematic false
   positives in correct authoring bursts" rationale. When `RowAbsent` and version > 1.0, the
   hook blocks (postcondition 4) — so advisory is selective, not the default for all absent rows.
4. `RowAbsent` (BC ID not in BC-INDEX body table) AND `version:` > `"1.0"`:
   `HookResult::block_with_fix(...)` — `RowAbsent` with version > 1.0 means the BC was previously
   registered; a dropped row is a structural fault, not bootstrap ordering. This is the ONLY path
   that blocks for the absent/no-version-cell family.

   `RowPresentNoVersion` (body-table row found; row carries no `\bv[0-9]+\.[0-9]+\b` version
   cell): `HookResult::Continue` silently, regardless of frontmatter `version:`.
   **Rationale**: the 5-column canonical shape — no version cell — is the standard for
   approximately 1,945 of 1,985 BC-INDEX rows (~98%). The absence of a version cell is NOT a
   deficiency in the registration. Emitting an advisory for every write to a 5-column row would
   produce noise on ~98% of BC writes, making it indistinguishable from a genuine advisory signal
   and training operators to ignore advisories. The genuine structural fault this postcondition was
   authored to catch — a dropped registration — is exclusively the `RowAbsent` case (postcondition
   4 above), which retains its BLOCKING severity unchanged. The `RowPresentNoVersion` state is the
   correct `Continue` path for all BCs whose INDEX row has never been annotated with a version chain.
4a. `RowMalformed` (a locator-matched line was found but has <5 non-empty fields after escape-
    aware splitting): `host::log_warn` advisory + `HookResult::Continue`, regardless of
    frontmatter `version:`.
    Advisory message (**NORMATIVE — implementation MUST reproduce this text verbatim**, with
    `<id>` and `<N>` as the only interpolated substitutions):
    `"validate-cross-site-correspondence [Class A Arm1]: BC-INDEX.md contains a malformed
    candidate line for <id> (<N> fields found; expected ≥5 for a valid body-table row). This
    line is structurally not a BC-INDEX body-table row (likely a Changelog entry or notes
    table). Registration status cannot be determined from this line. Verify BC-INDEX body-table
    registration manually."`.
    The clauses "Registration status cannot be determined from this line" and "Verify BC-INDEX
    body-table registration manually" are the operator-actionable instructions — omitting them
    is **NON-CONFORMING** regardless of what other text is injected. **Test-writer MUST assert
    the COMPLETE formatted message text verbatim** (equality check on the full rendered string,
    NOT only `.contains()` assertions on individual clauses) in the `RowMalformed` unit test and
    bats fixture. A test that asserts only two `.contains()` substrings does NOT satisfy the
    MUST-verbatim requirement: it cannot detect an injected sentence, an altered phrase, or a
    dropped sentence that leaves both target substrings intact. Full-string equality is the
    load-bearing gate on the MUST-verbatim clause. The only permissible interpolation points are
    `<id>` and `<N>`; all surrounding text including the hook-name+class prefix, the preamble,
    sentence 2, and both operator-actionable clauses must be reproduced exactly.
    **Rationale**: a found-but-malformed line indicates structural ambiguity, not a confirmed
    dropped registration. The genuine dropped-registration case (no locator-matched line at all)
    is `RowAbsent` (postcondition 4). Blocking on `RowMalformed` would false-positive on any
    non-body-table line that carries the BC ID link pattern in its first cell.
    `RowMalformed` is advisory-only and NEVER reaches the postcondition 4 blocking path.
5. Multiple simultaneous BC writes: each Write is a separate hook invocation; violations are NOT
   accumulated across invocations.

**Class A latency gap (not a coverage gap)**: Arm1 fires on every BC write and catches stale
BC-INDEX immediately. Arm2 fires on every story write and catches stale story citations when the
story is edited. The remaining gap is **latency**: if a BC bumps version in a burst that does not
also touch the story, Arm2 does not fire until the story is next edited. This is not a coverage
gap — in all six observed Class A failures (passes 28-30), the story WAS edited in the same burst
as the BC bump, meaning Arm2 would have caught every observed failure. The empirically relevant
trigger path is fully covered. The POLICY 14 leg 5 obligation to update story citations same-burst
as a BC bump remains binding on the author; Arm2 provides the gate for that case.

**BC-version-pin design ruling (product-owner, 2026-07-30)**: BC-version cells in STORY-INDEX and
all BC-version citations in story bodies carry **datum-copy semantics** — they must match the BC's
current frontmatter `version:` field at the time of any story edit. They do NOT carry
reconciliation-marker semantics (a frozen record of "which BC version the ACs were last reconciled
against that could legitimately lag"). Rationale: (1) POLICY 14 leg 5's "update same-burst"
obligation is incoherent under reconciliation-marker semantics — a reconciliation marker could
intentionally lag, making a same-burst update obligation meaningless; (2) the six Class A failures
across passes 28-30 were correctly identified and closed as defects — retroactive reclassification
as "correct-but-lagging markers" has no new evidentiary basis and would reopen closed findings; (3)
STORY-INDEX.md language "BCs cells reconciled with source frontmatter" describes the act of
updating values to match their source of truth (datum maintenance), not the preservation of a
frozen point-in-time marker. **Implication for Arm A2**: Arm A2 correctly blocks on any
present-but-stale version citation in a story body. No change to Arm A2 blocking logic or behavior
is warranted by this ruling.

### Part A Arm2 postconditions

6. For a given BC ID in `behavioral_contracts:`: all version-citing rows in the story agree with
   the BC's current frontmatter `version:` (normalized): `HookResult::Continue` for that BC.
7. One or more version-citing rows show a version that does NOT match BC frontmatter version:
   `HookResult::block_with_fix(...)`:
   `"validate-cross-site-correspondence [Class A Arm2]: story <story_id> cites <bc_id> at
   v<cited> (in <location>) but BC frontmatter version: is \"<bc_version>\". Update story
   citation same-burst per POLICY 14 leg 5."`.
   All mismatching BCs reported in one combined block message (cascade — do not stop on first).
8. No version-citing row found for a given BC ID: skip that BC, `HookResult::Continue`. The hook
   does NOT block on missing citations; only on present-but-stale citations.
9. `behavioral_contracts:` absent or empty: Arm A2 skips, `HookResult::Continue`.
10. BC file returns `HostError::NotFound` for a cited BC: advisory per PC15; continue checking
    remaining BCs. `HookResult::Continue` for that BC.
    **Advisory rationale**: a cited BC may be retired, may have a non-standard path, or may be a
    new BC added in the same burst whose file exists but the path derivation encounters an edge
    case. `HostError::NotFound` is a known-legitimate transient. `HostError::CapabilityDenied` on
    a BC file blocks (PC15) — sandbox misconfiguration is never legitimate, regardless of target
    type.

### Part B postconditions

11. Arm B1 — all three sites present and equal: `HookResult::Continue`.
12. Arm B1 — B2 or B3 absent: `host::log_warn` advisory + `HookResult::Continue`.
    **Advisory rationale**: a story with `input-hash:` populated but not yet in STORY-INDEX.md is
    the expected state when a story is first authored or when STORY-INDEX.md is updated in a later
    tool call within the same burst. This arm is NOT "advisory because the check is partial." The
    hook blocks when present values disagree (postcondition 13) — it blocks on what it CAN verify.
    Advisory fires only when absence is the anomaly, and absence has a known-legitimate
    interpretation (correct burst ordering). Blocking on absence would cause systematic false
    positives in correct new-story authoring bursts.
13. Arm B1 — B2 or B3 present but differs from B1. Two sub-cases (directional carve-out per
    F-P6-001 Option 1; rationale mirrors PC3/PC12 and PC2a — POLICY 3 (`state_manager_runs_last`)
    forces STORY-INDEX secondary sites to be updated AFTER the primary story write in every
    conforming burst):

    **13a. STORY-INDEX internally consistent, story just rewritten** (B2 == B3 AND B1 ≠ B2):
    `host::log_warn` advisory + `HookResult::Continue`:
    `"validate-cross-site-correspondence [Class B] advisory: Story <id> input-hash mismatch —
    frontmatter=<h1>; STORY-INDEX-catalog=<h2>; STORY-INDEX-blockquote=<h3>. STORY-INDEX sites
    agree with each other; story frontmatter differs. State-manager STORY-INDEX update pending;
    Class B BLOCK suspended."`.
    **Advisory rationale**: POLICY 3 forces STORY-INDEX (both catalog row and blockquote) to be
    updated AFTER the primary story write. When B2==B3 but B1≠B2, the natural explanation is that
    the story was rewritten this burst and the hash has not yet propagated — the POLICY 3 ordering
    artefact. No STORY-INDEX inconsistency exists. Blocking produces a spurious `exit 2` on every
    correct story authoring burst.

    **13b. STORY-INDEX internally inconsistent** (B2 ≠ B3, regardless of B1):
    `HookResult::block_with_fix(...)` (**NORMATIVE — implementation MUST enumerate all three
    provenance categories; a `classify_provenance` heuristic that picks one label is
    NON-CONFORMING per invariant 11**):
    `"validate-cross-site-correspondence [Class B]: Story <id> input-hash three-way mismatch:
    frontmatter=<h1> STORY-INDEX-catalog=<h2 or absent> STORY-INDEX-blockquote=<h3 or absent>.
    STORY-INDEX catalog and blockquote disagree — this is anomalous and has no burst-ordering
    explanation. Update per POLICY 18 (D-923). This hook detects inconsistency only — operator
    MUST determine which of the following applies before remediating: (a) STALE: previously valid
    hash; inputs changed after authoring; remedy: rerun \`compute-input-hash --update\` on the
    story. (b) FABRICATED: hash was never output of \`compute-input-hash --update\` at any
    revision (POLICY 18 violation); remedy: acknowledge PROVENANCE-BREAK in burst-log before
    recomputing. (c) ALGORITHM-DIVERGENT: hash produced by prior binary version per ADR-036
    §Decision 4; NOT fabricated; remedy: recompute with current authoritative binary, no
    PROVENANCE-BREAK annotation required."`.
    **Rationale for blocking on B2≠B3 regardless of B1**: internal STORY-INDEX inconsistency has
    no burst-ordering explanation — both catalog row and blockquote are written by state-manager in
    the same commit. B2≠B3 indicates partial-write, wrong-story editing, or a state-manager bug,
    none of which POLICY 3 explains.

    **13c. Half-present case (PC12 extension, ADR-038 §Decision 3)**: exactly one of {B2, B3} is
    present AND differs from B1; the other is absent. Disposition: `host::log_warn` advisory +
    `HookResult::Continue`. Rationale: the inclusive-or of PC12 ("B2 or B3 absent") covers this
    case — the absent site cannot be verified, and the present-but-differing site may reflect a
    mid-burst state where only one STORY-INDEX update has completed. Blocking produces a self-lock
    on stories with missing blockquote entries (e.g., S-18.11, S-18.12) that have no burst-ordering
    escape. The long-term remedy is state-manager adding the missing blockquote entries, not a
    hook block. Implementation MUST NOT block for `(Some(b2), None)` or `(None, Some(b3))` cases.

14. Arm B2 — catalog and blockquote agree for all blockquote stories: `HookResult::Continue`.
15. Arm B2 — catalog ≠ blockquote for any story: `HookResult::block_with_fix(...)` reporting ALL
    mismatching stories in one message (cascade).

**Fabricated vs stale vs algorithm-divergent distinction — not resolved by this hook**: Class B
detects cross-site *inconsistency* only. It cannot distinguish among: a stale hash (previously
valid computed value; inputs changed after authoring; remedy: `compute-input-hash --update`); a
fabricated hash (never the output of `compute-input-hash --update` at any revision, constituting
a POLICY 18 violation; remedy: PROVENANCE-BREAK annotation in burst-log before recompute); or an
algorithm-divergent hash (legitimately computed by a prior binary version whose algorithm differs
from the current authoritative binary per ADR-036 §Decision 4; NOT fabricated; remedy: recompute
with current authoritative binary, no PROVENANCE-BREAK annotation required). All three trigger
Class B. See Invariant 11 for the three-category remediation protocol.

### Part D postconditions — **[DEFERRED v1.6; see §Deferred Scope]**

16. **[DEFERRED v1.6]** All finding-like tokens on Closes/Refs lines start with `F-`: `HookResult::Continue`.
17. **[DEFERRED v1.6]** Any finding-like token does NOT start with `F-`: `host::log_warn` per token + `HookResult::Continue`.
    Message: `"validate-cross-site-correspondence [Class D] advisory: non-canonical finding-ID
    token '<token>' on line '<line>' in <section> of <file>. Finding IDs must start with 'F-'.
    Verify this is not a phantom ID or retracted reference."`.
    **Advisory rationale**: blocking on syntactic format alone, even after namespace exclusions,
    causes false positives on tokens from unknown future namespaces added in later cycles. The
    namespace exclusion list (PC32) catches known-legitimate non-finding tokens (`D-NNN`, `S-NNN`,
    etc.). The residual advisory population is tokens that look like finding IDs (right shape, not
    in any known-safe namespace) but don't start with `F-`. Blocking these would prevent forward-
    compatibility as new ID classes are introduced. This is NOT "partial check = advisory"; it is
    "the namespace registry cannot be closed, so blocking cannot be made false-positive-free."
18. **[DEFERRED v1.6]** `HostError::CapabilityDenied` or `HostError::Timeout` on the cycle artifact:
    `HookResult::block_with_fix(...)` naming the error variant and artifact path. These are
    infrastructure faults — not finding-content decisions — so invariant 6 does not apply to
    them. Consistent with PC33 (`HostError::Timeout or other: block`) and invariant 5
    (sandbox misconfiguration is never a legitimate state). See invariant 6 adjudication
    ruling for scope clarification.

**Gap — semantic existence**: whether a cited `F-` ID exists in an adversary pass record is
infeasible in WASM (unbounded scan). Routed to Rust workspace test per POLICY 21.

### Part E postconditions

19. E1 version match: `HookResult::Continue`.
20. E1 mismatch: `HookResult::block_with_fix(...)`:
    `"validate-cross-site-correspondence [Class E1]: frontmatter version: \"<ver_fm>\" does not
    match last_amended: outermost text-prefix \"(v<ver_la>)\". Update last_amended: text-prefix
    to (v<ver_fm>) per POLICY 14 leg 4 / POLICY 17."`.
21. E2 non-decreasing (∀i: date[i] ≤ date[i+1], including equal dates): `HookResult::Continue`.
22. E2 strict-decrease: first pair where date[i] > date[i+1] after suffix-strip:
    `HookResult::block_with_fix(...)`:
    `"validate-cross-site-correspondence [Class E2]: modified[] date-decrease at index <i>:
    \"<date_i>\" > \"<date_i+1>\" (after suffix-strip). modified[] must be non-decreasing
    (∀j: date[j] ≤ date[j+1]). Update modified[] per POLICY 14 leg 3."`.
    **POLICY 14 leg enumeration**: leg 3 = `modified[]`; leg 4 = `last_amended:` prefix (cited
    in postcondition 20). An implementation citing "POLICY 14 leg 4" for an E2 violation is
    **NON-CONFORMING** — it routes the fixer to the wrong parity field.
23. Combined E1 + E2 violations: ONE combined block enumerating both.

### Part D — Scope-Count Advisory (postcondition 24) — **[DEFERRED v1.6]**

24. **[DEFERRED v1.6] Scope-count advisory (PC31a)**: after line extraction from the scoped region (PC30-31),
    a `host::log_warn` advisory is emitted with the count of Closes/Refs lines matched by the
    PC31 patterns:
    `"validate-cross-site-correspondence [Class D] scope: <N> Closes/Refs line(s) scanned in
    <section> of <file>"`. This fires unconditionally — before the namespace-format check loop
    (postconditions 16-17) and regardless of block state. When N = 0, the advisory makes a
    silent inert-scope observable. The scope-count advisory does NOT suppress or replace the
    namespace-format advisories (postcondition 17); both are emitted independently.

### Primary Target UTF-8 Decode Failure — postcondition (postcondition 25; v1.20 / ADV-RECON-007)

25. Primary target read succeeds but the returned bytes are not valid UTF-8:
    `HookResult::block_with_fix(...)`:
    `"validate-cross-site-correspondence [primary-read]: cannot decode primary target '<path>'
    as UTF-8: <decode_error>. Fail-closed per BC-5.39.010 invariant 4 (extended, v1.20) —
    invariant 9 governs slicing safety only and does not authorize Continue here. Fix: verify
    the file's encoding and re-save as UTF-8, then retry the write."`
    NEVER `HookResult::Continue` for this case, regardless of which arm(s) the primary target
    would otherwise dispatch to. See precondition 15a and EC-038.

### Secondary Index-File UTF-8 Decode Failure — postcondition (postcondition 26; v1.22 / ADV-RECON11-001)

26. A **secondary index-file target** — BC-INDEX.md (Arm A1) or STORY-INDEX.md (Arm B1 only —
    NOT Arm B2, see precondition 15b) — read succeeds as bytes but the returned bytes fail UTF-8
    decoding: `host::log_warn` **distinct advisory** + `HookResult::Continue`:
    `"validate-cross-site-correspondence: <index-file> failed UTF-8 decode — row/hash state for
    '<id>' is INDETERMINATE, not confirmed-absent. Fix: verify the index file's encoding and
    re-save as UTF-8."`
    `<index-file>` is the secondary target's path (`BC-INDEX.md` or `STORY-INDEX.md`); `<id>` is
    the BC ID (Arm A1) or story ID (Arm B1) being checked. MUST NOT fall through into `RowAbsent`
    (Arm A1; would otherwise reach postcondition 4's BLOCK path) or `(None, None)`/"not yet
    registered" (Arm B1; would otherwise reach postcondition 12's advisory path) — both of those
    dispositions remain reserved exclusively for genuinely-absent rows/hashes in a **decodable**
    index file. **Disposition is ADVISORY (Continue), not block** — deliberately lower severity
    than precondition 15a / postcondition 25's primary-target BLOCK: a secondary target's
    undecodable bytes leave the queried datum's state indeterminate, not confirmably faulty, so
    the low-disruption advisory posture is appropriate to this LOW/near-zero-reachability gap
    (ADV-RECON11-001, architect-adjudicated). Applies identically to Arm A1 (BC-INDEX.md) and
    Arm B1 (STORY-INDEX.md, non-primary-target case only). See precondition 15b and EC-040.

### Cross-arm combination

When a single file write triggers multiple arms, all run independently. Violations from all arms
are combined into one `HookResult::block_with_fix(...)`. Part D advisories are logged regardless
of block state.

## Honest Gap — Class C (Count/Enumeration/Word-Token Parity)

Three observed failure shapes from passes 29-30:

1. **Numeric assertion vs enumeration length** (pass-29 H01, pass-30 H04): prose claims "N gates"
   while the enumeration has N-1 items.
2. **Two count-word tokens disagreeing** (pass-30 H05): `bats:643` `Twenty-four` vs `bats:786`
   `twenty-three`.
3. **Asserted count vs runtime-derived count**: coupling gate compared two literal strings, not a
   mechanical count.

**Why NOT gatable in a generic WASM hook**:

(a) English cardinal parsing (`"twenty-three"` → 23) requires a non-closed lookup table; coupling
    a generic hook to this creates maintenance drag.
(b) The structural anchor for "the count assertion" vs "the enumeration" is file-specific. No
    generic regex reliably distinguishes the asserted count from line numbers or unrelated integers.
(c) Three-way correlation requires semantic region identification with no machine-readable separator.

**Recommended alternative (POLICY 21-compliant)**: Rust workspace integration test with per-file
fixtures. Parameterized test: (i) count enumeration items by structural anchor; (ii) extract
stated count by stable anchor; (iii) assert equality. Flagged for architect to commission as a
follow-on story.

**Class D semantic existence gap**: `F-` ID existence check requires enumerating
`adv-cycle-pass-*.md` files. Same routing: Rust workspace test or CI scan.

## Deferred Scope — Class D (Finding-ID Namespace Advisory)

**Status: DEFERRED as of v1.6. Human-approved scope decision 2026-08-04.**
**Dependency: Class D implementation requires Closes/Refs convention standardization first.**
**Target story: S-21.08 (E-21 epic). Story-writer must confirm next available ID against**
**`.factory/stories/STORY-INDEX.md` before allocating — S-21.07 is the current last entry.**

### Why Class D was deferred

Class D extracts finding-ID tokens from `Closes:` / `Refs:` lines in cycle burst-logs and emits
an advisory when tokens have non-canonical namespace format (not starting with `F-`). The premise
is unsound against the real corpus because the `Closes` convention is unstandardized.

**Corpus measurement (2026-08-04, orchestrator-measured across both cycle burst-logs
`v1.0-brownfield-backfill/burst-log.md` and `v1.0-feature-engine-discipline-pass-1/burst-log.md`):**

| Shape | Count |
|-------|-------|
| `**Closes:**` (bold bare-colon) | 70 |
| `**Closes (per …):**` (bold parenthetical) | 13 |
| `**Closes …` (bold, no colon at all) | 13 |
| `Closes …` (non-bold) | 12 |
| `Closes-…` (non-bold hyphen form) | 8 |
| **Bold total** | **96** |
| **Non-bold total** | **20** |
| **Grand total** | **116** |

The v1.5 predicate `^\*\*Closes\b[^:]*:\*\*` requires a colon — it covers 86 of 96 bold lines
but misses all 20 non-bold lines entirely, and misses the 10 no-colon bold lines. Across the full
corpus of 116 lines, v1.5 covers 86/116 = 74%. No regex is stable against an unstandardized
convention.

### Three failed PC31 iterations

| Version | Predicate | Matches in burst-log | Miss class |
|---------|-----------|---------------------|------------|
| v1.2 | `^Closes:\s*` (plain-colon) | 0 of 116 | All lines — D-444(c) blocks always bold |
| v1.3 | `^\*\*Closes:\*\*` (bold bare-colon) | 20 of 34 in one cycle | `**Closes (per ...):**` (11) + bare-word qualifiers (3) |
| v1.5 | `^\*\*Closes\b[^:]*:\*\*` (bold word-boundary-colon) | 86 of 96 bold; 0 of 20 non-bold | All non-bold lines |

Each iteration narrowed the miss for the bold subpopulation, but none covers non-bold lines at
all. The convention is unstandardized and no predicate is stable until the convention is fixed.

### Wrong cycle measurement (v1.5 defect — recorded for follow-up)

v1.5 Amendment 1 (PC31) took its corpus measurement against `v1.0-feature-engine-discipline-pass-1/burst-log.md`,
reporting the predicate score as 86/96 (90%). However, `STATE.md` `current_cycle:` is
`v1.0-brownfield-backfill` — the **active cycle** whose burst-log is the primary Class D target.
Measuring the inactive archived cycle is a defect: the active cycle has a different population.
Both cycles have non-bold lines; the defect class is cross-cycle in scope. The follow-up story
must measure against BOTH cycle burst-logs after convention standardization.

### L-EDP1- line-anchoring defect (F-S2107-P2-012, preserved for follow-up)

PC30 scopes `lessons.md` extraction to the "last `^L-EDP1-[0-9]+-[0-9]+:` anchor block."
The regex `L-EDP1-[0-9]+-[0-9]+:` expects a **two-group** pattern (e.g., `L-EDP1-001-01:`).
**Corpus measurement (2026-08-04) against `.factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md`:**
- Pattern `^L-EDP1-[0-9]+-[0-9]+:`: **0 matches** — no L-EDP1 entry uses two-group format
- Actual format in corpus: `^L-EDP1-[0-9]+:` (e.g., `L-EDP1-061:`) — single-group
- Pattern `^L-EDP1-[0-9]+:`: **61 matches** (L-EDP1-001 through L-EDP1-061)

The two-group predicate in PC30 never matches any real `lessons.md` anchor. The fallback "last
200 lines" fires for every invocation, making scope-limited extraction structurally broken for
`lessons.md`. The follow-up story MUST fix PC30 by changing the regex to `^L-EDP1-[0-9]+:`.

### D-449(d)(i) O- observation ID non-exclusion ruling (preserved)

PC32 deliberately excludes `D-`, `S-`, `BC-`, `VP-`, `R-`, `L-`, `ADR-`, `EC-`, `NFR-`, `ASM-`,
`FM-` from the finding-like namespace but NOT `O-`. This is intentional: D-449(d)(i) explicitly
scopes 4-index changelog `Refs:` cells to findings (`F-`), policy gates (`PG`), and D-NNN
decisions only — observation IDs (`O-P30-001`) are out of scope for those cells. Flagging `O-`
tokens as advisory surfaces misuse of observation IDs in `Closes:`/`Refs:` lines. **This ruling
MUST be carried forward intact** into the follow-up story. Any `O-` exclusion requires new D-NNN
authorization. No such authorization exists as of v1.6.

### Invariant 6 adjudication (preserved for follow-up story)

The v1.5 invariant-6 adjudication (inline above, preserved here for the follow-up story) establishes
two non-overlapping scopes:

- **Finding-content path** (advisory-only): the non-canonical namespace advisory cannot be made
  false-positive-free because the namespace registry cannot be closed. Applies to finding-content
  verdicts only.
- **I/O-error path** (blocking): `CapabilityDenied` and `Timeout` on the cycle artifact are
  infrastructure faults — fail-open silently disables Class D enforcement. Per invariant 5,
  `CapabilityDenied` on any target is blocking without exception.

The follow-up story MUST implement both paths. A catch-all error arm that swallows all
`HostError` variants as "advisory because Class D is advisory-only" is explicitly non-conforming
(TD-VSDD-059 non-conforming pattern, documented in invariant 6 above).

### Required follow-up story scope

**Target:** S-21.08, E-21 epic. Story-writer assigns; confirm next available number against
`.factory/stories/STORY-INDEX.md` (S-21.07 is current last as of 2026-08-04).

**Dependency direction:** Class D implementation DEPENDS ON Phase 1 (convention standardization).
Do not implement Phase 2 until Phase 1 is complete and all burst-log lines are in canonical form.

**Phase 1 — Convention standardization (prerequisite, must precede Phase 2):**

1. Audit both cycle burst-logs — approximately 116 existing Closes/Refs lines total across
   `v1.0-brownfield-backfill/burst-log.md` and `v1.0-feature-engine-discipline-pass-1/burst-log.md`.
2. Normalize all six shapes to a single canonical form. Recommended canonical: `**Closes:** F-XXXX`
   (bold bare-colon, matching the most common existing shape and D-444(c) intent).
3. Add a shape-enforcing gate (WASM hook or Rust workspace test) that blocks non-canonical
   Closes/Refs shapes on new burst-log entries going forward.
4. Record a corpus measurement after normalization: every shape variant must read 0.

**Phase 2 — Class D implementation (after Phase 1):**

5. Implement the `is_cycle_artifact` dispatch arm with the corrected PC30 L-EDP1 regex
   (`^L-EDP1-[0-9]+:` — single-group, not `^L-EDP1-[0-9]+-[0-9]+:`).
6. Implement PC31 against the now-stable canonical form. Re-take corpus measurement.
7. Restore `.factory/cycles/` to the registry `path_allow` list.
8. Un-retire invariant 6 and postconditions 16-18, 24 — remove DEFERRED markers.
9. Activate EC-010 through EC-013, EC-024, EC-033 and the three Class D test vectors.
10. Run the deferred VP entries (D Namespace-Excluded Token Pass, D Phantom-ID Advisory,
    D Historical-Excluded Pass).

## Invariants

1. The hook NEVER writes to any file. All arms are read-only post-write validators.
2. The hook fires PostToolUse only — writes succeed before the hook fires; the hook signals after.
3. All file-path matching uses path-component-strict guards. Raw string `ends_with` is forbidden.
4. **Fail-closed for primary targets**: any HostError on the PostToolUse trigger file →
   `HookResult::block_with_fix(...)`. No fail-open path for the primary target.
   (BC-5.39.008 v1.6 fail-closed ruling; Canonical Principle + TD-VSDD-059.) **(v1.20 /
   ADV-RECON-007) This posture extends to primary-target reads that succeed as bytes but fail
   UTF-8 decoding**: a successful `host::read_file` on the primary target whose returned bytes
   are not valid UTF-8 is NOT eligible for silent `Continue` — see precondition 15a and
   postcondition 25. Invariant 9 does not authorize fail-open disposition of this case; its
   scope is `is_char_boundary()` slicing safety on already-decoded strings only.
5. **Selective fail-open for secondary targets on NotFound only**: BC-INDEX.md, STORY-INDEX.md,
   and BC files cited in story `behavioral_contracts:` return advisory + Continue on
   `HostError::NotFound` (bootstrap/ordering). `HostError::CapabilityDenied` on any secondary
   target is blocking — sandbox misconfiguration is never a legitimate state.
   **(v1.22 / ADV-RECON11-001) This title's "NotFound only" phrase does not fully describe
   secondary-target disposition scope as of v1.22**: UTF-8 decode failure on BC-INDEX.md (Arm
   A1) or STORY-INDEX.md (Arm B1 only) is a distinct, non-`HostError` case — the `host::read_file`
   call itself succeeds — governed by precondition 15b / postcondition 26. It is disposed
   identically to `NotFound` (advisory + Continue) but for a different reason: an INDETERMINATE
   row/hash state, not a legitimate bootstrap absence. The two cases MUST NOT be conflated in the
   advisory message text — see postcondition 26's prescribed message and EC-040.
6. **[DEFERRED v1.6 — moves with Class D; see §Deferred Scope] Class D finding-content verdict is advisory-only, never blocking**: the non-canonical
   namespace advisory (postcondition 17) is the only verdict Class D emits based on what it
   *finds* — it never blocks based on finding-content decisions. **This invariant scopes to
   finding-content verdicts only.** `HostError::CapabilityDenied` and `HostError::Timeout` on
   the cycle artifact are infrastructure faults — not finding-content decisions — and remain
   BLOCKING per PC33 and postcondition 18. Invariant 6 does not affect them.

   **Adjudication ruling (product-owner, 2026-08-04; preserved in §Deferred Scope):** the
   apparent contradiction between invariant 6 ("Class D advisory-only") and
   PC33/postcondition 18/invariant 5 (`CapabilityDenied`/`Timeout` → block) is resolved as
   follows. Invariant 6 governs the *finding-content path*: the advisory on non-canonical
   finding-ID format is advisory because "blocking on syntactic format cannot be made
   false-positive-free" — that rationale applies to finding-content verdicts only. PC33 and
   postcondition 18 govern the *I/O-error path*: sandbox misconfiguration (`CapabilityDenied`)
   and transport failure (`Timeout`) are infrastructure faults where fail-open would silently
   disable Class D enforcement. Invariant 5 confirms: `CapabilityDenied` on any secondary
   target is blocking without exception. Neither scope overlaps — the two invariants are
   complementary, not contradictory. **This two-path distinction MUST be carried forward intact**
   when Class D is implemented in the follow-up story (S-21.08).

   **Non-conforming pattern (TD-VSDD-059):** an implementation using a single catch-all error
   arm (e.g., `Err(e) if cycle_kind.is_some()` → `log_warn` + Continue) to swallow all
   `HostError` variants on the grounds that "Class D is advisory-only" is non-conforming to
   this invariant's explicit scope. The conforming implementation handles each HostError variant
   independently: `NotFound` → advisory + Continue (PC33); `CapabilityDenied` and `Timeout`
   → `HookResult::block_with_fix(...)` naming the variant and path (PC33/postcondition 18).
7. **No cross-arm suppression**: all arms run independently; violations combine into one block;
   Part D advisories are logged regardless of block state.
8. **Class B Arm B2 cascade**: all STORY-INDEX mismatches reported in ONE combined block.
9. **is_char_boundary() guard**: byte-index slicing on extracted strings MUST use
   `is_char_boundary()` checks where multi-byte UTF-8 is possible (BC-5.39.008 inv-11).
10. **POLICY 21 compliance**: no `.sh` scripts. All gating uses WASM plugin or Rust workspace
    tests. Class C and the Class D existence-check gap are routed to Rust workspace tests.
11. **Stale vs fabricated vs algorithm-divergent hash provenance (Class B)**: this hook detects
    cross-site *inconsistency* only — it cannot distinguish among three categories, all of which
    trigger Class B: (a) **stale** — a previously valid computed value; inputs changed after
    authoring; remedy: `compute-input-hash --update`; (b) **fabricated** — a value that was never
    the output of `compute-input-hash --update` at any revision, constituting a POLICY 18
    violation; remedy: acknowledge PROVENANCE-BREAK in burst-log before running `--update`;
    (c) **algorithm-divergent** — legitimately computed by a prior binary version whose algorithm
    differs from the current authoritative binary (per ADR-036 §Decision 4); NOT fabricated, NOT
    a POLICY 18 violation; remedy: recompute with the current authoritative binary, no
    PROVENANCE-BREAK annotation required. The distinction is load-bearing for remediation. When
    Class B blocks, the fix team MUST verify which category applies before acting: (1) trace the
    stored hash to a prior `--update` invocation (stale path); (2) confirm no `--update`
    invocation ever produced that value and document PROVENANCE-BREAK (fabricated path); (3)
    identify the binary version that produced the hash — if it matches a known older algorithm,
    classify as algorithm-divergent and recompute with the authoritative binary (no annotation).

    **Correction (v1.7, per ADR-036 §Decision 4)**: Pass-30 M02 characterized S-21.04's stored
    hash `1acf3c6` as **fabricated** and asserted a POLICY 18 violation. That characterization is
    **retracted**. Per ADR-036 §Decision 4 and its §Ruling on four pass-29/30 annotations,
    `1acf3c6` is **ALGORITHM-DIVERGENT** — it was produced by the rc.23 CACHE binary
    (`~/.claude/plugins/cache/claude-mp/vsdd-factory/1.0.0-rc.23/bin/compute-input-hash`) which
    strips trailing newlines via `$(cat file)` command substitution. It was a legitimately computed
    value using that binary's algorithm. No PROVENANCE-BREAK annotation was warranted; the
    Pass-30 M02 POLICY 18 violation claim for this specific hash is incorrect and is retracted.

    **The hook block message MUST enumerate all three categories as possible explanations without
    asserting which one applies** — see postcondition 13 prescribed message (F-S2107-P4-006
    ruling: AC-009 must stop requiring classification; invariant 11 governs). The implementation
    pattern of a `classify_provenance` function that picks one of three return strings based on
    B1/B2/B3 positional heuristics — including asserting `"fabricated"` when B1≠B2 and B1==B3 —
    is **NON-CONFORMING**: (1) it asserts as fact a category invariant 11 declares undecidable
    from observed hash values alone; (2) "fabricated" triggers a governance-level PROVENANCE-BREAK
    obligation; a false accusation of fabrication has operational consequences. The conforming
    implementation emits the full three-category enumeration in every Class B block message.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | BC file written with version "1.0" not yet in BC-INDEX body table | Advisory + Continue. New BC; registration expected in same burst. |
| EC-002 | BC bumped v1.17→v1.18; BC-INDEX row still says v1.17 | Advisory + Continue: Class A Arm1, sub-case 2a (primary newer than index; fm=v1.18 > index=v1.17; state-manager index update pending; Class A BLOCK suspended per PC2a). |
| EC-003 | BC file write; BC-INDEX.md returns `HostError::NotFound` | Advisory + Continue. Secondary target NotFound = bootstrap. |
| EC-004 | BC file write; `host::read_file` for the BC file returns `HostError::CapabilityDenied` | Block. Primary target fail-closed. |
| EC-005 | Story S-21.04 frontmatter hash "47a65c9"; STORY-INDEX catalog "4be9d21"; blockquote "S-21.04=47a65c9" | Block: Class B Arm1, sub-case 13b (B2≠B3: catalog="4be9d21" ≠ blockquote="47a65c9"; STORY-INDEX internally inconsistent; no POLICY 3 ordering explanation). |
| EC-006 | Story file has no `input-hash:` field | Continue — Arm B1 skips. |
| EC-007 | STORY-INDEX.md written; blockquote "S-21.04=47a65c9"; catalog row "input-hash 47a65c9" | Continue — B2 == B3 (Arm B2 passes). |
| EC-008 | STORY-INDEX.md written; blockquote "S-21.04=47a65c9"; catalog row "input-hash 1acf3c6" | Block: Class B Arm B2 mismatch. |
| EC-009 | Story file write; STORY-INDEX.md returns `HostError::CapabilityDenied` | Block. Capability-class error on secondary target. |
| EC-010 | burst-log.md; last H2 "Closes: F-S2104-P29-H01, F-S2104-P29-H02" | **(DEFERRED v1.6 — Class D)** Continue (no advisory). |
| EC-011 | burst-log.md; last H2 "Closes: B01, F-S2104-P29-H02" | **(DEFERRED v1.6 — Class D)** Advisory for "B01" (finding-like, not F-, not excluded); Continue. |
| EC-012 | lessons.md; latest L-EDP1 entry "Closes: 001" (bare numeric) | **(DEFERRED v1.6 — Class D)** `001` starts with a digit → does NOT match shape `[A-Za-z][A-Za-z0-9-]*[0-9]+` → NOT flagged. Continue. |
| EC-013 | lessons.md; historical Changelog section contains `P45-001` | **(DEFERRED v1.6 — Class D)** Continue — Changelog is outside the scope-limited window (last L-EDP1 entry only). |
| EC-014 | BC written with `version: "1.6"` and `last_amended: "2026-07-29 (v1.6) — ..."` | Part E passes. |
| EC-015 | BC written with `version: "1.33"` and `last_amended: "2026-07-29 (v1.31) — ..."` | Block: Class E1 (v1.33 vs v1.31). |
| EC-016 | BC written with `modified: ["2026-05-14", "2026-05-18 (v1.1)", "2026-05-15"]` | Block: Class E2 (2026-05-15 follows 2026-05-18). |
| EC-017 | BC written with `modified: ["2026-05-14", "2026-05-18", "2026-05-20 (v1.3)"]` | Part E passes (ascending after suffix strip). |
| EC-018 | BC with `last_amended:` containing `[Prior: ... (v1.5) ...]`; version "1.6"; outermost prefix "(v1.6)" | Part E passes — positional anchor matches `(v1.6)` at date position; Prior tokens excluded structurally. |
| EC-019 | BC write triggers Class A Arm1 (index stale, primary-newer direction: fm=v1.18, index=v1.17) + Class E1 (fm version "1.18" ≠ last_amended "(v1.17)") | Advisory from Class A Arm1 (PC2a; BLOCK suspended) emitted to log + Block from Class E1. Arms run independently; Class A advisory does not suppress Class E1 block. Note: combined two-violation block requires PC2b direction (index-newer-than-primary) + E1 — see EC-035 for the PC2b block scenario. |
| EC-020 | VP file written with `version: "2.4"` and `last_amended: "2026-05-20 (v2.4)"` | Part E passes. VP files are in Part E scope (PC34). |
| EC-021 | Story S-21.04 written; B1 = "47a65c9"; B2 absent; B3 absent | Advisory + Continue. New story pre-registration. |
| EC-022 | `last_amended:` does not match `\d{4}-\d{2}-\d{2}\s+\(v` | Advisory + Continue. Do NOT block on unparseable format. |
| EC-023 | STORY-INDEX.md written | Only Arm B2 fires. STORY-INDEX.md does not match `S-*.md` pattern. |
| EC-024 | burst-log.md; last H2 "Refs: D-944" | **(DEFERRED v1.6 — Class D)** `D-944` matches shape BUT `D-` is in the exclusion list (PC32) → NOT flagged. Continue. |
| EC-025 | BC file write; both BC file AND BC-INDEX return `HostError::CapabilityDenied` | Combined block citing both failures. |
| EC-026 | Story S-21.04 written; `behavioral_contracts: [BC-6.26.001]`; Token Budget row cites "v1.17"; BC-6.26.001 fm "1.18" | Block: Class A Arm2 (story cites v1.17 vs BC v1.18). |
| EC-027 | Story S-21.04 written; `behavioral_contracts: [BC-5.39.010]`; BC-5.39.010.md returns `HostError::NotFound` | Advisory for NotFound BC + Continue. |
| EC-028 | Story S-21.04 written; `behavioral_contracts: [BC-6.26.001, BC-5.39.008]`; both stale | Single combined block listing both BCs (cascade). |
| EC-029 | Story written; `behavioral_contracts: [BC-6.26.001]`; BC cited only in prose, no version token in any table row | Arm A2 finds no version-citing rows → skip → Continue. |
| EC-030 | BC written with `modified: ["2026-07-30", "2026-07-30 (v1.1)"]` — two entries with equal date after suffix-strip | Part E passes. Equal consecutive dates satisfy `date[i] ≤ date[i+1]`; same-day multi-burst cadence is permitted. This BC's own v1.0/v1.1 `modified[]` is exactly this shape. |
| EC-031 | BC written with `modified: ["2026-07-30", "2026-07-29 (v1.1)"]` — date decreases after suffix-strip | Block: Class E2. `2026-07-30 > 2026-07-29` — strict decrease violates the non-decreasing requirement. |
| EC-032 | Story S-19.01 written; `inputs:` contains `.factory/cycles/v1.0-brownfield-backfill/lessons.md`; input-hash present; B2 and B3 absent | Advisory (volatile input detected per ADR-037 §Decision 2; Class B BLOCK suspended; story-writer must remove volatile input) + Continue. PC40 fires at Arm B1 before reading STORY-INDEX.md. |
| EC-033 | `burst-log.md` last H2 section contains no lines matching `^\*\*Closes\b` or `^\*\*Refs\b` | **(DEFERRED v1.6 — Class D)** Scope-count advisory "0 Closes/Refs line(s) scanned" emitted (postcondition 24) + Continue. Namespace-format check loop is skipped (nothing to tokenize). N=0 makes inert scope observable rather than vacuously passing. |
| EC-034 | Story S-21.04 written; input-hash B1="d4f8a12"; STORY-INDEX catalog B2="47a65c9"; blockquote B3="47a65c9" (B2==B3; B1≠B2) | Advisory + Continue: Class B Arm1, sub-case 13a (STORY-INDEX sites agree with each other; story frontmatter differs from index; state-manager STORY-INDEX update pending; Class B BLOCK suspended per PC13a). |
| EC-035 | BC at frontmatter v1.17; BC-INDEX row shows v1.18 (index version numerically greater than BC frontmatter) | Block: Class A Arm1, sub-case 2b (index newer than primary — anomalous; no POLICY 3 burst-ordering explanation; verify no index row was updated out-of-burst per PC2b message). |
| EC-036 | Story S-18.11 written; B1 = "d4f8a12" (new hash); STORY-INDEX catalog B2 = "47a65c9" (stale, pre-existing); blockquote for S-18.11 absent (B3 absent) | Advisory + Continue: Class B Arm1, sub-case 13c (half-present case per ADR-038 §Decision 3: B2 present and differs from B1; B3 absent; PC12 inclusive-or "B2 or B3 absent" governs; present-but-differing site cannot distinguish mid-burst from data-quality defect at PostToolUse trigger time; state-manager must add the missing blockquote entry; implementation MUST NOT block for `(Some(b2), None)`). |
| EC-037 | Story written; `## Behavioral Contracts` table has a row whose FIRST field names BC-X, Version cell "1.7", Notes cell mentions BC-Y inline; `behavioral_contracts: [BC-Y]`; BC-Y frontmatter version is "2.0" (differs from "1.7") | Continue for BC-Y: Phase 1 (v1.20, BC-ID-anchored) does NOT resolve BC-X's row (first field names BC-X, not BC-Y) when checking BC-Y; BC-Y has no version-citing row → skip per postcondition 8 (no false BLOCK from BC-X's Version cell). |
| EC-038 | BC file (or story file) write; `host::read_file` succeeds but returned bytes are not valid UTF-8 (e.g., binary paste or non-UTF-8 mojibake) | Block: primary-target UTF-8 decode failure (precondition 15a / invariant 4 extension, v1.20). NOT `HookResult::Continue`; implementation citing invariant 9 to justify Continue here is NON-CONFORMING. |
| EC-039 | Story S-4.08-rc1-release-gate written; `## Behavioral Contracts` row's first field names `BC-9.01.002`; row's Trace cell mentions `BC-9.01.001` and, later in the SAME cell (after an intervening `BC-9.01.002` mention), the unrelated text "v1.1 BC candidate"; `behavioral_contracts: [BC-9.01.001, ...]`; BC-9.01.001 frontmatter version is "1.0" | Continue for BC-9.01.001 (v1.21, same-field scan-stop): Phase 1 ineligible (row's first field names BC-9.01.002, not BC-9.01.001); Phase 2 anchors on the Trace field but the forward scan from the `BC-9.01.001` position encounters the different BC ID `BC-9.01.002` BEFORE any qualifying v-token — scan stops, field yields no citation; no other field in the row names BC-9.01.001; row produces zero citations (no false BLOCK against actual v1.0 despite the field's later "v1.1" text). |
| EC-040 | (a) BC file write triggers Arm A1; BC-INDEX.md `host::read_file` succeeds as bytes but fails UTF-8 decoding; the written BC's frontmatter `version:` is > "1.0". (b) Story file write triggers Arm B1 (not Arm B2); STORY-INDEX.md `host::read_file` succeeds as bytes but fails UTF-8 decoding | Advisory (precondition 15b / postcondition 26, v1.22 / ADV-RECON11-001): distinct advisory naming the index-file path, stating row/hash state for `<id>` is INDETERMINATE, not confirmed-absent + Continue, for both (a) and (b). (a) MUST NOT silently fall through to `RowAbsent` → postcondition 4 BLOCK ("dropped registration" — misleading; actual root cause is index-file corruption, not a missing row). (b) MUST NOT silently fall through to `(None, None)` → postcondition 12 "not yet registered" advisory (fail-open; would silently disable three-way hash checking with no disclosure of the decode failure). |

## Canonical Test Vectors

| Scenario | Input Condition | Expected Output | Part | Mutant | Control |
|----------|----------------|-----------------|------|--------|---------|
| A Arm1 — new BC | v1.0; no INDEX row | advisory + Continue | A Arm1 | v1.1, no row → block | v1.0 with INDEX row v1.0 → Continue |
| A Arm1 — primary-newer advisory (PC2a) | BC-5.39.008 v1.6; INDEX "v1.5" (primary newer than index) | advisory + Continue (PC2a) | A Arm1 | INDEX "v1.6" (equal) → Continue; INDEX "v1.7" (index newer, PC2b) → block | |
| A Arm2 — current | S-21.04; `behavioral_contracts: [BC-6.26.001]`; story Token Budget "v1.18"; BC fm "1.18" | Continue | A Arm2 | BC fm "1.19" while story says "v1.18" → block | `behavioral_contracts:` empty → Continue |
| A Arm2 — stale | S-21.04; story cites "v1.17"; BC fm "1.18" | block | A Arm2 | Both "v1.18" → Continue | |
| B Arm1 — match | hash "47a65c9"; catalog "47a65c9"; blockquote "47a65c9" | Continue | B Arm1 | B1="d4f8a12" (story rewritten), B2=B3="47a65c9" → advisory (PC13a); B3="4be9d21" while B2="47a65c9" (B2≠B3) → block (PC13b) | no input-hash → Continue |
| B Arm1 — half-present (PC13c) | B1="d4f8a12"; catalog B2="47a65c9" (stale, present); blockquote B3 absent | advisory + Continue (PC13c: B2 present-but-differing, B3 absent; PC12 inclusive-or governs; MUST NOT block) | B Arm1 | B2 absent + B3="47a65c9" (present-but-differing, mirror case) → advisory + Continue (PC13c); B2="d4f8a12" + B3 absent (B2 matches B1) → advisory + Continue (PC12: B3 absent) | all three present and equal → Continue |
| B Arm2 — mismatch | STORY-INDEX catalog "47a65c9"; blockquote "4be9d21" | block | B Arm2 | both "47a65c9" → Continue | |
| D — excluded token | "Closes: F-S2104-P29-H01, D-944" | Continue (D-944 excluded) | D (DEFERRED v1.6) | "Closes: B01" → advisory | |
| D — phantom | "Closes: B01, F-S2104-P29-H01" | advisory for B01 + Continue | D (DEFERRED v1.6) | only "F-..." → Continue | |
| E1 — match | version "1.6"; last_amended "(v1.6)" | Continue | E | "(v1.5)" → block | Prior chain "(v1.5)" deeper → Continue |
| E2 — out-of-order | modified: ["2026-05-14","2026-05-18","2026-05-15"] | block | E | Ascending → Continue | |
| E2 — equal dates | modified: ["2026-07-30","2026-07-30 (v1.1)"] — same date after suffix-strip | Continue (equal dates satisfy ≤) | E | strict-ascending impl rejects equal → block (wrong) | strictly ascending dates → Continue |
| E2 — genuine decrease | modified: ["2026-07-30","2026-07-29 (v1.1)"] — date decreases after suffix-strip | block: E2 (decrease) | E | non-decreasing → Continue | |
| Combined A+E | INDEX stale + E1 mismatch | single combined block | A+E | each alone → block | both fixed → Continue |
| B volatile-input | Story with `inputs: [".factory/cycles/v1.0-brownfield-backfill/lessons.md"]`; input-hash present | advisory (volatile; B suspended) + Continue | B Arm1 | `inputs:` corrected (no volatile) + B2==B3 but B1≠B2 → advisory (PC13a; story just rewritten, STORY-INDEX update pending); `inputs:` corrected + B2≠B3 → block (PC13b; STORY-INDEX internally inconsistent) | `inputs:` corrected + all three sites equal → Continue |
| D zero-scope | `burst-log.md` last H2 has no `**Closes`/`**Refs` lines | scope-count advisory N=0 + Continue | D (DEFERRED v1.6) | Scope has 1 Closes line → N=1 scope-count advisory emitted | |
| A Arm2 — Phase 1 cross-BC-row anchoring (v1.20 / ADV-RECON-003) | Row 1: first field `BC-X`, Version cell "1.7"; Row 2 (same table) mentions `BC-Y` in a Notes cell; `behavioral_contracts: [BC-Y]`; BC-Y frontmatter version "2.0" | Continue for BC-Y (Row 1's Version cell is NOT attributed to BC-Y; no version-citing row found for BC-Y) | A Arm2 | Un-anchored Phase 1 (NON-CONFORMING) → resolves "1.7" for BC-Y → false block against "2.0" | Row's first field is `BC-Y` itself with Version "2.0" → Continue (correctly anchored match) |
| Primary UTF-8 decode failure (v1.20 / ADV-RECON-007) | BC file write; `host::read_file` succeeds; bytes are not valid UTF-8 | block_with_fix (precondition 15a / postcondition 25) | All arms (primary-read step) | `Continue` (citing invariant 9) → NON-CONFORMING, misattributes invariant 9's scope | Valid UTF-8 bytes → normal arm dispatch proceeds |
| A Arm2 — Phase 2 same-field scan-stop (v1.21 / ADV-RECON5-003) | S-4.08-rc1-release-gate.md row: first field `BC-9.01.002`; Trace cell "...traces ONLY to BC-9.01.001 PC2... BC-9.01.002 covers... v1.1 BC candidate..."; `behavioral_contracts: [BC-9.01.001]`; BC-9.01.001 fm "1.0" | Continue for BC-9.01.001 (scan stops at intervening `BC-9.01.002` before reaching `v1.1`; no citation extracted) | A Arm2 | Pre-v1.21 field-scoped-only algorithm (NON-CONFORMING) → extracts "v1.1" for BC-9.01.001 → false block against fm "1.0" | Row's first field is `BC-9.01.001` itself with an isolated Version cell "1.1" → Phase 1 correctly resolves "1.1" (genuine citation, no scan-stop involved) |
| Secondary Index-File UTF-8 Decode Failure (v1.22 / ADV-RECON11-001) | (a) BC-INDEX.md write triggers Arm A1; secondary BC-INDEX.md read succeeds as bytes but fails UTF-8 decode; cited BC fm `version:` > "1.0". (b) Story file write triggers Arm B1 (NOT Arm B2); secondary STORY-INDEX.md read succeeds as bytes but fails UTF-8 decode | Advisory (precondition 15b / postcondition 26): row/hash state for `<id>` is INDETERMINATE, not confirmed-absent + Continue, for both (a) and (b) | A Arm1 / B Arm1 | (a) Silent fallthrough to `RowAbsent` → false BLOCK "dropped registration" (misleading: actual root cause is index corruption, not a missing row) — NON-CONFORMING. (b) Silent fallthrough to `(None, None)` → PC12 "not yet registered" advisory (fail-open; silently disables three-way hash checking) — NON-CONFORMING | Valid UTF-8 index-file bytes → normal Arm A1 / Arm B1 dispatch proceeds unaffected |

## Gate Specifications

### Registry entry

```toml
[[hooks]]
name = "validate-cross-site-correspondence"
plugin = "hook-plugins/validate-cross-site-correspondence.wasm"
event = "PostToolUse"
tool = "^(Edit|Write|MultiEdit)$"
tier = "sync"
on_error = "continue"
path_allow = [
  ".factory/specs/behavioral-contracts/",
  ".factory/specs/verification-properties/",
  ".factory/stories/",
  # ".factory/cycles/" — DEFERRED v1.6: Class D (cycle artifact arm) deferred pending
  # Closes/Refs convention standardization. Re-add when S-21.08 implements Class D.
]
timeout_ms = 8000
```

`on_error = "continue"`: fuel exhaustion or plugin crash is non-blocking. At current corpus scale,
fuel exhaustion is **measured and imminent**, not hypothetical.

**Measured fuel consumption (F-S2107-P9-002; performance-engineer, adversary-pass-9 at
factory-artifacts `0a6c8fda`, against production-scale BC-INDEX fixture sha256-identical to live
BC-INDEX.md at 576,396 bytes / 1,985 rows; verbatim stdout in §SDK Grounding Evidence):**
`fuel_consumed = 9,920,913`. This figure measures actual work performed by the scan and is
unaffected by which cap value is in force below, because 9,920,913 is under every cap discussed
in this section and the hook completed the scan without tripping any of them.

**§Fuel-cap verification (F-S2107-P10-004 correction; closes v1.16/v1.17 present-perfect
overstatement).** The v1.16/v1.17 text here previously asserted, in present perfect, that the
cap "has been raised to 20,000,000" and that the `≤ 12,000,000` margin gate was "satisfiable at
HEAD." At the S-21.07 pass-10 review's frozen snapshot (`feature/S-21.07` @ `5370db80`), that
claim was FALSE: literal-shell grep confirmed both `RegistryDefaults::default()`
(`crates/factory-dispatcher/src/registry.rs`) and `InvokeLimits::default()`
(`crates/factory-dispatcher/src/invoke.rs`) still returned `fuel_cap: 10_000_000` — the raise
had not landed on that snapshot. Because `fuel_consumed` is clamped to the operative cap, and
every observed exhaustion trap reports `fuel_consumed == cap` (bisection table below,
`extra_rows_before=5`: `fuel_consumed=10,000,000` at the former cap), a 10,000,000 cap makes
`fuel_consumed ≤ 12,000,000` true on **every** reachable path — completion or exhaustion —
purely because `10,000,000 < 12,000,000`. The gate was tautological at that snapshot, not
merely early-firing: no reachable execution could have failed it.

**Since that review, the raise has landed.** PR #774 (commit `62fbcf1a`,
"fix(dispatcher): raise WASM fuel cap 10M→20M + fuel-vs-epoch block_reason disambiguation";
ADR-042 §Decision 1) merged to `develop`. Re-verified by literal shell against current source
(`git merge-base --is-ancestor 62fbcf1a HEAD` confirms `62fbcf1a` is an ancestor of both
`origin/develop` and this repository's working HEAD): `crates/factory-dispatcher/src/invoke.rs`
now declares `pub const DEFAULT_FUEL_CAP: u64 = 20_000_000;`, and both
`InvokeLimits::default()` and `RegistryDefaults::default()` source `fuel_cap` from that constant
(no duplicated literal) — pinned against re-drift by the `fuel_cap_defaults_stay_in_sync`
regression test. The present-perfect claim "has been raised to 20,000,000" is now TRUE **at
source-HEAD**.

**Source-HEAD is not the same thing as operator-effective.** This project's own PostToolUse
hook chain runs the marketplace-cached `factory-dispatcher` binary
(`~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/hooks/dispatcher/bin/<platform>/`),
not `crates/` compiled ad hoc. Per `CLAUDE.md`'s fuel-exhaustion diagnostics row: the bundled
operator-level binary remains pinned to the pre-raise 10,000,000 cap through v1.0.0-rc.23; the
20,000,000 cap is consumed only after v1.0.0-rc.24 is cut and the marketplace cache is
refreshed. **Until rc.24 is consumed, the margin gate below remains tautological in live
operator enforcement of this hook, even though the underlying source has already moved to
20,000,000** — the fix is real at source-HEAD but not yet operationally effective for this
hook's own live PostToolUse gate.

Measured current consumption (9,920,913) < 12,000,000 (60% of the 20,000,000 source-HEAD cap).
Headroom from the former 10M cap: **79,087 fuel = 0.79%**; headroom from the 20M source-HEAD
cap: **~10,079,087 fuel ≈ 589 SS-05 rows (insert-before, adversarial)** at the insert-before
marginal cost of ~17,114 fuel/row (see placement-sensitivity below; ADR-042 §Decision 1).

**Early-return scope qualifier (ADR-042 §Context).** `extract_bc_index_version_state` returns
early once BC-5.39.010's row is found (row 921 of BC-INDEX.md). Only rows 1–920 are scanned on
a write to this BC. New BCs registered in sections after SS-05 §5.39 (rows > 921) cost nothing
for this hook trigger — only BCs landing in rows 1–920 consume runway.

**Placement sensitivity: per-row fuel cost varies ~29× depending on where a new BC row
lands relative to the early-return point at row 921 (ADR-042 §Context).**
`extract_bc_index_version_state` early-returns once BC-5.39.010's row is found; only rows 1–920
consume runway. The two valid measurement scenarios:

- **INSERT-BEFORE scenario** (adversarial: a new SS-05 entry inserted immediately before row
  921 contributes its full ~486 bytes to the scanned region): ~15,761 fuel/row (perf-fuel-2,
  single-row measurement); ~17,114 fuel/row (bisection average, 4-row span below). Runway
  under the former 10M cap: **4 rows safe, 5th exhausts** (bisection; confirmed by perf-fuel-2
  at ~5 rows). Under the new 20M cap: ~589 rows ((20,000,000 − 9,920,913) / 17,114 ≈ 589).

- **APPEND scenario** (realistic registration traffic: rows appended after existing SS-05
  entries; only the fraction landing before row 921 enters the scanned region): ~171.3
  bytes/row appended; ~1,018 of those bytes (5.9%) land in the scanned prefix; ~541.4
  fuel/row. Runway under the former 10M cap: **~146 rows** ((10,000,000 − 9,921,105) / 541.4
  ≈ 146), with exhaustion observed between 100 and 130 appended rows in §SDK Grounding
  Evidence. Under the new 20M cap: ~18,600 rows ((20,000,000 − 9,921,105) / 541.4 ≈ 18,623).
  **The ~110-row v1.15 figure was derived from an append-scenario measurement against the
  former 10M cap — it is correct for that scenario.** The v1.15-erratum and v1.16 claims that
  this figure was "falsified" due to two compounding errors are themselves incorrect and are
  retracted; see §Changelog.

**Exhaustion bisection — INSERT-BEFORE scenario (literal captured stdout,
performance-engineer, 2026-08-08):**

| extra_rows_before | bytes_scanned | fuel_consumed | status |
|---|---|---|---|
| 0 | 415,523 | 9,920,913 | OK |
| 3 | 416,981 | 9,970,304 | OK |
| 4 | 417,467 | 9,989,369 | OK |
| 5 | 417,953 | 10,000,000 | TIMEOUT/fuel (former 10M cap) |

**Fuel cost is linear in input bytes, not superlinear.** Regression over 24 measured points
(N=5..986 extra rows): `fuel = 2,585,970 + 53.18 × var_bytes`, R² = 0.998790. The ADR-035
§Decision 5 quadratic warning (`fd_readdir` concern) applied to a rejected
directory-enumeration alternative. The implemented `read_file` scan is **O(n) in input
bytes** — confirmed in ADR-042 §Decision 4 Correction 1.

**The prior claim — "The `max_bytes` caps in PC4 (1 MiB), PC10/12/15/35 (512 KiB), and
PC19/23/29 (2 MiB) are calibrated to bound reads inside the fuel budget at current artifact
sizes" — is retracted.** The 1 MiB PC4 cap permits a file of 1,048,576 bytes; exhaustion at the
former 10M cap occurred at approximately 594 KB — the cap was approximately **76% above the
exhaustion point**. Under the new 20M cap the fuel model places exhaustion at approximately
1,089 KB, giving ~513 KB of structural headroom below the 1 MiB cap.

**On exhaustion the dispatcher exits 0 with empty stdout** — output-identical to a clean scan.
This is the silent-guard-failure class this BC exists to prevent, applied to this BC itself.
With `on_error = "continue"`, fuel exhaustion is silenced at the registry level; there is no
WASM-side handling for it in this hook. The effect: Arm A1/A2/B/E validation stops entirely and
undetectably as BC-INDEX grows, while `cargo test`, `bats`, and the dispatcher all exit 0.
This is **ADR-042 §Decision 3 class (a)** — silent exhaustion for `on_error = "continue"`
plugins. The fix is observable signaling, not `on_error` escalation.

**NORMATIVE REQUIREMENT — Exhaustion MUST surface as an observable failure (ADR-042 §Decision 3
class (a)).** When `TimeoutCause::Fuel` is detected, the dispatcher MUST: (i) emit
`fuel_exhausted=true fuel_exhausted_plugins=<name> fuel_cap=<N>` in the stderr summary line;
(ii) include `{"type": "fuel_exhausted", "plugin_name": "<name>", "fuel_cap": <N>,
"validation_skipped": true}` in the `advisories[]` payload; (iii) emit the advisory log per
ADR-035 §Decision 5. `on_error = "continue"` semantics are preserved — PostToolUse cannot
revert a write and exhaustion is a resource-policy failure, not a validation failure. This
obligation is dispatched to implementer per ADR-042 §Downstream Routing; test-writer adds a bats
gate asserting a fuel-exhausting fixture produces non-empty stderr containing `fuel_exhausted=true`
and non-empty `advisories[]`. This is a **pending implementation item**, not a spec gap.

The production-scale bats gate for this hook MUST include both of the following assertions
(test-writer obligation per F-S2107-P9-002 and ADR-042 §Decision 3 routing):

1. **Margin gate**: execute the real dispatcher against the production-scale BC-INDEX fixture,
   parse `fuel_consumed` from the `plugin.completed` record in dispatcher output, and assert
   `fuel_consumed ≤ 12,000,000` (60% of the **20,000,000** source-HEAD cap — see §Fuel-cap
   verification above for the source-HEAD-vs-operator-effective distinction; **this test is only
   non-tautological when the dispatcher binary it exercises is actually built from source with
   `DEFAULT_FUEL_CAP = 20_000_000`. Run against the pre-rc.24 operator-cached binary (cap
   10,000,000) the assertion is vacuously true — see the tautology proof below — and passing MUST
   NOT be credited as coverage until rc.24 is consumed.**). Satisfiable at source-HEAD: measured
   current consumption is **9,920,913 < 12,000,000**.

   **Why this is a real gate, not a tautological one, at the 20,000,000 cap (F-S2107-P10-004
   closure).** `fuel_consumed` is clamped to the operative cap. At the former 10,000,000 cap the
   assertion held on *every* reachable execution path — completion (`fuel_consumed ≤ 10,000,000`)
   and exhaustion (`fuel_consumed == 10,000,000`, per the bisection table above) both satisfy
   `≤ 12,000,000` unconditionally, because `10,000,000 < 12,000,000` regardless of what the hook
   does. No reachable state could fail it: that is the definition of tautological. At the
   20,000,000 cap, a genuine failing region exists and is reachable without exhaustion:
   **`fuel_consumed` in `(12,000,000, 20,000,000)` is a state in which the hook completes
   successfully — no exhaustion, no crash — yet the assertion FAILS.** That region is 8,000,000
   fuel wide, larger than the current passing margin (9,920,913 to 12,000,000 ≈ 2,079,087 fuel).
   The gate can only be trusted to have this discriminating power when the cap actually measured
   by the test run is 20,000,000; the same assertion text against a 10,000,000-cap binary reverts
   to the tautological shape described in §Fuel-cap verification.

   Warning horizon depends on placement scenario (see placement-sensitivity above):
   - **Insert-before (adversarial; governing)**: gate fires at ~121 additional SS-05 rows before
     row 921 ((12,000,000 − 9,920,913) / 17,114 ≈ 121 rows); ~468 rows before 20M exhaustion.
   - **Append (realistic traffic)**: gate fires at ~3,840 additional rows appended
     ((12,000,000 − 9,920,913) / 541 ≈ 3,840 rows); ~18,600 rows before 20M exhaustion.
   The gate threshold is calibrated against the insert-before scenario as the governing
   constraint. **A gate asserting ≤ 6,000,000 (the v1.15 form) is NON-CONFORMING: current
   consumption 9,920,913 > 6,000,000, making the gate permanently failing at HEAD and
   unsatisfiable without a cap change.** **A gate that passes at 99.21% of any cap is
   NON-CONFORMING under this BC even if bats exits 0 today.** **A gate whose failing region is
   empty for every reachable execution — as `≤ 12,000,000` was against the 10,000,000 cap — is
   NON-CONFORMING regardless of measured value: vacuous truth is not coverage
   (F-S2107-P10-004).**

2. **Fixture drift gate**: assert that the production-scale fixture's `BC-INDEX.md` sha256 matches
   the live `.factory/specs/behavioral-contracts/BC-INDEX.md`, or that the fixture's BC-INDEX row
   count is within ±10 rows of the live count. A stale snapshot permanently greens the margin gate
   while production silently exhausts — the same predicate-narrower-than-claim shape the fixture
   was added to close, displaced one level up. **Behavior in worktrees**: the gate currently skips
   in worktrees lacking a `.factory/` mount and runs in CI (three `git worktree add .factory` steps
   in `ci.yml`). If fail-closed-when-absent behavior is required normatively, state so here; the
   current behavior is skip-in-worktree / run-in-CI.

**Operational consequence.** With the global fuel cap raised to 20M at source-HEAD (ADR-042
§Decision 1; commit `62fbcf1a` / PR #774 — **not yet the operator-effective cap for this
project's own live PostToolUse enforcement of this hook until v1.0.0-rc.24 is consumed; see
§Fuel-cap verification above**), runway before re-exhaustion is scenario-dependent: **~589 rows** for SS-05-sized entries inserted
immediately before row 921 (adversarial / insert-before; ~17,114 fuel/row); **~18,600 rows** for
rows appended after existing SS-05 entries (realistic registration traffic; ~541 fuel/row). At a
realistic production rate of 20–50 new BCs per feature wave (append scenario), this covers
hundreds of feature waves. The insert-before bound governs gate design. A long-term structural
fix (Option C:
`read_file_range` targeted row lookup, making fuel cost O(1) in corpus size) is specified in
ADR-042 §Decision 5 and anchored to E-21 W7 after ADR-039 Phase 1 (S-21.10) and Phase 2
(S-21.11) ship.

**`fuel_cap` field (ADR-042 §Decision 2).** The prohibition on `fuel_cap` entries (v1.15) is
**lifted** — the premise that `max_bytes` caps keep reads inside the budget is
measurement-falsified. The per-plugin `fuel_cap` registry field (ADR-035 §Decision 5, ADR-039
§Decision 2) is **not yet implemented**: `grep -c 'fuel_cap|failure_policy'
hooks-registry.toml` → 0 as of 2026-08-08. Once ADR-039 Phase 1 (registry `fuel_cap` schema
extension) ships, a per-plugin `fuel_cap` **SHOULD** be set for
`validate-cross-site-correspondence`, calibrated to p99×1.5 per ADR-039 §Decision 4 Option A
(minimum 50M for Phase 3 fail-closed annotation). Until then, the only available lever is the
global `InvokeLimits::default()` cap at 20M.

### Internal dispatch logic

```
fn run(payload):
    file_path = payload.tool_input.file_path
    violations = []
    advisories = []

    if is_bc_file(file_path):           // Part A Arm1 + Part E
        content = read_primary(file_path, 524288, 3000)
        violations += run_part_a_arm1(file_path, content)
        violations += run_part_e(file_path, content)
    elif is_story_file(file_path):      // Part A Arm2 + Part B Arm1 + Part E
        content = read_primary(file_path, 524288, 3000)
        violations += run_part_a_arm2(file_path, content)
        violations += run_part_b_arm1(file_path, content)
        violations += run_part_e(file_path, content)
    elif is_story_index(file_path):     // Part B Arm2
        content = read_primary(file_path, 2097152, 5000)
        violations += run_part_b_arm2(file_path, content)
    // DEFERRED v1.6 — Class D descoped; is_cycle_artifact arm inactive until S-21.08
    // elif is_cycle_artifact(file_path):  // Part D
    //     content = read_primary(file_path, 2097152, 5000)
    //     advisories += run_part_d(file_path, content)

    for adv in advisories: host::log_warn(adv)
    if violations: return combined_block(violations)
    return HookResult::Continue
```

### Part A Arm1: `run_part_a_arm1(file_path, content)`

```
bc_id = basename(file_path).trim_suffix(".md")
fm_version = extract_frontmatter_field(content, "version") |> strip_v_prefix
index_content = host::read_file(BC_INDEX_PATH, 1048576, 3000)
  // NotFound → log_warn advisory + return []
  // CapabilityDenied → return [block(cap_denied)]
match extract_bc_index_version_state(index_content, bc_id):
    // four-state per PC5 v1.11; two-level predicates:
    //   locator predicate: (1) line starts with |; (2) first non-empty field matches ^\[<id>\] or equals <id>
    //   body-table row predicate: locator-matched AND (3) non-empty field count ≥5 after escape-aware split
    //   scan MUST prefer first (1)+(2)+(3) line — full-file scan; RowMalformed only if ALL (1)+(2) lines fail (3)
    //   if no (1)+(2) line exists anywhere → RowAbsent
    RowAbsent:
        // no locator-matched line found at all — genuinely dropped registration
        if fm_version == "1.0": log_warn(new_bc_advisory); return []
        else: return [block(row_absent_v_gt_1_message)]     // postcondition 4 blocking path
    RowPresentNoVersion:
        return []   // 5-column canonical shape; no version cell; silent-continue (postcondition 4)
    RowMalformed:
        // locator-matched line found but ALL such lines have <5 fields — not a body-table row
        log_warn(malformed_candidate_advisory(bc_id, field_count)); return []  // postcondition 4a; never blocks
    Version(index_version):
        // index_version extracted by first-token-of-last-chain-entry algorithm per ADR-038 §Decision 1
        // (fields[5..].join("|") → split on \x00 → last entry → first v-token); rightmost-of-field[5] NON-CONFORMING
        // PC2 directional carve-out (v1.11 / F-P6-001 Option 1):
        // compare major.minor as decimal integers after stripping 'v' prefix
        if parse_version(fm_version) > parse_version(index_version):
            // 2a: primary newer than index — POLICY 3 ordering artefact; advisory only
            log_warn(advisory_primary_newer_msg(bc_id, index_version, fm_version)); return []
        elif parse_version(fm_version) < parse_version(index_version):
            // 2b: index newer than primary — anomalous; retain BLOCK
            return [block(anomalous_index_newer_msg(bc_id, index_version, fm_version))]
        // equal: postcondition 1 — Continue
        return []
        // parse_version failure (unparseable token) → treat as anomalous → 2b block
```

### Part A Arm2: `run_part_a_arm2(story_path, story_content)`

```
bc_ids = extract_frontmatter_sequence(story_content, "behavioral_contracts")
if bc_ids.is_empty(): return []
story_id = extract_story_id_prefix(story_path)
violations = []
for bc_id in bc_ids:
    // Deterministic path derivation — no list_dir
    section = bc_id.trim_start_matches("BC-").split('.').next().parse::<u32>()
    bc_path = format!(".factory/specs/behavioral-contracts/ss-{:02}/{}.md", section, bc_id)
    bc_content = match host::read_file(bc_path, 524288, 3000):
        Ok(c) => c
        Err(HostError::NotFound) => { log_warn(not_found_advisory(bc_id)); continue }
        Err(e) => return [block(bc_read_error(bc_id, bc_path, e))]
    bc_version = extract_frontmatter_field(bc_content, "version") |> strip_v_prefix
    citations = extract_story_bc_version_citations(story_content, bc_id)
    // Vec<(location: String, cited_version: String)>
    // scoped to sections matching ^## Behavioral Contracts\b or ^## Token Budget\b (PC13 prefix predicates)
    // bc_id must be an exact word-boundary token (\b on both sides)
    // TWO-PHASE version extraction (PC13 v1.20 — BC-ID-anchored per ADV-RECON-003):
    //   Phase 1: row eligible only if row's FIRST non-empty field matches bc_id at a word
    //   boundary (locator predicate, same test as Part A Arm1); within an eligible row, any
    //   field matching ^v?([0-9]+\.[0-9]+)$ exactly (pure-version field) → use it. Un-anchored
    //   full-row Phase 1 scan is NON-CONFORMING (cross-BC-row contamination).
    //   Phase 2 (fallback): BC-ID-anchored first-v-token after BC ID in anchor field; mandatory v-prefix;
    //   rightmost-first NON-CONFORMING per ADR-038 §Decision 5
    // prior optional-v \bv?([0-9]+\.[0-9]+)\b is NON-CONFORMING: story-ID collision (29 rows),
    // BC-section-number collision (Token Budget), ACs-column collision (1 row, S-21.07)
    for (location, cited_ver) in citations:
        if strip_v_prefix(cited_ver) != bc_version:
            violations.push(stale_arm2_msg(story_id, bc_id, location, cited_ver, bc_version))
return violations
```

### Parts B, D, E

`run_part_b_arm2` and `run_part_e` are unchanged from v1.0 specification.

`run_part_b_arm1` changed per v1.5 Amendment 4 (PC40): after extracting `input-hash` (PC18),
call `parse_story_volatile_inputs(story_content)` to extract the `inputs:` sequence, then check
each entry against `is_volatile_path(path)` before reading STORY-INDEX.md. If any volatile path
is found, emit the PC40 prescribed advisory and return `HookResult::Continue` immediately
(skipping PC19-21). See PC40 for the volatile-pattern table and prescribed advisory message. If
`inputs:` is absent or empty, `volatile_found` is empty and B1 proceeds normally. (Corrected
from `check_volatile_inputs` which was never the shipped function name — F-P6-004.)

`run_part_b_arm1` also changed per v1.11 (F-P6-001 PC13 directional carve-out): after the
three-way comparison (PC19-21), when B2 or B3 is present but the values do not all agree, the
hook MUST distinguish two sub-cases per postcondition 13:
- **(13a) B2==B3 AND B1≠B2**: `host::log_warn` advisory + `HookResult::Continue`. STORY-INDEX
  catalog row and blockquote agree with each other; the story frontmatter just differs. This is
  the POLICY 3 burst-ordering artefact — state-manager writes STORY-INDEX AFTER the primary
  story write in every conforming burst. No inconsistency exists in STORY-INDEX itself.
- **(13b) B2≠B3, regardless of B1**: `HookResult::block_with_fix(...)`. STORY-INDEX catalog
  row and blockquote disagree with each other. No burst-ordering argument explains this; it
  indicates a partial write, wrong-story edit, or state-manager bug.
- **(13c) Half-present case — exactly one of {B2, B3} present and differing; the other absent**:
  `host::log_warn` advisory + `HookResult::Continue`. PC12's inclusive-or ("B2 or B3 absent")
  covers this case; the absent site cannot be verified at PostToolUse trigger time and the
  present-but-differing site may reflect a mid-burst or pre-existing data-quality state.
  Implementation MUST NOT block for `(Some(b2), None)` or `(None, Some(b3))` cases. See
  postcondition 13c and ADR-038 §Decision 3.
**The prior any-mismatch-blocks behavior** (blocking whenever B1≠B2 OR B1≠B3, regardless of
the B2/B3 relationship) **is NON-CONFORMING** under v1.11. Conforming implementations MUST
check B2≠B3 as the blocking condition, and MUST NOT block for the half-present case (PC13c).
See postconditions 13a, 13b, and 13c for prescribed advisory and block messages respectively.

**[DEFERRED v1.6 — `run_part_d` is NOT part of the v1.6 hook implementation]**
`run_part_d` specification: after scope-limited line extraction (PC30), count the lines matched
by the PC31 patterns and emit the scope-count advisory via `host::log_warn` before entering the
namespace-format check loop. (Preserved here per POLICY 1 for the follow-up story S-21.08.)

```
fn run_part_d(file_path, content):
    scoped = extract_scope_limited_region(file_path, content)   // PC30
    closes_refs_lines = extract_closes_refs_lines(scoped)       // PC31 patterns
    // PC31a — always emit before namespace check
    host::log_warn(scope_count_msg(closes_refs_lines.len(), file_path))
    for line in closes_refs_lines:                              // postconditions 16-17
        for token in tokenize(line):
            if is_finding_like(token) && !starts_with_F(token):
                host::log_warn(namespace_advisory(token, line, file_path))
    // postcondition 18 — handled via read_primary (PC29) before this point
    return []   // Class D never returns violations; all advisory-only

// Volatile-check shape (F-P6-004 correction — check_volatile_inputs was never the shipped fn):
// run_arm_b1 calls: is_volatile_path(path: &str) -> bool          [arm_b.rs]
//                   parse_story_volatile_inputs(content: &str) -> Vec<String>  [arm_b.rs]
// Inline logic in run_arm_b1:
//   inputs = parse_story_volatile_inputs(story_content)
//   volatile_found = [p for p in inputs if is_volatile_path(p)]   // PC40 table
//   if volatile_found is non-empty:
//     host::log_warn(volatile_advisory_msg(story_id, volatile_found))
//     return HookResult::Continue   // skip PC19-21
//   // proceed to PC19 normally
```

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | A Arm1 Index-Newer-than-Primary Block (PC2b) | bats integration test |
| (pending) | A Arm1 Primary-Newer-than-Index Advisory (PC2a) | bats integration test |
| (pending) | A Arm1 New-BC Advisory | bats integration test |
| (pending) | A Arm1 Primary-CapabilityDenied Block | bats integration test |
| (pending) | A Arm2 Stale-Citation Block | bats integration test (story Token Budget stale) |
| (pending) | A Arm2 No-Citation Skip (Continue) | bats integration test (no version-citing rows) |
| (pending) | A Arm2 BC-NotFound Advisory (Continue) | bats integration test |
| (pending) | A Arm2 BC-CapabilityDenied Block | bats integration test |
| (pending) | B Arm1 STORY-INDEX-Internally-Inconsistent Block (PC13b) | bats integration test |
| (pending) | B Arm1 STORY-INDEX-Consistent Advisory (PC13a) | bats integration test |
| (pending) | B Arm1 Absent-Sites Advisory | bats integration test |
| (pending) | B Arm2 Internal Mismatch Block | bats integration test |
| (pending; DEFERRED v1.6 — Class D) | D Namespace-Excluded Token Pass (D-944) | bats integration test |
| (pending; DEFERRED v1.6 — Class D) | D Phantom-ID Advisory | bats integration test |
| (pending; DEFERRED v1.6 — Class D) | D Historical-Excluded Pass | bats integration test |
| (pending) | E1 Version-Mismatch Block | bats integration test |
| (pending) | E2 Non-Ascending Block | bats integration test |
| (pending) | E Prior-Chain Exclusion Pass | bats integration test |
| (pending) | Combined A+E Block | bats integration test |

VP IDs pending VP-INDEX allocation by state-manager at post-merge burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | E-12 (Engine Governance — cross-site value correspondence enforcement) |
| Capability Anchor Justification | E-12 governs factory engine discipline automation. This BC formalizes PostToolUse gates enforcing cross-site value correspondence invariants recurring across passes 28-30 of the F5 adversarial cycle: Class A = POLICY 14 leg 5 (two-arm: BC-INDEX Arm1 + story citation Arm2); Class B = POLICY 18 THREE-WAY-INPUT-HASH-EQUALITY-GATE (D-923); Class E = POLICY 14 leg 4 / POLICY 17; Class D = finding-ID namespace advisory. No formal CAP-NNN from domain-spec/capabilities.md covers engine process governance automation at this layer; E-12 is the established sub-capability anchor for this BC family per BC-5.39.008 §Traceability. |
| Architecture Module | `crates/hook-plugins/validate-cross-site-correspondence/` (new WASM crate); `plugins/vsdd-factory/hooks-registry.toml`; `plugins/vsdd-factory/hook-plugins/validate-cross-site-correspondence.wasm` |
| D-NNN Sub-Clauses Closed | POLICY 14 leg 5 (BC-INDEX body-table + story citation sync; Class A Arm1 + Arm2); POLICY 18 D-923 (Class B); POLICY 14 leg 4 / POLICY 17 (Class E); Canonical Principle + TD-VSDD-059 (fail-closed) |
| Stories | S-21.07 (implementing story; v1.5 in flight) |
| L2 Invariants | (none — process-automation gate) |

## Related BCs

- BC-5.39.003 — `validate-index-cite-refresh`: Class A Arm1 is the per-BC-write counterpart
- BC-5.39.004 — `validate-burst-log`: burst-log structural completeness
- BC-5.39.007 — `validate-closes-completeness`: Closes block presence; Class D checks format
- BC-5.39.008 — `validate-policies-schema`: establishes fail-closed ruling governing Invariant 4
- BC-6.26.001 — primary motivating artifact for Class A; repeatedly re-staled in passes 28-30

## Architecture Anchors

- `crates/hook-plugins/validate-cross-site-correspondence/`
- `crates/hook-sdk/src/host.rs` — `host::read_file`, `host::log_warn`
- `crates/hook-sdk/src/result.rs` — `HookResult::Continue`, `HookResult::block_with_fix`
- `extract_bc_index_version_state(content: &[u8], bc_id: &str) -> BcIndexVersionState` — four-state return per PC5 v1.11 (corrected from `extract_bc_index_version`, which was deleted by F-P4-016 in the pass-5 burst; spec named the deleted symbol): `RowAbsent` (no locator-matched line at all), `RowPresentNoVersion` (locator-matched; exactly 5 non-empty fields, OR ≥6 non-empty fields but no `\bv([0-9]+\.[0-9]+)\b` in field 6 — F-P6-018 normative addition), `Version(v)` (locator-matched; ≥6 non-empty fields AND field 6 contains a `\bv([0-9]+\.[0-9]+)\b` token; `v` extracted by the **first-token-of-last-chain-entry algorithm** per ADR-038 §Decision 1: `fields[5..].join("|")` → split on `\x00` → take last non-empty entry → extract first `\bv([0-9]+\.[0-9]+)\b` token; **rightmost-of-field[5] is NON-CONFORMING** per ADR-038 §Decision 1), `RowMalformed` (locator-matched but ALL such lines have <5 non-empty fields; NOT a body-table row — advisory + Continue). Two-level predicates: locator=(1)+(2); body-table=(1)+(2)+(3). Scan MUST prefer first valid (≥5-field) locator-matched line; first-match-wins on malformed line is NON-CONFORMING (F-S2107-P4-005). Uses escape-aware splitting (`\|` non-splitting). `RowMalformed` MUST NOT be collapsed into `RowAbsent`.
- `extract_frontmatter_field(content, field)` — extracts a named YAML field from frontmatter. **MUST handle block scalar forms** (`|`, `|-`, `>`, `>-`): when the field line ends with a block scalar indicator, collect the block body from subsequent indented lines (do not return the literal indicator). Returning `"|-"` for a block-scalar field is NON-CONFORMING (PC36 normative requirement; BC-5.39.010 and S-21.07 both use `last_amended: |-`).
- `derive_bc_path(bc_id)` — deterministic BC file path derivation from BC ID (no list_dir)
- `extract_story_bc_version_citations(content, bc_id)` — finds version-citing table rows for a given BC ID within sections matching `^## Behavioral Contracts\b` or `^## Token Budget\b` ONLY (PC13 prefix-with-word-boundary predicates; NOT exact equality); two-phase version extraction: Phase 1 pure-version field (`^v?[0-9]+\.[0-9]+$`), Phase 2 fallback BC-ID-anchored first-v-token (`\bv([0-9]+\.[0-9]+)\b` first token after BC ID in anchor field; **rightmost-first NON-CONFORMING per ADR-038 §Decision 5**: fails for annotation prose carrying older v-tokens after the citation, and does not scope to the BC ID's anchor field enabling cross-BC contamination); prior optional-v bare form NON-CONFORMING (29-row story-ID collision, Token Budget BC-section-number collision, 1-row ACs-column collision); returns Vec<(location, version)>
- `extract_frontmatter_sequence(content, field)` — parses YAML sequence field from frontmatter
- `is_volatile_path(path: &str) -> bool` — returns `true` if `path` matches any ADR-037 §Decision 2 volatile pattern from the PC40 table; used by `run_arm_b1` inline logic to scan the story's `inputs:` sequence. (Corrected from `check_volatile_inputs` which was never the shipped function name — F-P6-004.)
- `parse_story_volatile_inputs(content: &str) -> Vec<String>` — extracts the `inputs:` YAML sequence from story frontmatter content, returning the list of input paths. Called by `run_arm_b1` before invoking `is_volatile_path` per-path. (Part of the actual volatile-check shape; replaces the non-existent `check_volatile_inputs` wrapper.)
- `is_frontmatter_parity_target(file_path)` — PC34 classifier (BC, VP, story, epic files); VP arm uses flat `verification-properties/` path + `^VP-[0-9]+\.md$` predicate (no `ss-*/`)

## Story Anchor

S-21.07 — `validate-cross-site-correspondence` WASM hook (v1.5 in flight; BC-5.39.010 is the governing behavioral contract per story frontmatter `behavioral_contracts: [BC-5.39.010]`).

## VP Anchors

VP-102 through VP-120 (19 VPs) are planned for this story per D-945 (anchored to S-21.07
post-merge). The reserved range covers all 19 rows enumerated in §Verification Properties
(Class A: 8; Class B: 4; Class D: 3 DEFERRED; Class E: 4) — grown from the original 17 at
v1.12, when two advisory-path property rows ("A Arm1 Primary-Newer-than-Index Advisory (PC2a)"
and "B Arm1 STORY-INDEX-Consistent Advisory (PC13a)") were added per F-P6-001 Option 1. Both
advisory-path rows are first-class verification properties eligible for the same VP-INDEX
allocation as the other 17 — the reservation is a straight range expansion, not a split
between allocatable and advisory-only entries. VP IDs have not yet been formally allocated to
VP-INDEX — that occurs at the state-manager post-merge burst per established practice. Once
allocated, this section will be updated to a full VP table with property names and proof
methods. The story carries `verification_properties: []` reflecting this pending state.
(v1.19, F-S2107-P18-001: corrected from the stale "VP-102 through VP-118 (17 VPs)" statement,
which was never swept when the table grew to 19 rows at v1.12.)

## SDK Grounding Evidence

Verbatim stdout captured 2026-08-07 per POLICY 15, using exact production locator semantics
(`line.starts_with('|')`; escape-aware split on `\|`→`\x00`; first non-empty cell matches BC-ID
in `^\[BC-\d+\.\d+\.\d+\]` link form or `^BC-\d+\.\d+\.\d+$` plain form).

### Corpus count re-derivation (F-S2107-P8-005 — 1,943/1,983 → 1,945/1,985)

```
candidate rows (production semantics): 1985
histogram: {5: 1945, 6: 40}
>6-field rows: 0 []
n>=6 arm: 40
RowPresentNoVersion (n==5): 1945
RowPresentNoVersion of total: 1945 of 1985
```

**Structural invariants (POLICY 5 v1.3.6 HEAD-REPRODUCIBILITY-OR-STRUCTURAL-FORM MANDATE):**
- RowPresentNoVersion count = total_candidates − n≥6_count (currently 1,985 − 40 = 1,945)
- The n≥6 arm count equals the number of BC-INDEX rows carrying a version-chain cell
- All 40 current n≥6 rows have exactly 6 non-empty fields (zero n>6 rows at 2026-08-07)

**Re-derivation predicate** (regenerates figures at any HEAD without trusting stored counts):
Apply escape-aware split (`\|`→`\x00`, split on `|`, restore, strip whitespace) to every
BC-INDEX post-frontmatter line where: (1) line starts with `|`; (2) first non-empty field matches
`^\[BC-\d+\.\d+\.\d+\]` (link form) or `^BC-\d+\.\d+\.\d+$` (plain form). Count non-empty fields
per matching line; histogram by field count. RowPresentNoVersion = histogram[5]; n≥6 = sum of
histogram[k] for k≥6; total candidates = sum of all histogram values.

### Four-row proof re-traced (F-S2107-P8-005 — three-of-four → two-of-four)

```
BC              frontmatter  first-of-last  oldalg-field5  oldalg_differs
BC-3.08.001     1.24         1.24           1.23           True
BC-7.03.079     v1.5         1.5            1.4            True
BC-4.13.001     1.18         1.18           1.18           False
BC-5.24.006     v1.3         1.3            1.3            False

Rows where algorithms differ: 2 of 4
```

BC-4.13.001 stopped differing because the same burst's leg D escaped its annotation pipes
(`Edit\|Write\|MultiEdit\|Agent` unescaped → `Edit\|Write\|MultiEdit\|Agent` escaped in
BC-INDEX), reducing that row from 9 non-empty fields to 6. The BC v1.13 spec was authored
in leg C before leg D ran.

### Fuel consumption measurement (F-S2107-P9-002 — 99.21% consumed at production scale)

Captured from adversary-pass-9 (`adversary-pass-9.md` at factory-artifacts `0a6c8fda`), executing
the real dispatcher against the production-scale BC-INDEX fixture (sha256-identical to live
BC-INDEX.md at 576,396 bytes / 1,985 rows). Direct invocation output:

```
type= plugin.completed fuel_consumed= 9920913  (of 10,000,000)  exit_code= 0
```

Bisection (real BC-INDEX rows appended to fixture; `dispatch_exit=0` in all cases — exhausted
invocations exit 0 with empty stdout, identical to a clean scan):

```
added_real_rows=0    bytes=576396  -> plugin.completed fuel=9921105 cause=-
added_real_rows=20   bytes=579889  -> plugin.completed fuel=9932167 cause=-
added_real_rows=60   bytes=586350  -> plugin.completed fuel=9952042 cause=-
added_real_rows=100  bytes=593525  -> plugin.completed fuel=9975246 cause=-
added_real_rows=130  bytes=601548  -> plugin.timeout   fuel=10000000 cause=fuel
added_real_rows=400  bytes=649702  -> plugin.timeout   fuel=10000000 cause=fuel
```

Exhaustion threshold: between 593,525 and 601,548 bytes (~110 additional rows above current
576,396-byte corpus). Headroom: 10,000,000 − 9,920,913 = 79,087 fuel = 0.79% of budget.
PC4 cap at 1,048,576 bytes vs exhaustion at ~594,000 bytes: (1,048,576 − 594,000) / 594,000
≈ 76% — the cap is ~76% above the exhaustion point.

### ADR-037 roster re-derivation (F-S2107-P8-010 — 77 → 76)

ADR-037 §Context records 76 volatile-input stories (union) and 61 ARCH-INDEX-leg stories
(corrected from 77/62 in the same burst as BC-5.39.010 v1.13 was authored; BC not swept).
The adversary independently re-derived 76/61 at factory-artifacts `10914a73` using
`is_volatile_path` semantics over 159 story files, matching ADR-037 exactly.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.21 | 2026-08-14 | **ADV-RECON5-003 (MEDIUM)**: product-owner adjudication of an adversary-flagged spec-conforming residual in `validate-cross-site-correspondence`, human-approved "Amend now (v1.21)". §Preconditions PC13 Phase 2 gains a **same-field scan-stop**: within the anchor field, the forward scan for `\bv([0-9]+\.[0-9]+)\b` after the BC ID's position MUST terminate — without producing a version — the moment it encounters a DIFFERENT `BC-S.SS.NNN` word-boundary token before any qualifying v-token; the field then yields no citation and the caller proceeds to the next pipe-delimited field. Corpus-confirmed reachable (2026-08-14, not hypothetical): `.factory/stories/S-4.08-rc1-release-gate.md`'s `## Behavioral Contracts` row for `BC-9.01.002` — Trace cell mentions `BC-9.01.001` (present in this story's `behavioral_contracts:` array) then, later in the SAME field (past an intervening `BC-9.01.002` mention), an unrelated "v1.1 BC candidate" phrase — the v1.20 field-scoped-only algorithm still extracted "v1.1" as `BC-9.01.001`'s citation despite its actual frontmatter version being "1.0", a live false BLOCK. A correction paragraph clarifies that v1.19/ADR-038 §Decision 5 field-scoping alone did not resolve the S-4.08 case its own text cited as the motivating example — the same-field scan-stop is what closes it. New EC-039 + new Canonical Test Vector row added ("A Arm2 — Phase 2 same-field scan-stop"). No BC H1 change; no ID renumbering (POLICY 1 append-only) — PC13 Phase 2 text amended in place, EC-039 is net-new. BC-INDEX, the S-21.07 story, and `find_phase2_version` (`crates/hook-plugins/validate-cross-site-correspondence/src/arm_a2.rs`) are NOT amended in this burst — BC-INDEX version-cell sync owed to state-manager; S-21.07 story BC-table/Token Budget cite owed to story-writer; `find_phase2_version` scan-stop implementation owed to implementer; RED-gate unit test for the S-4.08 shape owed to test-writer. (product-owner; closes ADV-RECON5-003.) |
| 1.20 | 2026-08-14 | Two spec-correctness amendments from fresh-context adversarial review, human-approved "fix both now" (production-grade default; no deferral to S-21.08 — its Class D scope has a genuine blocking prerequisite neither item shares). **ADV-RECON-003 (MEDIUM)**: §Preconditions PC13 Phase 1 (pure-version-field extraction) rearchitected from an unanchored full-row scan to BC-ID-anchored — a row is eligible for Phase 1 extraction for BC ID X only if the row's FIRST non-empty pipe-delimited field contains X at a word boundary (same locator-predicate test already normative for Part A Arm1 and Phase 2); closes a hazard where a multi-BC `## Behavioral Contracts` table row could cause Phase 1 to resolve the wrong BC's Version cell (false BLOCK or false PASS). Corpus-confirmed reachable (2026-08-14): 14 of 480 examined body-table rows across `.factory/stories/*.md` name more than one distinct BC ID within a single row. EC-037 + new Canonical Test Vector row added; Gate Spec `run_part_a_arm2` Phase 1 comment updated (TD-VSDD-060 sibling-site sweep). **ADV-RECON-007 (LOW)**: invariant 4 extended to cover primary-target reads that succeed as bytes but fail UTF-8 decoding (previously undefined by this BC; the shipped implementation had filled the silence by mis-citing invariant 9, which governs slicing safety only, not read-decode disposition). New precondition 15a and postcondition 25 require `HookResult::block_with_fix(...)` on primary-target decode failure under the same fail-closed posture as invariant 4; the invariant-9 misattribution is named NON-CONFORMING. EC-038 added. No BC H1 change; no ID renumbering (POLICY 1 append-only). BC-INDEX version-cell sync owed to state-manager; S-21.07 story BC-table/Token Budget cite + new ACs owed to story-writer (not amended in this burst). (product-owner; closes ADV-RECON-003, ADV-RECON-007.) |
| 1.19 | 2026-08-13 | §VP Anchors count-reconciliation sweep (S-21.07 pass-18, F-S2107-P18-001, MEDIUM — POLICY 5 category-(i) + POLICY 4): §VP Anchors stated "VP-102 through VP-118 (17 VPs)" but §Verification Properties enumerates 19 rows (Class A: 8; Class B: 4; Class D: 3 DEFERRED; Class E: 4) — count grew 17→19 at v1.12 (two advisory-path rows added per F-P6-001 Option 1: "A Arm1 Primary-Newer-than-Index Advisory (PC2a)" and "B Arm1 STORY-INDEX-Consistent Advisory (PC13a)") but §VP Anchors was never swept (TD-VSDD-060 sibling-site gap). Cross-checked against S-21.07's Token Budget, which already correctly cites "19 VPs" — 19 confirmed canonical. §VP Anchors corrected to "VP-102 through VP-120 (19 VPs)": both v1.12 advisory-path rows are first-class properties eligible for the same VP-INDEX allocation as the original 17 (range expansion, not an allocatable/advisory-only split). Full BC count-parity sweep performed (TD-VSDD-060): §Edge Cases table confirmed at 36 rows (EC-001–EC-036), matching S-21.07's "36 ECs" cite — no change; this BC has no BC-scoped ACs section (ACs are story-scoped) so no "N ACs" claim required reconciliation against S-21.07's "24 ACs" cite — the historical AC-001–AC-021 text in §Preconditions Part A Arm2 is a fixed corpus-example illustration, not a count assertion, left unchanged; Class A/B/D/E sub-counts (8/4/3/4) asserted nowhere else in the BC. No behavioral/normative change — §Verification Properties rows, §Postconditions, §Preconditions, §Edge Cases all unchanged; this is a count/reservation-statement correction only. (product-owner; closes F-S2107-P18-001.) |
| 1.18 | 2026-08-13 | §Gate Spec fuel-cap present-perfect claim corrected (S-21.07 pass-10, F-S2107-P10-004, HIGH — TD-VSDD-059 + POLICY 11 analog): v1.16/v1.17 asserted the cap "has been raised to 20,000,000" and the margin gate was "satisfiable at HEAD" in present perfect; verified FALSE at the review's frozen snapshot `5370db80` (both `RegistryDefaults::default()` and `InvokeLimits::default()` still `fuel_cap: 10_000_000` there) — the gate was tautological (fuel_consumed clamped to cap; `10,000,000 < 12,000,000` unconditionally on every reachable path). Stale-baseline re-verification (production-grade default) confirmed PR #774 (`62fbcf1a`; ADR-042 §Decision 1) has since merged to `develop` and is an ancestor of current HEAD: `DEFAULT_FUEL_CAP = 20,000,000`, both defaults source it, pinned by `fuel_cap_defaults_stay_in_sync`. Present-perfect claim now TRUE at source-HEAD; source-HEAD-vs-operator-effective distinction added (marketplace-cached dispatcher binary remains pinned to 10,000,000 through rc.23; not operator-effective until rc.24 is consumed). Margin-gate framing rewritten to state the precise non-tautological failure condition at 20,000,000 (`fuel_consumed` in `(12,000,000, 20,000,000)`: completes successfully but assertion fails) versus the empty failing region at the former 10,000,000 cap. No BC-table row / H1 / priority change. (product-owner; closes F-S2107-P10-004.) |
| 1.17 | 2026-08-08 | §Gate Spec placement-sensitivity correction: retracted v1.15-erratum "falsified" characterization and v1.16 "two compounding errors / ~22× overestimate" claim — the ~110-row figure was derived from an append-scenario measurement against the former 10M cap and is correct for that scenario. Both runway figures documented normatively: append (~146 rows under 10M cap, ~541 fuel/row; exhaustion observed between 100 and 130 appended rows in §SDK Grounding Evidence; ~18,600 rows under 20M cap) and insert-before (~4–5 rows under 10M cap, ~15,761 fuel/row single-row / ~17,114 fuel/row 4-row avg; ~589 rows under 20M cap); ratio ~29× (placement-driven). Margin gate `≤ 12,000,000` rationale restated with explicit governing scenario (insert-before, adversarial). v1.16 stale STORY-INDEX cite (v4.290) corrected to v4.291. Characterization errors originated with orchestrator analysis, not product-owner authoring; v1.17 supersedes both v1.15-erratum and v1.16 characterizations. 4-index: BC-INDEX v4.55 / VP-INDEX v2.76 / STORY-INDEX v4.291 / ARCH-INDEX v3.51. (product-owner.) |
| 1.16 | 2026-08-08 | §Gate Spec fuel corrections (ADR-042, F-S2107-P9-002): global cap raised 10M→20M (ADR-042 §Decision 1); margin gate updated to `fuel_consumed ≤ 12,000,000` (60% of 20M cap; satisfiable at HEAD — current consumption 9,920,913 < 12,000,000; ~121 SS-05 rows before gate fires, ~468 rows before exhaustion at 20M); ~110-row runway figure corrected — true runway 4 rows safe, 5th exhausts for SS-05-sized entries (~486 bytes/row); early-return scope qualifier added (extract_bc_index_version_state returns at row 921; only rows 1–920 scanned; rows beyond cost nothing); two compounding errors documented (scan-region mismatch: used total 1,985 rows instead of 921 pre-BC-5.39.010 scan rows; row-size mismatch: used ~155 bytes/row average instead of ~486 bytes/row for SS-05 entries; combined ~22× overestimate); ERRATUM block removed (measurement integrated into normative text); fuel_cap prohibition lifted per ADR-042 §Decision 2 — SHOULD set per-plugin cap once ADR-039 Phase 1 ships (p99×1.5, min 50M for Phase 3 fail-closed annotation); exhaustion visibility requirement re-scoped to ADR-042 §Decision 3 class (a): observable signaling (fuel_exhausted=true in dispatcher stderr + advisories[] entry), not on_error escalation — pending-implementation obligation, not spec gap. Changelog v1.15-erratum row preserved. 4-index: BC-INDEX v4.54 / VP-INDEX v2.76 / STORY-INDEX v4.291 / ARCH-INDEX v3.51. (product-owner; closes F-S2107-P9-002.) |
| 1.15-erratum | 2026-08-08 | ERRATUM (D-963): `~110 rows` runway figure falsified by direct measurement. True: 4 rows safe, 5th exhausts for SS-05-sized (~486 bytes/row) entries; ~17 rows for shorter entries. `extract_bc_index_version_state` early-returns at row 921 — only rows 1–920 scanned; rows beyond cost nothing. Two compounding errors recorded (total-index vs scan-region; average vs SS-05 row size). Linear, not O(n²): R²=0.998790 (quadratic coefficient negligible). Silent-in-production: `plugin.timeout` logged internally but dispatcher exits 0/empty — live agents receive no signal. Correction notice inserted before normative fuel text. v1.16 from product-owner pending. (state-manager, D-963.) |
| 1.15 | 2026-08-08 | §Gate Spec fuel-exhaustion section corrected (F-S2107-P9-002): false calibration claim "calibrated to bound reads inside the fuel budget" retracted — 1 MiB PC4 cap is ~76% above the ~594 KB exhaustion point; measured fuel 9,920,913/10,000,000 (99.21% consumed) at 576,396 bytes/1,985 rows; headroom 79,087 fuel (0.79%); exhaustion threshold 593,525–601,548 bytes (~110 additional BC-INDEX rows). Normative requirement added: bats gate MUST assert `fuel_consumed ≤ 6,000,000` (margin gate, fires on approach) AND fixture sha256/row-count drift gate (prevents stale snapshot). fuel_cap paragraph updated: evidentiary precondition now confirmed by 2026-08-08 measurement; prohibition retained per human ruling 2026-08-08 — operator chose margin-gate over cap increase, `fuel_cap` remains unauthorized. Operational consequence stated: exhaustion expected within ~110 rows of ordinary registration traffic, now surfaces loudly via margin gate, not silently. §SDK Grounding Evidence: fuel measurement subsection added. (product-owner.) |
| 1.14 | 2026-08-07 | PC5 corpus figures corrected (three stale claims falsified by same burst's leg D: registered two BCs + escaped BC-4.13.001 annotation pipes before leg C's spec was re-measured): RowPresentNoVersion count 1,943/1,983 → 1,945/1,985 (structural invariant documented; verbatim stdout in §SDK Grounding Evidence). Old-algorithm non-conformance count corrected two-of-four (not three): BC-4.13.001 oldalg now equals first-of-last after pipe-escaping; BC-3.08.001 and BC-7.03.079 still differ. fields[5..].join("\|") reassembly arm adjudicated DEFENSIVE: zero live n>6 rows at 2026-08-07; step retained for future bare-pipe annotation rows; pinning test reported to test-writer. PC40 77→76 at two sites (ADR-037 §Context corrected in same burst; BC not swept per POLICY 5 v1.3.4 SIBLING-SWEEP); ADR-037 v1.2 version pins replaced with ADR-037 §Context anchor form per TD-VSDD-091. §SDK Grounding Evidence section added. Refs: F-S2107-P8-005, F-S2107-P8-010. (product-owner.) |
| 1.13 | 2026-08-06 | PC5 escape-aware splitting paragraph corrected (ADR-038 §Decision 4 Change 1): "yields exactly 6" replaced with "yields 6 for most rows but MAY yield more for bare-`\|` annotation rows"; `fields[5..].join("\|")` reassembly step REQUIRED; corpus re-verified at 2026-08-06: 5-field 1943 and total 1983 confirmed unchanged from v1.8 measurement; 6-field figure refined to 39 six-field + 1 nine-field (BC-4.13.001) = 40 rows with n≥6. PC5 Version(v) bullet extraction algorithm replaced (ADR-038 §Decision 4 Change 2): rightmost-of-field[5] → first-token-of-last-chain-entry (join fields[5..] with `\|`, split on `\x00`, take last non-empty entry, extract first v-token); "The 6th field is the version-chain cell" removed (inaccurate for bare-pipe rows); rightmost-of-field[5] marked NON-CONFORMING with four-row empirical proof. PC6 Version(v) route replaced (ADR-038 §Decision 4 Change 3): same algorithm; "rightmost token is always the current" removed; rightmost-of-cell algorithm marked NON-CONFORMING. PC13c added (ADR-038 §Decision 4 Change 4): half-present case — exactly one of {B2, B3} present-and-differing, other absent → advisory + Continue per PC12 inclusive-or; MUST NOT block for (Some(b2), None) or (None, Some(b3)); live instances S-18.11/S-18.12. Gate Spec run_part_b_arm1 extended with 13c bullet and NON-CONFORMING note updated. Gate Spec run_part_a_arm1 pseudocode comment updated with algorithm reference. Architecture Anchor for extract_bc_index_version_state: Version(v) description updated with first-token-of-last-chain-entry algorithm and NON-CONFORMING note. EC-036 added (half-present advisory + Continue). Canonical test vector added (B Arm1 — half-present PC13c). Refs: F-S2107-P7-004, F-S2107-P7-017, F-S2107-P7-008, ADR-038. (product-owner; ADR-038 §Decision 4 routing.) PC13 Phase 2 algorithm replaced (ADR-038 §Decision 5 / §Decision 4 Change 5): reverse-field (rightmost-first) → BC-ID-anchored first-v-token (locate anchor field containing BC ID by word-boundary test identical to `line_contains_bc_id_at_boundary`; return FIRST `\bv([0-9]+\.[0-9]+)\b` token AFTER BC ID position in that field); return None if no field contains BC ID + subsequent v-token. Reverse-field algorithm NON-CONFORMING per ADR-038 §Decision 5: (a) S-15.17 BC-5.39.009 wrong answer — rightmost returns v1.3 from `POLICY 5 v1.3.6` annotation prose, correct is v1.9; (b) S-4.08 BC-9.01.002 cross-BC contamination — returns v1.1 from a field about BC-9.01.001. Corpus count updated: 30 rows (2026-08-04, stale) → 67 Phase 2 rows / 44 containing BC IDs (2026-08-06); same defect class as PC5 count corrected by ADR-038 §Decision 4 Change 1. Architecture Anchor for `extract_story_bc_version_citations` updated with BC-ID-anchored algorithm and NON-CONFORMING note. Gate Spec pseudocode comment updated. Phase 1 corpus date updated (2026-08-04 → 2026-08-06; count 58 confirmed unchanged per ADR-038 §Empirical Measurement v1.1). Refs: ADR-038 §Decision 5. (product-owner; ADR-038 §Decision 4 Change 5 routing.) |
| 1.12 | 2026-08-05 | EC table aligned with v1.11 normative postconditions (F-P6-001 Option 1 propagation): EC-002 corrected (primary-newer → advisory + Continue per PC2a; was Block — incorrect); EC-005 block trigger clarified as B2≠B3 per PC13b (was "B2 ≠ B1" — misleading); EC-019 updated (Class A Arm1 primary-newer is advisory, not a violation; output is advisory + E1 block, not combined-block); EC-034 added (PC13a advisory: B2==B3/B1≠B2 → advisory + Continue); EC-035 added (PC2b block: index-newer-than-primary → Block). Canonical Test Vector "A Arm1 — stale" corrected to advisory + Continue per PC2a; PC2b block mutant added. Gate Spec run_part_b_arm1 prose extended with v1.11 PC13a/13b split (prior prose described only PC40/v1.5 change). VP table updated: "A Arm1 Stale-Index Block" → "A Arm1 Index-Newer-than-Primary Block (PC2b)"; new VP entries for PC2a advisory and PC13a advisory added. Refs: F-P6-001, Option 1. (product-owner.) |
| 1.11 | 2026-08-05 | PC2 directional carve-out (F-P6-001 Option 1): primary-newer-than-index → advisory ("state-manager index update pending; Class A BLOCK suspended"); index-newer-than-primary → retains BLOCK (anomalous). PC13 two sub-cases (F-P6-001): B2==B3/B1≠B2 → advisory (POLICY 3 artefact); B2≠B3 → retains BLOCK (STORY-INDEX internal inconsistency). Both carve-outs mirror PC3/PC12 POLICY 3 ordering rationale. PC4a verbatim assertion strengthened (F-P6-002): test-writer MUST assert COMPLETE formatted string by equality check; .contains()-only is NON-CONFORMING. PC5 ≥6-field/no-v-token state normatively defined as RowPresentNoVersion (F-P6-018); empty-cell counting confirmed canonical. PC36 corpus updated 2→3 (F-P6-012): ADR-037 adopted last_amended: |- in v1.1 amendment; grep -rl confirmed 3 files. PC40 transitional clause corrected (F-P6-007): both conditions satisfied in pass-5 burst; PC40 vacuous for S-21.07; "widening" characterization WITHDRAWN — ARCH-INDEX.md in is_volatile_path is normative per ADR-037 §Decision 2; 77 stories remain. Architecture Anchors (F-P6-004): extract_bc_index_version_state replaces deleted extract_bc_index_version; is_volatile_path + parse_story_volatile_inputs replace non-existent check_volatile_inputs. §Traceability + §Story Anchor: v1.4→v1.5 (F-P6-005). §VP Anchors: D-945 basis documented; erratum added to v1.1 row (F-P6-024). Gate Spec: PC2a/PC2b Version arm split; PC13a/PC13b B1 split; check_volatile_inputs reference corrected. (product-owner; S-21.07 pass-6 fix burst.) |
| 1.10 | 2026-08-05 | PC5 self-contradiction fixed (F-S2107-P4-022): split into two-level locator/body-table predicates; `RowMalformed` redefined as "locator-matched line (conditions (1)+(2)) with <5 fields". PC5 candidate-selection order added (F-S2107-P4-005): full-file scan must prefer first valid (≥5-field) locator-matched line; first-match-wins on malformed is NON-CONFORMING. Postcondition 4a pinned normatively (F-S2107-P4-025): prescribed advisory text is MUST-verbatim; omitting operator-actionable clauses is NON-CONFORMING. Postcondition 13 expanded with three-category enumeration (F-S2107-P4-006 ruling: hook MUST enumerate stale/fabricated/algorithm-divergent as possibilities without classifying; `classify_provenance` heuristic is NON-CONFORMING; AC-009 stops requiring classification; invariant 11 governs). Invariant 11 SHOULD→MUST. Postcondition 22 prescribed message added citing POLICY 14 leg 3 (F-S2107-P4-008 sibling sweep: all other leg citations confirmed correct — postconditions 2 and 7 both already cite leg 5 correctly; postcondition 20 cites leg 4 correctly). PC36 block-scalar normative requirement: `extract_frontmatter_field` MUST handle `\|-` block scalars; returning `"|-"` is NON-CONFORMING (F-S2107-P4-004 coupling declared; corpus: 2 occurrences). PC40 transitional clause updated: "no permanent weakening" guarantee requires exhaustive ADR-037 §Context; S-21.07 absent from that table; architect-routed (F-S2107-P4-013 BC-side ruling). Gate Spec pseudocode updated for two-level predicates and selection order. Architecture Anchors: `extract_bc_index_version` description updated to v1.10 terminology; `extract_frontmatter_field` block-scalar requirement added. Story Anchor and §Traceability Stories updated TBD→S-21.07. (product-owner; pass-5 fix burst.) |
| 1.9 | 2026-08-04 | PC5 fourth state `RowMalformed`: a candidate line matching the locator pattern (`^\| \[<id>\]` or `^\| <id> \|`) was found but has <5 non-empty fields after escape-aware splitting — it is NOT a valid body-table row (likely a Changelog entry, subsystem-section row, or notes table that incidentally carries the BC ID link). `RowMalformed` disposition: advisory + Continue; NEVER reaches postcondition 4 blocking path. Narrows `RowAbsent` to exclusively mean "no candidate line found at all." Normative body-table row recognition predicate: first-cell link/plain match AND field count ≥5 both required for valid body-table classification. Corpus-validated 2026-08-04: 0 RowMalformed lines in real BC-INDEX (all 1,983 BC-ID-matching lines have ≥5 fields); forward-looking protection. Postcondition 4a added. Gate Spec `run_part_a_arm1` match extended to four arms. (product-owner; resolves internal contradiction discovered by implementer during v1.8 implementation.) |
| 1.8 | 2026-08-04 | PC5 column-anchored locator: state classification changed from token-search-based to escape-aware column-count-anchored — split by unescaped `|` (treating `\|` as non-splitting), count non-empty fields: 5 → RowPresentNoVersion unconditionally; ≥6 → Version(v) from 6th column. Token-search approach was NON-CONFORMING: 194 of 1,943 canonical rows carry story IDs (e.g., `S-15.01`) in the Stories column whose digits match bare `\bv?([0-9]+\.[0-9]+)\b` → Version("15.01") false-BLOCK; 194 is the load-bearing corpus count. Corpus-validated: 1983 total / 1943 five-field / 40 six-field / 194 story-ID hazard rows. PC6 updated: Version(v) extraction uses rightmost `\bv([0-9]+\.[0-9]+)\b` match in 6th field. PC13 two-phase algorithm replaces LAST rightmost pipe-field algorithm (prior optional-v form NON-CONFORMING): Phase 1 pure-version field (`^v?[0-9]+\.[0-9]+$`, 58 BC-section rows); Phase 2 mandatory-v inline (`\bv([0-9]+\.[0-9]+)\b`, 30 Token Budget rows); eliminates: (1) story-ID collision 29 rows/6 stories, (2) BC-section-number collision in Token Budget rows, (3) ACs-column collision 1 row S-21.07/BC-5.39.010. Gate Spec pseudocode and Architecture Anchor updated. PC40 ruling and ALGORITHM-DIVERGENT corrections from v1.7 retained. (product-owner.) |
| 1.7 | 2026-08-04 | PC5 corrected: BC-INDEX canonical shape 5-column (`| BC ID | Title | Status | Capability | Stories |`); version-chain cell is ad-hoc 6th column present on only 40 of 1983 body-table rows (corpus 2026-08-04, adversary pass-3 verified); `extract_bc_index_version` rearchitected to three-state return `RowAbsent` / `RowPresentNoVersion` / `Version(v)` — prior two-state `Option<String>` conflating `RowAbsent` and `RowPresentNoVersion` into a single `None` produced ≥1,712 false BLOCKs per F-S2107-P3-001. Postcondition 4 expanded: `RowAbsent` + version > "1.0" → block (unchanged, genuine structural fault); `RowPresentNoVersion` → silent-continue (5-column canonical shape is standard for ~98% of rows; advisory would be unactionable noise). Part B postconditions note and invariant 11 corrected: `1acf3c6` reclassified from fabricated to ALGORITHM-DIVERGENT per ADR-036 §Decision 4 — produced by rc.23 CACHE binary (trailing-newline-stripping algorithm), not fabricated; no PROVENANCE-BREAK annotation was warranted; Pass-30 M02 POLICY 18 violation claim for `1acf3c6` retracted. Invariant 11 updated to three-category taxonomy. Gate Spec `run_part_a_arm1` pseudocode updated to reflect three-state match. PC40 confirmed conformant — F-S2107-P3-002 is implementation non-conformance, not a spec defect; no PC40 amendment warranted. (product-owner; F-S2107-P3-001 spec-side; ADR-036 §Decision 4 annotation corrections per D-952.) |
| 1.6 | 2026-08-04 | Class D (finding-ID namespace advisory in Closes/Refs lines) descoped entirely per human-approved scope decision 2026-08-04. Active gated classes: A, B, E only. `is_cycle_artifact` dispatch branch marked DEFERRED — must not be compiled into v1.6 hook. `.factory/cycles/` removed from registry `path_allow`. Rationale: Closes/Refs convention is unstandardized (six shapes across both cycle burst-logs; PC31 failed three iterations: v1.2 plain-colon→0 matches, v1.3 bold-bare-colon→20/34, v1.5 bold-word-boundary-colon→86/96 bold but 0/20 non-bold); v1.5 measurement taken against wrong cycle (`v1.0-feature-engine-discipline-pass-1` while active cycle is `v1.0-brownfield-backfill`). PC28-PC33 and postconditions 16-18/24 and invariant 6 marked DEFERRED; IDs preserved per POLICY 1 append-only. Knowledge carried forward in §Deferred Scope with follow-up story target S-21.08 (E-21 epic). Class A, B, E amendments from v1.4-v1.5 (PC34 VP-path correction, PC40 volatile-input precondition, invariant-6 adjudication) survive intact. (product-owner; human-approved scope decision.) |
| 1.5 | 2026-08-04 | Amendment 1 (PC31): Closes/Refs regex corrected — bare-colon `^\*\*Closes:\*\*` matched only 20 of 34 burst-log Closes lines (corpus 2026-08-04; 14 missed due to parenthetical/bare-word qualifiers); new form `^\*\*Closes\b[^:]*:\*\*` covers all 34; Refs=0 corpus instances, forward-looking; PC31a scope-count advisory added (postcondition 24) so inert scope is observable. Amendment 2 (PC34): VP path corrected from `ss-*/VP-*.md` (inert for all 102 VPs) to flat `^VP-[0-9]+\.md$` with VP-INDEX.md exclusion (corpus 2026-08-04: zero ss-* dirs, 102 VPs flat); epics clause added (dispatch.rs carried it without PC34 counterpart; closes spec gap). Amendment 3 (invariant 6 adjudication): CapabilityDenied/Timeout on cycle artifact remain BLOCKING per PC33/postcondition 18/invariant 5 — invariant 6 scopes to finding-content verdicts only; postcondition 18 expanded to include Timeout; non-conforming swallow-all error arm named explicitly. Amendment 4 (PC40): volatile-input precondition for Class B Arm B1 per ADR-037 §Decision 4 — scan story inputs: for volatile patterns (decision-log/lessons/burst-log/STATE.md/catalog indexes); emit prescribed advisory + Continue if found; transitional clause (vacuous once 19-story remediation complete, per ADR-037 §Decision 5); EC-032 + test vector added. (product-owner; S-21.07 LOCAL adversary pass-2 fix.) |
| 1.4 | 2026-08-03 | PC13 amended: bounding-section heading-match predicates changed from exact equality to prefix-with-word-boundary (`^## Behavioral Contracts\b`, `^## Token Budget\b`). v1.3 named exact heading text which the implementer rendered as `heading == "Behavioral Contracts"` and `heading == "Token Budget"` — causing 133 of 144 production stories (those using `## Token Budget Estimate` or `## Token Budget Estimate (MANDATORY)`) to be skipped, making stale Token Budget citations invisible. Corpus check (2026-08-03) against `.factory/stories/*.md` confirmed all measured variants covered and zero false positives: no other `^## ` heading in the corpus matches either predicate; `## Edge Cases` (148 occurrences) begins with `Edge` and remains excluded, preserving the ≥9-spurious-block regression fix from v1.3. Architecture Anchor for `extract_story_bc_version_citations` updated. Pseudocode scoping comment updated. Explicit non-conformance note added to PC13 body. (product-owner; S-21.07 LOCAL adversary cascade pass-1b fix.) |
| 1.3 | 2026-07-30 | PC13 amended: bounding section added — scan confined to `## Behavioral Contracts` and `## Token Budget` sections only; unbounded scan caused ≥9 spurious blocking violations on stories that document BC edge cases in a table (Edge Cases rows carry `BC-5.39.010 EC-0NN` + prose `v1.x` tokens). Dual version-token format: `\bv?([0-9]+\.[0-9]+)\b` matches both bare `1.2` (body BC-table) and v-prefixed `v1.2` (Token Budget rows); prior regex `\bv([0-9]+\.[0-9]+)\b` was unreachable for bare form. LAST rightmost pipe-field algorithm stated explicitly (was already mandated but not algorithmic). Amendment 2 (PC31): bold-markdown form `**Closes:**`/`**Refs:**` required to match D-444(c) real burst-log format; prior plain-colon `^Closes:\s*` matched zero real burst-log lines; union scan (not `else if`) required so compound lines carrying both markers (e.g., `**Closes:** F-X ... **Refs:** B01`) must scan both segments. Amendment 3 (PC38 + postcondition 21): non-decreasing relation stated explicitly as `∀i: date[i] ≤ date[i+1]`; equal same-day dates PERMITTED (not a violation); prior "ascending" wording admitted strict-comparison re-implementation that would self-violate on this BC's own `modified[]`; EC-030/031 added; test vectors for equal-dates and genuine-decrease added. Amendment 4: no spec change — PC29 (`max_bytes = 2097152`) and PC33 (NotFound → advisory+Continue on cycle artifact) already unambiguous; implementation used wrong 1 MiB constant and wrong NotFound handling, both purely implementational faults. Amendment 5 (PC32): `O-` observation IDs deliberately non-excluded per D-449(d)(i) which scopes Closes/Refs to findings, PG, D-NNN only; ruling made explicit to prevent ambiguity. POLICY 14 five-leg parity; v1.1 `modified[]` entry restored (was missing — irony: this hook verifies `modified[]` monotonicity but not `modified[]`↔Changelog row correspondence, so it structurally cannot catch this defect in its own governing BC). (product-owner; S-21.07 LOCAL adversary pass-1 fix-burst.) |
| 1.2 | 2026-07-30 | Registry entry corrected: `tools = [...]` array replaced with `tool = "^(Edit\|Write\|MultiEdit)$"` regex string (field name singular + MultiEdit added; all 41 Edit/Write hooks in live registry guard this pattern; omitting MultiEdit was a POLICY 13 ESCAPE-SCOPE-PARITY gap identical in class to F-S2104-P29-H02). Fuel-exhaustion note added to Gate Specifications per ADR-035 §Decision 5: `on_error = "continue"` silences the hook non-blockingly on fuel exhaustion (not WASM-side logic); `max_bytes` caps bound reads inside the fuel budget; `fuel_cap` not required. BC-version-pin datum-copy ruling added as design note in §Postconditions Part A Arm2. (product-owner.) |
| 1.1 | 2026-07-30 | Part A Arm2 (story-file-side trigger) added: PostToolUse on story → read each `behavioral_contracts:` BC via deterministic path derivation → compare against story version citations (Token Budget + BC-table rows). No list_dir required. Latency gap correctly sized (all 6 observed failures occurred during story-editing bursts). Advisory rationales made explicit for every advisory arm; confirmed no arm is advisory merely because the check is partial. Class D tokenizer namespace-exclusion list added (D-, S-, BC-, VP-, R-, L-, ADR-, EC-, NFR-, ASM-, FM-); EC-024 rationale corrected (D-944 matches shape BUT is excluded by namespace list). Invariant 11 added: stale vs fabricated hash provenance — stale = sweep fix, fabricated = POLICY 18 acknowledgment required (pass-30 M02 precedent). EC-026/027/028/029 added. Gate Spec updated with run_part_a_arm2 pseudocode and story-file dispatch branch. *[Erratum v1.11 (F-P6-024): "VP table extended to 17 entries" was a planning annotation — VP-102..VP-118 (17 VPs) were anchored to S-21.07 post-merge per D-945, but no VP table was written into the BC body at v1.1; §VP Anchors placeholder correctly reflects pending state-manager allocation.]* |
| 1.0 | 2026-07-30 | Initial authoring (product-owner; pre-pass-30 fix-burst). Classes A Arm1/B/D/E gated; Class C honest-gap + Rust test recommendation. |
