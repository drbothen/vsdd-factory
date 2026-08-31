---
document_type: behavioral-contract
level: L3
version: "1.5"
status: draft
producer: product-owner
timestamp: 2026-08-31T00:00:00Z
phase: F2
inputs:
  - .factory/specs/architecture/decisions/ADR-047-indeterminate-outcome-model-durable-mutation-marker-next-advance-gate.md
  - .factory/specs/architecture/decisions/ADR-048-fail-closed-but-recoverable-gate-block-if-marker-crash-policy-marker-ttl-deadman-and-ungated-escape-invariant.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.001.md
  - .factory/feature-delta/validation-integrity-layer1/F1-delta-analysis.md
input-hash: "2448fd6"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-041"
lifecycle_status: draft
introduced: v1.0-feature-validation-integrity-layer1
modified: ["v1.1-2026-08-31-exact-subcommand-clarification", "v1.2-2026-08-31-command-detection-comprehensive-expansion", "v1.3-2026-08-31-fail-open-reconciliation-threat-model-quoting-scope", "v1.4-2026-08-31-read-error-vs-malformed-marker-distinction", "v1.5-2026-08-31-block_if_marker-crash-policy-TTL-ungated-escape"]
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
production-grade security filter (v1.3): it splits compound commands on shell operators
(`&&`, `||`, `;`, `|`, `&`, newline), identifies the git executable by basename (matching
`/usr/bin/git`, `./git`, `env VAR=x git`, etc.), skips the complete recognized set of git
global options (seven arg-taking options that consume a separate-token argument, plus
recognized no-arg flags), applies a fail-safe posture for unrecognized options (block when
subcommand position is uncertain), and performs exact subcommand matching (`commit` or `push`
only). Both arms are unblocked simultaneously when the marker is absent. Non-advancing tool
dispatches (Read, Edit, Write, non-git-commit/push Bash) are never gated. The gate plugin is
registered `on_error = "block_if_marker"` (dispatcher-level conditional-fail-closed-on-crash:
when the WASM plugin itself errors — crash, fuel exhaustion, epoch timeout — the dispatcher
executes a native marker-presence-and-TTL check; BLOCKS iff a non-expired
`.factory/unvalidated-mutation.marker` exists; ALLOWS otherwise; ADR-048 §Decision 1; ADR-039
two-axis model — the new `on_error` value extends but does not collapse the two axes). This is
orthogonal to the filter's internal fail-SAFE posture (Phase 3 blocks on unrecognized git global
options — a within-filter conservative choice, not a dispatcher-crash policy). As of v1.3,
QUOTED-literal git invocations (`git "commit"`, `git 'push' origin`, `g'i't commit`) are IN
SCOPE, handled via Phase 1b POSIX quote-aware tokenization using the `shell-words` crate.

## Preconditions

