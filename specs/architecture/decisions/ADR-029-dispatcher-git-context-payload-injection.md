---
document_type: architecture-decision-record
level: L3
adr_id: ADR-029
version: "1.3"
title: "ADR-029: Dispatcher git-context payload injection for WASM chain-detection gates"
status: proposed
producer: architect
timestamp: 2026-06-24T00:00:00Z
deciders:
  - architect
  - human (Option A approval)
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
related_adrs:
  - ADR-028 (E-18 precompact-flush native WASM migration — established WASM-native hook pattern)
  - ADR-026 (wave-boundary checkpoint — defines PreCompact flush lifecycle and precompact-flush-log append-log)
  - ADR-002 (WASM plugin ABI — defines host function contract)
  - ADR-006 (host ABI version — governs HOST_ABI_VERSION bump policy)
anchors:
  - SS-01
  - SS-04
subsystems_affected:
  - SS-01
  - SS-04
last_amended: "2026-06-25 (v1.3) — Decision 5 annotation: `check_factory_artifacts_chain()` is the pre-rewire name; the S-18.04b implementation renamed this function to `check_chain_from_git_context()` (O-P4-001 / TD-VSDD-091). [Prior: 2026-06-24 (v1.2) — Decision 8 added (architect adjudication of S-18.04b LOCAL adversarial cascade finding): pure-Rust unit tests in exemption.rs are the load-bearing proof vehicle for is_precompact_flush_exempt / check_multi_commit_chain; VP-084 bats tests are correctly scoped to dispatcher injection plumbing + MULTI_COMMIT_CHAIN detection end-to-end (not exemption-flip); BC-5.41.003 PC4 wording and VP-084 Feasibility Assessment require targeted amendment to reflect the corrected scoping. Exemption is NOT dead code and is NOT a no-op: the real-world topology that motivates it is confirmed valid; the topology is exercised by real git repo setup in vp084-proof.bats (F-P1-001 fix); the proof is load-bearing. No production code change is required. Option A selected. [Prior: 2026-06-24 (v1.1) — O-1 subsystem anchor correction: frontmatter subsystem field corrected from SS-04 to SS-01+SS-04 (per ARCH-INDEX Subsystem Registry POLICY 6: SS-01=Hook Dispatcher Core owns crates/factory-dispatcher/src/invoke.rs; SS-04=Plugin Ecosystem owns the WASM plugin consumer side); §ARCH-INDEX subsystem prose corrected from erroneous SS-03 (Dispatcher Core) to SS-01 (Hook Dispatcher Core) + SS-04 (Plugin Ecosystem).].]"
---

# ADR-029: Dispatcher git-context payload injection for WASM chain-detection gates

## Status

Proposed — pending S-18.04b implementation.

## Context

`validate-burst-log` and `validate-dispatch-advance` (WASM hook plugins under
`crates/hook-plugins/`) implement the `MULTI_COMMIT_CHAIN_NOT_ALLOWED` detector
(TD-VSDD-053) and the PreCompact flush exemption (BC-5.41.003). Both plugins
currently call `host::exec_subprocess("git", ...)` inside `check_factory_artifacts_chain()`
to obtain HEAD and HEAD^ commit subjects and SHAs from the factory-artifacts worktree.

This is a correctness defect with two dimensions:

**Decoupling defect.** The chain check fires on PostToolUse Edit/Write events (file
writes) rather than on Bash git-commit events. A file write that is unrelated to a
commit will cause the WASM to exec git, read the CURRENT factory-artifacts HEAD (which
was committed earlier in the session, not "being committed now"), and evaluate that
stale HEAD/HEAD^ pair. The gate conflates "file was written" with "a commit just
occurred". This is structurally incorrect: the gate must evaluate the commit being made,
not an arbitrary prior HEAD.

**PC1 violation.** BC-5.41.003 PC1 specifies that "the WASM gate does NOT exec
`git cat-file -t`". By extension, BC-5.41.003 Architecture Anchors state "WASM does
NOT exec git; no bash equivalent exists". The current `check_factory_artifacts_chain()`
implementation uses `exec_subprocess("git", ...)` to obtain commit subjects and SHAs,
violating PC1. ADR-028 establishes WASM-native migration as the architectural direction
(E-18 alignment); exec-git-in-WASM contradicts this.

**Human decision.** The human approved Option A: keep chain-detection WASM-native
(consistent with ADR-028). The host dispatcher (allowed to exec git) injects commit
context into the hook payload as a `git_context` field. The WASM plugin reads
`git_context` from the payload instead of exeving git.

