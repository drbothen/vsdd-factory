---
document_type: adversarial-review
level: adversary
version: "1.0"
status: complete
story_id: S-21.07
pass: 2
cycle: v1.0-brownfield-backfill
producer: adversary
timestamp: "2026-08-03T00:00:00Z"
phase: 3
inputs:
  - stories/S-21.07-story-validate-cross-site-correspondence.md
  - specs/behavioral-contracts/ss-05/BC-5.39.010.md
  - policies.yaml
input-hash: "52f0bf3"
traces_to: "BC-5.39.010 v1.4; S-21.07 v1.2"
reviewed_head: "e28aa098"
reviewed_code_head: "e28aa098"
factory_artifacts_head: "0d2e8c3e"
base: "948f0fb1"
story_version: "1.2"
bc: "BC-5.39.010 v1.4"
verdict: NOT-CLEAN
novelty: high
findings_count: 18
severity_breakdown: "B3/H7/M5/L3"
observations_count: 4
streak: "0/3"
passes: 2
previous_review: "cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-1.md"
trajectory_append: 18
model_override: false
model_resolved: "claude-sonnet-4-6"
dispatch_shape: "HOLISTIC — one unified fresh-context adversary; deliberate change from pass-1 three-scope split to detect cross-scope defects"
asymmetry_enforcement: "Pass-2: reviewed prior pass Part A only (adversary-pass-1.md); no access to fix-burst internals or orchestrator routing decisions. Source perimeter: story v1.2 + BC-5.39.010 v1.4 + implementation at e28aa098 + adversary-pass-1.md Part A."
policy22_note: "POLICY 22 SUBAGENT-REPORT-FIDELITY active: orchestrator independently re-derived the three BLOCKER claims via literal shell before transcription; verified items marked [ORCH-VERIFIED]. Finding IDs, severities, policy cites, and anchors preserved exactly as given."
persistence_note: "Persisted in record burst (orchestrator-transcribed from holistic fresh-context adversary subagent). Verbatim from authoritative source per POLICY 22."
---

# Adversary Pass 2 — S-21.07 LOCAL cascade (BC-5.39.001)

**Reviewed HEAD:** `e28aa098` (feature/S-21.07; bats 41/0/0 GREEN) · **Story** v1.2 · **BC-5.39.010** v1.4
**Verdict:** **NOT-CLEAN** — B3 / H7 / M5 / L3 = 18 findings + 4 observations · **Streak: 0/3**

---

## Finding ID Convention

Finding IDs: `F-S2107-P2-NNN` (this pass). Holistic dispatch — no scope split.

---

## 1. Dispatch Shape

Pass-1 used a three-scope split; pass-2 is one unified holistic adversary. Rationale: the three-scope split under-detected cross-scope defects, which pass-2 confirms were present (F-P2-001, F-P2-002, F-P2-006, F-P2-007, F-P2-008 span multiple arms or BC clauses).

---

## 2. Part A — Pass-1 Closure Verification

### Outcomes

**CLOSED:**
- **F-P1C-001** (classify-then-read) — all five classifiers precede the read; unclassified → Continue; `.factory/STATE.md` and `policies.yaml` both correctly classified as unclassified. CLOSED.
- **F-P1B-003 / F-P1B-004 / F-P1B-008 / F-P1B-009** (arm_b: within-line B3, all-pairs blockquote extraction, catalog first-cell anchor, hex-bounded retry) — **lowercase-only shipped** — no `is_ascii_hexdigit` anywhere in the crate. All four CLOSED.
- **F-P1C-014 / F-P1C-015** (arm_e: guard now `len < 14`, 15-byte form parses to `Some("2")`; E1 absent-field emits Advisory). Both CLOSED.
- **Caps (all seven read sites)** — carry spec values. CLOSED.

