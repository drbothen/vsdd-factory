---
document_type: adversary-pass
level: ops
pass: 2
cascade: LOCAL
story: S-15.07
producer: adversary
timestamp: 2026-05-16T00:00:00Z
diff_base: c62f952c
diff_head: 1e07416c
verdict: MEDIUM
finding_count_by_severity:
  critical: 0
  high: 0
  medium: 1
  low: 2
  nitpick: 0
policies_evaluated: [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18]
---

# S-15.07 LOCAL Adversary Pass-2

## Verdict

**MEDIUM** -- Fix-burst landed all five pass-1 remediations on the surface, but a fresh-eyes read on F-002's stated rationale reveals an unresolved structural divergence: the block message says `v1.5` while the body literal says `v1.05`. The Ctrl-F UX problem the F-002 fix was supposed to close persists in inverted form. One MEDIUM finding (F-002 partial-fix) plus two LOW findings (bats inline-registry sibling-sweep gap from Q5 sweep; multi-violation test assertion weakness) require remediation before 3-CLEAN.

## Findings Table

| ID | Severity | Policy-Triggered | Category | Summary | Recommended Action |
|----|----------|------------------|----------|---------|--------------------|
| F-S15.07-LOCAL-P2-001 | MEDIUM | POLICY 4 (semantic_anchoring_integrity); TD-VSDD-059 paper-fix | spec-vs-impl drift | F-002's stated goal -- block-message string byte-identical to the body cite so authors can Ctrl-F -- is NOT achieved by removing `{:02}`. Parser converts `BC-INDEX v1.05` &rarr; `(major=1, minor=5)`, then re-renders as `v1.5`. Body literal `v1.05` no longer appears anywhere in the block message. BC-5.39.003 &sect;Edge Cases EC-005 example block message uses `v1.05` literally; impl emits `v1.5`. Spec deviation persists, just inverted form vs pass-1. | Plumb the original string slice (e.g., `pub cited_raw: String` carrying the literal `"1.05"` from `extract_index_cites`), and emit `cited_raw` in the block message. Drop the `(major, minor)` re-render from the cited side. Live side keeps integer-rendered canonical form. Alternative: amend spec EC-005 to drop the `v1.05` literal and document `v1.5` as the canonical bare-rendering. |
| F-S15.07-LOCAL-P2-002 | LOW | TD-VSDD-060 sibling-site sweep | convention-drift | Architect Q5 &sect;Future-Story Convention Lock established `tool = "Edit&#124;Write"` as canonical for hook registry entries. Sweep applied to production `plugins/vsdd-factory/hooks-registry.toml` (3 sites: 844/861/995). All 8 bats fixture files (`plugins/vsdd-factory/tests/validate-index-cite-refresh/*.bats`) still write inline synthetic registries with `tool = "Write&#124;Edit"` at line 37-43 of each. Per architect Q5 dispatcher analysis these are functionally equivalent (regex alternation is order-insensitive), but they violate the just-codified convention lock -- sibling-sweep is incomplete. | Sweep all 8 bats inline `_write_registry()` heredocs to `tool = "Edit&#124;Write"`. Single in-scope edit per CLAUDE.md Canonical Principle Rule 4 (~5 min total). Alternatively, escalate to architect for explicit Q6 confirming bats inline registries are out-of-scope for the convention lock; document in test README. |
| F-S15.07-LOCAL-P2-003 | LOW | POLICY 11 (no_test_tautologies) | test-assertion-weakness | `fail-multi-stale-cites.bats` test 2 asserts `[[ "$output" == *"1.5"* ]]` to verify stale BC-INDEX cite rendering. Substring `"1.5"` is a numerically-weak discriminator -- would also match if output contained "3.31.5", "v0.1.5", "anything-with-1.5-in-it". The fixture's STORY-INDEX cite is `v3.28` and live is `v3.32`. There is no test guarding against accidental rendering of "1.5" appearing without the leading "v" or in a non-version context. Same applies to test 1's `1.5` assertion on `fail-stale-bc-index.bats`. | Tighten to `[[ "$output" == *"v1.5"* ]]` or `[[ "$output" == *"BC-INDEX v1.5"* ]]` for full discriminating power. Apply same hardening to test 2's substring assertions. Trivial single-line edit per .bats file. |

