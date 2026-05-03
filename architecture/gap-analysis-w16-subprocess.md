---
document_type: architecture-gap-analysis
level: L4
version: "1.0"
status: draft
producer: architect
timestamp: 2026-05-03T00:00:00Z
phase: "Phase D — W-16 spec foundation"
traces_to: .factory/architecture/audit-w16.md
subject: host::exec_subprocess (production) vs host::run_subprocess (proposed BC-2.02.013)
---

## Correction 2026-05-03

This gap analysis incorrectly identified BC-2.02.005 as the exec_subprocess BC.
BC-2.02.005 actually documents the SDK `read_string` re-call protocol; the
exec_subprocess behavioral contracts live in SS-01 cluster (BC-1.05.001..034).
All references to BC-2.02.005 in this document should be read as
"BC-1.05.001..034 (existing) + BC-1.05.035 + BC-1.05.036 (new 2026-05-03)".

---

# Gap Analysis: host::exec_subprocess vs host::run_subprocess (W-16)

> Scope: the W-16 use case is ONE plugin — `validate-wave-gate-prerequisite` invoking
> `verify-sha-currency.sh`. This document answers whether building `host::run_subprocess`
> (BC-2.02.013, S-9.30, 224-line spec, 24 MUST invariants) is justified, or whether
> the existing `host::exec_subprocess` is sufficient with minor extension.

---

## Section 1 — Existing host::exec_subprocess Capabilities

Sources read: `crates/hook-sdk/HOST_ABI.md` lines 280–312, `crates/hook-sdk/src/host.rs`
lines 280–344, `crates/hook-sdk/src/ffi.rs` lines 34–45 + 103–116,
`crates/factory-dispatcher/src/host/exec_subprocess.rs` (full),
`crates/factory-dispatcher/src/registry.rs` lines 69–88.

