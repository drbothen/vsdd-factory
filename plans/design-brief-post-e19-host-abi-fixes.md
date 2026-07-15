---
document_type: design-brief
version: "1.0"
date: 2026-07-15
producer: architect
status: ready-for-story-writer
authorized_by: human-directive-2026-07-15
scope: post-E-19 host ABI implementation fixes (5 systemic items from E-19 cascades + wave gate)
traces_to:
  - .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md
  - .factory/cycles/v1.0-brownfield-backfill/e-19-wave-gate-w1-w2.md
decisions:
  - ADR-025 Decision 16 (read_prefix production path registration)
  - ADR-025 Decision 17 (out_ptr=0 two-linker protocol documentation)
  - ADR-025 Decision 18 (timeout_ms non-enforcement framing)
  - ADR-025 Decision 19 (INVALID_ARGUMENT not in schema)
findings_closed:
  - F-WG-002 (wave gate LOW — bare string literals)
  - F-WG-003 (wave gate LOW — plugin.completed missing timestamp)
  - EC-006 (S-19.06 PR — timeout_ms drop / epoch interruption boundary)
  - SEC-001 sub-clause (two-linker protocol documentation)
  - SEC-003 (CWE-833 — confirmed LOW, framing corrected)
---

# Design Brief: post-E-19 Host ABI Implementation Fixes

**Authorization:** human-directed 2026-07-15 (post-E-19 follow-up story)
**Architect adjudication:** ADR-025 v1.16 (Decisions 16-19)
**Routing:** story-writer writes story spec; implementer executes under strict TDD

## Summary

Five systemic items surfaced during the E-19 S-19.06/S-19.08 cascades and the combined
W1+W2 wave gate require a follow-up implementation story. The architect has adjudicated
all five items in ADR-025 v1.16. This brief defines the proposed story scope, per-AC
grounding, affected files, purity boundaries, and verification approach for the
story-writer.

## Adjudication Verdicts

| Item | Source | Verdict | Action |
|------|--------|---------|--------|
| `read_prefix` production path gap | Architect analysis post-E-19 | Production gap confirmed — `read_prefix` absent from `setup_host_on_store_data` in `invoke.rs`; 0-hit grep confirmed 2026-07-15 | D19: register `read_prefix` in `setup_host_on_store_data` |
| `out_ptr=0` sentinel (SEC-001 sub-clause) | S-19.06 PR review | Two-linker protocol duality is intentional design; production path always writes at `current_bytes > 0`; accepted-with-record status confirmed correct | D20 partial: comment clarification in `read_file.rs` only; no code change |
| EC-006 / `timeout_ms` drop | S-19.06 PR review | "enforced via epoch interruption" comment is incorrect; epoch cannot preempt blocking `func_wrap`; `timeout_ms` is ABI-forward-reserved; SEC-003 LOW severity confirmed | D20: correct doc comments in `read_file.rs` + `read_prefix.rs` |
| SEC-003 lock-order (CWE-833) | S-19.06 PR review | LOW severity confirmed; `path_allow` is operator-configured; normal SSD paths never block; severity escalation not warranted | D20: framing correction only; no code change |
| `-4 INVALID_ARGUMENT` schema | S-19.06 EC-006 | NOT added to `hooks-registry.toml` capability schema table; marshalling-internal code; operator-invisible; current table (0,-1,-2,-5,-99) is correct | No change required |
| F-WG-002 bare string literals | Wave gate LOW | Named constants needed; `"internal.file_not_found"` and `"plugin.abandoned"` duplicated as bare literals across modules | D21: add constants to `internal_log.rs`; sweep call sites |
| F-WG-003 `plugin.completed` missing `timestamp` | Wave gate LOW | `emit_plugin_completed_async` omits `.with_field("timestamp", ...)` present on all sibling events | D22: add field matching sibling pattern |

## Proposed Story

**Suggested ID:** story-writer assigns per STORY-INDEX sequence; S-19.10 is a reasonable
candidate if W3 story numbering continues from S-19.08 as the last W2 story
**Title (suggested):** post-E-19 host ABI fixes — production `read_prefix`, `timeout_ms` framing, telemetry hygiene
**TDD mode:** strict
**Wave:** W3
**Depends on:** S-19.06 (merged 9787c056), S-19.08 (merged 1304d280)
**Blocks:** S-19.07 (BC-4.13.001 Phase-B migration requires functional `read_prefix` on production path)

## Acceptance Criteria Sketch

Story-writer formalizes these into BC-traced ACs with formal gating language. The
descriptions below are architect-intent.

### AC-001 — `read_prefix` instantiates on production path without link error (D19)

A WASM plugin binary with a `vsdd::read_prefix` import declaration MUST instantiate
without error when dispatched via `invoke_plugin` / `proxy_host_imports` (the production
dispatch path). Prior to this story, instantiation fails with a wasmtime link error for
any such plugin.

