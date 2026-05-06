---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-05-03T00:00:00Z
last_amended: 2026-05-05
phase: 1.4b
inputs: [gap-analysis-w16-subprocess.md]
input-hash: "[pending-recompute]"
traces_to: gap-analysis-w16-subprocess.md
origin: brownfield
extracted_from: ".factory/architecture/gap-analysis-w16-subprocess.md:Section 5"
subsystem: "SS-01"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.0.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-1.05.035: factory-dispatcher::host::exec_subprocess::canonicalizes_binary_path_before_allow_check — Path::canonicalize() applied before binary_allow match; TOCTOU prevention via canonicalize-then-allow-list-check ordering

## Description

**ADR-015 Awareness (added per E-9 v1.7 Post-Audit Amendment, propagated to BC at v1.22 per TD-VSDD-074):**
This BC's denial-path postcondition references the existing `internal.capability_denied` event name. Per ADR-015 D-15.2 reverse-DNS naming requirement and gap-analysis-w16-subprocess.md §"Existing denial-path telemetry" lines 341-351, this event MUST be renamed to `vsdd.capability.denied.exec_subprocess.v1` to map to the `audit` category per ADR-015 D-15.2 registry line 329 (`vsdd.capability.denied.* | audit`). The current name is INTERIM. The S-9.07 implementer (or the SS-01 implementer of any host-emit-fix story) MUST rename `internal.capability_denied` → `vsdd.capability.denied.exec_subprocess.v1` before merge. No new OQ filed; rename target is unambiguous per gap-analysis amendment.

The dispatcher MUST call `Path::new(cmd).canonicalize()` on the binary path BEFORE the binary_allow capability check. Canonicalization resolves symlinks and eliminates `..` segments. NUL-byte rejection is performed earlier by the existing `read_wasm_string` error path (see §Postcondition 2 and the Precedence Ladder). This closes a defense-in-depth gap identified in gap-analysis-w16-subprocess.md Section 5: `exec_subprocess.rs:230` currently passes `cmd` directly to `Command::new` with no traversal check.

**Canonicalization purpose (per gap-analysis Section 5 + TOCTOU mitigation discipline):** The canonicalize step's value is TOCTOU (time-of-check-time-of-use) prevention. Without canonicalization, an attacker could create a symlink that points to an allowed binary at allow-list-check time but to a different target at `Command::spawn` time. By canonicalizing BEFORE the allow-list check (and feeding the canonical path to `binary_allowed()`), the dispatcher ensures the spawned binary is the same as the allow-listed target. Symlink resolving to a non-allow-list path becomes a normal allow-list miss → CAPABILITY_DENIED via existing `emit_denial("binary_not_on_allow_list")` at exec_subprocess.rs:155 (the existing denial-path; no novel error-code pairing introduced).

## Preconditions

1. Plugin calls `vsdd::exec_subprocess`.
2. Plugin has a valid `Capabilities.exec_subprocess` block.
3. A `cmd` argument is provided.

## Postconditions

