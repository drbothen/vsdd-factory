---
pass: 22
verdict: NOT-CLEAN
reviewed_head: 7d195cfa
fixes_landed_head: 63eae07d
novelty: high
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-21.md"
---

# S-21.04 Adversarial Review — Pass 22 (Sweep Burst)

**Date:** 2026-07-28
**Story:** S-21.04 — story-worktree write-path discipline and teardown preflight
**Reviewed HEAD:** `7d195cfa` (post-pass-21 fail-closed whitelist fix)
**Fixes landed HEAD:** `63eae07d` (pass-22 sweep fixes)
**Verdict:** NOT-CLEAN
**Streak:** 0/3 (reset per BC-5.39.001; B1 resets streak)

## Provenance Disclosures (Mandatory — record verbatim in substance)

1. **Model dispatch:** Dispatched with NO model override; agent-definition `model: opus` pin applied and resolved to `claude-opus-5`. The D-931 override mitigation is retracted and was NOT used.

2. **ADR-033 limitation:** The adversary's agent definition claims cross-family diversity (GPT-5); it ran on Claude — the same family that authored every artifact reviewed. Fresh context and information asymmetry are intact; **cross-family independence is absent.** Convergence claims resting on this pass inherit this annotation.

3. **Information-asymmetry deviation (adversary self-reported, `[process-gap]`):** The adversary was instructed to read only Part A + Fix Mapping of pass-21, but the `Read` tool has no partial-section mode — it received the whole file. The adversary states findings were derived from first-principles re-derivation, but the asymmetry was mechanically weaker than specified. Remedy: future dispatches must pre-extract Part A to a scratch file before dispatch. **This is an orchestrator dispatch defect, not an adversary defect.**

**COMPLETENESS: FULL** — all four pass-21-deferred surfaces swept and individually attested: T-002..T-009 (all 8 read in full), `step-g-cleanup.md` §G.1 (141-line worktree copy), red-gate-log POLICY-15 attestation gate, `worktree-identity-preflight.bats` (all 14 assertions audited). Plus F-S2104-P21-004 status verified.

## Part A — Findings

**Counts: B1 / H5 / M4 / L2 = 12 findings + 3 observations**
**Trajectory:** 14→18→17→12→11→11→9→9→10→11→7→10→10→13→7→6→7→7→12→3→3→**12**
**Novelty:** 1.00 (12 new, 0 duplicate)

### Finding Table