| Feature | exec_subprocess current | Source citation |
|---------|------------------------|-----------------|
| Binary path arg | YES — `cmd: &str` in SDK (host.rs:300); mapped to `cmd_ptr/cmd_len` in FFI (ffi.rs:34–45) | host.rs:300, ffi.rs:34 |
| Args (multiple) | YES — `args: &[&str]` length-prefixed encoding (host.rs:307, `encode_args` host.rs:336–343) | host.rs:299, 336 |
| STDIN bytes | YES — `stdin: &[u8]`; `Stdio::null()` when empty; `Stdio::piped()` + write_all when non-empty (exec_subprocess.rs:237–265) | exec_subprocess.rs:237–265 |
| Timeout | YES — `timeout_ms: u32`, wall-clock poll loop (exec_subprocess.rs:270–301); no documented default or max cap | exec_subprocess.rs:270–300 |
| Single output cap (`max_output_bytes`) | YES — single cap covers both stdout and stderr combined (exec_subprocess.rs:278–283); caller passes `max_output_bytes: u32` | exec_subprocess.rs:278–283 |
| Separate stdout/stderr caps | NO — one `max_output_bytes` value governs both streams; no per-stream cap | exec_subprocess.rs:278–283 |
| Truncated flag in result | NO — truncation returns `Err(codes::OUTPUT_TOO_LARGE)` (-3) rather than `Ok(result_with_flag)` (exec_subprocess.rs:283); SDK maps to `HostError::Other(-3)` | exec_subprocess.rs:282–284; HOST_ABI.md:309 |
| Capability check (binary_allow) | YES — exact basename or full-path match against `caps.binary_allow: Vec<String>` (exec_subprocess.rs:152–157, 186–192); NO glob support | exec_subprocess.rs:186–192 |
| Capability check (arg_allow) | NO — no per-argument allowlist in `ExecSubprocessCaps`; args pass through unchecked | registry.rs:71–88 |
| Capability check (env_allow / env_strip) | YES — `caps.env_allow: Vec<String>` (registry.rs:87); `env_clear()` then forward only listed names (exec_subprocess.rs:242–247) | registry.rs:87, exec_subprocess.rs:242–247 |
| Capability check (cwd_allow) | PARTIAL — `caps.cwd_allow: Vec<String>` declared in registry (registry.rs:83), but exec path uses `ctx.cwd` (dispatcher's own cwd) directly; `cwd_allow` is stored but not enforced at exec time | registry.rs:83, exec_subprocess.rs:248–250 |
| shell_bypass_acknowledged flag | YES — if `cmd` basename is in `SHELL_NAMES` (bash/sh/zsh/pwsh/fish/csh/tcsh/ksh) AND `caps.shell_bypass_acknowledged.is_none()` → `CAPABILITY_DENIED` (exec_subprocess.rs:159–164, 31) | exec_subprocess.rs:31, 159–164 |
| FD inheritance prevention | PARTIAL — `Command::new` with `stdout(Stdio::piped())` and `stderr(Stdio::piped())` prevent those FDs leaking; no explicit `close_range`/`O_CLOEXEC` call; dispatcher's other open FDs are not explicitly closed | exec_subprocess.rs:231–233 |
| Path canonicalization (anti-traversal) | NO — `cmd` is passed directly to `Command::new`; no `../` check before allow-list or before exec (exec_subprocess.rs:230); S-1.5 note mentions "pre-resolved full paths in binary_allow" but the enforcement is doc-future | exec_subprocess.rs:15–16, 230 |
| Error variants (typed) | `-1` CAPABILITY_DENIED, `-2` TIMEOUT, `-3` OUTPUT_TOO_LARGE, `-4` INVALID_ARGUMENT, `INTERNAL_ERROR` (not in HOST_ABI docs, from `codes::INTERNAL_ERROR`) | HOST_ABI.md:304–310; exec_subprocess.rs:252 |
| Telemetry event on denial | YES — `emit_denial` emits `internal.capability_denied` for all 4 denial paths (exec_subprocess.rs:304–309) | exec_subprocess.rs:148, 155, 162, 169 |
| Telemetry event on success | NO — no success-path telemetry event is emitted; only denials are logged | exec_subprocess.rs:285–288 (no emit call) |
| Cross-platform parity | PARTIAL — `#[cfg(unix)]` / `#[cfg(not(unix))]` for setuid check (exec_subprocess.rs:202–217); `SIGKILL` kill path not abstracted (exec_subprocess.rs:294); `Command` is std and cross-platform otherwise | exec_subprocess.rs:202–217 |

### Derived `ExecSubprocessCaps` shape (registry.rs:69–88)

```rust
pub struct ExecSubprocessCaps {
    pub binary_allow: Vec<String>,          // exact basename or full-path match
    pub shell_bypass_acknowledged: Option<String>,
    pub cwd_allow: Vec<String>,             // declared but not enforced at exec
    pub env_allow: Vec<String>,             // enforced via env_clear + selective forward
}
```

---

## Section 2 — BC-2.02.013 / Proposed run_subprocess Capabilities

Sources: `.factory/specs/behavioral-contracts/ss-02/BC-2.02.013-host-run-subprocess.md`
(full), `.factory/specs/architecture/SS-02-hook-sdk.md` lines 328–442.

| Feature | run_subprocess proposed | Source citation |
|---------|------------------------|-----------------|
| Binary path arg | YES — `SubprocessSpec.binary: String` (SS-02:348) | SS-02:348 |
| Args (multiple) | YES — `SubprocessSpec.args: Vec<String>` (SS-02:351) | SS-02:351 |
| STDIN bytes | NO — `SubprocessSpec` has no `stdin` field; no stdin bytes can be passed | SS-02:346–360 |
| Timeout | YES — `SubprocessSpec.timeout_ms: u64` (caller-supplied); capped at `caps.max_timeout_ms` (SS-02:413) | SS-02:355, 413 |
| Single output cap | NO — separate caps only (no combined `max_output_bytes`) | SS-02:402–408 |
| Separate stdout/stderr caps | YES — `caps.max_stdout_bytes: u64` (default 1 MiB) and `caps.max_stderr_bytes: u64` (default 256 KiB) (SS-02:402–408) | SS-02:402–408 |
| Truncated flag in result | YES — `SubprocessResult.truncated: bool`; `Ok(SubprocessResult { truncated: true })` on cap exceeded; result is returned (BC-2.02.013:7,8) | BC-2.02.013:54–55 |
| Capability check (binary_allow — glob) | YES — `caps.binary_allowlist: Vec<String>` with glob patterns (SS-02:389–394); e.g. `"*/hooks/verify-sha-currency.sh"` | SS-02:385–394 |
| Capability check (arg_allow — glob) | YES — `caps.arg_allowlist: Vec<String>` with glob patterns; each arg must match at least one pattern; all-or-nothing (BC-2.02.013:I-1.3) | BC-2.02.013:67, SS-02:395–396 |
| Capability check (env_allow / env_strip) | YES — `caps.env_allowlist: Vec<String>`; strip-by-default before exec (BC-2.02.013:I-1.4, SS-02:400) | BC-2.02.013:68, SS-02:398–401 |
| Capability check (cwd_allow) | YES — `SubprocessSpec.working_dir` must be CLAUDE_PROJECT_DIR or descendant; violation → `CapabilityDenied` (BC-2.02.013:I-2.8) | BC-2.02.013:75 |
| shell_bypass_acknowledged flag | NO — no separate flag; shell binaries must appear in `binary_allowlist` with explicit justification (BC-2.02.013:EC-020) | BC-2.02.013:131 |
| FD inheritance prevention | YES — MUST NOT inherit extra FDs beyond stdin/stdout/stderr (BC-2.02.013:I-2.9) | BC-2.02.013:77 |
| Path canonicalization (anti-traversal) | YES — MUST reject `../` sequences after canonicalization, before allow-list match (BC-2.02.013:I-2.6) | BC-2.02.013:73 |
| Error variants (typed) | `CapabilityDenied`, `Timeout`, `BinaryNotFound`, `IoError(io::Error)` — typed Rust variants (BC-2.02.013:I-7.20–24); NO new i32 codes (reuses existing set) | BC-2.02.013:101–106 |
| Telemetry event on denial | YES (implied by I-6.18) | BC-2.02.013:97 |
| Telemetry event on success | YES — `host.subprocess.exec` MUST be emitted on every call including errors; fields: `{plugin_id, binary, args_count, exit_code, duration_ms, stdout_bytes, stderr_bytes, truncated, timed_out}` (BC-2.02.013:I-6.18) | BC-2.02.013:97 |
| Cross-platform parity | YES — explicit SIGKILL (Unix) / TerminateProcess (Windows) (BC-2.02.013:I-4.13); cross-platform kill semantics specified | BC-2.02.013:86 |
| `duration_ms` in result | YES — `SubprocessResult.duration_ms: u64` (SS-02:374) | SS-02:372–374 |
| `working_dir` per-call override | YES — `SubprocessSpec.working_dir: Option<PathBuf>` (SS-02:358) | SS-02:358 |

### `SubprocessCaps` shape (SS-02:384–414)

```rust
pub struct SubprocessCaps {
    pub binary_allowlist: Vec<String>,   // glob patterns
    pub arg_allowlist: Vec<String>,      // glob patterns — PER-ARGUMENT check
    pub env_allowlist: Vec<String>,      // exact name match — strip-by-default
    pub max_stdout_bytes: u64,           // default 1 MiB
    pub max_stderr_bytes: u64,           // default 256 KiB
    pub max_timeout_ms: u64,             // default 30_000 ms; hard cap
}
```

---

## Section 3 — Per-Feature Delta

| Feature | Status | exec_subprocess | run_subprocess |
|---------|--------|-----------------|----------------|
| Binary path arg | Same | `cmd: &str` | `spec.binary: String` |
| Args (multiple) | Same | `args: &[&str]` | `spec.args: Vec<String>` |
| STDIN bytes | **Better in exec_subprocess** | YES — `stdin: &[u8]`; wired and tested | NO — not in SubprocessSpec |
| Timeout | Different | `timeout_ms: u32` mandatory, no cap | `spec.timeout_ms` clamped to `caps.max_timeout_ms`; `u64` |
| Single shared output cap | Different | `max_output_bytes: u32` covers both stdout + stderr | Not present; separate per-stream caps only |
| Separate stdout/stderr caps | **Better in run_subprocess** | NO | YES — `max_stdout_bytes` + `max_stderr_bytes` |
| Truncated flag in result | **Better in run_subprocess** | NO — returns Err(-3) on truncation | YES — `SubprocessResult.truncated: bool`; Ok path with flag |
| Binary allow-list (glob) | **Better in run_subprocess** | Exact match only (basename or full path) | Glob patterns (`*/hooks/verify-sha-currency.sh`) |
| Arg allow-list | **Better in run_subprocess** | NOT PRESENT | YES — per-argument glob check |
| Env strip-by-default | Same | YES — `env_clear()` + forward listed names | YES — same semantics |
| CWD enforcement | **Better in run_subprocess** | Declared but not enforced at exec | Enforced: must be CLAUDE_PROJECT_DIR or descendant |
| shell_bypass flag | Different | Separate `shell_bypass_acknowledged: Option<String>` | No flag; shell binaries explicitly in binary_allowlist |
| FD inheritance prevention | **Better in run_subprocess** | Partial (stdout/stderr piped; no explicit FD close) | Explicit MUST in spec (I-2.9) |
| Path traversal check | **Better in run_subprocess** | NOT PRESENT (S-1.5 future) | Explicit MUST before allow-list (I-2.6) |
| Error variants | Different | Numeric i32 codes (-1 to -4) | Typed Rust enums (CapabilityDenied, Timeout, BinaryNotFound, IoError) |
| Telemetry on success | **Better in run_subprocess** | NO | YES — `host.subprocess.exec` every call |
| Telemetry on denial | Same | YES — `internal.capability_denied` | YES — implied |
| Cross-platform kill | **Better in run_subprocess** | Not abstracted | Explicit SIGKILL/TerminateProcess spec |
| `duration_ms` in result | **Better in run_subprocess** | NOT PRESENT | YES — `SubprocessResult.duration_ms` |
| Per-call `working_dir` | **Better in run_subprocess** | NOT PRESENT (uses dispatcher cwd) | YES — `SubprocessSpec.working_dir` |
| Missing in both | — | — | No async execution model; synchronous only |

---

## Section 4 — What Does validate-wave-gate-prerequisite Actually Need?

Source: `plugins/vsdd-factory/hooks/validate-wave-gate-prerequisite.sh` lines 47–99,
`plugins/vsdd-factory/templates/verify-sha-currency.sh` (full).

### Exact subprocess invocation pattern (lines 71–74)

```bash
if HOOK_OUTPUT=$(bash "$SHA_HOOK" --project-root "$SHA_PROJECT_ROOT" 2>&1); then
  exit 0
fi
```

| Aspect | Value | Used in feature? |
|--------|-------|-----------------|
| Binary invoked | `bash` — the shell interpreter | YES — binary path arg |
| Arguments | `["$SHA_HOOK", "--project-root", "$SHA_PROJECT_ROOT"]` | YES — multiple args |
| STDIN | None — the subprocess does not read stdin (verify-sha-currency.sh uses `$@` args) | NO — stdin not needed |
| Env vars needed | `PATH` (for git, grep, python3 in the script); `HOME` may be needed for git config | YES — env_allow: PATH (at minimum) |
| Output captured | Merged stdout+stderr (`2>&1`), used only in error message output; the important signal is exit code 0 vs non-zero | YES — stdout needed; stderr merging is a bash-level detail |
| Output size | verify-sha-currency.sh prints diagnostic lines (~30–80 lines); easily under 64KB | YES — output cap needed, very small |
| Timeout | Not specified in bash hook; but the script is pure git+grep+python3, ~200ms expected. 30s cap is fine | YES — timeout needed |
| Error handling | If exit code != 0: emit event + print diagnostic block to stderr + exit 2. If script missing or bash missing: `exit 0` (graceful degrade) | YES — exit code in result |

### Feature-by-feature use map

| Feature | Does validate-wave-gate-prerequisite use it? |
|---------|---------------------------------------------|
| Binary path arg | YES — must invoke `bash` (shell interpreter) |
| Args (multiple) | YES — `["$SHA_HOOK", "--project-root", "$SHA_PROJECT_ROOT"]` (3 args) |
| STDIN bytes | NO — verify-sha-currency.sh reads from args and git, not stdin |
| Timeout | YES — needs a hard cap to prevent stalled git calls from blocking dispatcher |
| Single output cap | YES — both stdout+stderr need capping; combined is fine since they're merged via `2>&1` in bash |
| Separate stdout/stderr caps | NO — the bash `2>&1` merge means stderr goes to stdout; no separate stderr capture needed |
| Truncated flag | NICE-TO-HAVE — but since output is small, truncation is not a real risk |
| Binary allow-list (exact match) | YES — `bash` must be allowed; exact match on `bash` works |
| Binary allow-list (glob) | NO — exact match on `bash` is sufficient |
| Arg allow-list | NO — the hook does not need to constrain which args bash accepts; the plugin controls the args |
| Env allow/strip | YES — at minimum `PATH` must be forwarded for git, python3, grep to be found |
| CWD enforcement | NO — the script uses `$SHA_PROJECT_ROOT` flag, not cwd; cwd doesn't need to change |
| shell_bypass_acknowledged | YES — critical, because the binary IS `bash` |
| FD inheritance prevention | YES — good hygiene but not use-case-blocking |
| Path traversal check | NICE-TO-HAVE — low risk since binary is hardcoded to `bash` |
| Typed error variants | YES — distinguishes "capability denied" vs "timeout" vs "exec failed" |
| Success telemetry | NICE-TO-HAVE — useful but not required for correctness |
| duration_ms in result | NO |
| Per-call working_dir | NO |

---

## Section 5 — Gap Classification

### Gaps where exec_subprocess is sufficient

These are features the use case needs that exec_subprocess already provides:

1. **Binary path arg** — `cmd: "bash"` maps directly.
2. **Multiple args** — `args: &[&str]` handles `["$SHA_HOOK", "--project-root", "$SHA_PROJECT_ROOT"]`.
3. **Env allow/strip** — `env_clear()` + `caps.env_allow` already forwards `PATH` and any listed env; satisfies the need.
4. **shell_bypass_acknowledged** — The gate for invoking `bash` is already present (exec_subprocess.rs:159–164). The plugin declares the field and the check passes. This is the exact mechanism needed.
5. **Timeout enforcement** — `timeout_ms: u32` wall-clock enforcement via poll loop (exec_subprocess.rs:270–296) is sufficient for a 200ms script with a 5–30s cap.
6. **Exit code in result** — `SubprocessResult.exit_code: i32` already returned (host.rs:282–285); non-zero exit maps to block logic.
7. **stdout + stderr captured** — Both streams are captured and returned in the result envelope (exec_subprocess.rs:274–277, `encode_envelope` at 101–109).
8. **Capability denial telemetry** — `emit_denial` already fires on all 4 denial paths (exec_subprocess.rs:148, 155, 162, 169).

### Gaps where exec_subprocess needs minor extension

Features the use case would benefit from that exec_subprocess could add additively without ABI breaking changes (HOST_ABI_VERSION stays at 1, additive new cap fields):

1. **Path traversal check on binary arg** — Currently absent (exec_subprocess.rs:230, no `../` check). For `validate-wave-gate-prerequisite`, the binary is always `bash`, so the risk is near-zero in practice. But a 5-line guard adding `if cmd.contains("../") { return CAPABILITY_DENIED; }` before `binary_allowed()` would close the gap. **Effort: trivial (< 30 min).**

2. **Success-path telemetry event** — Currently only denial events are emitted (exec_subprocess.rs:285–288, no emit call on success). Adding a `host.exec_subprocess.completed` event with `{binary, exit_code, stdout_bytes, stderr_bytes, duration_ms}` would give observability parity. **Effort: small (1–2 hours); analogous to existing `deny` path).**