## Decision

### Decision 1: Trigger on PostToolUse Bash (git commit), not PostToolUse Edit/Write

The chain-detection gate MUST run on the PostToolUse event for a Bash `git commit` tool
call on the factory-artifacts worktree, NOT unconditionally on every Edit/Write
PostToolUse event.

Rationale: the chain detector evaluates the commit being made. A file write unrelated to
a commit must not trigger git inspection. The existing Edit/Write hooks
(`validate-burst-log`, `validate-dispatch-advance`) validate file content structure;
that duty remains. The chain check is separate: it belongs on the commit event.

Implementation: the chain-detection WASM gate runs as a new PostToolUse hook on
`tool = "Bash"`, filtered by command content. The dispatcher detects a `git commit`
command in the Bash `tool_input.command` field during the PostToolUse envelope
enrichment phase (after the command has executed successfully), then injects
`git_context` into the payload before routing to chain-detection plugins. The existing
`validate-burst-log` and `validate-dispatch-advance` hooks on `Edit|Write` MUST have
their `check_factory_artifacts_chain()` call removed entirely — that call is the
exec-git violation. The chain check moves to a new trigger.

### Decision 2: `git_context` payload schema

The dispatcher injects a top-level `git_context` JSON object into the hook payload for
qualifying PostToolUse Bash git-commit events. The schema is:

```json
{
  "git_context": {
    "head_subject":        "<string — git log --format=%s -1 HEAD of factory-artifacts worktree>",
    "head_sha":            "<string — git rev-parse HEAD of factory-artifacts worktree, full 40-char hex>",
    "head_parent_subject": "<string — git log --format=%s -1 HEAD^ of factory-artifacts worktree>",
    "head_parent_sha":     "<string — git rev-parse HEAD^ of factory-artifacts worktree, full 40-char hex>"
  }
}
```

Field types: all four are `String`. Empty string indicates the field could not be
populated (e.g., HEAD^ does not exist on initial commit). The WASM plugin treats an
empty `head_parent_subject` as "no chain possible — skip check" (fail-open, consistent
with BC-5.41.003 invariant 5).

Payload delivery: `git_context` is inserted into the `extra` map of `HookPayload`
(the `#[serde(flatten)]` field in `crates/factory-dispatcher/src/payload.rs`). No
new named field is added to `HookPayload` itself — `git_context` rides in `extra`,
which WASM plugins access via `payload.extra.get("git_context")`. This avoids a
`payload.rs` schema change and keeps HOST_ABI_VERSION at 1 (see Decision 4).

### Decision 3: Dispatcher responsibility — host-side git execution

When the dispatcher processes a PostToolUse event for `tool = "Bash"` and the
`tool_input.command` string contains a `git commit` invocation targeting the
factory-artifacts worktree (detected by the presence of `-C <factory_dir>` or
by command containing `.factory` directory reference), the dispatcher MUST:

1. Determine the factory-artifacts worktree path: `<CLAUDE_PROJECT_DIR>/.factory`
   (same convention as ADR-028 §Decision canonical invariant).
2. Execute `git -C <factory_dir> log --format=%s -1 HEAD` to obtain `head_subject`.
3. Execute `git -C <factory_dir> rev-parse HEAD` to obtain `head_sha`.
4. Execute `git -C <factory_dir> log --format=%s -1 HEAD^` to obtain
   `head_parent_subject`. If HEAD^ does not exist (exit non-zero), set
   `head_parent_subject = ""` and `head_parent_sha = ""`.
5. Execute `git -C <factory_dir> rev-parse HEAD^` to obtain `head_parent_sha` (same
   conditional as step 4).
6. Inject the populated `git_context` object into the hook payload's `extra` map.
7. Route the enriched payload to registered PostToolUse Bash plugins.

The dispatcher already has `exec_subprocess` capability at the host layer
(`crates/factory-dispatcher/src/host/exec_subprocess.rs`). No new host capability
is required. The WASM plugins are exec-free (PC1 preserved).

Fail-open: if any git command fails (git not found, not a git repo, network error),
the dispatcher logs a warning and injects `git_context` with all fields set to `""`.
The WASM plugin treats all-empty as "no chain context — skip check" (same fail-open
as the current implementation, but without the exec-git-in-WASM violation).

### Decision 4: HOST_ABI_VERSION remains 1

`git_context` is injected into the `extra` field of `HookPayload` (the flatten map),
not as a new named host function. No new host function is added to the SDK's `host`
module. Therefore HOST_ABI_VERSION does not change. Existing plugins that do not read
`git_context` from `extra` are unaffected.

### Decision 5: WASM gate reads `git_context` from payload.extra, not from exec_subprocess

The WASM plugin's `check_factory_artifacts_chain()` function (pre-rewire name; S-18.04b implementation renamed this to `check_chain_from_git_context()`) is redesigned to:

1. Read `git_context` from `payload.extra.get("git_context")`.
2. Extract `head_subject`, `head_sha`, `head_parent_subject`, `head_parent_sha` as strings.
3. If `git_context` is absent or all fields are empty: return `None` (fail-open).
4. Read precompact-flush-log via `host::read_file` (unchanged — this is correct).
5. Call `check_multi_commit_chain(head_subject, head_sha, head_parent_subject, head_parent_sha, flush_log_last_line)`.

The function signature of `check_multi_commit_chain` and `is_precompact_flush_exempt`
is unchanged. The pure logic is correct. Only the I/O wiring changes: exec_subprocess
calls are removed; payload.extra read replaces them.

### Decision 6: vp084-proof.bats schema alignment

The negative-control test in `vp084-proof.bats` (line 298) already passes
`git_context` as a top-level envelope field with `head_subject` and `head_parent_subject`:

```bash
envelope=$(printf '{"event":"PostToolUse","tool":"Edit",...,"git_context":{"head_subject":"stage 1 backfill","head_parent_subject":"stage 2 backfill"}}' ...)
```

This schema is PARTIALLY aligned with ADR-029. The gap: the bats test uses
`tool = "Edit"` (not `tool = "Bash"`), and the schema omits `head_sha` and
`head_parent_sha`. The corrected production schema for bats is:

```bash
envelope=$(printf '{"event":"PostToolUse","tool":"Bash","tool_input":{"command":"git -C .factory commit ..."},"git_context":{"head_subject":"...","head_sha":"<40-char-hex>","head_parent_subject":"...","head_parent_sha":"<40-char-hex>"}}' ...)
```

The bats test must be updated to match production trigger semantics (`tool = "Bash"`)
and include `head_sha` / `head_parent_sha`. The WASM plugin must accept the schema
with all four fields present; it MUST NOT require `head_sha`/`head_parent_sha` to be
non-empty for basic chain detection (they are only needed for the SHA-corroboration
path of `is_precompact_flush_exempt`).

### Decision 7: Prerequisite story for dispatcher git-context injection

The dispatcher-side `git_context` injection (decisions 1, 2, 3) requires changes to:
- `crates/factory-dispatcher/src/main.rs` (payload enrichment before routing)
- `crates/factory-dispatcher/src/invoke.rs` (git-context injection at PostToolUse Bash)
- `plugins/vsdd-factory/hooks-registry.toml` (new chain-detection hook entry or
  updated trigger for existing plugins)

This is a dispatcher release-level change (new binary behavior; requires release for
operator-level cache to pick up). It is analogous in scope to the `write_file.rs`
path-resolution fix that warranted S-18.04a-prereq. A SEPARATE prerequisite story
(tentatively S-18.04b-prereq) is warranted for the dispatcher injection side, so that:

- S-18.04b-prereq: dispatcher `git_context` injection on Bash PostToolUse git-commit
  events; no WASM changes in this story.
- S-18.04b: WASM plugins updated to read `git_context` from payload.extra; exec_subprocess
  calls removed; bats proof updated.

This preserves Red Gate discipline: S-18.04b's WASM changes fail (no `git_context` in
payload) until S-18.04b-prereq delivers the dispatcher injection.

### Decision 8: Proof-vehicle scoping for `is_precompact_flush_exempt` and VP-084 bats tests

**Context.** During the S-18.04b LOCAL adversarial cascade, a mutation-test finding
was raised: forcing `is_precompact_flush_exempt → return false` and rebuilding the WASM
leaves all three `vp084-proof.bats` tests GREEN. The test-writer's conclusion was that the
bats tests are structurally tautological for the exemption decision, and asked whether
the exemption is (a) correct-but-defensive, (b) evidence of a semantic gap, or (c) dead
code to be removed.

**Analysis and determination (architect).**

The mutation result is correct and mechanically sound. Examining the composition:

1. `is_precompact_flush_exempt` fires ONLY when a commit's subject matches
   `^PreCompact flush `. Real PreCompact flush subjects (from `COMMIT_PREFIX` in
   `crates/hook-plugins/precompact-flush/src/lib.rs`) contain no "backfill", "Stage 1",
   or "Stage 2" tokens — they are non-sentinel.

2. `contains_sentinel(s)` returns true only for those three patterns. A PreCompact subject
   is therefore never a sentinel by predicate.

3. `check_multi_commit_chain` fires the MULTI_COMMIT_CHAIN block only when BOTH
   `is_sentinel(head_subject)` AND `is_sentinel(head_parent_subject)` are true, AFTER
   checking the exemption on each. A PreCompact flush subject fails the sentinel predicate
   before the exemption is even evaluated. Therefore, for any real factory-artifacts
   topology involving a genuine PreCompact flush commit, the block would not fire even
   WITHOUT the exemption — the non-sentinel predicate short-circuits first.

4. The mutation (force-`return false`) cannot flip a bats test result because the bats
   tests, as written with real git repos (F-P1-001 fix in vp084-proof.bats), set up a
   genuine PreCompact HEAD commit. That commit's real subject is non-sentinel. The
   dispatcher injects that real non-sentinel subject as `head_subject`. The chain
   detector sees a non-sentinel HEAD and does not fire — the exemption is not the
   deciding factor. The positive tests are not tautological for the FULL correctness
   property; they are tautological ONLY for the exemption decision path specifically.

**Is the exemption dead code?** No. The exemption is not dead code in the specificationintent sense. The spec's stated purpose (BC-5.41.003 §Description) is defensively correct:
the exemption provides an explicit, documented, named bypass for PreCompact flush commits
so that if the sentinel predicate is ever broadened, or if the commit message format
produces an accidental sentinel match (e.g., a cycle name that happens to contain "Stage"),
the exemption remains a load-bearing firebreak. The exemption is a deliberate defense-in-depth
layer. Removing it (Option C) would create brittleness against future sentinel-set changes
and would contradict BC-5.41.003's explicitly stated invariant (INV4: "TD-VSDD-053
baseline is unchanged... the exemption adds a conditional skip").

**Is there a semantic gap (Option B)?** No. The current sentinel predicate (`backfill`,
`Stage 1`, `Stage 2`) is deliberately narrow. There is no spec intent for the chain
detector to also catch flush-on-sentinel topologies beyond what the current predicate
covers. Option B (restructure the guard) would change production semantics without a
corresponding spec requirement. Rejected.

**Decision: Option A — authorize pure-Rust unit tests as load-bearing proof vehicle;
rescope VP-084 bats test description.**

The resolution is:

1. **Pure-Rust unit tests in `exemption.rs` (Section 1) are the load-bearing proof
   vehicle** for `is_precompact_flush_exempt` and `check_multi_commit_chain`. These tests
   call the pure functions directly with synthetic inputs where breaking the exemption
   deterministically flips the result. They are already GREEN and will remain so. They
   are NOT the Red Gate (Section 2 wiring tests are the Red Gate for ADR-029); they are
   the CORRECT place to assert exemption-decision behavior at the pure-function level.
   This is the correct VSDD layering: pure-core logic is verified by pure-Rust unit tests;
   the effectful dispatcher-integration path is verified by the bats integration tests.

2. **VP-084 bats tests prove a different, also load-bearing property**: that the
   dispatcher injection is wired end-to-end such that (a) a real PreCompact HEAD commit
   in a real git repo causes the dispatcher to inject a non-sentinel `head_subject` into
   `git_context`, (b) the WASM gate reads that injected context, (c) the chain detector
   does NOT fire (correct real-world behavior — the non-sentinel subject + exemption logic
   combine to produce Continue), and (d) for the negative-control test, a real sentinel
   chain causes the dispatcher to inject sentinel subjects and the WASM gate fires
   MULTI_COMMIT_CHAIN_NOT_ALLOWED. The negative-control test IS load-bearing: it proves
   the full dispatcher→WASM chain-detection path works, which is what VP-084's Postcondition
   C asserts. If the entire chain-detection path were broken (e.g., WASM always returns
   Continue regardless of input), the negative control would fail.

3. **The mutation proof of non-tautology is in the negative-control test (Test 3 of
   vp084-proof.bats)**, not Tests 1 and 2. Test 3 (sentinel chain → block) fails if the
   exemption logic is "always Continue" — it requires the WASM to actually detect the
   sentinel chain and block. Tests 1 and 2 (PreCompact → Continue) are evidence that the
   correct topology does NOT block, which together with Test 3 proves the discrimination.
   The test suite as a whole is non-tautological at the suite level.

4. **No production code change** is required. The exemption logic in
   `is_precompact_flush_exempt` and `check_multi_commit_chain` is correct and should
   be preserved exactly as specified. The pure-Rust unit tests in `exemption.rs`
   (Section 1) already cover all 3 exemption cases.

**What must change:**

- BC-5.41.003 PC4 wording must be amended by product-owner to accurately describe what
  the bats tests actually prove (dispatcher injection + end-to-end chain detection, not
  the exemption-decision flip itself). The current PC4 wording says "verify that
  validate-burst-log and validate-dispatch-advance return `block_intent = false`" which
  is correct behavior-wise but does not distinguish which proof mechanism verifies the
  exemption decision vs. the injection plumbing. Proposed PC4 amendment: split the
  coverage statement to clarify that the bats tests cover the dispatcher integration
  path (AC-007 + injection plumbing) and the pure-Rust unit tests in `exemption.rs`
  cover the 3-case exemption decision logic.

- VP-084 §Feasibility Assessment must be amended by architect (in a follow-up to this
  ADR) to state explicitly: "The pure-Rust unit tests in `exemption.rs` (Section 1 of
  the exemption test file) are the load-bearing proof vehicle for `is_precompact_flush_exempt`
  and `check_multi_commit_chain`. The bats integration tests prove the dispatcher
  injection path and overall chain-detection plumbing end-to-end. Together, the two
  layers constitute a complete proof of VP-084's three postconditions: pure-function
  correctness (unit tests) + injection wiring correctness (bats integration)."

- The `vp084-proof.bats` test comments (already updated in the S-18.04b worktree) are
  correct: they describe the tests as proving the injection path + discrimination property,
  not the exemption function per se. No change to the bats test logic itself is needed;
  the F-P1-001 fix (real git repos) already makes the tests non-tautological at the suite
  level.

**Human sign-off requirement.** This decision does NOT require human sign-off before
proceeding. The determination is:
- No production semantic change (exemption logic is preserved, not altered).
- No BC behavioral invariant is weakened (the exemption still fires on real PreCompact
  flush commits in all real topologies).
- The VP-084 scope clarification is an accuracy amendment (the tests always covered what
  they cover; we are clarifying the description), not a coverage reduction.
- The only changes are: (a) this ADR Decision 8 codification; (b) a product-owner BC-5.41.003
  PC4 wording amendment; (c) a VP-084 Feasibility Assessment update.
  None of these alter merged-prerequisite semantics or materially change what VP-084 asserts.

## Consequences

**Positive:**
- WASM plugins are exec-free (PC1 fully satisfied).
- Chain detection evaluates the actual commit being made, not a stale HEAD.
- `git_context` schema is explicit and testable in isolation.
- HOST_ABI_VERSION stays at 1 — no ABI bump, no plugin recompile cascade.
- Pattern is extensible: future plugins can read `git_context` from payload.extra
  without additional dispatcher changes.

**Negative:**
- Dispatcher must detect `git commit` commands in Bash tool_input (command-content
  inspection). This is a heuristic; a Bash command that contains "git commit" as a
  substring but is not a real commit (e.g., `echo "git commit"`) would trigger
  injection. This is acceptable: fail-open (the WASM reads empty/irrelevant context
  and skips the check) is the correct posture; there is no security consequence to
  injecting git_context on a non-commit bash call.
- Requires a prerequisite story before S-18.04b can be fully implemented.

## ARCH-INDEX subsystem

SS-01 (Hook Dispatcher Core) + SS-04 (Plugin Ecosystem) — git_context injection
straddles both subsystems. The dispatcher host-layer implementation (payload
enrichment in `crates/factory-dispatcher/src/invoke.rs` and `src/main.rs`) belongs
to SS-01 (Hook Dispatcher Core). The WASM plugin consumer side
(`crates/hook-plugins/validate-burst-log/` and `crates/hook-plugins/validate-dispatch-advance/`
reading `git_context` from `payload.extra`) belongs to SS-04 (Plugin Ecosystem).
This ADR governs the contract between them; both subsystems are affected.

Note: SS-03 is "Event Emission (OTel-Aligned)" (`crates/sink-core/`, `crates/sink-file/`
and related sinks) — it is not the dispatcher. The dispatcher core is SS-01.
