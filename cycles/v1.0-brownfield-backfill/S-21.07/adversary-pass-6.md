# Adversarial Review — S-21.07 LOCAL cascade, Pass 6

```yaml
---
review_type: local-story-adversarial
story_id: S-21.07
cycle: v1.0-brownfield-backfill
pass: 6
passes: 6
reviewed_head: "b78b27ef (+ UNCOMMITTED working tree in BOTH repos — see F-S2107-P7-001)"
reviewed_branch: develop @ /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07
worktree: /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07
story_version: "1.7 (uncommitted)"
bc: "BC-5.39.010 v1.12 (uncommitted)"
adrs_read: [ADR-037 v1.2 (uncommitted), ADR-035 (cited only)]
verdict: NOT-CLEAN
findings_count: 20
severity_breakdown:
  BLOCKER: 4
  HIGH: 7
  MEDIUM: 8
  LOW: 1
observations_count: 6
streak: 0/3
trajectory_append: 20
trajectory: 47 → 18 → 25 → 25 → 24 → 20
gates_independently_executed:
  cargo_test: "143 passed / 1 failed / 17 ignored — CLAIM VERIFIED"
  cargo_fmt: "exit 0 — CLAIM VERIFIED"
  cargo_clippy: "clean, --all-targets -D warnings — CLAIM VERIFIED"
  bats: "NOT executed; 51 @test statically, 5 unconditional Class-D skips → 46 executable"
prior_pass_records_read:
  - adversary-pass-5.md — Part A only (Iron Law honoured)
---
```

## Part A — Findings

### BLOCKER

#### F-S2107-P7-001 — BLOCKER — The entire pass-6 fix burst is uncommitted in **both** repositories. `HEAD` is `b78b27ef` — the exact SHA pass-5 reviewed. No commit exists to attest against, the POLICY 15 attestation-location gate returns 0, and one `git checkout` destroys the burst.

**Location:** worktree root `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07` (code) and `/Users/zious/Documents/GITHUB/vsdd-factory/.factory` (factory-artifacts).

**Clause violated:** POLICY 5 v1.3.6 HEAD-REPRODUCIBILITY-OR-STRUCTURAL-FORM MANDATE; POLICY 15 ATTESTATION-LOCATION GATE (D-912); POLICY 16 RECORD-PERSIST-BURSTS-INCLUDE-SAME-COMMIT-CODIFICATION; POLICY 22; D-946 precedent.

**Evidence — literal shell:**

```
$ git rev-parse --short HEAD
b78b27ef
$ git log --oneline -1
b78b27ef build(S-21.07): deploy WASM v1.10 — 226,794 bytes (D-693 pass-5 gate PASS)
```

`b78b27ef` is `reviewed_head` in `adversary-pass-5.md` frontmatter. All pass-6 work is working-tree only:

```
$ git status --porcelain
 M CHANGELOG.md
 M crates/hook-plugins/validate-cross-site-correspondence/Cargo.toml
 M crates/hook-plugins/validate-cross-site-correspondence/src/arm_a1.rs
 M crates/hook-plugins/validate-cross-site-correspondence/src/arm_a2.rs
 M crates/hook-plugins/validate-cross-site-correspondence/src/arm_b.rs
 M crates/hook-plugins/validate-cross-site-correspondence/src/arm_e.rs
 M crates/hook-plugins/validate-cross-site-correspondence/src/frontmatter.rs
 M crates/hook-plugins/validate-cross-site-correspondence/src/lib.rs
 M plugins/vsdd-factory/hook-plugins/validate-cross-site-correspondence.wasm
 M plugins/vsdd-factory/hooks-registry.toml
 M plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats
 (+6 fixture files)
?? .../fixtures/.../a1-index-ahead-of-primary/
?? .../fixtures/.../a1-index-behind-primary/
?? .../fixtures/.../b1-story-index-consistent-stale/
?? .../fixtures/.../b1-story-index-inconsistent/
?? plugins/vsdd-factory/tests/helpers/dispatcher-provenance.bash
```

```
$ git -C .factory log --oneline -1
aa07075f factory(pause): session wrap — S-21.07 pass-5 recorded, 3 BLOCKERs open, streak 0/3
$ git -C .factory status --porcelain
 M specs/architecture/ARCH-INDEX.md
 M specs/architecture/decisions/ADR-037-...-volatile-artifacts-excluded.md
 M specs/behavioral-contracts/BC-INDEX.md
 M specs/behavioral-contracts/ss-05/BC-5.39.010.md
 M stories/S-21.07-validate-cross-site-correspondence.md
 M stories/STORY-INDEX.md
 (+ regression-state.json, sidecar-learning.md, 2 log files)
```

POLICY 15's gate, executed literally:

```
$ grep -c "assertion-site attestation" crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md
0
```

**Failure scenario.** (1) **Non-replayable review** — POLICY 5 v1.3.6 requires captured stdout be reproducible at a SHA; there is no SHA. Every gate figure the burst records, including the ones I verified, describes a tree that exists only on this machine. A fresh-context pass-7 cannot reproduce any of it. (2) **Unrecoverable on any tree-clearing operation** — BC v1.11→v1.12, story v1.6→v1.7, ADR-037 v1.2, six code fixes, four new fixtures, `helpers/dispatcher-provenance.bash`, and the new corpus/guard/control tests are all in a dirty tree; the project's own git-safety protocol names `git checkout --` / `git stash` as routine recovery moves. (3) **POLICY 3 is unevaluable** — `state_manager_runs_last` is a commit-ordering constraint; with zero commits there is no ordering to inspect, so the central question this burst set out to settle cannot be answered from the record. This subsumes the merge-order adjudication entirely.

**Routing.** Orchestrator, before any further specialist dispatch: land the code burst and the factory-artifacts burst as commits (state-manager last per POLICY 3), with `red-gate-log.md` bundled in the same commit per POLICY 15 + POLICY 16, then re-run this review against the resulting SHA.

---

#### F-S2107-P7-002 — BLOCKER — Arm B2 blocks on the **live** `STORY-INDEX.md` right now. Two committed stories have catalog ≠ blockquote, so every write to STORY-INDEX.md exits 2 — including state-manager's own index write. The Option 1 carve-out closed 2 of the 4 self-lock legs; this one it deliberately left BLOCKING, and it is live.

**Location:** `arm_b::run_arm_b2` (the catalog→blockquote comparison loop) and `arm_b::run_arm_b1_with_index_result` PC13b arm, in `crates/hook-plugins/validate-cross-site-correspondence/src/arm_b.rs`; `.factory/stories/STORY-INDEX.md` catalog rows + aggregation blockquote for `S-18.06` and `S-18.08`; `BC-5.39.010.md` postconditions 13b and 15.

**Clause violated:** BC-5.39.010 PC13b / postcondition 15 vs POLICY 3 (`state_manager_runs_last`); POLICY 11; production-grade default.

**Evidence.** Re-implemented the production parser semantics (`extract_story_id_from_table_row` — first pipe-cell must equal `S-[0-9]+\.[0-9]+` exactly; `extract_input_hash_token` / `extract_blockquote_pairs` — `[0-9a-f]{7,40}`) and ran them over the live file:

```
catalog entries: 29
blockquote entries: 27
ARM B2 VIOLATIONS (catalog!=blockquote): 2
    ('S-18.06', '63d94a3', 'cf37976')
    ('S-18.08', 'fe61c2c', '747b3eb')
catalog rows with NO blockquote entry: 2 [('S-18.11', 'c45c0fc'), ('S-18.12', '345086c')]
```

**Failure scenario.** `run_arm_b2` fires on any PostToolUse Edit/Write whose basename is `STORY-INDEX.md` under `stories` (PC22). It returns 2 violations → `block_with_fix` → `exit 2`. The registry entry is `event = "PostToolUse"`, `tool = "^(Edit|Write|MultiEdit)$"`, `on_error = "continue"` — `on_error` covers crash/fuel only; an explicit block still blocks. So the first STORY-INDEX.md write after this hook ships blocks, and that write is the state-manager Commit D this story's own burst requires. Separately, PC13b blocks any write to `S-18.06` or `S-18.08`'s story file regardless of B1, because `b2 != b3`.

