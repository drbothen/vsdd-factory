---

```yaml
---
document_type: adversarial-review
level: adversary
version: "1.0"
status: complete
story_id: S-21.07
pass: 3
cycle: v1.0-brownfield-backfill
producer: adversary
timestamp: "2026-08-04T00:00:00Z"
phase: 3
inputs:
  - stories/S-21.07-validate-cross-site-correspondence.md
  - specs/behavioral-contracts/ss-05/BC-5.39.010.md
  - specs/architecture/decisions/ADR-035-cross-site-correspondence-validation-three-tier-architecture.md
  - specs/architecture/decisions/ADR-037-input-hash-stable-input-constraint-volatile-artifacts-excluded.md
  - specs/behavioral-contracts/BC-INDEX.md
  - stories/STORY-INDEX.md
  - policies.yaml (v1.4.19 rubric, supplied in dispatch)
input-hash-basis: "reviewed feature tree 6854a951 (worktree .worktrees/S-21.07) + factory-artifacts as of 2026-08-04; story v1.3 input-hash 9603a5b; BC v1.6 input-hash ae99a83"
traces_to: "BC-5.39.010 v1.6; S-21.07 v1.3"
reviewed_head: "6854a951"
reviewed_code_head: "6854a951"
story_version: "1.3"
bc: "BC-5.39.010 v1.6"
verdict: NOT-CLEAN
novelty: high
findings_count: 25
severity_breakdown: "B3/H7/M12/L3"
observations_count: 5
streak: "0/3"
passes: 3
previous_review: "cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-2.md"
trajectory_append: 25
dispatch_shape: "HOLISTIC — one unified fresh-context adversary (same shape as pass-2)"
asymmetry_enforcement: "Read prior-pass Part A finding sets only (adversary-pass-1.md grepped for duplicate-detection anchors; adversary-pass-2.md finding list). No fix-burst closure report read. No prior-pass Part B verdict reasoning consumed."
execution_limits: "Read-only tool profile (Read/Grep/Glob). Could NOT execute cargo test / bats / git. The claimed 108-pass/0-fail/2-ignored crate result and bats 43/43 are NOT independently verified in this pass; all findings below are derived from source and corpus reading, not from test execution."
---
```

# Adversary Pass 3 — S-21.07 LOCAL cascade (BC-5.39.010 3-CLEAN)

**Reviewed HEAD:** `6854a951` (`feature/S-21.07-validate-cross-site-correspondence`) · **Story** v1.3 · **BC-5.39.010** v1.6
**Verdict:** **NOT-CLEAN** — B3 / H7 / M12 / L3 = 25 findings + 5 observations · **Streak: 0/3**

---

## Finding ID Convention

`F-S2107-P3-NNN` (this pass); observations `O-P3-NN`. Matches pass-1/pass-2 convention.

---

## 1. Part A — Pass-2 Closure Verification

### CLOSED (independently verified)

- **F-P2-001** — `arm_a2.rs::extract_story_bc_version_citations` now has `let mut skip_section = true;` unconditionally, with no `has_headings` conditional. The stale docstring that described unrestricted scanning of heading-free content has been corrected. Load-bearing tests present (`..._frontmatter_preamble_not_scanned_skip_section_true`, `..._heading_free_story_yields_zero_citations`, plus the S-21.04 corpus test). **CLOSED.**
- **F-P2-002** — `extract_bc_index_version` now anchors via `line.splitn(3, '|')` and `first_cell.contains(bc_id)`; the stale "any-cell" docstring is corrected. Unit + corpus red gates present. **CLOSED** for the cross-reference-row defect specifically (but see **F-S2107-P3-001** — the same function has a second, larger defect that pass-2 did not reach).
- **F-P2-003 / F-P2-008 / F-P1B-013** — `is_canonical_vp_filename` implements `^VP-[0-9]+\.md$`; flat `verification-properties/` path (no `ss-*`). Corpus red gate + complement guard present. **CLOSED** on behaviour (see F-S2107-P3-011 for the missing spec-mandated redundant guard).
- **F-P2-011** — `is_canonical_story_basename` requires a numeric section before and a digit after the first dot; `S-README.md` red gate present. **CLOSED** in `dispatch.rs` (see F-S2107-P3-015 for the un-swept sibling).
- **F-P2-013** — T-046 MUTANT added at `validate-cross-site-correspondence.bats:1148`, asserting exit 2 with `[Class E1]` on a genuine VP version/last_amended mismatch. Genuinely load-bearing (an inert Class E now fails T-046). **CLOSED.**
- **F-P1C-016 / AC-018** — `test_BC_5_39_010_invariant_7_ac018_multi_arm_violations_both_in_combined_block` added at `lib.rs`. **CLOSED.**
- **F-P2-014** — story `## Token Budget Estimate (MANDATORY)` row now reads `ADR-035 (§Decisions 1-5 + Rationale)` with no version pin. **CLOSED.**
- **F-P2-007 (Class D route)** — `is_cycle_artifact` returns `None` unconditionally; the arm_d dispatch block is removed from `lib.rs`; `.factory/cycles/` is removed from registry `path_allow`. The BC-v1.6 clause "the `is_cycle_artifact` dispatch branch MUST NOT be compiled into the hook" is satisfied at the dispatch level. **CLOSED at dispatch level** — but the fail-open error arm the finding was actually about was retained; see **F-S2107-P3-008**.
- **PC40** — `is_volatile_path` + `parse_story_volatile_inputs` exist and `run_arm_b1` checks them after PC18 and before PC19. T-047 bats test is load-bearing (exercises the real dispatcher with a deliberately mismatched STORY-INDEX hash and asserts exit 0 + a `volatile` advisory in the log). **Structurally CLOSED; substantively NON-CONFORMANT** — see **F-S2107-P3-002**.

### NOT CLOSED (carried forward)

| Pass-2 finding | Status | Pass-3 ID |
|---|---|---|
| **F-P2-006** (HIGH) `line.contains(bc_id)` — the construct PC13 explicitly forbids | **still present**, `arm_a2.rs::extract_story_bc_version_citations` | F-S2107-P3-004 |
| **F-P2-009** (HIGH) `CHANGELOG.md` `[Unreleased]` empty; Task 20 gates PR creation | **still empty** (placeholder comment only) | F-S2107-P3-007 |
| **F-P2-016** (LOW) duplicate `T-038 CONTROL` bats test | **both still present** (bats:668 and bats:1077) | F-S2107-P3-018 |
| **F-P2-017** (LOW) stale `BC-5.39.010 v1.2` cites on 5 code/config surfaces | **zero of ~8 sites fixed**, and now materially wrong (advertise descoped Class D) → re-severitised HIGH | F-S2107-P3-010 |
| **F-P2-018** (LOW) false comment "STORY-INDEX.md: no `version:`/`last_amended:` frontmatter" | **still present** at `lib.rs` `if is_si` branch; STORY-INDEX.md has `version: "4.282"` (line 4) and `last_amended:` (line 8) | F-S2107-P3-019 |
| **VP-039 fixture ID reuse** (pass-2 known-open confirmation) | **partially closed** — the two `VP-9999-test.md` files were renamed, but the original `VP-039.md` fixture remains as dead residue | F-S2107-P3-020 |

### Corpus claims re-verified this pass

