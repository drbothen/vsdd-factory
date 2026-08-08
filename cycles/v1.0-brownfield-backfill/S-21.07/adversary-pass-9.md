---
pass: 9
verdict: NOT-CLEAN
reviewed_head: 67ffbdcc
reviewed_heads_additional: "fix/nested-factory-path-derivation @ 09f052a9; fix/d999-sentinel-code-migration @ bf642fd9; diff base origin/develop @ 700b4dd3"
factory_artifacts_head: 0a6c8fda
novelty: 0.75
previous_review: "cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-7.md"
---

## Summary

Pass-9 fresh-context adversarial review. **8 findings: B0 / H3 / M3 / L1 / NIT1.** Trajectory `47→18→25→25→24→20→16→8`. Streak: **0/3** (BC-5.39.001 reset — NOT-CLEAN).

**Reviewed SHAs (stated explicitly):**

| Tree | SHA |
|---|---|
| `feature/S-21.07-validate-cross-site-correspondence` (worktree `.worktrees/S-21.07`) | **`67ffbdccda5302a4e1fbffd8b2f2b8bdd0aed3ce`** (clean tree; 1 ahead of origin `37022ecc`) |
| `fix/nested-factory-path-derivation` (main worktree) | **`09f052a9`** |
| `fix/d999-sentinel-code-migration` (worktree `.worktrees/d999-migration`) | **`bf642fd9`** |
| diff base `origin/develop` | **`700b4dd32251bc9ce40dc59bed8cc7441a9afcb0`** |
| `factory-artifacts` (worktree `.factory`) | **`0a6c8fda`** |

**Tooling disclosure (POLICY 22):** I had Bash and used it. Every figure below is captured stdout from execution at the SHAs above. Where I initially mis-derived something with my own parser, I re-derived it and dropped the candidate finding rather than reporting it — two candidates were discarded this way (see Part C).

### The three re-verified findings

- **F-S2107-P8-007 — GENUINELY CLOSED.** The fabricated `echo` is gone. No pass/fail figure is synthesized anywhere in the suite; the only arithmetic left is over structural declaration counts and carries no `passed`/`failed` words. `run-all.sh` TAP accounting is genuinely runtime-derived and propagates suite exit codes. I re-ran the full suite: **`Coverage: 2198 executed, 51 skipped (2249 total across 254 suites)` / `All tests passed` / 0 `not ok` lines / exit 0** — the attested figures reproduce exactly.

- **F-S2107-P8-013 — CLOSED as specified, but the measurement it enabled surfaces a worse fact.** The fixture is genuinely production-scale (sha256-identical to the live `BC-INDEX.md`, 576,396 bytes, 1,985 rows) and the assertion genuinely can fire: `TimeoutCause::Fuel` maps to `PLUGIN_TIMEOUT` with no `plugin.completed`, and `_assert_plugin_ran_not_crashed` rejects that. I proved the whole chain end-to-end against the real plugin. **But the live corpus now consumes 99.21% of the fuel budget and exhausts ~3–4% above current size** — see F-S2107-P9-002.

- **F-S2107-P8-006 — primary fix CLOSED; its mutant attestation is FALSE.** The helper is genuinely deleted (`grep 'fn bc_index_row_contains_version'` → exit 1) and the comparison is genuinely terminal-value-based. But the three "bypass mutant" tests exercise only `arm_a1::extract_bc_index_version_state`, which is **byte-identical** between the pre-fix and post-fix trees. I injected the three tests verbatim into `37022ecc` — where the weak helper was live *and* was the comparison the corpus gate used — and **all three passed**. The comments claiming they "FAILED against the previous `bc_index_row_contains_version` helper" are false, and the strengthening is unpinned. See F-S2107-P9-001.

### What genuinely landed, verified by execution

`fmt` exit 0, `clippy -D warnings` exit 0 (zero diagnostics), and `cargo test --workspace --all-targets` exit 0 on **all three** branches — S-21.07 **2432 passed / 0 failed / 22 ignored**; d999-migration **2265/0/5**; nested-factory **exit 0**. Plugin-scoped: **167 passed / 0 failed / 17 ignored**. bats `validate-cross-site-correspondence.bats`: **52 declared, 47 executed, 5 skipped, 0 failed, exit 0** — the pass-7 BLOCKER (`AC-023 / T-P6C`) is fixed and green. **D-693 WASM gate genuinely PASSES**: the deployed artifact is sha256-identical to a fresh `wasm32-wasip1 --release` build at HEAD (`853c802e…`, 231,661 bytes) despite four src-touching commits postdating the WASM commit — the cite sweeps were comment-only, as claimed.

**F-S2107-P8-004 is genuinely closed.** Pass-7 measured 88 stale live `.rs` cites. At v1.14 the same predicate over `src/*.rs` returns **5**, and I audited every non-v1.14 line without the exemption filter: all 16 are either `[DEFERRED v1.6 — Class D]`-frozen or literal test-fixture strings. The line-scoped `grep -v` is **not** masking a live authority cite. 100 of 116 src cites are now v1.14.

**ADR-041's ALLOCATOR-CEILING GATE is sound and its self-tests are real.** I ran the policies.yaml predicate verbatim (`PASS: global max D-961 < D-9000 ceiling`), reproduced all four self-tests independently, confirmed both legs match real corpus forms (365 heading + 379 table-cell matches), confirmed the glob covers both `decision-log.md` files, and confirmed fail-closed on both empty-file and absent-corpus. I hunted for an uncovered allocation form and found none — every line-initial `D-NNN` outside the two legs is `### Summary` prose. ADR-041 §Decision 4 is honored: zero `D-99999` tokens in STATE.md.

**Sentinel migration is complete.** `git grep -nE 'D-999([^0-9]|$)'` over `bf642fd9` returns **zero** matches. No un-migrated normative site; no inverse error found.