**PARTIAL:**
- **F-P1B-001 / F-P1B-002** — v1.4 word-boundary predicate correctly implemented and `## Edge Cases` excluded, BUT the leading edge remains unbounded → **F-P2-001** (BLOCKER regression).
- **F-P1B-005 / F-P1B-010** — `is_canonical_bc_filename` rejects BC-INDEX.md (CLOSED); but `is_story_file` ships `starts_with("S-")` not PC9's regex → **F-P2-011** (MEDIUM).
- **F-P1B-006 / F-P1B-007** — LAST-wins and `starts_with('|')` both CLOSED, but row *selection* still unanchored → **F-P2-002** (BLOCKER).

**NOT-CLOSED:**
- **F-P1C-011 / F-P1C-012** — arm_d implements neither PC31's bold-markdown anchoring nor a line-anchored `L-EDP1-` — see **F-P2-005**, **F-P2-012**.
- **Arm-D error disposition** — see **F-P2-007**.

### Corpus claim verification

Corpus claim in PC13 v1.4 independently verified by the adversary: **144** `^## Token Budget` and **139** `^## Behavioral Contracts` sections in `.factory/stories/*.md`.

### Known-open confirmations

- **F-S2107-P1C-016 CONFIRMED** — AC-018 has bats coverage only; no Rust assertion of the invariant-7 aggregation path exists.
- **F-S2107-P1B-013 CONFIRMED** — and **its proposed remediation is WRONG** (see F-P2-008). This finding was previously **UNRECORDED in both INDEX.md and decision-log.md** (orchestrator-verified: zero occurrences in either file). Recording it now closes that ledger gap — the bookkeeping gap is recorded explicitly as a defect in this pass.
- **T-045 coupled trap** — NOT vacuous today (the flat VP path IS admitted, so E1 runs), but the fixture's discriminating power depends on that classifier behaviour and the test has no positive-coverage assertion → **F-P2-013**.
- **VP-039 ID reuse CONFIRMED** — VP-INDEX.md:418 registers a real VP-039 ("SDK Wire Format Encoding Is Symmetric with Dispatcher Decoding", anchored to S-1.03 at :514); the fixture reuses that live ID with fabricated content while siblings use the `-test` suffix. POLICY 1 / ID hygiene.

---

## 3. Part B — New Findings

### F-S2107-P2-001 — BLOCKER [regression]

**Policy/Precondition:** POLICY/PC13 + TD-VSDD-059
**Location:** `arm_a2.rs::extract_story_bc_version_citations`

`let mut skip_section = false;` leaves the leading edge unbounded, so YAML frontmatter falls inside the scan window. **[ORCH-VERIFIED: skip_section=false at line 96; S-21.04 line 11 contains a pipe, 20 occurrences of BC-6.26.001, last v-token v1.3; BC-6.26.001 is version "1.18"]** → false `[Class A Arm2]` block on every write to S-21.04. Same shape in S-19.05/06/07, S-15.17, S-18.09. Direct regression of F-P1B-001's claimed closure — bounding applied on the trailing edge only.

**Fix:** Initialise `skip_section = true`.

---

### F-S2107-P2-002 — BLOCKER

**Policy/Precondition:** PC5 / postcondition 2
**Location:** `arm_a1.rs::extract_bc_index_version`

Row filter is `starts_with('|') && contains(bc_id)` with LAST-wins across ALL rows, so a later row merely *mentioning* the BC wins over its own row. **[ORCH-VERIFIED: BC-INDEX row 657 = BC-1.17.001's own row, last token v1.7; row 691 = BC-2.07.001's row, starts with '|', mentions BC-1.17.001, last token v1.6; 691>657; BC-1.17.001 is version "1.7"]** → false block on every BC-1.17.001 write. Seven BC-INDEX body rows contain a foreign BC ID (657, 691, 714, 715, 716, 735, 752); blast radius grows with every cross-reference.

**Fix:** Anchored first-cell row selection, then LAST-wins within that row only.

---

### F-S2107-P2-003 — BLOCKER

