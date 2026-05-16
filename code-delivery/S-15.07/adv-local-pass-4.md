---
document_type: adversary-pass
level: ops
pass: 4
cascade: LOCAL
story: S-15.07
producer: adversary
timestamp: 2026-05-16T00:00:00Z
diff_base: c62f952c
diff_head: 8f7d9071
verdict: CLEAN
finding_count_by_severity:
  critical: 0
  high: 0
  medium: 0
  low: 1
  nitpick: 0
policies_evaluated: [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18]
---

# S-15.07 LOCAL Adversary Pass-4

## Verdict

**CLEAN (LOW-only)** -- Fix-burst-3 successfully closed both pass-3 findings with verified evidence in source. F-P3-001 (`VersionCite::minor_raw` &rarr; `cite_raw` rename) propagated cleanly: struct field, doc-comment, construction site, all 3 consumer sites, and inline comment all coherent. F-P3-002 (story spec `Write&#124;Edit` &rarr; `Edit&#124;Write` amendment) propagated to all 3 spec sites (line 208 AC-9, line 282 heading comment, line 289 TOML body); AC-9 verification predicates are runnable and match; spec v1.1 + STORY-INDEX v3.33 + changelog row present. Fresh-context sweep finds one LOW residue: 2 doc-comment narrative `Write&#124;Edit` strings in `lib.rs` survived the architect Q5 canonical lock -- narrative prose, not registry config, so the sweep is reporting-only (pending intent verification per S-7.01). No MEDIUM+ findings. Streak advances 0/3 &rarr; 1/3.

## Findings Table

| ID | Severity | Policy-Triggered | Category | Summary | Recommended Action |
|----|----------|------------------|----------|---------|--------------------|
| F-S15.07-LOCAL-P4-001 | LOW (pending intent verification) | TD-VSDD-060 sibling-site sweep | doc-comment-vs-canonical-form drift | After architect Q5 canonical lock (registry `tool = "Edit&#124;Write"`) + fix-burst-1 production sweep + fix-burst-2 bats sweep + fix-burst-3 story-spec sweep, two doc-comment narrative strings in `crates/hook-plugins/validate-index-cite-refresh/src/lib.rs` (lines 485, 510) still describe the dispatcher tool-filter form as `Write&#124;Edit`. Per architect Q5 the canonical lock text explicitly scopes to registry entries; doc-comment narrative is functionally indifferent (regex alternation is order-insensitive). However, downstream story-writers reading lib.rs doc-comments to understand dispatcher routing will see the minority form and may propagate it. | Trivial sibling-sweep: change both occurrences `Write&#124;Edit` &rarr; `Edit&#124;Write` in lib.rs doc-comments (lines 485, 510). Tagged (pending intent verification): orchestrator/architect may confirm doc-comment prose is out-of-scope for the Q5 lock, in which case the finding closes as documented-no-op. |

## Finding Details

### F-S15.07-LOCAL-P4-001 -- LOW (pending intent verification) -- Doc-comment narrative `Write|Edit` in lib.rs survived Q5 canonical-form lock

**Anchor:** `crates/hook-plugins/validate-index-cite-refresh/src/lib.rs` lines 485 and 510 (doc-comment narrative inside `on_post_tool_use` function)

**Evidence (verbatim grep stdout):**

```
crates/hook-plugins/validate-index-cite-refresh/src/lib.rs:485:///    routes by event+tool, so any PostToolUse Write|Edit reaches this hook).
crates/hook-plugins/validate-index-cite-refresh/src/lib.rs:510:    // The dispatcher routes by event+tool (PostToolUse + Write|Edit) but not by
```

The registry-side canonical form is correctly `Edit|Write`:
```
plugins/vsdd-factory/hooks-registry.toml:861:tool = "Edit|Write"
```

All 8 bats inline registries are `Edit|Write` (fix-burst-2). Story spec body is `Edit|Write` (fix-burst-3). The only residue of the minority form within S-15.07-owned artifacts is these two doc-comment narrative strings. Architect Q5 &sect;Future-Story Convention Lock text explicitly binds "registry entries added by S-15.09, S-15.11, S-15.14, and any subsequent hook story" -- doc-comment prose is not named.

**Rationale:** TD-VSDD-060 sibling-site sweep. The pass-3 fix-burst-3 swept registry/bats/spec; doc-comment narrative was not in scope. Pass-1 F-005 / pass-2 F-P2-002 / pass-3 F-P3-002 all targeted registry-entry-shaped artifacts. This is the fourth (and apparently final) propagation site, in a narrative-prose context. Per S-7.01 Partial-Fix Regression Discipline &sect;Intent adjudication rule: "The adversary cannot adjudicate whether a sibling should receive the same fix -- that depends on authorial intent. When the intent is unclear, report the difference as a finding with severity LOW and tag it (pending intent verification)." This is exactly that case: Q5 explicitly scoped to registry entries, but downstream readers of lib.rs doc-comments may copy the minority form, recreating the exact drift Q5 was authored to prevent.

