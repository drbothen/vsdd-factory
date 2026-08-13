# PR #775 Review — S-21.09 validate-factory-path-staging.wasm Artifact Restore + Registry Parity Gate

## Verdict: APPROVE

`covered_sha: 6ae075a6d6d197ac56182e04de93ffffab69c3dd`

Fresh-eyes final PR diff review. Reviewed the diff, PR description, story spec
(S-21.09 v1.28), governing BC (BC-4.16.001 v1.8), and test evidence — not a re-run of
the 19-pass LOCAL cascade. Every load-bearing claim was independently reproduced on a
throwaway worktree at the PR HEAD rather than taken from the evidence report.

**Scope framing:** this PR changes **zero production lines**. All 5,199 added Rust
lines are tests and fixtures; the only behavioral change is the restored WASM binary.

---

## Independent Verification (not just trusting the evidence report)

| Claim | Method | Result |
|-------|--------|--------|
| 20 files; registry.rs +64; bundle_orphan_check.rs +5135; WASM 193,427 B | `git diff --stat 62fbcf1a..6ae075a6` | PASS — matches exactly |
| WASM provenance SHA-256 `6f6570f9…f6ce17` | Rebuilt: `cargo build -p validate-factory-path-staging --target wasm32-wasip1 --release`, rustc 1.95.0 | PASS — **byte-identical**; reproducible build confirms the blob is the build of the in-repo source |
| WASM is a valid module | magic bytes | PASS — `00 61 73 6d 01 00 00 00` |
| 51/51 cargo tests | `cargo test -p factory-dispatcher --test bundle_orphan_check` | PASS — 51 passed, 0 failed |
| **T-012 is load-bearing, not a paper gate** | `git rm --cached` the WASM in a throwaway worktree, re-ran T-012 | **PASS — goes RED** with exactly `MISSING: hook-plugins/validate-factory-path-staging.wasm` (TD-VSDD-059) |
| 36/36 bats, zero skips | Built release dispatcher; `CI_REQUIRE_ARTIFACTS=1 bats validate-factory-path-staging.bats` | PASS — 36/36 `ok`; the env var makes a missing artifact a hard failure, so this proves the WASM loaded and executed |
| AC-002's "25 previously-skipping tests" | `grep -c '^  _require_artifacts'` | PASS — exactly 25 of 36 |
| fmt / clippy / full workspace suite | `cargo fmt --check --all`; `clippy --workspace --all-targets -D warnings`; `cargo test --workspace --all-targets` | PASS — all clean, zero regressions |
| Registry-inventory precondition (exactly 2 `.toml`) | `git ls-tree plugins/vsdd-factory/*.toml` | PASS |
| declared = tracked = 36 | 35 unique hooks + 1 resolver vs. 36 tracked `.wasm` | PASS |
| CI durably enforces zero-skip | `ci.yml:286` sets `CI_REQUIRE_ARTIFACTS: "1"` | PASS — cannot silently regress to skip-pass |
| Demo evidence authenticity | GIF/WebM magic bytes; all four `.tape` scripts | PASS — valid media; every taped command is read-only |

### AC traceability (7/7)

| AC | Implemented by | Verified |
|----|----------------|----------|
| AC-001 artifact tracked | WASM committed via `git add -f` | reproducible-build match + `git ls-tree` |
| AC-002 zero WASM-absent skips | artifact restore | 36/36 bats, zero skips, `CI_REQUIRE_ARTIFACTS=1` |
| AC-003 block on product branch | S-21.01 guard, now reachable | bats T-001 `ok` |
| AC-004 pass non-`.factory/` | same | bats T-004 `ok` |
| AC-005 pass on factory-artifacts | same | bats T-002 `ok` |
| AC-006 declared ⊆ tracked | `run_t012_gate` + T-012..T-056 | 51/51 + RED-on-regression proven |
| AC-007 no other gaps | T-012 subset assertion | subset check is strictly stronger than a count assertion |

`verification_properties: []` is a disclosed VP gap with a routing proposal attached in
the story spec — accepted residual, not re-litigated.

---

## 8-Item Checklist

1. **Diff coherence** — Coherent. Artifact restore + inverse-direction test family +
   demo evidence + CHANGELOG. Nothing tangential.
2. **Description accuracy** — Matches the diff on all structural claims (file count,
   line counts, byte count, SHA-256, 51/51, 36/36, 19 passes). Two labelling
   inaccuracies, both non-blocking — see F-2 and F-7.