**F-S2107-P8-016's sweep is complete.** All three claimed sites are guarded via one shared `is_dot_factory_basename` helper, with both-polarity tests. I swept every `.join(".factory")`/`push(".factory")` in `crates/`: the only other production derivations are `log_dir::resolve_log_dir_from_params` (already guarded at Level C, and Level D's `walk_up_to_factory` checks `start` itself, so Level E cannot be reached with a `.factory` cwd) and `hook_sdk::path_util` (the new shared guard). No remaining unguarded site.

**4-index + epic parity holds exactly.** BC v4.52 / VP v2.76 / STORY v4.289 / ARCH v3.47 / policies v1.4.21, each with matching `last_amended` version token. E-21 epic `v1.9`, STORY-INDEX H2 cites `v1.9`, epic lists 11 stories, and STORY-INDEX points sum to **83** — both claims exact. **sprint-state.yaml ordering holds**: S-21.10/S-21.12 (no deps → depth 1) sit in the depth-1 zone, S-21.11 (deps S-21.10) and the moved S-21.06 (deps S-21.01) sit in the depth-2 zone, and both zones are lexicographic-ascending. `sprint-state-format.bats` T-12 genuinely compares every entry's status to its STORY-INDEX catalog row and T-14 independently recomputes full-graph depth — 14/14 green.

