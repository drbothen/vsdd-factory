---
document_type: behavioral-contract
level: L3
version: "1.0"
status: withdrawn
producer: product-owner
timestamp: 2026-05-03T00:00:00Z
phase: 1
inputs:
  - .factory/specs/architecture/decisions/ADR-014-tier-2-native-wasm-migration.md
  - .factory/specs/architecture/SS-02-hook-sdk.md
input-hash: "[pending-recompute]"
traces_to: .factory/specs/domain-spec/capabilities.md
origin: greenfield
subsystem: "SS-02"
capability: "CAP-022"
lifecycle_status: withdrawn
withdrawn_date: 2026-05-03
withdrawn_reason: |
  ADR-014 D-9.2 amendment 2026-05-03 — gap analysis confirmed
  existing host::exec_subprocess (BC-2.02.005) sufficient for the only
  W-16 use case (validate-wave-gate-prerequisite invoking
  verify-sha-currency.sh). Section 7 of gap-analysis-w16-subprocess.md.
  Preserved as audit trail per POLICY 1 (append-only numbering).
superseded_by: "BC-1.05.035 (path traversal guard) + BC-1.05.036 (success telemetry) (SS-01 cluster, NEW 2026-05-03)"
introduced: v1.2
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

> **WITHDRAWN 2026-05-03.** This BC was authored 2026-05-03 (D-219) and
> withdrawn the same day after gap analysis revealed `host::exec_subprocess`
> (BC-2.02.005) already in production and sufficient for the W-16 use case.
> All 24 MUST invariants below preserved verbatim as audit trail. The 2 minor
> additive extensions actually adopted are recorded in BC-1.05.035 (path
> traversal guard) + BC-1.05.036 (success telemetry) in the SS-01 cluster
> (NEW 2026-05-03). See ADR-014 Amendment 2026-05-03 for full reasoning.
> Preserved per POLICY 1 (append-only numbering).

# BC-2.02.013: host::run_subprocess ABI invariants — SubprocessCaps enforcement, SubprocessResult contract, security boundaries

## Description

`host::run_subprocess` executes a guest-specified binary directly (no shell) through the dispatcher's capability-gated subprocess invocation path. The call is governed by a per-plugin `SubprocessCaps` schema that enforces a binary allow-list (glob), per-argument allow-list (glob), env var allow-list (strip-by-default), and bounded output/timeout limits. This is the third additive host function extension after `host::write_file` (BC-2.02.011, W-15 D-6 Option A) and HookPayload SubagentStop fields (BC-2.02.012, D-183). `HOST_ABI_VERSION` stays at 1.

## Preconditions

1. A plugin author calls `host::run_subprocess(spec: &SubprocessSpec)` where `SubprocessSpec` contains `binary: String`, `args: Vec<String>`, `env: HashMap<String, String>`, `timeout_ms: u64`, and `working_dir: Option<PathBuf>`.
2. The dispatcher context (`ctx`) carries the plugin's declared capabilities, including a `capabilities.run_subprocess` block of type `SubprocessCaps` in `hooks-registry.toml`.
3. `SubprocessCaps.binary_allowlist` is non-empty (an empty allowlist means no binary can ever be approved; callers with empty allowlist receive `CapabilityDenied` on every call).
4. `spec.binary` is a non-empty string (empty binary string is rejected as `InvalidArgument`).
5. The dispatcher has resolved its own working directory as a preopened path (CLAUDE_PROJECT_DIR or descendant) that bounds where subprocess execution is permitted.

## Postconditions

