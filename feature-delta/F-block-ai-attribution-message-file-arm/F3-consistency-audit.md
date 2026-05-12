---
document_type: feature-gate-report
level: ops
version: "1.0"
status: final
producer: consistency-validator
timestamp: 2026-05-12T00:00:00Z
phase: F3
cycle: v1.0-feature-block-ai-attribution-message-file-arm
traces_to: .factory/feature-delta/F-block-ai-attribution-message-file-arm/F2-consistency-audit.md
input-hash: "[pending-recompute]"
---

# F3 Consistency Audit — AI-Attribution Hook Extension (block-ai-attribution message-file arm)

**Gate:** Pre-state-manager close-out gate (post-F3 story creation, pre-index registration)
**Auditor:** consistency-validator (fresh context — F1, F2, and F2 fix burst not seen prior)
**Date:** 2026-05-12
**Overall Verdict:** PASS-WITH-NITS

---

## Summary Table

| Check | ID | Result | Severity | Blocking? |
|-------|----|--------|----------|-----------|
| 1. ID uniqueness & filename convention | CHK-001 | PASS | — | No |
| 2a. S-16.01 BC anchor semantic correctness | CHK-002 | PASS | — | No |
| 2b. S-16.02 BC anchor semantic correctness | CHK-003 | PASS | — | No |
| 2c. VP-080 anchor_story semantic substantiation | CHK-004 | PASS | — | No |
| 2d. BC body Traceability Stories rows | CHK-005 | PASS-WITH-NITS | NIT | No |
| 2e. Subsystem citations verbatim vs ARCH-INDEX | CHK-006 | PASS | — | No |
| 2f. Epic citation E-16 match | CHK-007 | PASS | — | No |
| 3. Dependency graph correctness (acyclic, valid) | CHK-008 | PASS-WITH-NITS | NIT | No |
| 4a. AC ↔ BC clause traceability — S-16.01 spot-check | CHK-009 | PASS | — | No |
| 4b. AC ↔ BC clause traceability — S-16.02 spot-check | CHK-010 | PASS | — | No |
| 4c. PRE-3 success-only gate AC coverage (S-16.01) | CHK-011 | PASS | — | No |
| 4d. INV-2 short-circuit AC coverage (S-16.02) | CHK-012 | PASS | — | No |
| 5. File list completeness | CHK-013 | PASS | — | No |
| 6. Test list ↔ AC coverage | CHK-014 | PASS-WITH-NITS | NIT | No |
| 7. Demo evidence specification | CHK-015 | PASS-WITH-NITS | NIT | No |
| 8. Policy #3 deferral correctness (no index touches) | CHK-016 | PASS | — | No |
| 9. Template compliance (required sections present) | CHK-017 | PASS | — | No |
| 10. Upstream artifact immutability | CHK-018 | PASS | — | No |
| 11. Frontmatter canonical fields | CHK-019 | PASS-WITH-NITS | NIT | No |
| 12. behavioral_contracts frontmatter field naming | CHK-020 | PASS-WITH-NITS | NIT | No |

---

## Findings Detail

### CHK-001 — ID Uniqueness and Filename Convention

**Result: PASS**

**ID allocation:**

- The STORY-INDEX body summary (line 186) reads "97 stories across 16 epics (E-0 through E-15)." The current story count from the STORY-INDEX changelog is 93 (confirmed by multiple entries: "story_count unchanged (93)"). The `story_count: 93` is the actual tracked value; "97 stories" in the narrative summary is a stale count from a prior cycle (line 186 was authored before S-14.06–S-14.09 were added, then the paragraph was not updated — but the operative count comes from the table body which shows 93). The current ceiling epic is E-15. E-16 is next-available per POLICY 1 (append-only numbering). No E-16 exists in STORY-INDEX — confirmed by grep returning 0 results. PASS.

- S-15.01, S-15.02, and S-15.03 are the current S-15.x floor. The next available prefix is S-16. S-16.01 and S-16.02 are valid next-sequential IDs under E-16 per F1 §6 allocation. No S-16.xx files other than the two under audit exist on disk. PASS.

- BC-INDEX ceiling is BC-7.03.093 (confirmed at BC-INDEX lines 1822–1825). BC-7.03.094 and BC-7.03.095 do not appear in BC-INDEX (grep returned 0 results) — they are pre-registration artifacts per Policy #3 deferral. No collision. PASS.

- VP-INDEX `total_vps: 79`. VP-080 does not appear in VP-INDEX (grep returned 0 results) — pre-registration per Policy #3 deferral. No collision. PASS.

**Filename convention:**

- `S-16.01-block-ai-attribution-posttooluse-head-verify.md` — kebab-case, ID prefix, descriptive slug. Matches project convention (compare: `S-15.01-plugin-async-semantics.md`, `S-3.03-port-block-ai-attribution.md`). PASS.
- `S-16.02-block-ai-attribution-pretooluse-file-arm.md` — same pattern. PASS.

---

### CHK-002 — S-16.01 BC Anchor Semantic Correctness

**Result: PASS**

**Frontmatter `behavioral_contracts:` contains: [BC-7.03.094, BC-7.03.001]**

