---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-20T00:00:00Z
phase: 3
inputs:
  - .factory/stories/S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.018.md
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
input-hash: "b0d5375"
traces_to: S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
pass: 7
cascade: S-21.11-v2
previous_review: adv-s21.11-v2-local-pass-6.md
---

# Adversarial Review — S-21.11 v2 (LOCAL cascade, pass 7)

> **Persistence note (state-manager, this burst):** content below is transcribed verbatim from the
> orchestrator-relayed adversary dispatch per POLICY 22 (orchestrator-transcribed; the live
> adversary session ran in a separate context this burst resumes from), following the
> `adv-s21.11-v2-local-pass-N.md` convention established at pass-3.

Artifacts reviewed: story `S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md`
v2.6 (input-hash `97029a5`); `BC-1.03.017.md` v1.17; `BC-1.03.018.md` v1.1; `ADR-039` v1.13
(§Erratum E-005, `status: ratified`); factory-artifacts bundle HEAD `5fcdc851` (D-1046 commit).
Rubric: full `.factory/policies.yaml` (POLICY 1-22).

## Verdict: NOT-CLEAN

1 MEDIUM finding (streak-resetting, `[process-gap]`). Multiple grounding confirmations.

## Finding ID Convention

Finding IDs for the S-21.11 v2 cascade use the format `F-S2111V2-P<PASS>-<SEQ>`, established at
pass-1 (F-S2111V2-P1-001) and continued at pass-2 (F-S2111V2-P2-001..005), pass-3
(F-S2111V2-P3-001), pass-4 (F-S2111V2-P4-001), pass-5 (F-S2111V2-P5-001), and pass-6
(F-S2111V2-P6-001).

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S2111V2-P6-001 | HIGH | RESOLVED | story-writer's EC-011 rewrite (+ AC-013's embedded paragraph + AC-013b's grouping-label residual sites) re-verified this pass — no residual `on_error`-alone / axes-conflation phrasing anywhere across the story's Edge Cases/AC surface. |

## Part B — New Findings (or all findings for pass 1)

### MEDIUM

#### F-S2111V2-P7-001 (MEDIUM, streak-resetting, `[process-gap]`)

**Location:** S-21.11 story Task #29 directive for
`test_all_six_validator_class_plugins_are_fail_closed` (AC-009) and
`test_no_on_error_block_without_fail_closed_when_3arg_executor` (AC-012) — the cite is
LINE-WRAPPED: `…(AC-012, BC-1.03.017` ends at line 1546 and `v1.14 PC11)…` begins at line 1547.

**Defect:** the story pins BC-1.03.017 at v1.17 everywhere else (frontmatter
`behavioral_contracts:` array, body BC table, all other narrative/AC/Task/EC sites), but Task #29's
directive retained a stale `v1.14` cite — a genuine live version-cite parity break under POLICY 8
(`bc_array_changes_propagate_to_body_and_acs`) / POLICY 3 (citation staleness) / POLICY 5
(changelog-exemption scoping). The cite is stale because it is LINE-WRAPPED across a physical line
boundary: `BC-1.03.017` terminates one line and `v1.14 PC11` begins the next. Every prior
cite-parity sweep this cascade has run against this story (v2.2/pass-2, v2.3/pass-3→v2.4/pass-4,
v2.4/pass-5, v2.5/pass-6) used a **contiguous-string grep** (`grep -n "BC-1.03.017 v1.1[0-9]"` or
equivalent single-line pattern), which structurally CANNOT match a cite whose version token sits on
the line AFTER the BC identifier — the pattern's own literal space between `BC-1.03.017` and
`v1.1x` does not exist in the source text at this site; a newline sits there instead. Worse: the
v2.2 and v2.3 changelog rows **falsely attested** sweeping Task #29 — both rows list "Task #29" (or
an equivalent task-number enumeration) among the sites swept, because their sweep matched Task
#29's OTHER, non-wrapped `BC-1.03.017` occurrences on adjacent lines (the same task also contains
correctly-versioned cites elsewhere in its body), not this specific wrapped occurrence. A
task-number-list attestation is not evidence that every cite WITHIN that task was actually
inspected — it only proves the task number appeared somewhere in the sweep's match set. This is a
NEW defect class for this cascade, distinct from the version-cite-propagates/algorithm-
content-does-not family (D-1006) that drove passes 3-6: those were CONTENT-coherence residues
(a predicate correctly narrowed in one artifact/site but not propagated to a sibling stating the
same concept in different WORDING); this is a **sweep-METHODOLOGY** gap — the correct version
number was never in dispute, but the detection mechanism itself (contiguous-string grep) is
structurally blind to a whitespace/line-wrap variant of the exact same literal string it was built
to find. PC11's content is byte-identical v1.14→v1.17 (no algorithm/predicate drift across those
version bumps touched PC11), so this occurrence carried **zero semantic misdirection risk** — an
implementer reading Task #29 would still build the correct PC11 gate — but the version-cite parity
break is real and, per this cascade's own standing discipline, must be closed on citation-hygiene
grounds alone, independent of semantic impact.