| ID | Severity | Location | Description | Refs |
|----|----------|----------|-------------|------|
| F-S2104-P22-001 | **BLOCKER** | story AC-001 Gate cell vs bats HEAD | Gate cell materially misdescribes 4 T-001 predicates and documents the fail-open prohibition-token mechanism that pass-21's BLOCKER fix removed; since spec wins (CLAUDE.md auth r.12) an implementer would rebuild the BLOCKER. Cell's own same-burst coupling mandate violated by passes 20 AND 21 (story had zero `P20`/`P21` occurrences). Compounding `[process-gap]`: the `NAME-SET EQUALITY` gate compares gate *names* while the mandate governs *predicates* — it reported PASS through both violations. A gate that structurally cannot fail on its own violation class is a false-green generator. | POLICY 8; POLICY 4; CLAUDE.md auth r.12 |
| F-S2104-P22-002 | HIGH | BC-6.26.001 §Description/PC2/Inv5/EC-008/T-7 | Self-contradiction on `find` symlink descent; both claims introduced same burst (v1.7). Secondary factual error: `rm -rf` does not follow symlinks. Propagated to 7 downstream sites. | POLICY 4; TD-VSDD-059 |
| F-S2104-P22-003 | HIGH | bats T-002/T-005 vs §G.1 | `[ ! -e ]` DOC-PARITY gate satisfied by prose backtick mentions → deleting the normative predicate leaves suite GREEN. Sibling `[ -L ]` gate hardened for this exact case at F-S2104-P6-003a; never propagated. | TD-VSDD-059; TD-VSDD-060 |
| F-S2104-P22-004 | HIGH | red-gate-log Summary | Story-Task-10-designated single-source-of-truth for suite GREEN attestation stale by three passes (cites `a4ec37d3`, pass-18). D-923/D-925/D-929 each appended an attestation section and each omitted the Summary advance. | POLICY 15; POLICY 22 |
| F-S2104-P22-005 | HIGH | red-gate-log (multiple) | POLICY 15 verbatim-stdout non-compliance: mutant records are narrative paraphrase with no verbatim command; the BLOCKER F-S2104-P21-002 closure rests on "test-writer proved 23 vectors RED" with zero per-vector evidence; a section titled *POLICY-15 attestation gate* is itself a narrative paraphrase. Zero per-guard mutant verification existed for any of `worktree-identity-preflight.bats`'s 14 guards — the direct enabling cause of F-22-006. | POLICY 15 + D-889; POLICY 22; TD-VSDD-059 |
| F-S2104-P22-006 | HIGH | `worktree-identity-preflight.bats` | 11 of 14 assertions were bare whole-file token-presence greps — no section scoping, no negation transparency, no nullification guard. 19 passes of sibling-suite hardening never propagated to the co-attested suite gating the same BC on the same files. Nullification/negation/scope-restriction/relocation mutants all passed. | TD-VSDD-060; F-S2104-P7-003/P9-001/P16-001/P17-003 classes |
| F-S2104-P22-007 | MEDIUM | bats T-002 | `[ ! -d ]` gate used lexical `grep -Ev 'MUST NOT\|wrong\|alone\|…'` → fail-open; same mechanism as the pass-21 BLOCKER, un-swept sibling. | POLICY 13 FAIL-CLOSED-IMPLICATION-DIRECTION; TD-VSDD-060 |
| F-S2104-P22-008 | MEDIUM | bats T-004 PC2c vs §G.1 PC2b | PC2c gate forbade any `proceed to\|with`, including the canonical negated form the sibling PC2b block uses verbatim. Asymmetric with the PC2b gate. F-S2104-P21-004 class, 2nd instance. | F-S2104-P21-004; POLICY 13 |
| F-S2104-P22-009 | MEDIUM | bats T-008 | AC-007(d) mandates preflight *before dispatch*; gate checked only whole-file path presence — no mandate-token, no ordering. T-001 already implements an awk ordering assertion; never applied to the six surfaces. | AC-007(d); TD-VSDD-060 |
| F-S2104-P22-010 | MEDIUM | bats T-004 | Root-skip preceded environment-independent doc-parity gates → on a root runner all AC-006 coverage silently vanishes while bats reports `ok`. "9/9 ok" ≠ "9 executed". | CI positive-coverage axis; AC-006 |
| F-S2104-P22-011 | LOW | preflight bats test (j) | Comment claimed uppercase `ASSERT` anchor "to match the imperative form"; implementation used `grep -iE`. Stated intent not implemented. | stated-intent/impl mismatch |
| F-S2104-P22-012 | LOW | red-gate-log | Volatile line pin `§G.1 L31-40` already decayed (guard block now at §G.1 L36-45). | TD-VSDD-091 |

### Observations (non-findings)

1. **`[process-gap]` test-ID ordinals:** T-007 is physically #9, T-009 is #8 in the bats file — ordinal cites do not match physical bats order. Recommend dropping ordinal cites for `F-S2104-P4-00N` names.
2. **`eval` of doc-extracted text in `_run_teardown_preflight`:** Acceptable for a test harness and load-bearing for anti-tautology (POLICY 11); noted so it is a conscious decision.
3. **`[UNVERIFIED-BY-EXECUTION]` input-hash claims** across story/red-gate-log/BC — POLICY 18 requires literal `bin/compute-input-hash` stdout; adversary had no shell. Field has a defect precedent (F-S2104-P18-007, D-920).

## Structural Finding (Main Payload of This Pass)

Six of twelve findings are the same fix applied once and never swept to siblings:

| Applied-once fix | Un-swept sibling (this pass) |
|-----------------|------------------------------|
| `[ -L ]`→`[ ! -e ]` (P6-003a) | T-002/T-005 `[ ! -e ]` DOC-PARITY gates (F-22-003) |
| pass-21 fail-closed inversion | T-002 `[ ! -d ]` lexical exclusion (F-22-007) |
| PC2b narrow-negative | PC2c `proceed to/with` prohibition (F-22-008) |
| AC-007(a)-(c) section-bounding + T-001 ordering | AC-007(d) six surfaces (F-22-009) |
| 19 passes of hardening | `worktree-identity-preflight.bats` (F-22-006) |
| Pass-14R POLICY-15 discipline | per-test addenda in pass-21 attestation (F-22-005) |

**The asymptotic floor is sustained by a propagation deficit, not a detection deficit** — every class was already discovered, named, and closed once. Hence this burst was scoped as a sweep.

## Coherence Axes That PASSED (no re-derivation needed in pass-23+)

- Frontmatter `behavioral_contracts: [BC-6.26.001]` ↔ body BC table ↔ AC traces bidirectionally complete (AC-001..AC-010)
- BC pin matched BC file version (no pin drift)
- `subsystems: [SS-06]` matched BC `subsystem: "SS-06"`
- 9-test inventory agreed across §Test Plan / §Architecture Mapping / §File Structure Requirements / red-gate-log Summary
- POLICY 21 satisfied

## Part B — Fix Mapping

### Fixes already landed at 63eae07d (orchestrator-verified by literal shell — NOT state-manager's closure)

- **F-22-001 CLOSED** — story v1.26: all 4 divergences resolved; coupling note adds F-S2104-P22 + TD-VSDD-060 sibling-sweep mandate; Gate PW-B authoring constraint added; stale `predicate unchanged from pass-17` / `MUST carry a prohibition token` absent; 44 naked pipes escaped.
- **F-22-002 CLOSED** — BC v1.12 + propagation to 7 sites incl. worktree §G.1; self-contradiction on trailing-slash find dereference corrected; rm-rf symlink-target claim corrected.
- **F-22-003 CLOSED** — structural `^[[:space:]]+\[ ! -e ` in T-002+T-005 + `[ ! -e ]`-before-`[ -L ]` ordering assertion.
- **F-22-006 CLOSED** — all 11 assertions hardened in worktree-identity-preflight.bats.
- **F-22-007 CLOSED** — fail-closed structural exemption replacing lexical exclusion.
- **F-22-008 CLOSED** — negation-transparent pipeline.
- **F-22-009 CLOSED** — mandate-token + ordering + option-first anti-pattern.
- **F-22-010 CLOSED** — root-skip relocated after doc-parity legs.
- **F-22-011 CLOSED** — `-i` flag dropped from preflight bats test (j).
- **F-S2104-P21-004 CLOSED** — widened remedy: "Gate-imposed authoring constraints" blocks on `_shared-context.md`, `step-g-cleanup.md`, and the six AC-007(d) surfaces.

### Findings closed by state-manager this burst

- **F-22-004 CLOSED** — red-gate-log Summary HEAD advanced `a4ec37d3`→`63eae07d` (23/23: 9/9 + 14/14, 2026-07-28); three-pass omission recorded in modified[]; Summary-HEAD advance added to per-pass closure checklist.
- **F-22-005 CLOSED** — non-compliant narrative mutant records re-authored with literal command + captured verbatim stdout; test-writer's 14 mutant-verification records persisted verbatim; Pass-22 attestation section added; pass-21 BLOCKER closure "23 vectors" narrative-only status honestly noted.
- **F-22-012 CLOSED** — volatile `§G.1 L31-40` pin replaced with behavioral anchor ("the `[ -L ]` symlink-guard paragraph in §G.1"); sweep for other volatile pins performed.

---

*Adversary: vsdd-factory:adversary (claude-opus-5; ADR-033 cross-family deviation disclosed above)*
