---
document_type: behavioral-contract
level: L3
version: "1.1"
status: active
producer: product-owner
timestamp: 2026-06-24T00:00:00Z
last_amended: "2026-06-24 (v1.1) — D-696 post-merge burst (state-manager): POL-14 auto-promotion draft→active. S-18.04b-prereq PR #262 squash-merged a177d76e to develop 2026-06-25T00:29:56Z. lifecycle_status draft→active; status draft→active. No behavioral change. BC-1.16.001 v1.1. [Prior: 2026-06-24 (v1.0) — S-18.04b-prereq BC authoring burst (product-owner): Initial creation. Dispatcher git_context payload injection on PostToolUse Bash git-commit events for WASM chain-detection gates (ADR-029). Establishes host-side git execution contract, git_context schema, fail-open posture, and exec-free WASM boundary invariant. BC-1.16.001 v1.0.]"
phase: F2
inputs:
  - .factory/specs/architecture/decisions/ADR-029-dispatcher-git-context-payload-injection.md
  - .factory/feature-delta/issue-173/F1-delta-analysis.md
input-hash: "TBD"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-032"
lifecycle_status: active
introduced: v1.0-feature-context-durability-E18
modified:
  - "v1.1 (2026-06-24): D-696 POL-14 auto-promotion draft→active (S-18.04b-prereq PR #262 merged a177d76e)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.16.001: Dispatcher injects git_context (HEAD/HEAD^ subject+SHA) into payload.extra on PostToolUse Bash git-commit events targeting factory-artifacts worktree — exec-free WASM boundary; fail-open on git error

## Description

The factory-dispatcher binary must inspect PostToolUse Bash events and, when the Bash command is a `git commit` targeting the factory-artifacts worktree, execute git host-side to obtain HEAD and HEAD^ commit subjects and SHAs, then inject a `git_context` JSON object into `payload.extra` before routing the payload to registered WASM chain-detection plugins (`validate-burst-log`, `validate-dispatch-advance`). This design keeps WASM plugins exec-free (satisfying BC-5.41.003 PC1 and ADR-029 §Decision 3) while correcting the structural defect where chain detection previously fired on Edit/Write events against a stale HEAD rather than on the actual commit event (ADR-029 §Decision 1). Fail-open behavior (all-empty `git_context` fields) on any git error preserves pipeline availability at the cost of skipping chain detection for the affected event.

## Preconditions

1. The PostToolUse event has `event = "PostToolUse"` and `tool = "Bash"`.
2. The `tool_input.command` string contains a `git commit` invocation targeting the factory-artifacts worktree (detected by the presence of `-C .factory`, `-C <CLAUDE_PROJECT_DIR>/.factory`, or a `--work-tree`/`-C` flag pointing to the `.factory` directory).
3. The dispatcher binary is `factory-dispatcher` (source: `crates/factory-dispatcher/src/`), operating as the host layer with `exec_subprocess` capability.
4. The factory-artifacts worktree is accessible at `<CLAUDE_PROJECT_DIR>/.factory` (same path convention as ADR-028 §Decision canonical invariant).

## Postconditions

1. **git_context injection on qualifying events**: For any PostToolUse Bash event satisfying the preconditions, the dispatcher MUST inject a `git_context` JSON object into `payload.extra` with the following four fields before routing to registered plugins:
   - `head_subject`: result of `git -C <factory_dir> log --format=%s -1 HEAD` (string; the commit message subject line of the most recent commit).
   - `head_sha`: result of `git -C <factory_dir> rev-parse HEAD` (string; full 40-character hex SHA of the most recent commit).
   - `head_parent_subject`: result of `git -C <factory_dir> log --format=%s -1 HEAD^` (string; subject of the parent commit; empty string if HEAD^ does not exist, i.e., initial commit).
   - `head_parent_sha`: result of `git -C <factory_dir> rev-parse HEAD^` (string; full 40-character hex SHA of the parent commit; empty string if HEAD^ does not exist).

2. **Fail-open on git error**: If any git command in postcondition 1 fails (git not found, not a git repo, permission denied, network error, or any non-zero exit code), the dispatcher MUST log a warning at the `warn` tracing level and inject `git_context` with all four fields set to the empty string `""`. The enriched payload is then routed to registered plugins as normal. The dispatcher MUST NOT block, abort, or fail-close the hook dispatch on a git error.

