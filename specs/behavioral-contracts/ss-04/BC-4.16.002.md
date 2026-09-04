---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: product-owner
timestamp: 2026-09-04T00:00:00Z
phase: F2
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/S-25.04-f1-delta-analysis.md
  - .factory/cycles/v1.0-brownfield-backfill/S-25.04-f2-architecture-decisions.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.16.001.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.001.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.004.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/stories/S-25.04-close-validate-factory-path-staging-zero-enforcement-gap.md
input-hash: "3123436"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-034"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified:
  - "2026-09-04 (v1.1) — Formal spec closure for the staged-path-listing fail-open behavior (product-owner; S-25.04 post-3-CLEAN finalization sweep; D-1127 precedent — doc-only spec completion after LOCAL convergence, certified CODE FROZEN at feature/S-25.04 HEAD `ff54428a`, UNCHANGED): Postcondition 6 added (fail-open on `git diff --cached --name-only` staged-path-listing failure — non-zero exit or host `Err` — mirroring PC4's branch-detection fail-open and BC-4.16.001 Invariant 3, made explicitly distinct from PC3's fuel-exhaustion/epoch-timeout/`OutputTooLarge` fail-loud INDETERMINATE path); Invariant 9 added (same fail-open obligation, stated as an invariant); EC-009 added (staged-path-listing subprocess failure → fail-open + WARN); T-10 added (mirrors T-5 fail-open style, applied to the staged-path-listing call). Documents AS-IMPLEMENTED behavior only — no semantic change; the intentional inline comment above the `diff_result` match arm in `hook_logic` (`crates/hook-plugins/validate-factory-path-staged/src/lib.rs`) already marked this path as deliberate pending this formal codification. Coverage parity with PC4's fail-open tests applies. BC-INDEX sync (v1.0→v1.1) is a state-manager follow-up, not part of this burst."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.16.002: validate-factory-path-staged WASM PostToolUse Guard MUST Detect Any `.factory/` Path Staged on a Product Branch After a Bash Command Completes — Unconditional Broad-Trigger Check on Every `^Bash$` Dispatch, and MUST Reach the INDETERMINATE Marker-Write Path When It Cannot Complete

## Description

The `validate-factory-path-staged` native-WASM plugin is the **post-hoc detective mirror** of
`validate-factory-path-staging` (BC-4.16.001): it enforces the same nested-worktree
path-exclusivity invariant (INV-E21-001, CAP-034) from the completed-state side rather than the
preventive side. It fires on every completed `PostToolUse` event for the `Bash` tool and,
**unconditionally — with no command-text pre-filter of any kind** (BROAD trigger scope, ratified
2026-09-04; this is final, not provisional), inspects the actual git index and current-branch
state via two `host::exec_subprocess` calls: `git diff --cached --name-only` and
`git branch --show-current`. If any staged path matches `.factory/**` (or `\.factory/` as a path
component, matched case-insensitively, reusing BC-4.16.001's own path-matching predicate
verbatim) AND the current branch is a product branch (any branch other than
`factory-artifacts`), the plugin reports a detected violation. This closes the residual-risk
class BC-4.16.001's own PreToolUse text-based detector cannot see — staging that occurs through
git plumbing, aliases, wrapper scripts, or any invocation whose completed command text never
contains a recognizable `git add`/`git stage` substring (BC-4.16.001 Invariant 6 Accepted
Residuals) — because the check runs against actual index state, not payload text.

This BC does **not** validate the *content* of a `.factory/` artifact write; like its sibling, it
is a narrow git-staging exclusivity guard, applied post-hoc. Its second, equally load-bearing
purpose (S-25.04 AC-001) is to give S-25.01's durable-marker + next-advance-gate mechanism
(BC-1.18.001/BC-1.18.002/BC-1.18.003) a **structurally-reachable PostToolUse trigger path** — a
capability `validate-factory-path-staging`'s own PreToolUse-only registration can never provide
(BC-1.18.001 Invariant 4; BC-1.18.004 Postcondition 4). This validator is a **detective**, not a
**preventive**, control: a PostToolUse hook cannot retroactively undo the write/staging it
observes (ADR-039 §Decision 3) — its "blocking outcome" on detection governs continuation of
subsequent dispatches per this registry's existing PostToolUse block-intent convention, not
reversal of the already-completed git operation.

