---
document_type: architecture-decision-record
level: L3
adr_id: ADR-040
version: "1.1"
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
  2026-08-08 (v1.1) — AMENDED (architect): §Decision 6 added — detection-scope
  correction (obligation was commit-class-conditional; detection clause was
  unconditional — contradiction; resolved by propagating applicability condition into
  detection) + self-match prevention (line-anchored predicate ^### .*) + optional
  stability-record heading form; §Proposed policies.yaml Replacement Text updated to
  v1.4.22. Triggered by F-S2107-P9-003 close revealing the scope mismatch in practice.
  [Prior: 2026-08-07 (v1.0) — Initial ruling (architect; F-S2107-P8-003 impossibility
  diagnosis; S-21.07 pass-7 fix wave design): self-referential SHA predicate in POLICY 15
  ATTESTATION-LOCATION GATE (D-912) is logically unsatisfiable; parent-SHA predicate
  preserves all three of D-912's original goals while removing the impossibility.
  policies.yaml NOT yet edited — human ratification required per §Status.
  ADR-040 PROPOSED 2026-08-07.]
modified:
  - "2026-08-07 (v1.0)"
  - "2026-08-08 (v1.1)"
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
`"ATTESTATION-LOCATION GATE (v1.4.20;...)` (the v1.4.20 text applied from ADR-040 v1.0;
v1.4.21 applied ADR-041 POLICY 16 ceiling gate).

**Version after replacement: v1.4.22.** (v1.4.20 = ADR-040 §Decision 2 parent-SHA form;
v1.4.21 = ADR-041 POLICY 16; v1.4.22 = ADR-040 §Decision 6 conditional detection +
line-anchored predicate.)

```
- "ATTESTATION-LOCATION GATE (v1.4.22; ADR-040 §Decision 6; amends v1.4.20 detection
  clause; F-S2107-P9-003 scope-mismatch + self-match resolution): a fix wave that adds
  or strengthens any bats assertion site MUST NOT be pushed until the matching
  red-gate-log.md attestation section EXISTS at that commit. Pre-check (commit-class
  conditional): `git diff --name-only HEAD^1 HEAD -- '*.rs' '*.bats'`; if EMPTY, gate is
  INAPPLICABLE for this commit — no attestation required, no flag raised. If NON-EMPTY
  (assertion-site files changed), literal shell check: `PARENT=$(git -C
  <factory-worktree-root> rev-parse HEAD^1) && grep -cE '^### .*assertion-site
  attestation \\($PARENT\\)' <red-gate-log-path>` → 1 (line-anchored to prevent false
  count from prose that quotes a heading string; PARENT = parent commit SHA, not HEAD
  itself, removing the D-912 circular dependency). If the count is 0 or ≠ 1, the push is
  BLOCKED until the state-manager appends the attestation section and bundles it in the
  same commit per TD-VSDD-053. The attestation section heading MUST be `### <Pass-N>
  assertion-site attestation (<PARENT-SHA>)` (where PARENT-SHA = git rev-parse HEAD^1 in
  the factory-artifacts worktree) so the check is SHA-bound and cannot be satisfied by a
  prior pass's section — parent SHAs advance monotonically in Git history. A voluntary
  `### <Pass-N> assertion-site stability-record (<PARENT-SHA>)` heading form is permitted
  for no-change commits (informational only; does NOT contain 'attestation'; does NOT
  satisfy the gate; MUST NOT substitute for a real attestation on commits with
  assertion-site changes). Detection: adversary (a) runs `git diff --name-only H^1 H --
  '*.rs' '*.bats'`; if empty, record 'POLICY 15 INAPPLICABLE at <H>' and skip; (b) if
  non-empty, compute parent P = git rev-parse H^1 and run `grep -cE '^### .*assertion-site
  attestation \\(P\\)'` against red-gate-log.md; if count ≠ 1, flag POLICY 15 HIGH. Root
  cause of D-912 impossibility (F-S2107-P8-003): HEAD SHA is circular; PARENT SHA is
  available before commit finalization. Same-commit bundling (TD-VSDD-053) retained:
  assertion-site changes and attestation section land in one commit. The gate MUST be
  invoked as literal shell with captured stdout per D-449(a)/META-LEVEL-24; narrative
  attestation is forbidden."
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

PROPOSED 2026-08-07; ADR-040 v1.0. AMENDED 2026-08-08; ADR-040 v1.1 (architect):
§Decision 6 added — detection-scope correction (obligation was commit-class-conditional;
detection was unconditional — contradiction resolved by propagating applicability condition
into detection) + self-match prevention (line-anchored predicate `^### .*`) + optional
stability-record heading form; §Proposed `policies.yaml` Replacement Text updated to v1.4.22.

**HUMAN RATIFICATION REQUIRED** before `policies.yaml` is edited. The exact replacement
text is in §Proposed `policies.yaml` Replacement Text (v1.4.22). No `policies.yaml` edit has
been made in this burst.

Adjudicates:
- F-S2107-P8-003 (HIGH [process-gap]) from adversarial pass-7 of S-21.07 — POLICY 15
  ATTESTATION-LOCATION GATE HEAD-SHA predicate is logically unsatisfiable (§Decisions 1–5).
- F-S2107-P9-003 (detection-scope defect) — obligation vs detection contradiction and
  self-match vulnerability (§Decision 6).

Supersedes the POLICY 15 ATTESTATION-LOCATION GATE clause added at D-912. D-912's
POLICY 13 extension (mutant-derived-gate alternation mandate) is unaffected.
**Supersession scope: pending human confirmation.** This scoping (supersede only the
ATTESTATION-LOCATION GATE clause; leave D-912's POLICY 13 extension intact) is proposed
by the architect and surfaced to the human alongside the policies.yaml ratification
request. If the human rules differently, the supersedes: frontmatter and this Status
section are the only artifacts requiring amendment before state-manager codifies the D-NNN.

Implementation routing (after human ratification):
- **state-manager:** Edit `policies.yaml` POLICY 15 ATTESTATION-LOCATION GATE bullet per
  ADR-040 §Decision 6 and the exact text in §Proposed `policies.yaml` Replacement Text;
  bump `policies.yaml` version v1.4.21 → v1.4.22; advance ARCH-INDEX frontmatter if
  needed (total_adrs unchanged — ADR-040 v1.1 is an amendment, not a new ADR).
- **state-manager:** In subsequent fix waves with assertion-site changes, include
  `### Pass-N assertion-site attestation (<PARENT-SHA>)` where PARENT-SHA is obtained via
  `git -C <factory-worktree-root> rev-parse HEAD^1` before staging; invoke the §Decision 6
  gate script at Commit-E with captured stdout per D-449(a).
- **state-manager:** For fix waves without assertion-site changes, no attestation is
  required; optional `### Pass-N assertion-site stability-record (<PARENT-SHA>)` may be
  included at author's discretion per Ruling 6(c).