3. **Test coverage** — Exhaustive. Negative controls confirm the gate is load-bearing:
   T-015 (MISSING fires), T-036 (gitignored probe), T-037 (staged-not-committed),
   T-038/T-040 (ungated declaration), T-047 (outside-repo), T-050/T-051 (both `in_repo`
   conjuncts independently isolated). I re-executed the whole suite and separately
   proved T-012 goes RED when the artifact is untracked.
4. **Demo evidence** — 7/7 ACs covered (VHS `.gif`+`.webm` plus one captured log).
   Media validated; every recording command is read-only against the real worktree.
5. **Commit quality** — HEAD matches `covered_sha`; no AI attribution observed. 9 of 29
   commits carry the scope `test(policy15-gate):` rather than S-21.09 — cosmetic,
   collapsed by squash-merge.
6. **Diff size** — 5,572 insertions, but production surface is zero; the bulk is the
   mutation-isolation test family plus binary evidence. Intentional density per the
   production-grade default. Not a concern.
7. **Missing changes** — None. All 7 ACs and all 5 RG-001..RG-005 red gates accounted
   for and independently confirmed.
8. **Dependency status** — `depends_on: []`, `blocks: []`. No upstream PR gating, no
   downstream story blocked.

---

## CI Status at review

**9 pass / 4 pending.** Every logic-bearing leg is green: `cargo-host (ubuntu-latest)`
**and** `cargo-host (macos-latest)`, `bats-full-suite (linux)`, `bats-darwin-leg`,
`bats-wave-handoff`, `SAST (Semgrep)`, `validate`, `platforms-drift`,
`build-dispatcher (linux-arm64)`.

The ubuntu-latest pass is the materially important one: it exercises T-012 on a
case-sensitive filesystem, which is the platform the case-variant handling
(T-031/T-042, the `eq_ignore_ascii_case` gate-3 admission + verbatim return) was
hardened for. The 4 pending are `build-dispatcher` cross-compile legs (linux-x64,
darwin-x64, darwin-arm64, windows-x64) — long builds unrelated to this diff's logic.
`mergeStateStatus: UNSTABLE` reflects only that pendency.

---

## Findings (all non-blocking)

### [LOW] doc-accuracy — CHANGELOG.md is stale by five adversary passes

`CHANGELOG.md` was last touched at `1c59a669` (pass-11). Passes 12–16 then added T-050,
T-051, T-052, T-053, T-054, T-055, T-056 and the `registry.rs` `on_error` control — none
reflected. The entry reads "Adds **T-012..T-049**"; the delivered range is
**T-012..T-056**. It also omits the `registry.rs` test entirely and still says
**"PR pending"** rather than `PR #775`.

This is the same comment-drift class the cascade fixed *inside* the test file at
`46e334da` ("fix stale T-012..T-050 range cite → T-012..T-051 in test docstring") — the
sweep covered source docstrings but never reached the release-facing changelog. Worth
fixing before this text is drained into a `## <version>` section at release time
(RELEASING.md Step 2), because it becomes permanent release history at that point.

### [LOW] doc-accuracy — PR body mislabels 64 test lines as production lines

PR body → Test Evidence → Coverage Analysis: `| Lines added (production) | 64
(crates/factory-dispatcher/src/registry.rs) |`. All 64 lines are inside
`#[cfg(test)] mod tests` (`registry.rs:503`). **Zero production lines changed.** This
contradicts the PR's own Security Review section, which correctly states "no source
changes to the guard logic itself." The accurate framing strengthens the risk story.

### [LOW] test-quality — dead assertions + one provably-wrong mutation narrative in T-026

Post-pass-9, `extract_hook_plugin_name` returns
`joined_parts[expected_depth..].join("/")` — it can never return a bare basename. So
neither of these can fail:

- T-026(a) L2644: `assert!(!refs.contains("ghost-absolute.wasm"))`
- T-026(b) L2693: `assert!(!refs_depth.contains("evil.wasm"))`

The adjacent `refs.is_empty()` / `refs_depth.is_empty()` assertions (L2652, L2703) carry
the load. Worse, T-026(b)'s attached mutation narrative is wrong: deleting the gate-2
prefix loop yields `{"hook-plugins/evil.wasm"}`, not `{"evil.wasm"}` — the comment
credits the dead assertion with a kill only `is_empty()` achieves. Leftovers from the
pre-pass-9 basename design.

### [LOW] test-quality — T-054's `should_panic` substring is broader than its stated purpose

`#[should_panic(expected = "but production requires 1")]`. The name and comment claim to
isolate the `.unwrap_or(-1)` **sentinel value**, but the substring matches any value ≠ 1,
so `.unwrap_or(0)` / `.unwrap_or(2)` mutants also panic and the test still passes. Only
`.unwrap_or(1)` is killed.

