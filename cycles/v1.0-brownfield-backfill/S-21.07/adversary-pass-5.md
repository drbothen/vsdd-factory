# Adversarial Review — S-21.07 LOCAL cascade, Pass 6

```yaml
---
review_type: local-story-adversarial
story_id: S-21.07
cycle: v1.0-brownfield-backfill
pass: 6
passes: 6
reviewed_head: "b78b27ef"
reviewed_branch: feature/S-21.07-validate-cross-site-correspondence
worktree: /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07
story_version: "1.5"
bc: "BC-5.39.010 v1.10"
adrs_read: [ADR-035, ADR-036 (cited only), ADR-037 v1.1]
verdict: NOT-CLEAN
findings_count: 24
severity_breakdown:
  BLOCKER: 3
  HIGH: 8
  MEDIUM: 10
  LOW: 3
observations_count: 5
streak: 0/3
trajectory_append: 24
trajectory: 47 → 18 → 25 → 25 → 24
input_hash_basis:
  story_frontmatter_declared: "dd5c9d2"
  story_index_catalog_row: "25c7324"
  story_index_blockquote: "25c7324"
  mechanically_verified: false
  note: >
    Read-only profile — `bin/compute-input-hash` could not be invoked, so the
    `25c7324 → dd5c9d2` claim in story v1.5 `last_amended` is NOT mechanically
    verified (POLICY 18 remains OWED). The three-way inequality is live and is now
    BLOCKING for S-21.07 because ARCH-INDEX.md was removed from `inputs:` in the
    same burst, retiring PC40's suppression. See F-S2107-P6-001.
prior_pass_records_read:
  - adversary-pass-1.md (Part A only)
  - adversary-pass-2.md (Part A only)
  - adversary-pass-3.md (Part A only)
  - adversary-pass-4.md (Part A only)
  - adversary-pass-5.md — ABSENT from .factory/cycles/v1.0-brownfield-backfill/S-21.07/ (see F-S2107-P6-024)
---
```

---

## Part A — Findings

### BLOCKER

---

#### F-S2107-P6-001 — BLOCKER — The hook's four blocking postconditions are evaluated at the primary-write instant, but POLICY 3 forces the secondary sites to be updated LAST. Both governing artifacts are live-blocked right now, and the pass-5 burst removed the only shield.

**Location:**
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md:1461` (6th escape-aware field = `v1.9`)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md:4` (`version: "1.10"`)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/stories/STORY-INDEX.md:730` (`input-hash 25c7324`), `:733` (`S-21.07=25c7324`)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/stories/S-21.07-validate-cross-site-correspondence.md:15-23` (`inputs:` no longer volatile; `input-hash: "dd5c9d2"`)
- `.../src/arm_a1.rs:381-396`, `.../src/arm_b.rs:217-247`, `.../src/arm_b.rs:404-424`

**Clause violated:** BC-5.39.010 postconditions 2, 4, 13 vs. postconditions 3 and 12 rationales; PC40 transitional clause; POLICY 3 (`state_manager_runs_last`); POLICY 14 leg 5 / POLICY 17.

**Evidence — Arm A1, live.** BC-INDEX row for the governing BC:

```
| [BC-5.39.010](ss-05/BC-5.39.010.md) | validate-cross-site-correspondence WASM hook MUST block … | draft | E-12 | S-21.07 | v1.9 |
```

Escape-aware split → 6 non-empty fields → `extract_last_v_token("v1.9")` → `Some("1.9")` → `Version("1.9")`. BC frontmatter is `1.10`. `arm_a1.rs:383` `index_version == bc_version` is false → `Violation`. Every write to `BC-5.39.010.md` blocks.

**Evidence — Arm B1, live and newly unshielded.** Story `inputs:` after the pass-5 edit contains only `BC-5.39.010.md`, `ADR-035-*.md`, `SS-04-*.md`, `SS-05-*.md`, `hooks-registry.toml`, `host.rs`, `result.rs`. Traced through `is_volatile_path` (`arm_b.rs:336-366`): none equals `.factory/STATE.md`, none contains `.factory/cycles/`, none equals the three index paths → `volatile_found` is empty → `run_arm_b1` proceeds to the three-way check (`arm_b.rs:413`). B1=`dd5c9d2`, B2=`25c7324` (STORY-INDEX:730), B3=`25c7324` (STORY-INDEX:733) → `!b2_match || !b3_match` → `Violation` (`arm_b.rs:221`). Every write to `S-21.07-validate-cross-site-correspondence.md` blocks.

**Why this is a spec defect, not a data-staleness defect.** This is the fourth consecutive pass where the same shape is live (F-S2107-P3-003, F-S2107-P4-001, F-S2107-P4-002, F-S2107-P4-012). Sweeping the data has not stopped it, and it cannot: the BC itself documents the ordering that guarantees it. Postcondition 3's rationale states verbatim: *"Blocking would make correct BC authoring impossible — the BC file is always written before the INDEX row."* Postcondition 12's rationale states the same for Class B: *"Blocking on absence would cause systematic false positives in correct new-story authoring bursts."* The BC applied that reasoning **only to the absence cases** (`RowAbsent`+v1.0; B2/B3 absent) and **not to the staleness cases** (postcondition 2 `Version(v)` mismatch; postcondition 13 present-but-differs) — which are burst-ordered in exactly the same way, and under POLICY 3 are *guaranteed* to be stale at the instant the primary write fires, because state-manager owns BC-INDEX and STORY-INDEX and commits last.

Consequence: for each of the 40 BC-INDEX rows carrying a version-chain cell, and for every story whose `input-hash` changes, a conforming authoring burst produces a spurious `exit 2`. `BC-5.39.010.md` is one of the 40. This is the same self-lock class that ADR-037 §Rationale ("The self-locking risk") identifies for Class B, left unclosed for Class A and for the *stale* (as opposed to *absent*) Class B branch.

**Scheduled-vs-defect adjudication (requested by the brief).** The two *data* legs are legitimately owed-but-unapplied: BC-INDEX's last recorded bump is `v4.47` covering `v1.6→v1.9` for pass-4 (BC-INDEX.md:16), so the `v1.10` cell and the `dd5c9d2` STORY-INDEX legs are state-manager Commit-D work for this burst. **The defect is not the staleness — it is that the gate's blocking predicates are unsatisfiable at their own trigger instant under POLICY 3.** Sweeping the data closes today's instance and re-opens next burst. Product-owner routing: extend the postcondition-3/12 carve-out reasoning to the staleness branches (e.g. downgrade `Version(v)`-mismatch and B2/B3-stale to advisory when the primary artifact's own version/hash is *newer* than the index cite, reserving BLOCK for the reverse direction), or move Arm A1/B1 blocking to the index write (Arm B2's trigger point) where both operands are settled.

**Additional aggravating fact:** the pass-5 burst removed `ARCH-INDEX.md` from the story's `inputs:` (closing F-S2107-P4-013's story side) **in the same burst that left the STORY-INDEX hash legs stale**, converting a suppressed advisory into a live BLOCK on the story's own file. The remediation ordering was inverted: the `inputs:` cleanup must follow, not precede, the hash propagation.

---

#### F-S2107-P6-002 — BLOCKER — Postcondition 4a is MUST-verbatim with `<id>`/`<N>` as the only substitutions. The shipped advisory drops a whole normative sentence, alters four phrases, and injects three sentences. F-P4-003/F-P4-025 were closed test-shaped, not spec-shaped.

**Location:** `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07/crates/hook-plugins/validate-cross-site-correspondence/src/arm_a1.rs:406-417` vs. `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md:584-594`

**Clause violated:** BC-5.39.010 postcondition 4a — *"**NORMATIVE — implementation MUST reproduce this text verbatim**, with `<id>` and `<N>` as the only interpolated substitutions"*. POLICY 15; TD-VSDD-059.

**Evidence.** Spec text (BC:586-590):

```
validate-cross-site-correspondence [Class A Arm1]: BC-INDEX.md contains a malformed
candidate line for <id> (<N> fields found; expected ≥5 for a valid body-table row). This
line is structurally not a BC-INDEX body-table row (likely a Changelog entry or notes
table). Registration status cannot be determined from this line. Verify BC-INDEX body-table
registration manually.
```

Shipped text (`arm_a1.rs:408-416`):

```
validate-cross-site-correspondence [Class A Arm1] advisory: malformed candidate line for
'{bc_id}' ({field_count} non-empty fields found; expected ≥5 for a valid BC-INDEX body-table
row). Registration status cannot be determined from this line. Verify BC-INDEX body-table
registration manually. Not blocking — this is not a dropped registration. The genuine
dropped-registration case (no candidate line at all) is RowAbsent (postcondition 4).
BC-5.39.010 v1.10 PC5 postcondition 4a.
```

