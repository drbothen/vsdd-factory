---
document_type: adversary-pass
level: ops
pass: 3
cascade: LOCAL
story: S-15.07
producer: adversary
timestamp: 2026-05-16T00:00:00Z
diff_base: c62f952c
diff_head: 0487a4e6
verdict: MEDIUM
finding_count_by_severity:
  critical: 0
  high: 0
  medium: 1
  low: 1
  nitpick: 0
policies_evaluated: [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18]
---

# S-15.07 LOCAL Adversary Pass-3

## Verdict

**MEDIUM** -- Fix-burst-2 successfully closed all three pass-2 findings (F-P2-001 cited_raw plumbing, F-P2-002 bats inline-registry sweep, F-P2-003 substring tightening) with verified evidence in source. However, a fresh-context read of the F-P2-001 implementation reveals a structural anchor defect: `VersionCite::minor_raw` is named "minor_raw" and documented to contain just the minor digits (e.g. `"05"`), but actually stores `"major.minor"` (e.g. `"1.05"`). Field naming, doc-comment, and stored value disagree -- a POLICY 4 semantic-anchoring defect introduced by fix-burst-2. One LOW finding (story spec frontmatter+body still cites `Write&#124;Edit` after architect Q5 canonical lock + production sweep + bats sweep -- propagation gap).

## Findings Table

| ID | Severity | Policy-Triggered | Category | Summary | Recommended Action |
|----|----------|------------------|----------|---------|--------------------|
| F-S15.07-LOCAL-P3-001 | MEDIUM | POLICY 4 (semantic_anchoring_integrity); TD-VSDD-059 paper-fix indicator | impl naming/doc/value drift | `VersionCite::minor_raw` field name + doc-comment + stored value disagree. Field is named `minor_raw`. Doc-comment (lib.rs:102-105) says: "The raw minor string as it appeared in the source document body (e.g. `\"05\"` for `\"v1.05\"`, `\"28\"` for `\"v3.28\"`)". Actual stored value (lib.rs:206) is `format!("{major_raw}.{minor_raw}") = "1.05"` (full major.minor literal). A future maintainer reading the field would expect bare-minor string `"05"` but receive `"1.05"`. Causes downstream re-render bugs if anyone formats `v{minor_raw}` expecting it to be the minor digits. | Either: (a) rename field to `cite_raw` or `version_raw` and update doc-comment to say "raw major.minor literal as it appeared, e.g. `\"1.05\"` for `BC-INDEX v1.05`"; OR (b) split into two fields (`major_raw: String` + `minor_raw: String`) and have `Violation::cited_raw` consume both. Option (a) is cheaper and matches existing `Violation::cited_raw` field naming downstream. Either way, the `Violation::cited_raw` clone-source assignment (`cited_raw: cite.minor_raw.clone()`) is a name mismatch that the rename closes. |
| F-S15.07-LOCAL-P3-002 | LOW | TD-VSDD-060 sibling-site sweep | spec-vs-impl drift | Story spec `.factory/stories/S-15.07-validate-index-cite-refresh.md` line 282 (heading comment) and line 289 (TOML body) still cite `tool = "Write&#124;Edit"` after architect Q5 canonicalized `Edit&#124;Write`, fix-burst-1 swept production registry, and fix-burst-2 swept bats inline registries. AC-9 (line 208) literal-verification predicate also still references `tool = "Write&#124;Edit"`. Spec body now diverges from impl, production registry, and bats fixtures. Standing Rule: spec wins on conflicts but spec MUST be amended to reflect the canonical decision. | State-manager or product-owner edits S-15.07 spec lines 282, 289, 208 from `Write&#124;Edit` to `Edit&#124;Write` (3 sites). AC-9 verification predicate should also be amended to either (a) drop the `file_pattern` mention since the registry schema lacks that field (pass-1 O-001 process-gap) -- replace with "tool_input.file_path ends_with(\"ARCH-INDEX.md\") guard present in lib.rs" -- OR (b) document the deferral. Trivial single-file edit. |

## Finding Details

