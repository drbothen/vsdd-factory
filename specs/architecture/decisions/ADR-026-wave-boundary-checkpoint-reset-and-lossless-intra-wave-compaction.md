---
document_type: architecture-decision-record
level: L3
adr_id: ADR-026
version: "1.1"
status: accepted
producer: architect
timestamp: 2026-06-14T00:00:00Z
title: "ADR-026: Wave-boundary checkpoint+reset and lossless intra-wave compaction"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
anchors:
  - SS-04
  - SS-05
  - SS-06
  - SS-07
  - issue-173
subsystems_affected:
  - SS-04
  - SS-05
  - SS-06
  - SS-07
supersedes: null
superseded_by: null
decision_status: accepted
human_gate_required: false
human_gate_reason: "All open questions from F1 (OQ-18-001 through OQ-18-004) are resolved by LOCKED DECISIONS D1–D5 confirmed by human prior to F2. Harness-version precondition documented. No remaining human-gated questions. Implementation dispatch ready via E-18 story decomposition."
last_amended: "2026-06-14 (v1.1) — F2 adversarial-pass-1 revision: (F-1) Re-anchored wave-identity to real substrate: HANDOFF.md `wave_id` derives from sprint-state.yaml `current_wave` (product-pipeline context) OR STATE.md `phase:`/`current_step:` (engine-self context); Decision 2 HANDOFF.md schema `wave_id` source clarified to both contexts; precompact-flush.sh reads STATE.md frontmatter `current_cycle:` field for engine-context identification; downstream BC anchor text updated in propagation worklist. (F-2) Reconciled with ADR-025 opt-in lock model: `factory_lock` is absent-by-default; flush lock-renewal step is no-op when `factory_lock` absent in STATE.md; Decision 2 `factory_lock_holder` field is nullable (null OR absent = no lock held, flush may proceed); Decision 6 hermetic-flush lock-renew step annotated with no-op-when-unlocked. (F-3) wave-state.yaml `next_wave_stories` real derivation specified: derives from sprint-state.yaml story `status: pending` OR `status: draft` entries ordered by dependency graph, NOT from a non-existent story `wave:` frontmatter field; empty list = hard error, not silent no-op (SOUL.md #4). (F-4) Bounded timeout_ms added to PreCompact hook registration (timeout_ms = 30000); timeout semantics specified: timeout is treated as `on_error = continue` (fail-open, compaction proceeds); git push failure (exit 2 returned cleanly) is distinct from timeout (no response) and is treated as a blocking failure. (F-5) Harness-version runtime assertion wired to check-state-health skill and SessionStart advisory; specific degrade behavior on pre-v2.1.105 documented honestly. (F-6) PostCompact re-anchor (BC-7.07.002) is EXPLICITLY best-effort and is removed from the CAP-032 continuity-guarantee chain; continuity guarantee rests exclusively on Part A (HANDOFF.md verified) and Part B (PreCompact flush). (F-7) All TOML registration blocks corrected to real registry schema: plugin = `hook-plugins/legacy-bash-adapter.wasm` with full entry fields; tool matcher canonical order `Edit|Write` (where applicable). (F-9) ADR-026 `anchors:` list: SS-08 NOT included because S-18.07 terminology changes target SS-06/SS-08 documentation but the architectural decision itself sits in SS-07 (shell hooks) and SS-04 (plugin ecosystem); explicit no-BC justification for S-18.07 added; S-18.06 `validate-heavy-op-delegation` WASM gate BC deferred — S-18.06 is advisory-only in v1 (no blocking behavior, no BC needed for v1 advisory; product-owner to author BC if v2 promotes to blocking). (F-10) VP-086 allocated: dispatcher exit-2 propagation for PreCompact (BC-1.15.001 PC4); VP-INDEX v2.07→v2.08. (F-11) 83% clamp claim downgraded to MEDIUM-confidence research finding; headroom argument annotated as independent of F-4 bounded-timeout fix; settings.json env-verification note added. (F-15) Prerequisite-verification discipline applied: real fields/artifacts the design depends on are enumerated with existence checks specified. [Prior: 2026-06-14 (v1.0) — Initial ADR authored in F2 spec-evolution phase for E-18 (CAP-032 context-durability). Records architecture decisions for wave-boundary checkpoint/reset (Part A), PreCompact synchronous flush (Part B), and WASM/shell hook split (Part C). Records D3/D4/D5 v2 deferrals explicitly. Closes OQ-18-001 through OQ-18-004.]"
---

# ADR-026: Wave-boundary checkpoint+reset and lossless intra-wave compaction

## Status

**ACCEPTED — all open questions resolved via LOCKED DECISIONS D1–D5 (human-authorized F2 gate). F2 adversarial-pass-1 revision (v1.1) complete. E-18 story decomposition may proceed (F3). Implementation dispatch ready.**

This ADR resolves the architecture for issue #173: enforced wave-boundary checkpoint/reset (Part A), synchronous PreCompact flush for lossless intra-wave compaction (Part B), and the PreToolUse delegation guard (Part C). All four open questions from F1 (OQ-18-001 through OQ-18-004) are answered by the locked decisions recorded here.

---

## Context

### The gap: context-window loss is currently unremediated

The factory externalizes durable pipeline state to `STATE.md` and the `factory-artifacts` orphan branch, but does not use that externalized state as a *deliberate context-management* mechanism. Long autonomous runs exhaust the context window. When the Claude Code harness triggers auto-compaction mid-wave, the summarizer runs without any factory-side coordination: critical SHAs, active decisions, BC identifiers, and open-findings lists may be silently dropped or hallucinated.

Two independently verified failure modes motivate this ADR:

1. **Fabricated-SHA risk:** The jira-cli sequence documented in issue #170 (and research file `issue-173.md`) shows that a hallucinated SHA in STATE.md can survive a compaction-summarization and re-enter the next turn as authoritative state. The PreCompact flush (Part B) closes this by committing the real, verified SHA to `factory-artifacts` before compaction can run.

2. **Cross-wave continuity collapse:** Without an enforced wave-boundary handoff, the in-context state at wave N close is the only record of what must be carried forward to wave N+1. A session reset (for any reason) after wave N closes but before the handoff is written loses that record entirely. The wave-boundary checkpoint (Part A) closes this by requiring a verified `HANDOFF.md` on `factory-artifacts` before a wave can be considered closed.

### Prior art in this codebase

The building blocks exist:
- `state-burst` skill: single-commit push to `factory-artifacts` — this is the flush primitive (SS-06).
- `wave-gate` skill: already gates wave close on multiple prerequisites (SS-06). Reads `sprint-state.yaml` as authority for story status per wave.
- `sprint-state.yaml` at `.factory/stories/sprint-state.yaml`: current authoritative source for story status (merged/ready/draft/partial/withdrawn) and epic membership.
- `factory-artifacts` orphan branch: already the durable external state store (SS-05).
- `hooks-registry.toml` plus the `legacy-bash-adapter` pattern: established model for shell hooks (SS-07/SS-04).
- WASM plugin fleet: established model for deterministic parse-heavy gate validators (SS-04).
- ADR-025 (single-writer lock/lease): provides the `factory_lock` frontmatter block (absent = no lock held) and `state-burst` renewal step — Part B's flush MUST invoke lock renewal before committing WHEN a lock is held; when `factory_lock` is absent, the renewal step is a no-op.

### Real state substrate (F-1 / F-15 re-anchor)

The design anchors to TWO real substrates depending on pipeline context:

| Context | Wave/Phase identity source | Story sequence source |
|---------|--------------------------|----------------------|
| **Product pipeline** (wirerust, jira-cli, engineering-report — products the factory builds) | `sprint-state.yaml` `current_wave`-equivalent grouping derived from dependency order produced by `wave-scheduling` skill | `sprint-state.yaml` story entries with `status: pending` or `status: draft` ordered by dependency graph |
| **Self-referential engine** (vsdd-factory's own STATE.md) | STATE.md frontmatter `current_cycle:` + `phase:` + `current_step:` fields | Story-INDEX.md + sprint-state.yaml hybrid (same `.factory/stories/sprint-state.yaml` path) |

**No `current_wave:` field is invented.** No `wave:` frontmatter on story files. The design reads the real existing fields.

For the `precompact-flush.sh` hermetic flush (Part B): the hook determines context identification from STATE.md `current_cycle:` + `current_step:` fields (always present). The HANDOFF.md `wave_id` is a logical identifier that the `wave-handoff` skill derives from the sprint-state.yaml wave-group numbering for product pipelines, OR from the cycle pass number (e.g., the engine's `pass-N` in `current_step:`) for the self-referential engine.

### Confirmed harness capability (F1 research)

The Claude Code harness (v2.1.105+) supports:
- `PreCompact` hook: fires before context compaction; can block via `exit 2` or `{"decision":"block","reason":"..."}`.
- `PostCompact` hook: fires after compaction; cannot block (advisory only).
- `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` env var: triggers auto-compaction earlier (effective band ≤ approximately 83% internal ceiling per MEDIUM-confidence research; see Decision 5 note).
- Sub-agent isolation: sub-agent tool-call history stays in the sub-agent's context; only the final summary reaches the parent.

**Runtime precondition:** The PreCompact blocking capability requires Claude Code harness version ≥ v2.1.105. On earlier versions, PreCompact fires as a notification-only hook (no veto). This ADR documents this as a hard runtime precondition. The factory cannot assume an older harness is sufficient for Part B.

**Honest degrade behavior on pre-v2.1.105 (F-5):** On pre-v2.1.105, the `precompact-flush.sh` hook fires as a notification; `exit 2` is visible to the user as stderr output but does NOT block compaction. The flush runs (state is written to `factory-artifacts`) but cannot prevent the context window from being replaced. The CAP-032 continuity guarantee is NOT satisfied on pre-v2.1.105. This is not a safe degradation — it is a reduction to best-effort only. Operators on pre-v2.1.105 must be warned.

**Settings.json env-verification (F-11):** The `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` setting requires verification in the active `settings.json`. The `check-state-health` skill must verify this env var is present with value `70` in `settings.json` and emit an advisory if absent.

---

## Prerequisite Verification Discipline (F-15)

The design depends on the following real artifacts and fields. S-18.00 (prerequisite verification story) MUST verify each exists and carries the needed data before E-18 implementation proceeds:

| Artifact / Field | Required content | Verification action |
|-----------------|-----------------|---------------------|
| `plugins/vsdd-factory/hooks-registry.toml` | Parseable TOML; `[[hooks]]` array present | `cargo test --workspace` passes; registry loads without error |
| `.factory/stories/sprint-state.yaml` | Contains `stories:` list with status-tagged entries | `grep -n 'status:' .factory/stories/sprint-state.yaml` returns non-empty |
| STATE.md frontmatter `current_cycle:` | Present, non-empty string | S-18.00 inspection of `head -5 .factory/STATE.md` |
| STATE.md frontmatter `phase:` | Present, non-empty string | S-18.00 inspection |
| `crates/factory-dispatcher/src/` | Contains event-routing logic; `PreCompact`/`PostCompact` enumerated OR absent (S-18.00 determines which) | S-18.00 reads source and documents finding |
| `plugins/vsdd-factory/bin/factory-cas-push.sh` | Exists and is executable | `test -x plugins/vsdd-factory/bin/factory-cas-push.sh` |
| ADR-025 lock model (`factory_lock` block in STATE.md) | `factory_lock:` block present = lock held; absent = no lock; null holder = no lock | Confirmed by ADR-025 §Decision 2 and ADR-025 §D2 canonical form |

---

## Decisions

### Decision 1 — Part A is primary; Part B is the safety net

**Decision:** Wave-boundary hard session reset is the PRIMARY cross-wave continuity mechanism. Intra-wave PreCompact flush (Part B) is the SAFETY NET for compactions that happen mid-wave before a wave boundary is reached.

**Rationale:** External research consensus (Anthropic context-engineering blog; Microsoft Agent Framework; multiple secondary sources cited in `issue-173.md`) favors hard session resets at wave boundaries over continuous compaction for pipelines with externalized durable state. Resets avoid stacking multiple lossy summarization passes. The precondition for preferring reset over compaction — that durable external state exists and is reliable — is already met by the factory's `factory-artifacts` branch. The PreCompact flush ensures no in-wave state is lost on compaction events that occur between wave boundaries.

**Alternative rejected:** Treating auto-compaction as reliable enough on its own (without PreCompact flush). The documented absence of retention-steering capability (research finding #3 in `research-precompact-gating-2026-06-13.md`) means the summarizer can silently drop any fact. The flush is a required safety net.

---

### Decision 2 — HANDOFF.md on factory-artifacts as the verified wave-close checkpoint

**Decision:** Wave close is declared by writing a structured `HANDOFF.md` file on the `factory-artifacts` branch. The file is the authoritative cross-wave checkpoint artifact. A wave is not closed until `HANDOFF.md` exists with all required fields verified against external git/test sources (anti-fabrication).

**Schema — required fields:**

| Field | Type | Source (real substrate) | Anti-fabrication rule |
|-------|------|------------------------|----------------------|
| `wave_id` | integer | Product pipelines: wave group number derived by `wave-handoff` from sprint-state.yaml dependency order. Engine self-referential: pass number from STATE.md `current_step:` | Cross-checked: `wave_id` in HANDOFF.md must match the value wave-handoff skill computed from the real sprint-state.yaml or STATE.md — not from any phantom `current_wave:` frontmatter field |
| `last_verified_develop_sha` | string (40-char hex) | `git rev-parse origin/develop` at handoff time | Cross-checked: must equal `git rev-parse origin/develop` at handoff time |
| `active_bcs` | list of strings | BC-INDEX.md file list | Each must resolve to an existing file under `.factory/specs/behavioral-contracts/` |
| `next_wave_stories` | list of objects `{id, status}` | sprint-state.yaml entries with `status: pending` OR `status: draft`, ordered by dependency graph | Each `id` must exist in STORY-INDEX.md; **empty list is a hard error** (see Decision 3a) |
| `open_decisions` | list of objects `{id, anchor_type, anchor_ref}` | decision-log.md open rows | `anchor_ref` must be a commit hash, test function name, or file path — NOT a memory assertion |
| `pending_fixes` | list of objects `{finding_id, pr_or_issue_ref}` | Active adversary findings | Each must cite a PR number or issue ref — NOT a memory assertion |
| `process_gaps` | list (may be empty) | Carry-forward from issue #171 mechanism | May be empty; must be explicitly listed or `[]` |
| `precompact_flush_sha` | string (40-char hex) OR null | last-precompact-flush-sha side-channel file | SHA of last commit written by `precompact-flush.sh` in this wave (null on wave-1 if no PreCompact fired) |
| `factory_lock_holder` | string OR null | STATE.md `factory_lock.holder` (if present) OR null | Must match `factory_lock.holder` in STATE.md at handoff time; null when `factory_lock` block is absent from STATE.md (lock not held) |

**Empty `next_wave_stories` is a hard error (F-3):** If the sprint-state.yaml contains no stories with `status: pending` or `status: draft`, the `wave-handoff` skill MUST abort with a non-zero exit and an explicit error message: "No next-wave stories found in sprint-state.yaml — either this is the final wave (declare epic complete) or sprint-state.yaml needs updating." A silent advisory no-op is forbidden (SOUL.md #4).

**Wave-1 special case:** On the first wave (wave_id = 1) where no prior HANDOFF.md exists, the `precompact_flush_sha` field is allowed to be `null`. The `wave-handoff` skill documents this in its output header.

**Rationale:** Every field that could be fabricated from in-context memory requires cross-checking against a verifiable external source (git, filesystem, index files). This directly closes the jira-cli fabricated-SHA failure class. The schema is intentionally narrow — only what the next session needs to resume, not a full state dump.

---

### Decision 3 — Wave-boundary reset: prompt-the-human (D3 LOCKED)

**Decision:** The wave-boundary session reset is initiated by prompting the human. The orchestrator writes and verifies `HANDOFF.md`, then asks the human to clear the session and start wave N+1. The human clears the session and the new session rehydrates from `wave-state.yaml`.

**Auto-reset is an explicit v2 deferral.** Auto-reset (orchestrator self-clearing its own context) is a destructive, irreversible action. If the handoff is incomplete or wrong, in-session state is lost with no recovery path. This risk is not acceptable for v1 without additional safeguards (human confirmation is the safeguard). Auto-reset may be revisited after wave-boundary checkpoint mechanisms have been validated in production.

**v2 deferral recorded:** `auto-reset: enable-when-handoff-verified-for-N-consecutive-waves` — deferred to E-18 follow-up or a future feature cycle.

---

### Decision 4 — Scoped rehydration: curated wave-state.yaml manifest (D4 LOCKED)

**Decision:** After a wave-boundary reset, the new session rehydrates from a curated `wave-state.yaml` manifest. The manifest is produced by the `wave-handoff` skill as part of the wave-close checkpoint. It explicitly lists the next wave's stories and the spec files they depend on (BC files, ADR files, relevant SS-NN files).

**next_wave_stories derivation from real substrate (F-3):** The `wave-handoff` skill derives the next wave's story list from `sprint-state.yaml` by selecting entries with `status: pending` OR `status: draft`, then applying the dependency order graph from STORY-INDEX.md `depends_on:` arrays to produce the wave sequence. This is the SAME algorithm used by the `wave-scheduling` skill's topological sort step. No `wave:` frontmatter field on story files is referenced — that field does not exist.

**RAG is an explicit v2 deferral.** Semantic retrieval over the spec corpus is non-deterministic and introduces the same hallucination risk that wave-boundary resets are designed to eliminate. The curated manifest approach is deterministic and auditable. The manifest can be generated mechanically from STORY-INDEX.md dependency lists and sprint-state.yaml status entries.

**v2 deferral recorded:** `rehydration: rag-over-spec-corpus` — deferred to E-18 follow-on or a future feature cycle when manifest approach proves too rigid for large epics.

**`wave-state.yaml` schema (minimum required fields):**

```yaml
wave_id: 2
generated_at: "2026-06-14T00:00:00Z"
generated_from_handoff_sha: "<sha>"
stories:
  - id: "S-18.02"
    status: "pending"
    spec_files:
      - ".factory/specs/behavioral-contracts/ss-04/BC-4.XX.001.md"
      - ".factory/specs/architecture/decisions/ADR-026.md"
arch_files:
  - ".factory/specs/architecture/ARCH-INDEX.md"
  - ".factory/specs/architecture/decisions/ADR-025.md"
  - ".factory/specs/architecture/decisions/ADR-026.md"
state_pointer: ".factory/STATE.md"
```

---

### Decision 5 — Proactive compaction threshold: 70% via CLAUDE_AUTOCOMPACT_PCT_OVERRIDE (D5 LOCKED)

**Decision:** The proactive auto-compaction threshold is 70% of the effective context window capacity, configured via `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` in the factory's `settings.json` env block. This fires auto-compaction earlier than the harness default, giving the PreCompact flush enough headroom to complete synchronously before the context is fully exhausted.

**Rationale:** Research consensus from Anthropic context-engineering guidance and Microsoft Agent Framework documentation places the optimal proactive-compaction band at 70–75%. 70% is chosen as a conservative default that leaves headroom above the threshold before the harness's own ceiling.

**MEDIUM-confidence note on 83% ceiling (F-11):** The claim that `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` is clamped to approximately 83% is a MEDIUM-confidence research finding, not a formally documented API guarantee. The 70% setting is chosen to be well within any plausible effective range. This claim does NOT substitute for the bounded `timeout_ms` specified in Decision 6 — the timeout is the correct engineering control for flush-time bounding; the headroom argument is an additional belt.

**Settings.json env-verification requirement:** Deployment of E-18 MUST include verifying that `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` is present in `settings.json`. The `check-state-health` skill MUST emit an explicit advisory if this var is absent or set to a value > 80.

**Per-autonomy-level configurability is an explicit v2 deferral.** v1 uses a single global default. A fully-autonomous long-run may want a lower threshold (e.g., 60%); a human-in-the-loop interactive session may want 75%. Configurability per autonomy level is deferred to a follow-on feature cycle.

**v2 deferral recorded:** `threshold-config: per-autonomy-level` — deferred.

---

### Decision 6 — PreCompact flush: synchronous shell hook, blocking on flush incompleteness

**Decision:** The intra-wave flush is implemented as a shell hook (`precompact-flush.sh`) registered under `[[hooks]] event = "PreCompact"` in `hooks-registry.toml`. The hook:

1. Reads `STATE.md` frontmatter fields `current_cycle:` and `current_step:` to determine context identity. Does NOT rely on in-context reasoning — reads STATE.md from the filesystem only. Does NOT look for a non-existent `current_wave:` field.
2. Reads the `factory_lock:` block from STATE.md. If `factory_lock:` is absent or `factory_lock.holder` is absent/null, the lock-renewal step is skipped (no lock held; ADR-025 opt-in model). If lock is held, renews per ADR-025 Decision 11 Mechanism 1 (calls `factory-lock-write.sh renew .factory/STATE.md` before `git add`/commit).
3. Invokes `state-burst` flush logic (equivalent to the flush portion of the `state-burst` skill) to write current wave-critical state to `factory-artifacts`.
4. Commits to `factory-artifacts` with message: `PreCompact flush <cycle>/<step> <timestamp>`.
5. **Exits with `exit 2` (blocking)** if the flush was required and the commit did not land successfully (git commit failure, git push failure). Exits 0 if the flush landed or was not needed (no state changes since last flush).
6. Emits `precompact_flush_sha` to a side-channel file (`.factory/hooks/last-precompact-flush-sha`) for `HANDOFF.md` population.

**Timeout semantics (F-4):** A `timeout_ms = 30000` (30 second) cap is registered in `hooks-registry.toml`. Timeout (no response within 30s) is treated as a hook failure with `on_error = "continue"` semantics: compaction proceeds, flush is assumed non-blocking (best-effort). This prevents a hung git push from wedging the session indefinitely. Timeout is explicitly distinguished from commit failure: a git push that returns a non-zero exit code causes `exit 2` (blocking); a git push that simply hangs until the 30s timeout is treated as a crash and fails open. The hook MUST emit progress to stderr within 5 seconds of invocation to aid diagnostics.

**`on_error = "continue"` (fail-open):** If the hook script crashes before emitting a result, the harness must not wedge the session. Durability is best-effort on crash. A crashed flush is a loss of a flush cycle, not a session-ending event.

**Hermetic requirement:** The flush hook reads ONLY from `STATE.md` and git. It MUST NOT read from in-context state (it runs in a subprocess, not as an LLM tool). This is the anti-deadlock invariant (F1 regression risk §4.1 R1 closure).

**`custom_instructions` is NOT used:** Research confirmed that `custom_instructions` is unreliable on auto-compaction (live official docs omit it; older docs indicate it is empty for `auto` trigger). The flush relies entirely on external state persistence, not summarizer retention.

**Hook registration spec (corrected TOML schema per F-7):**

```toml
[[hooks]]
name = "precompact-flush"
event = "PreCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
priority = 100
timeout_ms = 30000
on_error = "continue"
async = false

[hooks.config]
script_path = "hooks/precompact-flush.sh"

[hooks.capabilities]
env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash", "git", "jq"]
shell_bypass_acknowledged = "legacy-bash-adapter runs unported hooks"
env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]

[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md", ".factory/hooks/last-precompact-flush-sha"]

[hooks.capabilities.write_file]
path_allow = [".factory/hooks/last-precompact-flush-sha"]
```

---

### Decision 7 — PostCompact re-anchor: advisory shell hook (cannot block) — BEST-EFFORT ONLY

**Decision:** A `postcompact-reanchor.sh` advisory hook is registered under `[[hooks]] event = "PostCompact"`. It fires after compaction completes, re-reads the `factory-artifacts` STATE.md pointer, re-asserts the current cycle/phase context, and emits a summary to the harness (visible to the LLM as context). It CANNOT block (PostCompact is inherently non-blocking in the Claude Code harness).

**CAP-032 continuity guarantee clarification (F-6):** The PostCompact re-anchor provides convenience context injection after compaction but is NOT a correctness guarantee and is NOT part of the CAP-032 continuity-guarantee chain. The CAP-032 guarantee rests exclusively on:
- Part A: HANDOFF.md verified on `factory-artifacts` before wave close (Decision 2)
- Part B: PreCompact flush (Decision 6) — commits state BEFORE compaction

If the PostCompact re-anchor hook fails, crashes, or is skipped, the CAP-032 guarantee is unaffected — Part A and Part B are sufficient. The re-anchor is explicitly best-effort.

**Hook registration spec (corrected TOML schema per F-7):**

```toml
[[hooks]]
name = "postcompact-reanchor"
event = "PostCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
priority = 100
timeout_ms = 10000
on_error = "continue"
async = false

[hooks.config]
script_path = "hooks/postcompact-reanchor.sh"

[hooks.capabilities]
env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash", "git", "jq"]
shell_bypass_acknowledged = "legacy-bash-adapter runs unported hooks"
env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID"]

[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md", ".factory/hooks/last-precompact-flush-sha"]
```

**Note:** PostCompact firing is harness-internal. `on_error = "continue"` is required (PostCompact cannot block regardless; setting is defensive).

---

### Decision 8 — WASM for completeness gate; shell for flush (hook split)

**Decision:** The wave-close completeness gate (`validate-wave-handoff-completeness`) is implemented as a native WASM plugin. The PreCompact flush and PostCompact re-anchor are implemented as shell scripts via `legacy-bash-adapter`. This split follows the factory's established convention (ADR-014, ADR-019):

| Hook | Implementation | Why |
|------|---------------|-----|
| `validate-wave-handoff-completeness` | Native WASM (new crate) | Deterministic parse-heavy validation: checks required fields present, `last_verified_develop_sha` format valid, `active_bcs` list non-empty. No git/FS side effects needed for the check itself. Tamper-resistant; auditable. |
| `precompact-flush.sh` | Shell via `hook-plugins/legacy-bash-adapter.wasm` | Effectful: git commit to `factory-artifacts`. WASM cannot exec git in the sandbox per ADR-002/ADR-003 WASI preview 1. |
| `postcompact-reanchor.sh` | Shell via `hook-plugins/legacy-bash-adapter.wasm` | Effectful: reads git refs, emits stdout. Same constraint. |
| `validate-heavy-op-delegation` | Native WASM (new crate) | Deterministic: command-string pattern matching. Pure function; no side effects. |

**WASM fuel budget:** Both new WASM crates use `timeout_ms = 5000` (consistent with existing WASM gates). HANDOFF.md body is capped at 200 lines (similar to STATE.md soft limit) to prevent fuel exhaustion.

---

### Decision 9 — validate-wave-handoff-completeness: no-op on wave-1 / HANDOFF.md absent

**Decision:** The `validate-wave-handoff-completeness` WASM gate fires as a PostToolUse gate on Write/Edit operations that produce `HANDOFF.md` on `factory-artifacts`. Its blocking behavior is:

- **HANDOFF.md does not yet exist AND wave_id = 1 (or current wave is first wave per sprint-state.yaml):** Return `Continue` (no-op). The gate fires only when a HANDOFF.md write is in progress or when `wave-gate` skill explicitly invokes the completeness check.
- **HANDOFF.md write in progress:** Validate all required fields are present and syntactically correct. Block with `HandoffIncomplete` if any required field is missing or malformed.
- **Wave close attempted (via `wave-gate`) with no HANDOFF.md present AND current wave > 1:** Block with `HandoffMissing`.

**Rationale:** This closes F1 regression risk §4.1 R3 — the gate must not add friction for short single-wave runs or for users who have not yet produced a HANDOFF.md. The gate only activates when a transition between waves (wave N > 1) is being attempted.

---

### Decision 10 — PreCompact flush lifecycle is distinct from state-manager burst lifecycle

**Decision:** The PreCompact flush commit on `factory-artifacts` is a separate, distinct lifecycle from a state-manager burst (the A/B/C/D/E sequence per TD-VSDD-053). The PreCompact flush:

- Fires on a harness-internal event between LLM turns, NOT during a state-manager burst.
- Its commit to `factory-artifacts` is NOT counted as a "burst commit" for TD-VSDD-053 single-commit-per-burst enforcement.
- The burst-log entry for the enclosing burst MUST NOT cite the PreCompact commit as a "Commit A/B/C/D/E" — it is a lifecycle-orthogonal commit.
- The `validate-burst-log` and `validate-dispatch-advance` hooks MUST be configured to ignore PreCompact flush commits (identified by commit message prefix `PreCompact flush `).

**Rationale:** This closes F1 regression risk §4.1 R5. Without this explicit boundary, a future adversary could flag the PreCompact commit as a TD-VSDD-053 multi-commit-chain violation.

**Bats test requirement:** A bats test MUST verify that firing a simulated PreCompact hook (triggering a flush commit) and then a state-manager burst produces exactly one "burst commit" in the burst-log entry — not two.

---

### Decision 11 — S-18.00: Dispatcher routing addition for PreCompact/PostCompact

**Decision:** Before E-18 Part B stories (S-18.04, S-18.05) can be implemented, the vsdd-factory dispatcher binary MUST route `PreCompact` and `PostCompact` events to registered plugins. F1 confirmed the Claude Code harness emits these events (live docs, v2.1.105+). The gap question is whether the vsdd-factory dispatcher's plugin invocation layer passes them through.

**Resolution approach (S-18.00):** Story S-18.00 (wave-1 prerequisite) MUST verify by inspection of the dispatcher event-routing source in `crates/factory-dispatcher/src/` (the specific file to inspect is determined by S-18.00 — look for the event-type enum or match arms) whether `PreCompact` and `PostCompact` are enumerated as supported events. If absent, S-18.00 adds routing support. If present, S-18.00 is a no-op verification story that documents the confirmation.

**This decision does NOT pre-judge the outcome.** The architect does not have access to the live dispatcher source in this F2 phase. S-18.00 is the resolution vehicle.

**Release requirement:** Any change to `crates/factory-dispatcher/` requires a release cut for the operator-level cache to pick up the update (CLAUDE.md self-referential note). E-18 stories that depend on PreCompact/PostCompact MUST be sequenced after an rc cut that includes S-18.00's changes (if any).

---

### Decision 12 — validate-heavy-op-delegation WASM gate: advisory-only in v1

**Decision:** The `validate-heavy-op-delegation` WASM gate (PreToolUse on `Bash` tool calls) launches in advisory mode in v1. It emits a finding to stderr but does NOT block. Promotion to blocking mode requires calibration in F3 adversarial review after measuring false-positive rates against real production pipeline sessions.

**No BC required for v1 advisory (F-9):** An advisory-only gate with no blocking behavior has no enforceable behavioral postcondition suitable for a BC. If v2 promotes this gate to blocking mode, product-owner MUST author a BC at that time. S-18.06 story is correctly scoped as advisory-only; no BC is authored in F2.

**Pattern set for advisory nudge (v1):**
- Commands matching `cargo test --release` with output likely > 10MB
- Commands matching `grep -r` or `find . -name` traversals against large directory trees
- Commands matching known heavy bats test runners (`.run-all.sh`, `./run-bats.sh`)

**Rationale:** False-positive blocking of legitimate Bash commands is a harder regression than missing a delegation nudge. The advisory mode collects real data without impeding pipeline operation.

---

### Decision 13 — Harness version runtime assertion (F-5)

**Decision:** The harness ≥ v2.1.105 precondition must be actively checked at runtime, not merely documented. Two enforcement mechanisms:

1. **SessionStart advisory hook:** A `check-harness-version.sh` advisory hook (or extension of the existing `check-state-health` skill) reads the harness version from the Claude Code environment (if accessible via env var or harness-provided metadata) and emits an ADVISORY warning if the version is below v2.1.105. Advisory only — does not block (a misconfigured harness that blocks SessionStart is worse than one that silently runs without PreCompact blocking).

2. **check-state-health skill addition:** The `check-state-health` skill MUST include a harness-version check step that reads any available version signal and reports: "PreCompact blocking: SUPPORTED (harness >= v2.1.105)" or "PreCompact blocking: UNSUPPORTED (harness < v2.1.105 — Part B guarantee not enforced)."

**Honest degrade documentation (F-5):** S-18.04 AC-001 and the `precompact-flush.sh` script header MUST state plainly: "On harness < v2.1.105, this script runs but cannot prevent compaction. exit 2 is visible as stderr only. CAP-032 Part B guarantee is not satisfied."

---

## SS-08 and S-18.07 Scope Clarification (F-9)

**Why SS-08 is not in `anchors:`:** The ADR-026 `anchors:` list enumerates subsystems where architectural decisions are made (SS-04: plugin ecosystem; SS-05: factory-artifacts state store; SS-06: skill catalog; SS-07: bash hook layer). SS-08 (Templates and Rules) owns templates and rules artifacts, not the hook pipeline.

**S-18.07 terminology disambiguation:** This story updates documentation in `compact-state/SKILL.md`, `check-state-health/SKILL.md`, and `CLAUDE.md`. These documentation targets span SS-06 (skills) and SS-08 (templates/rules). However, the *architectural decision* is that three terms are distinct — "context compaction" (harness), "state compaction" (`compact-state` file op), "PreCompact flush" (new). This disambiguation is a documentation-only change; it makes no new architectural commitments and creates no behavioral postcondition that requires a BC. **No new BC is required for S-18.07.** Product-owner does not need to act on S-18.07.

---

## Consequences

### Positive

- **Wave-boundary continuity becomes a hard guarantee**, not a best-effort. Every wave close is verifiable against git/filesystem (not memory).
- **Mid-wave compaction becomes lossless** for the factory's load-bearing state. The flush guarantees that `STATE.md`, wave context, and active decision SHAs land on `factory-artifacts` before any compaction event can drop them.
- **Fabricated-SHA risk class is directly addressed.** The anti-fabrication cross-check in `HANDOFF.md` (git-verified SHAs, filesystem-verified BC paths) eliminates the fabrication failure mode documented in issues #170 and #173.
- **Sub-agent isolation becomes the recommended first-line defense** for heavy ops, reducing orchestrator window pressure without requiring any new hook (the delegation guard is advisory in v1).
- **Terminology collision is resolved** by the disambiguation story (S-18.07): "context compaction" (harness), "state compaction" (`compact-state` file op), and "PreCompact flush" (new, this ADR) are distinct concepts with distinct documentation targets.

### Negative / Trade-offs

- **Wave-boundary reset requires human action.** The human must clear the session and confirm the handoff before wave N+1 begins. This is a deliberate friction point — it is the safety mechanism. Automation (auto-reset) is an explicit v2 deferral per Decision 3.
- **PreCompact flush adds latency at every compaction event.** The flush runs synchronously inside the PreCompact hook. If the flush takes more than 30 seconds (e.g., slow git push under poor network conditions), the 30s timeout triggers and the hook fails open (compaction proceeds unblocked). The timeout is the engineering control for this trade-off.
- **New WASM crates inflate the CI WASM floor-count gate.** Two new `[[bin]]`-bearing crates (`validate-wave-handoff-completeness`, `validate-heavy-op-delegation`) must be added to the floor-count expectation in CI at the same commit that adds them (standard procedure per ADR-014 precedent).
- **E-18 stories must ship atomically** in a single rc cut. Partial shipping (e.g., shell hooks without the WASM completeness gate) would create a regression where the flush fires but the wave-close gate does not block. S-18.00 through S-18.07 must be sequenced into a single rc cut.

---

## Risks Addressed

This ADR directly addresses the F1 regression risks:

| Risk | F1 ID | Mitigation in This ADR |
|------|-------|------------------------|
| PreCompact flush deadlock (flush needs in-context reasoning) | §4.1 R1 | Decision 6 hermetic requirement: flush reads STATE.md+git only, never in-context reasoning |
| Blocking PreCompact on crash wedges session | §4.1 R2 | Decision 6 `on_error = "continue"` |
| Completeness gate blocks wave-1 / short-run no-ops | §4.1 R3 | Decision 9 explicit no-op rule for wave_id = 1 / HANDOFF.md absent |
| WASM fuel exhaustion on large HANDOFF.md | §4.1 R4 | Decision 8 200-line body cap on HANDOFF.md |
| Single-commit-per-burst discipline ambiguity | §4.1 R5 | Decision 10 explicit lifecycle boundary + bats test |
| Hung git push wedges session (F-4) | new | Decision 6 `timeout_ms = 30000`; timeout = fail-open |
| Precondition not checked at runtime (F-5) | new | Decision 13 SessionStart advisory hook + check-state-health extension |
| PostCompact re-anchor misrepresented as guarantee (F-6) | new | Decision 7 explicit best-effort annotation; removed from CAP-032 chain |
| `wave_id` source phantom / next_wave_stories phantom source (F-1/F-3) | new | Decision 2 explicit real-substrate derivation; empty list = hard error |

---

## v2 Deferrals (explicit, with rationale)

| Deferred Capability | Deferral Reason | Tracking |
|--------------------|----------------|---------|
| Auto-reset at wave boundary | Destructive irreversible action; requires human safeguard in v1 | E-18 follow-on or separate feature cycle |
| RAG over spec corpus for rehydration | Non-deterministic; hallucination risk; deterministic manifest is safer in v1 | E-18 follow-on or separate feature cycle |
| Per-autonomy-level threshold configuration | Single global default (70%) is sufficient for v1 | E-18 follow-on |
| `validate-heavy-op-delegation` blocking mode | Requires false-positive calibration before blocking | S-18.06 F3 adversarial review → promotion if rate acceptable; BC authored at promotion time |

---

## Deliverables (for story-writer reference in F3)

| Deliverable | Story | Subsystem(s) |
|-------------|-------|-------------|
| `HANDOFF.md` schema (defined in this ADR §Decision 2) | S-18.01 | SS-05 |
| `wave-handoff` skill | S-18.01 | SS-06 |
| `wave-state.yaml` manifest schema (defined in this ADR §Decision 4) | S-18.01 | SS-05 |
| S-18.00 dispatcher routing verification/addition for PreCompact/PostCompact | S-18.00 | SS-01 |
| `validate-wave-handoff-completeness` WASM crate (`crates/hook-plugins/validate-wave-handoff-completeness/`) | S-18.02 | SS-04 |
| `hooks-registry.toml` WASM gate entry for `validate-wave-handoff-completeness` | S-18.02 | SS-07 |
| `wave-reset` skill (loads `wave-state.yaml`, rehydrates session) | S-18.03 | SS-06 |
| `precompact-flush.sh` shell hook with 30s timeout | S-18.04 | SS-07 |
| `hooks-registry.toml` PreCompact entry (corrected TOML schema) | S-18.04 | SS-07 |
| `postcompact-reanchor.sh` advisory hook (best-effort, not in CAP-032 guarantee chain) | S-18.05 | SS-07 |
| `hooks-registry.toml` PostCompact entry (corrected TOML schema) | S-18.05 | SS-07 |
| `validate-heavy-op-delegation` WASM crate (advisory mode; no BC in v1) | S-18.06 | SS-04 |
| Terminology disambiguation: `compact-state/SKILL.md`, `check-state-health/SKILL.md`, `CLAUDE.md` callout | S-18.07 | SS-06, SS-08 |
| `check-harness-version.sh` advisory hook OR check-state-health extension | S-18.04 or S-18.00 | SS-07, SS-06 |
| `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` settings.json verification in check-state-health | S-18.03 or S-18.07 | SS-06 |

All story estimates are per the F1 provisional decomposition in the F1 delta analysis. Story-writer produces final AC-level decomposition in F3.

---

## Dependency Chain

```
S-18.00 (dispatcher routing verification; prerequisite field/artifact verification)
    |
S-18.01 (HANDOFF.md schema + wave-handoff skill; derives next_wave_stories from sprint-state.yaml)
    |
    +---> S-18.02 (validate-wave-handoff-completeness WASM)
    |         |
    +---> S-18.04 (precompact-flush.sh; reads current_cycle+current_step; lock-renew no-op when unlocked)  [depends_on: S-17.04 for lock renewal]
              |
              +---> S-18.03 (wave-reset skill)
                        |
                        +---> S-18.05 (postcompact-reanchor.sh — advisory, not in guarantee chain)
                        +---> S-18.06 (validate-heavy-op-delegation WASM — advisory)
                                    |
                                    +---> S-18.07 (terminology disambiguation — doc-only, no BC)
```

Dependency on E-17: S-18.04 (`precompact-flush.sh`) MUST invoke `factory-lock-write.sh renew` when lock is held per ADR-025 Decision 11 Mechanism 1. When `factory_lock` is absent from STATE.md, the lock-renew call is skipped (no-op). Story S-18.04 MUST declare `depends_on: [S-17.04]` at story-writer authoring time.

---

## Harness Version Precondition

**REQUIRED:** Claude Code harness version ≥ v2.1.105.

The PreCompact blocking capability (Decision 6) is a no-op on harness versions < v2.1.105. On pre-v2.1.105, `PreCompact` fires as a notification-only hook; `exit 2` shows stderr to the user but does NOT block compaction. **On pre-v2.1.105, the CAP-032 Part B guarantee is NOT satisfied — context loss remains possible.** This is not a safe degradation. The E-18 Part B stories MUST document this precondition in their AC-001.

The current operator harness version is confirmed as Claude Code v2.1.177 (>= v2.1.105) per F1 delta analysis — this precondition is currently satisfied.

Runtime check: Decision 13 wires an active assertion via check-state-health skill and SessionStart advisory hook.

---

## VP Allocations (F-10)

| VP | Title | BC | Type |
|----|-------|-----|------|
| VP-081 | Wave Cannot Close Without Verified Handoff | BC-5.41.001, BC-4.14.001 | safety/integration |
| VP-082 | PreCompact Flush Commits Before Compaction | BC-7.07.001 | safety/integration |
| VP-083 | Completeness Gate Is No-Op on Wave-1 | BC-4.14.001 | invariant/unit-test |
| VP-084 | PreCompact Flush Lifecycle Distinct From Burst | BC-5.41.003 | invariant/integration |
| VP-085 | PreCompact Flush Hook Is Hermetic | BC-7.07.001 | safety/unit-test |
| **VP-086** | **Dispatcher Exit-2 Propagation for PreCompact Block-Intent** | **BC-1.15.001** | **safety/integration** |

VP-086 is the concrete verification property for BC-1.15.001 PC4 (exit-2 propagation from PreCompact plugin to harness). This was the only BC with `TBD-VP` in its Traceability section and represents a safety-critical linchpin: if the dispatcher silently drops exit-2 block-intent on PreCompact, the entire PreCompact flush blocking mechanism is a silent no-op. VP-086 MUST be authored and added to VP-INDEX as part of this ADR revision. VP-086 file: `.factory/specs/verification-properties/VP-086.md`. VP-INDEX bumped to v2.08.

---

## Traceability

- **Feature:** issue #173
- **Epic:** E-18 (CAP-032 context-durability)
- **Composes with:** E-17 (CAP-031 single-writer lock/lease) via S-18.04 depends_on S-17.04
- **Composes with:** issue #171 (deferred process-gaps carry-forward via `HANDOFF.md` process_gaps field)
- **Subsystems affected:** SS-04, SS-05, SS-06, SS-07 (and potentially SS-01 per S-18.00 outcome)
- **ADRs composed with:** ADR-019 (async semantics — PreCompact hooks must be `async: false`), ADR-025 (factory lock — flush must renew lock per Decision 11 Mechanism 1 WHEN lock is held; no-op when `factory_lock` absent)
- **ADRs not conflicting:** ADR-012 (legacy-bash-adapter — shell hooks route through it per established pattern), ADR-014 (Tier 2 WASM migration — new WASM crates follow the native migration path)
- **Real substrate fields used:** STATE.md `current_cycle:`, STATE.md `phase:`, STATE.md `current_step:`, STATE.md `factory_lock:` (optional), sprint-state.yaml story status entries, STORY-INDEX.md `depends_on:` arrays
- **VP-INDEX:** v2.07→v2.08 (VP-086 added)
- **ARCH-INDEX:** v2.28→v2.29 (ADR-026 v1.0→v1.1 amendment recorded)
