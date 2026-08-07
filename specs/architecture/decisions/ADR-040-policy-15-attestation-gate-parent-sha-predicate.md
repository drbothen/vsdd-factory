---
document_type: architecture-decision-record
level: L3
adr_id: ADR-040
version: "1.0"
title: "ADR-040: POLICY 15 ATTESTATION-LOCATION GATE — parent-SHA predicate replaces self-referential HEAD-SHA (resolves F-S2107-P8-003 logical impossibility)"
status: proposed
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
  2026-08-07 (v1.0) — Initial ruling (architect; F-S2107-P8-003 impossibility diagnosis;
  S-21.07 pass-7 fix wave design): self-referential SHA predicate in POLICY 15
  ATTESTATION-LOCATION GATE (D-912) is logically unsatisfiable; parent-SHA predicate
  preserves all three of D-912's original goals while removing the impossibility.
  policies.yaml NOT yet edited — human ratification required per §Status.
  ADR-040 PROPOSED 2026-08-07.
modified:
  - "2026-08-07 (v1.0)"
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

This ADR makes five rulings covering the predicate replacement, heading format, TD-VSDD-053
interaction, mechanization verdict, and sibling-sweep finding.

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
MUST be invoked as a literal shell command with captured stdout at Commit-E to prevent
recurrence of the same class.

---

## Consequences

### Positive

- The ATTESTATION-LOCATION GATE becomes satisfiable for the first time since D-912.
  Every fix wave that correctly applies ADR-040 §Decision 2 will see count = 1 on the
  first attempt, rather than count = 0 at every attempt.
- Same-commit bundling is restored for the attestation section, closing the split-commit
  window that has been accepted as an unofficial workaround.
- The gate is fully mechanizable as a hook or CI step per Decision 4, enabling automated
  enforcement rather than relying on adversarial review to catch violations.
- Pass-7's finding F-S2107-P8-003 is closed: the predicate that produced a stable
  self-perpetuating finding consuming one slot each pass without converging is replaced.
- TD-VSDD-059 (paper-fix detection) is respected: the fix is structural (predicate
  replacement), not nominal (rename / doc-comment only).

### Negative / Trade-offs

- The heading format changes from `### Pass-N assertion-site attestation (<HEAD-SHA>)` to
  `### Pass-N assertion-site attestation (<PARENT-SHA>)`. Existing attestation sections
  in `red-gate-log.md` that use the HEAD-SHA form are historical records; they do not need
  to be retroactively corrected. The new predicate only checks the current push's parent
  SHA, not historical sections.
- An agent unfamiliar with the change may author a section with the old HEAD-SHA form.
  Mitigation: the POLICY 15 text must be updated atomically with ratification; the ADR
  text is the normative source until then.

### Status as of v1.0

PROPOSED 2026-08-07. **Human ratification required before `policies.yaml` is edited.**
The ADR file and ARCH-INDEX body row are the full extent of artifacts written in this
burst per the team-lead's explicit constraint ("write the ADR, leave `policies.yaml`
untouched until the human rules").

The exact replacement text for POLICY 15's ATTESTATION-LOCATION GATE bullet is
provided verbatim in §Proposed `policies.yaml` Replacement Text below.

---

## Proposed `policies.yaml` Replacement Text

The following is the exact string to replace the current ATTESTATION-LOCATION GATE bullet
in POLICY 15's `verification_steps` array. It replaces the entire bullet starting with
`"ATTESTATION-LOCATION GATE (v1.4.11;...)`.

**Version after replacement: v1.4.20.**

```
- "ATTESTATION-LOCATION GATE (v1.4.20; ADR-040 §Decision 2; supersedes D-912 HEAD-SHA
  form; F-S2107-P8-003 resolution): a fix wave that adds or strengthens any bats
  assertion site MUST NOT be pushed until the matching red-gate-log.md attestation
  section EXISTS at that commit. Literal shell check: `PARENT=$(git -C
  <factory-worktree-root> rev-parse HEAD^1) && grep -c \"assertion-site attestation
  ($PARENT)\" <red-gate-log-path>` → 1 (where HEAD^1 is the parent commit of the
  commit being pushed, known at authoring time before the commit is finalized). If the
  count is 0, the push is BLOCKED until the state-manager appends the attestation section
  and bundles it in the same commit per TD-VSDD-053. The attestation section heading MUST
  be `### <Pass-N> assertion-site attestation (<PARENT-SHA>)` (where PARENT-SHA =
  git rev-parse HEAD^1 in the factory-artifacts worktree) so the check is SHA-bound and
  cannot be satisfied by a prior pass's section — parent SHAs advance monotonically in
  Git history and each fix burst's parent commit is distinct from all prior fix bursts'
  parent commits. Detection: adversary verifies that for the reviewed_head SHA H, the
  parent SHA P = git rev-parse H^1 is computed, and a matching `### *attestation (P)`
  section (P full 40-char or any unambiguous prefix) exists in red-gate-log.md; if
  absent, flag as POLICY 15 HIGH (attestation-location violation). Root cause of D-912
  impossibility (F-S2107-P8-003): HEAD SHA = hash(tree_including_attestation_section) —
  circular and computationally infeasible to satisfy without brute-force SHA mining;
  PARENT SHA = hash(prior-commit) — not circular, available before the current commit is
  finalized. Same-commit bundling (TD-VSDD-053) is retained: assertion-site changes and
  attestation section land in one commit. The gate MUST be invoked as a literal shell
  command with captured stdout per D-449(a)/META-LEVEL-24; narrative attestation is
  forbidden."