### F-S15.07-LOCAL-P3-001 -- MEDIUM -- VersionCite::minor_raw field name/doc/value disagree

**Anchor:** `VersionCite::minor_raw` field + doc-comment + construction site in `crates/hook-plugins/validate-index-cite-refresh/src/lib.rs`

**Evidence (verbatim file:line: from Read):**

Field declaration + doc-comment (lib.rs:94-107):
```
 94 /// A parsed version string extracted from document body text.
 95 #[derive(Debug, Clone, PartialEq, Eq)]
 96 pub struct VersionCite {
 97     pub index_name: IndexName,
 98     pub major: u32,
 99     pub minor: u32,
100     /// 1-based line number within the source document where this cite appears.
101     pub line: u32,
102     /// The raw minor string as it appeared in the source document body
103     /// (e.g. "05" for "v1.05", "28" for "v3.28").  Preserved so the block
104     /// message can reproduce the body-literal form byte-for-byte, satisfying
105     /// BC-5.39.003 EC-005 Ctrl-F UX requirement.
106     pub minor_raw: String,
107 }
```

Construction site (lib.rs:194-207):
```
194                                 parse_leading_digits(&content[after_dot..])
195                             {
196                                 // Capture raw minor slice (e.g. "05" from "v1.05").
197                                 // The major_raw is just the digit text before the dot;
198                                 // we reconstruct the full raw cite as "major_raw.minor_raw".
199                                 let major_raw = &content[after_v..after_major];
200                                 let minor_raw = &content[after_dot..after_dot + minor_len];
201                                 cites.push(VersionCite {
202                                     index_name,
203                                     major,
204                                     minor,
205                                     line: current_line,
206                                     minor_raw: format!("{major_raw}.{minor_raw}"),
207                                 });
```

Consumer site (lib.rs:391, 426, 558):
```
391                                 cited_raw: cite.minor_raw.clone(),
426                                 cited_raw: cite.minor_raw.clone(),
558                         cited_raw: cite.minor_raw.clone(),
```

Three independent disagreements in the same field:

1. **Name says "minor_raw"** -- implies only the minor portion (e.g. "05").
2. **Doc-comment example says `"05" for "v1.05"`** -- explicitly states bare-minor form.
3. **Actual stored value at line 206 is `format!("{major_raw}.{minor_raw}")` = `"1.05"`** -- full major-dot-minor.

The consumer (line 391/426/558) assigns the field to `Violation::cited_raw` which IS documented to hold major.minor form (lib.rs:125-131: `"1.05" for BC-INDEX v1.05`). So the consumer's name/doc are correct; the producer's name/doc are wrong. The shadow local at line 200 (`let minor_raw = ...`) further obscures this: the local genuinely IS the minor part, but the struct field assignment on line 206 immediately combines it with `major_raw` and stores the combined form under the same name. Reader reasoning at the call-site is broken.

**Rationale:** This is POLICY 4 semantic-anchoring drift introduced by fix-burst-2. The pass-2 F-P2-001 recommended adding a string field carrying the body-literal cite; the implementer correctly added the structural plumbing but mis-named/mis-documented the source field. TD-VSDD-059 paper-fix indicator: the rename of the struct field (to match downstream `cited_raw` doc text) would have caught this; what we have is plumbing-correct but naming-stale. A future maintainer adding a third consumer (e.g. for STATE.md cell-location reporting) would write `format!("v{}", v.minor_raw)` expecting "v05" and get "v1.05" -- silent bug.

Production-grade default lens (CLAUDE.md &sect;Canonical Principle Rule 1, Rule 4): the correct fix-burst-2 would have renamed `VersionCite::minor_raw` &rarr; `cite_raw` (or `version_raw`) and updated the doc-comment. The implementer hit the structural goal but not the naming-clarity goal. Sibling-sweep on the local shadow at line 200 is also needed if the field is renamed (the local can stay; the field clarifies).

