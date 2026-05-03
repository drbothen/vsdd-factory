---
document_type: architecture-audit
level: L4
section: "w16-tier2-migration-audit"
version: "1.0"
status: draft
producer: architect
timestamp: 2026-05-03T00:00:00Z
phase: "Phase D — W-16 spec foundation"
traces_to: STATE.md (D-9.1/D-9.2/D-9.3 open decisions)
---

# W-16 Tier 2 Native WASM Migration — Audit & Recommendations

> Produced for orchestrator review before ADR-014 and SS-02/SS-04 updates.
> **DO NOT modify this file to add ADR-014 or spec changes until orchestrator approves
> recommendations in Sections 2–4.**

---

## Section 1 — Hook Inventory & Capability Matrix

All 23 hooks currently route through `legacy-bash-adapter.wasm` with `script_path`
pointing to `hooks/validate-*.sh`. Confirmed via `hooks-registry.toml` lines 145–797.

**Note on `validate-pr-review-posted`:** this hook is already a native WASM plugin
(shipped in W-15, PR #57). It does NOT appear in this inventory because it is not a
`validate-*.sh` file. The 23 hooks below are the complete Tier 2 scope.

| Hook name | Inputs (stdin/env) | External tools used | File reads | File writes | Subprocess calls | git ops | Behavioral category |
|-----------|-------------------|---------------------|-----------|-------------|-----------------|---------|---------------------|
| validate-anchor-capabilities-union | stdin JSON (PostToolUse); `FILE_PATH` from tool_input | jq, awk, grep, sed, sort, tr | story file + BC files (multi-file) | none | none | none | frontmatter-related |
| validate-bc-title | stdin JSON (PostToolUse); `FILE_PATH` | jq, awk, grep, sed | BC file | none | none | none | frontmatter-related |
| validate-changelog-monotonicity | stdin JSON (PostToolUse); `FILE_PATH` | jq, awk, grep | story/arch file | none (ERRORS var only) | none | none | frontmatter-related |
| validate-count-propagation | stdin JSON (PostToolUse); `FILE_PATH` | jq, grep | index/state/arch file + corpus siblings | none | none | none | state-related |
| validate-demo-evidence-story-scoped | stdin JSON (PostToolUse); `FILE_PATH` | jq | demo-evidence file | none | none | none | story-related |
| validate-factory-path-root | stdin JSON (PostToolUse); `FILE_PATH` | jq, grep, head | written file path | none | none | none | story-related |
| validate-finding-format | stdin JSON (PostToolUse); `FILE_PATH` | jq, grep, sort | adversarial review file | none (ERRORS in-memory only) | none | none | story-related |
| validate-index-self-reference | stdin JSON (PostToolUse); `FILE_PATH` | jq, awk, grep, head | cycle INDEX/burst-log file | none | none | none | state-related |
| validate-input-hash | stdin JSON (PostToolUse); `FILE_PATH` | jq, awk, grep | VSDD spec file with `input-hash:` | none | none | none | frontmatter-related |
| validate-novelty-assessment | stdin JSON (PostToolUse); `FILE_PATH` | jq, grep | adversarial review file | none | none | none | story-related |
| validate-pr-description-completeness | stdin JSON (PostToolUse); `FILE_PATH` | jq, grep, head, sed, sort, tr | pr-description.md | none | none | none | PR-related |
| validate-pr-merge-prerequisites | stdin JSON (PreToolUse); `tool_input.prompt` | jq, grep, head, sed, find | code-delivery/STORY-*/pr-description.md, pr-review.md, security-review.md | none | none (gh match is text pattern in prompt, not CLI call) | none | PR-related |
| validate-red-ratio | stdin JSON (PostToolUse); `FILE_PATH` | jq, grep, sed, tr | story file with `red_ratio:` | none | none | none | story-related |
| validate-state-index-status-coherence | stdin JSON (PostToolUse); `FILE_PATH` | jq, awk, tr | STATE.md or cycle INDEX.md | none | none | none | state-related |
| validate-state-pin-freshness | stdin JSON (PostToolUse); `FILE_PATH` | jq, awk | STATE.md | none | none | none | state-related |
| validate-state-size | stdin JSON (PostToolUse); `FILE_PATH` | jq, wc, tr | STATE.md | none | none | git show HEAD:STATE.md (compaction check; graceful skip if git absent) | state-related |
| validate-story-bc-sync | stdin JSON (PostToolUse); `FILE_PATH` | jq, awk, grep, sort | story file | temp file (ERRFILE via mktemp, deleted on EXIT) | none | none | story-related |
| validate-subsystem-names | stdin JSON (PostToolUse); `FILE_PATH` | jq, awk, grep, sed | BC or story file + ARCH-INDEX.md | none | none | none | frontmatter-related |
| validate-table-cell-count | stdin JSON (PostToolUse); `FILE_PATH` | jq, grep, tr, wc | any .factory markdown file | none | none | none | frontmatter-related |
| validate-template-compliance | stdin JSON (PostToolUse); `FILE_PATH` | jq, awk, grep, sed, sort, tr, wc | target file + template file | none | none | none | frontmatter-related |
| validate-vp-consistency | stdin JSON (PostToolUse); `FILE_PATH` | jq, awk, grep, sed, sort, sh | VP-INDEX.md + verification-architecture.md + coverage-matrix.md | temp files (ERRFILE + .tools/.declared/.matrix_sums, all deleted on EXIT) | none | none | frontmatter-related |
| validate-wave-gate-completeness | stdin JSON (PostToolUse); `FILE_PATH` | jq, grep, python3 (yaml) | wave-state.yaml + gate_report file | none | none | none | wave-related |
| validate-wave-gate-prerequisite | stdin JSON (PreToolUse); `tool_input.prompt` | jq, grep, head, sed, python3 (yaml), bash, sh, find | wave-state.yaml + STORY-INDEX (story metadata) | none | bash subprocess to `verify-sha-currency.sh` (optional; graceful skip if absent) | git in error-message text only (not called as subprocess) | gate-related |

### Notes on "File Writes" column

- `validate-story-bc-sync` and `validate-vp-consistency` use `mktemp` for temporary
  accumulator files. These are NOT persisted outputs — they are deleted in `trap EXIT`.
  In Rust, these would be `Vec<String>` in memory; no `host::write_file` needed.
- `validate-finding-format` and `validate-changelog-monotonicity` write to an in-memory
  `ERRORS` variable (a string), not to disk. No file I/O.

### Summary counts by category

| Category | Count | Hooks |
|----------|-------|-------|
| frontmatter-related | 9 | anchor-capabilities-union, bc-title, changelog-monotonicity, input-hash, subsystem-names, table-cell-count, template-compliance, vp-consistency, (also: state-pin-freshness is borderline) |
| state-related | 5 | count-propagation, index-self-reference, state-index-status-coherence, state-pin-freshness, state-size |
| story-related | 6 | demo-evidence-story-scoped, factory-path-root, finding-format, novelty-assessment, red-ratio, story-bc-sync |
| PR-related | 2 | pr-description-completeness, pr-merge-prerequisites |
| wave-related | 1 | wave-gate-completeness |
| gate-related | 1 | wave-gate-prerequisite |

**Total: 23 hooks in 6 behavioral categories.**

---

## Section 2 — D-9.1 Recommendation: Port Strategy

### The three candidates

- **(a) rewrite-clean** — Idiomatic Rust: `regex` crate for pattern matching, `serde_json` for
  JSON, `serde_yaml` for YAML, `walkdir`/`glob` for filesystem traversal. Does not preserve
  bash-specific regex syntax or toolchain quoting behavior.
- **(b) port-as-is** — 1:1 bash-to-Rust translation. Every awk field-split, every sed
  substitution, every `jq -r '.x // empty'` translated identically. Preserves all quirks.
- **(c) hybrid** — Rewrite-clean for low-complexity validators; port-as-is only where
  correctness depends on exact bash behavior with documented quirks.

### Analysis

**Complexity distribution from Section 1:**

Looking at line counts and tool usage:

- 55–100 lines, jq + grep only (9 hooks): demo-evidence-story-scoped, factory-path-root,
  finding-format, novelty-assessment, pr-description-completeness, state-index-status-coherence,
  state-size, story-bc-sync, table-cell-count. These are simple file-read + pattern-check
  pipelines. Zero awk. Trivially rewritable in Rust.
- 100–160 lines, jq + awk + multi-file (10 hooks): anchor-capabilities-union, bc-title,
  changelog-monotonicity, count-propagation, index-self-reference, input-hash, red-ratio,
  state-pin-freshness, subsystem-names, template-compliance. Moderate complexity. awk
  field-splitters replace cleanly with `serde_yaml`/frontmatter parsers in Rust.
- 160–250 lines, complex multi-tool (4 hooks): vp-consistency (248 lines; 4 temp files;
  awk arithmetic; multiple grep passes over 3 arch files), wave-gate-prerequisite (204 lines;
  bash subprocess; python3 YAML; complex prompt parsing), wave-gate-completeness (140 lines;
  python3 YAML), validate-pr-merge-prerequisites (123 lines; file presence + grep scan).

**W-15 lesson: OQ-001 / S-8.04 bash regex precedence (the definitive "preserved quirk" case)**

`update-wave-state-on-merge.sh` used the POSIX ERE pattern:
```
STEP_COMPLETE: step=8.*status=ok|merged|squash.*merge
```
which bash/grep parse as `(STEP_COMPLETE...ok) | (merged) | (squash.*merge)` due to `|`
having lower precedence than concatenation in POSIX ERE. The intended logic was
`STEP_COMPLETE.*step=8.*(ok|merged|squash.*merge)`. During the W-15 adversarial review
(ADV-S8.04-P1 finding OQ-001), the team considered port-as-is versus fix. They chose
port-as-is (D-2 Option C) and filed a TD entry for a grouped-alternation fix at v1.2.

This is the canonical "preserved bash quirk that recompiles correctly but behaves unexpectedly
once the bash regex environment is gone." In a pure Rust rewrite with the `regex` crate, the
developer would naturally write the correct grouped pattern — but the bash tests (which test
the `.sh` file, not the `.wasm`) would continue to pass the quirked behavior, masking the
behavioral change. This is the key risk that favors rewrite-clean: the bash test suite can only
validate bash execution, not WASM execution.

**Concrete line-count impact:**

The bash hooks use `awk` field-splitting, `sed` in-place substitution idioms, and `jq` for JSON
parsing (all done in-process via shell piping). Rust equivalents:
- `jq -r '.x // empty'` → direct `serde_json` field access: shorter.
- `awk '/^---/{fm++;next} fm==1 && /^key:/ {print}` → YAML frontmatter parse with `serde_yaml`:
  shorter and more robust.
- `grep -oE 'pattern'` → `regex::Regex::find_iter`: comparable length.
- `sed 's/a/b/g'` → `str::replace` or `regex::Regex::replace_all`: shorter.

Expected line-count change: `validate-*.sh` average 143 lines; Rust equivalent average
estimated 80–120 lines (44% reduction). Complex hooks (vp-consistency at 248 lines) may
reduce to 150 lines.

### Recommendation: **(a) rewrite-clean**

**Rationale:**

1. **No high-risk quirks outside S-8.04's OQ-001.** The audit found only two hooks with
   genuine behavioral complexity deserving care:
   - `validate-vp-consistency`: awk arithmetic on multi-file parse. Rewrite-clean using
     `serde_yaml` is safer; the current awk is ad-hoc YAML tokenization that would be
     fragile in Rust's type system anyway.
   - `validate-wave-gate-prerequisite`: python3 YAML parsing. Replace with `serde_yaml`;
     exact same correctness profile, no quirk risk.
2. **OQ-001 is the only confirmed preserved-quirk case in Tier 1, and it was deliberate.**
   None of the 23 Tier 2 hooks show a comparable documented quirk. A fresh survey for each
   hook in the story spec (adversarial pass discipline) will surface any cases before
   implementation.
3. **Rewrite-clean eliminates the jq dependency entirely.** All 23 hooks begin with
   `if ! command -v jq &>/dev/null; then exit 0; fi` — jq absence causes a silent
   pass. A Rust WASM plugin has no such fragility.
4. **The bash test suite tests bash execution.** After WASM port, the `.sh` files still
   exist in the repo (until Phase H). Port-as-is creates a permanent divergence: the bats
   tests validate the `.sh` quirk, but the WASM dispatcher runs the Rust version. If a quirk
   is preserved in Rust, it must be documented; if it is not, the bats test will give a false
   green. Rewrite-clean forces the story spec to explicitly document expected behavior,
   making the WASM tests the ground truth.
5. **20/23 hooks have zero subprocess needs.** They are pure file-read + regex + text-compare
   pipelines. Idiomatic Rust is strictly cleaner here.

**Mitigation for rewrite-clean risk:** Each story spec must list every behavioral edge case
from the bash original as ACs (mirroring S-8.04's EC tables for YAML trichotomy, TOCTOU, and
OQ-001). The adversarial pass discipline (ADR-013) catches behavioral divergences before
implementation.

---

## Section 3 — D-9.2 Recommendation: Subprocess Capability

### The three candidates

- **(a) New `host::run_subprocess` ABI** — generalizable binary+arg allow-list under
  `BC-2.02.013`. Covers any binary the capability schema permits.
- **(b) Specific host fns** — `host::git_log`, `host::gh_pr_view`,
  `host::gh_pr_checks`. Narrower attack surface.
- **(c) Skip subprocess-needing validators for W-16, defer to W-17** — Port only the
  19 no-subprocess hooks in W-16; defer 4 subprocess hooks to W-17.

### Survey: subprocess requirements by category

**jq only → `serde_json` sufficient (no subprocess):**
All 23 hooks use jq to parse the Claude Code hook envelope from stdin. In the WASM
environment, stdin is deserialized to `HookPayload` by the SDK. The hooks then use
jq to extract `FILE_PATH` or `PROMPT`. In Rust this is direct field access on `HookPayload`.
**No subprocess needed for jq replacement.**

**awk/sed/grep/tr → `regex` crate (no subprocess):**
18 hooks use awk for field-splitting frontmatter. The `regex` crate replaces this without
subprocess. `sed`/`tr`/`sort` are string operations available in Rust stdlib.
**No subprocess needed.**

**find → `walkdir` / path operations (no subprocess):**
`validate-pr-merge-prerequisites` uses `find` to locate delivery directories. Rust's
`std::fs::read_dir` or `walkdir` replaces this. **No subprocess needed.**

**python3 (YAML parsing) → `serde_yaml` (no subprocess):**
`validate-wave-gate-completeness` and `validate-wave-gate-prerequisite` use Python3 inline
to parse YAML (`yaml.safe_load`). `serde_yaml` replaces this. **No subprocess needed.**

**Remaining genuine subprocess needs:**

| Hook | Subprocess | Binary | Purpose | Can replace? |
|------|-----------|--------|---------|-------------|
| validate-state-size | `git -C "$PARENT_DIR" show HEAD:STATE.md` | git | Compare current line count against committed version to allow compaction writes | Potentially libgit2 binding; or simplify to: skip the compaction-detection branch |
| validate-wave-gate-prerequisite | `bash "$SHA_HOOK" --project-root ...` | bash (running verify-sha-currency.sh) | Currency check for adversary SHA gate | Hook is optional (graceful exit if absent); can skip in WASM |
| validate-wave-gate-prerequisite | `git -C .factory reset --soft HEAD~N` | git | Appears only in error message text, NOT as an actual subprocess call | Not a real subprocess call — text in stderr diagnostic. No action needed. |

**Count:**
- subprocess-free after serde_json/serde_yaml/regex/walkdir replacement: **21 hooks**
- genuine optional subprocess (validate-state-size git; validate-wave-gate-prerequisite bash):
  **2 hooks** (both are optional/graceful-skip paths)
- genuine blocking subprocess: **0 hooks**

**Critical insight:** Neither of the two subprocess calls is in the critical execution path.

`validate-state-size`'s git call is guarded by `if command -v git &>/dev/null; then ... fi` —
if git is absent, the hook uses `PRIOR_COUNT=0` and only blocks if the file exceeds 500 lines
regardless of growth direction. The compaction-detection logic (`if LINE_COUNT < PRIOR_COUNT:
exit 0`) can be dropped in the WASM port without changing the primary invariant (block at 500
lines). The git call is a convenience feature, not a correctness requirement.

`validate-wave-gate-prerequisite`'s bash call is guarded by
`if [[ -z "$SHA_HOOK" ]]; then exit 0; fi` — if `verify-sha-currency.sh` is not found,
the hook exits 0. For W-16 WASM port, this optional subprocess can be dropped; it is a
defense-in-depth check that verifies SHA currency of the verify-sha-currency.sh file itself.
This can remain as a TODO for W-17 if a `host::exec_subprocess` capability is desired.

### Recommendation: **(c) Skip subprocess-needing validators for W-16, with the specific
note that the 2 "subprocess" cases are optional paths that can be simplified away.**

**Refined scope: ALL 23 hooks can ship in W-16 without any new host function.**

The audit reveals the practical scope is broader than the pre-audit D-209 framing suggested.
The two subprocess calls are both optional/graceful-degradation paths:

1. `validate-state-size` git subprocess: drop the compaction-detection logic in W-16. The
   hook's primary invariant (block at >500 lines) is preserved. File the simplified behavior
   as a deliberate simplification in the story EC table, with a TD for v1.2 if git-aware
   compaction detection is desired via `host::exec_subprocess`.
2. `validate-wave-gate-prerequisite` bash subprocess: drop the `verify-sha-currency.sh`
   invocation in W-16. The hook's primary invariant (wave gate ordering) is preserved. File
   the dropped SHA currency check as a deliberate simplification with a TD for W-17.

**Formal recommendation: (c) Skip new host fns for W-16.** All 23 hooks can port to WASM
using only the existing ABI (`read_file`, `emit_event`, `log`). No `host::run_subprocess`,
no `host::git_log`, no `host::gh_pr_view` needed.

**Why not (a)?** A new `host::run_subprocess` ABI (BC-2.02.013) requires PO BC authoring,
adversarial convergence, HOST_ABI.md update, dispatcher implementation, and security review
per OQ-6 pattern (S-8.09 precedent). That is several days of spec work for two optional paths
that can be cleanly simplified. The security tradeoff is asymmetric: `exec_subprocess` with
a bash allow-list is a meaningful attack surface expansion (the entire reason legacy-bash-adapter
required `shell_bypass_acknowledged = true`).

**Why not (b)?** Specific host fns (`host::git_log` etc.) solve a narrow problem that the
simplification approach makes irrelevant. If subprocess capability is added in W-17 for Tier 3
hooks where it is truly needed (rather than optional), (a) or (b) should be revisited then.

---

## Section 4 — D-9.3 Recommendation: Story Granularity

### Analysis of batching options

**Current plan:** "~7 batched stories by capability cluster."

**Behavioral category counts from Section 1:**

| Category | Count |
|----------|-------|
| frontmatter-related | 9 |
| story-related | 6 |
| state-related | 5 |
| PR-related | 2 |
| wave-related | 1 |
| gate-related | 1 |

**Option (a) — One story per validator (23 stories):** The W-15 pattern for high-risk Tier 1
hooks (each hook was 3–5 pts with its own adversarial convergence). Appropriate when each
hook has unique behavioral complexity, distinct BC families, and high risk. For Tier 2 validate
hooks, the behavioral profile is more uniform: mostly read_file + regex + emit_event. 23
stories would over-granulate and produce 23 × ~8 adversarial passes = ~184 passes. Slow.

**Option (b) — One story per behavioral category (~7 stories):** Groups semantically related
hooks together. Each story has 1–9 hooks. Variable scope: a "story-related" story with 6 hooks
at mixed complexity levels risks adversarial review finding inconsistencies between hook
implementations within the story. The "frontmatter-related" category with 9 hooks would be a
large story.

**Option (c) — Capability-cluster batches:** Group by what host functions and capabilities the
Rust implementation requires. This is the natural ordering from the perspective of the
adversarial reviewer (who focuses on capability correctness, BC anchoring, and parity with
the bash original).

**Recommended batching: 7 stories, capability-cluster scheme**

The Section 3 analysis reveals a clean capability hierarchy:

- **Pure stdin-parse + emit**: hooks that only parse `HookPayload.tool_input.file_path` from
  stdin and then call `emit_event`. These need only `emit_event` + Rust stdlib (no `read_file`).
- **Single file-read + regex**: hooks that call `read_file` once (on `FILE_PATH`), run regex
  or text matching, then `emit_event`. The most common pattern.
- **Multi-file-read + cross-document**: hooks that call `read_file` on `FILE_PATH` PLUS read
  one or more additional VSDD spec files (e.g., ARCH-INDEX.md, VP-INDEX.md).
- **Subprocess-simplified**: the two hooks with optional subprocess paths that are simplified
  away in the WASM port.

**Proposed batches (7 stories):**

| Batch | Story ID | Hooks | Description | Host fns needed | Complexity |
|-------|----------|-------|-------------|----------------|------------|
| Batch 1 | S-9.01 | validate-demo-evidence-story-scoped, validate-factory-path-root, validate-finding-format, validate-novelty-assessment | Pure stdin-parse + path-check + emit_event. No file reads. 4 hooks; 55–90 lines each. | emit_event, log | LOW |
| Batch 2 | S-9.02 | validate-bc-title, validate-changelog-monotonicity, validate-red-ratio, validate-input-hash | Single file-read, frontmatter parse. Each validates one field/invariant in the target file. | read_file, emit_event, log | LOW-MED |
| Batch 3 | S-9.03 | validate-pr-description-completeness, validate-table-cell-count, validate-pr-merge-prerequisites | File-read validators for PR/delivery artifacts. pr-merge-prerequisites reads 3 files with presence checks. | read_file, emit_event, log | MED |
| Batch 4 | S-9.04 | validate-state-index-status-coherence, validate-state-pin-freshness, validate-state-size | STATE.md + cycle INDEX validators. state-size gets the subprocess-simplified treatment (drop git compaction-detection). | read_file, emit_event, log | MED |
| Batch 5 | S-9.05 | validate-story-bc-sync, validate-count-propagation, validate-index-self-reference | Multi-file: story file + BC shard files or corpus index lookup. Most complex awk-to-serde conversions. | read_file, emit_event, log | MED-HIGH |
| Batch 6 | S-9.06 | validate-anchor-capabilities-union, validate-subsystem-names, validate-template-compliance | Multi-file: target file + ARCH-INDEX.md + BC shard files + template files. Cross-document lookups. | read_file, emit_event, log | HIGH |
| Batch 7 | S-9.07 | validate-vp-consistency, validate-wave-gate-completeness, validate-wave-gate-prerequisite | Highest complexity: VP-INDEX + 2 arch files (vp-consistency); wave-state.yaml YAML parse (wave-gate-completeness); wave-state.yaml + story metadata + subprocess-simplified (wave-gate-prerequisite). | read_file, emit_event, log | HIGH |

**Total: 7 stories, 23 hooks, capability-cluster grouping.**

This batching gives the story-writer and adversary a consistent "capability profile" per
story — every hook within a batch uses the same host functions, enabling uniform BC anchoring
patterns and efficient adversarial review focus on the cluster's specific risk (e.g., for
Batch 6 the adversary focuses exclusively on cross-document lookup correctness; for Batch 7,
on YAML parsing fidelity and the subprocess simplification decisions).

