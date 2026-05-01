---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.07-native-port-warn-pending-wave-gate.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: d068e19
traces_to: prd.md
pass: 2
previous_review: adv-s8.07-p1.md
target: S-8.07 v1.1
target_file: .factory/stories/S-8.07-native-port-warn-pending-wave-gate.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 11
findings_high: 4
findings_med: 4
findings_low: 2
findings_nit: 1
---

# Adversarial Review: S-8.07 v1.1 (Pass 2)

## Finding ID Convention

Finding IDs use the format: `F-S807-P2-NNN`

- `F`: Fixed prefix
- `S807`: Story 8.07
- `P2`: Pass 2
- `NNN`: Three-digit sequence (001, 002, ...)

## Summary

Pass-2 fresh-context review of S-8.07 v1.1 (433 lines). 13 of 14 pass-1
findings fully closed; F-012 (subsystem anchor) was partially closed —
SS-04 was added but the canonical SS-04 identity was mis-applied, and the
path dependency fix introduced a new structural error.
11 new findings: 4 HIGH, 4 MED, 2 LOW, 1 NIT.
Verdict: **SUBSTANTIVE** — fix burst required before pass-3. Clock held at
0_of_3. This story has the MOST new HIGH findings in the Tier 1 pass-2 batch.

Trajectory: 14 → 11 (21% decay). Critical new discoveries include wrong
vsdd-hook-sdk path (cargo build-breaking), missing workspace registration
(cargo build-breaking), and SS-04/SS-02 universal mis-anchoring.

---

## Part A — Fix Verification (Pass-1 Closure Audit)

13 of 14 pass-1 findings fully closed. F-012 partially closed.

| Finding | Description | Closed? | Evidence |
|---------|-------------|---------|---------|
| F-S807-P1-001 | wasm32-wasi → wasm32-wasip1 | CLOSED | Cargo.toml updated |
| F-S807-P1-002 | hooks.json deletion → positive verification | CLOSED | AC reframed |
| F-S807-P1-003 | CAP-022 cross-CAP stretch disclosure | CLOSED | Disclosure present |
| F-S807-P1-004 | wave: 15 [process-gap] disclosure | CLOSED | Disclosure present |
| F-S807-P1-005 | input-hash convention | CLOSED | Convention applied |
| F-S807-P1-006 | AC perf gate dropped (Tier 1 exclusion) | CLOSED | AC updated |
| F-S807-P1-007 | serde_yaml pinned 0.9.34 with TD entry | CLOSED | Pin present; TD logged |
| F-S807-P1-008 | emit_event signature mapped | CLOSED | Signature referenced |
| F-S807-P1-009 | python3 soft-dep path mapped | CLOSED | Path mapped |
| F-S807-P1-010 | read_file capability declaration | CLOSED | Declaration present |
| F-S807-P1-011 | wave: 15 [process-gap] disclosure added | CLOSED | Disclosure present |
| F-S807-P1-012 | subsystems SS-04 added | CLOSED (PARTIAL → F-S807-P2-001) | SS-04 added but ARCH-INDEX meaning mis-applied; introduces new path defect |
| F-S807-P1-013 | vsdd-hook-sdk path = crates/hook-sdk | CLOSED (PARTIAL → F-S807-P2-002) | Path added but resolves to wrong directory |
| F-S807-P1-014 | EC-005 read_file error handling | CLOSED | EC updated |

---

## Part B — New Findings (Pass-2 Fresh-Context Discovery)

### F-S807-P2-001 [HIGH] SS-04 canonical name mis-anchored — SDK surface concerns belong in SS-02

**Location:** Frontmatter `subsystems:` and story body Architecture section.

**Issue:** The pass-1 burst added SS-04 to subsystems with the description
"Hook SDK." Per ARCH-INDEX.md, SS-04 is "Plugin Ecosystem" — not "Hook SDK."
SS-02 is "Hook SDK and Plugin ABI." For hooks exercising the vsdd-hook-sdk
surface (emit_event, read_file, the WASM ABI), the correct primary subsystem
anchor is SS-02. SS-04 is appropriate for the plugin crate packaging and
registry aspects. This universal mis-anchoring affects S-8.07, S-8.08, and
S-8.09 — the pass-1 burst applied SS-04 inconsistently across all three.

