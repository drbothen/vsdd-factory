# Demo Evidence: S-5.03 (WorktreeCreate / WorktreeRemove hook wiring)

**Story:** S-5.03 — WorktreeCreate / WorktreeRemove hook wiring
**Spec convergence:** pass-14 (v2.5), CONVERGENCE_REACHED per ADR-013
**GREEN commit:** `8336cd0`
**Test status:** 11/11 integration tests GREEN; clippy clean; no workspace regressions
**VP:** VP-067 — Worktree Hook Plugin Surface Invariant (all BC-4.07.001–004 postconditions)

---

## AC Evidence Files

| File | AC | BCs | One-sentence summary |
|------|----|-----|----------------------|
| [AC1-routing-path.md](AC1-routing-path.md) | AC1 | BC-4.07.003 + BC-4.07.004 | Verifies Layer 1 (`hooks.json.template`) routes both worktree events to `factory-dispatcher` binary, and Layer 2 (`hooks-registry.toml`) routes both to `worktree-hooks.wasm`. |
| [AC2-worktree-create-wire-payload.md](AC2-worktree-create-wire-payload.md) | AC2 | BC-4.07.001 | Documents the 10-field wire payload (2 plugin-set + 4 host-enriched + 4 construction-time) for `worktree.created`; includes zero-capability proof (Option A scoping). |
| [AC3-worktree-remove-wire-payload.md](AC3-worktree-remove-wire-payload.md) | AC3 | BC-4.07.002 | Documents the 9-field wire payload (1 plugin-set + 4 host-enriched + 4 construction-time) for `worktree.removed`; includes EC-002 unknown-worktree behavior. |
| [AC4-hooks-json-template.md](AC4-hooks-json-template.md) | AC4 | BC-4.07.003 | Captures both worktree entries: `command` = dispatcher binary path, `once` key **completely absent**, `async: true`, `timeout: 10000`. |
| [AC5-hooks-registry-toml.md](AC5-hooks-registry-toml.md) | AC5 | BC-4.07.004 | Captures TWO `[[hooks]]` entries routing to the same `worktree-hooks.wasm`; zero capability tables; no `once` field; single-crate-two-entries design. |
| [AC6-vp067-integration-test.md](AC6-vp067-integration-test.md) | AC6 | VP-067 (all 4 BCs) | Full `cargo test -p worktree-hooks` output; test-to-BC-to-AC coverage map for all 11 tests. |

---

## Architecture Context

S-5.03 wires the third and fourth lifecycle events in epic E-5 (FR-046), following S-5.01 (SessionStart) and S-5.02 (SessionEnd). The same dual-routing-table architecture (ADR-011) applies:

- **Layer 1** (`hooks.json.template`): Claude Code harness routing — references only the dispatcher binary; `once` key **absent** (worktree events can re-fire on reconnect — differs from session events which use `once: true`)
- **Layer 2** (`hooks-registry.toml`): Dispatcher routing — references only WASM plugin paths; zero capability tables declared

Single-crate design: `worktree-hooks.wasm` handles both events (BC-4.07.004 single-crate-two-entries decision). This contrasts with S-5.01/S-5.02 which each used separate crates per event.

Key contrasts with sibling stories:

| Property | WorktreeCreate (S-5.03) | WorktreeRemove (S-5.03) | SessionEnd (S-5.02) | SessionStart (S-5.01) |
|----------|------------------------|------------------------|--------------------|-----------------------|
| Plugin-set fields | 2 (`worktree_path`, `worktree_name`) | 1 (`worktree_path`) | 3 | 6 |
| Wire total | 10 fields | 9 fields | 11 fields | 14 fields |
| `exec_subprocess` calls | 0 | 0 | 0 | 1 |
| Capability tables | 0 | 0 | 0 | 2 |
| `once` in hooks.json.template | absent | absent | `true` | `true` |
| `timeout_ms` (Layer 2) | 5000 | 5000 | 5000 | 8000 |
| Crate | shared (`worktree-hooks`) | shared (`worktree-hooks`) | separate | separate |

Timeout hierarchy (two-level): `timeout_ms = 5000` (dispatcher) < `timeout = 10000` (harness).

---

## Verification Property

**VP-067** — Worktree Hook Plugin Surface Invariant
Proof method: integration
BCs covered: BC-4.07.001, BC-4.07.002, BC-4.07.003, BC-4.07.004
Test file: `crates/hook-plugins/worktree-hooks/tests/integration_test.rs`