1. The dispatcher MUST call `Path::new(cmd).canonicalize()` BEFORE the `binary_allowed()` check at exec_subprocess.rs:152. The canonical path (resolved symlinks, no `..` segments) is computed ONCE before the allow-list check and propagated through ALL subsequent uses in the call chain — specifically: (a) `binary_allowed(canonical_path, &caps.binary_allow)` at line 152 instead of raw `cmd`, AND (b) `execute_bounded(canonical_path, args, ...)` at line 173 instead of raw `cmd`, ensuring `Command::new(canonical_path)` at line 230 spawns the same binary that was allow-list-checked. This closes the TOCTOU window between allow-check and spawn — without this propagation, an attacker swapping a symlink target between line 152 and line 230 would defeat the allow-list. (See Postcondition 2 for the INVALID_ARGUMENT pre-canonicalize guard.)
2. If `read_wasm_string` returns Err (non-UTF-8 byte sequence in WASM memory), the existing host-call error path returns `codes::INVALID_ARGUMENT` (-4) before any canonicalize attempt. (NOTE: NUL bytes are valid UTF-8 and pass `read_wasm_string`; NUL-containing paths fail at `Path::new(cmd).canonicalize()` returning EINVAL via Unix CString conversion in std::path layer → Precedence Ladder step 2 → `CAPABILITY_DENIED` (-1), NOT step 1.)
3. If `Path::new(cmd).canonicalize()` fails (binary not on disk, path invalid, IO error), returns `codes::CAPABILITY_DENIED` (-1). (NOTE: This is a BEHAVIOR CHANGE vs current implementation. Currently, missing-binary `cmd` paths fail at `command.spawn()` returning `INTERNAL_ERROR (-99)` per `exec_subprocess.rs:252`. Adding `canonicalize()` check pre-spawn changes the error code for the missing-binary case from `-99` → `-1`. Tests asserting `INTERNAL_ERROR` for missing-binary `cmd` will need to be updated to expect `CAPABILITY_DENIED`. This change is INTENTIONAL: aligning canonicalize-failure semantics with the existing 4 emit_denial paths under CAPABILITY_DENIED.) Emits `internal.capability_denied` (INTERIM name) via `emit_denial(ctx, cmd, "binary_canonicalize_failed", details)` BEFORE returning `CAPABILITY_DENIED` (-1) — preserves observability symmetry with the four pre-existing denial paths at exec_subprocess.rs:148/155/162/169.
4. **No new error path is introduced for symlink-resolved targets.** A symlink resolving to a path NOT in `binary_allow` → existing allow-list miss path → `emit_denial("binary_not_on_allow_list")` at exec_subprocess.rs:155 → CAPABILITY_DENIED (-1). This aligns with the existing 4 denial paths (binary_not_on_allow_list, shell_bypass_not_acknowledged, no_exec_subprocess_capability, setuid_or_setgid_binary at exec_subprocess.rs:148/155/162/169 — all CAPABILITY_DENIED -1) and avoids introducing a novel INVALID_ARGUMENT+capability_denied pairing.