3. **`duration_ms` in SubprocessResult** — Currently absent from `SubprocessResult` (host.rs:282–285). Adding it requires a `Instant::now()` capture at spawn + diff at process exit; the wall-clock loop already has `Instant` (exec_subprocess.rs:270). **Effort: trivial (< 1 hour).**

None of these require changing the ABI signature. They extend the `ExecSubprocessCaps` struct or `SubprocessResult` struct additively (new optional fields), consistent with the D-6 Option A precedent already used for `write_file`.

### Gaps where exec_subprocess is fundamentally insufficient

Features that would require ABI-breaking changes or a separate host function:

1. **Truncated flag as Ok value (not Err)** — exec_subprocess returns `Err(codes::OUTPUT_TOO_LARGE)` (-3) when output exceeds cap (exec_subprocess.rs:282–283). run_subprocess returns `Ok(SubprocessResult { truncated: true })`. **For the validate-wave-gate-prerequisite use case this is IRRELEVANT** — verify-sha-currency.sh output is ~30–80 lines, never near any reasonable cap. This is a quality-of-life improvement for future use cases with large output, not a requirement for W-16.

2. **Per-argument glob allow-list** — exec_subprocess has no `arg_allowlist` in `ExecSubprocessCaps` (registry.rs:71–88). Adding it would be an additive registry struct field (minor extension), but the W-16 use case **does not need it** — the plugin controls its own args; constraining them at the cap layer adds no security benefit when the plugin is already trusted wasm code.

