---
document_type: adr
adr_id: ADR-041
version: "1.2"
status: active
ratified: "2026-08-13"
ratification_note: "Human ratification 2026-08-13 (S-21.07 pass-10 fix burst, D-992): the D-960 SRC ruling ('migrate the sentinel, not merely reserve it') already authorized this ADR's substance; the sentinel migration and POLICY 16 ALLOCATOR-CEILING GATE (§Decision 3) have been running as a live blocking pre-allocation gate since D-961 with zero adverse findings across 10+ bursts. This ratification closes the F-S2107-P10-005 ADR-041 leg — the frontmatter status/decision-log-narrative gap the v1.1 §Status section flagged NEEDS-HUMAN. Ratified alongside ADR-042 in the same human decision, per v1.1 §Status recommendation (c)."
date: 2026-08-07
last_amended: "2026-08-13 (v1.2) — AMENDED (architect; S-21.07 pass-10 fix cascade, BODY-
  vs-FRONTMATTER reconciliation pass, pre-adversary-pass-11): §Status section's
  'RATIFICATION STATUS — NEEDS HUMAN ADJUDICATION (F-S2107-P10-005)' paragraph and
  'Recommendation to human' paragraph reconciled to present-tense — human ratification
  OCCURRED 2026-08-13 (D-992; see frontmatter `status: active` / `ratified: '2026-08-13'` /
  `ratification_note` above). The NEEDS-HUMAN framing is now historical, not current; the
  §Status prose is corrected in place (superseded, not deleted) so body agrees with
  frontmatter. No change to §Decision 1-4 substance.
  [Prior: 2026-08-13 (v1.1) — AMENDED (architect; S-21.07 pass-10 ADR-anchored fix
  cascade, closes F-S2107-P10-005 ADR-041 leg): added `version`/`last_amended`/`modified`
  frontmatter fields (absent since v1.0 authoring — this ADR predates the version-tracking
  convention other ADR-04x documents use); §Consequences 'Status as of 2026-08-07' subsection
  corrected — it read 'Proposed... implementation is gated on this ADR's acceptance,' which
  is stale: decision-log.md D-961(a)/(b) confirm the sentinel migration (product-owner,
  story-writer, test-writer, architect legs) was already executed 2026-08-07, the same date
  this subsection was authored and never revisited; new '## Status' section added
  documenting the frontmatter `status: proposed`/no-`ratified:`-field gap against the
  decision-log's 'ADR-041 ratified' narrative language, flagged NEEDS-HUMAN — architect does
  not claim unilateral ratification authority (ADR-040/D-965 precedent: narrative 'ratified'
  language in a burst summary is not equivalent to a genuine, dated human ratification event).
  No change to §Decision 1-4 substance (sentinel value, ceiling, gate predicate, authoring
  discipline) — this amendment is documentary/status-tracking only.]"
modified:
  - "2026-08-07 (v1.0)"
  - "2026-08-13 (v1.1)"
  - "2026-08-13 (v1.2)"
subsystems_affected:
  - SS-05
  - SS-01
supersedes: null
superseded_by: null
---

# ADR-041: D-999 sentinel migration — allocator ceiling gate and D-99999 replacement sentinel

## Context

The D-NNN identifier namespace is global across all cycles (POLICY 16). The allocator is
monotonically increasing: each fix burst consumes one ID and state-manager pre-burst runs
`grep "^## D-"` across all `decision-log.md` files to find the current global max, then
allocates `max + 1`. As of commit 46b7cef2 (D-960), the global max is **D-960**.

`D-999` is used as a **sentinel value** — a magic in-band identifier meaning "correctly
formatted D-NNN citation that does not exist in the decision-log." It appears as a fixture
value in two normative artifact locations:

1. `BC-5.39.007 EC-010` — edge case row for "Closes line reads `**Closes:** D-999` where
   D-999 does not exist in decision-log.md": expected behavior `HookResult::Continue` with
   advisory log (Phase 1 only; not a block).
2. `S-15.12 AC-18` — acceptance criterion for "cross-site staleness produces advisory log
   only — NOT a block": fixture `**Closes:** D-999` where D-999 nonexistent → pass + log_warn.

When the allocator reaches D-999 (approximately 39 decisions from D-960 at current
allocation rate), two failure modes activate simultaneously:

- **Silent semantic inversion**: The bats test `pass-phase1-advisory-only.bats` would still
  pass numerically, but it would no longer be testing "correctly formatted but nonexistent
  D-NNN" — it would be testing "existing D-NNN." The AC description and EC row would
  describe the wrong scenario. The test would pass for the wrong reason.
- **Prose-reservation fragility (D-960(e))**: D-960(e) reserved D-999 as "never-allocatable"
  via narrative prose in the decision-log. This is exactly the failure class the cycle keeps
  rediscovering (cf. L-EDP1 lessons on narrative-attested gates and META-LEVEL-24): prose
  discipline cannot detect its own scope-degradation. The POLICY 16 global-max grep would
  return 999 if the allocator counted to 999, and state-manager would have no mechanical
  signal to stop.

D-960(e) is therefore insufficient. The human ruling is: **MIGRATE the sentinel** (not merely
reserve the current value via prose).

An additional constraint is that `validate-dispatch-advance`'s `scan_max_d_nnn` function
scans all `D-\d+` tokens in STATE.md content (not only decision-log headers) to compute the
D-chain currency max. If STATE.md narrative embeds the sentinel value as a bare D-NNN token,
`scan_max_d_nnn` would return the sentinel integer as `max_in_file`, requiring `current_step:`
to cite the sentinel or a higher integer — an undesirable constraint on state-manager authoring.
The authoring discipline in Decision §4 addresses this.

## Decision

**Decision 1 — Replace sentinel value D-999 with D-99999.**

D-99999 is the canonical sentinel for all "correctly formatted but nonexistent D-NNN" test
fixtures going forward. It is a valid `D-\d+` format, which preserves the test scenario
semantics (Phase 1 advisory-only boundary for correctly-formatted-but-nonexistent citations).
All sites listed in §Migration Sequence that cite D-999 as a fixture sentinel MUST be
migrated to D-99999 by the dispatched specialists (product-owner, story-writer, test-writer).

**Decision 2 — Establish allocator ceiling gate at D-8999.**

The maximum allocatable D-NNN is D-8999. D-9000 through D-99999 are the reserved sentinel
range. The mechanical gate (Decision 3) fires if the global max allocation reaches or exceeds
D-9000, alerting state-manager well before the sentinel value D-99999 is reachable.

This ceiling provides approximately 8,039 additional allocations from D-960 before the gate
fires — ample for any foreseeable project lifespan at the observed allocation rate. The
D-9000-to-D-99999 buffer (90,000 values) means even a gate-bypass incident would not
immediately collide with the sentinel.

**Decision 3 — Mechanical gate via structural-anchor scan (all forms).**

The gate predicate inspects line-anchored structural forms in all cycle decision-logs — h2
headers (`^#{2,} D-NNN`), h3 headers (`^#{2,} D-NNN`), and leading table-cell rows
(`^[|] *D-NNN`) — covering every format currently used across all cycles. It explicitly
excludes prose mentions, avoiding false-positive fires from narrative references to D-99999
(e.g., in ADR body text or BC descriptions that contain D-99999 in prose paragraphs).

**Why three forms, not just `^## D-`:**

Corpus analysis (2026-08-07) confirms the two-cycle corpus uses three structural forms with
distinct per-cycle distributions:

| Form | F5 cycle max | brownfield cycle max |
|------|-------------|----------------------|
| `^#{2,} D-` (h2 and h3 headings) | D-454 (h3) | D-960 (h2), D-900 (h3) |
| `^[pipe-char] *D-` (table-cell leading) | D-454 | D-731 |

The original `^## D-` predicate (exactly two hashes) returns **zero** matches from the F5
cycle and misses all h3 entries (26 existing entries, brownfield max D-900). Both defects are
currently harmless — brownfield's D-960 h2 entries hold the global max — but structurally
fail-open: if the brownfield cycle closes and a successor cycle adopts table-form exclusively
(as F5 already does), `^## D-` returns empty, triggering the `max_d=0` default and producing
`PASS: global max D-0 < D-9000` — permanently green while blind.

**Fail-closed on empty:** If the corpus scan returns no matching lines, that signals either
a broken predicate or a missing decision-log path — not an empty allocation space. The gate
exits non-zero with a distinct error message.

Literal-shell gate predicate (D-449(a) compliant — invocable via literal shell with captured stdout):

```bash
max_d=$(
  {
    grep -hE '^#{2,} D-[0-9]+' \
      /Users/zious/Documents/GITHUB/vsdd-factory/.factory/cycles/*/decision-log.md 2>/dev/null
    grep -hE '^[|] *D-[0-9]+' \
      /Users/zious/Documents/GITHUB/vsdd-factory/.factory/cycles/*/decision-log.md 2>/dev/null
  } | grep -oE 'D-[0-9]+' | sed 's/D-//' | sort -n | tail -1
)
if [ -z "$max_d" ]; then
  printf 'FAIL: D-NNN ceiling gate: corpus scan found zero allocated D-NNN entries across all structural forms — decision-log path missing or predicate broken; gate fails closed\n'
  exit 1
fi
[ "$max_d" -lt 9000 ] \
  && printf 'PASS: global max D-%s < D-9000 ceiling (forms: h2 ^#{2,}, h3 ^#{2,}, table-cell ^[|])\n' "$max_d" \
  || { printf 'FAIL: D-NNN allocation ceiling breach: max=D-%s exceeds D-8999 maximum allocatable\n' "$max_d"; exit 1; }
```

**Live corpus run — D-449(a) captured stdout (run 2026-08-07):**