**Suggested fix:** Add SS-02 as primary SDK-surface anchor. Keep SS-04 for
plugin ecosystem coverage. Update Architecture section to reflect both with
correct descriptions: SS-02 (Hook SDK and Plugin ABI) + SS-04 (Plugin Ecosystem).

**Policy:** POLICY 2 (BC anchor integrity — subsystem anchors must match
ARCH-INDEX canonical names).

---

### F-S807-P2-002 [HIGH] vsdd-hook-sdk path dependency resolves to non-existent directory

**Location:** Cargo.toml dependency section, path = field.

**Issue:** The story specifies `path = "../../crates/hook-sdk"` for the
vsdd-hook-sdk dependency. From the crate root at
`crates/hook-plugins/warn-pending-wave-gate/`, this resolves to:
`crates/hook-plugins/warn-pending-wave-gate/../../crates/hook-sdk` =
`crates/crates/hook-sdk` — which does not exist. The correct relative path
is `../../hook-sdk` (two levels up from the crate, landing at the workspace
root's `crates/` level, then into `hook-sdk`). This is a cargo build-breaking
defect that would prevent compilation.

**Suggested fix:** Change `path = "../../crates/hook-sdk"` to
`path = "../../hook-sdk"`.

**Policy:** POLICY 6 (measurability — path must resolve to an existing
directory).

---

### F-S807-P2-003 [HIGH] New crate not added to workspace `members = [...]`

**Location:** File Structure / Tasks — workspace Cargo.toml.

**Issue:** The story specifies creation of a new crate at
`crates/hook-plugins/warn-pending-wave-gate/` but does not include a Task
to add this crate to the workspace root `Cargo.toml` `members = [...]` array.
Without workspace registration, `cargo build --workspace` will not include
the new crate, and `cargo build -p warn-pending-wave-gate` will fail with
"package not found in workspace."

**Suggested fix:** Add to File Structure: `Cargo.toml (workspace root) — add
"crates/hook-plugins/warn-pending-wave-gate" to members = [...]`. Add
corresponding Task.

**Policy:** POLICY 1 (lifecycle completeness — all file changes must be listed).

---

### F-S807-P2-004 [HIGH] read_file `max_bytes` and `timeout_ms` mandatory arguments unspecified

**Location:** T-3, read_file call description.

**Issue:** The story references `host::read_file` for reading wave-state.yaml
but does not specify the mandatory `max_bytes` and `timeout_ms` arguments. The
vsdd-hook-sdk read_file signature requires these positional arguments. The sibling
pattern established in converged stories specifies `(path, 65536, 1000)` —
64KB max and 1s timeout. Without pinning these, the implementer must guess,
risking silent truncation of large wave-state files or indefinite hangs.

**Suggested fix:** Pin the call in T-3:
```rust
host::read_file(&wave_state_path, 65536, 1000)
```
Note: "max_bytes=65536 (64KB); wave-state.yaml is expected to be <10KB;
64KB provides a 6× safety margin."

**Policy:** POLICY 6 (measurability — all mandatory arguments must be pinned).

---

### F-S807-P2-005 [MED] HookResult return value unspecified — Continue vs Block vs Error

**Location:** T-3, decision logic description.

**Issue:** The story describes the hook's warn vs block vs pass decision logic
but does not specify which `HookResult` enum variant is returned in each case.
The SDK exposes `HookResult::Continue`, `HookResult::Block`, and
`HookResult::Error`. The distinction between Block and Error matters: Block
returns structured JSON to the harness (displayed to the user); Error is a
runtime failure (may be silently suppressed or logged differently).

**Suggested fix:** Add a decision table to T-3:
- Wave gate pending + soft warn: `HookResult::Continue` + stderr warning
- Wave gate pending + hard block: `HookResult::Block { message: "..." }`
- Read failure: `HookResult::Error { message: "..." }`

**Policy:** POLICY 6 (measurability — return value must be pinned per branch).

---

### F-S807-P2-006 [MED] src/main.rs binary entry point unspecified — [[bin]] vs #[hook] macro

**Location:** File Structure section, src/main.rs.

**Issue:** The File Structure lists `src/main.rs` but does not specify whether
the binary entry point uses the `[[bin]]` target in Cargo.toml or a `#[hook]`
macro pattern. The choice affects compilation output and wasm32-wasip1 build
flags. The sibling stories that have converged should be cited as pattern.

**Suggested fix:** Specify: "src/main.rs — `[[bin]]` entry per capture-commit-
activity sibling pattern. No `#[hook]` macro; direct fn main() entry point."

**Policy:** POLICY 1 (lifecycle completeness — entry point pattern must be
unambiguous).

---

### F-S807-P2-007 [MED] AC-003 stderr format escape-sequence rendering ambiguous

**Location:** AC-003, stderr warning format string.

**Issue:** AC-003 specifies a stderr warning format string that includes `\n`
escape sequences. It is ambiguous whether `\n` should appear as the literal
two-character sequence in the output or as a rendered newline character. Bats
tests assert on exact string matches; the test must know which form to expect.

**Suggested fix:** Specify which rendering: "Newlines in the warning message
are rendered as actual newline characters (0x0A), not as literal backslash-n.
In bats: use `$'...\n...'` quoting syntax to match."

**Policy:** POLICY 6 (measurability — escape rendering must be pinned for
bats assertions).

---

### F-S807-P2-008 [MED] AC-005 bats test invocation method unspecified

**Location:** AC-005.

**Issue:** AC-005 specifies a bats test but does not specify whether bats
invokes the hook binary directly (.wasm via wasmtime), via the dispatcher, or
both. For WASM hooks, the dispatcher is the production invocation path; direct
.wasm invocation requires a separate wasmtime harness. The test design depends
on this choice and affects what can be asserted.

**Suggested fix:** Specify: "AC-005 bats test invokes the hook via the
vsdd-factory dispatcher (production path). Direct .wasm invocation is NOT
required for this AC; it is covered by the wasm32-wasip1 compilation check
in T-2."

**Policy:** POLICY 6 (measurability — test invocation method must be explicit).

---

### F-S807-P2-009 [LOW] T-5 stdin payload undeclared — Stop hooks expect a JSON envelope

**Location:** Task T-5, bats test setup.

**Issue:** T-5 describes the bats test setup but does not declare the stdin JSON
payload. Stop hooks receive a structured JSON envelope from Claude Code. Without
specifying the envelope shape (or citing the canonical E-8 envelope schema),
the bats test author cannot construct a valid test input.

**Suggested fix:** Add to T-5: "Provide stdin with the canonical Stop hook
envelope: `{"hook_event_name": "Stop", "session_id": "test-session-001",
"transcript_path": "/tmp/test-transcript.jsonl"}`."

**Policy:** POLICY 6 (measurability — test inputs must be complete).

---

### F-S807-P2-010 [LOW] Token Budget table cites "200K for Sonnet" — dated reference

**Location:** Token Budget section.

**Issue:** The Token Budget section cites "200K for Sonnet" as the context
window. The current Sonnet 4.6 model has a different window configuration.
The reference should be made version-independent.

**Suggested fix:** Replace "200K for Sonnet" with "model context window (see
current model documentation)" or remove the specific figure.