```

---

## Alternatives Considered

**Alternative A — Leave the predicate as-is and rely on adversarial review:** Rejected.
The predicate has consumed a finding slot in every adversarial pass since D-912 and has
produced zero convergence. This is the exact stable-self-perpetuating-finding class the
production-grade default forbids deferring.

**Alternative B — Remove the gate entirely:** Rejected. D-912's goal (preventing
attestation content from landing in the wrong file or being omitted) is valid. The fix is
to make the gate satisfiable, not to remove it.

**Alternative C — Use a content hash of `red-gate-log.md` instead of a SHA:** Rejected.
A content hash can be fabricated by appending any content; the SHA binding ties the
attestation to a specific point in Git history, which is more robust.

**Alternative D — Use a timestamp-based heading (e.g., `### Pass-N attestation
2026-08-07`) and drop SHA binding:** Rejected. Timestamps do not provide uniqueness
guarantees without external oracle dependencies. The parent SHA is available from Git
natively, is globally unique within the repository, and requires no external service.

---

## Source / Origin

- **F-S2107-P8-003 (HIGH [process-gap]):** Adversarial pass-7 of S-21.07
  (reviewed-head `fbb5183c`) — POLICY 15 ATTESTATION-LOCATION GATE returns 0 at every
  HEAD because the predicate is logically unsatisfiable; the project has silently adopted
  a parent-SHA convention the literal predicate rejects; mutually exclusive with
  TD-VSDD-053.
- **D-912:** Original codification of the ATTESTATION-LOCATION GATE; established the
  three goals this ADR preserves; extended POLICY 13 with mutant-derived-gate alternation
  mandate (unchanged).
- **TD-VSDD-053:** Single-commit-per-burst discipline; same-commit bundling retained.
- **D-449(a) / META-LEVEL-24:** Literal-shell-execution-evidence mandate; narrative
  attestation forbidden; gates must be mechanically invokable.

---

## Status

PROPOSED 2026-08-07; ADR-040 v1.0.

**HUMAN RATIFICATION REQUIRED** before `policies.yaml` is edited. The exact replacement
text is in §Proposed `policies.yaml` Replacement Text. No `policies.yaml` edit has been
made in this burst.

Adjudicates F-S2107-P8-003 (HIGH [process-gap]) from adversarial pass-7 of S-21.07.

Supersedes the POLICY 15 ATTESTATION-LOCATION GATE clause added at D-912. D-912's
POLICY 13 extension (mutant-derived-gate alternation mandate) is unaffected.
**Supersession scope: pending human confirmation.** This scoping (supersede only the
ATTESTATION-LOCATION GATE clause; leave D-912's POLICY 13 extension intact) is proposed
by the architect and surfaced to the human alongside the policies.yaml ratification
request. If the human rules differently, the supersedes: frontmatter and this Status
section are the only artifacts requiring amendment before state-manager codifies the D-NNN.

Implementation routing (after human ratification):
- **state-manager:** Edit `policies.yaml` POLICY 15 ATTESTATION-LOCATION GATE bullet per
  ADR-040 §Decision 2 and the exact text in §Proposed `policies.yaml` Replacement Text;
  bump `policies.yaml` version v1.4.19 → v1.4.20; advance ARCH-INDEX frontmatter
  `total_adrs 39 → 40` and version row.
- **state-manager:** In subsequent fix waves with assertion-site changes, include
  `### Pass-N assertion-site attestation (<PARENT-SHA>)` where PARENT-SHA is obtained via
  `git -C <factory-worktree-root> rev-parse HEAD^1` before staging; invoke gate at
  Commit-E with captured stdout per D-449(a).