**POLICY 21: no new `.sh` file on any of the three branches.** The only `.sh` in any diff is the grandfathered `run-all.sh`, and it is unchanged since the ruling (absent from `67ffbdcc`'s diff) — scope has not grown further.

### The dominant pattern this pass

Six of eight findings are still **the gate whose predicate is narrower than the claim it makes** — but the shape has shifted. The predicates are now largely correct; what is narrower than the claim is the **attestation layer**: a mutant set that proves the wrong proposition (P9-001), a budget gate with no margin term and a frozen fixture (P9-002), a policy gate that became satisfiable and was then not satisfied (P9-003), and a "verbatim copy-in" that is paraphrased (P9-004). Notably, the burst's own lesson `L-BB-gate-narrower-than-its-claim` names this class exactly — and P9-001 is a fresh instance authored in the same session that codified it.

---

## Part A — Findings

### HIGH

#### F-S2107-P9-001 — HIGH — The three F-S2107-P8-006 "bypass mutant" tests are vacuous with respect to the fix: all three PASS against the pre-fix tree where the weak helper was live and in use. The claim that they were RED against `bc_index_row_contains_version` is false by execution, and the strengthening is therefore unpinned.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/src/lib.rs` — `test_bypass_1_index_newer_than_primary_detected_as_mismatch`, `test_bypass_2_annotation_rollback_detected_as_mismatch`, `test_bypass_3_chain_rollback_detected_as_mismatch`, and their shared preamble comment block (`BYPASS MUTANT TESTS (F-S2107-P8-006 fix attestation)`); same class in `test_corpus_version_sync_gate_teeth`. Against `test_BC_corpus_version_sync_all_indexed_bcs_match_frontmatter`, the gate they purport to pin.

**Clause violated:** POLICY 15 per-guard mutant-verification mandate (a guard not backed by a mutant that actually fires is a POLICY 15 HIGH); POLICY 11 `no_test_tautologies`; TD-VSDD-059 (self-disclosure of closure is not authoritative; a comment asserting a property the code does not have); D-449(a)/META-LEVEL-24.

**Evidence — literal shell.** The primary fix is real; I state that first:

```
$ cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07
$ grep -rn "fn bc_index_row_contains_version" --include='*.rs' .
$ echo "EXIT_FN=$?"
EXIT_FN=1
```

The mutants call only the extractor, and the extractor did not change in the fix commit:

```
$ git diff --stat 37022ecc..67ffbdcc -- crates/hook-plugins/validate-cross-site-correspondence/src/arm_a1.rs
$ echo "ARM_A1_DIFF_EXIT=$?"
ARM_A1_DIFF_EXIT=0
```

(empty diff — `extract_bc_index_version_state` is byte-identical across the fix.)

The weak helper was live *and* was the comparison the corpus gate used at the parent commit:

```
$ git show 37022ecc:crates/hook-plugins/validate-cross-site-correspondence/src/lib.rs | grep -n "bc_index_row_contains_version("
1771:    fn bc_index_row_contains_version(
...
2021:                            if !bc_index_row_contains_version(&bc_id, normalized_fv, &bc_index_str)
```

I extracted the three mutant functions **verbatim** from HEAD, injected them into the `mod tests` of the pre-fix tree at `37022ecc` (helper still present, confirmed), and ran them:

```
$ git worktree add --detach /tmp/adv9-prefix 37022ecc
HEAD is now at 37022ecc test(S-21.07): DEFENSIVE-arm pinning test + Pass-8 attestation correction
$ python3 ... # inject the 3 functions verbatim
injected OK; helper still present: True
$ cd /tmp/adv9-prefix && cargo test -p validate-cross-site-correspondence test_bypass_
running 3 tests
test tests::test_bypass_3_chain_rollback_detected_as_mismatch ... ok
test tests::test_bypass_1_index_newer_than_primary_detected_as_mismatch ... ok
test tests::test_bypass_2_annotation_rollback_detected_as_mismatch ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 181 filtered out
```

**3 passed, 0 failed** — against the exact tree the comment says they were RED on. The comment's claims are verbatim:

> `// These all FAILED against the previous bc_index_row_contains_version helper (demonstrated RED above; helper deleted per F-S2107-P8-006).`
> `/// This test was RED against bc_index_row_contains_version (helper returned true).`

**Why it matters.** The mutants assert a property of the *extractor* (`Version("1.19") != "1.18"`), which was already correct at v1.13 before the fix. The defect F-S2107-P8-006 identified was never in the extractor — it was that the corpus gate **discarded** the extracted value (`Version(_)`) and called the helper instead. No test in the suite distinguishes the two comparison mechanisms, so nothing fails if a future edit reintroduces a whole-row substring comparison: the corpus gate is GREEN on a synced corpus under either semantics. F-S2107-P8-006's routing asked specifically to "retain the three vectors above as bypass mutants **so the strengthening is pinned**"; the vectors were retained and the pinning was not achieved. This is the burst's own `L-BB-gate-narrower-than-its-claim` lesson instantiated by the burst that codified it — the third such self-instantiation recorded in this cascade.

**Failure scenario.** A refactor reintroduces `row.contains(&format!("v{version}"))` inside the `Version(_)` arm. `cargo test -p validate-cross-site-correspondence` stays green (167/0/17), all three bypass mutants stay green, the teeth test stays green, and all three bypass vectors the adversary demonstrated by execution become live again — with a permanent test suite whose comments assert they cannot.

**Routing.** test-writer: make the mutants exercise the **gate**, not the extractor — extract the comparison into a named function (e.g. `index_matches_frontmatter(&BcIndexVersionState, &str) -> bool`), call it from the corpus gate, and assert `false` for each of the three vectors. Then re-run the injection experiment above against `37022ecc` and confirm the mutants now go RED, capturing that stdout. Correct or delete the four false RED-state claims in the comments regardless of which mechanism is chosen.

---

#### F-S2107-P9-002 — HIGH — The fuel budget at production scale is **99.21% consumed** (9,921,105 of 10,000,000). Exhaustion occurs ~3–4% above the current index size, and on exhaustion the dispatcher exits 0 with empty stdout. BC-5.39.010 v1.14's normative claim that the `max_bytes` caps are "calibrated to bound reads inside the fuel budget" is falsified: the 1 MiB PC4 cap permits a file ~74% past the exhaustion point.

**Location:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md` §Gate Spec — the `on_error = "continue"` rationale paragraph ("The `max_bytes` caps in PC4 (1 MiB), PC10/12/15/35 (512 KiB), and PC19/23/29 (2 MiB) are calibrated to bound reads inside the fuel budget at current artifact sizes") and the **`fuel_cap` field** paragraph ("A future implementer MUST NOT add a `fuel_cap` entry without first confirming that `max_bytes` caps are genuinely insufficient"); `plugins/vsdd-factory/hooks-registry.toml` — the `validate-cross-site-correspondence` entry's `No fuel_cap field:` rationale comment; `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats` — `F-S2107-P8-013: production-scale BC-INDEX (576KB) …` and its fixture `a1-production-scale`.

**Clause violated:** POLICY 5 SDK-GROUNDING MANDATE + v1.3.6 HEAD-REPRODUCIBILITY (a normative calibration claim that does not hold at HEAD); POLICY 13 BOUNDARY-POLARITY MANDATE (a budget boundary asserted without the excluded-region analysis); TD-VSDD-059; production-grade default.

**Evidence — literal shell.** First, the gate as shipped is real and non-vacuous, which is why I credit F-S2107-P8-013 as closed. The fixture is byte-identical to the live index:

```
$ shasum -a 256 .worktrees/S-21.07/plugins/.../a1-production-scale/factory/specs/behavioral-contracts/BC-INDEX.md .factory/specs/behavioral-contracts/BC-INDEX.md
081957bbca7c7d3d0703ad569a06e61c7cd602bdd6113cf4e7ee5ac3495120bc  .../a1-production-scale/.../BC-INDEX.md
081957bbca7c7d3d0703ad569a06e61c7cd602bdd6113cf4e7ee5ac3495120bc  .factory/specs/behavioral-contracts/BC-INDEX.md
$ grep -cE '^\| \[?BC-' .../a1-production-scale/.../BC-INDEX.md
1985
$ bats --tap -f "F-S2107-P8-013" validate-cross-site-correspondence.bats
ok 1 F-S2107-P8-013: production-scale BC-INDEX (576KB) — plugin.completed present, plugin.timeout absent
```

The fixture also genuinely exercises the comparison (BC frontmatter `1.14`; INDEX last chain entry `v1.14`), so it is a real full-index happy-path scan, not an early return.

Now the measurement the gate does not make. I ran the real dispatcher against that fixture and read `fuel_consumed` from the `plugin.completed` record:

```
$ VSDD_LOG_DIR=$W/.factory/logs CLAUDE_PLUGIN_ROOT=$W CLAUDE_PROJECT_DIR=$W \
    .worktrees/S-21.07/target/debug/factory-dispatcher < $W/env.json
DISPATCH_EXIT=0
type= plugin.completed fuel_consumed= 9920913 elapsed_ms= 1 exit_code= 0
```

Fuel scales with index size, so this is not a fixed cost:

```
fixture=a1-stale-index        bcindex_bytes=  2383  -> plugin.completed fuel=149079
fixture=a1-production-scale   bcindex_bytes=576396  -> plugin.completed fuel=9921105
```

The cap is 10,000,000 — no `fuel_cap` on the entry and no `[defaults]` section in the registry, so `RegistryDefaults::default().fuel_cap` applies:

```
$ grep -n "fuel_cap" plugins/vsdd-factory/hooks-registry.toml
674:#   non-blocking, not spurious block (ADR-035 §Decision 5). max_bytes caps calibrated
679:# No fuel_cap field: max_bytes caps (512 KiB for BC/story, 1–2 MiB for index/cycle)
681:#   Adding fuel_cap without evidence of exhaustion is premature (BC-5.39.010 v1.14
$ grep -n "defaults" plugins/vsdd-factory/hooks-registry.toml
$ echo "EXIT=$?"   # no [defaults] section
```

I then appended **real rows duplicated from the live index** and bisected the exhaustion threshold:

```
added_real_rows=0    bytes=576396 dispatch_exit=0 stdout_bytes=0 -> plugin.completed fuel=9921105 cause=-
added_real_rows=20   bytes=579889 dispatch_exit=0 stdout_bytes=0 -> plugin.completed fuel=9932167 cause=-
added_real_rows=60   bytes=586350 dispatch_exit=0 stdout_bytes=0 -> plugin.completed fuel=9952042 cause=-
added_real_rows=100  bytes=593525 dispatch_exit=0 stdout_bytes=0 -> plugin.completed fuel=9975246 cause=-
added_real_rows=130  bytes=601548 dispatch_exit=0 stdout_bytes=0 -> plugin.timeout   fuel=10000000 cause=fuel
added_real_rows=400  bytes=649702 dispatch_exit=0 stdout_bytes=0 -> plugin.timeout   fuel=10000000 cause=fuel
```

Three facts fall out. **(1)** Headroom is 78,895 fuel = **0.79%** of budget; exhaustion lands between **593,525 and 601,548 bytes**, i.e. **~3–4% above the current 576,396-byte live index**, about **110 additional BC-INDEX rows**. `total_bcs` moved 1983→1985 in a single burst during this very cascade. **(2)** At exhaustion the observable outcome is `dispatch_exit=0` with `stdout_bytes=0` — byte-identical to a clean corpus, exactly as pass-7 predicted. **(3)** The `max_bytes` calibration claim is false in the direction that matters: PC4 permits 1,048,576 bytes, and exhaustion occurs at roughly **57% of that cap**, so the cap does not bound the read inside the fuel budget — it permits a file ~74% past exhaustion.

**Why it matters.** BC-5.39.010 v1.14 forbids adding a `fuel_cap` "without first confirming that `max_bytes` caps are genuinely insufficient." That confirmation now exists and is reproducible. Separately, the new gate cannot detect the condition: its predicate is binary (`plugin.completed` present / `plugin.timeout` absent) with **no margin term**, so it reads identically at 0.79% headroom and at 79%. And its fixture is a **frozen byte-copy** of the index as of fixture creation with no drift gate binding it to the live file — so as `BC-INDEX.md` grows past ~594 KB, production will silently stop validating while this gate stays green on a stale snapshot. That is the same predicate-narrower-than-claim shape the fixture was added to close, displaced one level up.

**Failure scenario.** Within roughly 110 BC-INDEX rows of ordinary registration traffic, every `Edit`/`Write` under `.factory/specs/behavioral-contracts/` exhausts fuel and returns `Continue`. Arm A1/A2/B/E validation stops entirely. `bats` stays green (frozen fixture), `cargo test` stays green (native, no fuel meter), the dispatcher exits 0, and the registry comment still reads "sufficient … at current sizes." No gate at any layer distinguishes this from a clean corpus.

**Routing.** test-writer: add a **margin** assertion to the production-scale gate — parse `fuel_consumed` from the `plugin.completed` record and require it below a stated fraction of the effective cap (e.g. ≤ 60%), so the gate fails on approach rather than on arrival; and add a drift gate asserting the fixture's `BC-INDEX.md` sha256 (or row count within a tolerance) tracks the live file, so the fixture cannot silently fall below production scale. architect: adjudicate the `fuel_cap` decision now that the BC's own evidentiary precondition is met — either raise the cap for this entry or reduce PC4's `max_bytes` to a value that genuinely bounds the read inside the budget. product-owner: correct the two falsified claims in BC-5.39.010 §Gate Spec, citing the measured threshold rather than an asserted calibration.

---

#### F-S2107-P9-003 — HIGH [regression] — POLICY 15's ATTESTATION-LOCATION GATE became satisfiable at v1.4.20 (PARENT-SHA form, D-960 / ADR-040 §Decision 2, closing pass-7's F-S2107-P8-003) and the next two assertion-site commits did not satisfy it. The required section heading form does not exist anywhere in `red-gate-log.md`, and the gate returns 0 at reviewed HEAD.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md` — absence of a `### <Pass-N> assertion-site attestation (<PARENT-SHA>)` section; against `.factory/policies.yaml` POLICY 15 ATTESTATION-LOCATION GATE (v1.4.20) and commits `67ffbdcc` / `37022ecc`.

**Clause violated:** POLICY 15 ATTESTATION-LOCATION GATE, severity HIGH — *"a fix wave that adds or strengthens any bats assertion site MUST NOT be pushed until the matching red-gate-log … attestation section exists at that commit"*, with the literal check `PARENT=$(git rev-parse HEAD^1) && grep -c "assertion-site attestation ($PARENT)" <red-gate-log-path>` → 1, and *"The attestation section heading MUST be `### <Pass-N> assertion-site attestation (<PARENT-SHA>)`"*. Also TD-VSDD-053 same-commit bundling (retained by the amendment).

**Evidence — literal shell at `67ffbdcc`:**

```
$ RG='crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md'
$ grep -nE '^#{3,} .*assertion-site attestation' "$RG"; echo "GREP_EXIT=$?"
GREP_EXIT=1
```

Zero section headings of the mandated form exist. The only two attestation strings in the file are inline prose from earlier bursts, and neither names this commit's parent:

```
$ grep -nE 'assertion-site attestation \(' "$RG"
1191:assertion-site attestation (b78b27ef402f11e36c8c23f68f65d6335c37dd14)
1353:assertion-site attestation (295585185308629f10ff4333647a15b474192c3f)
$ PARENT=$(git rev-parse HEAD^1); echo "PARENT=$PARENT"
PARENT=37022ecc5398514744b72660da47bfb2964abb55
$ grep -c "assertion-site attestation ($PARENT)" "$RG"
0
```

HEAD is unambiguously an assertion-site commit — it changed both the bats file and the lib.rs test module:

```
$ git diff --name-only HEAD^1..HEAD | grep -E '\.bats$|src/lib\.rs$'
crates/hook-plugins/validate-cross-site-correspondence/src/lib.rs
plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats
```

The preceding commit is in the same position: `37022ecc` changed `src/arm_a1.rs` (assertion sites) plus the log, and no attestation names its parent `7d087d78` either.

**Why it matters.** Pass-7 established that the D-912 HEAD-SHA form was mathematically unsatisfiable and routed it to architect for a reproducible redefinition. That work was done and human-ratified — ADR-040 §Decision 2, policies v1.4.20, D-960. The amendment's entire purpose was to make the gate *meetable*, and its very next opportunity was missed twice. This is a strictly worse state than pass-7's: previously the gate could not be satisfied by anyone; now it can be, so a 0 is a genuine compliance failure rather than a structural impossibility, and the finding is no longer self-perpetuating-by-design. There is a Pass-8 section in the log (`## Pass-8 Fix Burst — BC-5.39.010 v1.14 …`), so the authoring habit exists — only the SHA-bound heading is absent, which is the one element that makes the gate un-satisfiable-by-a-prior-pass.

**Failure scenario.** Assertion sites keep changing without a SHA-bound attestation, so no reviewer or wave gate can establish *which tree* any recorded gate figure was measured against. That is the precondition for the F-S2107-P8-001/-002 class (a GREEN cell over a red suite), which this cascade has already paid for once.

**Routing.** test-writer: add `### Pass-9 assertion-site attestation (37022ecc5398514744b72660da47bfb2964abb55)` to `red-gate-log.md`, bundled in the same commit as the next assertion-site change, and run the literal predicate with captured stdout before push. state-manager: add the predicate to the pre-push gate set so it is invoked rather than asserted.

---

### MEDIUM

#### F-S2107-P9-004 — MEDIUM [process-gap] — The D-961 burst-log Dim-2 block labels a bats evidence block "verbatim copy-in" while it is a paraphrased composite: test labels are rewritten, lines are elided with `...`, and it ends with a summary line that neither `bats` nor `bats --tap` emits alongside TAP lines. Two further gate blocks in the same section are narrative-only with no literal command.

**Location:** `.factory/cycles/v1.0-brownfield-backfill/burst-log.md` — the `## D-961-RECORDING-BURST` entry, `### Block 5: Dim-2 Attestation — Literal Shell Gates (D-449(a))`, the sub-blocks headed *"Bats evidence (provided by team-lead specialists; verbatim copy-in)"*, *"Workspace gate (feature/S-21.07 @ 67ffbdcc — provided by test-writer)"* and *"Implementer gate (F-S2107-P8-016 fix …)"*.

**Clause violated:** POLICY 15 `ll_n_verbatim_stdout_discipline` — literal command AND captured stdout; *narrative paraphrase NOT compliant*; POLICY 22 `subagent_report_fidelity_literal_shell`; D-449(a) / META-LEVEL-24.

**Evidence — the attested block records:**

```
$ bats sprint-state-format.bats
1..14
...
ok 12 completeness/status-fidelity
...
ok 14 partition depth ordering
14 tests, 0 failures
```

The actual labels are different, and no single invocation produces that composite:

```
$ bats --tap sprint-state-format.bats | grep -E '^ok (12|14) '
ok 12 test_real_production_file_completeness_and_status_fidelity
ok 14 test_partitions_sorted_by_full_graph_depth_def_b
$ bats sprint-state-format.bats | tail -4
ok 11 test_consumer_rejects_complete_status
ok 12 test_real_production_file_completeness_and_status_fidelity
ok 13 test_supersession_edge_tolerated_partition_placement
ok 14 test_partitions_sorted_by_full_graph_depth_def_b
```

`ok 12 completeness/status-fidelity` and `ok 14 partition depth ordering` are paraphrases; the trailing `14 tests, 0 failures` line appears in neither form. The two adjacent sub-blocks carry no command at all — `FMT_EXIT=0, CLIPPY_EXIT=0, cargo test 167 passed / 0 failed / 17 ignored` / `bats 2198 executed, 51 skipped, 254 suites, BATS_EXIT=0, not-ok count 0`, and `fmt/clippy/test all exit 0; 189 ok lines`.

**Stated precisely: every figure is TRUE.** I reproduced all of them — `Coverage: 2198 executed, 51 skipped (2249 total across 254 suites)`, `All tests passed`, exit 0, zero `not ok`; cargo `167 passed / 0 failed / 17 ignored`; fmt and clippy exit 0 on all three branches. And the same Block 5 contains two *genuinely* compliant gates whose stdout I reproduced exactly (the POLICY 16 ceiling gate → `PASS: global max D-961 < D-9000 ceiling`, and the 8-block self-gate → `8`). The defect is form, and the provenance is honestly disclosed ("provided by … specialists"). One member of the referenced set is not reproducible at any tree in the diff: `pass-phase1-advisory-d99999.bats` exists only on `bf642fd9`, and the block does not say so.

**Why it matters.** D-449(a) exists because narrative-attested gates cannot detect their own scope-degradation, and this cascade's BLOCKER pair came from exactly that. Paraphrased labels are the specific failure mode POLICY 15 names: a renamed or deleted test cannot be detected against a paraphrase, whereas a verbatim label is a name-set that can be diffed. Two of the three sub-blocks also cross a tree boundary (S-21.07, nested-factory, d999-migration are three unmerged branches) without stating which tree each figure came from — a reviewer cannot replay them without guessing.

**Routing.** state-manager: replace paraphrased labels with verbatim TAP lines and the exact invocation, one command per block, and name the tree/SHA each was captured at; where a suite exists only on a side branch, say so. For relayed figures, require the specialist to return literal stdout per POLICY 22 rather than a summary line.

---

#### F-S2107-P9-005 — MEDIUM — The retained E-22 epic file was never reconciled with the re-anchor it lost its only story to. It sits at `v1.0` unamended with `status: draft` and `story_count: 1`, carries no dissolution marker and no D-961 reference, and its §Epic Placement Justification still argues at length against exactly the placement that happened. STORY-INDEX marks E-22 DISSOLVED; the epic file does not.

**Location:** `.factory/stories/epics/E-22-dependency-security-hardening.md` — frontmatter `version`, `status`, `story_count`, `modified[]`, `last_amended`; body §Epic Placement Justification; against `.factory/stories/STORY-INDEX.md` `## Epic E-22 …` heading and `.factory/stories/S-21.12-*.md` frontmatter.

**Clause violated:** POLICY 17 `nn_n_frontmatter_parity_full_file_type_scope` (HIGH; scope explicitly includes `epic`) — a state transition recorded in the index without the matching epic-file frontmatter/changelog advance; POLICY 4 `semantic_anchoring_integrity`; POLICY 14 leg 5.

**Evidence — literal shell at `0a6c8fda`:**

```
$ git -C .factory log --oneline -- stories/epics/E-22-dependency-security-hardening.md
d32207bf factory(D-958/C): content corrections + registrations — … S-21.10/11/12+E-22 registered, …
```

One commit ever — the file has not been touched since creation, across the entire D-961 burst that dissolved it.

```
$ grep -niE 'dissolv|retired|superseded|D-961|E-21 W4|re-anchor' .factory/stories/epics/E-22-dependency-security-hardening.md
(NONE — no dissolution marker, no D-961 reference)
$ grep -nE '^(epic_id|version|status|story_count):' .factory/stories/epics/E-22-dependency-security-hardening.md
3:epic_id: "E-22"
4:version: "v1.0"
5:status: draft
11:story_count: 1
```

The index says otherwise, and the story has moved:

```
$ grep -nE '^## Epic E-22' .factory/stories/STORY-INDEX.md
742:## Epic E-22 — Dependency Security Hardening (v1.0-brownfield-backfill) — DISSOLVED 2026-08-07
$ grep -nE '^(epic_id|wave):' .factory/stories/S-21.12-*.md
5:epic_id: "E-21"
25:wave: 4
$ grep -l 'epic_id: "E-22"' .factory/stories/S-*.md || echo "(none)"
(none)
```

And the body now argues against the state of the world:

> *"**Why separate from E-21:** … Placing S-21.12 in E-21 would conflate runtime-defect fixes with dependency security maintenance"* — against E-21 with 11 stories including S-21.12, and the same section describing E-21's bounded set as "S-21.01..S-21.06".

**Why it matters.** This is the reversal recorded inconsistently, which is the case the dispatch asked me to flag. The 2026-08-08 ruling retaining the file is properly recorded in `decision-log.md` — D-961(c) carries an explicit `**[2026-08-08 supersession — file-deletion clause only]:**` clause stating *"E-22 epic file RETAINED … E-22-the-epic is dissolved; E-22-the-file is retained for historical record"* — so the reversal itself is captured. What is not captured is the consequence: retention converts a file that was about to disappear into a live artifact, and a live artifact reading `status: draft` / `story_count: 1` with a placement argument contradicting the index is a spec that says the opposite of the governing record. POLICY 5 v1.3.5 Part A enumerates the historical-by-construction classes exhaustively; an entire epic file is not among them, and `status` and `story_count` are live frontmatter fields, not narrative. I did **not** raise the `deletion PENDING operator execution` staleness in STATE.md, per the dispatch's scoping.

**Routing.** story-writer: amend E-22 to v1.1 — `status: dissolved` (or the canonical terminal marker), `story_count: 0`, a `modified[]` entry citing D-961(c) and the 2026-08-08 retention ruling, and a §Placement Justification note recording that S-21.12 was re-anchored to E-21 W4 so the argument reads as superseded history rather than current guidance.

---

#### F-S2107-P9-006 — MEDIUM — BC-5.41.004 PC5 and INV-3 assert as fact that no `wave:` frontmatter field exists on story files. 156 of 159 story files carry one, and `S-21.12` — re-anchored this burst — carries `wave: 4`. A false precondition leaves the BC's postconditions vacuously untriggered.

**Location:** `.factory/specs/behavioral-contracts/ss-05/BC-5.41.004.md` — §Preconditions item 5 (*"No `wave:` frontmatter field exists on individual story files. Wave assignment is derived exclusively from the dependency graph."*) and §Invariants item 3 (*"No phantom `wave:` field … No such field exists on story specs."*); against `.factory/stories/S-*.md` frontmatter.

**Clause violated:** POLICY 5 SDK-GROUNDING MANDATE (a narrative claim about external artifacts must be grounded in literal-shell evidence of the actual source) + v1.3.6 HEAD-reproducibility; POLICY 13 FAIL-CLOSED-IMPLICATION-DIRECTION (a precondition that is false disables the obligation rather than failing closed); POLICY 4.

**Evidence — literal shell at `0a6c8fda`** (frontmatter-scoped: enter on line-1 `---`, exit on the closing `---`):

```
$ cnt=0; for f in .factory/stories/S-*.md; do
    awk 'NR==1&&/^---/{p=1;next} p&&/^---/{exit} p&&/^wave:/{print}' "$f" | grep -q . && cnt=$((cnt+1)); done; echo "COUNT=$cnt"
COUNT=156
$ ls .factory/stories/S-*.md | wc -l
     159
$ for f in .factory/stories/S-*.md; do awk 'NR==1&&/^---/{p=1;next} p&&/^---/{exit} p&&/^wave:/{print FILENAME": "$0}' "$f"; done | head -4
.factory/stories/S-0.01-bump-version-prerelease.md: wave: 1
.factory/stories/S-0.02-release-workflow-prerelease.md: wave: 1
.factory/stories/S-1.01-cargo-workspace-setup.md: wave: 1
.factory/stories/S-1.02-dispatcher-core.md: wave: 2
$ grep -nE '^wave:' .factory/stories/S-21.12-*.md
25:wave: 4
```

**Why it matters.** The BC's *normative intent* is sound and is correctly implemented — ordering must come from the dependency graph, and `sprint-state-format.bats` T-14 genuinely recomputes full-graph depth from STORY-INDEX `depends_on` rather than trusting any `wave:` value. The defect is that the intent is expressed as a **factual claim about the corpus** that is false at 156/159 sites, and expressed in the **precondition** position. PC5 is what gates whether the BC's postconditions apply at all; a producer or reviewer checking preconditions literally finds one that does not hold and can conclude the obligations are not triggered. This is the boundary-polarity failure mode in its plainest form: the guard is stated as "the hazard does not exist" rather than "the hazard must not be consulted." The burst that re-anchored S-21.12 added a fresh instance of the field the BC says does not exist.

**Routing.** product-owner: restate PC5 and INV-3 as prohibitions rather than existence claims — "a `wave:` frontmatter field MAY be present on story files and MUST NOT be consulted for ordering; ordering is derived exclusively from STORY-INDEX `depends_on`" — and add the literal-shell grounding above so the claim is measured. Optionally route to architect the separate question of whether the widely-present `wave:` field should be retired or given a documented non-ordering meaning.

---

### LOW

#### F-S2107-P9-007 — LOW — The STATE.md Identifier Conventions epic-count cell reads `20`; 22 epic files exist on disk, and D-961(c)'s own enumeration sums to 22.

**Location:** `.factory/STATE.md` §Identifier Conventions — the `| Epic | E-N | …` row's count cell; against `.factory/stories/epics/`.

**Clause violated:** POLICY 14 leg 5 (upstream-index body-table cells); POLICY 5 v1.3.7 CATEGORY (i) (aggregation cells duplicating per-row values).

**Evidence — literal shell at `0a6c8fda`:**

```
$ ls -1 .factory/stories/epics/E-*.md | wc -l
      22
$ grep -n "| Epic | E-N |" .factory/STATE.md
123:| Epic | E-N | `stories/epics/E-N-<short>.md` | 20 (pre-D-961 baseline; E-22 dissolution PENDING operator execution — file on disk, agent git rm blocked per D-961(c); count becomes 21 once file is removed) |
```

The decision-log's own accounting for the same fact resolves to 22:

> D-961(c): *"STATE.md Identifier Conventions: Epic count updated (E-0..E-19=20 contiguous + E-20 never allocated + E-21 active + E-22 dissolved)."*

E-0..E-19 is 20 files, plus E-21 and E-22 → 22, which matches the directory listing. The cell's `20` is also wrong under its own `pre-D-961 baseline` label: E-22 was created 2026-08-06, before the D-961 burst of 2026-08-07, so the pre-burst count was already 22.

**Scoped deliberately:** I am reporting only the count value. The `deletion PENDING` / `becomes 21` projection clause is the staleness the dispatch scoped out as already being corrected, and the `becomes 21` arithmetic is in fact consistent with a true current count of 22 — it is the headline `20` that is wrong.

**Routing.** state-manager: set the count to 22 and state the enumeration basis inline so the cell can be re-derived by `ls -1 .factory/stories/epics/E-*.md | wc -l`.

---

### NITPICK

#### F-S2107-P9-008 — NITPICK — `derive_factory_dir` carries a fully duplicated doc-comment block: two consecutive near-identical copies of the same 12-line comment, a copy-paste artifact of the F-S2107-P8-016 fix.

**Location:** `crates/factory-dispatcher/src/main.rs` — the doc comment immediately preceding `fn derive_factory_dir` (branch `fix/nested-factory-path-derivation` @ `09f052a9`).

**Clause violated:** none normative; production-grade default (rustdoc renders both paragraphs, so the published docs contain the guard rationale twice).

**Evidence — literal shell at `09f052a9`:**

```
$ awk 'NR>=838 && NR<=861 {printf "%d\t%s\n", NR, $0}' crates/factory-dispatcher/src/main.rs
838	/// Derive the factory-artifacts directory from the dispatcher's host context CWD.
...
849	/// (TD-VSDD-060 sibling-site sweep).
850	/// Derive the factory-artifacts directory from the dispatcher's host context CWD.
...
860	/// (TD-VSDD-060 sibling-site sweep).
861	fn derive_factory_dir(cwd: &std::path::Path) -> PathBuf {
```

Both blocks open with the identical H1 sentence and close with the identical `(TD-VSDD-060 sibling-site sweep).` line. `cargo fmt --check --all` and `cargo clippy -- -D warnings` both exit 0 — neither detects duplicated doc paragraphs.

**Routing.** implementer: delete the first of the two blocks (the second is the better-worded one — it names `is_dot_factory_basename` reuse explicitly).

---

## Part B — Observations

**O-S2107-P9-01 — `run-all.sh` "executed" conflates passes with failures.** `total_executed=$((total_tests - total_skips))` counts a failing test as executed, and the `Coverage:` line carries no failed count. This is not a false-green risk — `bats_exit != 0` sets `fail_count`, the failing suites are named, and the script exits 1 — but the single summary line a reviewer is most likely to quote cannot distinguish 2198 passing from 2190 passing plus 8 failing. Adding a `failed=N` term to the coverage line would close the gap. (Not raised as a finding: the exit-code path is correct and F-S2107-P8-014 grandfathers scope changes here.)

**O-S2107-P9-02 — the S-21.07 worktree contains a partial `.factory/`.** `find .worktrees/S-21.07/.factory -mindepth 1` returns 10 entries — `cycles/` and `logs/` only, including `logs/dispatcher-internal-2026-08-04.jsonl` and `-08-06.jsonl`. This is **not** an F-S2107-P8-016 regression: the cwd basename there is `S-21.07`, not `.factory`, so `log_dir` Level E legitimately creates a `.factory/logs` child. It is relevant to S-21.04 (story-worktree write-path discipline) — a story worktree accumulating factory-shaped state. It is also harmless to the corpus tests, because `discover_factory_root`'s `is_real_corpus` predicate requires `specs/behavioral-contracts/` and therefore skips this directory and walks up to the real corpus. That predicate is a genuinely well-built guard and worth preserving as-is.

**O-S2107-P9-03 — ADR-041's live-corpus capture will never reproduce.** The ADR records `PASS: global max D-960 < D-9000 ceiling (run 2026-08-07)`; at factory HEAD the same predicate yields `D-961`. POLICY 5 v1.3.6 prefers structural form over a snapshot for exactly this reason, and a monotonically-increasing corpus maximum can never reproduce at a later HEAD. Not raised as a finding because the value is not load-bearing — the gate's semantics depend only on `max_d < 9000`, and the four self-tests (which I reproduced exactly) are the actual proof of correctness. A structural phrasing ("the current global max, whatever it is, is < 9000") would remove the decay.

**O-S2107-P9-04 — F-S2107-P8-014 scope has not grown.** `run-all.sh` is absent from `67ffbdcc`'s diff and unchanged since the grandfather ruling; `git diff --name-status origin/develop...<branch> | grep -E '\.sh$'` returns it on S-21.07 only (the pre-existing TAP accumulator) and nothing on either fix branch. No new `.sh` file was added anywhere. The new `pass-phase1-advisory-d99999.bats` is a `.bats` file and outside POLICY 21's `.sh` scope. Recorded per the dispatch's request to note scope growth; none found.

**O-S2107-P9-05 — `policies.yaml` is a two-document YAML stream.** `yaml.safe_load()` raises `ComposerError: expected a single document in the stream` on it (frontmatter block at lines 1–4, body from line 5). The `validate-policies-schema` plugin handles it and its tests pass, so this is not a defect — recorded only so that any future tool author reading the file uses a frontmatter-aware loader rather than a single-document one.

---

## Part C — Analysis

**Two candidate findings were discarded after re-derivation.** Both were artifacts of my own parsers, and both are worth recording because each would have been a plausible-sounding HIGH.

First, a naive scan of `sprint-state.yaml` reported `S-5.07: completed` — a status outside BC-5.41.004 INV-1's 8-value enum, which INV-1 requires the producer to hard-abort on. On inspection, the top-level `stories:` list runs lines 13–373 and gives `S-5.07` status `draft` at line 371, matching its STORY-INDEX catalog row; my parser had run past line 373 into the `epics:` block and picked up an epic's `status: completed`. The suite's T-12 (`test_real_production_file_completeness_and_status_fidelity`) anchors its awk at column-0 `/^stories:/` precisely to prevent this, and is correct.

Second, my full-graph depth computation reported 60 of 61 Partition B entries out of order. The cause was my treating `depends_on: ["E-9"]` (an epic reference) as no dependency, collapsing multi-level stories to depth 1. T-14 (`test_partitions_sorted_by_full_graph_depth_def_b`) performs epic-expansion for exactly these text refs. The four placements the dispatch asked me to verify all hold on direct inspection: S-21.10 and S-21.12 have empty `depends_on` and sit in the lexicographically-ordered depth-1 zone; S-21.11 (`depends_on: [S-21.10]`) and the relocated S-21.06 (`depends_on: [S-21.01]`) sit in the depth-2 zone, adjacent and in `S-10.02 < S-14.02 < S-15.02 < S-21.06 < S-21.11 < S-7.07` string order. BC-5.41.004 PC3 / ADR-026 §Decision 3a def-b ordering holds.

**On the pattern.** The dispatch predicted the dominant class — a gate whose predicate is narrower than its claim, with absence of signal converted to pass — and it recurs, but displaced. The *predicates* in this burst are mostly right: the ceiling gate covers every form present in the corpus and fails closed; the cite sweep is real; the corpus-root discovery is properly guarded; the coverage gate stopped synthesising figures; the production-scale fixture is byte-identical to production. What is still narrower than its claim is the layer above: a mutant set proving a proposition that was already true before the fix (P9-001), a budget assertion with no margin term over a frozen snapshot (P9-002), a policy gate made satisfiable and then not satisfied (P9-003), and a "verbatim" block that is paraphrased (P9-004). The severity distribution moved accordingly — no BLOCKER for the first time in this cascade, and the count halved from 16 to 8.

**On F-S2107-P8-013 specifically.** This is the pass's most useful result and it deserves an explicit note. The finding is genuinely closed on the axis pass-7 specified, and closing it is what made the underlying problem measurable for the first time: nobody could have known the budget was 99.21% consumed without a production-scale sandbox fixture to measure against. The correct reading is not that the fix failed, but that the fix worked and immediately revealed a latent production defect roughly 110 index rows away. That is what a gate is for.

**On the streak.** Streak remains **0/3** — this pass is NOT-CLEAN, so no advance. Trajectory `47→18→25→25→24→20→16→8`. Findings by severity: **BLOCKER 0, HIGH 3, MEDIUM 3, LOW 1, NITPICK 1 = 8.** Novelty 0.75 against pass-7 Part A: F-S2107-P9-003 is a lineage descendant of F-S2107-P8-003 (same gate, inverted cause), F-S2107-P9-004 descends from the P8-002/P8-007 attestation class, and the remaining six are new. Three of pass-7's five HIGHs and both BLOCKERs are closed and independently verified.

**Verdict: NOT-CLEAN.**