1. **No capability block → deny:** If the plugin's registry entry has no `capabilities.run_subprocess` block → returns `Err(HostError::CapabilityDenied)`; no subprocess is spawned.
2. **Binary not in allowlist → deny:** If the resolved absolute path of `spec.binary` does not match at least one glob pattern in `SubprocessCaps.binary_allowlist` → returns `Err(HostError::CapabilityDenied)`; no subprocess is spawned.
3. **Path traversal in binary → deny:** If `spec.binary` contains `../` sequences (detected before allow-list matching, after path resolution) → returns `Err(HostError::CapabilityDenied)`; no subprocess is spawned.
4. **Arg not in allowlist → deny:** If any element of `spec.args` does not match at least one glob pattern in `SubprocessCaps.arg_allowlist` → returns `Err(HostError::CapabilityDenied)`; no subprocess is spawned.
5. **Env filtering:** Before exec, every env var key in `spec.env` that is NOT listed in `SubprocessCaps.env_allowlist` is stripped. The subprocess inherits only env vars whose keys appear in `env_allowlist`. An empty `env_allowlist` means the subprocess inherits zero env vars.
6. **Successful execution → SubprocessResult:** If all capability checks pass and the subprocess completes within `caps.max_timeout_ms` → returns `Ok(SubprocessResult)` where:
   - `exit_code` equals the actual process exit status (0 = success by convention; negative = signal termination on Unix).
   - `stdout` contains up to `caps.max_stdout_bytes` bytes captured from the subprocess stdout.
   - `stderr` contains up to `caps.max_stderr_bytes` bytes captured from the subprocess stderr.
   - `duration_ms` equals the actual wall-clock milliseconds of subprocess execution.
   - `truncated = false` if neither stdout nor stderr exceeded their respective byte caps; `truncated = true` if either was truncated.
7. **stdout truncation:** If the subprocess writes more than `caps.max_stdout_bytes` bytes to stdout, the dispatcher captures exactly `max_stdout_bytes` bytes and discards the remainder; `SubprocessResult.truncated = true`. The subprocess's actual stdout stream is consumed (not blocked) after the cap is reached.
8. **stderr truncation:** If the subprocess writes more than `caps.max_stderr_bytes` bytes to stderr, the dispatcher captures exactly `max_stderr_bytes` bytes and discards the remainder; `SubprocessResult.truncated = true`.
9. **Timeout:** If the subprocess runs for longer than `caps.max_timeout_ms` wall-clock milliseconds, the dispatcher sends SIGKILL on Unix (or `TerminateProcess` on Windows) and returns `Err(HostError::Timeout)`. Output captured up to the kill point is NOT returned (the error variant carries no partial output).
10. **Binary not found:** If the resolved binary path does not exist or is not executable → returns `Err(HostError::BinaryNotFound)`.
11. **I/O error during exec:** Any other OS-level failure during subprocess spawn or I/O → returns `Err(HostError::IoError(io::Error))`.

## Invariants

### I-1: Capability Gating

1. **MUST declare SubprocessCaps:** A plugin MUST have a `[hooks.<id>.capabilities.run_subprocess]` block in `hooks-registry.toml`; absence of the block MUST produce `HostError::CapabilityDenied` with no subprocess spawn on every call.
2. **MUST reject binary outside allowlist:** The dispatcher MUST reject any `spec.binary` whose resolved absolute path does not match at least one glob pattern in `SubprocessCaps.binary_allowlist`. The check MUST happen before any `execvp`-equivalent call.
3. **MUST reject arg outside allowlist:** The dispatcher MUST reject the entire call if any single element of `spec.args` does not match at least one glob pattern in `SubprocessCaps.arg_allowlist`. Partial arg approval is NOT permitted — all args must be approved or the call is denied.
4. **MUST strip env vars not in allowlist:** The dispatcher MUST remove from the subprocess environment every env var key NOT present in `SubprocessCaps.env_allowlist` before exec. The stripping is unconditional — it applies even if the plugin explicitly sets those vars in `spec.env`.

### I-2: Security Boundaries

