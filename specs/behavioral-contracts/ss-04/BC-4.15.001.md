---
document_type: behavioral-contract
level: L3
version: "1.6"
status: draft
producer: product-owner
timestamp: 2026-06-16T00:00:00Z
last_amended: "2026-06-26 (v1.6) — fix-burst (product-owner): (O-1 Pass-1 double-dash clarification) INV5 Pass 1 trigger-scope sentence disambiguated: changed from 'tokens starting with `-`' to 'tokens starting with `--` (double-dash long flag tokens)'; step 1 updated to 'strip the leading `--`'; closing carve-out rewritten to explain single-dash tokens are NOT processed by Pass 1 (real-world secret flags use `--token`/`--password`; single-dash long-name secret flags outside scoped threat model). Spec-text clarification only; no behavior change. [Prior: 2026-06-26 (v1.5): F-RD2-001/F-RD2-002 Pass 3 bounded-consumption. INV5 Pass 3 replaced with bounded-consumption rule per ADR-026 §Decision 12 SEC-002 Redaction Sub-clause (ADR v1.29) + VP-091 §6 PC6d (VP v1.5). Form A (inline value in same token): mask after `:` within token; stop — no further-token consumption. Form B (following-token value): mask immediate next token(s); stop at first token starting with `-` (b1-CLI flag), containing `://` (b2-URL), or closing a quoted value (b3). At minimum the immediate next token is always masked. Malformed/unbalanced-quote fails safe (stops at b1/b2 or end-of-tokens; tail NOT swallowed). EC-016 annotation updated for Form A/B distinction. EC-022..EC-025 added (append-only): Bearer+tok+URL (URL preserved); Bearer+tok+--verbose (flag preserved); unbalanced-quote (stops at flag/end); quoted -H Form A regression. Test vectors: 4 bounded-consumption rows added. Changelog v1.5 row. [Prior: 2026-06-26 (v1.4): 4-pass design. [Prior: 2026-06-26 (v1.3): two-pass (superseded). [Prior: 2026-06-17 (v1.2): PC-B-B1/PC-B-B2 headings. [Prior: 2026-06-16 (v1.1): VP-091 proof_method fix.]]]]"
phase: F3
inputs:
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "2135855"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-032"
lifecycle_status: active
introduced: v1.0-feature-context-durability-E18
modified: ["1.1", "1.2", "1.3", "1.4", "1.5", "1.6"]
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

4. **command_preview ≤120-character truncation is invariant (applied AFTER redaction):** The `command_preview` field in BOTH the stderr message (PC-B-B1) and the plugin.log record (PC-B-B2) MUST be the result of applying the INV5 secret-redaction algorithm FIRST, then truncating the redacted string to ≤120 characters. Ordering is mandatory: redaction (INV5) → truncation (INV4). If the post-redaction string is ≤120 characters, the preview equals the full post-redaction string. If the post-redaction string exceeds 120 characters, the preview is the first 120 characters of the redacted string followed by the ellipsis character `…` (U+2026). This truncation is applied identically in both emission channels via a single shared utility function, ensuring byte-identical `command_preview` values across both channels. Because `***REDACTED***` (14 chars) may be longer or shorter than the secret it replaces, the post-redaction string length may differ from the original — truncation after redaction guarantees the preview is always ≤120 chars and never contains a partial secret token at the truncation boundary.

