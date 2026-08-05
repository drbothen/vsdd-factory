# Adversarial Review — S-21.07 LOCAL cascade, Pass 4

```yaml
---
review_type: local-story-adversarial
story_id: S-21.07
cycle: v1.0-brownfield-backfill
pass: 4
passes: 4
reviewed_head: "256023c6"
reviewed_branch: feature/S-21.07-validate-cross-site-correspondence
worktree: /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07
story_version: "1.4"
bc: "BC-5.39.010 v1.9"
adrs_read: [ADR-035, ADR-037]
verdict: NOT-CLEAN
findings_count: 25
severity_breakdown:
  BLOCKER: 4
  HIGH: 9
  MEDIUM: 9
  LOW: 3
observations_count: 5
streak: 0/3
trajectory_append: 25
trajectory: 47 → 18 → 25 → 25
input_hash_basis:
  story_frontmatter_declared: "25c7324"
  story_index_catalog_row: "9603a5b"
  story_index_blockquote: "9603a5b"
  mechanically_verified: false
  note: >
    Read-only profile — `bin/compute-input-hash` could not be invoked. The declared
    frontmatter value is taken as-is; the three-way inequality against both STORY-INDEX
    sites is recorded as F-S2107-P4-012. POLICY 18 mechanical verification remains OWED.
---
```

---

## Part A — Findings

### BLOCKER

---

#### F-S2107-P4-001 — BLOCKER — BC-INDEX.md version cell for BC-5.39.010 is `v1.6`; frontmatter is `1.9`. Arm A1 blocks on its own governing BC.

**Location:** `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md:1459` · `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md:4`

**Clause violated:** POLICY 14 leg 5 (upstream-index cells sync same-burst); BC-5.39.010 postcondition 2.

**Evidence.** BC-INDEX row, 6th escape-aware field:

```
| [BC-5.39.010](ss-05/BC-5.39.010.md) | validate-cross-site-correspondence WASM hook MUST block … | draft | E-12 | TBD | v1.6 |
```

BC frontmatter: `version: "1.9"`.

The BC was amended **three times** this session (v1.6→v1.7→v1.8→v1.9). None of the three hops updated the BC-INDEX version cell. Traced through the shipped code: `first_cell_matches_bc_id` → link-form match; escape-aware split → 6 non-empty fields; `extract_last_v_token("v1.6")` → `Some("1.6")` → `Version("1.6")`; `"1.6" != "1.9"` → `Violation` (`arm_a1.rs:382-397`). The hook, once live, blocks every write to its own governing BC.

This is a POLICY 14 leg-5 defect of exactly the class BC-5.39.010 exists to gate, sitting on BC-5.39.010's own index row, produced by the amendment cascade the brief asked me to scrutinise. Three consecutive product-owner amendments each failed the five-leg parity gate on leg 5.

---

#### F-S2107-P4-002 — BLOCKER — Story S-21.07 v1.4 cites BC v1.7 at both gate-visible sites; BC is v1.9. Arm A2 emits two violations against its own story.

**Location:** `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/stories/S-21.07-validate-cross-site-correspondence.md:565` (Behavioral Contracts table), `:650` (Token Budget), plus `:7` (frontmatter `title:`), `:60` (H1), `:96-111` (BC Status), `:565` version cell, `:669` (Task 1), `:839` (Changelog row 1.4)

**Clause violated:** POLICY 8; POLICY 14 leg 5; BC-5.39.010 postcondition 7 + §BC-version-pin datum-copy ruling.

**Evidence.** Body BC table (`:565`):

```
| BC-5.39.010 | validate-cross-site-correspondence WASM hook MUST block … | 1.7 | AC-001 through AC-021 (AC-012/013/014 DEFERRED v1.6 — Class D) |
```

Token Budget (`:650`):

```
| BC-5.39.010 v1.7 (full text, 33 ECs, 17 VPs, Gate Spec pseudocode) | ~8,000 |
```

Traced through `extract_story_bc_version_citations` + `extract_version_token_from_table_row` (`arm_a2.rs:97-296`):
- `## Behavioral Contracts` is a target heading; Phase 1 reverse-scan skips the ACs cell (not pure-version) and returns the isolated `1.7`.
- `## Token Budget Estimate (MANDATORY)` is a target heading; Phase 1 finds no pure-version field (`~8,000` rejected), Phase 2 reverse-scan finds `v1.7` → `1.7`.

BC frontmatter is `1.9` → **two** `[Class A Arm2]` violations, combined into one block. The story-writer advanced v1.3→v1.4 propagating **v1.7** and was never re-dispatched for the v1.8 and v1.9 hops. The prompt states story-writer "propagated BC v1.7" — that propagation is now two versions stale, and the story's own title/H1 pin the stale version.

---

#### F-S2107-P4-003 — BLOCKER — `RowMalformed` / postcondition 4a has ZERO test coverage. The v1.9 headline amendment is a paper-fix.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/src/arm_a1.rs:104` (variant), `:184` (construction), `:398-418` (disposition)

**Clause violated:** TD-VSDD-059 paper-fix detection; POLICY 15 per-guard mutant-verification; BC-5.39.010 v1.9 postcondition 4a; story Task 19 (per-arm mutant + control).

**Evidence.** Corpus-wide grep for the symbol:

```
grep -c RowMalformed <worktree>
  crates/.../src/arm_a1.rs        : 12
  crates/.../docs/red-gate-log.md :  4
  (all tests)                     :  0
  (bats suite)                    :  0
  (fixtures)                      :  0
```

Every one of the 12 `arm_a1.rs` occurrences is an enum variant, a doc comment, a match arm, or the wrapper's `None` collapse. **No unit test constructs a <5-field candidate line. No bats fixture produces one. No test asserts the postcondition-4a advisory fires, or that it does not block.**

Worse: the red-gate-log records that the burst *removed* the only fixture that reached the state. `docs/red-gate-log.md:849-873` "Fixture Correction 3" rewrites a 4-field body row that was hitting `RowMalformed(4)` into a 6-field canonical row — and adds no replacement test for the abandoned path. The fourth state was added to the enum, wired into the match, documented at length, and left completely unexercised.

The `#[derive(PartialEq)]` on `BcIndexVersionState` and the `RowMalformed(usize)` payload are both dead weight in tests. The `field_count` interpolated into the advisory message is unverified; the message text is unverified (see F-S2107-P4-025).

---

#### F-S2107-P4-004 — BLOCKER — `extract_frontmatter_field` cannot parse `last_amended: |-`. Class E1 is structurally inert on BC-5.39.010.md and S-21.07's own story.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/src/frontmatter.rs:28-67`

**Clause violated:** BC-5.39.010 PC36/PC37, postcondition 20; H1 promise "frontmatter version↔last_amended text-prefix mismatch … MUST block".

**Evidence.** `extract_frontmatter_field` does a line-prefix scan and returns everything after `<field>:` on that single line:

```rust
if line.starts_with(&prefix) {
    let rest = &line[prefix.len()..];
    let trimmed = rest.trim();
    …
    return Some(value.to_string());
}
```

There is no handling of YAML block scalars. Corpus:

```
grep -rc '^last_amended: |-' .factory/
  .factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md            : 1  (line 46)
  .factory/stories/S-21.07-validate-cross-site-correspondence.md      : 1  (line 11)
  → 2 total occurrences across 2 files
```

For both files the function returns the literal string `"|-"`. `extract_last_amended_outer_version("|-")` → `len 2 < 14` → `None` (`arm_e.rs:62-64`) → `run_arm_e1` takes the unparseable branch and emits an advisory (`arm_e.rs:177-187`). **Class E1 can never block on either artifact.**

