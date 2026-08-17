---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-12T13:30:00Z
phase: 10
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "48547f6"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 10
previous_review: adv-s21.09-local-pass-9.md
---

# Adversarial Review: S-21.09 WASM Artifact Restore and Registry Parity (Pass 10)

**Verdict: NOT-CLEAN**
**Finding summary: 2 BLOCKER / 3 HIGH / 6 MEDIUM / 3 LOW / 1 NIT**
**Reviewed commit: `b951461a` (feature/S-21.09)**
**LOCAL streak: 0/3 — ten passes, zero CLEAN**
**D-chain: D-972**

**Convergence note:** Pass 10 reviewed story v1.19 (42 tests T-006..T-047 all green at `b951461a`). `cargo fmt --check --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1 cargo test --workspace --all-targets` all exit 0; 189 `test result: ok` lines, zero `FAILED`; `bundle_orphan_check` 42/42. Diff scope: `CHANGELOG.md`, `bundle_orphan_check.rs`, two new fixtures (`hooks-registry-{nospace,dotslash}-fixture.toml`), and the `.wasm` binary — no forbidden path touched. Binary is a WASM core module, 193,427 bytes, SHA-256 `6f6570f9…ce17`, blob `611303b3b8edcebc70cf014919e2f73809e7ef52` at `b951461a`; provenance block verifiable from a fresh clone. Pass-9 carry-overs: P09-BLK-001 verified closed; T-042 cfg guard resolved (`cfg_attr` applied); P09-MED-002/003 resolved; P09-LOW-002 resolved; P09-NIT-001 resolved; P09-LOW-001 reclassified out-of-scope (HIGH SECURITY drift D-971); P09-MED-001 (ADR-043-gated) deferred — removed from active tracking. Two new BLOCKERs discovered: (1) `detect_ungated_declarations`'s containment predicate has four conjuncts of which three survive mutation independently — M1+M4 composed re-open a silent drop and an identifier misclassification with a 100%-green suite; (2) the totality invariant (`extract.is_some()` ⟺ `detect.is_empty()`) rests on a hand-written second copy of `extract_hook_plugin_name`'s gates inside `detect_ungated_declarations` — the invariant is asserted by no test. Both BLOCKERs confirmed CLOSED in `1c59a669` (single-copy detect refactor + T-048 totality property assertion; T-049 closes HIGH-2). Five findings remain open after the full fix wave: MEDIUM-1 (directory-only staging control), MEDIUM-4 (T-047 boundary proof over-determined), LOW-1 (NUL/trailing-space names), LOW-2 (fail-open arms with unasserted call-ordering), LOW-3 (`workspace_root()` untested directly).

---

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix
- `<CYCLE>`: `BB` (v1.0-brownfield-backfill per `.factory/current-cycle`)
- `<PASS>`: Two-digit pass number — `P10` for this pass
- `<SEQ>`: Three-digit sequence

---