## Finding Details

### F-S15.07-LOCAL-P2-001 -- MEDIUM -- F-002 fix incomplete: body literal `v1.05` &ne; block message `v1.5`

**Anchor:** `format_violation` function + `Violation::cited` field in `crates/hook-plugins/validate-index-cite-refresh/src/lib.rs`

**Evidence (verbatim file:line: from Read):**

```
crates/hook-plugins/validate-index-cite-refresh/src/lib.rs:117:    pub cited: (u32, u32),
crates/hook-plugins/validate-index-cite-refresh/src/lib.rs:438:    format!(
crates/hook-plugins/validate-index-cite-refresh/src/lib.rs:439:        "  [{}] {} cites {} v{}.{} but live version is v{}.{}. Update cite to v{}.{}.",
crates/hook-plugins/validate-index-cite-refresh/src/lib.rs:443:        v.cited.0,
crates/hook-plugins/validate-index-cite-refresh/src/lib.rs:444:        v.cited.1,
```

Fixture body contains literal `BC-INDEX v1.05`:
```
plugins/vsdd-factory/tests/fixtures/validate-index-cite-refresh/fail-stale-bc-index/factory/specs/architecture/ARCH-INDEX.md:13:- BC-INDEX v1.05
plugins/vsdd-factory/tests/fixtures/validate-index-cite-refresh/fail-multi-stale-cites/factory/specs/architecture/ARCH-INDEX.md:13:- BC-INDEX v1.05
plugins/vsdd-factory/tests/fixtures/validate-index-cite-refresh/fail-open-missing-index/factory/specs/architecture/ARCH-INDEX.md:11:- BC-INDEX v1.05
```

Parser path: `parse_leading_digits` consumes `"05"` and `.parse::<u32>()` produces `5`. Storage is `(1, 5)`. `format_violation` re-renders via `v{}.{}` &rarr; `v1.5`. Body has `v1.05`; message says `v1.5`. The pass-1 F-002 recommendation explicitly said "cited-version output must be byte-identical to the body string the user wrote" -- the implementer hit the spec format-string verbatim (no width specifier) but the by-construction integer reparse strips the leading zero.

BC-5.39.003 EC-005 (line 114) cites the example: `BlockWithFix: "ARCH-INDEX cites BC-INDEX v1.05 but live version is v2.24 -- update cite to v2.24"`. Implementation will produce `v1.5` not `v1.05`. Unit test at lib.rs:741 was retrofitted to assert `"1.5"` rather than `"1.05"` -- codifying the regression rather than fixing it.

**Rationale:** This is TD-VSDD-059 paper-fix detection again. Pass-1 F-002 identified the symptom (zero-pad mismatch in one direction). The fix-burst removed the zero-pad -- but flipping the rendering form does not solve the underlying problem (parser loses the body literal). The author still cannot Ctrl-F the cited version from the block message into the body. POLICY 4 (semantic_anchoring_integrity): the `Violation::cited` field's anchor to the spec's "cite locality help" intent is structurally partial -- it preserves comparison semantics but loses display fidelity. The unit-test update from `"1.05"` &rarr; `"1.5"` (lib.rs `test_BC_5_39_003_emit_block_names_index_and_versions`) regression-codifies the deficiency: passing the test now PROVES the block message doesn't match body literal.

**Recommended fix:** Add `pub cited_raw: String` to `Violation`. During `extract_index_cites`, capture the literal slice `content[after_v..end]` (the `"1.05"` substring) and propagate. In `format_violation`, emit `v{cited_raw}` for the cited side; keep `(u32, u32)` integer comparison for staleness logic and live-side rendering. Update unit test assertion back to `"1.05"`. Sibling-sweep: same plumbing in `cross_cell_check` STATE.md/INDEX.md paths.