**Policy:** NIT-adjacent — minor staleness risk.

---

### F-S807-P2-011 [NIT] T-3 second sub-bullet refers to `data.get` — should be `wave_data.get`

**Location:** Task T-3, second sub-bullet.

**Issue:** In T-3, the second sub-bullet refers to `data.get("gate_status")`
where the binding established in the first sub-bullet is `wave_data`. Using
`data` rather than `wave_data` is a variable name inconsistency that would not
compile.

**Suggested fix:** Change `data.get("gate_status")` to
`wave_data.get("gate_status")`.

**Policy:** NIT — internal consistency.

---

## Part C — Cross-Document Consistency Audit

| Check | Target | Result |
|-------|--------|--------|
| S-8.07 subsystems includes SS-02 for SDK surface | Frontmatter | FAIL (F-S807-P2-001) |
| vsdd-hook-sdk path resolves correctly | Cargo.toml path = | FAIL (F-S807-P2-002) |
| Workspace Cargo.toml members update present | File Structure / Tasks | FAIL (F-S807-P2-003) |
| read_file max_bytes + timeout_ms pinned | T-3 | FAIL (F-S807-P2-004) |
| wasm32-wasip1 Architecture Compliance row | Story body | PASS |
| BC-7.03.091/092 trace present | BC Trace table | PASS |
| wave: 15 [process-gap] disclosure | Story body | PASS |
| serde_yaml 0.9.34 pin + TD entry | Cargo.toml + story body | PASS |