```
$ max_d=$({ grep -hE '^#{2,} D-[0-9]+' /Users/zious/Documents/GITHUB/vsdd-factory/.factory/cycles/*/decision-log.md 2>/dev/null; grep -hE '^[|] *D-[0-9]+' /Users/zious/Documents/GITHUB/vsdd-factory/.factory/cycles/*/decision-log.md 2>/dev/null; } | grep -oE 'D-[0-9]+' | sed 's/D-//' | sort -n | tail -1) && if [ -z "$max_d" ]; then printf 'FAIL: D-NNN ceiling gate: corpus scan found zero allocated D-NNN entries across all structural forms — decision-log path missing or predicate broken; gate fails closed\n'; exit 1; fi && [ "$max_d" -lt 9000 ] && printf 'PASS: global max D-%s < D-9000 ceiling (forms: h2 ^#{2,}, h3 ^#{2,}, table-cell ^[|])\n' "$max_d" || { printf 'FAIL: D-NNN allocation ceiling breach: max=D-%s exceeds D-8999 maximum allocatable\n' "$max_d"; exit 1; }
PASS: global max D-960 < D-9000 ceiling (forms: h2 ^#{2,}, h3 ^#{2,}, table-cell ^[|])
```

**Self-test evidence — gate fires FAIL on synthetic breach for all three forms (D-449(a) captured stdout):**

Self-test 1: h2 form (`## D-9200` — over-ceiling h2 header):

```
$ printf '## D-9200 Some decision title\n' > /tmp/st1-adr041.md && max_d=$({ grep -hE '^#{2,} D-[0-9]+' /tmp/st1-adr041.md 2>/dev/null; grep -hE '^[|] *D-[0-9]+' /tmp/st1-adr041.md 2>/dev/null; } | grep -oE 'D-[0-9]+' | sed 's/D-//' | sort -n | tail -1) && if [ -z "$max_d" ]; then printf 'FAIL: D-NNN ceiling gate: corpus scan found zero allocated D-NNN entries across all structural forms — decision-log path missing or predicate broken; gate fails closed\n'; else [ "$max_d" -lt 9000 ] && printf 'PASS: global max D-%s < D-9000 ceiling (forms: h2 ^#{2,}, h3 ^#{2,}, table-cell ^[|])\n' "$max_d" || printf 'FAIL: D-NNN allocation ceiling breach: max=D-%s exceeds D-8999 maximum allocatable\n' "$max_d"; fi
FAIL: D-NNN allocation ceiling breach: max=D-9200 exceeds D-8999 maximum allocatable
```

Self-test 2: h3 form (`### D-9200` — over-ceiling h3 header):

```
$ printf '### D-9200 Some sub-decision title\n' > /tmp/st2-adr041.md && max_d=$({ grep -hE '^#{2,} D-[0-9]+' /tmp/st2-adr041.md 2>/dev/null; grep -hE '^[|] *D-[0-9]+' /tmp/st2-adr041.md 2>/dev/null; } | grep -oE 'D-[0-9]+' | sed 's/D-//' | sort -n | tail -1) && if [ -z "$max_d" ]; then printf 'FAIL: D-NNN ceiling gate: corpus scan found zero allocated D-NNN entries across all structural forms — decision-log path missing or predicate broken; gate fails closed\n'; else [ "$max_d" -lt 9000 ] && printf 'PASS: global max D-%s < D-9000 ceiling (forms: h2 ^#{2,}, h3 ^#{2,}, table-cell ^[|])\n' "$max_d" || printf 'FAIL: D-NNN allocation ceiling breach: max=D-%s exceeds D-8999 maximum allocatable\n' "$max_d"; fi
FAIL: D-NNN allocation ceiling breach: max=D-9200 exceeds D-8999 maximum allocatable
```

Self-test 3: table-cell form (`| D-9200 |` — over-ceiling table-cell row):

```
$ printf '| D-9200 | Some decision title | SS-01 | path/to/adr.md |\n' > /tmp/st3-adr041.md && max_d=$({ grep -hE '^#{2,} D-[0-9]+' /tmp/st3-adr041.md 2>/dev/null; grep -hE '^[|] *D-[0-9]+' /tmp/st3-adr041.md 2>/dev/null; } | grep -oE 'D-[0-9]+' | sed 's/D-//' | sort -n | tail -1) && if [ -z "$max_d" ]; then printf 'FAIL: D-NNN ceiling gate: corpus scan found zero allocated D-NNN entries across all structural forms — decision-log path missing or predicate broken; gate fails closed\n'; else [ "$max_d" -lt 9000 ] && printf 'PASS: global max D-%s < D-9000 ceiling (forms: h2 ^#{2,}, h3 ^#{2,}, table-cell ^[|])\n' "$max_d" || printf 'FAIL: D-NNN allocation ceiling breach: max=D-%s exceeds D-8999 maximum allocatable\n' "$max_d"; fi
FAIL: D-NNN allocation ceiling breach: max=D-9200 exceeds D-8999 maximum allocatable
```

Self-test 4: empty corpus (no decision-log files — fail-closed branch):

```
$ TMPDIR=$(mktemp -d) && max_d=$({ grep -hE '^#{2,} D-[0-9]+' "$TMPDIR"/*/decision-log.md 2>/dev/null; grep -hE '^[|] *D-[0-9]+' "$TMPDIR"/*/decision-log.md 2>/dev/null; } | grep -oE 'D-[0-9]+' | sed 's/D-//' | sort -n | tail -1) && if [ -z "$max_d" ]; then printf 'FAIL: D-NNN ceiling gate: corpus scan found zero allocated D-NNN entries across all structural forms — decision-log path missing or predicate broken; gate fails closed\n'; else [ "$max_d" -lt 9000 ] && printf 'PASS: global max D-%s < D-9000 ceiling (forms: h2 ^#{2,}, h3 ^#{2,}, table-cell ^[|])\n' "$max_d" || printf 'FAIL: D-NNN allocation ceiling breach: max=D-%s exceeds D-8999 maximum allocatable\n' "$max_d"; fi; rm -rf "$TMPDIR"
FAIL: D-NNN ceiling gate: corpus scan found zero allocated D-NNN entries across all structural forms — decision-log path missing or predicate broken; gate fails closed
```

State-manager MUST run this predicate (or the functional equivalent registered in POLICY 16)
before each burst's D-NNN allocation step. A non-zero exit blocks allocation.

**Decision 4 — STATE.md authoring discipline for sentinel references.**

STATE.md narrative that describes the sentinel migration MUST NOT embed D-99999 as a bare
parseable D-NNN token (e.g., must not write "sentinel D-99999" directly in STATE.md body or
`current_step:`). Instead, use "per ADR-041" or "sentinel value documented in ADR-041."
Rationale: `scan_max_d_nnn` in `validate-dispatch-advance` scans the full STATE.md content —
any D-NNN token, including sentinel values, would be returned as `max_in_file`, requiring
`current_step:` to cite that integer or higher and thus poisoning the D-chain currency gate.
The ADR document (`.factory/specs/architecture/decisions/ADR-041-*.md`) and BC/story spec files
are NOT scanned by `scan_max_d_nnn`, so they may freely contain D-99999.

## Rationale

**Why D-99999 and not D-9999 or D-NEVER?**

D-9999 is within the range that the allocator could theoretically reach in a long-lived
project and collides ambiguously with the D-9000–D-9999 ceiling-reserved range defined in
Decision 2. D-99999 provides a 90,000-value buffer above the ceiling gate threshold and is
unambiguously out-of-reach. It is also valid `D-\d+` format, preserving the test scenario's
"correctly formatted" precondition required by BC-5.39.007 EC-010 and S-15.12 AC-18.

D-NEVER (a symbolic non-numeric token) would break the "correctly formatted D-NNN" scenario
entirely. BC-5.39.007 PC5 requires format-check to pass; a non-numeric token would fail
`is_d_nnn_format()` and convert the test from "valid format but nonexistent" to "invalid
format." That is a different — and weaker — test of the Phase 1/2 boundary.

**Why structural-anchor scan rather than prose grep for the gate?**

Any prose grep would pick up narrative citations of D-99999 (in ADR-041 body text, BC
descriptions, story AC text) and return 99999 as the max, which would either (a) require
current_step: to cite D-99999 forever, or (b) be a false-positive FAIL on every burst after
this ADR is written. Restricting the scan to line-anchored structural forms (heading prefixes
`^#{2,} D-` and table-cell prefix `^[|] *D-`) targets only actual allocation records and is
immune to prose mentions of the sentinel. The two-command structure (one grep per form) avoids
ERE alternation with literal `|` characters, which is non-portable across POSIX ERE
implementations.

**Why POLICY 16 extension rather than a new POLICY?**

Per D-497 parsimony rule, an allocator ceiling constraint is a verification-step extension of
the existing D-NNN global-max allocation gate already in POLICY 16, not a new governance
domain. Adding a ceiling check to the existing pre-burst gate procedure extends the same
POLICY rather than proliferating new numbered policies for closely related invariants.

**Why not a structural absence-of-field approach?**

The test scenario in AC-18/EC-010 specifically tests a correctly-formatted D-NNN citation
that the validator cannot verify cross-site in Phase 1. An absence-of-field approach (e.g.,
omitting the Closes line entirely) would test a completely different code path. The sentinel
must be a valid-format D-NNN to exercise the Phase 1 advisory-only boundary faithfully.

## Consequences

### Positive

- **Structural impossibility of collision**: The ceiling gate (D-8999 max) with D-99999
  sentinel creates a 90,000-value gap. Collision requires both gate bypass AND ~90,000
  subsequent allocations — structurally impossible in practice, not just statistically
  unlikely.
- **Mechanical enforcement**: The gate predicate is literal-shell-invocable with captured
  stdout per D-449(a), replacing the prose-only D-960(e) reservation that was vulnerable to
  the same narrative-attest failure class as META-LEVEL-24.
