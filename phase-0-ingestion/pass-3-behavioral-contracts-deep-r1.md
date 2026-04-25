# Pass 3 — Phase B Deepening Round 1: Behavioral Contracts

**Date:** 2026-04-25
**Pass:** 3 (Behavioral Contracts) | **Round:** 1
**Project:** vsdd-factory (self-referential ingest; engine + product in same repo)

## 1. Round metadata

**Inputs read (verbatim):**

- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-0-inventory.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-1-architecture.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-2-domain-model.md` (referenced; orchestrator confirmed presence)
- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-3-behavioral-contracts.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-4-nfr-catalog.md` (referenced)
- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-5-conventions.md` (referenced)
- `/Users/jmagady/Dev/vsdd-factory/.factory/phase-0-ingestion/pass-6-synthesis.md`

**New source files read this round (BC-citation provenance):**

- `crates/factory-dispatcher/src/plugin_loader.rs` (183 LOC, full)
- `crates/factory-dispatcher/src/sinks/mod.rs` (310 LOC, full)
- `crates/factory-dispatcher/src/sinks/router.rs` (67 LOC, full)
- `crates/hook-sdk/src/host.rs` (375 LOC, full)
- `crates/hook-sdk/src/ffi.rs` (127 LOC, full)
- `crates/hook-sdk/src/lib.rs` (59 LOC, full)
- `crates/hook-sdk-macros/src/lib.rs` (104 LOC, full)
- `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (377 LOC, full)
- `crates/sink-otel-grpc/src/lib.rs` (1,134 LOC; constructor + worker_loop + flush_buffer + record_failure + Sink impl + tests heading)
- `plugins/vsdd-factory/workflows/greenfield.lobster` (1,409 LOC, full sweep)
- `plugins/vsdd-factory/workflows/brownfield.lobster` (401 LOC, full)
- `plugins/vsdd-factory/workflows/code-delivery.lobster` (first 150 of full file)
- `plugins/vsdd-factory/bin/lobster-parse` (52 LOC, full — confirms YAML+yq+jq pipeline)
- `plugins/vsdd-factory/skills/activate/SKILL.md` (frontmatter + procedure)
- `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md` (top 80 LOC)
- `plugins/vsdd-factory/skills/wave-gate/SKILL.md` (top 80 LOC)
- `plugins/vsdd-factory/skills/code-delivery/SKILL.md` (top 80 LOC)
- `plugins/vsdd-factory/skills/release/SKILL.md` (top 80 LOC)
- `plugins/vsdd-factory/skills/create-prd/SKILL.md` (top 60 LOC)
- `plugins/vsdd-factory/skills/deliver-story/SKILL.md` (top 60 LOC)
- `plugins/vsdd-factory/skills/state-burst/SKILL.md` (top 60 LOC)
- `plugins/vsdd-factory/skills/disposition-pass/SKILL.md` (top 60 LOC)
- `plugins/vsdd-factory/hooks/validate-novelty-assessment.sh` (99 LOC, full)
- `plugins/vsdd-factory/hooks/validate-bc-title.sh` (top 50 LOC)
- `plugins/vsdd-factory/hooks/validate-state-size.sh` (top 40 LOC)
- `plugins/vsdd-factory/hooks/check-factory-commit.sh` (24 LOC, full)
- `plugins/vsdd-factory/hooks/protect-secrets.sh` (196 LOC, full)
- `plugins/vsdd-factory/hooks/protect-bc.sh` (top 50 LOC)
- `plugins/vsdd-factory/hooks/pr-manager-completion-guard.sh` (top 50 LOC)
- `plugins/vsdd-factory/hooks-registry.toml` (top 100 LOC + on_error / priority distribution counts)

**Independent metric reconfirmation (find + wc):**

- Total Rust LOC: **10,226** (`find /Users/jmagady/Dev/vsdd-factory/crates -name '*.rs' -type f -exec wc -l {} + | tail -1`) — matches Pass 0.
- `SKILL.md` files: **119** — matches Pass 0.
- Top-level `hooks/*.sh` files: **44** — matches Pass 0.
- `*.lobster` files: **16** — matches Pass 0 (8 top-level + 8 phase sub-flows).
- `validate-*.sh`: **22** files (NOT 24+).
- `verify-*.sh`: **1** file in `hooks/` (`verify-git-push.sh`).
- `hooks-registry.toml` `[[hooks]]` entries: **45**.
- Distribution: `on_error = "block"`: 18; `on_error = "continue"`: 27.

## 2. Round 1 audit — broad-sweep against the 5 Known Hallucination Classes

### Class 1 — Over-extrapolated token lists

**No retraction.** Pass 3's BC-AUDIT-068 lists 24 validators by name. Cross-checking against `find … -name 'validate-*.sh'` returns 22 files; `verify-*.sh` returns 1 (`verify-git-push.sh`). The token `verify-sha-currency.sh` listed in BC-AUDIT-068 is **not** a registered hook — it lives at `plugins/vsdd-factory/templates/verify-sha-currency.sh` (a template file copied into operator projects on opt-in, per DRIFT-009). See **CONV-ABS-1** below.

### Class 2 — Miscounted enumerations

**Two retractions.**

- **CONV-ABS-1:** Pass 3 BC-AUDIT-068 mistakenly conflates a registered hook with a template file. `verify-sha-currency.sh` is a **template** distributed for operators to opt into; it is NOT a vsdd-factory-internal validator hook and is NOT registered in `hooks-registry.toml`. Its peer `verify-git-push.sh` is real and is registered. Net: validators (`validate-*` + `verify-*`) = **23** distinct registered scripts (22 + 1), not 24.
- **CONV-ABS-2:** Pass 3 BC-AUDIT-068 lists `validate-anchor-capabilities-union` **twice** in the same enumeration (both at the start and the end of the list). Single occurrence in `hooks/`. Distinct validator count = 22 unique `validate-*.sh` + 1 `verify-git-push.sh` = **23 unique validators**.

### Class 3 — Named pattern conflation / fabrication

**No retraction.** All BCs in Pass 3 cite real source-file evidence; spot-check of BC-AUDIT-009..014 (invoke.rs tests), BC-AUDIT-035..040 (internal_log.rs tests), BC-AUDIT-050..052 (hook-sdk/result.rs tests) all have matching test names that exist.

### Class 4 — Same-basename artifact conflation

**One retraction.**

- **CONV-ABS-3:** The two `verify-sha-currency.sh` paths (`templates/verify-sha-currency.sh` and the hypothetical `hooks/verify-sha-currency.sh`) were conflated in BC-AUDIT-068. Only the templates path exists. The CHANGELOG entry for v1.0.0-beta.4 says "ships the verify-sha-currency.sh template + state-burst skill" — the **template** is the deliverable, and operators copy it into their own `hooks/` if they choose to opt in. (See DRIFT-009 in pass-6 for the same observation framed as drift-not-bug.)

### Class 5 — Inflated or deflated metrics

**No retraction (round 1 metrics confirmed).** Independent recount:

| Pass-0 claim | Recount | Verdict |
|---|---|---|
| 10,226 Rust LOC | 10,226 | confirmed |
| 119 skills | 119 | confirmed |
| 44 top-level hooks/*.sh | 44 | confirmed |
| 16 workflow .lobster files | 16 | confirmed |
| 45 hooks-registry.toml entries | 45 | confirmed |

The pass-6 reference to "1245+ baseline bats per CHANGELOG" is unverifiable from raw source (it's a CHANGELOG claim) but is consistent with the test directory structure (71 .bats helpers + per-version regression suites).

## 3. GAP-A findings — Skill execution class contracts (BC-AUDIT-087 onward)

Sample base for class extraction: `activate`, `brownfield-ingest`, `wave-gate`, `code-delivery`, `release`, `create-prd`, `deliver-story`, `state-burst`, `disposition-pass` (9 skills spanning all 5 functional categories the orchestrator uses).

### BC-AUDIT-087: SKILL.md frontmatter requires `name` and `description`; both are non-empty strings

- **Preconditions:** A SKILL.md file exists under `plugins/vsdd-factory/skills/<dir>/SKILL.md`.
- **Postconditions:** YAML frontmatter (between `---` markers) declares non-empty `name:` and `description:`. Optional fields: `argument-hint:`, `disable-model-invocation:`, `allowed-tools:`, `model:`, `color:`, `tools:`. The frontmatter `name` matches the filesystem directory name (canonical skill identity).
- **Evidence:** `plugins/vsdd-factory/skills/activate/SKILL.md:1-4` (`name: activate`, `description: Opt in to the VSDD factory persona…`); `plugins/vsdd-factory/skills/brownfield-ingest/SKILL.md:1-5`; `plugins/vsdd-factory/skills/wave-gate/SKILL.md:1-7`; `plugins/vsdd-factory/skills/release/SKILL.md:1-8`; `plugins/vsdd-factory/skills/create-prd/SKILL.md:1-6`; `plugins/vsdd-factory/skills/deliver-story/SKILL.md:1-7`. All 6 sampled skills have both fields.
- **Confidence:** HIGH (uniform across all 9 sampled skills).

### BC-AUDIT-088: SKILL.md description supports YAML block scalar (`>` folded) for multi-line text

- **Preconditions:** Skill description exceeds one line of YAML.
- **Postconditions:** Description uses `>` folded scalar, e.g. `description: >\n  Opt in to…`. Resulting parsed `description` is a single string with newlines folded to spaces.
- **Evidence:** `release/SKILL.md:3-7` (folded scalar across 5 lines); `code-delivery/SKILL.md:2-7`; `deliver-story/SKILL.md:3-4`.
- **Confidence:** HIGH.

### BC-AUDIT-089: Skill invocation surface is `/vsdd-factory:<skill-name>` slash command

- **Preconditions:** Operator running Claude Code with vsdd-factory plugin activated.
- **Postconditions:** Typing `/vsdd-factory:<skill-name>` (where `<skill-name>` matches the frontmatter `name`) invokes the procedure body. The slash-command surface is enumerated separately in `plugins/vsdd-factory/commands/*.md` (110 files in pass-0 inventory).
- **Evidence:** `pass-0-inventory.md` Section 5; `skills/activate/SKILL.md` "See also" section invokes `/vsdd-factory:deactivate`. Each skill has a 1:1 slash command in `commands/`.
- **Confidence:** MEDIUM (110 commands vs 119 skills — there's a 9-skill gap; not every skill has a slash command. Verified by file count discrepancy, not yet fully enumerated).

### BC-AUDIT-090: Skills with `disable-model-invocation: true` are dispatcher-only — model cannot self-invoke

- **Preconditions:** Skill frontmatter contains `disable-model-invocation: true`.
- **Postconditions:** Model cannot launch the skill via reasoning; only orchestrator dispatch (Task tool + agent handoff) or explicit slash command invocation activates it.
- **Evidence:** `skills/wave-gate/SKILL.md:6` (`disable-model-invocation: true`); `skills/create-prd/SKILL.md:4`; `skills/deliver-story/SKILL.md:5`. Pattern is used for high-stakes orchestration skills to prevent inadvertent self-dispatch.
- **Confidence:** HIGH (consistent flag usage across high-risk skills).

### BC-AUDIT-091: Skills with `allowed-tools:` whitelist restrict tool surface inside the skill body

- **Preconditions:** Skill frontmatter declares `allowed-tools: Read, Write, Edit, …`.
- **Postconditions:** Inside the skill's procedure, only the listed tools may be invoked. Claude Code enforces the whitelist at tool-dispatch time.
- **Evidence:** `wave-gate/SKILL.md:7` (`allowed-tools: Read, Write, Edit, Bash, Glob, Grep, AskUserQuestion`); `create-prd/SKILL.md:5` (`allowed-tools: Read, Write, Edit, Bash, AskUserQuestion`); `deliver-story/SKILL.md:6` (`allowed-tools: Read, Bash, Glob, Grep, AskUserQuestion, Task`).
- **Confidence:** HIGH (frontmatter pattern uniform on the sampled skills that use it).

### BC-AUDIT-092: "Announce at Start" protocol — verbatim opening line per skill

- **Preconditions:** Skill body declares an "Announce at Start" section.
- **Postconditions:** Before any other action, the skill emits a verbatim sentence such as "I'm using the wave-gate skill to run the post-wave integration gate for wave-N." Every sampled skill uses an identical convention; the verb tense and skill-name self-reference are part of the contract. This is the orchestrator's auditable handoff signal.
- **Evidence:** `wave-gate/SKILL.md:21-23` ("I'm using the wave-gate skill…"); `brownfield-ingest/SKILL.md:19-21`; `deliver-story/SKILL.md:25-27`; `release/SKILL.md:21-22`; `state-burst/SKILL.md:17-22`.
- **Confidence:** HIGH (5 of 5 sampled "orchestrator-managed" skills follow this pattern).

### BC-AUDIT-093: Skills SHALL link to template files via `${CLAUDE_PLUGIN_ROOT}/templates/...` references

- **Preconditions:** Skill produces output in a templated format.
- **Postconditions:** Skill body lists relevant template paths under a "Templates" heading, using the `${CLAUDE_PLUGIN_ROOT}/templates/<name>` form (NOT a relative path). The orchestrator resolves this against the active plugin's install location at runtime.
- **Evidence:** `brownfield-ingest/SKILL.md:42-44` (`${CLAUDE_PLUGIN_ROOT}/templates/recovered-architecture-template.md`); `create-prd/SKILL.md:18-24` (lists 6 templates).
- **Confidence:** HIGH.

### BC-AUDIT-094: Skill quality gates expressed as a "Hard Gate" or "Iron Law" prose section

- **Preconditions:** Skill enforces an invariant the operator/agent must not violate.
- **Postconditions:** Skill body declares the invariant under "## Hard Gate" or "## The Iron Law" with imperative wording ("DO NOT skip…", "NO ROUND COMPLETION…"). These are the testable acceptance gates that downstream BC extraction can crystallize.
- **Evidence:** `create-prd/SKILL.md:9-11` ("Hard Gate: Do NOT skip to architecture design…"); `brownfield-ingest/SKILL.md:11-15` ("The Iron Law: NO ROUND COMPLETION WITHOUT HONEST CONVERGENCE CHECK FIRST"); `wave-gate/SKILL.md:13-17` ("NO WAVE ADVANCE WITHOUT ALL SIX GATES PASSING FIRST"); `deliver-story/SKILL.md:17-21` ("NO IMPLEMENTATION WITHOUT RED GATE VERIFICATION FIRST").
- **Confidence:** HIGH (consistent across 4 sampled "high-stakes" skills).

### BC-AUDIT-095: Skill bodies MAY include a "Red Flags" table to enumerate fabrication / shortcut anti-patterns

- **Preconditions:** Skill is high-risk for self-deception or operator shortcut.
- **Postconditions:** A markdown table maps "Thought" (operator's tempting deviation) to "Reality" (the canonical correction). Used as an in-skill counter-narrative tool.
- **Evidence:** `brownfield-ingest/SKILL.md:25-39` (10-row table); `wave-gate/SKILL.md:27-39` (8-row table).
- **Confidence:** HIGH (pattern present on the two skills with the highest fabrication risk; not universal — release/SKILL.md doesn't use it).

### BC-AUDIT-096: Skills that dispatch sub-agents declare a "Canonical Source" or single-source-of-truth playbook reference

- **Preconditions:** Skill is a dispatch-only entry point (e.g., delegates execution to specialist agents).
- **Postconditions:** Skill body identifies a single load-bearing playbook (typically `agents/orchestrator/<flow>.md`) with explicit "if the two ever disagree, the orchestrator file wins." Re-read at the start of every dispatch.
- **Evidence:** `deliver-story/SKILL.md:45-49` (`agents/orchestrator/per-story-delivery.md` is the playbook). The skill itself enforces "Read the orchestrator file at the start of every dispatch. Do not cache it between runs."
- **Confidence:** HIGH (single-source-of-truth pattern is explicit).

### BC-AUDIT-097: Skills with `argument-hint:` declare inline `$ARGUMENTS[N]` / `$ARGUMENTS` semantics

- **Preconditions:** Skill takes user-supplied positional arguments.
- **Postconditions:** Frontmatter `argument-hint:` shows the expected shape (e.g., `[STORY-NNN]`, `[<repo>|--all] [--rollup] [--update-vision]`). Body references `$ARGUMENTS[0]`, `$ARGUMENTS[1]`, etc., with explicit type/semantics.
- **Evidence:** `release/SKILL.md:7` (`argument-hint: "[init | <version> | --dry-run]"`); `deliver-story/SKILL.md:4` (`argument-hint: "[STORY-NNN]"`); `disposition-pass/SKILL.md:4` (`argument-hint: "[<repo>|--all] [--rollup] [--update-vision]"`).
- **Confidence:** HIGH.

### BC-AUDIT-098: Skill output paths follow `${CLAUDE_PLUGIN_ROOT}` / `.factory/` placement convention

- **Preconditions:** Skill produces artifacts.
- **Postconditions:** Outputs go to `.factory/<subtree>/` (project-local persistence); skill steps NEVER write outside `.factory/` or product working directories. Sandbox patterns: `.factory/specs/`, `.factory/stories/`, `.factory/cycles/`, `.factory/phase-0-ingestion/`, `.factory/semport/`. Plugin-distributed reference content lives at `${CLAUDE_PLUGIN_ROOT}/templates/`.
- **Evidence:** `create-prd/SKILL.md:3` ("writes to .factory/specs/prd.md and supplements"); `brownfield-ingest/SKILL.md:74-78` (analysis writes under `.factory/semport/<project>/`); `disposition-pass/SKILL.md:46` (reads `.factory/semport/<repo>/`).
- **Confidence:** HIGH (universal across all sampled skills).

## 4. GAP-B findings — Validator hook class contracts

Sample base: `validate-novelty-assessment.sh`, `validate-bc-title.sh`, `validate-state-size.sh`, `protect-secrets.sh`, `protect-bc.sh`, `pr-manager-completion-guard.sh`, `check-factory-commit.sh`. All read at full body for `validate-novelty-assessment.sh`, `protect-secrets.sh`, `check-factory-commit.sh`, head-of-body for the others.

### BC-AUDIT-099: Hooks read JSON envelope from stdin and parse with jq

- **Preconditions:** Dispatcher (via `legacy-bash-adapter`) pipes the original `HookPayload` JSON to the hook via stdin.
- **Postconditions:** First substantive lines invoke `INPUT=$(cat)` then `echo "$INPUT" | jq -r '.field // empty'` to extract `tool_name`, `tool_input.file_path`, `tool_input.command`, `agent_type`, `last_assistant_message`, etc. Hooks SHALL gracefully no-op (`exit 0`) if jq is missing on the host.
- **Evidence:** `validate-novelty-assessment.sh:14-22` (`if ! command -v jq …; exit 0; INPUT=$(cat); FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')`); `protect-secrets.sh:22-29`; `validate-bc-title.sh:14-21`; `pr-manager-completion-guard.sh:18-27`; `check-factory-commit.sh:1-7`.
- **Confidence:** HIGH (uniform across all 6 sampled validators).

### BC-AUDIT-100: Hook exit code semantics: 0 = pass/allow, 2 = block, with stderr diagnostic

- **Preconditions:** Hook script executes a check.
- **Postconditions:** Exit `0` to allow the wrapped tool call (PreToolUse) or accept the file (PostToolUse). Exit `2` to BLOCK with a multi-line stderr diagnostic that includes (a) violation summary, (b) per-error bullet list, (c) remediation hint, (d) reference to template/policy doc. Some hooks (PreToolUse on Read/Bash) emit a JSON `permissionDecision` envelope on stdout instead of exit-coding.
- **Evidence:** `validate-novelty-assessment.sh:84-96` (exit 2 with `echo "NOVELTY ASSESSMENT VIOLATION:" >&2`); `validate-state-size.sh:1-12` (header docstring asserts exit 2 on bloat); `protect-bc.sh:35-46` (`emit_deny` writes JSON `permissionDecision: "deny"` on stdout for PreToolUse PermissionDecision shape); `pr-manager-completion-guard.sh:12-15`.
- **Confidence:** HIGH.

### BC-AUDIT-101: Hooks emit `hook.block` event on block via `${CLAUDE_PLUGIN_ROOT}/bin/emit-event`

- **Preconditions:** Hook decided to block.
- **Postconditions:** Before exit 2, the hook calls `_emit type=hook.block hook=<hook-name> matcher=<event_name> reason=<reason_code> [file_path=… | command=…]`. The `_emit` helper resolves the path via `${CLAUDE_PLUGIN_ROOT}/bin/emit-event` and silently no-ops if the bin or env var is missing (`2>/dev/null || true`). This is the only mechanism by which validator hooks contribute to the dispatcher's event stream.
- **Evidence:** `validate-novelty-assessment.sh:23-28` and `:86-87` (exact `_emit type=hook.block hook=validate-novelty-assessment matcher=PostToolUse reason=novelty_assessment_incomplete`); `protect-secrets.sh:31-36` and `:44`; `protect-bc.sh:23-28` and `:38`.
- **Confidence:** HIGH.

### BC-AUDIT-102: Hook scoping uses `case "$FILE_PATH"` glob narrowing — early `exit 0` on irrelevant files

- **Preconditions:** PostToolUse fires on every file write.
- **Postconditions:** Hook narrows scope with `case "$FILE_PATH" in <pattern>) ;; *) exit 0 ;; esac`. Violation: a hook that processes every file would block CPU on irrelevant paths. Standard pattern: 3-5 lines of glob narrowing, then optional exclude list (e.g., INDEX files, finding files), then content check.
- **Evidence:** `validate-novelty-assessment.sh:34-49` (target glob `*pass-[0-9]*.md`, `adversarial-*review*.md`, then exclude `INDEX/FINDINGS/ADV-*`); `validate-bc-title.sh:34-43`; `validate-state-size.sh:33-37`.
- **Confidence:** HIGH.

### BC-AUDIT-103: Hook latency budget is sub-100ms; deterministic; LLM-free

- **Preconditions:** Any validator hook.
- **Postconditions:** Each script's docstring explicitly asserts "Deterministic, <100ms, no LLM" or similar. Body contains no network calls, no LLM SDK invocations, only `grep`/`sed`/`jq`/`awk` against bounded file content.
- **Evidence:** `validate-novelty-assessment.sh:11-12` ("Deterministic, <100ms, no LLM"); `validate-bc-title.sh:11-12`; `validate-state-size.sh:10-11`; `pr-manager-completion-guard.sh:14-15`; `protect-secrets.sh:19-20` ("Deterministic, <50ms, no LLM").
- **Confidence:** HIGH (declared explicitly across 5 of 5 sampled validators).

### BC-AUDIT-104: factory-dispatcher routing binds hooks via `[[hooks]]` entry: `name`, `event`, optional `tool` regex, `priority`, `timeout_ms`, `on_error`

- **Preconditions:** A `hooks-registry.toml` `[[hooks]]` stanza loaded via `Registry::parse_str`.
- **Postconditions:** Routing tuple is `(name, event, tool?, priority, timeout_ms, on_error)`. Independently confirmed: registry has 45 entries, `priority` ranges 20..950, `on_error` is one of `block` (18 entries) or `continue` (27 entries). All 45 entries route via `plugin = "hook-plugins/legacy-bash-adapter.wasm"`. Per-entry capabilities live at `[hooks.capabilities]` and `[hooks.capabilities.exec_subprocess]`.
- **Evidence:** `hooks-registry.toml:19-36` (capture-commit-activity entry: priority 110, on_error continue, exec_subprocess capability with `binary_allow = ["bash", "git", "gh", "jq"]`); broader stanza-count + on_error distribution from independent `grep | sort | uniq -c` (18 block / 27 continue).
- **Confidence:** HIGH.

### BC-AUDIT-105: Validator hooks at `tool = "Edit|Write"` regex run on EVERY post-edit / post-write event regardless of file path

- **Preconditions:** PostToolUse fires after Edit or Write.
- **Postconditions:** All 23 distinct validator hooks are routed via the `tool = "Edit|Write"` regex match (per `routing.rs::tool_matches` BC-AUDIT-005). The validators must therefore self-narrow via the `case "$FILE_PATH"` pattern from BC-AUDIT-102 to avoid burning latency on irrelevant writes.
- **Evidence:** `hooks-registry.toml` validate-* stanzas all have `tool = "Edit|Write"` (read 100 LOC + spot-checks); `routing.rs::tests::match_respects_tool_regex_anchoring` (cited by Pass 3 BC-AUDIT-005).
- **Confidence:** HIGH.

### BC-AUDIT-106: Hook capability model: every legacy-routed hook declares `[hooks.capabilities.exec_subprocess]` with `binary_allow` + `shell_bypass_acknowledged`

- **Preconditions:** Hook entry routes through `legacy-bash-adapter.wasm`.
- **Postconditions:** Stanza declares `binary_allow = ["bash", …]` (always includes "bash" since the adapter shells out to bash); `shell_bypass_acknowledged = "legacy-bash-adapter runs unported hooks"` (verbatim string, matches `legacy-bash-adapter/src/lib.rs:20`); `env_allow` includes `["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID"]` baseline.
- **Evidence:** `hooks-registry.toml:30-36` (capture-commit-activity capabilities); `legacy-bash-adapter/src/lib.rs:20` (the verbatim shell_bypass acknowledgement string); pass-3 BC-AUDIT-025 already confirms shell-bypass enforcement in the dispatcher.
- **Confidence:** HIGH.

### BC-AUDIT-107: Native (non-legacy) hook plugins MUST link `vsdd-hook-sdk` and use the `#[hook]` macro (not currently used in production registry)

- **Preconditions:** A `[[hooks]]` entry references a non-`legacy-bash-adapter.wasm` plugin path.
- **Postconditions:** The plugin must be a Cargo crate whose entry function is annotated `#[hook]` and whose signature is exactly `fn(payload: HookPayload) -> HookResult`. Async / unsafe / multi-arg / non-`HookResult`-return signatures fail the proc-macro check at compile time. Sole production-ready candidate today is the stub `hook-plugins/capture-commit-activity` (S-3.1, not yet swapped in).
- **Evidence:** `hook-sdk-macros/src/lib.rs:38-95` (validate_signature enforces async/unsafe/arg-count/return-type); `hook-sdk-macros/src/lib.rs:47-55` (generates `fn main() { run(user_fn) }`); pass-6 DRIFT/story-coverage map confirms no native hook in production registry as of beta.4.
- **Confidence:** HIGH (compile-time-enforced; trybuild tests covered in pass-3 BC-AUDIT-050..053 already).

## 5. GAP-C findings — Workflow `.lobster` protocol contracts

Sample base: `greenfield.lobster` (1,409 LOC, full sweep), `brownfield.lobster` (401 LOC, full), `code-delivery.lobster` (top 150 LOC).

### BC-AUDIT-108: A `.lobster` file is YAML at the top level with a single `workflow:` key

- **Preconditions:** File extension `.lobster`; lives under `plugins/vsdd-factory/workflows/` or `workflows/phases/`.
- **Postconditions:** Top-level YAML object with exactly one `workflow:` key whose value is a map containing `name`, `description`, `version`, `defaults`, `steps[]`, and optionally `inputs[]`, `cost_monitoring{}`. The `version` field is SemVer-ish (`"2.1.0"`, `"3.0.0"`). Lobster files are pure data — `bin/lobster-parse` consumes them via `yq eval --output-format=json | jq …`.
- **Evidence:** `greenfield.lobster:1-30` (workflow.name, description, version "2.1.0", cost_monitoring, defaults); `brownfield.lobster:1-23` (workflow.name, version "3.0.0"); `code-delivery.lobster:1-32` (workflow.name, inputs[], defaults); `bin/lobster-parse:39-51` (`yq eval --output-format=json '.' | jq "$EXPR"`).
- **Confidence:** HIGH.

### BC-AUDIT-109: Workflow `defaults:` block sets default `on_failure`, `max_retries`, `timeout` for unspecified steps

- **Preconditions:** `defaults:` declared at workflow level.
- **Postconditions:** Three keys: `on_failure: escalate` (universal across sampled), `max_retries: 2` (universal), `timeout: "2h"` or `"1h"`. Each step inherits these unless it overrides at the step level.
- **Evidence:** `greenfield.lobster:26-29` (`on_failure: escalate, max_retries: 2, timeout: "2h"`); `brownfield.lobster:19-22` (same shape); `code-delivery.lobster:28-31` (same).
- **Confidence:** HIGH.

### BC-AUDIT-110: Step taxonomy: `type:` enumerated as `skill`, `agent`, `gate`, `loop`, `human-approval`, `sub-workflow`, `parallel`, `compound`

- **Preconditions:** A step in `workflow.steps[]` declares `type:`.
- **Postconditions:** `type` is exactly one of the 8 listed values:
  - `skill` — invoke a SKILL.md file (`skill: "skills/<name>/SKILL.md"`)
  - `agent` — dispatch a sub-agent identity (`agent: <name>`, `task: "…"`)
  - `gate` — assert pass criteria with `gate: { criteria: [...], fail_action: block }`
  - `loop` — bounded iteration with `loop: { max_iterations: N, exit_condition: "…", steps: [...] }`
  - `human-approval` — pause for operator sign-off with `approval: { prompt, artifacts, timeout }`
  - `sub-workflow` — invoke another `.lobster` file (`sub_workflow: "<file>.lobster"`)
  - `parallel` — fan-out with `for_each` and inner `steps[]`
  - `compound` — sequence of sub-steps that share a `depends_on` parent
- **Evidence:** `greenfield.lobster:51, 74, 81-82, 110, 117, 132, 174, 234, 261, 296, 336, 411, 437, 466, 568, 638, 645, 651-655, 711-720, 736, 798-799, 891-892, 1001, 1062, 1192, 1280` — every type appears multiple times. `code-delivery.lobster:38-40, 65-71, 73-80, 102-106, 144-145` confirms `agent`, `gate`, `loop`, `parallel`. `brownfield.lobster:144-150` confirms `sub-workflow`.
- **Confidence:** HIGH.

### BC-AUDIT-111: Step ordering is by `depends_on:` topological resolution (NOT array position)

- **Preconditions:** Steps array contains entries with `depends_on: [other-step-name, …]`.
- **Postconditions:** The orchestrator computes a topological sort of the `depends_on` DAG and dispatches steps in that order. Steps with empty `depends_on: []` are roots and run first. Steps may run in parallel iff they share no transitive dependency.
- **Evidence:** `greenfield.lobster:40-41` (root step `repo-initialization` with `depends_on: []`); `:55` (`factory-worktree-health depends_on: [repo-initialization]`); `:233-235` (gate depends on multiple parallel siblings: `[phase-1-spec-crystallization, architect-feasibility-review, prd-revision, …]`).
- **Confidence:** HIGH.

### BC-AUDIT-112: Steps SHALL declare `condition:` for conditional execution; condition is a string expression evaluated against scoped context

- **Preconditions:** Step is conditionally activated.
- **Postconditions:** `condition: "<expr>"` references upstream-step results, config keys, or feature flags. Operator semantics: `==`, `!=`, `in [...]`, `OR`, `AND`. Examples:
  - `condition: "feature_type in ['ui', 'full-stack']"`
  - `condition: "architect.verdict == 'request-changes'"`
  - `condition: "human_approved_multi_repo == true"`
  - `condition: "!file_exists('CLAUDE.md')"`
- **Evidence:** `greenfield.lobster:74` (`!file_exists`); `:128` (`architect.verdict ==`); `:176` (`feature_type in [...]`); `:393` (`human_approved_multi_repo == true`); `:421` (compound: `!= 'multi-service' OR human_approved_multi_repo == false`).
- **Confidence:** HIGH.

### BC-AUDIT-113: Failure handling — `on_failure: escalate` is the workflow default; per-step override via `on_failure: <action>`; `gate.fail_action: block` is the explicit blocking shape

- **Preconditions:** Step declares failure handling, OR step is a `gate` type.
- **Postconditions:** `on_failure: escalate` (default) — bubble to operator. Other observed values: gate steps use `fail_action: block` (halt the workflow on criteria miss). Steps may declare `optional: true` to make failure non-blocking. (Beyond `escalate` and `block`, no other `on_failure` values were observed in the sampled workflows.)
- **Evidence:** `greenfield.lobster:27` (defaults: `on_failure: escalate`); `:75` (`optional: true`); `:257` (gate: `fail_action: block`); `:495` (`fail_action: block`); `:614` (`fail_action: block`).
- **Confidence:** HIGH for `escalate` + `block` + `optional: true`; MEDIUM for the universe of values (sample limited to greenfield + brownfield + first 150 LOC of code-delivery; the broader retry/timeout-action mappings may exist in the 13 unread workflows).

### BC-AUDIT-114: `loop:` blocks are bounded; require `max_iterations` and `exit_condition`

- **Preconditions:** Step `type: loop`.
- **Postconditions:** `loop:` map declares `max_iterations: N` (typical: 10) and `exit_condition: "<expr>"` (typical: `adversary.verdict == 'CONVERGENCE_REACHED'`, `spec_reviewer.verdict == 'APPROVED'`). The orchestrator will iterate the inner `steps[]` until the exit condition fires OR `max_iterations` is reached. No infinite loops are permitted.
- **Evidence:** `greenfield.lobster:262-266` (`max_iterations: 10, exit_condition: "adversary.verdict == 'CONVERGENCE_REACHED'"`); `:301-303` (spec_reviewer); `:738-741` (pr_reviewer); `:815-817` (wave-adversarial); `:895-897` (integration-fix); `:1063-1066` (phase-5-adversarial); `code-delivery.lobster:104-106` (per-story adversarial review).
- **Confidence:** HIGH (consistent shape across 7 distinct loop sites).

### BC-AUDIT-115: `human-approval` steps declare `approval: { prompt, artifacts, timeout }`

- **Preconditions:** Step `type: human-approval`.
- **Postconditions:** `approval:` block contains `prompt:` (folded scalar string shown to operator), `artifacts:` (list of file globs the operator should review), and `timeout:` (typically `"24h"`, `"48h"`, or `"72h"`). The orchestrator pauses the workflow and only resumes after explicit operator approval inside the timeout.
- **Evidence:** `greenfield.lobster:181-191` (design-system-approval); `:201-211` (multi-variant-approval); `:336-355` (phase-1-human-approval, 24h timeout, 12 listed artifacts); `:1311-1322` (phase-6-human-approval, 48h timeout); `brownfield.lobster:170-188` (phase-0-human-approval, 24h timeout, 9 artifacts).
- **Confidence:** HIGH.

### BC-AUDIT-116: `agent` steps with `model_tier:` override the default agent model assignment

- **Preconditions:** Agent dispatch needs a stronger / different model than the agent's default.
- **Postconditions:** `model_tier:` accepts at least `adversary` and `review` tier-keys. `adversary` is used for adversary agent dispatch (e.g., GPT-5.4 for fresh-eyes review); `review` is used for code-reviewer / visual-reviewer (Gemini 3.1 Pro for secondary review).
- **Evidence:** `greenfield.lobster:269` (`model_tier: adversary` for adversary spec review); `:435` (story decomposition adversary); `:822` (wave adversary); `:1070` (phase-5 adversary); `:1113` (`model_tier: review` for code-reviewer); `:1289` (visual-reviewer review tier).
- **Confidence:** HIGH.

### BC-AUDIT-117: `agent` steps declare `context: { include: [...], exclude: [...] }` to enforce information-asymmetry walls

- **Preconditions:** A sub-agent should NOT have visibility into certain files (e.g., adversary cannot see prior adversarial findings; pr-reviewer cannot see `.factory/`; holdout-evaluator cannot see source).
- **Postconditions:** `context:` block declares glob includes/excludes. The exclude-list comments often use `▓ WALL:` prefix to mark deliberate information-asymmetry boundaries. Common walls: prior adversarial reviews, implementer notes, holdout scenarios, semport history, factory cycles.
- **Evidence:** `greenfield.lobster:276-289` (adversary spec review excludes `.factory/holdout-scenarios/**`); `:716-720` (PR reviewer excludes `.factory/**`); `:840-843` (wave adversary excludes prior adversarial reviews); `:1086-1095` (phase-5 adversary excludes implementer notes, prior adversarial reviews, semport); `:1128-1130` (Gemini code reviewer cannot see adversary findings).
- **Confidence:** HIGH (load-bearing pattern, used 12+ times in greenfield alone).

### BC-AUDIT-118: Sub-workflow invocation: `type: sub-workflow` with `sub_workflow: "<filename>.lobster"`

- **Preconditions:** Reusable workflow logic.
- **Postconditions:** Step declares `sub_workflow: "<file>.lobster"`; the orchestrator parses and inlines the referenced workflow's steps. Inputs flow via the parent's variable scope.
- **Evidence:** `greenfield.lobster:98-100` (planning.lobster sub-workflow); `:907-909` (code-delivery.lobster invoked from wave-integration-fix loop); `:1237-1239` (code-delivery as ui-fix-delivery); `brownfield.lobster:336-338` (greenfield.lobster invoked as sub-workflow); `:359-362` (multi-repo.lobster sub-workflow).
- **Confidence:** HIGH.

## 6. GAP-E findings — PluginCache invalidation + caching semantics

### BC-AUDIT-119: PluginCache key is `path` only; invalidation is mtime-driven

- **Preconditions:** A plugin path passed to `PluginCache::get_or_compile`.
- **Postconditions:** Cache lookup is by `PathBuf` only. Hit returns the cached `Module` IF the stored mtime matches the current file mtime (`probe()` re-stat'd on every call). On mtime mismatch OR first sight, recompile via `Module::from_binary(&engine, &bytes)` and replace the entry in place (cache size stays constant for the same path).
- **Evidence:** `plugin_loader.rs:56-84` (`probe()` reads metadata + modified()`; `entries.lock(); if mtime matches return cached; else read+compile+insert`).
- **Confidence:** HIGH.

### BC-AUDIT-120: PluginCache.get_or_compile is thread-safe via Mutex<HashMap>

- **Preconditions:** Concurrent calls.
- **Postconditions:** Internal storage is `Mutex<HashMap<PathBuf, (SystemTime, Module)>>`. Two readers serialize through the mutex; the lock is held briefly (single get + early return) on cache hits, and released before the file read + compile on cache miss (lock taken again to insert). `expect("plugin cache poisoned")` would panic if a prior holder panicked, but that's a never-in-practice case.
- **Evidence:** `plugin_loader.rs:43-46` (Mutex<HashMap>); `:62-69` (lock briefly on hit-check, drop, then read/compile, then re-lock to insert at `:81-83`).
- **Confidence:** HIGH.

### BC-AUDIT-121: PluginCache has no eviction policy — entries live for the dispatcher's process lifetime

- **Preconditions:** Long-running dispatcher process (today: per-event short-lived).
- **Postconditions:** No LRU, TTL, or memory-pressure eviction. Cache grows monotonically with distinct plugin paths seen. Acceptable today because:
  1. Dispatcher process is per-event (cold start; the cache never builds up beyond one event's plugin set).
  2. Plugin set is bounded (45 entries, all currently the same `legacy-bash-adapter.wasm`).
- **Evidence:** `plugin_loader.rs:43-91` — the only mutation method is `get_or_compile`. No `evict`, `clear`, `prune`. Test `compiles_on_first_use_and_caches` asserts size remains 1 after 2 calls to same path; `invalidates_on_mtime_change` asserts size stays 1 (replace, not grow).
- **Confidence:** HIGH.

### BC-AUDIT-122: Missing plugin path returns NotFound; corrupt bytes return Compile; IO errors carry path context

- **Preconditions:** Path doesn't exist / file unreadable / bytes are not valid wasm.
- **Postconditions:** Three distinct error variants: `PluginLoadError::NotFound(PathBuf)`, `PluginLoadError::Io { path, source }`, `PluginLoadError::Compile { path, source }`. Each carries the offending path so dispatcher diagnostics can name the bad plugin.
- **Evidence:** `plugin_loader.rs:18-34` (PluginLoadError variants); `:71-79` (Io and Compile arms); `:93-106` (probe returns NotFound); `:128-135` and `:174-181` (tests for NotFound and Compile errors).
- **Confidence:** HIGH.

## 7. GAP-F findings — Router (sinks/router.rs) actual behavior

### BC-AUDIT-123: Router is currently a thin pass-through wrapper around SinkRegistry

- **Preconditions:** The dispatcher's main does not yet call `Router::submit`.
- **Postconditions:** `Router::new(SinkRegistry)` wraps a registry; `submit(SinkEvent)` just delegates to `registry.submit_all(event)`; `flush()` to `registry.flush_all()`; `shutdown()` to `registry.shutdown_all()`. There is no extra logic today. The module's docstring explicitly notes "no call sites exist — see the `TODO(integration)` in `sinks::mod.rs`".
- **Evidence:** `sinks/router.rs:1-9` (docstring); `:33-47` (Router::new, submit, flush, shutdown — each is one-line delegation); `sinks/mod.rs:11-21` (the matching `TODO(integration)` saying main.rs hasn't been wired yet).
- **Confidence:** HIGH.

### BC-AUDIT-124: Router exists as the future extension point for S-4.x retry / circuit-breaker / batching / routing-tag enrichment

- **Preconditions:** Tier E stories S-4.4 through S-4.6 haven't shipped.
- **Postconditions:** The Router module documents the intended extension surface: "Stable extension point that S-4.x will graft retry / circuit-breaker / batching behavior in at this layer without touching the call sites or the driver implementations." Rust `pub fn registry(&self) -> &SinkRegistry` is exposed for tests but is a public surface that suggests future inspection use.
- **Evidence:** `sinks/router.rs:1-9, 17-22` (docstring + struct-level comment). Pass-6 DRIFT-002 / DRIFT-005 / story-coverage S-4.4..S-4.6 corroborate.
- **Confidence:** HIGH.

## 8. GAP-G findings — SDK host module (`hook-sdk/src/host.rs`, `hook-sdk/src/ffi.rs`)

### BC-AUDIT-125: Plugin-author API surface is `vsdd_hook_sdk::host::*`; raw FFI is private (`mod ffi;`)

- **Preconditions:** Plugin author writes against the SDK.
- **Postconditions:** Public re-exports: `host::log` + `host::log_info/log_warn/log_error`, `host::emit_event`, `host::session_id` / `dispatcher_trace_id` / `plugin_root` / `plugin_version` / `cwd`, `host::env`, `host::read_file`, `host::exec_subprocess`, plus types `LogLevel`, `HostError`, `SubprocessResult`. The `ffi` module is **not** re-exported — `hook-sdk/src/lib.rs:37` has `mod ffi;` (private). Plugin authors who reach into FFI directly bypass the type-safe wrappers.
- **Evidence:** `hook-sdk/src/lib.rs:37-47` (mod ffi; pub mod host;); `hook-sdk/src/host.rs:1-10` (docstring affirms "These are the surface plugin authors should use").
- **Confidence:** HIGH.

### BC-AUDIT-126: Bounded host calls are mandatory — `read_file` and `exec_subprocess` REQUIRE `timeout_ms` and a byte cap

- **Preconditions:** Plugin author calls `host::read_file` or `host::exec_subprocess`.
- **Postconditions:** Function signatures REQUIRE `max_bytes: u32` (or `max_output_bytes: u32`) and `timeout_ms: u32`. Caller cannot opt out of bounds at the type level — the SDK enforces the bounds in the compile-time signature; the dispatcher enforces them again at runtime (defense in depth).
- **Evidence:** `host.rs:184-205` (`read_file(path, max_bytes, timeout_ms)`); `:215-256` (`exec_subprocess(cmd, args, stdin, timeout_ms, max_output_bytes)`); `:1-10` (docstring asserts "Every wrapper is bounded — timeouts and byte caps are mandatory at the API level").
- **Confidence:** HIGH.

### BC-AUDIT-127: HostError code mapping: -1 = CapabilityDenied, -2 = Timeout, -3 = OutputTooLarge, -4 = InvalidArgument, other negative = Other(i32)

- **Preconditions:** Host call returns a negative `i32`.
- **Postconditions:** SDK calls `HostError::from_code(code)` to convert. Mapping is 1:1 to dispatcher-side `codes::*` constants (paired Pass 3 BC-AUDIT-023, -027, -029 — same numeric values).
- **Evidence:** `host.rs:81-106` (HostError enum + from_code); `:339-345` (test `host_error_code_mapping` asserts each variant). Compare to `factory-dispatcher/src/host/mod.rs::codes::*` per pass-1 architecture.
- **Confidence:** HIGH (compile-time-stable constants tested both sides).

### BC-AUDIT-128: SubprocessResult envelope decoding is paranoid — rejects truncated input rather than panicking

- **Preconditions:** Host-returned envelope bytes might be malformed (e.g., truncated by host buffer cap).
- **Postconditions:** `decode_subprocess_result` performs explicit `len < N` checks before each slice; returns `None` rather than panicking. Caller maps `None` to `HostError::Other(-99)`.
- **Evidence:** `host.rs:272-295` (decode_subprocess_result with explicit length checks); `:255` (caller maps None → Other(-99)); `:362-365` (test `decode_subprocess_result_rejects_truncated`).
- **Confidence:** HIGH.

### BC-AUDIT-129: SDK-side `read_string` re-call protocol — host returns required size; SDK reallocates and re-calls

- **Preconditions:** Host writes more bytes than `out_cap`.
- **Postconditions:** Convention is: if `written > buf.len()`, the host returns the required size; the SDK resizes the buffer to that size and re-calls the host fn with the new capacity. This 2-call protocol covers `session_id`, `dispatcher_trace_id`, `plugin_root`, `plugin_version`, `cwd`.
- **Evidence:** `host.rs:108-125` (read_string helper). The test surface is indirect (host stubs return 0 on non-wasm targets so the second-call path is exercised manually).
- **Confidence:** HIGH (logic explicit; test coverage is indirect).

### BC-AUDIT-130: SDK ffi.rs uses `#[link(wasm_import_module = "vsdd")]` on wasm32 targets, host stubs on others

- **Preconditions:** Crate built for `wasm32-wasip1` (production) or any other arch (tests).
- **Postconditions:** wasm32: `unsafe extern "C"` block with `pub safe fn` items linked from import module `vsdd`. Non-wasm32: `host_stubs` module with no-op stub fns so unit tests link. Host stubs for capability-bearing fns return -1 (CapabilityDenied) so test paths see the expected error variant.
- **Evidence:** `ffi.rs:13-58` (wasm32 imports); `:62-127` (non-wasm host_stubs with -1 returns for read_file / exec_subprocess / env).
- **Confidence:** HIGH.

## 9. GAP-H findings — legacy-bash-adapter stderr / exit-code edge cases (incl. v1.0.0-beta.4 fix)

### BC-AUDIT-131: Adapter forwards stdout AND stderr to host log via `host::log_info` / `host::log_warn` (per-stream, non-empty)

- **Preconditions:** Bash subprocess emitted any stdout or stderr.
- **Postconditions:** Adapter calls `host::log_info(format!("legacy-bash[{path}] stdout: {stdout}"))` if stdout non-empty; `host::log_warn(format!("legacy-bash[{path}] stderr: {stderr}"))` if stderr non-empty. Level mapping is intentional: stdout→info (debug-level chatter), stderr→warn (operational signal). Both are dropped on the floor if empty (no noise).
- **Evidence:** `legacy-bash-adapter/src/lib.rs:151-158` (the post-exec_subprocess forwarding loop). This is the v1.0.0-beta.4 stderr-capture wiring (CHANGELOG entry "Plugin stderr capture on lifecycle events").
- **Confidence:** HIGH (stream-routing logic is direct and stream-symmetric).

### BC-AUDIT-132: Adapter exit-code mapping: 0 → Continue, 2 → Block (reason=first stderr line OR synthetic), other → Error (message includes script path + code + stderr)

- **Preconditions:** Bash subprocess exits with code N.
- **Postconditions:**
  - `N == 0` → `HookResult::Continue`
  - `N == 2` → `HookResult::Block { reason }` where reason = first non-empty stderr line, OR `"legacy bash hook {script_path} blocked"` synthetic if stderr empty
  - Other N → `HookResult::Error { message }` where message = `"legacy bash hook {script_path} exited with code {N}: {stderr}"`
- **Evidence:** `legacy-bash-adapter/src/lib.rs:103-119` (the exit-code match arms). 5 unit tests pin every arm (`maps_exit_zero_to_continue`, `maps_exit_two_to_block_with_first_stderr_line`, `maps_exit_two_with_no_stderr_to_synthetic_block_reason`, `maps_other_nonzero_to_error_with_stderr`, `surfaces_runner_error_as_hook_error`).
- **Confidence:** HIGH (5 distinct test cases pin every branch).

### BC-AUDIT-133: Adapter's plugin_config.script_path validation is checked BEFORE any subprocess invocation

- **Preconditions:** Registry entry routes through legacy-bash-adapter.
- **Postconditions:** First action of `adapter_logic` is `payload.plugin_config.get("script_path")`. If missing → return `HookResult::Error` with verbose hint mentioning `[hooks.config] script_path`. If present but not-a-string → return Error mentioning "non-empty string". If empty string → return Error mentioning "non-empty". `run_bash` is never called when validation fails (tests use `panic!("must not run")` to confirm this guarantee).
- **Evidence:** `legacy-bash-adapter/src/lib.rs:62-77` (validation block); `:218-247` (3 tests with `panic!("must not run")` runner closures asserting bash is never invoked on validation failure).
- **Confidence:** HIGH.

### BC-AUDIT-134: Adapter strips plugin_config to Null before piping to bash — bash hooks predate the field

- **Preconditions:** Adapter has resolved script_path; about to pipe payload.
- **Postconditions:** Adapter clones `payload`, sets `bash_payload.plugin_config = Value::Null`, then `serde_json::to_vec(&bash_payload)` is what's piped. Original `payload.plugin_config` is preserved on the adapter's stack but never reaches the bash hook. Mirrors pass-3 BC-AUDIT-055 with full source provenance.
- **Evidence:** `legacy-bash-adapter/src/lib.rs:81-90` (clone + null + serialize); `:298-331` (test `passes_payload_bytes_to_bash_with_plugin_config_stripped` parses round-tripped JSON and asserts `plugin_config: null` while preserving `event_name`, `dispatcher_trace_id`).
- **Confidence:** HIGH.

### BC-AUDIT-135: Adapter resolves relative `script_path` under `${CLAUDE_PLUGIN_ROOT}`; absolute paths bypass the join

- **Preconditions:** `script_path` is non-empty.
- **Postconditions:** `is_absolute(p)` is `true` if `p.starts_with('/')` (Unix root) OR `p[1] == ':'` (Windows drive letter, e.g., `C:/...`). Absolute → use as-is. Relative → `join_path(plugin_root, script_path)` which inserts a `/` separator if needed, respects existing trailing separator, and returns `rel` as-is when root is empty.
- **Evidence:** `legacy-bash-adapter/src/lib.rs:135-186` (run_bash_via_host + is_absolute + join_path); `:347-376` (5 unit tests pinning Unix root, Windows drive, separator insertion, trailing separator, empty root).
- **Confidence:** HIGH.

### BC-AUDIT-136: Adapter's wall-clock cap (BASH_TIMEOUT_MS = 60_000) is a backstop; real per-call deadline = dispatcher's epoch-interruption (default 5_000ms)

- **Preconditions:** Bash hook hangs.
- **Postconditions:** The 60s adapter cap is intentionally MUCH larger than the dispatcher's per-plugin epoch deadline (default 5s, per pass-3 BC-AUDIT-009). The dispatcher will terminate the wasm instance via epoch interrupt long before the adapter's bash subprocess timeout fires. The adapter cap exists as a safety net for the rare case where the dispatcher's epoch deadline doesn't fire (theoretically: misconfigured ticker).
- **Evidence:** `legacy-bash-adapter/src/lib.rs:48-54` (BASH_TIMEOUT_MS = 60_000 with comment "epoch interrupt is the source of truth — the bash timeout is a backstop"). Pass-3 BC-AUDIT-009 confirms epoch precedence.
- **Confidence:** HIGH.

## 10. GAP-I findings — sink-otel-grpc batching + flush behavior

### BC-AUDIT-137: Batch trigger thresholds are independent — `size` (default 100) AND `interval_ms` (default 5000); first-to-fire flushes

- **Preconditions:** Worker accumulates events.
- **Postconditions:** Two independent triggers:
  - **Size:** `buffer.len() >= batch_size` after pushing → flush.
  - **Time:** `tokio::time::sleep_until(buffer_deadline)` fires at `Instant::now() + batch_interval` (set on the FIRST event of a buffer) → flush.
  Whichever fires first triggers `flush_buffer`, which clears the buffer and resets the deadline. An idle worker has `buffer_deadline = None` (no timer running until the next event arrives).
- **Evidence:** `sink-otel-grpc/src/lib.rs:486-549` (worker_loop with `tokio::select! { msg = recv_fut => …, _ = sleep_until(deadline) => flush_buffer; continue }`); `:519-534` (size-trigger after push). DEFAULT_BATCH_SIZE = 100 / DEFAULT_BATCH_INTERVAL_MS = 5000 declared at `:78-83`.
- **Confidence:** HIGH.

### BC-AUDIT-138: Send failure protocol — drop the gRPC client on error; rebuild on next batch (self-healing transient blips)

- **Preconditions:** Previously-built `LogsServiceClient` returns `Err(tonic::Status)` on `client.export(request).await`.
- **Postconditions:** `*client_slot = None` (drop the client), then `record_failure(shared, endpoint, batch_len, format!("{status}"))`. The next `flush_buffer` invocation sees `client_slot.is_none()` and rebuilds via `build_client(endpoint).await`. h2 connections that "soured silently after a peer reset" self-heal on the next batch — the comment explicitly says "the cheapest fix is reconnect-on-error."
- **Evidence:** `sink-otel-grpc/src/lib.rs:598-604` (drop + record_failure on export error); `:582-590` (lazy reconnect on next call).
- **Confidence:** HIGH.

### BC-AUDIT-139: Connection lifecycle — endpoint validated EAGERLY at constructor; channel built LAZILY in worker on first send

- **Preconditions:** `OtelGrpcSink::new(config)` called.
- **Postconditions:** Constructor validates `tonic::transport::Endpoint::from_shared(endpoint)` SHAPE (returns `OtelGrpcError::InvalidEndpoint` on parse failure); spawns the worker thread; returns. The channel itself is built inside `flush_buffer` on first send via `build_client(endpoint).await` with `connect_timeout(5s)` and `timeout(10s)`. Failures here record `SinkFailure { endpoint, batch_size, reason: "connect: ..." }` — the buffer is dropped, NOT retried inline.
- **Evidence:** `sink-otel-grpc/src/lib.rs:285-291` (constructor validates endpoint shape via Endpoint::from_shared); `:607-619` (build_client with 5s connect, 10s call timeout); `:583-590` (lazy build in flush_buffer).
- **Confidence:** HIGH.

### BC-AUDIT-140: Worker thread owns its own current_thread tokio runtime on a dedicated OS thread

- **Preconditions:** Sink is constructed.
- **Postconditions:** `std::thread::Builder::new().name(format!("sink-otel-grpc:{name}")).spawn(move || { let runtime = Builder::new_current_thread().enable_all().build()?; runtime.block_on(worker_loop(...)) })`. Failure to build the runtime records a single `SinkFailure { reason: "failed to build tokio runtime: …" }` and exits the thread cleanly. The dispatcher's synchronous main never sees a tokio dependency. Pass-6 DRIFT-003 documents the design intent to migrate to a shared dispatcher runtime once S-1.6 lands; the migration has not happened (DRIFT-003 still active).
- **Evidence:** `sink-otel-grpc/src/lib.rs:307-335` (worker thread spawn); `:309-333` (runtime build and block_on).
- **Confidence:** HIGH.

### BC-AUDIT-141: Producer-side `submit` is fully non-blocking via `try_send`; overflow increments `queue_full_count` (atomic, monotonic)

- **Preconditions:** Internal mpsc bounded at `queue_depth.max(1)` (default 1000).
- **Postconditions:** `Sink::submit` calls `sender.try_send(Message::Event(enriched)).is_err()` → if Err, `shared.queue_full_count.fetch_add(1, Relaxed)`. No producer-side blocking, no panic, no allocation beyond the event itself. Tests read `queue_full_count` to verify backpressure.
- **Evidence:** `sink-otel-grpc/src/lib.rs:406-419` (Sink::submit impl with try_send + queue_full_count); `:351-355` (public `queue_full_count()` accessor).
- **Confidence:** HIGH.

### BC-AUDIT-142: `flush()` is a synchronous oneshot round-trip; producer blocks on `rx.blocking_recv()` until the worker drains

- **Preconditions:** Producer calls `Sink::flush()`.
- **Postconditions:** Producer creates `(tx, rx)` oneshot; sends `Message::Flush(tx)` to the worker; `rx.blocking_recv()` blocks the producer thread until the worker handles the flush message, calls `flush_buffer`, and `signal.send(())`. Errors:
  - Try-send failure on the mpsc channel → `Err(anyhow!("sink '{name}' flush channel full or closed"))`
  - Worker dropped the oneshot (rare; only if the worker exited mid-flush) → `Err(OtelGrpcError::FlushLost)`
- **Evidence:** `sink-otel-grpc/src/lib.rs:421-438` (flush impl); `:536-547` (worker_loop's Flush arm: drains buffer then `signal.send(())`).
- **Confidence:** HIGH.

### BC-AUDIT-143: Shutdown drains and joins the worker thread; idempotent post-`accepts` rejection

- **Preconditions:** Drop-or-explicit-shutdown.
- **Postconditions:** `shared.shutdown.store(true, Release)`; sender lifted out of its mutex (channel closes when last sender drops); worker `JoinHandle` taken and `join()`-ed. After shutdown, `Sink::accepts` returns false (early return on `shared.shutdown.load(Acquire)`); `Sink::submit` is a no-op when the sender is None. `Drop` trampolines to `shutdown()` if the worker is still active — this guarantees the worker drains pending events before the producer's main exits.
- **Evidence:** `sink-otel-grpc/src/lib.rs:440-461` (shutdown impl + Drop); `:392-403` (accepts checks shutdown flag); `:406-419` (submit checks accepts via early return).
- **Confidence:** HIGH.

## 11. Cross-cutting findings (not specific to any single GAP)

### CCF-1: Reserved-fields top-level attribute set is consistent across emit_event (BC-AUDIT-034) and otel-grpc serialization

The dispatcher-side `emit_event` host fn filters reserved field names from plugin payload (BC-AUDIT-034). The otel-grpc sink's `event_to_log_record` (sink-otel-grpc/src/lib.rs:638-643) declares its OWN list of "RESERVED_TOP_LEVEL_ATTRS = [dispatcher_trace_id, session_id, plugin_name, plugin_version]" for OTLP attribute promotion — but this set is a SUBSET of the broader emit_event reserved list (`[dispatcher_trace_id, session_id, plugin_name, plugin_version, ts, ts_epoch, schema_version, type]`). The OTel sink intentionally promotes only the 4 correlation-bearing fields to top-level OTLP `KeyValue` attributes; `ts`, `ts_epoch`, `schema_version`, `type` are handled separately (`type` → record body, `ts_epoch` → `time_unix_nano`, `ts` and `schema_version` flatten). No conflict; documenting the intentional asymmetry. Source: sink-otel-grpc/src/lib.rs:632-718.

### CCF-2: Pass 3's BC-AUDIT-068 monolith is now decomposable into Pass-Round-1 class BCs

BC-AUDIT-068 (pass 3) was a single composite BC standing in for "the validate-* family." This deepening round provides the **class** decomposition (BC-AUDIT-099 through BC-AUDIT-107) — the validators all share these contracts. Per-validator instance BCs are NOT in scope this round (would balloon to ~70 BCs at 3 per validator); a future round may sample 5-8 validators for instance-level pinning, but the class contracts are the load-bearing story.

### CCF-3: Workflow .lobster files implicitly enforce information-asymmetry walls via `context.exclude` (12+ wall sites in greenfield alone)

The `▓ WALL:` prefix convention in workflow comments is load-bearing — it marks deliberate context-restriction boundaries that the orchestrator MUST respect. Walls observed: pr-reviewer ⊥ `.factory/`; adversary ⊥ prior adversarial reviews; holdout-evaluator ⊥ source code; semport agents ⊥ `.factory/semport/**`; visual-reviewer ⊥ implementer notes. This pattern is one of vsdd-factory's distinctive architectural innovations and deserves its own BC (deferred to next round if specific walls become acceptance criteria).

### CCF-4: `legacy-bash-adapter` stderr-capture fix (CHANGELOG v1.0.0-beta.4) is structurally simple — info/warn fork at lines 153-158

The fix that motivated the v1.0.0-beta.4 release is a single 6-line block in `legacy-bash-adapter/src/lib.rs:151-158`. Before the fix, only stdout was forwarded; stderr was dropped on the floor. Now both are forwarded with stream-appropriate severity (stdout→info, stderr→warn). The fix has no dedicated test; it's covered indirectly by the existing `passes_payload_bytes_to_bash_with_plugin_config_stripped` and `maps_other_nonzero_to_error_with_stderr` tests. Future round MAY add an instance-level BC if test coverage of the stderr-routing branch is added.

## 12. Delta summary

**New BCs added this round (BC-AUDIT-087 .. BC-AUDIT-143):** 57 total

- GAP-A (Skill class contracts): **12** new (BC-AUDIT-087..098)
- GAP-B (Validator class contracts): **9** new (BC-AUDIT-099..107)
- GAP-C (Workflow protocol contracts): **11** new (BC-AUDIT-108..118)
- GAP-E (PluginCache): **4** new (BC-AUDIT-119..122)
- GAP-F (Router): **2** new (BC-AUDIT-123..124)
- GAP-G (SDK host module): **6** new (BC-AUDIT-125..130)
- GAP-H (legacy-bash-adapter): **6** new (BC-AUDIT-131..136)
- GAP-I (sink-otel-grpc batching): **7** new (BC-AUDIT-137..143)

**Cross-cutting findings:** 4 (CCF-1..CCF-4) — observations not yet promoted to BCs.

**Existing items refined:** 0 (round 1 is additive — no prior BC numbering rewritten or contradicted).

**Retracted (CONV-ABS markers):**

- **CONV-ABS-1:** `verify-sha-currency.sh` is a TEMPLATE (`templates/verify-sha-currency.sh`), not a registered hook. Pass 3 BC-AUDIT-068 erroneously listed it.
- **CONV-ABS-2:** `validate-anchor-capabilities-union` was duplicated in Pass 3 BC-AUDIT-068's enumeration (listed twice).
- **CONV-ABS-3:** Same-basename conflation between the template and the (nonexistent) hook of the same name was the root cause of CONV-ABS-1.

**Net validator count correction:** Pass 3 said "24+"; actual is 23 unique scripts (22 `validate-*.sh` + 1 `verify-git-push.sh`).

**Remaining gaps for next round:**

1. **GAP-A residual:** Sample 6-10 skills NOT in the 9-skill base used here (e.g., `factory-dashboard`, `state-update`, `wave-status`, `decompose-stories`, `discovery-engine`, `disposition-pass` — test the class contracts hold across "data-product" and "research" skill categories).
2. **GAP-C residual:** 13 `.lobster` files NOT yet read (especially `feature.lobster`, `multi-repo.lobster`, `maintenance.lobster`, and the 8 `phases/phase-N-*.lobster` sub-flows). Workflow class contracts above are derived from greenfield + brownfield + first 150 LOC of code-delivery only; phase sub-flows may introduce step types or `on_failure` values not yet observed.
3. **CCF-3 promotion:** Information-asymmetry walls (`▓ WALL:` convention) merit their own dedicated BC class — what guarantees the orchestrator delivers? Are walls per-step or per-agent?
4. **NEW (discovered this round):** What is the contract for a skill's `disable-model-invocation: true` declaration? Is the enforcement Claude-Code-side or vsdd-factory-side? If Claude-Code-side, what version added it? (Frontmatter is declarative; the enforcement mechanism is not visible in source.)
5. **NEW:** `sub-workflow` invocation must pass inputs (`code-delivery.lobster:21-27` declares typed `inputs`); how do these resolve from the parent workflow's variable scope? No source for the resolution rule.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **Round** | 1 |
| **Novelty score** | SUBSTANTIVE |
| **Trajectory** | First deepening round on Pass 3. Broad sweep produced 86 BCs; this round produced 57 net-new BCs (BC-AUDIT-087..143) + 3 retractions of broad-sweep errors (CONV-ABS-1..3). Findings change the model of three previously-thin areas: (a) skill execution is now a class contract with frontmatter + lifecycle + hard-gate semantics, not 119 opaque procedures; (b) validator hooks are a uniform class with stdin-jq-stderr-emit-event invariants, not a flat list of 24 names; (c) workflow `.lobster` files have a first-class 8-type step taxonomy, dependency DAG semantics, and 12-site wall convention. The 3 CONV-ABS retractions correct broad-sweep miscounts that downstream skills (create-prd / create-domain-spec) would have inherited as structural truth. |
| **Verdict** | FINDINGS_REMAIN |

**Verdict justification:** Would removing this round's findings change how downstream skills would spec the system? YES. The class contracts for skills/validators/workflows are the difference between treating Subsystem B as 119+24+16 = 159 opaque entities vs. 3 small contract sets that govern all of them.

**Convergence declaration:** Another round needed — substantive gaps remain (see Remaining gaps for next round, items 1-5).

## 14. Remaining gaps for next round (verbatim, for orchestrator carryover)

### Carryover 1 — Subsystem B class-BC residual coverage

- **GAP-A-r2:** Verify class contracts (BC-AUDIT-087..098) hold for skills NOT in the 9-skill round-1 sample. Target: 6-10 skills from "data-product" (`factory-dashboard`, `factory-obs`, `wave-status`) and "research / introspection" (`disposition-pass`, `discovery-engine`, `decompose-stories`, `factory-health`) categories. If the class contracts hold → declare class-level convergence on GAP-A. If a new variant is found → add new class BCs.
- **GAP-C-r2:** Read the remaining 13 `.lobster` files (especially `feature.lobster`, `multi-repo.lobster`, `maintenance.lobster`, `discovery.lobster`, `planning.lobster`, and the 8 `phases/phase-N-*.lobster`) to confirm step-type taxonomy (BC-AUDIT-110), failure-handling values (BC-AUDIT-113), and check for unused step types or sub-workflow input/output mechanisms.

### Carryover 2 — Information-asymmetry walls (newly surfaced, CCF-3)

The `▓ WALL:` convention in workflow `context.exclude` declarations is structurally load-bearing. Round 2 should:
- Catalog all wall sites across the 13 unread workflow files.
- Derive a class BC for "walls are absolute — orchestrator MUST honor exclude globs."
- Identify which agent identities are wall-protected and from which artifacts.

### Carryover 3 — Newly discovered open questions

- **OPEN-Q1:** `disable-model-invocation: true` enforcement boundary — Claude-Code-runtime or vsdd-factory-side? If runtime, what version of Claude Code introduced it? If dispatcher-side, where is the gate?
- **OPEN-Q2:** `sub-workflow` typed `inputs:` resolution — how does `code-delivery.lobster:21-27`'s declared inputs (`story_id`, `worktree_path`, `feature_type`, `module_criticality`, `implementation_strategy`) bind from parent workflow scope? No source for the resolution rule visible in the workflows themselves.

### Carryover 4 — Subsystem A residual

Pass 3 round 1 closed all Subsystem A gaps the orchestrator scoped (E, F, G, H, I). Subsystem A is now densely covered. Round 2 may consider:
- Promote CCF-1 (reserved-attribute-set asymmetry between emit_event and otel-grpc) to its own BC if downstream OTel ingestion code expects parity.
- Add a stderr-routing BC for `legacy-bash-adapter:151-158` tied to the v1.0.0-beta.4 release evidence, with a minimum repro test request as part of the BC.

### Carryover 5 — Out-of-scope reconfirmation

GAP-D (template-skill cross-walk), GAP-J (bin-tool I/O contracts), GAP-K (docs/guide currency), GAP-L (CI workflows), GAP-M (rules/) remain deferred per orchestrator scoping. Round 2 of Pass 3 deepening should NOT pursue these unless the orchestrator widens scope.

## State Checkpoint

```yaml
pass: 3
round: 1
status: complete
deepening_phase: B
new_bcs: 57
new_bc_range: BC-AUDIT-087..BC-AUDIT-143
cross_cutting_findings: 4
retractions: 3 (CONV-ABS-1, CONV-ABS-2, CONV-ABS-3)
metric_recount: confirmed (10226 LOC, 119 skills, 44 hooks, 16 workflows, 45 registry entries, 23 validators)
files_read_full: 11 Rust source files + 9 SKILL.md (top sections) + 7 hook scripts + 3 workflow files + 1 registry sample + 1 lobster-parse helper
timestamp: 2026-04-25
novelty: SUBSTANTIVE
next_action: Phase B Round 2 — sample beyond 9-skill base, read remaining 13 workflow files, promote CCF-3 (walls) to BC class
```
