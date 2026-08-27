# ADR-046 BC-5.39.001 Spec-Convergence Gate — Adversary Pass 61

**Date:** 2026-08-27
**Reviewer:** vsdd-factory:adversary (fresh-context, independent of all prior passes)
**Scope:** ADR-046 "fix-state-writes" spec-convergence gate — frozen set review
**Frozen set entering this pass:** ADR-046 v1.23 + BC-4.17.001 v1.26 + BC-5.40.001 v1.21 + BC-7.07.001 v1.39
**Streak entering this pass:** 1/3 (from pass-60 CLEAN, D-1117)

---

## PART A: VERDICT

**VERDICT: CLEAN — zero blocking findings at any severity.**

This is a **substantive** clean pass. The adversary read ADR-046 v1.23, BC-4.17.001 v1.26,
BC-5.40.001 v1.21, and BC-7.07.001 v1.39 in full and independently verified every behavioral
claim against actual code at:
- `crates/factory-lock-parse/src/lib.rs` — `parse_factory_lock` (lines 207-227),
  `extract_frontmatter`, `extract_yaml_string_value`
- `crates/factory-lock/src/lib.rs` — `renew_lock_with_now` (bare Duration::seconds(2700) /
  byte-compare only), `has_factory_lock_key` (key-line-only detection)
- `crates/hook-plugins/verify-factory-lock/src/lib.rs` — `is_expired` (now>=expires_at),
  `trim_git_email`, `parse_iso8601`
- `crates/hook-plugins/precompact-flush/src/lib.rs` — Step-4 identity-blind `renew_lock`
- `plugins/vsdd-factory/bin/factory-lock-write.sh` — TTL literal (`TTL_SECONDS=2700`)

**All claims MATCH the code.** Zero BLOCKER, HIGH, MEDIUM, or LOW streak-resetting findings.

Confirmed absent from code (design-only; S-17.05 unimplemented): `renew_lock_if_holder`,
`classify_identity_resolution`, `SkipReason`, `IdentityResolution` — consistent with the frozen
specs' deferred-implementation framing.

**BC-5.39.001 3-CLEAN streak: ADVANCES 1/3 → 2/3.**
**Frozen set: UNCHANGED.** ADR-046 v1.23 + BC-4.17.001 v1.26 + BC-5.40.001 v1.21 + BC-7.07.001 v1.39.

---

## PART B: SPEC-VS-CODE GROUND-TRUTH VERIFICATION

| Claim | Location | Code Site | Verdict |
|-------|----------|-----------|---------|
| `parse_factory_lock` returns `Err(Malformed)` for empty/absent `holder` (empty-string holder OR absent holder w/ siblings present → Err, NOT silent Ok(None)) | ADR-046 §Decision 2 / BC-4.17.001 PC2/case-1 / BC-7.07.001 PC2 | `crates/factory-lock-parse/src/lib.rs` `parse_factory_lock` lines 207-227 | MATCH — empty/absent holder → `Err(LockError::Malformed(…))` confirmed |
| `extract_yaml_string_value` performs no null-special-casing of the holder field (literal "null" is treated as a non-empty holder string) | ADR-046 §Decision 2 / BC-4.17.001 PC2 | `crates/factory-lock-parse/src/lib.rs` `extract_yaml_string_value` | MATCH — no special handling of "null" literal |
| `renew_lock_with_now` uses bare Duration::seconds(2700) and byte-compares `expires_at` only (no holder/locked_at change) | ADR-046 §Decision 1/F-004 / BC-5.40.001 Invariant 7 | `crates/factory-lock/src/lib.rs` `renew_lock_with_now` | MATCH — bare Duration arithmetic, only `expires_at` rewritten |
| `has_factory_lock_key` detects presence via key-line-only check | ADR-046 §Decision 1 / BC-4.17.001 PC1 | `crates/factory-lock/src/lib.rs` `has_factory_lock_key` | MATCH |
| `is_expired` treats a lock as expired when `now >= expires_at` | ADR-046 §Decision 2 / BC-7.07.001 PC5 | `crates/hook-plugins/verify-factory-lock/src/lib.rs` `is_expired` | MATCH — `now >= expires_at` (inclusive) |
| `trim_git_email` normalizes the holder identity string (strips `<email>` suffix) | ADR-046 §Decision 2 / BC-7.07.001 | `crates/hook-plugins/verify-factory-lock/src/lib.rs` `trim_git_email` | MATCH |
| Step-4 `renew_lock` in `precompact-flush` is identity-blind (renews regardless of holder) | ADR-046 §Decision 3 / BC-5.40.001 Invariant 8 | `crates/hook-plugins/precompact-flush/src/lib.rs` Step 4 | MATCH — no identity check before renew |
| `TTL_SECONDS=2700` (45 minutes) is the non-configurable TTL constant | ADR-046 §Decision 1/F-004 / BC-5.40.001 Invariant 7 | `plugins/vsdd-factory/bin/factory-lock-write.sh` TTL literal | MATCH — literal `TTL_SECONDS=2700` |
| `renew_lock_if_holder`, `classify_identity_resolution`, `SkipReason`, `IdentityResolution` absent from code | ADR-046 design-only; S-17.05 unimplemented | All crates | MATCH — none present; S-17.05 not yet started |