- BC-INDEX body-table rows matching `^\| \[BC-[0-9]+\.[0-9]+\.[0-9]+\]`: **1983**. Rows carrying any `| v[0-9]+\.[0-9]+` cell: **40**. Rows terminating in a bare `| vN.N |` cell: **21**.
- BC files with `version: "1.0"` under `.factory/specs/behavioral-contracts/`: **231**.
- `^last_amended: |` (YAML block scalar) across the whole corpus: **exactly 2 files** — `ss-05/BC-5.39.010.md` and `stories/S-21.07-validate-cross-site-correspondence.md`.
- `^## Behavioral Contracts[^ (]` / `^## Token Budget[^ (]` in `.factory/stories/*.md`: **0 matches** — `is_target_heading`'s `' '`/`'('`/EOL boundary set is corpus-conformant. No finding.
- `run-all.sh` globs `tests/*.bats`, so the new suite is auto-collected. Task 16 is structurally satisfied. No finding.
- ADR-035 §Decision 1/§Consequences scope Class D only as "Class D-semantic" routed to Tier 3; it never scoped the Class D format arm into Tier 2A (line 249: "BC-5.39.010 v1.1 Parts A, B, and E are confirmed as the correct Tier 2A + Tier 1"). The v1.6 descope is **architecturally coherent**; no ADR amendment is owed. No finding.

---

## 2. Part A — New Findings

### F-S2107-P3-001 — BLOCKER

**Policy/Precondition:** PC5 + postcondition 4; POLICY 14 leg 5; TD-VSDD-059
**Location:** `arm_a1.rs::extract_bc_index_version` + `run_arm_a1_with_index_result` `None` branch
**Confidence:** HIGH

`extract_bc_index_version` returns `Option<String>`, and `run_arm_a1_with_index_result` treats `None` as **"the BC has no row in BC-INDEX"**, emitting for any `version != "1.0"`:

> `'{bc_id}' (v{bc_version}) has no row in BC-INDEX.md — previous registration appears to have been dropped.`

But `None` is also returned when the row **is present and correctly anchored** and simply carries **no version cell**. The two conditions are conflated, and the second is the overwhelming corpus majority.

**Corpus evidence:**

- BC-INDEX body table header is 5-column: `| BC ID | Title | Status | Capability | Stories |` (`BC-INDEX.md:539`). The version-chain cell is an ad-hoc 6th column present on only **40 of 1983** rows.
- `BC-INDEX.md:541` — `| [BC-1.01.001](ss-01/BC-1.01.001.md) | Registry rejects unknown schema version | draft | CAP-TBD | S-15.01 |`. No cell contains a `v`-prefixed token at a word boundary (`extract_version_token` requires `b'v'` + digit), so the function returns `None`. `ss-01/BC-1.01.001.md:4` → `version: "1.2"`. `"1.2" != "1.0"` → **BLOCK on every write to BC-1.01.001**.
- `BC-INDEX.md:656` — `| [BC-1.14.001](...) | ... | draft | CAP-002 | S-15.01 |`, no version cell; `ss-01/BC-1.14.001.md:4` → `version: "1.12"` → **BLOCK**.

**Blast radius:** ≥1,700 BC files (1983 rows − 40 with version cells − at most 231 at v1.0). Every write to any of them produces a false blocking violation with a *false diagnosis* ("registration dropped") that would direct the fixer to re-add a row that already exists.

**Spec side (routes to product-owner):** PC5's "The last non-empty pipe-delimited column is the version cell" is false for ~98% of live rows, and postcondition 4's premise ("version > 1.0 means the BC was previously registered; an absent row is a structural fault") cannot be evaluated from a `None` that does not distinguish absent-row from absent-cell. This is the **seventh** spec-describes-imagined-shape instance in this story, and the first one the newly added corpus tests should have caught but do not: `corpus_arm_a1` picked BC-1.17.001, one of the 40 rows that *does* carry a version chain.

**Fix:** make the extractor return a three-state result (`RowAbsent` / `RowPresentNoVersion` / `Version(v)`); `RowPresentNoVersion` must not take the postcondition-4 block path. Amend PC5 to state what the corpus actually is.

---

### F-S2107-P3-002 — BLOCKER

**Policy/Precondition:** PC40 + ADR-037 §Decision 2; POLICY 4; D-950 / `L-BB-spec-drift-by-widening`
**Location:** `arm_b.rs::is_volatile_path`
**Confidence:** HIGH

ADR-037 §Decision 2 (`ADR-037-...md:60-71`) and BC-5.39.010 PC40's mirrored table enumerate **exactly six** volatile patterns, matched by "string equality and suffix checks only":

`.factory/STATE.md` · `.factory/cycles/**/STATE.md` · `.factory/cycles/**/{decision-log,lessons,burst-log}.md` · `.factory/stories/STORY-INDEX.md` · `.factory/specs/behavioral-contracts/BC-INDEX.md` · `.factory/specs/architecture/ARCH-INDEX.md`

The implementation diverges in **three** directions simultaneously:

**(a) `ARCH-INDEX.md` is MISSING (drift-by-narrowing) — live false BLOCK on the story's own file.**
`is_volatile_path` matches only `BC-INDEX.md | VP-INDEX.md | STORY-INDEX.md`. ARCH-INDEX.md is never matched. Corpus:
- `S-21.07-validate-cross-site-correspondence.md:18` → `  - .factory/specs/architecture/ARCH-INDEX.md`
- Same file `:24` → `input-hash: "9603a5b"`
- `STORY-INDEX.md:730` catalog row → `input-hash 52f0bf3`
- `STORY-INDEX.md:733` blockquote → `S-21.07=52f0bf3`

→ `volatile_found` is empty → Arm B1 proceeds → `B1=9603a5b`, `B2=B3=52f0bf3` → **Class B BLOCK on every write to S-21.07 itself**. This is precisely the self-locking failure mode PC40's transitional clause was authored to prevent, reproduced on the deliverable that authors the gate. `^  - \.factory/specs/architecture/ARCH-INDEX\.md` matches **66 files** under `.factory/stories/`.

**(b) Blanket `.factory/cycles/**` (drift-by-widening) — permanent suppression of a BLOCKING gate beyond spec.**
The final clause returns `true` for *any* path with a `cycles` component. ADR-037 exempts only four filenames under `cycles/`. Immutable historical cycle artifacts are therefore wrongly treated as volatile, permanently suppressing Class B for stories such as:
`S-14.04:14` `adv-cycle-pass-1.md` · `S-14.05:14` `adv-cycle-pass-1.md` · `S-14.06:14` `adv-cycle-pass-5.md` · `S-14.08:14` `adv-cycle-pass-3.md` · `S-13.01:24` `F1-delta-analysis.md` · `S-12.03:24` / `S-12.05:23` `F5-pass-2-architect-decisions.md` · `S-21.03:15` / `S-21.05:15` `e-21-arch-delta-analysis.md` · `S-8.09:19` `adv-s8.09-p1.md` · `S-8.10:16-17` `adv-s8.04-p2.md` / `adv-s8.09-p2.md` · `S-15.10/11/15` `s-15.03-wave-plan-2026-05-15.md` · `S-7.01:16` `adversarial-reviews/s6.01-pass-1.md` — ~20 stories. This directly violates PC40's own guarantee: *"This clause imposes no permanent weakening."* An adversary-pass report never changes; there is no volatility rationale.

**(c) `VP-INDEX.md` added, and index files matched anywhere under `.factory/`.**
`VP-INDEX.md` appears in neither ADR-037 §Decision 2 nor PC40. PC40 specifies "path equals … exactly" for the three index files; the implementation matches them at any depth.

PC40 also requires the list be implemented "as a compile-time constant slice" kept "in sync with ADR-037 §Decision 2"; it is inlined `matches!` arms with no constant and no cross-reference, which is what allowed the three-way drift.

**Test coverage:** the only PC40 test (`test_..._pc40_volatile_input_detection_required` + bats T-047) covers `.factory/STATE.md` — **1 of 6 spec rows**. Neither (a), (b), nor (c) is detectable by any test in the tree.

**Fix:** replace with a `const VOLATILE_PATTERNS` slice transcribed 1:1 from ADR-037 §Decision 2; add per-row unit tests including an ARCH-INDEX positive and a `.factory/cycles/**/adv-cycle-pass-1.md` **negative**.

---

### F-S2107-P3-003 — BLOCKER

**Policy/Precondition:** POLICY 18 (D-923); POLICY 14 leg 5; POLICY 7; invariant 11
**Location:** `.factory/stories/S-21.07-...md:24`; `.factory/stories/STORY-INDEX.md:730`, `:733`
**Confidence:** HIGH

The story's own three Class B sites disagree, and its STORY-INDEX row is stale on two further axes. This divergence was **introduced by this burst** — pass-2 recorded S-21.07's three legs as mutually equal at `52f0bf3`; story v1.3 advanced the frontmatter hash to `9603a5b` without the same-burst STORY-INDEX sweep POLICY 14 leg 5 requires.

- B1 `input-hash: "9603a5b"` vs B2 catalog `input-hash 52f0bf3` vs B3 blockquote `S-21.07=52f0bf3`.
- `STORY-INDEX.md:730` Title cell: `validate-cross-site-correspondence WASM hook — **seven-arm** PostToolUse cross-site value-correspondence gate (Classes **A/B/D/E**; BC-5.39.010 **v1.4**)` — the story title (frontmatter `title:` and H1) reads `**six-arm** … BC-5.39.010 **v1.6** Classes **A/B/E**; Class D deferred`. POLICY 7 title-sync + POLICY 14 leg 5 upstream-index cell sync both broken.
- Same row's BC-version cell: `[BC-5.39.010 v1.4]` — BC is v1.6.

Consequence once shipped (and independent of F-S2107-P3-002(a)): every write to `S-21.07-...md` and to `STORY-INDEX.md` interacts with a live Class B disagreement. Per invariant 11 the fix team must first adjudicate stale-vs-fabricated for `9603a5b` (POLICY 18 requires an `bin/compute-input-hash` invocation with captured stdout — POLICY 18 / POLICY 22, not a narrative assertion) before any `--update`.

**Route:** state-manager (STORY-INDEX row + hash sweep); story-writer confirms the Title cell verbatim against the story H1.

---

### F-S2107-P3-004 — HIGH [carried, F-P2-006 unclosed]

**Policy/Precondition:** PC13 (normative MUST NOT)
**Location:** `arm_a2.rs::extract_story_bc_version_citations`
**Confidence:** HIGH

PC13: *"The BC ID is present as an **exact word-boundary token** … A line ending `| BC-5.39.010 EC-001 |` does NOT match … **Implementations MUST NOT use a plain `line.contains(bc_id)` test.**"*

The implementation is `if !line.contains(bc_id) { continue; }` — verbatim the forbidden construct, unchanged from pass-2. It is absent from the red-gate-log's §Finding Coverage table, so no red gate was written for it and it was silently dropped from the burst.

Live consequences within the two bounded sections: (i) prefix collisions (`contains` admits any superstring); (ii) a single row citing two BC IDs is attributed the same rightmost version token for both, so one of the two comparisons is against the wrong datum.

**Fix:** implement the `\b`-on-both-sides token test PC13 specifies, and add the PC13 worked counter-example (`| BC-5.39.010 EC-001 |` → no citation) as the red gate.

---

### F-S2107-P3-005 — HIGH

**Policy/Precondition:** PC15; invariant 5 ("blocking without exception"); SOUL.md #4
**Location:** `arm_a2.rs::run_arm_a2_for_bc_with_result`
**Confidence:** HIGH

```rust
// Empty citations → skip (postcondition 8)
if citations.is_empty() {
    return (vec![], vec![]);
}
match bc_read_result { … Err(other) => /* block */ … }
```

The `citations.is_empty()` early return precedes the `bc_read_result` match, so a `HostError::CapabilityDenied` on a cited BC file is **silently discarded** for every BC that has no version-citing row. PC15: *"`HostError::CapabilityDenied` on any BC file: `block_with_fix(...)` … sandbox misconfiguration is blocking regardless of whether the target is primary."* Invariant 5: *"`CapabilityDenied` on any secondary target is blocking — sandbox misconfiguration is never a legitimate state."*

Postcondition 8 governs the **verdict** for a BC with no citations, not the disposition of an I/O fault. This is the identical fail-open shape BC v1.6 §Deferred Scope names as non-conforming for Class D, reproduced in Arm A2. Note that `run_arm_a2_for_bc` performs the read *before* delegating to the seam that early-returns on empty citations, so the error is genuinely produced and then dropped.

Reachable today: EC-029-shaped stories (BC cited in prose only) are common.

**Fix:** match the read result first; return the CapabilityDenied violation regardless of citation count. Add a unit test with `citations = []` and `Err(HostError::CapabilityDenied)` asserting a non-empty violation vector.

---

### F-S2107-P3-006 — HIGH

**Policy/Precondition:** PC36/PC37; postconditions 19-20; POLICY 14 leg 4 / POLICY 17
**Location:** `frontmatter.rs::extract_frontmatter_field` → `arm_e.rs::run_arm_e1`
**Confidence:** HIGH

`extract_frontmatter_field` has no YAML block-scalar handling. For `last_amended: |-` it matches the prefix, takes `rest = " |-"`, trims to `"|-"`, and returns `Some("|-")`. `extract_last_amended_outer_version("|-")` hits `if len < 14 { return None }` → `run_arm_e1` emits the *unparseable format* advisory and Continues. **Class E1 never evaluates the field.**

Corpus: `^last_amended: |` matches **exactly two files in the entire repository** — `.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md:11` and `.factory/stories/S-21.07-validate-cross-site-correspondence.md:11`. Those are the story's own governing BC and the story itself: the hook is structurally blind to E1 parity on precisely the two artifacts this story delivers and edits most.

Because the failure mode is an advisory, exit 0 is indistinguishable from "E1 ran and found no violation" — the same false-green class as the inert-scope condition PC31a was invented to surface. Note that **corpus test 5 selected `VP-100.md`**, which uses the inline form, so the newly added corpus coverage does not touch this path.

**Fix:** handle `|`, `|-`, `>`, `>-` block scalars (accumulate indented continuation lines) in `extract_frontmatter_field`; add a corpus test that reads live `BC-5.39.010.md` and asserts `extract_last_amended_outer_version(last_amended) == Some(version)`.

---

### F-S2107-P3-007 — HIGH [carried, F-P2-009 unclosed]

**Policy/Precondition:** S-21.07 Task 20 (MANDATORY); BC-8.31.001; CLAUDE.md story-template mandate
**Location:** `CHANGELOG.md` `## [Unreleased]` (lines 3-7)
**Confidence:** HIGH

`## [Unreleased]` still contains only the placeholder HTML comment. Zero occurrences of `cross-site-correspondence` or `S-21.07` anywhere in `CHANGELOG.md`. Task 20 — *"Add a CHANGELOG entry under [Unreleased] > Added describing the shipped behavior **before creating the PR**"* — is an explicit PR-creation gate that remains unsatisfied after a full fix burst.

---

### F-S2107-P3-008 — HIGH

**Policy/Precondition:** BC-5.39.010 §Deferred Scope "Non-conforming pattern (TD-VSDD-059)"; invariant 6 adjudication; SOUL.md #4
**Location:** `lib.rs::on_post_tool_use` — `Err(e) if cycle_kind.is_some()` arm, and the `[DEFERRED v1.6 — Class D]` comment block that follows
**Confidence:** HIGH

BC-5.39.010 §Deferred Scope, verbatim:

> **Non-conforming pattern (TD-VSDD-059):** an implementation using a single catch-all error arm (e.g., `Err(e) if cycle_kind.is_some()` → `log_warn` + Continue) to swallow all `HostError` variants on the grounds that "Class D is advisory-only" is non-conforming to this invariant's explicit scope. … **This two-path distinction MUST be carried forward intact** when Class D is implemented in the follow-up story (S-21.08).

The burst retained that exact arm, byte-for-byte the pattern the BC names. It is now unreachable dead code (`cycle_kind` is always `None`), so it is not a live defect — the defect is that it is **preserved as the template for re-enablement**:

- The arm's own comment still asserts the adjudicated-wrong rationale: *"NotFound → advisory + Continue per PC33; any other error → advisory (invariant 6)."* The v1.5 Amendment 3 adjudication established that `CapabilityDenied`/`Timeout` are **blocking** and that invariant 6 does not reach the I/O path.
- The dispatch-site comment instructs: *"the cap-selection and primary-read-error branches above are kept intact so re-enabling Class D only requires restoring `is_cycle_artifact` body + adding back this dispatch block."* Following that instruction re-introduces the non-conforming fail-open verbatim, which is the opposite of "carry the two-path distinction forward intact."

**Fix:** delete the guarded arm; leave a comment citing BC-5.39.010 §Deferred Scope's two-path requirement (`NotFound` → advisory; `CapabilityDenied`/`Timeout` → block) as the shape S-21.08 must implement.

---

### F-S2107-P3-009 — HIGH

**Policy/Precondition:** PC34 bullet 4; invariant 3; D-950 drift-by-widening
**Location:** `dispatch.rs::is_frontmatter_parity_target` — epic arm
**Confidence:** HIGH

PC34 bullet 4 requires **both** conditions and a basename regex:

> Under `.factory/stories/epics/` (both `stories` and `epics` path components present, each verified via `components().any(…)`, component-strict) with basename matching `^E-[0-9]+-.*\.md$`

The implementation requires only `.factory` + `epics` components and `filename.ends_with(".md")`. Neither the `stories` component nor the `^E-[0-9]+-` basename predicate is checked. The docstring compounds it: `// Epic files under .factory/epics/` — the wrong path (real location verified: `.factory/stories/epics/E-1-dispatcher-foundation.md`, …, `E-21-factory-state-data-loss-hardening.md`).

Consequence is not benign: `.factory/<anything>/epics/*.md` classifies as a Class E target → `lib.rs` performs the primary read → those paths are outside registry `path_allow` (`.factory/specs/behavioral-contracts/`, `.factory/specs/verification-properties/`, `.factory/stories/`) → `CapabilityDenied` → the non-cycle `Err(e)` arm **fail-closed BLOCKs**. Also admits `.factory/stories/epics/README.md`-class non-epics.

This is the same widening class as D-950 (`is_ascii_hexdigit`) that the crate elsewhere guards against with explicit "do NOT widen" comments in `arm_b.rs`. No test covers the epic arm at all — `is_story_file` has an epics-exclusion test (`..._dispatch_epic_file_not_story_file`), but there is no positive/negative test for `is_frontmatter_parity_target`'s epic arm.

---

### F-S2107-P3-010 — HIGH [carried, F-P2-017 unclosed; blast radius 6 files / 9 sites]

**Policy/Precondition:** TD-VSDD-060 sibling sweep; partial-fix regression discipline; documentation-vs-code
**Confidence:** HIGH

Pass-2 named five code/config surfaces carrying stale `BC-5.39.010 v1.2` cites. **Zero were fixed**, and the burst made them materially wrong by descoping Class D while leaving Class D documented as a shipping arm:

| Site | Text |
|---|---|
| `src/lib.rs:15` | `BC-5.39.010 v1.2 — five-arm PostToolUse cross-site value-correspondence gate` (wrong version **and** wrong arm count) |
| `src/lib.rs:10` | `**Class D (cycle artifact write):** finding-ID namespace format advisory (NEVER blocks).` — descoped |
| `src/lib.rs:66-70` | `Violation` doc claims it carries "the arm `class` for message prefixing"; the struct has only `description` |
| `src/lib.rs:75` | `Advisory` doc lists "Class D finding-ID namespace format anomalies (invariant 6: advisory-only)" — descoped |
| `src/main.rs:11` | lists `arm_d` as a live arm module |
| `src/main.rs:14`, `:19` | `# Compliance notes (BC-5.39.010 v1.2)`; `(BC-5.39.010 v1.2 §Gate Spec)` |
| `Cargo.toml:9` | `description = "… and finding-ID namespace format (BC-5.39.010 v1.2 Classes A/B/D/E)"` — the crate's published description advertises a descoped class |
| `hooks-registry.toml:666`, `:681` | `BC-5.39.010 v1.2 §Gate Spec` |
| `hooks-registry.toml:667-671` | `# Six arms: … Class D (finding-ID namespace format advisory — advisory-only, never blocks) …` |
| `tests/…bats:38`, `:305` | `Governing BC: BC-5.39.010 v1.2`; `(BC-5.39.010 v1.2 §Gate Spec)` |

Severity is HIGH, not LOW: an implementer reading `Cargo.toml`, `lib.rs`, `main.rs`, or the registry comment would conclude Class D ships. This is the identical failure the burst *did* correct in two arm docstrings — the sweep stopped at the two docstrings the pass-2 finding happened to name.

---

### F-S2107-P3-011 — MEDIUM

**Policy/Precondition:** PC34 bullet 2 (normative "REQUIRED")
**Location:** `dispatch.rs::is_canonical_vp_filename`
**Confidence:** HIGH

PC34: *"The `[0-9]+` digit requirement naturally excludes `VP-INDEX.md`; **an explicit basename guard `file_name() != "VP-INDEX.md"` is REQUIRED for defence-in-depth**, because `starts_with("VP-") && ends_with(".md")` admits VP-INDEX.md."*

The implementation ships only the digit predicate. Behaviour is correct today; the spec-mandated redundant guard is absent, so a future relaxation of the digit predicate silently re-admits `VP-INDEX.md`. The two tests present (`..._vp_index_excluded_from_class_e`, `..._corpus_dispatch_vp_index_excluded_from_class_e_live_path`) assert the behaviour, not the guard, and would pass with the guard still missing.

---

### F-S2107-P3-012 — MEDIUM

**Policy/Precondition:** PC40 ("the prescribed message from ADR-037 §Decision 4"); ADR-037 §Decision 5
**Location:** `arm_b.rs::run_arm_b1` volatile advisory
**Confidence:** HIGH

PC40 and ADR-037 §Decision 4 point 2 prescribe the advisory verbatim:

> `"Story <id> has volatile inputs per ADR-037 §Decision 2 — three-way equality is unsatisfiable until story-writer removes volatile inputs and state-manager recomputes the hash; Class B BLOCK suspended. Volatile path(s): <list>"`

Shipped text: `"…has volatile inputs {volatile_found:?} — skipping three-way input-hash comparison per BC-5.39.010 v1.6 PC40. Volatile paths do not produce stable hashes. **Update input-hash manually when non-volatile inputs change.**"`

No `ADR-037 §Decision 2` cite, no `Class B BLOCK suspended`, and the remediation instruction is wrong in a way that will produce bad operator behaviour: ADR-037 §Decision 5 remediation is *story-writer removes the volatile input, state-manager recomputes* — never a manual hash edit (which is exactly the fabricated-hash provenance break invariant 11 warns about). T-047's assertion is `grep -qi 'volatile'`, which cannot detect the divergence.

---

### F-S2107-P3-013 — MEDIUM

**Policy/Precondition:** POLICY 11; test hermeticity; TD-VSDD-059
**Location:** `lib.rs` tests — `live_factory_root`, `corpus_arm_a1…`, `corpus_arm_a2…`, `corpus_arm_e1…`
**Confidence:** HIGH

The corpus tests conflate two distinct propositions: *"the extractor is correct"* and *"the live corpus is currently clean."* They assert the second and attribute failures to the first.

- `corpus_arm_a2…` iterates every S-21.04 BC-6.26.001 citation and asserts equality with live BC frontmatter, with the failure message *"Extractor returned a stale or phantom citation."* A **genuine** stale citation in S-21.04 — the exact defect Arm A2 exists to detect — fails this test with a false diagnosis and breaks `cargo test --workspace` for an unrelated factory-artifacts burst.
- `corpus_arm_e1…` self-admits the ambiguity in its own message: *"Extractor has a parsing bug **OR** VP-100.md has a live E1 violation (hook failure)."* An assertion that cannot distinguish its two failure causes is not a regression test.
- `live_factory_root()` walks up from `CARGO_MANIFEST_DIR`. From the story worktree (`/…/.worktrees/S-21.07/crates/hook-plugins/validate-cross-site-correspondence`) the ascent reaches the **main checkout's** `.factory/` — i.e. the tests validate a tree that is *not part of the reviewed commit* and is not pinned by any SHA. The `is_real_corpus` guard (requires `specs/behavioral-contracts/`) prevents resolving to a worktree stub, but does nothing to prevent resolving to a *different revision* of the corpus than the one under review.

Combined with the hardcoded artifact names (`BC-1.17.001.md`, `BC-6.26.001.md`, `S-21.04-story-worktree-write-path-discipline.md`, `VP-039.md`, `VP-100.md`), the predictable outcome is rot, and the predictable response to rot is weakening the assertions.

**Fix:** split each corpus test into (a) a *shape* assertion that is invariant under corpus churn (e.g. "the extractor returns the value from the first-cell-anchored row", verified by locating that row independently in the test) and (b) an explicit, separately-named corpus-cleanliness check that is allowed to fail loudly and is attributed correctly.

---

### F-S2107-P3-014 — MEDIUM

**Policy/Precondition:** TD-VSDD-059 (root-cause closure claim); POLICY 11
**Confidence:** HIGH

Judged independently, the 5 corpus tests are **not** a token gesture — three are genuine red gates that caught real defects (including one the test-writer mis-predicted, per the red-gate-log's own §Note). But the root cause is **materially not closed**, because coverage is thin exactly where the live blast radius is largest:

| Extractor / predicate | Corpus coverage |
|---|---|
| `extract_bc_index_version` | 1 row shape (BC-1.17.001 — one of the 40 rows *with* a version chain). The 1,943-row no-version-cell class is uncovered → **F-S2107-P3-001** survived |
| `extract_story_bc_version_citations` | 1 story (S-21.04) |
| `is_frontmatter_parity_target` | 2 VP paths; BC/story/**epic** arms uncovered → **F-S2107-P3-009** survived |
| `extract_last_amended_outer_version` | 1 VP (inline `last_amended`); block-scalar form uncovered → **F-S2107-P3-006** survived |
| `parse_story_index_catalog_hash` / `parse_story_index_blockquote_hash` / `run_arm_b2` | **zero** corpus coverage against live STORY-INDEX.md |
| `is_volatile_path` / `parse_story_volatile_inputs` | **zero** corpus coverage → **F-S2107-P3-002** survived |
| `extract_frontmatter_field` / `extract_frontmatter_sequence` | **zero** corpus coverage |

All three of this pass's BLOCKERs sit in the uncovered set. A single additional corpus test — "for every `.md` under `.factory/stories/`, `is_volatile_path` agrees with the ADR-037 pattern list" or "for a sample of 20 BC files, `extract_bc_index_version` does not report `RowAbsent`" — would have caught two of them.

---

### F-S2107-P3-015 — MEDIUM

**Policy/Precondition:** TD-VSDD-060 sibling-site sweep; PC9/PC16
**Location:** `arm_b.rs::extract_story_id_from_table_row`
**Confidence:** HIGH

F-P2-011 tightened the story-ID predicate in `dispatch.rs` to require a numeric section. The sibling story-ID predicate in `arm_b.rs` was not swept:

```rust
if id.starts_with("S-") { Some(id) } else { None }
```

Arm B2 therefore ingests any STORY-INDEX first cell beginning `S-` into its catalog map (`S-README`, `S-ARCH`, `S-TBD`), while `extract_blockquote_pairs` correctly uses `parse_story_id_len` (`S-[0-9]+\.[0-9]+`). The two halves of the same comparison use different ID grammars. No corpus instance today; the divergence is the defect.

---

### F-S2107-P3-016 — MEDIUM

**Policy/Precondition:** PC24 + PC25; POLICY 4 (semantic anchoring — fabricated spec citation)
**Location:** `arm_b.rs::run_arm_b2`
**Confidence:** HIGH

PC24-25 specify a **blockquote-driven** traversal: *"All `S-NNN.MM=HHHHHHH` pairs extracted from aggregation blockquote region. **For each story ID in the blockquote set**, the catalog row is located and `input-hash` token extracted."*

The implementation inverts it (`for (cat_story_id, cat_hash) in &catalog { … find blockquote … }`) and justifies the inversion in-code with a **citation that does not exist**:

```rust
// Compare catalog→blockquote direction (BC-5.39.010 PC22 note: "scans all story IDs
// in the catalog").
```

PC22 reads only: *"PostToolUse on `STORY-INDEX.md` (basename guard + component `stories`)."* The quoted phrase appears nowhere in PC22 or elsewhere in the BC. A fabricated spec quotation used to justify a spec deviation is a POLICY 4 mis-anchor and blocks convergence per the mis-anchoring rule.

Behavioural consequence: a blockquote entry whose catalog row is absent, or whose catalog row's `input-hash` token fails `extract_input_hash_token` (non-hex, malformed, missing whitespace), is dropped from `catalog` and therefore never compared — silent fail-open on exactly the malformed-hash case invariant 11 flags as the fabricated-hash signal. The one test in this area (`..._arm_b2_no_blockquote_entries_not_vacuous`) asserts the empty-blockquote case, not the absent-catalog-row case.

---

### F-S2107-P3-017 — MEDIUM

**Policy/Precondition:** POLICY 11; POLICY 1; BC-5.39.010 §Deferred Scope; S-21.07 §File Structure / §Architecture Mapping / §Purity Classification
**Location:** `src/arm_d.rs` (622 lines, 15 non-ignored tests); `dispatch.rs` two `#[ignore]`d tests
**Confidence:** HIGH

The descope is internally inconsistent, and the residue carries known-wrong content forward:

1. **Asymmetric deferral treatment.** The two `dispatch.rs` Class-D tests were `#[ignore]`d with a DEFERRED reason (the "2 ignored"), while `arm_d.rs` retains **15 live, GREEN tests** asserting the behaviour of a descoped arm. Both sets test the same descoped feature; only one set was quarantined.
2. **Green tests asserting measured-wrong spec.** Those 15 tests validate PC30/PC31 semantics that BC v1.6 §Deferred Scope *itself* records as corpus-measured-wrong: `^L-EDP1-[0-9]+-[0-9]+:` = **0 matches** in real `lessons.md`; the lowercase `closes:` scan covers **0 of 20** non-bold Closes lines. They contribute to the 108-pass figure and constitute a false-confidence surface.
3. **Wrong forward-carry.** `arm_d.rs:27` still documents the scope predicate as `LAST ^L-EDP1-[0-9]+-[0-9]+: anchor block` — the exact regex BC v1.6 mandates S-21.08 replace with `^L-EDP1-[0-9]+:` — while `arm_d.rs:8-11` instructs S-21.08 to restore the module "intact" and "Do NOT delete this module." S-21.08 will be handed the wrong regex by the tombstone.
4. **Story-spec contradiction.** S-21.07 v1.3 §File Structure marks `src/arm_d.rs` as **"DEFERRED v1.6 — Class D; do NOT create"**, §Architecture Mapping as **"[DEFERRED v1.6 — Class D; do not create in v1.6]"**, and Task 12 as **"do not implement in v1.6"**. The file is present and `pub mod arm_d;` is declared. Under the "SPEC wins" standing rule this is a spec-vs-tree divergence requiring either the file's removal or a story-spec amendment recording the tombstone decision (POLICY 1 append-only arguably favours the latter — but the story spec currently says the opposite of what shipped, and one of the two must move).

**Fix (minimum):** `#[ignore]` the 15 arm_d tests with the same DEFERRED reason string used in `dispatch.rs`; correct `arm_d.rs:27` to the single-group regex with a pointer to §Deferred Scope; amend S-21.07 §File Structure/§Architecture Mapping/Task 12 to say "retain tombstoned" rather than "do NOT create".

---

### F-S2107-P3-018 — MEDIUM [carried, F-P2-016 unclosed]

**Policy/Precondition:** POLICY 11; POLICY 1 (T-ID reuse)
**Location:** `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats:668` and `:1077`
**Confidence:** HIGH

```
668: @test "T-038 CONTROL: cross-story catalog row lookup returns own-story hash (exit code 0)"
1077: @test "T-038 CONTROL: cross-story catalog lookup returns own-story hash (exit code 0)"
```

Two tests bearing the **same T-ID**, differing by the word "row". Both remain after the burst. This inflates the reported 43-ok count by one and violates single-T-ID-single-test. Pass-2 flagged it; the red-gate-log's §Finding Coverage table does not list it.

---

### F-S2107-P3-019 — MEDIUM [carried, F-P2-018 unclosed]

**Policy/Precondition:** documentation-vs-code correctness; partial-fix regression discipline
**Location:** `lib.rs::on_post_tool_use`, `if is_si` branch comment
**Confidence:** HIGH

```rust
// STORY-INDEX.md: no E arm (no version:/last_amended: frontmatter)
```

Verified false: `.factory/stories/STORY-INDEX.md:4` → `version: "4.282"`; `:8` → `last_amended:`. The behaviour (no Class E on STORY-INDEX) is correct — PC34 simply does not scope it — but the stated reason is factually wrong and would lead a future implementer to add Class E to STORY-INDEX on the belief that the exclusion was a data-shape accident. Re-raised at MEDIUM (from LOW) because it survived a full burst untouched despite being explicitly named.

---

### F-S2107-P3-020 — MEDIUM

**Policy/Precondition:** POLICY 1 (append-only / no live-ID reuse); TD-VSDD-060
**Location:** `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/e1-15-byte-last-amended/factory/specs/verification-properties/VP-039.md`; sibling `…/behavioral-contracts/ss-05/BC-5.39.010.md:16`, `:21`
**Confidence:** HIGH

The pass-2 known-open "VP-039 ID reuse" was only half-closed. The two `VP-9999-test.md` files were renamed to `VP-9999.md`, but the **original `VP-039.md` fixture** created in RG-009 remains in `e1-15-byte-last-amended/`, still reusing the live VP-039 ID (`VP-INDEX.md:418` registers a real VP-039 anchored to S-1.03) with fabricated content. It is now **dead** — no bats test references it after the T-045 envelope moved to `VP-9999.md` (only bats:1103's rename-history comment mentions it).

Related stale documentation: the sibling fixture `…/ss-05/BC-5.39.010.md:16,21` still reads *"T-045 triggers a VP file write (VP-039.md)"* and *"the write event points to VP-039.md, not BC-5.39.010.md"* — false after the rename.

The red-gate-log's fixture sweep classified `VP-039.md` as **"CONFORMANT — inner '039' all digits"**, which answers the predicate question while sidestepping the ID-hygiene question the finding was about. That is a scope substitution, not a closure.

**Fix:** delete the dead `VP-039.md` fixture; correct the two stale NOTE lines.

---

### F-S2107-P3-021 — MEDIUM

**Policy/Precondition:** POLICY 8 (BC change → body + ACs + Token Budget); POLICY 14 leg 5
**Location:** `S-21.07-...md` §Acceptance Criteria, §Test Plan, §BC Status
**Confidence:** HIGH

BC v1.6 carries **PC40** as a normative precondition, with **EC-032** and a canonical test vector ("B volatile-input"). The story v1.3 propagation mirrored EC-032 into §Edge Cases (line 603) but:

- **No AC covers PC40.** ACs run AC-001…AC-021; AC-009/AC-010 describe Class B Arm1 without any volatile-input carve-out. The implementation shipped `is_volatile_path` + `parse_story_volatile_inputs` + a new `run_arm_b1` branch under **no AC**.
- **No Test Plan row covers PC40.** T-001…T-034 contain no volatile-input entry (T-047 exists in bats but is unlisted in the story's Test Plan).
- §BC Status still asserts *"each of the **4 blocking arms** is covered by at least one blocking AC"* — the story's own title says **six-arm** (A1/A2/B1/B2/E1/E2). A stale internal count, and the sentence is the story's own AC↔BC completeness attestation.

The consequence is not academic: F-S2107-P3-002 and F-S2107-P3-012 are both PC40 conformance defects that shipped precisely because no AC constrained the behaviour.

---

### F-S2107-P3-022 — MEDIUM

**Policy/Precondition:** PC13 "LAST rightmost pipe-field algorithm"; documentation-vs-code
**Location:** `arm_a2.rs::extract_version_token_from_table_row`
**Confidence:** HIGH

PC13 prescribes a **field**-based algorithm: *"split the row by `|` delimiter; iterate fields in REVERSE order (right to left); return the version token from the first (rightmost) field whose stripped content contains a match."* The implementation never splits on `|`; it walks the whole line left-to-right and keeps the last match. The two coincide in most rows but the shipped code is not the specified algorithm, so the specified algorithm is untested.

The docstring overstates the protection it provides:

> *"Per PC13 (LAST/rightmost pipe-field token): scans the entire line and returns the LAST matching token … **This prevents spurious matches from BC ID fragments like "BC-5.39.010" (which contains "5.39")** from masking the actual version column."*

It does not prevent them — it only defers to a later token *if one exists*. `BC-5.39.010` yields the token `5.39` (word boundaries: `-`→`5`, `39`→`.`), so a BC-citing row with no other version token reports the cited version as **`5.39`**.

**Live latent self-block:** S-21.07's own `## Behavioral Contracts` row (line 551) carries **two** version tokens — the Version cell `1.6` and `DEFERRED v1.6` inside the ACs cell. Under PC13's actual field algorithm the ACs cell is the rightmost field containing a token, so the authoritative Version cell is never consulted. Both currently read `1.6`; the next BC bump that updates one and not the other produces a false stale-citation block on the story that authors the gate.

**Fix:** implement PC13's reverse-field iteration; add a test with an empty Version cell asserting no `5.39` citation; remove the version token from the ACs cell (or exclude the BC-ID substring from token candidacy).

---

### F-S2107-P3-023 — LOW (pending intent verification)

**Policy/Precondition:** POLICY 4 semantic anchoring
**Location:** `S-21.07-...md:13-14`
**Confidence:** MEDIUM

`phase: brownfield-backfill` but `cycle: v1.0-feature-engine-discipline-pass-1`, while the governing cascade, the adversary-pass directory, and the burst-log for this story are all `v1.0-brownfield-backfill`. Either the `cycle:` field is stale or the story deliberately anchors to the older cycle; the adversary cannot adjudicate authorial intent. Tagged pending intent verification per the intent-adjudication rule.

---

### F-S2107-P3-024 — LOW

**Policy/Precondition:** AC-019 internal accuracy
**Location:** `S-21.07-...md:409-410`
**Confidence:** HIGH

```
- STORY-INDEX.md (B2): max_bytes = 1048576, timeout_ms = 3000
- STORY-INDEX.md (B2 — Arm B2 write trigger): max_bytes = 2097152, timeout_ms = 5000
```

The first row is the **Arm B1 secondary read** (`STORY_INDEX_B1_MAX_BYTES`), mislabelled `(B2)`. Two rows labelled B2 with different caps invites the exact constant-mixup that F-P1C-002/003 were about. The crate constants are correct.

---

### F-S2107-P3-025 — LOW

**Policy/Precondition:** code correctness / assertion semantics
**Location:** `lib.rs::test_BC_5_39_010_invariant_7_ac018_multi_arm_violations_both_in_combined_block`, `_ =>` arm
**Confidence:** HIGH

```rust
_ => unreachable!("two violations must produce HookResult::Block, not Continue — \
    if this arm is reached, combine_violations_into_block has a correctness defect …")
```

The arm's own message states it **is** reachable on defect. `unreachable!()` asserts a state the programmer believes cannot occur; the correct construct for "this must not happen and the test must fail if it does" is `panic!()` or `assert!(matches!(…))`. Functionally identical today; semantically inverted, and it will read as a compiler-provable invariant to the next maintainer.

---

## 3. Observations

- **O-P3-01** [process-gap] The red-gate-log's own pass-2 section states: *"Four of nine required fixes were never implemented because the Red Gate did not encode them."* The same pattern recurred this burst — **F-P2-006, F-P2-009, F-P2-016, F-P2-017, F-P2-018** are all absent from the §Finding Coverage table and all remain unfixed. This is the third consecutive occurrence, which crosses the 3+ recurrence threshold for a process gap: the fix-burst protocol needs a mechanical gate that every pass-N finding ID appears in the red-gate-log §Finding Coverage table with either a test row or an explicit, justified "no test possible" entry (the F-P1B-012 precedent shows the honest-gap form works). Non-code findings (CHANGELOG, doc cites, duplicate test names) need an equivalent checklist because they cannot have red gates.

- **O-P3-02** `arm_a2::run_arm_a2` calls `run_arm_a2_for_bc` for every cited BC, and `run_arm_a2_for_bc` performs `host::read_file` **before** delegating to the seam that early-returns on empty citations. A story citing N BCs with no version-citing rows performs N wasted reads inside a 10M-instruction fuel budget. (Also the mechanism behind F-S2107-P3-005.)

- **O-P3-03** `frontmatter.rs::extract_frontmatter_field` and `extract_frontmatter_sequence` both require the literal first line `"---"`. A CRLF-terminated or BOM-prefixed artifact yields `"---\r"` / `"\u{feff}---"` → `None` → Arms A1, A2, B1, and E all silently no-op for that file. No corpus instance today; the failure mode is silent, which is the concern.

- **O-P3-04** Two incompatible arm-count conventions coexist: `hooks-registry.toml:667` says "Six arms" counting Class D and lumping E1+E2; the story title says "six-arm" counting A1/A2/B1/B2/E1/E2 and excluding D; `lib.rs:15` says "five-arm". Three different counts of the same hook across three files.

- **O-P3-05** `arm_e::strip_date_annotation` strips at the first whitespace. A `modified[]` entry written without a space before the annotation (`"2026-05-18(v1.1)"`) is compared verbatim and sorts **after** a following bare `"2026-05-18"`, producing a false E2 strict-decrease. PC38's example (`" (v1.3)"`) includes the leading space, so the spec is silent on the no-space form. No corpus instance today.

---

## 4. Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 3 |
| HIGH | 7 |
| MEDIUM | 12 |
| LOW | 3 |
| **Total findings** | **25** |
| Observations | 5 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision

---

## 5. Part B — Analysis and Verdict

**Verdict: NOT-CLEAN. Streak remains 0/3.**

### On the root cause (brief item 1)

The 5 `corpus_*` tests are **not** a token gesture. Three are genuine red gates, and the red-gate-log honestly records that the test-writer's prediction about `corpus_arm_a2` was **wrong** and only the live corpus revealed the truth — that is the mechanism working exactly as intended, and it is the single most valuable thing this burst produced.

But the root cause is not closed, and the reason is structural rather than effortful. The corpus tests were written **finding-first**: one test per pass-2 finding, each pointed at the one artifact that finding named. What was needed was **extractor-first** coverage: for each extractor, sample the corpus broadly enough to discover shapes nobody had thought of. The evidence is decisive — all three BLOCKERs in this pass live in extractors with zero or single-artifact corpus coverage:

- `extract_bc_index_version` was corpus-tested against BC-1.17.001, one of the **40** rows carrying a version chain. Sampling any of the other **1,943** rows would have surfaced F-S2107-P3-001 immediately.
- `is_volatile_path` has **no** corpus test. A single assertion sweeping `.factory/stories/*.md` `inputs:` arrays against ADR-037's six patterns would have surfaced all three legs of F-S2107-P3-002 — including the one that false-blocks the story's own file.
- `extract_frontmatter_field` has **no** corpus test. `corpus_arm_e1` selected VP-100 (inline `last_amended`), and the only two block-scalar artifacts in the entire repository are this story and its BC — F-S2107-P3-006.

Separately, the corpus tests as built are **non-hermetic in a way that will rot** (F-S2107-P3-013): they resolve to the main checkout's `.factory/` — a tree outside the reviewed commit, pinned by nothing — and they assert corpus cleanliness while attributing failure to the extractor. The predictable trajectory is that an unrelated factory-artifacts burst reddens `cargo test --workspace`, and the cheapest repair is to weaken the assertion. The mechanism is right; the wiring needs a hermeticity boundary and honest failure attribution before it can be load-bearing across bursts.

### On spec conformance (brief item 2)

The drift-by-widening pattern that D-950 named is **not** contained. Two of the five predicates the brief asked me to check are non-conformant, and both drift in the dangerous direction:

- `is_volatile_path` (F-S2107-P3-002) is wider than spec on two axes and narrower on one, and the wide axis **permanently disables a BLOCKING gate** for ~20 stories — directly contradicting PC40's own guarantee that the clause "imposes no permanent weakening."
- The PC34 epic arm (F-S2107-P3-009) drops both the `stories` component check and the `^E-[0-9]+-` basename regex that PC34 states explicitly, and its docstring names the wrong directory.

The three predicates that *are* conformant — `is_canonical_vp_filename`, `is_canonical_story_basename`, the first-cell anchor — are conformant because pass-2 named them individually and a red gate was written for each. The two non-conformant ones are the two nobody wrote a red gate for. That is the same finding-first-not-extractor-first pathology, at the predicate level: conformance is being achieved one adversary finding at a time rather than by systematic transcription of the spec clause. The `skip_section` bound is correct and correctly unconditional.

Two further conformance items are quieter but real: PC13's normative `MUST NOT use a plain line.contains(bc_id)` is still violated verbatim (F-S2107-P3-004), and PC13's reverse-field extraction algorithm was never implemented (F-S2107-P3-022) — with the story's own BC table row already carrying the two-token shape that makes it a latent self-block.

### On Class D removal completeness (brief item 3)

The dispatch-level removal is clean and the registry `path_allow` change is correct. The descope is **incoherent in its residue**, in three specific ways: the deferral was applied asymmetrically (2 dispatch tests ignored, 15 arm_d tests left green); the tombstone carries a regex that BC v1.6 §Deferred Scope *itself* documents as matching zero real lines, and instructs S-21.08 to restore the module "intact"; and the retained fail-open error arm is the pattern the BC names verbatim as non-conforming, with an adjacent comment telling the next implementer to switch it back on (F-S2107-P3-008, F-S2107-P3-017). Nothing here is reachable at runtime — which is exactly why it is dangerous: it is a correct-looking template for S-21.08 that encodes three known-wrong decisions. The story spec meanwhile says arm_d.rs must not exist, so spec and tree disagree and one of them must move.

### On fixture-shape conformance (brief item 4)

The sweep in the red-gate-log is genuinely good work — the `VP-9999-test.md` catch is exactly the right instinct, and the T-045 latent-false-green analysis is honest and correct. Two residues remain: the dead `VP-039.md` fixture with its live-ID reuse and its two stale sibling NOTE lines (F-S2107-P3-020), and the duplicate T-038 test ID (F-S2107-P3-018). The sweep answered "does the basename satisfy the predicate?" for VP-039 and returned CONFORMANT, which is true but is not the question pass-2 asked.

### On documentation-vs-code (brief item 5)

The two docstrings pass-2 named were fixed. **Nine other sites were not**, spanning `lib.rs`, `main.rs`, `Cargo.toml`, `hooks-registry.toml`, and the bats header — and the burst made them worse by descoping Class D while leaving Class D documented as shipping in the crate's own published description (F-S2107-P3-010). This is the clearest instance in the perimeter of a fix applied to the named instances and not to the class.

### On red-gate integrity (brief item 6)

POLICY 15 compliance for the fixes that *were* attempted is strong: verbatim captured failure output, per-assertion-site attribution, and — notably — an honest "no test is possible" entry for F-P1B-012 with the reasoning shown. That is the correct discipline and it should be preserved.

The integrity gap is one of **scope, not fidelity**. The red-gate-log's §Finding Coverage table lists 19 findings; five pass-2 findings are absent from it and all five are unfixed. The log itself diagnosed this failure mode one pass ago ("Four of nine required fixes were never implemented because the Red Gate did not encode them") and the burst reproduced it. That is why O-P3-01 is tagged `[process-gap]` rather than filed as a content defect: the log is accurate about what it covers, and the coverage set is chosen without a completeness gate.

### Convergence assessment

**Novelty: HIGH.** 19 of 25 findings are new; 6 are pass-2 findings verified unclosed. Nothing here is a rewording — the three BLOCKERs are corpus-verified live false-positives or gate-suppressions on artifacts in this very story's perimeter, and none of the three was visible to pass-2 (which explicitly listed "arm B2 full-corpus sweep incomplete", "ADR-035 not read at all", and "Red-gate-log unread" as NOT REACHED). The spec has not converged; the trajectory is 47 → 18 → 25.

The count rising is not a regression signal in itself. Pass-2 reached 18 findings without reading ADR-037, the red-gate-log, or the registry; those three artifacts account for 8 of this pass's findings. What the count does indicate is that the perimeter is larger than two passes have covered, and that the fix protocol is closing named instances faster than it is closing classes.

### Highest-leverage sequencing for the next burst

1. **F-S2107-P3-002 then F-S2107-P3-001** — both are live false-blocks/gate-suppressions with corpus-verified blast radius; P3-002(a) blocks writes to the story's own file.
2. **F-S2107-P3-003** — must precede any Class B work, and requires the invariant-11 stale-vs-fabricated adjudication for `9603a5b` with a literal `bin/compute-input-hash` invocation and captured stdout (POLICY 18 / POLICY 22), not a narrative claim.
3. **F-S2107-P3-014** — add extractor-first corpus sweeps (`is_volatile_path` × all story `inputs:`; `extract_bc_index_version` × a 20-row sample spanning both row shapes; `extract_frontmatter_field` × the two block-scalar artifacts) **before** re-running the cascade. These three sweeps would have caught all three BLOCKERs.
4. **O-P3-01** — add the red-gate-log completeness gate, or the next pass will re-verify the same carried findings a third time.

---

## 6. NOT REACHED (drives pass-4 scope)

- Bats bodies for AC-001…AC-011 and AC-015…AC-018 (lines 363–1046): names and the Class D skip conversions were read; the fixture contents and per-assertion strength of the ~30 payload tests were not audited.
- `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/` — only the VP fixtures and two BC fixture NOTE lines were read. The BC-INDEX and STORY-INDEX fixture stubs were **not** examined for realism regression; per pass-2 this is what masks Arm A1 defects in the suite, and it is what would mask **F-S2107-P3-001** (a fixture BC-INDEX whose every row carries a version cell cannot expose the no-version-cell class).
- `crates/hook-sdk` — `host::read_file` error taxonomy and the non-wasm stub's `-1`/CapabilityDenied behaviour were taken from in-tree comments, not read directly.
- Test execution: `cargo test`, `cargo clippy`, `cargo fmt`, and bats were **not run** (read-only profile). The claimed 108/0/2 and bats 43/43 are unverified by this pass.
- WASM binary not inspected; `plugins/vsdd-factory/hook-plugins/validate-cross-site-correspondence.wasm` presence/staleness and AC-021 smoke-test evidence not checked.
- `combine_violations_into_block`'s `" | "` join not checked against dispatcher `block_reason` escaping or length limits (carried NOT-REACHED from pass-2).
- ADR-035 §Decision 5 fuel-budget arithmetic vs. the actual per-invocation read volume (Arm A2 performs one read per cited BC — see O-P3-02).
- ADR-034 not read.
- `arm_d.rs` bodies (lines 60–370) read only in part; the 15 retained tests were enumerated but their assertions were not individually audited.

---

**Files whose exact text is load-bearing to the findings above (all absolute):**

- `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07/crates/hook-plugins/validate-cross-site-correspondence/src/arm_a1.rs` — `extract_bc_index_version`, `run_arm_a1_with_index_result` (F-001)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07/crates/hook-plugins/validate-cross-site-correspondence/src/arm_b.rs` — `is_volatile_path`, `run_arm_b1`, `run_arm_b2`, `extract_story_id_from_table_row` (F-002, F-012, F-015, F-016)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07/crates/hook-plugins/validate-cross-site-correspondence/src/arm_a2.rs` — `extract_story_bc_version_citations`, `run_arm_a2_for_bc_with_result`, `extract_version_token_from_table_row` (F-004, F-005, F-022)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07/crates/hook-plugins/validate-cross-site-correspondence/src/frontmatter.rs` — `extract_frontmatter_field` (F-006)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07/crates/hook-plugins/validate-cross-site-correspondence/src/dispatch.rs` — `is_frontmatter_parity_target` epic arm, `is_canonical_vp_filename` (F-009, F-011)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07/crates/hook-plugins/validate-cross-site-correspondence/src/lib.rs` — `on_post_tool_use` error arms, corpus tests (F-008, F-010, F-013, F-014, F-019, F-025)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md` — PC5, PC13, PC15, PC34, PC40, §Deferred Scope
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/architecture/decisions/ADR-037-input-hash-stable-input-constraint-volatile-artifacts-excluded.md` — §Decision 2 lines 60-71, §Decision 4 lines 85-97
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/stories/S-21.07-validate-cross-site-correspondence.md` — lines 18, 24, 409-410, 551, §File Structure, §BC Status
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/stories/STORY-INDEX.md` — lines 4, 8, 730, 733
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md` — lines 539, 541, 656, 659, 1459