This is the F-P6-001 shape, not a new one. Option 1 fixed the two legs whose live instances were the story's *own* artifacts (Arm A1 on BC-5.39.010, Arm B1 on S-21.07 — both verified clean now, O-P7-04). The two legs whose live instances are *other* artifacts were left BLOCKING and never measured. PC13b's rationale — "both catalog row and blockquote are written by state-manager in the same commit, so B2≠B3 has no burst-ordering explanation" — is sound as reasoning and false as a premise about this corpus: there are two committed counterexamples.

**No gate covers this.** The eight corpus tests in `lib.rs` are `corpus_arm_a1_*` ×3, `corpus_arm_a2_*`, `corpus_dispatch_*` ×2, `corpus_arm_e1_*` ×2, `corpus_arm_b1_s21_07_no_violations`, `corpus_is_volatile_path_live_story_inputs`, `corpus_version_sync_*`. **There is no `corpus_arm_b2_*` test.** A single assertion `run_arm_b2(live STORY-INDEX) == []` would be RED right now and would have been RED at pass-5 and pass-4. F-P6-010 asked for exactly that class for A1 and B1; the sweep stopped at the two arms the finding named.

**Routing.** state-manager: reconcile S-18.06/S-18.08 catalog↔blockquote; add S-18.11/S-18.12 blockquote entries. product-owner: adjudicate PC13b's premise (see F-S2107-P7-008). test-writer: add the missing `corpus_arm_b2` assertion.

---

#### F-S2107-P7-003 — BLOCKER — The compensating corpus sync test — the human-approved substitute for the block PC2a removed — **never executes in CI**. `cargo test` runs before `.factory/` is mounted in every job, and neither `VSDD_CORPUS_ROOT` nor `CI_REQUIRE_ARTIFACTS` is set for it. All eight corpus tests skip silently on every platform.

**Location:** `.github/workflows/ci.yml` — job `cargo-host`, step `cargo test (workspace, all targets)` vs. the later step `Mount factory artifacts (for perf-baseline bats)`; job `build-dispatcher`, step `Test (cargo)`. `crates/.../src/lib.rs` — `live_factory_root()`, the `corpus_root_or_skip!` macro, `test_BC_corpus_version_sync_all_indexed_bcs_match_frontmatter`.

**Clause violated:** TD-VSDD-059 (paper-fix: compensating guard inert where it must run); POLICY 11; POLICY 15; production-grade default.

**Evidence.** `corpus_root_or_skip!` resolves via `live_factory_root()`, which requires a `.factory/` containing `specs/behavioral-contracts/` within 8 ancestors of `CARGO_MANIFEST_DIR`, or an explicit `VSDD_CORPUS_ROOT`. `.factory/` is not tracked on the code branch:

```
$ git ls-files | grep -c '^\.factory/'
0
$ grep -n '^\.factory' .gitignore
6:.factory/
```

In `cargo-host` the step order is: `cargo fmt` → `cargo clippy` → `cargo check` → **`cargo test --workspace --all-targets`** (env: `VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1` only) → `cargo publish --dry-run` → wasm builds → **`Mount factory artifacts`** (`git worktree add .factory origin/factory-artifacts`) → bats. The mount is downstream of `cargo test`, and `cargo test` is never re-run. In `build-dispatcher`, `Test (cargo)` runs with no mount at all — the step's own comment says "which is not mounted in this job". `bats-full-suite` mounts `.factory` but runs bats, not cargo. `CI_REQUIRE_ARTIFACTS: "1"` appears only on the two bats steps.

Therefore `live_factory_root()` returns `None`, the macro prints `[CORPUS-SKIP] .factory/ not found; …` and `return`s, and the test passes vacuously.

**Failure scenario.** The design intent is stated verbatim in the test's own comment block: *"write-time advisory, commit-time hard failure"* — PC2a tolerates the mid-burst state, CI restores the invariant once committed. The commit-time half does not exist. A BC bumped without its BC-INDEX row updated produces a write-time advisory (correct, PC2a) and a green CI run (incorrect). The invariant PC2 v1.10 enforced is now enforced **nowhere in any automated gate** — only in a local `cargo test` on a machine that happens to have `.factory/` mounted. That is the brief's question 1, and the answer is: nothing catches a committed BC/BC-INDEX desync. The same inertness applies to all seven other corpus tests, including the two added to close F-P6-010.

**Routing.** devops-engineer: move `Mount factory artifacts` above `cargo test` in `cargo-host` and set `CI_REQUIRE_ARTIFACTS=1` for it, or add a corpus-gated cargo job that mounts first. The graceful-skip design is correct; the CI wiring that would make it non-vacuous was never added.

---

#### F-S2107-P7-004 — BLOCKER — BC-5.39.010 v1.12 PC5 and PC6 still normatively mandate **rightmost**-token extraction from the **6th non-empty field**. The implementation deliberately implements first-token-of-**last chain entry** over a **join of fields 5..**. The spec was amended twice this burst and neither clause was touched. Per CLAUDE.md §12 the spec wins, so the shipped extractor is non-conforming to its governing BC.

**Location:** `BC-5.39.010.md` §Preconditions PC5 `Version(v)` bullet (*"Extract the **latest (rightmost)** such token"*) and PC6 (*"the version token is the LAST (rightmost) … match in the 6th column's cell content … the rightmost token is always the current."*); `arm_a1::extract_first_v_token_of_last_entry`, `arm_a1::extract_first_v_token`, and the `n if n >= 6` arm of `arm_a1::extract_bc_index_version_state` (which computes `non_empty_fields[5..].join("|")`).

**Clause violated:** CLAUDE.md Architectural Authority §12 (code-vs-spec → SPEC wins); POLICY 4; POLICY 8.

**Evidence.** `grep -n 'rightmost\|latest (' BC-5.39.010.md` matches at PC5, PC6 (twice), PC13 Phase 2, and the v1.8 Changelog row. `grep -n 'first v-token\|FIRST'` over the BC returns **zero matches** — the word "first" never appears as an extraction direction anywhere in the governing spec. The v1.11 and v1.12 `last_amended` entries enumerate PC2, PC13, PC4a, PC5 (≥6-field/no-v-token), PC36, PC40, §Architecture Anchors, §Traceability, §VP Anchors, the EC table, Canonical Test Vectors, §Gate Spec prose and the VP table. Neither mentions extraction direction.

The code's own doc comments admit the divergence: *"F-P6-019b/019c: first-token-of-last-entry extraction (**replaces** F-S2107-P1B-006 last-wins)"* and *"F-P6-019d: join fields[5..] to recover cell content fragmented by bare `|` characters."* PC5 states the opposite as fact: *"A naive `|` split on a version-chain row produces 15+ fields instead of 6; the escape-aware split yields **exactly 6**."*

**Failure scenario.** A conforming re-implementation from the BC alone reproduces the four defects this burst just fixed. Traced by hand against live rows:

| Row | Spec algorithm (rightmost in field 6) | Frontmatter | Spec-conforming result |
|---|---|---|---|
| `BC-3.08.001` | `v1.23` (inside `(promoted v1.23 D-839 …)`) | `1.24` | mismatch → PC2a advisory |
| `BC-7.03.079` | `v1.4` (inside `[prior: v1.4 …]`) | `v1.5` | mismatch → PC2a advisory |
| `BC-4.13.001` | `v1.16` (field 6 truncated at the bare `\|` in `^(Edit\|Write\|MultiEdit\|Agent)$`) | `1.18` | mismatch → PC2a advisory |
| `BC-5.24.006` | `1.3` | `v1.3` | equal only *after* the v-strip PC2 now mandates |

