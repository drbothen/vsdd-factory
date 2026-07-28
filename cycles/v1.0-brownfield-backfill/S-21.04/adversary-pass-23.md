---
pass: 23
verdict: NOT-CLEAN
reviewed_head: 63eae07d
fixes_landed_head: 888b5b73
novelty: high
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-22.md"
---

# S-21.04 Adversarial Review — Pass 23 (BLOCKER Closure Burst)

**Date:** 2026-07-28
**Story:** S-21.04 — story-worktree write-path discipline and teardown preflight
**Reviewed HEAD:** `63eae07d` (post-pass-22 sweep fixes)
**Fixes landed HEAD:** `888b5b73` (pass-23 BLOCKER closures: F-S2104-P23-001 + F-S2104-P23-002)
**Verdict:** NOT-CLEAN
**Streak:** 0/3 (reset per BC-5.39.001; B2 resets streak)

## Provenance Disclosures (Mandatory — record verbatim in substance)

1. **Model dispatch:** Dispatched with NO model override; agent-definition `model: opus` pin applied and resolved to `claude-opus-5`.

2. **ADR-033 limitation:** The adversary's agent definition claims cross-family diversity (GPT-5); it ran on Claude — the same family that authored every artifact reviewed. Fresh context and information asymmetry are intact; **cross-family independence is absent.** Convergence claims resting on this pass inherit this annotation.

3. **Information-asymmetry remedy (this pass):** Pass-22's Part A was pre-extracted and inlined into the dispatch; the pass-22 and pass-21 full files plus the cycle `INDEX.md` were placed off-limits. This remedies the pass-22 `Read`-has-no-partial-section-mode defect (structural deviation documented in adversary-pass-22.md Provenance §3 and in `[[L-BB-read-tool-no-partial-section-mode]]`). **Adversary confirmed it read no prior pass file.** Asymmetry enforced structurally this pass.

**COMPLETENESS:** Full reads of story spec, BC-6.26.001, `worktree-identity-preflight.bats` (all 14 guards), `step-g-cleanup.md`; partial reads of the main bats suite (ranges given per pass) and red-gate-log; NOT reviewed: bats L593-700, L1350-1992, L2098-2291, L2413-2582 (T-002/T-003/T-005/T-006/T-009 bodies; Gates 2a/2b/3/6/7), `agents/devops-engineer.md`, `step-d5-adversary-convergence.md`, ADR-031, the two `.lobster` bodies (`workflows/code-delivery.lobster`, `workflows/greenfield.lobster`). Carry these forward as pass-24 sweep scope.

## Part A — Findings

**Counts: B2 / H4 / M6 / L2 = 14 findings**
**Trajectory:** 14→18→17→12→11→11→9→9→10→11→7→10→10→13→7→6→7→7→12→3→3→12→**14**
**Novelty:** high (14 new findings; 0 duplicates)

### Finding Table