---

## Part D — Policy Compliance Sweep

| Policy | Description | Result | Notes |
|--------|-------------|--------|-------|
| POLICY 1 — Lifecycle completeness | Workspace members Task missing; main.rs entry pattern | FAIL | F-S807-P2-003/006 |
| POLICY 2 — BC anchor integrity | SS-04 identity mis-applied | FAIL | F-S807-P2-001 |
| POLICY 3 — State-manager-runs-last | No state-manager scope items | PASS | |
| POLICY 4 — Input-hash currency | input-hash d068e19 present | PASS | |
| POLICY 5 — Dependency symmetry | depends_on=[S-8.00] | PASS | |
| POLICY 6 — Measurability | Path, max_bytes, HookResult, escape rendering, invocation | FAIL | F-S807-P2-002/004/005/007/008 |
| POLICY 7 — Cross-document consistency | SS-02 vs SS-04 naming | FAIL | F-S807-P2-001 |
| POLICY 8 — Scope boundary | No scope violations | PASS | |

---

## Part E — Self-Validation Loop

**Iteration 1:** Re-read all 11 findings. F-S807-P2-002 confirmed HIGH: the
path `../../crates/hook-sdk` from `crates/hook-plugins/warn-pending-wave-gate`
definitely resolves to `crates/crates/hook-sdk` which does not exist.
F-S807-P2-003 confirmed HIGH: no workspace Cargo.toml task means new crate
is invisible to cargo. F-S807-P2-004 confirmed HIGH: SDK read_file requires
max_bytes and timeout_ms as mandatory positional args.

**Iteration 2:** Severity confirmed. 4 HIGH (2 are cargo build-breaking;
1 is ARCH-INDEX mis-anchor; 1 is mandatory arg omission). 4 MED (bats test
blocking). 2 LOW (informational). 1 NIT.

**Iteration 3:** No findings withdrawn. 11 findings stand.

---

## Part F — Novelty Assessment

| Finding | Novel or Pass-1 Prior? | Notes |
|---------|----------------------|-------|
| F-S807-P2-001 | Pass-1 partial closure | F-012 added SS-04 but wrong identity; new path defect introduced |
| F-S807-P2-002 | Pass-1 partial closure | F-013 added path dep but wrong relative path |
| F-S807-P2-003 | Novel | Workspace members registration not examined at pass-1 |
| F-S807-P2-004 | Novel | read_file mandatory args not examined at pass-1 |
| F-S807-P2-005 | Novel | HookResult variant mapping not examined at pass-1 |
| F-S807-P2-006 | Novel | main.rs entry point pattern not examined at pass-1 |
| F-S807-P2-007 | Novel | Escape sequence rendering not examined at pass-1 |
| F-S807-P2-008 | Novel | Bats invocation method not examined at pass-1 |
| F-S807-P2-009 | Novel | Stop hook stdin payload not examined at pass-1 |
| F-S807-P2-010 | Novel | Sonnet context window staleness not examined at pass-1 |
| F-S807-P2-011 | Novel | Variable name inconsistency not examined at pass-1 |

2 pass-1 partial closures (both introduced new defects) + 9 net-new findings.

---

## Part G — Process-Gap Tags

None.

---

## Verdict

**SUBSTANTIVE** — 4 HIGH + 4 MED findings require fix burst before pass-3.
Two HIGH findings (F-S807-P2-002 wrong path, F-S807-P2-003 missing workspace
member) are cargo build-breaking and must be treated as P0 in fix priority.

**Clock:** 0_of_3 → 0_of_3 (held per ADR-013; 4 HIGH findings present).

**Trajectory:** 14 → 11 (21% decay). Deep fix burst required.

**Pass-3 priors for adversary:**
- Verify vsdd-hook-sdk path = "../../hook-sdk" (not "../../crates/hook-sdk")
- Verify workspace Cargo.toml members Task present
- Verify read_file called with (path, 65536, 1000) — max_bytes + timeout_ms
- Verify SS-02 added for SDK-surface anchor; SS-04 for plugin ecosystem
- Verify HookResult variant mapped per decision branch