The three advisories are noise, not blocks — Option 1 masks the spec's defect. But the divergence is load-bearing in the other direction: a future BC whose chain-entry annotation cites a *higher* version than the entry's own token produces a spurious PC2b **block** under the spec algorithm. More immediately, the BC is what the next implementer reads, and it prescribes an algorithm the burst rejected with no record that it was rejected.

**Routing.** product-owner: amend PC5/PC6 to the first-token-of-last-chain-entry algorithm and the fields-5.. reassembly, with the four corpus rows as justification, and correct the "yields exactly 6" claim (F-S2107-P7-017).

---

### HIGH

#### F-S2107-P7-005 — HIGH — The declared "v1.11→v1.12 live-cite sweep across story, bats and `crates/`" covered four sites and missed ~53 in `crates/`, including the crate's own module-level governing-spec header (`lib.rs`, still v1.10) and `main.rs`'s §Compliance notes. Third consecutive pass on the identical class; the sweep fixed the sibling site (`bats` header) and left this one.

**Location:** `crates/.../src/lib.rs` module doc header (`//! BC-5.39.010 v1.10 — six-arm PostToolUse cross-site value-correspondence gate …`); `crates/.../src/main.rs` `//! # Compliance notes (BC-5.39.010 v1.10)` + the `§Gate Spec` line beneath; `crates/.../src/arm_b.rs` `run_arm_b1` doc comment `PC40 (BC-5.39.010 v1.6)` and its `# BC trace`; `arm_a1.rs` RowMalformed unit-test block and assertion messages (`v1.10 postcondition 4a` ×7, `v1.10 PC5` ×9); `arm_a2.rs` PC13 collision-class docs (`v1.10 PC13` ×6); `arm_e.rs`/`dispatch.rs`/`arm_d.rs`/`arm_b.rs` `v1.3 §E1`, `v1.3 §Classification invariant`, `v1.3 PC16`, `v1.3 precondition 17`, `v1.3 §B3 invariant`, `v1.3 §D precondition 31`; `crates/.../docs/red-gate-log.md` `**Governing spec:** BC-5.39.010 v1.10` ×2.

**Clause violated:** POLICY 5 v1.3.4 SIBLING-SWEEP LITERAL-SHELL VERIFICATION GATE + v1.3.5 Part A HISTORICAL-BY-CONSTRUCTION ENUMERATION; POLICY 14 leg 5; S-7.01. Lineage: F-P2-017 → F-S2107-P3-010 → F-S2107-P4-011 → F-S2107-P6-008 → this pass.

**Evidence — literal shell:**

```
$ grep -rnE 'BC-5\.39\.010 v1\.[0-9]+' crates/ | grep -vE 'v1\.12' | wc -l
      53
```

```
crates/.../src/lib.rs:15://! BC-5.39.010 v1.10 — six-arm PostToolUse cross-site value-correspondence gate …
crates/.../src/main.rs:14://! # Compliance notes (BC-5.39.010 v1.10)
crates/.../src/main.rs:19://! - `event = "PostToolUse"`, `tool = "^(Edit|Write|MultiEdit)$"` (BC-5.39.010 v1.10 §Gate Spec).
crates/.../src/arm_b.rs:414:/// PC40 (BC-5.39.010 v1.6): if the story's `inputs:` list contains any volatile
crates/.../docs/red-gate-log.md:585:**Governing spec:** BC-5.39.010 v1.10
crates/.../docs/red-gate-log.md:1087:**Governing spec:** BC-5.39.010 v1.10
```

By contrast `Cargo.toml` (`description = "… (BC-5.39.010 v1.12 …)"`), `hooks-registry.toml` (`(BC-5.39.010 v1.12 §Gate Spec; ADR-035 §Decision 5)`) and `validate-cross-site-correspondence.bats` (`# Governing BC: BC-5.39.010 v1.12`) **were** swept. `bats:38` was the site F-P6-008 named; `lib.rs`'s module header is its exact structural twin — the file-level declaration of which spec version the deliverable implements — and was not swept.

POLICY 5 v1.3.5 Part A enumerates the five historical-by-construction classes exhaustively: frontmatter `modified[]`, body Changelog rows, `[Prior:]` clauses, §Adversary Pass Coverage entries, `lessons.md` cross-references. Rust doc comments and assertion messages are in none of them. The `[DEFERRED v1.6 — Class D]` markers **are** legitimately frozen (version-stamped deferral events); every other cite above is a live authority cite to a clause still live at v1.12, and three of them (PC4a, PC5, PC40) were substantively amended after the version cited.

**Failure scenario.** A reader opening `lib.rs` is told the crate implements v1.10 — the version whose PC2 mandated undirected blocking, i.e. the version this burst's central change repudiates. `arm_b.rs`'s `run_arm_b1` doc comment cites PC40 at v1.6, the version whose transitional clause v1.11 replaced and whose "widening" characterization v1.11 withdrew. Each is a wrong pointer for the next implementer, and the class has survived five passes with two closure attestations claiming completeness.

---

#### F-S2107-P7-006 — HIGH — F-P6-009's four-site mis-anchor was closed at the two `bats` sites and left live at the two `arm_a1.rs` sites. "v1.3 invariant 10" still labels version-token extraction; invariant 10 is POLICY 21 `.sh`-script compliance.

**Location:** `crates/.../src/arm_a1.rs` — the two doc comments introducing the escaped-pipe-chain extraction test and the table-body-scan test (both begin `/// BC-5.39.010 v1.3 invariant 10:`); against `BC-5.39.010.md` §Invariants item 10.

**Clause violated:** POLICY 4; S-7.01 Partial-Fix Regression Discipline; `[regression]`.

**Evidence — literal shell:**

```
$ grep -rn 'invariant 1[01]' plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats \
    crates/hook-plugins/validate-cross-site-correspondence/src/*.rs
…bats:1637:  # Block message must enumerate all three provenance categories (invariant 11 requirement).
…bats:1640:    echo "  BC-5.39.010 v1.12 PC13b + invariant 11: all three provenance categories required."
…arm_a1.rs:728:    /// BC-5.39.010 v1.3 invariant 10: when version_history has escaped-pipe delimiter,
…arm_a1.rs:763:    /// BC-5.39.010 v1.3 invariant 10: extract_bc_index_version_state must scan only table body
```

BC §Invariants, verbatim:

```
10. **POLICY 21 compliance**: no `.sh` scripts. All gating uses WASM plugin or Rust workspace
    tests. Class C and the Class D existence-check gap are routed to Rust workspace tests.
```

The two former bats mis-anchors are gone and the surviving bats invariant-11 cites are correct — so the burst read the finding, corrected the sites in the file it was already editing, and did not sweep the other file the finding named. Correct anchors: PC5/PC6 for token extraction; PC5 condition (1) for the table-body scan.

---

#### F-S2107-P7-007 — HIGH — All three touched indexes have **body-table rows edited with no `version:` or `last_amended:` bump**. BC-5.39.010's row was advanced to `v1.11` — one version behind the BC. POLICY 14's own leg-4 self-application gate reports PASS on all four indexes, proving the gate cannot see this class.

**Location:** `.factory/specs/behavioral-contracts/BC-INDEX.md`, `.factory/stories/STORY-INDEX.md`, `.factory/specs/architecture/ARCH-INDEX.md` — frontmatter `version:` / `last_amended:` vs their working-tree body diffs.

**Clause violated:** POLICY 14 legs 1/2/4/5 (5-leg quintuple parity, same-burst); POLICY 17; POLICY 3.

**Evidence — literal shell:**

```
$ git -C .factory diff --stat specs/behavioral-contracts/BC-INDEX.md stories/STORY-INDEX.md specs/architecture/ARCH-INDEX.md
 specs/behavioral-contracts/BC-INDEX.md | 2 +-
 stories/STORY-INDEX.md                 | 6 +++---
 specs/architecture/ARCH-INDEX.md       | 2 +-

$ grep -m1 '^version:' .factory/specs/behavioral-contracts/BC-INDEX.md
version: "4.48"
$ grep -m1 '^last_amended:' .factory/specs/behavioral-contracts/BC-INDEX.md | cut -c1-140
last_amended: "2026-08-05 (v4.48) — D-956 S-21.07 pass-5 record burst (state-manager): BC-5.39.010 body-table row v1.9→v1.10 …
```

