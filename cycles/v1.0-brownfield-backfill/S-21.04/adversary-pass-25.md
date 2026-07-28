---
pass: 25
verdict: NOT-CLEAN
reviewed_head: "5ccf5869"
fixes_landed_head: "4dc27251"
novelty: high
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-24.md"
findings_count: 17
severity_breakdown: "B3/H4/M8/L2"
deferred_count: 1
streak: "0/3"
trajectory_append: 17
model_override: false
model_resolved: "claude-opus-5"
adr033_deviation: "ADR-033 cross-family limitation — cross-family claim (GPT-5) NOT satisfied; ran on Claude, same family as authoring agents; fresh context + information asymmetry intact, cross-family independence absent"
asymmetry_enforcement: "pass-24 Part A inlined; pass-24/23/22 files and cycle INDEX.md off-limits; adversary confirmed it opened none"
policy22_note: "Adversary explicitly did NOT accept orchestrator marker-strip verification as authoritative; re-derived it independently — POLICY 22 success"
---

# Adversary Pass 25 — S-21.04 story-worktree write-path discipline

**Date:** 2026-07-28
**Reviewed HEAD:** `5ccf5869` (corrected from `5ccf5669`; see M04)
**Fixes-landed HEAD:** `4dc27251`
**Verdict:** NOT-CLEAN — B3/H4/M8/L2 = 17 findings + 1 deferred
**Streak:** 0/3 (BC-5.39.001)

## Provenance Disclosure

(1) NO model override; `model: opus` resolved to `claude-opus-5`.

(2) **ADR-033 limitation** — cross-family claim (GPT-5) NOT satisfied; ran on Claude, same family as authoring agents; fresh context + information asymmetry intact, cross-family independence absent.

(3) Asymmetry enforced structurally — pass-24 Part A inlined; pass-24/23/22 files and cycle `INDEX.md` off-limits; adversary confirmed it opened none.

(4) The adversary explicitly did NOT accept the orchestrator's marker-strip verification as authoritative and re-derived it independently — record as a POLICY 22 success.

## Completeness Statement

Full reads of: main bats suite (all 2891 lines incl. T-002 and T-003 bodies); preflight guards (a)-(g) in full and (h)-(n) by header + shared predicate; `step-g-cleanup.md` (147 lines); `_shared-context.md` L50-139; **`step-d5-adversary-convergence.md` in full**; `devops-engineer.md` §Worktree Cleanup; `adversary.md` L40-79; **both `.lobster` bodies** at the T-008 surface; **ADR-031 in full (568 lines)**; BC-6.26.001 edge cases + changelog; story spec; red-gate-log attestations.

NOT covered: preflight guards (h)-(n) bodies in depth; bats T-004/T-005/T-006/T-009 bodies; five SKILL.md/rules surfaces; BC-6.26.001 §Description/§Preconditions/§Postconditions/§Invariants bodies; story §Behavioral Contracts table / §Token Budget. Carry forward as pass-26 scope.

## Closure Verification of Prior Passes

**Pass-24 set:** All 6 GENUINELY-CLOSED. P24-001 and P24-004 closed for probed vectors with a residual class → B03; P24-003 with a caveat → H02.

**Pass-23 set:** 2 REGRESSED (P23-006, P23-007); P23-010 genuinely closed in the log but class re-introduced elsewhere (→ M06); 9 GENUINELY-CLOSED; 1 non-finding; 1 partly unverifiable. **No classic PAPER-FIX — the fixes were real; two made their guards weaker, and the burst had no evidence discipline to catch it.**

## Part A — Findings

### F-S2104-P25-B01 | BLOCKER | preflight guards (e)+(g) | REGRESSION

**Summary:** REGRESSION introduced by the pass-23 fix. The all-lines affirmative-set migration made both guards WEAKER and converted the red-gate-log's OWN recorded mutants from RED to GREEN.

`any-affirmative` passes if ANY non-nullified occurrence survives anywhere in the file, so an in-place mutation of the mandate line is masked by an incidental sibling occurrence. Orchestrator-verified: `path-corroborated` occurs on exactly 2 lines (L58 mandate, L61); mutating L58 leaves L61 affirmative → GREEN. Under the replaced `head -1` form, `head -1` returned L58 → RED. Guard (e) identical (`checks out NOTHING under` ×2). Guard (f) fail-open in both forms (×3).

POLICY 13 FAIL-CLOSED-IMPLICATION-DIRECTION: the fail-closed form is zero-nullified AND ≥1-affirmative, or rule-bound.

**References:** POLICY 13; POLICY 15/D-889; F-S2104-P23-006/007

---

### F-S2104-P25-B02 | BLOCKER | write-directive gate

**Summary:** Gate PW-B became a fail-closed `**Forbidden:**` whitelist at F-S2104-P21-002, but the write-directive gate — the ONLY gate covering `### Spec-Path Discipline` above `#### Write Discipline` — retained the vocabulary-enumeration perl neutralizer that fix declared structurally defective. All 12 catalogued negation forms (M-P21-D/F/G/H/I..T) SILENT there, reviving M-P19-H. Sibling-sweep gap surviving four passes.

**References:** TD-VSDD-060; POLICY 13

---

### F-S2104-P25-B03 | BLOCKER | `_build_section_prose` + `PWBD_DIRECTIVE_CLASS`

**Summary:** `^`-anchored bare-imperative class remained position-dependent and the normalizer did not guarantee position.

(a) Marker strip single-pass/order-fixed: `- > `, `- - `, `1. - `, `* * ` left residue; `- **Anchor**` missed (class requires `:\*\*`).
(b) Marker-free: `tr '\n' ' '` destroys line boundaries, so an imperative whose preceding line does not end `.`+capital gets no clause boundary → `^Anchor` never matches → PW-B and write-directive gate both SILENT.

**References:** extends F-S2104-P24-001/004; POLICY 13

---

### F-S2104-P25-H01 | HIGH | red-gate-log §Pass-23-closure

**Summary:** 8 of 12 pass-23 closures (003/004/005/006/007/008/011/012) appeared ONLY as narrative claims in the Summary row + routing table — zero verbatim command/stdout, including all four bats-guard closures. **This is the mechanism that let B01 through:** had the recorded (e)/(g) mutants been re-run, the regression would have surfaced in-burst.

**References:** POLICY 15; POLICY 22; D-889

---

### F-S2104-P25-H02 | HIGH | pipeline probe Leg E

**Summary:** The call-site-parity gate keyed on one rigid form `_prose="$(`. Unquoted assignment, any domain not suffixed exactly `_prose`, and `*_prose_nosplit` were invisible; `grep -v '_build_'` is a whole-line lexical test defeated by a trailing comment.

**References:** POLICY 13; F-S2104-P24-003

---

### F-S2104-P25-H03 | HIGH | bats ×3 + story ×3

**Summary:** A fix-burst specialist pre-allocated `F-S2104-P25-001` — an ID in the ADVERSARY's next-pass namespace — which collided with pass-25's real allocation. Adversary finding IDs are allocated by adversary passes only.

**References:** POLICY 1; POLICY 16

---

### F-S2104-P25-H04 | HIGH | `step-d5-adversary-convergence.md`

**Summary:** Instructed a `.factory/**` write via a bare CWD-relative path — byte-equivalent to the form `_shared-context.md` labels **Forbidden** and which BC-6.26.001 PC1 governs — AND assigned that write to the `adversary`, whose profile denies Write/Edit/Bash, making it unexecutable. Same file canonicalized its READ path at L86, so reads were swept and the commissioned write was not.

**References:** BC-6.26.001 PC1; TD-VSDD-060

---

### F-S2104-P25-M01 | MEDIUM | `_assert_g1_ref`

**Summary:** Mandate gate and ordering gate validated DIFFERENT lines (ordering used `head -1` of the path; mandate accepted any later line), so a mandate placed AFTER `git worktree remove` passed both — the trailing-footnote escape F-S2104-P22-009 existed to close.

**References:** F-S2104-P22-009

---

### F-S2104-P25-M02 | MEDIUM | `_assert_no_inline_find_antipattern`

**Summary:** Line-based predicate; both `.lobster` surfaces use folded YAML scalars, so a `find … .factory/` / `-type f` split across folded lines evades while parsing to a live inline find.

**References:** AC-007(d)

---

### F-S2104-P25-M03 | MEDIUM | ADR-031 §Decision 1

**Summary:** Lead-in said "**five** cross-cutting invariants"; six enumerated (INV-E21-001..006). INV-E21-006 was appended at v1.1 and the aggregation cell never swept — identical class to the §Decision 7 Four→Five fix at v1.2.

**References:** POLICY 4

---

### F-S2104-P25-M04 | MEDIUM | red-gate-log ×6 sites

**Summary:** Cited `5ccf5669` where the verified HEAD was `5ccf5869`. **Orchestrator-verified: the correct SHA appeared ZERO times and the wrong one 6 times**, including the frontmatter and the "24/24 at \<SHA\>" attestation — so the evidence was bound to a nonexistent commit. The orchestrator supplied the correct SHA in the dispatch; a digit was transposed.

**References:** SHA-drift; POLICY 15; POLICY 22

---

### F-S2104-P25-M05 | MEDIUM | story ×2

**Summary:** "bats test 9" for T-007 stale — the probe insertion made T-007 test **10**. The prior burst updated the inventory 9→10 but not the two ordinal pins the insertion invalidated.

**References:** TD-VSDD-060; TD-VSDD-091

---

### F-S2104-P25-M06 | MEDIUM | bats comments