1. The `validate-unvalidated-mutation-marker` plugin is registered in `hooks-registry.toml`
   with TWO `[[hook]]` entries:
   - **Arm 1:** `event = "PreToolUse"`, `tool = "^Agent$"`, `on_error = "block_if_marker"`, `async = false`,
     `failure_policy = "fail-open"`, `name = "validate-unvalidated-mutation-marker"`.
   - **Arm 2:** `event = "PreToolUse"`, `tool = "^Bash$"`, `on_error = "block_if_marker"`, `async = false`,
     `failure_policy = "fail-open"`, `name = "validate-unvalidated-mutation-marker-git"`.

   **Two-axis model (ADR-039; ADR-048 §Decision 1):** Two orthogonal failure-mode axes govern
   the gate's behavior and must never be conflated:
   - **(i) `block_if_marker` crash policy** (`on_error = "block_if_marker"`): when the WASM
     plugin itself fails at the dispatcher layer — crash, fuel exhaustion, epoch timeout — the
     dispatcher executes a NATIVE (non-WASM) marker-presence-and-TTL check: BLOCK iff a
     non-expired `.factory/unvalidated-mutation.marker` exists; ALLOW otherwise (marker absent
     or `expires_at` elapsed). This supersedes the v1.3 `on_error = "continue"` unconditional-
     allow-on-crash posture (ADR-047 §Decision 4 crash behavior superseded by ADR-048 §Decision
     1; D-1135 fail-open-on-crash ratification reversed). The self-lock hazard (INV2) is avoided
     by the no-marker → Allow branch, NOT by unconditional allow: a crash with no quarantine
     signal cannot produce a self-lock. See PC5 (crash+non-expired-marker → Block) and PC6
     (crash+absent/expired-marker → Allow). See INV6 for the ungated-escape invariant that
     ensures recoverability under the tightened crash policy.
   - **(ii) Filter-internal fail-SAFE posture** (Phase 3): when the `is_git_commit_or_push`
     filter encounters an UNRECOGNIZED git global option, it returns BLOCK (subcommand position
     is uncertain; under-blocking is the dangerous failure mode for the command filter). This is a
     conservative within-filter design choice, orthogonal to axis (i): the filter's internal
     posture has no bearing on what the dispatcher does when the WASM plugin itself crashes.

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

   **`is_git_commit_or_push(command) → bool` — Authoritative Algorithm (v1.3):**

   The function is a production-grade security filter. Under-blocking is the dangerous failure
   mode; the algorithm is fail-safe on ambiguity.

   **Phase 1 — Compound command splitting.** Split `command` on the shell operators `&&`,
   `||`, `;`, `|`, `&`, and newline (`\n`) into segments. Trim leading and trailing whitespace
   from each segment. Apply Phases 1b–4 to **each** segment independently. Return `true` if
   **any** segment returns `true`.

   **Phase 1b — POSIX quote-aware tokenization.** Before applying Phases 2–4, tokenize each
   segment from Phase 1 using POSIX word-splitting rules via the `shell-words` crate. Quote
   removal and token concatenation occur at this phase — Phases 2–4 operate on the resulting
   token list, not on the raw whitespace-split string:
   - `git "commit"` → tokens: `["git", "commit"]` (double-quoted `commit` is equivalent to
     unquoted after POSIX quote removal)
   - `git 'push' origin` → tokens: `["git", "push", "origin"]` (single-quoted `push` is
     equivalent to unquoted)
   - `g'i't commit` → tokens: `["git", "commit"]` (POSIX concatenates the unquoted `g`,
     single-quoted `i`, and unquoted `t` character runs into the token `git`)

   **Do NOT implement ad-hoc quote-stripping** (e.g., `s/["']//g` regex) — the `shell-words`
   crate provides POSIX parser-equivalence, eliminating the parser-differential bypass class
   where ad-hoc stripping could produce different tokenization than the actual shell.

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
   - `git "commit" -m "msg"`, `git 'push' origin main`, `g'i't commit -m "x"` (Phase 1b quote-aware tokenization; v1.3)

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

5. **Gate crash + non-expired marker present → dispatcher BLOCKS (block_if_marker).** When the
   gate plugin crashes, fuel-exhausts (wasmtime `Trap::OutOfFuel`), or times out (wasmtime
   `Trap::Interrupt`) while a non-expired `.factory/unvalidated-mutation.marker` exists, the
   dispatcher's native `block_if_marker` crash-handler reads the marker file, parses `expires_at`,
   confirms `expires_at > now (UTC)`, and emits `exit_code = 2` (block). The block message
   includes the marker's `plugin_name`, `artifact_path`, `cause`, `expires_at`, and the three
   recovery options: (a) `rm .factory/unvalidated-mutation.marker`; (b) re-validate via
   Edit/Write to the failing artifact; (c) wait for TTL expiry (ADR-048 §Decision 1;
   fail-closed on the durable quarantine signal even through WASM plugin unavailability).
   I/O failures in the crash-handler's native file-read (ENOENT, EACCES) are treated as
   "absent" (Allow) — keeping the crash-handler fail-open on its own I/O failures; not a
   contradiction of PC5 because the marker's existence cannot be confirmed.

6. **Gate crash + marker absent or TTL expired → dispatcher ALLOWS.** When the gate plugin
   crashes while `.factory/unvalidated-mutation.marker` is absent OR while an existing marker's
   `expires_at` is elapsed (UTC), the dispatcher's native `block_if_marker` crash-handler finds
   no valid quarantine signal and allows the dispatch (exit_code = 0). No unconditional self-lock
   is possible: a crash with no valid quarantine signal cannot block indefinitely. This preserves
   the ADR-047 §Decision 4 core insight (avoid unconditional self-lock) while closing the
   CWE-636 gap for the crash+non-expired-marker sub-case (ADR-048 §Decision 1).

## Invariants

1. **Same WASM binary, two registrations.** Arm 1 and Arm 2 are two `[[hook]]` entries pointing
   to the same `validate-unvalidated-mutation-marker.wasm` plugin binary. The plugin reads the
   tool payload (`command` parameter for Bash dispatches) to apply the command-content filter in
   Arm 2. Plugin-internal: for Bash events, if `command` does not identify `commit` or `push` as
   the git subcommand (i.e., `is_git_commit_or_push(command)` returns `false`), the plugin
   returns `exit_code = 0` immediately (before reading the marker).