**Policy/Precondition:** POLICY 8 + POLICY 14 leg 3/5 + POLICY 17
**Location:** Story S-21.07 v1.2 body; nine live sites

Story S-21.07 v1.2 body still pinned to BC-5.39.010 **v1.3** while the BC is v1.4 and BC-INDEX/STORY-INDEX both advanced. Nine live sites: frontmatter `title:`, H1, `## BC Status`, `## Behavioral Contracts` Version cell, `## Token Budget Estimate (MANDATORY)` row, AC-020 (×2), Task 1, Task 10. Story `version:` still "1.2", `last_amended:` still `(v1.2)`, no `## Changelog` row for the v1.4 propagation. Under the shipped hook, writing this story yields two `[Class A Arm2]` violations — the deliverable blocks its own spec.

**Route:** story-writer sweep, then state-manager STORY-INDEX Title re-sync.

---

### F-S2107-P2-004 — HIGH [process-gap]

**Policy/Precondition:** PC31
**Classification:** fifth spec-describes-imagined-shape instance

PC31 v1.3 mandates `^\*\*Closes:\*\*` on the rationale that plain-colon forms match zero real lines, but the corpus shows `^\*\*Closes` = 38 lines of which only 20 are `^\*\*Closes:\*\*`; the other 18 use `**Closes (per …):**`, including the current last-H2 entry and 12 of the 13 most recent. `^\*\*Refs` = **0** matches in the entire burst-log. Class D therefore has zero matchable lines in the live scoped region; exit 0 is indistinguishable from "nothing scanned".

**Fix:** PC31 → `^\*\*Closes\b[^:]*:\*\*`, re-anchor or drop the Refs clause, add positive-coverage assertion.

---

### F-S2107-P2-005 — HIGH

**Policy/Precondition:** PC31 / "SPEC wins"
**Location:** `arm_d.rs::run_arm_d` + `find_keyword_word_boundary`

Searches lowercase `"closes:"`/`"refs:"` anywhere in the line with no `**` requirement and no `^` anchor; simultaneously over-broad (narrative prose fires) and blind to the dominant real form. Keyword lengths hardcoded 7/5.

---

### F-S2107-P2-006 — HIGH

**Policy/Precondition:** PC13
**Location:** `arm_a2.rs::extract_story_bc_version_citations`

Implements `if !line.contains(bc_id) { continue; }` — verbatim the construct PC13 explicitly forbids ("Implementations MUST NOT use a plain `line.contains(bc_id)` test"). PC13's own worked counter-example `| BC-5.39.010 EC-001 |` is admitted. No boundary-requirement test exists.

---

### F-S2107-P2-007 — HIGH

**Policy/Precondition:** PC33 + postcondition 18 + invariant 5 vs invariant 6; SOUL.md #4
**Location:** `lib.rs::on_post_tool_use`

`arm Err(e) if cycle_kind.is_some()` swallows EVERY `HostError` into `log_warn` + Continue. PC33 and postcondition 18 both require BLOCK for `CapabilityDenied` and `Timeout`; only `NotFound` is advisory. The BC contains an unadjudicated self-contradiction (invariant 6 vs PC33/PC18) and the code silently resolved it fail-open.

**Record explicitly:** the pass-1 closure note — and the orchestrator's own D-949 dispatch wording "advisory+Continue per PC33" — **misread PC33**, which requires BLOCK on `CapabilityDenied`/`Timeout`.

**Route:** product-owner adjudicates in v1.5, then implementer aligns.

---

### F-S2107-P2-008 — HIGH

**Policy/Precondition:** POLICY 4 / POLICY 9
**Classification:** sixth spec-describes-imagined-shape instance, inverts the pass-1 remediation

PC34 bullet 2 scopes Class E to `verification-properties/ss-*/VP-*.md`, but **[ORCH-VERIFIED: zero ss-* directories; 102 flat VP-NNN.md files directly under .factory/specs/verification-properties/]**. Enforcing PC34's `ss-*` clause — the fix pass-1 proposed for F-P1B-013 — would make Class E inert for every VP in the repo AND make T-045 vacuous.