5. **MUST NOT shell-interpret:** The dispatcher MUST execute `spec.binary` directly via `Command::new(binary).args(args)` (or platform equivalent). The execution path MUST NOT invoke a shell interpreter (`/bin/sh -c`, `bash -c`, etc.) implicitly. Shell interpreter binaries may only be invoked if explicitly listed in `binary_allowlist`.
6. **MUST NOT allow path traversal:** The dispatcher MUST reject any `spec.binary` whose value contains `../` sequences after canonicalization. The check MUST occur before allow-list pattern matching; path traversal MUST NOT be allowed to "pass through" to the binary allow-list glob match.
7. **MUST use absolute path or preopened relative path:** `spec.binary` MUST be an absolute path or a path resolvable within the dispatcher's WASI preopened directories. PATH-based lookup (searching `$PATH` for the binary name) is NOT performed.
8. **MUST constrain working directory:** The subprocess working directory, if specified via `spec.working_dir`, MUST be CLAUDE_PROJECT_DIR or a descendant. Any working directory outside this tree MUST be rejected with `HostError::CapabilityDenied`.
9. **MUST NOT inherit extra file descriptors:** The subprocess MUST NOT inherit any file descriptors beyond stdin, stdout, and stderr. The dispatcher's `Command` builder MUST NOT pass through open FDs from the dispatcher process.

### I-3: Output Bounds

10. **MUST truncate stdout at cap:** If subprocess stdout exceeds `SubprocessCaps.max_stdout_bytes`, the dispatcher MUST capture exactly `max_stdout_bytes` bytes and set `SubprocessResult.truncated = true`. Excess bytes MUST be consumed and discarded by the dispatcher (not left blocking the subprocess pipe).
11. **MUST truncate stderr at cap:** If subprocess stderr exceeds `SubprocessCaps.max_stderr_bytes`, the dispatcher MUST capture exactly `max_stderr_bytes` bytes and set `SubprocessResult.truncated = true`. Same discard semantics as stdout.
12. **Truncation MUST happen at dispatcher:** The subprocess itself is never aware of the cap — it writes freely; the dispatcher enforces the cap on the read side. The cap does NOT cause the subprocess to receive SIGPIPE or a broken pipe unless it writes to a closed stdout/stderr.

### I-4: Timeout

13. **MUST kill at timeout deadline:** The dispatcher MUST send SIGKILL (Unix) or call `TerminateProcess` (Windows) when `caps.max_timeout_ms` wall-clock milliseconds elapse from subprocess spawn, regardless of subprocess state.
14. **MUST return HostError::Timeout on kill:** After killing the process, the dispatcher MUST return `Err(HostError::Timeout)`. The error MUST NOT carry partial stdout/stderr (no partial SubprocessResult on timeout path).
15. **Duration field accuracy:** On non-timeout completion, `SubprocessResult.duration_ms` MUST reflect the actual wall-clock elapsed time from spawn to process exit, measured by the dispatcher. It MUST NOT exceed `caps.max_timeout_ms`.

### I-5: Schema Evolution

16. **HOST_ABI_VERSION MUST stay at 1:** Adding `host::run_subprocess` is an additive ABI extension per ADR-014 D-9.2, following the precedent of D-6 Option A (`host::write_file`) and D-183 (SubagentStop fields). `crates/hook-sdk/src/lib.rs` and `crates/factory-dispatcher/src/lib.rs` MUST both retain `pub const HOST_ABI_VERSION: u32 = 1;` after the SDK-ext story merges. No bump is permitted in v1.x per D-6 Option B prohibition.
17. **BC-2.02.013 MUST appear in SS-02 Schema Evolution table:** After the SDK-ext story merges, the SS-02 Schema Evolution table MUST contain a row for BC-2.02.013 citing the `run_subprocess` additive extension and confirming `HOST_ABI_VERSION = 1`.

### I-6: Telemetry

18. **MUST emit host.subprocess.exec event:** The dispatcher MUST emit a `host.subprocess.exec` telemetry event on every `host::run_subprocess` call — both successful calls and error returns. The event payload MUST include: `{plugin_id, binary, args_count, exit_code, duration_ms, stdout_bytes, stderr_bytes, truncated, timed_out}`.
19. **MUST route event through normal sink chain:** The `host.subprocess.exec` event MUST be dispatched through the existing telemetry sink chain (file sink / Datadog / Honeycomb per config), identical to how `host.file.read` and `host.file.write` events are routed. No subprocess-specific sink bypass is permitted.

### I-7: Error Semantics