2. **Gate is fail-closed-but-recoverable on crash (block_if_marker).** Both registrations have
   `failure_policy = "fail-open"`. On a gate plugin CRASH detected at the dispatcher level (fuel
   exhaustion as WASM trap, epoch timeout as WASM trap — not normal plugin exit), the dispatcher
   executes a native `block_if_marker` check (ADR-048 §Decision 1): reads
   `.factory/unvalidated-mutation.marker`, parses `expires_at`; if a non-expired marker exists →
   Block; if marker is absent or TTL expired → Allow (PC5/PC6). This ensures real quarantine
   signals are enforced even through plugin crashes while preserving the no-marker → Allow branch
   to prevent unconditional self-lock. The self-lock hazard is bounded by three recoverability
   guarantees (INV6): marker is removable via `rm` (ungated); re-validation via Edit/Write is
   ungated; TTL auto-expiry provides passive self-healing within 86400s.
   **The fail-open posture for IO-read errors within normal gate plugin execution is UNCHANGED:**
   if the marker file exists but cannot be read by the plugin (e.g., EACCES, EPERM during normal
   non-crash execution), `guard_logic::evaluate_gate` MUST return Allow — an unreadable marker is
   an infrastructure fault ("cannot-complete") encountered during normal plugin execution, not a
   dispatcher-crash event, and the operator `rm` escape hatch may also fail under the same fault,
   creating an unclearable self-lock. This is orthogonal to the dispatcher-crash path (handled by
   `block_if_marker` native check). This case is distinct from: (a) a successfully-read but
   malformed marker (EC-008 → BLOCK); (b) dispatcher-detected plugin crash (EC-031, handled by
   `block_if_marker`; PC5/PC6). See EC-030 for the IO-read-failure-during-normal-execution case.

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
   §Decision 3 applied to the marker gate. The tightened block_if_marker crash policy (INV2/PC5)
   does not narrow the escape surface: see INV6 for the formal ungated-escape guarantee.

6. **Ungated-Escape invariant (ADR-048 §Decision 3).** The three recovery operations MUST NEVER
   be gated by this plugin's PreToolUse registrations while the system is in a quarantined
   (marker-active) state:
   - **(i) `rm .factory/unvalidated-mutation.marker` (Bash dispatch):** `is_git_commit_or_push`
     returns `false` for this command (`basename("rm") = "rm" ≠ "git"`); Arm 2 does NOT gate
     this dispatch. Arm 1 fires only on `^Agent$`; a Bash `rm` dispatch is not `^Agent$`. This
     escape is ungated by construction. Block_if_marker on crash (INV2/PC5) does not affect it
     because a Bash `rm` dispatch does not match Arm 1's `^Agent$` pattern.
   - **(ii) Edit/Write to the artifact (re-validation dispatch):** Edit and Write tool dispatches
     do not match `^Agent$` (Arm 1) or the `is_git_commit_or_push` filter (Arm 2); they are
     ungated per PC3. Neither arm applies block_if_marker to these tools. Confirmed ungated
     by construction.
   - **(iii) TTL auto-expiry (no dispatch required):** After `UNVALIDATED_MUTATION_MARKER_TTL_SECONDS`
     (86400s, defined in `crates/factory-dispatcher/src/indeterminate_marker.rs`) have elapsed
     since the INDETERMINATE event, any subsequent gate evaluation on the normal path finds
     `expires_at ≤ now (UTC)` → Allow (gate plugin auto-deletes the marker per BC-1.18.003 PC4).
     Any crash-path evaluation also finds marker expired → Allow (PC6). No operator dispatch is
     required. This escape is ungated — passive self-healing.
   Any future change to this gate's `tool` patterns or Arm 2 command filter MUST verify all
   three escape paths remain ungated after the change. This invariant is verified by VP-107
   (to be authored by test-writer after ADR-048 ratification).

## Threat Model & Scope

**Actor model.** The gate defends against a **cooperating AI agent** that may be nudged onto
an unusual or unintended execution path — for example, an agent that has confused its context
and is about to commit or push while a mutation marker is present. The gate is NOT an
adversary-resistant authorization boundary; it is NOT designed to stop a deliberately-evasive
actor constructing crafted shell invocations to evade detection. Claims that the gate provides
adversarial-boundary guarantees are evaluating the wrong property.

**Canonical gate claim language (use verbatim in all derived artifacts):**
> "A fast, fail-open, accidental-misuse interlock for a cooperating agent — NOT an
> adversary-resistant authorization boundary."

**IN SCOPE — gate blocks (committed by design):**