**Recommendation: (c) capability-cluster, 7 batched stories as described above.**

The story-per-behavioral-category option (b) would also produce ~7 stories but with less
uniform scope per story. The 9-hook "frontmatter-related" group would be too large for a
single story while the 1-hook "wave-related" and "gate-related" groups would be too small.
Capability clustering produces more balanced scope (3–4 hooks per story in Batches 1–5;
2–3 hooks in Batches 6–7).

---

## Section 5 — Risks & Open Questions

### R-W16-001: Legacy-bash-adapter retirement migration sequence

After W-16 delivers all 23 WASM ports, the migration sequence to reach Phase H
(v1.3.0 adapter deletion) proceeds as:

1. **W-16 COMPLETE (all 23 ported):** hooks-registry.toml gains 23 new WASM entries;
   the 23 legacy-bash-adapter entries for validate-*.sh remain co-registered but should
   be disabled (set `enabled = false` or removed). The `.sh` files remain on disk.
2. **W-17 Tier 3 (11 specialty hooks):** After W-17, legacy-bash-adapter has zero
   active registry entries. At this point `legacy-bash-adapter.wasm` is a zombie — it
   is built and bundled but dispatches no hooks.
3. **Phase H (v1.3.0):** Delete `legacy-bash-adapter.wasm` crate, delete all `.sh` bash
   hook files from `plugins/vsdd-factory/hooks/`, delete `hooks-registry.toml` entries
   with `script_path`, update SS-04 architecture doc to remove legacy-bash-adapter module,
   delete bats test files that invoke `.sh` files directly (per TD-020 orphan pattern).