Deltas: (1) sentence 2 of the normative text — *"This line is structurally not a BC-INDEX body-table row (likely a Changelog entry or notes table)."* — is **absent entirely**; (2) `[Class A Arm1]:` → `[Class A Arm1] advisory:`; (3) `BC-INDEX.md contains a` prefix dropped; (4) `<N> fields found` → `{field_count} non-empty fields found`; (5) `a valid body-table row` → `a valid BC-INDEX body-table row`; (6) `<id>` rendered as `'{bc_id}'` (quotes are not a permitted substitution); (7) three sentences injected.

**Why the closure is test-shaped.** The four new unit tests (`arm_a1.rs:1049-1136`) and the new bats fixture (`bats:1279-1309`) assert exactly two `.contains()` substrings — the two clauses the pass-4 finding named. **No test asserts the full verbatim string, and no test asserts the absence of injected text**, so the MUST-verbatim requirement the BC added in v1.10 is pinned by nothing. The implementer added the two greps the tests demanded rather than the text the spec prescribed. Per the brief's item 2: the tests exist but are not load-bearing on the requirement they claim to close.

---

#### F-S2107-P6-003 — BLOCKER — F-P4-014's closure attestation is self-refuting: `def456` is 6 characters, the extractors require `{7,40}`, so T-047's stated PC40 discrimination is *still* structurally unreachable — and the bats comment still names the old `xyz789`.

**Location:**
- `.../docs/red-gate-log.md:1045-1050` (attestation)
- `.../plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/b1-volatile-input/factory/stories/STORY-INDEX.md:11-12,17,19`
- `.../plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats:1227`
- `.../src/arm_b.rs:132` and `:654` (`hash.len() >= 7 && hash.len() <= 40`)

**Clause violated:** POLICY 15 (verbatim-stdout discipline — an attestation that contradicts the artifact it describes); TD-VSDD-059 (paper-fix); BC-5.39.010 PC20/PC21 (`[0-9a-f]{7,40}`).

**Evidence.** red-gate-log:1045-1048 states:

> **Fix:** Replaced all occurrences of `xyz789` with `def456` (**valid 7-char hex string**, different from B1 hash `abc123`). With valid hex: — WITHOUT PC40: B1=Some("abc123") ≠ B2=Some("def456") → three-way mismatch → exit 2

`def456` is **six** characters. `extract_input_hash_token` (`arm_b.rs:561`) and `extract_blockquote_pairs` (`arm_b.rs:654`) both gate on `hash.len() >= 7`, so B2 and B3 are `None`, the `(None, None)` arm fires (`arm_b.rs:206-216`), and the run exits 0 with a "not yet registered" advisory **with or without PC40**. `abc123` is also 6 chars — but `parse_story_input_hash` applies no length bound, so B1 is `Some("abc123")` and the fixture's asymmetry is invisible.

T-047 currently passes because the *volatile* early return fires first (`arm_b.rs:413-424`), which is the correct path — but the test's claimed discriminating power ("without PC40 this would produce exit 2", bats:1234) remains false. **PC40 is the mechanism that prevents the self-lock in F-S2107-P6-001, and its only integration-level gate is non-discriminating.** Mutating `is_volatile_path` to `false` would leave T-047 green.

Additionally, "Replaced **all** occurrences" is false: `bats:1227` still reads *"STORY-INDEX.md: S-21.07 catalog row has hash xyz789 (≠ abc123)"*. Fix requires a ≥7-char hex mismatch value (e.g. `def4567`) in both fixture files and the bats comment, plus a control assertion that the fixture blocks when the volatile entry is removed.

---

### HIGH

---

#### F-S2107-P6-004 — HIGH — Two of eight BC §Architecture Anchors name functions that do not exist in the shipped crate. One was deleted by F-P4-016 *in this burst*; the other has never existed.

**Location:** `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md:1241` and `:1246`

**Clause violated:** POLICY 4 (`semantic_anchoring_integrity` — "Do referenced module/package names and file paths resolve to real workspace artifacts?"); POLICY 5; TD-VSDD-060 sibling-site sweep.

**Evidence.**

- BC:1241 anchors `extract_bc_index_version(content, bc_id)`. F-P4-016's closure (red-gate-log:1129-1145) **deleted** that symbol: *"Deleted the wrapper function entirely from arm_a1.rs."* `grep -n "fn extract_bc_index_version" arm_a1.rs` resolves only to `extract_bc_index_version_state` (`arm_a1.rs:145`). The v1.10 `last_amended` claims *"Architecture Anchors: extract_bc_index_version updated v1.9→v1.10 terminology"* — the description prose was updated, the symbol name was not, in the same burst that deleted the symbol.
- BC:1246 anchors `check_volatile_inputs(story_content, story_id)` — *"returns `Some(Continue)` with prescribed advisory if found, `None` to proceed"*. No such function exists. The shipped shape is `is_volatile_path(&str) -> bool` + `parse_story_volatile_inputs(&str) -> Vec<String>` + inline logic in `run_arm_b1` returning `(Vec<Violation>, Vec<Advisory>)`. This anchor has been wrong since v1.5 and survived five passes.

Mis-anchoring blocks convergence regardless of severity class. An implementer working from §Architecture Anchors would build a third API surface.

---

#### F-S2107-P6-005 — HIGH — BC v1.10 §Traceability and §Story Anchor pin the story at "v1.4 in flight". The story went to v1.5 in the same burst. F-P4-021's two sites were re-staled rather than fixed.

**Location:** `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md:1225` and `:1251`; story `version: "1.5"` at `S-21.07-validate-cross-site-correspondence.md:6`

**Clause violated:** POLICY 14 leg 5 / POLICY 17 (cross-layer frontmatter parity); POLICY 4.

**Evidence.**

```
BC:1225  | Stories | S-21.07 (implementing story; v1.4 in flight) |
BC:1251  S-21.07 — `validate-cross-site-correspondence` WASM hook (v1.4 in flight; …)
```

The v1.10 `last_amended` (BC:48) claims *"Story Anchor and §Traceability Stories TBD→S-21.07"*. The `TBD→S-21.07` substitution landed; the version token was written as `v1.4` while story-writer bumped the story to `v1.5` in the same fix burst. This is the mirror direction of Arm A2 (BC→story version pins in a BC, rather than story→BC pins in a story) and is therefore **ungated by the hook this BC specifies** — the same self-blindness the v1.3 changelog already confessed for `modified[]`↔Changelog correspondence (BC:1268). Two sites, one file; both were flagged as `TBD` by F-P4-021 and both are now wrong in a new way.

---

#### F-S2107-P6-006 — HIGH — ADR-037 v1.1 §Context, §Decision 5 and §Status all describe S-21.07's `ARCH-INDEX.md` input as present and story-writer-pending. The story removed it in the same burst. The 78-story roster is stale by one and the ARCH-INDEX derivation now measures 62, not 63.

**Location:** `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/architecture/decisions/ADR-037-input-hash-stable-input-constraint-volatile-artifacts-excluded.md:30, 65, 85-88, 148-150, 214`

**Clause violated:** POLICY 4; POLICY 14 leg 5 / POLICY 17 (same-burst propagation); production-grade default (a §Status block asserting a superseded live state).

**Evidence.** ADR-037 v1.1 (dated 2026-08-05, the same date as story v1.5):

- `:65` — `| S-21.07 | ARCH-INDEX.md |` in the §Context volatile-roster table. S-21.07's `inputs:` (`:15-22`) contains no volatile path.
- `:150` — *"S-21.07 is explicitly included: its `inputs:` array **carries** `ARCH-INDEX.md` (corpus-verified; line 18 of story frontmatter). Story-writer **removes** that entry"* — present tense, already done. Line 18 of the story is now `.factory/specs/architecture/SS-04-plugin-ecosystem.md`.
- `:214` — *"S-21.07 is included in the sweep; its volatile `ARCH-INDEX.md` input **has been identified and is story-writer-routed** (the actual `inputs:` edit is story-writer scope; this ADR records the obligation)."* The obligation was discharged in the same burst.
- `:30`, `:148`, `:186`, `:198`, `:207` — blast radius stated as **78**. With S-21.07 remediated the remaining scope is **77**.