**Recommended fix:** Rename `VersionCite::minor_raw` &rarr; `VersionCite::cite_raw` (matches downstream `Violation::cited_raw` semantically). Update doc-comment (lib.rs:102-105) to: `"The raw major.minor literal as it appeared in the source document body (e.g. \"1.05\" for \"v1.05\", \"3.28\" for \"v3.28\"). Preserved so the block message can reproduce body-literal form byte-for-byte (BC-5.39.003 EC-005)."` Update construction site (line 206: `cite_raw: format!(...)`), the 3 consumer sites (`cited_raw: cite.cite_raw.clone()`), and the inline comment at line 196 ("Capture raw cite slice ..."). Single mechanical sibling-sweep, ~5 min.

---

### F-S15.07-LOCAL-P3-002 -- LOW -- Story spec body+AC-9 still cite `Write|Edit` after canonical lock

**Anchor:** `.factory/stories/S-15.07-validate-index-cite-refresh.md` lines 208, 282, 289

**Evidence (verbatim file:line: from Grep stdout):**

```
.factory/stories/S-15.07-validate-index-cite-refresh.md:208:| AC-9 | hooks-registry.toml entry present with `event = "PostToolUse"`, `tool = "Write|Edit"`, correct `file_pattern` matching ARCH-INDEX.md | `grep -A10 'validate-index-cite-refresh' plugins/vsdd-factory/hooks-registry.toml` confirms all fields | Architect Q4 registry pattern |
.factory/stories/S-15.07-validate-index-cite-refresh.md:282:  # ---------- validate-index-cite-refresh (PostToolUse, Write|Edit) ----------
.factory/stories/S-15.07-validate-index-cite-refresh.md:289:  tool = "Write|Edit"
```

Architect Q5 (`architect-m2-q5-tool-attribute-2026-05-16.md` &sect;Future-Story Convention Lock):

> "All registry entries added by S-15.09, S-15.11, S-15.14, and any subsequent hook story MUST use `tool = "Edit|Write"` (not `"Write|Edit"`) when the hook fires on file-write operations; this is the canonical form..."

Fix-burst-1 production registry sweep landed at line 861 (`tool = "Edit|Write"`). Fix-burst-2 swept all 8 bats inline registries (verified: zero `Write|Edit` workspace-wide). But the story spec body itself was authored before Q5 and remains un-amended. The spec is now a stale-template hazard: any operator using the spec as a template for downstream M2 stories (S-15.11, S-15.09, S-15.14) will copy the `Write|Edit` form, recreating the exact drift Q5 was authored to prevent.

Pass-1 O-001 [process-gap] also identified AC-9's `file_pattern` clause as un-runnable against the registry schema; pass-2 O-005 re-surfaced it. Same un-amended spec state.

**Rationale:** POLICY 4 semantic-anchoring + TD-VSDD-060 sibling-site sweep. The spec is one of the source-of-truth artifacts (per CLAUDE.md "code-vs-spec conflicts &rarr; SPEC wins"). When the architect codified `Edit|Write` as canonical AND swept production+tests, the spec itself MUST be brought along -- or the spec wins and the impl is wrong. We know architect intent says impl is correct; therefore the spec is stale and must be corrected via state-manager/product-owner edit. LOW because the spec is treated as historical-documentary by downstream operators in practice, not LOW because it doesn't matter.

**Recommended fix:** Single 3-site edit in `.factory/stories/S-15.07-validate-index-cite-refresh.md`:
- Line 208 AC-9 row: `tool = "Edit|Write"` (and consider rewording the `file_pattern` clause to reflect the in-plugin file-path guard, OR drop the `file_pattern` reference and document the deferral inline)
- Line 282 heading comment: `# ---------- validate-index-cite-refresh (PostToolUse, Edit|Write) ----------`
- Line 289 TOML body: `tool = "Edit|Write"`

Routing: state-manager or product-owner (spec content owner) per CLAUDE.md Agent Routing Table. Trivial single-file edit; closes both the post-Q5 spec drift AND the pass-1 O-001 / pass-2 O-005 process-gap re-surface.

---

## Observations

### O-S15.07-LOCAL-P3-001 -- Asymmetric block-message version-string assertion coverage across fail-stale-*.bats