**Recommended fix:** Single 2-line edit in `crates/hook-plugins/validate-index-cite-refresh/src/lib.rs`:
- Line 485: `///    routes by event+tool, so any PostToolUse Edit|Write reaches this hook).`
- Line 510: `    // The dispatcher routes by event+tool (PostToolUse + Edit|Write) but not by`

Routing: implementer (single 2-line edit; ~2 min) IF the orchestrator/architect confirms Q5 binds doc-comment narrative. Otherwise close as documented no-op.

Severity LOW because: (a) functional equivalence guaranteed by Q5 dispatcher analysis (regex alternation order-insensitive); (b) narrative prose has no runtime effect; (c) Q5 text doesn't explicitly bind prose. NOT MEDIUM because the canonical lock text scopes to registry entries, and per pass-3 fix-burst-3 the orchestrator's interpretation of Q5 scope appears to be "registry config artifacts only."

---

## Observations

### O-S15.07-LOCAL-P4-001 -- F-P3-001 cite_raw rename verified clean across all sites

Workspace-wide grep `minor_raw` returns exactly 2 matches:
```
crates/hook-plugins/validate-index-cite-refresh/src/lib.rs:199:                                let minor_raw = &content[after_dot..after_dot + minor_len];
crates/hook-plugins/validate-index-cite-refresh/src/lib.rs:205:                                    cite_raw: format!("{major_raw}.{minor_raw}"),
```

Both are local shadow bindings inside `extract_index_cites` where `minor_raw` genuinely is the minor portion (e.g. "05"); the struct field is consistently `cite_raw` carrying the combined "1.05" form. The doc-comment at lib.rs:102-105 now accurately describes the actual stored value (`"1.05" for "v1.05", "3.28" for "v3.28"`). All 3 consumer sites (lib.rs:390, 425, 557) read `cite.cite_raw.clone()` -- naming/semantics aligned. Inline comment at lib.rs:196 also updated ("Capture raw cite slice (e.g. \"1.05\" from \"v1.05\")"). F-P3-001 closure is structurally complete with no residue.

### O-S15.07-LOCAL-P4-002 -- F-P3-002 spec amendment verified clean

Verified all 3 spec sites use canonical form:
```
.factory/stories/S-15.07-validate-index-cite-refresh.md:208 (AC-9): `tool = "Edit|Write"`
.factory/stories/S-15.07-validate-index-cite-refresh.md:282 (heading comment): `(PostToolUse, Edit|Write)`
.factory/stories/S-15.07-validate-index-cite-refresh.md:289 (TOML body): `tool = "Edit|Write"`
```

Spec frontmatter `version: "1.1"` ✓. Changelog row at lines 696-697: v1.1 entry describes the architect Q5/Q6 amendment and explicitly closes pass-1 O-001 / pass-2 O-005 / pass-3 F-P3-002. STORY-INDEX frontmatter `version: "3.33"` ✓ (pass-3 cited bump from v3.32 &rarr; v3.33).

AC-9 verification predicates are both runnable and match:
- `grep -A10 'validate-index-cite-refresh' plugins/vsdd-factory/hooks-registry.toml` matches lines 853-874, including `tool = "Edit|Write"` at line 861.
- `grep 'ends_with.*ARCH-INDEX.md' crates/hook-plugins/validate-index-cite-refresh/src/lib.rs` matches line 512 (`if !file_path.ends_with("ARCH-INDEX.md") { return Continue; }`).

The AC-9 process-gap (pass-1 O-001) is structurally closed.

### O-S15.07-LOCAL-P4-003 -- Out-of-scope `Write|Edit` matches in workspace are pre-existing comment headers, not S-15.07 regressions

Workspace-wide grep `Write|Edit` returns 11 matches outside S-15.07's scope:
- `crates/factory-dispatcher/tests/full_stack_plugin_invocation.rs` (5 matches) -- dispatcher integration tests testing the regex-alternation behavior in both orders; intentional cross-form coverage; not S-15.07's domain.
- `plugins/vsdd-factory/hooks-registry.toml:836` (validate-artifact-path comment header) -- preexisting; not changed in fix-burst-1 sweep because the validate-artifact-path entry itself is `tool = "Edit|Write"` (line 844) but its comment header carries the older form. Out of S-15.07 scope.
- `plugins/vsdd-factory/hooks-registry.toml:980` (validate-stable-anchors comment header) -- same preexisting pattern.

None of these are within S-15.07's blast radius per S-7.01 &sect;sibling-sweep "same architectural layer" definition. They predate S-15.07 and were untouched by the diff.

