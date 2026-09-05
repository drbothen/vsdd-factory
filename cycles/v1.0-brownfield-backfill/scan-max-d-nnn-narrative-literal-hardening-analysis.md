---
document_type: architecture-design-analysis
level: L4
producer: architect
status: draft
timestamp: 2026-09-05T00:00:00Z
cycle: v1.0-brownfield-backfill
inputs:
  - crates/hook-plugins/validate-dispatch-advance/src/lib.rs
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md
  - .factory/STATE.md
subsystem: "SS-05"
capability: "E-12"
related_bc: BC-5.39.006
related_pr: "#813 (5e009dc0)"
input-hash: "0951cfd"
---

# D-2026 self-referential narrative-literal false-positive — root-cause hardening design

## 1. Problem restated (verified against code + spec, not assumed)

`develop` CI is red on `validate_production_state_md_no_false_positive`
(`crates/hook-plugins/validate-dispatch-advance/src/lib.rs`), which reads the
live `.factory/STATE.md` and asserts `validate_state_md(&content)` returns no
violations. It currently returns one:

```
"D-chain cite in current_step: is stale: max_cited D-1163 but STATE.md body
shows D-2026 as latest; update D-chain cite to include D-2026 per D-443(a)"
```

### 1.1 Confirmed root cause

`check_d_chain_currency` (line ~473) computes:

- `max_cited = scan_max_d_nnn(current_step_value)` → **1163** (verified by
  direct extraction of `current_step:` and re-running the word-boundary scan
  in isolation — matches the task's stated value exactly).
- `max_in_file = scan_max_d_nnn(content)` → **2026** (verified empirically —
  see §1.2).

`scan_max_d_nnn` (post-PR-#813, `5e009dc0`, merged to `origin/develop`) already
carries a word-boundary guard: a `D-\d+` match counts only if the byte before
`D` is either start-of-string or **not** ASCII-alphanumeric
(`crates/hook-plugins/validate-dispatch-advance/src/lib.rs:543-575`). This
guard correctly excludes `D` embedded in identifiers/dates such as
`RC25-RELEASED-2026-09-04` (preceding char `E`, alphanumeric) and
`TD-VSDD-053` (preceding char `T`). It does **not**, and structurally
**cannot**, exclude a `D-2026` token whose preceding character is a quote
(`"`) or a space — both are non-alphanumeric, i.e. legitimate "word
boundaries" by the existing rule, and the rule has no way to distinguish
"boundary because this is a real decision cite" from "boundary because this
is a quoted/prose *mention* of the token."

### 1.2 Empirical reproduction (literal-shell, not narrative)

Extracted the exact byte-offset context of every `D-2026` occurrence in the
live `.factory/STATE.md` via a small Python scan (`re.finditer`, prev-char
check):

```
line  27 pos 371  prevchar='E'  ctx=...RC25-RELEASED-2026-09-04...        (excluded — correct)
line 123 pos  16  prevchar='E'  ctx=...**RC25-RELEASED-2026-09-04**...    (excluded — correct)
line 124 pos 224  prevchar='E'  ctx=...RC25-RELEASED-2026-09-04 (D...     (excluded — correct)
line 124 pos 830  prevchar='E'  ctx=...RC25-RELEASED-2026-09-04) a...     (excluded — correct)
line 140 pos 194  prevchar='E'  ctx=...RC25-RELEASED-2026-09-04 (D...     (excluded — correct)
line 140 pos 724  prevchar='E'  ctx=...RC25-RELEASED-2026-09-04 ch...     (excluded — correct)
line 173 pos 412  prevchar='"'  ctx=...misreading "D-2026" inside ...     (COUNTED — false positive)
line 173 pos 440  prevchar='E'  ctx=..."D-2026" inside "RC25-RELEASED...  (excluded — correct)
line 198 pos 6304 prevchar='"'  ctx=...misreading "D-2026" inside ...     (COUNTED — false positive)
line 198 pos 6332 prevchar='E'  ctx=..."D-2026" inside "RC25-RELEASED...  (excluded — correct)
line 208 pos 2529 prevchar='E'  ctx=...ing "D" inside "RC25-RELEASED...   (excluded — correct)
line 208 pos 2555 prevchar=' '  ctx=...as decision D-2026 and falsely...  (COUNTED — false positive)
line 211 pos 2560 prevchar='E'  ctx=...**RC25-RELEASED-2026-09-04 cl...   (excluded — correct)
line 383 pos  50  prevchar='"'  ctx=...the "D-2026" false-p...            (COUNTED — false positive)
```

Exactly **4** word-boundary-passing `D-2026` tokens — 3 quoted-literal
(`prevchar='"'`), 1 prose (`prevchar=' '`) — matching the task's description
precisely. All 4 occur in narrative prose *describing* the bug (the "Skip
Log"/"Active Branches"/"Decisions Log"/"§4 Pending" sections), never as a
genuine decision reference. `scan_max_d_nnn(content)` returns `2026` because
its only signal is "is the preceding byte alphanumeric," and a quote or space
satisfies that signal identically whether the token is a real cite or a
mention of a string.

This is a **narrative-literal** class, structurally distinct from and not
addressed by PR #813's fix (which only filtered *word-internal* embeddings,
not free-standing but non-decision tokens).

## 2. Is the D-chain-currency check correctly scoped?

**Read `extract_current_cycle` / `is_f5_cycle` / `validate_state_md`
(lines 563-724):** the 4-index-citation check (`check_index_version_cites`)
and the trajectory-tail check (`check_trajectory_tail_length`) are gated
behind `apply_f5_checks = is_f5_cycle(extract_current_cycle(content))`, i.e.
they run only when `current_cycle: v1.0-feature-engine-discipline-pass-1`.
`check_forbidden_meta_commentary` and `check_d_chain_currency` are explicitly
documented and implemented to run **unconditionally, for every cycle**,
including the live `current_cycle: v1.0-brownfield-backfill`.

**Is that correct?** Yes. Evidence:

- The brownfield `current_step:` value itself cites a decision inline —
  `"...resting at the post-S2504-POST-MERGE-BURST-2026-09-04 (D-1163)
  position..."` — so "does this cycle's `current_step:` cite an up-to-date
  decision" is a meaningful, checkable property in brownfield too. The
  4-index-cite and trajectory-tail conventions
  (`BC-INDEX vX / VP-INDEX vX / ...`, `trajectory-tail →N→N→N→N`) are
  F5-engine-discipline-specific dispatch-protocol artifacts that brownfield
  `current_step:` values never contain by design — gating those two off for
  non-F5 cycles is correct and already working (no false positives observed
  from that gate).
- D-chain currency is a cross-cutting governance property (every cycle's
  STATE.md dispatch cite should not go stale), not an F5-specific stylistic
  convention. Universality is intentional, not a scoping bug.

**Conclusion: mis-scoping is NOT part of this defect.** The bug is entirely
in `scan_max_d_nnn`'s inability to distinguish "cite" from "mention," not in
which cycles the check applies to. (See §6 for a *separate*, pre-existing
documentation gap this uncovered: the F5-cycle-gate feature itself has no BC
text at all — flagged as a companion fix, not part of this design.)

## 3. Candidate strategies evaluated

### (a) Structured-context scanning — PRIMARY, recommended

Restrict the **`max_in_file`** comparator (the "ground truth latest decision"
side of the comparison) to genuine structural decision citations: rows under
the `## Decisions Log` h2 heading (exact match, same technique already used
by `validate_index_md`'s `## Adversarial Reviews` state machine one function
up) whose **first** pipe-delimited cell, trimmed, is a *whole-cell* match to
`D-\d+` (not a substring match — the ID column, and nothing else, is
authoritative).

**Why this is correct, not just convenient:**

- It is the literal ground truth the BC prose already gestures at:
  postcondition 1 says the D-chain cite must include "a reference to the
  latest D-NNN recorded in the **cycle decision-log**" — i.e. the intent was
  always "the Decisions Log," not "any D-NNN substring anywhere in the
  file." The current implementation over-generalizes the intent into a
  whole-body scan; narrowing it back to the Decisions Log ID column is a
  precision fix that matches the BC's own stated rationale, not a new
  invention.
- `## Decisions Log` with a `| D-NNN | ... |` first column is the **canonical
  template structure** (`plugins/vsdd-factory/templates/state-template.md:71-75`),
  used by every vsdd-factory STATE.md, not an ad-hoc pattern local to this
  repo's history. This generalizes to any project built on this engine.
- Per this project's own Commit-B discipline (D-448(b): "decision-log D-NNN
  codification block + canonical 6-column rows" every burst), a decision is
  never "real" for currency-checking purposes until it has a Decisions Log
  row — so restricting the scan to that row set cannot cause a **missed**
  genuine staleness case; it can only stop counting things that were never
  genuine citations.
- It is immune to the narrative-literal class **by construction**: prose or
  quoted mentions of a `D-NNN` token can occur anywhere in free text, but
  they cannot occupy the first-cell position of an actual Decisions Log
  table row without becoming an actual (intentional) decision-log entry.
  There is no heuristic here to defeat with a cleverer sentence — the
  distinguishing signal is structural position, not lexical shape.
- It does not touch or weaken PR #813's word-boundary guard, which remains
  exactly as-is for scanning `current_step:` (`max_cited`) — see §4 for why
  that scan is left alone.

**Empirical validation (literal shell, run against the actual unmodified
STATE.md):**

```
current_step: (word-boundary scan)               → max_cited    = 1163
STATE.md body (OLD word-boundary whole-body scan) → max_in_file  = 2026  (BUG)
STATE.md body (NEW Decisions Log ID-column scan)  → max_in_file  = 1163  (FIXED)
```

1163 >= 1163 → no violation. **No STATE.md content edit is required** — the
hardened scanner alone resolves the false positive against the file as it
stands today. (Full reproduction script and output captured in this
session's tool transcript; re-run trivially: parse `## Decisions Log`
section, take first pipe-cell of each non-separator row, regex `^D-(\d+)$`,
take max.)

**Tradeoffs / residual risk:**

- If a burst ever cites a brand-new decision in `current_step:` *before*
  adding its Decisions Log row in the same commit (a process violation of
  D-448(b) itself), the currency check would not flag it as "ahead" (which
  is correct — nothing is stale yet) but also could not flag the *next*
  burst if that Decisions Log row is later added retroactively out of order.
  This is an extremely narrow edge case, already precluded by the existing
  Commit-B same-burst discipline, and is not a regression relative to
  today's behavior (today's behavior is actively broken on the much more
  common case — narrative discussion of the bug itself).
- The row-parsing is hand-rolled pipe-splitting (no regex crate, per the
  file's existing WASM-fuel-budget constraint) and inherits the same
  approximation already accepted by `validate_index_md` for pipe-delimited
  rows (e.g., a literal `|` inside inline code spans within a Decisions Log
  cell could misalign columns). This is a pre-existing, accepted class of
  approximation in this file, not a new risk introduced by this design.

### (b) Quoted-literal / delimiter exclusion — rejected as primary

Skip a `D-NNN` match if it is immediately enclosed in a matching pair of `"`
or `` ` `` characters. This would correctly exclude 3 of the 4 confirmed
false-positive tokens (the quoted ones), but **not** the 4th — `... as
decision D-2026 and falsely ...` at STATE.md line 208, which is bare prose
with no delimiter at all. Confirmed by direct inspection: `prevchar=' '` for
that occurrence, no quote or backtick anywhere nearby. Since the whole point
of this design is to close the narrative-literal class *permanently*, and
unquoted narrative mentions of a `D-NNN` token are exactly as likely to
recur as quoted ones (arguably more likely — this file's own text bounces
between both styles when discussing itself), (b) alone leaves a known,
already-reproduced hole open. It could be layered on top of (a) for
defense-in-depth, but it is not sufficient standalone and is not needed once
(a) is in place (max_in_file no longer scans free narrative at all).

### (c) Plausibility bound — rejected, dead end

Reject implausibly large `D-NNN` values (e.g., "looks like a year," or "more
than Nx the current known max"). The task's own framing already identifies
the fatal flaw: **real decisions will reach D-2000+ well within this
project's observed cadence.** Empirically, the live decision count is
already at D-1163 as of 2026-09-05, with a demonstrated historical velocity
of roughly 3-10 decisions per active session/day across the F5 and
brownfield cycles (visible directly in the Decisions Log and burst-log
history). At that rate D-2026 is plausibly less than a year away — a
"year-like value is suspicious" heuristic would need to be re-tuned or
retired almost immediately, and a "relative to current max" heuristic
(reject anything >Nx above the last confirmed max) reintroduces exactly the
kind of magic-number/threshold smell the project's own CLAUDE.md production-
grade principle flags as a rationalization, not an engineering decision. No
variant of (c) is durable. Rejected outright, not adopted even as a
secondary layer.

### (d) Combination — adopted in the narrow sense described in §4

The actual recommendation combines (a) as the structural fix for
`max_in_file`, retaining the existing word-boundary guard (a lightweight
form of the PR-#813-shipped fix) unchanged for `max_cited`'s scan of
`current_step:`. This is not "(a)+(b)" — no delimiter-exclusion logic is
added — it is "(a) for the body-wide comparator, existing-guard-unchanged
for the frontmatter-field comparator," because the two scans have different
risk profiles (see §4).

## 4. Why `current_step:`'s scan (`max_cited`) is left unchanged

`current_step:` is a single short, deliberately-authored frontmatter value
(per D-441(a) "verbatim-strict," it is explicitly NOT supposed to contain
narrative meta-commentary). It has never produced a narrative-literal false
positive in this project's history, and the existing word-boundary guard
already correctly handles the classes that have actually occurred there
(embedded dates, embedded TD-identifiers). Changing its scan semantics is
unnecessary scope expansion with no reproduced defect to justify it. If a
future burst ever writes a quoted/narrative `D-NNN` mention *inside*
`current_step:` itself, that would itself be a D-441(a) verbatim-strict
violation independent of this hook (current_step is meant to be a terse
dispatch instruction, not narrative), so the correct fix for that
hypothetical is "don't write narrative into current_step," not "harden the
scanner further." No design change is proposed for this leg.

## 5. Precise semantic change for BC-5.39.006 invariant 7 (product-owner to author)

The BC is currently at v1.7 (2026-05-20) and **already does not reflect**
the PR #813 word-boundary guard shipped 2026-09-04 (`5e009dc0`) — a
pre-existing, unrelated spec-drift gap this review surfaced (§6). The
following is the *complete* intended replacement text for invariant 7,
covering both the retroactive PR #813 catch-up and this design's new
narrative-literal hardening, for product-owner to author as a single v1.8
amendment (not two separate amendments — same burst, same root artifact):

**Invariant 7 — replace current v1.7 text with:**

> 7. D-chain currency validation is a two-sided comparison with **different
>    scanning rules on each side**, and runs unconditionally for every
>    cycle (not gated to the F5 engine-discipline cycle; see companion
>    F5-scope backfill in §6/precondition list below).
>
>    **Side A — `max_cited` (from `current_step:` only):** the hook extracts
>    all `D-(\d+)` integers from the `current_step:` value where the
>    character immediately preceding `D` is either absent (start of value)
>    or not an ASCII-alphanumeric byte (word-boundary rule; closes the
>    `RC25-RELEASED-2026` / `TD-VSDD-053` word-internal-embedding class —
>    F-P1-006-B, PR #813). Takes the maximum such integer.
>
>    **Side B — `max_in_file` (from the STATE.md body):** the hook scans
>    **only** rows under the exact `## Decisions Log` h2 heading (up to the
>    next `## ` heading or end-of-file), and within that section considers
>    only lines that are pipe-delimited table rows (trimmed content starts
>    and ends with `|`) that are not separator rows (containing `---`).
>    For each such row, the hook takes the **first** pipe-delimited cell,
>    trims it, and — **only if the trimmed cell is a whole-string match to
>    `D-\d+` (the entire cell, not a substring)** — includes that integer in
>    the candidate set. `max_in_file` is the maximum of that set (0 if the
>    section is absent or contains no matching rows).
>
>    **Rationale for the asymmetry:** `max_cited` must tolerate free prose
>    (`current_step:` legitimately uses forms like "D-chain cite D-1163
>    latest brownfield"), so a lexical word-boundary heuristic is the right
>    tool there. `max_in_file` must instead answer "what is the highest
>    decision this file has actually *codified*" — which is a structural
>    fact, not a lexical one, and is answered precisely and permanently by
>    the Decisions Log ID column: no prose, quoted literal, backtick-quoted
>    token, banner-history mention, or self-referential narrative about a
>    `D-NNN` string (however that string is delimited or preceded) can ever
>    satisfy "is the entire first cell of a `## Decisions Log` row," because
>    that column is populated exclusively by state-manager at decision-
>    codification time (D-448(b)) and by no other authoring path. Closes the
>    narrative-literal class (task-verified: STATE.md's own prose describing
>    a prior instance of this exact bug — `"D-2026"` quoted 3x, `decision
>    D-2026` once, all outside `## Decisions Log` — no longer inflates
>    `max_in_file`).
>
>    If `max_cited` is absent (no `D-\d+` in `current_step:` at all): violation
>    (absent cite). If `max_cited < max_in_file`: violation (stale cite). If
>    `max_cited >= max_in_file`: current (or fail-open if `max_in_file = 0`).

**New Edge Case row (EC-024):**

> | EC-024 | STATE.md narrative body (outside `## Decisions Log`) contains a
> quoted or prose mention of a `D-NNN` token numerically larger than any
> real Decisions Log row (e.g. `"D-2026"`, `decision D-2026`) while
> `current_step:` correctly cites the true latest ID (e.g. `D-1163`) | Continue
> — narrative mentions outside the Decisions Log ID column are excluded from
> `max_in_file` by construction; only whole-cell `D-\d+` matches in the
> first column of `## Decisions Log` rows count |

**Canonical Test Vector addition:** the literal STATE.md excerpt reproduced
in §1.2 of this analysis (4 word-boundary-passing `D-2026` tokens, 3 quoted
+ 1 prose, all outside `## Decisions Log`) should be added verbatim (or as a
faithful minimal excerpt) as a Canonical Test Vector row, citing this
analysis document and PR #813 as prior art.

## 6. Companion backfill (same amendment burst, not a separate design question)

Two additional, already-shipped-but-unspecced behaviors were discovered
while grounding this design against the live BC text. Per this project's own
production-grade default (fix mechanical spec gaps in scope rather than
deferring), product-owner should fold both into the same v1.8 amendment
rather than opening a second BC amendment cycle for them:

1. **F5-cycle-scope gate is entirely undocumented.** `extract_current_cycle`
   / `is_f5_cycle` / `F5_CYCLE_ID` gate `check_index_version_cites` and
   `check_trajectory_tail_length` to `current_cycle:
   v1.0-feature-engine-discipline-pass-1` only, with brownfield and other
   cycles exempt. None of preconditions, postconditions, or invariants in
   BC-5.39.006 v1.7 mention this gate at all. Per §2 above, the gate's
   *behavior* is correct — only its *documentation* is missing. Recommend
   adding a precondition clause naming the gate and confirming (per §2's
   analysis) that `check_forbidden_meta_commentary` and
   `check_d_chain_currency` are explicitly universal/cycle-agnostic by
   design, not omitted by oversight.
2. **PR #813's word-boundary guard on `scan_max_d_nnn`** (already covered
   inside the Side A rewrite in §5) needs no separate write-up beyond what's
   already folded into the invariant-7 replacement text above.

This is a documentation-catch-up, mechanically answerable now (both
behaviors are already fully implemented and tested in code) — it is
explicitly the kind of "TODO for architect" this project's CLAUDE.md
forbids leaving open when the answer is mechanical.

## 7. Is an ADR needed? Is human ratification (POLICY 22) required?

**No ADR, and no mandatory POLICY 22 human ratification for the code fix
itself.** Assessed honestly against direct precedent and actual policy text,
not by default caution:

- **Precedent:** BC-5.39.006's own changelog (v1.0 → v1.7) shows seven prior
  amendments to this exact hook's scan semantics — including a directly
  analogous regex-relaxation (v1.1, "D-chain pattern relaxed to D-(\d+)
  max-extraction") and a directly analogous scope-narrowing clarification
  (v1.3, trajectory-tail semicolon-segment scoping) — none of which went
  through an ADR or POLICY 22 ratification. They were ordinary product-owner
  BC amendments backed by test-writer/implementer Red Gate fixes. PR #813
  itself (the immediately-prior, structurally identical word-boundary fix)
  was merged as a plain `fix(ci)` PR with 3 Red Gate tests and **no** ADR or
  ratification step.
- **Policy text check:** `.factory/policies.yaml` POLICY 22
  (`subagent_report_fidelity_literal_shell`) governs the *fidelity of
  evidence* presented in agent-to-agent reporting and (per its v1.4.23
  extension) in material presented to a human for ratification — it does
  not itself mandate that *this class* of change requires ratification.
  Scanning `policies.yaml` for any hook-plugin-specific ratification
  requirement (`grep -n "hook-plugins\|dispatcher"`) returns no policy
  requiring ADR/ratification for `validate-dispatch-advance` scan-semantics
  changes.
- **Substantive test:** this change does not alter system architecture,
  introduce a new subsystem, change a purity boundary, or make a
  cross-cutting technology choice — the hallmarks that route work to
  architect+ADR in this project's Agent Routing Table. It is a precision
  correction to an existing invariant's *scanning implementation*, scoped
  entirely inside one WASM hook plugin, that a bug already reproduces and a
  BC amendment already has a template for.

**Recommended routing (per CLAUDE.md Companion Principle):** orchestrator
dispatches product-owner for the BC-5.39.006 v1.8 amendment (§5 + §6
combined, one amendment), then test-writer for the Red Gate tests (§8),
then implementer for the code change — an ordinary fix-burst, not an
architect/ADR/ratification path. This design document is the artifact that
lets product-owner author the amendment without re-deriving the analysis.

## 8. Red Gate tests the test-writer must author

All in `crates/hook-plugins/validate-dispatch-advance/src/lib.rs` `#[cfg(test)]`
module, alongside the existing `scan_max_d_nnn` word-boundary tests:

1. **`test_scan_max_decision_log_id_ignores_narrative_and_quoted_mentions`**
   — feed a synthetic STATE.md-shaped string containing (a) a `##
   Decisions Log` section with a real header + rows up to `| D-1163 | ... |`,
   and (b) narrative text *outside* that section containing exactly the
   task-reproduced tokens: quoted `"D-2026"` (x3 equivalent) and bare prose
   `decision D-2026`. Assert the new structural scan function returns
   `1163`, not `2026`.
2. **`test_check_d_chain_currency_no_false_positive_on_narrative_literal`**
   — full reproduction of the live-bug shape: `current_step:` citing
   `D-1163`; `## Decisions Log` section whose max ID-column row is
   `D-1163`; narrative elsewhere in the body containing the 4
   word-boundary-passing `D-2026` mentions. Assert
   `check_d_chain_currency` returns `None`.
3. **`test_check_d_chain_currency_still_flags_stale_via_decisions_log`**
   (positive control) — `current_step:` cites `D-1163`; `## Decisions Log`
   section's max ID-column row is `D-1164`. Assert a violation is returned
   citing `max_in_file=1164`. Confirms the structural narrowing does not
   silently disable genuine staleness detection.
4. **`test_scan_max_decision_log_id_fail_open_on_missing_section`** —
   content with no `## Decisions Log` heading at all. Assert the function
   returns `0` (fail-open preserved, consistent with existing invariant 7
   design philosophy).
5. **Regression guard (no new test needed, re-run existing):**
   `test_scan_max_d_nnn_ignores_d_yyyy_embedded_in_dates` and
   `test_scan_max_d_nnn_ignores_word_internal_td_vsdd_053` must continue to
   pass unmodified — `scan_max_d_nnn` itself (used for `max_cited`) is not
   touched by this design.
6. **Literal-shell verbatim reproduction (recommended, not strictly a unit
   test):** a fixture-backed test embedding the *actual* offending sentences
   from `.factory/STATE.md` (copy-pasted verbatim, not paraphrased) so the
   Red Gate log has load-bearing evidence tying the fix to the exact
   production defect, independent of the production-file integration test
   (which will stop exercising this exact text once STATE.md is next
   compacted).

## 9. Self-validation confirmation (CI green with zero STATE.md edits)

Confirmed by direct reproduction in this session (§3(a) empirical
validation): running the proposed structural scan against the **unmodified,
live** `.factory/STATE.md` content yields `max_in_file = 1163`, which equals
`max_cited = 1163` from the existing unchanged `current_step:` scan. This
satisfies `check_d_chain_currency`'s `max_cited >= max_in_file` pass
condition with no other STATE.md validation gate affected (forbidden
meta-commentary, 4-index-citations, trajectory-tail are all independent
checks, unmodified by this design). Therefore, once the hardened
`scan_max_decision_log_id` lands, `validate_production_state_md_no_false_positive`
passes against STATE.md exactly as it stands today — no narrative edit,
compaction, or content change to `.factory/STATE.md` is required as part of
this fix. (Should the STATE.md content shift before the fix lands — e.g. a
future wrap/pause burst — the same structural argument holds as long as the
Decisions Log table's max ID-column row is not itself artificially altered;
re-verify with the literal-shell reproduction in §1.2/§3(a) at fix time.)

## 10. Summary recommendation

**Primary:** implement (a) — replace the whole-body word-boundary scan used
for `max_in_file` in `check_d_chain_currency` with a new
`scan_max_decision_log_id(content: &str) -> u64` function that scans only
the `## Decisions Log` h2 section's first-column, whole-cell `D-\d+` matches.
Leave `scan_max_d_nnn` (word-boundary guard, PR #813) unchanged and continue
using it for `max_cited` extraction from `current_step:`. Reject (b) as
insufficient standalone (misses the bare-prose case) and (c) as a dead end
(year-collision is near-term plausible given current decision velocity).

**No ADR. No mandatory POLICY 22 ratification.** Route as an ordinary
product-owner BC-5.39.006 v1.8 amendment (§5+§6 combined) →
test-writer Red Gate tests (§8) → implementer fix, per the standard
Agent Routing Table fix-burst path, consistent with 7 prior precedent
amendments to this exact BC and with PR #813 itself.