`fail-stale-bc-index.bats` (line 109-111) asserts `BC-INDEX`, `v1.05`, `v2.24` in the block message -- three discriminators including both cited and live versions. `fail-stale-vp-index.bats` (line 89) asserts only `VP-INDEX` -- no version string. `fail-stale-story-index.bats` (line 89) asserts only `STORY-INDEX` -- no version string.

Pre-existing coverage gap (not a regression from fix-burst-2). Per POLICY 11 the index-name-only assertion is weakly-discriminating: a future regression that emitted "validate-index-cite-refresh: 1 stale cite(s) found:\n  [ARCH-INDEX.md] line N cites VP-INDEX vM.N..." with WRONG cited/live versions would still pass these tests as long as "VP-INDEX" appears in the block message. fail-stale-bc-index.bats is the only fixture testing the full version-rendering pipeline post-F-P2-001.

Not promoting to a finding because all three test files were authored to the same template and pass-2 F-P2-003 explicitly tightened only `1.5` &rarr; `v1.05`/`v2.24` on the BC-INDEX path. The pattern is consistent (each test asserts the index-name it's testing); the gap is structural, not a regression. Recommend adding parallel `v1.80`/`v1.97` assertions to fail-stale-vp-index.bats and `v3.28`/`v3.31` assertions to fail-stale-story-index.bats in a follow-up; current state is acceptable for 3-CLEAN.

### O-S15.07-LOCAL-P3-002 -- pass-1 O-001 / pass-2 O-005 process-gap re-surfaces as F-P3-002 spec drift

The AC-9 file_pattern process-gap originally noted at pass-1 O-001 (un-runnable verification command) and re-surfaced at pass-2 O-005 (still un-closed) now manifests as part of F-S15.07-LOCAL-P3-002 (story spec body+AC-9 still cite stale `Write|Edit` form). The spec edit recommended in F-P3-002 should also amend AC-9's `file_pattern` clause per the routing in pass-1 O-001's recommended-fix. Two-birds-one-stone: state-manager/product-owner edit closes both the convention-lock spec drift AND the AC-9 process-gap in a single change.

### O-S15.07-LOCAL-P3-003 -- fix-burst-2 successfully closed F-P2-001 cited_raw plumbing across all 5 construction sites

Verified all 5 `Violation` construction sites populate `cited_raw`:
- lib.rs:391 (cross_cell_check STATE.md path)
- lib.rs:426 (cross_cell_check INDEX.md path)
- lib.rs:558 (on_post_tool_use ARCH-INDEX path)
- lib.rs:762 (test_BC_5_39_003_emit_block_names_index_and_versions -- asserts "1.05")
- lib.rs:792 (test_BC_5_39_003_emit_block_exit_code_2)

`format_violation` at lib.rs:468 consumes `v.cited_raw` for the cited side; live side at lib.rs:469-472 stays integer-rendered. Unit test at lib.rs:771 asserts `"1.05"` (body-literal form). Excellent structural fix -- only the field-naming defect (F-P3-001) prevents this being CLEAN.

### O-S15.07-LOCAL-P3-004 -- fix-burst-2 successfully closed F-P2-002 bats inline-registry sweep

All 8 bats files in `plugins/vsdd-factory/tests/validate-index-cite-refresh/` now use `tool = "Edit|Write"`. Workspace-wide grep for `tool = "Write|Edit"` returns zero matches. Architect Q5+Q6 convention lock now propagates correctly to bats inline registries.

### O-S15.07-LOCAL-P3-005 -- fix-burst-2 successfully closed F-P2-003 substring tightening

`fail-stale-bc-index.bats` lines 110-111 now assert `*"v1.05"*` and `*"v2.24"*` (version-prefix-anchored); `fail-multi-stale-cites.bats` lines 119-120 now assert `*"BC-INDEX v1.05"*` and `*"v3.28"*` (full-canonical anchored). No bare `*"1.5"*` or `*"1.05"*` assertions remain. POLICY 11 discrimination concern closed for the asserted paths.

### O-S15.07-LOCAL-P3-006 -- Cross-cycle path hardcode + AC-9 file_pattern + cross-cell sibling-detection scope all remain legitimately scope-deferred

Per pass-1 O-002, pass-1 O-003, pass-2 O-005: these are explicitly deferred to S-15.10 and S-15.03 PRIORITY-A respectively, with concrete future-story attachment per CLAUDE.md Canonical Principle Rule 3. Not findings.

---

## Part B -- Pass-Internal Notes (NOT visible to subsequent passes)

### Convergence Streak Status

- **Entering pass-3:** 0/3 (pass-2 had 1 MEDIUM -> reset)
- **After pass-3 verdict (MEDIUM with 1 MEDIUM + 1 LOW):** 0/3 (streak does not advance; MEDIUM resets per BC-5.39.001 3-CLEAN protocol)

Per S-15.08 cascade convention disambiguation: "LOW does NOT reset; only MEDIUM+ resets." One MEDIUM finding exists -> streak resets/holds at 0/3.

### Fresh-Context Pattern Observations

- Fix-burst-2's F-P2-001 closure is structurally correct but exhibits a different paper-fix pattern than passes 1-2: instead of papering over with a doc-only fix or a rename-only fix, fix-burst-2 added real plumbing but mis-named the producer field. This is a Cycle-3 novelty pattern -- implementer correctly listened to "plumb the string slice" but didn't sibling-sweep the doc-comment to match the actual stored value. POLICY 4 semantic-anchoring catches it.
- F-P3-002 (story spec drift) is a textbook example of why pass-1 O-001 / pass-2 O-005 should have been re-classified as a content-defect at pass-2 rather than re-surfaced as observation. The architect Q5/Q6 decisions made the spec authoritatively wrong; deferring spec amendment to "process-gap" caused it to persist 3 passes. Recommend the orchestrator route this as a content-defect (state-manager/product-owner) at fix-burst-3, not as a continued observation.
- The asymmetric block-message coverage in fail-stale-vp-index.bats/fail-stale-story-index.bats (O-P3-001) is a real coverage gap but is below the LOW threshold for blocking convergence. Worth a follow-up burst in M2 but not 3-CLEAN-blocking.

### Recommendation to Orchestrator

**Dispatch fix-burst-3** before pass-4 -- one MEDIUM finding (F-P3-001) requires remediation.

Routing by finding:
- F-S15.07-LOCAL-P3-001 (`VersionCite::minor_raw` rename + doc-comment fix): **implementer** -- rename field, update doc-comment, sibling-sweep 3 consumer sites + 1 inline comment at lib.rs:196. Single mechanical edit, ~5 min.
- F-S15.07-LOCAL-P3-002 (story spec `Write|Edit` -> `Edit|Write` + AC-9 amendment): **state-manager** or **product-owner** -- 3-site spec edit, also closes pass-1 O-001 / pass-2 O-005 process-gap. Single mechanical edit, ~5 min.

After fix-burst-3 lands on `feature/S-15.07-index-cite-refresh-hook`, dispatch adversary pass-4 (fresh context, Iron Law). Pass-4 should verify (a) field rename propagation, (b) no other call-sites of `minor_raw` left dangling, (c) spec edit consistency with impl/registry/bats.

Note on F-P3-001 severity: this is MEDIUM rather than LOW because the field/name/value triple-disagreement is a future-bug magnet (any new consumer formatting `v{minor_raw}` expecting bare minor will silently produce wrong output) AND it represents a TD-VSDD-059 pattern where a structural fix was almost-but-not-quite complete. It is MEDIUM rather than HIGH because (a) no current call site is affected (all 3 consumers happen to do `cited_raw: cite.minor_raw.clone()` which preserves the surprising-but-correct value), and (b) the deficiency is invisible at runtime -- only static-reading exposes it.

Note on F-P3-002 severity: LOW because the spec is documentary-historical for already-shipped impl. Promoting to MEDIUM if the orchestrator treats Q5 convention-lock as binding on spec body text (which would be defensible per Standing Rule "spec wins on code-vs-spec conflicts" + per POLICY 4).