### O-S15.07-LOCAL-P4-004 -- Asymmetric block-message version-string assertion coverage between fail-stale-{bc,vp,story}-index.bats persists (pass-3 O-001 carry-forward)

`fail-stale-bc-index.bats` asserts `v1.05` AND `v2.24` in block message. `fail-stale-vp-index.bats` and `fail-stale-story-index.bats` assert only the index name string. Per pass-3 O-001 this is structural pre-existing test-template state, not a regression from fix-burst-3. Below LOW threshold for blocking convergence; recommend follow-up burst in M2 to add parallel `v1.80`/`v1.97` (VP-INDEX) and `v3.28`/`v3.31` (STORY-INDEX) assertions for full per-index discrimination.

### O-S15.07-LOCAL-P4-005 -- Cross-cycle path hardcode + AC-9 file_pattern semantics + cross-cell BC-INDEX coverage all remain legitimately scope-deferred

Per pass-1 O-002, O-003 + pass-2 O-005 + pass-3 O-006: cross-cycle path automation deferred to S-15.10 (M3); STATE.md body-cell validation beyond cross-cell version-cite deferred to S-15.09 (M2 next); AC-9 file_pattern process-gap is structurally closed by fix-burst-3 (in-plugin guard verified). Not findings.

### O-S15.07-LOCAL-P4-006 -- Cumulative-cascade trajectory: P1(2M+3L) &rarr; P2(1M+2L) &rarr; P3(1M+1L) &rarr; P4(0M+1L). First downward-floor pass.

The cumulative trajectory shows monotonic decay: 2 MEDIUM &rarr; 1 MEDIUM &rarr; 1 MEDIUM &rarr; 0 MEDIUM. Fix-burst-3 was the first to land all closures cleanly without introducing a paper-fix regression (passes 1&rarr;2 introduced F-P2-001 zero-pad inversion; passes 2&rarr;3 introduced F-P3-001 field-rename gap; pass 3&rarr;4 introduces zero new defects). The single LOW finding in pass-4 is a sibling-sweep residue, not a new defect class. Convergence pattern is consistent with S-15.08 cascade trajectory (6 passes + 4 fix-bursts to 3-CLEAN). If pass-4 LOW does not reset the streak (per S-15.08 convention disambiguation: "LOW does NOT reset; only MEDIUM+ resets"), streak advances to 1/3 and 2 more clean passes can converge.

### O-S15.07-LOCAL-P4-007 -- Production-grade default lens: all aspects compliant

- No `unwrap()` / `expect()` in production paths (all `host::read_file` and `String::from_utf8` results pattern-matched with explicit error arms returning `Continue` + `log_warn`).
- No `println!` in production paths (all logging via `host::log_warn`).
- No `todo!()` / `unimplemented!()`.
- No MVP shortcuts: cite_raw plumbing was real plumbing, not paper-fix.
- Fail-open semantics correct: every `host::read_file` error path returns `HookResult::Continue` after `log_warn` (BC-5.39.003 invariant 5).
- UTF-8 multi-byte safety via `is_char_boundary` (lib.rs:247-249); regression-tested by `test_BC_5_39_003_extract_handles_multibyte_chars_no_panic`.
- No regex crate (per fuel-budget constraint); hand-rolled scanner.
- HOST_ABI_VERSION = 1 (no new host functions).

### O-S15.07-LOCAL-P4-008 -- POLICY 18 input-hash spot-check non-verifiable in read-only context

Story spec frontmatter declares `input-hash: "df9db17"`. The adversary lacks `Bash` access to invoke `compute-input-hash --scan .factory` to verify. Not a finding (verifier missing, not defect present). Recommend state-manager runs `compute-input-hash` parity check at commit time if material edits land between v1.0 &rarr; v1.1 (spec body lines 208/282/289 were modified for the F-P3-002 closure -- the hash may need bumping).

---

## Part B -- Pass-Internal Notes (NOT visible to subsequent passes)

### Convergence Streak Status

- **Entering pass-4:** 0/3 (pass-3 had 1 MEDIUM &rarr; reset)
- **After pass-4 verdict (CLEAN with 0 MEDIUM + 1 LOW + 8 observations):** **1/3** (streak advances; LOW does NOT reset per S-15.08 cascade convention disambiguation)

First positive streak movement in the cascade. The pass-4 LOW (F-S15.07-LOCAL-P4-001 doc-comment sibling-sweep) is tagged (pending intent verification) per S-7.01 &sect;Intent adjudication rule -- orchestrator/architect adjudicates whether Q5 binds doc-comment narrative. Both adjudications (in-scope &rarr; fix in fix-burst-4; out-of-scope &rarr; document and close as no-op) leave the streak at 1/3 &rarr; can proceed to pass-5.

### Fresh-Context Pattern Observations