20. **CapabilityDenied is a value, not a panic:** `HostError::CapabilityDenied` MUST be returned as a `Result::Err` value to the guest. The dispatcher MUST NOT panic on capability check failure.
21. **BinaryNotFound is a value, not a panic:** If the binary path does not exist or is not executable, `HostError::BinaryNotFound` MUST be returned as `Result::Err`. No panic.
22. **Timeout is a value, not a panic:** `HostError::Timeout` MUST be returned as `Result::Err` after SIGKILL/TerminateProcess. No panic.
23. **IoError is a value, not a panic:** OS-level failures during spawn, pipe I/O, or wait MUST be returned as `Err(HostError::IoError(io::Error))`. No panic.
24. **No new error codes:** `host::run_subprocess` MUST reuse the existing HostError variant set. No new negative i32 codes are introduced by the SDK-ext story (per Architecture Compliance Rule 4 in S-8.10 and BC-2.02.003).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `spec.binary` is empty string | `Err(HostError::CapabilityDenied)` (fails binary allowlist match before spawn) |
| EC-002 | `spec.binary = "../../etc/passwd"` (path traversal) | `Err(HostError::CapabilityDenied)` — rejected at traversal check before allowlist match |
| EC-003 | Plugin registry has no `run_subprocess` capability block | `Err(HostError::CapabilityDenied)` on every call; no spawn |
| EC-004 | `binary_allowlist = []` (empty allowlist) | `Err(HostError::CapabilityDenied)` — no binary can ever match an empty glob list |
| EC-005 | `arg_allowlist = []` but `spec.args` is non-empty | `Err(HostError::CapabilityDenied)` — first arg cannot match any pattern |
| EC-006 | `spec.args = []` (empty args) with `arg_allowlist = []` | Allowed (no args to check) — proceeds to binary resolution and exec |
| EC-007 | `env_allowlist = []` | Subprocess inherits zero env vars; `spec.env` entries are all stripped |
| EC-008 | `env_allowlist = ["CLAUDE_PROJECT_DIR"]`, `spec.env = {"PATH": "/usr/bin"}` | `PATH` is stripped; subprocess env contains only CLAUDE_PROJECT_DIR if set |
| EC-009 | Subprocess writes exactly `max_stdout_bytes` bytes | `truncated = false`; full output captured |
| EC-010 | Subprocess writes `max_stdout_bytes + 1` bytes to stdout | `truncated = true`; exactly `max_stdout_bytes` bytes in `SubprocessResult.stdout` |
| EC-011 | Subprocess writes 0 bytes to both stdout and stderr | `stdout = []`, `stderr = []`, `truncated = false`; exit code captured normally |
| EC-012 | Subprocess exits with code 1 (non-zero) | `Ok(SubprocessResult { exit_code: 1, ... })`; non-zero exit is NOT an error at the ABI level |
| EC-013 | Subprocess terminates via signal (e.g. SIGSEGV) on Unix | `Ok(SubprocessResult { exit_code: <negative sentinel or OS-mapped value>, ... })` |
| EC-014 | Subprocess exceeds `max_timeout_ms` by 1 ms | `Err(HostError::Timeout)`; SIGKILL sent; no partial output returned |
| EC-015 | Binary path is a relative path (not absolute) | Resolved against dispatcher's WASI preopened dirs; must match allowlist after resolution |
| EC-016 | `spec.working_dir` points outside CLAUDE_PROJECT_DIR | `Err(HostError::CapabilityDenied)` — working directory constraint violated |
| EC-017 | Binary listed in allowlist does not exist on disk | `Err(HostError::BinaryNotFound)` |
| EC-018 | Plugin compiled against SDK 0.1.x (no `run_subprocess` import) loaded against dispatcher that exports `run_subprocess` | Plugin loads normally; wasmtime ignores unimported host exports; no error — ABI-safe per additive extension policy (D-6 Option A) |
| EC-019 | `spec.timeout_ms` exceeds `caps.max_timeout_ms` | Dispatcher uses `caps.max_timeout_ms` as the hard cap; `spec.timeout_ms` is clamped |
| EC-020 | Shell interpreter binary (e.g. `/bin/bash`) in `binary_allowlist` | Allowed; must be explicitly declared with justification in the registry. The dispatcher still does NOT use shell expansion — `/bin/bash` is exec'd directly with `spec.args` as argv. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `run_subprocess(SubprocessSpec { binary: "/usr/bin/git", args: ["--version"], env: {}, timeout_ms: 5000, working_dir: None })` where `binary_allowlist = ["/usr/bin/git"]`, `arg_allowlist = ["--version"]` | `Ok(SubprocessResult { exit_code: 0, stdout: b"git version ...", stderr: b"", truncated: false, duration_ms: <≤5000> })` | happy-path |
| Same call but `binary_allowlist = ["/usr/bin/grep"]` (git not listed) | `Err(HostError::CapabilityDenied)` | error |
| `args = ["--porcelain"]` but `arg_allowlist = ["--version"]` | `Err(HostError::CapabilityDenied)` | error |
| `binary = "../../etc/passwd"` | `Err(HostError::CapabilityDenied)` | security |
| `run_subprocess(...)` where subprocess writes `max_stdout_bytes + 1024` bytes | `Ok(SubprocessResult { truncated: true, stdout.len() == max_stdout_bytes })` | edge-case |
| `run_subprocess(...)` where subprocess sleeps longer than `caps.max_timeout_ms` | `Err(HostError::Timeout)` | edge-case |
| No `run_subprocess` capability block in plugin registry | `Err(HostError::CapabilityDenied)` | error |
| `binary_allowlist = []` (empty) | `Err(HostError::CapabilityDenied)` | edge-case |
| Plugin compiled without `run_subprocess` import loaded against dispatcher with `run_subprocess` export | Plugin loads normally; `HOST_ABI_VERSION` check passes | ABI-compat |
| `run_subprocess(...)` where subprocess exits with code 42 | `Ok(SubprocessResult { exit_code: 42, ... })` | happy-path |
| `env = {"PATH": "/usr/bin", "CLAUDE_PROJECT_DIR": "/proj"}` with `env_allowlist = ["CLAUDE_PROJECT_DIR"]` | Subprocess environment contains only `CLAUDE_PROJECT_DIR`; `PATH` is stripped | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — assigned in Phase 1.6b) | `binary_allowlist_check` is a pure deterministic function testable in isolation | Unit test (pure-core) |
| (TBD) | Arg allowlist check rejects on first non-matching arg, not after all args are checked | Unit test with assertion that rejection occurs on arg[0] mismatch |
| (TBD) | Env stripping removes all keys not in `env_allowlist` regardless of what `spec.env` sets | Unit test: set 5 vars, allowlist 2, verify subprocess env has exactly 2 |
| (TBD) | `HOST_ABI_VERSION = 1` in both crates after SDK-ext story merge | grep assertion (AC mirroring S-8.10 AC-3) |
| (TBD) | `host.subprocess.exec` telemetry event is emitted on every call including errors | Integration test: assert event sink receives event on CapabilityDenied, Timeout, and Ok paths |
| (TBD) | Stdout truncation does not block subprocess pipe | Integration test: subprocess writes 2× max_stdout_bytes; verify it completes, not hangs |