The two files carrying this form are precisely the story's governing BC and the story itself — i.e. the two artifacts an AC-021 live smoke test would target, and the two artifacts whose four-hop version cascade this burst mismanaged (F-S2107-P4-001, F-S2107-P4-002). E1's stated purpose is POLICY 14 leg-4 parity; it is silently unable to enforce it where it matters most.

Every E1 test and fixture uses the inline double-quoted form (`arm_e.rs:319-320`, `:337-338`, `:362-364`, `:385-387`; `e1-*/…/BC-5.39.010.md`; `e1-*/…/VP-9999.md`). The single live-corpus E1 test (`lib.rs:936-954`) samples `VP-100.md`, which uses the inline form — so the block-scalar shape is invisible to the whole suite. This is the "spec-describes-imagined-shape" class the burst claims to have closed eight instances of; here is a ninth, in the shared frontmatter parser rather than in a fixture.

---

### HIGH

---

#### F-S2107-P4-005 — HIGH — First-candidate-wins ordering turns `RowMalformed` into a silencer for postconditions 2 **and** 4.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/src/arm_a1.rs:142-189`

**Clause violated:** BC-5.39.010 v1.9 PC5 ("the hook **locates the body-table row** for the BC ID"); postconditions 2 and 4.

**Evidence.** The scan loop `return`s on the **first** line satisfying conditions (1)+(2):

```rust
for line in content.lines() {
    if !line.starts_with('|') { continue; }
    …
    if !first_cell_matches_bc_id(first_cell, bc_id) { continue; }
    // Conditions (1)+(2) satisfied — this is a candidate line.
    …
    return match non_empty_fields.len() {
        5 => RowPresentNoVersion,
        n if n >= 6 => …,
        n => RowMalformed(n),
    };
}
RowAbsent
```

There is no continuation on a malformed candidate. Consequence: a **single** malformed candidate line appearing *earlier* in BC-INDEX.md than the real body-table row — e.g. a notes/changelog table row `| [BC-5.39.010](ss-05/BC-5.39.010.md) | see D-954 |` — yields `RowMalformed(2)` → advisory + Continue, and the genuine row is never examined. That permanently and silently suppresses **both** blocking paths for that BC: the stale-version block (PC2) and the dropped-registration block (PC4).

This is the mirror-image false-negative the brief asked me to rule on, and it is present. `RowMalformed` does not *absorb* `RowAbsent` cases by definition — but by **scan order** it absorbs the `Version(v)` and `RowAbsent` verdicts for any BC that acquires one earlier malformed line. The corpus is safe today (0 malformed lines — see O-1), so this is latent; but the state was introduced expressly as "forward-looking protection", and in the forward case it protects the *defect*, not the gate.

Conforming behaviour: prefer a valid (≥5-field) candidate across the whole file, and return `RowMalformed` only if **no** valid candidate exists. PC5 says "locates the body-table row", not "the first BC-ID-bearing line". Neither the BC nor the Gate Spec pseudocode (`BC-5.39.010.md:1027-1045`) prescribes first-match-wins; the implementation invented it.

---

#### F-S2107-P4-006 — HIGH — AC-009's three-category provenance note is unimplemented, untested, and the implementation asserts a classification invariant 11 declares undecidable.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/src/arm_b.rs:669-685` (`classify_provenance`), `:19-24` (module doc), `:730-733` (test assertion)

**Clause violated:** Story AC-009 (`S-21.07…md:233-251`); BC-5.39.010 invariant 11 + Part B postconditions note.

**Evidence.** Story AC-009 v1.4:

> "The block message MUST include a note distinguishing among **all three** provenance categories per BC-5.39.010 invariant 11: stale …; fabricated …; **algorithm-divergent** (legitimately computed by a prior binary version per ADR-036 §Decision 4; NOT fabricated; remedy: recompute with current authoritative binary, no PROVENANCE-BREAK annotation required)."

Shipped implementation returns one of exactly three strings, none containing "algorithm-divergent":

```rust
fn classify_provenance(b1: &str, b2: &str, b3: &str) -> &'static str {
    if b2 == b3 && b1 != b2 { "stale — STORY-INDEX.md needs `compute-input-hash --update` sweep" }
    else if b1 == b2 && b1 != b3 { "fabricated — blockquote hash disagrees with story frontmatter and catalog row" }
    else { "stale — multiple hash mismatches; run `compute-input-hash --update`" }
}
```

Two distinct defects:

1. **Unimplemented MUST.** The third category never appears in any block message. The module doc (`:19-24`) still enumerates two categories only — a pre-v1.7 artifact left unswept.
2. **Asserts an undecidable label.** Invariant 11 states the hook "detects cross-site *inconsistency* only — it **cannot distinguish** among three categories" and that the message "SHOULD note all three categories to guide correct remediation". The implementation instead *picks one* from a positional heuristic and asserts `"fabricated"` — the category whose remedy is a PROVENANCE-BREAK acknowledgment in the burst-log, a governance act. This reproduces the exact error v1.7 retracted: pass-30 M02 mislabelled `1acf3c6` as fabricated, and BC v1.7 invariant 11 §Correction retracts that claim explicitly (`BC-5.39.010.md:884-891`). The hook will now generate that same false accusation automatically.

Untested: the sole assertion is `msg.contains("stale") || msg.contains("fabricated")` (`arm_b.rs:730-733`) — the pre-v1.7 two-category predicate, which passes under any of the three return strings and would pass if the third category were added or removed. bats AC-009 asserts only `[Class B]`.

---

#### F-S2107-P4-007 — HIGH — Seven E-class bats BC-INDEX fixtures carry the 5-field shape defect; the burst self-disclosed it and declared it "FUNCTIONALLY CORRECT" instead of fixing it.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md:931` plus:

| Fixture BC-INDEX.md | Line | Row | Comment claim |
|---|---|---|---|
| `e1-version-match` | 14 / 8 | `\| [BC-5.39.010](…) \| test fixture \| v1.6 \| 2026-07-29 \| active \|` | "INDEX row also v1.6 (A1 clean)" |
| `e1-version-mismatch` | 13 / 8 | `… \| v1.33 \| 2026-07-29 \| active \|` | "A1 must NOT fire (row matches BC version 1.33)" |
| `e1-unparseable` | 12 / 8 | `… \| v1.6 \| …` | "A1 clean (INDEX v1.6 matches BC v1.6)" |
| `e1-prior-chain-correct` | 12 / 8 | `… \| v1.6 \| …` | "A1 clean (INDEX v1.6 matches BC v1.6)" |
| `e1-prior-chain-wrong-outermost` | 13 / 8 | `… \| v1.6 \| …` | "A1 clean (INDEX v1.6 matches BC v1.6)" |
| `e2-non-monotonic` | 12 / 8 | `… \| v1.3 \| …` | "A1 clean (INDEX v1.3 matches BC v1.3)" |
| `e2-ascending` | 12 / 8 | `… \| v1.3 \| …` | "A1 clean (INDEX v1.3 matches BC v1.3)" |

**Clause violated:** TD-VSDD-060 sibling-site sweep; CLAUDE.md Canonical Principle Rules 1 and 4; the brief's dominant defect class ("fixture … reaches a different state than the test's assertion implies").

**Evidence.** All seven rows have **5** non-empty escape-aware fields with the version token in field 3. Under v1.9 PC5 these classify `RowPresentNoVersion` → `arm_a1.rs:376-381` returns `(vec![], vec![])` **without reading any field**. No version comparison occurs. Every one of the seven fixture comments asserts that A1 is clean *because the versions match* — a semantics the shape cannot produce.

The burst identified this and dispensed itself:

```
| `e1-*/e2-* BC-INDEX.md` files | 5-field shape defect; A1 silent | E1/E2 arms fire independently of A1 | FUNCTIONALLY CORRECT ✓ |
```
— `red-gate-log.md:931`

Its sibling in the same table, `combined-a1-e1/BC-INDEX.md`, **was** corrected to 6 fields in the same amendment (`red-gate-log.md:875-899`). Same file class, same shape defect, same sweep, seven siblings skipped with a self-authored pass. Per TD-VSDD-059 and CLAUDE.md ("Implementer claims … *deferred to follow-up* | Adversary independently verifies the claim; implementer self-disclosure of risk severity is NOT authoritative"), the "FUNCTIONALLY CORRECT" dispensation carries no weight. Blast radius 7 → HIGH per the partial-fix regression severity rule.

---

#### F-S2107-P4-008 — HIGH — Arm A2 block message cites "POLICY 14 leg 3" where postcondition 7 prescribes leg 5; Class E2 cites leg 4 where the defect is leg 3.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/src/arm_a2.rs:377` · `crates/hook-plugins/validate-cross-site-correspondence/src/arm_e.rs:242`

**Clause violated:** BC-5.39.010 postcondition 7 (prescribed message text); POLICY 14 leg enumeration.

**Evidence.** POLICY 14's five legs, in order: (1) `version:` frontmatter, (2) body Changelog row, (3) `modified[]`, (4) `last_amended:` prefix, (5) upstream-index cells.

Postcondition 7 prescribes: `"… Update story citation same-burst per POLICY 14 leg 5."`

Shipped (`arm_a2.rs:372-379`):

```rust
"validate-cross-site-correspondence [Class A Arm2]: story '{story_id}' cites '{bc_id}' at version v{cited_version} in {location}, but BC frontmatter says version {bc_version}. Update the story's BC-table citation to v{bc_version}. POLICY 14 leg 3."
```

Leg 3 is `modified[]`. A fixer following the block message is routed to the wrong parity leg. Arm A1's counterpart message correctly cites leg 5 (`arm_a1.rs:392`) — so the two arms of the same class disagree on their own policy anchor.

Second site: `run_arm_e2` cites `"POLICY 14 leg 4."` (`arm_e.rs:242`) for a `modified[]` monotonicity violation — that is leg 3. E1 correctly cites `"POLICY 14 leg 4 / POLICY 17"` (`arm_e.rs:197`).

Untested at both sites. The Arm A2 test asserts only `[Class A Arm2]`, `v1.17`, `1.18` (`arm_a2.rs:509-521`) — no policy token. The E2 test asserts only `[Class E2]` (`arm_e.rs:413-416`). By contrast the Arm A1 test *does* assert `POLICY 14 leg 5` (`arm_a1.rs:513-516`), which is why A1's token stayed correct and A2's drifted. Mis-anchoring; blocks convergence.

---

#### F-S2107-P4-009 — HIGH — Four mutually contradictory arm counts across shipped artifacts.

**Location:**

| Count | Site |
|---|---|
| **five-arm** | `crates/.../src/lib.rs:15` — "BC-5.39.010 v1.9 — five-arm PostToolUse cross-site value-correspondence gate" |
| **Six arms** (enumerating Class D as active) | `plugins/vsdd-factory/hooks-registry.toml:667-671` |
| **six-arm** | `.factory/stories/S-21.07…md:7`, `:60` (title/H1); BC-5.39.010 H1 implies 6 blocking arms |
| **seven arms** | `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats:6` — "covering the seven arms (A1, A2, B1, B2, D, E1, E2)" |
| **seven-arm** | `.factory/stories/STORY-INDEX.md:730`, `:734` |

**Clause violated:** POLICY 7 (title source-of-truth propagation); POLICY 4 semantic anchoring; BC-5.39.010 §Honest Gap Class C (count/enumeration parity).

**Evidence.** The actual shipped arm set is **six** blocking arms (A1, A2, B1, B2, E1, E2) with Class D deferred and not compiled (`dispatch.rs:193-197` returns `None` unconditionally; `lib.rs:214-218` dispatch block removed). `lib.rs:15` undercounts to five; bats and STORY-INDEX overcount to seven by counting the deferred Class D. The registry comment says "Six arms" but its enumeration includes Class D and omits the E1/E2 split — arriving at six by a different, also-wrong, accounting.

This is precisely the Class C failure shape BC-5.39.010 §Honest Gap declares ungatable ("prose claims *N gates* while the enumeration has N−1 items") — reproduced five times inside the artifact set of the story that documents it.

---

#### F-S2107-P4-010 — HIGH — Class D advertised as a shipped capability in crate metadata, crate docs, and the production registry, contradicted within the same comment block.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/Cargo.toml:9` · `crates/.../src/lib.rs:10` · `plugins/vsdd-factory/hooks-registry.toml:669-671` vs `:685-686`

**Clause violated:** BC-5.39.010 v1.6 §Deferred Scope ("The `is_cycle_artifact` dispatch branch MUST NOT be compiled into the hook"); CLAUDE.md Standing Rule 3 §3 ("Doc comment claiming X with no X → either implement the gate or remove the docs").

**Evidence.**

`Cargo.toml:9` — the crate's published `description`, swept to v1.9 this burst while retaining the false capability claim:

```toml
description = "WASM hook plugin: PostToolUse gate that validates … and finding-ID namespace format (BC-5.39.010 v1.9 Classes A/B/D/E)"
```

`lib.rs:10` — crate-level rustdoc lists Class D among the arms:

```rust
//! **Class D (cycle artifact write):** finding-ID namespace format advisory (NEVER blocks).
```

`hooks-registry.toml:669-671` lists "Class D (finding-ID namespace format advisory — advisory-only, never blocks)" as one of the six arms — and then, **fourteen lines later in the same comment block**, states `:685-686`: "`.factory/cycles/` was removed with Class D deferral (BC-5.39.010 v1.6 / D-953): cycle artifact dispatch descoped; no reads needed." Self-contradiction inside one document → HIGH per the semantic-anchoring severity table.

An operator reading `Cargo.toml` or the registry preamble will believe cycle-artifact writes are validated. They are not — the write is unclassified and returns `Continue` before any read (`lib.rs:139-143`).

---

#### F-S2107-P4-011 — HIGH — F-P3-010's stale-cite sweep is incomplete and the CHANGELOG asserts a completion that did not occur (partial-fix regression, S-7.01).

**Location:**

| Site | Cite | Should be |
|---|---|---|
| `plugins/vsdd-factory/hooks-registry.toml:666` | `BC-5.39.010 v1.2 §Gate Spec` | v1.9 |
| `plugins/vsdd-factory/hooks-registry.toml:681` | `BC-5.39.010 v1.2 §Gate Spec` | v1.9 |
| `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats:38` | `Governing BC: BC-5.39.010 v1.2` | v1.9 |
| `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats:305` | `BC-5.39.010 v1.2 §Gate Spec` | v1.9 |
| `crates/.../src/arm_a1.rs:775` | `BC-5.39.010 v1.7 PC5 (amended)` | v1.9 |
| `crates/.../src/arm_b.rs:356`, `:366`, `:374` | `BC-5.39.010 v1.6 PC40` | v1.9 |
| `crates/.../src/arm_b.rs:1029`, `:1152`, `:1192` | `BC-5.39.010 v1.7 PC40` | v1.9 |

**Clause violated:** POLICY 14 leg 5 / POLICY 19 downstream cite currency; TD-VSDD-091; S-7.01 partial-fix regression discipline.

**Evidence.** The brief records F-P3-010 as "~8 stale `BC-5.39.010 v1.2` cites swept … to v1.8, then again to v1.9". The sweep hit `Cargo.toml`, `lib.rs`, `main.rs` and `red-gate-log.md` (verified: `Cargo.toml:9` and `main.rs:14`,`:19` now read v1.9) but **missed the two highest-visibility sites in the entire deliverable** — the shipped registry comment and the bats suite header, both still at **v1.2**, seven versions stale.

`arm_b.rs` is internally incoherent: the *same* precondition PC40 is cited as v1.6 at three sites and v1.7 at three sites in one file.

The CHANGELOG asserts the sweep completed (`CHANGELOG.md:11`): *"Stale `BC-5.39.010 v1.2` cites updated to v1.8 across Cargo.toml, lib.rs, main.rs, and red-gate-log.md."* — the enumeration silently excludes `hooks-registry.toml` and the bats suite, and the stated target (v1.8) is itself now stale. A reader of the CHANGELOG would conclude the sweep is done.

Blast radius ≥ 4 files → HIGH.

---

#### F-S2107-P4-012 — HIGH — STORY-INDEX.md carries a live three-way input-hash inequality plus four further stale legs for S-21.07. F-P3-003 is not closed.

**Location:** `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/stories/STORY-INDEX.md:730` (catalog row), `:733` (aggregation blockquote), `:734` (BC coverage blockquote)

**Clause violated:** POLICY 18 THREE-WAY-INPUT-HASH-EQUALITY-GATE (D-923); POLICY 14 leg 5 (per STORY-INDEX.md:735, "catalog-row BC pins are the fifth leg of POLICY 14/17 quintuple parity"); POLICY 7.

**Evidence.** Six distinct drifts on one row (`:730`):

| Leg | STORY-INDEX value | Actual |
|---|---|---|
| B2 catalog `input-hash` | `9603a5b` | story frontmatter `25c7324` |
| B3 blockquote `S-21.07=` | `9603a5b` (`:733`) | story frontmatter `25c7324` |
| BC pin | `[BC-5.39.010 v1.4]` | BC v1.9 |
| story version | "story v1.3" | story v1.4 |
| title / arm count | "seven-arm … (Classes A/B/D/E; BC-5.39.010 v1.4)" | six-arm, Classes A/B/E, Class D deferred |
| EC count | "31 ECs" | BC has 33 ECs (EC-001..EC-033) |

Plus `:734` BC coverage blockquote: "BC-5.39.010 v1.4 (S-21.07; … seven-arm …; Class D format-only — semantic existence gap out-of-scope)" — Class D is *deferred*, not format-only-in-scope.

The brief states the STORY-INDEX:730/733/734 corrections were "still OWED to state-manager and not yet applied", and directs me to treat the live state as authoritative. They remain unapplied, and the hash leg has **regressed further** during this session: the story's `input-hash` moved `52f0bf3` → `9603a5b` → `25c7324`, so both index sites now agree with each other on a value the story no longer declares. Under POLICY 18 the three-way equality is violated at two of three sites simultaneously.

Note the interaction with F-S2107-P4-013: because S-21.07's `inputs:` contains a volatile path, PC40 **suppresses** the Class B block for exactly this story — the gate cannot surface its own three-way inequality.

---

#### F-S2107-P4-013 — HIGH — PC40's "imposes no permanent weakening" guarantee is false for the governing story; ADR-037's remediation table omits S-21.07.

**Location:** `.factory/stories/S-21.07-validate-cross-site-correspondence.md:18` (`inputs:` entry) · `.factory/specs/architecture/decisions/ADR-037-input-hash-stable-input-constraint-volatile-artifacts-excluded.md:31-42` (§Context table), `:99-108` (§Decision 5) · `crates/.../src/arm_b.rs:337-342` · `BC-5.39.010.md:486-493` (PC40 transitional clause)

**Clause violated:** BC-5.39.010 PC40 transitional clause; ADR-037 §Decision 5 blast-radius enumeration.

**Evidence.** S-21.07's own frontmatter declares:

```yaml
inputs:
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md
  - …
  - .factory/specs/architecture/ARCH-INDEX.md      # ← line 18
```

`ARCH-INDEX.md` was **added** to `is_volatile_path` in this burst (`arm_b.rs:339`), closing F-P3-002(a) whose stated rationale was "ARCH-INDEX.md absent from impl — S-21.07 self-blocks on every write" (`arm_b.rs:1116-1122`). The chosen remedy widened the suppression rather than removing the volatile input — so `run_arm_b1` now returns advisory + Continue for S-21.07 unconditionally (`arm_b.rs:384-401`), permanently disabling Class B for the story that ships Class B.

PC40's transitional clause states the precondition is "vacuous once all stories with volatile inputs (**the 19 stories listed in ADR-037 §Context**) have had their `inputs:` arrays corrected per ADR-037 §Decision 5. After remediation, no story matches the volatile patterns and Class B enforces full BLOCKING severity for all stories with no carve-outs."

ADR-037 §Context enumerates: S-7.01, S-7.02, S-7.06-09, S-14.01, S-14.06-09, S-15.08, S-15.09, S-15.12, S-15.14, S-15.15, S-15.17, S-18.09, S-19.01. **S-21.07 is not in the table.** The §Decision 5 sweep will therefore not reach it, PC40 will never become vacuous, and the "imposes no permanent weakening" claim is untrue as written. Two coupled defects: ADR-037's blast-radius table is incomplete, and S-21.07's own `inputs:` array is unremediated. Under CLAUDE.md Rule 1 the correct in-scope action was to remove the volatile input from the story, not to widen the suppression list.

---

### MEDIUM

---

#### F-S2107-P4-014 — MEDIUM — `b1-volatile-input` fixture cannot produce the hash mismatch it documents; T-047's stated discrimination is unreachable.

**Location:** `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/b1-volatile-input/factory/stories/STORY-INDEX.md:11-13`, `:17`, `:19` · `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats:1177-1206`

**Evidence.** Fixture comments (`:11-13`):

```
<!-- B2 = catalog row input-hash = "xyz789"  (MISMATCH — deliberate) -->
<!-- B3 = blockquote S-21.07=xyz789 (MISMATCH — deliberate) -->
<!-- WITHOUT PC40: three-way comparison → B1≠B2 → exit 2 (BLOCK) -->
```

`xyz789` is **not hex**. `extract_input_hash_token` validates `matches!(c, '0'..='9' | 'a'..='f')` and requires length ≥7 (`arm_b.rs:533-540`) → returns `None`. The blockquote extractor applies the same charset (`arm_b.rs:626-633`) → `take_while` yields `""` → `None`.

So with PC40 removed the branch reached is `(None, None)` → "not yet registered in STORY-INDEX.md" **advisory** + Continue → **exit 0** (`arm_b.rs:206-216`). The claimed mutant does not exist. The exit-code assertion `_assert_exit 0` (bats:1192) holds identically with and without PC40.

T-047 retains partial discrimination via the `grep -qi 'volatile'` advisory check (bats:1196-1205), which would fail without PC40. But the fixture does **not** verify PC40's normative requirement that the hook "return `HookResult::Continue` for this story **WITHOUT proceeding to PC19-21 or performing the three-way comparison**". An implementation that emitted the volatile advisory *and then* ran the three-way check would pass T-047 unchanged. Same defect class as the eight fixtures repaired this burst, in the newest arm.

---

#### F-S2107-P4-015 — MEDIUM — AC-006 cascade is not verified at the Rust level; the test's own docstring concedes it never enters the comparison path.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/src/arm_a2.rs:538-554`

**Clause violated:** POLICY 11 (no test tautologies); story AC-006 ("all violations MUST be reported in ONE combined block … message references both BC IDs"); TD-VSDD-059.

**Evidence.** The test body:

```rust
/// AC-006: two stale BCs → single combined block (postcondition 7 cascade).
// Citations are extracted → run_arm_a2_for_bc called → host::read_file
// returns CapabilityDenied (non-WASM stub: -1) → fail-closed violation.
let (violations, _) = run_arm_a2("S-21.07", story_content);
assert!(!violations.is_empty(), "two stale BCs must produce combined violations");
```

The docstring states outright that the violations produced are `CapabilityDenied` host-read failures, not stale-citation comparisons. The assertion is `!is_empty()` only — it does not assert `[Class A Arm2]`, does not assert either BC ID, and does not assert a count of 2. Every fixture value (`v1.17`, `v1.5`, the BC-version frontmatter) is inert: the test passes with any story content whose `behavioral_contracts:` is non-empty and whose sections yield ≥1 citation.

bats AC-006 (`bats:539-563`) does assert both BC IDs and `[Class A Arm2]`, so the AC is covered at integration level — but the Rust test named for it is a tautology that reports green while exercising the error path.

---

#### F-S2107-P4-016 — MEDIUM — Shipped `pub fn extract_bc_index_version` re-collapses three states to `None` — the shape PC5 declares NON-CONFORMING.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/src/arm_a1.rs:280-287`

**Clause violated:** BC-5.39.010 v1.9 PC5 ("An implementation returning `Option<String>` that maps both `RowAbsent` and `RowPresentNoVersion` to `None` is **NON-CONFORMING**" … "A `RowMalformed` result MUST NOT be collapsed into `RowAbsent`"); §Architecture Anchors.

**Evidence.**

```rust
pub fn extract_bc_index_version(bc_id: &str, index_content: &[u8]) -> Option<String> {
    match extract_bc_index_version_state(bc_id, index_content) {
        BcIndexVersionState::Version(v) => Some(v),
        BcIndexVersionState::RowPresentNoVersion
        | BcIndexVersionState::RowAbsent
        | BcIndexVersionState::RowMalformed(_) => None,
    }
}
```

The doc comment justifies it as a "**Backward-compatibility wrapper** for the test at line 575 that calls this function directly." Three problems: (a) it is `pub`, i.e. part of the crate's API surface, and the BC names `extract_bc_index_version` as the §Architecture Anchor — a future caller reaching for the anchored name gets the non-conforming two-state shape; (b) the referenced "test at line 575" is a stale line pin (TD-VSDD-091) — the actual callers are `arm_a1.rs:728` and `lib.rs:741`; (c) it is exercised by a **live-corpus** test (`lib.rs:731-751`), so the non-conforming form is load-bearing in the suite, not merely vestigial. The conforming remedy is to delete it and re-target both callers at `extract_bc_index_version_state`.

---

#### F-S2107-P4-017 — MEDIUM — Non-empty-field counting and positional first-cell extraction diverge from PC5's normative predicate; a 6-column row with any empty cell silently escapes Arm A1.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/src/arm_a1.rs:153-179`

**Clause violated:** BC-5.39.010 v1.9 PC5 recognition predicate condition (2) ("the **first non-empty field**'s stripped content"); postconditions 2 and 4.

**Evidence — two divergences.**

(a) **Field counting drops empties before indexing.**

```rust
let non_empty_fields: Vec<&str> = escaped.split('|').map(str::trim)
    .filter(|s| !s.is_empty()).collect();
…
n if n >= 6 => { let sixth = non_empty_fields[5]; … }
```

A canonical 6-column row with an empty cell anywhere among the first five (e.g. an unassigned Capability or Stories cell) yields **5** non-empty fields → `RowPresentNoVersion` → the version cell is never read → a genuinely stale version cell produces **no violation**. That is a silent false-negative in the primary blocking arm. Symmetrically, a canonical 5-column row with one empty cell yields 4 → `RowMalformed`. Indexing `non_empty_fields[5]` also assumes the version cell survives at non-empty position 5, which only holds when all preceding cells are populated.

(b) **First cell is positional, not first-non-empty.** `line.splitn(3, '|')` skips segment 0 and takes segment 1 verbatim (`:153-155`). PC5 says "first **non-empty** field … after escape-aware splitting". A row with a leading empty cell (`|  | BC-5.39.010 | … |`) gives `first_cell == ""` → predicate fails → the line is not a candidate → `RowAbsent` → **blocking path** for any BC at version > 1.0. Note also that `splitn` operates on the raw line, not the escape-substituted one, so it is not the escape-aware split PC5 mandates.

Live corpus exposure is currently nil — one empty-cell line exists in BC-INDEX.md (`:533`, the `| **Total** | | **1975** | |` summary row) and it is not a BC-ID candidate. Latent, not live, hence MEDIUM rather than BLOCKER; but (a) is a false-negative in the arm whose false-*positive* (F-P3-001) was the pass-3 BLOCKER, and no test covers either boundary.

---

#### F-S2107-P4-018 — MEDIUM — bats synthetic registry grants `.factory/cycles/`, diverging from the AC-020 production capability set; the suite never exercises the shipped `path_allow`.

**Location:** `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats:130-133`, `:490-491` vs `plugins/vsdd-factory/hooks-registry.toml:700-704`

**Clause violated:** Story AC-020 (`path_allow` with `.factory/cycles/` commented out, DEFERRED v1.6); BC-5.39.010 v1.6 §Deferred Scope ("The `.factory/cycles/` path_allow entry is removed from the registry").

**Evidence.** `_write_registry`'s default:

```bash
path_allow_lines='".factory/specs/behavioral-contracts/",
  ".factory/specs/verification-properties/",
  ".factory/stories/",
  ".factory/cycles/"'
```

Production registry (correct, verified):

```toml
path_allow = [
  ".factory/specs/behavioral-contracts/",
  ".factory/specs/verification-properties/",
  ".factory/stories/",
]
```

AC-004's override (`:490-491`) likewise passes `".factory/stories/", ".factory/cycles/"`. All 38 non-skipped payload tests therefore run against a capability set the product does not ship. Functionally benign today because no active arm reads under `cycles/`, but the divergence means the suite cannot detect a regression that reintroduces a `cycles/` read, and it contradicts the AC that defines the registry shape. It is also the last un-swept residue of the Class D deferral in the test layer.

---

#### F-S2107-P4-019 — MEDIUM — CHANGELOG `[Unreleased]` entry is stale and factually wrong on three counts.

**Location:** `CHANGELOG.md:11`

**Clause violated:** Story Task 20 / AC-level CHANGELOG delivery; POLICY 14 leg-5-analogous downstream currency.

**Evidence.** The entry (F-P3-007's claimed closure) states:

1. `"(BC-5.39.010 v1.8, E-21 W4)"` — BC is **v1.9**. The entry was written for v1.8 and not re-swept when v1.9 landed.
2. `"rearchitected to three-state BcIndexVersionState enum (RowAbsent / RowPresentNoVersion / Version(String))"` — the shipped enum is **four**-state, `RowMalformed(usize)` included (`arm_a1.rs:93-105`). The release note describes an implementation that does not exist.
3. `"Stale BC-5.39.010 v1.2 cites updated to v1.8 across Cargo.toml, lib.rs, main.rs, and red-gate-log.md."` — false as a completion claim; see F-S2107-P4-011 (registry and bats still v1.2).

Two further inaccuracies: F-S2107-P3-002 is labelled `(HIGH)` where the burst brief and `red-gate-log.md:682-684` record it as **BLOCKER**; and "Closes six adversarial findings from pass-3" under-reports a burst that the brief credits with ~16 closures (F-P3-003/007/008/010/017/018/019/020/022/025 are all absent from the entry).

---

#### F-S2107-P4-020 — MEDIUM — `is_volatile_path` narrows PC40 patterns 2–5 from "contains" to `starts_with`; its doc table says "Six canonical patterns" and enumerates eight.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/src/arm_b.rs:282-343`

**Clause violated:** BC-5.39.010 PC40 pattern table ("path **contains** `.factory/cycles/` AND ends with `/STATE.md`" etc.); D-950 `L-BB-spec-drift-by-widening` (inverse direction — narrowing).

**Evidence.** PC40 specifies *contains*:

| Pattern | Matches when |
|---|---|
| `.factory/cycles/**/STATE.md` | path **contains** `.factory/cycles/` AND ends with `/STATE.md` |

Implementation (`:326`):

```rust
if path.starts_with(".factory/cycles/") {
    let filename = Path::new(path).file_name()…;
    return VOLATILE_PATTERNS_CYCLES_NAMED.contains(&filename);
}
```

Any spelling of a cycles path that is not repo-root-relative escapes classification and re-enters the blocking three-way path. The narrowing is undetectable by the current tests, all of which use repo-root-relative literals (`arm_b.rs:1064-1085`).

Secondary defect in the same block: the doc comment declares "**Six** canonical patterns; each is checked in order by `is_volatile_path`" (`:284`) and then enumerates a table of **eight** numbered rows (`:288-296`). The constant is also named `VOLATILE_PATTERNS_CYCLES_NAMED`, not the `VOLATILE_PATTERNS` the burst brief records as the closure artefact — a reader grepping for the named deliverable finds nothing.

---

#### F-S2107-P4-021 — MEDIUM — BC-5.39.010 §Story Anchor, §Traceability `Stories`, and BC-INDEX Stories cell all read `TBD` while S-21.07 is v1.4 and in flight.

**Location:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md:1170-1172` (§Story Anchor), `:1147` (§Traceability `Stories | TBD`) · `.factory/specs/behavioral-contracts/BC-INDEX.md:1459` (Stories cell `TBD`)

**Clause violated:** POLICY 4 semantic anchoring integrity; POLICY 5 (creators justify anchors).

**Evidence.** BC §Story Anchor reads verbatim: `"TBD — no story allocated yet."` — yet `.factory/stories/S-21.07-validate-cross-site-correspondence.md:34` declares `behavioral_contracts: [BC-5.39.010]`, the story is at v1.4, and `.factory/stories/STORY-INDEX.md:730` registers it. Three amendments (v1.7, v1.8, v1.9) were authored *in response to findings raised against S-21.07's implementation* without ever anchoring the story back.

Consequence: a reader arriving at BC-INDEX.md:1459 sees `Stories = TBD` and cannot reach the implementing story; a reader of the BC's §Story Anchor is told none exists. Bidirectional traceability BC↔story is broken in the BC→story direction. Mis-anchoring never converges.

---

#### F-S2107-P4-022 — MEDIUM — PC5's "valid body-table row candidate" definition is self-contradictory with the `RowMalformed` definition; the Gate Spec pseudocode resolves it, the normative prose does not.

**Location:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md:123-132` (`RowMalformed`) vs `:134-144` (normative recognition predicate) vs `:1028-1032` (Gate Spec pseudocode)

**Ruling requested by the brief — my adjudication: the orchestrator was correct that it is operationally unambiguous, and wrong to leave it unfixed. It is a genuine MEDIUM defect, not a non-issue.**

**Evidence.** `:134-138` defines candidacy as requiring all three conditions:

> "A line is a valid BC-INDEX body-table row **candidate** for BC ID X if and only if **ALL THREE** conditions hold: (1) … (2) … AND (3) … the total non-empty field count is **≥5**."

`:123-130` defines `RowMalformed` as:

> "a **candidate line was found** (matches the locator pattern) but after escape-aware splitting the total non-empty field count is **<5**."

Under the `:134` definition, a line failing (3) is *not* a candidate — so `RowMalformed` is empty by construction. The two clauses cannot both be read literally.

The contradiction *is* resolvable: two distinct predicates are in play — a **locator** predicate (conditions 1+2) governing candidacy, and a **validity** predicate (condition 3) governing classification. The Gate Spec pseudocode states this explicitly and correctly at `:1032`:

```
//   if (1)+(2) match but (3) fails → RowMalformed; if no (1)+(2) match → RowAbsent
```

and the implementation follows the pseudocode faithfully (`arm_a1.rs:146-185`). So the orchestrator's judgment that a fourth product-owner round was unnecessary to *unblock implementation* was sound.

But the normative prose at `:134-144` still says what it says, and PC5 never names the locator predicate as a separate concept — it uses "candidate" for both. The single word "valid" at `:134` is carrying the entire distinction, and `:142-144`'s corpus verification ("0 lines match conditions (1)+(2) but not (3)") demonstrates the authors *knew* (1)+(2)-only lines were a coherent category while the sentence above declares they are not candidates. An implementer working from the prose rather than the pseudocode would conclude `RowMalformed` is unreachable and omit it — which is close to what happened (F-S2107-P4-003: the state is implemented but wholly untested). The fix is one sentence naming the locator predicate. Under CLAUDE.md Rule 6 that is mechanical and answerable in scope.

---

### LOW

---

#### F-S2107-P4-023 — LOW — `b1-b3-only-mismatch` story fixture is a verbatim copy of `b1-hash-match`'s, carrying the opposite expectation in its docstring.

**Location:** `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/b1-b3-only-mismatch/factory/stories/S-21.07-test.md:8`, `:15`, `:17-21`

**Evidence.** The fixture for T-037 (which expects **exit 2** on a B3-only divergence) reads:

```yaml
last_amended: "2026-01-01 (v1.1) — test fixture: B1 hash-match control"
```
```
# S-21.07: test fixture for B1 hash-match control

Fixture: all three sites B1=B2=B3="47a65c9". Expected: Class B Arm1 passes, exit 0.
```

Byte-identical to `b1-hash-match/factory/stories/S-21.07-test.md`. In the `b1-b3-only-mismatch` scenario the sibling STORY-INDEX sets the blockquote to `deadbee` (`:25`), so the three sites do **not** agree and the expectation is exit 2 (bats:643-657). Behaviour is correct — `input-hash: "47a65c9"` is the right value — but the fixture self-documents the inverse scenario. Third instance of the stale-docstring class after the two repaired this burst; a maintainer debugging T-037 is actively misdirected.

---

#### F-S2107-P4-024 — LOW — Pass-numbering incoherence across red-gate-log, CHANGELOG, and cascade state.

**Location:** `crates/.../docs/red-gate-log.md:701` ("Pass-5 Amendment"), `:704` ("while pass-4 burst was in flight"), `:794` ("This is the 5th fixture-shape defect of this burst"), `:785` ("Amendment 2") · `CHANGELOG.md:11` ("pass-5 adversarial fix burst")

**Evidence.** Convergence state is 3 passes done, this review is pass-4, and the burst under review closes pass-3 findings. The red-gate-log labels the same work "Pass-5" while its own subtitle says "pass-4 burst was in flight", and CHANGELOG says "pass-5". `:794` counts "the 5th fixture-shape defect of this burst" where the brief records eight. POLICY 16-adjacent (identifier discipline): a reader cannot map log sections to adversary passes.

---

#### F-S2107-P4-025 — LOW — Postcondition 4a's prescribed advisory text is not used verbatim and nothing pins it.

**Location:** `crates/.../src/arm_a1.rs:406-415` vs `BC-5.39.010.md:535-539`

**Evidence.** Prescribed:

> `"… (<N> fields found; expected ≥5 for a valid body-table row). This line is structurally not a BC-INDEX body-table row (likely a Changelog entry or notes table). Registration status cannot be determined from this line. Verify BC-INDEX body-table registration manually."`

Shipped omits "Registration status cannot be determined from this line" and "Verify BC-INDEX body-table registration manually", and injects unspecified prose ("Not blocking — this is not a dropped registration", "The genuine dropped-registration case (no candidate line at all) is RowAbsent (postcondition 4)"). The operator-actionable instruction — verify registration manually — is the clause dropped. Compounded by F-S2107-P4-003: with zero tests touching this state, no assertion pins any part of the message.

---

## Observations

- **O-S2107-P4-001 — BC v1.9's load-bearing corpus figures verify exactly.** Independent counts against the live BC-INDEX:
  ```
  grep -c '^| \[?BC-[0-9]\+\.[0-9]\+\.[0-9]\+'                          → 1983
  grep -c '^| \[?BC-[0-9]\+\.[0-9]\+\.[0-9]\+.*| v[0-9]\+\.[0-9]\+'    →   40
  ⇒ 5-field (RowPresentNoVersion) = 1943;  RowMalformed = 0
  ```
  PC5's 1983 / 1943 / 40 and the "0 RowMalformed lines" claim are all accurate. The product-owner's corpus record is sound; the 194-row story-ID hazard figure is consistent with these totals. No finding.

- **O-S2107-P4-002 — Class D tombstoning is complete on the reachability axis.** `dispatch::is_cycle_artifact` returns `None` unconditionally (`dispatch.rs:193-197`); the `lib.rs` dispatch block is removed (`:214-218`); the dead `Err(e) if cycle_kind.is_some()` fail-open arm is gone (`:162-165`); `.factory/cycles/` is out of the production `path_allow`; 15 `arm_d` + 2 `dispatch` tests are `#[ignore]`d = the claimed 17. `arm_d.rs` and `CycleArtifactKind` are unreachable from `on_post_tool_use`. **No reachable residue and no behavioural contradiction remains.** The residue is purely documentary and is captured as F-S2107-P4-010 and F-S2107-P4-009. Verdict on brief item 5: Class D deferral is complete.

- **O-S2107-P4-003 — The registry entry itself is AC-020-conformant.** `hooks-registry.toml:689-704`: 8 fields present (`name`, `event`, `tool = "^(Edit|Write|MultiEdit)$"`, `plugin`, `priority = 460`, `timeout_ms = 8000`, `on_error = "continue"`, `async = false`), `path_allow` at three subtrees, no `fuel_cap`. Only the surrounding comment block is defective. The WASM artefact is present at `plugins/vsdd-factory/hook-plugins/validate-cross-site-correspondence.wasm` (D-693 satisfied structurally; the claimed 224,597-byte size is unverifiable read-only).

- **O-S2107-P4-004 — CI coverage for this suite uses the release leg, so AC-019's release mandate is met where it matters.** `ci.yml:219-259` (cargo-host job) stages **debug** WASM but runs only a named subset of bats files (`:315`). The `bats-full-suite` job that executes `run-all.sh` — and therefore this story's suite via the `tests/*.bats` glob — stages from `--release` builds (`:604-654`). Story Task 16 needs no `run-all.sh` edit for the same reason. **Not filed as a finding**; recorded so a later pass does not re-derive it as one.

- **O-S2107-P4-005 — `[process-gap]` — the red-gate-log permits self-authored dispensations with no reviewer gate.** `red-gate-log.md:931` records a known defect in seven fixtures and closes it with `FUNCTIONALLY CORRECT ✓` in the same table where its sibling was corrected. POLICY 15 governs *evidence format* (verbatim command + captured stdout) but imposes no constraint on **disposition**: a row may say "CONFORMANT ✓" or "FUNCTIONALLY CORRECT ✓" without any test, mutant, or reviewer sign-off. The result (F-S2107-P4-007) is a documented defect that survived into a claimed-clean burst by self-certification. Recommend a POLICY 15 extension: any red-gate-log row whose disposition is not `CORRECTED` or `CONFORMANT-with-test` must carry an explicit deferral target (story ID) per CLAUDE.md Rule 3, or be re-opened.

---

## Part B — Analysis and Verdict

### Verdict: **NOT-CLEAN**

25 findings — 4 BLOCKER, 9 HIGH, 9 MEDIUM, 3 LOW. Streak resets to **0/3**. Trajectory: 47 → 18 → 25 → **25**.

### Why the count did not fall

Pass-3 produced 25; pass-4 also produces 25. That is not a plateau of nitpicks — the composition changed substantially. Only two of the 25 are re-raises of unclosed pass-3 items (F-S2107-P4-012 re-raising F-P3-003's STORY-INDEX legs; F-S2107-P4-011 re-raising F-P3-010's incomplete sweep). The remaining 23 are new, and the four BLOCKERs are all first-observations.

The reason novelty stayed high is structural: **the burst amended the governing spec three times while implementing it.** Each hop (v1.7 → v1.8 → v1.9) created a new propagation surface, and the propagation was performed once — at v1.7 — for the story, and zero times for BC-INDEX. Three of the four BLOCKERs are direct consequences:

- F-S2107-P4-001 (BC-INDEX stuck at v1.6, three hops behind)
- F-S2107-P4-002 (story stuck at v1.7, two hops behind)
- F-S2107-P4-003 (`RowMalformed`, introduced at the *last* hop, arrived after test-writing had already concluded — the red-gate-log's final amendment moved a fixture *off* the state and added nothing back)

This is the failure mode the BC itself is designed to gate, occurring in the BC's own delivery. Both Class A arms and Class B would fire against S-21.07's own artefacts if the hook were live today. I verified this by tracing the shipped predicates, not by inference:

| Arm | Trigger artefact | Traced result |
|---|---|---|
| A1 | `ss-05/BC-5.39.010.md` | `Version("1.6")` vs fm `1.9` → **BLOCK** |
| A2 | `S-21.07-…md` §Behavioral Contracts | Phase-1 pure-version `1.7` vs fm `1.9` → **BLOCK** |
| A2 | `S-21.07-…md` §Token Budget | Phase-2 mandatory-v `1.7` vs fm `1.9` → **BLOCK** |
| B1 | `S-21.07-…md` | PC40 volatile (`ARCH-INDEX.md`) → **suppressed** (F-S2107-P4-013) |
| E1 | either artefact | `last_amended: \|-` → `"\|-"` → unparseable **advisory** (F-S2107-P4-004) |

Two of the five would-be signals are suppressed by the hook's own carve-outs, on the two artefacts most in need of them.

### The dominant defect class is not closed

The brief identified fixture-shape defects as the dominant class and asked for an exhaustive sweep. I found four more instances:

1. **F-S2107-P4-007** — seven E-class BC-INDEX fixtures, 5-field shape, comments asserting a version-match A1 semantics the shape cannot produce. **Self-disclosed at `red-gate-log.md:931` and dispensed as "FUNCTIONALLY CORRECT ✓"** while the byte-identical sibling defect in `combined-a1-e1` was fixed in the same amendment. This is the single most important finding of the pass after the BLOCKERs, because it is not an oversight — it is a recorded decision to leave the class open.
2. **F-S2107-P4-014** — `b1-volatile-input`'s deliberate "mismatch" is non-hex and therefore invisible to both extractors; T-047 cannot fail for the reason it documents.
3. **F-S2107-P4-023** — a third stale-docstring fixture (`b1-b3-only-mismatch` carrying `b1-hash-match`'s expectations verbatim).
4. **F-S2107-P4-004** — the same class promoted from fixture to *production parser*: `extract_frontmatter_field` encodes an imagined `last_amended:` shape, and every test and fixture encodes the same imagined shape, so the block-scalar form used by both governing artefacts is invisible to the whole suite.

Four green tests that do not enter the paths they name (F-S2107-P4-003 zero-coverage, F-S2107-P4-014 unreachable mutant, F-S2107-P4-015 self-confessed wrong path, F-S2107-P4-007 × 7 short-circuited) sit alongside 118 passing crate tests and 43 passing bats tests. Exit codes are green; several of them are green for the wrong reason.

### Ruling on brief item 2 — the four-state boundaries

Three separate questions, three separate answers:

**Is the `RowAbsent`/`RowMalformed` split coherent?** Yes, as *definitions*. `RowAbsent` = no line satisfies locator conditions (1)+(2); `RowMalformed` = ≥1 such line with <5 fields. Disjoint, exhaustive with the other two, and the disposition asymmetry (block vs advisory) is correctly motivated: found-but-ambiguous is genuinely not evidence of a dropped registration.

**Can `RowMalformed` absorb cases belonging to `RowAbsent`, neutering the dropped-registration catch?** Not by definition — but **yes, by scan order**, and this is the pass's most consequential new finding (F-S2107-P4-005). `extract_bc_index_version_state` returns on the *first* candidate line rather than searching for a *valid* one. One earlier malformed candidate line silently disables **both** PC2 (stale version) and PC4 (dropped registration) for that BC, permanently. The v1.9 amendment was authored to eliminate a false-BLOCK class (≥1,712 rows); the implementation of it introduced a false-*silence* class gated only on document ordering. The BC does not prescribe first-match-wins; the code invented it. That is the mirror-image risk the brief asked about, and it is present in the shipped binary.

**Is the recognition predicate implementable without inference?** Yes — conditions (1), (2), (3) are all mechanical and the Gate Spec pseudocode is unambiguous. Two implementation-level divergences remain (F-S2107-P4-017): non-empty filtering before positional indexing, and a raw-`splitn` positional first cell instead of the escape-aware first-non-empty field. Both are latent against today's corpus, and (a) is a false-negative in the primary blocking arm.

**Was the orchestrator right to decline a fourth product-owner round on the self-referential wording?** On the narrow question — can an implementer proceed unambiguously — **yes**. The Gate Spec pseudocode at `BC-5.39.010.md:1032` states the locator/validity split explicitly, and the implementation followed it correctly. On the broader question — is the spec fit to ship — **no** (F-S2107-P4-022). The normative prose at `:134-144` still contradicts `:123-132`, and the corpus-verification sentence at `:142-144` proves the authors were reasoning about a category the sentence above declares nonexistent. The remedy is one sentence naming the locator predicate; under CLAUDE.md Rule 6 that is mechanical and owed in scope. And there is circumstantial support for the "unfit prose has consequences" reading: the state whose definition is self-referential is precisely the state that shipped with zero tests.

### Spec-conformance sweep (brief item 3) — results

Checked every predicate for both narrowing and widening against v1.9:

| Predicate | Verdict |
|---|---|
| four-state classifier | **narrowing + ordering defect** — F-S2107-P4-005, F-S2107-P4-017 |
| escape-aware split | conformant for field counting; **not applied** to first-cell extraction — F-S2107-P4-017(b) |
| `VOLATILE_PATTERNS` | **narrowing** (`starts_with` vs spec `contains`) — F-S2107-P4-020. Content otherwise exact: ARCH-INDEX added, VP-INDEX removed, blanket `cycles/**` removed, path-equals for the three indexes. F-P3-002 substantively closed. |
| two-phase PC13 | **conformant.** Phase 1 rightmost pure-version, Phase 2 rightmost mandatory-v, three collision classes covered by tests grounded in real corpus rows. No drift found. |
| `is_canonical_vp_filename` | **conformant** — digit predicate + explicit `VP-INDEX.md` guard per PC34 bullet 2's normative REQUIRED. |
| `is_canonical_story_basename` | **conformant** — `^S-[0-9]+\.[0-9]+`; sibling-swept to `arm_b::extract_story_id_from_table_row` via `parse_story_id_len` (F-P3-015 genuinely closed, verified at `arm_b.rs:560-573`). |
| epic basename guard | **conformant** — three components + `^E-[0-9]+-.*\.md$`, with a GREEN over-exclusion guard. |
| `first_cell_matches_bc_id` | **narrowing** on "first non-empty field" — F-S2107-P4-017(b) |
| `extract_frontmatter_field` | **imagined-shape defect** — F-S2107-P4-004 (BLOCKER) |

No instance of the D-950 widening pattern (`is_ascii_hexdigit`, conditional initializers) recurred: `arm_b.rs:128-131` and `:624-629` both carry explicit `matches!(c, '0'..='9' | 'a'..='f')` with a "do not widen to is_ascii_hexdigit()" comment, and `skip_section` is initialised to `true` unconditionally with the rationale inline (`arm_a2.rs:99-108`). **Both prior drift-by-widening lessons held.** The drift this pass is in the opposite direction — narrowing — plus one imagined-shape parser.

### Red-gate integrity (brief item 6)

POLICY 15 evidence quality is high where it exists: `red-gate-log.md:713-735` carries a verbatim command and 18 captured `file:line:` panic sites with the real `test result: FAILED. 100 passed; 18 failed; 17 ignored` line — genuine captured stdout, not paraphrase. Amendment 2 records before/after fixture bodies and the `117 → 118 passed` transition.

The failure is one of **completeness, not fidelity**:
- The log stops at v1.8 for red-gate authorship. The v1.9 `RowMalformed` amendment has no red-gate entry, no mutant, no control (F-S2107-P4-003).
- `:931` closes a live defect by self-certification with no test (F-S2107-P4-007, O-S2107-P4-005).
- `:671` asserts "No eighth spec-describes-imagined-shape instance found. All 13 RED GATE tests are grounded in behavior observable in the live corpus" — F-S2107-P4-004 is a ninth instance, in the shared parser, and it is not corpus-grounded: the sole corpus E1 test samples `VP-100.md`, which uses the inline form.

### What must land before pass-5

The four BLOCKERs are all mechanical and none requires a spec amendment:

1. BC-INDEX.md:1459 version cell `v1.6` → `v1.9`; Stories cell `TBD` → `S-21.07` (F-S2107-P4-001, F-S2107-P4-021) — **state-manager**.
2. Story S-21.07 v1.4 → v1.5, propagating BC v1.9 to title/H1/BC Status/BC table/Token Budget/Task 1, five-leg parity (F-S2107-P4-002) — **story-writer**.
3. `RowMalformed` mutant + control at unit level and one bats fixture; assert `field_count` and the prescribed message text (F-S2107-P4-003, F-S2107-P4-025) — **test-writer**.
4. `extract_frontmatter_field` block-scalar support (`|`, `|-`, `>`, `>-`) with a fixture carrying the real `last_amended: |-` shape, plus a corpus test sampling BC-5.39.010.md itself (F-S2107-P4-004) — **test-writer + implementer**.

F-S2107-P4-005 (first-candidate-wins) is the highest-value HIGH: a five-line change to prefer a valid candidate, plus the ordering mutant that proves it. F-S2107-P4-007's seven fixtures are a ten-minute sweep whose only obstacle is that the burst already decided not to do it.

One routing note: F-S2107-P4-013 and F-S2107-P4-022 are spec-side (ADR-037 §Context table incomplete; BC PC5 prose contradiction) and route to **architect** and **product-owner** respectively — not to the implementer. F-S2107-P4-012's STORY-INDEX legs remain OWED to **state-manager** from pass-3 and have regressed further since.

**Novelty: HIGH.** Four first-observation BLOCKERs, including a blocking arm that is structurally inert on its own governing artefacts and a new false-silence channel in the arm that was pass-3's BLOCKER. This spec is not converging; it is being amended faster than it is being propagated.
agentId: ae4af706ef4549c74 (use SendMessage with to: 'ae4af706ef4549c74', summary: '<5-10 word recap>' to continue this agent)
<usage>subagent_tokens: 423479
tool_uses: 69
duration_ms: 968862</usage>