3. **No injection on non-qualifying events**: For PostToolUse Bash events whose command does NOT contain a `git commit` targeting the factory-artifacts worktree, the dispatcher MUST NOT inject `git_context` into `payload.extra`. Non-qualifying Bash events are routed without `git_context`.

4. **No injection on non-Bash PostToolUse events**: PostToolUse events for `tool = "Edit"`, `tool = "Write"`, `tool = "Agent"`, or any other non-Bash tool are not inspected for `git commit` content. No `git_context` injection occurs for these events.

5. **HOST_ABI_VERSION unchanged**: `git_context` rides in the existing `extra` map field of `HookPayload` (the `#[serde(flatten)]` field in `crates/factory-dispatcher/src/payload.rs`). No new named field is added to `HookPayload`. No new host function is added to the SDK's `host` module. `HOST_ABI_VERSION` remains 1. Existing plugins that do not read `git_context` from `extra` are unaffected (ADR-029 §Decision 4).

6. **payload.extra schema for git_context**:
   ```json
   {
     "git_context": {
       "head_subject":        "<string>",
       "head_sha":            "<string — 40-char hex or empty>",
       "head_parent_subject": "<string or empty>",
       "head_parent_sha":     "<string — 40-char hex or empty>"
     }
   }
   ```
   All four fields are always present when `git_context` is injected (no field may be omitted, even if empty). WASM plugins access via `payload.extra.get("git_context")`.

## Invariants

1. **Exec-free WASM boundary (ADR-029 §Decision 3)**: The `git_context` injection is EXCLUSIVELY the responsibility of the dispatcher host layer. No WASM plugin (`validate-burst-log`, `validate-dispatch-advance`, or any other) may call `host::exec_subprocess` to obtain commit subjects or SHAs. This invariant is enforced by the host capability model: WASM plugins that lack the `exec_subprocess` capability declaration cannot invoke it. The dispatcher's host-layer execution is the only legitimate path.

2. **Trigger correctness — commit event, not file-write event**: The chain-detection evaluation MUST occur on the PostToolUse Bash git-commit event, not on PostToolUse Edit/Write events. The dispatcher MUST NOT inject `git_context` on Edit/Write PostToolUse events. This invariant corrects the decoupling defect described in ADR-029 §Context: the prior implementation invoked git on file-write events against a stale HEAD, not against the commit being made. (ADR-029 §Decision 1.)

3. **Fail-open preserves pipeline availability**: A git-context-acquisition failure (any non-zero git exit code, network error, missing git binary) MUST result in fail-open (all-empty `git_context` fields injected, dispatch proceeds). The dispatcher MUST NOT treat git context acquisition as a blocking operation. WASM plugins receiving all-empty `git_context` treat it as "no chain context — skip check" (consistent with BC-5.41.003 Invariant 5 and ADR-029 §Decision 3).

4. **Heuristic command detection is acceptable**: The dispatcher detects `git commit` by inspecting the `tool_input.command` string for the presence of "git commit" as a substring along with a factory-artifacts worktree indicator (`-C .factory`, `-C <CLAUDE_PROJECT_DIR>/.factory`, or equivalent path reference). A Bash command that contains "git commit" as a substring but is not a real commit (e.g., `echo "git commit"`) may trigger injection; this is acceptable because the resulting `git_context` values are authoritative (obtained from the actual git HEAD of the factory-artifacts worktree post-execution) and the WASM plugin's logic is robust to valid-but-irrelevant git_context (it evaluates subjects against known sentinel words). False injection of valid git_context has no security consequence (fail-open posture).