---

### F-S15.07-LOCAL-P2-002 -- LOW -- Bats inline-registry sibling-sweep gap on Q5 canonical form

**Anchor:** All 8 bats test files in `plugins/vsdd-factory/tests/validate-index-cite-refresh/*.bats` `_write_registry()` heredocs

**Evidence (verbatim grep stdout):**

```
plugins/vsdd-factory/tests/validate-index-cite-refresh/pass-all-current.bats:37:tool = "Write|Edit"
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-stale-vp-index.bats:38:tool = "Write|Edit"
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-open-missing-index.bats:43:tool = "Write|Edit"
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-cross-cell-index-md.bats:39:tool = "Write|Edit"
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-stale-bc-index.bats:38:tool = "Write|Edit"
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-multi-stale-cites.bats:42:tool = "Write|Edit"
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-stale-story-index.bats:38:tool = "Write|Edit"
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-cross-cell-state-md.bats:40:tool = "Write|Edit"
```

Architect Q5 (`architect-m2-q5-tool-attribute-2026-05-16.md`) &sect;Future-Story Convention Lock:

> "All registry entries added by S-15.09, S-15.11, S-15.14, and any subsequent hook story MUST use `tool = "Edit|Write"` (not `"Write|Edit"`) when the hook fires on file-write operations; this is the canonical form..."

Production registry sweep (commit `8f1b6254`) correctly aligned the 3 production sites (844/861/995). But bats inline `_write_registry()` heredocs are registries -- they are passed to the dispatcher subprocess as `$WORK/hooks-registry.toml`. They are registry entries authored as part of S-15.07 delivery and now drift from the canonical form Q5 codified.

**Rationale:** TD-VSDD-060 sibling-site sweep gap. The fix-burst caught the production registry but not the test-side registry constants. Per architect Q5 functional analysis the dispatcher's `tool_matches()` is regex-alternation-order-insensitive (`Edit|Write` and `Write|Edit` behave identically), so this is functionally equivalent -- but the convention lock was explicitly stated, and 8 inline-registry sites that copy from each other are exactly the propagation-vector that motivated Option A in Q5 (avoiding minority-form propagation to downstream stories). Reporting as LOW per the dispatcher-equivalence; promote to MEDIUM if architect Q5 intended bats inline registries within scope of the lock.

**Recommended fix:** Sweep all 8 `_write_registry()` heredocs to `tool = "Edit|Write"`. Single batched edit (5 min). Verification: `grep -rn 'tool = "Write|Edit"' plugins/vsdd-factory/tests/validate-index-cite-refresh/` returns empty.

Tagged `(pending intent verification)` per Partial-Fix Regression Discipline -- orchestrator/architect may confirm bats inline registries are out-of-scope for the convention lock, in which case the finding closes as no-op.

---

### F-S15.07-LOCAL-P2-003 -- LOW -- Bats substring assertion `"1.5"` is numerically weak

**Anchor:** `fail-multi-stale-cites.bats` `@test "EC-008: block message names BC-INDEX with stale cite v1.05 and STORY-INDEX with stale cite v3.28"` test body; `fail-stale-bc-index.bats` AC-10 test body

**Evidence (verbatim file:line: from Read):**

```
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-multi-stale-cites.bats:117:  # BC-INDEX cite is "v1.05" in the fixture; hook renders minor without zero-pad => "1.5".
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-multi-stale-cites.bats:118:  # STORY-INDEX cite is "v3.28" in the fixture; renders as "3.28".
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-multi-stale-cites.bats:119:  [[ "$output" == *"1.5"* ]]
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-multi-stale-cites.bats:120:  [[ "$output" == *"3.28"* ]]
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-stale-bc-index.bats:109:  [[ "$output" == *"1.5"* ]]
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-stale-bc-index.bats:110:  [[ "$output" == *"2.24"* ]]
```