Registered in `hooks-registry.toml`: `event = "PostToolUse"`, `tool = "^Bash$"`,
`priority = 161` (lowest free slot above the occupied 150–160 band), `failure_policy =
"fail-closed"`, `on_error = "continue"`, `timeout_ms = 5000`, native-WASM
fuel-axis-only calibration (`fuel_cap = max(observed_max × 1.5, 50_000_000)`; ADR-039
§Decision 2 seventh-member entry, native-WASM sub-list alongside `validate-cross-site-
correspondence`). This BC governs the `validate-factory-path-staged` WASM crate at
`crates/hook-plugins/validate-factory-path-staged/` (new crate to be created by S-25.04; named
per the architect's F2 decision — past participle "staged," a completed state being inspected,
distinguishing it from the sibling's gerund "staging," an action inspected before it runs).

## Preconditions

1. A `PostToolUse` event has fired for the `Bash` tool, and the triggering Bash command has
   already completed execution (this plugin runs strictly after the underlying Bash tool call
   finished — regardless of that command's own exit code, and regardless of whether the command
   text itself looked git-related).

2. **Trigger condition — BROAD scope, FINAL (ratified 2026-09-04; not provisional).** The
   plugin's internal git-index/branch check runs **unconditionally** on every completed
   `PostToolUse ^Bash$` dispatch. No command-text pre-filter is applied to the just-completed
   Bash payload — the check does NOT gate on whether the payload contains `git`, `add`, `stage`,
   or any other substring. Both `host::exec_subprocess` calls (`git diff --cached --name-only`,
   `git branch --show-current`) are issued on every invocation, regardless of what the triggering
   command's text looked like. This is the architect-recommended and human-ratified resolution of
   the completeness-vs-cost tradeoff (S-25.04 F2 architecture decisions §1): a narrower,
   text-gated internal filter (e.g., re-using BC-4.16.001's own `git\s+(add|stage)` detector
   against the completed payload) would make this validator a near-tautological confirmation of
   BC-4.16.001's own successes and functionally blind to BC-4.16.001's own documented failure
   modes — the opposite of what a detective mirror exists for, since the check being performed is
   against actual index state, not payload text. The cost is bounded: both `exec_subprocess`
   calls read only git's index metadata and current-branch pointer, governed by staged-path
   cardinality and repository ref state — not by `.factory/` cycle-artifact size (the resource-
   exhaustion driver ADR-039/ADR-047 were written to address) or by triggering payload size.

3. `host::exec_subprocess` capability is granted with `binary_allow = ["git"]` (mirrors
   BC-4.16.001 Precondition 3's `host::exec_subprocess` dependency for its own branch-detection
   call). Two calls are issued per invocation: `git diff --cached --name-only` (staged-path
   list) and `git branch --show-current` (current branch).

4. The dispatcher has invoked the `validate-factory-path-staged` WASM plugin, registered in
   `hooks-registry.toml` as: `event = "PostToolUse"`, `tool = "^Bash$"`, `priority = 161`,
   `failure_policy = "fail-closed"`, `on_error = "continue"`, `timeout_ms = 5000`. Unlike
   `validate-factory-path-staging` (BC-4.16.001, PreToolUse), this registration IS a
   structurally-reachable PostToolUse fail-closed callsite for the marker-write mechanism
   (Postcondition 3 below).

## Postconditions

### PC1 — Detected: `.factory/` path staged on a product branch after Bash command completion

When, after the triggering Bash command completes, `git diff --cached --name-only` returns at
least one path matching `^\.factory/` (or containing `/.factory/` as a path component), matched
**case-insensitively** (reusing BC-4.16.001 Invariant 4's predicate verbatim — e.g.,
`.Factory/STATE.md` matches), AND `git branch --show-current` reports a product branch (any
branch other than `factory-artifacts`):

1. The plugin returns a non-zero exit code (`block_intent = true` in this registry's existing
   PostToolUse block-intent convention).
2. The plugin emits a blocking error message:
   ```
   DETECTED: .factory/ path staged on product branch '<branch>' (post-hoc check).
   .factory/ paths are exclusively owned by the factory-artifacts worktree. A staging
   operation reached the git index without being intercepted by validate-factory-path-
   staging's PreToolUse guard (git plumbing, alias, wrapper script, or under-matched
   invocation text). Unstage immediately: git restore --staged <path> (or equivalent),
   or switch to the .factory/ worktree and commit from there on the factory-artifacts
   branch.
   ```
3. The already-staged `.factory/` path is **NOT** automatically unstaged or reverted by this
   plugin — a PostToolUse detective check cannot retroactively undo a completed git operation
   (ADR-039 §Decision 3). The blocking outcome governs continuation of subsequent dispatches
   per this registry's existing PostToolUse block-intent convention, not reversal of the
   already-completed `git add`/`git stage`/plumbing operation.

**Error variant:** `FactoryPathStagedOnProductBranch`

### PC2 — Passed: no `.factory/` path staged, or branch is `factory-artifacts`

When `git diff --cached --name-only` returns no path matching `.factory/`, OR the current branch
is `factory-artifacts`, the plugin returns exit code 0 (`block_intent = false`) unconditionally.
Mirrors BC-4.16.001 PC2 (no `.factory/` path) and PC3 (legitimate `factory-artifacts`-branch
staging) — both collapse to the same passed outcome here since this plugin does not distinguish
"nothing relevant staged" from "branch is factory-artifacts" at the postcondition level (both
are non-violations of INV-E21-001).

### PC3 — INDETERMINATE trigger: plugin's own execution cannot complete (AC-001 closure criterion)

When this plugin's own WASM execution cannot complete during either `exec_subprocess` call — fuel
exhaustion (`Trap::OutOfFuel`), epoch timeout (`Trap::Interrupt`), or host `OutputTooLarge`
followed by the plugin returning `exit_code = 0` — the dispatcher classifies the outcome as
INDETERMINATE (BC-1.18.001 Postcondition 1). Because `failure_policy = "fail-closed"` AND this
plugin is registered `PostToolUse` (the triggering Bash command has already completed — a write,
in the general PostToolUse sense BC-1.18.001 Postcondition 4 scopes, has occurred), the dispatcher
reaches `write_indeterminate_marker` (BC-1.18.001 Postcondition 4) and emits `plugin.indeterminate`
(BC-3.08.001 Event 8). **This is the story's own AC-001 closure criterion**: this is the first
structurally-reachable production trigger path for the fail-closed marker-write + next-advance-gate
mechanism, since `validate-factory-path-staging`'s own PreToolUse registration can never reach it
(BC-1.18.001 Invariant 4; BC-1.18.004 Postcondition 4). No new marker file, TTL, or gate logic is
introduced — `write_indeterminate_marker`, `should_write_marker`, and `classify_outcome` are
invoked verbatim (BC-1.18.001 Architecture Anchors); this BC specifies only the trigger condition,
not the generic mechanism.

### PC4 — Fail-open: branch-detection failure

When `git branch --show-current` fails (non-zero exit, empty output, git unavailable, detached
HEAD) — a clean, completed `PluginResult::Ok` with an inconclusive answer, distinct from PC3's
resource-exhaustion INDETERMINATE case — the plugin fails open: exit code 0 (`block_intent =
false`), with an advisory warning logged. This mirrors BC-4.16.001 Invariant 3 exactly: an
uncertain branch state is not, on its own, evidence of a `.factory/`-on-product-branch violation,
and is not a blocking condition.

### PC5 — Advisory-only on plugin crash (non-resource-exhaustion Trap)

When the plugin crashes for a reason other than fuel exhaustion, epoch timeout, or OutputTooLarge
(e.g., `Trap::UnreachableCodeReached`, host-ABI mismatch, deserialization failure), `on_error =
"continue"` applies: the crash/timeout event is logged, the outcome is advisory-only, and no
block and no marker write occur. This is orthogonal to PC3 — `on_error` governs the crash-class
failure mode; `failure_policy` governs the resource-exhaustion class (PC3) — per ADR-039
§Decision 1's axes-independence invariant. Setting `on_error = "block"` would add no protective
value here (this plugin's own crash cannot retroactively undo the already-completed Bash
dispatch it was checking); this registration deliberately matches BC-4.16.001 Invariant 2 and
every other PostToolUse validator in this registry's `on_error = "continue"` steady state.

### PC6 — Fail-open: staged-path-listing failure

When `git diff --cached --name-only` fails (non-zero exit, or the `host::exec_subprocess` call
itself returns a host `Err`) — a clean, completed `PluginResult::Ok` with an inconclusive answer
as to what is staged, distinct from PC3's resource-exhaustion INDETERMINATE case (fuel exhaustion,
epoch timeout, or host `OutputTooLarge`) — the plugin fails open: the staged-factory-path lookup
resolves to "nothing detected," exit code 0 (`block_intent = false`), with an advisory `WARN` log
recording the failure (exit code + stderr, or the error string). This mirrors PC4's
branch-detection fail-open and BC-4.16.001 Invariant 3 exactly, extended to this validator's OTHER
`host::exec_subprocess` call: an inconclusive staged-path answer is not, on its own, evidence of a
`.factory/`-on-product-branch violation, and is not a blocking condition. The dispatch falls
through to PC2 (Passed).