5. **Four-field completeness**: When `git_context` IS injected (including on fail-open), all four fields (`head_subject`, `head_sha`, `head_parent_subject`, `head_parent_sha`) MUST be present as string-typed JSON properties. Absent fields or type mismatches (e.g., null instead of empty string) violate the schema and may cause WASM deserialization errors. On initial commit (no HEAD^), `head_parent_subject` and `head_parent_sha` are set to `""` (not null, not omitted).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | PostToolUse Bash event; command is `git -C .factory commit -m "state: burst-24 Commit A"` | Dispatcher injects `git_context` with HEAD subject `state: burst-24 Commit A`, HEAD sha (40-char hex), HEAD^ subject (prior commit), HEAD^ sha; routes enriched payload to registered plugins. |
| EC-002 | PostToolUse Bash event; command is `git -C .factory commit -m "PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-24T00:00:00Z"` | Dispatcher injects git_context; head_subject = `PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-24T00:00:00Z`; WASM plugin (BC-5.41.003 PC1) reads git_context and applies exemption logic. No exec in WASM. |
| EC-003 | Initial commit — factory-artifacts has no HEAD^ (first commit ever) | Dispatcher injects git_context with head_subject/head_sha populated; head_parent_subject = ""; head_parent_sha = ""; WASM plugin reads empty head_parent_subject and treats as "no chain possible — skip check" per BC-5.41.003 Invariant 5. |
| EC-004 | PostToolUse Bash event; command is `git -C .factory push origin factory-artifacts` (push, not commit) | Dispatcher does NOT inject git_context (command contains git but not "git commit"). Payload routed without git_context. |
| EC-005 | PostToolUse Edit event; file is `.factory/STATE.md` | Dispatcher does NOT inject git_context (event tool is Edit, not Bash). No command inspection performed. Payload routed without git_context. |
| EC-006 | PostToolUse Bash event; command is `git -C .factory commit -m "..."` but factory-artifacts git repo is unreadable (permissions error) | Dispatcher fails git commands, logs warn, injects git_context with all four fields = ""; routes enriched payload. WASM plugin receives all-empty git_context and skips chain check (fail-open). |
| EC-007 | PostToolUse Bash event; command contains substring "git commit" but is not actually a factory-artifacts commit (e.g., targets develop branch) | Dispatcher may inject git_context (heuristic detection); HEAD/HEAD^ reflect factory-artifacts current state; WASM plugin evaluates based on valid git_context; no harm (no false MULTI_COMMIT_CHAIN_NOT_ALLOWED fires unless factory-artifacts actually has a chain). |
| EC-008 | PostToolUse Bash event; command is `echo "git commit is idempotent"` | Dispatcher detects "git commit" substring but no factory-artifacts worktree indicator; no injection. Alternatively, if the heuristic triggers, git_context populated from actual factory-artifacts HEAD; still no harm. Implementation SHOULD scope the heuristic to commands that also reference `.factory`. |
| EC-009 | PostToolUse Bash event; `git commit` succeeds but `git rev-parse HEAD^` exits non-zero (initial commit) | head_parent_subject = ""; head_parent_sha = "" (fail-open for HEAD^ specifically; HEAD fields are populated normally). |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| PostToolUse `tool='Bash'`, `command='git -C .factory commit -m "stage 1 backfill"'`; factory-artifacts HEAD^=`stage 2 backfill`; HEAD=`stage 1 backfill` | `git_context` injected: `head_subject="stage 1 backfill"`, `head_sha="<40-char-hex>"`, `head_parent_subject="stage 2 backfill"`, `head_parent_sha="<40-char-hex>"`; WASM receives full git_context | happy-path-chain-detection |
| PostToolUse `tool='Bash'`, `command='git -C .factory commit -m "PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-24T00:00:00Z"'`; HEAD^=`state: burst-23 Commit D` | `git_context` injected; validate-burst-log reads git_context and applies exemption (BC-5.41.003 PC1); no MULTI_COMMIT_CHAIN_NOT_ALLOWED | precompact-exempt |
| PostToolUse `tool='Edit'`, `path='.factory/STATE.md'` | No git_context injection; payload routed without extra fields | non-bash-no-op |
| PostToolUse `tool='Bash'`, `command='git -C .factory commit -m "..."'`; git exits non-zero (repo error) | `git_context` injected with all four fields = `""`; dispatch proceeds; WASM skips chain check | fail-open |
| PostToolUse `tool='Bash'`, `command='git -C .factory commit -m "initial"'`; HEAD^ does not exist (initial commit) | `git_context`: head_subject=`initial`, head_sha=`<40-char-hex>`, head_parent_subject=`""`, head_parent_sha=`""`; WASM receives empty parent fields and skips chain check | initial-commit-edge |

## Related BCs

