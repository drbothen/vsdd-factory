---
pass: 24
verdict: NOT-CLEAN
reviewed_head: 888b5b73
fixes_landed_head: 9b12aa00
novelty: high
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-23.md"
---

# S-21.04 Adversarial Review — Pass 24 (Targeted Re-Review: Pass-23 BLOCKERs)

**Date:** 2026-07-28
**Story:** S-21.04 — story-worktree write-path discipline and teardown preflight
**Reviewed HEAD:** `888b5b73` (post-pass-23 BLOCKER closures)
**Fixes landed HEAD:** `9b12aa00` (pass-24 full fix chain: F-S2104-P24-001/002/003/004/005/006)
**Verdict:** NOT-CLEAN
**Streak:** 0/3 (B2 resets; zero CLEAN verdicts across 24 passes)

## Provenance Disclosures (Mandatory — record verbatim in substance)

1. **Model dispatch:** NO model override applied. Agent-definition `model: opus` resolved to `claude-opus-5`.

2. **ADR-033 limitation:** Cross-family claim (GPT-5) NOT satisfied. The adversary ran on Claude — the same family as all authoring agents in this cascade. Fresh context and structural information asymmetry are intact; **cross-family independence is absent.** Convergence claims resting on this pass inherit this annotation. Recorded per ADR-033 governance.

3. **Asymmetry enforcement (this pass):** Pass-23 Part A was pre-extracted and inlined into the dispatch prompt; the pass-23 and pass-22 full files plus the cycle `INDEX.md` were placed off-limits. The adversary confirmed it opened no prior pass file. Asymmetry enforced structurally. **The adversary independently re-derived the marker-strip verification and found it correct for single `>` and insufficient for `>>`, list prefixes, and the sibling domain — NOT accepting the orchestrator's marker-strip evidence as authoritative. This is a POLICY 22 success.**

4. **Scope:** This was a **targeted re-review** scoped by human direction to the two pass-23 BLOCKERs (`F-S2104-P23-001` and `F-S2104-P23-002`) and their remediation. Not a full-surface pass. The 12 open findings from pass-23 are out of scope per explicit human direction.

**COMPLETENESS:** Main bats suite (extensive ranges including full AC-001 gate battery and all M-P21 probes; T-005/T-006/T-008/T-009/T-007 bodies); `_shared-context.md` L55-174; `rules/worktree-protocol.md` §Cleanup; full `worktree-identity-preflight.bats`; red-gate-log L1190-1540. NOT covered (carry to pass-25): bats T-002/T-003 bodies; `agents/devops-engineer.md` body; `step-d5-adversary-convergence.md` body; ADR-031 content; the two `.lobster` bodies.

## Part A — Findings

**Counts: B2 / H2 / M2 / L0 = 6 findings**
**Trajectory:** 14→18→17→12→11→11→9→9→10→11→7→10→10→13→7→6→7→7→12→3→3→12→14→**6**
**Novelty:** high (findings are recurrences of previously identified classes but at previously undetected sites)

### Finding Table

| ID | Severity | Location | Description | Refs |
|----|----------|----------|-------------|------|
| F-S2104-P24-001 | **BLOCKER** | bats `_build_section_prose` marker strip | The pass-23 fix applied a marker-only strip (`sed 's/^[[:space:]]*>[[:space:]]*//'`) that is non-recursive: `>> Anchor every write to the story worktree CWD.` retains a residual `> ` after one application, leaving `> Anchor...` in the rendered prose. The `^`-anchored bare-imperative class in `PWBD_DIRECTIVE_CLASS` matches `^Anchor` not `^> Anchor`, so Gate PW-B fires SILENTLY. M-P21-A evades detection with one extra `>` character. | POLICY 13 FAIL-CLOSED; TD-VSDD-059 |
| F-S2104-P24-002 | **BLOCKER** | bats `spec_path_prose` construction | The pass-23 marker-normalization fix applied to `write_discipline_prose` was NOT applied to the sibling domain `spec_path_prose`. The `spec_path_prose` extraction used bare `tr '\n' ' '` with no blockquote strip. The pass-23 fix RELOCATED its authoring annotation into precisely this unnormalized domain — a blockquoted `.factory/` write-directive above `#### Write Discipline` now fires nothing despite the referent matching. Textbook TD-VSDD-060 sibling-site sweep failure: the fix normalized one domain and moved its own new content into the sibling it did not normalize. | TD-VSDD-060; POLICY 13 |
| F-S2104-P24-003 | HIGH | bats pass-23 fix | The F-S2104-P23-001 fix had ZERO load-bearing assertions verifying its own predicate. Reverting the strip left the suite 9/9 GREEN — the in-file `Proof:` claim was unexecuted by any probe. The M-P21 probe family re-implements gate predicates over synthetic literals and never exercises the domain-construction pipeline (`extract → strip → tr → split`) where all three BLOCKERs lived. Root cause of the whole recurrence pattern in this cascade: a probe that verifies a helper works is NOT sufficient if it does not verify that production calls the helper. Call-site parity gates are the missing mechanism. | TD-VSDD-059; POLICY 11; POLICY 22 |
| F-S2104-P24-004 | HIGH | bats `PWBD_DIRECTIVE_CLASS` | The bare-imperative alternation is `^`-anchored with a `**Label:** ` prefix allowance but NO allowance for a `- ` list-item prefix. A hyphen-bulleted imperative (e.g., `- Anchor every write to the story worktree CWD.`) leaves the imperative mid-clause after the list marker, so the `^`-anchor fails. Numbered lists DO split on sentence boundaries and fire correctly (the sentence-split makes them line-initial), creating an arbitrary asymmetry: numbered-list imperatives are guarded, hyphen-list imperatives are not. | POLICY 13 |
| F-S2104-P24-005 | MEDIUM | red-gate-log `### Pass-23 assertion-site attestation` | POLICY 15 self-disclosure in D-934 understated its own shortfall: claimed "3 of 14 records include the explicit command." True count: 1-2 of 14 RED records included an explicit `bats -f` command; 0 of 14 records included the explicit `git restore` command in their GREEN stdout headings (the undisclosed gap). Mutations were narrated in prose rather than executed. **Attribution: the inaccurate "3 of 14" figure originated with the ORCHESTRATOR**, which asserted it without verification and passed it to state-manager, which persisted it. This is the second orchestrator evidence-fidelity failure of the session in the same relay class as F-S2104-P23-002. | POLICY 15; POLICY 22 |
| F-S2104-P24-006 | MEDIUM | bats `_assert_doc_marker 'Write Discipline'` | The pass-23 relocated annotation contains the literal `` `#### Write Discipline` `` (in backtick-code form), so the positive gate `_assert_doc_marker 'Write Discipline'` over `spec_path_section` became satisfiable by the annotation alone, independently of whether the real `#### Write Discipline` heading is present. The defense-in-depth properties (anchor-count guard and empty-block guard) still catch heading deletion, but the positive gate is no longer a discriminating test — it passes on a spec that contains only the annotation and no heading. | POLICY 11; TD-VSDD-059 |

