# Demo Evidence — S-18.07: Terminology Disambiguation Docs

**Story:** S-18.07 — E-18 terminology disambiguation docs (compact-state vs PreCompact flush; cross-references in SKILL.md files)
**Story version:** v1.7
**tdd_mode:** facade (doc-only; no behavioral contracts; no runtime execution)
**Evidence type:** Documentation review gates (4 gates over 3 deliverable `.md` files)

This story has no CLI tool, web app, or API to record. The deliverables are edits to
three markdown files. Evidence is captured as verbatim excerpts from the delivered
content, proving each acceptance criterion is satisfied. VHS/Playwright recordings are
not applicable.

---

## AC-001 — compact-state SKILL.md "Terminology Note" section

**Gate:** `review_compact_state_skill_terminology_note`
**File:** `plugins/vsdd-factory/skills/compact-state/SKILL.md`
**Verdict:** GREEN

All three required points from AC-001 are present in the "Terminology Note" section
(lines 143–154 of the delivered file):

### Delivered excerpt — `plugins/vsdd-factory/skills/compact-state/SKILL.md` (§Terminology Note)

```markdown
## Terminology Note

> **`/compact-state` (this skill) vs `PreCompact` hook event — these are distinct mechanisms.**

| Concept | What it is | When it fires |
|---------|-----------|---------------|
| `/compact-state` (this skill) | A manually invoked skill that extracts historical content from STATE.md into cycle files (burst logs, adversary passes, session checkpoints, lessons) and slims STATE.md to <200 lines. It does NOT invoke the Claude Code `/compact` command — it reorganizes the STATE.md file, not the conversation context. | Only when an operator or agent explicitly invokes `/compact-state`. |
| `PreCompact` hook event | A Claude Code harness event fired automatically before the harness performs context compaction. Triggers the `precompact-flush` PreCompact WASM plugin (`precompact-flush.wasm`) (S-18.04a deliverable, native WASM per ADR-028 §Decision 2), which persists wave-boundary state to `factory-artifacts` before context is lost. | Only when the Claude Code harness triggers automatic compaction (e.g., when context usage reaches the configured autocompact threshold). |
| `PostCompact` hook event | A Claude Code harness event fired automatically after compaction completes. Triggers `postcompact-reanchor.sh` (S-18.05 deliverable), which emits a `[PostCompact Re-anchor]` block to stdout so the LLM session can re-ground itself after compaction. | Only when the Claude Code harness triggers automatic compaction (same trigger condition as `PreCompact`). |

**Invoking `/compact-state` does NOT fire the `PreCompact` hook chain.** The `precompact-flush` WASM plugin fires automatically before Claude Code compacts — not during a manual `/compact-state` invocation. These are independent paths: `/compact-state` reorganizes STATE.md content into cycle files; the `PreCompact`/`PostCompact` hook chain persists and restores session context across a harness-driven compaction event.

**After any session clear or context reset**, the mandatory first step before any pipeline work is `/rehydrate-wave`. See `plugins/vsdd-factory/skills/rehydrate-wave/SKILL.md` for the full invocation contract (BC-6.24.001 / ADR-026 §Decision 4).
```

### Point-by-point AC-001 confirmation

| AC-001 Point | Requirement | Evidence | Status |
|---|---|---|---|
| Point 1 | Distinguishes `/compact-state` (extracts STATE.md historical content into cycle files; does NOT invoke Claude Code `/compact`) from `PreCompact` hook event (`precompact-flush.wasm`, fires automatically before harness compaction) | Table rows for `/compact-state` and `PreCompact` hook event; explicit sentence "It does NOT invoke the Claude Code `/compact` command — it reorganizes the STATE.md file, not the conversation context." | PRESENT |
| Point 2 | States invoking `/compact-state` does NOT fire the `PreCompact` hook chain; `precompact-flush` WASM plugin fires only when the Claude Code harness triggers automatic compaction | Bold sentence: "Invoking `/compact-state` does NOT fire the `PreCompact` hook chain." plus explanation that the harness triggers firing, not `/compact-state` | PRESENT |
| Point 3 | Cross-references `/rehydrate-wave` as the mandatory first step after any session clear or context reset | "After any session clear or context reset, the mandatory first step before any pipeline work is `/rehydrate-wave`. See `plugins/vsdd-factory/skills/rehydrate-wave/SKILL.md` for the full invocation contract (BC-6.24.001 / ADR-026 §Decision 4)." | PRESENT |

---