The substring `"1.5"` is a 3-character literal that would match many incidental strings (e.g., a future change that renders dispatcher version "1.5.0", a path containing "1.5", a build number, etc.). Discrimination relies on the contextual surrounding text, which the assertion does not verify. Stronger assertion would be `"v1.5"` (4-char, version-prefix anchored) or `"BC-INDEX v1.5"` (full canonical-form anchored).

**Rationale:** POLICY 11 (no_test_tautologies) -- not a tautology per se, but weakly-discriminating. The test passes as long as `"1.5"` appears anywhere in stderr, which is a low bar for a substring-mode assertion intended to verify per-version block-message rendering. POLICY 11 says: "tests must substantively exercise the property under test, not just any string that happens to look related."

**Recommended fix:** Replace `*"1.5"*` with `*"v1.5"*` (or `*"BC-INDEX v1.5"*` for full anchor) in both `fail-multi-stale-cites.bats` test 2 and `fail-stale-bc-index.bats` AC-10 test. Similarly tighten `*"2.24"*` &rarr; `*"v2.24"*`, `*"3.28"*` &rarr; `*"v3.28"*`. Trivial edit; no fixture change.

---

## Observations

### O-S15.07-LOCAL-P2-001 -- F-001 line-number computation is correct on multibyte content

The `extract_index_cites` line-tracking logic (lib.rs `current_line` counter) correctly increments on newline-bytes and uses `is_char_boundary` to step past UTF-8 multi-byte characters. Unit test `test_BC_5_39_003_extract_handles_multibyte_chars_no_panic` exercises em-dash content. F-001 fix is structurally sound. Not a finding -- confirming-the-good observation.

### O-S15.07-LOCAL-P2-002 -- Partial-prefix-match path traverses no newlines

The "partial match" branch in `extract_index_cites` (lines 207-215) counts newlines inside the consumed prefix span. Since all four canonical names (`BC-INDEX`, `VP-INDEX`, `STORY-INDEX`, `ARCH-INDEX`) contain no embedded newlines, this loop is effectively a no-op. Not a bug; intentionally defensive. Worth noting for any future expansion of `INDEX_PREFIXES` that the assumption holds. Not a finding.

### O-S15.07-LOCAL-P2-003 -- `INDEX_PREFIXES` ordering rationale comment is misleading

The comment at lib.rs:80-82 says "Longer names must come first to avoid 'BC-INDEX' matching 'BC-INDEX' in 'BC-INDEX'" -- this is tautological. None of the four canonical names is a prefix of another (no name is a substring of any other at position 0). The priority ordering is functionally indifferent. Suggest replacing the comment with a more accurate statement (e.g., "Names are listed longest-first as a defensive convention; current set has no prefix-overlap, but future additions might"). Not a content-defect finding; documentation clarity nitpick only.

### O-S15.07-LOCAL-P2-004 -- Story spec/BC EC-005 example wording will require amendment if F-002 fix-002 lands

If F-002 partial-fix (this pass) is closed via the recommended-fix path (plumb `cited_raw`), story spec line 191/382 and BC-5.39.003 EC-005 line 114 already cite `v1.05` literal -- they would be consistent with the impl after fix. If instead the spec/BC are amended to use `v1.5` rendering, the unit-test assertion regression-codification at lib.rs:741 becomes correct. Either path requires a same-burst spec/impl consistency check. Surfacing for orchestrator awareness; not a finding by itself.

### O-S15.07-LOCAL-P2-005 -- AC-9 process-gap from pass-1 still applies

Pass-1 O-001 [process-gap] identified AC-9's literal-verification predicate as un-runnable against the registry schema (no `file_pattern` field). Fix-burst did not address it (out of scope -- process gap, not story-defect). Re-surfacing here for orchestrator AC-9 closure adjudication.

---

## Part B -- Pass-Internal Notes (NOT visible to subsequent passes)