- Pass-4 is the natural "fold" point in the cascade. Passes 1-3 each introduced a follow-on paper-fix regression: P1&rarr;P2 introduced F-P2-001 zero-pad inversion when papering over F-001 zero-pad; P2&rarr;P3 introduced F-P3-001 field-rename gap when papering over F-P2-001 cited_raw plumbing; P3&rarr;P4 introduces no regression. Fix-burst-3 was structurally complete and did not perpetuate the chain.
- The F-P3-001 closure was the smallest fix-burst in the cascade (single field rename + 5 callsite sweep + doc-comment update + inline comment update; commit message describes it as "rename to match stored value semantics"). This is the canonical TD-VSDD-060 sibling-sweep pattern: identify the value-semantics mismatch, rename the field, sweep all callsites. Implementer did it cleanly.
- The F-P3-002 closure was the canonical product-owner/state-manager spec amendment: identify the spec body sites where canonical form has not propagated, edit them, bump spec version, add changelog row. The story spec is now byte-coherent with the impl + registry + bats fixtures.
- The doc-comment residue at lib.rs:485 and lib.rs:510 was authored before architect Q5 codified the canonical lock. They survived passes 1-3 because all prior adversary focus was on registry config (production registry, bats inline registries) and spec body -- doc-comment prose was not under the lens. Pass-4 fresh-context catches it precisely because the prior-pass scope-anchoring no longer applies under fresh context.
- The asymmetric block-message coverage gap (O-P4-004 / pass-3 O-001) is real but below LOW threshold. Worth a 2-line tightening per file in a follow-up burst.

### Recommendation to Orchestrator

**Pass-4 verdict: CLEAN (LOW-only)** -- fix-burst-4 is OPTIONAL per S-15.08 cascade convention:

Routing options:

- **Path A (recommended): No fix-burst-4; advance to pass-5 directly.** Per S-15.08 convention "LOW does NOT reset; only MEDIUM+ resets." Streak advances 0/3 &rarr; 1/3. Pass-5 is dispatched immediately under fresh context. The F-P4-001 doc-comment finding is documented but not blocking; orchestrator may attach it to a follow-up bookkeeping burst that bundles all LOW-class residues (F-P4-001 + O-P4-004 asymmetric coverage + O-P4-008 input-hash check) and apply at the streak-3/3 convergence commit.

- **Path B (alternative): Quick fix-burst-4 to close F-P4-001 in scope.** Per CLAUDE.md Canonical Principle Rule 4 ("AI-built defects are the AI's responsibility to fix") + Rule 6 ("Surface vs defer -- Surface is production-grade"), a 2-line doc-comment edit in lib.rs is well within the 45-minute threshold. Routing: implementer. Streak still advances 1/3 because no MEDIUM+ findings present. Pass-5 dispatched after fix-burst-4 lands.

Recommendation: **Path A**. Path B is defensible but unnecessary; the LOW finding is genuinely intent-ambiguous per Q5 scope text, and orchestrator/architect adjudication is the legitimate use of human time per CLAUDE.md &sect;Companion Principle Rule 4. Bundling F-P4-001 with any other LOW residues at convergence commit is mechanically cheaper than a separate burst.

Note on F-P4-001 severity: LOW (pending intent verification). NOT MEDIUM because:
- Architect Q5 text explicitly scopes to registry entries.
- Dispatcher regex alternation is order-insensitive (Q5 functional analysis).
- No runtime effect -- narrative prose only.
- Per S-7.01 &sect;Intent adjudication rule, the adversary cannot adjudicate authorial intent for sibling-sweep boundaries; LOW + (pending) is the correct severity per the discipline.

Promote to MEDIUM only if the orchestrator/architect explicitly extends Q5 scope to include doc-comment narrative -- then F-P4-001 becomes a genuine sibling-sweep regression and fix-burst-4 is required.

### Cascade Trajectory Summary (4 passes, 3 fix-bursts so far)

| Pass | Verdict | Critical | High | Medium | Low | Streak | Fix-burst landed |
|------|---------|----------|------|--------|-----|--------|-------------------|
| P1 | MEDIUM | 0 | 0 | 2 | 3 | 0/3 reset | FB-1 (5 fixes) |
| P2 | MEDIUM | 0 | 0 | 1 | 2 | 0/3 reset | FB-2 (3 fixes) |
| P3 | MEDIUM | 0 | 0 | 1 | 1 | 0/3 reset | FB-3 (2 fixes) |
| **P4** | **CLEAN** | **0** | **0** | **0** | **1** | **1/3 ADVANCE** | (none required) |

First positive streak movement. Two more LOW-or-cleaner passes required for 3/3 CONVERGED. Per S-15.08 cumulative cascade (6 passes + 4 fix-bursts), S-15.07 is on track for 6-7 pass convergence -- slightly tighter trajectory due to LOW-only pass-4 versus S-15.08's pass-4 mid-cascade defect.