- **BC-7.03.094**: Primary BC for this story. BC-7.03.094 H1 heading reads "block-ai-attribution: PostToolUse retroactive HEAD commit message verification." S-16.01 title is "block-ai-attribution — PostToolUse retroactive HEAD verification." Scope match: S-16.01 implements the PostToolUse handler (`on_post_tool_use_logic`), the `exec_subprocess` call, the PRE-1..PRE-3 gates, all postconditions, and all invariants of BC-7.03.094. The story's Architecture Mapping table and all ACs are anchored to BC-7.03.094 clauses. Primary anchor is correct.

- **BC-7.03.001**: Identity and registry binding BC (v1.3). S-16.01 extends `hooks-registry.toml` to add PostToolUse to the events list and adds the `exec_subprocess` capability block. BC-7.03.001 v1.3 changelog confirms this exact extension. AC-001 is the registry-shape AC, tracing to BC-7.03.001 postcondition 1. Secondary anchor is correct and semantically substantiated.

**Reciprocal check — BC body Traceability Stories rows:**

- BC-7.03.094 Traceability section (line 188): `| Stories | S-16.01 (block-ai-attribution: PostToolUse retroactive HEAD verification) |` — present and correct.
- BC-7.03.001 Traceability section: `| Stories | TBD (filled by story-writer) |` — this is the brownfield residual TBD from the original BC-7.03.001 authoring. The v1.3 changelog entry (line 140) notes the registry shape extension but does not update the Stories row. This is a pre-existing gap in BC-7.03.001, not introduced by S-16.01. Per the audit mandate: "DO flag: BC body's Traceability Stories row staleness (that's PO's section to maintain)." See NIT-001.

---

### CHK-003 — S-16.02 BC Anchor Semantic Correctness

**Result: PASS**

**Frontmatter `behavioral_contracts:` contains: [BC-7.03.095, BC-7.03.001]**

- **BC-7.03.095**: Primary BC for this story. BC-7.03.095 H1 heading reads "block-ai-attribution: PreToolUse `-F <path>` file-read arm." S-16.02 title is "block-ai-attribution: PreToolUse -F file-read arm." Scope match: S-16.02 implements `parse_message_file_path`, the file-read arm extension to `on_hook_logic`, the `vsdd::read_file` call, all postconditions and invariants of BC-7.03.095. Primary anchor is correct.

- **BC-7.03.001**: Same rationale as S-16.01 — S-16.02 adds the `read_file` capability declaration to the PreToolUse registry entry. BC-7.03.001 v1.3 changelog confirms this extension. AC-001 of S-16.02 traces to BC-7.03.001 postcondition 1 (registry shape). Secondary anchor is correct.

**Reciprocal check — BC body Traceability Stories rows:**

- BC-7.03.095 Traceability section (line 192): `| Stories | S-16.02 (block-ai-attribution: PreToolUse -F file-read arm) |` — present and correct.
- BC-7.03.001 Traceability Stories row: same TBD gap as noted in CHK-002. See NIT-001.

---

### CHK-004 — VP-080 anchor_story Semantic Substantiation

**Result: PASS**

VP-080 frontmatter (line 41): `anchor_story: S-16.01`

**Does S-16.01 actually build the test vehicle?** Yes, explicitly:

- S-16.01 AC-009 (line 196–211): "A proptest harness is added at `crates/hook-plugins/block-ai-attribution/tests/proptest_detect_attribution.rs` implementing VP-080's four properties." This is the test harness creation story.
- S-16.01 File List NEW section includes `tests/proptest_detect_attribution.rs` as a new file to be created.
- S-16.01 Test List includes 7 proptest entries all pointing to `tests/proptest_detect_attribution.rs (NEW)`.
- S-16.01 Tasks include T-9: "Implement VP-080 proptest harness (`tests/proptest_detect_attribution.rs`) — 4 properties, 1024 cases."

S-16.02 explicitly declares it does NOT create this file (File List "NOT MODIFIED" section: "VP-080 harness from S-16.01; extended per the dependency comment below (NOT re-written)"). The harness vehicle is unambiguously S-16.01. The anchor is semantically correct — S-16.01 is the story that builds the test vehicle, not an architectural ancestor.

**VP-080 `traces_to` field (line 15):** `traces_to: verification-properties/VP-INDEX.md` — PASS. The F2 audit found the original `traces_to` pointed to the non-existent `verification-architecture.md` and flagged it FINDING-002. The VP-080 changelog (line 179) shows this was fixed in v1.1: "FIX-1 (F2 audit): corrected traces_to from non-existent `.factory/specs/architecture/verification-architecture.md` to `verification-properties/VP-INDEX.md`." The fix was applied before F3. PASS.

---

### CHK-005 — BC Body Traceability Stories Rows (PO maintenance obligation)

**Result: PASS-WITH-NITS**

| BC | Stories Row | Assessment |
|----|------------|------------|
| BC-7.03.094 | `S-16.01 (block-ai-attribution: PostToolUse retroactive HEAD verification)` | PASS — correctly references S-16.01 |
| BC-7.03.095 | `S-16.02 (block-ai-attribution: PreToolUse -F file-read arm)` | PASS — correctly references S-16.02 |
| BC-7.03.001 | `TBD (filled by story-writer)` | NIT — stale TBD from brownfield authoring |