**Correct fix:** Amend PC34 to the flat path + `^VP-[0-9]+\.md$`, and separately add the `VP-INDEX.md` exclusion and resolve the unspecified `epics` clause.

---

### F-S2107-P2-009 — HIGH

**Policy/Precondition:** S-21.07 Task 20 (MANDATORY) / BC-8.31.001
**Location:** `CHANGELOG.md` `## [Unreleased]`

`## [Unreleased]` holds only the placeholder comment; zero matches for `cross-site-correspondence` or `S-21.07`. Task 20 gates PR creation.

---

### F-S2107-P2-010 — HIGH

**Policy/Precondition:** POLICY 18 / invariant 11
**Classification:** live block-on-ship condition

Catalog↔blockquote disagree for S-19.02 (`1184c9b` vs `bb1288e`), S-19.04 (`7a827c2` vs `67eee80`), S-19.07 (`6ba76b6` vs `8de858c`); story-frontmatter↔index disagree for **all nine** E-19 stories. Once shipped, every E-19 story write and every STORY-INDEX write blocks. Requires the invariant-11 stale-vs-fabricated adjudication before any `--update` sweep, and is a hard prerequisite for AC-021 staging.

**Note:** S-21.07's own three legs are mutually EQUAL at `52f0bf3` — no authority claim made, given the unresolved binary divergence.

---

### F-S2107-P2-011 — MEDIUM

**Policy/Precondition:** PC9/PC16
**Location:** `is_story_file` / `extract_story_id_from_path`

`is_story_file` ships `starts_with("S-")`, not the regex; `extract_story_id_from_path`'s `splitn(3,'-')` would synthesise a nonsense story_id for an `S-README.md`-shaped file. No corpus instance today.

---

### F-S2107-P2-012 — MEDIUM

**Policy/Precondition:** PC30
**Location:** `arm_d.rs::extract_scope_limited_region` — Lessons arm

Uses `line.find("L-EDP1-")` (any offset); real `lessons.md` lines 392/442/458/479/528 are mid-line `**Sibling-corrigendum to L-EDP1-055:**`-class mentions that qualify as anchors. Correct today only by ordering luck. PC30's `[0-9]+-[0-9]+` two-group regex also does not match the real single-group `L-EDP1-078:` form.

---

### F-S2107-P2-013 — MEDIUM

**Policy/Precondition:** POLICY 11
**Location:** bats `T-045 CONTROL`

Asserts only exit 0 plus ABSENCE of a warn record; both hold identically if Class E never ran. One classifier change from silent vacuity with no signal.

**Fix:** Mutant sibling fixture asserting exit 2.

---

### F-S2107-P2-014 — MEDIUM

**Policy/Precondition:** POLICY 19 / TD-VSDD-091 / TD-VSDD-060
**Location:** S-21.07 `## Token Budget Estimate` row

`ADR-035 v1.0` volatile pin in the story's live `## Token Budget Estimate` row. E-21 adversary pass-4 already normalised this for S-21.01/04/05; S-21.07 was authored after that burst and never swept.

---

### F-S2107-P2-015 — MEDIUM

**Policy/Precondition:** POLICY 4 / POLICY 14 leg 5
**Location:** BC-5.39.010 `## Story Anchor` / `## Traceability` / BC-INDEX Stories cell

`## Story Anchor` reads "TBD — no story allocated yet." and `## Traceability` Stories row is `TBD`, and BC-INDEX's Stories cell is `TBD`, while S-21.07 declares the BC and is registered at v1.2. Sibling BCs carry concrete anchors. Anchoring is one-directional.

---

### F-S2107-P2-016 — LOW

**Policy/Precondition:** POLICY 11
**Location:** bats test suite