All nine ground-truth checks MATCH. No spec-vs-code divergence found.

---

## PART C: INTERNAL-CONSISTENCY AND CROSS-ARTIFACT AUDIT

### ADR-046 v1.23

- All five decision headings (§Decision 1-5) present and numbered correctly.
- F-004 TTL and factory-lock-write.sh mechanics: internally consistent across §Decision 1,
  File-Change Plan, and Companion Amendment 1.
- Cross-reference to BC-4.17.001 / BC-5.40.001 / BC-7.07.001: each BC correctly named as a
  migration participant.
- §Decision 5 migration participants (BC-4.17.001 as TARGET, BC-5.40.001 as SOURCE): confirmed.
- F-P56-001 fix (empty/absent/null holder = case-1, not silent 0th case): confirmed correct
  across ADR and all three companion BCs.

### BC-4.17.001 v1.26

- §Description enumerates ADR-046 §Decision 1/2/4/5 coverage: CONFIRMED PRESENT (fix from
  F-P58-001, pass-58).
- §Traceability ADR Reference row enumerates §Decision 1(b)/2/4/5: CONFIRMED.
- PC2 (`parse_factory_lock` error path) and PC4 (`extract_frontmatter`): internally consistent
  with Part B code verification.
- Invariant 7/8/EC-015/VP-TBD-7/8/9 carry MIGRATED-per-Decision-5 annotations: CONFIRMED.
- O-P57-001 (no `holder: null` illustrative EC vs BC-7.07.001 EC-011): CONFIRMED STANDING
  NON-DEFECT per D-1114 — BC-4.17.001 makes no false claim about holder-null inputs.
- O-P58-001 (provenance-ID split F-P27-001 vs F-P25-002): CONFIRMED STANDING NON-DEFECT per
  D-1115 — correct provenance, not an inconsistency.
- O-P60-001 (extract_frontmatter opening-fence assumption): CONFIRMED STANDING NON-DEFECT per
  D-1117 — PC2 enforces opening-delimiter upstream.

### BC-5.40.001 v1.21

- §Description includes Decision-5 reconciliation sentence: CONFIRMED (fix from F-P59-001).
- §Traceability ADR Reference row cites `ADR-046 §Decision 1(b)/§Decision 5`: CONFIRMED.
- O-P42-001 (modified: array v1.1-v1.4 rows lack disposition prose): ACCEPTED-tracked,
  UNCHANGED, not a new instance.
- O-P53-DESC-NOOP (§Description "no-op" phrasing under malformed-input arm): CONFIRMED STANDING
  DEFENSIBLE per D-1110.