**Precedence ladder (per pass-22 M-P22-003; corrected per pass-34 HIGH-P34-001; reframed per pass-36 D-279 — step (3) prefix-check DROPPED; renumbered):** When multiple validation conditions could fire, the host applies them in this order: (1) Non-UTF-8 byte sequence in `cmd` (per `read_wasm_string` at `host/memory.rs:47-54`) → `Err(INVALID_ARGUMENT -4)` — NUL bytes are valid UTF-8 and pass to step (2); (2) `Path::new(cmd).canonicalize()` returns Err → `emit_denial(ctx, cmd, "binary_canonicalize_failed", details)` then `Err(CAPABILITY_DENIED -1)` (path doesn't exist, NUL-containing path via EINVAL, symlink loop via ELOOP, name too long via ENAMETOOLONG, or any IO error including ETIMEDOUT on networked filesystems); (3) canonicalized path not in `binary_allow` list (including symlink resolved to non-allow-list path) → existing `emit_denial("binary_not_on_allow_list")` at exec_subprocess.rs:155 → `Err(CAPABILITY_DENIED -1)`. No step (3) prefix-check; symlink resolution handled within step (2) canonicalize + step (3) allow-list miss. Per `crates/factory-dispatcher/src/host/exec_subprocess.rs:152` (`binary_allowed()` call site) and BC-1.05.005/BC-1.05.032 sibling contracts.

**Panic semantics (v1 known limitation):** Any panic arising within the host-call body propagates uncaught to the wasmtime host-call boundary. Wasmtime converts the Rust panic to a wasmtime `Trap` and propagates it to the plugin caller. No `catch_unwind` wraps the host-call surface in `exec_subprocess.rs`. The functions in the prescribed canonicalize/allow-check/spawn chain (`Path::canonicalize`, `Command::new`, `command.spawn`) do not have documented panic vectors on the inputs this BC governs — `Path::canonicalize` returns `io::Result<PathBuf>` and `Command::new` is an infallible builder. Accordingly, panics from this call chain would indicate a stdlib programming error or an `unwrap()`/`expect()` introduced in a future helper, not a normal runtime failure mode. The `internal.host_function_panic` event class (`INTERNAL_HOST_FUNCTION_PANIC` at internal_log.rs:83) is declared as a `pub const` but has zero emit call sites in `exec_subprocess` or any other host function (confirmed: grep of `crates/factory-dispatcher/src/` finds only the const declaration at internal_log.rs:83 and one import in lib.rs:27 — no call sites). Panics in host calls are therefore unobservable in v1. Track host-call panic-handling decision in **OQ-W16-008** (v1: panic propagates as wasmtime Trap; v2 candidate: `catch_unwind` wrapper emitting `INTERNAL_HOST_FUNCTION_PANIC` before re-raising). See EC-014.

## Invariants

1. Path canonicalization MUST precede the binary_allow list match on every exec_subprocess invocation.

## Related BCs

- BC-1.05.001 — exec_subprocess capability check (depends on: this BC extends the pre-check chain)
- BC-1.05.002 — binary allow-list enforcement (composes with: this BC adds canonicalization before the allow-list match)
- BC-1.05.003 — shell bypass gate (sibling pre-check in the same gate chain)
- BC-1.05.036 — success-path telemetry (sibling extension from same gap analysis). NOTE: BC-1.05.036 introduces the FIRST non-denial event emitted via `ctx.emit_internal` (`host.exec_subprocess.completed`) — a structurally novel event class beyond the 4 existing denial events. Test-writers building event-taxonomy coverage MUST include this success-path event class. BC-1.05.035 adds canonicalization for TOCTOU prevention; symlink-resolved targets that fall outside `binary_allow` reach the existing CAPABILITY_DENIED path via emit_denial('binary_not_on_allow_list'). No novel error-code pairing introduced. Cross-dependency: BC-1.05.036 Postcondition 2 declares the success-path event payload `binary` field as `binary: String /* canonicalized full path */`. That field's correctness DEPENDS on BC-1.05.035 Postcondition 1 propagating the canonical path through to line 230 — see BC-1.05.035 EC-007. (NOTE: BC-1.05.036 EC-006 lists the field types table but does NOT carry the 'canonicalized full path' annotation; P2 is the canonical schema source-of-truth for that semantic.)

## Architecture Anchors

- `crates/factory-dispatcher/src/host/exec_subprocess.rs:152` — current `binary_allowed()` call site; canonicalize step added BEFORE this line, feeding canonical path to `binary_allowed()` instead of raw `cmd`. Line 230 (`Command::new(cmd)`) is INSIDE `execute_bounded()` which runs AFTER all 4 capability checks — the canonicalize insertion site is at line 152, NOT line 230.
- `crates/factory-dispatcher/src/host/read_file.rs:122-148` — sibling-pattern reference: `path_allow` canonicalize-then-check loop (the canonical-path-then-allow-check pattern that BC-1.05.035's Postcondition 1 mirrors for `binary_allow`)
- `.factory/architecture/gap-analysis-w16-subprocess.md` Section 5 — authority for this extension

**Cross-platform note:** Allow-list comparison at `binary_allowed` (host/exec_subprocess.rs:191) is byte-exact (`b == cmd || b == &basename`). On macOS HFS+, `Path::canonicalize` may return NFD-normalized paths that do not byte-equal NFC-normalized allow-list entries (Unicode normalization edge case). For ASCII-only allow-list entries (typical W-16 use case `bash`), this is a non-issue. Track in **OQ-W16-006** if non-ASCII allow-list entries are introduced.

## Story Anchor

S-9.07 (validate-wave-gate-prerequisite WASM port) — implementation task

## VP Anchors

(TBD — to be assigned in Phase 1.6b)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `cmd = "../etc/passwd"` (literal traversal); `binary_allow = ["bash"]` (typical S-9.07 capability shape per OQ-3); `cmd` basename "passwd" not in `binary_allow` | Returns `CAPABILITY_DENIED` (-1) via one of two ladder branches: **Branch A** (canonicalize succeeds): if `/etc/passwd` exists, `Path::new(cmd).canonicalize()` resolves to absolute `/etc/passwd` (the `../` segments resolve away). Fall-through past step (2) to step (3): allow-list miss (basename `passwd` not in `binary_allow`) → `emit_denial(ctx, cmd, "binary_not_on_allow_list", ...)` at exec_subprocess.rs:155 → CAPABILITY_DENIED (-1). **Branch B** (canonicalize fails): if `/etc/passwd` does NOT exist, `Path::canonicalize` returns Err with `io::ErrorKind::NotFound` (ENOENT) — step (2) fires: `emit_denial(ctx, cmd, "binary_canonicalize_failed", details)` → CAPABILITY_DENIED (-1). NOTE: EINVAL is the kernel response for NUL-byte paths (see EC-005), NOT for missing paths. Pre-canonicalize string-level `../` reject is NOT a separate guard. |
| EC-002 | `cmd` is a symlink pointing outside `binary_allow` (e.g., `bin/cmd` symlinks to `/etc/passwd`); `binary_allow = ["bash"]` | `Path::new(cmd).canonicalize()` resolves the symlink to absolute path `/etc/passwd`; `binary_allowed("/etc/passwd", &["bash"])` returns false (basename "passwd" not in allow list) → existing emit_denial("binary_not_on_allow_list") at exec_subprocess.rs:155 → CAPABILITY_DENIED (-1). NO novel error-code pairing introduced. |
| EC-003 | `cmd` is a valid absolute path with no traversal | `canonicalize()` succeeds; allow-list check proceeds normally |
| EC-004 | `cmd` binary does not exist on disk | `Path::new(cmd).canonicalize()` returns Err with `io::ErrorKind::NotFound` (ENOENT) — step (2) fires: `emit_denial(ctx, cmd, "binary_canonicalize_failed", details)` → CAPABILITY_DENIED (-1). (NOTE: This is the FOUNDATIONAL case described in Postcondition 3's BEHAVIOR CHANGE note — missing-binary `cmd` previously reached `command.spawn()` returning INTERNAL_ERROR (-99); adding canonicalize() pre-spawn changes this to CAPABILITY_DENIED (-1) with observable denial event.) [error] |
| EC-005 | `cmd` contains NUL byte | Returns `CAPABILITY_DENIED` (-1) via Precedence Ladder step (2) (`Path::new(cmd).canonicalize()` returns Err on NUL-containing paths across all supported std::path platforms — Unix path layer rejects via CString conversion (EINVAL); Windows path layer rejects in WTF-16 conversion. Cross-platform parity confirmed by std behavior). Emits `internal.capability_denied` with reason `binary_canonicalize_failed` per Postcondition 3. NOT `INVALID_ARGUMENT` (NUL bytes are valid UTF-8 and pass `read_wasm_string`). |
| EC-006 | `cmd` resolves but `Path::canonicalize()` IO error (e.g., missing binary, broken symlink, permission denied) | Returns `CAPABILITY_DENIED` (-1) AND emits `internal.capability_denied` with reason `binary_canonicalize_failed`; details include the IO error category. NOT silent. |
| EC-007 | Implementer feeds canonical to allow-check at line 152 but leaves raw `cmd` in `execute_bounded(...)` call at line 173 | TOCTOU window NOT closed. Failing implementation; covered by VP for this BC. The implementer MUST replace `execute_bounded(cmd, args, ...)` → `execute_bounded(canonical_path.as_str(), args, ...)`. (Witnessed by Test Vector row 6 — toctou symlink swap negative case.) |
| EC-008 | `cmd` resolves through symlink loop (`a → b → a`) — `Path::canonicalize` returns Err with `io::ErrorKind::FilesystemLoop` (ELOOP after kernel limit: 40 on Linux, 32 on macOS) | Step (2) fires: `emit_denial(ctx, cmd, "binary_canonicalize_failed", details)` → CAPABILITY_DENIED (-1). [edge-case] |
| EC-009 | `cmd = "/usr/bin"` (a directory, not a binary file); `binary_allow = ["bin"]` (unusual but possible config) | `Path::canonicalize` SUCCEEDS for directories on Unix (returns canonical directory path). Step (3) allow-check passes (basename `bin` matches allow-list). `execute_bounded` reaches `Command::new(canonical_path)` at line 230. `command.spawn()` at line 252 returns Err (EACCES or EISDIR — kernel refuses to exec a directory) → post-Ladder spawn failure path (exec_subprocess.rs:252) → `Err(codes::INTERNAL_ERROR)` (-99); no emit_denial; no event. **NOTE:** This masks a broken-capability-config (allow-list contains a directory name). v1 returns INTERNAL_ERROR; arguably should be CAPABILITY_DENIED. Track in **OQ-W16-005**. [known-limitation] |
| EC-010 | `cmd` length > kernel PATH_MAX (typically 4096 bytes Linux, 1024 macOS) — `Path::canonicalize` returns Err with `io::ErrorKind::Other` (ENAMETOOLONG) | Step (2) fires: `emit_denial(ctx, cmd, "binary_canonicalize_failed", details)` → CAPABILITY_DENIED (-1). [edge-case] |
| EC-011 | `cmd = ""` (empty string); `binary_allow = ["bash"]` | Returns `CAPABILITY_DENIED` (-1) via Precedence Ladder step (2): `Path::new("").canonicalize()` returns Err with `io::ErrorKind::NotFound` (ENOENT on Unix — empty path is not a valid filesystem entry); `emit_denial(ctx, cmd, "binary_canonicalize_failed", details)` → CAPABILITY_DENIED (-1). Same observable outcome as missing-binary (EC-004), but input is distinguishable in the denial event's `details.command` field (empty string vs absent path). [edge-case] |
| EC-012 | `args` bytes contain non-UTF-8 byte sequences (e.g., `args = [b"\xFF\xFE"]`) | At exec_subprocess.rs:127: `String::from_utf8_lossy(&bytes[i..i+len]).into_owned()` — non-UTF-8 bytes are silently replaced with U+FFFD (`?`) before the argument is passed to `Command::new`. **This is asymmetric with `cmd` strict UTF-8 enforcement** (Precondition step 1 returns INVALID_ARGUMENT on non-UTF-8 `cmd` via `read_wasm_string`). `args` lossy conversion is v1 known limitation — subprocess receives silently-mangled arguments. Track symmetry decision in **OQ-W16-008** (or extend OQ-W16-008 scope if panic-handling and args-encoding decisions are grouped). [known-limitation] |
| EC-013 | `binary_allow` entry contains a basename that could match an unintended target via canonicalize+basename resolution (e.g., `binary_allow = ["passwd"]` — after canonicalize, `/usr/bin/passwd` basename `passwd` matches; also, if `binary_allow = ["bin"]` and `cmd = "/usr/bin"`, canonicalize resolves the directory and basename `bin` matches `binary_allow`). See also EC-009 for the directory-spawn-INTERNAL_ERROR path. | `Path::canonicalize(cmd)` succeeds; `binary_allowed(canonical, &caps.binary_allow)` returns true (basename match); `execute_bounded` proceeds. If canonical path is a valid executable, subprocess spawns. **Operator MUST audit `binary_allow` entries for unintended substring matches** — allow-list does NOT restrict to `/usr/bin/passwd` specifically, only to basename `passwd`. A plugin with `binary_allow = ["passwd"]` can invoke any executable named `passwd` found on PATH or specified by full path in `cmd`. This is an operator-configuration responsibility, not a dispatcher enforcement gap. [known-limitation] |
| EC-014 | A panic arises within the host-call body (e.g., from a future `unwrap()` in a helper, or a stdlib programming error) | Panic propagates up to the wasmtime host-call boundary; wasmtime converts the Rust panic to a wasmtime Trap and propagates it to the plugin caller. **v1 known limitation:** no `catch_unwind` wraps the host-call surface in `exec_subprocess.rs`. The `internal.host_function_panic` event class is declared as a `pub const` at internal_log.rs:83 (`INTERNAL_HOST_FUNCTION_PANIC`) but has zero emit call sites in `exec_subprocess` or any other host function (grep of `crates/factory-dispatcher/src/` finds only the const declaration at internal_log.rs:83 and the import at lib.rs:27). The `HostContext.internal_log` field doc-comment (host/mod.rs:71-75) documents that the field exists to support `internal.capability_denied` and `internal.host_function_panic` emission — this is field documentation, not a TODO. The emit call for `INTERNAL_HOST_FUNCTION_PANIC` has simply not been written. Unhandled panics therefore produce no observability event. Track host-call panic-handling decision in **OQ-W16-008**. [known-limitation] |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `cmd = "../etc/passwd"` | `CAPABILITY_DENIED` (-1) | error |
| `cmd = "/usr/bin/bash"` (exists, no traversal) | Proceeds to allow-list check | happy-path |
| Symlink `cmd` resolving to path not in `binary_allow` (e.g., resolves to `/etc/passwd`; `binary_allow = ["bash"]`) | `CAPABILITY_DENIED` (-1) via allow-list miss → emit_denial("binary_not_on_allow_list") at exec_subprocess.rs:155. No novel error-code pairing. | edge-case |
| Non-existent binary path | Returns CAPABILITY_DENIED (-1); emits `internal.capability_denied` (INTERIM name) via `emit_denial(ctx, cmd, "binary_canonicalize_failed", details)` from canonicalize-fails path (Precedence Ladder step 2 — ENOENT from `Path::canonicalize`). | error |
| `cmd = "bin\x00ary"` (cmd contains NUL byte); `binary_allow = ["bash"]` | Returns `CAPABILITY_DENIED` (-1) via Precedence Ladder step (2); `Path::canonicalize` returns Err on NUL via CString EINVAL (Unix) / WTF-16 (Windows); emits `internal.capability_denied` via `emit_denial(ctx, cmd, "binary_canonicalize_failed", details)`. NOT INVALID_ARGUMENT — denial-class per ADR-015 D-15.2 line 329 mapping. | edge-case (NUL handling) |
| `cmd = "bin/cmd"` symlink; allow-check at line 152 sees canonical resolving to allow-listed `/usr/bin/cmd`; attacker swaps symlink between line 152 and line 230 to point at `/tmp/attacker` | If implementer correctly propagates canonical_path through `execute_bounded(canonical_path.as_str(), args, ...)` at line 173 → `Command::new(canonical_path)` at line 230 spawns the allow-listed target → success path. If implementer leaves raw `cmd` in execute_bounded → spawn against `/tmp/attacker` → TOCTOU defect; BC-1.05.035 Postcondition 1 violated. (Negative witness for EC-007 canonical-path propagation requirement.) | toctou-witness (negative test) |
| `cmd = ""`; `binary_allow = ["bash"]` | `Path::new("").canonicalize()` returns Err (ENOENT); `emit_denial(ctx, "", "binary_canonicalize_failed", details)` emits `internal.capability_denied`; returns `CAPABILITY_DENIED` (-1). Witnesses EC-011. | edge-case |
| `args = [b"\xFF\xFE"]` (non-UTF-8 arg bytes); `cmd = "bash"` (on allow-list); capability passes | After canonicalize/allow-check pass for `cmd`, `decode_args` at exec_subprocess.rs:127 calls `String::from_utf8_lossy` → arg arrives at subprocess as `"??"` (U+FFFD substitution for each non-UTF-8 byte). No error returned; subprocess is spawned with mangled argument. Witnesses EC-012 args-lossy-UTF8 path. | edge-case |
| `binary_allow = ["passwd"]`; `cmd = "/usr/bin/passwd"` (or any executable with basename `passwd`) | Canonicalize succeeds → basename `passwd` matches `binary_allow` → subprocess spawned. Witnesses EC-013 operator-must-audit-allow-list note. Subprocess receives full `passwd` access. | security-operator-audit |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/exec_subprocess.rs` |
| Stories | S-9.07 |
| Capability Anchor Justification | CAP-TBD — capability anchor to be confirmed in Phase 1.5; this BC governs exec_subprocess security hardening in factory-dispatcher |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `.factory/architecture/gap-analysis-w16-subprocess.md` Section 5 item 1; `crates/factory-dispatcher/src/host/exec_subprocess.rs:152` (binary_allowed call site; canonicalize insertion point) |
| **Confidence** | HIGH (gap explicitly identified by architect gap analysis) |
| **Extraction Date** | 2026-05-03 |
| **Extracted from** | `.factory/architecture/gap-analysis-w16-subprocess.md` Section 5 |

#### Evidence Types Used

- gap-analysis (architect-identified missing guard at exec_subprocess.rs:230)
- inferred (analogous to BC-2.02.013 I-2.6 which specified the same traversal check for the withdrawn run_subprocess path)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | YES — `Path::canonicalize()` performs filesystem stat; failure is handled deterministically |
| **Global state access** | No |
| **Deterministic** | YES given a fixed filesystem state |
| **Thread safety** | YES — read-only filesystem stat |
| **Overall classification** | Deterministic with external I/O dependency |

#### Refactoring Notes

(TBD — to be assessed in Phase 1.6b verification properties pass)