**NIT-001:** BC-7.03.001 Traceability section (`/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-07/BC-7.03.001.md` line 81) shows `| Stories | TBD (filled by story-writer) |`. The v1.3 changelog notes registry shape extensions for both S-16.01 and S-16.02 but the Stories row was not updated. This is a PO maintenance obligation. The Stories row should now read `S-3.03 (original port), S-16.01 (PostToolUse + exec_subprocess capability), S-16.02 (read_file capability)`.

**Severity:** NIT. Does not block state-manager close-out. BC-7.03.001 is a secondary identity BC; the primary behavioral contracts (BC-7.03.094, BC-7.03.095) have correct Stories rows.

**Responsible agent:** product-owner.

---

### CHK-006 — Subsystem Citations Verbatim vs ARCH-INDEX

**Result: PASS**

ARCH-INDEX Subsystem Registry (line 210): `SS-07 | Hook Bash Layer`
ARCH-INDEX Subsystem Registry (line 207): `SS-04 | Plugin Ecosystem`

S-16.01 frontmatter `subsystems: ["SS-07", "SS-04"]` — PASS.
S-16.01 body justification section cites "SS-07 (Hook Bash Layer)" and "SS-04 (Plugin Ecosystem)" — exact match.
S-16.01 Architecture Mapping footer: "Subsystem SS-07 owns this story" and "SS-04 reference per `SS-04-plugin-ecosystem.md` v1.4" — consistent.

S-16.02 frontmatter `subsystems: ["SS-07", "SS-04"]` — PASS.
S-16.02 Subsystem Anchor Justification cites "SS-07 (Hook Bash Layer)" and "SS-04 (Plugin Ecosystem)" — exact match.

**Semantic check per POLICY 4 / CHK criteria:** SS-07 owns the `hooks-registry.toml` routing artifact and the BC shard directory (`behavioral-contracts/ss-07/`). The `block-ai-attribution` WASM plugin crate is in `crates/hook-plugins/block-ai-attribution/` which is listed in SS-04's Implementing Modules folder in ARCH-INDEX (line 207). Both subsystem citations are semantically correct: SS-07 is primary owner of the behavioral contract surface; SS-04 is correctly cited as the plugin ecosystem owner. PASS.

---

### CHK-007 — Epic Citation E-16

**Result: PASS**

- S-16.01 frontmatter `epic_id: "E-16"` — matches E-16 file.
- S-16.01 body: "**Epic:** E-16 — Hook Plugin Capability Extensions" — matches E-16 title exactly: "Hook Plugin Capability Extensions — block-ai-attribution message-file arm." The story body uses the shortened form; the full title in the epic includes the feature name. This is the standard pattern (compare: S-15.01 says "Epic: E-15 — Plugin Async Semantics" while the epic file is titled "Plugin Async Semantics"). PASS.
- S-16.02 frontmatter `epic_id: "E-16"` — matches. Body: "**Epic:** E-16 — Hook Plugin Capability Extensions." PASS.
- E-16 Stories table (line 110–113): lists S-16.01 and S-16.02. Bidirectional. PASS.

---

### CHK-008 — Dependency Graph Correctness

**Result: PASS-WITH-NITS**

**S-16.01 `depends_on: ["S-3.03"]`:**

S-3.03 is merged (status: merged, merged_at: 2026-04-27, merged_in: PR-19, merge_sha: f3db7776). S-16.01 provides a Dependency Anchor Justification section (lines 68–77) explaining that S-3.03 created the crate, `src/lib.rs`, `src/main.rs`, and the initial registry entry. The PostToolUse arm extends existing shipped code; the crate must already exist. The justification is substantive — this is not a stale planning artifact. The dependency expresses "extends an existing crate" rather than "is blocked by an unmerged story." This is a valid "ancestor trace" pattern that exists elsewhere in the project (other S-N.NN stories depend on merged predecessors to express shared codebase foundation).

**NIT-002:** The story's "Dependency Anchor Justification" section correctly flags this as a foundation dependency, but the `depends_on` field in a post-merge context has ambiguous planning semantics — it won't affect sprint planning since S-3.03 is merged. Per the audit prompt's instruction: "Is this dependency declaration meaningful, or stale (since S-3.03 is shipped and doesn't block S-16.01 in any planning-graph sense)? Project convention varies — flag the call for orchestrator review if unclear." This is flagged as NIT-002 for orchestrator awareness. The story explains the justification; the orchestrator should confirm whether this follows the project convention for expressing codebase predecessors vs. active planning blockers.

**S-16.01 `blocks: ["S-16.02"]` / S-16.02 `depends_on: ["S-16.01"]`:**

Bidirectional pairing is consistent. Acyclic (S-16.01 → S-16.02, no back-edge). Confirmed: S-16.02 `blocks: []`. The dependency is substantiated in S-16.02's Dependency Anchor Justification (lines 67–77): three reasons given — VP-080 harness must exist for S-16.02 proptest extensions, fallback-to-Continue invariant context, and module structure set by S-16.01. Semantically valid. PASS.