5. **Secret-redaction; 4-pass best-effort advisory; pure-function; no-regex; enumerable trigger lists (SEC-002):** The `command_preview` field in BOTH emission channels (PC-B-B1 stderr and PC-B-B2 plugin.log) MUST have secrets redacted before the preview string is constructed or truncated. Redaction occurs BEFORE truncation (redact-then-truncate ordering mandate per EC-020 / ADR-026 §Decision 12 SEC-002 Redaction Sub-clause).

   **Best-effort advisory framing (not a security boundary):** This redaction mechanism is a best-effort advisory measure. It is NOT a security boundary. It reduces the probability that common high-risk patterns appear in advisory records; false negatives are acknowledged below. Operators requiring hard secret isolation must not rely solely on this mechanism.

   The algorithm is hard-coded in the WASM plugin source using pure standard string operations (no regex engine). It MUST NOT be made configurable at runtime. All trigger keyword lists are compile-time constants; extending them requires a source code change and a new release. The four passes are applied sequentially; each pass operates on the output of the previous pass; passes are non-iterating (single scan each).

   **Pass 1 — flag-argument masking:** Split the command string on ASCII whitespace. For each token starting with `--` (a double-dash long flag token — single-dash tokens such as `-p`, `-u`, `-t`, and any single-dash long names are NOT processed by Pass 1):
   1. Strip the leading `--` to get the flag name. If the name contains `=`, the name is the portion before the first `=`.
   2. Lowercase the flag name.
   3. If the lowercased name **equals or contains** any of the following trigger substrings, it is a secret-bearing flag:

      Trigger substrings: `password`, `passwd`, `token`, `secret`, `api-key`, `apikey`, `api_key`, `client-secret`, `auth-token`, `access-token`, `access-key`, `secret-key`, `credential`, `passphrase`, `private-key`

   4. Mask the secret value:
      - **`--flag value` form:** replace the immediately following whitespace-delimited token with `***REDACTED***`.
      - **`--flag=value` form:** replace the portion after the first `=` with `***REDACTED***` (yielding `--flag=***REDACTED***`).
      - If the flag appears at the end of the command string with no following token, leave it as-is (no value to redact).

   **Bare `--key` is NOT a trigger** (`sort --key`, `cut --key`, and similar standard utilities use `--key` non-secretly; `key` alone does not appear in the trigger substring list). Single-dash tokens (`-p`, `-u`, `-t`, and all other single-dash forms) are NOT processed by Pass 1 — real-world secret flags use double-dash long names; there are no single-dash long-name secret flags in the scoped threat model.

   **Pass 2 — env-var assignment-prefix masking:** For tokens at the start of the command that match the pattern `IDENT=value` (assignment tokens appearing before the first non-assignment command token), where `IDENT` uppercased **contains** one of the following compound trigger substrings:

   Compound triggers: `PASSWORD`, `PASSWD`, `SECRET`, `TOKEN`, `APIKEY`, `API_KEY`, `ACCESS_KEY`, `PRIVATE_KEY`, `AUTH_TOKEN`, `CLIENT_SECRET`, `CREDENTIAL`, `PASSPHRASE`

   Bare `KEY` alone is NOT a trigger — this prevents matching `PUBLIC_KEY`, `PRIMARY_KEY`, `FOREIGN_KEY`, and similar non-secret identifiers.

   **Allowlist — never masked regardless of trigger match (check allowlist BEFORE trigger heuristic):**
   - `SSH_AUTH_SOCK`
   - `SSH_ASKPASS`
   - Any identifier ending in `_SERVICE_HOST` (e.g., `KUBERNETES_SERVICE_HOST`)

   For a matching assignment token (after allowlist check), replace the value portion after the first `=` with `***REDACTED***`, yielding `IDENT=***REDACTED***`.

   **Pass 3 — Authorization/Cookie header masking (bounded consumption):** For any whitespace-delimited token whose lowercased form starts with `authorization:`, `cookie:`, or `set-cookie:`, apply the following bounded-consumption rule. Covers all Authorization schemes (Basic, Bearer, ApiKey, etc.) uniformly without scheme enumeration.

   **Form A — inline value** (content exists after the `:` within the same token, e.g. `Authorization:Bearertoken` or a single quoted token `"Authorization: Bearer eyJtoken"`): mask everything after the first `:` within that token, yielding `HeaderName:***REDACTED***`. Stop — do NOT consume any further tokens.

   **Form B — following-token value** (the `:` is at the end of the header token; the value is in subsequent whitespace-delimited tokens, e.g. `Authorization: Bearer eyJtoken`): mask subsequent tokens, but bound the consumption — STOP (do NOT consume) at the FIRST subsequent token that meets any of the following stop conditions:
   - **(b1)** The token starts with `-` (a CLI flag, e.g. `--include`, `-H`, `--verbose`).
   - **(b2)** The token contains `://` (a URL or positional endpoint, e.g. `https://api.example.com`).
   - **(b3)** [Quoted-value form] The token contains the closing quote character (`"` or `'`) that matches the opening quote of the header value — consume up to and including that closing-quote token, then stop.

   The immediate next token IS always masked when it is neither a b1-flag nor a b2-URL token (this ensures a two-token value `Bearer <credential>` has both the scheme token and the credential token masked). Consumption stops as soon as any stop condition fires.

   **Malformed / unbalanced-quote input (fail-safe):** If a quoted header value opens with `"` or `'` but no matching closing quote is encountered before end-of-tokens, consumption stops at the first b1-flag or b2-URL token (whichever comes first), or at end-of-tokens — the command tail is NOT swallowed.

   **Pass 4 — inline URL credential masking:** For any whitespace-delimited token containing `://`: if there is an `@` character after the `://` and before the next `/` (or end of token), mask the userinfo substring between `://` and `@` (the `user:pass` portion) with `***REDACTED***`, yielding `scheme://***REDACTED***@host/path`. Covers `https://user:pass@host`, `postgres://user:pass@host/db`, and equivalent URL forms.

   **Mask token:** The literal string `***REDACTED***` — exactly **14 ASCII characters** (`***` = 3, `REDACTED` = 8, `***` = 3; total = 14). Hard-coded and not configurable.

   **Provider-prefix shape detection deferred (documented future enhancement):** Detection of provider-specific secret shapes (`sk-` OpenAI prefix, `AKIA` AWS access key prefix, `ghp_` GitHub PAT prefix) is explicitly out of scope for v1. The keyword-based passes above catch the most common leakage vectors for these providers indirectly. Provider-prefix detection may be added in a future release as a separate named pass.

   **False-negative acknowledgment:** Undetected patterns include: positional arguments containing secrets, secrets in subshell expansions (`$(cat /secret)`), secrets in here-documents or multi-line strings, and env-var assignments with non-standard naming (e.g., `MY_SPECIAL_CRED=abc`). These are acknowledged out-of-scope for v1.

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
| EC-014 | Bash command contains a Pass 1 trigger flag followed by a secret value, e.g., `grep -r . --token abc123secret`; command matches heavy-op pattern `grep -r` | INV5 Pass 1 masks the secret: `command_preview` in BOTH stderr (PC-B-B1) and plugin.log (PC-B-B2) contains `--token ***REDACTED***`; raw secret `abc123secret` is absent from both channels; gate returns Continue with DelegationRecommended advisory |
| EC-015 | Bash command begins with an env-var assignment whose IDENT contains a Pass 2 compound trigger, e.g., `API_KEY=sk-abc123 grep -r .`; command matches `grep -r` | INV5 Pass 2 masks the assignment value: `command_preview` contains `API_KEY=***REDACTED***`; raw `sk-abc123` is absent from both channels; gate returns Continue with advisory |
| EC-016 | Bash command contains an Authorization header in a quoted CLI argument (Form A inline value), e.g., `grep -r . -H "Authorization: Bearer eyJtoken123"`; command matches `grep -r` | INV5 Pass 3 Form A: the quoted token `"Authorization: Bearer eyJtoken123"` has content after `:` within the same token → masked to `Authorization:***REDACTED***`; no further tokens consumed; trailing command arguments preserved; raw token absent from both channels; gate returns Continue with advisory |
| EC-017 | Bash command contains an inline URL credential, e.g., `./run-all.sh https://user:pass@example.com/db`; command matches `./run-all.sh` | INV5 Pass 4 masks the userinfo: `command_preview` contains `https://***REDACTED***@example.com/db`; raw `user:pass` absent from both channels; gate returns Continue with advisory |
| EC-018 | Bash command contains no Pass 1–4 trigger patterns, e.g., `grep -r TODO . --include="*.rs"`; command matches `grep -r` | All four INV5 redaction passes produce no substitutions; `command_preview` is identical to the pre-redaction command string (subject only to INV4 truncation); no `***REDACTED***` token appears in either channel; no false-positive masking; gate returns Continue with advisory |
| EC-019 | Bash command begins with `SSH_AUTH_SOCK=/tmp/agent.1 ssh-add` (allowlist identifier); command matches no heavy-op pattern but allowlist check is validated here | INV5 Pass 2 allowlist check precedes trigger heuristic: `SSH_AUTH_SOCK` is in the allowlist; value `/tmp/agent.1` is NOT masked; `command_preview` shows the original assignment; even though `SOCK` does not contain a compound trigger, the allowlist guarantee is absolute |
| EC-020 | Bash command contains `sort --key 3,3 file.txt`; command matches no default heavy-op pattern but `--key` presence is validated here | INV5 Pass 1: `key` alone is NOT in the trigger substring list; `--key` is not a secret-bearing flag; `command_preview` is NOT redacted for `--key`; the argument value `3,3` appears in preview; no false-positive masking |
| EC-021 | Bash command is 115 characters long, contains `--token a` (1-char secret), and matches `grep -r`; after INV5 redaction the string becomes 128 characters (secret `a` replaced with `***REDACTED***` = 14 chars, net +13), exceeding 120 | INV4 truncation is applied to the 128-char post-redaction string: `command_preview` is the first 120 chars of the redacted string + `…`; the partial secret is never exposed because redaction precedes truncation (EC-020 → EC-021 renumbered to preserve append-only numbering); gate returns Continue with advisory |
| EC-022 | Bash command has an unquoted Form B Authorization header followed by a URL: `curl Authorization: Bearer tok https://api.example.com`; command matches no default heavy-op pattern but Pass 3 bounded-consumption is validated here | INV5 Pass 3 Form B: `Bearer` and `tok` are masked (immediate next token + credential token, neither is b1-flag nor b2-URL); `https://api.example.com` is a b2-stop token (contains `://`) — NOT consumed/masked; `command_preview` contains `Authorization: ***REDACTED*** https://api.example.com`; URL preserved; raw credential absent from both channels |
| EC-023 | Bash command has an unquoted Form B Authorization header followed by a CLI flag: `curl Authorization: Bearer tok --verbose`; command matches no default heavy-op pattern | INV5 Pass 3 Form B: `Bearer` and `tok` masked; `--verbose` is a b1-stop token (starts with `-`) — NOT consumed/masked; `command_preview` contains `Authorization: ***REDACTED*** --verbose`; flag preserved; raw credential absent from both channels |
| EC-024 | Bash command has an unbalanced-quote Authorization header: `grep -r . -H "Authorization: Bearer tok --flag`; command matches `grep -r`; no closing `"` present | INV5 Pass 3 malformed/unbalanced-quote fail-safe: consumption stops at `--flag` (b1-stop, starts with `-`) or at end-of-tokens; the command tail is NOT swallowed; `command_preview` masks the credential up to the stop token; raw credential absent from both channels; gate returns Continue with advisory |
| EC-025 | Bash command has a quoted Form A Authorization header in a `-H` curl argument: `grep -r . -H "Authorization: Bearer eyJtoken"` (closing `"` in the same token); command matches `grep -r` | INV5 Pass 3 Form A regression guard: the quoted token `"Authorization: Bearer eyJtoken"` has content after `:` within the same token → still fully masked to `Authorization:***REDACTED***`; this validates Form A continues to work correctly after the bounded-consumption rule change; raw token absent from both channels; gate returns Continue with advisory |

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
| Bash command: `grep -r . --token supersecrettoken123` | Continue; DelegationRecommended advisory; `command_preview` in plugin.log and stderr contains `--token ***REDACTED***`; raw `supersecrettoken123` absent from both channels (Pass 1) | redaction-pass1-trigger-flag |
| Bash command: `API_KEY=sk-abc123 grep -r .` | Continue; DelegationRecommended advisory; `command_preview` contains `API_KEY=***REDACTED***`; raw `sk-abc123` absent from both channels (Pass 2) | redaction-pass2-env-assignment |
| Bash command: `grep -r . -H "Authorization: Bearer eyJtoken123"` | Continue; DelegationRecommended advisory; `command_preview` shows `Authorization:***REDACTED***` (Form A inline — no further-token consumption); raw token absent from both channels (Pass 3 Form A) | redaction-pass3-form-a-quoted |
| Bash command: `curl Authorization: Bearer tok https://api.example.com` (unquoted two-token header + trailing URL) | Continue (not dispatched by default pattern list, but Pass 3 bounded-consumption validated); `command_preview` contains `Authorization: ***REDACTED*** https://api.example.com`; URL preserved (b2-stop); raw `tok` absent (Pass 3 Form B bounded) | redaction-pass3-form-b-bearer-url-preserved |
| Bash command: `grep -r . Authorization: Bearer tok --verbose` (unquoted two-token header + trailing flag) | Continue; DelegationRecommended advisory; `command_preview` contains `Authorization: ***REDACTED*** --verbose`; flag preserved (b1-stop); raw `tok` absent (Pass 3 Form B bounded) | redaction-pass3-form-b-bearer-flag-preserved |
| Bash command: `grep -r . -H "Authorization: Bearer tok --flag` (unbalanced-quote header) | Continue; DelegationRecommended advisory; consumption stops at `--flag` (b1-stop) or end-of-tokens; command tail NOT swallowed; `command_preview` masks credential up to stop; raw credential absent (Pass 3 fail-safe) | redaction-pass3-unbalanced-quote-fail-safe |
| Bash command: `./run-all.sh https://user:pass@example.com/db` | Continue; DelegationRecommended advisory; `command_preview` contains `https://***REDACTED***@example.com/db`; raw `user:pass` absent from both channels (Pass 4) | redaction-pass4-url-credential |
| Bash command: `grep -r TODO . --include="*.rs"` (no Pass 1–4 triggers) | Continue; DelegationRecommended advisory; `command_preview` is unmodified (no `***REDACTED***` token); no false-positive redaction | redaction-clean-no-redaction |

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

