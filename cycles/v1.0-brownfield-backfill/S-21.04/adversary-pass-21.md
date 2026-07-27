---
pass: 21
verdict: NOT-CLEAN
reviewed_head: 17921772
fixes_landed_head: 7d195cfa
novelty: high
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-20.md"
---

## MANDATORY PROVENANCE DISCLOSURE

**This pass-21 review is the cascade's FIRST genuinely independent pass.** Produced by the real `vsdd-factory:adversary` agent with intact information asymmetry (read only pass-20 Part A + Fix Mapping; no access to fix implementation details or prior-pass rationale chains). Dispatched with an explicit `model` override because the agent's `model: opus` frontmatter pin fails to resolve (root cause D-927).

**Model-diversity deviation:** The adversary ran on the session model (claude-sonnet-4-6), not the intended `claude-opus` model. The stated "genuine perspective diversity" property this role normally provides was ABSENT for this pass. Dispatch used `model: sonnet` override (validated mitigation per D-927; smoke-test confirmed delivery: `DELIVERY: OK`). This deviation is disclosed here; every future pass using this override MUST carry the same disclosure. ARCHITECTURE DECISION REQUIRED (route to architect) remains open: which model these 7 agents should pin, and whether agents should fail loudly rather than silently on unresolvable model pins (registered at pass-20).

**COMPLETENESS: PARTIAL** — the adversary reached T-001 (Gate PW-B and write-directive gate surfaces) but did not reach T-002..T-009, `step-g-cleanup.md` §G.1, the red-gate-log POLICY-15 attestation gate, or `worktree-identity-preflight.bats`. These surfaces constitute the **pass-22 sweep scope**. The pass-22 adversary MUST explicitly attest coverage of each of these surfaces.

---

## Summary

Pass-21 adversary review of S-21.04 at `reviewed_head 17921772` (post-D-928 PW-B directive-class unification fix). **Adversary verdict: NOT-CLEAN B1/H0/M1 (F-S2104-P21-002 BLOCKER + F-S2104-P21-003 MEDIUM = 2 adversary findings).** Secondary analysis (orchestrator-executed post-fix): F-S2104-P21-004 MEDIUM OPEN. **Total pass-21 record: 3 findings (B1/H0/M2).**

**Count derivation (literal shell per D-449(a), never authored independently):**
```
$ printf '%s\n' F-S2104-P21-002 F-S2104-P21-003 F-S2104-P21-004 | wc -l
3
```
Convention: all `F-S2104-P21-NNN` finding IDs allocated in this pass record (adversary-found + secondary). F-S2104-P21-001 was allocated at D-928 (pre-adversary PW-B directive-class gap) and is NOT counted in the pass-21 tally. Adversary verdict B1/H0/M1 covers F-S2104-P21-002 (BLOCKER) + F-S2104-P21-003 (MEDIUM). F-S2104-P21-004 (MEDIUM OPEN) is a secondary finding discovered during the fix-process analysis.

Trajectory: 14→18→17→12→11→11→9→9→10→11→7→10→10→13→7→6→7→7→12→3→3. Streak: **0/3** (NOT-CLEAN resets per BC-5.39.001; model-diversity deviation does not satisfy fresh-context requirement per pass-20 precedent).

Baseline at `reviewed_head 17921772`: `bats story-worktree-write-path-discipline.bats` → 9/9 ok; `bats worktree-identity-preflight.bats` → 14/14 ok.

**Novelty vs pass-20:** HIGH — both F-S2104-P21-002 and F-S2104-P21-003 are new vectors, not in pass-20's remediation scope. F-S2104-P21-002 is a POLICY 13 FAIL-CLOSED-IMPLICATION-DIRECTION violation introduced by the `F-S2104-P20-003` closure fix (`a5068252`). **Twenty prior passes did not surface it** — material evidence that those passes were degraded by the D-927 adversary defect.

---

## Part A — Findings

| ID | Severity | Location | Description | Refs |
|----|----------|----------|-------------|------|
| F-S2104-P21-002 | BLOCKER | `story-worktree-write-path-discipline.bats` Gate PW-B + write-directive gate | Negated-prohibition escapes both gates: lexical `grep -Ev 'FORBIDDEN\|...\|forbidden\|...\|never\|forbid'` exclusion drops any clause CONTAINING a prohibition token — including when negated or governing a different subject. POLICY 13 FAIL-CLOSED-IMPLICATION-DIRECTION: ambiguity resolved to exclusion (fail-open). Mutant `"Writes that are not yet forbidden under prior policy may target the story worktree CWD."` → PW-B=SILENT(GREEN) WD=SILENT(GREEN). Predates pass-20/21 work; 20 prior passes did not surface it (D-927 degradation evidence). | BC-6.26.001 PC1; POLICY 13; TD-VSDD-059; POLICY 16 |
| F-S2104-P21-003 | MEDIUM | `story-worktree-write-path-discipline.bats` `PWBD_DIRECTIVE_CLASS` | `may` added to `PWBD_DIRECTIVE_CLASS` at D-928 unification (commit `17921772`) had no PW-B-context probe; all of M-P21-A/B/C fire via the bare-imperative alternation, so deleting `may` would regress invisibly. No probe confirmed `may` fires in Gate PW-B context. | BC-6.26.001 PC1; TD-VSDD-059 |

---

### F-S2104-P21-002 — BLOCKER — negated-prohibition escapes both gates

Both Gate PW-B and the write-directive gate ended with a **lexical** exclusion `grep -Ev 'FORBIDDEN|...|forbidden|...|never|forbid'`, which drops any clause merely CONTAINING a prohibition token — including when negated or governing a different subject. POLICY 13 FAIL-CLOSED-IMPLICATION-DIRECTION: ambiguity must be resolved to BLOCK (fail-closed), not PASS (fail-open). The `grep -Ev` lexical exclusion resolves to fail-open.

**Not a re-seed.** The exclusion predates the pass-20/21 work and was inherited without negation analysis. Twenty prior passes did not surface it — material evidence that those passes were degraded by the D-927 adversary defect.

**Orchestrator-executed proof at `17921772` (verbatim):**
```
MUTANT  "Writes that are not yet forbidden under prior policy may target the story worktree CWD."
                                     PW-B=SILENT(GREEN)  WD=SILENT(GREEN)   <-- EVADES BOTH
CONTROL "Writes may target the story worktree CWD."
                                     PW-B=FIRES(RED)     WD=SILENT(GREEN)
```

**Fix history — TWO attempts; first attempt REJECTED as paper fix (TD-VSDD-059). Both recorded.**

*Attempt 1 — commit `b7d3ca58`, REJECTED as paper fix (TD-VSDD-059).* Replaced the lexical check with a perl substitution neutralizing an ENUMERATED negator list (`not yet|no longer|never previously`). Self-certified as "Structural fix, not vocabulary-enumeration (TD-VSDD-059 compliant)" — that assertion was false; it moved the enumeration from prohibition vocabulary to negator vocabulary. Orchestrator-executed proof at `b7d3ca58` (verbatim):
```
CLOSED:   not yet forbidden / no longer forbidden / never previously prohibited   FIRES(RED)
EVADING:  not explicitly forbidden / not strictly forbidden / not currently forbidden /
          not entirely forbidden / isn't forbidden / hardly forbidden /
          not, at present, forbidden / far from forbidden                          ALL SILENT(GREEN)
```

*Attempt 2 — commit `7d195cfa`, ACCEPTED.* Inverted to **fail-closed**: a clause carrying both a prohibited-target and a directive is a violation BY DEFAULT, exempted only by matching a whitelist. **Key design insight: the whitelist is `**Forbidden:**` — a STRUCTURAL marker (bullet label), not a prose lexeme. That is precisely why it is safe: a bullet label cannot be negated. Whitelisting a prose lexeme such as `MUST NOT` would reopen the attack via double negation ("it is not the case that writes MUST NOT..."). Under fail-closed the unbounded negation space works against the attacker: a novel phrasing fails to match the whitelist and therefore FIRES.**

Orchestrator-executed verification at `7d195cfa` (verbatim):
```
MUTANTS FIRE:   M-P17-A / M-P17-C-S2 / M-P20-A / M-P21-D / M-P21-I      all FIRES(RED)
STAY GREEN:     M-P20-B (explanatory) / pristine **Forbidden:** bullet /
                pristine S2 FORBIDDEN sentence                          all SILENT(GREEN)
```
Test-writer additionally proved 23 vectors RED including all 11 negator forms and 4 novel adversarial forms it authored (double negative; FORBIDDEN on a different subject; interposed parenthetical; contraction). Suites at `7d195cfa`: 9/9 and 14/14 (orchestrator-executed). **F-S2104-P21-002 CLOSED at `7d195cfa`.**

---

### F-S2104-P21-003 — MEDIUM — `may` in PWBD_DIRECTIVE_CLASS has no PW-B-context probe (CLOSED)

`may` was added to `PWBD_DIRECTIVE_CLASS` by the P21-001 unification but had no PW-B-context probe; all of M-P21-A/B/C fire via the bare-imperative alternation, so deleting `may` would regress invisibly. No probe existed at `17921772` to confirm that `may` fires independently in Gate PW-B context.