**Grounding:** ADR-025 Decision 16; confirmed by 0-hit grep in `invoke.rs`.

### AC-002 — `read_prefix` delivers correct bytes on production path (D19)

When a plugin calls `vsdd::read_prefix` on an existing, path-allowed file via the
production path, the hook-sdk `read_prefix()` wrapper MUST return `Ok(bytes)` with
content matching the actual file content up to `max_bytes`. The WASM output pointer
MUST be non-zero (production memory-grow protocol at `current_bytes`).

**Grounding:** ADR-025 Decision 16 + Decision 17 (production memory-grow protocol).

### AC-003 — `read_prefix` capability enforcement on production path (D19)

A `vsdd::read_prefix` call for a path not covered by `[hooks.capabilities.read_prefix]`
MUST return `CAPABILITY_DENIED (-1)` via the production path, matching `read_file`
behavior.

**Grounding:** ADR-025 Decision 15 (capability block requirement) + Decision 16.

### AC-004 — `timeout_ms` comments corrected in both host modules (D20)

`crates/factory-dispatcher/src/host/read_file.rs` and
`crates/factory-dispatcher/src/host/read_prefix.rs` MUST NOT contain the text
"enforced via epoch interruption" in any `let _ = timeout_ms` comment. The replacement
text MUST accurately characterize `timeout_ms` as ABI-forward-reserved with the
`func_wrap` dispatch path being structurally unenforced.

**Grounding:** ADR-025 Decision 18.

### AC-005 — Two-linker `out_ptr=0` protocol comment present (D20 partial)

`crates/factory-dispatcher/src/host/read_file.rs` MUST contain a comment adjacent to
the `prepare()` call in `register()` (or the `Ok((bytes, 0))` return form) that
distinguishes the test-path constant-0 return from the production memory-grow protocol
used in `invoke.rs::setup_host_on_store_data`.

**Grounding:** ADR-025 Decision 17.

### AC-006 — `INTERNAL_FILE_NOT_FOUND` named constant exported (D21)

`crates/factory-dispatcher/src/internal_log.rs` MUST export
`pub const INTERNAL_FILE_NOT_FOUND: &str = "internal.file_not_found";`.

**Grounding:** F-WG-002.

### AC-007 — `PLUGIN_ABANDONED` named constant exported (D21)

`crates/factory-dispatcher/src/internal_log.rs` MUST export
`pub const PLUGIN_ABANDONED: &str = "plugin.abandoned";`.

**Grounding:** F-WG-002.

### AC-008 — Bare string literals swept to named constants (D21)

No bare string literals `"internal.file_not_found"` or `"plugin.abandoned"` MAY appear
in `read_file.rs`, `read_prefix.rs`, or `host/emit_event.rs` after this story lands.
All sites MUST reference the named constants. All existing tests that assert on event
type strings MUST pass unmodified (the constant values are unchanged).

**Grounding:** F-WG-002. Verification: grep across affected files returns 0 hits.

### AC-009 — `plugin.completed` async event carries `timestamp` field (D22)

`emit_plugin_completed_async` in `crates/factory-dispatcher/src/host/emit_event.rs`
MUST emit an `InternalEvent` with a `timestamp` field containing a non-empty string,
matching the pattern established by all sibling async event emitters in the same file
(`emit_plugin_abandoned`, `emit_plugin_timeout_async`, etc.).

**Grounding:** F-WG-003; BC-3.08.001 §Common Fields (mandatory `timestamp` for all
`plugin.*` events); `emit_event.rs` lines 158-163 (sibling pattern).

### AC-010 — All existing tests pass with zero regressions (regression gate)

`cargo test --workspace --all-targets` and `cd plugins/vsdd-factory/tests && ./run-all.sh`
MUST both produce 0 failures after all changes.

## Affected Files

| File | Change type | Deliverable |
|------|-------------|-------------|
| `crates/factory-dispatcher/src/invoke.rs` | Add `read_prefix` binding to `setup_host_on_store_data` (behavioral: production path fix) | D19 |
| `crates/factory-dispatcher/src/host/read_file.rs` | Doc comment corrections: `timeout_ms` + `out_ptr=0` two-linker duality (doc only) | D20 |
| `crates/factory-dispatcher/src/host/read_prefix.rs` | Doc comment correction: `timeout_ms` framing (doc only) | D20 |
| `crates/factory-dispatcher/src/internal_log.rs` | Add `INTERNAL_FILE_NOT_FOUND` + `PLUGIN_ABANDONED` pub constants (refactor, no behavioral change) | D21 |
| `crates/factory-dispatcher/src/host/emit_event.rs` | Sweep bare literals to named constants; add `timestamp` field to `emit_plugin_completed_async` (D21 refactor + D22 behavioral fix) | D21 + D22 |