- VP-091 — verifies: gate always returns Continue; advisory emitted on match; never blocks; pure-parse; non-Bash no-op; command_preview ≤120-char truncation (Invariant 4); secret-redaction 4-pass best-effort advisory (Invariant 5 / SEC-002; VP-091 §6 PC6a–PC6g: Pass 1 flag-arg masked; Pass 2 env-assignment masked; Pass 3 Authorization/Cookie header masked; Pass 4 URL credential masked; allowlist SSH_AUTH_SOCK/SSH_ASKPASS/*_SERVICE_HOST NOT masked; bare --key NOT masked; EC-020/EC-021 redact-then-truncate ordering invariant)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-091 | validate-heavy-op-delegation returns Continue under all conditions (match, no-match, crash); emits `DelegationRecommended` advisory to both stderr and plugin.log on first pattern match; emits nothing on no-match; is a no-op on non-Bash tool calls; performs no filesystem, subprocess, or context reads; `command_preview` is secret-redacted via 4-pass best-effort advisory algorithm (INV5) then truncated ≤120 chars (INV4); redaction is byte-identical across both channels; raw secrets absent from both channels; allowlist enforced before trigger heuristic; bare `--key` not masked (VP-091 §6 PC6a–PC6g) | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC specifies the advisory WASM delegation gate that nudges orchestrators and agents to offload heavy operations (large-output cargo test runs, recursive grep traversals, full bats suite) to sub-agents or worktrees rather than executing them in the primary orchestrator context window. By reducing context-window pressure from heavy ops, the gate is a preventive-advisory measure that supports the CAP-032 continuity guarantee: a context window that is not saturated by heavy-op output is less likely to trigger an uncoordinated auto-compaction event that loses load-bearing pipeline state. This BC anchors to CAP-032 as a preventive-advisory measure per OQ-4.4 (adopted), NOT as a direct enforcer of HANDOFF.md integrity (that is BC-4.14.001's role). |
| L2 Domain Invariants | DI-020 (Wave/phase boundary transitions must not lose load-bearing pipeline state) — this gate is a PREVENTIVE-ADVISORY measure that reduces the probability of context-window saturation triggering an uncoordinated mid-wave compaction event; it does NOT directly enforce DI-020 (no blocking behavior) but supports it by reducing heavy-op context pressure (per OQ-4.4 adopted resolution) |
| Architecture Module | SS-04 (Plugin Ecosystem) — new WASM crate under `crates/hook-plugins/validate-heavy-op-delegation/` |
| ADR | ADR-026 §Decision 12 (validate-heavy-op-delegation advisory-only in v1; emits advisory to stderr; never blocks; F3 adversarial calibration before blocking-promotion; S-18.06 correctly scoped as advisory-only; §Decision 12 SEC-002 Redaction Sub-clause governs INV5 secret-redaction: best-effort advisory framing; redact-then-truncate ordering mandate; 4-pass no-regex algorithm; compile-time trigger lists; mask token `***REDACTED***` (14 chars); non-configurable; provider-prefix detection deferred), §Decision 8 (WASM for pure-function command-string pattern matching; no filesystem or git side effects; timeout_ms=5000) |
| Stories | S-18.06 |
| Cycle | v1.0-feature-context-durability-E18 (F3) |
| Feature | issue #173 / E-18 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.6 | 2026-06-26 | product-owner | O-1 Pass-1 double-dash clarification (spec-text only; no behavior change): INV5 Pass 1 trigger-scope sentence changed from 'tokens starting with `-`' to 'tokens starting with `--` (double-dash long flag tokens)'; step 1 reworded to 'strip the leading `--`'; closing carve-out rewritten to state single-dash tokens are NOT processed by Pass 1. Eliminates text tension between the step prose and the former short-flag carve-out; both now consistently describe `starts_with("--")` semantics. Rationale: real-world secret flags use `--token`/`--password` or a short alias; single-dash long-name secret flags do not exist in the scoped threat model. Refs: O-1, ADR-026 §Decision 12, VP-091 §6, S-18.06. |
| v1.5 | 2026-06-26 | product-owner | F-RD2-001/F-RD2-002: INV5 Pass 3 replaced with bounded-consumption rule per ADR-026 §Decision 12 SEC-002 Redaction Sub-clause (ADR v1.29) + VP-091 §6 PC6d (VP v1.5). Form A (inline value — content after `:` in same token): mask after `:` within that token; stop — no further-token consumption. Form B (following-token value — `:` at end of header token): mask immediate next token(s); stop at first subsequent token starting with `-` (b1-CLI flag), containing `://` (b2-URL), or closing a quoted value (b3); immediate next token always masked. Malformed/unbalanced-quote fails safe: stops at b1/b2 token or end-of-tokens; tail NOT swallowed. Fixes: F-RD2-001 (unquoted Form B was consuming trailing URLs, destroying them); F-RD2-002 (unbalanced-quote was swallowing command tail). EC-016 annotation updated (Form A vs Form B distinction). EC-022..EC-025 added (append-only): EC-022 (Form B + trailing URL: URL preserved); EC-023 (Form B + trailing flag: flag preserved); EC-024 (unbalanced-quote fail-safe: stops at b1/end); EC-025 (quoted Form A regression guard: still fully masked). 4 canonical test vector rows added: redaction-pass3-form-a-quoted; redaction-pass3-form-b-bearer-url-preserved; redaction-pass3-form-b-bearer-flag-preserved; redaction-pass3-unbalanced-quote-fail-safe. Refs: F-RD2-001, F-RD2-002, ADR-026 §Decision 12, VP-091 §6, S-18.06, E-18, issue-173. |
| v1.4 | 2026-06-26 | product-owner | SEC-002 revised: INV5 rewritten to final canonical 4-pass best-effort-advisory design per ADR-026 §Decision 12 SEC-002 Redaction Sub-clause + VP-091 §6. Best-effort-advisory framing added (not a security boundary). Mask token corrected to 14 ASCII chars (v1.3 incorrectly stated 15). Pass 1 revised: bare `--key` REMOVED (false-positive trap); lowercased flag-name equals-or-contains compound trigger substrings (password, passwd, token, secret, api-key, apikey, api_key, client-secret, auth-token, access-token, access-key, secret-key, credential, passphrase, private-key); bare short flags excluded. Pass 2 added: env-var assignment-prefix masking (compound IDENT triggers; bare KEY not matched; allowlist SSH_AUTH_SOCK/SSH_ASKPASS/*_SERVICE_HOST checked first). Pass 3 added: Authorization/Cookie/Set-Cookie header masking (all schemes). Pass 4 added: inline URL credential masking (://user:pass@host → ://***REDACTED***@host). Provider-prefix detection (sk-/AKIA/ghp_) explicitly deferred as documented future enhancement. EC-014..EC-020 (from v1.3) retired (append-only: kept with superseded marker); EC-014..EC-021 added (8 ECs covering all 4 passes + negatives: flag-arg masked, env-assignment masked, Authorization/Cookie masked, URL-cred masked, clean unchanged, SSH_AUTH_SOCK allowlist not masked, bare --key not masked, redact-expands-past-120 truncation still applies). Canonical test vectors revised: 5 redaction rows (Pass 1 flag, Pass 2 env-assignment, Pass 3 authorization, Pass 4 URL-cred, clean-no-redaction). PC-B-B1/PC-B-B2 command_preview definition updated to reference 4-pass INV5. INV4 mask-token count corrected to 14 chars. VP Anchors updated to PC6a–PC6g. Traceability ADR row updated. Refs: SEC-002, ADR-026 §Decision 12, VP-091 §6, S-18.06, E-18, issue-173. |
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