**Fail-open vs. fail-loud, made explicit.** A transient/clean git failure on the staged-path-
listing call (this PC) is fail-open. The plugin's OWN inability to complete the call at all — fuel
exhaustion, epoch timeout, or host `OutputTooLarge` — remains PC3's fail-closed INDETERMINATE
path, unaffected by this postcondition. The two are structurally distinct: PC6 is a git-command-
level failure the plugin observes and handles gracefully; PC3 is a plugin-execution-level failure
the dispatcher observes because the plugin never returned control at all.

**As-implemented note.** This documents the behavior already shipped at `feature/S-25.04` HEAD
`ff54428a` in `hook_logic` (`crates/hook-plugins/validate-factory-path-staged/src/lib.rs`), where
the intentional inline comment directly above the `diff_result` match arm marks this fail-open
path as deliberate — mirroring "the branch-detection fail-open below (PC4) and BC-4.16.001
Invariant 3's philosophy" — not an oversight, pending this formal BC codification. Coverage parity
with PC4's fail-open tests applies (same exit-code/`Err` matrix, applied to this call).

**Error variant:** none — this is a passed (non-error) outcome; no error variant is raised.

## Invariants

1. **INV-E21-001 — second enforcement instantiation (detective, post-hoc).** This plugin is the
   second concrete instantiation of INV-E21-001/CAP-034 within `hooks-registry.toml`, companion
   to BC-4.16.001's preventive (PreToolUse) instantiation. Neither is a replacement for the
   other; they are additive layers of the same capability (mirrors BC-4.16.001 Invariant 1's
   framing, applied to the detective side).