`STORY-INDEX.md` → `version: "4.285"`, `last_amended` cites the D-956 **pass-5** record burst. `ARCH-INDEX.md` → `version: "3.43"`, same. All three carry pass-5 frontmatter with pass-6 bodies. The BC-INDEX row post-image:

```
| [BC-5.39.010](ss-05/BC-5.39.010.md) | … | draft | E-12 | S-21.07 | v1.10 \| v1.11 |
```

against `BC-5.39.010.md` frontmatter `version: "1.12"`.

POLICY 14's mandated leg-4 gate, executed verbatim from the policy template:

```
PASS specs/behavioral-contracts/BC-INDEX.md v=4.48 la=4.48
PASS specs/verification-properties/VP-INDEX.md v=2.74 la=2.74
PASS stories/STORY-INDEX.md v=4.285 la=4.285
PASS specs/architecture/ARCH-INDEX.md v=3.43 la=3.43
```

**Failure scenario.** Four PASS lines on a burst that edited three index bodies without bumping any index version. The gate compares `version:` against its own `last_amended:` v-prefix — an internal-consistency check satisfied by *not touching either field*. It is structurally incapable of detecting a body edit without a version bump, which is the defect POLICY 14 leg 5 exists to prevent. The `v1.11` cell is also the direct cause of the one failing `cargo test` — and per F-S2107-P7-003, CI would not have caught it. The row was advanced to the version the *prior* finding named rather than to current: the same one-behind pattern as F-S2107-P7-005.

**Routing.** state-manager: BC-INDEX row → `v1.10 \| v1.11 \| v1.12`; bump all three index `version:` + `last_amended:` in the same commit; extend the leg-4 gate with a body-diff-implies-version-bump predicate.

---

#### F-S2107-P7-008 — HIGH — PC13's v1.11 split into 13a (`B2==B3`) and 13b (`B2≠B3`) leaves the **half-present** case — one site present and differing, the other absent — with no normative disposition. The implementation retains the v1.10 BLOCK there, contradicting PC12 as literally written. Two live corpus instances.

**Location:** `BC-5.39.010.md` postconditions 12, 13, 13a, 13b; `arm_b::run_arm_b1_with_index_result` — the `(Some(b2), None)` and `(None, Some(b3))` match arms.