**Risk:** The bats test files (`policy-enforcement.bats`, `wave-gate-hooks.bats`,
`hooks.bats`, `structural-validators-emission.bats`, `policy-validators-emission.bats`,
`workflow-validators-emission.bats`) test the `.sh` bash hooks directly via subprocess.
After Phase H deletes the `.sh` files, these bats tests become orphans (the W-15 TD-017
problem, now for Tier 2). **Each W-16 story spec must include a task to create a bats
deletion checklist** (mirroring the W-15 bats-orphan-detection CI gate established in
Phase A D-213).

**Mitigation sequence for each W-16 story:**
- Task A: Port bash hook(s) to Rust WASM crate(s).
- Task B: Update hooks-registry.toml to add new WASM entries + disable (or remove)
  legacy-bash-adapter entries.
- Task C: Update run-all.sh SKIP_SUITES or delete/update the corresponding bats test
  files for the ported hooks. File a TD if bats deletion is deferred.
- CI gate: bats-orphan-detection (TD-017) runs on every PR and blocks if `.sh`
  references in bats files have no corresponding `.sh` on disk.

### R-W16-002: WASI preopens and read_file path coverage

The Section 1 audit shows that 19 of 23 hooks read `FILE_PATH` from the hook payload,
where `FILE_PATH` is the path of the file the agent just wrote. This is a Claude Code
PostToolUse field (`tool_input.file_path`). The paths are `.factory/**/*.md`,
`stories/S-*.md`, `docs/demo-evidence/**`, etc. — all under `CLAUDE_PROJECT_DIR`.