3. **STDIN not present in run_subprocess** — Paradoxically, exec_subprocess has a capability run_subprocess lacks: STDIN. This matters for the legacy-bash-adapter (which pipes the Claude Code hook envelope as stdin to bash hooks) and possibly future use cases. This is a regression, not a gap.

**Summary: there are zero fundamentally insufficient features for the W-16 use case.** exec_subprocess already handles binary arg, multiple args, env filtering, shell_bypass gate, timeout, and exit code — which is the complete set the use case requires.

---

## Section 6 — Recommendation Matrix

| Option | Implementation Cost | Maintenance Burden | Security Surface |
|--------|--------------------|--------------------|-----------------|
| **(a) Use existing exec_subprocess** | LOW — zero new code; plugin uses existing host fn directly; optionally add trivial minor extensions | LOW — one subprocess implementation; no fork; BC-4.04.002 (session-start-telemetry) already tests the path | LOW — existing audited path; shell_bypass gate already in place; exec_subprocess already used in production |
| **(b) Build new run_subprocess + deprecate exec_subprocess** | HIGH — 224-line BC spec, new ffi.rs extern, new host binding file, new registry struct, new SDK wrapper; migration of session-start-telemetry BC-4.04.002 off exec_subprocess | HIGH — two implementations during transition; deprecation timeline; BC-4.04.002 migration story needed; STDIN regression must be resolved before session-start-telemetry can migrate | MED — new code path introduces new attack surface; arg_allowlist adds defense-in-depth but exec_subprocess lacks it today with no incident |
| **(c) Coexist (both functions)** | MED — S-9.30 builds run_subprocess; exec_subprocess stays; no migration cost | HIGH — two parallel subprocess host functions; two sets of tests; two capability schemas; confusion about which to use for new plugins | MED — duplicated surface; more code to audit; spec drift risk between the two paths over time |