**Corpus re-verification of the derivation (this review).** `Grep '^  - \.factory/specs/architecture/ARCH-INDEX\.md'` over `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/stories` returns **65 files**, of which 3 are non-story (`STORY-INDEX.md`, `epics/E-12-engine-governance.md`, `epics/E-13-artifact-integrity.md`) → **62 stories**. ADR-037:85-86 records `→ 63 stories (65 including epics)`. The delta is exactly S-21.07. The 78-row table arithmetic itself checks out (I summed the 28 table rows: 2+4+5+1+1+1+1+1+1+1+1+5+9+8+4+8+7+1+1+1+1+1+1+1+3+6+1+1 = 78), and the frozen-only figure of 23 is not independently checkable read-only — but the union is now 77 and the ARCH-INDEX leg is 62.

Architect routing: correct §Context row, §Decision 5 item 1, §Status, and the five blast-radius figures; re-run the derivation block per D-950 with captured stdout (POLICY 15).

---

#### F-S2107-P6-007 — HIGH — BC v1.10's PC40 transitional clause asserts two conditions as unmet that were satisfied in the same burst, and simultaneously condemns as "widening" the very entry PC40's own pattern table mandates.

**Location:** `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md:530-542`, against `:512` (PC40 pattern table row 6) and ADR-037 §Decision 2 `:118`

**Clause violated:** internal consistency of PC40; POLICY 4.

**Evidence.** BC:534-541 reads:

> *"As of pass-4 adversarial review (F-S2107-P4-013), S-21.07 is absent from ADR-037 §Context's 19-story table; the v1.10 amendment added `ARCH-INDEX.md` to `is_volatile_path` to unblock implementation rather than removing it from S-21.07's `inputs:`, **widening the suppression rather than closing it**. Until ADR-037 §Context is corrected to include S-21.07 … and S-21.07's `inputs:` array removes the volatile `ARCH-INDEX.md` entry …, PC40 remains non-vacuous for S-21.07 and the 'no permanent weakening' claim does not hold for that story."*

Three defects in one clause:

1. **Both stated preconditions are now satisfied** — ADR-037 v1.1 §Context includes S-21.07 (`ADR-037:65`), and the story's `inputs:` no longer carries `ARCH-INDEX.md`. PC40 **is** vacuous for S-21.07. The clause states the opposite as the current condition.
2. **It cites the superseded "19-story table"** while ADR-037 v1.1 (same burst) replaced it with 78.
3. **It contradicts PC40's own normative table.** BC:512 lists `.factory/specs/architecture/ARCH-INDEX.md` as volatile pattern 6, and ADR-037 §Decision 2 `:118` lists it as one of the six canonical patterns. Including it in `is_volatile_path` is therefore *mandated conformance*, not "widening the suppression". The pass-4 remediation of `is_volatile_path` (F-S2107-P3-002(a)) is now framed by the BC as a defect.

Product-owner routing: the transitional clause should now read that PC40 is vacuous for S-21.07 and that 77 stories remain, and the "widening" characterisation must be withdrawn — pattern 6 is normative.

---

#### F-S2107-P6-008 — HIGH — Third consecutive incomplete stale-cite sweep. Three load-bearing `BC-5.39.010 v1.2` cites remain, including the bats suite's own "Governing BC" header, while two attestations claim the sweep is complete.

**Location:**
- `.../plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats:38` — `# Governing BC: BC-5.39.010 v1.2`
- `.../plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats:305` — `# fuel_cap must NOT be present (BC-5.39.010 v1.2 §Gate Spec; ADR-035 §Decision 5)`
- `.../plugins/vsdd-factory/hooks-registry.toml:681` — `Adding fuel_cap without evidence of exhaustion is premature (BC-5.39.010 v1.2 §Gate Spec; ADR-035 §Decision 5)`

**Clause violated:** POLICY 14 leg 5; POLICY 15; S-7.01 Partial-Fix Regression Discipline (blast radius 2+ files → HIGH). Lineage: F-P2-017 (LOW) → F-S2107-P3-010 (HIGH, re-severitised) → F-S2107-P4-011 (HIGH) → this pass.

**Evidence — the two false attestations.**

`docs/red-gate-log.md:1158-1168`, §"v1.7/v1.8/v1.9 → v1.10 governing-spec cite refresh": *"**All** governing-spec cites in `#[cfg(test)]` modules and the red-gate-log were updated to v1.10."* The scope table lists exactly four files: `arm_a1.rs`, `arm_a2.rs`, `arm_b.rs`, `red-gate-log.md`. **The bats suite and `hooks-registry.toml` are not in scope**, and bats:38 is a governing-spec cite by definition.

`CHANGELOG.md:11` (final sentence): *"Stale `BC-5.39.010 v1.2/v1.8/v1.9` cites updated to v1.10 across Cargo.toml, lib.rs, arm_a1.rs."* Same under-scoping.

bats:38 is the single most-read cite in the deliverable — it is the file header that tells a reader which spec version the suite gates. The two `fuel_cap` cites are load-bearing authority cites (they name v1.2 §Gate Spec as the *reason* no `fuel_cap` field exists), not frozen-historical provenance, so the D-906/D-907 class-death exemption does not apply. By contrast `hooks-registry.toml:677` (`BC-5.39.010 v1.1 §Gate Spec had MultiEdit omitted — v1.2 corrects`) and `:686` (`v1.6 / D-953`) *are* legitimate historical narrative and correctly retained.

---

#### F-S2107-P6-009 — HIGH — Four sites mis-anchor BC invariant numbers: "v1.3 invariant 10" is claimed to govern version-token extraction (it governs POLICY 21 compliance) and "v1.3 invariant 11" is claimed to govern B3≠B1 blocking (it governs the three-category hash-provenance taxonomy).

**Location:**
- `.../plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats:438` — `# BC-5.39.010 v1.3 invariant 10: LAST version token in escaped-pipe chain is authoritative.`
- `.../plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats:683` — `# BC-5.39.010 v1.3 invariant 11: B3≠B1 must block (blockquote hash mismatch).`
- `.../src/arm_a1.rs:624` — `/// BC-5.39.010 v1.3 invariant 10: when version_history has escaped-pipe delimiter, only the FINAL version token is authoritative.`
- `.../src/arm_a1.rs:659` — `/// BC-5.39.010 v1.3 invariant 10: extract_bc_index_version_state must scan only table body rows …`

**Clause violated:** POLICY 4 (`semantic_anchoring_integrity`) — mis-anchors that contradict the cited document.

**Evidence.** BC-5.39.010 §Invariants (`BC-5.39.010.md:936-947`):

```
10. **POLICY 21 compliance**: no `.sh` scripts. All gating uses WASM plugin or Rust workspace tests…
11. **Stale vs fabricated vs algorithm-divergent hash provenance (Class B)**: this hook detects cross-site *inconsistency* only…
```

Neither invariant has ever concerned version-token extraction (that is PC5/PC6) nor B3≠B1 blocking (that is postcondition 13). Per the v1.1 changelog (`BC:1270`), invariant 11 was *introduced* as the provenance invariant, so these anchors were wrong at authoring and have survived five passes. Blast radius 4 sites / 2 files. Correct anchors: PC5/PC6 (rightmost `v`-token in the 6th field) and postcondition 13 (three-way mismatch → block).

---

#### F-S2107-P6-010 — HIGH — The corpus-test suite still has no assertion covering the two artifacts the hook must not block: no corpus test exercises Arm A1 on `BC-5.39.010` or Arm B1 on `S-21.07`, and `is_volatile_path` still has zero corpus coverage — the exact three sweeps pass-3 prescribed.

**Location:** `.../src/lib.rs:646-1012` (the entire corpus-test block)

**Clause violated:** POLICY 11 (no test tautologies — coverage that cannot fail on the live defect); TD-VSDD-059.

**Evidence.** The seven corpus tests in `lib.rs` are: `corpus_arm_a1_bc_1_17_001_own_row_version_not_cross_ref` (BC-1.17.001), `corpus_arm_a1_row_present_no_version_cell_majority_shape` (BC-1.01.001), `corpus_arm_a2_s21_04_bc_citations_match_live_bc_frontmatter` (S-21.04), `corpus_dispatch_vp_index_excluded_from_class_e_live_path`, `corpus_dispatch_vp_canonical_file_accepted_by_class_e_live_path`, `corpus_arm_e1_vp100_last_amended_outer_version_matches_version_field`, `corpus_arm_e1_bc5_39_010_block_scalar_last_amended_parseable`.

