---
document_type: adversary-pass
level: ops
pass: 1
cascade: LOCAL
story: S-15.07
producer: adversary
timestamp: 2026-05-16T00:00:00Z
diff_base: c62f952c
diff_head: eb327a77
verdict: MEDIUM
finding_count_by_severity:
  critical: 0
  high: 0
  medium: 2
  low: 3
  nitpick: 0
policies_evaluated: [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18]
---

# S-15.07 LOCAL Adversary Pass-1

## Verdict

**MEDIUM** — Implementation broadly satisfies BC-5.39.003 and AC-1..AC-11, but two MEDIUM findings (paper-fix in violation-line formatter; minor-version zero-pad creates body-vs-block-message string mismatch) and three LOW findings (test-fixture documentation drift; missing bats coverage for EC-008 multi-violation accumulation; tool-attribute convention inconsistency) require remediation before 3-CLEAN.

## Findings Table

| ID | Severity | Policy-Triggered | Category | Summary | Recommended Action |
|----|----------|------------------|----------|---------|--------------------|
| F-S15.07-LOCAL-P1-001 | MEDIUM | POLICY 4 (semantic_anchoring_integrity); TD-VSDD-059 paper-fix | spec-vs-impl drift | `format_violation()` uses `v.source` for BOTH the bracketed prefix AND the "location" placeholder, producing redundant `[ARCH-INDEX.md] ARCH-INDEX.md cites ...`. Spec §Block message format calls for `v.location` (per-cite line/cell location within the source). Violation struct lacks the `location` field entirely. | Add `pub location: String` field to `Violation`; populate during extraction (line number or section heading); update `format_violation` to consume `v.location` for the second placeholder. OR amend spec block-message contract to a single-source form and update the `format_violation` arity to match. |
| F-S15.07-LOCAL-P1-002 | MEDIUM | POLICY 4 (semantic_anchoring_integrity) | block-message-quality | `format_violation` formats minor version with `{:02}` (zero-pad to 2 digits). When body cite is single-digit minor (e.g., `BC-INDEX v1.5`), the block message says "cites BC-INDEX v1.05" — author must hunt for "v1.05" but body contains "v1.5". Spec §Block message format used `v{}.{}` (no pad). | Drop the `{:02}` width specifier on the cited minor; preserve as-written form. Keep zero-pad ONLY on the live-version side if a canonical printed form is required, but the cited-version output must be byte-identical to the body string the user wrote. |
| F-S15.07-LOCAL-P1-003 | LOW | none | test-doc drift | `tests/validate-index-cite-refresh/README.md` (lines 36-39) states fixtures use `factory/` and `setup()` renames `factory/` → `.factory/`. Actual `_setup_fixture()` in every .bats file does `cp -r "$FIXTURE_SRC/factory/." "$WORK/.factory/"` — no rename; tmpdir always has `.factory/` pre-created by `mkdir -p "$WORK/.factory/logs"` in `setup()`. README misdescribes test mechanic. | Edit README §"Fixture Layout" to describe the actual copy-into-.factory mechanic, OR change `_setup_fixture` to actually rename `factory/` → `.factory/` as README claims. |
| F-S15.07-LOCAL-P1-004 | LOW | POLICY 11 (no_test_tautologies) | test-coverage | EC-008 (multiple stale cites in one Edit accumulating into a single block message enumerating all violations) has NO bats fixture or test. Only `format_violation` is exercised in unit tests; the accumulate-then-emit pipeline is not exercised end-to-end with multi-violation input. | Add `fail-multi-stale-cites/` fixture (ARCH-INDEX body cites BOTH `BC-INDEX v1.05` AND `STORY-INDEX v3.28` stale) + `fail-multi-stale-cites.bats` asserting exit==2 AND output contains both "BC-INDEX" AND "STORY-INDEX" AND the "stale cite(s) found" count is >=2. |
| F-S15.07-LOCAL-P1-005 | LOW | none (pending intent verification) | convention-drift | hooks-registry.toml new entry uses `tool = "Write&#124;Edit"` while majority of registry uses `tool = "Edit&#124;Write"` (15 of ~17 entries are Edit-first; only `validate-artifact-path` and the new `validate-index-cite-refresh` use Write-first). Implementer matched the adjacent neighbor convention, but the project canonical form appears to be `Edit&#124;Write`. | Either align to majority canonical form `Edit&#124;Write`, or escalate to orchestrator to confirm `Write&#124;Edit` is acceptable. Tag (pending intent verification). |

## Finding Details

### F-S15.07-LOCAL-P1-001 — MEDIUM — format_violation source-doubling (paper-fix detection)

**Anchor:** `format_violation` function in `crates/hook-plugins/validate-index-cite-refresh/src/lib.rs`

