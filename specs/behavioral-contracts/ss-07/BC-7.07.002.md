---
document_type: behavioral-contract
level: L3
version: "1.6"
status: draft
producer: product-owner
timestamp: 2026-06-14T00:00:00Z
last_amended: "2026-06-14 (v1.6) — F2 pass-6 fix-burst: (E-18) ADR cite convention: v1.4 version token dropped per ADR-026 §BC Traceability Cite Convention (TD-VSDD-091 anti-volatile-pin); stable §Decision anchor adopted (cite-only change). [Prior: 2026-06-14 (v1.4) — F2 pass-4 fix-burst: (F-P4-003) ADR cite v1.3→v1.4 (cite-only). [Prior: 2026-06-14 (v1.3) — F2 pass-3 fix-burst: ADR cite v1.1→v1.3. [Prior: 2026-06-14 (v1.2) — F2 pass-2 fix-burst: (F-P2-002/009) PC1 stdout template: `wave=<current_wave>` → `context=<current_cycle>/<current_step>` (no phantom current_wave: field; hook derives from STATE.md current_cycle: + current_step:). Inv 2: `current_wave`, `current_step`, `last_verified_develop_sha` → `current_cycle:`, `current_step:`, `last_verified_develop_sha`; explicit statement that current_wave: does not exist. EC-003: `current_wave field absent` → `current_cycle: or current_step: absent`; emit `context=UNKNOWN`. Test vector updated to new stdout format. [Prior: 2026-06-14 (v1.1) — F2 pass-1 fix-burst: (F-6) Best-effort status made explicit in §Description and §Invariants; NOT in CAP-032 continuity-guarantee chain. (F-13 POLICY 7) H1 title corrected: 'reads compaction summary' removed from H1 because PC7 makes it optional; H1 now accurately describes what postconditions specify. (DI) TBD-DI replaced with DI-024. TBD-VP retained with justification.]"
phase: F2
inputs:
  - .factory/feature-delta/issue-173/F1-delta-analysis.md
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
input-hash: "c2426d5"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-07"
capability: "CAP-032"
lifecycle_status: draft
introduced: v1.0-feature-context-durability-E18
modified:
  - "2026-06-14 (v1.6) — F2 pass-6 fix-burst: ADR cite convention: stable §Decision anchor (TD-VSDD-091); cite-only."
  - "2026-06-14 (v1.4) — F2 pass-4 fix-burst: (F-P4-003) ADR cite v1.3→v1.4 (cite-only)."
  - "2026-06-14 (v1.3) — F2 pass-3 fix-burst: ADR cite v1.1→v1.3."
  - "2026-06-14 (v1.2) — F2 pass-2 fix-burst: PC1 stdout template + Inv 2 + EC-003 + test vector: phantom current_wave: removed; current_cycle+current_step from STATE.md frontmatter used instead."
  - "2026-06-14 (v1.1) — F2 pass-1 fix-burst: H1 title corrected (removed 'reads compaction summary' per F-13 POLICY 7 H1↔postcondition parity); §Description + §Invariants updated to state best-effort explicitly (F-6); NOT in CAP-032 guarantee chain; TBD-DI replaced with DI-024; TBD-VP retained with justification; ADR cite v1.0→v1.1."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-7.07.002: postcompact-reanchor.sh fires on PostCompact (advisory only; best-effort; cannot block); emits re-anchor block to stdout from factory-artifacts STATE.md; logs to .factory/logs/; does NOT commit to factory-artifacts

## Description