Not one of them asserts *"`run_arm_a1_with_index_result` on live `BC-5.39.010` + live `BC-INDEX.md` produces no violations"*, and not one asserts *"`run_arm_b1_with_index_result` on live `S-21.07` + live `STORY-INDEX.md` produces no violations"*. Either assertion would be **RED right now** and would have been RED at pass-4 and pass-3 as well — F-S2107-P4-001, F-S2107-P4-002, F-S2107-P4-012 and F-S2107-P3-003 all reached the adversary instead of the test suite.

`is_volatile_path` has no corpus test at all. Pass-3 named this explicitly and the pass-5 red-gate-log records no such test being added. That sweep (`is_volatile_path` × every `.factory/stories/*.md` `inputs:` array) is also the only mechanical check that would keep ADR-037 §Decision 2 and `VOLATILE_PATTERNS_CYCLES_NAMED` in sync as PC40's implementation note (`BC:527-528`) requires.

Note the discovery mechanics work: `live_factory_root()` (`lib.rs:654-680`) ascends from `CARGO_MANIFEST_DIR` and validates `specs/behavioral-contracts/`, so from this worktree it resolves the main-checkout `.factory/` at depth 5 — inside the 8-level budget. The corpus is reachable; the assertions are simply absent.

---

#### F-S2107-P6-011 — HIGH — ADR-037 v1.1's own `modified[]` array is date-DECREASING, contradicting the ADR-corpus convention and the exact non-decreasing relation this story's Class E2 exists to enforce. Class E cannot catch it because PC34 excludes ADRs.

**Location:** `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/architecture/decisions/ADR-037-input-hash-stable-input-constraint-volatile-artifacts-excluded.md:19-21`

**Clause violated:** POLICY 14 leg 3; BC-5.39.010 PC38 / postconditions 21-22 (`∀i: date[i] ≤ date[i+1]`).

**Evidence.**

```yaml
modified:
  - "2026-08-05 (v1.1)"
  - "2026-08-04 (v1.0)"
```

`strip_date_annotation` → `["2026-08-05", "2026-08-04"]`; `run_arm_e2` (`arm_e.rs:236`) `curr < prev` → true → E2 violation. Corpus convention is ascending: `ADR-025:7-10` (v1.3→v1.6), `ADR-034:22-24` (v1.0→v1.2), `ADR-036:26`, `ADR-035:28`. BC-5.39.010's own `modified[]` (`:27-38`) is ascending. The story's is ascending (`:47-53`).

`is_frontmatter_parity_target` (`dispatch.rs:232-277`) admits BC, story, VP and epic paths only — ADR files carry no `specs/verification-properties` or `stories`/`epics` components, so Class E never fires on them. PC34 therefore leaves the ADR layer ungated while POLICY 17 enumerates only "BC, epic, story, VP layers". Two routings: architect fixes ADR-037's array; product-owner adjudicates whether PC34/POLICY 17 should extend to `.factory/specs/architecture/decisions/ADR-*.md` (the layer demonstrably needs it — the first ADR authored under this gate's own cascade violated the invariant).

---

### MEDIUM

---

#### F-S2107-P6-012 — MEDIUM — PC36's load-bearing corpus figure is stale within its own burst: three files use `last_amended: |-`, not two, and the third is ADR-037 v1.1.

**Location:** `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md:476-481`; repeated at `:1261` (v1.10 Changelog row), story `:109` and `:12`

**Clause violated:** POLICY 15 / D-950 corpus-count discipline.

**Evidence.** BC:479-481: *"Corpus (2026-08-04): `grep -rc '^last_amended: |-' .factory/` → **2 occurrences (these two files only)**."* and `:477-478`: *"BC-5.39.010 itself uses `last_amended: |-` … as does S-21.07's own story file — **these are the two files** this hook most needs to gate."*

Re-measured this review — `Grep '^last_amended: \|-?$'` over `/Users/zious/Documents/GITHUB/vsdd-factory/.factory` returns **3 files**:

```
.factory/stories/S-21.07-validate-cross-site-correspondence.md
.factory/specs/architecture/decisions/ADR-037-...-volatile-artifacts-excluded.md
.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md
```

ADR-037's `last_amended: |-` was introduced by the v1.1 amendment in the same pass-5 burst that wrote PC36's "2 occurrences". The figure is declared load-bearing ("Block-scalar support is a load-bearing normative requirement") so a wrong count is not cosmetic. It also compounds F-S2107-P6-011: the third block-scalar artifact is in a class Class E does not gate.

---

#### F-S2107-P6-013 — MEDIUM — `collect_block_scalar_body` accepts `_is_strip` and ignores it; its docstring asserts clip semantics and then contradicts itself two lines later; the folded multi-line branch has zero coverage; all four block-scalar tests use single-line bodies, so literal-vs-folded and clip-vs-strip are untested by construction.

**Location:** `.../src/frontmatter.rs:104-202` (doc + body), `:119-123` (signature), `:175-195` (folded branch), tests at `:404-498`; production docstring claim at `:24-33`

**Clause violated:** Standing Rule 3 §3 / production-grade default (doc comment claiming a capability the code does not implement); POLICY 11.

**Evidence — self-contradicting docstring.** `frontmatter.rs:108-112`:

```
/// - **Strip** (`is_strip = true`, `-` suffix): all trailing blank lines removed.
/// - **Clip** (`is_strip = false`, no suffix): one trailing newline preserved.
///   For field-value extraction …, the trailing `\n` is omitted in both modes …
```

The clip bullet states a behaviour the next sentence retracts, and the parameter is bound as `_is_strip` (`:122`) with the inline comment *"Both modes strip trailing blanks for field-value extraction."* The public docstring at `:26-29` nonetheless advertises all four modes with distinct semantics ("literal/folded, clip/strip").

**Evidence — the four tests cannot distinguish the modes.** All four fixtures (`:412`, `:435`, `:462`, `:483`) use a single-line body:

```rust
let content = "---\nlast_amended: |\n  2026-08-05 (v1.10) — test fixture\n---\n";
```

For a single content line, literal (`join("\n")` on a 1-element Vec) and folded (`buf.push_str(line)` once) produce byte-identical output, and clip/strip are indistinguishable because both strip. Consequently the entire `is_folded` branch (`:175-191`, including the `trim_end_matches(' ')`/`truncate` paragraph-separator path) is **never executed by any test**, and no corpus artifact uses `>`/`>-`. The four tests are load-bearing for exactly one property — "the indicator is not returned as the value" — which is what PC36 actually mandates. That answers the brief's question directly: **all four indicators are exercised, but only at the indicator-detection level; the literal-vs-folded and clip-vs-strip semantics the code and docs claim are asserted by nothing.** Either add multi-line and blank-line-bearing fixtures for both modes, or reduce the docstring to what is implemented and delete the dead `is_folded` branch.

---

#### F-S2107-P6-014 — MEDIUM — F-P4-007's "7 E-class fixtures" sweep missed an eighth file with the identical defect, and the two fixture files in that directory now contradict each other.

**Location:** `.../tests/fixtures/validate-cross-site-correspondence/e1-15-byte-last-amended/factory/specs/behavioral-contracts/BC-INDEX.md:10-12,16` vs. `.../e1-15-byte-last-amended/factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md:16-18`

**Clause violated:** S-7.01 Partial-Fix Regression Discipline (b) — sibling files in the same layer; POLICY 4.

**Evidence.** The BC-INDEX fixture comment claims:

```
<!-- Supporting fixture for T-045. BC-5.39.010 is at version "2" in BC-INDEX →        -->
<!-- A1 arm: "2" found in INDEX → matches BC frontmatter "2" → no A1 violation.        -->
```

with row `… | S-21.07 | v2 |`. Traced: 6 non-empty fields → `extract_last_v_token("v2")` requires `v` + digits + `.` + digits (`arm_a1.rs:267-272`) → `v2` has no decimal → `None` → the `n if n >= 6` arm falls through to `RowPresentNoVersion` (`arm_a1.rs:195`). A1 is silent via the *no-version-cell* path, not via a version match. The sibling fixture in the same directory states the truth correctly: *"Arm A1 cannot handle single-integer versions like `v2` in BC-INDEX (extract_version_token requires vN.N format with a decimal point)."*

red-gate-log:1017-1037 enumerates exactly seven E-class fixtures corrected; this eighth BC-INDEX fixture was not audited. T-045 triggers a VP write so the mis-description is currently inert, which is precisely why it survived — the same reason the original seven survived.

---

#### F-S2107-P6-015 — MEDIUM — Three factual defects in the CHANGELOG `[Unreleased]` entry, including a description that does not match the shipped `is_volatile_path` and a `### Fixed`-only filing for a story whose primary deliverable is a new crate.

**Location:** `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07/CHANGELOG.md:9-11`

