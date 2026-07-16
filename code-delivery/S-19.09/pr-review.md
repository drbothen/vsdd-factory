# PR Review — S-19.09 (PR #659)

**Reviewer:** pr-reviewer (fresh-eyes, different-model cognitive diversity)
**PR:** #659 — feat(S-19.09): D19-D22 post-E-19 host ABI fixes
**Base:** develop (9787c056) · **Head:** a15d3bec
**Verdict:** ✅ **APPROVE** — no blocking findings

---

## Scope reviewed

Full diff reviewed independently (diff + PR description + test evidence only; no `.factory/` internal artifacts):

- **D19** — `crates/factory-dispatcher/src/invoke.rs`: `read_prefix` registration on the production `Linker<StoreData>` path (`setup_host_on_store_data`).
- **D20** — `read_file.rs` / `read_prefix.rs`: `timeout_ms` comment corrections + two-linker protocol note.
- **D21** — `internal_log.rs` / `emit_event.rs`: named-constant refactor for telemetry literals.
- **D22** — `emit_event.rs`: `timestamp` field on `plugin.completed` async event.
- Tests: 10 new cargo tests + `host-abi-hygiene.bats` (T-004..T-012) + demo evidence.

## 8-item checklist

1. **Diff coherence** — PASS. Every change maps to D19–D22; no stray edits; no spec/hook-sdk/hooks-registry/HOST_ABI_VERSION changes (matches stated non-scope).
2. **Description accuracy** — PASS with one LOW discrepancy (test count; see findings).
3. **Test coverage** — PASS. Changed lines covered by load-bearing tests (byte-equality round-trip, head-c clamp, empty-file no-grow, capability denial, const value-pins, timestamp field, bats hygiene sweep with mutation-liveness).
4. **Demo evidence** — PASS. Rust library / host-ABI change (no UI); captured-stdout transcripts are the correct demo-recorder mode. `evidence-report.md` present with per-AC coverage matrix (all 10 ACs). `.txt` transcripts NOT flagged blocking — the `.txt`=BLOCKING rule targets UI products.
5. **Commit quality** — PASS (conventional `feat(S-19.09):`).
6. **Diff size** — 1822 additions but production code is small; bulk is tests (~600 invoke.rs, ~477 bats) + evidence (~500). Appropriate.
7. **Missing changes** — none identified; all 10 ACs traced to tests.
8. **Dependency status** — Upstream #657 (S-19.06) and #646 (S-19.08) MERGED. S-19.07 correctly BLOCKED downstream on this merge (O-P3-001 deferral gate).

## Verification highlights

- **D19 parity:** the new `read_prefix` `func_wrap` block is a faithful port of the adjacent proven `read_file` production block — same string read → `prepare` capability delegation → empty-body fast path → `memory.grow(ceil(len/65536))` → write at `current_bytes` → out-param marshalling. `read_prefix::prepare` enforces its own capability block (no fallback to `read_file`, per BC-1.17.001 Invariant 3). WASM page alignment guarantees a valid `write_offset`.
- **D22:** `let ts = ev.ts.clone()` + `.with_field("timestamp", ts.as_str())` exactly mirrors sibling emitters (`emit_plugin_timeout_async`, `emit_plugin_abandoned`) — consistent, not a novel wire shape.
- **Bats T-011 literal sweep:** strips `#[cfg(test)]` boundary + block comments before grepping the four literals, with mutation-liveness injections asserting the gate fires. Real regression lock, not a paper-fix.

## Findings

| Severity | Category | Finding | Suggestion |
|----------|----------|---------|------------|
| LOW | description | PR claims "8 new Cargo tests" but diff adds 10 (omits `plugin_completed_value_pin` + `plugin_timeout_value_pin` in internal_log.rs, which the PR acknowledges in-code as "adjudicated value-pins"). | Update the count to 10 for accuracy. Cosmetic; no correctness impact. |
| LOW | process | Checklist items "Security review completed", "PR reviewer convergence", "All CI status checks passing" unchecked; security review marked pending. | Human merge gate (D-665 STOP) should confirm the security-reviewer pass runs before merge. D19 touches path resolution + capability enforcement + WASM memory grow, all mirroring audited `read_file` code, so risk is LOW. |
| INFO | pre-existing | `write_offset = current_bytes as u32` / `body.len() as u32` share the theoretical 4GiB-wrap edge with the existing `read_file` block — identical parity, bounded by `max_bytes: u32`, not a regression. PR's "Accepted-with-Record" already anchors the untested `memory.grow`-failure branch as a follow-on hardening candidate; concur it is out of S-19.09 AC scope. | No action for this PR. |

## Summary

Clean, disciplined fix. The critical latent gap (production `Linker<StoreData>` missing the `read_prefix` binding that the test-path linker had) is closed with a verbatim-parity port of proven code, backed by tests exercising link success, byte-correct round-trip, head-c truncation, empty-file no-grow, and capability denial through the actual production path. Only two LOW findings (test-count wording, human-gated process checkboxes), neither blocking. **APPROVE**, subject to the standard D-665 human merge gate and completion of the pending security review.
