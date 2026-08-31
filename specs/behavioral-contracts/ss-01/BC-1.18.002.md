---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-08-31T00:00:00Z
phase: F2
inputs:
  - .factory/specs/architecture/decisions/ADR-047-indeterminate-outcome-model-durable-mutation-marker-next-advance-gate.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.001.md
  - .factory/feature-delta/validation-integrity-layer1/F1-delta-analysis.md
input-hash: "d56de00"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-041"
lifecycle_status: draft
introduced: v1.0-feature-validation-integrity-layer1
modified: ["v1.1-2026-08-31-exact-subcommand-clarification", "v1.2-2026-08-31-command-detection-comprehensive-expansion"]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.18.002: Next State-Advancing Dispatch Is Blocked While Unvalidated-Mutation Marker Exists (Agent Arm and git commit/push Arm)

## Description

When `.factory/unvalidated-mutation.marker` exists, the `validate-unvalidated-mutation-marker`
PreToolUse gate plugin enforces a two-arm quarantine covering the complete durable-propagation
surface: (Arm 1) all `^Agent$` tool dispatches are blocked; (Arm 2) all Bash dispatches for
which `is_git_commit_or_push(command)` returns `true` are blocked. The Arm 2 filter is a
production-grade security filter (v1.2): it splits compound commands on shell operators
(`&&`, `||`, `;`, `|`, `&`, newline), identifies the git executable by basename (matching
`/usr/bin/git`, `./git`, `env VAR=x git`, etc.), skips the complete recognized set of git
global options (seven arg-taking options that consume a separate-token argument, plus
recognized no-arg flags), applies a fail-safe posture for unrecognized options (block when
subcommand position is uncertain), and performs exact subcommand matching (`commit` or `push`
only). Both arms are unblocked simultaneously when the marker is absent. Non-advancing tool
dispatches (Read, Edit, Write, non-git-commit/push Bash) are never gated. The gate plugin is
registered `failure_policy = "fail-open"` to prevent self-lock: if the gate itself cannot
complete, the dispatch proceeds rather than creating an unconditional deadlock (ADR-047
§Decision 4 rationale).

## Preconditions

1. The `validate-unvalidated-mutation-marker` plugin is registered in `hooks-registry.toml`
   with TWO `[[hook]]` entries:
   - **Arm 1:** `event = "PreToolUse"`, `tool = "^Agent$"`, `on_error = "block"`, `async = false`,
     `failure_policy = "fail-open"`, `name = "validate-unvalidated-mutation-marker"`.
   - **Arm 2:** `event = "PreToolUse"`, `tool = "^Bash$"`, `on_error = "block"`, `async = false`,
     `failure_policy = "fail-open"`, `name = "validate-unvalidated-mutation-marker-git"`.
2. The `.factory/` directory is accessible by the gate plugin (path allowlist covers
   `.factory/unvalidated-mutation.marker`).
3. The tool dispatch is arriving at the PreToolUse gate evaluation phase.

## Postconditions

1. **Marker exists → Agent dispatch blocked (Arm 1).** When `.factory/unvalidated-mutation.marker`
   exists, the `validate-unvalidated-mutation-marker` PreToolUse registration fires on `^Agent$`
   dispatches and returns `exit_code = 2` (block). The block message MUST contain:
   - The `plugin_name` field from the marker (the plugin that produced INDETERMINATE).
   - The `artifact_path` field from the marker (the artifact written in the triggering PostToolUse).
   - The `cause` field from the marker (fuel | epoch | output-too-large).
   - A recommended re-validation command: `cargo test --test <plugin_name>_integration` (or
     equivalent — the exact command pattern is informational; the block message MUST name the
     plugin).
   - The manual escape hatch: `rm .factory/unvalidated-mutation.marker`.