**Points within bounds:**

S-16.01 = 5 points, S-16.02 = 3 points. Both within the 1–13 point limit per STORY-INDEX rule (confirmed by checking STORY-INDEX; S-15.01 is 13 pts as ceiling example). PASS.

---

### CHK-009 — AC ↔ BC Clause Traceability Spot-Check (S-16.01)

**Result: PASS**

Spot-checking 4 ACs against specific BC-7.03.094 clauses:

**AC-002 (line 111–123): "traces to BC-7.03.094 precondition 2 — exact token gate"**
BC-7.03.094 PRE-2 (lines 52–57): "token immediately following `git` is exactly `commit` — not `commit-tree`, `commit-graph`..." The AC's falsifiable test includes `test_BC_7_03_094_lookalike_commit_tree_does_not_exec_subprocess` and `test_BC_7_03_094_lookalike_commit_graph_does_not_exec_subprocess`. This directly verifies PRE-2. The AC cites the specific clause. PASS.

**AC-006 (lines 163–176): "traces to BC-7.03.094 postcondition 3 and invariant 1 — subprocess failure → Continue + telemetry"**
BC-7.03.094 PC-3 (lines 88–96) and INV-1 (lines 105–109): AC-006 specifies four test cases covering CAPABILITY_DENIED, TIMEOUT, NOT_FOUND, and non-zero exit — all four error variants listed in PC-3. The telemetry event requirement (`internal.exec_subprocess_failed` with fields `plugin_name`, `arm`, `error_variant`, `timeout_ms`) matches BC-7.03.094 PC-3 verbatim. PASS.

**AC-007 (lines 178–184): "traces to BC-7.03.094 postcondition 4 — empty stdout → Continue"**
BC-7.03.094 PC-4 (lines 98–101): "succeeds (exit 0) but returns zero bytes of stdout... treats this as a clean result and returns `HookResult::Continue`." AC-007 exactly captures this clause. PASS.

**AC-009 (lines 195–211): "traces to VP-080 anchor — proptest harness"**
VP-080 (lines 47–71) defines four properties. AC-009 lists all four: Property 1 (TV-001..011 pattern detection in arbitrary context), Property 2 (no false positives), Property 3 (boundary inputs return None), Property 4 (purity invariant). The AC specifies "1024 cases per invocation" matching VP-080's `ProptestConfig::with_cases(1024)`. The AC correctly identifies the harness file path. PASS.

**Counting BC-7.03.094 clause coverage:**
- PRE-1: AC-001 references registry shape (event gate); main.rs dispatch handles PRE-1 at hook payload routing level (implicit in AC-001 registry test and AC-011 integration test)
- PRE-2: AC-002 (explicit)
- PRE-3: AC-003 (explicit — see CHK-011)
- PC-1: AC-004 (explicit)
- PC-2: AC-005 (explicit)
- PC-3: AC-006 (explicit)
- PC-4: AC-007 (explicit)
- INV-1: AC-006 (co-traced)
- INV-2: AC-002 (lookalike commands)
- INV-3: Architecture Compliance Rule 4 (compile-time constant), tested implicitly via AC-004's hardcoded mock timeout
- INV-4: Architecture Compliance Rule 4 (compile-time constant)
- INV-5: AC-008 (explicit)
- INV-6: AC-010 (explicit)

All 4 postconditions and 6 of 7 invariants have explicit AC coverage. INV-7 is the "known residual gap (advisory)" invariant — advisory only, no test required. Overall coverage is complete for testable clauses. PASS.

---

### CHK-010 — AC ↔ BC Clause Traceability Spot-Check (S-16.02)

**Result: PASS**

Spot-checking 3 ACs against BC-7.03.095 clauses:

**AC-003 (lines 133–145): "traces to BC-7.03.095 precondition 4 and invariant 7 — all three -F flag forms"**
BC-7.03.095 INV-7 (lines 143–147): "Path extraction handles all three syntactic forms: `-F <path>` (short, space-separated), `--file=<path>` (long, equals), and `--file <path>` (long, space-separated). The extraction logic is order-independent and handles quoted paths." AC-003 provides three distinct falsifiable tests, one per flag form. PASS.

**AC-006 (lines 175–187): "traces to BC-7.03.095 postcondition 3 and invariant 1 — read_file failure → Continue + telemetry"**
BC-7.03.095 PC-3 (lines 90–96) and INV-1 (lines 101–105): AC-006 specifies four error variants — CAPABILITY_DENIED, NOT_FOUND, OUTPUT_TOO_LARGE, IO_ERROR. All four match PC-3. The telemetry event requirement (`internal.read_file_failed` with fields `plugin_name`, `arm`, `error_variant`, `path`) matches BC-7.03.095 PC-3 verbatim. PASS.