| Class | Examples |
|-------|---------|
| Direct invocation | `git commit -m "..."`, `git push origin main` |
| Compound commands | `git status && git commit -m "x"`, `git diff ; git push` |
| Path-prefixed / basename-matched git | `/usr/bin/git commit`, `./git push origin main` |
| Env-prefixed git | `env GIT_DIR=.git git commit -m "x"` |
| Global-option-bearing invocations | `git -C .factory commit`, `git --git-dir .git commit` |
| **QUOTED-literal subcommand or executable** (v1.3) | `git "commit"`, `git 'push' origin`, `g'i't commit` |

QUOTED-literal forms are in scope as of v1.3 via Phase 1b POSIX quote-aware tokenization
(implemented using the `shell-words` crate). Ad-hoc quote-stripping is explicitly forbidden
— the crate provides parser-equivalence with POSIX shell quoting semantics, eliminating the
parser-differential bypass class.

**OUT OF SCOPE — NOT blocked by this gate (allowed under fail-open; caught by other layers):**

Reliably blocking the forms below is UNDECIDABLE in general: a static string analysis cannot
resolve arbitrary dynamic shell expansion (Rice's theorem). These inputs are INDETERMINATE for
this filter and are ALLOWED under fail-open posture. They are caught by the durable marker
written at PostToolUse time (regardless of how the triggering dispatch arrived), the `^Agent$`
next-advance arm, and GitHub server-side branch protection on durable branches.

| Class | Examples | Rationale for exclusion |
|-------|---------|------------------------|
| Variable / alias / function indirection | `c=git; $c push`, `alias gp="git push"; gp` | Dynamic expansion; undecidable for static string analysis |
| `xargs` / command-as-string-arg indirection | `echo "git push" \| xargs bash -c`, `bash -c "git push"` | `git` is a string arg to another program, not the top-level executable |
| `eval` and `source` / `.` | `eval "git push origin main"`, `. ./git-script.sh` | Dynamic evaluation; undecidable |
| Command substitution at top level | `$(git commit -m "x")`, `` `git push` `` | `git` is not a statically-literal top-level token; the outer command is the substitution |

### Fail-Closed-But-Recoverable Design (ADR-048 §Decision 1)

The gate's crash behavior follows a fail-closed-but-recoverable design: the ONLY allow-on-failure
case is crash-with-no-(valid)-marker, which enforces nothing because there is no quarantine signal.
Every real (non-expired) quarantine signal is enforced even through plugin crashes (PC5).

Three independent recoverability guarantees ensure the system is never in an irrecoverable
quarantine state (INV6 — Ungated-Escape invariant; ADR-048 §Decision 3):

| Guarantee | Mechanism | Dispatch required? |
|-----------|-----------|-------------------|
| Marker is removable | `rm .factory/unvalidated-mutation.marker` (Bash; INV6-i) | Yes, Bash `rm` (never gated) |
| Re-validation is ungated | Edit/Write to the failing artifact (INV6-ii; PC3) | Yes, Edit/Write (never gated) |
| TTL deadman | marker `expires_at` elapses after 86400s; gate auto-deletes on next normal eval (INV6-iii) | No — passive self-healing |

The canonical gate claim language remains: "A fast, fail-open [on no-signal], accidental-misuse
interlock for a cooperating agent — NOT an adversary-resistant authorization boundary." The
block_if_marker tightening applies only to the crash+valid-quarantine-signal sub-case.

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
| EC-008 | Marker file is present and READABLE (no IO/permission error) but content is malformed or unparseable (bad TOML, missing required fields such as `plugin_name`) | Gate returns exit_code=2 (block). Scope: this EC applies ONLY when the file is successfully opened and read but its content fails parsing. The block message includes a note that the marker content could not be parsed; the operator must manually `rm` the marker (the file is readable and removable in this case). For the complementary case where the marker is present but UNREADABLE due to an IO/permission error, see EC-030 (→ ALLOW, fail-open). |
| EC-009 | Gate plugin crashes/fuel-exhausts (Arm 1 or Arm 2) — marker ABSENT or marker TTL expired | Allow (exit_code=0). Dispatcher native block_if_marker check finds no valid quarantine signal → Allow (PC6). Advisory `plugin.indeterminate` event emitted for the gate itself; no nested marker written (gate is fail-open; failure_policy = "fail-open"). |
| EC-031 | Gate plugin crashes/fuel-exhausts (Arm 1 or Arm 2) — non-expired marker present | Block (exit_code=2). Dispatcher native block_if_marker check: non-expired marker exists → Block even though WASM plugin crashed (PC5). Block message includes marker fields (`plugin_name`, `artifact_path`, `cause`, `expires_at`) + three recovery options: (a) rm; (b) Edit/Write re-validate; (c) wait for TTL expiry. |
| EC-032 | Gate plugin crashes/fuel-exhausts (Arm 1 or Arm 2) — marker present but TTL expired (expires_at ≤ now UTC) | Allow (exit_code=0). Dispatcher native block_if_marker check: expired marker treated as absent → Allow (PC6). Marker is NOT auto-deleted on the crash path (crash-handler keeps simple; auto-delete happens on next normal-path plugin execution). |
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
| EC-024 | Marker present; `git "commit"` Bash dispatch | BLOCKED. Phase 1b POSIX tokenization: `"commit"` (double-quoted) → token `commit` after quote removal. Phase 4: subcommand is `commit`. Newly in scope as of v1.3. |
| EC-025 | Marker present; `git 'push' origin` Bash dispatch | BLOCKED. Phase 1b POSIX tokenization: `'push'` (single-quoted) → token `push` after quote removal. Phase 4: subcommand is `push`. |
| EC-026 | Marker present; `g'i't commit` Bash dispatch | BLOCKED. Phase 1b POSIX tokenization: `g'i't` → token `git` (POSIX concatenates unquoted `g`, single-quoted `i`, and unquoted `t` character runs). Phase 2: `basename("git") = "git"`. Phase 4: subcommand is `commit`. |
| EC-027 | Marker present; `$(git commit -m "x")` Bash dispatch | NOT blocked. OUT OF SCOPE by design. The top-level command is a command substitution, not a statically-literal `git` invocation. Allowed under fail-open posture; caught by durable marker and other controls (see Threat Model §Out-of-scope). |
| EC-028 | Marker present; `echo x \| xargs git commit` Bash dispatch | NOT blocked. Phase 1 splits on `\|`: segment 1 = `echo x` (executable `echo` ≠ `git`, false); segment 2 = `xargs git commit` (executable `xargs` ≠ `git`, false). Any-segment returns false. OUT OF SCOPE by design — `xargs` indirection is not the statically-literal top-level `git` executable (Threat Model §Out-of-scope). |
| EC-029 | Marker present; `eval "git push origin main"` Bash dispatch | NOT blocked. OUT OF SCOPE by design. The executable is `eval`; the `git push` invocation is a string argument subject to dynamic evaluation. Allowed under fail-open posture; caught by durable marker and other controls (see Threat Model §Out-of-scope). |
| EC-030 | Marker file exists but is UNREADABLE due to an IO or permission error (e.g., EACCES, EPERM) — marker bytes are not accessible to the plugin | `guard_logic::evaluate_gate` returns Allow (exit_code=0, fail-open). Rationale grounded in INV2: an IO-read fault is a cannot-complete infrastructure failure; the gate MUST NOT block on it because (a) the marker cannot be verified as a valid quarantine signal when its bytes are unreadable, and (b) the operator `rm` escape hatch may also fail under the same permission fault, producing an unclearable self-lock — the exact failure class INV2 prohibits. This case is orthogonal to EC-008 (readable bytes, parse failure → BLOCK) and EC-009 (plugin crashes before attempting the read → ALLOW via axis-i dispatcher). |

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
| Exists | Bash `git "commit"` | Block (exit_code=2) — Arm 2; Phase 1b quote-aware tokenization, `"commit"` → `commit`; subcommand matches (EC-024) |
| Exists | Bash `git 'push' origin` | Block (exit_code=2) — Arm 2; Phase 1b quote-aware tokenization, `'push'` → `push`; subcommand matches (EC-025) |
| Exists | Bash `g'i't commit` | Block (exit_code=2) — Arm 2; Phase 1b quote-aware tokenization, `g'i't` → `git`; subcommand `commit` matches (EC-026) |
| Exists | Bash `$(git commit -m "x")` | Allow (exit_code=0) — NOT blocked; command substitution OUT OF SCOPE by design; allowed under fail-open (EC-027) |
| Exists | Bash `echo x \| xargs git commit` | Allow (exit_code=0) — NOT blocked; `xargs` indirection OUT OF SCOPE by design; executable is `xargs` not `git` (EC-028) |
| Exists | Bash `eval "git push origin main"` | Allow (exit_code=0) — NOT blocked; `eval` indirection OUT OF SCOPE by design; allowed under fail-open (EC-029) |
| Exists (UNREADABLE: EACCES on marker file) | `^Agent$` dispatch | Allow (exit_code=0) — fail-open on IO read error; marker bytes inaccessible = cannot-complete; no self-lock per INV2 (EC-030) |
| Exists (UNREADABLE: EACCES on marker file) | Bash `git commit -m "test"` | Allow (exit_code=0) — Arm 2; fail-open on IO read error; same INV2 rationale; EC-030 |
| Exists (READABLE, malformed TOML content) | `^Agent$` dispatch | Block (exit_code=2) — malformed-readable marker; bytes accessible but unparseable; block message notes unparseable content; EC-008 |
| Exists (READABLE, malformed TOML content) | Bash `git commit -m "test"` | Block (exit_code=2) — Arm 2; malformed-readable marker; EC-008 |
| Exists (non-expired, expires_at in future) | `^Agent$` dispatch — gate plugin crashes (fuel-exhaustion) | Block (exit_code=2) — dispatcher native block_if_marker: non-expired marker present → Block (PC5, EC-031) |
| Absent | `^Agent$` dispatch — gate plugin crashes (fuel-exhaustion) | Allow (exit_code=0) — dispatcher native block_if_marker: no marker → Allow (PC6, EC-009) |
| Exists (TTL expired, expires_at ≤ now UTC) | `^Agent$` dispatch — gate plugin crashes | Allow (exit_code=0) — dispatcher native check: expired marker treated as absent → Allow (PC6, EC-032) |
| Exists (non-expired) | Bash `git commit -m "test"` — gate plugin crashes | Block (exit_code=2) — Arm 2; dispatcher native block_if_marker: non-expired marker → Block (PC5, EC-031) |
| Exists (TTL expired) | Bash `git commit -m "test"` — gate plugin crashes | Allow (exit_code=0) — Arm 2; dispatcher native check: expired marker → Allow (PC6, EC-032) |

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

- VP-105 — Next-Advance Gate Blocks Agent Dispatch and git commit/push While Marker Exists, Passes When Absent; block_if_marker crash policy (integration + unit-test; covers PC1/PC2/PC3/PC4/PC5/PC6 all arms)
- VP-107 — Ungated-Escape Invariant: `rm`, Edit/Write re-validation, and TTL auto-expiry are never gated even under quarantine (unit-test; covers INV6; to be authored by test-writer after ADR-048 ratification)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-105 | Marker exists → Agent dispatch blocked (Arm 1, exit_code=2); marker absent → Agent dispatch allowed (Arm 1); manual rm unblocks; Edit not gated (PC3) | integration (bats) |
| VP-105 | Marker exists + git commit Bash → blocked (Arm 2); marker absent + git commit → allowed; git status not gated even when marker exists (PC3) | integration (bats) |
| VP-105 | guard_logic::evaluate_gate — four cases, all distinct: (1) marker absent (NotFound) → Allow; (2) marker present and readable with valid TOML content → BlockDispatch (exit_code=2); (3) marker present and readable but content malformed/unparseable (bad TOML, missing plugin_name) → BlockDispatch (exit_code=2, EC-008); (4) marker present but UNREADABLE due to IO/permission error (EACCES/EPERM) → Allow (fail-open, EC-030). Cases (3) and (4) MUST NOT be conflated: the block/allow split hinges on whether the file bytes were successfully read, not merely whether the file exists. | unit-test |
| VP-105 | guard_logic::is_git_commit_or_push (v1.3 algorithm): (1) exact subcommand — `commit`/`push` match; `status`/`log`/`diff`/`fetch`/`commit-graph` do NOT match; (2) complete arg-taking option set — each of `-C`, `-c`, `--namespace`, `--git-dir`, `--work-tree`, `--super-prefix`, `--config-env` skips its following separate-token argument before subcommand identification; (3) compound splitting — `&&`, `\|\|`, `;`, `\|`, `&`, newline each split the command into independent segments; any-segment `true` returns `true`; (4) basename matching — `/usr/bin/git commit` → blocked, `./git push` → blocked, `env GIT_DIR=x git commit` → blocked, `cat gitfile` → NOT blocked; (5) fail-safe on unrecognized options — `git --unknown-flag commit` → blocked (subcommand position uncertain); (6) POSIX quote-aware tokenization via `shell-words` crate (Phase 1b) — `git "commit"` → blocked (EC-024), `git 'push' origin` → blocked (EC-025), `g'i't commit` → blocked (EC-026); documented out-of-scope (NOT blocked by design): `$(git commit)` → NOT blocked (command substitution; EC-027), `xargs git commit` → NOT blocked (command indirection, `xargs` is the executable; EC-028), `eval "git push"` → NOT blocked (dynamic evaluation; EC-029) | unit-test |
| VP-105 | block_if_marker crash policy — dispatcher native check on plugin crash/fuel/epoch: (1) crash + non-expired marker → Block (exit_code=2, PC5, EC-031) for both Arm 1 (^Agent$) and Arm 2 (^Bash$ git commit/push); (2) crash + no marker → Allow (exit_code=0, PC6, EC-009); (3) crash + expired marker (expires_at ≤ now UTC) → Allow (exit_code=0, PC6, EC-032); (4) crash-handler I/O failures (ENOENT on marker path) → Allow (conservative; crash-handler fail-open on its own I/O, PC5 note) | unit-test |
| VP-107 | Ungated-Escape invariant (INV6): `rm .factory/unvalidated-mutation.marker` Bash dispatch → NOT gated (is_git_commit_or_push returns false; Arm 1 does not match Bash `rm`); Edit/Write dispatch → NOT gated (PC3; neither arm matches); TTL auto-expiry → gate allows without any dispatch after expires_at elapsed; none of the three recovery operations is blockable by this gate's registrations | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-041 |
| Capability Anchor Justification | CAP-041 ("Validation Integrity: INDETERMINATE Outcome, Durable Mutation Marker, and Next-Advance Gate") per capabilities.md §CAP-041 — this BC specifies the next-advance gate behavior that is the third element of what CAP-041 defines: "blocking of the next state-advancing dispatch — the `validate-unvalidated-mutation-marker` PreToolUse plugin … blocks ALL `^Agent$` tool dispatches AND all Bash dispatches whose `command` identifies `commit` or `push` as the git subcommand (D9 extended gate; the illustrative regex `\bgit\b.*\b(commit\|push)\b` approximates but is not authoritative — see v1.1 clarification) while the marker exists." |
| L2 Domain Invariants | none (dispatcher runtime gate invariant, not L2 domain spec) |
| Architecture Module | SS-04 (Plugin Ecosystem — new `validate-unvalidated-mutation-marker` WASM plugin crate); SS-01 (Hook Dispatcher Core — evaluates block_intent from plugin exit_code=2 in PreToolUse dispatch chain) |
| ADR | ADR-047 §Decision 4 (Next-Advance Gate plugin specification — two-arm registration, exit_code=2 block; crash behavior superseded by ADR-048 §Decision 1); ADR-047 §Decision 9 (extended gate scope — Agent dispatch AND git commit/push Bash arm; two-axis model); ADR-047 §Decision 5 (Marker clear protocol — rm unblocks both arms simultaneously); ADR-039 §Decision 1 (two-axis model — on_error orthogonal to failure_policy; axes-independence invariant); ADR-048 §Decision 1 (block_if_marker new on_error value — crash + non-expired marker → Block; crash + no/expired marker → Allow; supersedes ADR-047 §D4 crash posture; D-1135 reversed); ADR-048 §Decision 2 (expires_at TTL — gate plugin checks expires_at on normal path; dispatcher native check uses expires_at on crash path); ADR-048 §Decision 3 (ungated-escape invariant — rm/Edit/Write/TTL escape paths confirmed ungated; VP-107 verification) |
| Stories | S-25.01 |
| Cycle | v1.0-feature-validation-integrity-layer1 (F2 — product-owner spec burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.5 | 2026-08-31 | product-owner | ADR-048 §Decision 1/2/3 — fail-closed-but-recoverable gate redesign. (1) Both Arm 1 and Arm 2 `on_error` changed from `"continue"` to `"block_if_marker"` in PC1. (2) Two-axis model note in PC1 rewritten: axis (i) now describes the native block_if_marker crash-path (block iff non-expired marker; allow otherwise) superseding D-1135 fail-open-on-crash. (3) Added PC5: gate crash + non-expired marker → dispatcher BLOCKS. (4) Added PC6: gate crash + absent/expired marker → dispatcher ALLOWS. (5) INV2 rewritten: fail-closed-but-recoverable on crash replacing unconditional fail-open; IO-read-error fail-open (EC-030) unchanged. (6) Added INV6: Ungated-Escape invariant — `rm` + Edit/Write + TTL auto-expiry are never gated; VP-107 verification anchor. (7) Threat Model: added Fail-Closed-But-Recoverable subsection with three recoverability guarantees. (8) EC-009 updated for block_if_marker (crash+no-marker → allow); added EC-031 (crash+non-expired-marker → block) and EC-032 (crash+expired-marker → allow). (9) Canonical test vectors: added PC5/PC6 scenarios. (10) VP-105 updated to cover block_if_marker. (11) Traceability ADR: ADR-048 §D1/D2/D3 citations added. ADR-048 added to inputs. |
| 1.4 | 2026-08-31 | product-owner | Adversary MEDIUM-1 adjudication: resolved read-error vs malformed-marker self-contradiction. (1) EC-008 rewritten to scope explicitly to "marker present and READABLE but content malformed/unparseable (bad TOML, missing required fields) → BLOCK (exit_code=2)" — the unreadable-IO-error case is now excluded from EC-008. (2) Added EC-030: "marker present but UNREADABLE (IO/permission error, e.g. EACCES) → ALLOW (fail-open)" citing INV2 — an unreadable marker cannot be trusted as a quarantine signal, and rm may also fail under the same fault, creating an unclearable self-lock; fail-open is the correct posture. (3) INV2 expanded to explicitly cover the unreadable-marker self-lock avoidance case (not just fuel-exhaustion/plugin-crash). (4) VP-105 unit-test property row updated to distinguish all four cases: absent→Allow; readable+valid→Block; readable+malformed→Block (EC-008); unreadable IO error→Allow (EC-030). (5) Added four canonical test vectors: unreadable-EACCES Agent→Allow, unreadable-EACCES git commit→Allow, readable-malformed Agent→Block, readable-malformed git commit→Block. |
| 1.3 | 2026-08-31 | product-owner | Adversary ruling reconciliation (HIGH-1 + MEDIUM-2). (1) Fail-open-on-crash self-contradiction resolved: `on_error = "block"` → `on_error = "continue"` for both Arm 1 and Arm 2 registrations; added two-axis model note (ADR-039; ADR-047 §Decision 9) distinguishing dispatcher-level fail-open-on-crash from filter-internal fail-SAFE posture (Phase 3 unrecognized-option block) — these axes are orthogonal and must not be conflated. (2) Threat model bounded: actor model = cooperating AI agent, NOT adversarial exfiltrator; canonical gate-claim language established. QUOTED-literal forms (`git "commit"`, `git 'push' origin`, `g'i't commit`) added to IN SCOPE via Phase 1b POSIX quote-aware tokenization (`shell-words` crate); ad-hoc quote-stripping forbidden. OUT-OF-SCOPE class documented with Rice's theorem rationale: `eval`, variable/alias/function indirection, `xargs`/`bash -c`, command substitution. Added EC-024..EC-029 and 7 canonical test vectors (3 IN-SCOPE quoting + 4 OUT-OF-SCOPE-documented). Updated VP-105 unit-test property to v1.3 algorithm. |
| 1.2 | 2026-08-31 | product-owner | Senior-architect gate expansion: production-grade security filter closing three adversary-identified under-blocking classes. (1) Incomplete arg-taking set (adversary HIGH): defined the complete seven-option arg-taking set (`-C`, `-c`, `--namespace`, `--git-dir`, `--work-tree`, `--super-prefix`, `--config-env`) with explicit no-arg list and fail-safe posture for unrecognized options. (2) Compound commands (adversary LOW-1): Phase 1 compound splitting on `&&`, `\|\|`, `;`, `\|`, `&`, newline — any-segment match returns true. (3) Path-prefixed/basename git (adversary LOW-2): Phase 2 basename identification strips leading path components; `env VAR=x` prefix stripped. Added EC-013 through EC-023 and 10 new canonical test vectors. Updated VP-105 unit-test property row. ADR-047 §Decision 9. |
| 1.1 | 2026-08-31 | product-owner | Spec adjudication: clarify Arm 2 command filter from illustrative regex to authoritative exact-subcommand matching. The regex `\bgit\b.*\b(commit\|push)\b` false-positives on `git commit-graph write` (hyphen is a word boundary, making `\bcommit\b` match inside `commit-graph`). Authoritative rule: `is_git_commit_or_push(command)` identifies the git subcommand (first non-option token after `git` + global options) and returns true iff it is exactly `commit` or `push`. Added EC-011 (`git commit-graph write` → NOT blocked), EC-012 (`git -C path commit` → blocked), two canonical test vector rows, and updated VP-105 unit-test property to name `commit-graph` as a NOT-matched case. Updated `modified[]` frontmatter. |
| 1.0 | 2026-08-30 | product-owner | Initial creation. F2 spec-evolution burst, validation-integrity-layer1. BC-1.18.002: two-arm gate (Agent + git commit/push Bash), PC1-PC4 full coverage, Arm 2 command-pattern matching, fail-open gate posture, self-lock-hazard invariant. D9 human ratification (extended gate scope) reflected. VP-105 anchored. CAP-041 capability anchor. ADR-047 §D4/D9/D5 citations. |