2. **Marker exists + git commit/push command → Bash dispatch blocked (Arm 2).** When
   `.factory/unvalidated-mutation.marker` exists AND `is_git_commit_or_push(command)` returns
   `true`, the `validate-unvalidated-mutation-marker-git` registration fires and returns
   `exit_code = 2` (block). The block message contains the same recovery information as Arm 1
   (plugin_name, artifact_path, cause, recovery command, manual escape hatch).

   **`is_git_commit_or_push(command) → bool` — Authoritative Algorithm (v1.2):**

   The function is a production-grade security filter. Under-blocking is the dangerous failure
   mode; the algorithm is fail-safe on ambiguity.

   **Phase 1 — Compound command splitting.** Split `command` on the shell operators `&&`,
   `||`, `;`, `|`, `&`, and newline (`\n`) into segments. Trim leading and trailing whitespace
   from each segment. Apply Phases 2–4 to **each** segment independently. Return `true` if
   **any** segment returns `true`.

   **Phase 2 — Executable identification by basename.** For each segment:
   a. Strip any leading env-var assignment tokens (tokens matching the pattern
      `^[A-Za-z_][A-Za-z0-9_]*=`) and the leading literal token `env` if it is the first
      token. These are environment-setup prefixes, not the executable.
   b. The first remaining token is the executable. Compute `basename` by stripping all
      characters up to and including the last `/`. Examples: `basename("/usr/bin/git") = "git"`,
      `basename("./git") = "git"`, `basename("git") = "git"`.
   c. If `basename(executable_token) ≠ "git"` (case-sensitive), return `false` for this
      segment.

   **Phase 3 — Skip global options.** Scan subsequent tokens after the `git` executable:

   - **Token is `--`:** end-of-options marker; stop Phase 3; the next token is the subcommand.
   - **Token starts with `-` AND contains `=`:** inline option with embedded value
     (e.g., `--git-dir=.git`, `-c foo=bar`, `--exec-path=/usr/lib/git-core`); skip this
     token only — does NOT consume the next token.
   - **Token matches a recognized arg-taking option (separate-token argument):** skip this
     token AND the immediately following token. The **complete** recognized arg-taking set is:

     | Token | Argument consumed |
     |-------|-------------------|
     | `-C` | next token (working-directory path) |
     | `-c` | next token (name=value config pair) |
     | `--namespace` | next token (namespace string) |
     | `--git-dir` | next token (repository path) |
     | `--work-tree` | next token (working-tree path) |
     | `--super-prefix` | next token (super-project prefix path) |
     | `--config-env` | next token (name=envvar pair) |

     Note: `--exec-path` does NOT consume a separate token. It appears as `--exec-path`
     (standalone no-arg form, prints exec path) or `--exec-path=<path>` (inline form, covered
     by the inline-option rule above). There is no valid `git --exec-path <space> <path>`
     invocation in any released git version; treat `--exec-path` as no-arg.

   - **Token matches a recognized no-arg option:** skip this token only. The recognized no-arg
     set: `--no-pager`, `--paginate`, `-p`, `--bare`, `--literal-pathspecs`,
     `--no-literal-pathspecs`, `--glob-pathspecs`, `--noglob-pathspecs`, `--icase-pathspecs`,
     `--no-replace-objects`, `--no-optional-locks`, `--exec-path`, `--html-path`, `--man-path`,
     `--info-path`, `--version`, `--help`, `-v`.
   - **Token starts with `-` AND is NOT matched by any of the above rules:** **fail-safe —
     return `true` (block).** An unrecognized option's arity cannot be determined from the
     closed recognized set; the subcommand position is uncertain. Since this is
     security-critical gate infrastructure where under-blocking is the dangerous failure mode,
     the conservative posture treats any command with an unrecognized flag as gated.
   - **Token does NOT start with `-`:** proceed to Phase 4 with this token as the candidate
     subcommand.

   **Phase 4 — Subcommand match.** Return `true` iff the candidate subcommand token is
   exactly `commit` or `push` (case-sensitive, full-token match). Return `false` for
   `commit-graph`, `commit-tree`, `push-pack`, or any other token that merely contains
   `commit` or `push` as a substring or prefix.

   **Command filter MUST match (non-exhaustive examples):**
   - `git commit -m "..."`, `git commit --amend`, `git commit --no-edit`
   - `git -C .factory commit -m "state"`, `git -c user.email=x push origin main`
   - `git push origin factory-artifacts`, `git push --force-with-lease`
   - `git --git-dir .git commit -m "state"`, `git --work-tree /tmp push`
   - `git status && git commit -m "x"`, `git diff ; git push`
   - `/usr/bin/git commit -m "init"`, `./git push origin main`
   - `env GIT_DIR=.git git commit -m "x"`

   **Command filter MUST NOT match:**
   - `git status`, `git log`, `git diff`, `git fetch`, `git pull`, `git rebase`, `git stash`
   - `git commit-graph write` — subcommand is `commit-graph`, not `commit`
   - `git status && git log --oneline` — no segment resolves to `commit` or `push`
   - `cat gitfile`, `grep git` — `basename(executable) ≠ "git"`

   Note: the illustrative regex `\bgit\b.*\b(commit\|push)\b` approximates but is NOT the
   authoritative rule — it false-positives on `git commit-graph write` (EC-011), cannot handle
   compound commands (EC-015, EC-016), and does not enforce basename matching (EC-017, EC-018).
   The three-phase algorithm above is the sole authoritative specification.