## AC-002 — check-state-health SKILL.md PostCompact re-anchor section

**Gate:** `review_check_state_health_skill_postcompact_xref`
**File:** `plugins/vsdd-factory/skills/check-state-health/SKILL.md`
**Verdict:** GREEN

All three required points from AC-002 are present in the "PostCompact Re-anchor
Verification" section (lines 113–136 of the delivered file):

### Delivered excerpt — `plugins/vsdd-factory/skills/check-state-health/SKILL.md` (§PostCompact Re-anchor Verification)

```markdown
## PostCompact Re-anchor Verification

After automatic context compaction by the Claude Code harness, the PostCompact advisory hook (`plugins/vsdd-factory/hooks/postcompact-reanchor.sh`, S-18.05 deliverable) fires and emits a structured re-anchor block to stdout. This block grounds the LLM session in the current pipeline state immediately after compaction.

**The re-anchor block looks like:**

```
[PostCompact Re-anchor] context=<current_cycle>/<current_step> sha=<develop_sha>
Source: factory-artifacts STATE.md (verified at <timestamp>)
```

The hook reads `current_cycle` and `current_step` from `factory-artifacts:STATE.md` via `git show` (never from the working tree or in-context reasoning). The `develop_sha` is sourced from `git rev-parse refs/remotes/origin/develop` at hook invocation time.

**This hook is advisory-only — it cannot block or prevent compaction** (PostCompact fires after compaction is complete; `on_error=continue` in `hooks-registry.toml`).

### Operator Step: Verify Re-anchor Block After Automatic Compaction

When resuming work after an automatic compaction event, verify the `[PostCompact Re-anchor]` block appeared in the session output before proceeding with pipeline work:

1. **Scan the session output** for a line matching `[PostCompact Re-anchor] context=...`.
2. **If the re-anchor block is present:** Confirm the `context=` value matches the expected `current_cycle/current_step`. If it does, the session is re-grounded and pipeline work may continue.
3. **If the re-anchor block is absent** (the hook did not fire, or the session output was truncated): run `/rehydrate-wave` before starting any pipeline work. `/rehydrate-wave` reads `wave-state.yaml` from `factory-artifacts` and injects the correct spec files into context — see `plugins/vsdd-factory/skills/rehydrate-wave/SKILL.md` for the full invocation contract (BC-6.24.001 / ADR-026 §Decision 4).

> **Note:** `check-state-health` is a diagnostic skill — it reads and reports, but does not block compaction or prevent it from occurring. The PostCompact hook fires independently of this skill.
```

### Point-by-point AC-002 confirmation

| AC-002 Point | Requirement | Evidence | Status |
|---|---|---|---|
| Point 1 | Cross-references PostCompact re-anchor advisory hook (`postcompact-reanchor.sh`) and explains that after automatic compaction the hook emits a `[PostCompact Re-anchor]` block to stdout that operators should verify | Opening paragraph names `plugins/vsdd-factory/hooks/postcompact-reanchor.sh` (S-18.05 deliverable) and shows the exact stdout block format `[PostCompact Re-anchor] context=<current_cycle>/<current_step> sha=<develop_sha>` | PRESENT |
| Point 2 | Adds a `check-state-health` step instructing operators to confirm the re-anchor block appeared (or run `/rehydrate-wave` if it did not) before resuming pipeline work | "Operator Step: Verify Re-anchor Block After Automatic Compaction" — 3-step numbered procedure; step 3 explicitly says run `/rehydrate-wave` if block is absent | PRESENT |
| Point 3 | Does NOT imply that `check-state-health` blocks or prevents compaction (advisory only) | Explicit advisory note: "This hook is advisory-only — it cannot block or prevent compaction (PostCompact fires after compaction is complete; `on_error=continue`)" and closing note: "`check-state-health` is a diagnostic skill — it reads and reports, but does not block compaction or prevent it from occurring." | PRESENT |

---

## AC-003 — CLAUDE.md §Conventions compact-state vs PreCompact/PostCompact disambiguation callout

**Gate:** `review_claude_md_compact_disambiguation`
**File:** `CLAUDE.md`
**Verdict:** GREEN

The three-way disambiguation callout is present in the `CLAUDE.md` §Conventions section
under the heading "Context compaction — three distinct mechanisms (S-18.07 / ADR-026
§Decision 7)":

### Delivered excerpt — `CLAUDE.md` (§Conventions — Context compaction subsection)