| ID | Severity | Location | Description | Refs |
|----|----------|----------|-------------|------|
| F-S2104-P23-001 | **BLOCKER** | bats `write_discipline_prose` construction | Blockquote-strip added by the pass-22 sweep (`grep -Ev '^[[:space:]]*>'`) blinded SIX section-wide negative gates (PW-B, 2b(a), 2b(c), scope-restriction, 4, 5) and re-opened M-P21-A/B/C + M-P17-D verbatim. The in-file mutant-proof comment was FALSE: the anchor-count gate reads `rendered_write_discipline`, which has no blockquote strip, so the claimed RED never occurred. Directly regressed the F-S2104-P18-002(b) anti-fail-open ruling stated 3 lines above. Root cause: a gate-fires-on-correct-prose false positive was remedied by blinding the gate family instead of narrowing the predicate. | POLICY 13 FAIL-CLOSED; TD-VSDD-059; F-S2104-P18-002(b) |
| F-S2104-P23-002 | **BLOCKER** | red-gate-log pass-22 mutant records | 7 of 13 records mutated `step-g-cleanup.md` to "verify" guards that never read it; zero `bats` invocations; zero `not ok` frames; 0/14 preflight guards actually verified. F-S2104-P22-005 paper-fixed with tautological evidence in POLICY 15 surface form — strictly more dangerous than the honest narrative gap it replaced. | POLICY 15 + D-889; POLICY 11; POLICY 22; TD-VSDD-059 |
| F-S2104-P23-003 | HIGH | story AC-002 Gate cell vs bats | Cell states the T-004 PC2c gate fires "unconditionally … on any match"; at HEAD it is negation-transparent. Spec-wins ⇒ rebuild reinstates the P22-008 false positive. F-S2104-P22-001 class RECURRED on a sibling AC in the same burst that rewrote AC-001. `step-g-cleanup.md` carries the identical false description. | POLICY 8; AC-001 coupling mandate; TD-VSDD-060 |
| F-S2104-P23-004 | HIGH | story AC-001 Gate cell vs bats | Cell describes the domain with fenced code INCLUDED and omits the blockquote strip entirely; same-burst coupling mandate violated in the very burst that edited this cell. | POLICY 8; TD-VSDD-060 |
| F-S2104-P23-005 | HIGH | BC-6.26.001 EC-005 | Retains the v1.11 retracted claim "falls through to find which returns empty → false PC2a". v1.12 corrected six sites; EC-005 is an un-swept seventh in the same file, now contradicting the corrected EC-008 three rows below. | F-S2104-P22-002; TD-VSDD-060 |
| F-S2104-P23-006 | HIGH | preflight guards (d)(e)(f)(g) | All four "nullification guards" evaluate only `head -1`; an APPENDED nullifier is missed — fail-open in exactly the direction P22-006 hardened against. Sibling (l) implements the correct all-lines/affirmative-set form and was not swept. Blast radius 4. | Partial-Fix Regression Discipline (b); POLICY 13; TD-VSDD-060 |
| F-S2104-P23-007 | MEDIUM | preflight guard (g) vs `agents/adversary.md` | Scope-restriction class `\bis not\b\|\bdoes not\b` matches adversary.md's own canonical prose ("that is NOT path-corroborated … MUST NOT be reported"); non-firing depends solely on `head -1` line order. THIRD live instance of gate-fires-on-correct-prose. | F-S2104-P22-008 class |
| F-S2104-P23-008 | MEDIUM | preflight guard (b) extractor | Bounds only on `^#### `/`^---`; over-captures `### Perimeter 2` + `### Perimeter 3`. In-story precedent `_extract_write_discipline_section` bounds on `^#### `,`^### `,`^## ` — un-swept. | TD-VSDD-060 |
| F-S2104-P23-009 | MEDIUM | 5 sites | Mis-anchored finding IDs: blockquote-strip cites P22-004 (stale-SHA finding); PC2c constraint cites P22-002 (BC symlink) instead of P22-008; T-008 constraint cites P22-003 instead of the carried P21-004; ordering record labelled F-22-006 vs bats anchor P22-003b; log uses non-canonical `F-22-NNN` namespace. | POLICY 4; POLICY 5; POLICY 1 |
| F-S2104-P23-010 | MEDIUM | red-gate-log narrative | ≥5 stale narrative bats line pins, all decayed ("line 652"→713, "line 668"→730, "line 695"→767, "line 820"→1029, "line 897"). F-S2104-P22-012 closed ONE pin and left ≥5 same-class siblings in the same file. | TD-VSDD-091; TD-VSDD-060 |
| F-S2104-P23-011 | MEDIUM | 2 of 6 AC-007(d) surfaces | The P21-004 remedy landed constraint blocks on 4 of 6 surfaces; `workflows/code-delivery.lobster` and `workflows/greenfield.lobster` have none though T-008 gates them identically. *(pending intent verification — YAML comment form may have been judged inapplicable, though `fix-pr-delivery/SKILL.md` used a `#` comment form)* | Partial-Fix Regression Discipline (b); AC-007(d) |
| F-S2104-P23-012 | MEDIUM | bats `_assert_g1_ref` | P22-009's "mandate-token" gate accepts `before\|first\|run` case-insensitively — tokens so common the gate approximates the presence check it was meant to strengthen. Near-vacuous. | F-S2104-P22-009; POLICY 13 |
| F-S2104-P23-013 | LOW | **NON-FINDING — re-evaluated** | Filed as forward-dated attestation (`2026-07-28` vs then-current `2026-07-27`). The session crossed midnight; the current date is now 2026-07-28, so the dates are consistent and no defect remains. **Record as NON-FINDING.** General lesson: date-monotonicity checks must account for session-spanning bursts (F-S2104-P12-010 class; not applicable here). | F-S2104-P12-010 class (not applicable) |
| F-S2104-P23-014 | LOW | red-gate-log records header | Header claimed "14 vectors"; 13 code blocks present; guards (b)(c)(e)(f)(i)(j)(k) had no record despite (e)/(f)/(k) being hardened that burst. | POLICY 15 |

### Pass-22 Closure Verification Table

| Finding | Status | Evidence |
|---------|--------|----------|
| F-S2104-P22-001 | GENUINELY-CLOSED | Story AC-001 rewritten; coupling mandate added; divergences resolved at 63eae07d |
| F-S2104-P22-002 | PARTIAL | BC v1.12 corrected 6 of 7 sites; EC-005 un-swept → becomes F-S2104-P23-005 |
| F-S2104-P22-003 | GENUINELY-CLOSED | Structural `[ ! -e ]` gate + ordering assertion in T-002+T-005 verified at 63eae07d |
| F-S2104-P22-004 | GENUINELY-CLOSED | Summary HEAD advanced 63eae07d (23/23) by D-933; recurrence class noted |
| F-S2104-P22-005 | PAPER-FIX | 14 tautological mutant records replaced records that mutated wrong artifact — BLOCKER F-S2104-P23-002 |
| F-S2104-P22-006 | PARTIAL | 11/14 preflight assertions hardened; head-1 limitation on guards (d)(e)(f)(g) → F-S2104-P23-006 |
| F-S2104-P22-007 | GENUINELY-CLOSED | Fail-closed structural exemption verified at 63eae07d |
| F-S2104-P22-008 | GATE-CLOSED-BUT-SPEC-REGRESSED | Negation-transparent pipeline implemented; AC-002 spec still describes unconditional fire → F-S2104-P23-003 |
| F-S2104-P22-009 | PARTIAL | Mandate-token + ordering gates added; token set near-vacuous → F-S2104-P23-012 |
| F-S2104-P22-010 | GENUINELY-CLOSED | Root-skip relocated after doc-parity legs verified at 63eae07d |
| F-S2104-P22-011 | GENUINELY-CLOSED | `-i` flag dropped from preflight bats test (j); case-sensitive match confirmed |
| F-S2104-P22-012 | PARTIAL | ONE volatile pin replaced; ≥5 same-class siblings remain → F-S2104-P23-010 |