No files outside `crates/factory-dispatcher/` require modification. No spec files, no
hook-sdk files, no hooks-registry.toml changes (Decision 19 confirms existing table is
correct).

## Purity Boundaries

**In scope for implementer:**

- All five files listed above
- Adding cargo tests for AC-001 through AC-009 as Red Gate stubs first (TDD strict)
- Behavioral fixes: D19 (`read_prefix` production registration), D22 (`timestamp` field)
- Refactors: D21 (named constants + bare-literal sweep)
- Documentation corrections: D20 (doc comments in `read_file.rs`, `read_prefix.rs`)

**Out of scope — do NOT touch:**

- `plugins/vsdd-factory/config/artifact-path-registry.yaml` — no change needed
- `plugins/vsdd-factory/hooks-registry.toml` — Decision 19 confirms the existing
  `[hooks.capabilities.read_prefix]` preamble table (0,-1,-2,-5,-99) is correct; `-4`
  is NOT added
- `crates/hook-sdk/` — no changes required; `read_owned_bytes` ptr==0 guard and
  `read_prefix` SDK wrapper are correct as-is
- Any `.factory/` spec or story files — all adjudication is complete in ADR-025 v1.16;
  the implementer follows the decisions, does not amend them
- `HOST_ABI_VERSION` — remains `1`; no bump needed
- BC-3.08.001 or any other behavioral contract — the `timestamp` requirement already
  exists in BC-3.08.001 §Common Fields; D22 implements the missing field; no BC
  amendment is needed

## D19 Implementation Note

`setup_host_on_store_data` in `invoke.rs` already contains a complete `read_file`
production implementation. The `read_prefix` production implementation MUST follow
the same pattern:

1. Resolve path from WASM memory via `read_wasm_string`.
2. Check `ctx.capabilities.read_prefix.path_allow` — return `CAPABILITY_DENIED (-1)` if
   path is not covered.
3. Call `crate::host::read_prefix::prepare(&ctx, &path, max_bytes)` for the bounded
   file read (this function exists in `host/read_prefix.rs` and handles capability
   logic and the `read_prefix_bounded` call).
4. Empty body: write `ptr=0, len=0` via `write_wasm_u32_sd`, return `codes::OK`.
5. Non-empty body: grow WASM memory by `body.len().div_ceil(65536)` pages, capture
   `current_bytes` before grow, write body at `current_bytes` via `write_wasm_bytes_sd`,
   write real address and length to `out_ptr_out`/`out_len_out`.
6. Pass through error codes (`CAPABILITY_DENIED`, `NOT_FOUND`, `INVALID_ARGUMENT`,
   `INTERNAL_ERROR`) from `prepare()` return, matching the `read_file` production
   error-forwarding pattern in the same function.

## D22 Implementation Note

`emit_plugin_completed_async` in `host/emit_event.rs` (lines 374-394) chains
`.with_field(...)` calls on an `InternalEvent`. All sibling event emitters in the same
file capture a timestamp `ts` variable and add `.with_field("timestamp", ts.as_str())`.
The exact pattern (how `ts` is captured) is visible at lines 158-163, 194, 222, 260,
302, 343 — copy the same form. Add the missing `.with_field("timestamp", ts.as_str())`
before the final `ctx.emit_internal(ev)` call.

## Verification Approach

1. **TDD discipline (strict):** one failing Red Gate test per AC, then minimum code to
   pass, then micro-commit. No batch implementation.

2. **D19 test pattern:** extend the existing integration test infrastructure in
   `crates/factory-dispatcher/src/host/read_prefix.rs` tests (T-001 through T-013).
   Add new tests exercising `setup_host_on_store_data` directly, or add bats tests in
   `plugins/vsdd-factory/tests/` following the existing pattern for `read_prefix.bats`
   (if it exists) or `read-file.bats` as precedent.

3. **D20 comment verification:** grep-based assertion that the text
   `"enforced via epoch interruption"` does not appear in `read_file.rs` or
   `read_prefix.rs`. Can be a bats test or a cargo test using `include_str!` + assertion.

4. **D21 constant sweep:** grep-based assertions verifying 0 bare-literal occurrences
   of `"internal.file_not_found"` and `"plugin.abandoned"` in the three target files
   after the sweep.

5. **D22 timestamp test:** unit test in `emit_event.rs` mod tests following the T-003
   pattern (uses capture-based mock or direct `HostContext` with injected sink). Assert
   the captured `plugin.completed` event has `fields["timestamp"]` as a non-empty string.

6. **Regression gate:** full `cargo test --workspace --all-targets` and full bats suite
   green at PR open. CI enforces this.