Per BC-2.02.011 (`host::read_file`) and HOST_ABI.md, WASI preopens give plugins
read access to the `CLAUDE_PROJECT_DIR` subtree. This means `host::read_file` with
`path_allow = [".factory/"]` or `path_allow = ["."]` (project root) covers all
`FILE_PATH` values these hooks encounter.

**Risk:** `validate-anchor-capabilities-union`, `validate-subsystem-names`, and
`validate-vp-consistency` read ADDITIONAL files beyond `FILE_PATH` — BC shards,
ARCH-INDEX.md, VP-INDEX.md. These additional reads require explicit `path_allow`
entries in the capability declaration:

```toml
[hooks.capabilities.read_file]
path_allow = [
  ".factory/",       # story files, STATE.md, BC shards, VP files, architecture docs
]
```

The `.factory/` path allow covers all these since BC shards, VP files, and arch docs
all live under `.factory/specs/`. **Recommendation:** Use `path_allow = [".factory/"]`
as the canonical read_file capability for all W-16 WASM hooks that read spec files.
For hooks that only read `FILE_PATH` (which could be outside `.factory/`), use
`path_allow = ["."]` to cover the project root.

This should be pinned per-hook in each story spec's capability declaration table
(following the S-8.04/S-8.10 `path_allow` pattern from BC-7.03.083 and S-8.09).

### R-W16-003: Bundle size delta

Current W-15 WASM bundle: 19 files built by `release.yml` (16 native + hello-hook + example
stubs). Observed sizes from `plugins/vsdd-factory/hook-plugins/`:

- Minimum (lifecycle telemetry, emit_event only): ~116 KB (session-learning.wasm)
- Typical (read_file + serde_json + regex): ~163–320 KB (handoff-validator, warn-pending-wave-gate)
- Maximum (serde_yaml + complex logic): ~1.2 MB (track-agent-start, validate-pr-review-posted)

For W-16, the 23 validate hooks are medium complexity: single read_file call, regex matching,
no serde_yaml. Estimated size: 160–350 KB per plugin (similar to handoff-validator and
warn-pending-wave-gate).

**Bundle delta estimate:** 23 plugins × 200–300 KB average = **4.6–6.9 MB** added to the
dispatcher bundle. The release bundle currently ships 5-platform binaries + 19 WASM plugins.
The `.wasm` files are embedded in the binary at build time, so the binary size increases
proportionally.

Cross-check against E-8 R-8.09 (bundle growth ceiling): The S-8.00 perf baseline measured
`all_hook_plugins_wasm_bytes` at the W-15 baseline. R-8.09 set a 25% growth ceiling.
W-16 adds ~23 plugins; the current ~19 WASM files total approximately:
116+155+156+163+171+173+206+216+228+320+1205+1205+1213+1216+1455 = ~7.2 MB.
A 25% growth ceiling would cap at ~9.0 MB. Adding 4.6–6.9 MB would reach 11.8–14.1 MB
— **potentially exceeding the R-8.09 ceiling**. This should be flagged in the E-9 epic
spec with an open question (OQ) and resolved at S-9.00 perf baseline measurement
(analogous to S-8.00 for W-15).