## Part A — Fix Verification

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-BB-P09-BLK-001 | BLOCKER | VERIFIED CLOSED | Worktree-root containment predicate confirmed in place at `b951461a`; both `UNGATED-DECLARATION` and `OUTSIDE-REPO-DECLARATION` routes present; no silent-drop path reachable. The **partition is correct today** — new BLOCKERs concern its enforcement discipline, not its correctness. |
| ADV-BB-P09-HIGH-001 | HIGH | RESOLVED | `#[cfg_attr(not(target_os = "linux"), ignore = "case-variant test requires case-sensitive Linux filesystem")]` confirmed at `b951461a`; T-042 no longer silently excluded on macOS. `cargo test` exit 0 with 42/42 including T-042 on macOS run. |
| ADV-BB-P09-MED-001 | MEDIUM | DEFERRED | ADR-043 still not ratified; no implementer action possible. Removed from active pass-10 finding set; remains a permanent external blocker until ADR-043 is ratified. |
| ADV-BB-P09-MED-002 | MEDIUM | RESOLVED | `lex_norm` updated to convert `\` to `/` as first normalisation step; confirmed effective: `CAND="hook-plugins\\x.wasm"` → `extract=None reported=["UNGATED-DECLARATION: hook-plugins\\x.wasm"]`. Windows-backslash form correctly routes to ungated. |
| ADV-BB-P09-MED-003 | MEDIUM | RESOLVED | T-039 now asserts `assert_eq!(result.unwrap(), Vec::<Declaration>::new())`; a bug returning a non-empty `Vec` would be caught. |
| ADV-BB-P09-LOW-001 | LOW | RECLASSIFIED | `refuse_setuid` module doc reclassified as out-of-scope for this story; tracked as HIGH SECURITY drift item under D-971. No story-scope action available without a separate security story. |
| ADV-BB-P09-LOW-002 | LOW | RESOLVED | T-031 fixture `merged_count: 107` assertion added; stale latent fragility closed. |
| ADV-BB-P09-NIT-001 | NIT | RESOLVED | `ThreatModelAcceptance` constant now carries one-line rustdoc. |

---

## Totality Probe (Section 1 of review — directly executed)

33 declaration values run through `extract_hook_plugin_name` + `detect_ungated_declarations`. **At HEAD the partition is exact**: for every candidate, `extract == Some` ⟺ `reported == []`. Selected captured output:

```
CAND=""                      extract=None reported=["UNGATED-DECLARATION: "]
CAND="."                     extract=None reported=["UNGATED-DECLARATION: ."]      <- registry parent
CAND=".."                    extract=None reported=["UNGATED-DECLARATION: .."]
CAND="/"                     extract=None reported=["OUTSIDE-REPO-DECLARATION: /"]
CAND="//"                    extract=None reported=["OUTSIDE-REPO-DECLARATION: //"]
CAND="hook-plugins"          extract=None reported=["UNGATED-DECLARATION: hook-plugins"]
CAND="hook-plugins/"         extract=None reported=["UNGATED-DECLARATION: hook-plugins/"]
CAND="hook-plugins/.."       extract=None reported=["UNGATED-DECLARATION: hook-plugins/.."]
CAND="hook-plugins\\x.wasm"  extract=None reported=["UNGATED-DECLARATION: hook-plugins\\x.wasm"]   <- Windows sep
CAND=" hook-plugins/x.wasm"  extract=None reported=["UNGATED-DECLARATION:  hook-plugins/x.wasm"]
CAND="../ghost.wasm"         extract=None reported=["UNGATED-DECLARATION: ../ghost.wasm"]
CAND="../../ghost.wasm"      extract=None reported=["UNGATED-DECLARATION: ../../ghost.wasm"]
CAND="../../../ghost.wasm"   extract=None reported=["OUTSIDE-REPO-DECLARATION: ../../../ghost.wasm"]
CAND="../../../../../../../../../../../../ghost.wasm"  reported=["OUTSIDE-REPO-DECLARATION: ..."]  <- 12 levels
CAND="/abs/hook-plugins/x.wasm"  extract=None reported=["OUTSIDE-REPO-DECLARATION: /abs/..."]
CAND="hook-plugins/x.wasm"       extract=Some("hook-plugins/x.wasm")        reported=[]
CAND="./hook-plugins/x.wasm"     extract=Some("hook-plugins/x.wasm")        reported=[]
CAND="hook-plugins//x.wasm"      extract=Some("hook-plugins/x.wasm")        reported=[]
CAND="hook-plugins/x.wasm/"      extract=Some("hook-plugins/x.wasm")        reported=[]   <- trailing slash
CAND="Hook-Plugins/x.wasm"       extract=Some("Hook-Plugins/x.wasm")        reported=[]
CAND="hook-plugins/x.wasm\0"     extract=Some("hook-plugins/x.wasm\0")      reported=[]   <- NUL admitted
CAND="hook-plugins/x.wasm "      extract=Some("hook-plugins/x.wasm ")       reported=[]
CAND="hook-plugins/é.wasm" extract=Some("hook-plugins/é.wasm")  reported=[]
CAND="hook-plugins/sub/"         extract=Some("hook-plugins/sub")           reported=[]   <- see MEDIUM-1
CAND="hook-plugins/../x.wasm"    extract=None reported=["UNGATED-DECLARATION: hook-plugins/../x.wasm"]
CAND="hook-plugins/sub/../../hook-plugins/x.wasm"  extract=Some("hook-plugins/x.wasm") reported=[]
```

**The partition is correct today. It is also unenforced.** That is the review.

---

## BLOCKER Findings (BOTH CLOSED in `1c59a669`)

### ADV-BB-P10-BLK-001 (CLOSED): `detect_ungated_declarations` containment predicate — four conjuncts, three survive mutation independently; M1+M4 composed re-opens silent drop and identifier misclassification

- **Severity:** BLOCKER
- **Category:** verification-gaps
- **Status:** CLOSED — `detect_ungated_declarations` refactored to call `extract_hook_plugin_name` (single copy of gates); `parent_parts`, `expected_depth`, and `is_hook_plugins` removed; T-048 totality property assertion added
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `detect_ungated_declarations`, containment predicate
- **Description:** `detect_ungated_declarations` checks `in_repo` (two conjuncts: prefix check + `len >`) and `is_hook_plugins` (two conjuncts: prefix check + `len >= expected_depth + 2`). Mutation results:

  | Mutant | Suite |
  |---|---|
  | M1 `in_repo` prefix conjunct → `true` | **42 passed** |
  | M2 `in_repo` `len >` → `len >=` | **42 passed** |
  | M3 `is_hook_plugins` prefix conjunct → `true` | **42 passed** |
  | M4 `is_hook_plugins` `>= expected_depth + 2` → `+ 1` | **42 passed** |
  | M5 `OUTSIDE-REPO` push → silent `continue` | FAILED (T-047) |
  | M6 `is_hook_plugins` `eq_ignore_ascii_case` → `true` | FAILED (T-038, T-040) |

  One of five structural legs is controlled. Composing M1+M4 — two single-token edits — reproduces exactly the class every prior pass found:

  ```
  === BASELINE ===
  CAND="hook-plugins/"                     reported=["UNGATED-DECLARATION: hook-plugins/"]
  CAND="/a/b/c/d/e/f/g/h/i/j/k/l/m/n/evil.wasm"  reported=["OUTSIDE-REPO-DECLARATION: .../evil.wasm"]
  === MUTATED (M1 + M4) ===
  CAND="hook-plugins/"                     reported=[]
  CAND="/a/b/c/d/e/f/g/h/i/j/k/l/m/n/evil.wasm"  reported=["UNGATED-DECLARATION: .../evil.wasm"]
  === SUITE UNDER BOTH MUTANTS ===
  test result: ok. 43 passed; 0 failed
  ```

  `hook-plugins/` becomes **silently dropped** — the precise "no declaration is silently dropped" violation — and an out-of-repo path is reported under the **wrong identifier**. D-970 Cod-1 demands controls that assert the identifier; identifier swaps between the two new classes are invisible to the entire suite.

  `detect_ungated_declarations` contains a hand-written second copy of `extract_hook_plugin_name`'s three gates (`in_repo` + `is_hook_plugins` + `eq_ignore_ascii_case`). The invariant making the partition total is their logical complementarity; that duplication is the structural defect. The fix collapses the predicate into a single call to `extract_hook_plugin_name`, removing `parent_parts`, `expected_depth`, and `is_hook_plugins` from `detect`.

- **Closure evidence:** `git diff b951461a..1c59a669` shows `detect_ungated_declarations` now calls `extract_hook_plugin_name` and treats `None` as report; the three independent legs are gone from `detect`. T-048 adds a property assertion over 18 candidates: `extract.is_some()` ⟺ `detect.is_empty()`.

---

### ADV-BB-P10-BLK-002 (CLOSED): Totality invariant (`extract.is_some()` ⟺ `detect.is_empty()`) asserted by no test — paper-fix under TD-VSDD-059; silence relocated into unasserted duplication

- **Severity:** BLOCKER
- **Category:** verification-gaps
- **Status:** CLOSED — T-048 property assertion over 18 candidates added; invariant now structurally enforced by single-call architecture
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` — no test asserts the complementarity invariant at `b951461a`
- **Description:** T-043..T-047 each pin one hand-chosen input value; none asserts the relationship between `extract_hook_plugin_name` and `detect_ungated_declarations`. The invariant that makes the partition total — that no input is neither gated nor reported — is not tested as an invariant; it is tested as a collection of spot-checks. This is why M1–M4 all pass: the spot-checks cover specific values but not the structural relationship. Under TD-VSDD-059 this is a paper-fix: the silence from prior passes was relocated from a missing-path bug into an unasserted duplication of the gates, not removed.

  The single-copy refactor (BLK-001 fix) resolves this structurally — when `detect` calls `extract`, the invariant becomes a tautology by construction. T-048 additionally asserts it explicitly over 18 candidates, providing a regression check that survives any future refactoring of the call relationship.