## Related BCs

- **BC-2.02.002** (composes with) — mandates bounded host calls; `run_subprocess` extends the bounded-call family with subprocess execution
- **BC-2.01.003** (depends on) — HOST_ABI_VERSION = 1 invariant; this BC MUST NOT violate it (Invariant I-5.16)
- **BC-2.02.003** (depends on) — HostError code mapping; `run_subprocess` reuses the existing error code set without adding new codes (Invariant I-7.24)
- **BC-2.02.011** (sibling) — `host::write_file` WriteFileCaps pattern; `SubprocessCaps` mirrors the same deny-by-default allowlist structure

## Architecture Anchors

- `crates/hook-sdk/src/host/run_subprocess.rs` (new `pub fn run_subprocess` SDK wrapper; `vsdd::run_subprocess(spec_json)` extern shim)
- `crates/hook-sdk/src/ffi.rs` (new `run_subprocess` extern in `#[cfg(target_arch = "wasm32")]` block + `host_stubs`)
- `crates/factory-dispatcher/src/host/run_subprocess.rs` (new file: dispatcher binding, capability enforcement, exec logic, env stripping, output capping, timeout enforcement)
- `crates/factory-dispatcher/src/host/mod.rs` (registration call in `setup_linker`)
- `crates/factory-dispatcher/src/registry.rs` (`SubprocessCaps` struct, `run_subprocess: Option<SubprocessCaps>` field)
- `crates/hook-sdk/HOST_ABI.md` (ABI catalog documentation — `run_subprocess` entry)
- `.factory/specs/architecture/SS-02-hook-sdk.md` (Schema Evolution table row for BC-2.02.013)