**Recommendation:** The W-16 epic spec (E-9) should include an S-9.00 perf baseline story
that measures the bundle size before first W-16 story ships, similar to S-8.00 for W-15.
The `bundle_size_bytes` growth from W-15 to W-16 should be tracked against an E-9-specific
growth ceiling. The E-9 epic should propose raising the R-8.09 ceiling or deprecating it in
favor of a per-wave growth measurement.

### R-W16-004: Test infrastructure — bats tests after WASM port (TD-020 class problem)

The bats test files that cover validate hooks currently test the `.sh` files directly via
subprocess invocation. After W-16 ports these hooks to WASM, the dispatcher routes execution
to the `.wasm` plugin, not the `.sh` file. The bats tests would continue to pass (because
the `.sh` file still exists on disk until Phase H) but would NOT exercise the WASM execution
path.

This is structurally identical to the TD-020 orphan-test problem identified during W-15
(the S-8.09 test infrastructure finding from the wave gate review). The W-15 approach was
to:
1. Keep the `.sh` file on disk (so bats tests pass and CI stays green).
2. Mark the bats tests as testing "bash reference behavior" (expected to match WASM).
3. Create WASM-level integration tests in the Rust test suite (similar to `factory-dispatcher/tests/integration_tests/`).
4. Delete the `.sh` files in Phase H, at which point the bats tests become orphans and
   must be either deleted or rewritten to invoke the WASM dispatch path.