- **Closure evidence:** `git diff b951461a..1c59a669` shows T-048 (`proptest` over 18 candidates asserting `extract.is_some()` ⟺ `detect.is_empty()`); no survivor on single-call-site mutations in T-048.

---

## Part B — New Findings

### HIGH

#### ADV-BB-P10-HIGH-001: Both new fixture files' content assertions are satisfied by their own comment headers — live-declaration-only mutation passes 42/42

- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` T-013, T-014; `tests/fixtures/bundle-orphan/hooks-registry-{nospace,dotslash}-fixture.toml`
- **Description:** T-013/T-014 carry `include_str!`-based content assertions over the two new fixture files. Both fixture files document their own discriminating syntax in a comment header. Mutating **only the live `[[hooks]]` declaration** (e.g., `plugin="..."` → `plugin = "..."`; `"./hook-plugins/..."` → `"hook-plugins/..."`), with comments untouched:

  ```
  === live-declaration-only mutant (comments untouched) ===
  test result: ok. 42 passed; 0 failed
  ```

  Both tests lose the only property they exist to prove; both content guards pass off comment prose. POLICY 13 normalisation-adversariality inverted — the assertion domain includes the region the TOML parser discards. Assert against the parsed entry or comment-stripped content.
- **Status in fix wave:** CLOSED — T-013/T-014 assertions updated to use comment-stripped or parsed form; live-declaration mutations now kill both tests.
- **Proposed Fix:** Assert against `include_str!(...).lines().filter(|l| !l.trim_start().starts_with('#')).collect::<String>()` or parse and compare entries.

#### ADV-BB-P10-HIGH-002: EC-005a outcome (`T-012 EC-005a` panic) has no control — `assert!(!tracked_raw.is_empty(), "T-012 EC-005a: ...")` neutralised passes 42/42

- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `run_t012_gate` EC-005a panic arm; T-020 covers EC-005**b**, not EC-005a
- **Description:** `assert!(!tracked_raw.is_empty(), "T-012 EC-005a: ...")` in `run_t012_gate` neutralised (→ `assert!(true, ...)`) → **42 passed**. T-020 explicitly pins EC-005b and the story says so. EC-005a is an enumerated AC outcome with zero control — a direct D-970 Cod-1 violation.
- **Status in fix wave:** CLOSED — T-049 added: `#[should_panic(expected = "T-012 EC-005a")]` with git fixture containing only `config.yaml` committed in `hook-plugins/`; neutralising the assert now kills T-049.
- **Proposed Fix:** Add `#[should_panic(expected = "T-012 EC-005a")]` test with a fixture where `hook-plugins/` contains zero `.wasm` files.

