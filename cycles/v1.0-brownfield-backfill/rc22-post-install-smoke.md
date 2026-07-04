# RC22 Post-Install Smoke Report

**Date:** 2026-07-04
**Producer:** orchestrator-coordinated 3-leg smoke (authorized by human)
**Scope:** rc.22 operator cache at `~/.claude/plugins/cache/claude-mp/vsdd-factory/1.0.0-rc.22/`
**Evidence commit (develop):** f5242bef; evidence JSONL: `.factory/logs/dispatcher-internal-2026-07-04.jsonl`
**Authorized by:** human 2026-07-04 (session resume after D-750 wrap)

---

## Combined Verdict: PASS-WITH-FINDINGS

**rc.22 operator install functionally sound.** No crash, no timeout, no unexpected block across 73 hook registrations. Three functional findings anchored to new stories (S-19.01..S-19.05).

---

## Leg 1 — Cache Inventory and Parity

**Verdict: PASS-WITH-FINDINGS**

Cache path: `~/.claude/plugins/cache/claude-mp/vsdd-factory/1.0.0-rc.22/`

- **37/37 WASMs present** — byte-identical to shipped bundle a04cb303 (darwin-arm64 dispatcher SHA-256: `8f6fbef360b05036c1f053bb82d00e706db5b1749501ac05cb250ec3b0766911`).
- **plugin.json version:** `1.0.0-rc.22` PASS.
- **hooks-registry.toml:** 73 hook registrations; 33 unique hook plugins referenced; 0 referenced-but-missing.
- **bash hooks:** 36/36 pass `bash -n` syntax check.
- **resolvers-registry.toml:** parses cleanly; `wave_context` resolver entry → `hook-plugins/vsdd-context-resolvers.wasm` present.

### Findings

- **F-1 MEDIUM** — 3 orphan WASMs unreferenced by hooks-registry.toml:
  - `hello-hook.wasm` (169,303 B)
  - `vsdd_context_resolvers.wasm` (341,975 B — underscore-variant; different from hyphen-named `vsdd-context-resolvers.wasm` which IS referenced by resolvers-registry.toml)
  - `wasm_resolver_export.wasm` (134,799 B)
  - Anchor: **S-19.04** (bundle hygiene + tool-filter anchoring)
- **F-3 LOW cosmetic** — `windows-x64` exe lacks POSIX exec bit (irrelevant for PE format; no action required).

**Correction recorded (D-751 append-only):** D-750/D-749 stated PR #431 deleted 11 orphan WASMs; `git show 35b345f4 --diff-filter=D` shows 10 deletions. D-750 stated "zero underscore stubs" (narrowly true for deleted ≤103B placeholders) but 2 full-size underscore-named artifacts (`vsdd_context_resolvers.wasm`, `wasm_resolver_export.wasm`) still ship unreferenced — cleanup anchored S-19.04.

---

## Leg 2 — 73-Registration Firing Matrix (D-748 Protocol)

**Verdict: 73/73 PASS — 0 crashes, 0 timeouts, 0 unexpected blocks**

Protocol: CACHE dispatcher/registry/WASMs (operator cache, not develop source). 18 positive dispatch groups + 2 negative controls all PASS. `registry_path` confirmed CACHE on every `dispatcher.started` record. Expected handoff-validator SubagentStop exit=2 advisory-block reproduced (block reason: `subagent_truncated_result`). All 17 `on_error=block` registrations silent on benign payloads.

Regex-search tool-filter semantics confirmed: `Edit|Write` pattern fires on MultiEdit events.

Evidence file: `.factory/logs/dispatcher-internal-2026-07-04.jsonl`

### Findings

- **FINDING-1 FUNCTIONAL** — `verify-factory-lock` internal `capability_denied read_file .factory/STATE.md reason=output_too_large` on every PreToolUse Edit/Write/MultiEdit/Agent dispatch (traces a4b26f12 / bcc3e6ef / cf4c2e4d / 2551d7db; `StateReadError: OutputTooLarge`). Lock gate silently degraded when STATE.md large — anchored **S-19.02**.
- **FINDING-2 FUNCTIONAL** — `warn-pending-wave-gate` `capability_denied read_file .factory/wave-state.yaml reason=path_not_allowed` (trace bc687a0f); root cause: `read_file.rs path_allowed()` `canonicalize()` returns false for non-existent files, conflating absent-file with path-not-allowed — anchored **S-19.03**.
- **FINDING-3 INFO** — tool-filter regex-SEARCH semantics undocumented — anchored **S-19.04**.
- **FINDING-4 INFO** — async plugins emit `plugin.invoked` but no `plugin.completed`; async hangs invisible below 5000ms timeout (4 real `capture-pr-activity` timeouts observed in live log) — anchored **S-19.05**.
- **FINDING-5 INFO / RECORD CORRECTION** — D-748 baseline stated 15 `on_error=block` registrations; rc.22 registry has **17** (additions: `lint-registry-async-invariant` PostToolUse; `validate-stable-anchors` PreToolUse `Edit|Write`).

---

## Leg 3 — Context-Durability System

**Verdict: PASS-WITH-FINDINGS**

- **Resolvers live-load:** "Compiled 1 resolver modules" — PASS.
- **precompact-flush PreCompact END-TO-END** (via `/tmp` fixture): factory-artifacts HEAD advanced in fixture; flush log written; real repo untouched — verified real HEAD `ecc04c78` unchanged — PASS.
- **postcompact-reanchor END-TO-END:** `[PostCompact Re-anchor]` block + BC-7.07.002 PC2-conformant 6-field JSONL; exit 0 both paths — PASS.
- **rehydrate-wave NEGATIVE:** `RehydrationError` exit 1 (BC-6.24.001 PC7) — PASS.
- **rehydrate-wave POSITIVE:** `INJECTED_FILE_COUNT=6` sentinel; dedup union; operator confirmation prompt; exit 0 — PASS.
- **EC-004 + PC6 warning paths** — PASS.
- **handoff-validator advisory-block on empty subagent result** — PASS.
- **git_context injection arm (ADR-029):** plugin invoked; fail-open; no crash — PASS.
- **wave-handoff.sh + lib scripts:** `bash -n` clean — PASS.
- **destructive-command-guard:** correctly blocked two `rm -rf .factory` compound commands during fixture teardown (bonus live evidence).

### Findings

- **F1 LOW** — `VSDD_SINK_FILE` sink gated `#[cfg(debug_assertions)]`; release dispatcher emits no sink JSONL — anchored **S-19.05**.
- **F2 INFO** — duplicate resolver WASM variants (~0.5MB total; `vsdd_context_resolvers.wasm` underscore variant + `wasm_resolver_export.wasm`) — anchored **S-19.04**.

---

## Summary Table

| Leg | Verdict | Functional Findings | Anchored To |
|-----|---------|---------------------|-------------|
| Leg 1 Cache inventory | PASS-WITH-FINDINGS | F-1 MEDIUM (3 orphan WASMs); F-3 LOW cosmetic | S-19.04 |
| Leg 2 73-registration firing | PASS-WITH-FINDINGS | FINDING-1 lock gate silent; FINDING-2 path_not_allowed; FINDING-3 tool-filter docs; FINDING-4 async telemetry | S-19.01–S-19.05 |
| Leg 3 Context durability | PASS-WITH-FINDINGS | F1 sink no-op in release; F2 duplicate resolver WASMs | S-19.04, S-19.05 |

**Overall: PASS-WITH-FINDINGS. rc.22 operator install functionally sound. Hardening stories drafted: S-19.01..S-19.05 (E-19 epic, 34pts, 2-wave DAG).**