**For W-16:** Each story spec should include:
- A Task to confirm the bats test file(s) for ported hooks still pass (regression check).
- A Task to create WASM integration tests in `factory-dispatcher/tests/` that exercise the
  WASM execution path end-to-end.
- A note in the story's "Test Infrastructure" section citing this TD-020 class problem and
  deferring bats test migration to Phase H.
- A TD ticket per-hook for bats test migration, similar to TD-020.

This preserves CI green throughout W-16 while ensuring WASM behavior has coverage
at the Rust integration test level.

---

## Section 6 — Recommendation Summary (TLDR)

| Decision | Recommendation | One-line rationale |
|----------|---------------|-------------------|
| D-9.1 | **(a) rewrite-clean** | 20/23 hooks are trivial regex+file-read pipelines; jq/awk replacement is cleaner in Rust than 1:1; OQ-001 class quirks are caught by adversarial spec review before implementation |
| D-9.2 | **(c) skip new host fns — all 23 hooks port in W-16** | Both apparent subprocess calls are optional/graceful-skip paths that can be dropped with deliberate simplification; no new ABI needed; zero security surface expansion |
| D-9.3 | **(c) 7 capability-cluster stories** | Balanced scope (3–4 hooks per story); uniform host-fn profile per batch enables focused adversarial review; 7 stories matches pre-audit estimate |