#### ADV-BB-P10-HIGH-003: `parse_plugin_refs` parse-domain equivalence with `Registry` claimed but not modelled — malformed sibling entry inerts entire production registry while T-012 passes

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `parse_plugin_refs` doc; T-012 fixture
- **Description:** `parse_plugin_refs`'s doc claims it uses "the same parser the dispatcher uses via `registry.rs` so that any TOML-legal spelling of the `plugin` key is handled identically to production." Ground truth: `Registry` and `ResolversRegistryToml` are `#[serde(deny_unknown_fields)]`; `schema_version: u32` is required and `validate()` rejects ≠ 2; `plugin: PathBuf` is required. The test reads untyped `toml::Value` and applies none of it:

  ```
  WEIRD nonstring-array  → reported=[] parse_refs=Some({})
  WEIRD nonstring-int    → reported=[] parse_refs=Some({})
  WEIRD no-plugin-key    → reported=[] parse_refs=Some({})
  WEIRD hooks-as-table   → reported=[] parse_refs=Some({})
  WEIRD dotted-plugin    → reported=[] parse_refs=Some({})
  ```

  Alone the floors catch these. **Mixed with 35 valid siblings they do not**: one entry with an unknown key, a non-string `plugin`, or `schema_version = 1` makes production reject the *entire* registry — all 75 `[[hooks]]` inert, the exact S-21.09 failure mode — while T-012 parses 35 refs, clears both floors, and returns `Ok`. Either model production's validation (run `Registry::parse_str` against the real file) or delete the equivalence claim. Documented narrowings that ARE true: non-recursive `fs::read_dir` (verified; 9 subdirectory `.toml` confirmed) and case-sensitive `.ends_with(".toml")`.
- **Status in fix wave:** CLOSED — `parse_plugin_refs` doc updated to remove the equivalence claim; a new test verifies that a fixture with a malformed sibling entry does *not* cause T-012 to pass when real `Registry::parse_str` would reject it.
- **Proposed Fix:** Remove the equivalence claim from the doc; add a test that runs `Registry::parse_str` on a fixture containing one malformed entry and asserts the entire registry is rejected.

### MEDIUM

#### ADV-BB-P10-MED-001: `hook-plugins/sub/` returns `extract=Some("hook-plugins/sub")` — directory-only declaration admitted; gate-(a) doc and T-033's closing claim both false

- **Severity:** MEDIUM
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` gate-(a) predicate; T-033
- **Description:** Gate (a) is documented as "has a filename component after `hook-plugins/`" and T-033's Expected Outcome generalises to "directory-only declaration is correctly excluded before any name is extracted." Both are false: the threshold requires `>= expected_depth + 2` components, meaning "has **at least one** component after `hook-plugins/`." A one-level-deep directory declaration like `hook-plugins/sub/` satisfies that threshold — `extract_hook_plugin_name` returns `Some("hook-plugins/sub")`, the name enters `declared_set`, and a spurious `MISSING: hook-plugins/sub` fires when no matching `.wasm` is tracked. Loud, but the gate semantics, the closing claim in the spec, and T-033's framing are all wrong. No control asserts the directory-exclusion claim.
- **Status:** OPEN — not addressed in fix wave through `1c59a669`.
- **Proposed Fix:** Add a `!declaration.ends_with('/')` guard to `extract_hook_plugin_name` gate-(a); update T-033 to assert `hook-plugins/sub/` → `extract=None`; update the gate-(a) doc.

#### ADV-BB-P10-MED-002 `[process-gap]`: Three stale intra-file doc claims left beside their own corrections — TD-VSDD-060 sibling-sweep miss from pass-10 fix wave

- **Severity:** MEDIUM
- **Category:** code-quality
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` inline doc comments
- **Description:** Three doc-comment passages were not updated when their neighbouring code was rewritten in the pass-10 fix wave: (1) `extract_hook_plugin_name`'s doc says `detect_ungated_declarations` "uses a lower gate-1 threshold (`expected_depth + 1`)" while `detect`'s own doc says that gate "is gone from this function" — both describe a prior version; (2) `detect_ungated_declarations`'s doc says "the gate is silent on [out-of-repo absolutes]" while the function hard-fails them via `OUTSIDE-REPO-DECLARATION`; (3) `parse_plugin_refs`'s table calls absolute paths "excluded" rather than a hard error.
- **Status in fix wave:** CLOSED — three stale doc passages updated to match current behaviour.
- **Proposed Fix:** Update all three doc passages to reflect current code.

#### ADV-BB-P10-MED-003: `REGISTRY_PARENT_PREFIX` "declared once here (TD-VSDD-060 sibling-site discipline)" — four sibling literals exist; the "single declaration" claim is false

- **Severity:** MEDIUM
- **Category:** code-quality
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `REGISTRY_PARENT_PREFIX` constant; sibling literals in git pathspecs, `root.join("plugins/vsdd-factory")`, and `workspace_root()`'s marker
- **Description:** `REGISTRY_PARENT_PREFIX`'s docblock claims it is "declared once here (TD-VSDD-060 sibling-site discipline) so that a mutation to one copy is caught by ALL callers." The strip via this constant is load-bearing (M13 kills T-034). However, the same literal is hard-coded in both git pathspecs, `root.join("plugins/vsdd-factory")`, and `workspace_root()`'s marker file path. The "declared once" claim is false; a mutation to one of the four sibling literals would NOT be caught by the tests guarding `REGISTRY_PARENT_PREFIX`. The TD-VSDD-060 discipline is documented but not enacted.
- **Status in fix wave:** CLOSED — sibling literals consolidated to reference `REGISTRY_PARENT_PREFIX`.
- **Proposed Fix:** Replace all four sibling literals with references to `REGISTRY_PARENT_PREFIX`; update the docblock to cite the actual reference count.

#### ADV-BB-P10-MED-004: T-047's boundary proof is over-determined — M2 (`len >` → `len >=`) leaves T-047 green; prefix conjunct also fails at root depth; length conjunct not load-bearing

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` T-047; story §AC T-047 closing claim
- **Description:** The story asserts T-047 proves "`len > root_parts.len()` fails at root depth" and that T-046/T-047 are "deliberately symmetric about the containment boundary." M2 (`len >` → `len >=`) leaves T-047 green: the T-047 fixture also fails the prefix conjunct simultaneously. A control that survives inversion of the boundary operator proves nothing about the boundary. No control isolates the prefix conjunct at all; neither the length boundary nor the prefix boundary is independently asserted.
- **Status:** OPEN — not addressed in fix wave through `1c59a669`.
- **Proposed Fix:** Split T-047 into two tests: T-047a isolates the length conjunct (path has correct prefix but `len == root_parts.len()`; only the length check can fail); T-047b isolates the prefix conjunct (path has `len > root_parts.len()` but wrong prefix; only the prefix check can fail). Update the story's closing claim.

#### ADV-BB-P10-MED-005 `[PRE-EXISTING, .factory/]`: POLICY 14 leg-4 FAIL — `STORY-INDEX.md` `version: 4.299` vs `last_amended (v4.295)` at review time

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `.factory/stories/STORY-INDEX.md` frontmatter
- **Description:** Literal execution of POLICY 14's own 4-index gate at `b951461a` review time:

  ```
  PASS specs/behavioral-contracts/BC-INDEX.md v=4.56 la=4.56
  PASS specs/verification-properties/VP-INDEX.md v=2.76 la=2.76
  FAIL stories/STORY-INDEX.md v=4.299 la=4.295
  PASS specs/architecture/ARCH-INDEX.md v=3.55 la=3.55
  ```

  The `last_amended` field at v4.295 was stale relative to `version: 4.299`; four bump-cycles (v4.296–v4.299) were unrecorded. This is a `.factory/` defect, not a branch defect; flagged as PRE-EXISTING.
- **Status:** CLOSED — STORY-INDEX `last_amended` updated with retrospective v4.296–v4.299 entries in session-wrap commit `d36c5844`.
- **Resolution:** Sealed in wrap commit `d36c5844`; POLICY 14 leg-4 now passes.

#### ADV-BB-P10-MED-006: `collect_orphans_hooks_only` key construction uncontrolled — M22 (`format!("hook-plugins/{}", ...)` → `format!("bogus/{}", ...)`) passes 42/42; T-008 is one-sided

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `collect_orphans_hooks_only`; T-008
- **Description:** `collect_orphans_hooks_only` was modified by this branch. M22 (`format!("hook-plugins/{}", ...)` → `format!("bogus/{}", ...)`) → **42 passed**. T-008 is the stated negative control for this function but only asserts the "IS orphan" direction; with a bogus key every file is orphan, so it still passes. Nothing asserts that a hooks-declared WASM is recognised as NOT orphan. (`collect_orphans_dual`'s equivalents are controlled — M21 and M23 each kill T-006 + T-010.)
- **Status in fix wave:** CLOSED — T-008 extended with a NOT-orphan assertion; M22 now kills the extended T-008.
- **Proposed Fix:** Add `assert!(!orphans.contains("hook-plugins/<known-declared-name>.wasm"))` to T-008.

### LOW

#### ADV-BB-P10-LOW-001: NUL byte and trailing-space names admitted verbatim — `hook-plugins/x.wasm\0` and `hook-plugins/x.wasm ` return `Some(...)` and enter `declared_set`, producing spurious `MISSING:` reports

- **Severity:** LOW
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `extract_hook_plugin_name`, `lex_norm`
- **Description:** `hook-plugins/x.wasm\0` → `extract=Some("hook-plugins/x.wasm\0")` (from totality probe). NUL bytes and trailing spaces are admitted verbatim into `declared_set`, producing a false-positive `MISSING:` for a name no standard filesystem can hold. The true non-UTF8 byte is unreachable through TOML (TOML mandates UTF-8), but a NUL embedded in a UTF-8 string is valid UTF-8 and can appear. Likewise `hook-plugins/x.wasm ` (trailing space): `extract=Some("hook-plugins/x.wasm ")`.
- **Status:** OPEN — not addressed in fix wave through `1c59a669`.
- **Proposed Fix:** Add NUL-byte rejection and trailing-whitespace trimming to `lex_norm` as first-pass normalisation; add proptest candidates covering these forms.

#### ADV-BB-P10-LOW-002: `detect_ungated_declarations` fail-open arms (`unwrap_or_default`, `Err(_) => Vec::new()`) guarded only by unasserted call-ordering assumption

- **Severity:** LOW
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `detect_ungated_declarations` — `fs::read_to_string(...).unwrap_or_default()` and `Err(_) => Vec::new()`
- **Description:** `detect_ungated_declarations` contains two fail-open arms: `fs::read_to_string(...).unwrap_or_default()` and a TOML `Err(_) => Vec::new()`. These are safe only because `parse_plugin_refs` panics earlier in `run_t012_gate` on an unreadable file; that ordering is load-bearing and unasserted. If a caller invokes `detect_ungated_declarations` directly (as tests do), the fail-open arms are reachable with no prior panic. Neither arm is documented with `// INVARIANT:` nor guarded with `debug_assert!`.
- **Status:** OPEN — not addressed in fix wave through `1c59a669`.
- **Proposed Fix:** Document both arms with `// INVARIANT: caller guarantees the registry file is readable (run_t012_gate ensures this via parse_plugin_refs)` and add `debug_assert!(/* file readable */)` guards; add a test verifying fail-closed behaviour when the invariant is violated.

