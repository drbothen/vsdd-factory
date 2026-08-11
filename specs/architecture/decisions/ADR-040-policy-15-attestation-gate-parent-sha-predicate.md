---
document_type: architecture-decision-record
level: L3
adr_id: ADR-040
version: "1.12"
title: "ADR-040: POLICY 15 ATTESTATION-LOCATION GATE — parent-SHA predicate replaces self-referential HEAD-SHA (resolves F-S2107-P8-003 logical impossibility)"
status: active
ratified: 2026-08-10
ratification_note: "D-965 ratification was PROCURED-ON-MISCHARACTERIZATION and remains historically invalid; v1.12 is ratified on its own merits with mutation-verified evidence per D-970. D-965 erratum preserved in §Status."
date: 2026-08-07
producer: architect
timestamp: 2026-08-07T00:00:00Z
deciders:
  - architect
  - human (ratification required — see §Status)
subsystems_affected: [SS-05]
supersedes: "POLICY 15 ATTESTATION-LOCATION GATE clause codified at D-912 (D-912's POLICY 13 mutant-derived-gate alternation mandate is unaffected and unchanged)"
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
last_amended: |-
  2026-08-10 (v1.12) — AMENDED (architect): corrects two factual errors in the v1.11
  seventh-generation record — (1) wrong test credited: generation 7 was found by mutation B
  (neutralising `run_gate` guard 1), surviving test was `test_unresolvable_base_fails_closed`
  (`!outcome.is_pass()` assertion), not `test_positive_2_no_attestation_heading`; (2) wrong
  timing: generation 7 was found after the crate landed at `d5a90e74`, not before. Full
  accurate account recorded in §Consequence "Execution discipline" and §Consequence
  "Mutation-testing acceptance criterion". §Consequence "Mutation-testing acceptance criterion"
  rewritten with the generation 7 case study, the coarse-assertion-gap vs mutant-killer
  distinction, the generation 5/7 altitude symmetry, and variant-specific `matches!()` example.
  §Status v1.12 added.
  [Prior: 2026-08-10 (v1.11) — AMENDED (architect): §Decisions 9 and 10 restructured — embedded bash
  scripts removed; mechanism delegated to Rust crate `crates/policy15-attestation-gate/`
  (landing commit `d2a3176a`, branch `feature/policy15-gate-rust`). §Decision 9 Ruling 9(a)
  replaced with reference to `run_gate` function and `GateOutcome` enum as normative
  representation; CI invocation documented. Guard ordering pinned by
  `test_run_gate_guard1_stale_pin_beats_unresolvable_base`. §Decision 10 heading updated to
  v1.11; Ruling 10(b) replaced with 16 Rust `#[test]` names; Ruling 10(d) inline bash script
  removed. §Rationale "Why six controls (v1.6)" heading corrected to "Why seven controls
  (v1.9)". §Consequence "Execution discipline" updated: seventh generation + note that Rust
  migration closes the prose-ADR defect class. New §Consequence "Mutation-testing acceptance
  criterion" added. §Proposed `policies.yaml` Replacement Text version chain updated to v1.11.
  §Status v1.11 added. Flag: `lib.rs` module doc cites `v1.8`; noted in Ruling 9(a) as cosmetic lag.
  [Prior: 2026-08-10 (v1.10) — AMENDED (architect): R3 seed-before-BASE3 structural fix + §Consequence
  note — R3 `BASE3` was captured before the seed commit, so the evaluated range contained the
  placeholder file inside `${PLUGIN_CRATE}/`; while the gate logic still returned
  PASS-zero-activations (placeholder.md is not `*.rs/*.bats`), the intent of the fixture is
  that the range contains only docs-outside-crate changes. Fixed: seed commit made first
  (`docs/.gitkeep` inside `${PLUGIN_CRATE}/`), `BASE3` captured after seed. R4 comment and
  seed file updated to match R3 convention (`src/.gitkeep`, "seed crate path into HEAD
  tree"). §Consequence added: "Execution discipline — scripts in ADR prose are not
  auto-tested" — documents the sixth-generation pattern and states the long-term fix (scripts
  migrate to real CI files). §Status v1.10 added.
  [Prior: 2026-08-10 (v1.9) — AMENDED (architect): Defect 4 + Defect 5 — (4) R3 and R4 fixtures used
  `mkdir -p` to pass the stale-pin guard, but `git cat-file -e "HEAD:${PLUGIN_CRATE}"`
  requires the crate to appear in the commit tree (empty dirs are not tracked by git). R3
  fix: commit `placeholder.md` inside `${PLUGIN_CRATE}/docs/` so the crate enters HEAD tree.
  R4 fix: commit placeholder first, then capture HEAD4 (so MERGE_BASE==HEAD4 produces a
  genuinely empty range, not a stale-pin false trigger). (5) Controls asserted exit codes
  only; FAIL=2 and EMPTY-or-UNREACHABLE=2 share an exit code, so a control could pass while
  the wrong non-zero path was exercised (R4 hit stale-pin while claiming empty-range). Fixed:
  all controls now capture stdout and grep for the specific outcome identifier. Third
  EMPTY-or-UNREACHABLE control added (empty-diff: --allow-empty commit → EMPTY-or-UNREACHABLE:
  unmeasurable diff). §Decision 10 Requirement 4 added to project-wide rule. Ruling 10(b)
  updated to seven controls with required-assertion column. Ruling 10(c/d): seven fixtures,
  all with outcome-string assertions. §Consequences updated: seven controls. §Decision 9,
  §Decision 10, `_run_gate` script comments updated from v1.6 to v1.9. §Status v1.9 added.
  [Prior: 2026-08-10 (v1.8) — AMENDED (architect): evidence-block reproducibility fix — §Decision 9
  literal-shell evidence block used `|| echo 0`, which is a third occurrence of the same
  double-output defect fixed in v1.7. The stray `0` lines were hand-tidied before commit,
  violating POLICY 15 verbatim-stdout and POLICY 5 HEAD-reproducibility. Fixed: `|| echo 0`
  replaced with `|| true`; captured output pasted verbatim (output unchanged — hand-tidied
  lines happened to match the correct output). §Rationale note added: "Three `|| echo 0`
  occurrences". §Decision 9 heading updated to AMENDED (v1.8). §Status v1.8 added.
  [Prior: 2026-08-10 (v1.7) — AMENDED (architect): fix three defects found by executing the v1.6
  controls — (1) `|| echo 0` double-output: `grep -cE` exits 1 on no-match and prints `0`;
  `|| echo 0` appended a second `0`; the two-line variable caused `[ "$VAL" -ne 1 ]` to
  exit 2 (invalid integer), bash took else branch, gate silently reported PASS on missing
  attestation headings. Fixed: `|| echo 0` → `|| true` at all four CHANGED/COUNT sites in
  §Decision 9 script and `_run_gate`. (2) `git add --quiet` invalid: not a valid git flag;
  control harness aborted. Fixed: `--quiet` removed from all five `git add .` calls.
  (3) Stale-pin guard filesystem check: `[ ! -d ... ]` tests working directory, not the git
  tree. Fixed: replaced with `git cat-file -e "HEAD:${PLUGIN_CRATE}"` (production) and
  `git -C "$REPO" cat-file -e "HEAD:${PLUGIN_CRATE}"` (`_run_gate`). Ruling 10(e) added:
  empirical execution record documenting that the controls caught defect 1 on first real
  execution, and that Positive Control 2 had the same bug. §Status v1.7 added.
  [Prior: 2026-08-10 (v1.6) — AMENDED (architect): stale-pin guard + outcome renaming —
  §Decision 8: INAPPLICABLE → PASS-zero-activations (named, observable, activation count
  always emitted); ERROR → EMPTY-or-UNREACHABLE (gains stale-pin as third trigger: crate
  dir absent at HEAD). §Decision 9 script: stale-pin guard before COMMITS check; terminal
  section uses PASS-zero-activations / PASS-N-activations / EMPTY-or-UNREACHABLE. §Decision
  10: project-wide rule three requirements updated (four-outcome with PASS-zero/EMPTY-or-
  UNREACHABLE; every-outcome-has-a-control generalization stated; stale-pin guard
  requirement); six controls: stale-pin control added; `_run_gate` updated with stale-pin
  guard + new output labels; Ruling 10(b) table updated; script updated; Ruling 10(c/d)
  updated. §Rationale: Why six controls. §Consequences: five→six. §Proposed policies.yaml
  v1.4.23 text updated. §Status v1.6 added.
  [Prior: 2026-08-10 (v1.5) — AMENDED (architect): INAPPLICABLE/ERROR split — §Decision 8
  four-outcome table (FAIL/PASS/INAPPLICABLE/ERROR); INAPPLICABLE requires confirmed non-empty
  diffs per commit; ERROR on empty range OR empty per-commit diff. §Decision 9 script
  rewritten — ALL_CHANGED per-commit emptiness check (trigger-2 ERROR); ERROR counter;
  named INAPPLICABLE exit path with INAPPLICABLE output; TOTAL_COMMITS computed for
  INAPPLICABLE message. §Decision 7 Ruling 7(a) trigger updated to reference INAPPLICABLE
  and ERROR. §Decision 10: project-wide rule updated to four-outcome model; `_run_gate`
  rewritten with ALL_CHANGED check and four-outcome output lines (FAIL/ERROR/INAPPLICABLE/
  PASS); five controls: inapplicable-control (docs-only fixture, output verified
  INAPPLICABLE) added; error-control renamed from positive-3; controls table updated;
  project-wide-rule explanation updated (INAPPLICABLE/ERROR split; INAPPLICABLE-reachability
  control rationale). §Rationale: Why five controls section updated — INAPPLICABLE-
  reachability rationale, F-001 three-redesign lesson. §Consequences positive updated
  (four→five controls). §Proposed policies.yaml v1.4.23 text updated — four-outcome verdict,
  ALL_CHANGED check, five controls. §Status v1.5 added.
  [Prior: 2026-08-10 (v1.4) — AMENDED (architect): §Decision 8 redesigned — unconditional
  obligation + path-pinned (replaces per-crate dynamic derivation); INAPPLICABLE branch
  RETIRED (was vehicle for F-003 antecedent-failure; unconditional obligation eliminates
  antecedent entirely); EMPTY-or-UNREACHABLE added as third named outcome (exits non-zero;
  zero-commit range is CI setup defect). §Decision 9 script updated — PLUGIN_CRATE +
  RED_GATE_LOG constants replace per-crate loop; COMMITS emptiness check (EMPTY-or-
  UNREACHABLE) added before iteration; grep-cE scoped to pinned crate. Ruling 9(c) point 1
  updated — INAPPLICABLE → EMPTY-or-UNREACHABLE. §Decision 10 redesigned as PROJECT-WIDE
  three-outcome gate rule (not POLICY 15-specific; first application to this gate); four
  controls now required: positive-1 (absent-log→FAIL), positive-2 (no-attestation→FAIL),
  positive-3 (empty git range→non-zero), negative (compliant→PASS); _run_gate helper
  accepts MERGE_BASE parameter; fixtures now use pinned PLUGIN_CRATE path. §Rationale new
  and updated subsections: unconditional obligation eliminates antecedent failure (in-toto
  REQUIRE-rule conceptual precedent); vacuity-not-tautology updated — Beer et al. CAV 1997
  added; two-failure decomposition (premise-sound/implementation-vacuous) added; SLSA
  rationale updated to reference two-failure section; Why path-pinned over per-crate-derived
  added; EICAR section updated — four controls, v1.4 framing; §Consequences bullet updated
  (three→four controls). Policies.yaml v1.4.23 proposed text updated — unconditional +
  path-pinned + EMPTY-or-UNREACHABLE + four controls + project-wide gate rule.
  [Prior: 2026-08-10 (v1.3) — AMENDED (architect): Gap A fixed — §Decision 8 replaced
  hardcoded path (absent on origin/develop) + SKIP-when-absent with per-crate derived path
  via grep -oE 'crates/hook-plugins/[^/]+' + FAIL-when-absent; three-outcome verdict added
  (FAIL/PASS-N-activations/INAPPLICABLE); scope narrowed to hook-plugin crates only.
  §Decision 9 script updated — per-crate HOOK_CRATES loop; three-outcome exit codes.
  Ruling 9(c) expanded — unconditional job; merge-base; parentless-commit. Gap B fixed —
  §Decision 10 EICAR-style synthetic fixtures; three controls. §Rationale new subsections:
  unconditional-job; vacuity-not-tautology (Kupferman & Vardi CHARME 1999); SLSA v1.2;
  EICAR rationale.
  [Prior: 2026-08-10 (v1.2) — REOPENED + AMENDED (architect): §Decision 6 erratum —
  5370db80 was not a benign stability entry but the attestation-provision commit itself;
  INAPPLICABLE branch introduced by v1.1 was exempting precisely the attestation-provision
  commit class; D-965 ratification PROCURED-ON-MISCHARACTERIZATION (F-S2107-P10-003).
  §Decision 7 added — execution-context correction: gate was running in factory-artifacts
  worktree (0 *.rs/*.bats files of 3743 total — permanently INAPPLICABLE; root cause of
  F-S2107-P10-001 BLOCKER); gate relocated to code-repo CI (required-check GitHub Actions
  step). §Decision 8 added — deterministic log path replaces find|head-1 over 14
  factory-artifacts candidates; S-21.07 log lives in code repo at
  crates/hook-plugins/validate-cross-site-correspondence/docs/. §Decision 9 added —
  per-commit iteration over PR branch history (not push-tip only) closes retroactive-
  attestation window (F-S2107-P10-002 BLOCKER pattern). §Decision 10 added — mandatory
  negative-control CI job using 67ffbdcc fixture (non-vacuity proof gate). Policies
  replacement text updated to v1.4.23. ADR re-opened to PROPOSED; human re-ratification
  required before policies.yaml is edited. POLICY 21 compliance: gate is inline YAML
  run: step, not a new .sh file.
  [Prior: 2026-08-08 (v1.1) — AMENDED (architect): §Decision 6 added — detection-scope
  correction (obligation was commit-class-conditional; detection clause was unconditional —
  contradiction; resolved by propagating applicability condition into detection) + self-match
  prevention (line-anchored predicate ^### .*) + optional stability-record heading form;
  §Proposed policies.yaml Replacement Text updated to v1.4.22. Triggered by F-S2107-P9-003
  close revealing the scope mismatch in practice. [Prior: 2026-08-07 (v1.0) — Initial ruling
  (architect; F-S2107-P8-003 impossibility diagnosis; S-21.07 pass-7 fix wave design):
  self-referential SHA predicate in POLICY 15 ATTESTATION-LOCATION GATE (D-912) is logically
  unsatisfiable; parent-SHA predicate preserves all three of D-912's original goals while
  removing the impossibility. policies.yaml NOT yet edited — human ratification required per
  §Status. ADR-040 PROPOSED 2026-08-07.]]
modified:
  - "2026-08-07 (v1.0)"
  - "2026-08-08 (v1.1)"
  - "2026-08-10 (v1.2)"
  - "2026-08-10 (v1.3)"
  - "2026-08-10 (v1.4)"
  - "2026-08-10 (v1.5)"
  - "2026-08-10 (v1.6)"
  - "2026-08-10 (v1.7)"
  - "2026-08-10 (v1.8)"
  - "2026-08-10 (v1.9)"
  - "2026-08-10 (v1.10)"
  - "2026-08-10 (v1.11)"
  - "2026-08-10 (v1.12)"
---

# ADR-040: POLICY 15 ATTESTATION-LOCATION GATE — parent-SHA predicate replaces self-referential HEAD-SHA

## Context

**D-912** added the ATTESTATION-LOCATION GATE to POLICY 15's `verification_steps`. Its stated
purpose was to prevent fix waves from pushing assertion-site changes to bats tests without
simultaneously updating `red-gate-log.md` with a corresponding attestation section. The stated
mechanism was:

> Literal shell check: `grep -c 'assertion-site attestation (<HEAD-SHA>)' red-gate-log.md`
> → 1 (where `<HEAD-SHA>` is the actual SHA being pushed). The attestation section heading MUST
> be `### <Pass-N> assertion-site attestation (<HEAD-SHA>)` so the check is SHA-bound and cannot
> be satisfied by a prior pass's section.

**F-S2107-P8-003 (HIGH [process-gap])**, raised at adversarial pass-7 of S-21.07
(reviewed-head `fbb5183c`), diagnosed this predicate as logically unsatisfiable.

### The impossibility proof

A git commit's SHA is computed as:

```
SHA1(tree-SHA, parent-SHA, author, committer, timestamp, commit-message)
```

where `tree-SHA` is the Merkle root over all tracked file contents in that commit. If
`red-gate-log.md` within the commit contains the string
`assertion-site attestation (fbb5183c)`, then the tree-SHA already incorporates
`fbb5183c`. But `fbb5183c` is itself derived from that tree-SHA. This is a circular
dependency. The only way to construct a file whose content contains its own eventual
SHA is to brute-force a SHA preimage that produces the desired hexadecimal string — a
computationally infeasible operation (SHA-1 preimage resistance at ~2^160 operations).

**Empirical confirmation at `fbb5183c`:**

```
$ grep -c 'assertion-site attestation (fbb5183c)' \
    crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md
0
$ grep -oE 'assertion-site attestation \([0-9a-f]{7,40}\)' \
    crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md
assertion-site attestation (b78b27ef402f11e36c8c23f68f65d6335c37dd14)
assertion-site attestation (295585185308629f10ff4333647a15b474192c3f)
```

The project has silently adopted a "parent-SHA in the next commit" convention: `fbb5183c`
attests `29558518` (its parent), and the commit before it attests `b78b27ef`. No commit
contains its own SHA in `red-gate-log.md`, and none can. The predicate has returned 0 at
every reviewed HEAD since D-912.

### The TD-VSDD-053 collision

D-912's gate also states: "the push is BLOCKED until the state-manager appends the
attestation section and bundles it **in the same commit** per TD-VSDD-053."

For a commit H to contain `assertion-site attestation (H)` in the same commit, H must
contain its own SHA — the impossibility above. But if the attestation is placed in the
_following_ commit (the current unofficial convention), that violates the "same commit"
half of the D-912 gate. The two halves of the gate are mutually exclusive. The gate
therefore cannot be satisfied on either half in isolation or both halves together.

### D-912's three original goals (from the policy text)

D-912 was designed to achieve three properties:

1. **SHA-bound**: The attestation must reference a specific SHA so it cannot be satisfied
   by an arbitrary prior pass's section.
2. **Location-correct**: The attestation must be in `red-gate-log.md`, not in a bats file
   or elsewhere (the root cause D-912 was addressing).
3. **Same-commit bundling**: The attestation must land in the same commit as the
   assertion-site changes, per TD-VSDD-053, to prevent split-commit drift.

Goal 2 and Goal 3 are achievable. Goal 1, in the form "SHA of the commit that contains
this attestation", is not achievable. This ADR preserves all three goals by replacing Goal
1's SHA reference with the parent SHA.

---

## Decision

This ADR makes six rulings covering the predicate replacement, heading format, TD-VSDD-053
interaction, mechanization verdict, sibling-sweep finding, and (v1.1) detection-scope
correction plus self-match prevention.

---

## Decisions

### Decision 1 — The D-912 HEAD-SHA predicate is logically unsatisfiable and is replaced

The ATTESTATION-LOCATION GATE clause in POLICY 15 (codified at D-912) is declared
structurally defective. It has returned 0 at every HEAD since D-912 and will continue to
return 0 at every future HEAD because the predicate is unsatisfiable by construction.

The clause is superseded in its entirety by ADR-040 §Decisions 2–5. D-912's POLICY 13
extension (mutant-derived-gate alternation mandate) is unaffected and unchanged.

### Decision 2 — Replacement predicate and heading format

**New attestation heading format** (required in `red-gate-log.md`):

```
### Pass-N assertion-site attestation (<PARENT-SHA>)
```

where:
- `N` is the adversarial pass number that drove the current fix wave (matches the
  `pass:` frontmatter of the adversarial pass file driving this burst).
- `<PARENT-SHA>` is the full 40-character SHA of the parent commit of the commit being
  pushed — i.e., `git rev-parse HEAD^1` evaluated in the factory-artifacts worktree at
  the moment the push is prepared.

**Replacement literal shell check** (to be invoked at Commit-E push time per D-449(a)):

```bash
PARENT=$(git -C <factory-worktree-root> rev-parse HEAD^1)
grep -c "assertion-site attestation ($PARENT)" <red-gate-log-path>
# expected stdout: 1
```

The count MUST be exactly 1. If 0, the push is BLOCKED until the state-manager appends
the attestation section (same-commit bundling per Decision 3).

**Why parent-SHA preserves Goal 1 (SHA-bound):**

The parent SHA of the current fix burst's commit (`HEAD^1`) is the SHA of the prior
factory-artifacts commit. Git history is append-only and SHA collisions are
cryptographically negligible; each fix burst's parent is a distinct, unique value. A prior
pass's attestation section embeds _that burst's_ parent SHA, which differs from the
current burst's parent SHA. Therefore:

- Pass-N's attestation section contains `assertion-site attestation (P_N)` where `P_N =
  git rev-parse (Pass-N commit)^1`.
- Pass-(N+1)'s predicate checks for `assertion-site attestation (P_{N+1})` where
  `P_{N+1} ≠ P_N` (different parent).
- Pass-N's section cannot satisfy Pass-(N+1)'s predicate.

**Role of the Pass-N ordinal:** The ordinal is a human-readable navigation label. It is
NOT the primary discriminator — the parent SHA is. If two passes were hypothetically to
share a parent (impossible in the normal append-only model), the ordinal would be the
tiebreaker. In practice, the parent SHA is sufficient and the ordinal adds no load-bearing
cryptographic strength.

**Detection rule for adversary:** For the reviewed-head SHA H, compute the parent
`P = git rev-parse H^1` (or equivalently the parent SHA cited in `git cat-file commit H`).
Verify that `red-gate-log.md` at H contains `### Pass-N assertion-site attestation (P)`
(matching on full or any unambiguous prefix of P). If absent or if only `P_prior ≠ P` is
present, flag as POLICY 15 HIGH (attestation-location violation).

### Decision 3 — TD-VSDD-053 interaction: same-commit bundling is retained unchanged

The "same-commit bundling" requirement from D-912 is preserved verbatim. In a single
factory-artifacts commit, the state-manager writes:

1. Any bats assertion-site additions or strengthenings.
2. The `### Pass-N assertion-site attestation (<PARENT-SHA>)` section in
   `red-gate-log.md`.

This is one commit. TD-VSDD-053 single-commit-per-burst is satisfied.

**Why this is now achievable:** The parent SHA (`HEAD^1`) is known _before_ the commit is
finalized. The state-manager knows the current HEAD of the factory-artifacts worktree before
staging any files; that HEAD becomes `HEAD^1` of the commit being prepared. There is no
circular dependency:
- Current factory-artifacts HEAD = P (the parent SHA to embed)
- State-manager stages files including `assertion-site attestation (P)` in `red-gate-log.md`
- State-manager commits: the new commit H has SHA = hash(tree_with_attestation(P), P, ...)
- At push time: `git rev-parse H^1` = P, grep finds the attestation → count = 1. ✓

The D-912 convention of "parent-SHA in the follow-up commit" (unofficially adopted by prior
bursts) is retired. The attestation belongs in the same commit as the assertion-site changes.

### Decision 4 — Mechanization verdict: the predicate is fully mechanizable

The replacement predicate can be encoded as a PreToolUse push-hook or CI step with no
narrative or reconstruction required:

```bash
#!/usr/bin/env bash
# POLICY 15 ATTESTATION-LOCATION GATE (ADR-040 §Decision 2)
set -euo pipefail
FACTORY_ROOT="${1:-.factory}"
RED_GATE_LOG=$(find "$FACTORY_ROOT" -name "red-gate-log.md" | head -1)
if [ -z "$RED_GATE_LOG" ]; then
  echo "SKIP: no red-gate-log.md found — non-S-21.07 push"
  exit 0
fi
PARENT=$(git -C "$FACTORY_ROOT" rev-parse HEAD^1)
COUNT=$(grep -c "assertion-site attestation ($PARENT)" "$RED_GATE_LOG" || true)
if [ "$COUNT" -ne 1 ]; then
  echo "FAIL: POLICY 15 ATTESTATION-LOCATION GATE"
  echo "  expected: grep count = 1 for 'assertion-site attestation ($PARENT)' in $RED_GATE_LOG"
  echo "  actual:   $COUNT"
  exit 2
fi
echo "PASS: attestation section for parent $PARENT found (count=$COUNT)"
exit 0
```

Per D-449(a) and META-LEVEL-24, the gate MUST be invoked as a literal shell command at
Commit-E push time with captured stdout recorded in the burst-log Dim-2 evidence section.
Pseudocode or narrative attestation (`"grep would return 1"`) is forbidden.

### Decision 5 — Sibling-sweep: F-S2107-P8-003 is an instance, not a class

The adversary brief requested a sibling-sweep of `policies.yaml` for verification_steps
predicates that embed a commit's own SHA or otherwise reference state unavailable at
authoring time.

**Literal shell sweep (executed at factory-artifacts `10914a73`):**

```
$ grep -oE "HEAD-SHA|actual SHA being pushed|SHA being committed|grep.*HEAD[^:]" \
    .factory/policies.yaml
assertion-site attestation (<HEAD-SHA>)
actual SHA being pushed
```

Both matches are in the same POLICY 15 ATTESTATION-LOCATION GATE bullet (line 294 in the
policies.yaml at `10914a73`). No other verification_steps predicate embeds a commit's own
SHA or references state that cannot be known at authoring time.

**Conclusion:** F-S2107-P8-003 is an instance defect, not a class defect. The
ATTESTATION-LOCATION GATE is the only predicate in the registry that contains this defect.
No further sibling sweep is required in this burst.

### Decision 6 — AMENDED (v1.1): detection-scope correction and self-match prevention

Two defects in the §Decision 2 detection procedure, both surfaced during F-S2107-P9-003 close:

**Defect 1 — Scope mismatch (obligation vs detection).** The obligation is commit-class-conditional:
"a fix wave that **adds or strengthens any bats assertion site** MUST NOT be pushed until..."
A docs-only or state-bookkeeping commit is exempted by the obligation. The §Decision 2 detection
clause is unconditional: "adversary verifies that... a matching section... exists in
red-gate-log.md; if absent, flag as POLICY 15 HIGH." The detection fires even when the
obligation does not apply. This produced a false positive: test-writer commit `5370db80`
(a red-gate-log.md stability entry with no assertion-site files changed) was flagged, because
the detection clause checked for an attestation heading that the obligation never required.

> **ERRATUM (v1.2 — F-S2107-P10-003):** The characterisation of `5370db80` as "a
> red-gate-log.md stability entry" is technically accurate (no `*.rs`/`*.bats` files were
> changed) but materially misleading. `5370db80` was not a benign bookkeeping commit —
> it was the attestation-provision commit itself: `docs(S-21.07): add POLICY 15 attestation
> sections for pass-8, pass-9, and docs commits (F-S2107-P9-003)`. It changed exactly one
> file: `red-gate-log.md` (literal shell: `git diff --name-only 5370db80^1..5370db80` →
> `crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md`). The
> INAPPLICABLE branch introduced by Ruling 6(a) exempts commits that change only `red-gate-log.md`
> but not `*.rs`/`*.bats` files. This is precisely the attestation-provision commit class.
> Consequence: the gate exempts the exact commits that provide attestations, leaving the
> "at that commit" obligation structurally unverifiable. The human ratification of D-965 was
> based on this framing and is therefore PROCURED-ON-MISCHARACTERIZATION. ADR-040 has been
> re-opened to `proposed`; re-ratification is required after §Decisions 7–10 are reviewed.
> The scope-mismatch fix in Ruling 6(a) is still correct in principle (docs-only factory-artifacts
> commits should be exempt); the error was in mischaracterising which commit was the
> "false positive" and in applying the fix in the wrong repository context. §Decision 7
> addresses the execution context. §Decision 9 addresses the per-commit enforcement gap.

**Defect 2 — Self-match vulnerability.** The §Decision 4 predicate
`grep -c "assertion-site attestation ($PARENT)"` is not line-anchored. If a prior attestation
heading string is quoted inside a prose paragraph in `red-gate-log.md` (e.g., in a changelog
or "see-also" reference), that prose line produces a second grep hit, yielding count = 2. The
count ≠ 1 check then incorrectly blocks a valid push. Observed during F-S2107-P9-003: quoting
a prior attestation heading in an inline reference caused count = 2 at a HEAD whose attestation
was correctly present.

#### Ruling 6(a) — Propagate the applicability condition into detection

The gate (both the §Decision 4 mechanized hook and the adversary manual check) MUST first
determine whether the reviewed commit touched assertion-site files. Literal shell:

```bash
git diff --name-only "${H}^1" "${H}" -- '*.rs' '*.bats' 2>/dev/null
```

- **Empty output:** commit did not touch assertion-site files → gate is INAPPLICABLE →
  no POLICY 15 flag is raised. Adversary records:
  `POLICY 15: no assertion-site files changed at <H> — ATTESTATION-LOCATION GATE inapplicable`
- **Non-empty output:** at least one *.rs or *.bats file changed → proceed to the attestation
  check.

#### Ruling 6(b) — Line-anchor the grep predicate

Replace the §Decision 2 literal shell check:

```bash
# Before (§Decision 2 / §Decision 4):
grep -c "assertion-site attestation ($PARENT)"

# After (§Decision 6):
grep -cE "^### .*assertion-site attestation \($PARENT\)"
```

The `^### ` anchor restricts matches to lines beginning with `### ` (markdown H3 heading
syntax). Prose inside section bodies — paragraphs, code spans, inline references — does not
begin with `^### ` and cannot trigger a false count. This closes the count = 2 self-match
defect without altering the gate's semantics for correctly authored attestation headings.

**Full replacement literal shell gate** (supersedes §Decision 4 script; incorporates Rulings
6(a) and 6(b)):

```bash
#!/usr/bin/env bash
# POLICY 15 ATTESTATION-LOCATION GATE — ADR-040 §Decision 6 (conditional + line-anchored)
set -euo pipefail
FACTORY_ROOT="${1:-.factory}"
RED_GATE_LOG=$(find "$FACTORY_ROOT" -name "red-gate-log.md" | head -1)
if [ -z "$RED_GATE_LOG" ]; then
  echo "SKIP: no red-gate-log.md found — non-S-21.07 push"
  exit 0
fi

H=$(git -C "$FACTORY_ROOT" rev-parse HEAD)
PARENT=$(git -C "$FACTORY_ROOT" rev-parse HEAD^1)

# Step 1: conditional — skip if no assertion-site files changed
CHANGED_FILES=$(git -C "$FACTORY_ROOT" diff --name-only "${PARENT}" "${H}" \
  -- '*.rs' '*.bats' 2>/dev/null | wc -l | tr -d ' ')
if [ "$CHANGED_FILES" -eq 0 ]; then
  printf 'PASS: POLICY 15 — no assertion-site files changed, gate inapplicable\n'
  exit 0
fi

# Step 2: verify attestation section (line-anchored predicate)
COUNT=$(grep -cE "^### .*assertion-site attestation \\(${PARENT}\\)" \
  "$RED_GATE_LOG" 2>/dev/null || echo 0)
if [ "$COUNT" -ne 1 ]; then
  printf 'FAIL: POLICY 15 HIGH — assertion-site files changed (%s files)\n' "$CHANGED_FILES"
  printf '  expected: grep -cE "^### .*attestation (%s...)" = 1\n' "${PARENT:0:12}"
  printf '  actual:   %s\n' "$COUNT"
  exit 2
fi
printf 'PASS: attestation for parent %s found (count=%s, assertion-site files changed: %s)\n' \
  "${PARENT:0:12}" "$COUNT" "$CHANGED_FILES"
exit 0
```

Per D-449(a) / META-LEVEL-24, this gate MUST be invoked as literal shell with captured stdout
at Commit-E. Pseudocode or narrative attestation is forbidden.

#### Ruling 6(c) — Optional stability-record heading form

Authors MAY write a voluntary stability record in `red-gate-log.md` for commits where no
assertion-site files were changed:

```
### Pass-N assertion-site stability-record (<PARENT-SHA>)
```

This form:
- Does NOT contain the word "attestation" and is NOT matched by `^### .*attestation \(P\)`
- Does NOT satisfy the ATTESTATION-LOCATION GATE (gate is inapplicable for no-change commits
  under Ruling 6(a) anyway)
- MUST NOT be written in place of a real attestation on commits that DO touch assertion-site
  files — count = 0 → gate fires correctly → push blocked
- Purpose: voluntary drift tracking only; records that the assertion name-set was inspected
  and confirmed stable at this commit without changes; informational, aids post-hoc audit

**POLICY 13 BOUNDARY-POLARITY — excluded region analysis for narrowed detection scope:**

Decision 6(a) narrows the gate's domain from "every commit" to "commits touching *.rs or
*.bats files." POLICY 13 BOUNDARY-POLARITY requires excluded-region analysis:

| Dimension | Analysis |
|-----------|----------|
| **False-positive class** | Any commit whose diff touches only non-*.rs / non-*.bats files: markdown docs, TOML, YAML, shell scripts, state-manager artifacts (*.md), `.factory/` bookkeeping. These were always outside the obligation's scope; the unconditional detection clause was a false-positive generator for them. |
| **Can harmful content occupy excluded region?** | No. Bats assertion sites are authored in *.bats files; Rust assertion sites are in *.rs files. Any commit that introduces or strengthens an assertion site must modify at least one *.bats or *.rs file, producing non-empty output from the diff check and falling through to the full attestation verification. An assertion-site change cannot be concealed in the excluded file types. |
| **Mutant** | A commit that adds `assert_eq!(result, expected_name_set)` inside a `#[test]` block in a `*.rs` file: `git diff --name-only H^1 H -- '*.rs' '*.bats'` produces non-empty output → gate active → if no `### Pass-N assertion-site attestation (<PARENT>)` heading present → count = 0 → FAIL: POLICY 15 HIGH. Narrowed scope does not exempt this mutant. |

**Why Option A (conditional detection) over Option B (unconditional + distinct heading forms):**

Option B was considered: keep detection unconditional; require every commit to include either
an `attestation` entry or a `stability-record` entry; update the predicate to match both
heading forms. Rejected for three reasons:

1. **Semantic conflation.** An attestation section ("assertion sites changed and reviewed")
   and a stability-record section ("assertion sites not changed, optionally inspected") carry
   different semantic weight. A reviewer scanning the log for genuine assertion-site change
   events cannot quickly filter away stability-record entries. Log signal-to-noise degrades
   as every bookkeeping commit adds a stability-record.
2. **Predicate complexity.** Matching two heading forms requires regex alternation, adding
   edge-case mismatch risk. Option A uses a single anchored string.
3. **Obligation alignment.** The obligation was written with a conditional for good reason.
   Making every commit gate-obligated inverts the obligation's own scoping. Option A restores
   the semantic alignment between obligation and detection that D-912 intended but the
   unconditional detection clause violated.

This is a scope-precision fix, not a repeal. The gate correctly caught two genuine missing
attestations during F-S2107-P9-003 (commits that did touch *.rs / *.bats files). Those
commits are still subject to the gate under Decision 6(a). Only commits that never touched
assertion-site files are exempted — which was always the obligation's intent.

---

### Decision 7 — AMENDED (v1.2): execution-context correction — gate relocates to code-repo CI

**Root cause of F-S2107-P10-001 (BLOCKER):** §Decisions 1–6 specified gate logic that is
formally correct, but the gate was being evaluated in the factory-artifacts worktree. The
factory-artifacts orphan branch tracks only `.md`, `.yaml`, `.json`, `.toml`, and `.sh`
files — zero `*.rs` or `*.bats` files out of 3,743 total. Literal-shell evidence at HEAD
(2026-08-10):

```
$ git -C .factory ls-tree -r --name-only HEAD | grep -cE '\.(rs|bats)$'
0
$ git -C .factory ls-tree -r --name-only HEAD | wc -l
    3743
```

The §Decision 6 pre-check `git diff --name-only HEAD^1 HEAD -- '*.rs' '*.bats'` therefore
returns empty for every factory-artifacts commit, making the gate permanently INAPPLICABLE
on every commit where it could fire. The D-965 ratification of POLICY 15 v1.4.22 cemented
this vacuity into policy.

**This is a category error, not a predicate defect.** No revision of the predicate, the SHA
binding, the heading format, or the self-match prevention resolves it. The obligation governs
code-repo commits — assertion sites live in `*.rs` and `*.bats` files on branches like
`feature/S-21.07`. The evaluation context must be the code repo.

**Ruling 7(a) — Gate relocates to code-repo CI:**

The ATTESTATION-LOCATION GATE MUST run as a required-check step in the code repo's GitHub
Actions CI (`.github/workflows/`). It MUST NOT run in the factory-artifacts worktree context.

- **Context:** code-repo CI (GitHub Actions)
- **Trigger:** `on: pull_request:` (or `push:`) unconditionally — no `paths:` filter. A job
  skipped via `paths:` filter reports SUCCESS as a required check, which is itself a vacuous
  pass. The PASS-zero-activations and EMPTY-or-UNREACHABLE outcomes are handled inside the
  script (§Decision 8 Ruling 8(c)); the CI job is never conditionally skipped by the harness.
- **Repository:** code repo, working directory = repository root
- **Job name (required check):** `policy-15-attestation-location`

**Ruling 7(b) — §Decisions 1–6 carry forward in semantics:**

The parent-SHA predicate (§Decision 2), the heading format
`### Pass-N assertion-site attestation (<PARENT-SHA>)`, same-commit bundling (§Decision 3),
the mechanized script structure (§Decision 4), the sibling-sweep conclusion (§Decision 5),
and the conditional pre-check + line-anchor (§Decision 6) are all correct in semantics. The
error was exclusively in the execution context. These rulings are preserved; §Decisions 7–10
specify the correct deployment context and per-commit enforcement.

**POLICY 21 compliance:** The gate is implemented as an inline YAML `run:` block in a
GitHub Actions workflow file — not as a new `.sh` file. POLICY 21 prohibits adding `*.sh`
scripts to the repository; inline workflow `run:` steps are exempt from that prohibition.

---

### Decision 8 — AMENDED (v1.6): unconditional obligation + path-pinned + four-outcome verdict

The §Decision 4/6 gate script used `find "$FACTORY_ROOT" -name "red-gate-log.md" | head -1`,
returning one of 14 factory-artifacts candidates (non-deterministic; wrong file). In the
code-repo CI context (§Decision 7), the S-21.07 governing log lives at:

```
crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md
```

**v1.2 Gap A — Hardcoded path absent on `origin/develop`:**

ADR-040 v1.2 hardcoded this path with SKIP-when-absent behaviour:

```bash
RED_GATE_LOG="crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md"
if [ ! -f "$RED_GATE_LOG" ]; then
  echo "SKIP: red-gate-log.md not found at expected path — non-S-21.07 context"
  exit 0
fi
```

Literal-shell verification (2026-08-10):

```
$ git ls-tree -r --name-only origin/develop \
    | grep 'validate-cross-site-correspondence/docs/red-gate-log.md' | wc -l
0
```

The path does not exist on `origin/develop` (only on the unmerged `feature/S-21.07-...`
branch). SKIP-when-absent means every PR against a branch without the log exits 0 —
rebuilding the vacuous-pass condition F-S2107-P10-001 diagnosed.

**v1.3 gap — per-crate derivation + INAPPLICABLE branch:**

ADR-040 v1.3 replaced the hardcoded path with per-crate dynamic derivation and retained an
INAPPLICABLE branch for commits with no hook-plugin crate changes. This is insufficient:
INAPPLICABLE was the vehicle for F-003 — the exemption that let attestation-provision commits
slip through. Reconstituting it under a different name does not close the structural gap.
The decision to make the obligation unconditional eliminates the antecedent entirely.

**Ruling 8(a) — Path-pinned per story context; obligation is unconditional:**

The governing log path for S-21.07 is pinned as an architectural constant:

```
crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md
```

The obligation is **unconditional** within the gate's scope: every commit in the PR branch
that changes `*.rs` or `*.bats` files under `crates/hook-plugins/validate-cross-site-
correspondence/` MUST include a matching attestation section in the pinned log — with no
further sub-conditions on which functions, lines, or commit types are covered.

**Why unconditional:** An obligation with a sub-condition (`if assertion-site files changed`)
has an antecedent. An antecedent can suffer model-dependent vacuity / antecedent failure
(Beer et al. CAV 1997; Kupferman & Vardi CHARME 1999): whenever the evaluation decides the
condition is not met, the obligation trivially holds. The INAPPLICABLE class was precisely
this failure mode — commits that changed only `red-gate-log.md` were deemed outside the
obligation's scope. Under an unconditional obligation there is no antecedent to fail. The
obligation binds for every `*.rs`/`*.bats` change in the pinned crate, period.

The conceptual precedent is in-toto's REQUIRE layout rule: a named artifact MUST appear
among a step's products unconditionally (in-toto specification §4.3). We do not adopt
in-toto's framework, but the analogy is exact — `red-gate-log.md` must appear as a product
of any commit touching the crate's `*.rs`/`*.bats` files, with no sub-condition qualifying
which changes are covered.

**Why path-pinned, not per-crate-derived:** Each story's attestation log is an explicit
architectural decision recorded in this ADR and the CI step. A pinned path cannot silently
mis-resolve when multiple crates are in-flight simultaneously. When a future story adds a
new hook-plugin crate, the devops-engineer adds a new pinned CI step for that crate — an
explicit decision per story, not a runtime derivation. The standard path pattern is
`crates/hook-plugins/<X>/docs/red-gate-log.md` per crate.

**Ruling 8(b) — FAIL-when-absent; SKIP is forbidden:**

If a commit changes `*.rs`/`*.bats` under the pinned crate prefix and the pinned log path
does not exist at that commit → FAIL (exit 2).

**Why FAIL-when-absent:** A commit that introduces assertion-site files without co-creating
`red-gate-log.md` is a violation. FAIL-when-absent enforces co-creation discipline: the log
MUST be bootstrapped in the same commit that first introduces `*.rs`/`*.bats` files in the
crate. That commit's attestation heading references `git rev-parse HEAD^1` — known before
staging, no circular dependency (§Decision 3).

**Ruling 8(c) — Four-outcome verdict (defence-in-depth):**

The gate MUST produce exactly one of four outcomes:

| Outcome | Exit code | Condition |
|---------|-----------|-----------|
| `FAIL` | 2 | One or more commits violated the obligation |
| `PASS-N-activations (N ≥ 1)` | 0 | Gate activated for N ≥ 1 commits; all compliant |
| `PASS-zero-activations` | 0 | Scope target exists; commit range non-empty; all diffs non-empty; no commits touched the pinned crate's `*.rs`/`*.bats` |
| `EMPTY-or-UNREACHABLE` | 2 | Pinned crate path absent from HEAD tree (stale pin); OR commit range empty; OR any commit produced an empty diff |

**Why PASS-zero-activations is a distinct named outcome:** A docs-only PR legitimately
produces zero activations — no obligation applies and the gate MUST NOT block it. Naming
this outcome (rather than an anonymous exit-0) makes it observable: "this gate produced
PASS-zero-activations across N CI runs" is a countable data point, whereas an unnamed
exit-0 is not. The SystemVerilog practice of pairing `assert property (A |-> B)` with
`cover property (A)` captures the same idea: zero antecedent hits must be visible, not
silently passed over. The activation count is always emitted in the output.

**Why PASS-zero-activations requires scope-target presence:** Reaching PASS-zero-activations
means: (1) the pinned crate directory exists at HEAD, (2) the commit range is non-empty,
(3) all diffs are measurable, (4) none touched the pinned crate. A gate that exited 0 with
zero activations without confirming (1) could be silently consuming every PR after a refactor
renamed the crate — a stale pin is byte-identical to a legitimate out-of-scope PR. The stale-
pin guard converts that class from silent-green to EMPTY-or-UNREACHABLE (exit 2).

**Why EMPTY-or-UNREACHABLE exits non-zero (three triggers):**

1. **Stale pin:** `$PLUGIN_CRATE` directory does not exist in the checked-out tree at HEAD.
   Possible causes: crate renamed, moved, or restructured; the pinned constant in the gate
   script was not updated. A gate whose scope target has disappeared will produce zero
   activations forever — which is byte-identical to a legitimate docs-only PR — until someone
   counts "this gate has never fired across N runs." The stale-pin guard surfaces this
   immediately. Fail closed.

2. **Empty commit range:** `git log MERGE_BASE..HEAD` returned no commits. Possible causes:
   `fetch-depth: 0` not set, wrong `MERGE_BASE` computation, or mis-scoped job. A gate that
   processed zero commits cannot be distinguished from a gate that was misconfigured.

3. **Empty per-commit diff:** a commit exists in the range but `git diff --name-only C^1 C`
   returned empty. Possible causes: `git commit --allow-empty`, measurement tool failure, or
   a shallow-clone boundary that passed the parentless check but still can't diff. An empty
   diff is unmeasurable scope — the gate cannot tell whether the commit touched the pinned
   crate. Fail closed.

In all three cases, the gate exits non-zero to surface the CI setup defect immediately. This
is the same principle as in-toto's missing-link detection and `opa test --fail-on-empty`: a
step that produced no verifiable record is a verification failure, not a clean pass.

---

### Decision 9 — AMENDED (v1.11): unconditional per-commit iteration + Rust gate

F-S2107-P10-002 (BLOCKER [regression]) demonstrated that attestation sections for commits
`67ffbdcc` and `38c70f9e` were added retroactively at `5370db80`. Literal-shell evidence
(run in the factory-artifacts worktree; `|| true` replaces `|| echo 0` per v1.7 fix — see
§Rationale "Three `|| echo 0` occurrences"):

```
$ RG="crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md"
$ for sha in 37022ecc 67ffbdcc 38c70f9e 5370db80; do
    count=$(git show "${sha}:${RG}" 2>/dev/null \
      | grep -cE '^### .*assertion-site attestation' || true)
    echo "${sha}: attestation-headings=${count}"
  done
37022ecc: attestation-headings=0
67ffbdcc: attestation-headings=0
38c70f9e: attestation-headings=0
5370db80: attestation-headings=3
```

A push-tip-only check passes at `5370db80` (3 headings) while missing the violation at
`67ffbdcc` and `38c70f9e`. The POLICY 15 obligation states "EXISTS at that commit" —
this is a per-commit predicate. The gate must iterate.

**Ruling 9(a) — Per-commit iteration (§Decisions 8+9): implemented by `crates/policy15-attestation-gate/`**

The production gate mechanism is implemented in the Rust crate
`crates/policy15-attestation-gate/` (landing commit `d2a3176a`, branch
`feature/policy15-gate-rust`). The crate lives in the workspace and is exercised by
`cargo test --workspace --all-targets` on every CI push — no manual extraction required.

**Scope:** unconditional obligation, path-pinned. The governing log path and crate prefix are
architectural constants declared in the crate's public API (`src/lib.rs`):

- `PLUGIN_CRATE = "crates/hook-plugins/validate-cross-site-correspondence"`
- `RED_GATE_LOG = "crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md"`

**Four-outcome verdict — `GateOutcome` enum (normative representation):**

| Outcome | `GateOutcome` variant | Exit code |
|---------|----------------------|-----------|
| `FAIL` | `GateOutcome::Fail(Vec<FailedCommit>)` | 2 |
| `PASS-N-activations (N ≥ 1)` | `GateOutcome::PassWithActivations(usize)` | 0 |
| `PASS-zero-activations` | `GateOutcome::PassZeroActivations` | 0 |
| `EMPTY-or-UNREACHABLE` | `GateOutcome::EmptyOrUnreachable(UnreachableCause)` | 2 |

`UnreachableCause` is `StalePin | EmptyRange | UnmeasurableDiff { commit }`.
`FailReason` is `LogAbsent | AttestationMissing`.

`#[non_exhaustive]` is deliberately absent from `GateOutcome`: adding a new outcome variant
MUST produce a compile error at every `match` site, forcing test coverage to be updated.

**Why a type, not an integer:** Representing outcomes as a Rust enum makes the v1.9 level-5
defect (four gate outcomes sharing two exit codes) structurally impossible. In the bash
implementation, `FAIL=2` and `EMPTY-or-UNREACHABLE=2` shared an exit code; a control
asserting only `$? -ne 0` could pass while the wrong non-zero variant was exercised (the v1.9
R4 bash fixture exercised stale-pin while claiming to test empty-range — the exact failure
§Decision 10 Requirement 4 was added to address). In the Rust implementation, `Fail` and
`EmptyOrUnreachable` are distinct variants; `matches!()` assertions in the tests cannot
conflate them. The type system enforces Requirement 4 structurally.

**Guard ordering:** The stale-pin guard (`tree_path_exists` against `HEAD:${PLUGIN_CRATE}`)
runs **before** the `git merge-base` lookup. This ordering invariant is documented in the
`run_gate` function body and pinned by test
`test_run_gate_guard1_stale_pin_beats_unresolvable_base`: when the pinned crate is absent
AND the base branch is unresolvable, the gate returns `StalePin`, not `EmptyRange`. Without
guard 1, a crate-absent + remote-removed repo would silently report `EmptyRange`, masking
the more actionable stale-pin diagnosis.

**CI invocation (for `devops-engineer`):**

```
policy15-attestation-gate [<base-branch>]
```

- Default base branch: `develop`. Override via `BASE_BRANCH` environment variable.
- Requires `fetch-depth: 0` in the CI checkout action (full history for `git merge-base`
  and `git show ${C}:${path}` — §Decision 9 Ruling 9(c) item 2).
- Unconditional job — no `paths:` filter. A filtered job reports SUCCESS when skipped,
  which is itself a vacuous pass (§Decision 7 Ruling 7(a)).
- Required-check job name: `policy-15-attestation-location`.
- Exit codes: 0 for either Pass variant (`PassWithActivations` or `PassZeroActivations`);
  2 for `Fail` or `EmptyOrUnreachable`; 1 for hard error (`GateError` — git not installed,
  repository inaccessible, unexpected I/O).

> **Implementation note — `lib.rs` module doc version reference:** The crate's module-level
> doc comment reads `//! POLICY 15 ATTESTATION-LOCATION GATE — ADR-040 §Decisions 8+9
> (v1.8)`. The `v1.8` cite is stale — the ADR is at v1.11. This is a cosmetic lag in the
> code; the mechanism is authoritative. The module doc should be updated to cite
> `§Decisions 8+9 (v1.11)` in the next crate update.

POLICY 21 compliance: the crate is a workspace Rust binary, not a new `.sh` file.

**Ruling 9(b) — Same-commit bundling (TD-VSDD-053) retained:**

The code-repo commit that adds or strengthens a bats assertion site MUST include, in the
same commit, the `### Pass-N assertion-site attestation (<PARENT-SHA>)` section in
`red-gate-log.md`. The parent SHA is `git rev-parse HEAD^1` evaluated in the code repo at
the moment the commit is prepared. The author knows `HEAD^1` before staging — no circular
dependency. §Decision 3's reasoning carries forward unchanged.

**Ruling 9(c) — Checkout requirements (fetch depth, merge-base, unconditional job):**

1. **Unconditional job — no `paths:` filter:** The `policy-15-attestation-location` CI job
   MUST run on every PR with no `paths:` filter. A job skipped via `paths:` filter reports
   SUCCESS as a required check — itself a vacuous pass. The EMPTY-or-UNREACHABLE case
   (stale pin, empty commit range, or empty per-commit diff → exit 2) and the
   PASS-zero-activations case (scope target exists, all diffs measurable, no crate matches →
   exit 0) are both handled inside the script. The job is never skipped by the CI harness.

2. **`fetch-depth: 0`:** The CI checkout action MUST use `fetch-depth: 0` (full history).
   The GitHub Actions default is `fetch-depth: 1` (shallow). A shallow clone causes
   `git merge-base` to fail or misreport, `git log` to silently truncate the commit range,
   and `git rev-parse C^1` to fail at the shallow boundary — all producing incomplete
   iteration or false parentless-commit warnings. Full history allows `git show ${C}:${LOG}`
   to resolve files at any commit in the PR branch.

3. **`git merge-base` instead of `origin/${BASE_BRANCH}` tip:** The script uses
   `MERGE_BASE=$(git merge-base HEAD "origin/${BASE_BRANCH}")` as the range base, not
   `origin/${BASE_BRANCH}` directly. `origin/${BASE_BRANCH}` is the current remote tip,
   which advances as other PRs merge. Using the tip directly causes the commit loop to
   over-skip or under-include commits depending on tip advancement timing. `git merge-base`
   is the actual branch-point — stable for the lifetime of the PR.

4. **Parentless-commit detection:** `git rev-parse --verify "${C}^1"` is checked before
   processing each commit. A parentless commit (root commit, or at a shallow boundary in
   edge cases) would cause `git diff ${C}^1 ${C}` to fail. The check emits a WARNING and
   skips the commit rather than crashing the job with an undiagnosed error.

---

### Decision 10 — AMENDED (v1.11): PROJECT-WIDE four-outcome gate rule with required controls

Per the pass-10 Part C codification-2 discipline (and the production-grade default
self-audit checklist): every gate whose PASS is recorded as evidence must be demonstrably
non-vacuous. The §Decision 6 gate returned a structurally trivial PASS from D-965 ratification
through pass-10 — it was incapable of returning FAIL because its trigger domain was empty.
A future gate must prove it can fail.

**v1.2 Gap B — `67ffbdcc` fixture will be orphaned at squash-merge:**

ADR-040 v1.2 designated `67ffbdcc` as the fixture for the non-vacuity job. Literal-shell
verification (2026-08-10):

```
$ git branch -r --contains 67ffbdcc
  origin/feature/S-21.07-validate-cross-site-correspondence
```

`67ffbdcc` is reachable only from the `feature/S-21.07` branch. When that branch is
squash-merged to `develop`, the commit is orphaned — not reachable from any surviving ref.
Git's garbage collector may prune it on the next `git gc`. The fixture's durability is
bounded by the S-21.07 merge event.

Additionally, the v1.2 draft inverted fixture terminology: a violating fixture (gate must
FAIL) is a **positive control**, not a negative control. A **negative control** is a
compliant fixture (gate must not fire — PASS). This ADR corrects that inversion throughout.

**Project-wide codification (general rule):**

Every mechanical gate in this codebase whose PASS verdict is recorded as evidence — in an
adversarial pass report, a burst-log Dim-2 attestation, or a STATE.md decision-log row —
MUST satisfy all three of the following requirements:

1. **Four-outcome verdict:** The gate MUST report exactly four distinct outcomes:
   - `FAIL` (non-zero exit): the obligation was violated.
   - `PASS-N-activations (N ≥ 1)` (zero exit): the gate activated and verified at least one
     subject without finding a violation. The count N must appear in the output.
   - `PASS-zero-activations` (zero exit): the scope target was confirmed to exist; the
     candidate set was fully measured; none fell within the gate's scope. Legitimate out-of-
     scope subjects must not be blocked. The activation count (0) must appear in the output
     so it is observable across CI runs.
   - `EMPTY-or-UNREACHABLE` (non-zero exit): the scope target does not exist (stale pin), or
     the candidate set was empty, or any candidate produced an empty change set (unmeasurable
     scope). This is a CI setup defect or a scope-staleness defect, not a clean run.

   A gate with only two outcomes (FAIL / PASS) cannot distinguish "checked N things and
   found no violations" from "checked zero things and found nothing" — the latter is a
   vacuous pass. A gate with three outcomes that conflates PASS-zero-activations and
   EMPTY-or-UNREACHABLE into a single exit-0 cannot distinguish "legitimately out of scope"
   from "scope target stale." A control set that only exercises FAIL and PASS-N-activations
   **cannot detect if PASS-zero-activations is reached instead of PASS-N-activations** —
   this is the exact defect class of F-001 across four redesign iterations.

2. **A control for every outcome the gate can return.** Not merely for FAIL. This is the
   generalization the four-iteration F-001 history forces: every named outcome is a
   specification claim; every specification claim needs a corresponding verification. For
   each outcome, a dedicated EICAR-style synthetic fixture MUST exist that reaches that
   outcome by the correct mechanism — not by accident, and not by an unnamed path. The
   fixture MUST be constructed synthetically (`mktemp -d` scratch repos). Referencing
   historical SHAs is forbidden — a SHA may be orphaned by squash-merge, pruned by `git gc`,
   or invalidated by force-push. The Semgrep Registry enforces the analogous principle:
   every rule must include both true-positive and true-negative test cases. This rule
   extends that principle to all four outcomes.

3. **Stale-pin guard for pinned scope targets.** A gate with a pinned path constant MUST
   verify the path exists in the checked-out tree before iterating over candidates. A stale
   pin produces PASS-zero-activations forever after a refactor that renames or restructures
   the target — silent-green, indistinguishable from a legitimate docs-only PR without a
   count-over-time audit. The guard converts that silent-green into EMPTY-or-UNREACHABLE
   (exit 2) immediately.

4. **When a gate's outcome set is larger than its exit-code set, controls MUST assert the
   outcome identifier, not the exit code alone.** A gate with four outcomes mapped to two
   exit codes (FAIL=2/EMPTY-or-UNREACHABLE=2; PASS-N-activations=0/PASS-zero-activations=0)
   cannot be distinguished by exit code. A control that asserts only `exit != 0` may pass
   when the wrong non-zero outcome was reached — as when R4 hit EMPTY-or-UNREACHABLE (stale
   pin) while claiming to exercise EMPTY-or-UNREACHABLE (empty range). This is the same
   defect class as F-001 one level up: the control, rather than the gate, reports success
   without executing its intended path. Each control MUST capture the gate's stdout and
   `grep` for the specific outcome identifier string (e.g., `'EMPTY-or-UNREACHABLE: git
   range returned no commits'` versus `'EMPTY-or-UNREACHABLE: stale pin'` versus
   `'EMPTY-or-UNREACHABLE: unmeasurable diff'`). Exit code is a secondary check; the
   outcome string is the primary assertion.

**Application to POLICY 15 ATTESTATION-LOCATION GATE:**

This gate is the first instance of the project-wide rule. The gate covers two violation
modes and three non-violation outcomes requiring separate reachability proof:

- **Violation mode (i) — obligation violated:** a commit changes `*.rs`/`*.bats` in the
  pinned crate but the pinned log is absent or has no matching attestation heading → FAIL.
  Two separately-implemented FAIL paths require two controls.
- **Violation mode (ii) — EMPTY-or-UNREACHABLE:** three distinct trigger paths, each
  requiring its own control (Requirement 4):
  - (a) stale pin — crate absent from HEAD tree;
  - (b) empty commit range — `git log MERGE_BASE..HEAD` returns no commits;
  - (c) empty diff — a commit in range produced an empty changed-file set.
- **PASS-zero-activations:** scope target exists, all diffs measurable, none touch the pinned
  crate → exit 0 (docs-only PRs must not be blocked).
- **PASS-N-activations:** gate activated, all compliant → exit 0.

**Ruling 10(a) — EICAR-style synthetic fixtures; no SHA dependency:**

The non-vacuity controls job MUST construct all fixture repositories synthetically in
`mktemp -d` scratch space. No dependency on any commit SHA in the code-repo or
factory-artifacts history. The job is immune to squash-merge orphaning, force-push history
rewrites, and repository pruning events.

**Ruling 10(b) — Sixteen required tests (`#[test]` cases in `crates/policy15-attestation-gate/src/lib.rs`, commit `d2a3176a`):**

Nine of the sixteen tests map directly to the seven original outcome controls (two stale-pin
and unresolvable-base variants extend the original stale-pin and empty-range controls). The
remaining seven cover additional correctness properties. All tests use `matches!()` assertions
against enum variants, not exit-code integer comparisons.

| Test function | `GateOutcome` variant exercised | Maps to original control |
|---------------|---------------------------------|--------------------------|
| `test_positive_1_absent_log` | `Fail(LogAbsent)` | Positive control 1 |
| `test_positive_2_no_attestation_heading` | `Fail(AttestationMissing)` | Positive control 2 |
| `test_negative_compliant_attestation` | `PassWithActivations` | Negative control |
| `test_pass_zero_activations` | `PassZeroActivations` | PASS-zero control |
| `test_empty_range` | `EmptyOrUnreachable(EmptyRange)` | EMPTY-range control |
| `test_stale_pin` | `EmptyOrUnreachable(StalePin)` | EMPTY-stale-pin control |
| `test_unmeasurable_diff` | `EmptyOrUnreachable(UnmeasurableDiff)` | EMPTY-diff control |
| `test_disk_present_tree_absent_is_stale_pin` | `EmptyOrUnreachable(StalePin)` — disk present, git tree absent | Stale-pin variant |
| `test_unresolvable_base_fails_closed` | `EmptyOrUnreachable(EmptyRange)` — unresolvable base branch | Unresolvable-base variant |
| `test_guard_ordering_stale_pin_beats_empty_range` | Guard ordering: `StalePin` before `EmptyRange` | Additional |
| `test_run_gate_guard1_stale_pin_beats_unresolvable_base` | Guard 1: `StalePin` before unresolvable base at `run_gate` entry | Additional |
| `test_bats_file_activates_gate` | `*.bats` file changes activate the gate | Additional |
| `test_rs_outside_crate_does_not_activate` | `*.rs` file outside pinned crate does not activate | Additional |
| `test_prose_sha_does_not_cause_false_count` | Prose SHA reference does not cause false attestation count | Additional |
| `test_multiple_activations_all_compliant` | Multiple activations; `PassWithActivations(N)` count correct | Additional |
| `test_identifier_strings_are_greppable` | `GateOutcome::identifier()` strings are machine-greppable | Additional |

Positive controls 1 and 2 cover violation mode (i) via two distinct `FailReason` variants
(`LogAbsent` vs `AttestationMissing`). The PASS-zero-activations test proves a docs-only
scenario reaches `PassZeroActivations`, not `PassWithActivations`. The three
EMPTY-or-UNREACHABLE controls each exercise one `UnreachableCause` variant and assert the
specific variant via `matches!()` — a test that only asserts `exit_code() == 2` cannot
distinguish which variant was returned (Requirement 4; this is what the Rust type system
enforces structurally). The negative control proves the gate does not false-positive on
compliant attestations.

Note (bash fixture era, v1.4–v1.10): `mkdir -p` alone was insufficient in the bash controls
because empty directories are not tracked by git. A committed `.gitkeep` inside
`${PLUGIN_CRATE}/` was required for `git cat-file -e "HEAD:${PLUGIN_CRATE}"` to succeed.
For the PASS-zero fixture, the seed commit was required before capturing `BASE3`; for the
EMPTY-range fixture, `HEAD4` was captured after the seed commit. In the Rust test suite,
the in-process `git2` (or equivalent) API constructs fixtures programmatically without these
ceremony constraints — the seed/scenario separation is encoded in test structure, not in bash
variable capture order.

**Ruling 10(c) — Job name: `attestation-gate-non-vacuity-controls`:**

The job is named `attestation-gate-non-vacuity-controls` (not `attestation-gate-negative-
control` — that label covers only one of seven fixtures).

**Ruling 10(d) — Controls implementation:** The 16 tests listed in Ruling 10(b) are the
controls. They live in `crates/policy15-attestation-gate/src/lib.rs` (commit `d2a3176a`) and
run under `cargo test --workspace --all-targets`. No separate inline bash script is needed —
the crate IS the controls job. The `attestation-gate-non-vacuity-controls` CI job (Ruling
10(c)) invokes `cargo test -p policy15-attestation-gate`.

**Ruling 10(e) — Empirical execution record:**

The v1.6 version of this script was executed against the v1.6 §Decision 9 production gate.
The run found two defects that would have permitted the gate's primary violation class to
pass silently:

1. **`|| echo 0` double-output defect (CRITICAL):** `grep -cE` prints `0` to stdout and exits 1
   on no-match; `|| echo 0` captured both `0` lines. The resulting variable held `"0\n0"`,
   which `[ "$VAL" -ne 1 ]` cannot evaluate as integer — bash exits with code 2 (invalid
   integer), which the `if` condition interprets as false, taking the `else` (PASS) branch.
   A commit with crate changes, log present, and NO matching attestation heading reported
   PASS. Positive Control 2 — the control designed to catch this exact violation — failed
   for the same reason, discovering both the gate defect and the control defect simultaneously.
   **Fixed in v1.7:** `|| echo 0` → `|| true` at all four sites (CHANGED and COUNT in §Decision
   9 script; CHANGED and COUNT in `_run_gate`).

2. **`git add --quiet` invalid flag:** `git add` has no `--quiet` option. Execution aborted
   with `error: unknown option 'quiet'` on the first control that called it. The control
   harness could not run at all as specified.
   **Fixed in v1.7:** `--quiet` removed from all five `git add` calls in the controls script.

This execution record is the single strongest empirical justification for §Decision 10's
project-wide rule. **A control that is specified but never executed is worth nothing.** The
controls caught, on their first real execution, a defect that would have passed every commit
in the retroactive-attestation violation class that this ADR exists to prevent. The controls
themselves had the same bug, which is why the defect survived specification review — only
execution revealed it. This is recorded here permanently so that any future reader
understands both why the rule exists and why execution is non-negotiable.

---

## Rationale

### Why parent-SHA and not an alternative discriminator

**Alternative A — Pass-N ordinal only (no SHA):** Rejected. Ordinals are human-assigned
and collide across stories (two stories can have a "Pass-8"). Within a single story they
are monotonically increasing, but an adversary forging a pass ordinal would pass the gate.
SHA binding is the correct cryptographic anchor.

**Alternative B — Timestamp in heading:** Rejected. Timestamps are not universally
monotonic (clock skew, NTP jumps) and are not mechanically verifiable without external
time-oracle dependencies. SHA binding is available from Git natively.

**Alternative C — Content hash of the assertion-site diff:** Rejected. Computing a content
hash of the diff at commit time requires running `git diff HEAD^1` _within_ the commit
process, which creates a race condition between staging and diffing. Parent SHA is
simpler, universally available, and equally binding.

**Alternative D — Keep HEAD-SHA; fix via git commit --amend + git notes:** Rejected.
`git commit --amend` changes the SHA of the commit being amended, requiring a second
amendment to update the embedded SHA, creating infinite regress. Git notes are stored
outside the commit's content hash and cannot satisfy a predicate that checks file content.

### Why same-commit bundling (TD-VSDD-053) is retained rather than relaxed

The "follow-up commit" convention that has been practiced since D-912 satisfies the
SHA-binding goal (a different commit can know the prior commit's SHA) but violates
TD-VSDD-053's single-commit-per-burst requirement and also creates a split between the
assertion-site changes and their attestation. The split enables a state where changes are
pushed and visible to CI but the attestation is not yet present. The same-commit bundling
prevents this window entirely and is achievable under the parent-SHA form.

### Why the gate must remain mechanically executed, not narratively asserted

Per D-449(a) / META-LEVEL-24: a gate codified in prose but never executed cannot detect
its own scope degradation. The D-912 predicate was a prose gate that always returned 0 —
this is exactly the self-degradation META-LEVEL-24 documents. The replacement predicate
MUST be invoked as a literal shell command with captured stdout to prevent recurrence of
the same class. In the code-repo CI context, captured stdout is the CI job log, which is
automatically retained and auditable.

### Why unconditional job over paths-filtered (§Decision 9 Ruling 9(c))

A GitHub Actions job with a `paths:` filter is automatically skipped when the PR touches
no files matching the filter. A skipped job reports SUCCESS as a required check. This means
a `paths: ['**.rs', '**.bats']` filtered job passes vacuously for every PR that touches only
infrastructure files, docs, TOML, YAML, or other non-assertion-site content — which is the
same category error §Decision 7 diagnosed in the factory-artifacts context. The gate returns
"success" without having examined anything.

The PASS-zero-activations and EMPTY-or-UNREACHABLE outcomes are handled inside the script
(§Decision 8 Ruling 8(c)): stale pin → EMPTY-or-UNREACHABLE (exit 2); empty commit range →
EMPTY-or-UNREACHABLE (exit 2); empty per-commit diff → EMPTY-or-UNREACHABLE (exit 2); scope
target exists, all diffs measurable, no crate matches → PASS-zero-activations (exit 0, with
activation count in output). The job runs unconditionally and always terminates — there is no
path where the CI harness silently skips it.

**Why unconditional obligation eliminates antecedent failure (§Decision 8 Ruling 8(a)):**

A paths-filtered job is one form of antecedent failure at the CI harness layer. The
corresponding failure at the obligation layer is a conditional obligation: "if assertion-site
files changed, then attestation is required." This sub-condition is itself an antecedent —
and it can fail to fire. Under an unconditional obligation, the antecedent is absent. There
is no condition to fail. This is the structural cure for the INAPPLICABLE defect class.

The conceptual anchor is in-toto's REQUIRE layout rule (in-toto specification §4.3): a named
artifact must appear among a step's products with no qualifying condition. We do not adopt
in-toto's framework; the analogy holds at the semantic level — `red-gate-log.md` must appear
as a product of any commit touching the crate's `*.rs`/`*.bats` files, unconditionally.

### Why vacuity, not tautology (§Decision 7 characterization)

The v1.2 analysis referred to the gate's permanent-PASS behaviour as a "tautology." This is
imprecise. A **tautology** (in logic) is a formula that is true under all interpretations,
independent of model. A **vacuous pass / antecedent failure** is a property that holds
trivially because its trigger condition is never satisfied — not because the consequent is
true in all models. The seminal reference is Beer et al., "RuleBase: Model Checking at IBM"
(CAV 1997), which defined model-dependent vacuity and introduced the property-coverage
literature. Kupferman & Vardi, "Vacuity Detection in Temporal Model Checking" (CHARME 1999)
formalized the antecedent-failure sub-class that applies here.

The distinction matters: the gate is not logically necessary (it would fail if evaluation
were moved to the correct context), so "tautology" implies it cannot be fixed. "Vacuous pass"
correctly identifies that the gate fires in an empty domain and that relocating evaluation to
a non-empty domain (code-repo CI) makes the predicate contingent and non-vacuous.

**The premise was sound; the implementation was vacuous (two separable failures):**

This ADR identifies two distinct failures in the original D-912/D-965 implementation:

1. **Premise failure (false):** §Decision 6 mischaracterised commit `5370db80` as a
   stability entry rather than a retroactive attestation. This was an error in the rationale
   supporting D-965 ratification — corrected in §Decision 7 §Erratum.

2. **Implementation failure (vacuous execution context):** The gate ran in factory-artifacts,
   where 0 of 3743 files matched `*.rs`/`*.bats`. The antecedent was never satisfied. The
   gate always returned PASS without checking anything.

These are independent failures. The premise failure (F-003) affects ratification history.
The implementation failure (F-001) affects every gate run since D-912. Both are remediated
by v1.4; neither was a flaw in the contemporaneity obligation itself (which SLSA v1.2
endorses — see below).

### Why SLSA v1.2 Source Track endorses the premise (contemporaneity requirement)

The ATTESTATION-LOCATION GATE's underlying obligation is a **contemporaneity requirement**:
the attestation must appear in the same commit as the assertion-site change. SLSA v1.2 Source
Track Level 2 (APPROVED, available at slsa.dev/spec/v1.2/source-requirements) independently
names this as load-bearing: "Source Provenance MUST be created contemporaneously with the
branch being updated."

This confirms the gate's goal (D-912) was sound. The defects corrected in this ADR — vacuous
execution context (F-001) and false ratification premise (F-003) — are implementation flaws,
not goal flaws (see "Why vacuity, not tautology" above for the two-failure decomposition).
The contemporaneity obligation is retained in v1.4.

### Why code-repo CI and not factory-artifacts dispatcher (§Decision 7)

The factory-artifacts worktree is the correct execution context for factory-artifacts
governance: checking STATE.md structure, burst-log 8-block completeness, 4-index
consistency. It is the wrong context for obligations that bind code-repo commits.

The ATTESTATION-LOCATION GATE obligation reads: "a fix wave that adds or strengthens any
bats assertion site MUST NOT be pushed until…" This is a code-repo obligation. The natural
enforcement locus is the code-repo CI required-check, which:

1. Has direct access to the code repo's git history without cross-repo path threading
2. Runs on the code-repo push/PR event that triggers the obligation
3. Fails in a way that is visible to the PR author (required-check UI)
4. Is testable locally (`act`) without factory-artifacts worktree setup

**Alternative considered: WASM plugin in the dispatcher chain.** A WASM plugin could
intercept `git push` calls from Claude Code. Rejected: (a) the dispatcher runs in the
factory-artifacts context; intercepting code-repo pushes requires cross-repo path threading
that adds complexity without benefit; (b) pushes issued directly from a shell bypass the
dispatcher entirely; (c) WASM fuel budget risks documented at D-965 (four plugins timed
out on large-file writes) add false-positive blocking risk to the hook chain.

### Why per-commit iteration is required (§Decision 9)

The POLICY 15 obligation uses the phrase "EXISTS at that commit." A push-tip check
satisfies the obligation only at the final commit of a push. If a PR contains:

- Commit A: adds `*.rs` assertion site (no attestation)
- Commit B: adds `### Pass-N assertion-site attestation (<SHA-A>)` to `red-gate-log.md`

A push-tip check at Commit B sees attestation present and passes. But Commit A was pushed
to the server without an attestation — violating "MUST NOT be pushed until…". The
`5370db80` episode is a concrete historical instance: three attestation sections were
added retroactively, and a push-tip check would have passed at `5370db80`. Per-commit
iteration closes this window by evaluating the obligation at each commit independently.

### Why EICAR-style synthetic fixture over a SHA-pinned commit (§Decision 10, v1.4)

ADR-040 v1.2 used commit `67ffbdcc` as the single non-vacuity fixture. Literal-shell
verification (2026-08-10) showed the fatal durability flaw:

```
$ git branch -r --contains 67ffbdcc
  origin/feature/S-21.07-validate-cross-site-correspondence
```

`67ffbdcc` is reachable only from the `feature/S-21.07` branch. When that branch is
squash-merged to `develop`, the commit is orphaned — not reachable from any surviving ref.
Git's garbage collector prunes unreachable objects on the next `git gc`. From that point,
`git show 67ffbdcc:...` fails with "bad object" and the control job reports ERROR rather
than FAIL — the gate becomes silently invalid after the very merge event the gate is supposed
to protect.

The v1.2 rationale against synthetic fixtures ("requires `git commit` in CI, which needs
author configuration and fetch-depth planning") underweighted the orphaning risk. The
`_make_repo` helper in §Decision 10's inline script addresses the setup concern in ~10 lines;
the orphaning risk has no comparable patch.

**Why seven controls (v1.9, up from six in v1.6):**

v1.3 had three controls covering failure mode (i) (two FAIL paths) and the PASS path. v1.4
added a control for failure mode (ii) (empty commit range → EMPTY-or-UNREACHABLE). v1.5 added
a PASS-zero-activations reachability control (docs-only PR must exit 0 via the named path,
not an unnamed exit-0). v1.6 added a stale-pin control — proving that a tree without the
pinned crate directory exits non-zero as EMPTY-or-UNREACHABLE (not PASS-zero-activations).
v1.9 added a seventh control for the third EMPTY-or-UNREACHABLE trigger: an `--allow-empty`
commit in range produces an unmeasurable diff, which must return `UnmeasurableDiff`, not be
silently skipped. v1.11 migrates all seven controls to Rust `#[test]` cases (16 total;
Ruling 10(b)) where the type system enforces Requirement 4 structurally.

**A control set that only exercises FAIL and PASS-N-activations cannot detect if
PASS-zero-activations is silently reached instead of PASS-N-activations.** This is the
precise defect class of F-001 across four redesign iterations: the gate exited 0 via a zero-
activation path that was structurally indistinguishable from a genuine PASS. Four redesigns
(v1.2, v1.3, v1.4, v1.5) each added a guard without adding a stale-pin reachability control.

The stale-pin control (v1.6) is the member of the control set that would have caught F-001
if it had existed when the gate was first written. The path-pinning decision (v1.4) increased
the stale-pin exposure relative to dynamic derivation, because a pinned constant is exactly
the kind of artifact that goes stale on a codebase refactor while the gate script stays
unchanged.

Seven controls together confirm: (1) absent-log → `Fail(LogAbsent)`, (2) no-attestation →
`Fail(AttestationMissing)`, (3) docs-only commit → `PassZeroActivations` with matching output,
(4) empty range → `EmptyOrUnreachable(EmptyRange)`, (5) stale pin (crate dir absent) →
`EmptyOrUnreachable(StalePin)` with matching output, (6) empty diff (`--allow-empty` commit) →
`EmptyOrUnreachable(UnmeasurableDiff)`, (7) compliant commit → `PassWithActivations` with
matching output. Controls 3 and 7 are distinct: control 7 proves `PassWithActivations` is
reachable when the gate activates; control 3 proves `PassZeroActivations` (not
`PassWithActivations`) is returned when no crate changes are found.

### Why path-pinned over per-crate-derived (§Decision 8, v1.4)

ADR-040 v1.3 used per-crate dynamic derivation: `grep -oE 'crates/hook-plugins/[^/]+'`
extracted crate prefixes from the diff, then constructed `<prefix>/docs/red-gate-log.md`
per crate. The rationale was generalization — any future hook-plugin crate would be covered
automatically without an ADR amendment.

Per-crate derivation is still a step change from `find|head-1`, but it retains an antecedent:
the loop iterates over derived crates and the INAPPLICABLE branch fires when no crates are
derived. That is precisely the antecedent-failure mechanism that enables F-001. The cure
requires eliminating the antecedent, which requires naming the subject explicitly.

**Why path-pinning is the correct generalization pattern:** Each story's attestation-location
gate is an explicit architectural decision. When S-21.07 ships, devops adds the S-21.07 gate.
When S-22.XX ships with a new hook-plugin crate, devops adds the S-22.XX gate for that crate.
Each gate is a named CI step with a pinned path — explicit, auditable, immune to runtime
mis-resolution. The cost is one 40-line CI YAML block per story, identical to the current
structure. Per-crate runtime derivation would obscure which crates are gated and under which
policy version.

**Why single-crate pinning does not exclude future crates:** Ruling 8(a) pins
`crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md` as the S-21.07
log path. Future stories add their own pinned CI steps. Nothing in the architecture prevents
two pinned steps from running in parallel on the same PR — one for each in-flight story's
crate. This is the compositional model used by static analysis tools (Semgrep, Clippy): each
rule targets a named subject; the runtime engine runs all rules in parallel.

### Three `|| echo 0` occurrences — evidence blocks must be captured verbatim (v1.8)

The `|| echo 0` defect appeared at three distinct sites across this ADR's history:

1. **§Decision 9 production script — CHANGED and COUNT variables** (fixed v1.7): `grep -cE`
   exits 1 on no-match and prints `0`. `|| echo 0` appended a second `0`, producing a
   two-line variable. `[ "$VAL" -ne 1 ]` on a two-line string exits 2 (invalid integer); bash
   took the else branch; the gate silently passed missing-attestation commits. This is the
   CRITICAL functional defect.

2. **§Decision 10 `_run_gate` — CHANGED and COUNT variables** (fixed v1.7): Identical
   defect in the control-harness helper function. Positive Control 2 exercised this path and
   confirmed the false-pass behaviour before the v1.7 fix. The controls caught the functional
   failure on first execution — documented at Ruling 10(e).

3. **§Decision 9 literal-shell evidence block — count variable in the for loop** (fixed v1.8):
   The same `|| echo 0` in the evidence-capture command produced stray `0` lines after each
   zero-count line. The recorded stdout was hand-tidied — the stray lines were deleted before
   the block was committed. This violated POLICY 15 verbatim-stdout and POLICY 5
   HEAD-reproducibility: a reader re-running the recorded command against the same commits
   would see different output from what was recorded. Fixed: command uses `|| true`; captured
   output is pasted verbatim (output is unchanged — the hand-tidied lines happened to match
   the correct `|| true` output because all four commits genuinely had the heading counts shown).

**Lesson:** If the evidence block had been captured verbatim at authoring time, the stray `0`
lines would have appeared in the committed text. The divergence between "command produces
these lines" and "we recorded different lines" would have been immediately visible — either
triggering a fix before commit, or causing a reviewer to ask why the output differs. Verbatim
capture is not just a formality; it is a defect-detection step.

This extends D-449(a) / META-LEVEL-24 (mechanical gates must be invoked with captured stdout)
to evidence blocks in narrative sections: recorded output that is edited before committing
loses its evidentiary value. The three-site pattern also illustrates how a single idiom error
can propagate silently across layers (production logic, test harness, evidence record) when
each copy-paste site is reviewed in isolation rather than as a class.

---

## Consequences

### Positive

- The ATTESTATION-LOCATION GATE becomes non-vacuously satisfiable for the first time
  since D-912. §Decisions 1–6 specified a correct predicate; §Decisions 7–10 correct the
  execution context that made the predicate permanently INAPPLICABLE. In the code-repo CI
  context, the gate can return both PASS and FAIL.
- Same-commit bundling (TD-VSDD-053) is retained in the code-repo context.
- Per-commit iteration (§Decision 9) closes the retroactive-attestation window demonstrated
  by F-S2107-P10-002: attestation sections added in a later commit no longer satisfy the
  obligation at the commit where the assertion site was introduced.
- The mandatory non-vacuity controls job (§Decision 10) provides a standing non-vacuity
  proof via seven EICAR-style synthetic fixtures (positive-1, positive-2, pass-zero,
  empty-range, stale-pin, empty-diff, negative). Any future adversary or reviewer can observe
  all seven controls confirming the gate reaches both obligation-violation FAIL paths, the
  PASS-zero-activations path (with output verification), all three EMPTY-or-UNREACHABLE
  trigger paths (stale pin, empty range, and empty diff), and the PASS-N-activations path.
  Each control asserts the specific outcome identifier string (Requirement 4), not the exit
  code alone — preventing a control from passing while the wrong outcome path was exercised.
- The false premise in §Decision 6 (§Decision 7 §Erratum) is corrected in the permanent
  ADR record. D-965's ratification row is annotated as PROCURED-ON-MISCHARACTERIZATION.
- TD-VSDD-059 (paper-fix detection) is respected: the fix is structural (execution context
  relocation), not nominal (rename / doc-comment only).
- POLICY 21 compliance: no new `.sh` files; gate is inline CI YAML.

### Execution discipline — executable content in an ADR is exercised only by manual extraction

Every defect generation in this ADR was found by execution, not by prose review:

- v1.1–v1.4: the gate ran in the wrong context (factory-artifacts, empty domain) — found by
  execution.
- v1.6: stale-pin guard used a filesystem check (`[ ! -d ]`) instead of a git-tree check —
  found by executing the controls.
- v1.7: `|| echo 0` double-output defect in the production gate and control harness — found
  by executing the controls on their first real run.
- v1.8: `|| echo 0` in the evidence-block command produced stray output that was hand-tidied
  — found by re-running the evidence block verbatim.
- v1.9–v1.10: fixture-level defect (R3 `BASE3` captured before seed commit; R4 `mkdir -p`
  guard invalid against `cat-file -e`) — found by executing the fixture against the fixed
  gate, not by reading the script.
- v1.11: crate landed and pushed at `d5a90e74` — 15 tests, full fmt/clippy/test gate clean.
  Mutation A (attestation-count check `if count != 1` → `if false`): **killed** by
  `test_positive_2_no_attestation_heading`. Mutation B (`run_gate` stale-pin early return
  neutralised): **SURVIVED**. `test_unresolvable_base_fails_closed` asserted only
  `!outcome.is_pass()`; with guard 1 removed, outcome degraded from `StalePin` to `EmptyRange`
  — still non-passing, assertion still green. Fix at `d2a3176a`: `test_unresolvable_base_fails_closed`
  tightened to assert `EmptyOrUnreachable(EmptyRange)` specifically; new test
  `test_run_gate_guard1_stale_pin_beats_unresolvable_base` added asserting
  `EmptyOrUnreachable(StalePin)` — this is the mutant killer. Generation 7 was found **after**
  the crate landed; a full green suite shipped the surviving mutant.

This is the seventh generation of the same pattern: a defect class is found and fixed at
level N, and an instance of the same class is discovered at level N+1 on the next execution.
Prose review does not catch these because each level of indirection (gate predicate → gate
context → control harness → fixture setup → assertion logic) requires a running environment
to distinguish "this does what the comment says" from "this does something subtly different."

**v1.11 closes the structural defect for the bash scripts:** the controls now live as
`#[test]` cases in `crates/policy15-attestation-gate/src/lib.rs`. `cargo test` runs them on
every CI push. No manual extraction is required. The prose-ADR defect class (unexecuted
code fence → defect survives n review passes) no longer applies to the gate implementation
or its controls. The seventh generation (coarse-assertion gap at the test layer) was found
**after** the crate landed and passed CI — a full green suite shipped a surviving mutant, and
only post-push mutation testing surfaced it. "Tests pass" is not the acceptance criterion;
"each load-bearing check fails when its target is removed" is.

### Mutation-testing acceptance criterion

The acceptance criterion for the Rust gate controls is not "tests pass" but "each
load-bearing check has a test that fails when the check is removed."

**The generation 7 case study:**
1. Crate pushed at `d5a90e74` — 15 tests, all green, full fmt/clippy/test gate clean.
2. Mutation A (neutralise attestation-count check `if count != 1` → `if false`): **killed** by
   `test_positive_2_no_attestation_heading`.
3. Mutation B (neutralise `run_gate` stale-pin early return, guard 1): **SURVIVED**. The only
   test reaching that path, `test_unresolvable_base_fails_closed`, asserted `!outcome.is_pass()`.
   With guard 1 removed, `StalePin` degraded to `EmptyRange` — still non-passing. Assertion
   still green.
4. Fix at `d2a3176a`: `test_unresolvable_base_fails_closed` tightened to assert
   `EmptyOrUnreachable(EmptyRange)` specifically. New test
   `test_run_gate_guard1_stale_pin_beats_unresolvable_base` added (crate absent, no origin
   remote, invoked via `run_gate`) asserting `EmptyOrUnreachable(StalePin)`. This new test is
   the mutant killer; the tightening alone does not kill mutation B (with the crate seeded, the
   outcome is `EmptyRange` either way — only the new test, which removes the crate, triggers
   `StalePin` and thus fails when guard 1 is absent).
5. Both mutations re-run against `d2a3176a`: B now fails the new test; A still fails its test.

**A fix that closes a coarse-assertion gap and a fix that kills the mutant are two different
achievements.** Conflating them is how generation 8 happens.

**The altitude symmetry with generation 5:** Generation 5 (§Decision 10 Requirement 4,
v1.9 bash controls) asserted an exit code where four outcomes shared two codes — FAIL=2 and
EMPTY-or-UNREACHABLE=2. Generation 7 asserted a boolean category (`!outcome.is_pass()`) where
two failure causes shared one category — `StalePin` and `EmptyRange` are both non-passing.
Same structural error at different altitudes: a coarse predicate that is true for multiple
distinct values when only one is correct. A `matches!()` assertion against the specific
variant closes both.

**Why `matches!()` is non-negotiable:**
`matches!(outcome, GateOutcome::EmptyOrUnreachable(UnreachableCause::StalePin))` fails if the
implementation returns `EmptyOrUnreachable(EmptyRange)` — the precise confusion mutation B
exploited. Boolean-category assertions (`!is_pass()`, `exit_code() != 0`) and exit-code
assertions (`exit_code() == 2`) are not controls; they are documentation with extra syntax.

**Why mutation testing is non-negotiable for this crate:** The gate's entire value is in
distinguishing outcomes. A test suite that passes under a mutant that conflates two variants
is not a test suite — it is documentation that compiles. Mutation testing is the minimum bar
for any gate whose PASS verdict is recorded as evidence in an adversarial pass report or
burst-log Dim-2 attestation.

### Negative / Trade-offs

- The heading format changes from `### Pass-N assertion-site attestation (<HEAD-SHA>)` to
  `### Pass-N assertion-site attestation (<PARENT-SHA>)`. Existing attestation sections
  in `red-gate-log.md` that use the HEAD-SHA form are historical records; they do not need
  to be retroactively corrected. The new predicate only checks the current commit's parent
  SHA, not historical sections.
- An author unfamiliar with the parent-SHA form may write a section with the HEAD-SHA form.
  Mitigation: POLICY 15 text is updated atomically with ratification; the ADR text is the
  normative source until then.
- F-S2107-P10-002 (retroactive attestation at `67ffbdcc`/`38c70f9e`) is a permanent
  historical violation. History is immutable; the violation cannot be retroactively cured.
  An erratum note in `red-gate-log.md` and the D-965 decision-log row is the correct record
  response (see §Status).

### Status as of v1.3

PROPOSED 2026-08-10. **Human re-ratification required before `policies.yaml` v1.4.23 is
applied.** ADR re-opened from `active` due to PROCURED-ON-MISCHARACTERIZATION ratification.
The exact replacement text for POLICY 15's ATTESTATION-LOCATION GATE bullet is provided
verbatim in §Proposed `policies.yaml` Replacement Text below (v1.4.23).

### Status as of v1.4

PROPOSED 2026-08-10 (amended from v1.3). **Human re-ratification still required before
`policies.yaml` v1.4.23 is applied.** v1.4 incorporates two human decisions received after
v1.3 was drafted: (1) obligation becomes unconditional + path-pinned; INAPPLICABLE branch
retired; EMPTY-or-UNREACHABLE added as third outcome; (2) §Decision 10 codified as
project-wide three-outcome gate rule (not POLICY 15-specific). §Proposed Replacement Text
below is updated to reflect the v1.4 gate design.

---

## Proposed `policies.yaml` Replacement Text

The following is the exact string to replace the current ATTESTATION-LOCATION GATE bullet
in POLICY 15's `verification_steps` array. It replaces the entire bullet starting with
`"ATTESTATION-LOCATION GATE (v1.4.22;...)`.

**IMPORTANT: Do NOT apply this text until human re-ratification of ADR-040 v1.4.**
The v1.4.22 text (applied at D-965) is PROCURED-ON-MISCHARACTERIZATION; it remains in
`policies.yaml` until re-ratification. `policies.yaml` MUST NOT be edited to v1.4.23
before a D-NNN re-ratification decision row is recorded.

**Version after replacement: v1.4.23.** Version chain: v1.4.20 = ADR-040 §Decision 2
parent-SHA form; v1.4.21 = ADR-041 POLICY 16; v1.4.22 = ADR-040 §Decision 6 conditional
detection + line-anchored predicate (PROCURED-ON-MISCHARACTERIZATION); v1.4.23 = ADR-040
§Decisions 7–10 (v1.12): execution-context relocation to code-repo CI + unconditional
obligation + path-pinned log + FAIL-when-absent + stale-pin guard + four-outcome verdict
(`GateOutcome` Rust enum: `Fail`/`PassWithActivations`/`PassZeroActivations`/`EmptyOrUnreachable`)
+ per-commit iteration + unconditional job + Rust non-vacuity controls (16 `#[test]` cases
in `crates/policy15-attestation-gate/`, mutation-tested; project-wide four-outcome gate rule
with stale-pin guard + outcome-identifier assertion requirement codified at §Decision 10).

```
- "ATTESTATION-LOCATION GATE (v1.4.23; ADR-040 §Decisions 7-10; supersedes v1.4.22;
  F-S2107-P10-001/002/003 BLOCKER resolution): a code-repo commit that adds or strengthens
  any *.rs/*.bats file in crates/hook-plugins/validate-cross-site-correspondence/ MUST NOT
  be pushed until the pinned red-gate-log.md attestation section EXISTS at that same commit
  (per-commit obligation — not push-tip-only). Gate runs as a required-check CI job in the
  code repo (NOT in factory-artifacts worktree — running in factory-artifacts was the
  v1.4.22 vacuity defect: 0 *.rs/*.bats files of 3743 total). Job is UNCONDITIONAL — no
  paths: filter (a paths-filtered skipped job reports SUCCESS, itself a vacuous pass). CI
  checkout MUST use fetch-depth: 0. Log path is PATH-PINNED (architectural constant):
  crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md. FAIL-when-
  absent: if the pinned log does not exist at the commit being checked, gate FAILS — absence
  is a violation, not a skip condition. Stale-pin guard (BEFORE range check): if
  crates/hook-plugins/validate-cross-site-correspondence dir does not exist at HEAD →
  EMPTY-or-UNREACHABLE (exit 2, stale pin — crate renamed or moved). Per-commit check:
  COMMITS=$(git log MERGE_BASE..HEAD --format=%H); if COMMITS empty → EMPTY-or-UNREACHABLE
  (exit 2, CI setup defect); for each commit C: ALL_CHANGED=$(git diff --name-only C^1 C);
  if ALL_CHANGED empty → EMPTY-or-UNREACHABLE (exit 2, unmeasurable scope); CHANGED=$(echo
  ALL_CHANGED | grep -cE '^crates/hook-plugins/validate-cross-site-correspondence/
  .*\\.(rs|bats)$'); if CHANGED=0, skip C; else PARENT=$(git rev-parse C^1); check pinned
  log absent at C → FAIL; check git show C:<log-path> | grep -cE '^### .*assertion-site
  attestation \\(PARENT\\)' → 1; if count ≠ 1, FAIL: POLICY 15 HIGH. Four-outcome verdict:
  FAIL (exit 2) / PASS-N-activations (exit 0, N≥1) / PASS-zero-activations (exit 0, scope
  target exists AND all diffs measurable AND no crate matches) / EMPTY-or-UNREACHABLE (exit
  2, stale pin OR empty range OR empty diff). Activation count always emitted in output for
  observability across CI runs. The attestation section heading MUST be '### <Pass-N>
  assertion-site attestation (<PARENT-SHA>)' where PARENT-SHA = git rev-parse HEAD^1 in the
  code repo at commit-preparation time. Same-commit bundling (TD-VSDD-053) retained.
  A voluntary '### <Pass-N> assertion-site stability-record (<PARENT-SHA>)' form is
  permitted for no-change commits (informational only; MUST NOT substitute for a real
  attestation on assertion-site commits). Non-vacuity controls (MANDATORY, project-wide gate
  rule — §Decision 10): a separate CI job 'attestation-gate-non-vacuity-controls' MUST run
  unconditionally; it verifies seven EICAR-style controls: pc-1 (crate *.rs, no log →
  FAIL: obligation violated), pc-2 (log present, no heading → FAIL: obligation violated),
  pass-zero (docs-only + non-rs/bats file committed inside crate dir → PASS-zero-activations
  exit 0), empty-range (crate in tree, MERGE_BASE=HEAD → EMPTY-or-UNREACHABLE: git range
  returned no commits exit 2), stale-pin (non-empty range, no crate in tree →
  EMPTY-or-UNREACHABLE: stale pin exit 2), empty-diff (crate in tree, --allow-empty commit
  → EMPTY-or-UNREACHABLE: unmeasurable diff exit 2), negative (correct heading →
  PASS-N-activations exit 0). Each control asserts the specific outcome identifier string
  (Requirement 4). All fixtures in mktemp tmpdir repos — no repo SHA dependency. Detection (adversary manual check): for each commit H
  in reviewed branch with *.rs/*.bats changes in
  crates/hook-plugins/validate-cross-site-correspondence: check pinned log absent at H →
  flag FAIL; log present, compute parent P = git rev-parse H^1; check git show H:<log-path>
  | grep -cE '^### .*assertion-site attestation (P)' → 1; if count ≠ 1, flag POLICY 15 HIGH
  with per-commit evidence. Do NOT run this check in factory-artifacts context."
```

---

## Alternatives Considered

**Alternative A (v1.0/v1.1) — Leave the predicate as-is and rely on adversarial review:**
Rejected. The predicate consumed a finding slot in every adversarial pass since D-912 and
produced zero convergence. This is the exact stable-self-perpetuating-finding class the
production-grade default forbids deferring.

**Alternative B (v1.0/v1.1) — Remove the gate entirely:** Rejected. D-912's goal
(preventing attestation content from landing in the wrong file or being omitted) is valid.
The fix is to make the gate satisfiable, not to remove it.

**Alternative C (v1.0/v1.1) — Use a content hash of `red-gate-log.md` instead of a SHA:**
Rejected. A content hash can be fabricated by appending any content; the SHA binding ties
the attestation to a specific point in Git history, which is more robust.

**Alternative D (v1.0/v1.1) — Use a timestamp-based heading and drop SHA binding:**
Rejected. Timestamps do not provide uniqueness guarantees without external oracle
dependencies. The parent SHA is available from Git natively.

**Alternative E (v1.2) — WASM hook plugin in factory-artifacts dispatcher chain:** Rejected.
The dispatcher runs in the factory-artifacts context; intercepting code-repo pushes requires
cross-repo path threading. Pushes issued directly from a shell bypass the dispatcher. WASM
fuel budget risks (D-965: four timeout failures) add false-positive blocking risk. See
§Rationale: "Why code-repo CI and not factory-artifacts dispatcher."

**Alternative F (v1.2) — Push-tip-only CI check (Option A from analysis):** Rejected as
insufficient. A push-tip check passes at `5370db80` (retroactive attestation present at tip)
while missing the per-commit violation at `67ffbdcc` and `38c70f9e`. The POLICY 15 obligation
says "at that commit" — per-commit iteration is required to enforce that binding. Push-tip is
acceptable only if the project accepts the retroactive-attestation window; it does not.

**Alternative G (v1.2) — Retire the obligation; rely on adversarial review with explicit
per-commit rubric:** Considered. The obligation is valid and Option D cost is low (~40-line CI
YAML). F-S2107-P10-002 proved that review without per-commit mechanical checking missed
retroactive attestation for 7 passes. Retiring mechanical enforcement when a correct, cheap
enforcement path is available is not production-grade. Rejected.

**Alternative H (v1.2) — Synthetic fixture commit in CI for negative control:** Rejected in
v1.2. NOTE: ADR-040 v1.3 (§Decision 10) ADOPTS synthetic EICAR-style fixtures, overriding
this rejection. The v1.2 rationale underweighted the orphaning risk: `67ffbdcc` is reachable
only from `origin/feature/S-21.07-...`; squash-merge orphans the SHA, invalidating the
fixture post-merge. The "requires git commit in CI" concern is addressed by the `_make_repo`
helper in §Decision 10's inline script (~10 lines of setup). See §Rationale "Why EICAR-style
synthetic fixture."

---

## Source / Origin

- **F-S2107-P8-003 (HIGH [process-gap]):** Adversarial pass-7 of S-21.07
  (reviewed-head `fbb5183c`) — POLICY 15 ATTESTATION-LOCATION GATE returns 0 at every
  HEAD because the predicate is logically unsatisfiable; the project has silently adopted
  a parent-SHA convention the literal predicate rejects; mutually exclusive with
  TD-VSDD-053. Addressed by §Decisions 1–5.
- **F-S2107-P9-003 (HIGH):** detection-scope defect and self-match vulnerability.
  Addressed by §Decision 6 (v1.1).
- **F-S2107-P10-001 (BLOCKER):** Gate is vacuous — pre-check runs in factory-artifacts
  worktree (0 `*.rs`/`*.bats` of 3743 files); gate permanently INAPPLICABLE; `find|head-1`
  over 14 candidates resolves to wrong log. Addressed by §Decisions 7 and 8 (v1.2).
- **F-S2107-P10-002 (BLOCKER [regression]):** Retroactive attestation at `5370db80`
  for `67ffbdcc`/`38c70f9e`; push-tip-only check cannot detect per-commit violations;
  permanent historical violation. Addressed by §Decision 9 (v1.2); historical violation
  recorded as erratum.
- **F-S2107-P10-003 (HIGH):** ADR-040 §Decision 6 justifying premise is false; `5370db80`
  was the attestation-provision commit, not a stability entry; D-965 ratification
  PROCURED-ON-MISCHARACTERIZATION. Addressed by §Decision 6 erratum + ADR re-opening (v1.2).
- **D-912:** Original codification of the ATTESTATION-LOCATION GATE; established the
  three goals this ADR preserves; extended POLICY 13 with mutant-derived-gate alternation
  mandate (unchanged).
- **TD-VSDD-053:** Single-commit-per-burst discipline; same-commit bundling retained.
- **D-449(a) / META-LEVEL-24:** Literal-shell-execution-evidence mandate; narrative
  attestation forbidden; gates must be mechanically invokable. Codification-2 (pass-10
  Part C): non-vacuity negative-control requirement for gates whose PASS is recorded as
  evidence. Addressed by §Decision 10 (v1.2).
- **POLICY 21:** No new `.sh` files. Addressed: gate is inline CI YAML `run:` block.

---

## Status

PROPOSED 2026-08-07; ADR-040 v1.0. AMENDED 2026-08-08; ADR-040 v1.1 (architect):
§Decision 6 added — detection-scope correction + self-match prevention + optional
stability-record heading form; policies.yaml Replacement Text v1.4.22. RATIFIED 2026-08-09
by human (D-965) — NOTE: ratification is now PROCURED-ON-MISCHARACTERIZATION per
F-S2107-P10-003; ADR re-opened to PROPOSED. REOPENED + AMENDED 2026-08-10; ADR-040 v1.2
(architect): §Decision 6 erratum (5370db80 false premise corrected); §Decisions 7–10 added
(execution-context relocation + deterministic path + per-commit iteration + negative control);
policies.yaml Replacement Text updated to v1.4.23. AMENDED 2026-08-10; ADR-040 v1.3
(architect): Gap A — §Decision 8 per-crate derived path + FAIL-when-absent + three-outcome
verdict; §Decision 9 script updated (merge-base, per-crate, three-outcome); Ruling 9(c)
expanded (unconditional job, fetch-depth rationale, merge-base, parentless detection).
Gap B — §Decision 10 EICAR-style synthetic fixture (three controls: positive-1, positive-2,
negative); job renamed attestation-gate-non-vacuity-controls; terminology corrected.
§Rationale: unconditional-job, vacuity-not-tautology, SLSA v1.2, EICAR rationale. §Decision
7 trigger corrected to unconditional. policies.yaml v1.4.23 text updated accordingly.
AMENDED 2026-08-10; ADR-040 v1.4 (architect): §Decision 8 unconditional obligation + path-
pinned (INAPPLICABLE retired; EMPTY-or-UNREACHABLE added); §Decision 9 script updated —
PLUGIN_CRATE constant, EMPTY-or-UNREACHABLE candidate check; §Decision 10 project-wide
three-outcome gate rule — four controls (positive-3 empty-range added); §Rationale: Beer
et al. CAV 1997, two-failure decomposition, in-toto REQUIRE-rule, Why path-pinned. §Status
v1.4 added. AMENDED 2026-08-10; ADR-040 v1.5 (architect): INAPPLICABLE/ERROR split —
§Decision 8 four-outcome table (FAIL/PASS/INAPPLICABLE/ERROR; INAPPLICABLE requires
confirmed non-empty diffs; ERROR on empty range or empty per-commit diff); §Decision 9
script rewritten with ALL_CHANGED per-commit emptiness check, ERROR counter, named
INAPPLICABLE exit with output; §Decision 7 Ruling 7(a) updated to reference INAPPLICABLE
and ERROR; §Decision 10 project-wide rule updated to four-outcome — five controls (added
inapplicable-control for docs-only fixture + INAPPLICABLE output verification); `_run_gate`
rewritten with ALL_CHANGED check and four-outcome output lines; §Rationale: Why four controls
section updated to five controls with INAPPLICABLE-reachability rationale; §Consequences
updated to five controls; §Proposed policies.yaml v1.4.23 text updated to four-outcome.
§Status v1.5 added. AMENDED 2026-08-10; ADR-040 v1.6 (architect): stale-pin guard +
PASS-zero-activations naming + EMPTY-or-UNREACHABLE consolidation — §Decision 8 verdict
table: INAPPLICABLE → PASS-zero-activations (named, observable, activation count emitted);
ERROR → EMPTY-or-UNREACHABLE (gains stale-pin as third trigger); stale-pin guard added (verify
PLUGIN_CRATE dir exists before range check; if absent → EMPTY-or-UNREACHABLE exit 2).
§Decision 9 script: stale-pin guard added before COMMITS check; EMPTY_DIFF counter replaces
ERROR; terminal output updated to PASS-zero-activations / PASS-N-activations /
EMPTY-or-UNREACHABLE. §Decision 7 Ruling 7(a) updated. §Decision 9 Ruling 9(c) item 1
updated. §Decision 10: project-wide rule updated — three requirements (four-outcome with
PASS-zero/EMPTY-or-UNREACHABLE; every-outcome-has-a-control generalization; stale-pin guard
for pinned targets); six controls (stale-pin control added); `_run_gate` gains stale-pin
guard and new output labels; controls table and script updated; §Rationale: Why six controls
(v1.6). §Consequences: five → six. §Proposed policies.yaml updated to v1.6 terminology.
§Status v1.6 added. AMENDED 2026-08-10; ADR-040 v1.7 (architect): three defects found by
executing the v1.6 controls — (1) `|| echo 0` double-output (CRITICAL): `grep -cE` exits 1
on no-match and prints `0`; `|| echo 0` appended a second `0`; two-line variable caused
`[ "$VAL" -ne 1 ]` to exit 2 (invalid integer), gate silently passed missing-attestation
commits; Positive Control 2 had the same bug; fixed `|| true` at all four CHANGED/COUNT
sites; (2) `git add --quiet` not a valid flag; controls harness aborted; removed from five
calls; (3) stale-pin guard used filesystem `[ ! -d ]` instead of git-tree check; replaced
with `git cat-file -e "HEAD:${PLUGIN_CRATE}"` at both sites. Ruling 10(e) added: empirical
execution record — controls caught the defect on first real run. §Status v1.7 added. AMENDED 2026-08-10; ADR-040 v1.8 (architect): evidence-block
reproducibility fix — §Decision 9 literal-shell evidence block used `|| echo 0` (third
occurrence of the double-output defect); stray `0` lines were hand-tidied before commit,
violating POLICY 15 verbatim-stdout and POLICY 5 HEAD-reproducibility; fixed with `|| true`;
captured output pasted verbatim (output unchanged — hand-tidied result happened to match
correct `|| true` output). §Rationale: "Three `|| echo 0` occurrences" added — three-site
propagation pattern, verbatim-capture as defect-detection, D-449(a) extension to evidence
blocks. §Decision 9 heading updated to AMENDED (v1.8). §Status v1.8 added. AMENDED 2026-08-10; ADR-040 v1.9 (architect): Defect 4 — R3/R4 fixtures
used `mkdir -p` to satisfy stale-pin guard, but `cat-file -e` requires a git-tree entry;
fixed by committing `placeholder.md` inside `${PLUGIN_CRATE}/` before guard check (R3: in
scenario commit; R4: in dedicated setup commit before HEAD4 capture). Defect 5 — controls
asserted exit codes only; FAIL=2 and EMPTY-or-UNREACHABLE=2 share an exit code, permitting
R4 to pass while exercising the wrong EMPTY path; fixed by capturing stdout and grepping for
specific outcome identifiers in all controls. New EMPTY-diff control added (seventh):
`--allow-empty` commit → `EMPTY-or-UNREACHABLE: unmeasurable diff`. §Decision 10 Requirement
4 (outcome-identifier assertion) added to project-wide rule. Ruling 10(b): seven-row table
with required-assertion column. Script inline comments updated v1.6→v1.9 (three sites).
§Consequences updated to seven controls. §Status v1.9 added. AMENDED 2026-08-10; ADR-040 v1.10 (architect): R3 fixture structural
fix — `BASE3` was captured before the seed commit (range incorrectly included the placeholder
file inside `${PLUGIN_CRATE}/`); fixed by committing seed first (`docs/.gitkeep`) then
capturing `BASE3`, so the evaluated range contains only the outer-docs commit. R4 comment
and seed file updated to `src/.gitkeep` / "seed crate path" convention. §Consequence added:
"Execution discipline — scripts in ADR prose are not auto-tested" — sixth-generation pattern;
long-term fix is migration to real CI files. §Status v1.10 added. AMENDED 2026-08-10;
ADR-040 v1.11 (architect): §Decisions 9 and 10 restructured — embedded bash scripts removed;
mechanism delegated to Rust crate `crates/policy15-attestation-gate/` (landing commit
`d2a3176a`, branch `feature/policy15-gate-rust`). §Decision 9 Ruling 9(a) rewritten as
reference to `run_gate`/`GateOutcome` enum (normative representation); CI invocation documented;
`lib.rs` module doc version lag (`v1.8`) flagged as cosmetic. §Decision 10 Ruling 10(b)
replaced with 16 Rust `#[test]` names (mutation-verified); Ruling 10(d) inline bash replaced
by one-line `cargo test -p policy15-attestation-gate` reference. §Rationale "Why six controls
(v1.6)" heading corrected to "Why seven controls (v1.9)"; body updated to cite `GateOutcome`
variants. §Consequences: "Execution discipline" updated — seventh generation (mutation testing
found coarse-assertion in Rust tests before crate landed; `matches!()` fix); v1.11 closes the
prose-ADR defect class. New §Consequence: "Mutation-testing acceptance criterion" added.
§Proposed policies.yaml Replacement Text version chain updated to v1.11. Implementation
routing updated to reference Rust binary + `cargo test`. §Status v1.11 added. AMENDED 2026-08-10; ADR-040 v1.12 (architect): corrects two factual
errors in the v1.11 seventh-generation record — (1) generation 7 was found by mutation B
(neutralising `run_gate` guard 1), surviving test was `test_unresolvable_base_fails_closed`
(`!outcome.is_pass()`), not `test_positive_2_no_attestation_heading`; (2) generation 7 was
found after the crate landed at `d5a90e74`, not before. §Consequence "Execution discipline"
generation 7 bullet rewritten with accurate step-by-step account. §Consequence
"Mutation-testing acceptance criterion" rewritten: generation 7 case study (steps 1–5),
coarse-assertion-gap vs mutant-killer distinction, generation 5/7 altitude symmetry, variant-
specific `matches!()` example. §Status v1.12 added.

**HUMAN RE-RATIFICATION REQUIRED** before `policies.yaml` v1.4.23 is applied.
- The v1.4.22 text is PROCURED-ON-MISCHARACTERIZATION; it remains in force until
  re-ratification. Agents MUST NOT apply v1.4.23 before a D-NNN re-ratification row.
- The exact replacement text for v1.4.23 is in §Proposed `policies.yaml` Replacement Text.
- No `policies.yaml` edit has been made in this ADR v1.4 burst.

Adjudicates (cumulative):
- F-S2107-P8-003 — HEAD-SHA predicate logically unsatisfiable (§Decisions 1–5, v1.0).
- F-S2107-P9-003 — detection-scope + self-match (§Decision 6, v1.1).
- F-S2107-P10-001 — gate vacuous: wrong repo context + wrong log (§Decisions 7–8, v1.2).
- F-S2107-P10-002 — retroactive attestation window (§Decision 9, v1.2). Historical
  violation at `67ffbdcc`/`38c70f9e` is permanent; no history rewrite. An erratum note
  MUST be added to `red-gate-log.md` documenting this permanent gap, and D-965's row in
  the decision-log annotated as PROCURED-ON-MISCHARACTERIZATION.
- F-S2107-P10-003 — false premise in §Decision 6 (§Decision 6 erratum, v1.2).

Supersedes the POLICY 15 ATTESTATION-LOCATION GATE clause added at D-912. D-912's POLICY 13
extension (mutant-derived-gate alternation mandate) is unaffected and unchanged. Supersession
scope is confirmed: supersede only the ATTESTATION-LOCATION GATE clause; D-912's POLICY 13
extension is not in scope.

Implementation routing (after human re-ratification of v1.11):
- **devops-engineer:** Add two jobs to `.github/workflows/ci.yml` (or a dedicated
  `policy-15-attestation.yml`): (1) `policy-15-attestation-location` — required check,
  unconditional (no `paths:` filter), `fetch-depth: 0`, invokes
  `policy15-attestation-gate [<base-branch>]` per §Decision 9 Ruling 9(a); (2)
  `attestation-gate-non-vacuity-controls` — unconditional, invokes
  `cargo test -p policy15-attestation-gate` per §Decision 10 Ruling 10(d). Both jobs comply
  with POLICY 21 (no new `.sh` files; Rust binary + cargo test).
- **state-manager:** Edit `policies.yaml` POLICY 15 ATTESTATION-LOCATION GATE bullet per
  §Proposed `policies.yaml` Replacement Text; bump version v1.4.22 → v1.4.23; advance
  ARCH-INDEX frontmatter if needed (total_adrs unchanged — ADR-040 v1.4 is an amendment).
- **state-manager:** Add erratum note to
  `crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md`
  documenting the permanent F-S2107-P10-002 historical violation at `67ffbdcc`/`38c70f9e`.
- **state-manager:** Annotate D-965 row in decision-log as PROCURED-ON-MISCHARACTERIZATION
  with forward pointer to the D-NNN v1.2 re-ratification row.
- **state-manager:** For future assertion-site commits in the code repo, include
  `### Pass-N assertion-site attestation (<PARENT-SHA>)` in the same commit, where
  PARENT-SHA is `git rev-parse HEAD^1` in the code repo before staging. The §Decision 6
  factory-artifacts attestation convention is retired; all new attestations belong in the
  code repo alongside the assertion-site change.