---

## Section 7 — Final Recommendation

**Recommendation: (d) Hybrid — extend exec_subprocess additively; drop S-9.30 and BC-2.02.013 from W-16 scope.**

### Defense

The Section 4 analysis identifies that `validate-wave-gate-prerequisite` needs exactly: binary path arg, multiple args, shell_bypass_acknowledged gate, env forwarding (PATH), timeout, and exit code capture. Every single one of these is already present and working in `host::exec_subprocess` (production-verified via `session-start-telemetry` BC-4.04.002).

The Section 5 "fundamentally insufficient" list is **empty** for the W-16 use case. The Section 5 "minor extension" list contains three improvements (path traversal check, success telemetry, duration_ms) — all trivial, none required for S-9.07 correctness.

The two headline features of `run_subprocess` that are genuinely better (glob binary_allowlist, arg_allowlist, truncated-as-Ok, per-stream caps) are **not required by validate-wave-gate-prerequisite**:
- Glob binary_allowlist: `bash` is an exact match; glob adds no value.
- arg_allowlist: the plugin author controls the args; capping them at the capability layer adds no security for trusted wasm.
- Truncated-as-Ok: verify-sha-currency.sh output is ~1–5 KB; no realistic truncation risk.
- Per-stream caps: the bash `2>&1` merges stderr into stdout anyway.

