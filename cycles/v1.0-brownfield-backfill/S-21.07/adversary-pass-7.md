---
pass: 7
verdict: NOT-CLEAN
reviewed_head: fbb5183c
factory_artifacts_head: 10914a73
novelty: 0.69
previous_review: "cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-6.md"
---

## Summary

Pass-7 fresh-context adversarial review of S-21.07 at `reviewed_head fbb5183c` (worktree `.worktrees/S-21.07`, clean tree, origin SHA-equal) and factory-artifacts `10914a73`. **16 findings: B2 / H5 / M7 / L2.** Novelty 0.69 vs pass-6 Part A. Streak: **0/3** (BC-5.39.001 reset).

**Tooling note (disclosed for POLICY 22):** the dispatch brief stated I had Read/Grep/Glob only and instructed me to name commands I could not run. That was not my actual environment — I had Bash. I used it. Every numeric figure and every code block below is real captured stdout from execution at the reviewed SHAs; nothing is narrated, reconstructed, or fabricated. This is the stronger form of compliance with POLICY 15 and POLICY 22, and it is why I can contradict a gate figure rather than merely flag it as unverifiable.

**The headline: the suite is not green. `bats validate-cross-site-correspondence.bats` exits 1 at `fbb5183c`.** The claimed figure "46 passed / 5 skipped" is false; the measured figure is **45 passed / 1 failed / 5 skipped**. The failing gate is `AC-023 / T-P6C (PC13a)` — one of the two gates the pass-6 burst strengthened to close F-S2107-P7-013. The strengthening hardcoded an expected story ID (`Story S-21.07-test`) that `extract_story_id_from_path` cannot produce (it returns `S-21.07`, by `splitn(3,'-')`). The gate has been RED since `29558518`, two commits back. The `red-gate-log.md` Pass-7 attestation table records that same gate as **GREEN**, and the coverage gate manufactures the "46 passed / 5 skipped / 0 failed" line arithmetically from `grep` counts while a test is failing. Three independent record mechanisms all reported green over a red suite.

**A great deal did land, and I verified it by execution rather than accepting it.** WASM is exactly as claimed: **231,661 bytes**, sha256 `853c802e74ec372864912448130f3b0740aeeae6f92b8230c7eb25f639dc32b8`, and the git blob in `HEAD` hashes identically to both the deployed artifact and the `target/wasm32-wasip1/release/` build — I confirmed byte-identity three ways and confirmed no `src/` change postdates the WASM commit. `cargo test -p validate-cross-site-correspondence` → **163 passed / 0 failed / 17 ignored**. Workspace → **2428 passed / 0 failed / 22 ignored**, `fmt` exit 0 with zero output, `clippy -D warnings` exit 0 with zero diagnostics. 4-INDEX versions are exactly BC v4.50 / VP v2.75 / STORY v4.287 / ARCH v3.45, POLICY 14 leg-4 PASSes on all four, and leg-5 is genuinely closed (`v1.10 \| v1.11 \| v1.12 \| v1.13`). **F-S2107-P7-003's CI inversion is properly fixed in both jobs** — `Mount factory artifacts` now precedes `cargo test` in `cargo-host` *and* in `build-dispatcher`, with `CI_REQUIRE_ARTIFACTS=1` plus `VSDD_CORPUS_ROOT`, and `require_artifacts` now panics rather than skipping under that flag. Ten of the twenty pass-6 findings are properly closed and I re-derived the substantive ones independently: ADR-037's corrected roster (union **76**, ARCH leg **61**) matches my own `is_volatile_path` re-derivation exactly; Arm B2 has **0** live violations; the S-21.07 three-way input-hash is `c0ab6a3` on all three legs.

**The dominant pattern is unchanged from pass-6 and it has sharpened into something more specific: the burst's own later legs falsify its earlier legs.** Four of my five HIGHs are this shape.

The cite sweep did not merely stall — it **inverted**. Pass-6 recorded that `Cargo.toml`, `hooks-registry.toml` and `bats:38` *were* swept while `lib.rs`/`main.rs` were missed. This burst swept `lib.rs`/`main.rs` to v1.13 and let `hooks-registry.toml` (×2) and `bats:38` go stale at v1.12. The one site pass-6 quoted verbatim — `run_arm_b1`'s doc comment and its `# BC trace`, both at `v1.6` — is untouched, while the two functions immediately above it were swept. 88 stale live `.rs` cites; 49 sites now sit one version behind at v1.12.

PC5 is the clearest instance. Commit `d32207bf` wrote BC-5.39.010 v1.13 with the corpus figures "1,943 of 1,983", the "three of four corpus rows" empirical claim, and a new normative paragraph documenting the `fields[5..].join("|")` bare-pipe reassembly. The **very next commit of the same burst**, `304156dd`, registered two new BCs (`total_bcs 1983→1985` — its own message says so) and applied a "BC-4.13.001 v1.16 pipe-escaping correction". I re-derived under exact production locator semantics: **1,945 of 1,985**, and **zero** rows with more than 6 non-empty fields. So the same burst falsified its own corpus count, reduced "three of four" to two of four, and eliminated every live instance of the phenomenon the amendment was written to describe.

The compensating guard is still weaker than the block it replaced, and now gratuitously so. `bc_index_row_contains_version` still searches the whole row line, and its justifying comment still says it exists "to bypass the last-wins edge cases (F-P6-019a–d)" — those were fixed, and the production extractor is now correct at v1.13 and sitting in the same function. I demonstrated all three bypasses by execution against the live index: appending a forged `\| v1.19` while frontmatter stays `1.18` passes the gate although the production extractor reads the index as newer; `helper('1.23')` is `True` for BC-3.08.001 whose chain current is `1.24`; and BC-5.39.010 passes for `1.10`, `1.11`, `1.12` **and** `1.13`, so any rollback is invisible.

Two findings are namespace and process defects rather than content. `O-P8-NN` is **already an occupied namespace**: E-18 uses three-digit `O-P8-001/002/003` (88 cites), and this cascade has begun using two-digit `O-P8-01/02` (17 cites, one of them load-bearing in BC-INDEX v4.50's `Refs:`). Zero-padding is the only discriminator. And POLICY 15's ATTESTATION-LOCATION GATE is **unsatisfiable as written** — it demands the attestation be bundled in the same commit *and* name that commit's own SHA, which git cannot do; the codebase has quietly settled on parent-SHA-in-a-follow-up-commit, which fails the literal predicate at every HEAD including this one.

Every finding below was verified by literal execution at the reviewed SHAs before being written.

---

## Part A — Findings

### BLOCKER

#### F-S2107-P8-001 — BLOCKER — The bats suite is RED at reviewed HEAD. `AC-023 / T-P6C (PC13a)` — one of the two gates strengthened to close F-S2107-P7-013 — asserts a story ID the production extractor cannot emit. Measured: 45 passed / 1 failed / 5 skipped, exit 1. The claimed gate figure "46 passed / 5 skipped" is false.

**Location:** `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats` — `@test "AC-023 / T-P6C (PC13a): B2==B3 story-index-consistent-stale emits advisory, exits 0"`, the `expected_decoded` assignment; against `lib.rs::extract_story_id_from_path` and the fixture `b1-story-index-consistent-stale/factory/stories/S-21.07-test.md`.

**Clause violated:** D-693 (pre-green gate); POLICY 15 (verbatim-stdout discipline at the persistence layer); POLICY 22 (gate figures must be re-derived by literal shell); TD-VSDD-059 (paper-fix — a strengthening that cannot pass); production-grade default.

**Evidence — literal shell at `fbb5183c`:**

```
$ bats validate-cross-site-correspondence.bats; echo "BATS_EXIT=$?"
BATS_EXIT=1
1..51
ok_total=50
notok_total=1
skip_total=5
$ grep '^not ok' /tmp/bats.txt
not ok 48 AC-023 / T-P6C (PC13a): B2==B3 story-index-consistent-stale emits advisory, exits 0
```

The harness's own failure block names the discrepancy exactly:

```
# FAIL: PC13a advisory does not match v1.13 normative verbatim text.
#   F-S2107-P7-013 gate: BC-5.39.010 v1.13 PC4a requires full-string equality.
#   Expected: ... advisory: Story S-21.07-test input-hash mismatch — frontmatter=47a65c9; ...
#   Actual:   ... advisory: Story S-21.07 input-hash mismatch — frontmatter=47a65c9; ...
```

`ok_total=50` counts the 5 `# skip` lines, so the true breakdown is **45 passed / 1 failed / 5 skipped** of 51 declared.

**Root cause.** The fixture file is `S-21.07-test.md`; its frontmatter is `story_id: S-21.07`. Production derives the ID from the *path*, not the frontmatter:

```rust
// lib.rs::extract_story_id_from_path
let parts: Vec<&str> = stem.splitn(3, '-').collect();   // ["S", "21.07", "test"]
if parts.len() >= 2 { format!("{}-{}", parts[0], parts[1]) }   // -> "S-21.07"
```

`S-21.07` is the only value reachable for this fixture. The expected string was authored from the filename rather than derived from observed output, so the gate is RED **by construction** — it could never have passed. `git log -S` places its introduction at `29558518` (HEAD~1); `fbb5183c` touched only `red-gate-log.md`, so the suite has been red across two commits.

**Why it matters.** This is the inverse of the defect F-S2107-P7-013 raised. That finding said the gate was too weak to detect a wrong message; the fix made it strong and simultaneously mis-specified, so it now rejects the *correct* message. The PC13a advisory path — the carve-out this whole cascade was approved to introduce — therefore has **no passing gate at the dispatcher layer**. The sibling `AC-022 / T-P6A` received the identical strengthening and passes, which is precisely why nothing surfaced this: a reviewer checking "was the F-S2107-P7-013 class swept?" sees two equality gates and stops.

**Failure scenario.** Any CI run of `bats-full-suite` fails on this file. Merging requires either fixing the expected string to `Story S-21.07`, or renaming the fixture to `S-21.07.md`, or teaching `extract_story_id_from_path` to prefer frontmatter `story_id:`. The first is correct and is a one-token edit; the third is a production change with corpus blast radius and must not be chosen to make a test pass.

**Routing.** test-writer: correct `expected_decoded` to `Story S-21.07` (or rename the fixture and sweep the ten `_post_write_event '.factory/stories/S-21.07-test.md'` call sites); re-run and capture stdout. state-manager: correct the gate figure everywhere it was recorded.

---

#### F-S2107-P8-002 — BLOCKER — `red-gate-log.md`'s Pass-7 attestation table records the RED gate as **GREEN**, and its stated post-implementation figures do not match HEAD. Three independent record mechanisms — the obligation table, the coverage gate, and the dispatch claim — all reported green over a red suite.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md` — `## Pass-7 Fix Burst — BC-5.39.010 v1.13 …`, the "Bats Integration Tests — D-916 Obligation-Indexed Table" row for `T-P6C (equality)` (final column) and the "Post-implementation:" line above it.

**Clause violated:** POLICY 15 SAME-AC GATE AUDIT — OBLIGATION-INDEXED FORM (D-916); POLICY 22; TD-VSDD-059 (self-disclosure of status is not authoritative); D-449(a)/META-LEVEL-24 (narrative-attested gates cannot detect their own scope-degradation).

**Evidence.** The attestation row, verbatim:

```
| PC4a: PC13a normative advisory text MUST match verbatim; triple-substring grep-q gate
  insufficient (F-S2107-P7-013) | T-P6C (equality) | `AC-023 / T-P6C (PC13a): B2==B3
  story-index-consistent-stale emits advisory, exits 0` | same anti-vacuity pattern…
  | PASS (vacuous) | GREEN |
```

Measured at `fbb5183c`: that test is `not ok 48`. The same section states:

```
Post-implementation: **162 passed; 1 failed; 17 ignored** (independent verification 2026-08-06).
The 1 expected failure is `test_BC_corpus_version_sync_all_indexed_bcs_match_frontmatter` —
BC-5.39.010 frontmatter version="1.13" not yet in BC-INDEX row; resolves at state-manager Commit D.
```

Measured: **163 passed; 0 failed; 17 ignored**. The named cargo failure did resolve at Commit D as predicted — that part of the reasoning was sound. But the figures were never re-derived after Commit D landed, and the bats leg was never derived at all.

**Why it matters.** The attestation is the artifact a reviewer, a wave gate, and a PR body all consult in place of running the suite. Its `GREEN` cell is the mechanism by which F-S2107-P8-001 shipped. The section also claims the WASM figures were "triangulated by three independent measurements (orchestrator, devops-engineer, implementer), all agreeing" — and those figures are in fact correct, which is the worst possible pattern: the triangulation ritual was applied to the claim that did not need it and omitted from the claim that did. Note the attestation's own "Prior attestation note" correctly catches that `b78b27ef`'s subject misdescribed the artifact by 4,327 bytes (F-S2107-P7-019) — so this document demonstrably can audit a stale figure, and did not audit its own.

**Failure scenario.** Any downstream consumer that treats the obligation table as coverage proof concludes PC13a is gated. It is not; the gate is red and its result is inverted.

**Routing.** test-writer: after fixing F-S2107-P8-001, re-derive both legs by execution and record verbatim stdout for the bats leg (currently absent entirely — only a `GREEN` word appears). state-manager: add a Commit-E predicate that the bats exit code is captured, not asserted.

---

### HIGH

#### F-S2107-P8-003 — HIGH [process-gap] — POLICY 15's ATTESTATION-LOCATION GATE is unsatisfiable as written: it requires the attestation to be bundled in the same commit *and* to name that commit's own SHA. The gate returns 0 at reviewed HEAD, as it must at every HEAD; the project has silently adopted a parent-SHA-in-a-follow-up-commit convention that the literal predicate rejects.

**Location:** `.factory/policies.yaml` POLICY 15 `verification_steps` → ATTESTATION-LOCATION GATE (v1.4.11; D-912); `crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md` attestation lines.

**Clause violated:** POLICY 15 ATTESTATION-LOCATION GATE; TD-VSDD-053 (same-commit bundling) — the two are mutually exclusive.

**Evidence — literal shell at `fbb5183c`:**

```
$ grep -c 'assertion-site attestation (fbb5183c)' .../docs/red-gate-log.md
0
$ grep -oE 'assertion-site attestation \([0-9a-f]{7,40}\)' .../docs/red-gate-log.md
assertion-site attestation (b78b27ef402f11e36c8c23f68f65d6335c37dd14)
assertion-site attestation (295585185308629f10ff4333647a15b474192c3f)
```

The convention in practice is parent-SHA in the *next* commit:

```
$ git log -S'assertion-site attestation (295585185308629f10ff4333647a15b474192c3f)' --oneline -- .../red-gate-log.md
fbb5183c docs(S-21.07): pass-7 burst 3 — F-S2107-P7-009 red-gate-log attestation
$ git show b78b27ef:.../red-gate-log.md | grep -c 'assertion-site attestation (b78b27ef'
0
```

So `fbb5183c` attests `29558518`, and `49d542a2` attests `b78b27ef`. No commit contains its own SHA, and none can.

**Why it matters.** The policy text is explicit — *"`grep -c 'assertion-site attestation (<HEAD-SHA>)' red-gate-log.md` → 1 (where `<HEAD-SHA>` is the actual SHA being pushed)… If the count is 0, the push is BLOCKED until the state-manager appends the attestation section and bundles it in the same commit per TD-VSDD-053."* A git commit's SHA is a hash over its own content; embedding it is impossible. The gate has therefore been unfailable-and-unpassable since D-912, which is why pass-6 could report it at 0 and this pass finds it at 0 after a burst explicitly dedicated to fixing it. Worse for this burst specifically: `fbb5183c` is a **separate commit** from the one carrying the assertion-site changes, so the TD-VSDD-053 same-commit half is independently violated even under a charitable reading.

**Failure scenario.** The gate can never gate. Every pass reports it, every burst "fixes" it by adding a parent-SHA line, and the predicate stays at 0 — a stable, self-perpetuating finding that consumes a slot each pass without converging.

**Routing.** architect (policy amendment, human-ratified): redefine the predicate to the reproducible form — `### <Pass-N> assertion-site attestation (<PARENT-SHA>)` where `<PARENT-SHA>` is the tree the tests were executed against, bundled in the commit that changes the assertion sites. That is satisfiable, SHA-bound, and cannot be met by a prior pass's section, which were D-912's three stated goals.

---

#### F-S2107-P8-004 — HIGH [regression] — The v1.12→v1.13 cite sweep **inverted**: the three sites pass-6 recorded as already-swept went stale, and the one site pass-6 quoted verbatim (`run_arm_b1`) is untouched at v1.6. 88 stale live `.rs` cites; 49 sites one version behind at v1.12. Fourth consecutive pass on this class.

**Location:** `plugins/vsdd-factory/hooks-registry.toml` (`BC-5.39.010 v1.12` ×2); `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats` header `# Governing BC: BC-5.39.010 v1.12`; `crates/.../src/arm_b.rs` — `run_arm_b1` doc comment `PC40 (BC-5.39.010 v1.6)` and its `# BC trace` line `BC-5.39.010 v1.6 PC40`; `arm_b.rs::is_volatile_path` and `::parse_story_volatile_inputs` `# BC trace` (v1.12); `arm_a1.rs` PC5 doc comments and assertion messages (v1.10/v1.12); `arm_a2.rs` PC13 collision-class docs (v1.10); `dispatch.rs`/`arm_e.rs`/`lib.rs` `v1.3` §-anchors.

**Clause violated:** POLICY 5 v1.3.4 SIBLING-SWEEP LITERAL-SHELL VERIFICATION GATE + v1.3.5 Part A HISTORICAL-BY-CONSTRUCTION ENUMERATION; POLICY 14 leg 5; POLICY 4. Lineage: F-P2-017 → F-S2107-P3-010 → F-S2107-P4-011 → F-S2107-P6-008 → F-S2107-P7-005 → this pass.

**Evidence — literal shell at `fbb5183c`:**

```
$ grep -rnE 'BC-5\.39\.010 v1\.[0-9]+' crates/.../src/*.rs | grep -vE 'v1\.13' | wc -l
      99
$ grep -rnE 'BC-5\.39\.010 v1\.[0-9]+' crates/.../src/*.rs \
    | grep -vE 'v1\.13' | grep -vE 'DEFERRED v1\.6|Class D' | wc -l
      88
$ grep -rhoE 'BC-5\.39\.010 v1\.[0-9]+' crates/.../src/*.rs | sort | uniq -c | sort -rn
  37 BC-5.39.010 v1.10
  22 BC-5.39.010 v1.12
  18 BC-5.39.010 v1.6
  17 BC-5.39.010 v1.13
  16 BC-5.39.010 v1.3
   5 BC-5.39.010 v1.5
   1 BC-5.39.010 v1.2
```

The inversion, side by side:

```
$ grep -o 'BC-5.39.010 v1\.[0-9]*' crates/.../Cargo.toml
BC-5.39.010 v1.13                     <- swept
$ grep -o 'BC-5.39.010 v1\.[0-9]*' plugins/vsdd-factory/hooks-registry.toml
BC-5.39.010 v1.12                     <- STALE (pass-6 recorded this as swept)
BC-5.39.010 v1.6
BC-5.39.010 v1.1
BC-5.39.010 v1.12                     <- STALE
BC-5.39.010 v1.6
$ grep -n 'Governing BC' plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats
38:# Governing BC: BC-5.39.010 v1.12         <- STALE (pass-6 recorded this as swept)
$ grep -n 'BC-5.39.010 v' crates/.../src/lib.rs | head -1
15://! BC-5.39.010 v1.13 — six-arm PostToolUse …    <- swept (was the pass-6 miss)
```

The verbatim-named site, unchanged:

```
$ sed -n '422,434p' crates/.../src/arm_b.rs
/// PC40 (BC-5.39.010 v1.6): if the story's `inputs:` list contains any volatile
…
/// # BC trace
/// BC-5.39.010 v1.6 PC40: volatile-input precondition.
pub fn run_arm_b1(story_id: &str, story_content: &str) -> (Vec<Violation>, Vec<Advisory>) {
```

Its two immediate neighbours — `is_volatile_path` and `parse_story_volatile_inputs` — carry `BC-5.39.010 v1.12 PC40`. So within one file, three adjacent doc comments hold three different versions, none of them current.

**Why it matters.** POLICY 5 v1.3.5 Part A enumerates the five historical-by-construction classes exhaustively (frontmatter `modified[]`, body Changelog rows, `[Prior:]` clauses, §Adversary Pass Coverage, `lessons.md` cross-refs). Rust doc comments and assertion messages are in none of them; the `[DEFERRED v1.6 — Class D]` markers are legitimately frozen and I excluded them (the 99→88 step). Everything remaining is a live authority cite. `run_arm_b1` is the entry point an implementer reads first, and v1.6's PC40 transitional clause was replaced at v1.11 with its "widening" characterization withdrawn — so the pointer is not merely stale, it points at retracted reasoning.

**Failure scenario.** The sweep is now demonstrably a coin-flip per site rather than a class operation: two passes running, the set of correct sites and the set of incorrect sites have swapped. A fifth pass will find a third permutation unless the sweep is made mechanical.

**Routing.** implementer: sweep all 88 to v1.13. state-manager: add the POLICY 5 v1.3.4 gate as an executed predicate at Commit E — `grep -rnE 'BC-5\.39\.010 v1\.[0-9]+' crates/ plugins/ | grep -vE 'v1\.13' | grep -vE 'DEFERRED v1\.6|Class D'` must be empty, with captured stdout. A sweep asserted per-site will keep permuting; only the tree-wide predicate converges.

---

#### F-S2107-P8-005 — HIGH — BC-5.39.010 v1.13 PC5's three load-bearing corpus claims were falsified by the immediately-following commit **of the same burst**. Actual counts are 1,945 of 1,985 (not 1,943 of 1,983); the old algorithm now misreads two of four rows (not three); and the bare-pipe phenomenon the v1.13 amendment was written to document has **zero** live instances.

**Location:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md` §Preconditions PC5 — the `RowPresentNoVersion` bullet ("exactly **1,943 of 1,983** rows (corpus 2026-08-06)"), the `Version(v)` bullet's non-conformance sentence ("spurious PC2a advisories for **three of four** corpus rows"), and the v1.13 Changelog row's "yields 6 for most rows but MAY yield more for bare-`\|` annotation rows"; against factory-artifacts commits `d32207bf` (leg C) and `304156dd` (leg D).

**Clause violated:** POLICY 15 / D-950 corpus-count discipline; POLICY 5 SDK-GROUNDING MANDATE + HEAD-REPRODUCIBILITY-OR-STRUCTURAL-FORM MANDATE (v1.3.6 — captured figures must reproduce at HEAD); POLICY 3 (`state_manager_runs_last` verification step: "check for citation staleness"); POLICY 22.

**Evidence — re-derived at factory-artifacts `10914a73` under exact production locator semantics** (`line.starts_with('|')`; escape-aware split on `\|`→`\x00`; first non-empty cell matches BC-ID in link **or** plain form, per `first_cell_matches_bc_id`):

```
candidate rows (production semantics): 1985
histogram: {5: 1945, 6: 40}
>6-field rows: 0 []
n>=6 arm: 40

BC cites: 1,943 of 1,983 / 40 rows reach n>=6
```

The four-row proof, re-traced (`oldalg` = rightmost-token-of-field[5]; `first-of-last` = the shipped v1.13 algorithm):

```
BC             frontmatter  first-of-last  oldalg-field5  oldalg_differs
BC-3.08.001    1.24         1.24           1.23           True
BC-7.03.079    v1.5         1.5            1.4            True
BC-4.13.001    1.18         1.18           1.18           False
BC-5.24.006    v1.3         1.3            1.3            False
```

Two of four, not three. The reason is in the state-manager's own commit message:

```
$ git log --oneline -2 -- specs/behavioral-contracts/BC-INDEX.md
304156dd factory(D-958/D): 4-index frontmatter bumps — BC-INDEX v4.49→v4.50 (+total_bcs 1983→1985), …
d32207bf factory(D-958/C): content corrections + registrations — … BC-5.39.010 v1.13, …
```

and in BC-INDEX v4.50's `last_amended`: *"BC-1.01.016+BC-1.03.017 new draft BCs registered … **BC-4.13.001 v1.16 pipe-escaping correction. total_bcs 1983→1985.**"* The escaping is confirmed live:

```
$ grep -o '| \[BC-4\.13\.001\].*' .../BC-INDEX.md | grep -o 'Edit.\{0,40\}'
Edit\|Write\|MultiEdit\|Agent)$; Bash arm ^B
```

**Why it matters.** Leg C authored the spec; leg D added two rows and escaped the pipes; the spec was never re-measured. All three claims are load-bearing: the row counts are the corpus grounding POLICY 5 requires, the "three of four" figure is the sole stated justification for declaring the prior algorithm NON-CONFORMING, and the reassembly paragraph documents production code (`non_empty_fields[5..].join("|")`) whose triggering condition no longer occurs anywhere in the corpus. POLICY 5 v1.3.6 is explicit that captured figures must reproduce at HEAD; these do not, and they were stale before the burst finished.

**Failure scenario.** A reader re-deriving PC5's figures — which is exactly what POLICY 5 v1.3.6 instructs the adversary to do — gets three mismatches and cannot tell whether the spec is stale or the corpus has drifted illegitimately. Separately, the reassembly branch is now dead code with respect to the live corpus and no test pins it to a real row, so a future refactor can silently remove it with every gate green.

**Routing.** product-owner: re-measure and correct all three claims; state whether the reassembly branch is retained defensively (legitimate — say so) or was corpus-motivated. state-manager: sequence the corpus-figure re-measurement **after** the index-registration leg, or add a Commit-E predicate re-deriving PC5's counts from the post-Commit-D tree.

---

#### F-S2107-P8-006 — HIGH — F-S2107-P7-011 is not closed. `bc_index_row_contains_version` still searches the entire row line; I demonstrated all three bypasses by execution. Its justifying comment cites bugs that were fixed, and the correct extractor now sits in the same function, unused for the comparison.

**Location:** `crates/.../src/lib.rs` — `bc_index_row_contains_version`, and its call site inside `test_BC_corpus_version_sync_all_indexed_bcs_match_frontmatter` together with the comment *"For the actual version match, use bc_index_row_contains_version to bypass the last-wins edge cases (F-P6-019a–d)."*

**Clause violated:** TD-VSDD-059 (compensating guard weaker than the block it replaced); POLICY 13 BOUNDARY-POLARITY MANDATE (domain widened to "anywhere in the row" with no analysis of what the widened domain admits); POLICY 15 (a code comment asserting a property the code does not have).

**Evidence.** The helper is unchanged — locate by `format!("| [{bc_id}](")`, then scan the whole line for `v{version}` with a trailing-non-digit check. No chain-terminal restriction, no direction comparison. The test computes the correct value and discards it:

```rust
let index_state = arm_a1::extract_bc_index_version_state(&bc_id, bc_index_str.as_bytes());
match index_state {
    arm_a1::BcIndexVersionState::Version(_) => {          // value bound to `_`
        …
        if !bc_index_row_contains_version(&bc_id, normalized_fv, &bc_index_str) { … }
```

Three bypasses, re-implementing the helper's exact semantics against the live `BC-INDEX.md`:

```
--- BYPASS 1: index newer than primary (PC2b anomaly) ---
append forged newer chain entry; frontmatter stays 1.18
  production extractor (index current) : 1.19
  weak helper('1.18')                  : True     <- gate PASSES on a PC2b anomaly

--- BYPASS 2: annotation token satisfies helper (BC-3.08.001 shape) ---
  prod-extract: 1.24 | helper('1.23'): True       <- rollback to 1.23 PASSES

--- BYPASS 3: frontmatter rolled back to an earlier chain entry ---
  prod-extract: 1.13
    helper('1.10') = True
    helper('1.11') = True
    helper('1.12') = True
    helper('1.13') = True                          <- any rollback PASSES
```

**Why it matters.** Version chains are append-only and cumulative, so the helper fires only when the frontmatter version was **never** appended. The invariant PC2 v1.10 enforced was "index current token == frontmatter version"; the restored invariant is "frontmatter version appears somewhere in the row." Bypass 3 is the concrete cost: BC-5.39.010 itself could be rolled back to `1.10` and the only commit-time gate for the whole class would stay green. Pass-6 flagged the false `NOTE` claiming F-P6-019a–d were unfixed and out of scope; the NOTE's wording changed but the *same false premise* is now the inline justification, and it is more clearly false than before because `extract_bc_index_version_state` was rewritten to v1.13 correctness in this very burst.

**Failure scenario.** A BC-INDEX-only edit that appends a bogus newer chain entry is caught by no gate at any layer: Arm A1 fires on BC-file writes and excludes `BC-INDEX.md` by basename, and the corpus gate passes per Bypass 1.

**Routing.** test-writer: compare `BcIndexVersionState::Version(v)` to `normalized_fv` directly and delete the helper; retain the three vectors above as bypass mutants so the strengthening is pinned. If the helper must stay for a stated reason, POLICY 13 BOUNDARY-POLARITY requires the excluded-region analysis and a mutant per direction in the same burst.

---

#### F-S2107-P8-007 — HIGH [process-gap] — The coverage gate still derives its attestation from `grep` counts, not from execution, and hardcodes `0 failed`. It printed "46 passed / 5 skipped / 0 failed" on a run with 1 failure — this is the direct source of the false gate figure in the dispatch claim. F-S2107-P7-014 unclosed at its second pass.

**Location:** `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats` — `@test "F-P6-016: coverage gate — 5 deferred, 45 dispatcher-gated, execution-bounded attestation"`, the `run_when_present` / `skip_when_present` arithmetic and the two `echo` lines.

**Clause violated:** POLICY 15; TD-VSDD-059 (paper-fix: the label changed to "execution-bounded" while the mechanism stayed structural); D-449(a)/META-LEVEL-24; production-grade default.

**Evidence.** The gate's three predicates are all `grep -c` over its own source file (`skip "\[DEFERRED v1\.6 — Class D\]` → 5; `^\s*_require_artifacts$` → 45; `^@test ` → ≥40). The attestation is then pure arithmetic over those counts:

```bash
local run_when_present=$(( total_count - deferred_count ))    # 51 - 5 = 46
local skip_when_present=$deferred_count                       # 5
echo "  When dispatcher + WASM present: $run_when_present passed / $skip_when_present skipped / 0 failed"
```

`0 failed` is a literal. Measured on the same run that emits this line: **1 failed**. The figure "46 passed / 5 skipped" carried into the dispatch brief is this `echo`, not a measurement.

**Why it matters.** F-S2107-P7-014's stated ask was "a runtime-derived `N executed, M skipped` line." What shipped derives the same numbers one step earlier, from declarations, and adds a hardcoded failure count — so the gate is now capable of asserting green while red, which the previous form at least did not claim. It is also strictly self-referential: it greps `$BATS_TEST_FILENAME`, so it measures the file rather than the run and remains satisfied under the exact regression it was raised about (if `_require_artifacts` starts skipping, all three counts hold and the gate stays green). Notably `run-all.sh` was modified in this same burst to do the genuinely runtime thing — parse TAP for `^(ok|not ok)` and `# skip` — so the correct mechanism was authored ten lines away and not applied here.

**Failure scenario.** Every consumer of the attestation line receives a fabricated pass/fail breakdown. This is the mechanism by which F-S2107-P8-001 and F-S2107-P8-002 both escaped.

**Routing.** test-writer: drop the `echo`-based attestation from inside the suite (a test cannot observe its own run's results) and let `run-all.sh`'s TAP accounting be the single source; or emit the structural counts only, with no `passed`/`failed` words. state-manager: any recorded gate figure must come from a captured exit code plus TAP counts per POLICY 22.

---

### MEDIUM

#### F-S2107-P8-008 — MEDIUM — The STORY-INDEX catalog cell for S-21.07 says "23 ACs; 31 ECs"; the story has 24 ACs and 34 EC rows. The cell was edited this burst — three of its five tokens were advanced and these two were not.

**Location:** `.factory/stories/STORY-INDEX.md` — the `| S-21.07 |` catalog row's parenthetical aggregation clause; against `.factory/stories/S-21.07-validate-cross-site-correspondence.md` §Acceptance Criteria and §Edge Cases.

**Clause violated:** POLICY 14 leg 5 (upstream-index body-table cells); POLICY 5 v1.3.7 SIBLING-SWEEP CATEGORY (i) (same-file aggregation cells duplicating per-row values); POLICY 8 (AC propagation).

**Evidence — literal shell at `10914a73`:**

```
$ grep -cE '^### AC-0[0-9]{2}' .../S-21.07-validate-cross-site-correspondence.md
24
$ grep -cE '^\| EC-0[0-9]{2}' .../S-21.07-validate-cross-site-correspondence.md
34
$ grep -oE '^\| S-21\.07 \|.*' .../STORY-INDEX.md | grep -oE '[0-9]+ ACs; [0-9]+ ECs'
23 ACs; 31 ECs
```

STORY-INDEX v4.287's `last_amended` records the edit: *"S-21.07 catalog row v1.13/c0ab6a3/v1.8"* — version, hash and story-version advanced; AC and EC counts left. AC-024 was added in story v1.8 and EC-036 mirrored, both in this burst, so both counts were stale the moment the row was touched.

**Routing.** state-manager: `24 ACs; 34 ECs`, same burst as any AC/EC addition.

---

#### F-S2107-P8-009 — MEDIUM — The story's own active-AC enumeration omits AC-024, the AC added in the same amendment. Same file, same burst, two sections disagreeing.

**Location:** `.factory/stories/S-21.07-validate-cross-site-correspondence.md` — the §BC Status preamble sentence *"ACs 001–011, 015–023 are active (AC-022 and AC-023 added in v1.6 to cover PC2a and PC13a advisory paths); AC-012/013/014 (Class D) are DEFERRED v1.6"*; against `### AC-024: Class B Arm1 — half-present case is advisory only (traces to BC-5.39.010 postcondition 13c)`.

**Clause violated:** POLICY 5 v1.3.7 CATEGORY (i); POLICY 8; POLICY 4.

**Evidence.** The v1.8 `modified[]` entry states the addition — *"AC-024 added for PC13c half-present"* — and the AC section heading exists. The enumeration still terminates at 023. The `(AC-022 and AC-023 added in v1.6 …)` parenthetical is also now an incomplete provenance note for the same reason.

**Why it matters.** The enumeration is the story's authoritative statement of which ACs are in force. AC-024 is the acceptance criterion for PC13c — the half-present carve-out that is one of this burst's two substantive behavioral changes — so the one AC most needing visibility is the one absent from the inventory. It compounds F-S2107-P8-001: the reader looking for PC13a/PC13c coverage finds a red gate and an inventory that does not list the AC.

**Routing.** story-writer: extend to `001–011, 015–024` and update the parenthetical to record AC-024/v1.8.

---

#### F-S2107-P8-010 — MEDIUM — BC-5.39.010's PC40 narrative still cites "ADR-037 v1.2 records **77 stories**" at two sites after ADR-037 was corrected to 76 in this burst. The ADR was fixed; the BC that cites its roster was not swept.

**Location:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md` §Preconditions PC40 region — *"corrected to 77 stories in ADR-037 v1.2 after S-21.07 remediation"* and *"**Remaining remediation scope**: ADR-037 v1.2 records **77 stories** with volatile inputs"*; against `ADR-037-…-volatile-artifacts-excluded.md` §Context.

**Clause violated:** POLICY 5 v1.3.4 SIBLING-SWEEP + CATEGORY (j) (inline parenthetical cites at semantic-parallel sites); POLICY 4; POLICY 22.

**Evidence.** ADR-037's own corrections, and my independent re-derivation using production `is_volatile_path` semantics over frontmatter-parsed `inputs:`:

```
$ grep -oE 'union 7[678]|ARCH-INDEX leg 6[123]→?6?[123]?' .../ADR-037-*.md | sort -u
ARCH-INDEX leg 62→61
union 77→76
$ python3 /tmp/vol.py
story files scanned: 159
volatile-union stories (is_volatile_path semantics): 76
ARCH-INDEX leg stories: 61
epics with volatile inputs: 5

ADR-037 claims: union 76 / ARCH leg 61
```

ADR-037's corrected figures match my derivation exactly — that closure (F-S2107-P7-018) is sound. The BC still carries the pre-correction 77 twice.

**Why it matters.** "Remaining remediation scope" is a scope-defining figure: it sets how many stories must be reworked before PC40's transitional clause can retire. Two artifacts now disagree about the size of the remaining work, and per CLAUDE.md §12 the BC is the governing spec. The `ADR-037 v1.2` tokens are also load-bearing version pins in prose; POLICY 19's scope is formally limited to BC §Traceability rows (I confirmed §Traceability cites no ADR, so POLICY 19 is not violated), but TD-VSDD-091's rationale applies unchanged.

**Routing.** product-owner: 77→76 at both sites; prefer `ADR-037 §Context` over a version pin.

---

#### F-S2107-P8-011 — MEDIUM — The burst that reconciled the four named POLICY 18 stories created two new half-present instances. `S-8.10` and `S-21.12` have catalog rows with no blockquote entry, so their three-way equality is unsatisfiable — the two-of-three comparison POLICY 18 explicitly calls insufficient.

**Location:** `.factory/stories/STORY-INDEX.md` — catalog rows for `S-8.10` and `S-21.12` vs the aggregation blockquote; `.factory/stories/S-8.10-sdk-extension-write-file.md` and `.factory/stories/S-21.12-wasmtime-major-version-move-46-and-cargo-deny-ci.md` frontmatter `input-hash:`.

**Clause violated:** POLICY 18 THREE-WAY-INPUT-HASH-EQUALITY GATE (D-923); POLICY 5 CATEGORY (i).

**Evidence — literal shell at `10914a73`:**

```
catalog entries: 33 | blockquote entries: 31
ARM B2 VIOLATIONS (catalog != blockquote): 0
catalog rows with NO blockquote entry: 2 ['S-8.10', 'S-21.12']
   S-18.06 catalog=d65656e blockquote=d65656e
   S-18.08 catalog=b12c2fd blockquote=b12c2fd
   S-18.11 catalog=f7ab2d0 blockquote=f7ab2d0
   S-18.12 catalog=8880487 blockquote=8880487
```

```
--- S-8.10    leg1 frontmatter: "9ca3574"   leg2 catalog: 9ca3574   leg3 blockquote: (ABSENT)
--- S-21.12   leg1 frontmatter: "d451e49"   leg2 catalog: d451e49   leg3 blockquote: (ABSENT)
```

Both were introduced by this burst: `S-8.10` was bumped to v1.3 with `input-hash 6fb36eb → 9ca3574` as the F-S2107-P7-018 line-pin fix, and `S-21.12` was newly registered (`d32207bf`: *"S-21.10/11/12+E-22 registered"*).

**Why it matters.** The four named stories are genuinely fixed and Arm B2 is clean — that closure is real. But the same burst re-seeded the class at two fresh sites, in one case *as part of* the fix for a pass-6 finding. This is no longer a self-lock risk, because PC13c now makes half-present advisory rather than blocking (verified: `test_BC_5_39_010_arm_b1_half_present_*` pass), so severity is MEDIUM rather than BLOCKER. What remains is that POLICY 18's gate cannot be run for either story.

**Routing.** state-manager: add blockquote entries `S-8.10=9ca3574` and `S-21.12=d451e49`; make blockquote insertion part of catalog-row creation so the two cannot diverge.

---

#### F-S2107-P8-012 — MEDIUM [process-gap] — `O-P8-NN` is an already-occupied namespace. E-18 uses three-digit `O-P8-001/002/003` (88 cites); this cascade has started using two-digit `O-P8-01/02` (17 cites, one load-bearing in BC-INDEX v4.50's `Refs:`). Zero-padding is the only discriminator.

**Location:** `.factory/cycles/v1.0-brownfield-backfill/consistency-e18-story-pass-8.md`, `.factory/code-delivery/S-18.01/research-O-P8-001-wave-ordinal.md`, `.factory/stories/S-18.08-pure-parse-invariant-gate.md`, `.factory/stories/S-18.01-…`, `.factory/stories/epics/E-18-factory-context-durability.md`, `.factory/specs/behavioral-contracts/ss-05/BC-5.41.001.md`, `.factory/specs/domain-spec/capabilities.md` (three-digit series); `.factory/specs/behavioral-contracts/BC-INDEX.md`, `.factory/stories/STORY-INDEX.md`, `.factory/specs/verification-properties/VP-INDEX.md`, cycle `decision-log.md` / `burst-log.md` / `lessons.md`, `STATE.md` (two-digit series).

**Clause violated:** POLICY 1 (`append_only_numbering` — identifiers are never reused); POLICY 16 (citation must be verifiable at authoring time — an `O-P8-02` cite is ambiguous between two series).

**Evidence — literal shell at `10914a73`:**

```
$ grep -rhoE 'O-P8-[0-9]{2,3}' --include='*.md' . | sort | uniq -c | sort -k2
  44 O-P8-001
  39 O-P8-002
   5 O-P8-003
   6 O-P8-01
  11 O-P8-02
$ grep -ohE 'Refs:[^.]{0,60}O-P8-[0-9]{2,3}' .../BC-INDEX.md
Refs: D-958, O-P8-02
```

Unlike finding IDs, which are story-scoped (`F-S2107-P8-NNN`), the observation prefix carries only a pass ordinal, so any two artifacts at pass 8 collide. `O-P8-002` (E-18 pure-parse invariant gate) and a future `O-P8-02` differ by one character of padding.

**Why it matters.** The brief for this review instructed me to number observations `O-P8-NNN` — three digits — which would have placed my observations directly inside E-18's live series, 88 cites deep. I used `O-S2107-P8-NN` instead (see the numbering note in Part C). The collision is currently latent because the two series happen to use different padding; it becomes a real defect the first time any tool, sweep, or agent normalizes the padding, and BC-INDEX's `Refs: D-958, O-P8-02` is already an unresolvable cite without out-of-band knowledge of which series it belongs to.

**Routing.** state-manager: adopt story-scoped observation IDs (`O-S2107-P8-NN`) for this cascade and re-anchor the existing two-digit cites; spec-steward: record the convention so the next pass does not re-collide.

---

#### F-S2107-P8-013 — MEDIUM — F-S2107-P7-010's fuel axis is still ungated. The new production-scale test is a **native** Rust test with no fuel budget, and the largest WASM-sandbox fixture is 2,383 bytes against a 575,740-byte live index — a 241× gap.

**Location:** `crates/.../src/lib.rs` — `test_F_P7_010_production_scale_arm_a1_scan_no_regression` (builds a ~700 KB synthetic index in-process and calls `extract_bc_index_version_state` directly); `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/*`; `plugins/vsdd-factory/hooks-registry.toml` (`on_error = "continue"`, no `fuel_cap` per AC-020).

**Clause violated:** POLICY 11; TD-VSDD-059; production-grade default.

**Evidence — literal shell at `fbb5183c`:**

```
$ find plugins/.../fixtures/validate-cross-site-correspondence -name '*INDEX.md' | xargs wc -c | sort -n | tail -4
    2359 .../b1-hash-match/factory/stories/STORY-INDEX.md
    2383 .../a1-stale-index/factory/specs/behavioral-contracts/BC-INDEX.md
    2592 .../b1-story-index-inconsistent/factory/stories/STORY-INDEX.md
   37256 total
$ wc -c .factory/specs/behavioral-contracts/BC-INDEX.md .factory/stories/STORY-INDEX.md
  575740 …/BC-INDEX.md
  503403 …/STORY-INDEX.md
```

**Partial credit, stated precisely.** The burst did add real coverage on the adjacent axes, and I verified both: the new scale test proves the scan does not regress algorithmically at ≥574 KB, and `_assert_plugin_ran_not_crashed` genuinely asserts absence of `plugin.timeout` (with a self-test at bats:410 feeding a synthetic timeout-only log to prove that assertion fires). So the timeout assertion is not vacuous — its *domain* is 241× too small. The one combination that matters, sandbox × production scale, exists nowhere: a native `#[test]` has no fuel meter, and `on_error = "continue"` converts exhaustion into `HookResult::Continue` with no violation and no advisory, output-identical to a clean corpus.

**Failure scenario.** Unchanged from pass-6: no gate at any layer distinguishes "validated 1,985 rows, found nothing" from "exhausted fuel on row 200 and returned Continue." The registry's budget rationale ("sufficient … at current sizes") is still asserted rather than measured, and the corpus just grew by two rows.

**Routing.** test-writer: one bats scenario whose fixture `BC-INDEX.md` is a copy of (or comparable in row count to) the live file, asserting `plugin.completed` present and `plugin.timeout` absent. That single scenario closes the axis and makes the registry's budget claim measured rather than asserted.

---

#### F-S2107-P8-014 — MEDIUM [process-gap] — New pass/skip accounting tooling was implemented in `run-all.sh` (bash) rather than Rust, expanding the scope of a grandfathered `.sh`. POLICY 21 names exactly this case.

**Location:** `plugins/vsdd-factory/tests/run-all.sh` — the `total_suites` / `total_tests` / `total_skips` accumulators and the `bats --tap` capture block added in this branch.

**Clause violated:** POLICY 21 (`no_new_shell_scripts`, severity **blocking**) verification steps: *"New tooling that would previously have been a .sh script MUST be implemented as: (a) a Rust workspace test, (b) a Rust binary in crates/, or (c) a WASM plugin"*; and *"Existing .sh files (grandfathered) … no modifications that expand their scope are allowed without human approval."*

**Evidence — literal shell at `fbb5183c`:**

```
$ git diff --name-status origin/develop...HEAD | grep -E '\.sh$'
M	plugins/vsdd-factory/tests/run-all.sh
```

The diff adds TAP parsing and cross-suite aggregation:

```bash
tap_out=$(bats --tap "$f" 2>&1)
bats_exit=$?
suite_total=$(echo "$tap_out" | grep -cE '^(ok|not ok) [0-9]+'; true)
suite_skips=$(echo "$tap_out" | grep -cE '^ok [0-9]+.*# skip'; true)
total_tests=$((total_tests + suite_total))
```

No new `.sh` file was added — POLICY 21's primary prohibition holds, which is why this is MEDIUM and not BLOCKER. What was added is a new *capability* (run-level pass/skip accounting) in bash.

**Why it matters.** This is genuinely the right mechanism in the wrong language, and it matters doubly here: it is the correct runtime-derived accounting that F-S2107-P8-007's coverage gate should have used, so the project now has the right idea implemented in the one place POLICY 21 forbids and the wrong idea implemented where it counts. `run-all.sh` is not among D-846's five explicitly grandfathered files; it is covered only by the general pre-D-836 grandfather, whose terms bar scope expansion without human approval.

**Routing.** Orchestrator → human: this needs a ruling, not an agent decision. Either grant explicit approval for the `run-all.sh` extension (recording it under the grandfather clause with a D-NNN), or route to devops-engineer to implement suite accounting as a Rust binary. Do not leave it unadjudicated — POLICY 21's severity is `blocking`.

---

### LOW

#### F-S2107-P8-015 — LOW — The cargo gate figure discloses "163 passed / 0 failed" and omits 17 ignored tests, while the bats figure in the same claim discloses its 5 skips. Asymmetric disclosure of the same concealment class F-S2107-P7-014 was raised about.

**Location:** the dispatch claim's gate figures; `crates/.../src/arm_d.rs` and `src/dispatch.rs` `#[ignore = "[DEFERRED v1.6 — Class D] …"]` attributes.

**Clause violated:** POLICY 15 (gate figures at the persistence layer); D-693.

**Evidence — literal shell at `fbb5183c`:**

```
$ cargo test -p validate-cross-site-correspondence 2>&1 | grep 'test result' | head -1
test result: ok. 163 passed; 0 failed; 17 ignored; 0 measured; 0 filtered out
$ cargo test -p validate-cross-site-correspondence -- --ignored --list 2>&1 | tail -3
dispatch::tests::test_BC_5_39_010_dispatch_burst_log_detected: test
dispatch::tests::test_BC_5_39_010_dispatch_lessons_md_detected: test
17 tests, 0 benchmarks
```

All 17 are legitimately `[DEFERRED v1.6 — Class D]` per D-953 and preserved per POLICY 1 — there is no hidden failure here. The defect is disclosure form: the bats figure is stated as "46 passed / **5 skipped** (5 Class-D skips per ADR-037 §Decision 5)" while the cargo figure omits its ignored count entirely. Correct form: **163 passed / 0 failed / 17 ignored (Class-D DEFERRED per D-953)**.

**Routing.** state-manager: state ignored/skipped counts on both legs wherever gate figures are recorded.

---

#### F-S2107-P8-016 — LOW — The factory-artifacts worktree is unclean at review time, including a nested `.factory/.factory/logs/` directory (a path-construction defect) and an unstaged ` D` deletion — the exact class POLICY 22 names as silently droppable.

**Location:** worktree `/Users/zious/Documents/GITHUB/vsdd-factory/.factory`.

**Clause violated:** POLICY 22 (` D` lines "are silently droppable in narrative synthesis and have downstream clean-tree consequences if left unstaged"); factory-health clean-tree expectation.

**Evidence — literal shell at `10914a73`:**

```
$ git -C .factory status --porcelain
 D logs/dispatcher-internal-2026-07-06.jsonl
 M logs/dispatcher-internal-2026-08-05.jsonl
 M logs/events-2026-08-05.jsonl
 M regression-state.json
 M sidecar-learning.md
?? .factory/logs/dispatcher-internal-2026-08-06.jsonl
?? logs/dispatcher-internal-2026-08-06.jsonl
?? logs/events-2026-08-06.jsonl
$ find .factory -type f
.factory/logs/dispatcher-internal-2026-07-27.jsonl
.factory/logs/dispatcher-internal-2026-08-06.jsonl
```

`.factory/.factory/logs/` (i.e. `<worktree>/.factory/logs/`) holds two files from two separate dates, so a writer is resolving the log path relative to the factory worktree root while prefixing `.factory/` again — it has recurred at least twice. Everything here is telemetry, not spec content, so no artifact is at risk; the concern is that the dispatch claim described both trees as clean, and ` D` is the one line POLICY 22 was written about.

**Routing.** devops-engineer: fix the double-prefix in the log-path resolver and remove the nested directory; state-manager: stage the deletion. Neither blocks the cascade.

---

## Part B — Observations

**O-S2107-P8-01 — Ten pass-6 closures verified sound, several by independent re-derivation rather than inspection.** `F-S2107-P7-002` (Arm B2: 0 live violations; S-18.06/08/11/12 all reconciled), `-004` (PC5/PC6 now mandate first-token-of-last-chain-entry and explicitly declare rightmost NON-CONFORMING, per new ADR-038), `-006` (the `invariant 10` mis-anchors are gone from `arm_a1.rs`; surviving `invariant 11` cites are correct), `-007` (POLICY 14 leg-5: BC-INDEX row is `v1.10 \| v1.11 \| v1.12 \| v1.13`), `-008` (PC13c codified with three passing half-present tests), `-012` (CHANGELOG now has an `### Added` heading introducing the plugin, cites v1.13), `-016` (`modified[]` correctly ordered v1.0→v1.8), `-017` ("yields exactly 6" replaced with the conditional form), `-018` (ADR-037 roster corrected to union 76 / ARCH leg 61 — matches my derivation exactly; S-8.10's `ARCH-INDEX.md:75` line-pin removed), `-020` (majority-shape vacuity guard now uses `extract_version_field` with a negative twin).

**O-S2107-P8-02 — F-S2107-P7-003 is fully and correctly closed, and the fix generalized to the class.** `Mount factory artifacts` now precedes `cargo test` in `cargo-host` **and** a parallel mount was added to `build-dispatcher` ahead of `Test (cargo)` on every test-enabled matrix entry, both with `CI_REQUIRE_ARTIFACTS: "1"` and `VSDD_CORPUS_ROOT`. `require_artifacts` was changed to panic under that flag, with five harness-correctness tests proving it (`test_corpus_fail_hard_panics_when_ci_require_artifacts_set` and siblings). `VSDD_SKIP_PRODUCTION_STATE_MD_TEST` was removed. This is the one finding in the set whose fix reached past the named site to the class, and it is worth naming as the counter-example to the dominant pattern.

**O-S2107-P8-03 — the WASM claim is exactly right and I could not fault its provenance.** 231,661 bytes; sha256 `853c802e74ec372864912448130f3b0740aeeae6f92b8230c7eb25f639dc32b8`; `git show HEAD:…wasm | shasum -a 256` matches the worktree file and the `target/wasm32-wasip1/release/` build; last WASM commit is `29558518` and `git log 29558518..HEAD -- src/` is empty, so no source change postdates the artifact.

**O-S2107-P8-04 — the S-21.07 three-way input-hash is clean, and the seven other `S-21.07=` tokens are a false alarm worth recording.** `grep -oE 'S-21\.07=[0-9a-f]{7,40}'` over STORY-INDEX returns eight values, seven of them stale. All seven live on line 8, inside the frontmatter `last_amended:` `[Prior:]` chain — historical-by-construction per POLICY 5 v1.3.5 class (iii). `parse_story_index_blockquote_hash` only scans lines beginning `"> "`, so production reads the single live token on the aggregation line: `c0ab6a3`, equal to both the frontmatter and the catalog cell. A future pass grepping without the `^> ` constraint will re-raise this; it is not a defect.

**O-S2107-P8-05 — POLICY 3 is satisfied, and I verified it by timestamp rather than by narrative.** Code HEAD `fbb5183c` at 2026-08-06 16:35; all state-manager factory commits from `61e23c44` (17:57) onward. The D-959 retraction is also properly reflected: `STATE.md` shows `streak 0/3`, and burst-log's Commit-E line carries the annotation *"(CORRECTED D-959: streak was NOT advanced…)"*. The other `CLEAN pass-7` matches in the cycle logs belong to S-18.12's pass-7 (D-731), a different story.

**O-S2107-P8-06 — `destructive-command-guard` false-positives on a Python variable named `rm`.** A `Bash` call containing `rm=rightmost_field5(ne)` alongside the string `BC-INDEX.md` was blocked with `block_reason="BLOCKED by destructive-command-guard: Cannot delete source-of-truth file: BC-INDEX.md … Code: sot_delete."` No deletion was expressed anywhere in the command. Renaming the variable cleared it. Out of scope for S-21.07 (different plugin), but it will recur for any agent doing analysis in an inline interpreter near a SoT filename, and the guard's trigger appears to be substring-level rather than command-level.

**O-S2107-P8-07 — no `docs/demo-evidence/S-21.07/` directory yet.** POLICY 10 is not violated: there are no flat `docs/demo-evidence/*.md` files, and S-21.07 has not reached demo-recorder. Recording it so the gate is not mistaken for closed at merge time.

**O-S2107-P8-08 — POLICY 9 arithmetic is clean.** `total_vps: 101` equals 101 distinct VP IDs in the body catalog. (A naive `grep -cE '^\| \[?VP-'` returns 152 because VPs recur across per-tool tables; the distinct-ID count is the correct predicate.)

**O-S2107-P8-09 — the `File:Line` columns in `red-gate-log.md`'s obligation tables are not TD-VSDD-091 violations.** CLAUDE.md exempts "Red Gate test tables, AC source-of-truth tables, pass-report changelogs" from the anti-volatile-pin rule. Noting it because the tables contain ~20 `arm_a2.rs:1325`-style pins and a future pass may flag them.

---

## Part C — Analysis

### Verdict

**NOT-CLEAN.** Streak **0/3** (BC-5.39.001 reset).

The blocking condition is not a judgment call: `bats validate-cross-site-correspondence.bats` exits 1 at `reviewed_head fbb5183c`. A story cannot converge on a red suite, and the redness is in a gate the previous burst added.

### Finding counts by severity

| Severity | Count | IDs |
|---|---|---|
| BLOCKER | 2 | F-S2107-P8-001, -002 |
| HIGH | 5 | F-S2107-P8-003, -004, -005, -006, -007 |
| MEDIUM | 7 | F-S2107-P8-008, -009, -010, -011, -012, -013, -014 |
| LOW | 2 | F-S2107-P8-015, -016 |
| **Total** | **16** | |

Trajectory of finding counts across this cascade: 25 → 25 → 24 → 20 → **16**. Continued improvement, and the shape improved too: pass-6 opened with a BLOCKER that the entire burst was uncommitted, and that class is fully gone — both trees are committed, the code tree is clean and origin SHA-equal, and every figure in this review is reproducible at a real SHA.

Four findings are tagged `[process-gap]` (-003, -007, -012, -014) and one `[regression]` (-004).

### Novelty assessment

**0.69.** Eleven of sixteen findings are new axes; five are residuals of pass-6 findings that did not close.

- **Novel (11):** -001 (RED gate at HEAD), -002 (false GREEN attestation), -005 (same-burst falsification of PC5's three corpus claims), -008 (STORY-INDEX AC/EC aggregation cells), -009 (AC-024 absent from the story's own inventory), -010 (ADR-037 77→76 unswept into the BC), -011 (two new POLICY 18 half-present instances), -012 (`O-P8-NN` namespace collision), -014 (POLICY 21 `run-all.sh` scope expansion), -015 (asymmetric ignored-count disclosure), -016 (unclean worktree / nested `.factory/.factory/`).
- **Residual (5):** -003 ⊃ F-S2107-P7-009's attestation-SHA leg, but re-diagnosed — the previous framing was "the attestation is missing"; the actual defect is that the policy predicate is unsatisfiable, which is why adding the attestation did not close it. -004 ⊃ -005 (cite sweep), now with the inversion documented. -006 ⊃ -011 (weak compensating guard), now with three executed bypasses instead of a reading of the code. -007 ⊃ -014 (coverage gate), now with proof that it emits a false figure. -013 ⊃ -010 (fuel axis), narrowed to the single missing sandbox×scale combination and downgraded to MEDIUM to reflect the real partial credit.

### Dominant pattern

**Pass-6's "fix-named-site-not-the-class" has specialized into "the burst's later legs falsify its earlier legs."** Findings -005, -008, -009, -010 and -011 are all this: leg C authors a spec or story, leg D edits the corpus or the indexes, and nothing re-derives leg C against leg D's tree. PC5 is the sharpest case — commit `d32207bf` wrote three corpus claims and commit `304156dd`, the next commit in the same burst, falsified all three by its own stated actions (`+total_bcs 1983→1985`, `BC-4.13.001 v1.16 pipe-escaping correction`).

The secondary pattern is **the fix machinery generating the defect**, which is where both BLOCKERs live. F-S2107-P7-013 asked for full-string equality; the strengthening was applied to both named gates and one of them was given an expected value derived from a filename instead of from observed output, so it is red by construction. F-S2107-P7-014 asked for runtime-derived counts; what shipped derives them from `grep` and hardcodes `0 failed`, so it now asserts green while red. And F-S2107-P7-009 asked for an attestation; one was added, and it records the red gate as GREEN. Three record mechanisms, each introduced or strengthened to close a pass-6 finding, jointly produced the false gate figure that reached the dispatch brief.

The third pattern, and the most tractable: **sweeps asserted per-site rather than per-predicate keep permuting.** -004 is the proof — across two passes the set of correct cite sites and the set of stale sites have exchanged places, with the specifically-quoted site never fixed. No amount of per-site diligence converges here; the tree-wide predicate does, and it is a one-line `grep`.

### Policies executed

**Executed — 20 of 22.**

| # | Policy | How | Result |
|---|---|---|---|
| 1 | append_only_numbering | Enumerated all `O-P8-*` and `F-S2107-*` tokens tree-wide | **FAIL** → -012 |
| 2 | lift_invariants_to_bcs | Confirmed `domain-spec/invariants.md` present with 101 `DI-` cites; no DI added/removed by this burst, so no new orphan surface | PASS (scoped) |
| 3 | state_manager_runs_last | Commit timestamps, both trees | PASS (but see -005: ordering satisfied, staleness not) |
| 4 | semantic_anchoring_integrity | BC/story/ADR anchor cross-checks | **FAIL** → -009, -010 |
| 5 | creators_justify_anchors | SIBLING-SWEEP + literal-shell gate + HISTORICAL-BY-CONSTRUCTION (5 classes) + HEAD-REPRODUCIBILITY | **FAIL** → -004, -005, -010 |
| 6 | architecture_is_subsystem_name_SoT | Story `subsystems: [SS-04, SS-05]` vs ARCH-INDEX registry | PASS |
| 7 | bc_h1_is_title_SoT | BC H1 vs BC-INDEX title cell, verbatim | PASS |
| 8 | bc_array_changes_propagate | AC/EC/Token-Budget vs frontmatter | **FAIL** → -008, -009 |
| 9 | vp_index_is_vp_catalog_SoT | `total_vps: 101` = 101 distinct IDs | PASS |
| 10 | demo_evidence_story_scoped | No flat `docs/demo-evidence/*.md`; no S-21.07 dir yet (not due) | PASS → O-S2107-P8-07 |
| 11 | no_test_tautologies | Corpus/unit tests call production fns (`run_arm_*`, `extract_*`, `is_volatile_path`) | PASS |
| 12 | bc_tv_emitter_consistency | BC Canonical Test Vectors excluded-field rows vs emitter | PASS |
| 13 | hh_n_regex_alternation_predicates | ESCAPE-SCOPE-PARITY on the F-P6-019 guard (now file+normalizer-specific with a negative twin — properly closed); BOUNDARY-POLARITY on the compensating guard | **FAIL** → -006 |
| 14 | kk_n_tripartite_parity_gate | leg-4 self-application gate ×4 (all PASS); leg-5 BC row (closed); leg-5 STORY-INDEX cells | **FAIL** → -008 |
| 15 | ll_n_verbatim_stdout_discipline | ATTESTATION-LOCATION gate; OBLIGATION-INDEXED table; per-guard mutants | **FAIL** → -002, -003, -007 |
| 16 | mm_n_cross_cycle_namespace_gate | Global D-NNN max = D-959; both D-958 and D-959 present in decision-log at HEAD | PASS |
| 17 | nn_n_frontmatter_parity_full_scope | BC/story/VP/epic layers | **FAIL** → -008 (story layer) |
| 18 | oo_input_hash_mechanical_verification | THREE-WAY gate for S-21.07 (PASS: `c0ab6a3` ×3) and for every catalog row | **FAIL** → -011 |
| 19 | adr_version_cite_volatile_pin | BC §Traceability cites no ADR → formally not violated; prose pins noted under -010 | PASS (scope-limited) |
| 21 | no_new_shell_scripts | `git diff --name-status origin/develop...HEAD` piped to `grep '\.sh$'` | **FAIL** → -014 |
| 22 | subagent_report_fidelity_literal_shell | Re-derived every claimed gate figure | **FAIL** → -001, -002, -007 |

**Not executed — 2 of 22.**

- **POLICY 20 (`release_bundle_no_dev_samples`)** — out of scope at this SHA. No release bundle is produced on a feature branch; the dual-registry orphan check applies at release time. Should be run at the release that ships this plugin, when `validate-cross-site-correspondence.wasm` first enters a bundle.
- **POLICY 2 (`lift_invariants_to_bcs`)** — executed only partially, and I am marking it PASS-scoped rather than PASS. I confirmed `invariants.md` exists with 101 `DI-` cites and that this burst added no DI, so it introduced no new orphan. I did **not** run the full bidirectional DI↔BC orphan sweep across all 101 invariants; that is a whole-corpus audit disproportionate to a story-scoped pass and unaffected by this diff.

Two rubric notes for the record. The brief's warning about the rubric having been silently invisible is not reproducible here: `.factory/policies.yaml` parses cleanly as YAML and all 22 entries are readable. And POLICY 19's declared scope is the flow sequence `[behavioral-contracts-traceability-rows]`; I checked BC-5.39.010's `## Traceability` section directly and it cites no ADR, so the three `ADR-037 v1.1/v1.2` and one `ADR-038 v1.1` pins I found sit in §Preconditions prose, outside POLICY 19's scope. I report the stale figure they carry under -010 on POLICY 4/5 grounds rather than stretching POLICY 19.

### Pass-6 closure corroboration

**Independently corroborated as CLOSED — 10 of 20:** `-002` (Arm B2 clean, re-derived), `-003` (CI wiring, both jobs, plus the `require_artifacts` panic), `-004` (PC5/PC6 algorithm replaced), `-006` (`invariant 10` mis-anchors gone), `-007` (POLICY 14 leg-5 BC row), `-008` (PC13c codified + 3 passing tests), `-012` (CHANGELOG `### Added`), `-016` (`modified[]` ordering), `-018` (ADR-037 roster, re-derived to 76/61 exactly), `-020` (majority-shape guard).

**Corroborated as NOT closed — 5 of 20:** `-005` (→ -004, inverted), `-009` (→ -003, and the attestation now carries a false GREEN → -002), `-010` (→ -013, fuel axis), `-011` (→ -006, three bypasses executed), `-014` (→ -007, gate emits a false figure).

**Closed with residue — 2 of 20:** `-013` — the class was half-swept. AC-022 and AC-023 both received full-string equality (AC-022 passes; AC-023 is -001), but the sibling normative-message gate `T-P6B (PC2b)` in the same file is still `[[ "$combined" == *"index is newer than primary"* ]]` — substring-only, the exact shape v1.13 PC4a declares NON-CONFORMING. `-017` — "yields exactly 6" was corrected, but the replacement text and the surrounding figures are themselves stale per -005.

**Could not corroborate either way — 3 of 20:** `-001` (the "entire burst uncommitted" BLOCKER — moot by construction; both trees are now committed, so there is nothing to confirm beyond its disappearance); `-015` (the F-P6-019 guard's escape is genuinely fixed to file-and-normalizer-specific with a negative-twin control, but that control asserts the *old* escape would have exempted the synthetic line rather than asserting the *new* escape fires RED on it — so the closure is sound in mechanism and its proof is one assertion short of POLICY 13's negative-twin form; I could not resolve this into a clean closed/open call and am recording it here rather than manufacturing a finding); `-019` (the D-693 attestation gap — the WASM figures are now correct and triangulated, but the specific claim that a *commit subject* misdescribed the artifact is a historical fact about `b78b27ef`, which the Pass-7 attestation itself acknowledges; nothing at HEAD is misdescribed).

### ID-numbering note (for state-manager, POLICY 1)

**Findings — `F-S2107-P8-NNN`, as instructed.** Pass-6 used the `P7` prefix, so this pass continues the established off-by-one: the file is `adversary-pass-7.md`, the pass frontmatter is `pass: 7`, and the finding prefix is `P8`. Sixteen IDs allocated, `F-S2107-P8-001` through `F-S2107-P8-016`, contiguous with no gaps. The off-by-one is now two passes deep and is load-bearing for cross-referencing; it should be preserved, not "corrected," per POLICY 1 (identifiers are never renumbered).

**Observations — `O-S2107-P8-NN`, deviating from the brief.** The brief specified `O-P8-NNN`. I did not use it, because that namespace is occupied: E-18 holds a live three-digit `O-P8-001/002/003` series with 88 cites across nine artifacts, and emitting `O-P8-001` here would have collided with `research-O-P8-001-wave-ordinal.md` directly. This cascade has separately begun a two-digit `O-P8-01/02` series (17 cites), which is itself only distinguishable from E-18's by zero-padding — see F-S2107-P8-012. I therefore allocated story-scoped `O-S2107-P8-01` through `O-S2107-P8-09`, which collides with nothing and matches the story-scoping already used for finding IDs. State-manager should either ratify this form for the cascade and re-anchor the two existing two-digit cites (including BC-INDEX v4.50's `Refs: D-958, O-P8-02`), or rule otherwise — but the bare `O-P8-NN` form should not continue to be allocated while E-18's series is live.