#### ADV-BB-P10-LOW-003: `workspace_root()` has no direct test — covered only transitively via T-012; a regression manifests as a confusing containment-predicate failure

- **Severity:** LOW
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` — no unit test for `workspace_root()`
- **Description:** `workspace_root()` was introduced as part of the worktree-root containment predicate in pass-9. It is exercised only indirectly through T-012's integration path. No unit test directly verifies that `workspace_root()` returns the correct path for a repository root, a nested worktree, and a path outside any repository. A regression (e.g., returning the wrong directory level when `.git` is a gitlink file rather than a directory) would manifest as a spurious `OUTSIDE-REPO-DECLARATION` from the containment predicate rather than a `workspace_root() returned unexpected value` failure, making the root cause difficult to diagnose.
- **Status:** OPEN — not addressed in fix wave through `1c59a669`.
- **Proposed Fix:** Add three unit tests: `workspace_root_at_repo_root`, `workspace_root_in_nested_worktree` (`.git` as gitlink), `workspace_root_outside_any_repo` (returns `Err`).

### NIT

#### ADV-BB-P10-NIT-001: Story's "52 production registry entries" is stale — actual count is 75 `[[hooks]]` + 1 `[[resolvers]]` = 76 entries / 36 unique names

- **Severity:** NIT
- **Category:** spec-fidelity
- **Location:** `.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md` §2c
- **Description:** Story §2c says "all **52** production registry entries use `hook-plugins/<name>.wasm`." Actual at `b951461a`: 75 `[[hooks]]` + 1 `[[resolvers]]` = 76 entries; 36 unique names. The figure 52 appears to be an inherited rc.16 count that predates the hooks expansion. AC-001..AC-007 are all satisfiable; AC-002/003/004/005 are bats-mechanised but rely on this figure for floor assertion values.
- **Status in fix wave:** CLOSED — story §2c corrected in S-21.09 story spec v1.21 pass (part of session-wrap follow-up); floor assertion values updated accordingly.
- **Proposed Fix:** Update story §2c to reflect current production count; update any assertion values derived from the 52-entry figure.

---

## Summary

| Severity | Count | Open after `1c59a669` | Closed in fix wave |
|----------|-------|----------------------|-------------------|
| BLOCKER | 2 | 0 | 2 (in `1c59a669`: single-copy detect + T-048) |
| HIGH | 3 | 0 | 3 (HIGH-2 via T-049; HIGH-1/3 in earlier commits) |
| MEDIUM | 6 | 2 (MED-001, MED-004) | 4 (MED-002/003/005/006) |
| LOW | 3 | 3 | 0 |
| NIT | 1 | 0 | 1 |

**Overall Assessment:** block
**Convergence:** spec-vs-reality drift: **zero**; pre-existing-code defects: **zero**; remaining: 2 MEDIUM (directory-only staging control; T-047 over-determined boundary proof) + 3 LOW (NUL/trailing-space; fail-open arms; `workspace_root()` unit coverage)
**Readiness:** requires revision

The two BLOCKERs follow the same structural pattern: (1) the containment predicate's four legs are present but three survive mutation independently — M1+M4 composed re-opens the silent-drop class the story exists to prevent; (2) the totality invariant rests on a hand-duplicated second copy of `extract_hook_plugin_name`'s gates inside `detect_ungated_declarations`, asserted by no test. Both are confirmed CLOSED in `1c59a669` (single-copy refactor + T-048 property assertion). BC-5.39.001 3-CLEAN protocol requires zero findings of any severity for a CLEAN pass; streak remains 0/3 after ten passes.

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 10 |
| **Story version reviewed** | v1.19 |
| **Reviewed commit** | b951461a |
| **New findings (BLOCKERs closed in `1c59a669`)** | 2 (BLK-001 mutation-uncontrolled predicate; BLK-002 unasserted totality invariant) |
| **New findings (open after `1c59a669`)** | 5 (MED-001/004; LOW-001/002/003) |
| **New findings (closed in fix wave)** | 8 (BLK-001/002 + HIGH-001/002/003 + MED-002/003/006 + NIT-001) |
| **Carry-over findings** | 0 (all P09 items resolved or reclassified; ADR-043-gated deferred-dropped) |
| **Resolved vs. prior pass** | 8 carry-overs: P09-BLK-001 verified; P09-HIGH-001 resolved; P09-MED-002/003 resolved; P09-LOW-002 resolved; P09-NIT-001 resolved; P09-LOW-001 reclassified; P09-MED-001 deferred-dropped |
| **Mutation testing** | M1-M4 survive independently (BLK-001); M1+M4 composed reproduces silent-drop class; M22 survives T-008 (MED-006). T-048 kills mutation on single-call-site path. |
| **Novelty score** | 15 / (15 + 0) = 1.00 (no carry-overs) |
| **Median severity** | MEDIUM |
| **Severity trajectory (HIGH)** | 3→2→3→2→1→1→3→2→1→3 |
| **Total finding trajectory** | →9→9→8→8→15 (pass-7: 9; pass-8: 8; pass-9: 8; pass-10: 15 — regression driven by BLK+HIGH discovery) |
| **Verdict** | FINDINGS_REMAIN |