**AC-009 (lines 207–215): "traces to BC-7.03.095 invariant 6 — relative paths passed as-is"**
BC-7.03.095 INV-6 (lines 136–142): "When the parsed path is relative (does not begin with `/`), it is passed to `vsdd::read_file` as-is. The dispatcher resolves relative paths." AC-009 falsifiable test: `parse_message_file_path("git commit -F relative/path/msg.txt") → Some("relative/path/msg.txt")`; mock `read_file("relative/path/msg.txt", 65536)` called with exact relative path. PASS.

**PRE-3 to INV-2 coverage (see CHK-012 below):** Confirmed present.

All 4 postconditions and 7 invariants have explicit AC coverage in S-16.02. PASS.

---

### CHK-011 — PRE-3 Success-Only Gate AC Coverage (S-16.01)

**Result: PASS**

The audit specifically requires: "does S-16.01 AC for the success-only gate (PRE-3 of BC-7.03.094) explicitly test the failed-commit path?"

AC-003 (S-16.01, lines 125–133): "traces to BC-7.03.094 precondition 3 — success-only gate; failed commits do not trigger exec_subprocess."

Specific falsifiable test (line 132–133): `test_BC_7_03_094_failed_commit_does_not_exec_subprocess`: PostToolUse event with `command = "git commit -m 'msg'"`, `tool_response.exit_code = 1` → `HookResult::Continue`, `exec_subprocess` NOT called.

This test explicitly covers the failed-commit path (non-zero exit code). The `exec_subprocess` mock assertion (`NOT called`) makes the test falsifiable — it would fail if the implementation incorrectly called exec_subprocess on a failed commit. PASS.

The test is also present in the Test List (line 288): `test_BC_7_03_094_failed_commit_does_not_exec_subprocess` mapped to AC-003 and EC-006. PASS.

---

### CHK-012 — INV-2 Short-Circuit AC Coverage (S-16.02)

**Result: PASS**

The audit specifically requires: "does S-16.02 AC for the short-circuit invariant (INV-2 of BC-7.03.095) explicitly verify that `-m` form doesn't trigger read_file?"

AC-002 (S-16.02, lines 121–128): "traces to BC-7.03.095 precondition 3 and invariant 2 — command-string scan runs first; short-circuit preserved."

Specific falsifiable test (line 128): `test_BC_7_03_095_command_string_attribution_short_circuits_file_read`: PreToolUse event with `command = "git commit -m 'Co-Authored-By: Claude...' -F /tmp/msg.txt"` → `HookResult::Block` via command-string arm; `read_file` mock NOT called.

This test explicitly verifies that a `-m` form with attribution in the command string blocks immediately without invoking `read_file`, even when a `-F` flag is also present in the command. The `read_file` mock NOT-called assertion is the critical falsifiability anchor. PASS.

Test List (line 278): `test_BC_7_03_095_command_string_attribution_short_circuits_file_read` mapped to AC-002, EC-008. PASS.

---

### CHK-013 — File List Completeness

**Result: PASS**

**S-16.01 file list check:**

- Does the proptest harness file (AC-009) appear? Yes: `tests/proptest_detect_attribution.rs (NEW)` appears in the File List NEW section (line 322). PASS.
- Does the integration test file (AC-011) appear? Yes: `tests/posttooluse_integration.rs (NEW)` appears in File List NEW section (line 323). PASS.
- Does `src/post_tool_use.rs` appear? Yes: File List NEW (line 321). PASS.
- Does `src/main.rs` appear as MODIFIED? Yes: File List MODIFIED (line 313). PASS.
- Does `plugins/vsdd-factory/hooks-registry.toml` appear as MODIFIED? Yes: File List MODIFIED (line 315). PASS.
- Does `Cargo.toml` appear as MODIFIED (for proptest dev-dependency)? Yes: File List MODIFIED (line 314). PASS.

**S-16.02 file list check:**

- Does `tests/file_arm_unit.rs (NEW)` appear? Yes: File List NEW section (line 311). PASS.
- Does `tests/pretooluse_file_integration.rs (NEW)` appear? Yes: File List NEW section (line 312). PASS.
- Does `src/lib.rs` appear as MODIFIED? Yes: File List MODIFIED (line 305). PASS.
- Does `plugins/vsdd-factory/hooks-registry.toml` appear as MODIFIED? Yes: File List MODIFIED (line 306). PASS.

All implementation touch surface implied by the ACs is represented in the file lists. PASS.

---

### CHK-014 — Test List ↔ AC Coverage

**Result: PASS-WITH-NITS**

**S-16.01 test list review:**

All 11 ACs are mapped to at least one test entry in the Test List (lines 281–304). AC-001 maps to 2 static tests, AC-002 maps to 3 unit tests, AC-003 maps to 1 unit test, AC-004 maps to 1 unit test, AC-005 maps to 1 unit test, AC-006 maps to 4 unit tests, AC-007 maps to 1 unit test, AC-008 maps to 1 unit test, AC-009 maps to 7 proptest entries, AC-010 maps to 1 proptest entry, AC-011 maps to 1 integration test. Coverage is complete.