2. **Axes independence (on_error vs failure_policy).** `on_error` (crash class — PC5) and
   `failure_policy` (resource-exhaustion class — PC3) are orthogonal, non-unified axes (ADR-039
   §Decision 1). `on_error = "continue"` for crashes is advisory-only regardless of
   `failure_policy`. `failure_policy = "fail-closed"` governs ONLY the cannot-complete/resource-
   exhaustion path via PC3 — it never routes a genuine crash to the marker mechanism.

3. **Fail-open on branch-detection failure.** Mirrors BC-4.16.001 Invariant 3 exactly: if
   `git branch --show-current` fails, the plugin MUST fail open (PC4). Uncertain branch state is
   NOT a blocking condition. This is a distinct failure mode from the plugin's own cannot-complete
   case (PC3), which is a clean git-command failure with a `PluginResult::Ok` return, not a
   resource-exhaustion trap.

4. **Path matching is conservative and case-insensitive, reusing BC-4.16.001's predicate
   verbatim.** The `.factory/` path pattern is matched as a literal path prefix or path
   component, case-insensitively, using the SAME pure path-matching function BC-4.16.001's own
   plugin uses (per architect F2 §2.1: "pure and directly reusable, not merely pattern-mirrored")
   — not a re-derived or independently-specified predicate. Any divergence between the two
   plugins' matching behavior would itself be a coverage gap.

5. **Reuse-not-reimplementation of S-25.01's marker/gate machinery.** This plugin's INDETERMINATE
   handling (PC3) invokes `write_indeterminate_marker`, `should_write_marker`, and
   `classify_outcome` verbatim (BC-1.18.001 Architecture Anchors; `crates/factory-dispatcher/
   src/indeterminate_marker.rs`, `crates/factory-dispatcher/src/executor.rs`). This BC introduces
   NO second marker file, NO duplicate gate mechanism, and NO plugin-specific branching in the
   tier-execution loop — the existing generic PostToolUse fail-closed INDETERMINATE callsite
   already handles any qualifying caller, including this one, with zero mechanism changes
   (S-25.04 story Architecture Compliance Rules; F1 §2 point 1).