**Summary:** NEW/incorrect volatile pins introduced by the very burst that swept twelve of them: `L867` cited twice for `spec_path_prose` (actual L878); `_shared-context.md L112/L113` for the `**Forbidden:**` bullets (actual L115/L116); and the whitelist rationale "the leading `- ` breaks the line-anchor match" became false once list-marker stripping landed.

**References:** TD-VSDD-091

---

### F-S2104-P25-M07 | MEDIUM | BC-6.26.001 changelog

**Summary:** Unresolved `D-{TBD-pass-23-fix-burst}` sentinel shipped in a spec artifact — **D-936 was supposed to replace it and missed it.**

**References:** POLICY 16; CLAUDE.md Rule 6

---

### F-S2104-P25-M08 | MEDIUM | bats T-003 nesting guard

**Summary:** Asserted only the test's own `mv` destination; no doc/production artifact consulted, so it could not fail unless `mv` failed — yet was credited with covering the BC §Description nesting warning.

**References:** POLICY 11

---

### F-S2104-P25-L01 | LOW | BC-6.26.001 changelog

**Summary:** Row `1.0` misplaced between `1.4` and `1.3`.

**References:** POLICY 1

---

### F-S2104-P25-L02 | LOW | `devops-engineer.md` §Worktree Cleanup

**Summary:** Cited "ADR-031's caller-side primary ruling" with no `§Decision N` anchor.

**References:** POLICY 19

---

## Deferred Finding (cross-story → E-21 W2 wave gate)

ADR-031 §Consequences 5 records that **BC-5.44.001 v1.3** and **S-21.02 v1.1** cite `"ADR-031 v1.1 §Consequences #5"` — a load-bearing ADR version pin prohibited by POLICY 19 / TD-VSDD-091. Both artifacts are outside the S-21.04 perimeter. Architect appended a tracking note to ADR-031 §Consequences #5 and judged this a wave-gate item (one-line cite correction in each, below story-creation threshold); if the wave gate does not sweep it, the story-ID convention would be `S-21.WG2-001`. Anchor: `S-21.WG2-001` (wave-gate → E-21 W2).

## Fix Mapping Summary

| Finding | Closed by | At HEAD |
|---------|-----------|---------|
| B01 | test-writer (fail-closed rule-bound helpers + corpus regression test T-015) | `e1ff2553` |
| B02 | test-writer (perl neutralizer removed; fail-closed whitelist) | `e1ff2553` |
| B03 | test-writer (unified sed with `+`-quantified alternation; `^` removed; mixed-marker probes) | `e1ff2553` |
| H01 | state-manager (POLICY 15 evidence persisted; Summary advanced; pass-25 attestation) | this burst |
| H02 | test-writer (Leg E broadened to `_prose[a-zA-Z0-9_]*="?\$\(` with `_nosplit`/`leg_e_` exclusions) | `e1ff2553` |
| H03 (bats) | test-writer (`F-S2104-P25-001` removed ×3; re-anchored to `call-site-parity`) | `e1ff2553` |
| H03 (story) | story-writer (3 occurrences re-anchored; zero remain in body) | `e1ff2553` |
| H04 | devops-engineer (all 5 `.factory/cycles/...` refs canonicalized; adversary write → state-manager via orchestrator; step-c lower-class path noted; step-a/b/e/f clean; step-d read-only; step-g correct-by-design) | `4dc27251` |
| M01 | test-writer (ordering gate bound to `g1_mandated_lineno`) | `e1ff2553` |
| M02 | test-writer (collapsed-content check for folded YAML) | `e1ff2553` |
| M03 | architect (ADR-031 v1.13→v1.14: lead-in corrected "five"→"six"; count sweep verified 8 other count lead-ins CORRECT) | `e1ff2553` |
| M04 | state-manager (correct `5ccf5669`→`5ccf5869` at all 6 sites; verified grep count=0 wrong SHA) | this burst |
| M05 | story-writer (ordinal cites ELIMINATED; stable T-NNN name-based cites; literal counts verified: main 10, preflight 15, total 25) | `e1ff2553` |
| M06 | test-writer (volatile pins replaced with variable-name behavioral anchors) | `e1ff2553` |
| M07 | state-manager (sentinel replacement: `D-{TBD-pass-23-fix-burst}`→D-936, `D-{TBD-pass-25-fix-burst}`→D-937; grep proof) | this burst |
| M08 | test-writer (T-003 now asserts DOC-PARITY on `Option A:.*[Rr]elocat` before filesystem assertions) | `e1ff2553` |
| L01 | product-owner (BC-6.26.001 v1.13→v1.14: changelog row `1.0` moved to bottom; full-table sweep clean) | `e1ff2553` |
| L02 | architect (ADR-031 v1.13→v1.14: `§Decision 4` anchor added) | `e1ff2553` |