**Clause violated:** BC-5.39.010 postcondition 12 as written; PC13 state-enumeration completeness; POLICY 3. Same class as F-P6-018 (PC5's unenumerated fifth state) recurring in PC13.

**Evidence.** PC12, verbatim: *"Arm B1 — B2 or B3 absent: `host::log_warn` advisory + `HookResult::Continue`."* Unconditional. PC13's sub-cases are guarded on `B2 == B3` and `B2 ≠ B3`; when one site is absent neither predicate is defined. The shipped code:

```rust
(Some(b2), None) => {
    if b2 != story_hash {
        violations.push(Violation { … "input-hash mismatch for story {story_id} \
            — story={story_hash} catalog={b2} blockquote=absent — stale — update both …" });
    } else { advisories.push(…) }
}
```

plus the mirror `(None, Some(b3))` arm. Both block. PC13a's advisory rationale applies verbatim: *"POLICY 3 forces STORY-INDEX (both catalog row and blockquote) to be updated AFTER the primary story write."*

Live reachability, from the F-S2107-P7-002 measurement:

```
catalog rows with NO blockquote entry: 2 [('S-18.11', 'c45c0fc'), ('S-18.12', '345086c')]
```

**Failure scenario.** Any edit to `S-18.11` or `S-18.12` whose frontmatter `input-hash` differs from `c45c0fc` / `345086c` exits 2 with no burst-ordering escape — precisely the self-lock Option 1 was approved to close, surviving in the arms the carve-out did not enumerate.

**Routing.** product-owner answer required: does the directional carve-out extend to the half-present case (advisory, consistent with PC12 and PC13a's rationale) or does PC12 apply only when *both* are absent? The BC must state one.

---

#### F-S2107-P7-009 — HIGH — `red-gate-log.md` has **no pass-6 section and zero `F-P6-*` references**. Four bats gates, a control, a coverage gate, three corpus tests and a guard test were added with no attestation anywhere. The document is still titled "Pass-1 Fix Burst" and the pass-numbering incoherence F-P6-019 named is unchanged.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md` — H1 title; `## Pass-4 Fix Burst — RED GATE Tests (test-writer)` (whose own subtitle says pass-3); `## Amendment 3 — Pass-4 Adversary Findings`; `## Amendment 4 — Pass-4 Adversary Findings: Harness Parity, Wrapper Removal, Cite Refresh`.

**Clause violated:** POLICY 15 ATTESTATION-LOCATION GATE (D-912), SAME-AC GATE AUDIT — OBLIGATION-INDEXED FORM (D-916), NAME-SET-EQUALITY MANDATE (D-918); POLICY 1.

**Evidence — literal shell:**

```
$ grep -c "assertion-site attestation" .../docs/red-gate-log.md
0
$ grep -cE 'F-P6-0[0-9]{2}' .../docs/red-gate-log.md
0
$ head -1 .../docs/red-gate-log.md
# Red Gate Log — S-21.07 Pass-1 Fix Burst (Test-Writer)
$ grep -n '^## ' .../docs/red-gate-log.md | tail -3
 785:## Amendment 2 — BC-5.39.010 v1.9 Fixture Sweep & stale_index_blocks Correction (POLICY 15…
 954:## Amendment 3 — Pass-4 Adversary Findings (RED GATE + Fixture Corrections)
1082:## Amendment 4 — Pass-4 Adversary Findings: Harness Parity, Wrapper Removal, Cite Refresh
```

The file is **not** in `git status --porcelain` — untouched this burst. There is no Pass-5 section either, so F-P6-019's "hole where the pass-5 section should be" is now a two-pass hole.

**Failure scenario.** POLICY 15's gate is explicit: a fix wave adding or strengthening any bats assertion site MUST NOT be pushed until `grep -c 'assertion-site attestation (<HEAD-SHA>)' red-gate-log.md` → 1. It is 0. Beyond the letter: the four new PC2a/PC2b/PC13a/PC13b gates have no recorded mutant/control table, so no artifact states which obligation each gate asserts and which mutant proves it — which is how F-S2107-P7-013's substring-vs-equality gap went unnoticed. D-916's obligation-indexed AC table (one row per AC-022/AC-023 clause) and D-918's name-set-equality diff between story AC Gate cells and the audit table are both absent.

---

#### F-S2107-P7-010 — HIGH — Every corpus test calls the pure Rust functions natively. Nothing exercises the hook through the WASM sandbox at corpus scale, and `on_error = "continue"` makes fuel exhaustion silently indistinguishable from a clean pass. The largest fixture BC-INDEX is 2,383 bytes; the live one is 574,311.

**Location:** `crates/.../src/lib.rs` — all eight `corpus_*` tests (they call `arm_a1::run_arm_a1_with_index_result`, `arm_b::run_arm_b1_with_index_result`, `arm_b::is_volatile_path`, `arm_a1::extract_bc_index_version_state` directly); `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/*`; `plugins/vsdd-factory/hooks-registry.toml` (`on_error = "continue"`).

**Clause violated:** POLICY 11; TD-VSDD-059; production-grade default.

**Evidence — literal shell:**

```
$ find plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence \
    -name 'BC-INDEX.md' -o -name 'STORY-INDEX.md' | xargs wc -c | sort -n | tail -3
    2359 ./b1-hash-match/factory/stories/STORY-INDEX.md
    2383 ./a1-stale-index/factory/specs/behavioral-contracts/BC-INDEX.md
    2592 ./b1-story-index-inconsistent/factory/stories/STORY-INDEX.md

$ wc -c .factory/specs/behavioral-contracts/BC-INDEX.md .factory/stories/STORY-INDEX.md
  574311 specs/behavioral-contracts/BC-INDEX.md
  499750 stories/STORY-INDEX.md

$ grep -c 'VSDD_CORPUS_ROOT\|Documents/GITHUB' plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats
0
```

Read caps are within bounds (`BC_INDEX_MAX_BYTES = 1_048_576` vs 574 KB; `STORY_INDEX_B1_MAX_BYTES = 1_048_576`, `STORY_INDEX_B2_MAX_BYTES = 2_097_152` vs 500 KB), so truncation is not the risk — fuel is. `extract_bc_index_version_state` performs, per line of a 1,983-row / 574 KB file, a `String` allocation (`line.replace("\\|", "\x00")`), a `split('|')` + `map(trim)` + `filter` + `collect::<Vec<&str>>()`, plus a `join("|")` on the matching row. The registry documents the budget assumption — *"max_bytes caps … are sufficient to stay within the 10M-instruction fuel budget at current sizes. Adding fuel_cap without evidence of exhaustion is premature"* — but "current sizes" is asserted, never measured, and no test at any layer could produce the evidence.

**Failure scenario.** `on_error = "continue"` converts fuel exhaustion into `HookResult::Continue` with no violation and no advisory — identical output to a clean corpus. Combined with F-S2107-P7-003 and the 2.4 KB fixture ceiling, **no gate at any layer** distinguishes "validated 1,983 rows, found nothing" from "ran out of fuel on row 200 and returned Continue." That is the F-P6-010 root cause one level up: the corpus tests closed the *shape* gap and left the *execution-environment* gap open.

**Routing.** test-writer: one bats scenario whose fixture BC-INDEX is a copy of the live file (or comparable row count), asserting `plugin.completed` with no `plugin.timeout`.

---

#### F-S2107-P7-011 — HIGH — The compensating guard's match predicate searches the **entire raw row line**. Version chains are cumulative, so it cannot detect index-newer-than-primary or any desync where the frontmatter version appears anywhere in the chain. Its own assertions document the hole. The accompanying NOTE claims the production bugs it works around are unfixed and out of scope — they were fixed in this burst.

**Location:** `crates/.../src/lib.rs` — `bc_index_row_contains_version`, the `NOTE:` paragraph in the `F-P6-010 CORPUS TEST D` comment block, and `test_bc_index_row_contains_version_teeth`.

**Clause violated:** TD-VSDD-059 (compensating guard weaker than the block it replaces); POLICY 15 (false attestation in a code comment); POLICY 13 BOUNDARY-POLARITY MANDATE (domain widened from "field 6 current token" to "anywhere in the row" with no analysis of what the widened domain admits).

**Evidence.** The helper locates the row by `format!("| [{bc_id}](")` prefix, then scans the whole line for literal `v{version}` with a trailing-digit boundary check. It never restricts to the version-chain cell, never identifies the chain's terminal entry, never compares directions. The teeth test states the consequence:

```rust
// v1.23 also found in the row (it's in the note) — test would NOT flag it as missing.
// Only the frontmatter version matters; the test checks frontmatter, not the row's canonical.
assert!(
    bc_index_row_contains_version("BC-3.08.001", "1.23", index_backward_ref),
    "v1.23 is in the row text (it should be findable for reference)."
);
```

**Failure scenario.** Version chains are append-only and cumulative (`v1.10 \| v1.11`; `v1.0 \| … \| v1.18`). The guard fires only when the frontmatter version was **never** appended. It passes for:

- **Index newer than primary** (the PC2b anomaly): append a bogus `\| v1.19` to BC-4.13.001's chain while frontmatter stays `1.18` → `v1.18` still present → PASS. No write-time gate covers this either: Arm A1 fires on BC-file writes and BC-INDEX.md is excluded by the basename guard, so a BC-INDEX-only edit triggers no version comparison at all.
- **Frontmatter rolled back**: set BC-5.39.010 to `1.10` while the chain reads `v1.10 \| v1.11` → PASS.
- **Version present in an annotation but not the chain's current token** — the BC-3.08.001 shape the test itself demonstrates.

So the guard restores a strictly weaker invariant than PC2 v1.10 enforced ("current chain token == frontmatter version"), and the weakening is undocumented in the BC and the story.

Separately, the comment block asserts:

> `NOTE: F-P6-019a–d are pre-existing production bugs in arm_a1 (PC2a/PC2b). They are reported to team-lead and out of scope for S-21.07 test-writer work. This compensating test is deliberately at the CI layer only.`

All four were fixed in this same working tree (`extract_first_v_token_of_last_entry` + the `fields[5..].join("|")` reassembly + `trim_start_matches('v')` in `run_arm_a1_with_index_result`). I traced all four live rows through the shipped extractor; each now yields the correct version. The NOTE is a false statement in the shipped deliverable, and it is the stated justification for the weaker predicate.

---

### MEDIUM

#### F-S2107-P7-012 — MEDIUM — `CHANGELOG.md` cites `BC-5.39.010 v1.11` five times while shipping v1.12, never mentions the v1.12 amendment, and still files a new WASM crate + new registry entry + new 51-test suite under `### Fixed` only. F-P6-015 leg 1 unclosed at its third pass.

**Location:** `CHANGELOG.md` — the `## [Unreleased]` → `### Fixed` entry beginning `**S-21.07 — validate-cross-site-correspondence pass-6 adversarial fix burst** (BC-5.39.010 v1.11, E-21 W4)`.

**Clause violated:** POLICY 5 v1.3.4; production-grade default; S-7.01 (c).

**Evidence — literal shell:**

```
$ grep -o 'BC-5.39.010 v1\.11' CHANGELOG.md | wc -l
       5
$ grep -c 'BC-5.39.010 v1\.12' CHANGELOG.md
0
$ grep -n '^### ' CHANGELOG.md | head -2
9:### Fixed
```

The entry cites "per BC-5.39.010 v1.11" for PC4a, PC2a/PC2b and PC13a/PC13b — all of which v1.12 subsequently re-touched (EC table, Canonical Test Vectors, §Gate Spec prose, VP table). `CHANGELOG.md` **is** modified in `git status --porcelain`, so the sweep touched this file and still left it at v1.11: it updated content without re-reading version cites. There is no `### Added` heading under `[Unreleased]`; a release-notes reader learns fix bursts happened and nothing about the plugin being introduced.

---

#### F-S2107-P7-013 — MEDIUM — AC-022 and AC-023's gates assert three substrings against a multi-record blob rather than the full formatted string — the exact assertion shape v1.11 declared NON-CONFORMING for the sibling clause PC4a. The strengthening was applied to the named clause, not to the class.

**Location:** `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats` — `@test "AC-022 / T-P6A (PC2a): primary-newer-than-index emits advisory, exits 0"` and `@test "AC-023 / T-P6C (PC13a): B2==B3 story-index-consistent-stale emits advisory, exits 0"`, against `@test "F-P4-003 (RowMalformed): 2-field BC-INDEX candidate line emits verbatim advisory, exits 0"` in the same file.

**Clause violated:** BC-5.39.010 v1.11 PC4a rationale generalised; POLICY 5 v1.3.3 SIBLING-SWEEP MANDATE; POLICY 15 SAME-AC GATE AUDIT.

**Evidence.** v1.11's `last_amended`, verbatim: *"PC4a verbatim assertion strengthened (F-P6-002): test-writer MUST assert COMPLETE formatted string by equality check; `.contains()`-only on substrings is NON-CONFORMING."* The PC4a gate does exactly that (`python3` JSON decode of the record, then `[ "$actual_msg" = "$expected_decoded" ]`) — a genuine closure. The two new advisory gates do not:

```bash
warn_line="$(grep '"plugin_name":"validate-cross-site-correspondence"' "$log" \
  | grep '"type":"plugin.log"' | grep '"level":"warn"' || true)"
echo "$warn_line" | grep -q '\[Class A Arm1\] advisory:' || { … }
echo "$warn_line" | grep -q 'primary newer than index' || { … }
echo "$warn_line" | grep -q 'Class A BLOCK suspended' || { … }
```

Two weaknesses. (a) These are the `.contains()`-only assertions the BC just prohibited for the sibling clause; PC4a's rationale — *"it cannot detect an injected sentence, an altered phrase, or a dropped sentence that leaves both target substrings intact"* — applies unchanged. (b) `warn_line` is the concatenation of **all** warn records, so `grep -q` on the blob is satisfied if the three substrings appear in three *different* records. A mutant splitting the advisory across two log calls, or an unrelated arm supplying one substring, passes. The gate does not prove one coherent message exists. Story AC-022/AC-023 describe these as "verbatim assertions", which overstates what is asserted.

---

#### F-S2107-P7-014 — MEDIUM [process-gap] — The new coverage gate counts skip *markers* and `@test` *declarations*, not tests executed. The two `_require_artifacts` skips remain unbounded, and the reported attestation is still "51/51" — the concealing form F-P6-016 named.

**Location:** `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats` — `@test "F-P6-016: exactly 5 Class-D-DEFERRED skips and >=40 test declarations"`; the `skip` calls inside `_require_artifacts`.

**Clause violated:** POLICY 15; CI-as-Code positive-coverage axis; TD-VSDD-059.

**Evidence — literal shell:**

```
$ grep -c '^@test' plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats
51
$ grep -n '^\s*skip ' plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats
113:    skip "factory-dispatcher binary not found — run: cargo build --release -p factory-dispatcher"
122:    skip "validate-cross-site-correspondence.wasm not staged — see story S-21.07 AC-021"
880,898,937,972,995:  skip "[DEFERRED v1.6 — Class D] …"
```

The gate asserts `skip_count -eq 5` over `skip "\[DEFERRED v1\.6 — Class D\]` and `test_count -ge 40` over `^@test `. Both remain satisfied under the exact regression the finding was raised about: if `_require_artifacts` begins skipping (missing dispatcher, unstaged WASM, non-darwin-arm64 host without `CI_REQUIRE_ARTIFACTS=1`), 46 tests become `ok N # skip`, the marker count is still 5, the `@test` count is still 51, the gate is GREEN, and bats reports `51/51`. F-P6-016's ask was a runtime-derived `N executed, M skipped` line; it does not exist. Correct attestation for this tree: **46 passed / 5 skipped (Class D DEFERRED) / 0 failed**, contingent on dispatcher + WASM presence.

---

#### F-S2107-P7-015 — MEDIUM — The `F-P6-019-GUARD` escape is `line.contains("trim_start_matches")` — a content-token wildcard, not the named-site list the comment claims. No mutant proves the guard fires on a raw caller that normalises wrongly.

**Location:** `crates/.../src/lib.rs` — `test_F_P6_019_guard_no_raw_version_field_access_in_production_code` and its `EXCLUSION LIST` comment block.

**Clause violated:** POLICY 13 ESCAPE-SCOPE-PARITY MANDATE (escape unit broader than trigger unit ⇒ positive + negative-twin controls required); POLICY 15 PER-GUARD MUTANT-VERIFICATION MANDATE; POLICY 15 (comment asserts a property the code does not implement).

**Evidence.** The comment: *"EXCLUSION LIST (closed; every legitimate raw call must be named here) … Entry 1 — frontmatter.rs: the `extract_version_field` wrapper body … Detected by: line also contains `trim_start_matches`. Zero other entries. If a new legitimate raw caller is added without updating this list, this test fails."* The implementation:

```rust
if line.contains("trim_start_matches") {
    continue;
}
```

No file check, no function check, no `extract_version_field` check. The trigger is per-line-and-two-tokens (`extract_frontmatter_field` ∧ `"version"`); the escape is per-line-and-one-token. Any new production line satisfying the trigger *and* mentioning `trim_start_matches` is silently exempt — including `extract_frontmatter_field(c, "version").map(|s| s.trim_start_matches('x').to_string())`, which normalises the wrong character and defeats the F-P6-019 class. The claim "if a new legitimate raw caller is added without updating this list, this test fails" is false for that whole family. The guard has one positive control (pre-fix `lib.rs` RED, post-fix GREEN); POLICY 13's negative twin — the escape token in a non-wrapper raw caller, attested RED — does not exist and would currently be GREEN. The documented multiline blind spot compounds it: a two-line raw call is invisible to both trigger and exclusion.

---

#### F-S2107-P7-016 — MEDIUM — The story's `modified[]` array lists v1.6 before v1.5. Class E2's date-only predicate cannot detect a version-token decrease when dates are equal, so this BC's own gate is blind to the ordering defect in the story it governs.

**Location:** `.factory/stories/S-21.07-validate-cross-site-correspondence.md` frontmatter `modified:`; `BC-5.39.010.md` PC38 / postconditions 21-22; `arm_e::run_arm_e2`.

**Clause violated:** POLICY 14 leg 3; POLICY 17.

**Evidence.** Verbatim:

```yaml
  - "2026-08-04 v1.4: BC-5.39.010 v1.6→v1.7 propagation (POLICY 8; pass-4 fix burst)"
  - "2026-08-05 v1.6: BC-5.39.010 v1.10→v1.11 propagation (POLICY 8; pass-6 fix burst); …"
  - "2026-08-05 v1.5: BC-5.39.010 v1.7→v1.10 propagation; ARCH-INDEX.md removed from inputs:; …"
  - "2026-08-05 v1.7: BC-5.39.010 v1.11→v1.12 propagation (POLICY 8; story-writer)"
```

Version order: 1.0, 1.1, 1.2, 1.3, 1.4, **1.6, 1.5**, 1.7. The body `## Changelog` table is correctly ordered 1.7 → 1.0, so the two representations disagree. PC38 strips annotations and compares dates only: `["2026-08-04", "2026-08-05", "2026-08-05", "2026-08-05"]` is non-decreasing and equal dates are explicitly PERMITTED → E2 Continues. The same self-blindness the BC's v1.3 Changelog confessed for `modified[]`↔Changelog correspondence, now on a second axis: **version-token ordering within same-day entries is ungated**. This is the inverse of pass-5's F-P6-011 (ADR-037's date-decreasing array, which E2 *would* catch had ADRs been in PC34's scope) — the class recurs at a sibling site through a mechanism E2 cannot see.

**Routing.** story-writer: reorder. product-owner: decide whether PC38 should use the annotated version token as a secondary key when dates are equal.

---

#### F-S2107-P7-017 — MEDIUM — PC5's load-bearing corpus figure is wrong: the escape-aware split does **not** yield "exactly 6" fields, and the population is 39 six-field rows plus one nine-field row, not "40 six-field".

**Location:** `BC-5.39.010.md` §Preconditions PC5 — the "Escape-aware splitting is required" paragraph, the `RowPresentNoVersion` / `Version(v)` bullets, the corpus-verification block (*"5-field rows: 1943 / 6+-field rows: 40 / total: 1983"*), and the v1.8 Changelog row.

**Clause violated:** POLICY 15 / D-950 corpus-count discipline; POLICY 5 SDK-grounding.

**Evidence — literal shell (escape-aware split, locator-matched rows only):**

```
field-count histogram: {5: 1943, 6: 39, '7+': 1}
total locator-matched: 1983
rows with >6 non-empty fields: 1
    BC-4.13.001 9
```

`BC-4.13.001`'s row contains an unescaped `|` inside the v1.16 annotation `^(Edit|Write|MultiEdit|Agent)$`, producing nine non-empty fields. The aggregate "40 rows in `Version(v)` state" survives (39 + 1 = 40 rows reach the `n >= 6` arm), which is why the arithmetic looked consistent, but two normative claims are false: the split does not yield exactly 6, and the 6-field count is 39. This is the same measurement that motivated the shipped `fields[5..].join("|")` reassembly — so the BC simultaneously denies the phenomenon exists and omits the mechanism the code added for it (F-S2107-P7-004).

---

#### F-S2107-P7-018 — MEDIUM — ADR-037's 77-story roster is derived by prefix `grep`, not by `is_volatile_path`. The two disagree by one: `S-8.10`'s `inputs:` entry is `ARCH-INDEX.md:75` — a line-pinned path the production predicate rejects. The story is enumerated in the remediation scope but is not protected by PC40.

**Location:** `ADR-037-input-hash-stable-input-constraint-volatile-artifacts-excluded.md` §Context corpus re-derivation block (the six `grep -rl` commands and the "De-duplicated union: 77 stories total" line); `.factory/stories/S-8.10-sdk-extension-write-file.md` `inputs:` array; `arm_b::is_volatile_path`.

**Clause violated:** POLICY 22 (roster derived by a proxy predicate, not the load-bearing one); POLICY 5 v1.3.4; TD-VSDD-091 (line-number pin in a load-bearing field).

**Evidence — literal shell:**

```
$ grep -rn '^  - \.factory/specs/architecture/ARCH-INDEX\.md.' .factory/stories/
S-8.10-sdk-extension-write-file.md:18:  - .factory/specs/architecture/ARCH-INDEX.md:75
```

Re-derived with production predicate semantics (`is_volatile_path` over frontmatter-`inputs:`-parsed paths):

```
frontmatter-inputs ARCH-INDEX stories: 61     (ADR claims 62)
volatile-union stories:                76      (ADR claims 77)
epics with volatile inputs:             5
```

The ADR's own command yields 62 because it is a prefix match; `is_volatile_path(".factory/specs/architecture/ARCH-INDEX.md:75")` returns `false` — the `.factory/cycles/` branch does not apply and the three-way path equality fails.

**Failure scenario.** (1) `S-8.10` is counted in the ADR's 77-story remediation scope and in every blast-radius figure, but PC40 does not suppress Class B for it — if its three-way equality is violated it blocks with no volatile carve-out, contrary to what the ADR asserts. (2) `compute-input-hash` cannot resolve `ARCH-INDEX.md:75` as a file, so `S-8.10`'s stored hash `6fb36eb` has questionable provenance. ADR-037 v1.2 re-ran the derivation *this burst* and did not catch it, because the derivation predicate is not the gate predicate — the POLICY 22 pattern applied to a roster instead of a report.

**Routing.** architect: re-derive using `is_volatile_path` semantics (union 76, ARCH leg 61) and record the S-8.10 line-pin as a distinct TD-VSDD-091 defect. story-writer: correct `S-8.10`'s `inputs:` entry.

---

#### F-S2107-P7-019 — MEDIUM — The committed `HEAD`'s D-693 attestation names a WASM artifact that no longer exists. The deployed `.wasm` is 231,121 bytes; the attestation says 226,794. The shipped binary is uncommitted and unattested.

**Location:** commit `b78b27ef` subject line; `plugins/vsdd-factory/hook-plugins/validate-cross-site-correspondence.wasm` (modified, uncommitted).

**Clause violated:** D-693 (WASM pre-green gate); POLICY 15; POLICY 22.

**Evidence — literal shell:**

```
$ git log --oneline -1
b78b27ef build(S-21.07): deploy WASM v1.10 — 226,794 bytes (D-693 pass-5 gate PASS)
$ ls -la plugins/vsdd-factory/hook-plugins/validate-cross-site-correspondence.wasm
-rwxr-xr-x@ 1 zious staff 231121 Aug  5 20:30 …/validate-cross-site-correspondence.wasm
$ ls -la target/wasm32-wasip1/release/validate-cross-site-correspondence.wasm
-rwxr-xr-x@ 1 zious staff 231121 Aug  5 20:30 …/validate-cross-site-correspondence.wasm
```

The staged artifact matches a fresh release build (both 231,121 bytes, both 20:30), so the bats suite **is** exercising pass-6 logic — I confirm that. What is missing is attestation: the only D-693 record on this branch describes a 226,794-byte v1.10 artifact, 4,327 bytes smaller, and any burst-log or PR body carrying that figure forward misdescribes what ships. Compounded by F-S2107-P7-001, no SHA records the deployed WASM's size, hash, or provenance.

---

### LOW

#### F-S2107-P7-020 — LOW — The majority-shape corpus test's own vacuity guard reads raw frontmatter and compares to `"1.0"`, so a `version: "v1.0"` BC would defeat it; its assertion message cites a superseded clause version.

**Location:** `crates/.../src/lib.rs` — `test_BC_5_39_010_corpus_arm_a1_row_present_no_version_cell_majority_shape`.

**Clause violated:** F-P6-019 normalization-asymmetry class (residual, test layer); POLICY 5 v1.3.5 Part A (live cite).

**Evidence.** The test calls `frontmatter::extract_frontmatter_field(&bc_file_str, "version")` (raw — permitted, since the guard truncates at `#[cfg(test)]`) then:

```rust
assert_ne!(
    bc_version, "1.0",
    "BC-1.01.001 must have version != '1.0' for the RowPresentNoVersion test to \
    distinguish from the RowAbsent v1.0 advisory path"
);
```

`BC-7.03.079` and `BC-5.24.006` demonstrate `version: "vN.M"` occurs in this corpus. If `BC-1.01.001` adopted `version: "v1.0"` the guard would pass (`"v1.0" != "1.0"`) while production — which strips the prefix in `run_arm_a1_with_index_result` — would take the v1.0 RowAbsent-advisory branch, exactly the confusion the guard exists to prevent. `frontmatter::extract_version_field` makes it consistent with production. The message also cites `BC-5.39.010 v1.10 PC5` (subset of F-S2107-P7-005; called out because it sits inside the vacuity guard's own rationale).

---

## Part B — Observations

- **O-P7-01** — `arm_a1::extract_bc_index_version_state`'s `non_empty_fields[5..].join("|")` bound: confirmed as documented. `BC-4.13.001` is the only live row with >6 non-empty fields (9), and the join + `\x00`-split + first-token path yields the correct `1.18`. Forward hazard: a bare `|` in a field **before** the version cell (Title/Status/Capability/Stories) shifts indices so the join window opens mid-title; if the chain has no `\x00` separators the first v-token could come from title prose. Zero corpus instances, no test. Cheap to close with a unit vector carrying a bare pipe in the Title cell.

- **O-P7-02** — `BC-INDEX.md` is 574,311 bytes against `BC_INDEX_MAX_BYTES = 1_048_576` (55% of cap) and grows monotonically with every BC registration. `STORY-INDEX.md` is 499,750 against 1 MiB (B1) / 2 MiB (B2). No action now; worth a size-budget row alongside the `lessons.md` budget (D-442(e)), since silent truncation at the cap is the META-LEVEL-24 false-green class PC4 was written to prevent.

- **O-P7-03** — F-P6-013 is genuinely closed. `_is_strip` is gone from `collect_block_scalar_body`, the docstring's self-contradicting clip bullet is replaced with an explicit statement that the distinction is irrelevant at the extraction layer, and `test_..._frontmatter_folded_multi_line_space_joined` + `test_..._frontmatter_folded_blank_line_paragraph_break` now execute the previously-dead folded branch. Both GREEN in my run.

- **O-P7-04** — F-P6-001's two named legs are genuinely closed on live data. Arm A1 on `BC-5.39.010`: escape-aware split → 6 fields → chain `v1.10 \x00 v1.11` → last entry → `1.11` vs frontmatter `1.12` → `primary_newer` → PC2a advisory, no violation. Arm B1 on `S-21.07`: frontmatter `1bc3197` = catalog `1bc3197` = blockquote `1bc3197`. Both corpus tests pass. F-P6-020 (PC40 advisory prefix), F-P6-008's `bats:38` header and the `hooks-registry.toml` leg, F-P6-017 (dispatcher provenance, no operator-cache fallback), F-P6-023 (arm counts) and F-P6-003 (T-047 discrimination via the new control) are also closed. The gap is that closures stopped at the sites the findings named.

- **O-P7-05** — The PC4a full-equality gate extracts the **first** `level: warn` `plugin.log` record and compares it. If another arm logged a warn advisory earlier in the same invocation, the comparison targets the wrong message. The `a1-row-malformed` fixture is E1/E2-clean today so behaviour is correct now. Filtering on the `[Class A Arm1]` prefix before extraction removes the coupling.

- **O-P7-06** — `is_volatile_path`'s predicate asymmetry (carried from O-P6-04): `contains(".factory/cycles/")` for patterns 2–5 but exact path equality for patterns 1 and 6–8. PC40's table does say "path equals … exactly" for those four, so it is spec-conforming. It is now demonstrably reachable as a false-negative: `S-8.10`'s `ARCH-INDEX.md:75` (F-S2107-P7-018) is one character-class away from matching, and any caller passing an absolute path would match a cycle log but not `STATE.md` or the three indexes.

---

## Part C — Analysis

### Verdict: **NOT-CLEAN** — 20 findings (4 BLOCKER / 7 HIGH / 8 MEDIUM / 1 LOW). Streak remains **0/3**. Trajectory `47 → 18 → 25 → 25 → 24 → 20`.

### Gate claims — independently executed

| Claim | Verdict | Evidence |
|---|---|---|
| cargo 143 / 1 / 17 | **VERIFIED** | `cargo test -p validate-cross-site-correspondence` → `143 passed; 1 failed; 17 ignored`. Sole failure `test_BC_corpus_version_sync_all_indexed_bcs_match_frontmatter`: `BC-5.39.010: BC frontmatter version="1.12" (normalized="1.12") not found in BC-INDEX row`. Sweep summary: 1984 files scanned, 40 Version-state assertions, 1 RowAbsent, 1943 RowPresentNoVersion, 0 RowMalformed. |
| "resolves when state-manager advances BC-INDEX at Commit D" | **VERIFIED as to mechanism; framing understates two defects** | The row reads `v1.10 \| v1.11`; appending `\| v1.12` clears it. But the row was already edited this burst without an index-version bump (F-S2107-P7-007), and the test never runs in CI (F-S2107-P7-003) — so "resolves at Commit D" is true locally and vacuous in the gate that matters. |
| cargo fmt clean | **VERIFIED** | `cargo fmt --check --all` → exit 0, no output. |
| cargo clippy clean | **VERIFIED** | `cargo clippy -p validate-cross-site-correspondence --all-targets -- -D warnings` → `Finished dev profile`, no diagnostics. |
| bats 51/51 | **NOT VERIFIED; form misleading** | Not executed (read-only; no dispatcher/WASM staging). Static: 51 `@test`, 5 unconditional Class-D skips → 46 executable. Correct form: `46 passed / 5 skipped / 0 failed`. See F-S2107-P7-014. |

### Answers to the six questions

**1. Is the Option 1 carve-out safe?** No — one genuine defect can now pass silently. The write-time block for BC/BC-INDEX version desync is gone; the commit-time substitute is (a) inert in CI (F-S2107-P7-003) and (b) strictly weaker even when it runs, because it searches the whole raw row and version chains are cumulative — it fires only when the frontmatter version was never appended, and cannot see index-newer-than-primary at all (F-S2107-P7-011). Its `RowAbsent` / `RowPresentNoVersion` / `RowMalformed` branches **are** correct skips, and the `checked_count >= 5` plus `>= 100` file floors are real anti-vacuity guards. The gaps are the two above, not the branch dispositions.

**2. Sweep completeness — the missed sites.** Each of the four sweeps missed sites in the same pattern: the named site is fixed, the class is not. v1.11→v1.12 hit `Cargo.toml`, `hooks-registry.toml`, `bats:38` and the story, and missed ~53 cites in `crates/` including `lib.rs`'s module header and `main.rs` (F-S2107-P7-005). F-P6-009 fixed 2 of 4 sites (F-S2107-P7-006). The corpus-test sweep covered A1 and B1 and skipped B2, where the live corpus is red (F-S2107-P7-002). PC4a's verbatim strengthening was applied to PC4a and not to the two sibling advisory clauses added in the same burst (F-S2107-P7-013).

**3. Live-vs-frozen classification.** One class right, one wrong. **Correct:** the `[DEFERRED v1.6 — Class D]` markers, `hooks-registry.toml`'s `v1.1 §Gate Spec had MultiEdit omitted — v1.2 corrects` narrative, and the story's `last_amended` / `modified[]` / body-Changelog cites — all inside POLICY 5 v1.3.5 Part A's five enumerated classes. **Wrong:** ~53 Rust doc comments and assertion messages left at v1.3 / v1.6 / v1.10 citing clauses still live at v1.12, three of which (PC4a, PC5, PC40) were amended after the version cited. No genuine history was wrongly advanced; the error is entirely one-directional.

**4. New tests' teeth.** Mixed. **Real teeth:** the PC4a full-equality gate (a genuine F-P6-002 closure); `test_bc_index_row_contains_version_teeth` (perturbation pair + digit-boundary + absent-ID); `T-047-CONTROL` (same `B2≠B3` data minus the volatile input → exit 2 — this does prove PC40 is the discriminator rather than the `(None, None)` arm); the corpus tests' loud shape invariants (`volatile_found.is_empty()`, `input-hash` present, `checked_count >= 5`, `>= 100` BC files). **Insufficient:** the guard's token-wildcard exclusion with no negative twin (F-S2107-P7-015); the F-P6-016 gate measuring the wrong quantity (F-S2107-P7-014); AC-022/AC-023's substring-over-blob assertions (F-S2107-P7-013). And every corpus test, teeth or not, is inert in CI (F-S2107-P7-003).

**5. The advisory paths.** PC2a's and PC13a's shipped strings match their BC prescriptions clause-for-clause, and the ACs match the BC's normative text. What is not asserted is the *whole* string — see (4) and F-S2107-P7-013.

**6. Known-unresolved items.** `arm_a1`'s `join("|")` bound: confirmed, currently unreachable, untested (O-P7-01). `lib.rs`'s test guard comparing raw frontmatter to `"1.0"`: confirmed live (F-S2107-P7-020). WASM fuel: could not measure read-only, but I established something stronger and worse — **no test at any layer exercises this hook through the WASM sandbox above 2.4 KB of index data**, and `on_error = "continue"` makes exhaustion indistinguishable from clean (F-S2107-P7-010).

### Why the count fell to 20, and what that does and does not mean

The drop from 24 is real work. Seven pass-5 findings are closed at class level and I could not reopen them: F-P6-002, F-P6-013, F-P6-020, F-P6-017, F-P6-023, F-P6-003, and the two named legs of F-P6-001. The six normalization/extraction code defects are correctly fixed; I verified all four live rows by hand.

But the structure of what remains got worse in one specific way. Pass-5 characterised the dominant pattern as *same-burst cross-artifact incoherence* — four editors not re-reading each other. That is largely fixed: story, BC, ADR and code are now mutually consistent about PC2/PC13 semantics. What replaced it is narrower and harder: **fix-the-named-site-not-the-class**, now measurable at five independent sites in one burst (F-S2107-P7-002/005/006/013 and the F-P6-016 gate). Three of the four BLOCKERs are instances of it — B2 skipped while A1 and B1 were covered; the compensating guard built but never wired into CI; the BC amended at every clause a finding named and not at the clause the code silently diverged from.

One finding sits outside the taxonomy: **nothing is committed.** Every figure here, including the ones I verified myself, describes a working tree. I recorded `reviewed_head: b78b27ef` because that is what `git rev-parse` returns, but it is the SHA pass-5 reviewed and contains none of this burst's work. Until F-S2107-P7-001 is closed, no attestation in this cascade is replayable and a single tree-clearing command ends the burst — the D-946 failure mode, one keystroke away.