**Evidence (verbatim file:line: from Grep stdout):**

```
401:fn format_violation(v: &Violation) -> String {
402-    format!(
403-        "  [{}] {} cites {} v{}.{:02} but live version is v{}.{:02}. Update cite to v{}.{:02}.",
404-        v.source,
405-        v.source,
406-        v.index_name.as_str(),
```

Spec §Block message format (story spec line 583-595):

```
"  [{}] {} cites {} v{}.{} but live version is v{}.{}. Update cite to v{}.{}.",
v.source, v.location, v.index_name, v.cited.0, v.cited.1,
v.live.0, v.live.1, v.live.0, v.live.1
```

The struct `Violation` in lib.rs (lines 103-114) has no `location` field — only `source`, `index_name`, `cited`, `live`. The implementer met the format-string-arity requirement by passing `v.source` twice instead of adding the spec-required `location` field. Block output reads: `[ARCH-INDEX.md] ARCH-INDEX.md cites BC-INDEX v1.05 but live version is v2.24. Update cite to v2.24.` — the source-name is redundantly duplicated and there is no per-cite location indicator (line number, section heading, or table-cell identifier) to help the author locate the stale cite within a large ARCH-INDEX.

**Rationale:** This is TD-VSDD-059 paper-fix detection. The spec explicitly required a `location` field for cite locality, and the implementation closed the spec by renaming the placeholder use (doubled `v.source`) instead of plumbing the actual data. Bats tests pass on substring `"ARCH-INDEX"` / `"BC-INDEX"` / `"1.05"` / `"2.24"` containment, so the deficiency is invisible to AC-10's substring assertions. POLICY 4 (semantic_anchoring_integrity): the `Violation` struct's anchor to spec §Block message format is syntactically partial — the format ARITY matches, but the SEMANTIC field-content does not.

**Recommended fix:** Add `pub location: String` to `Violation`. Populate during ARCH-INDEX extraction with either (a) the markdown section heading containing the cite (e.g., "Index Version Summary"), or (b) a 1-based line number within the source document. Update `format_violation` to consume `v.location` as the second placeholder. Sibling-sweep: STATE.md and INDEX.md cross-cell paths in `cross_cell_check` must also populate `location` (e.g., "Convergence Trajectory section" or line number).

---

### F-S15.07-LOCAL-P1-002 — MEDIUM — `{:02}` zero-pad creates body-vs-message string mismatch

**Anchor:** `format_violation` function (same anchor as F-001)

**Evidence (verbatim file:line: from Grep stdout):**

```
403-        "  [{}] {} cites {} v{}.{:02} but live version is v{}.{}. Update cite to v{}.{}.",
```

The format spec `{:02}` zero-pads minor version to width 2. If body cite is `BC-INDEX v1.5` (parsed as major=1, minor=5), the block message will read `cites BC-INDEX v1.05`. Author Ctrl-F's "v1.05" in ARCH-INDEX body; nothing matches because body contains literal `v1.5`.

Spec §Block message format used the bare `v{}.{}` form (no width specifier), preserving the as-written cited form.

**Rationale:** The implementer applied `:02` presumably to canonicalize printed form (so `v1.5` and `v1.05` both render as `v1.05`). But the BLOCK MESSAGE's role is to help the author find and fix the stale cite — string-mismatch defeats that. The vsdd-factory convention DOES use two-digit minors (BC-INDEX v2.24, STORY-INDEX v3.31), so in steady state this is a low-impact UX issue. But it's a spec deviation and a real edge-case bug for any future cite that uses single-digit minor.

**Recommended fix:** Remove `:02` from the CITED side of the format string. Live-version side may keep `:02` only if rendering canonical form is desired. Preferred: bare `v{}.{}` on both sides matching the spec exactly.

---

### F-S15.07-LOCAL-P1-003 — LOW — README test-mechanic doc drift

**Anchor:** `tests/validate-index-cite-refresh/README.md` "Fixture Layout" section

**Evidence (verbatim file:line: from Read of README.md lines 36-39):**

```
36	Each fixture uses `factory/` as a directory name (not `.factory/`) to avoid `factory-branch-guard`
37	hook interference during test authoring. The bats `setup()` function copies the fixture to a tmpdir
38	and renames `factory/` to `.factory/` before running the dispatcher+WASM invocation. This ensures
39	the WASM hook sees the expected `.factory/...` path structure via `host::read_file`.
```

Actual fixture tree (from Glob):
```
plugins/vsdd-factory/tests/fixtures/validate-index-cite-refresh/pass-all-current/factory/specs/...
```

Actual `_setup_fixture()` in `pass-all-current.bats` line 26-28:
```
_setup_fixture() {
  cp -r "$FIXTURE_SRC/factory/." "$WORK/.factory/"
}
```

The actual mechanic is "copy contents of fixture's `factory/` INTO pre-created `$WORK/.factory/`" — there is no rename. README describes a different mechanic than the code performs.

**Rationale:** Minor doc drift. Has no functional impact; tests pass with the actual mechanic. Worth correcting before merge for downstream story-writers (S-15.11, S-15.09, S-15.14) who will copy this pattern.

**Recommended fix:** Edit README §Fixture Layout to: "The bats `setup()` function pre-creates `$WORK/.factory/` then copies the fixture's `factory/` directory contents into it via `cp -r $FIXTURE_SRC/factory/. $WORK/.factory/`."

---

### F-S15.07-LOCAL-P1-004 — LOW — Missing bats coverage for EC-008 multi-violation accumulation

**Anchor:** Test suite `tests/validate-index-cite-refresh/` directory; spec §Edge Cases EC-008

**Evidence (verbatim Glob stdout):**

```
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-cross-cell-index-md.bats
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-cross-cell-state-md.bats
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-open-missing-index.bats
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-stale-bc-index.bats
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-stale-story-index.bats
plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-stale-vp-index.bats
plugins/vsdd-factory/tests/validate-index-cite-refresh/pass-all-current.bats
```

Spec §Edge Cases EC-008 (story line 385): "ARCH-INDEX.md written with multiple stale cites in one Edit (e.g. both BC-INDEX and STORY-INDEX stale) → Hook accumulates all violations into a single BlockWithFix message listing all stale cites."

BC-5.39.003 §Canonical Test Vectors (line 132): "Multiple stale cites | ARCH-INDEX cites both BC-INDEX and STORY-INDEX stale | Single HookResult::BlockWithFix enumerating both violations | BLOCK".

No bats fixture or test exercises the multi-violation accumulation pipeline end-to-end. Unit test `test_BC_5_39_003_emit_block_names_index_and_versions` constructs a single-violation Vec; no test constructs a multi-element Vec to verify newline-joining or count formatting (`"{} stale cite(s) found"`).

**Rationale:** POLICY 11 (no_test_tautologies) is partially in scope: unit tests for `emit_block` test the formatter, but the end-to-end accumulate-extract-emit path is not exercised on multi-violation input. EC-008 is a documented canonical test vector with no test asserting it.

**Recommended fix:** Add fixture `fail-multi-stale-cites/` with ARCH-INDEX body containing TWO stale cites (e.g., `BC-INDEX v1.05` AND `STORY-INDEX v3.28`). Add `fail-multi-stale-cites.bats` asserting exit==2, output contains BOTH `"BC-INDEX"` AND `"STORY-INDEX"`, AND output contains `"2 stale cite(s) found"` (verbatim count).

---

### F-S15.07-LOCAL-P1-005 — LOW (pending intent verification) — `tool` attribute convention drift

**Anchor:** `plugins/vsdd-factory/hooks-registry.toml` new validate-index-cite-refresh entry

**Evidence (verbatim file:line: from Grep stdout):**

```
100:tool = "Edit|Write"
121:tool = "Edit|Write"
155:tool = "Edit|Write"
175:tool = "Edit|Write"
195:tool = "Edit|Write"
[...15 total Edit|Write entries...]
844:tool = "Write|Edit"   (validate-artifact-path)
861:tool = "Write|Edit"   (validate-index-cite-refresh — new entry)
```

The dominant convention is `Edit|Write` (15+ entries). Only `validate-artifact-path` and the new `validate-index-cite-refresh` use `Write|Edit`. The implementer matched the adjacent neighbor (`validate-artifact-path`) — defensible choice — but it propagates a minority convention.

**Rationale:** Per Partial-Fix Regression Discipline (S-7.01) the adversary cannot adjudicate authorial intent. The implementer's choice (match adjacent neighbor) is reasonable. Reporting under (pending intent verification) per the discipline.

**Recommended fix:** Orchestrator/architect adjudicates: align all `validate-*` PostToolUse hooks to `Edit|Write` (sweep validate-artifact-path too), OR accept the heterogeneous state. Either way, document the convention authoritatively so future M2 stories (S-15.11, S-15.09, S-15.14) don't propagate the drift further.

---

## Observations

### O-S15.07-LOCAL-P1-001 — AC-9 `file_pattern` literal-verification cannot pass (process gap)

