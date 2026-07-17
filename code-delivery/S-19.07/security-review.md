# Security Review — PR #670 (S-19.07)

**PR HEAD SHA reviewed:** e7b518e7  
**PR title:** feat(S-19.07): verify-factory-lock read_prefix migration (BC-4.13.001 v1.17 Phase-B)  
**Branch:** feature/S-19.07 → develop  
**Date reviewed:** 2026-07-17  
**Reviewer:** vsdd-factory:security-reviewer (fresh-context re-run for evidence persistence)  
**Verdict:** APPROVE  
**Finding summary:** 0 CRITICAL, 0 HIGH, 0 MEDIUM, 2 LOW (both pre-existing accepted-risk architectural decisions; no new vulnerabilities introduced by this diff)

---

## Finding Table

| ID | Severity | New-in-diff | CWE | Title | Disposition |
|----|----------|-------------|-----|-------|-------------|
| SEC-S1907-001 | LOW | Partial | CWE-494 | Binary artifacts without cryptographic provenance verification | Accepted-risk architectural: binaries declared and expected; cause is CI-BLOCKER-1 fix |
| SEC-S1907-002 | LOW | No (pre-existing) | CWE-390 | Fail-open on CapabilityDenied / StateReadError bypasses factory lock | Accepted-risk architectural: PC6 design decision; same behavior as Phase-A EC-007; tested by T-005-ec005 |

---

## Detailed Findings

### SEC-S1907-001 — LOW — Binary artifacts without cryptographic provenance verification

**CWE-494** (Download of Code Without Integrity Check)

**Files affected:**
- `plugins/vsdd-factory/hook-plugins/verify-factory-lock.wasm` (rebuilt, 224,681 B per PR description)
- `plugins/vsdd-factory/hooks/dispatcher/bin/darwin-arm64/factory-dispatcher` (rebuilt from feature branch)
- `plugins/vsdd-factory/hooks/dispatcher/bin/darwin-x64/factory-dispatcher` (cross-compiled from feature branch)

**Observation:** The PR bundles three pre-compiled binary artifacts. The PR description declares these as intentional (PR-level BLOCKER-1: rc.22 dispatcher binaries rejected `unknown field read_prefix` via `deny_unknown_fields`, failing bats-darwin-leg CI). However:

1. No SHA-256 checksums are provided in the PR description or alongside the binaries in the repository to allow independent verification that the binaries match their declared source.
2. The dispatcher binary update departs from the established git-history pattern: all prior dispatcher binary commits follow the form `chore: bundle dispatcher binaries for v1.0.0-rc.X` (8 such commits in `git log --follow` history). This PR updates darwin binaries via a feature commit, not a release pipeline commit. This is the first instance of this pattern.
3. The darwin-x64 binary is cross-compiled, which increases build complexity and reduces reproducibility confidence.

**Why LOW and not MEDIUM:** The WASM artifact is compiled from source within the same open-source repo (`crates/hook-plugins/verify-factory-lock/`). The dispatcher binaries are built from the same repo source (post-S-19.09, commit 13ece92c). The root cause of the non-release binary update is well-documented (CI BLOCKER-1: `deny_unknown_fields` rejection of the new `read_prefix` capability field introduced by S-19.07). The WASM sandbox itself constrains the WASM binary's capabilities to the declared `[hooks.capabilities]` registry entries.

**Recommendation:** For future PRs that update dispatcher binaries outside the release pipeline, add a `## Binary Artifact Build Evidence` section to the PR description citing: (a) the exact source commit SHA used for each platform build, (b) the `cargo build` command and target triple used, (c) SHA-256 checksums of the produced artifacts. This provides a verifiable provenance chain without requiring a full release cycle for CI-BLOCKER fixes.

---

### SEC-S1907-002 — LOW — Fail-open on CapabilityDenied / StateReadError bypasses factory lock

**CWE-390** (Detection of Error Condition Without Action)

**File affected:** `crates/hook-plugins/verify-factory-lock/src/lib.rs` (diff line ~330–341)

**Observation:** When `host::read_prefix` returns any `HostError` (including `CapabilityDenied` when the registry entry is misconfigured), the guard logs a warning and returns `HookResult::Continue` — silently allowing the operation that should have been checked against the factory lock to proceed. The relevant code path:

```rust
Err(e) => {
    let msg = if e.contains("CapabilityDenied") {
        format!("capability_denied: read_prefix ({})", e)
    } else {
        format!("StateReadError: {}", e)
    };
    (callbacks.log_warn)(&msg);
    return HookResult::Continue;  // Fail-open: lock check bypassed
}
```

**Pre-existing status:** This is not new in this diff. Phase-A (read_file) had the identical fail-open behavior on CapabilityDenied (EC-007) and on `StateReadError`. The Phase-B migration preserves the same semantics. The story spec documents this explicitly as EC-005: "Plugin maps `HostError` → `StateReadError` → `HookResult::Continue` (PC6 fail-open); lock silently never enforces."

**Exploitation scenario:** An operator who misconfigures the registry by omitting the `[hooks.capabilities.read_prefix]` block from both `verify-factory-lock` entries would cause the factory lock to be silently bypassed for all operations. The warning is emitted to `log_warn` (plugin telemetry log) but does not surface as a blocking event.