**Clause violated:** production-grade default; S-7.01 (c) prose referencing a changed value.

**Evidence.**

1. **Wrong section.** The only entry is under `### Fixed` and is titled *"S-21.07 — validate-cross-site-correspondence pass-3/4/5 adversarial fix burst"*. S-21.07's deliverable is a **new** WASM crate, a **new** `hooks-registry.toml` entry (`:689-704`), and a **new** 45-test bats suite. Nothing under `### Added` announces the plugin. A release-notes reader learns only that fix bursts happened. The template comment at `CHANGELOG.md:5-7` names `### Added / ### Changed / ### Fixed`.
2. **Wrong description of the shipped code.** *"`arm_b::is_volatile_path` rewritten with **exact six-pattern const slice** per ADR-037 §Decision 2."* The shipped implementation is a **4-element** const slice (`VOLATILE_PATTERNS_CYCLES_NAMED`, `arm_b.rs:320-321`) plus one path equality (`:340`) plus a 3-arm `matches!` (`:360-365`). F-P4-020's closure explicitly reframed this as "six canonical patterns expanded to eight concrete forms" (`arm_b.rs:299-319`); the CHANGELOG prose was not swept with it.
3. **Under-scoped sweep claim** — see F-S2107-P6-008.

---

#### F-S2107-P6-016 — MEDIUM [process-gap] — "bats 45/45" conceals five unconditionally-skipped tests and the suite emits no positive-coverage assertion of actively-executed count.

**Location:** `.../tests/validate-cross-site-correspondence.bats:807, 825, 864, 899, 922`

**Clause violated:** CI-as-Code positive-coverage axis; POLICY 15.

**Evidence.** Five tests begin with an unconditional `skip` before any dispatcher invocation:

```
bats:807  skip "[DEFERRED v1.6 — Class D] burst-log.md unclassified after Class D arm removal; …"
bats:825  (same)
bats:864  (same)
bats:899  (same)
bats:922  skip "[DEFERRED v1.6 — Class D] lessons.md unclassified …"
```

bats reports `ok N # skip …` for these, so a `45/45` tally is arithmetically true and semantically misleading: **40 tests execute, 5 do not.** The deferral itself is legitimate (BC v1.6 §Deferred Scope, POLICY 1 ID preservation). The gap is the attestation shape: there is no line of the form `Check passed: N tests executed, M skipped` derived at runtime, so a future regression that converts additional tests to `skip` (or to a vacuous `_require_artifacts` skip) is indistinguishable from the current state. Any burst-log or PR attestation must read `40 passed / 5 skipped (Class D DEFERRED) / 0 failed`.

---

#### F-S2107-P6-017 — MEDIUM [process-gap] — The bats suite selects its dispatcher from host state (operator plugin cache, darwin-arm64-only, highest-version-wins) and records nothing about which binary it ran. A local "45/45 GREEN" is neither reproducible nor auditable against D-693.

**Location:** `.../tests/validate-cross-site-correspondence.bats:54-59`

**Clause violated:** D-693 (WASM pre-green gate); POLICY 15; POLICY 22 (consumer must re-run load-bearing predicates — impossible if the operand is unrecorded).

**Evidence.**

```bash
DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
if [ ! -x "$DISPATCHER" ]; then
  DISPATCHER="$(find "$HOME/.claude/plugins/cache/claude-mp/vsdd-factory" \
    -name "factory-dispatcher" -path "*/darwin-arm64/*" 2>/dev/null | sort -V | tail -1)"
fi
```

Three consequences: (a) if the release dispatcher is absent or **stale relative to the current `hook-sdk`**, the suite silently runs against whichever operator-cache version happens to be newest — a different host ABI than the one under test; (b) the fallback is darwin-arm64-only, so on any other platform it yields empty and every payload test skips (hard-failed only when `CI_REQUIRE_ARTIFACTS=1`, which ci.yml sets only for the `bats-full-suite` job at `ci.yml:676`); (c) the executed dispatcher path/version is never echoed, so no attestation can identify the operand.

This is a concrete, file-level mechanism for the brief's item 6 (three runs → three failure sets). CI itself is sound — `ci.yml:261-271` builds the release dispatcher, `:620-654` stages WASM with a count floor, `:656-677` runs `run-all.sh` under `CI_REQUIRE_ARTIFACTS=1` — so the non-determinism is a *local-gate* integrity defect, not a CI one. Remediation: echo the resolved `$DISPATCHER` and its mtime/version in `setup_file`, and fail rather than fall back when `target/release/factory-dispatcher` is absent under `CI_REQUIRE_ARTIFACTS`.

---

#### F-S2107-P6-018 — MEDIUM — PC5 enumerates no state for "≥6 fields with no `v`-prefixed token in field 6"; the implementation silently degrades such rows — and any 6-column row with an empty cell — to `RowPresentNoVersion`, making Arm A1 silent. F-P4-017's empty-cell leg is not closed.

**Location:** `.../src/arm_a1.rs:188-208` (`match non_empty_fields.len()`), `:193-196` (the `None` fallthrough); BC-5.39.010 `:108-135`, `:155-158`

**Clause violated:** BC-5.39.010 PC5 four-state enumeration (the implementation has a fifth, unspecified path); POLICY 4.

**Evidence.** `arm_a1.rs:190-197`:

```rust
n if n >= 6 => {
    let sixth = non_empty_fields[5];
    return match extract_last_v_token(sixth) {
        Some(v) => BcIndexVersionState::Version(v),
        None => BcIndexVersionState::RowPresentNoVersion,
    };
}
```

PC5 defines `Version(v)` as "row found AND non-empty field count is ≥6" with no `None`-token branch. Two live escape channels follow:

- A 6-column row whose version cell is bare (`| … | S-21.07 | 1.9 |`) yields no `v`-token → `RowPresentNoVersion` → **no version comparison at all**. PC6's mandatory-`v` rule makes this spec-conforming in isolation, but the resulting silence is nowhere stated as a normative disposition.
- Because classification counts **non-empty** fields (per PC5's own wording), a 6-column row with any empty cell counts 5 → `RowPresentNoVersion` → the version cell in field 6 is never read. This is precisely F-S2107-P4-017's second leg ("a 6-column row with any empty cell silently escapes Arm A1"); the pass-5 closure addressed only the escape-aware *first-cell extraction* leg (`arm_a1.rs:166-173`), not the counting leg. Verified live: `BC-INDEX.md:1461` has all six cells populated, so the corpus is currently unaffected — this is a forward-looking false-negative of the same class `RowMalformed` was added to prevent in the opposite direction.

Product-owner routing: PC5 must state the disposition for "≥6 fields, no `v`-token in field 6" and decide whether column classification counts positional or non-empty fields.

---

#### F-S2107-P6-019 — MEDIUM — Pass-numbering incoherence in `red-gate-log.md` persists unchanged and now has a hole where the pass-5 section should be. F-P4-024 is not closed.

**Location:** `.../docs/red-gate-log.md:1` (title "Pass-1 Fix Burst"), `:580-585`, `:954-958`, `:1082-1087`

**Clause violated:** POLICY 15 (evidence traceable to the pass that produced it); POLICY 1 (append-only, non-reused labels).

**Evidence.**

- `:580` `## Pass-4 Fix Burst — RED GATE Tests (test-writer)` — but `:584` says *"Cycle: v1.0-brownfield-backfill / S-21.07 **adversary pass-3** fix burst (25 findings: B3/H7/M12/L3)"*. The heading and its own subtitle name different passes.
- `:954` `## Amendment 3 — **Pass-4** Adversary Findings`, `:1082` `## Amendment 4 — **Pass-4** Adversary Findings`. These are the pass-5 fix burst's work (they close F-P4-003/004/005/014/015/016/018/023) but are labelled by the pass that *found* the items, while the earlier `## Pass-4 Fix Burst` heading is labelled by the pass that *fixed* them. Both conventions appear in one document.
- No `## Pass-5` section exists, so a reader cannot locate this burst's evidence by pass number.
- `:1` titles the whole document "S-21.07 Pass-1 Fix Burst (Test-Writer)" though it now spans five bursts.

Given POLICY 15's requirement that persistence-layer evidence be locatable, and that this is the third pass to observe it, MEDIUM rather than LOW.

---

#### F-S2107-P6-020 — MEDIUM — PC40's prescribed advisory is not reproduced: `run_arm_b1` drops the mandated `validate-cross-site-correspondence [Class B] advisory: ` prefix while the adjacent comment attests "transcribed verbatim".

**Location:** `.../src/arm_b.rs:414-421` vs. BC-5.39.010 `:521-523`

**Clause violated:** BC-5.39.010 PC40 prescribed message; POLICY 15 (false attestation in a code comment).

**Evidence.** PC40 `:521-523` prescribes:

```
"validate-cross-site-correspondence [Class B] advisory: Story <id> has volatile inputs per
ADR-037 §Decision 2 — three-way equality is unsatisfiable until story-writer removes volatile
inputs and state-manager recomputes the hash; Class B BLOCK suspended. Volatile path(s): <list>"
```

Shipped (`arm_b.rs:416-421`), preceded at `:414` by `// ADR-037 §Decision 4 prescribed advisory text — transcribed verbatim.`:

```
"Story {story_id} has volatile inputs per ADR-037 §Decision 2 — three-way equality is
unsatisfiable until story-writer removes volatile inputs and state-manager recomputes the
hash; Class B BLOCK suspended. Volatile path(s): {volatile_found:?}"
```

The `validate-cross-site-correspondence [Class B] advisory: ` prefix is absent. Every other message in the crate carries the hook-name + class prefix (`arm_a1.rs:317, 329, 357, 408`; `arm_b.rs:174, 185, 195, 210, 224, 262, 283`; `arm_e.rs:167, 181, 194, 239`), and the internal-log greps that operators and bats use are keyed on it — bats T-047 only greps case-insensitively for `volatile` (`bats:1242`), which is why the omission is invisible to the suite. The unit test `arm_b1_volatile_advisory_prescribed_text` (`arm_b.rs:1216-1245`) asserts two interior substrings and not the prefix.

---

#### F-S2107-P6-021 — MEDIUM [process-gap] — `adversary-pass-5.md` is absent from the cascade directory although the dispatch records "5 passes done" and this review is numbered pass-6.

**Location:** `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/cycles/v1.0-brownfield-backfill/S-21.07/` — contains exactly `adversary-pass-1.md`, `adversary-pass-2.md`, `adversary-pass-3.md`, `adversary-pass-4.md` (path-corroborated against canonical factory-artifacts, not the worktree snapshot)

**Clause violated:** POLICY 15; D-448(a) source-attestation parity; BC-5.39.010 §3-CLEAN protocol auditability.

**Evidence.** The dispatch states *"streak 0/3, 5 passes done"* with `trajectory 47→18→25→25` — four values for four persisted records. Either (a) a pass-5 review was conducted and its Part A was never persisted, in which case this burst's fix set has no auditable source record and the pass-5 closure claims in `red-gate-log.md` Amendments 3-4 and `CHANGELOG.md:11` (all of which cite `F-P4-*` IDs, not `F-P5-*`) cannot be attested against a finding set; or (b) the pass counter double-counts an early sub-pass ("pass-1b" appears in BC:1267) and this review is really pass-5, in which case the file this review is persisted to will create a numbering hole.

Consequence for this review: I was unable to honour the Iron Law for pass 5 — there was no Part A to read — so recurrence-checking against pass-5 findings was impossible, and F-S2107-P6-001/002/003 are graded against pass-4's set only. Orchestrator adjudication required before the streak is advanced.

---

### LOW

---

#### F-S2107-P6-022 — LOW — TD-VSDD-091 volatile pins, both already decayed.

**Location:** `.../src/arm_a1.rs:993` — `The shipped message (arm_a1.rs:406-415) substitutes non-normative prose:`; `/Users/zious/.../decisions/ADR-037-...md:150` — `(corpus-verified; line 18 of story frontmatter)`

**Clause violated:** TD-VSDD-091 (narrative content cites function names + behavioural anchors, not `file.rs:NNN`).

**Evidence.** The advisory now spans `arm_a1.rs:406-417`, so the pin is off by two. ADR-037's "line 18 of story frontmatter" now points at `.factory/specs/architecture/SS-04-plugin-ecosystem.md`, not `ARCH-INDEX.md` — the pin decayed in the same burst that authored it (see F-S2107-P6-006). Neither is a Red-Gate table, AC source-of-truth table, or pass-report changelog, so neither qualifies for the TD-VSDD-091 exception.

---

#### F-S2107-P6-023 — LOW — Arm-count and test-count strings still contradict the six-arm reality. F-P4-009's class is not fully closed.

**Location:**
- `.../tests/validate-cross-site-correspondence.bats:6` — `covering the seven arms (A1, A2, B1, B2, D, E1, E2)`
- `.../tests/validate-cross-site-correspondence.bats:22` — `→ ALL 37+ payload tests FAIL in Red Gate` (the file has 45 `@test` blocks)
- fixture titles: `a1-current-index/.../BC-INDEX.md:31`, `a1-stale-index/.../BC-INDEX.md:34`, `b1-b3-only-mismatch/.../STORY-INDEX.md:23` all read `five-arm PostToolUse cross-site value-correspondence gate`

**Clause violated:** POLICY 4; S-7.01 (c).

**Evidence.** The canonical count is six blocking arms (A1/A2/B1/B2/E1/E2) with Class D deferred — as `hooks-registry.toml:667-671`, `lib.rs:6-15`, the story H1 (`:60`) and the BC H1 all now state. The bats header and three fixture titles still carry the pre-v1.6 counts. Fixture *titles* are illustrative rather than load-bearing, but bats:6 and bats:22 are the suite's own scope declaration.

---

#### F-S2107-P6-024 — LOW — BC v1.1 Changelog row asserts a 17-entry VP table that the document does not contain; §VP Anchors is a bare "pending" placeholder.

**Location:** `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md:1253-1255` and `:1270`

**Clause violated:** POLICY 9 (VP-INDEX propagation); POLICY 4.

**Evidence.** BC:1270 (v1.1 row) states *"VP table extended to 17 entries."* BC:1253-1255 is:

```
## VP Anchors

VP IDs pending VP-INDEX allocation by state-manager at post-merge burst.
```

There is no VP table anywhere in the 1,272-line document, and the story carries `verification_properties: []` (`:34`). The state-manager-allocates-at-post-merge pattern is established practice in this repo and the story documents it inline (`:35`), so the deferral is legitimate — but the v1.1 Changelog row asserts an artifact that either never existed or was removed without a Changelog row recording the removal, and a 21-AC gate ships with zero declared verification properties. LOW because the deferral mechanism is real and documented; flagged because the Changelog claim is dangling.

---

## Observations

- **O-P6-01** — `arm_d.rs` remains fully compiled (`lib.rs:34` `pub mod arm_d;`, 5 `pub fn`s, 15 `#[ignore]`d tests) per BC §Deferred Scope's explicit "Do NOT delete this module". BC v1.6's normative clause forbids only the *dispatch branch* being compiled, which `dispatch.rs:193-197` satisfies. Note that `arm_d.rs:27` reproduces the PC30 predicate `^L-EDP1-[0-9]+-[0-9]+:` that the BC's own §Deferred Scope (`:825-836`) records as matching **0 of 61** real anchors — the frozen module propagates a known-broken predicate without a marker on that line. Correct disposition is S-21.08, not this story.

- **O-P6-02** — `corpus_arm_e1_bc5_39_010_block_scalar_last_amended_parseable` (`lib.rs:978-1012`) asserts only `!starts_with("|-")`, `!starts_with('>')`, and `outer.is_some()`. Its sibling `corpus_arm_e1_vp100_...` (`lib.rs:936-954`) asserts `outer == version`. The stronger equality assertion would currently pass (BC `version: "1.10"`, block body opens `2026-08-05 (v1.10)`) and would additionally guard E1 on the artifact E1 most needs to gate. Cheap strengthening.

- **O-P6-03** — `run_arm_a2_for_bc` (`arm_a2.rs:412-421`) performs `host::read_file` before delegating to the seam that early-returns on empty citations (`:319`). A story citing N BCs with no version-citing rows performs N wasted reads inside the 10M-instruction fuel budget. Carried from O-P3-02, still open.

- **O-P6-04** — `is_volatile_path` uses `path.contains(".factory/cycles/")` for patterns 2-5 (`arm_b.rs:349`) but exact equality for patterns 1 and 6-8 (`:340`, `:360-365`). An absolute path therefore matches a cycle log but not `STATE.md` or the three indexes. PC40's table says "path equals … exactly" for those four, so this is spec-conforming — recorded because the asymmetry is a plausible future false-negative if any caller ever passes absolute paths.

- **O-P6-05** — `capability: "E-12"` in the BC frontmatter and `| E-12 |` in the BC-INDEX Capability column resolve correctly: BC §Traceability `:1221` names *"E-12 (Engine Governance — cross-site value correspondence enforcement)"* and 9 BC-INDEX rows use the same value. No mis-anchor. The fixtures use `CAP-032` in the same cell (`a1-row-malformed/.../BC-INDEX.md:18` sibling fixtures, `e2-*/BC-INDEX.md:13`), a cosmetic divergence with no arm reading that field.

---

## Part B — Analysis and Verdict

### Verdict: **NOT-CLEAN**

24 findings — 3 BLOCKER / 8 HIGH / 10 MEDIUM / 3 LOW. Streak remains **0/3**. Trajectory `47 → 18 → 25 → 25 → 24`.

### Corpus-verification commands and counts

All executed read-only via `Grep`/`Glob`/`Read` against `/Users/zious/Documents/GITHUB/vsdd-factory/.factory` (canonical factory-artifacts) and `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07` (feature checkout).

| Measurement | Command / target | Result |
|---|---|---|
| BC-INDEX version cell for BC-5.39.010 | `Read BC-INDEX.md:1461`, escape-aware split | 6 non-empty fields; field 6 = `v1.9`; BC frontmatter `1.10` → **Arm A1 blocks** |
| STORY-INDEX three-way for S-21.07 | `Read STORY-INDEX.md:730, :733`; story `:23` | catalog `25c7324`, blockquote `25c7324`, frontmatter `dd5c9d2` → **Arm B1 blocks** |
| S-21.07 volatile inputs | `Read story :15-22` × `is_volatile_path` | 7 inputs, **0 volatile** → PC40 vacuous → Class B BLOCKING active |
| Block-scalar corpus | `Grep '^last_amended: \|-?$' .factory` | **3 files** (BC-5.39.010, S-21.07, ADR-037) — PC36 claims 2 |
| ARCH-INDEX volatile-input derivation | `Grep '^  - \.factory/specs/architecture/ARCH-INDEX\.md' .factory/stories` | **65 files** → −`STORY-INDEX.md` −2 epics = **62 stories** (ADR-037 claims 63) |
| ADR-037 §Context table arithmetic | manual sum of the 28 table rows | **78** ✓ internally consistent; **77** after S-21.07 remediation |
| ADR `modified[]` ordering convention | `Grep '^modified:' -A4 .../decisions` | ADR-025/034/035/036 ascending; **ADR-037 descending** |
| bats test count | `Grep '^@test' validate-cross-site-correspondence.bats` | **45** blocks; **5** open with unconditional `skip` → 40 executable |
| Stale governing-spec cites | `Grep 'BC-5.39.010 v1\.[0-9]+'` over worktree non-`.md` + bats + toml | 3 load-bearing stale (`bats:38`, `bats:305`, `hooks-registry.toml:681`) |
| Mis-anchored invariant cites | `Grep 'invariant 1[01]'` + BC §Invariants `:936-947` | **4 sites / 2 files** |
| Crate test tally | `arm_d.rs` 15 `#[ignore]` + `dispatch.rs` 2 `#[ignore]` | 17 ignored ✓ consistent with the claimed 127/0/17 |
| BC §Architecture Anchor resolution | `Grep 'fn extract_bc_index_version'`, `'fn check_volatile_inputs'` | `extract_bc_index_version_state` only; `check_volatile_inputs` **absent** |

Claims I could **not** verify read-only, which therefore remain OWED under POLICY 15 / POLICY 22: `cargo test` 127/0/17, `cargo fmt`/`clippy` clean, bats 45/45 both legs, WASM 226,794 bytes, and the `25c7324 → dd5c9d2` `compute-input-hash` invocation (POLICY 18). The static evidence is consistent with the test tally and with a green suite; it is *inconsistent* with any claim that green implies conformance (see below).

### Why the count did not fall

Pass-4 → pass-6 moved 25 → 24. That flatness is not noise; it has a definite structure. Of this pass's 24 findings:

- **6 are the same defects pass-4 named, re-opened because the fix addressed the instance and not the class** — F-S2107-P6-002 (F-P4-003/025), F-S2107-P6-003 (F-P4-014), F-S2107-P6-008 (F-P4-011), F-S2107-P6-014 (F-P4-007), F-S2107-P6-018 (F-P4-017), F-S2107-P6-019 (F-P4-024).
- **4 are new defects introduced by the pass-5 burst itself** — F-S2107-P6-004 (BC anchor left naming a symbol the burst deleted), F-S2107-P6-005 (story pinned at v1.4 while the burst bumped it to v1.5), F-S2107-P6-006 and F-S2107-P6-007 (ADR and BC each asserting a state the other's same-burst edit superseded), plus F-S2107-P6-011 and F-S2107-P6-012 (ADR-037 v1.1's own frontmatter).
- **1 is a re-derivation of four prior findings to their common root** — F-S2107-P6-001.
- The remainder are genuinely novel: F-S2107-P6-009 (mis-anchors present since initial authoring, unexamined for five passes), F-S2107-P6-010 (the corpus-coverage hole that lets F-S2107-P6-001 recur), F-S2107-P6-013 (the block-scalar semantics gap the brief asked about), F-S2107-P6-017 and F-S2107-P6-021 (harness and bookkeeping process gaps).

The dominant pattern is **same-burst cross-artifact incoherence**. Four artifacts were amended in one burst — BC v1.9→v1.10, ADR-037 v1.0→v1.1, story v1.4→v1.5, and the crate — and each one records a state of the world that another one's edit in the same burst invalidated. This is a stronger and more specific failure mode than "amendment-cascade residue": it is not that a sweep stopped at v1.8 or v1.9 (I found no artifact left at an intermediate version except the three v1.2 cites in F-S2107-P6-008); it is that the four editors did not re-read each other's output before declaring done.

### Ruling on brief item 2 — are the BLOCKER fixes complete or merely present?

| Pass-4 BLOCKER | Verdict | Basis |
|---|---|---|
| **F-P4-004** block-scalar parser | **SUBSTANTIVELY CLOSED, semantics untested** | `extract_frontmatter_field:65-69` detects all four indicators and delegates; the corpus test reads the live BC and asserts the indicator is not returned. Traced by hand against `BC-5.39.010.md:47-49`: block indent 2, single content line, literal join → `"2026-08-05 (v1.10) — …"` → `extract_last_amended_outer_version` → `Some("1.10")` == `version` → E1 live and correct. **Class E1 is no longer inert.** But the answer to the brief's precise question is: only the indicator-detection property is gated. All four tests use single-line bodies where literal ≡ folded and clip ≡ strip, `_is_strip` is discarded, and the entire folded multi-line branch is unexecuted (F-S2107-P6-013). |
| **F-P4-003 / F-P4-025** RowMalformed + PC4a | **PAPER-FIX (test-shaped)** | The state, the advisory branch, four unit tests and a bats fixture all exist and are load-bearing for "does not block" and for the two named clauses. But the MUST-verbatim requirement the BC added at v1.10 is violated — one normative sentence dropped, four phrases altered, three injected — and nothing asserts the verbatim text (F-S2107-P6-002). The implementer satisfied the two `grep`s, not the clause. |
| **F-P4-002** story cite sweep to v1.10 | **CLOSED at the gate-visible sites, re-opened upstream** | Story BC-table `:593` cites `1.10`, Token Budget `:678` cites `v1.10`; traced through the two-phase PC13 extractor both yield `"1.10"` == BC `1.10` → Arm A2 clean on the live story. `ARCH-INDEX.md` is out of `inputs:`. But the *reciprocal* legs were not swept (BC pins the story at v1.4 — F-S2107-P6-005) and the hash propagation was not (F-S2107-P6-001). |

### Ruling on brief item 3 — fixture and harness shape sweep

I swept all 61 fixture files and all six bats helpers against the production predicates. Results:

- **Load-bearing and correct:** `a1-row-malformed` (2 non-empty fields → `RowMalformed(2)`; BC fixture E1/E2 clean so exit 0 is attributable to A1 alone), `b1-b3-only-mismatch` (catalog `47a65c9` 7-char hex, blockquote `deadbee` 7-char hex → B3≠B1 → exit 2; F-P4-023's docstring defect genuinely closed), the seven corrected E-class BC-INDEX fixtures (all now 6-field with the version in field 6 — verified `e2-non-monotonic:13` and `e1-version-mismatch:14` by hand), `a1-v1-1-not-in-index` (E1/E2 clean, so exit 2 is attributable to `RowAbsent`+v1.1).
- **Still non-discriminating:** `b1-volatile-input` — F-S2107-P6-003, the single most consequential fixture defect in this pass, because it is the only integration-level gate on the mechanism that prevents the self-lock.
- **Mis-describing:** `e1-15-byte-last-amended/BC-INDEX.md` — F-S2107-P6-014, the eighth instance of a class declared closed at seven.
- **Harness:** `_write_registry` now matches production three-for-three and is guarded at runtime by `PG-S-15.11` (`bats:321-351`) — F-P4-018 is genuinely closed, and the guard is the right shape (it parses both TOMLs at test time rather than hardcoding). The remaining harness defect is dispatcher provenance (F-S2107-P6-017).

### Ruling on brief item 4 — drift in both directions

| Predicate | vs. v1.10 | Verdict |
|---|---|---|
| `first_cell_matches_bc_id` | link `^\[X\]` + `(`-follow, or `== X` | **conforming**, neither widened nor narrowed |
| escape-aware split | `\|` → `\x00`, split `\|`, count non-empty | **conforming** on both legs, but the non-empty counting rule creates the empty-cell escape (F-S2107-P6-018) |
| `extract_last_v_token` | mandatory `v`, rightmost, alphanumeric boundaries | **conforming**; silently returns `None` for bare or single-integer tokens → undocumented fifth state |
| `is_volatile_path` | `contains` for cycles (F-P4-020), equality for 1/6/7/8 | **conforming**; narrowing genuinely reverted; asymmetry recorded as O-P6-04 |
| `is_target_heading` | prefix + `' '`/`'('`/EOL boundary | **conforming**; correctly admits the 133 `Token Budget Estimate` variants |
| `line_contains_bc_id_at_boundary` | trailing-alphanumeric boundary only | **narrower than PC13's `\b` on both sides** — a *leading* boundary is not checked, so `XBC-5.39.010` would match. No corpus instance; not raised as a finding, but it is the one residual asymmetry in this axis |
| `parse_pure_version_field` / `extract_mandatory_v_inline` | Phase 1 optional-`v` full-field, Phase 2 mandatory-`v` reverse | **conforming**; the three collision classes are individually gated |
| hash charset | `[0-9a-f]{7,40}` at both extraction sites, with the deliberate "do not widen to `is_ascii_hexdigit()`" comment (`arm_b.rs:126-127`, `:647-648`) | **conforming** — the D-950 widening lesson is correctly encoded, and it is precisely this correct bound that exposes F-S2107-P6-003 |

No new D-950-class widening found. The one narrowing (`starts_with`) is fixed. The residual drift is definitional rather than predicate-level (F-S2107-P6-018).

### Ruling on brief item 5 — the 78-story roster and PC40's guarantee

The mechanical re-derivation is **sound in method and stale in result**. The table's 28 rows sum to exactly 78 and the derivation block records per-pattern counts with a de-duplication note, which is the right shape under D-950. But the ARCH-INDEX leg now measures 62 stories against the recorded 63, the delta is exactly S-21.07, and the union is 77 — because story-writer discharged the obligation in the same burst that architect recorded it as outstanding (F-S2107-P6-006). §Decision 5's scope is therefore *coherent in structure* and *off by one in fact*, and §Status asserts a pending action that is complete.

On the guarantee itself: **PC40 is now vacuous for S-21.07, so "no permanent weakening" does hold for that story** — and the BC text says the opposite (F-S2107-P6-007). The dangerous consequence is not the stale prose: it is that vacuity arrived *before* the hash legs were reconciled, so the shield came off while the inequality was still live. That is the mechanism in F-S2107-P6-001. The correct remediation order is: propagate `dd5c9d2` to both STORY-INDEX legs → verify three-way equality → *then* remove volatile entries. The pass-5 burst inverted it.

### Ruling on brief item 6 — non-deterministic bats and D-693 integrity

CI is not the problem. `ci.yml:620-654` stages WASM with a runtime-derived count floor, `:261-271` builds the release dispatcher, and `:656-677` runs the full `run-all.sh` with `CI_REQUIRE_ARTIFACTS: "1"`, which converts every `_require_artifacts` skip into a hard failure. That is a correctly-shaped positive-coverage gate.

The local gate is the problem, and it has a named mechanism: `bats:54-59` silently substitutes an operator-cache dispatcher (darwin-arm64-only, highest-version-wins, unrecorded) whenever `target/release/factory-dispatcher` is absent or unbuilt. A run against an rc.23 cache binary and a run against a freshly built one are indistinguishable in the output. Combined with the 5 unconditional Class D skips (F-S2107-P6-016), a local `45/45` string carries no information about which dispatcher executed which 40 tests. **Any "bats N/N GREEN" attestation for this story must be treated as non-authoritative under POLICY 22 unless it also records the resolved dispatcher path and the passed/skipped split.** I could not reproduce the reported cross-run variance read-only, and the truncation of the original failure's assertion text by `tail -20` means that specific failure is now unrecoverable — which is itself the argument for capturing full `bats` output rather than a tail.

### Ruling on brief item 7 — red-gate integrity

Mostly strong, with two specific failures. The pass-4 red-gate section (`red-gate-log.md:587-607`) is exemplary POLICY 15: literal command, captured `panicked at <file>:<line>:` stdout for all 13 tests, and the `99 passed; 13 failed; 17 ignored` summary line. Amendment 3 (`:963-975`) repeats the pattern correctly for 8 tests.

The two failures:

1. **F-S2107-P6-003** — Amendment 3's F-P4-014 entry asserts a property of the artifact (`def456` is a "valid 7-char hex string") that the artifact contradicts. An attestation that is falsifiable by reading the file it describes is worse than a missing attestation, because it terminates review.
2. **F-S2107-P6-008** — Amendment 4's cite-sweep entry asserts "All governing-spec cites" over a four-file scope table that omits the file carrying the most prominent stale cite. Amendment 4 also records `127 passed; 0 failed; 17 ignored` for a maintenance burst with no new RED GATE tests, which is honest and correct.

Net: the red-gate discipline is real and mostly load-bearing; the two defects are both in *narrative claims about scope*, not in the captured evidence.

### What must land before pass-7

Ordered by whether it unblocks anything else:

1. **F-S2107-P6-001** — product-owner adjudication of the ordering defect (PC2/PC13 vs PC3/PC12 carve-out reasoning), then state-manager propagates `v1.10` to BC-INDEX and `dd5c9d2` to both STORY-INDEX legs with a literal `bin/compute-input-hash` invocation and captured stdout (POLICY 18 / POLICY 15). Data first, spec ruling same burst — otherwise this recurs at pass-7 by construction.
2. **F-S2107-P6-002** — replace the RowMalformed advisory with the verbatim PC4a text and add one assertion that compares the whole formatted string, not substrings.
3. **F-S2107-P6-003** — `def456` → a ≥7-char hex value in both `b1-volatile-input` files and in `bats:1227`; add a control proving the fixture blocks when the volatile entry is removed; correct the red-gate-log claim.
4. **F-S2107-P6-010** — add the three corpus sweeps pass-3 already prescribed: Arm A1 × live `BC-5.39.010`, Arm B1 × live `S-21.07`, `is_volatile_path` × every `.factory/stories/*.md` `inputs:` array. The first two would have been RED at passes 3, 4 and 6; the third is the only mechanical guard on PC40↔ADR-037 §Decision 2 sync.
5. **F-S2107-P6-004 / -005 / -006 / -007** — one coordinated product-owner + architect pass over the four same-burst-incoherent artifacts, with each editor re-reading the others' output before declaring done.
6. **F-S2107-P6-008 / -009** — sweep the three remaining v1.2 cites and the four mis-anchored invariant references; the sweep scope must be the whole worktree, not a hand-listed file set.
7. **F-S2107-P6-021** — orchestrator adjudicates the pass-5 record before the streak is advanced.

### Novelty assessment

**Novelty: MEDIUM-HIGH.** This is not a converging cascade. Three findings are BLOCKER-class with live runtime consequences, four defects were *created* by the burst under review, and the pass produced a root-cause re-derivation (F-S2107-P6-001) that reframes four prior findings from data-hygiene defects into a single spec-level ordering defect — which is the first structural explanation for why this particular shape has now recurred four passes running. Two findings (F-S2107-P6-009 mis-anchored invariants, F-S2107-P6-013 block-scalar semantics) are defects present since initial authoring that five prior passes did not examine, consistent with the fresh-context compounding-value effect. Findings are not refinements of wording; the spec and the implementation have not converged.agentId: a6ca6ad3bc068ae19 (use SendMessage with to: 'a6ca6ad3bc068ae19', summary: '<5-10 word recap>' to continue this agent)
<usage>subagent_tokens: 454304
tool_uses: 75
duration_ms: 1161758</usage>