## Story Anchor

SDK-ext (W-16) — "SDK extension: host::run_subprocess (ADR-014 D-9.2)" — implements this BC. Analogous to S-8.10 (`host::write_file`). Story must be sequenced before S-9.01..S-9.07. Story ID TBD (story-writer authors E-9 + S-9.x stories with BC-2.02.013 anchors).

## VP Anchors

(TBD — to be assigned in Phase 1.6b verification properties pass)

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-022 |
| Capability Anchor Justification | CAP-022 ("Port hook plugins from bash to native WASM") per capabilities.md §CAP-022. `host::run_subprocess` is a direct enabling dependency for porting the Tier 2 validate hooks (W-16) to native WASM; `validate-wave-gate-prerequisite` invokes `verify-sha-currency.sh` as a critical security check that MUST be preserved in the WASM port, and this host function is the only correct path to do so within the bounded capability model. |
| L2 Domain Invariants | TBD (Phase 1.5 invariant lift pass) |
| Architecture Module | SS-02 — `crates/hook-sdk/src/host/run_subprocess.rs`, `crates/factory-dispatcher/src/host/run_subprocess.rs` |
| ADR Source | ADR-014 D-9.2 — host::run_subprocess ABI decision |
| Stories | SDK-ext (W-16) (implementing story, TBD ID); S-9.07 (primary consumer: validate-wave-gate-prerequisite requires run_subprocess) |
| Risk Items | R-W16-001 (bats orphan deferred to Phase H), R-W16-003 (bundle size ceiling) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `.factory/specs/architecture/decisions/ADR-014-tier-2-native-wasm-migration.md` §D-9.2 (SubprocessCaps schema, security boundaries); `.factory/specs/architecture/SS-02-hook-sdk.md` §host::run_subprocess (function signature, SubprocessSpec, SubprocessResult, SubprocessCaps schema) |
| **Confidence** | HIGH — architect authored all schema details and security boundaries in SS-02; ADR-014 documents the user-approved override rationale |
| **Extraction Date** | 2026-05-03 |
| **Extracted from** | ADR-014 D-9.2 + SS-02-hook-sdk.md §host::run_subprocess |

#### Evidence Types Used

- type constraint (SubprocessCaps, SubprocessSpec, SubprocessResult signatures from SS-02)
- documentation (ADR-014 security boundaries and rationale)
- sibling implementation (BC-2.02.011 `host::write_file` — structural mirror for capability gating pattern)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | Yes (subprocess spawn, pipe I/O, process wait, FFI call to wasmtime host) |
| **Global state access** | No (all state passed via ctx parameter and SubprocessSpec) |
| **Deterministic** | No (subprocess execution is inherently environment-dependent) |
| **Thread safety** | TBD (Phase 1.6b will refine; wasmtime linker context threading) |
| **Overall classification** | effectful-shell (dispatcher binding + exec path); pure-core subsets: `binary_allowlist_check`, `arg_allowlist_check`, `env_strip` |

#### Refactoring Notes

`binary_allowlist_check`, `arg_allowlist_check`, and `env_strip` (pure-core subsets) SHOULD be extracted as standalone functions for unit testing without dispatcher context, mirroring the `path_allowed` pattern from `read_file.rs` lines 73-77. The remaining spawn/wait/pipe logic is inherently effectful-shell.