**Why LOW:** (a) Requires deliberate operator misconfiguration of the registry — not accessible to normal users; (b) documented and tested (T-005-ec005 covers this exact scenario); (c) consistent with the hook's `on_error = "continue"` registry declaration — the guard is an advisory control, not a hard enforcement gate; (d) `internal.capability_denied` events from the host provide operator visibility via the dispatcher log even without the plugin-level warn.

**Recommendation:** No change required for this PR. If hardening is desired in a future story, consider emitting a blocking event or escalating to a metrics/alert channel when `CapabilityDenied` is received, rather than silent Continue. This would be a BC amendment to BC-4.13.001 and out of scope for S-19.07.

---

## Coverage Analysis — Focus Areas

### WASM host ABI surface: host::read_prefix usage and bounds semantics

**Status: No findings.**

The production call site `(callbacks.read_prefix)(".factory/STATE.md", 262144, READ_TIMEOUT_MS)` is correct:
- `max_bytes=262144` matches the ADR-025 §Decision 15 v1.18 adjudicated bound
- The literal `262144` is hardcoded in the source (not derived from untrusted input)
- `READ_TIMEOUT_MS = 5000` is a compile-time constant
- The callback signature is `(path: &str, max_bytes: u32, timeout_ms: u32)` — no pointer/size confusion possible in the Rust type system
- BC-1.17.001 PC-3 guarantees `read_prefix` never returns `OutputTooLarge`; the removal of `TooLarge`/`OutputTooLarge` handling is architecturally correct
- The `GuardCallbacks` struct uses generic type parameters (`R`, `E`, `L`) with closure constraints, preventing injection at the type boundary

### Path handling: SEC-003-class traversal constraints

**Status: No findings.**

The path `.factory/STATE.md` is a hardcoded string literal in the production call site. It contains no `..` sequences (no path traversal), no null bytes, and no dynamic user-controlled components. The registry `path_allow = [".factory/STATE.md"]` constrains the capability to exactly this path — any attempt to call `read_prefix` with a different path would be rejected by the dispatcher's capability check at the host ABI boundary. This is defense-in-depth appropriate for a SEC-003-class concern.

### Binary artifacts: declared/expected, supply-chain considerations

**Status: SEC-S1907-001 LOW (see above).**

The three binary artifacts are declared and expected per the PR description. The WASM is compiled from `crates/hook-plugins/verify-factory-lock/src/lib.rs` in the same repository. The dispatcher binaries are built from the same repository's source (post-S-19.09). No unexpected binary was introduced. The supply-chain concern is the absence of SHA-256 checksums for independent verification, classified LOW above.

The WASM binary runs in the `wasmtime` sandbox with declared capabilities only. Even if the WASM were malicious, the sandbox confines it to: reading `.factory/STATE.md` (prefix only, 262144 bytes), executing `git config user.email`, and emitting log_warn messages. No file writes, no network access, no execution of arbitrary commands.

### Error handling: no silent failure swallowing

**Status: No findings (for new code in diff).**

Every error path in the new/modified code emits either a `log_warn` message or results in a `HookResult::Continue` with a log entry. No error is silently discarded. The PC6 fail-open pattern is intentional and documented. The UTF-8 decoding errors for both the delimiter-found and delimiter-absent paths both emit `log_warn` before returning Continue — correct behavior.

The only pre-existing concern (SEC-S1907-002) is that `Continue` on error means the lock check is bypassed, not that the error is hidden — the error is logged. This is a design tradeoff, not a silent swallow.

---

## Non-Findings (explicitly ruled out)

| Area | Verdict | Rationale |
|------|---------|-----------|
| Buffer overflow / memory safety | No finding | Rust + WASM double boundary; `max_bytes` is a `u32` constant, not user-controlled |
| YAML injection via STATE.md content | No finding | `extract_frontmatter` returns a byte slice bounded by the `---` delimiter; `parse_factory_lock` delegates to `factory_lock_parse` crate which performs line-by-line key scanning — not a full YAML parser with arbitrary eval |
| Format string injection | No finding | Error strings passed to `log_warn` via `format!()` with `{}` — values are not format strings themselves |
| Race condition on STATE.md read | No finding | WASM sandbox is single-threaded; the read is atomic from the plugin's perspective; concurrent writes to STATE.md would produce a stale-read Continue (fail-open), which is the documented PC6 behavior |
| Capability escalation via registry migration | No finding | `read_prefix` and `read_file` are separate capabilities (BC-1.17.001 Invariant 3); the migration removes `read_file` access and adds `read_prefix` access with the identical `path_allow = [".factory/STATE.md"]` restriction — no privilege escalation |
| Removal of soft-warning diagnostic (observability gap) | No finding | `state_md_approaching_cap` warn was Phase-A `read_file` specific; with `read_prefix`, the cap is enforced at read time. No observability gap for security purposes — `internal.capability_denied` events provide operator visibility for the relevant failure mode |
| darwin-x64 cross-compiled binary executing on wrong arch | No finding | Git stores binaries with the correct path-separated platform directories; the dispatcher is selected by platform at runtime |