- BC-5.41.003 — composes with: this BC provides the host-side git_context injection that BC-5.41.003 PC1 addendum requires for exec-free WASM operation; BC-5.41.003 specifies the WASM-side consumption contract
- BC-1.15.001 — sibling: dispatcher routes PreCompact/PostCompact events to plugins; this BC adds a parallel enrichment path for PostToolUse Bash git-commit events

## Architecture Anchors

- `crates/factory-dispatcher/src/invoke.rs` — primary implementation site; PostToolUse event dispatch loop; `git_context` injection logic added here per ADR-029 §Decision 3
- `crates/factory-dispatcher/src/payload.rs` — `HookPayload` struct; `extra` field (`HashMap<String, serde_json::Value>`) receives `git_context`; no new named field added; HOST_ABI_VERSION unchanged
- `crates/factory-dispatcher/src/host/exec_subprocess.rs` — existing host-layer exec capability; used by dispatcher to run git commands for git_context acquisition; WASM plugins have no access to this
- `plugins/vsdd-factory/hooks-registry.toml` — chain-detection hook entries for `validate-burst-log` and `validate-dispatch-advance` must declare `event = "PostToolUse"` with `tool = "Bash"` filter (updated from Edit/Write trigger); see ADR-029 §Decision 7
- ADR-029 §Decision 1 — trigger model: PostToolUse Bash (git commit), not PostToolUse Edit/Write
- ADR-029 §Decision 2 — git_context schema specification
- ADR-029 §Decision 3 — dispatcher host-side git execution; WASM exec-free boundary
- ADR-029 §Decision 4 — HOST_ABI_VERSION remains 1; payload.extra used; no new host function

## Story Anchor

S-18.04b-prereq (dispatcher git_context injection; prerequisite for S-18.04b WASM exec-removal)

## VP Anchors

- VP-093 (to be authored) — Dispatcher Injects git_context Into payload.extra on PostToolUse Bash git-commit Events; Fail-Open on Git Error

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-093 | On a qualifying PostToolUse Bash event (command contains `git commit` targeting factory-artifacts), dispatcher injects `git_context` into payload.extra with all four fields populated (head_subject, head_sha, head_parent_subject, head_parent_sha). On git error, all four fields are empty strings and dispatch proceeds (fail-open). On non-qualifying Bash events or non-Bash events, no injection occurs. | integration |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC is a prerequisite enabler of CAP-032 Part B (mid-wave compaction losslessness via precompact-flush.sh). The git_context injection corrects the structural defect where chain detection fired on file-write events against a stale HEAD (ADR-029 §Context decoupling defect). Without this dispatcher-side fix, the WASM chain-detection gate cannot be made exec-free (BC-5.41.003 PC1 requirement), and S-18.04b cannot be safely implemented. This BC implements the HOST-SIDE half of the ADR-029 §Decision 3 host/WASM split. |
| L2 Domain Invariants | DI-020 (Wave/phase boundary transitions must not lose load-bearing pipeline state — the git_context injection ensures chain detection evaluates the actual commit being made, not a stale HEAD, preserving correctness of TD-VSDD-053 enforcement across wave transitions); DI-025 (PreCompact flush commits are lifecycle-orthogonal to state-manager burst commits — the correct trigger model (PostToolUse Bash git-commit) directly enables detection of commit sequences rather than file-write sequences, preserving the orthogonality invariant by ensuring chain detection context is commit-scoped) |
| Architecture Module | SS-01 (Hook Dispatcher Core) — git_context injection is dispatcher host-layer behavior in `crates/factory-dispatcher/src/invoke.rs`; this is dispatcher core functionality, not plugin behavior |
| ADR | ADR-029 §Decision 1 (trigger: PostToolUse Bash git-commit, not Edit/Write); ADR-029 §Decision 2 (git_context schema); ADR-029 §Decision 3 (dispatcher host-side git execution; WASM exec-free; git_context in payload.extra); ADR-029 §Decision 4 (HOST_ABI_VERSION stays 1) |
| Stories | S-18.04b-prereq |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.0 | 2026-06-24 | product-owner | Initial creation (S-18.04b-prereq BC authoring burst; ADR-029 dispatcher git_context injection contract; exec-free WASM boundary; fail-open posture). |