- **Preserves test scenario semantics**: D-99999 is valid `D-\d+` format — all existing tests
  that verify "correctly formatted but nonexistent D-NNN produces advisory not block" continue
  to test the correct behavior.
- **No validator breakage**: `scan_max_d_nnn`, `is_d_nnn_format`, and all D-NNN citation
  validators accept D-99999 without modification. The ceiling gate is additive to POLICY 16.

### Negative / Trade-offs

- **Migration cost**: All sentinel-bearing sites (14 distinct locations, listed in §Migration
  Sequence) require coordinated updates across product-owner, story-writer, and test-writer
  domains. The work is mechanical but must be done in the correct order (BC first, then story,
  then test fixtures and Rust source) to maintain spec-code alignment.
- **STATE.md authoring constraint**: Decision 4 introduces a perpetual discipline requirement —
  any state-manager writing STATE.md migration narrative must avoid embedding the sentinel
  value as a bare D-NNN token. This is documented in this ADR as the authoritative reference.
- **POLICY 16 gate adds pre-burst step**: Each burst now includes one additional literal-shell
  check. This is negligible in practice but extends the pre-burst gate checklist.

### Status as of 2026-08-07 (superseded — see `## Status` below)

Proposed. The ADR was authored by architect in response to human ruling at D-960(e) and the
D-999 collision time-bomb identified at ~39 allocations from D-960. Implementation (BC/story
edits and test fixture updates) is gated on this ADR's acceptance and subsequent specialist
dispatch (product-owner, story-writer, test-writer per §Downstream Routing).

> **SUPERSEDED 2026-08-13 (v1.1 amendment):** this subsection describes the state at
> authoring time (2026-08-07) and was never revisited despite the migration completing the
> same day. `decision-log.md` D-961(a)/(b) confirm: sentinel migration executed across all 4
> layers (BC-5.39.007 v1.6→v1.7, S-15.12 v1.4→v1.5, 8 code/fixture sites at `bf642fd9`,
> POLICY 16 ALLOCATOR-CEILING GATE added to `policies.yaml`). Implementation is COMPLETE, not
> gated-pending. See `## Status` (end of document) for the current, accurate disposition —
> including the separate open question of whether the ADR *document itself* (frontmatter
> `status:`/`ratified:`) has been properly ratified, which is F-S2107-P10-005 and is distinct
> from "was the migration work done" (it was).

## Site Inventory — All D-999 Sentinel Locations

Citations use behavioral anchors per TD-VSDD-091 (no line numbers). All paths are absolute.

### Normative spec artifacts — MUST migrate (product-owner / story-writer)

| Location | Artifact | Anchor | Current sentinel | Action |
|----------|----------|--------|-----------------|--------|
| BC-5.39.007 | `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.007.md` | EC-010 row in §Edge Cases table; Phase 2 boundary table row "Cross-site staleness (Phase 2 scope)" | `D-999` (fixture value in both rows) | product-owner: replace D-999 with D-99999 in EC-010 and Phase 2 boundary row |
| S-15.12 AC-18 | `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/stories/S-15.12-validate-closes-completeness.md` | AC-18 row text, §Fixture Directory inventory entry `pass-phase1-advisory-only/`, Risk table row "Phase 1 advisory-only boundary", adversary-MUST-verify note | `D-999` (5 occurrences across 4 anchors) | story-writer: replace D-999 with D-99999 at all 4 AC-18-anchored locations |

### Test fixtures — MUST migrate (test-writer)

| Location | Artifact | Anchor | Current sentinel | Action |
|----------|----------|--------|-----------------|--------|
| Bats test | `/Users/zious/Documents/GITHUB/vsdd-factory/plugins/vsdd-factory/tests/validate-closes-completeness/pass-phase1-advisory-only.bats` | `@test "AC-18 PASS: hook emits Continue for correctly-formatted D-999 cite..."` test description and fixture construction | `D-999` | test-writer: update test description and fixture to D-99999 |
| Fixture lessons.md | `/Users/zious/Documents/GITHUB/vsdd-factory/plugins/vsdd-factory/tests/fixtures/validate-closes-completeness/pass-phase1-advisory-only/factory/cycles/v1.0-brownfield-backfill/lessons.md` | `**Closes:** D-999` entry body | `D-999` | test-writer: replace with `**Closes:** D-99999` |
| Rust unit test (validate-closes-completeness) | `/Users/zious/Documents/GITHUB/vsdd-factory/crates/hook-plugins/validate-closes-completeness/src/lib.rs` | Test block comment "D-999 is correctly formatted — should NOT produce a violation" and `let line = "**Closes:** D-999"` test vector | `D-999` | test-writer: update comment and test vector to D-99999 |
| Rust unit test (validate-state-structure) | `/Users/zious/Documents/GITHUB/vsdd-factory/crates/hook-plugins/validate-state-structure/src/lib.rs` | Doc comment "Example line: `D-999 ...`" on banner_line extractor; test fixture `banner_line` variable | `D-999` | test-writer: update doc comment and `banner_line` fixture to D-99999 |
| Rust unit test (validate-policies-schema) | `/Users/zious/Documents/GITHUB/vsdd-factory/crates/hook-plugins/validate-policies-schema/src/lib.rs` | `assert!(is_d_nnn_format("D-999"))` test statement | `D-999` | test-writer: assertion tests D-NNN format validity — this asserts D-999 is valid format; updating to D-99999 tests the same property; update test vector to D-99999 for consistency |
| Policies fixture | `/Users/zious/Documents/GITHUB/vsdd-factory/plugins/vsdd-factory/tests/fixtures/validate-policies-schema/integration-production-registry-nonexistent-plugin/factory/policies.yaml` | `codified_at: "D-999"` field in nonexistent-plugin fixture | `D-999` | test-writer: update to `D-99999` for consistency (this fixture tests plugin nonexistence, not decision nonexistence, so behavior unchanged; update prevents confusion when D-999 is later allocated as a real decision) |