**Summary:** GENUINELY-CLOSED 5 · PARTIAL 5 · PAPER-FIX 1 · GATE-CLOSED-BUT-SPEC-REGRESSED 1 · UNVERIFIABLE 0

### Structural Observations (high value — carry to pass-24)

1. **`[process-gap]` POLICY 15/D-889 loophole** — it mandates "verbatim command + captured stdout" but NOT that the command be *the predicate under test* against *the artifact that guard binds in `setup()`*. F-S2104-P23-002 satisfied the letter and defeated the purpose. Needs a fourth clause: a per-guard record MUST cite the guard's own predicate, its bound artifact, and the resulting `not ok` frame.

2. **`[process-gap]` No invariant over the bats domain-construction pipeline** — six negative gates share one `local` variable whose definition a one-line edit can narrow, with nothing detecting it. This class has now recurred twice (fence exclusion, then blockquote strip). A domain-invariant assertion would have caught both.

3. **The AC-001 Gate cell is structurally unmaintainable** — a ~9,000-character table cell describing 21 gates across 5 domains, whose coupling mandate has failed at passes 16, 17, 18, 19, 22, and 23. Narrative prose in a table cell cannot stay synchronized with a 2,582-line test file; a generated-and-diffed manifest is the mechanism that closes the class.

4. Two recurring meta-patterns, three live instances each: (a) gate-fires-on-correct-prose remedied by blinding the gate or constraining the author, never by a negation/meta-aware predicate; (b) `head -1` single-line evaluation presented as a whole-artifact guard.

## Part B — Fix Mapping

### Closed at `888b5b73` (orchestrator-verified by literal shell — NOT state-manager's closure)

- **F-S2104-P23-001 CLOSED** — blockquote whole-line strip replaced with a **marker-only** strip (`sed 's/^[[:space:]]*>[[:space:]]*//'`), which preserves content in the domain instead of removing it; the T-001/PW-B authoring-constraint annotation was relocated in `_shared-context.md` from inside `#### Write Discipline` to before the heading. Orchestrator proof (literal): old strip → mutant content empty (gate blind); new strip → `Anchor every write to the story worktree CWD.` survives and matches the bare-imperative directive class → gate FIRES. Old fail-open predicate count now 0.

- **F-S2104-P23-002 CLOSED** — the 13 tautological records were replaced with 14 genuine per-guard records, each naming the guard's bound artifact, a targeted mutation to THAT artifact, a real `bats -f` invocation with a verbatim `not ok` frame, and a restore `ok` frame. `not ok` frame count went 0 → 45. Orchestrator spot-check: guard (a) binds `$ADVERSARY_AGENT` and requires `^#{3,4}[[:space:]].*Worktree-Identity Preflight`; the record's cited frame (`line 48`, `[ "$status" -eq 0 ]' failed`) matches the implementation.

  **Residual gap (known POLICY 15 shortfall — do NOT mark P23-002 fully clean without it):** only 3 of the 14 records repeat the explicit `bats -f` command; records (b) onward show `RED stdout:` without restating the command. POLICY 15 requires command AND stdout per record. This is a known remaining shortfall on the new records that pass-24 must close.

### Open — 12 findings (human-directed scope limit this burst: BLOCKERs only)

| Finding | Status | Routing (pass-24) |
|---------|--------|-------------------|
| F-S2104-P23-003 | OPEN | story-writer |
| F-S2104-P23-004 | OPEN | story-writer |
| F-S2104-P23-005 | OPEN | product-owner |
| F-S2104-P23-006 | OPEN | test-writer |
| F-S2104-P23-007 | OPEN | test-writer |
| F-S2104-P23-008 | OPEN | test-writer |
| F-S2104-P23-009 | OPEN | state-manager |
| F-S2104-P23-010 | OPEN | state-manager |
| F-S2104-P23-011 | OPEN | story-writer *(pending intent verification)* |
| F-S2104-P23-012 | OPEN | test-writer |
| F-S2104-P23-013 | NON-FINDING | (re-evaluated; date-monotonicity session-spanning burst) |
| F-S2104-P23-014 | OPEN | state-manager |

These 12 OPEN findings (excluding P23-013 NON-FINDING) are the pass-24 anchor set.

---

*Adversary: vsdd-factory:adversary (claude-opus-5; ADR-033 cross-family deviation disclosed above)*