**Routed:** story-writer (definitive whitespace-normalized/multiline sweep of the full story body,
replacing the contiguous-string grep methodology that missed this site 4 consecutive times).

**RESOLVED this burst:**
- **story-writer** — S-21.11 v2.6→v2.7: fixed `BC-1.03.017 v1.14 PC11` → `BC-1.03.017 v1.17 PC11`
  at the wrapped site (line 1546-1547). Ran a **whitespace-normalized/multiline detector**
  (`tr '\n' ' ' < file | grep -oE 'BC-1\.03\.017 +v1\.[0-9]+' | sort | uniq -c`) instead of a
  contiguous-string grep, plus a body-only re-scan restricted to the story's live-content line range
  (excluding the `modified:` frontmatter array, the burst-summary blockquote, and the `##
  Changelog` table — the four established-exempt historical zones per POLICY 5 v1.3.5 and this
  cascade's own precedent). **Captured before-fix body-only residual:** exactly 1 wrapped `v1.14`
  hit (Task #29) + 1 `BC-1.03.017 | v1.17` (BC table split-cell) + 2 `v1.12` (the historical
  Previous Story Intelligence row, exempt) + 57 `v1.17` — confirming the wrapped Task #29 cite was
  the ONLY live non-v1.17 site. **Captured after-fix body-only residual (same command, re-run):**
  exactly 1 wrapped `v1.17` (Task #29, now corrected) + 1 `BC-1.03.017 | v1.17` (BC table) + 2
  `v1.12` (historical, unchanged) + 57 `v1.17` — ZERO non-v1.17 LIVE cites remain. Also verified,
  full-file normalized: `BC-1.01.016` (10× `v1.3` live-correct + 1× `v1.0` historical) and
  `BC-1.03.018` (15× `v1.1` live-correct + 6× `v1.0` historical) both clean, no fix needed; `ADR-039`
  cites are all dated `§AMD-002/§AMD-003 RATIFIED vX.Y` amendment-ratification citations, not a
  live document-version pin, so nothing was stale there either. Attested with the ACTUAL captured
  residual-set stdout (per the `[process-gap]` lesson this finding itself establishes), not a
  task-number list. `input-hash` (`97029a5`) intentionally left UNCHANGED — no declared `inputs:`
  file changed this burst; state-manager re-verifies below.

### Confirmations (converged)

- PC11 content byte-identical across v1.14→v1.17 — the parity break carried no semantic
  misdirection; this is a citation-hygiene fix, not a predicate/content fix.
- Predicate-coherence / axes-independence remain CLEAN across both artifacts (re-confirmed, no
  regression from the pass-6 remediation): BC-1.03.017 all Invariants incl. Invariant 10, all PCs
  incl. PC13, Architecture Anchors, Traceability, EC-011; story EC-009/EC-011/AC-005/AC-011/
  AC-013/AC-013b/Task #19b all consistent with the single narrow additive-only axes-independent
  predicate.
- `Timeout{Fuel|Epoch} + on_error=Block + failure_policy=FailOpen → exit 0 / NOT block` holds
  everywhere in the reviewed bundle.
- 18-entry `on_error="block"` `hooks-registry.toml` plugin enum remains EXACT against
  AC-024–AC-041 and PC13's Coverage Set table.
- POLICY 7 (BC H1 ↔ BC-INDEX title cell) and POLICY 8 parity both hold.
- POLICY 18 three-way input-hash parity confirmed (see Grounding confirmations).
- Erratum E-005's re-ratification-not-required disposition remains SOUND.
- `VP-TBD` on BC-1.03.017/BC-1.03.018 (`[F-007]`) remains a sanctioned, previously-disclosed
  deferral — re-observed unchanged, not a new finding.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 1 |
| LOW | 0 |
| ADVISORY | 0 |

**Overall Assessment:** pass-with-findings
**Convergence:** findings remain — iterate (streak resets 0/3; pass-8 required)
**Readiness:** requires revision (routed story-writer; RESOLVED this same burst)

## Grounding confirmations (non-findings, independently re-derived)

- **Story input-hash three-way parity confirmed.** `compute-input-hash` (operator-authoritative
  marketplace rc.23 binary, per-file, per L-EDP1-073) against the S-21.11 story returns `97029a5`,
  matching the frontmatter `input-hash`, the STORY-INDEX catalog row, and the STORY-INDEX E-21
  delivery blockquote — all four agree. Unchanged by this burst: none of the story's declared
  `inputs:` files (ADR-039, `wasm-fuel-exhaustion-detection.md`, `hooks-registry.toml`) changed.
- **Sibling-sweep (TD-VSDD-060) confirmed clean.** A corpus-wide whitespace-normalized grep for
  `BC-1.03.017 +v1\.1[0-6]` (any non-v1.17 live cite) across `.factory` (excluding `logs/`) found
  no other LIVE artifact carrying a stale or line-wrapped BC-1.03.017 cite outside this story's own
  historical Changelog rows.
- **No other line-wrapped stale cite found anywhere in the reviewed bundle.** The whitespace-
  normalized detector, run against the full story body (not just the Task #29 site), surfaced
  exactly one wrapped hit — the one fixed this burst.

## Observations (non-resetting)

- **`[carry-forward, = known F-007]`** BC-1.03.017/BC-1.03.018 both still carry `VP-TBD` under
  POLICY 9 — pre-existing, tracked in STATE.md Blocking Issues/Drift Items, anchored to a future
  dedicated VP-authoring pass. Not a new finding; re-observed unchanged.

## Novelty Assessment

Novelty MEDIUM — a NEW defect class for this cascade, distinct from the version-cite-propagates/
algorithm-content-does-not CONTENT family (D-1006) that drove passes 3-6. This is a
sweep-METHODOLOGY gap: contiguous-string grep is structurally blind to a version cite whose two
halves are split across a line-wrap boundary, and a task-number-list attestation can falsely
report a site as "swept" when the sweep's match only landed on a DIFFERENT occurrence within the
same numbered task. The defect survived 4 consecutive prior sweeps (v2.2/v2.3/v2.4/v2.5) for
exactly this structural reason — not analyst inattention, but a detection-mechanism blind spot.
Closed this burst not by a fifth iteration of the same grep, but by replacing the methodology
itself: a whitespace-normalized/multiline detector (`tr '\n' ' ' | grep -oE`) that treats line
boundaries as ordinary whitespace, plus attestation by CAPTURED residual-set stdout rather than a
task-number list. Orchestration lesson (anchored S-15.03 PRIORITY-A, per the standing methodology-
improvement anchor for this cascade): **cite-parity / version-propagation sweeps MUST use a
whitespace-normalized/multiline predicate, and MUST attest by captured residual-set stdout, NEVER
by a task-number/site-name list** — a task-number list proves the number was matched somewhere, not
that every cite inside that task was inspected.