- O-P60-002 (trim_git_email cross-ref as functional-dependency not migration-participant):
  CONFIRMED STANDING NON-DEFECT per D-1117.
- BC-5.40.001 PC6/Invariant 7/Invariant 8/EC-010/§VP Anchors T-001..T-007 carry
  MIGRATED/RETAINED-AS-HISTORICAL annotations per §Decision 5: CONFIRMED.

### BC-7.07.001 v1.39

- Body cites only `ADR-046 §Decision 1(b)/3/4`: CONFIRMED — not a §Decision 5 participant
  (independently verified in cluster-wide audit at pass-59/D-1116 and re-confirmed here).
- EC-011 (`holder: null` edge case): present at v1.39. No regression.
- §Traceability ADR Reference row correctly omits §Decision 5: CONFIRMED.
- O-P110-item (BC-7.07.001 §Description "no-op" phrasing): CONFIRMED STANDING DEFENSIBLE
  per D-1110.

### Cross-artifact cluster audit (D-1115-codified discipline, run proactively)

| Artifact | ADR-046 §Decisions cited in body | §Traceability enumerates same | Gap? |
|----------|----------------------------------|-------------------------------|------|
| ADR-046 v1.23 | N/A (is the ADR) | N/A | — |
| BC-4.17.001 v1.26 | 1(b)/2/4/5 | 1(b)/2/4/5 | NONE |
| BC-5.40.001 v1.21 | 1(b)/5 | 1(b)/5 | NONE |
| BC-7.07.001 v1.39 | 1(b)/3/4 | 1(b)/3/4 | NONE |

All three companion BCs show zero ADR-Decision-coverage-enumeration gaps. Cluster remains complete.

### Cross-cutting policy checks

- **POLICY 7** (H1↔BC-INDEX title byte-identical): BC-4.17.001/BC-5.40.001/BC-7.07.001 H1 titles
  confirmed byte-identical to BC-INDEX rows ×3. PASS.
- **POLICY 19** (stable ADR anchors, no load-bearing vX.Y): all three companion BCs checked —
  no volatile version pins in Traceability rows. PASS.
- **POLICY 4** (Decision-participation enumeration complete): BC-4.17.001 §Dec 1/2/4/5;
  BC-5.40.001 §Dec 1(b)/5; BC-7.07.001 §Dec 1(b)/3/4; all match body annotations. PASS.
- **POLICY 14/17** (5-leg parity): five-case return-value table verified across all three BCs —
  ADR ≡ BC-7.07.001 ≡ BC-4.17.001 PC2 return-value table identity confirmed. PASS ×3.
- **POLICY 1** (no renumbering; EC-011 new ID): EC-011 in BC-7.07.001 confirmed correct new ID,
  append-only numbering preserved. PASS.
- **POLICY 18** (inputs[] complete): all three companion BCs' `inputs:` frontmatter arrays verified
  cross-referencing each other (cyclic-hash TD [D-1082] acknowledged, not blocking). PASS.
- Capability anchoring: CAP-031/CAP-032 references in capabilities.md confirmed stable. PASS.

---

## PART D: STRUCTURAL AND TRACEABILITY AUDIT

- All four artifacts' `inputs:` frontmatter arrays cross-reference each other as expected
  (cyclic-hash TD [D-1082] acknowledged, not a blocking finding, anchored future architect touch).
- S-17.05 cited in all three companion BCs' `inputs:` arrays, §Traceability §Stories rows, and
  §Story Anchor fields: CONFIRMED. S-17.05 remains REGISTERED (STORY-INDEX v4.392, draft,
  E-17 Wave 5, 8 pts, tdd_mode: strict).
- ARCH-INDEX v3.93 / BC-INDEX v5.18 / VP-INDEX v2.79 / STORY-INDEX v4.392: no version-cell
  change needed — no artifact was edited this pass.
- Input-hashes: ADR-046 `3335ad4` / BC-4.17.001 `6b0b35c` / BC-5.40.001 `6a9cc08` /
  BC-7.07.001 `e73bc01` — all UNCHANGED from the pass-59-corrected values. No recompute needed.