[process-gap] AC-9 requires `grep -A10 'validate-index-cite-refresh' plugins/vsdd-factory/hooks-registry.toml` to confirm `file_pattern matching ARCH-INDEX.md` is present. Verified via Grep on the dispatcher schema sources: the registry schema does not include a `file_pattern` field. The implementer correctly implemented file-pattern filtering in-plugin via `if !file_path.ends_with("ARCH-INDEX.md") { return Continue; }` (lib.rs `on_post_tool_use` function, "Only act on writes to ARCH-INDEX.md" guard). This is the only mechanism available given the schema, and the dispatch package §Step 4 explicitly anticipates this resolution. The AC-9 verification command as written cannot return the expected output. Recommend amending the story spec post-merge OR amending future story templates to use a more flexible AC-9 verification predicate (e.g., "grep confirms file-path filtering present EITHER in registry `file_pattern` OR in plugin's tool_input handler").

### O-S15.07-LOCAL-P1-002 — Cross-cycle path hardcode legitimately scope-deferred

Cross-cell INDEX.md path `.factory/cycles/v1.0-brownfield-backfill/INDEX.md` is hardcoded in `cross_cell_check` (lib.rs line 363). Story spec §Implementation Detail and §Risk both acknowledge this as a known limitation, and §Scope §Not in scope explicitly defers cross-cycle propagation automation to S-15.10 in M3. This satisfies Canonical Principle Rule 3 (human-directed deferral with concrete future-story attachment to S-15.10). Not a finding.

### O-S15.07-LOCAL-P1-003 — Coverage gap: hook only fires on ARCH-INDEX writes, not STATE.md or INDEX.md writes

Per BC-5.39.003 invariant 2 and the registry entry (event=PostToolUse, file-filter=ARCH-INDEX.md only), STATE.md cross-cell drift is detected ONLY as a side-effect of an unrelated ARCH-INDEX write. If state-manager bumps STATE.md trajectory cell but doesn't touch ARCH-INDEX, the drift goes undetected until the next ARCH-INDEX edit. This is per-spec design but creates an asymmetric guard. Suggest documenting in the BC body that STATE.md/INDEX.md drift detection latency = "until next ARCH-INDEX edit". Not blocking.

### O-S15.07-LOCAL-P1-004 — Workspace Cargo.toml members order non-alphabetical (preexisting)

Story task T-4 specifies "maintain alphabetical order within the hook-plugins group" but the existing `members` array is not alphabetical (`block-ai-attribution` before `handoff-validator` is OK, but `validate-artifact-path` precedes `lint-registry-async-invariant` and `validate-per-story-adversary-convergence`). The new entry was inserted in a non-alphabetical adjacency. Preexisting state, not a regression from this story.

---

## Part B — Pass-Internal Notes (NOT visible to subsequent passes)

### Convergence Streak Status

- **Entering pass-1**: 0/3 (cascade start)
- **After pass-1 verdict (MEDIUM with 2 MEDIUM + 3 LOW findings)**: 0/3 (streak does not advance; MEDIUM findings reset/hold per BC-5.39.001 3-CLEAN protocol)

Per S-15.08 cascade convention disambiguation: "LOW does NOT reset; only MEDIUM+ resets." Two MEDIUM findings exist → streak resets/holds at 0/3.

### Implementer/Test-Writer/Story-Writer Collaboration Meta-Observation

- The story-writer produced a remarkably detailed spec including the specific block-message format string. The implementer followed the format-string ARITY faithfully but skipped plumbing the `Violation::location` field — classic paper-fix where the implementer hit the spec's surface form but missed the substance. This is TD-VSDD-059 territory.
- Test-writer's bats stubs are well-structured and properly use the `@pending` → activation pattern. Coverage is good for 7 of the spec's documented Acceptance Criteria but misses EC-008 multi-violation accumulation.
- Hand-rolled scanner (no `regex` crate) is correctly defended against regex-crate WASM fuel cost — see Cargo.toml comment lines 26-28. UTF-8 multi-byte safety in `extract_index_cites` (lines 195-204) is explicitly tested via `test_BC_5_39_003_extract_handles_multibyte_chars_no_panic` — implementer learned from the recovery-implementer fix history correctly.
- README documentation drift is a minor pattern — the test-writer wrote a README describing one mechanic, the implementer wrote bats code implementing another. Both work but they diverge.

### Recommendation to Orchestrator

**Dispatch fix-burst** before pass-2 — two MEDIUM findings require remediation.

Routing by finding:
- F-001 (Violation.location field): **implementer** — adds struct field, plumbs location during extraction, sibling-sweeps cross_cell_check
- F-002 (zero-pad mismatch): **implementer** — format-string adjustment
- F-003 (README drift): **technical-writer** OR **implementer** (single-line README edit)
- F-004 (EC-008 bats coverage): **test-writer** — new fixture + bats file
- F-005 (`Write|Edit` vs `Edit|Write`): **architect** (intent adjudication) → then **state-manager** for sweep if architect decides to align

After fix-burst lands on `feature/S-15.07-index-cite-refresh-hook`, dispatch adversary pass-2 (fresh context, Iron Law).
