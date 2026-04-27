---
name: validate-consistency
description: Cross-file consistency validation — verify spec IDs, anchor links, counts, naming, and traceability across all planning artifacts. Catches stale references, broken links, and mismatched counts.
disable-model-invocation: true
allowed-tools: Read, Bash, Glob, Grep
---

# Validate Consistency

Check cross-file consistency across all factory artifacts.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/consistency-report-template.md` — consistency validation report

## Checks

### 1. BC ID Integrity

- Every BC file in `behavioral-contracts/` is listed in BC-INDEX.md
- Every BC referenced in prd.md exists as a file
- No duplicate BC IDs
- BC numbering follows S.SS.NNN convention (no gaps expected, but no duplicates)

### 2. VP ID Integrity

- Every VP file in `verification-properties/` is listed in VP-INDEX.md
- Every VP referenced in architecture docs exists as a file
- VP status matches across index and file

### 3. Story Traceability

- Every story references at least one BC
- Every BC is referenced by at least one story (no orphan BCs)
- Story dependencies in dependency-graph.md match story file contents
- Wave assignments are consistent between story files and wave-schedule.md

### 4. Architecture Cross-References

- ARCH-INDEX.md lists all ARCH-NN files that exist
- No ARCH-NN files exist that aren't in the index
- Architecture section references in BCs point to valid sections

### 5. Count Consistency

- Story count in STORY-INDEX.md matches actual story files
- BC count in BC-INDEX.md matches actual BC files
- Epic story counts in epics.md match actual stories per epic

### 6. Status Consistency

- sprint-state.yaml statuses match STORY-INDEX.md statuses
- BC-INDEX.md statuses match individual BC file statuses

### 7. Naming Consistency

- Entity names in stories match domain-spec/ubiquitous-language.md
- Module names in architecture match story file lists

---

## Advisory Checks (non-blocking)

The following checks emit advisory findings only — they appear under `## Advisories` in the report and never count toward `Failed`. They are heuristic by nature; treat them as code-smell signals rather than gates. Both were motivated by a real production cycle (Prism Wave 2 Pass 7 — TD-W2-FIXK-001 + TD-W2-FIXK-002) where the patterns slipped through 6 prior adversarial passes.

### 8. Test Tautology Detector (advisory, severity: MEDIUM)

**Goal:** flag tests that look like behavior tests but only assert on data they themselves constructed — the test would pass even if the production code under test were deleted.

**Scope:**
- Rust test functions whose names match `^test_BC_`, `^test_TV_`, `^test_.*_BC_`, or `^test_.*_TV_` (case-sensitive prefix; case-insensitive `_BC_` / `_TV_` segment).
- Search root: `**/tests/**/*.rs`, `**/src/**/*.rs` files containing `#[cfg(test)]` modules, and `**/test_*.rs` / `**/*_test.rs`.

**Tautology shape (FLAG when ALL three hold):**
1. Body contains at least one struct literal construction (`let <ident> = <Path::>SomeStruct { ... };` or `Path::SomeStruct { ... }` returned/bound).
2. Body contains one or more `assert_eq!(<ident>.<field>, ...)`, `assert!(<ident>.<field>...)`, or `assert_matches!(<ident>.<field>, ...)` calls where `<ident>` is the locally-constructed value.
3. Body contains **zero calls** to any production-named function whose ident matches `(emit|process|apply|handle|execute|validate|compute|transform|render|serialize|encode|decode|parse|build|generate)_\w+`. Calls in `use` statements or comments don't count. Calls inside `assert_eq!(left, right)` where `right` is a production fn count as passing the gate.

**Allowed exceptions (DO NOT flag):**
- Tests named `test_BC_*_struct_shape` or `test_TV_*_fixture` are explicit data-shape pins; recognized only if the function has a doc-comment containing `// data-shape pin` or `/// data-shape pin`.
- Tests where the constructed struct is passed as an argument to a production fn (e.g., `emit_event(&entry)`) — this is the non-tautological case.
- Tests inside a `#[cfg(test)]` module where the file's top-level doc-comment contains `//! tautology-allowed: <reason>`.

**Detection procedure:**
1. `rg -nU --type rust 'fn (test_(?:[A-Za-z_0-9]*_)?(?:BC|TV)_[A-Za-z_0-9]+)\s*\([^)]*\)\s*\{' -o` to enumerate candidate functions. The non-capturing group ensures only the literal `BC` or `TV` segment matches — not `BV` or `TC`.
2. For each match, extract the function body (balanced braces, simple line counter from `{` to matching `}`).
3. Apply the three-condition predicate above. Production-fn regex: `\b(emit|process|apply|handle|execute|validate|compute|transform|render|serialize|encode|decode|parse|build|generate)_[a-z][a-z0-9_]*\s*\(`.
4. Apply exception filters; emit one advisory per surviving match.

**Advisory output (one row per finding):**
```
| File | Function | Evidence | Suggestion |
|------|----------|----------|------------|
| crates/foo/tests/bc.rs:42 | test_BC_3_02_001_emits_entry | Constructs `LogEntry { ts: 0, level: "info" }`, asserts on `entry.ts` and `entry.level`, no call to `emit_*`/`serialize_*`/`build_*` | Test exercises the assertion but not production code. Consider calling `emit_log_entry(&entry)` or invoking the actual emitter under test. If this is intentional (data-shape pin), add `/// data-shape pin` to the function. |
```

**Reference fixtures:** see `${CLAUDE_PLUGIN_ROOT}/skills/validate-consistency/fixtures/tautology/` for canonical positive (flagged) and negative (clean) examples.

### 9. BC Canonical Test Vector ↔ Emitter Field Consistency (advisory, severity: HIGH)

**Goal:** when a BC's "Canonical Test Vectors" section explicitly excludes a field from the emitted artifact, flag any production struct/emitter that unconditionally serializes that field. This catches the "spec says don't emit X but the code emits X anyway" class of drift — exactly the bug TD-W2-FIXK-002 hit in Prism.

**Scope:**
- BC files matching `.factory/specs/behavioral-contracts/ss-*/BC-*.md` (recursive) that contain a `## Canonical Test Vectors` (or `### Canonical Test Vectors`) section.
- Any column header in those tables matching one of these shapes (case-insensitive):
  - `<FieldName> in <Shape>?`
  - `Field <FieldName> in <Shape>?`
  - `Includes <FieldName>?`
  - `Emits <FieldName>?`
  - `Has <FieldName>?`