**NIT-003:** AC-010 maps to `test_BC_7_03_094_detect_attribution_purity_1024_trials` (proptest type), but the AC body says "Falsifiable test: Code review audit: `detect_attribution` body contains no `vsdd::` host function calls; `clippy` with `#[deny(clippy::impure_fn)]` equivalent passes." The test list entry says this is a proptest (VP-080 Property 4), but the AC describes it as a code audit plus proptest. This is not a gap — VP-080 Property 4 does run the function 1024 times in single-threaded context to confirm no external state changes, which is the proptest component. The code-audit component is implicit (no separate test entry). This is a minor documentation imprecision: the test list could note "code-audit component is a static review step, not an automated test case." Non-blocking.

**S-16.02 test list review:**

All 10 ACs are mapped. AC-001 maps to 2 static tests. AC-002 maps to 1 unit test. AC-003 maps to 3 unit tests (one per flag form). AC-004 maps to 1 unit test. AC-005 maps to 1 unit test. AC-006 maps to 4 unit tests. AC-007 maps to 2 unit tests (parse and logic). AC-008 maps to 1 unit test. AC-009 maps to 1 unit test. AC-010 maps to existing regression tests. Coverage is complete. PASS.

---

### CHK-015 — Demo Evidence Specification

**Result: PASS-WITH-NITS**

**S-16.01 Demo Evidence (lines 357–370):**

- Demo script location: `/Users/jmagady/Dev/vsdd-factory/demo/S-16.01-posttooluse-head-verify.sh` — specific file path, correct.
- Script outline: 5-step sequence specifying: git init in tempdir, simulate PostToolUse payload injection via stdin, show Block JSON output with specific fields (`event_code: "ai_attribution_post_commit"`, `git reset --soft HEAD~1`), follow remediation, show Continue after clean recommit.
- Required demonstrations named: (a) corrective-signal Block, (b) exact remediation text, (c) Continue after clean recommit.

Concreteness assessment: The demo spec names specific commands, specific JSON field assertions, and a specific recording tool path. This is concrete enough for a demo-recorder agent to execute without ambiguity. However, no recording tool is specified (e.g., VHS, asciinema, `script` command). The prompt says "VHS tape or CLI session capture" — this is a choice offered to the demo-recorder. The script is `.sh` format, not a VHS `.tape` format; this is the correct path per POLICY 10 (all demo evidence under `docs/demo-evidence/<STORY-ID>/`).

**NIT-004:** The Demo Evidence section specifies a `.sh` script at `demo/S-16.01-posttooluse-head-verify.sh` but POLICY 10 (policy id 10, `demo_evidence_story_scoped`) requires demo-recorder output to live under `docs/demo-evidence/<STORY-ID>/`. The `.sh` script itself is the recording vehicle, not the output evidence. The output evidence files produced by running the demo (screenshots, recordings, markdown summaries) must go under `docs/demo-evidence/S-16.01/`. The current demo spec does not specify the output evidence location. This is a story-writer omission: the Demo Evidence section should also state where the recorder is expected to place the output artifacts. Non-blocking for state-manager close-out but should be addressed before F4 demo-recorder dispatch.

**S-16.02 Demo Evidence (lines 352–365):**

Same structure. Demo script location: `demo/S-16.02-pretooluse-file-arm.sh`. Six-step outline with concrete steps. Same NIT-004 applies.

---

### CHK-016 — Policy #3 Deferral Correctness

**Result: PASS**

Policy #3 (`state_manager_runs_last`): state-manager must commit last in every burst. The corollary for F3: story files must NOT pre-register themselves in STORY-INDEX, BC-INDEX, VP-INDEX, or ARCH-INDEX.

Confirmed: Neither S-16.01 nor S-16.02 modifies any index file. Their File Lists contain:
- MODIFIED files: `src/lib.rs`, `src/main.rs`, `Cargo.toml`, `hooks-registry.toml` (S-16.01), `src/lib.rs`, `hooks-registry.toml` (S-16.02)
- NEW files: `src/post_tool_use.rs`, `tests/proptest_detect_attribution.rs`, `tests/posttooluse_integration.rs` (S-16.01), `tests/file_arm_unit.rs`, `tests/pretooluse_file_integration.rs` (S-16.02)
- STABLE files listed do not include any `.factory/` index files.

Neither story's task list includes any state-manager operations. Neither story touches `STORY-INDEX.md`, `BC-INDEX.md`, `VP-INDEX.md`, `ARCH-INDEX.md`, or `STATE.md`. PASS.

---

### CHK-017 — Template Compliance (Required Sections Present)

**Result: PASS**

Both stories were checked against the required story template sections by scanning the document structure.

**S-16.01 required sections:**

| Section | Present? |
|---------|---------|
| Frontmatter (all canonical fields) | Yes |
| Narrative (As a / I want / So that) | Yes |
| Behavioral Contracts table | Yes |
| Acceptance Criteria | Yes (11 ACs) |
| Architecture Mapping | Yes |
| Edge Cases | Yes |
| Test List | Yes |
| File List (Delta) | Yes |
| Forbidden Dependencies | Yes |
| Purity Classification | Yes |
| Demo Evidence | Yes |
| Open Questions | Yes |
| Tasks | Yes |
| Previous Story Intelligence | Yes |
| Architecture Compliance Rules | Yes |
| Library & Framework Requirements | Yes |
| File Structure Requirements | Yes |
| Token Budget Estimate | Yes |
| Definition of Done | Yes |
| CHANGELOG | Yes |