`run_subprocess` lacks STDIN (`SubprocessSpec` has no stdin field), which means it cannot replace exec_subprocess for `session-start-telemetry` (BC-4.04.002 pipes the JSON envelope). Coexistence (option c) carries a permanent dual-maintenance burden. Building run_subprocess to unlock one plugin (S-9.07) that doesn't need any of its unique features is speculative scope.

The hybrid is: extend exec_subprocess with the three minor additions (path traversal guard, success telemetry, optional duration_ms) and ship S-9.07 using exec_subprocess directly. This is additive and ABI-stable (D-6 Option A precedent).

### Spec Changes if Recommendation Adopted

1. **BC-2.02.013** (`BC-2.02.013-host-run-subprocess.md`) — Change `lifecycle_status: active` → `lifecycle_status: withdrawn`; add `removal_reason: "W-16 gap analysis confirms exec_subprocess sufficient for all W-16 use cases; build cost not justified; STDIN regression."` Do NOT delete — preserve as audit trail.

2. **SS-02-hook-sdk.md** — Remove or mark WITHDRAWN the `host::run_subprocess` section (lines 328–442). Remove the BC-2.02.013 row from the Schema Evolution table. Add a note: "D-9.2 withdrawn per gap-analysis-w16-subprocess.md; W-16 plugin S-9.07 uses exec_subprocess."