## Pass-23 BLOCKER Closure Verification

| Finding | Status | Evidence |
|---------|--------|----------|
| F-S2104-P23-001 | **PARTIAL** | The pass-23 fix is genuine, not a paper-fix: whole-line blockquote strip verifiably removed (marker-only strip applied instead); relocation of the authoring annotation is genuinely outside every `#### Write Discipline` extraction and verified unable to spuriously satisfy Gates 3/6(a)/7(a)/canonical-target. M-P17-D genuinely restored (Gate 2b(a) is an unanchored token grep that fires on the annotation). However, M-P21-A/B/C remain OPEN: the single-`>` sentence-boundary case is closed; `>>`, `- ` list prefix, and post-colon placement are not. The fix itself was unguarded — no probe verifies the strip is recursive rather than single-pass. |
| F-S2104-P23-002 | **GENUINELY-CLOSED** | Adversary verified all 14 records line-by-line against the implementation: every cited line number, quoted assertion text, and bound artifact matches exactly; RED payload text matches the `echo` message format verbatim; 14/14 guards (a)-(n) covered one record each with no duplicates or gaps. The pass-22 tautology (7/13 mutating `step-g-cleanup.md`, zero `bats`, zero `not ok`) is fully eliminated. |

## Part B — Fix Mapping

All 6 findings closed in the fix burst at `9b12aa00`. Summary:

| Finding | Closed at | Mechanism |
|---------|-----------|-----------|
| F-S2104-P24-001 | `02152c01` | `_build_section_prose` shared helper with recursive strip `sed -E 's/^([[:space:]]*>[[:space:]]*)+//'` — `+` quantifier handles `>>` and deeper nesting |
| F-S2104-P24-002 | `02152c01`/`20aee6a2` | `spec_path_prose` now routed through `_build_spec_path_section_prose` → `_build_section_prose`; call-site parity probe added at `9b12aa00` (Leg E) |
| F-S2104-P24-003 | `02152c01`+`20aee6a2`+`9b12aa00` | Structural fix: real-fixture pipeline probe (test 2 of 10) with 5 legs (A: `>>` double-prefix; B: list prefixes; C: spec_path region; D: pristine-silent; E: call-site parity Leg E). `_build_section_prose` is a shared helper called 21×; Leg E is a self-referential source gate asserting every `*_prose="$(` assignment routes through a `_build_*` builder |
| F-S2104-P24-004 | `02152c01` | `_build_section_prose` adds list-marker strip: `sed -E 's/^[[:space:]]*[-*+][[:space:]]+//'` plus `sed -E 's/^[[:space:]]*[0-9]+\.[[:space:]]+//'`; Leg B probes verify |
| F-S2104-P24-005 | `9b12aa00` (red-gate-log) | All 14 records (a)-(n) now carry explicit `bats -f "<test_name>" worktree-identity-preflight.bats` in RED stdout headings and explicit `git restore <artifact>` in GREEN stdout headings |
| F-S2104-P24-006 | `02152c01` | `_assert_doc_marker 'Write Discipline'` → `_assert_doc_marker '^#### Write Discipline'`; heading-form now required; annotation cannot self-satisfy |

**Three-iteration history (F-S2104-P24-003, record honestly):** At `02152c01` Leg C guarded only the builder implementation — the orchestrator proved reverting the CALL SITE still passed `ok`. At `20aee6a2` the `_build_spec_path_section_prose` wrapper was added — the orchestrator proved the call-site reversion STILL passed `ok`. At `9b12aa00` Leg E (call-site parity source gate) was added — the orchestrator proved all four reversion classes (builder impl, recursive strip, spec_path call site, write_discipline call site) now turn the suite RED. Three iterations, each independently verified by the orchestrator rather than accepted by report.

**Adversary meta-finding (record prominently):** Every remediation in this cascade has been applied to the one domain-construction site named in the finding, not to its siblings, and without a probe that exercises the domain construction itself. Until a probe injects a mutant into a real fixture and runs the actual construction pipeline, the class recurs and the suite reports green. The `9b12aa00` pipeline probe (Leg E call-site parity + Leg A/B/C/D builder coverage) is the first guard that makes all three prior BLOCKER classes detectable simultaneously.