**Next steps if approved:**

- (a) ADR-014 authoring (architect): document rewrite-clean decision, no-new-host-fn constraint,
  and subprocess-simplification rationale for validate-state-size and validate-wave-gate-prerequisite
- (b) E-9 epic authoring (product-owner): similar to E-8, with S-9.00 perf baseline story +
  7 batched stories S-9.01..S-9.07 + bundle-size OQ
- (c) BC authoring (product-owner): no new host function BCs needed; existing BC-2.02.002
  (read_file) and BC-2.02.004 (emit_event) cover all W-16 plugins; BC-7 sub-family backfill
  for 23 new hooks (mirroring BC-7.03.* pattern from W-15)
- (d) Adversarial convergence per ADR-013: 7 stories × ~6–10 passes each = ~42–70 passes
  (parallelizable within each batch; sequential across batches if dependency ordering required)
- (e) Per-story-delivery cycles per W-15 pattern: test-writer RED gate + implementer GREEN +
  demo-recorder + pr-manager 9-step

**Estimated spec-foundation effort if recommendations approved:**

| Activity | Agent | Estimated passes/cycles | Duration estimate |
|----------|-------|------------------------|-------------------|
| ADR-014 authoring + convergence | architect + adversary | ~4–6 passes | 0.5 days |
| E-9 epic authoring + convergence | product-owner + adversary | ~6–8 passes | 1 day |
| S-9.00 perf baseline + convergence | story-writer + adversary | ~5–6 passes | 0.5 days |
| S-9.01..S-9.07 authoring (7 stories) | story-writer | 7 authoring bursts | 1 day |
| S-9.01..S-9.07 adversarial convergence | adversary × 7 | ~7–10 passes each | 2–3 days |
| BC-7 backfill (23 hooks × ~3 BCs each) | product-owner | 1 authoring burst | 0.5 days |
| **Total Phase D spec foundation** | | | **~5–7 days** |