- Treat answers `No`, `❌`, `false`, `excluded`, `omitted`, `n/a`, or `--` as "field-excluded for this row".

**Detection procedure:**
1. Parse each BC's frontmatter for `target_module:` or `architecture_module:` (if present) — this names the production crate/file. If absent, fall back to grepping the BC body for ` `crates/<name>` ` references.
2. For each "field-excluded" row, identify the row's struct anchor — typically named in the row's first cell (e.g., `LogEntry`, `EmittedEvent`, `CommitMadeEvent`).
3. In the production crate, locate the matching struct definition (`struct <Name> { ... }`). Use `rg -nU --type rust "(?m)^(pub )?struct $NAME\b"` plus N lines of context.
4. Inspect the field declaration for the excluded field name:
   - **CLEAN** if the field is missing entirely, OR is `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`, OR has `#[serde(skip_serializing)]`, OR is gated by a feature flag, OR has `#[serde(skip_serializing_if = ...)]` with any predicate.
   - **FLAG** if the field is declared as a non-Option type with no `skip_serializing*` attribute, OR is `Option<T>` without `skip_serializing_if`, OR is plainly `pub <field>: T`.
5. Cross-check by grepping the matching `impl Serialize` or `impl <Name>` block for hand-rolled serialization that emits the field unconditionally.

**Why severity HIGH:** this represents a contract-implementation drift that adversarial review missed (Prism: 6 passes clean, defect surfaced in pass 7). The data shape doesn't match what the BC says is canonical, so any downstream consumer that trusts the BC will be wrong.

**Allowed exceptions (DO NOT flag):**
- BC frontmatter contains `tv_emitter_check: skip` or `tv_emitter_check: manual`.
- The BC explicitly notes "field tolerated when present" in the row's notes column.
- Rust struct has `#[serde(skip_serializing_if = ...)]` with ANY predicate, even a custom one.

**Advisory output (one row per finding):**
```
| BC | Excluded Field | Struct | Location | Evidence | Suggestion |
|----|----------------|--------|----------|----------|------------|
| BC-3.02.013 | trace_id | LogEntry | crates/sink-core/src/entry.rs:24 | Field declared `pub trace_id: String,` with no `#[serde(skip_serializing_if)]`; BC TV row marks `trace_id in Entry? = No` for retry-exhaustion case | Either (a) make field `Option<String>` with `#[serde(skip_serializing_if = "Option::is_none")]`, (b) gate emission via a wrapper, or (c) update BC to mark field included if drift was intentional (and document the spec change). |
```

**Reference fixtures:** see `${CLAUDE_PLUGIN_ROOT}/skills/validate-consistency/fixtures/bc-tv-consistency/` for canonical positive (flagged) and negative (clean) examples.

### Advisory check operating notes

- These checks **never** flip the report's overall PASS/FAIL — only blocking checks (1-7) do. Advisories appear in their own section.
- Advisories carry severity (MEDIUM/HIGH) for prioritization but are **not** policy gates by default. A project may promote them to blocking by adding the policy id to `.factory/policies.yaml` (`POLICY-VC-008`, `POLICY-VC-009`).
- If a project has no Rust source (e.g., docs-only, TypeScript, Python), these checks are no-ops — emit one advisory line `Check 8/9: skipped (no Rust sources detected)` and move on.
- Both checks are idempotent and side-effect free; they only read.

## Templates

Use `${CLAUDE_PLUGIN_ROOT}/templates/consistency-validation-report-template.md` for the consistency validation report format.

## Output

```markdown
# Consistency Validation Report

## Summary
- Checks run: <N>
- Passed: <N>
- Failed: <N>
- Warnings: <N>
- Advisories: <N>   <!-- Check 8 + Check 9 surfaced findings; non-blocking -->

## Failures
| Check | Issue | Files Involved |
|-------|-------|---------------|
| BC ID | BC-1.01.003 in prd.md but no file exists | prd.md |

## Warnings
| Check | Issue | Files Involved |
|-------|-------|---------------|
| Orphan BC | BC-2.01.001 not referenced by any story | BC-2.01.001.md |

## Advisories

### Check 8 — Test Tautologies (MEDIUM)
| File | Function | Evidence | Suggestion |
|------|----------|----------|------------|
| <path>:<line> | <test fn name> | <what was constructed/asserted> | <how to convert to a real behavior test> |

### Check 9 — BC Canonical TV vs Emitter Drift (HIGH)
| BC | Excluded Field | Struct | Location | Evidence | Suggestion |
|----|----------------|--------|----------|----------|------------|
| BC-N.NN.NNN | <field> | <StructName> | <path>:<line> | <why it serializes anyway> | <fix or spec-update suggestion> |

## All Passed
<List of checks that passed cleanly>
```

**Reading the Advisories section:** entries are non-blocking by default. Promote to blocking by adding `POLICY-VC-008` (tautology) or `POLICY-VC-009` (BC-TV) to `.factory/policies.yaml` if the project wants the checks gating.