Duplicate bats test: `T-038 CONTROL: cross-story catalog **row** lookup…` and `T-038 CONTROL: cross-story catalog lookup…` are identical in fixture, envelope and assertions; names differ by one word. Inflates the 41-ok count.

---

### F-S2107-P2-017 — LOW

**Policy/Precondition:** TD-VSDD-060
**Location:** `lib.rs` module doc, `main.rs` (×2), `Cargo.toml` description, bats header, `hooks-registry.toml` comments (×3)

Stale `BC-5.39.010 v1.2` cites on five code/config surfaces: `lib.rs` module doc, `main.rs` (×2), `Cargo.toml` description, the bats header, and `hooks-registry.toml` comments (×3). BC is at v1.4.

---

### F-S2107-P2-018 — LOW

**Policy/Precondition:** (code comment correctness)
**Location:** `lib.rs::on_post_tool_use` `if is_si` branch comment

Comment claims STORY-INDEX has "no version:/last_amended: frontmatter"; both are present (STORY-INDEX lines 4 and 8; BC-INDEX likewise). Behaviour correct, stated reason false — would mislead a future implementer.

---

## 4. Observations

- **O-P2-01** [process-gap] `run-all.sh`'s positive-coverage guard is aggregate-only — a single suite skipping all its payload tests still exits 0 locally; CI is covered because `bats-full-suite` sets `CI_REQUIRE_ARTIFACTS: "1"`.

- **O-P2-02** All four indexes carry `version:`+`last_amended:` and are where POLICY 14 leg-4 most often breaks, yet PC34 scopes Class E to BC/VP/story only — index drift is invisible to this hook.

- **O-P2-03** 17 VP rows all `(pending)`; POLICY 9 allocation still owed at post-merge.

- **O-P2-04** `arm_a1::extract_bc_index_version` uses `from_utf8(...).unwrap_or("")` — the A1 sibling never got the dedicated UTF-8 violation class that A2 received via F-P1B-016.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 3 |
| HIGH | 7 |
| MEDIUM | 5 |
| LOW | 3 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision — two BLOCKERs are live corpus-verified false positives on real in-flight artifacts; root cause NOT closed

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 2 |
| **New findings** | 18 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 18 / 18 = 1.0 |
| **Median severity** | HIGH |
| **Trajectory** | 47 → 18 |
| **Verdict** | FINDINGS_REMAIN |

---

## 5. Central Meta-Finding

**Pass-1's systemic root cause did NOT close.** Fixtures were rebuilt to *look like* production excerpts, but no test reads an actual `.factory/` artifact — which is exactly why two live corpus-verified false positives survived a green 41/41 bats suite and 2360/0 workspace run. The spec-describes-imagined-shape class is now at **six** instances: (1) PC13 v-prefix; (2) PC31 plain-colon; (3) PC13 exact heading text; (4) PC20/PC21 lowercase vs uppercase fixture; (5) PC31 bold-vs-parenthetical; (6) PC34 `ss-*` vs flat VP layout.

Three BLOCKERs: two are live corpus-verified false positives on real in-flight artifacts (S-21.04 via Arm A2, BC-1.17.001 via Arm A1) plus the story's own body still pinned to v1.3 of its governing BC while the hook reads v1.4.

---

## 6. NOT REACHED

The following were not examined in this pass and drive pass-3 scope:

- Red-gate-log unread
- bats lines 330–1056: only names extracted, not bodies
- 3 of 55 fixture files read — notably the BC-INDEX and STORY-INDEX stubs were unexamined for realism regression (this is what masks F-P2-002 in the test suite)
- WASM binary not inspected
- arm B2 full-corpus sweep incomplete
- `extract_input_hash_token` multi-token rows only spot-checked — part of F-P2-010's delta may be extraction artifact
- ADR-035 not read at all
- BurstLog/CycleIndex scope arms not exercised against real byte offsets
- No 4+-arm firing case examined; `combine_violations_into_block`'s `" | "` join not checked against dispatcher `block_reason` escaping/length limits