3. **ADR-014** — Add a Decision Update entry for D-9.2: "Withdrawn. Gap analysis (gap-analysis-w16-subprocess.md) found exec_subprocess sufficient for W-16 validate-wave-gate-prerequisite. exec_subprocess extended additively instead."

4. **S-9.07** (when authored) — Remove `depends_on: S-9.30` (or equivalent); add `depends_on: S-9.00` only. Note that subprocess capability is fulfilled by exec_subprocess with `shell_bypass_acknowledged`.

5. **hooks-registry.toml** for `validate-wave-gate-prerequisite` plugin — Capability block will use `[hooks.validate-wave-gate-prerequisite.capabilities.exec_subprocess]` with `binary_allow = ["bash"]`, `shell_bypass_acknowledged = "needed to invoke verify-sha-currency.sh"`, `env_allow = ["PATH"]`.

### Migration Plan

No migration required. `session-start-telemetry` continues using exec_subprocess as-is (BC-4.04.002 is satisfied). New S-9.07 plugin uses exec_subprocess from day one. No deprecation timeline.

Three minor additive extensions to exec_subprocess (path traversal guard, success telemetry, duration_ms) can be added as a small task within S-9.07 or as a pre-work subtask — each is under 2 hours. They do not require a new story; they fit within S-9.07's scope as "prerequisite: ensure exec_subprocess has path traversal guard."

### Phase D Scope Adjustment

**S-9.30 drops from W-16 scope.** The story has `status: draft` and `estimated_days: 2`. Dropping it saves approximately 2 engineering-days and removes 224 lines of spec (BC-2.02.013), a new ffi.rs extern, a new dispatcher binding file, a new registry struct (`SubprocessCaps`), and a new SDK wrapper. S-9.07 remains in scope but is unblocked (removes `depends_on: S-9.30`).

W-16 story count: 9 stories → 8 stories (S-9.00 + S-9.01..S-9.07; S-9.30 dropped).

STORY-INDEX.md and E-9 epic need updating to reflect S-9.30 removal. The `blocks: ["S-9.07"]` dependency in S-9.30's frontmatter is the only blocking link; removing it frees S-9.07 to depend only on S-9.00.

### Bundle Size Impact

Dropping S-9.30 eliminates the `crates/hook-sdk/src/host/run_subprocess.rs` and `crates/factory-dispatcher/src/host/run_subprocess.rs` additions. These are new files — no WASM symbol additions to validate-wave-gate-prerequisite.wasm from the SDK side (the plugin would import `vsdd::run_subprocess`; without that import, no code is linked in). Dropping S-9.30 therefore saves bundle size in the plugin by eliminating the `run_subprocess` import shim in the SDK. The SubprocessCaps struct addition to registry.rs is also eliminated, saving dispatcher binary size (minor, < 1 KB). Net impact: positive (smaller bundles); exact delta requires S-9.00 baseline measurement but is not a concern.

---

## Summary: Top 3 Gaps from Section 5

1. **Path traversal guard on binary arg** (minor extension, ~30 min): exec_subprocess passes `cmd` directly to `Command::new` with no `../` check. Trivial to add; closes a defense-in-depth gap with no ABI change.

2. **Success-path telemetry event** (minor extension, ~2 hr): exec_subprocess emits `internal.capability_denied` on failure paths but nothing on successful subprocess completion. A `host.exec_subprocess.completed` event closes the observability gap for all current and future callers.

3. **Truncated flag as Ok value** (not required for W-16; fundamentally different semantics): exec_subprocess returns `Err(OUTPUT_TOO_LARGE)` while run_subprocess would return `Ok(result_with_flag)`. This is the largest semantic gap, but the W-16 use case never encounters it. Noting for completeness; not blocking.