**This does not invalidate the SURV-04 closure.** The security property is "an absent
`schema_version` key must not silently coerce to the production-required value," and only
`.unwrap_or(1)` breaks it — T-054 does kill that. What is overstated is the test's
self-description. Tightening to `schema_version=-1 but production requires 1` would make
it match its docs.

### [LOW] test-quality — T-048's "extract ⟺ ¬detect" is partly definitional

After the pass-11 single-copy refactor, `detect_ungated_declarations` decides its UNGATED
branch by literally calling `extract_hook_plugin_name(...).is_none()` (L479). For UNGATED
rows the biconditional is the implementation restated, and the comment's claim that the
table kills the "M1+M4 composite" by cross-checking two independent copies is void — the
refactor it describes is what removed the second copy.

The test is **not** worthless: OUTSIDE rows are classified by the `in_repo` predicate
*before* delegation (so the biconditional is real there), and the hand-written
`expected_class` column is genuine ground truth for 18 concrete inputs. Real coverage,
overstated framing. Related: the comment at L4740–4742 credits "the GATED assertion for
`hook-plugins`" with killing M4, but `"hook-plugins"` is an **UNGATED** row — the catching
assertion is `extract_result.is_none()` in the other arm.

### [NIT] test-quality — T-015 doesn't pin the indent it claims to

Comment says it "locks the string format" verbatim, but it asserts
`"MISSING: hook-plugins/hooks-only.wasm"` without the two-space indent the production
format string `"  MISSING: {}"` (L811) emits. T-021 (L2258) does include it, so the indent
is pinned — just not where T-015 claims.

### [NIT] doc-accuracy — two counting/labelling slips in the PR body

- `Total suite (T-006..T-056 + registry.rs unit) | 51/51` — T-006..T-056 is *itself*
  exactly 51 (6 pre-existing + 45 new); adding the `registry.rs` test makes 52. The 51/51
  figure is correct for `bundle_orphan_check.rs` alone; the row label double-counts.
- 9 of 29 branch commits carry scope `test(policy15-gate):` rather than S-21.09.

---

## Out of diff scope (recorded, not a finding against #775)

A full audit of every test body (T-006..T-056) found one genuinely tautological
assertion — **T-007** L1362–1373 builds `format!("ORPHAN: {}", name)` from a collection
that L1354 already proved contains the name, then asserts the string it just built is
present. No production code emits `ORPHAN:` lines (`collect_orphans_dual` returns bare
filenames), so the "AC-006 clause (d) format" is checked against a test-local `format!`.
It cannot fail.

**This is pre-existing S-19.04 code on `develop` at L432–438, untouched by this PR**
(`git diff` shows no T-007 hunks). Flagged so it is known; a reasonable candidate for
whoever next touches that test.

---

## Negative results (what the audit specifically cleared)

These are what make the APPROVE more than a spot-check:

- **`#[should_panic]` passing for the wrong reason: none.** All nine
  (T-019/020/022/027/041/049/052/053/054) checked against every panic reachable from
  their own setup. Setup `.expect`/`assert!` messages are test-ID-prefixed
  (`"T-049: git init failed: …"`) and cannot collide with the `expected` substrings.
- **Shared-state mutation: none.** Every write is under `tempdir()`; suite is
  order-independent.
- **Writes against the real repo via git: none.** Every mutating invocation (`init`,
  `add`, `add -f`, `rm --cached`, `commit`, `config user.*`) in
  T-030B/034/036/037/039/049/052/053/054 uses `.current_dir(root)` with
  `root = tmp.path()`, each preceded by an asserted-successful `git init` in that
  tempdir. Real-repo git is confined to read-only `git ls-files` / `git ls-tree -r HEAD`.
- **Fixture/assertion mismatches hiding a defect: none.** Only the comment-accuracy
  items above.

---

## Notes for the merger

- **SURV-01 accepted residual is sound.** `Path::components()` never yields an interior
  `RootDir`, and every call site passes an absolute base, so `parts` is provably empty
  whenever `lex_norm`'s `RootDir | Prefix(_) => parts.clear()` arm fires. Documenting it
  rather than faking a test for it is the correct call.
- **New coupling worth knowing:** `cargo test --workspace` now requires a working `git`
  and a real repository checkout (T-012 / T-041 panic otherwise). Intentional and pinned
  by T-041; `factory-dispatcher` is `publish = false`, so no packaging path is affected.
- Human merge authorization remains a separate gate per the PR's own pre-merge checklist;
  this verdict satisfies the "pr-reviewer READY verdict with `covered_sha`" line only.