6. **Detective, not preventive.** A PostToolUse hook cannot retroactively block or undo the
   write/staging it observes (ADR-039 §Decision 3's "PostToolUse cannot retroactively undo a
   completed write" framing). PC1's blocking outcome governs continuation of subsequent
   dispatches per this registry's existing PostToolUse block-intent convention, never reversal
   of the already-completed git operation itself.

7. **Unconditional BROAD trigger scope is FINAL.** The internal git-index/branch check (PC1–PC4)
   runs on every completed `^Bash$` PostToolUse dispatch with no command-text pre-filter
   (Precondition 2). This was ratified by explicit human confirmation (2026-09-04, following the
   architect's F2 recommendation) — it is a settled design decision for this BC's authorship, not
   a placeholder pending further review.

8. **No self-lock risk introduced.** This validator's registration shape (`PostToolUse`,
   `on_error = "continue"` for crashes, `failure_policy = "fail-closed"` only for resource
   exhaustion) introduces no self-lock risk class — a detective PostToolUse check's own failure
   cannot wedge the session, unlike the two PreToolUse `^Agent$` Cohort A-deferred validators
   (mirrors the reasoning that qualified `validate-factory-path-staging` itself for Cohort
   A-immediate treatment; ADR-047 §Decision 8a).

9. **Fail-open on staged-path-listing failure.** Mirrors Invariant 3 above, extended to this
   validator's OTHER `host::exec_subprocess` call: if `git diff --cached --name-only` fails
   (non-zero exit, or the `exec_subprocess` call itself returns a host `Err`), the plugin MUST
   fail open (PC6). This is a distinct failure mode from the plugin's own cannot-complete case
   (PC3's fuel exhaustion / epoch timeout / `OutputTooLarge`), which remains fail-closed
   INDETERMINATE. Fail-open (a transient, clean git-command error) vs. fail-loud (the plugin's own
   execution cannot complete) is a structural distinction between two different failure classes,
   not a severity judgment applied per-invocation — the same axes-independence framing as
   Invariant 2, applied to this validator's own second `exec_subprocess` callsite.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin's own `exec_subprocess` calls fuel-exhaust or epoch-timeout during a completed PostToolUse Bash dispatch | INDETERMINATE classified; `write_indeterminate_marker` fires (PC3) — the AC-001 closure criterion demonstrated end-to-end; `plugin.indeterminate` emitted |
| EC-002 | A Bash command stages/writes files entirely outside `.factory/` (e.g. `git add src/main.rs`) and completes | The unconditional check still runs (`git diff --cached --name-only`, `git branch --show-current`) per BROAD scope, finds no `.factory/` path → PC2 passes; no block, no marker |
| EC-003 | `validate-factory-path-staging`'s (BC-4.16.001) own PreToolUse registration is unaffected by this BC's existence | BC-4.16.001's own INDETERMINATE outcomes (if any) still cannot reach `write_indeterminate_marker` (BC-1.18.001 Invariant 4) — confirms BC-4.16.001's own unchanged behavior (S-25.04 AC-003); this BC's companion is what closes the gap, not a change to the sibling |
| EC-004 | Both `validate-factory-path-staging`'s PreToolUse guard blocks a `git add` on one dispatch AND, on a later PostToolUse dispatch, `validate-factory-path-staged` (this plugin) itself goes INDETERMINATE fail-closed | The SAME single marker file / last-writer-wins policy (BC-1.18.001 Invariant 3) applies exactly as it would for any two distinct fail-closed plugins; `(plugin_name, artifact_path)` pairs distinguish the two validators' own INDETERMINATE events by construction, so BC-1.18.001's SUPERSEDED-clear corollary (same-pair vs cross-pair) governs identically; no new marker-persistence failure mode is introduced |
| EC-005 | Branch detection fails (detached HEAD, git unavailable) during the post-hoc check | Fail-open (PC4); advisory warning only; mirrors BC-4.16.001 EC-006 |
| EC-006 | `.factory/` path appears in `git diff --cached --name-only` and current branch is `factory-artifacts` (legitimate state-manager commit) | PC2 passes unconditionally; mirrors BC-4.16.001 PC3/EC-002 |
| EC-007 | Bash command was not git-related at all (e.g. `npm test`) and nothing is staged | The check still executes unconditionally per BROAD scope (Precondition 2) — this is NOT a text-based fast-pass skip; it resolves via PC2 because `git diff --cached --name-only` returns nothing relevant. Contrast: if an EARLIER git-plumbing/script/alias action had already staged a `.factory/` path before this non-git command ran, THIS check would still catch it — exactly the residual-risk class the BROAD design exists to close (BC-4.16.001 Invariant 6 Accepted Residuals) |
| EC-008 | Plugin crashes for a non-resource-exhaustion reason (`Trap::UnreachableCodeReached`, host-ABI mismatch) | `on_error = "continue"` → advisory only; no block; no marker (PC5) |
| EC-009 | Staged-path-listing (`git diff --cached --name-only`) subprocess fails: non-zero exit, or the `host::exec_subprocess` call itself returns a host `Err` | Fail-open (PC6); staged-factory-path lookup resolves to "nothing detected"; advisory WARN log; falls through to PC2 passed — no block, no marker. Distinct from a host `OutputTooLarge`/fuel-exhaustion/epoch-timeout cannot-complete condition, which remains PC3's fail-closed INDETERMINATE path |

## Canonical Test Vectors

| Test # | Precondition | Bash command that just completed | Branch | Expected Result |
|--------|-------------|-----------------------------------|--------|-----------------|
| T-1 | `.factory/STATE.md` now in staged-path list | `git add .factory/STATE.md` | `develop` | DETECTED: PC1 `FactoryPathStagedOnProductBranch` |
| T-2 | `.factory/STATE.md` now in staged-path list | `git add .factory/STATE.md` | `factory-artifacts` | PASSED: PC2 |
| T-3 | No `.factory/` path staged | `git add src/lib.rs` | `feature/S-25.04` | PASSED: PC2 (no `.factory/` path) |
| T-4 | `.factory/` path staged via a non-`git-add`-text wrapper script (e.g. a Python script calling git plumbing) | `python3 fix_index.py` | `develop` | DETECTED: PC1 fires — BROAD unconditional check catches it; this is exactly the case a text-gated internal filter would have missed |
| T-5 | Branch detection fails | (any command) | detached HEAD | PASSED: fail-open per PC4 |
| T-6 | Plugin's `exec_subprocess` calls fuel-exhaust mid-invocation | `git add .factory/STATE.md` | `develop` | INDETERMINATE classified; marker written per PC3 |
| T-7 | Plugin crashes (non-resource-exhaustion trap) | `git add src/lib.rs` | `develop` | Advisory only per PC5; no block |
| T-8 | Nothing staged; unrelated command | `npm test` | `develop` | PASSED: PC2 (unconditional check ran, found nothing relevant — not a skipped fast-pass) |
| T-9 | `.factory/stories/S-25.04.md` now in staged-path list | `git add .factory/stories/S-25.04.md` | `release/v1.0.0-rc.25` | DETECTED: PC1 (release branch is a product branch; mirrors BC-4.16.001 T-5) |
| T-10 | Staged-path-listing (`git diff --cached --name-only`) subprocess fails (non-zero exit, or `exec_subprocess` returns a host `Err`) | (any command) | `develop` | PASSED: fail-open per PC6 (mirrors T-5's fail-open style, applied to the staged-path-listing call instead of branch detection) |

## SDK Grounding Evidence

**Grep 1 — WASM crate stub location (to be created by S-25.04):**
```
ls crates/hook-plugins/validate-factory-path-staged/ 2>/dev/null || echo "PLANNED — not yet created"
```
Expected: "PLANNED — not yet created" at authoring time; crate created by S-25.04 implementation.

**Grep 2 — hooks-registry.toml priority 161 collision check (verified by architect, F2 §2.2):**
```
grep -n "priority = 161" plugins/vsdd-factory/hooks-registry.toml
```
Expected: no hit before S-25.04 implementation adds the `validate-factory-path-staged` entry;
priorities 150–160 are fully occupied and 161 is the lowest free slot (architect F2 §2.2 evidence).

**Grep 3 — hooks-registry.toml entry for validate-factory-path-staged (post-implementation):**
```
grep -n "validate-factory-path-staged" plugins/vsdd-factory/hooks-registry.toml
```
Expected: entry present (added by S-25.04) or absent before implementation.

## Verification Properties

VP-TBD — Phase-6 formal-verifier anchor (POLICY 9 sanctioned VP-TBD deferral; per S-25.04 F2
scope, verification-property authorship for this BC is deferred to the formal-hardening phase,
consistent with how BC-4.16.001's own VP row remained TBD through its initial authoring). The
architect's F2 calibration-corpus note (§3.2) applies to the eventual formal-verifier/test-writer
calibration pass for this validator specifically: use a synthetic worst-case index state (≥500
simultaneously staged paths in one dispatch), NOT the six-member large-`.factory/`-artifact
corpus used for the other ADR-039 §Decision 2 members — this validator's resource driver is git
index cardinality, not artifact byte size.

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — Phase-6 formal-verifier anchor) | Block/detect fires on `git diff --cached` containing `.factory/X` when branch != `factory-artifacts`, regardless of triggering command text | bats: invoke guard post-hoc with mocked staged-path list + branch=develop; assert non-zero exit + `FactoryPathStagedOnProductBranch` |
| (TBD) | Pass unconditionally when branch == `factory-artifacts` or no `.factory/` path staged | bats: mocked branch=factory-artifacts; assert exit 0 |
| (TBD) | Fail-open on branch-detection failure | unit: force branch-detection non-zero exit; assert `block_intent=false` advisory-only |
| (TBD) | INDETERMINATE + fail-closed + PostToolUse reaches `write_indeterminate_marker` for THIS plugin's own `plugin_name` | unit: force fuel exhaustion on this plugin's `exec_subprocess` call; assert marker written with `plugin_name = "validate-factory-path-staged"` |
| (TBD) | Calibration: synthetic ≥500-staged-path corpus produces `observed_max` within the `fuel_cap = max(observed_max × 1.5, 50_000_000)` bound | proptest/bench: synthetic worst-case index fixture per architect F2 §3.2 |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-034 |
| Capability Anchor Justification | CAP-034 ("Enforce factory artifact nested-worktree path exclusivity (E-21 INV-E21-001) — dual-layer defense: WASM Bash guard + skill-doc merge pre-check") per capabilities.md §CAP-034 — this BC is the SECOND enforcement instantiation of CAP-034's Layer-1 invariant enforcement (the first is BC-4.16.001's preventive PreToolUse guard); it is a detective, post-hoc mirror of the same `.factory/` path-staging exclusivity check, not a new capability. |
| L2 Domain Invariants | none (operational infrastructure — dispatcher/hook-plugin runtime concern, same classification as BC-4.16.001) |
| Architecture Module | `crates/hook-plugins/validate-factory-path-staged/` (new crate; to be created by S-25.04; architect F2 §2.1 naming/structure authority) |
| Stories | S-25.04 (E-25 follow-up to S-25.01) |
| Source Issues | #342 (product-branch merge silently rm's a `.factory/` file) — same underlying issue as BC-4.16.001; this BC closes the companion structural-reachability gap identified by ADR-047 §Decision 8a "Known Gap" note and BC-1.18.004 v1.2 Postcondition 4 |
| ADR Reference | ADR-039 §Decision 2 (seventh-member roadmap entry, native-WASM/fuel-axis-only sub-list, alongside `validate-cross-site-correspondence`; architect F2 §3.1); ADR-047 §Decision 8a (Known Gap this BC closes; clarification note added at F2, architect F2 §4); ADR-031 §Decision 3 (`validate-factory-path-*` naming family this crate extends) |

## Related BCs

- BC-4.16.001 — sibling; the preventive PreToolUse instantiation of the same CAP-034 invariant. This BC is its detective, post-hoc mirror — NOT a replacement. BC-4.16.001's own registration and behavior are unchanged (S-25.04 AC-003).
- BC-1.18.001 — composes with; this BC's own Postcondition 3 (INDETERMINATE trigger) invokes BC-1.18.001's generic marker-write mechanism (`write_indeterminate_marker`) verbatim. This BC is the SECOND concrete instantiation of BC-1.18.001 Invariant 4's "any qualifying PostToolUse fail-closed plugin" generic trigger class (the first being the mechanism's own S-25.01 delivery vehicle, `validate-unvalidated-mutation-marker`'s crash-path handling — a different, non-marker-writing code path; see BC-1.18.001 vs BC-1.18.002/003).
- BC-1.18.002 — composes with; the next-advance gate (Arm 1/Arm 2) fires generically on marker presence, keyed on the marker file, not on which plugin wrote it. No plugin-specific content to compose beyond marker-write reachability.
- BC-1.18.004 — related; this BC's live registration is what changes BC-1.18.004 Postcondition 4's "Layer-1 effective fail-closed count at S-25.01 merge: ZERO" statement to a non-zero count at S-25.04 merge (see BC-1.18.004 Postcondition 4 companion amendment, same burst).
- BC-5.43.001 — adjacent third layer of CAP-034 defense-in-depth (orchestrator merge-safety pre-check, a different, non-dispatcher-plugin mechanism); cited for disambiguation, not composition — it is a procedural Bash pre-check, not a WASM hook plugin, and cannot produce an INDETERMINATE outcome.

## Architecture Anchors

- `crates/hook-plugins/validate-factory-path-staged/` — WASM plugin source (new crate; to be created by S-25.04; mirrors `crates/hook-plugins/validate-factory-path-staging/` structure: `src/lib.rs`, `src/main.rs` WASI `_start` wrapper, `src/tests.rs`, `tests/proptest_*.rs` reusing BC-4.16.001's path-matching predicate verbatim; architect F2 §2.1)
- `plugins/vsdd-factory/hooks-registry.toml` — new `[[hooks]]` entry: `name = "validate-factory-path-staged"`, `event = "PostToolUse"`, `tool = "^Bash$"`, `priority = 161`, `timeout_ms = 5000`, `on_error = "continue"`, `async = false`, `failure_policy = "fail-closed"`, `[hooks.capabilities.exec_subprocess]` with `binary_allow = ["git"]` (architect F2 §2.3)
- `.factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md` §Decision 2 — seventh-member native-WASM roadmap entry, fuel-axis-only calibration (architect F2 §3)
- `crates/factory-dispatcher/src/indeterminate_marker.rs` — REUSE-UNCHANGED; `write_indeterminate_marker`, `should_write_marker`, `classify_outcome` consumed verbatim as this BC's Postcondition 3 trigger target; no signature or behavior change for this new caller (BC-1.18.001 Architecture Anchors)
- `crates/factory-dispatcher/src/executor.rs` — REUSE-UNCHANGED; tier-execution loop already generically invokes the marker-write callsite for any PostToolUse fail-closed INDETERMINATE outcome; no plugin-specific branching exists there to extend

## Story Anchor

S-25.04 — Close `validate-factory-path-staging` Zero-Enforcement Gap — Real Layer-1 Production Trigger (E-25 follow-up to S-25.01)

## VP Anchors

VP-TBD — Phase-6 formal-verifier anchor (POLICY 9 sanctioned VP-TBD deferral). See Verification
Properties section above for the calibration-corpus note governing the eventual VP authorship
pass for this specific validator.

## Changelog

| Version | Date | Description |
|---------|------|--------------|
| 1.1 | 2026-09-04 | Formal spec closure for the staged-path-listing fail-open behavior (product-owner; S-25.04 post-3-CLEAN finalization sweep; D-1127 precedent — doc-only spec completion after LOCAL convergence, certified CODE FROZEN at feature/S-25.04 HEAD `ff54428a`, UNCHANGED). Postcondition 6 added (fail-open on `git diff --cached --name-only` staged-path-listing failure — non-zero exit or host `Err` — mirroring PC4's branch-detection fail-open and BC-4.16.001 Invariant 3, made explicitly distinct from PC3's fuel-exhaustion/epoch-timeout/`OutputTooLarge` fail-loud INDETERMINATE path). Invariant 9 added (same fail-open obligation, stated as an invariant). EC-009 added (staged-path-listing subprocess failure → fail-open + WARN). T-10 added (mirrors T-5's fail-open style, applied to the staged-path-listing call). Documents AS-IMPLEMENTED behavior only — no semantic change; the intentional inline comment above the `diff_result` match arm in `hook_logic` (`crates/hook-plugins/validate-factory-path-staged/src/lib.rs`) already marked this path as deliberate pending this formal codification. Coverage parity with PC4's fail-open tests applies. BC-INDEX sync (v1.0→v1.1) is a state-manager follow-up. |
| 1.0 | 2026-09-04 | Initial authoring (product-owner; S-25.04 F2 BC authorship burst, following the human's BROAD trigger-scope ratification unblocking the architect's F2 last provisional item). New sibling of BC-4.16.001 under CAP-034/SS-04: `validate-factory-path-staged` WASM PostToolUse `^Bash$` detective mirror of BC-4.16.001's git-staging exclusivity guard, priority 161, `failure_policy = "fail-closed"`, `on_error = "continue"`, native-WASM fuel-axis-only calibration. 4 preconditions (incl. PC2 BROAD unconditional trigger condition, FINAL not provisional), 5 postconditions (PC1 detect/block, PC2 pass, PC3 INDETERMINATE-trigger/AC-001 closure criterion, PC4 branch-detection fail-open, PC5 crash-advisory), 8 invariants (incl. reuse-not-reimplementation of S-25.01 marker/gate machinery, branch-detection fail-open mirroring BC-4.16.001 Invariant 3, BROAD scope finality), 8 edge cases (EC-001..EC-004 per F1/F2 sketch plus EC-005..EC-008), 9 canonical test vectors mirroring BC-4.16.001's T-1..T-9 style. VP authorship deferred to Phase-6 formal-verifier per POLICY 9 (VP-TBD). Traces to ADR-047 §D8a, ADR-039 §D2 (seventh-member roadmap extension), BC-4.16.001, S-25.04. |