### Historical/narrative references — update to record migration (state-manager, last in sequence)

| Location | Artifact | Anchor | Content | Action |
|----------|----------|--------|---------|--------|
| STATE.md | `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/STATE.md` | Session Resume Checkpoint §Pending Decisions / Open Concerns; `current_step:` narrative | "D-999 collision time-bomb: ~39 allocations" | state-manager: update to "D-999 sentinel migrated to D-99999 per ADR-041; POLICY 16 ceiling gate D-8999 established; time-bomb resolved" — MUST NOT embed bare `D-99999` token (Decision 4 discipline) |
| session-checkpoints.md | `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/cycles/v1.0-brownfield-backfill/session-checkpoints.md` | D-960 Checkpoint bullet "D-999 collision time-bomb" | narrative reference | state-manager: historical record; mark as "RESOLVED per ADR-041" — no D-99999 token (Decision 4) |
| decision-log.md D-960(e) | `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/cycles/v1.0-brownfield-backfill/decision-log.md` | D-960(e) sub-clause "D-999 reserved as never-allocatable sentinel" | prose reservation | state-manager: historical; add follow-up note "SUPERSEDED by ADR-041 sentinel migration" after the sub-clause body — immutable historical record, no in-place rewrite |
| burst-log.md | `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/cycles/v1.0-brownfield-backfill/burst-log.md` | D-960 burst entry item (e) | "(e) D-999 sentinel reserved never-allocatable" | state-manager: historical; append "SUPERSEDED: migrated to D-99999 per ADR-041" — immutable historical record |

## Migration Sequence

Dispatch in this order to maintain spec-code alignment at each step:

1. **Architect** (this burst): Write ADR-041 (this document); insert ARCH-INDEX row; add POLICY 16 ceiling extension to `policies.yaml`.
2. **product-owner**: Migrate BC-5.39.007 EC-010 and Phase 2 boundary table row (D-999 → D-99999). BC version bump. BC-INDEX row update.
3. **story-writer**: Migrate S-15.12 AC-18 and the three co-located anchors (directory listing, risk table, adversary note). Story version bump. STORY-INDEX row update.
4. **test-writer**: Migrate test fixtures and Rust unit tests (6 sites listed in Test fixtures table above). Confirm bats suite passes after edits.
5. **state-manager**: Record migration in STATE.md + session-checkpoints.md + historical annotations in decision-log.md and burst-log.md. STATE.md narrative MUST NOT embed bare D-99999 token per Decision 4.

**Invariant**: At no point during migration should a spec artifact say D-99999 while the corresponding test fixture still says D-999 (or vice versa). Steps 2 and 3 must complete atomically before Step 4 migrates tests.

## Mechanical Gate — POLICY 16 Ceiling Extension

The following clause is added to POLICY 16 `verification_steps` in `policies.yaml`. It
replaces the single-form `^## D-` predicate originally written in this ADR's first draft
(corrected after corpus analysis revealed the F5 cycle uses table-cell form and both cycles
use h3 headers — see Decision 3 §Why three forms):

```
ALLOCATOR-CEILING GATE (ADR-041 §Decision 2-3): before each D-NNN allocation,
state-manager MUST assert that the global max ALLOCATED D-NNN — across all
structural forms in all cycle decision-log.md files — is < 9000. D-9000 through
D-99999 are the reserved sentinel range; D-99999 is the canonical test-fixture
sentinel. Forms covered: h2 headers (^#{2,} D-NNN), h3 headers (^#{2,} D-NNN),
leading table-cell rows (^[|] *D-NNN). Gate fails CLOSED on empty result.

Literal-shell predicate:
  max_d=$(
    { grep -hE '^#{2,} D-[0-9]+' .factory/cycles/*/decision-log.md 2>/dev/null
      grep -hE '^[|] *D-[0-9]+' .factory/cycles/*/decision-log.md 2>/dev/null
    } | grep -oE 'D-[0-9]+' | sed 's/D-//' | sort -n | tail -1
  )
  if [ -z "$max_d" ]; then
    printf 'FAIL: D-NNN ceiling gate: zero D-NNN allocation records found — corpus scan failure; gate fails closed\n'; exit 1
  fi
  [ "$max_d" -lt 9000 ] \
    && printf 'PASS: global max D-%s < D-9000 ceiling\n' "$max_d" \
    || { printf 'FAIL: D-NNN allocation ceiling breach: max=D-%s\n' "$max_d"; exit 1; }

Exit code 0 = allocation permitted. Exit code 1 = allocation blocked; human must
adjudicate sentinel re-migration before resuming. STATE.md authoring discipline
(ADR-041 §Decision 4): narrative about the sentinel MUST NOT embed bare D-99999
as a parseable D-NNN token in STATE.md body or current_step:.
```