`postcompact-reanchor.sh` is an advisory shell hook registered under `event = "PostCompact"` in `hooks-registry.toml`. It fires after the harness has completed context compaction. Its sole purpose is to emit a concise re-anchor block to stdout (visible to the LLM as fresh context) that re-states `current_step`, `current_cycle`, and `last_verified_develop_sha` from the `factory-artifacts` branch — not from in-context memory. PostCompact is inherently non-blocking in the Claude Code harness; this hook cannot prevent or reverse compaction regardless of its exit code. It does NOT commit to `factory-artifacts` (commits are the PreCompact flush's responsibility). **This hook is explicitly BEST-EFFORT and is NOT part of the CAP-032 continuity-guarantee chain.** The CAP-032 guarantee rests exclusively on Part A (HANDOFF.md) and Part B (PreCompact flush). This hook's failure, crash, or absence has no effect on the CAP-032 guarantee. Its output is purely informational re-anchoring after the fact.

## Preconditions

1. The hook is registered in `hooks-registry.toml` as:
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
   ```
   (Full capabilities block per ADR-026 §Decision 7 corrected TOML schema.)
2. The Claude Code harness version >= v2.1.105 (PostCompact event supported).
3. The dispatcher routes `PostCompact` events to registered plugins (BC-1.15.001 postconditions satisfied).
4. `factory-artifacts` branch is accessible via `git` for reading STATE.md fields.

## Postconditions

1. **Re-anchor block emitted to stdout**: The hook emits a concise (2–5 line) re-anchor summary to stdout in a structured format readable by the LLM:
   ```
   [PostCompact Re-anchor] context=<current_cycle>/<current_step> sha=<last_verified_develop_sha>
   Source: factory-artifacts STATE.md (verified at <ISO-timestamp>)
   ```
   The values are read from `factory-artifacts` STATE.md via `git show factory-artifacts:.factory/STATE.md`, NOT from in-context knowledge. Wave identity is derived — not stored as a `current_wave:` field. The hook emits `current_cycle:` and `current_step:` from STATE.md frontmatter; the human can derive the wave/pass ordinal from `current_step:` if needed.

2. **Log written to .factory/logs/**: The hook appends a structured log entry to `.factory/logs/postcompact-reanchor-YYYY-MM-DD.jsonl` (same daily-file pattern as the dispatcher internal log) with fields: `event`, `wave_id`, `current_step`, `last_verified_develop_sha`, `timestamp`, `status` ("ok" or "warn").

3. **Cannot block compaction**: The hook's exit code is ignored by the harness for blocking purposes. PostCompact is advisory-only in the Claude Code harness. The hook MUST NOT rely on exit-code semantics for any correctness guarantee.

4. **Does NOT commit to factory-artifacts**: The hook reads from `factory-artifacts` but does NOT write, stage, or commit anything to `factory-artifacts`. Commits are the PreCompact flush's (BC-7.07.001) responsibility. This invariant prevents double-commit issues and keeps the re-anchor hook's side effects limited to stdout + local log.

5. **Fail-open on any error**: Any failure (STATE.md unreadable, git not accessible, log file not writable) results in exit 0 with an advisory message to stdout. PostCompact failing silently is better than PostCompact interrupting the session.

6. **on_error = "continue"**: A hook crash results in the dispatcher treating it as advisory (exit 0). The session continues normally. Crash is logged to the dispatcher internal log.

7. **compaction summary optional**: If the harness provides a compaction summary payload (e.g., token counts before/after), the hook may log these to the `.factory/logs/` file but is not required to parse them. The re-anchor output is independent of the compaction summary content.

## Invariants

1. **No factory-artifacts writes**: The hook is read-only with respect to `factory-artifacts`. Any `git commit`, `git push`, or `git add` to the `factory-artifacts` branch from this hook is a specification violation.

2. **Values sourced from factory-artifacts, not in-context**: `current_cycle:`, `current_step:`, and `last_verified_develop_sha` values in the re-anchor output must be read from `git show factory-artifacts:.factory/STATE.md`, never from the LLM's in-context knowledge. There is no `current_wave:` field in STATE.md — the hook reads `current_cycle:` and `current_step:` and emits those. This is the re-anchor's entire purpose: providing authoritative grounding after compaction potentially corrupted in-context state.

3. **Best-effort and not in CAP-032 guarantee chain**: This hook is explicitly best-effort. It is NOT in the CAP-032 continuity-guarantee chain. The PostCompact hook is the "after the fact" complement to the PreCompact flush. It cannot undo compaction or restore lost context. It can only surface the externally persisted truth so the LLM can re-ground itself. Any design or implementation that depends on this hook for a correctness property is a specification violation (DI-024). If this hook is absent, crashes, or emits incorrect data, the CAP-032 guarantee is unaffected — Part A (HANDOFF.md) and Part B (PreCompact flush) are sufficient.

4. **Log-append is idempotent-safe**: If the log file already contains an entry for the current timestamp (e.g., two rapid PostCompact events), the hook appends rather than overwrites. No deduplication required.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | PostCompact fires; STATE.md on factory-artifacts readable | Re-anchor block emitted; log appended; exit 0 |
| EC-002 | PostCompact fires; factory-artifacts not accessible (network error) | Emit advisory to stdout: `[PostCompact Re-anchor] WARN: factory-artifacts unreachable; re-anchor skipped`; log entry with status=warn; exit 0 |
| EC-003 | PostCompact fires; STATE.md on factory-artifacts present but `current_cycle:` or `current_step:` fields absent (there is no `current_wave:` field — it does not exist) | Emit partial re-anchor with `context=UNKNOWN`; log warn; exit 0 |
| EC-004 | Hook crashes (set -euo pipefail exit) | on_error=continue; dispatcher exits 0; session continues; plugin.crashed in dispatcher log |
| EC-005 | PostCompact fires; .factory/logs/ directory does not exist | Attempt to create it; if creation fails, emit advisory to stdout only; exit 0 |
| EC-006 | Hook tries to commit to factory-artifacts | Specification violation; implementation MUST NOT include any git write commands targeting factory-artifacts |
| EC-007 | Hook exit 2 (implementation error) | Harness ignores block-intent on PostCompact; session continues; exit code logged |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| PostCompact event; factory-artifacts STATE.md has `current_cycle: v1.0-example`, `current_step: phase-F2`, `last_verified_develop_sha: abc...def` | stdout: `[PostCompact Re-anchor] context=v1.0-example/phase-F2 sha=abc...def`; log entry appended; exit 0 | happy-path |
| PostCompact event; factory-artifacts unreachable | stdout advisory WARN; log entry status=warn; exit 0 | unreachable-fail-open |
| PostCompact event; hook crashes | on_error=continue; exit 0; plugin.crashed log | crash-fail-open |
| PostCompact event followed by LLM tool call | LLM context includes the re-anchor stdout; LLM uses verified sha/wave/step from that block | re-anchor-visible |
| bats: simulate PostCompact event via factory-dispatcher | hook emits re-anchor to stdout; no factory-artifacts commits | integration-bats |

## Related BCs

- BC-7.07.001 — sibling: precompact-flush.sh is the PreCompact companion; commits to factory-artifacts (this hook does not)
- BC-1.15.001 — depends on: dispatcher must route PostCompact events for this hook to fire

## Architecture Anchors

- `plugins/vsdd-factory/hooks/postcompact-reanchor.sh` — NEW shell hook (S-18.05 deliverable); read-only with respect to factory-artifacts
- `plugins/vsdd-factory/hooks-registry.toml` — `[[hooks]] event = "PostCompact"` entry for this hook (S-18.05 deliverable)
- `.factory/logs/postcompact-reanchor-YYYY-MM-DD.jsonl` — daily log file written by this hook (same pattern as dispatcher-internal log)
- ADR-026 §Decision 7 (PostCompact re-anchor advisory hook; cannot block; re-reads STATE.md pointer from factory-artifacts)

## Story Anchor

S-18.05 (postcompact-reanchor.sh advisory hook)

## VP Anchors

TBD-VP — no dedicated VP assigned at F2. Justification for deferral: this hook is explicitly best-effort and not in the CAP-032 guarantee chain (DI-024). A VP for a best-effort advisory hook that carries no correctness guarantee is appropriate to defer — there is no blocking invariant that the VP would guard. A unit-test VP verifying the stdout format and log-write behavior would be appropriate at F3 to prevent silent regressions to the convenience re-anchor behavior. Story-writer assigns at F3. Flagged to architect for final VP allocation decision.

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| TBD-VP | postcompact-reanchor.sh emits re-anchor block to stdout from factory-artifacts STATE.md (git-sourced, not in-context); appends to .factory/logs/; does NOT commit or push to factory-artifacts; exits 0 on all paths including crash/factory-artifacts-unreachable | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC specifies the PostCompact re-anchor hook that completes the mid-wave compaction durability story: after compaction, the LLM's in-context understanding of wave/step/SHA may be stale or hallucinated; this hook emits the authoritative externally-persisted values so the LLM can re-ground itself before the next tool call (ADR-026 Decision 7) |
| L2 Domain Invariants | DI-024 (PostCompact re-anchor is best-effort and carries no correctness guarantee; it is not in the CAP-032 continuity-guarantee chain — enforced by hook design: cannot block, does not commit, failure has no CAP-032 impact) |
| Architecture Module | SS-07 (Hook Bash Layer) — shell hook in `plugins/vsdd-factory/hooks/`; registry entry in `hooks-registry.toml` |
| ADR | ADR-026 §Decision 7 (PostCompact re-anchor: advisory shell hook; best-effort; cannot block; NOT in CAP-032 guarantee chain; re-reads STATE.md pointer from factory-artifacts; emits re-anchor block; does not commit) |
| Stories | S-18.05 |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |
