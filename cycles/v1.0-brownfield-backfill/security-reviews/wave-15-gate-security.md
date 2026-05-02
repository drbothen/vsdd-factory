---
document_type: security-review
review_type: wave-gate
wave: 15
date: 2026-05-02
develop_head: "3adfe0b"
verdict: FINDINGS
finding_counts: {critical: 0, high: 1, medium: 3, low: 2}
producer: security-reviewer
---

# W-15 Wave Gate Security Review

**Date:** 2026-05-02
**Develop HEAD:** 3adfe0b
**Scope:** All 12 Tier 1 native WASM port stories merged to develop (S-8.00, S-8.01, S-8.02, S-8.03, S-8.04, S-8.05, S-8.06, S-8.07, S-8.08, S-8.09, S-8.10, S-8.30) plus supporting dispatcher changes in `crates/factory-dispatcher/`
**Verdict:** FINDINGS (1 HIGH requires remediation before rc.3 release)

---

## Findings Summary

| ID | Severity | CWE | Title |
|----|----------|-----|-------|
| SEC-003 | HIGH | CWE-73, CWE-22 | VSDD_SINK_FILE path injection in production dispatcher |
| SEC-001 | MEDIUM | CWE-732 | WASI preopened_dir over-permissioned |
| SEC-002 | MEDIUM | CWE-706 | Split-brain in write_file path resolution |
| SEC-004 | MEDIUM | CWE-400 | HookPayload.extra HashMap unbounded — DoS via large stdin |
| SEC-005 | LOW | CWE-426 | session-start-telemetry binary_allow uses bare binary name |
| SEC-006 | LOW | CWE-78, CWE-426 | legacy-bash-adapter bash+curl capability not retired (pre-existing) |

---

## HIGH Findings

### SEC-003: VSDD_SINK_FILE Path Injection in Production Dispatcher

| Field | Value |
|-------|-------|
| **Severity** | HIGH |
| **CWE** | CWE-73 (External Control of File Name or Path), CWE-22 (Path Traversal) |
| **File** | `crates/factory-dispatcher/src/main.rs` |
| **Remediation required** | YES — before rc.3 release |

**Description:**

The factory-dispatcher reads the `VSDD_SINK_FILE` environment variable without any validation and passes it directly to `std::fs::OpenOptions::open()` with `create(true).append(true)`. There is no `#[cfg(test)]` gate, no `debug_assertions` guard, and no path validation. The code runs unconditionally in production builds.

An attacker who controls the dispatcher's process environment can set `VSDD_SINK_FILE` to any path:
- `~/.ssh/authorized_keys` — append a malicious SSH public key
- `/etc/cron.d/evil` — schedule arbitrary commands (if running as root or in a container)
- `/tmp/../../etc/passwd` — path traversal to sensitive files
- Any world-writable path the dispatcher process has write access to

The VSDD_SINK_FILE feature appears to be a debug/development sink for capturing hook payloads. Shipping it unconditionally in production release builds is a security vulnerability.

**Proof of concept:**

```sh
VSDD_SINK_FILE=~/.ssh/authorized_keys factory-dispatcher < hook-payload.json
# Appends raw JSON payload to authorized_keys file
# SSH may still parse valid keys before the garbage; at minimum corrupts the file
```

**Fix (required before rc.3):**

Option A (recommended for v1.0): Gate behind `#[cfg(debug_assertions)]` so the sink is only active in debug builds:
```rust
#[cfg(debug_assertions)]
if let Ok(sink_path) = std::env::var("VSDD_SINK_FILE") {
    // validate path before use
    ...
}
```

Option B: Gate behind a compile-time feature flag (`--features debug-sink`). Feature must be absent from release profile.

Additionally, regardless of which gate is used, add path validation:
- Reject paths containing `..` components
- Reject absolute paths outside an explicit allowlist (e.g., `/tmp/`, `$TMPDIR`)
- Reject paths that resolve outside the project directory

**Severity rationale:** This is a production-present file write sink with user-controlled path — a textbook path injection. HIGH rather than CRITICAL because exploitation requires control of the dispatcher's environment (an already-privileged position), and the primary deployment context (Claude Code hooks) typically runs as the developer's own user account. Severity would be CRITICAL in a multi-tenant deployment.

---

## MEDIUM Findings

### SEC-001: WASI preopened_dir Over-Permissioned

| Field | Value |
|-------|-------|
| **Severity** | MEDIUM |
| **CWE** | CWE-732 (Incorrect Permission Assignment for Critical Resource) |
| **File** | `crates/factory-dispatcher/src/invoke.rs` |

**Description:**

WASI preopened directory entries use `DirPerms::all() | FilePerms::all()` — granting read, write, and execute permissions to every plugin for every preopened path. The intended security model (per SS-02 and BC-4.07.001/.002) is capability-gated writes; a plugin should only have write access if it declares a `write_file` capability.

The over-permissioned preopens mean the WASI sandbox boundary grants more than the capability model implies. A plugin with no declared capabilities can still write to the preopened paths via native WASI filesystem calls. This is the same issue as CRIT-W15-003 in the adversary review, viewed through a security lens.

**Fix:** See CRIT-W15-003 fix options. For v1.0, the documentation-route fix (Option A) addresses the spec/reality divergence without breaking plugins. Capability tightening (Option B) is the v1.1 hardening target.

---

### SEC-002: Split-Brain in write_file Path Resolution

| Field | Value |
|-------|-------|
| **Severity** | MEDIUM |
| **CWE** | CWE-706 (Use of Incorrectly-Resolved Name or Reference) |
| **Files** | `crates/factory-dispatcher/src/write_file.rs`, `crates/factory-dispatcher/src/invoke.rs` |

**Description:**

`write_file.rs` resolves the write target path relative to `plugin_root` (the directory containing the `.wasm` file). `invoke.rs` resolves paths relative to `cwd` (the process working directory). These two resolution strategies can produce different absolute paths for the same relative input.

Unit tests that verify `path_allowed()` behavior in `write_file.rs` exercise the `plugin_root`-relative logic. If the actual dispatch path uses `cwd`-relative resolution, the tests verify dead code and real writes bypass the tested path-allow logic.

**Fix:** Canonicalize on a single resolution strategy. Recommend: always resolve relative to `cwd` (predictable for callers) and update `write_file.rs` tests to use the same base. Alternatively, ban relative paths entirely in the `write_file` host function (require absolute paths from plugins, validated against allowlist).

---

### SEC-004: HookPayload.extra HashMap Unbounded — DoS via Large stdin

| Field | Value |
|-------|-------|
| **Severity** | MEDIUM |
| **CWE** | CWE-400 (Uncontrolled Resource Consumption) |
| **File** | `crates/vsdd-hook-sdk/src/payload.rs` (S-8.30 serde flatten implementation) |

**Description:**

S-8.30 introduced `HookPayload.extra: HashMap<String, Value>` via `#[serde(flatten)]` (or equivalent) to capture unknown top-level fields from hook payloads. This HashMap is unbounded: a malicious or malfunctioning process that can write to the dispatcher's stdin could send a payload with thousands of extra fields, each with arbitrarily large string values.

The dispatcher deserializes the full payload before dispatching to plugins. A large payload causes unbounded heap allocation in the dispatcher process. In the Claude Code context, this is a local DoS that could cause the dispatcher to OOM and kill the hook chain.

**Fix:** Add a payload size gate at the stdin read boundary before deserialization:
```rust
const MAX_PAYLOAD_BYTES: usize = 1_024 * 1_024; // 1 MiB
```
Reject stdin inputs exceeding the cap with a clear error. Additionally, cap `HashMap` key and value lengths at deserialization time (custom `Deserialize` impl or a post-deserialization validation step).

---

## LOW Findings

### SEC-005: session-start-telemetry binary_allow Uses Bare Binary Name

| Field | Value |
|-------|-------|
| **Severity** | LOW |
| **CWE** | CWE-426 (Untrusted Search Path) |
| **File** | `hooks-registry.toml` session-start-telemetry entry |

**Description:**

`session-start-telemetry`'s `binary_allow` list includes `"factory-health"` as a bare binary name (no absolute path). The dispatcher executes this binary by searching `PATH`. An attacker who can place a malicious `factory-health` binary earlier in `PATH` than the legitimate one can execute arbitrary code in the plugin's execution context.

In the typical Claude Code developer environment, `PATH` is the developer's own `PATH` — this is a low-severity issue. It becomes higher severity in shared or container environments where multiple users can write to `PATH` components.

**Fix:** Use the absolute path to `factory-health` in `binary_allow`, or document that `factory-health` must be installed in a path-hardened location. For v1.0, a note in the installation docs is acceptable; for GA, prefer absolute path.

---

### SEC-006: legacy-bash-adapter bash+curl Capability Not Retired (Pre-existing)

| Field | Value |
|-------|-------|
| **Severity** | LOW |
| **CWE** | CWE-78 (OS Command Injection), CWE-426 (Untrusted Search Path) |
| **Note** | Pre-existing finding; not introduced by W-15 |

**Description:**

The `legacy-bash-adapter` grants `bash` and `curl` capabilities to hooks that use it. These capabilities are broad: bash allows arbitrary command execution; curl allows network access to arbitrary endpoints. W-15 retired Tier 1 hooks from the adapter (12 hooks) but 30+ Tier 2/3 hooks remain. The `bash` + `curl` capability surface is still present in production.

This is a pre-existing finding registered at earlier wave gates. W-15 reduced the attack surface (12 fewer hooks on legacy adapter) but did not eliminate it. TD-014 tracks full retirement.

**Fix:** Track via TD-014. No new action required for W-15 gate.

---

## Remediation Guidance

### Before rc.3 (required)

1. **SEC-003 (HIGH):** Gate `VSDD_SINK_FILE` behind `#[cfg(debug_assertions)]` in `crates/factory-dispatcher/src/main.rs`. Add path validation (reject `..`, reject absolute paths outside allowlist). Verify debug build still enables the sink for development use.

### Before v1.0 GA (recommended)

2. **SEC-001 (MEDIUM):** Document WASI preopened_dir vs capability gating boundary clearly in HOST_ABI.md (Option A from CRIT-W15-003). Tightening to read-only preopens is v1.1.
3. **SEC-002 (MEDIUM):** Canonicalize write_file path resolution strategy; update tests to match.
4. **SEC-004 (MEDIUM):** Add 1 MiB stdin size gate in dispatcher before payload deserialization.
5. **SEC-005 (LOW):** Document `factory-health` path hardening requirement.

### Deferred (pre-existing)

6. **SEC-006 (LOW):** Tracked via TD-014 full Tier 2/3 adapter retirement.

---

## Verdict

**VERDICT: FINDINGS**

One HIGH finding (SEC-003: VSDD_SINK_FILE path injection) requires remediation before rc.3 release. The other findings are MEDIUM/LOW and can be addressed before v1.0 GA.

The HIGH finding does not block development work on the fix-burst branch but MUST be resolved before the rc.3 tag is cut.