## Downstream Routing

Work that this ADR does NOT perform (architect does not own BC content, story content, or
test fixture content per Agent Routing Table in CLAUDE.md):

| Specialist | Sites to change | Deliverable |
|------------|-----------------|-------------|
| **product-owner** | `BC-5.39.007` EC-010 row, Phase 2 boundary table row | D-999 → D-99999 in both rows; BC version bump; BC-INDEX row update |
| **story-writer** | `S-15.12` AC-18 row text, `pass-phase1-advisory-only/` directory listing entry, risk table row, adversary-MUST-verify note | D-999 → D-99999 at all 4 anchors; story version bump; STORY-INDEX row update |
| **test-writer** | `pass-phase1-advisory-only.bats` (test description + fixture), `pass-phase1-advisory-only/factory/cycles/.../lessons.md` (Closes line), `validate-closes-completeness/src/lib.rs` (comment + test vector), `validate-state-structure/src/lib.rs` (doc comment + banner_line fixture), `validate-policies-schema/src/lib.rs` (assert test vector), `integration-production-registry-nonexistent-plugin/factory/policies.yaml` (codified_at) | D-999 → D-99999 at all 6 test-fixture sites; bats suite must pass |
| **state-manager** | STATE.md, session-checkpoints.md, decision-log.md D-960(e) annotation, burst-log.md D-960 entry annotation | Migration recorded; historical D-960(e) annotated "SUPERSEDED by ADR-041"; STATE.md "time-bomb" narrative updated; NO bare D-99999 token in STATE.md per Decision 4 |

**Regression test** (test-writer): author one new bats test `pass-phase1-advisory-d99999.bats`
using a fixture with `**Closes:** D-99999` to confirm the migrated sentinel still produces
`HookResult::Continue` + advisory log only. This test replaces the behavioral assertion that
`pass-phase1-advisory-only.bats` carries after its D-999 → D-99999 edit.

## Alternatives Considered

- **Option A: Numerically distant value (D-9999) without ceiling gate** — Rejected because it
  is ~8,039 allocations away, potentially reachable, and still relies on prose discipline
  rather than a mechanical gate. The same failure class (narrative-attested reservation) that
  made D-960(e) insufficient applies here.

- **Option B: Symbolic non-numeric token (D-NEVER)** — Rejected because it fails `is_d_nnn_format()`
  validation. BC-5.39.007 EC-010 specifically tests a **correctly formatted** D-NNN citation;
  a format-invalid token tests a different code path (malformatted citation handling) and would
  require rewriting the edge case semantics, not just the sentinel value.

- **Option C: Prose-only reservation via POLICY 16 amendment (extend D-960(e))** — Rejected
  explicitly. This is the failure class ADR-041 is designed to close: prose-disciplined
  reservations cannot detect their own scope-degradation (META-LEVEL-24, L-EDP1 lessons on
  narrative-attested gates). The human ruling "MIGRATE the sentinel" confirms this option is
  off the table.

- **Option D: Absence-of-field approach (no in-band sentinel)** — Rejected because the test
  scenario requires a D-NNN that parses as valid format but is absent from the decision-log.
  Removing the sentinel entirely would require restructuring BC-5.39.007 EC-010 and AC-18 to
  test cross-site absence through a different mechanism (e.g., a separate fixture category),
  which is a significantly larger spec change than a sentinel value migration.

## Source / Origin

- `BC-5.39.007 EC-010` — Normative source for Phase 1 advisory-only boundary behavior using
  D-999 as fixture sentinel. Authored at S-15.12 brownfield M3 wave 3M3a.
- `S-15.12 AC-18` — Story acceptance criterion tracing to BC-5.39.007 EC-010.
- Decision-log `D-960(e)` — Prior prose reservation of D-999 as never-allocatable; this ADR
  supersedes the prose-discipline approach with a structural mechanical gate.
- `crates/hook-plugins/validate-dispatch-advance/src/lib.rs` behavioral anchor `scan_max_d_nnn`
  — Function that scans STATE.md for D-NNN integers; basis for Decision 4 authoring discipline.
- Human ruling at STATE.md Session Resume Checkpoint §Pending Decisions (D-960 SRC): "migrate
  the sentinel, not merely reserve it."

## Status