3. **Non-advancing dispatches are NOT gated.** The following dispatches proceed unconditionally
   regardless of marker state:
   - Bash dispatches whose `command` does NOT identify `commit` or `push` as the git subcommand (i.e., `is_git_commit_or_push(command)` returns `false`).
   - Read, Edit, Write, MultiEdit dispatches.
   - All other tool types not matching `^Agent$` or `^Bash$`.
   Routine diagnostic and authoring work is not frozen while the marker is active. The gate is
   designed to block ONLY the two state-advancing pipeline actions: Agent dispatch (which loads
   specs and plans the next mutation) and durable-branch propagation via git commit/push.

4. **Marker absent → both arms pass.** When `.factory/unvalidated-mutation.marker` is absent:
   - Arm 1: `validate-unvalidated-mutation-marker` returns `exit_code = 0` (allow) for
     `^Agent$` dispatches, unconditionally.
   - Arm 2: `validate-unvalidated-mutation-marker-git` returns `exit_code = 0` (allow) for
     matching Bash dispatches, unconditionally.
   Both arms pass simultaneously; a single marker absence unblocks the full dispatch surface.

## Invariants

1. **Same WASM binary, two registrations.** Arm 1 and Arm 2 are two `[[hook]]` entries pointing
   to the same `validate-unvalidated-mutation-marker.wasm` plugin binary. The plugin reads the
   tool payload (`command` parameter for Bash dispatches) to apply the command-content filter in
   Arm 2. Plugin-internal: for Bash events, if `command` does not identify `commit` or `push` as
   the git subcommand (i.e., `is_git_commit_or_push(command)` returns `false`), the plugin
   returns `exit_code = 0` immediately (before reading the marker).

