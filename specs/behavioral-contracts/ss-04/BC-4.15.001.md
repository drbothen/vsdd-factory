---
document_type: behavioral-contract
level: L3
version: "1.3"
status: draft
producer: product-owner
timestamp: 2026-06-16T00:00:00Z
last_amended: "2026-06-26 (v1.3) — fix-burst (product-owner): (SEC-002) INV5 (Secret-redaction invariant) added; PC-B-B1 and PC-B-B2 updated to reference INV5 (redact-then-truncate ordering); INV4 clarified with redact-then-truncate ordering and single-utility byte-identity guarantee; EC-014..EC-020 added (secret-bearing flag, Authorization/Bearer, Authorization=<value> assignment, clean command no-false-positive, redaction-expands-past-120 truncation still applies, both channels identical, redact-then-truncate ordering); 4 canonical test vector rows added for redaction (bearer header, flag+value, assignment, clean-no-redaction); Traceability ADR cite updated to cite SEC-002 sub-clause; VP Anchors updated to note VP-091 §6 coverage. Governs: ADR-026 §Decision 12 SEC-002 Redaction Sub-clause + VP-091 §6. [Prior: 2026-06-17 (v1.2) — fix-burst (product-owner): (F-P8-002/C-P8-001 BLOCKER) §Postconditions PC-B sub-bullets (B-1)/(B-2) promoted to formally-labeled sub-clause headings **PC-B-B1** and **PC-B-B2** so S-18.09 AC-008 gate can resolve PC-B-B1/PC-B-B2 citation tokens; Invariant 4 body citation tokens updated from (PC-B-1)/(PC-B-2) to PC-B-B1/PC-B-B2; §Changelog new row v1.2 added. [Prior: 2026-06-16 (v1.1) — micro-fix (product-owner): §Verification Properties VP-091 proof_method corrected from 'unit-test + integration' to 'unit-test' to match canonical VP-091 frontmatter + VP-INDEX classification. No behavioral content changed.]]"
phase: F3
inputs:
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "0a64afe"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-032"
lifecycle_status: draft
introduced: v1.0-feature-context-durability-E18
modified: ["1.1", "1.2", "1.3"]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.15.001: validate-heavy-op-delegation WASM gate emits advisory DelegationRecommended finding on PreToolUse Bash tool calls matching heavy-operation patterns; never blocks; pure-parse pattern matching; no filesystem or context access

## Description

`validate-heavy-op-delegation` is a native WASM plugin registered as a PreToolUse gate on `Bash` tool calls. When a Bash command string matches one of the configured heavy-operation patterns (first-match semantics), the gate emits a `DelegationRecommended` advisory finding to BOTH (a) stderr as a human/LLM-visible nudge message and (b) the dispatcher's `plugin.log` channel as a structured record. The gate ALWAYS returns `Continue` — it never sets `block_intent = true` regardless of pattern match result, crash, or any other condition. It fires only on `Bash` tool calls; all other tool types are immediate no-ops. The pattern set is registry-configurable via `[hooks.config] patterns = [...]`; the v1 default list is specified in PC1. This gate implements the advisory precursor to a possible future blocking-promotion mode (see §Future Mode). This design follows ADR-026 §Decision 12 (advisory-only in v1, never blocks) and §Decision 8 (WASM for pure-function command-string matching; no side effects).

## Preconditions

1. The plugin is registered in `hooks-registry.toml` as:
   ```toml
   [[hooks]]
   name = "validate-heavy-op-delegation"
   event = "PreToolUse"
   plugin = "hook-plugins/validate-heavy-op-delegation.wasm"
   tool = "Bash"
   on_error = "continue"
   async = false
   timeout_ms = 5000

   [hooks.config]
   patterns = [
     "cargo test --release",
     "grep -r",
     "grep -R",
     "find . -name",
     "find . -type",
     "./run-all.sh",
     "./run-bats.sh"
   ]
   ```
   The `name` and `plugin` fields MUST both be present (canonical native-WASM shape per ADR-026 §Decision 8 precedent). The `[hooks.config] patterns` list is the v1 default set; operators may override or extend it at deployment time. The gate performs first-match evaluation: the first pattern in the list that matches (as a substring of the command string) triggers the advisory emission. Subsequent patterns are not evaluated after a match.