**Probe M-P21-E added:** "Agents may deliver factory artifacts to the story worktree CWD." — orchestrator-verified `FIRES(RED)` at `b7d3ca58`, preserved through `7d195cfa`. **F-S2104-P21-003 CLOSED at `b7d3ca58`, preserved at `7d195cfa`.**

---

### Part B Observation — `gitignored-shadow` gap in prohibited-target alternation (CLOSED)

The adversary flagged (honestly, as unverified) that BC-6.26.001 Invariant 5 terminology might be missing from the prohibited-target alternation. Orchestrator-executed check (verbatim):
```
shadow subtree     COVERED
worktree's shadow  COVERED
gitignored-shadow  NOT COVERED
```
`gitignored-shadow` NOT COVERED. Added to the alternation with probe `M-P21-GS` at `b7d3ca58`, preserved through `7d195cfa`. Verified FIRES(RED). **Part B observation CLOSED at `b7d3ca58`, preserved at `7d195cfa`.** Recorded as a real gap found via an honestly-hedged adversary observation — correct use of the adversarial process.

---

## Fixes applied

**Attempt 1 — `b7d3ca58` (test-writer), REJECTED paper fix (TD-VSDD-059).** Perl negator enumeration: `not yet|no longer|never previously`. Also closed in this commit: F-S2104-P21-003 (M-P21-E probe for `may`), Part B observation (`gitignored-shadow` alternation coverage). Suites at `b7d3ca58`: 9/9 and 14/14.

**Attempt 2 — `7d195cfa` (test-writer), ACCEPTED.** Inverted to fail-closed whitelist (`**Forbidden:**` structural marker). Preserves F-S2104-P21-003 closure and Part B closure from `b7d3ca58`. 23-vector proof including 11 negator forms + 4 novel adversarial forms. Suites at `7d195cfa`: 9/9 and 14/14 (orchestrator-executed).

---

fixes_landed_head: 7d195cfa

## Fix Mapping — Pass-21 (F-S2104-P21-002..004)

| Finding | Fix | Artifact(s) | Status |
|---------|-----|-------------|--------|
| F-S2104-P21-002 | Fail-closed whitelist inversion (`**Forbidden:**` structural marker as sole whitelist token); attempt 1 `b7d3ca58` REJECTED TD-VSDD-059 (negator enumeration); attempt 2 `7d195cfa` ACCEPTED; 23-vector proof + suites 9/9+14/14 | bats (`7d195cfa`) | CLOSED |
| F-S2104-P21-003 | M-P21-E probe added ("Agents may deliver factory artifacts to the story worktree CWD.") → FIRES(RED) | bats (`b7d3ca58`, preserved `7d195cfa`) | CLOSED |
| F-S2104-P21-004 | No fix this pass — authoring-convention documentation required; routed story-writer/technical-writer; pass-22 anchor | — | **OPEN** |

---

## Secondary Finding (F-S2104-P21-004) — MEDIUM — OPEN

**`F-S2104-P21-004` — MEDIUM — fail-closed whitelist imposes undocumented `**Forbidden:**`-bullet authoring convention.**

Because the only exemption from the fail-closed gate is a `**Forbidden:**` bullet label, a canonical RFC-2119 prose prohibition now FIRES as a false positive. Orchestrator-executed at `7d195cfa` (verbatim):
```
"CWD-relative paths MUST NOT be used."            FIRES(RED)   <- false positive on correct prose
"Writers MUST NOT use the story worktree CWD."    FIRES(RED)   <- false positive on correct prose
"Relative paths ... are prohibited."              SILENT(GREEN) <- no directive token; excluded earlier
```
No pristine clause is currently affected (suites green at `7d195cfa`), so this is latent. A future author adding a `MUST NOT` prohibition in `#### Write Discipline` will encounter an opaque gate failure with no guidance in the doc surface.

**This is the same class as F-S2104-P20-003 (false positive on legitimate prose), and it is the accepted price of a non-negatable whitelist.** Do NOT record F-S2104-P21-004 as a defect in the F-S2104-P21-002 fix. The fail-closed design is correct; the documentation gap is the finding.

**Correct remediation:** Document the convention on the gated doc surface and in the story AC — "prohibitions in `#### Write Discipline` MUST be expressed as `**Forbidden:**` bullets, not prose `MUST NOT` or `are prohibited` forms." This is story-writer/technical-writer domain.

**Routing:** OPEN, routed to story-writer/technical-writer for authoring-convention documentation. **Anchored as pass-22 item.**