PROPOSED 2026-08-07; ADR-041 v1.0 (architect; human ruling at D-960 SRC authorized the
substance: "migrate the sentinel, not merely reserve it"). Migration executed same-day across
all 4 layers per `decision-log.md` D-961(a)/(b) — the ADR's *content* has been fully
implemented and is running live (POLICY 16 ALLOCATOR-CEILING GATE fires before every D-NNN
allocation; confirmed passing in every burst from D-961 through the current HEAD, e.g. D-970
"PASS: global max D-969 < D-9000 ceiling"). AMENDED 2026-08-13; ADR-041 v1.1 (architect;
S-21.07 pass-10 ADR-anchored fix cascade, closes F-S2107-P10-005 ADR-041 leg): added
version-tracking frontmatter (absent at v1.0); corrected the stale "Status as of 2026-08-07"
subsection under §Consequences (superseded in place, not deleted); this `## Status` section
added. AMENDED 2026-08-13; ADR-041 v1.2 (architect; S-21.07 pass-10 fix cascade, body-vs-
frontmatter reconciliation pass, pre-adversary-pass-11): `## Status` section's
"RATIFICATION STATUS — NEEDS HUMAN ADJUDICATION" and "Recommendation to human" paragraphs
reconciled to present-tense — human ratification OCCURRED 2026-08-13 (D-992); frontmatter
now carries `status: active`/`ratified: "2026-08-13"`. Historical text preserved with a
superseding note, not deleted.

> **SUPERSEDED 2026-08-13 (v1.2 amendment, body-vs-frontmatter reconciliation, S-21.07
> pass-10 fix cascade):** the two paragraphs immediately below this note ("RATIFICATION
> STATUS — NEEDS HUMAN ADJUDICATION" and "Recommendation to human") described the state as
> of v1.1 authoring (2026-08-13, earlier same day) and were not revisited when ratification
> occurred later the same day. **Human ratification HAS NOW OCCURRED**: D-992 (S-21.07
> pass-10 fix burst, 2026-08-13) — the human answered "Ratify both now" for ADR-041 and
> ADR-042 together, exactly the disposition (c) the v1.1 §Status recommendation proposed.
> Frontmatter now carries `status: active`, `ratified: "2026-08-13"`, and a
> `ratification_note` recording the D-992 event. The live-gate/ungoverned-ADR contradiction
> the v1.1 paragraphs flagged is RESOLVED — POLICY 16 ALLOCATOR-CEILING GATE (§Decision 3)
> now runs under a ratified, `status: active` ADR, not a `proposed` one. The v1.1 text is
> preserved below verbatim for historical continuity (it accurately reflects the ADR's
> pre-ratification state and the reasoning that led to ratification).

**RATIFICATION STATUS AS OF v1.1 AUTHORING (2026-08-13, HISTORICAL — F-S2107-P10-005,
RESOLVED BY D-992 LATER THE SAME DAY).** Frontmatter `status:
proposed`, no `ratified:` field, despite `decision-log.md` D-961(a) stating "ADR-041
ratified" in a state-manager multi-specialist recording-burst summary — narrative language,
not an explicit dated human ratification event with a `ratification_note` comparable to
ADR-040 v1.12's. The underlying substance ruling ("migrate the sentinel") IS genuinely
human-authorized and on record (§Context / §Source-Origin). What is missing is ratification
of the ADR *document*. Architect does not claim authority to self-declare this ratified —
same ADR-040/D-965 precedent cited in ADR-042 v1.3 §Status.

**Live-gate contradiction, concretely (as of v1.1, historical):** the POLICY 16 ALLOCATOR-CEILING GATE (this ADR
§Decision 3) has executed as a blocking pre-allocation gate in every single burst since
D-961 — at least 10 confirmed PASS invocations through the current HEAD — while its
governing ADR has never carried a `status: active`/`ratified:` frontmatter pair. The gate's
design has never produced an adverse finding in ~10 live invocations, which is evidence the
design is sound, but soundness-in-practice is not a substitute for the ratification record
the project's own governance model requires for a document whose rulings gate every burst.

**Recommendation to human (as of v1.1, historical — ACTED ON at D-992):** ratify ADR-041 v1.1 now (`status: proposed → active`,
`ratified: <date-of-explicit-confirmation>`), on the basis that (a) the D-960 SRC ruling
already authorizes the substance, (b) the mechanism has been running correctly in production
for ~6 days across 10+ bursts with zero adverse findings, and (c) ADR-042 carries the
identical F-005 gap and both should be resolved together in one human ratification pass
rather than staggered. If the human prefers a different disposition (e.g., a fresh review of
the gate's three structural-anchor forms before ratifying), that is also a legitimate
outcome — this section flags the decision point; it does not presume the answer.

**Disposition (v1.2): the human chose recommendation (c).** D-992 (2026-08-13) ratified
ADR-041 and ADR-042 together in one pass, per the recommendation above. This ADR's
ratification status is CLOSED — see frontmatter and the superseding note at the top of this
`## Status` section.