2. The plugin WASM binary exists at `plugins/vsdd-factory/hook-plugins/validate-heavy-op-delegation.wasm`.

3. The invoking tool call is a `Bash` tool call. The gate reads only the `command` field of the PreToolUse payload — no other fields, no filesystem reads, no subprocess execution, and no context-window reads are performed. The gate is a pure-parse WASM function.

## Postconditions

**PC-A — No pattern match:**
The Bash command string does not match any pattern in the configured list. The gate returns `Continue` immediately after exhausting the pattern list. No stderr emission. No `plugin.log` record. No other side effect.

**PC-B — Pattern match:**
The Bash command string matches at least one configured pattern (first-match wins). The gate returns `Continue`. In addition, BOTH of the following advisory emissions MUST occur:

**PC-B-B1 — Stderr emission:**
A human/LLM-visible nudge message is written to stderr. The message MUST convey that the command is a heavy operation and delegation to a sub-agent or worktree is recommended. Minimum content: matched pattern, a `command_preview` (the secret-redacted THEN truncated representation — see INV5 for the redaction algorithm; see INV4 for the ≤120-character truncation rule), and the advisory recommendation text. The gate MUST NOT write to stdout (stdout is part of the dispatcher's tool-result channel).

**PC-B-B2 — plugin.log structured record:**
A structured advisory record is emitted to the dispatcher's `plugin.log` channel with the following fields:
  - `level: warn`
  - `code: DelegationRecommended`
  - `matched_pattern`: the exact pattern string from the list that triggered the match
  - `command_preview`: the secret-redacted THEN truncated representation (see INV5 for redaction; see INV4 for ≤120-character truncation). The redaction step (INV5 two-pass algorithm) is applied BEFORE the truncation step (INV4 ≤120-char rule), so the preview never contains a partial secret token at the truncation boundary.
  - `message`: human-readable delegation recommendation string (derives from the already-redacted `command_preview`; needs no separate redaction step)

Both emissions (PC-B-B1 and PC-B-B2) are REQUIRED on a pattern match. A gate that emits only to one channel is a specification violation. The `command_preview` value MUST be byte-identical in both channels — produced by a single shared utility function applying INV5 then INV4 in order.

**PC-C — Gate crash (WASM panic, fuel exhaustion, ABI violation):**
The gate fails open: the dispatcher returns `Continue` per `on_error = "continue"`. The dispatcher records a `plugin.crashed` event in its internal log. The Bash tool call proceeds unblocked. The crash does not suppress any in-flight tool execution.

**PC-D — Non-Bash tool call:**
The `tool` filter in the registry entry (`tool = "Bash"`) prevents the dispatcher from invoking the plugin on non-Bash tool calls. From the gate's perspective this is a no-op: the dispatcher does not dispatch the plugin, and `Continue` is the effective result. No emission occurs.

## Invariants

1. **Pure-parse; no filesystem, subprocess, or context access:** The WASM gate reads ONLY the `command` field from the PreToolUse payload — specifically the Bash command string. It performs NO git subprocess invocation, NO filesystem reads, NO context-window reads, and NO network calls. It is a pure function of the command string and the configured pattern list. This is the invariant that makes WASM the correct implementation choice for this gate (ADR-026 §Decision 8).

2. **Never blocks; always returns Continue:** The gate MUST return `block_intent = false` (i.e., `Continue`) under ALL conditions — pattern match, no match, and crash. Setting `block_intent = true` or returning a non-`Continue` result from within the gate's own logic is a specification violation. Promotion to blocking mode is a FUTURE architectural decision requiring F3 adversarial calibration (see §Future Mode). The `on_error = "continue"` registry field independently guarantees fail-open behavior on crash; this invariant makes the non-blocking guarantee normative for the gate's own non-crash execution paths as well.

3. **First-match; deterministic pattern evaluation:** Patterns are evaluated in the order they appear in the `[hooks.config] patterns` list. The gate stops at the first matching pattern and emits the advisory. Only one `DelegationRecommended` advisory is emitted per invocation, regardless of how many patterns would match the command string. Pattern matching is substring containment (case-sensitive): a pattern P matches command C if `C.contains(P)`. The matching logic is deterministic — given the same command string and the same pattern list, the gate produces the same output on every invocation.

4. **command_preview ≤120-character truncation is invariant (applied AFTER redaction):** The `command_preview` field in BOTH the stderr message (PC-B-B1) and the plugin.log record (PC-B-B2) MUST be the result of applying the INV5 secret-redaction algorithm FIRST, then truncating the redacted string to ≤120 characters. Ordering is mandatory: redaction (INV5) → truncation (INV4). If the post-redaction string is ≤120 characters, the preview equals the full post-redaction string. If the post-redaction string exceeds 120 characters, the preview is the first 120 characters of the redacted string followed by the ellipsis character `…` (U+2026). This truncation is applied identically in both emission channels via a single shared utility function, ensuring byte-identical `command_preview` values across both channels. Because `***REDACTED***` (15 chars) may be longer than the secret it replaces, the post-redaction string may exceed 120 characters even when the original command did not — truncation after redaction guarantees the preview is always ≤120 chars and never contains a partial secret token at the truncation boundary.

5. **Secret-redaction; pure-function; no-regex; enumerable trigger list (SEC-002):** The `command_preview` field in BOTH emission channels (PC-B-B1 stderr and PC-B-B2 plugin.log) MUST have secrets redacted before the preview string is constructed or truncated. Redaction occurs BEFORE truncation (redact-then-truncate ordering mandate per ADR-026 §Decision 12 SEC-002 Redaction Sub-clause). The algorithm is a hard-coded two-pass no-regex procedure, non-configurable at runtime:

   **Pass 1 — flag-argument masking:** Scan the command string for flag tokens from the following hard-coded trigger flag list. Each trigger is matched by exact case-sensitive prefix equality against whitespace-delimited tokens.

   Trigger flag list:
   - `--token`
   - `--password`
   - `--secret`
   - `--api-key`
   - `--auth`
   - `--bearer`
   - `--credential`
   - `--private-key`
   - `--access-key`
   - `--secret-key`
   - `--key`

   For each trigger flag found, replace the immediately following whitespace-delimited token (the argument value) with `***REDACTED***`. If the flag and its value are joined by `=` (e.g., `--token=abc123`), replace the portion after `=` with `***REDACTED***` (yielding `--token=***REDACTED***`). If the trigger flag appears at the end of the command string with no following argument, it is left as-is (no argument to redact).

   **Pass 2 — inline assignment and Authorization header masking:** After Pass 1, scan the resulting string for the following patterns and replace the value portion with `***REDACTED***`:
   - Any `Authorization: Bearer <value>` or `Authorization: <scheme> <value>` substring: replace the value token(s) following `Authorization:` with `***REDACTED***`.
   - Any `Authorization=<value>` assignment (e.g., in query strings): replace `<value>` with `***REDACTED***`.

   Pass 2 is applied to the post-Pass-1 string. The two passes are sequential and non-iterating.

   **Mask token:** The literal string `***REDACTED***` (15 characters, all ASCII). Hard-coded and not configurable.

   **Non-configurable trigger list:** The trigger list is a compile-time constant in the WASM plugin source. It MUST NOT be made configurable at runtime (no config-file override, no `hooks-registry.toml` config key). Extending it requires a source code change and a new release.

   **False-negative acknowledgment:** This algorithm does not detect all possible secret forms. It does not parse environment variable assignments (e.g., `MY_SECRET=abc cmd`), positional arguments, or secrets embedded in subshell expansions (`$(cat /secret)`). These cases are acknowledged out-of-scope for v1. Over-mask (masking a non-secret flag) is the accepted trade-off over under-mask (leaking a secret).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Bash command is `cargo test --release --workspace`; pattern `cargo test --release` is in the list | Pattern matches (substring); DelegationRecommended advisory emitted to stderr + plugin.log; gate returns Continue |
| EC-002 | Bash command is `cargo test --workspace`; pattern list contains `cargo test --release` but NOT `cargo test --workspace` | No match; gate returns Continue silently (PC-A) |
| EC-003 | Bash command is `grep -r "pattern" .`; pattern `grep -r` is in the list | Pattern matches; DelegationRecommended advisory emitted; gate returns Continue |
| EC-004 | Bash command is `grep -R "pattern" .`; pattern `grep -R` is in the list | Pattern matches; DelegationRecommended advisory emitted; gate returns Continue |
| EC-005 | Bash command is `find . -name "*.rs"`; pattern `find . -name` is in the list | Pattern matches; DelegationRecommended advisory emitted; gate returns Continue |
| EC-006 | Bash command is `./run-all.sh`; pattern `./run-all.sh` is in the list | Pattern matches; DelegationRecommended advisory emitted; gate returns Continue |
| EC-007 | Bash command is `cargo fmt --check --all`; no pattern in the default list matches this command | No match; gate returns Continue silently (PC-A); no emission |
| EC-008 | Bash command string is 300 characters long and matches `grep -r`; command_preview is truncated to first 120 chars + `…` | Advisory emitted with 121-character command_preview field (`120 chars + U+2026`); gate returns Continue |
| EC-009 | Gate crashes (WASM panic mid-pattern-scan) | fail-open Continue; `plugin.crashed` in dispatcher internal log; Bash tool call proceeds |
| EC-010 | A Write tool call is issued (not a Bash tool call) | Plugin is not dispatched by the registry `tool = "Bash"` filter; effective Continue (PC-D) |
| EC-011 | Multiple patterns from the list would match the command string | First-match semantics: only the first matching pattern triggers advisory emission; single `DelegationRecommended` record emitted; gate returns Continue |
| EC-012 | Operator configures an empty `patterns = []` list | No pattern can ever match; all Bash commands pass silently (PC-A); no advisory emitted for any command |
| EC-013 | Operator adds a custom pattern `./ci.sh` to the patterns list | Custom pattern is evaluated in list order; `./ci.sh` commands trigger DelegationRecommended advisory; gate returns Continue |
| EC-014 | Bash command contains a trigger flag followed by a secret value, e.g., `grep -r . --token abc123secret`; command matches heavy-op pattern `grep -r` | INV5 Pass 1 masks the secret: `command_preview` in BOTH stderr (PC-B-B1) and plugin.log (PC-B-B2) contains `--token ***REDACTED***`; raw secret `abc123secret` is absent from both channels; gate returns Continue with DelegationRecommended advisory emitted |
| EC-015 | Bash command contains an Authorization/Bearer header, e.g., `grep -r . -H "Authorization: Bearer eyJtoken123"`; command matches `grep -r` | INV5 Pass 2 masks the bearer token: `command_preview` contains `Authorization: Bearer ***REDACTED***`; raw token absent from both emission channels; gate returns Continue with advisory |
| EC-016 | Bash command contains an `Authorization=<value>` assignment in a query string or environment-style context, e.g., `grep -r . --header "Authorization=secretval"`; command matches `grep -r` | INV5 Pass 2 masks the assignment value: `command_preview` contains `Authorization=***REDACTED***`; raw `secretval` absent from both channels; gate returns Continue with advisory |
| EC-017 | Bash command contains no trigger flags and no Authorization patterns, e.g., `grep -r TODO . --include="*.rs"`; command matches `grep -r` | INV5 two-pass redaction produces no substitutions; `command_preview` is identical to the pre-redaction command string (subject only to INV4 truncation); no `***REDACTED***` token appears in either channel; no false-positive masking; gate returns Continue with advisory |
| EC-018 | Bash command is 95 characters long, contains `--token abc123` (15 chars secret), and matches `grep -r`; after INV5 redaction the string becomes 111 characters (secret replaced with `***REDACTED***` = 15 chars, net +2), still ≤120 | INV4 truncation is applied to the 111-char post-redaction string; preview equals the full 111-char redacted string (no truncation needed); `command_preview` contains `***REDACTED***`; gate returns Continue with advisory |
| EC-019 | Bash command is 115 characters long, contains `--token a` (1-char secret), and matches `grep -r`; after INV5 redaction the string becomes 129 characters (secret `a` replaced with `***REDACTED***` = 15 chars, net +14), exceeding 120 | INV4 truncation is applied to the 129-char post-redaction string: `command_preview` is the first 120 chars of the redacted string + `…`; the partial secret is never exposed because redaction precedes truncation; gate returns Continue with advisory |
| EC-020 | Any Bash command where redaction precedes truncation in the implementation | Redaction (INV5) MUST be applied before truncation (INV4) in all code paths. The `command_preview` field MUST be computed as: apply-INV5-redaction → apply-INV4-truncation → emit to both channels. An implementation that truncates first and then redacts violates INV5 and may partially expose a secret that straddles the 120-character boundary |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Bash command: `cargo test --release` | Continue; stderr advisory + plugin.log `DelegationRecommended` with `matched_pattern: "cargo test --release"`; `command_preview: "cargo test --release"` | happy-path-match |
| Bash command: `cargo test --workspace` | Continue; no emission | no-match-workspace-test |
| Bash command: `grep -r "TODO" .` | Continue; stderr advisory + plugin.log `DelegationRecommended` with `matched_pattern: "grep -r"`; `command_preview: "grep -r \"TODO\" ."` | happy-path-grep-r |
| Bash command: `grep -R "TODO" .` | Continue; stderr advisory + plugin.log `DelegationRecommended` with `matched_pattern: "grep -R"` | happy-path-grep-R |
| Bash command: `find . -name "*.wasm"` | Continue; stderr advisory + plugin.log `DelegationRecommended` with `matched_pattern: "find . -name"` | happy-path-find-name |
| Bash command: `find . -type f` | Continue; stderr advisory + plugin.log `DelegationRecommended` with `matched_pattern: "find . -type"` | happy-path-find-type |
| Bash command: `./run-all.sh` | Continue; stderr advisory + plugin.log `DelegationRecommended` with `matched_pattern: "./run-all.sh"` | happy-path-run-all |
| Bash command: `./run-bats.sh` | Continue; stderr advisory + plugin.log `DelegationRecommended` with `matched_pattern: "./run-bats.sh"` | happy-path-run-bats |
| Bash command: `cargo fmt --check --all` | Continue; no emission (no pattern matches) | no-match-fmt |
| Bash command: 200-character string containing `grep -r` | Continue; advisory emitted; `command_preview` = first 120 chars + `…` (121-char field) | command-preview-truncation |
| WASM panic during pattern scan | Continue; `plugin.crashed` in dispatcher log; no advisory emitted | crash-fail-open |
| Write tool call (not Bash) | Plugin not dispatched (registry `tool = "Bash"` filter); effective Continue | non-bash-no-op |
| patterns list is `[]` (empty); any Bash command | Continue; no emission | empty-pattern-list |
| Bash command: `grep -r . -H "Authorization: Bearer supersecrettoken123"` | Continue; DelegationRecommended advisory; `command_preview` in plugin.log and stderr contains `Authorization: Bearer ***REDACTED***`; raw `supersecrettoken123` absent from both channels | redaction-bearer-header |
| Bash command: `grep -r . --token supersecrettoken123` | Continue; DelegationRecommended advisory; `command_preview` in plugin.log and stderr contains `--token ***REDACTED***`; raw `supersecrettoken123` absent from both channels | redaction-trigger-flag-value |
| Bash command: `grep -r . -H "Authorization=supersecretval"` | Continue; DelegationRecommended advisory; `command_preview` in plugin.log and stderr contains `Authorization=***REDACTED***`; raw `supersecretval` absent from both channels | redaction-assignment-value |
| Bash command: `grep -r TODO . --include="*.rs"` (no trigger flags, no Authorization patterns) | Continue; DelegationRecommended advisory; `command_preview` is unmodified (no `***REDACTED***` token); no false-positive redaction | redaction-clean-no-redaction |

## Related BCs

- BC-4.14.001 — sibling: validate-wave-handoff-completeness WASM gate; shares the same PreToolUse/PostToolUse WASM gate pattern, pure-parse invariant, and fail-open crash behavior; the canonical TOML registration shape (name + plugin fields) used in BC-4.14.001 PC1 is the source of truth for PC1 of this BC
- BC-4.13.001 — sibling: verify-factory-lock WASM PreToolUse guard; shares async=false, timeout_ms=5000, on_error=continue, and fail-open crash invariant

## Architecture Anchors

- `crates/hook-plugins/validate-heavy-op-delegation/` — NEW Rust crate; `[[bin]]`-bearing; produces `validate-heavy-op-delegation.wasm`
- `plugins/vsdd-factory/hooks-registry.toml` — `[[hooks]]` entry for `validate-heavy-op-delegation`; PreToolUse; tool=Bash; on_error=continue; async=false; timeout_ms=5000; `[hooks.config] patterns = [...]`
- ADR-026 §Decision 12 (advisory-only in v1; never blocks; F3 calibration before blocking-promotion) and §Decision 8 (WASM for pure-function command-string matching; no filesystem or git side effects)

## Story Anchor

S-18.06 (validate-heavy-op-delegation WASM gate crate + registry; advisory mode)

## VP Anchors

- VP-091 — verifies: gate always returns Continue; advisory emitted on match; never blocks; pure-parse; non-Bash no-op; command_preview ≤120-char truncation (Invariant 4); secret-redaction (Invariant 5 / SEC-002; VP-091 §6 PC6a + PC6b + EC-020 ordering invariant)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-091 | validate-heavy-op-delegation returns Continue under all conditions (match, no-match, crash); emits `DelegationRecommended` advisory to both stderr and plugin.log on first pattern match; emits nothing on no-match; is a no-op on non-Bash tool calls; performs no filesystem, subprocess, or context reads; `command_preview` is secret-redacted (INV5 two-pass algorithm) then truncated ≤120 chars (INV4); redaction is byte-identical across both channels; raw secrets absent from both channels (VP-091 §6 PC6a + PC6b) | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC specifies the advisory WASM delegation gate that nudges orchestrators and agents to offload heavy operations (large-output cargo test runs, recursive grep traversals, full bats suite) to sub-agents or worktrees rather than executing them in the primary orchestrator context window. By reducing context-window pressure from heavy ops, the gate is a preventive-advisory measure that supports the CAP-032 continuity guarantee: a context window that is not saturated by heavy-op output is less likely to trigger an uncoordinated auto-compaction event that loses load-bearing pipeline state. This BC anchors to CAP-032 as a preventive-advisory measure per OQ-4.4 (adopted), NOT as a direct enforcer of HANDOFF.md integrity (that is BC-4.14.001's role). |
| L2 Domain Invariants | DI-020 (Wave/phase boundary transitions must not lose load-bearing pipeline state) — this gate is a PREVENTIVE-ADVISORY measure that reduces the probability of context-window saturation triggering an uncoordinated mid-wave compaction event; it does NOT directly enforce DI-020 (no blocking behavior) but supports it by reducing heavy-op context pressure (per OQ-4.4 adopted resolution) |
| Architecture Module | SS-04 (Plugin Ecosystem) — new WASM crate under `crates/hook-plugins/validate-heavy-op-delegation/` |
| ADR | ADR-026 §Decision 12 (validate-heavy-op-delegation advisory-only in v1; emits advisory to stderr; never blocks; F3 adversarial calibration before blocking-promotion; S-18.06 correctly scoped as advisory-only; §Decision 12 SEC-002 Redaction Sub-clause governs INV5 secret-redaction: redact-then-truncate ordering mandate; two-pass no-regex algorithm; hard-coded trigger list; mask token `***REDACTED***`; non-configurable), §Decision 8 (WASM for pure-function command-string pattern matching; no filesystem or git side effects; timeout_ms=5000) |
| Stories | S-18.06 |
| Cycle | v1.0-feature-context-durability-E18 (F3) |
| Feature | issue #173 / E-18 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.3 | 2026-06-26 | product-owner | SEC-002: INV5 (Secret-redaction; pure-function; no-regex; enumerable trigger list) added. PC-B-B1 and PC-B-B2 updated to define `command_preview` as the secret-redacted THEN truncated representation (referencing INV5). INV4 updated to clarify redact-then-truncate ordering, byte-identical across both channels via single shared utility. EC-014..EC-020 added: EC-014 (secret-bearing flag masked in both channels), EC-015 (Authorization/Bearer header masked), EC-016 (Authorization=<value> assignment masked), EC-017 (clean command unchanged/no false-positive), EC-018 (redaction expands string but stays ≤120 — no truncation triggered), EC-019 (redaction expands string past 120 — truncation still applies; partial secret not exposed), EC-020 (redact-then-truncate ordering invariant). 4 canonical test vector rows added: bearer-header redaction, trigger-flag redaction, assignment-value redaction, clean-no-redaction. Traceability ADR row updated to cite ADR-026 §Decision 12 SEC-002 Redaction Sub-clause (governs INV5 algorithm). VP-091 Verification Properties row extended to cover §6 PC6a + PC6b. VP Anchors updated to note VP-091 §6. Governing refs: SEC-002 + ADR-026 §Decision 12 SEC-002 Redaction Sub-clause + VP-091 §6. |
| v1.2 | 2026-06-17 | product-owner | (F-P8-002/C-P8-001 BLOCKER) §Postconditions PC-B sub-bullets (B-1)/(B-2) promoted to formally-labeled sub-clause headings **PC-B-B1 — Stderr emission:** and **PC-B-B2 — plugin.log structured record:** so that citation tokens `PC-B-B1` and `PC-B-B2` are gate-resolvable as bold headings in the Postconditions section (S-18.09 AC-008 gate regex `(^|\*\*)PC-B-B1(\*\*|[: ])` now matches). Invariant 4 body updated from `(PC-B-1)/(PC-B-2)` to `(PC-B-B1)/(PC-B-B2)`. Final `Both emissions` sentence updated from `(B-1 and B-2)` to `(PC-B-B1 and PC-B-B2)`. No behavioral content changed. |
| v1.1 | 2026-06-16 | product-owner | Micro-fix: §Verification Properties VP-091 proof_method corrected from "unit-test + integration" to "unit-test" to match canonical VP-091 frontmatter + VP-INDEX classification. No behavioral content changed. |
| v1.0 | 2026-06-16 | product-owner | Initial creation per F3 OQ-4 human directive. Advisory-only validate-heavy-op-delegation WASM gate (ADR-026 §Decision 12 + §Decision 8; CAP-032; S-18.06). Pure-parse Invariant 1; never-blocks Invariant 2; first-match deterministic Invariant 3; command_preview ≤120-char Invariant 4. PC-A/PC-B/PC-C/PC-D postconditions. PC1 canonical TOML registry block (name + plugin + PreToolUse + tool=Bash + on_error=continue + async=false + timeout_ms=5000 + [hooks.config] patterns v1 defaults). PC-B dual-channel advisory (stderr nudge B-1 + plugin.log structured record B-2; code: DelegationRecommended). EC-001..EC-013 edge cases. 13-row test vector table. DI-020 preventive-advisory anchor (OQ-4.4 adopted). §Future Mode blocking-promotion non-normative sketch. |

---

## Future Mode (Non-Normative)

> **This section is NON-NORMATIVE.** It describes a POSSIBLE future blocking-promotion upgrade path. No behavioral postcondition in this section is currently enforceable. The normative behavior of this BC is advisory-only (Invariant 2: never blocks). A separate BC amendment or replacement BC will be required to make blocking mode normative.

If F3 adversarial calibration (ADR-026 §Decision 12) demonstrates that the false-positive rate on the v1 default pattern list is acceptable, the gate may be promoted to blocking mode via an **amend-not-replace** upgrade: the existing BC-4.15.001 is amended (new version row in §Changelog) with updated postconditions and invariants; BC-4.15.001 is NOT retired and replaced with a new ID (per POLICY 1 append-only / BC lifecycle rules).

### Sketched blocking-mode postconditions (non-normative)

In blocking mode, PC-B would be replaced with:

**PC-B (blocking) — Pattern match:**
The Bash command string matches a configured heavy-operation pattern (first-match). The gate returns `block_intent = true`, exit code 2, with a structured block message. Example block message format:
```
DelegationRequired: command matches heavy-operation pattern "<matched_pattern>".
Preview: <command_preview>
Recommendation: delegate this operation to a sub-agent or background worktree to
prevent orchestrator context-window saturation. Use /vsdd-factory:worktree-manage
or dispatch via the Agent tool with run_in_background=true.
```

Invariant 2 would be replaced with: "Blocks on match; fails open on crash."

### Promotion gating

Blocking-mode promotion requires ALL of the following (per ADR-026 §Decision 12):
1. F3 adversarial calibration completes with false-positive rate documented across ≥5 real production pipeline sessions.
2. Human explicitly authorizes promotion (not AI-directed).
3. BC-4.15.001 is amended by product-owner with updated normative postconditions and new §Changelog row.
4. VP-091 is updated to verify blocking behavior.
5. The registry entry `on_error = "continue"` remains (fail-open on crash is retained even in blocking mode).