2. **Gate is fail-open.** Both registrations have `failure_policy = "fail-open"`. If the gate
   plugin itself cannot complete (e.g., fuel exhaustion reading the marker), the dispatch proceeds
   (no unconditional self-lock). This is deliberate: the INDETERMINATE model protects WRITES
   (PostToolUse); a fail-open PreToolUse gate that guards Agent dispatch must not introduce a new
   unconditional self-lock class. (The write-integrity invariant is protected by the durable marker
   written at PostToolUse time, not by the gate's own fail-closed behavior.)

3. **Marker-clear unblocks both arms.** Deleting `.factory/unvalidated-mutation.marker` (by
   successful re-validation per BC-1.18.003 PC1 or by manual operator escape hatch per BC-1.18.003
   PC3) unblocks BOTH arms simultaneously. No separate per-arm clear action is required.

4. **Block message completeness.** The block message content is machine-parseable in addition to
   human-readable: it MUST name the `plugin_name` from the marker (so an operator can identify
   the exact re-validation command), the `artifact_path` (so the operator knows which write is
   unvalidated), and the `cause` (so the operator understands the resource-exhaustion class).

5. **Self-lock hazard awareness.** The gate blocks ALL Agent dispatches including those that would
   dispatch a re-validator. The escape hatch (`rm .factory/unvalidated-mutation.marker`) is
   intentionally simple: an operator with shell/process-environment access can unblock the session
   without requiring another Agent dispatch. This is the break-glass pattern from ADR-039
   §Decision 3 applied to the marker gate.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Marker present; `git status --porcelain` Bash dispatch | NOT blocked. Subcommand is `status`, not `commit` or `push`. Passes unconditionally. |
| EC-002 | Marker present; `git log --oneline -5` Bash dispatch | NOT blocked. `git log` does not match pattern. |
| EC-003 | Marker present; `git diff HEAD~1` Bash dispatch | NOT blocked. `git diff` does not match pattern. |
| EC-004 | Marker present; `git fetch origin` Bash dispatch | NOT blocked. `git fetch` does not match pattern. |
| EC-005 | Marker present; `cargo test --workspace` Bash dispatch | NOT blocked. Non-git command does not match pattern. |
| EC-006 | Marker present; `git commit --amend --no-edit` Bash dispatch | BLOCKED. Subcommand is `commit`. |
| EC-007 | Marker present; `git push --force-with-lease` Bash dispatch | BLOCKED. Subcommand is `push`. |
| EC-008 | Marker file is malformed (unparseable TOML) | Gate returns exit_code=2 (block). The gate checks marker FILE existence; if the file exists but is malformed, the block message includes a note that the marker could not be parsed; the operator must manually `rm` the marker. |
| EC-009 | Gate plugin fuel-exhausts reading the marker (Arm 1 or Arm 2) | PASS (fail-open posture). Dispatch allowed. Advisory `plugin.indeterminate` event emitted for the gate itself (if any — gate is fail-open so no nested marker written). |
| EC-010 | Marker absent; `git commit -m "fix"` Bash dispatch | NOT blocked. Arm 2 returns exit_code=0 immediately (marker absent check). |
| EC-011 | Marker present; `git commit-graph write` Bash dispatch | NOT blocked. The git subcommand is `commit-graph`, not `commit`. Exact-subcommand matching correctly excludes this maintenance command. Note: the illustrative regex `\bgit\b.*\b(commit\|push)\b` would false-positive here because the hyphen before `graph` is a word boundary, making `\bcommit\b` match inside `commit-graph` — this is precisely why the regex is NOT the authoritative rule (see PC2 v1.1 clarification). |
| EC-012 | Marker present; `git -C .factory commit -m "state"` Bash dispatch | BLOCKED. The git global option `-C .factory` is tolerated; the subcommand (first non-option token after `git` + global opts) is `commit`. Exact-subcommand matching fires. |
| EC-013 | Marker present; `git --git-dir .git commit -m "state"` Bash dispatch | BLOCKED. `--git-dir` is in the recognized arg-taking set; `.git` is consumed as its separate-token argument. The first remaining positional token is `commit`. Exact-subcommand match fires. |
| EC-014 | Marker present; `git --work-tree /tmp push` Bash dispatch | BLOCKED. `--work-tree` is in the recognized arg-taking set; `/tmp` is consumed as its argument. The first remaining positional token is `push`. Exact-subcommand match fires. |
| EC-015 | Marker present; `git status && git commit -m "x"` Bash dispatch | BLOCKED. Phase 1 splits on `&&`: segment 1 = `git status` (subcommand `status`, not gated); segment 2 = `git commit -m "x"` (subcommand `commit`, gated). Any-segment `true` → function returns `true`. |
| EC-016 | Marker present; `git diff ; git push` Bash dispatch | BLOCKED. Phase 1 splits on `;`: segment 1 = `git diff` (subcommand `diff`, not gated); segment 2 = `git push` (subcommand `push`, gated). |
| EC-017 | Marker present; `/usr/bin/git commit -m "init"` Bash dispatch | BLOCKED. Phase 2: `basename("/usr/bin/git") = "git"`. Subcommand is `commit`. |
| EC-018 | Marker present; `./git push origin main` Bash dispatch | BLOCKED. Phase 2: `basename("./git") = "git"`. Subcommand is `push`. |
| EC-019 | Marker present; `git status && git log --oneline` Bash dispatch | NOT blocked. Phase 1 splits on `&&`: segment 1 subcommand = `status`; segment 2 subcommand = `log`. No segment matches `commit` or `push`; returns `false`. |
| EC-020 | Marker present; `cat gitfile` Bash dispatch | NOT blocked. Phase 2: `basename("cat") = "cat" ≠ "git"`. Returns `false` for this segment. |
| EC-021 | Marker present; `env GIT_DIR=.git git commit -m "msg"` Bash dispatch | BLOCKED. Phase 2 strips leading literal token `env` and env-var assignment token `GIT_DIR=.git`. Executable is `git`. Subcommand is `commit`. |
| EC-022 | Marker present; `git --unknown-flag commit` Bash dispatch | BLOCKED. Phase 3: `--unknown-flag` is not in the recognized arg-taking or no-arg sets and does not contain `=`. Fail-safe posture: return `true` (block). Subcommand position cannot be determined with certainty from the closed recognized set. |
| EC-023 | Marker present; `git --config-env FOO=BAR commit` Bash dispatch | BLOCKED. `--config-env` is in the recognized arg-taking set; `FOO=BAR` is consumed as its argument. The first remaining positional token is `commit`. |

## Canonical Test Vectors

| Marker State | Tool / Command | Expected Gate Decision |
|-------------|---------------|----------------------|
| Exists | `^Agent$` dispatch | Block (exit_code=2); block message names plugin_name + artifact_path + cause |
| Absent | `^Agent$` dispatch | Allow (exit_code=0) |
| Exists | Bash `git commit -m "test"` | Block (exit_code=2) — Arm 2 |
| Absent | Bash `git commit -m "test"` | Allow (exit_code=0) — Arm 2 |
| Exists | Bash `git push origin factory-artifacts` | Block (exit_code=2) — Arm 2 |
| Absent | Bash `git push origin factory-artifacts` | Allow (exit_code=0) — Arm 2 |
| Exists | Bash `git status --porcelain` | Allow (exit_code=0) — not gated (PC3) |
| Exists | Bash `git log --oneline` | Allow (exit_code=0) — not gated (PC3) |
| Exists | Bash `cargo test --workspace` | Allow (exit_code=0) — not gated (PC3) |
| Exists | Read tool dispatch | Allow — not gated (PC3) |
| Exists | Edit tool dispatch | Allow — not gated (PC3) |
| Exists | Write tool dispatch | Allow — not gated (PC3) |
| Marker deleted (rm) | `^Agent$` dispatch | Allow — marker absent after rm (BC-1.18.003 PC3) |
| Exists | Bash `git commit-graph write` | Allow (exit_code=0) — not gated; subcommand is `commit-graph`, not `commit` (EC-011) |
| Exists | Bash `git -C .factory commit -m "fix"` | Block (exit_code=2) — Arm 2; global option `-C` tolerated, subcommand is `commit` (EC-012) |
| Exists | Bash `git --git-dir .git commit -m "state"` | Block (exit_code=2) — Arm 2; `--git-dir` arg-taking, `.git` consumed as arg, subcommand `commit` (EC-013) |
| Exists | Bash `git --work-tree /tmp push` | Block (exit_code=2) — Arm 2; `--work-tree` arg-taking, `/tmp` consumed as arg, subcommand `push` (EC-014) |
| Exists | Bash `git status && git commit -m "x"` | Block (exit_code=2) — Arm 2; compound split on `&&`, second segment subcommand `commit` (EC-015) |
| Exists | Bash `git diff ; git push` | Block (exit_code=2) — Arm 2; compound split on `;`, second segment subcommand `push` (EC-016) |
| Exists | Bash `/usr/bin/git commit -m "init"` | Block (exit_code=2) — Arm 2; basename `/usr/bin/git` → `git`, subcommand `commit` (EC-017) |
| Exists | Bash `./git push origin main` | Block (exit_code=2) — Arm 2; basename `./git` → `git`, subcommand `push` (EC-018) |
| Exists | Bash `git status && git log --oneline` | Allow (exit_code=0) — no segment subcommand is `commit` or `push` (EC-019) |
| Exists | Bash `cat gitfile` | Allow (exit_code=0) — basename `cat` ≠ `git` (EC-020) |
| Exists | Bash `env GIT_DIR=.git git commit -m "x"` | Block (exit_code=2) — Arm 2; env prefix stripped, executable `git`, subcommand `commit` (EC-021) |
| Exists | Bash `git --unknown-flag commit` | Block (exit_code=2) — Arm 2; fail-safe on unrecognized option, subcommand position uncertain (EC-022) |

## Related BCs

- BC-1.18.001 — INDETERMINATE outcome and marker write; this BC defines what happens when that marker is present (composes with)
- BC-1.18.003 — marker-clear protocol: successful re-validation or rm clears the marker and unblocks this gate (depends on)
- BC-1.18.004 — fail-open no-marker; fail-open INDETERMINATE never triggers this gate (sibling)

## Architecture Anchors

- `crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs` (new WASM plugin crate) — `guard_logic::evaluate_gate(dir) → GateDecision`; `guard_logic::is_git_commit_or_push(command) → bool`; Arm 1 (Agent) and Arm 2 (Bash command filter) implemented in same binary
- `plugins/vsdd-factory/hooks-registry.toml` — TWO `[[hook]]` entries: `validate-unvalidated-mutation-marker` (Arm 1: `tool = "^Agent$"`) and `validate-unvalidated-mutation-marker-git` (Arm 2: `tool = "^Bash$"`)
- `crates/hook-plugins/validate-unvalidated-mutation-marker/Cargo.toml` (new crate manifest)

## Story Anchor

S-25.01 — Dispatcher INDETERMINATE Outcome Layer 1: Fail-Loud on Cannot-Complete — durable marker + next-advance gate

## VP Anchors

- VP-105 — Next-Advance Gate Blocks Agent Dispatch and git commit/push While Marker Exists, Passes When Absent (integration + unit-test; covers PC1/PC2/PC3/PC4 all arms)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-105 | Marker exists → Agent dispatch blocked (Arm 1, exit_code=2); marker absent → Agent dispatch allowed (Arm 1); manual rm unblocks; Edit not gated (PC3) | integration (bats) |
| VP-105 | Marker exists + git commit Bash → blocked (Arm 2); marker absent + git commit → allowed; git status not gated even when marker exists (PC3) | integration (bats) |
| VP-105 | guard_logic::evaluate_gate: marker present → BlockDispatch; marker absent → Allow; read-error → Allow (fail-open) | unit-test |
| VP-105 | guard_logic::is_git_commit_or_push (v1.2 algorithm): (1) exact subcommand — `commit`/`push` match; `status`/`log`/`diff`/`fetch`/`commit-graph` do NOT match; (2) complete arg-taking option set — each of `-C`, `-c`, `--namespace`, `--git-dir`, `--work-tree`, `--super-prefix`, `--config-env` skips its following separate-token argument before subcommand identification; (3) compound splitting — `&&`, `\|\|`, `;`, `\|`, `&`, newline each split the command into independent segments; any-segment `true` returns `true`; (4) basename matching — `/usr/bin/git commit` → blocked, `./git push` → blocked, `env GIT_DIR=x git commit` → blocked, `cat gitfile` → NOT blocked; (5) fail-safe on unrecognized options — `git --unknown-flag commit` → blocked (subcommand position uncertain) | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-041 |
| Capability Anchor Justification | CAP-041 ("Validation Integrity: INDETERMINATE Outcome, Durable Mutation Marker, and Next-Advance Gate") per capabilities.md §CAP-041 — this BC specifies the next-advance gate behavior that is the third element of what CAP-041 defines: "blocking of the next state-advancing dispatch — the `validate-unvalidated-mutation-marker` PreToolUse plugin … blocks ALL `^Agent$` tool dispatches AND all Bash dispatches whose `command` identifies `commit` or `push` as the git subcommand (D9 extended gate; the illustrative regex `\bgit\b.*\b(commit\|push)\b` approximates but is not authoritative — see v1.1 clarification) while the marker exists." |
| L2 Domain Invariants | none (dispatcher runtime gate invariant, not L2 domain spec) |
| Architecture Module | SS-04 (Plugin Ecosystem — new `validate-unvalidated-mutation-marker` WASM plugin crate); SS-01 (Hook Dispatcher Core — evaluates block_intent from plugin exit_code=2 in PreToolUse dispatch chain) |
| ADR | ADR-047 §Decision 4 (Next-Advance Gate plugin specification — two-arm registration, exit_code=2 block, fail-open gate posture); ADR-047 §Decision 9 (extended gate scope — Agent dispatch AND git commit/push Bash arm — human ratification amendment); ADR-047 §Decision 5 (Marker clear protocol — rm unblocks both arms simultaneously) |
| Stories | S-25.01 |
| Cycle | v1.0-feature-validation-integrity-layer1 (F2 — product-owner spec burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.2 | 2026-08-31 | product-owner | Senior-architect gate expansion: production-grade security filter closing three adversary-identified under-blocking classes. (1) Incomplete arg-taking set (adversary HIGH): defined the complete seven-option arg-taking set (`-C`, `-c`, `--namespace`, `--git-dir`, `--work-tree`, `--super-prefix`, `--config-env`) with explicit no-arg list and fail-safe posture for unrecognized options. (2) Compound commands (adversary LOW-1): Phase 1 compound splitting on `&&`, `\|\|`, `;`, `\|`, `&`, newline — any-segment match returns true. (3) Path-prefixed/basename git (adversary LOW-2): Phase 2 basename identification strips leading path components; `env VAR=x` prefix stripped. Added EC-013 through EC-023 and 10 new canonical test vectors. Updated VP-105 unit-test property row. ADR-047 §Decision 9. |
| 1.1 | 2026-08-31 | product-owner | Spec adjudication: clarify Arm 2 command filter from illustrative regex to authoritative exact-subcommand matching. The regex `\bgit\b.*\b(commit\|push)\b` false-positives on `git commit-graph write` (hyphen is a word boundary, making `\bcommit\b` match inside `commit-graph`). Authoritative rule: `is_git_commit_or_push(command)` identifies the git subcommand (first non-option token after `git` + global options) and returns true iff it is exactly `commit` or `push`. Added EC-011 (`git commit-graph write` → NOT blocked), EC-012 (`git -C path commit` → blocked), two canonical test vector rows, and updated VP-105 unit-test property to name `commit-graph` as a NOT-matched case. Updated `modified[]` frontmatter. |
| 1.0 | 2026-08-30 | product-owner | Initial creation. F2 spec-evolution burst, validation-integrity-layer1. BC-1.18.002: two-arm gate (Agent + git commit/push Bash), PC1-PC4 full coverage, Arm 2 command-pattern matching, fail-open gate posture, self-lock-hazard invariant. D9 human ratification (extended gate scope) reflected. VP-105 anchored. CAP-041 capability anchor. ADR-047 §D4/D9/D5 citations. |