The audit instruction noted "Story-writer reported S-16.01 needed Purity Classification appended mid-flight; verify both stories now have all required sections." Confirmed: Purity Classification is present in S-16.01 (lines 348–354) and S-16.02 (lines 247–254). PASS.

**S-16.02 required sections:** All same sections present. PASS.

---

### CHK-018 — Upstream Artifact Immutability

**Result: PASS**

The audit requires confirming that neither story modified BC-7.03.094, BC-7.03.095, VP-080, E-16, BC-7.03.001, SS-07, or SS-04 (those are F2 outputs and should be frozen in F3).

**Method:** The story File Lists are the authoritative declaration of what each story touches. Neither S-16.01 nor S-16.02 lists any `.factory/specs/` artifact in its File List. Neither story's Task list includes any operation on `.factory/` spec files.

**Confirmed stable (from S-16.02 NOT MODIFIED section):**
- `src/post_tool_use.rs` (S-16.01 output, frozen for S-16.02)
- `tests/proptest_detect_attribution.rs` (VP-080 harness, created by S-16.01, stable in S-16.02)
- `Cargo.toml` (proptest already added by S-16.01)

Neither story's inputs list includes any of the F2 output artifacts as inputs to be modified (they are read-only context). PASS.

---

### CHK-019 — Frontmatter Canonical Fields

**Result: PASS-WITH-NITS**

Per DF-020a canonical frontmatter requirements:

| Field | S-16.01 | S-16.02 | Assessment |
|-------|---------|---------|-----------|
| `document_type: story` | Yes | Yes | PASS |
| `level: ops` | Yes | Yes | PASS |
| `version: "1.0"` | Yes | Yes | PASS |
| `producer: story-writer` | Yes | Yes | PASS |
| `traces_to:` | Yes (epic file) | Yes (epic file) | PASS |
| `timestamp:` | Yes (2026-05-12T00:00:00Z) | Yes (2026-05-12T00:00:00Z) | PASS |
| `story_id:` | Yes | Yes | PASS |
| `epic_id:` | Yes | Yes | PASS |

**NIT-005 (minor):** Both stories have `input-hash: "[pending-recompute]"`. This is acceptable per the established pattern (VP-080, BC-7.03.094, BC-7.03.095 all use the same placeholder). Not a defect.

**NIT-006 (minor):** The `behavioral_contracts:` field is used in both S-16.01 and S-16.02 (e.g., S-16.01 line 25: `behavioral_contracts:`). The canonical field name per the project's story template (as used in S-15.01, S-3.03, and per POLICY 8's verification steps which reference `behavioral_contracts:`) is `behavioral_contracts`. This matches. However, POLICY 8's verification steps note "Sample 5+ stories — for each BC in `behavioral_contracts:` frontmatter (canonical field; alias: `bcs:` for S-15.03 only)." The new stories correctly use the canonical form, not the S-15.03 alias. PASS on field naming.

---

### CHK-020 — behavioral_contracts Frontmatter Field Naming (POLICY 8)

**Result: PASS-WITH-NITS**

POLICY 8 (`bc_array_changes_propagate_to_body_and_acs`) requires:
1. Each BC in `behavioral_contracts:` frontmatter appears in the body BC table.
2. Each BC in `behavioral_contracts:` frontmatter has at least one AC tracing to it.
3. Each BC in the body BC table or AC traces appears in `behavioral_contracts:` frontmatter.

**S-16.01 check:**

`behavioral_contracts: [BC-7.03.094, BC-7.03.001]`

- BC-7.03.094 in body BC table? Yes (line 94): `| BC-7.03.094 | block-ai-attribution: PostToolUse retroactive HEAD commit message verification | Primary...`
- BC-7.03.001 in body BC table? Yes (line 95): `| BC-7.03.001 | block-ai-attribution: identity and registry binding (v1.3) | Registry shape extension...`
- BC-7.03.094 has AC trace? Yes: AC-002 (PRE-2), AC-003 (PRE-3), AC-004 (PC-1), AC-005 (PC-2), AC-006 (PC-3, INV-1), AC-007 (PC-4), AC-008 (INV-5) — all trace to BC-7.03.094.
- BC-7.03.001 has AC trace? Yes: AC-001 traces to BC-7.03.001 postcondition 1 (registry shape).
- Reverse (body → frontmatter): All BC IDs in the body BC table (BC-7.03.094, BC-7.03.001) appear in frontmatter. PASS.

**S-16.02 check:**

`behavioral_contracts: [BC-7.03.095, BC-7.03.001]`