---

## PART E: NON-BLOCKING OBSERVATIONS

### O-P61-001 (LOW severity, HIGH confidence — CORRECTABLE CODE DEFECT, outside frozen spec set)

**Observed:** `crates/factory-lock/src/lib.rs` doc-comments contain stale pre-F-P56-001
semantics at three loci:

1. `renew_lock` algorithm doc (~line 113): states `Ok(None)` is returned when "key absent or
   holder null/absent."
2. Inline comment at the `Ok(None)` arm (~lines 158-160): "Key was present but lock is
   null/absent holder → NoOp."
3. `parse_lock` doc (~line 318): "Ok(None) — key absent or holder is null/absent/empty."

**Ground truth (`factory-lock-parse/src/lib.rs` lines 207-227):** empty-string holder OR absent
holder w/ siblings present → `Err(MalformedLockBlock)`, NEVER `Ok(None)`. The F-P56-001 fix
corrected the FROZEN SPEC SET (ADR-046/BC-4.17.001/BC-7.07.001) to state this correctly; these
doc-comments in the SIBLING implementation file were not swept at the time.

**POLICY 15 status (spec-vs-code correctness for the frozen spec set):** SATISFIED — the frozen
specs are all CORRECT and match code behavior. This is an unswept SIBLING code-doc locus from
the same F-P56-001 class, outside the frozen spec perimeter.

**Classification:** CORRECTABLE CODE DEFECT (stale doc-comment semantics in implementation crate).
Does NOT reset the streak (non-blocking, outside frozen spec perimeter). NOT accept-and-forget.

**Disposition: TRACKED DEFECT-TO-FIX.** Per the CANONICAL PRINCIPLE, the default is to FIX.
Sequencing decision (fix now vs. bundle into S-17.05, which modifies these exact functions) is a
human sequencing decision. Candidate owner: **implementer**. Candidate anchor: **S-17.05**
(touches these exact functions) — PENDING human sequencing confirmation. Recorded as a tracked
defect in STATE.md Drift Items, not filed as accepted/deferred.

### O-P61-002 (adjudicated NON-DEFECT)

**Observed:** BC-4.17.001 has no `holder: null` EC analogous to BC-7.07.001 EC-011.

**Adjudication: NON-DEFECT.** BC-7.07.001 EC-011 was added to CORRECT a prior wrong EC-009
claim (that `holder: null` was a valid accepted input). BC-4.17.001 never carried that wrong
claim; `holder: "null"` is an ordinary non-empty holder string subsumed by case-3 (non-empty
holder, parseable timestamp) in BC-4.17.001's five-case dispatch. There is no missing coverage.
O-P57-001 (same asymmetry, different framing) was already adjudicated NON-DEFECT at D-1114;
this is a re-observation at a more specific locus — same adjudication stands. ACCEPTED-tracked.

### O-P61-003 (adjudicated NON-DEFECT)

**Observed:** BC-5.40.001 PC4 abstracts the empty-holder outcome into the generic "(a) fails
→ no renewal" clause without an explicit five-case enumeration mirroring BC-4.17.001/BC-7.07.001.