```markdown
- **Context compaction — three distinct mechanisms (S-18.07 / ADR-026 §Decision 7):**

  | Mechanism | What it is | Trigger |
  |-----------|-----------|---------|
  | `/compact-state` skill | Human/agent-initiated skill (`plugins/vsdd-factory/skills/compact-state/SKILL.md`). Extracts historical content from STATE.md into cycle files (burst logs, adversary passes, session checkpoints, lessons) and slims STATE.md to <200 lines. It does NOT invoke the Claude Code `/compact` command — it reorganizes the STATE.md file, not the conversation context. | Explicit operator or agent invocation only. |
  | `PreCompact` hook event | Automatic Claude Code harness event fired before context compaction. Triggers the `precompact-flush` PreCompact WASM plugin (`precompact-flush.wasm`), native WASM per ADR-028 §Decision 2 / S-18.04a, which persists wave-boundary state to `factory-artifacts` before context is lost. | Fired by the harness on automatic compaction (not by `/compact-state`). |
  | `PostCompact` hook event | Automatic Claude Code harness event fired after compaction completes. Triggers `postcompact-reanchor.sh`, which emits a `[PostCompact Re-anchor]` block to stdout so the LLM session can re-ground itself. | Fired by the harness on automatic compaction (not by `/compact-state`). |

  **Invoking `/compact-state` does NOT fire the `PreCompact` or `PostCompact` hook chains.** These are independent mechanisms. After any session clear or context reset (whether from automatic compaction or a manual clear), run `/rehydrate-wave` as the first step before any pipeline work.
```

### Point-by-point AC-003 confirmation

| AC-003 Point | Requirement | Evidence | Status |
|---|---|---|---|
| `/compact-state` skill | Human/agent-initiated manual context compaction (skill) | Table row: "Human/agent-initiated skill … Explicit operator or agent invocation only." | PRESENT |
| `PreCompact` hook event | Automatic Claude Code harness event triggering `precompact-flush.wasm` before compaction | Table row: "Automatic Claude Code harness event fired before context compaction. Triggers the `precompact-flush` PreCompact WASM plugin (`precompact-flush.wasm`), native WASM per ADR-028 §Decision 2 / S-18.04a" | PRESENT |
| `PostCompact` hook event | Automatic Claude Code harness event triggering `postcompact-reanchor.sh` after compaction | Table row: "Automatic Claude Code harness event fired after compaction completes. Triggers `postcompact-reanchor.sh`" | PRESENT |

**EC-001 check (prior-story content):** The callout was not already present in CLAUDE.md before this story; AC-003 adds it. No duplicate content is introduced. Per EC-001 handling, if equivalent content had already been present the story would have recorded a read-gate GREEN; instead the implementer added new content (correct path for when content is absent).

---

## AC-004 — doc-only scope boundary (no behavioral files modified)

**Gate:** `review_pr_diff_doc_only`
**Command executed:** `git diff --name-only develop` (run inside `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.07`)
**Verdict:** GREEN

### Literal command and captured output

```
$ git diff --name-only develop
CLAUDE.md
plugins/vsdd-factory/skills/check-state-health/SKILL.md
plugins/vsdd-factory/skills/compact-state/SKILL.md
```

### Scope-boundary analysis

| File in diff | Extension | Is it a behavioral file (`.sh`, `.wasm`, `.rs`, `.toml`)? | AC-004 compliant? |
|---|---|---|---|
| `CLAUDE.md` | `.md` | No | Yes |
| `plugins/vsdd-factory/skills/check-state-health/SKILL.md` | `.md` | No | Yes |
| `plugins/vsdd-factory/skills/compact-state/SKILL.md` | `.md` | No | Yes |

All three changed files are `.md` documentation files. No `.sh`, `.wasm`, `.rs`, or `.toml` files appear in the diff. The diff is exactly the 3 declared deliverable files — no unexpected additions.

---

## Coverage Summary

| Gate | AC | File | Verdict |
|------|----|------|---------|
| `review_compact_state_skill_terminology_note` | AC-001 | `plugins/vsdd-factory/skills/compact-state/SKILL.md` | GREEN |
| `review_check_state_health_skill_postcompact_xref` | AC-002 | `plugins/vsdd-factory/skills/check-state-health/SKILL.md` | GREEN |
| `review_claude_md_compact_disambiguation` | AC-003 | `CLAUDE.md` | GREEN |
| `review_pr_diff_doc_only` | AC-004 | (git diff output) | GREEN |

All 4 review gates PASS. Story S-18.07 acceptance criteria fully satisfied.