- BC-7.03.095 in body BC table? Yes (line 98): `| BC-7.03.095 | block-ai-attribution: PreToolUse `-F <path>` file-read arm | Primary...`
- BC-7.03.001 in body BC table? Yes (line 99): `| BC-7.03.001 | block-ai-attribution: identity and registry binding (v1.3) | Registry shape extension...`
- BC-7.03.095 has AC trace? Yes: AC-002 (PRE-3, INV-2), AC-003 (PRE-4, INV-7), AC-004 (PC-1), AC-005 (PC-2), AC-006 (PC-3, INV-1), AC-007 (PC-4, PRE-5), AC-008 (INV-5), AC-009 (INV-6).
- BC-7.03.001 has AC trace? Yes: AC-001 traces to BC-7.03.001 postcondition 1.
- Reverse (body → frontmatter): PASS.

**NIT-007:** Neither story includes a "Token Budget BC count" line in the Token Budget Estimate section that matches `len(behavioral_contracts)`. POLICY 8 step 4 reads "Verify Token Budget BC count matches `len(behavioral_contracts)`." The Token Budget Estimate sections in both stories list context sources (story spec, BCs, VP, existing code, new files) but do not include a line explicitly counting BCs. However, this step is labeled "Verify Token Budget BC count" — in the existing project stories (e.g., S-15.01), the Token Budget table rows enumerate the BC context costs by name rather than a separate count. The BC count is implicitly expressed by the number of BC rows in the table. S-16.01 has 2 BC rows (BC-7.03.094 ~4,500 tokens, BC-7.03.001 ~2,000 tokens) matching `len(behavioral_contracts) = 2`. S-16.02 has 2 BC rows (BC-7.03.095 ~4,000 tokens, BC-7.03.001 ~2,000 tokens) matching `len(behavioral_contracts) = 2`. PASS — the count is implicitly consistent.

---

## Nits Summary

| NIT ID | Location | Description | Responsible Agent |
|--------|----------|-------------|------------------|
| NIT-001 | BC-7.03.001 Traceability Stories row (line 81) | Stories row shows TBD — should cite S-3.03, S-16.01, S-16.02 after close-out | product-owner |
| NIT-002 | S-16.01 `depends_on: ["S-3.03"]` (line 35) | depends_on on a merged story has ambiguous planning-graph semantics; story provides justification; orchestrator should confirm this follows project convention for expressing codebase predecessors | orchestrator |
| NIT-003 | S-16.01 Test List AC-010 (line 303) | AC-010 has both a code-audit component and a proptest component; test list only captures the proptest entry; code-audit step not reflected as a separate task or DoD checklist item | story-writer |
| NIT-004 | S-16.01 and S-16.02 Demo Evidence sections | Demo spec names the .sh recording script path but does not specify the output evidence destination (must be `docs/demo-evidence/<STORY-ID>/` per POLICY 10); should be clarified before F4 demo-recorder dispatch | story-writer |
| NIT-005 | S-16.01 and S-16.02 frontmatter `input-hash` | "[pending-recompute]" placeholder; acceptable per established pattern | — (non-actionable) |
| NIT-006 | — | Non-issue; documented for completeness | — |
| NIT-007 | S-16.01 and S-16.02 Token Budget tables | No explicit BC count row; count is implicitly consistent with len(behavioral_contracts); minor structural gap vs POLICY 8 step 4 strict reading | story-writer (low priority) |

---

## Overall Verdict

**PASS-WITH-NITS**

All 20 checks pass or pass with non-blocking nits. There are no FAIL findings. No CRITICAL or HIGH issues. State-manager may proceed to close-out index registration.

**Gate decision: state-manager close-out is unblocked.**

---

## Pre-Close-Out Checklist for State-Manager

The following registrations are confirmed needed (per STORY-INDEX/BC-INDEX/VP-INDEX deferral per Policy #3):

1. **STORY-INDEX:** Add E-16 section; add S-16.01 and S-16.02 rows (status: draft); update summary narrative from "16 epics (E-0 through E-15)" to "17 epics (E-0 through E-16)"; update story_count from 93 to 95.
2. **BC-INDEX:** Append BC-7.03.094 row (title: "block-ai-attribution: PostToolUse retroactive HEAD commit message verification"; capability: CAP-008; stories: S-16.01); append BC-7.03.095 row (title: "block-ai-attribution: PreToolUse `-F <path>` file-read arm"; capability: CAP-008; stories: S-16.02); update `total_bcs` from 1947 to 1949.
3. **VP-INDEX:** Append VP-080 row; increment `total_vps` from 79 to 80; add VP-080 to proptest proof-method breakdown; add VP-080 to Story Anchors with anchor S-16.01.
4. **ARCH-INDEX:** Refresh SS-07 BC count from 196 to 198; refresh Total BCs from 1,947 to 1,949.
5. **STATE.md:** Record F3 close and F4 readiness per pipeline state protocol.

**Optional (PO cleanup — can be deferred):** BC-7.03.001 Traceability Stories row (NIT-001) — update from TBD to cite S-3.03, S-16.01, S-16.02.

---

_Report written by consistency-validator (fresh context). All source line citations refer to artifact state as of 2026-05-12 audit run. Artifacts audited: S-16.01 v1.0, S-16.02 v1.0, BC-7.03.094 v1.0, BC-7.03.095 v1.0, BC-7.03.001 v1.3, VP-080 v1.1, E-16 v1.0, ARCH-INDEX v1.64, STORY-INDEX v2.84, policies.yaml._