### Convergence Streak Status

- **Entering pass-2:** 0/3 (pass-1 had 2 MEDIUM &rarr; reset)
- **After pass-2 verdict (MEDIUM with 1 MEDIUM + 2 LOW):** 0/3 (streak does not advance; MEDIUM resets per BC-5.39.001 3-CLEAN protocol)

Per S-15.08 cascade convention disambiguation: "LOW does NOT reset; only MEDIUM+ resets." One MEDIUM finding exists &rarr; streak resets/holds at 0/3.

### Fresh-Context Pattern Observations

- The F-002 fix is a textbook paper-fix regression. Pass-1 found a zero-pad mismatch; the fix removed the zero-pad and updated the test assertion to match. But the fundamental Ctrl-F UX deficiency persists -- the implementation re-renders a numeric form regardless of which side has the zero-pad. The test-assertion update (`"1.05"` &rarr; `"1.5"`) is a regression-codification smell that the adversary should have caught at code-review time.
- F-005 fix landed correctly at the production registry layer. The fix-burst's commit message and the architect Q5 &sect;Implementation Plan explicitly scoped to production registry; bats inline registries weren't named. This is a legitimate scope-edge case. The architect should adjudicate whether the convention-lock binds the bats fixtures too. If yes &rarr; MEDIUM. If no &rarr; close as LOW (pending intent verification) per S-7.01 discipline.
- The line-number form `"line N"` chosen by implementer for `Violation::location` is good production-grade -- actionable, unambiguous, easy to grep. Better than section-heading-based alternatives.
- Multi-violation test coverage is real coverage; assertions exist for both indexes plus violation count. Test 3 (`"2 stale cite"`) is well-anchored. F-S15.07-LOCAL-P2-003 only flags the weak `"1.5"` substring assertion; the rest of test 2 (`*"BC-INDEX"*`, `*"STORY-INDEX"*`) and all of test 3 are substantively assertive.
- README F-003 fix is clean. README &sect;Fixture Layout now correctly describes the `cp -r factory/. .factory/` mechanic with the pre-creation step. Downstream stories (S-15.11, S-15.09, S-15.14) inheriting this test pattern will read accurate docs.

### Recommendation to Orchestrator

**Dispatch fix-burst-2** before pass-3 -- one MEDIUM finding (F-001) requires remediation.

Routing by finding:
- F-S15.07-LOCAL-P2-001 (`cited_raw` plumbing for body-form preservation): **implementer** -- adds field, plumbs string slice during extraction at all 3 cite-construction sites (`on_post_tool_use` ARCH-INDEX path; `cross_cell_check` STATE.md and INDEX.md paths); updates `format_violation` and unit test assertion. Sibling-sweep is mandatory per TD-VSDD-060.
- F-S15.07-LOCAL-P2-002 (bats inline-registry sweep): **architect** for Q6 quick-adjudication on scope of Q5 convention lock; if architect confirms bats are in scope &rarr; **implementer** for 8-line sweep. If architect confirms bats are out of scope &rarr; close finding with documented decision.
- F-S15.07-LOCAL-P2-003 (substring assertion tightening): **test-writer** -- 2-file edit to tighten 6 assertions.

After fix-burst-2 lands on `feature/S-15.07-index-cite-refresh-hook`, dispatch adversary pass-3 (fresh context, Iron Law).

Note on F-S15.07-LOCAL-P2-001 severity: this is MEDIUM rather than HIGH because the block message is still actionable in steady-state operation (vsdd-factory's index versions use 2-digit minors -- `BC-INDEX v2.24`, `STORY-INDEX v3.32`, etc. -- so the zero-pad mismatch only triggers when a fixture uses a single-digit minor cite, which is rare in production). It is MEDIUM rather than LOW because (a) the F-002 fix was supposed to close this exact UX problem, and (b) the unit-test regression-codification means the test now PROVES the deficiency rather than catching it.