**Adjudication: NON-DEFECT.** BC-5.40.001 is the lock-schema BC; it correctly delegates
granular five-case dispatch semantics to the shared truth table and to BC-4.17.001/BC-7.07.001.
BC-5.40.001 makes no contradictory Ok(None)/0th-case claim. The abstraction is architecturally
appropriate — BC-5.40.001 specifies the renewal OUTCOME (no renewal when (a) fails), not the
parsing MECHANISM (which is BC-4.17.001's domain). ACCEPTED-tracked.

---

## PART F: NOVELTY ASSESSMENT

**Novelty: LOW.** All seventeen previously-codified convergence-technique disciplines were
re-verified holding, with zero regression:

1. Append-only numbering (D-~1083)
2. Cross-cluster BC cross-reference (D-~1084)
3. Verification-property placeholder notation (D-~1085)
4. No-backfill version sequence (D-~1086)
5. Story-anchor parity (D-~1087)
6. Input-hash mechanical execution (D-~1088)
7. Inputs-array parity (D-~1089)
8. POLICY 19 stable-ADR-version-pin (D-~1090)
9. ADR-version-pin removal from Traceability (D-~1091)
10. ADR-Decision-complete enumeration (D-~1092)
11. AC-attribution class (D-1104)
12. BC-4.17.001 five-case-table boundary (D-1093)
13. VP-anchor traceability (D-~1094)
14. Cluster-wide audit on migration findings (D-1104)
15. Step-number citation (D-1111)
16. 0th-case/NoOp claim verification (D-1113)
17. ADR-Decision-coverage-enumeration (D-1115) + sweep-both-migration-parties-at-fix-time (D-1116)

O-P61-001 applies an extended sibling-sweep lens to the same F-P56-001 defect class — checking
IMPLEMENTATION crate doc-comments (not just spec files) for stale semantics. This is an application
of the existing TD-VSDD-060 sibling-sweep discipline to a new target locus (doc-comments in the
implementation crate), not a new discipline. Novelty LOW overall.

---

## PART G: FILES REVIEWED

- `.factory/specs/architecture/decisions/ADR-046-posttooluse-hook-authored-statemd-wall-clock-stamping-timestamp-lock-keep-alive.md` (v1.23, full read)
- `.factory/specs/behavioral-contracts/ss-04/BC-4.17.001.md` (v1.26, full read)
- `.factory/specs/behavioral-contracts/ss-05/BC-5.40.001.md` (v1.21, full read)
- `.factory/specs/behavioral-contracts/ss-07/BC-7.07.001.md` (v1.39, full read)
- `crates/factory-lock-parse/src/lib.rs` (parse_factory_lock lines 207-227, extract_frontmatter,
  extract_yaml_string_value — full read)
- `crates/factory-lock/src/lib.rs` (renew_lock_with_now, has_factory_lock_key, doc-comments
  — full read; O-P61-001 loci identified)
- `crates/hook-plugins/verify-factory-lock/src/lib.rs` (is_expired, trim_git_email,
  parse_iso8601 — full read)
- `crates/hook-plugins/precompact-flush/src/lib.rs` (Step-4 renew_lock identity-blind —
  targeted read)
- `plugins/vsdd-factory/bin/factory-lock-write.sh` (TTL literal — targeted read)

---

## Summary

**VERDICT: CLEAN.** Zero blocking findings at any severity. All nine spec-vs-code ground-truth
checks MATCH. Internal-consistency and cross-artifact cluster audit show zero gaps. All
cross-cutting policy checks (POLICY 1/4/7/14/15/17/18/19) PASS.

Three non-blocking observations: O-P61-001 (LOW, HIGH confidence — CORRECTABLE CODE DEFECT,
stale pre-F-P56-001 doc-comments in `crates/factory-lock/src/lib.rs`, outside frozen spec set,
TRACKED DEFECT-TO-FIX pending human sequencing, candidate anchor S-17.05); O-P61-002 (adjudicated
NON-DEFECT, ACCEPTED-tracked — BC-4.17.001 no `holder: null` EC asymmetry is correct per-design);
O-P61-003 (adjudicated NON-DEFECT, ACCEPTED-tracked — BC-5.40.001 PC4 abstraction is correct).

**BC-5.39.001 3-CLEAN streak: ADVANCES 1/3 → 2/3.**
**Frozen set UNCHANGED:** ADR-046 v1.23 + BC-4.17.001 v1.26 + BC-5.40.001 v1.21 + BC-7.07.001 v1.39.
**NEXT:** fresh adversary pass-62 against the SAME unchanged frozen set — 1 more consecutive
clean pass reaches literal BC-5.39.001 3-CLEAN, unblocking S-17.05 TDD.
