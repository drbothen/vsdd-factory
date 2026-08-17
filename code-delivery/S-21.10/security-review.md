---
story: S-21.10
pr: 780
reviewer: security-reviewer
date: 2026-08-17
verdict: PASS_WITH_OBSERVATIONS
total_findings: 3
critical: 0
high: 0
medium: 1
low: 0
informational: 2
files_reviewed: 3
---

# Security Review — S-21.10 / PR #780

**Story:** S-21.10 — `FailurePolicy` schema extension (ADR-039 Phase 1)
**Verdict: PASS_WITH_OBSERVATIONS**
**No CRITICAL or HIGH findings. One MEDIUM finding (Phase 2 risk registration). Safe to merge.**

---

## Scope

This review covers:
- `crates/factory-dispatcher/src/registry.rs` — `FailurePolicy` enum + `RegistryEntry::failure_policy` field
- `crates/factory-dispatcher/src/lib.rs` — public re-export of `FailurePolicy`
- `crates/factory-dispatcher/src/executor.rs` — sibling sweep + `plugin_fail_closed` boundary verification

Sibling-sweep files (`partition.rs`, 5 integration test files) are structural consistency changes only; no logic changes. `CHANGELOG.md` contains no executable content. Both are outside the security-relevant scope and are not reviewed here.

---

## Phase 1 ADR-039 Boundary Verification (Critical Gate)

**Finding: CONFIRMED SAFE — Phase 1 boundary maintained.**

`plugin_fail_closed()` in `executor.rs` (lines 638–646) reads:

```rust
fn plugin_fail_closed(result: &PluginResult, on_error: OnError) -> bool {
    if on_error != OnError::Block {
        return false;
    }
    matches!(
        result,
        PluginResult::Crashed { .. } | PluginResult::Timeout { .. }
    )
}
```

This function's signature takes `on_error: OnError` — the **existing** `OnError` field — and has no knowledge of `FailurePolicy`. The task description confirms this function is untouched. The callsite in `execute_tiers` (lines 104–109) passes `outcome.on_error`, not any `failure_policy` value. The Phase 1 constraint (schema parsing only, zero enforcement change) is structurally guaranteed: `FailurePolicy` is stored on `RegistryEntry` but no call path passes it to any gate decision function.

---

## Findings

### SEC-001: `FailurePolicy::FailOpen` Default Creates Phase 2 Security Footgun

- **Severity:** MEDIUM
- **CWE:** CWE-636 (Not Failing Securely — "Fail Open")
- **OWASP:** A05:2021 — Security Misconfiguration
- **Attack Vector:** When Phase 2 enforcement lands, any registry entry that omits `failure_policy` in TOML will silently inherit `FailOpen`. If a plugin was previously protected by an implicit fail-closed assumption, the TOML author may not realize they must explicitly add `failure_policy = "fail-closed"` to preserve that behavior.
- **Impact:** In Phase 1 this is latent — no enforcement exists, so no live bypass is possible today. In Phase 2, if an operator upgrades to a dispatcher version that enforces `failure_policy` without auditing their `hooks-registry.toml`, every plugin that omits the field will fail-open on crash or timeout, potentially allowing blocked operations through.
- **Evidence:** The PR introduces `#[default]` on `FailOpen`:
  ```rust
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
  #[serde(rename_all = "kebab-case")]
  pub enum FailurePolicy {
      #[default]
      FailOpen,
      FailClosed,
  }
  ```
  Combined with `#[serde(default)]` on `RegistryEntry::failure_policy`, every existing entry in `hooks-registry.toml` that lacks the field will deserialize to `FailurePolicy::FailOpen`. The existing 52 plugins in the production registry (`hooks-registry.toml`) do not currently carry this field, so they will all default to `FailOpen` when Phase 2 enforcement is implemented.
- **Proposed Mitigation (Phase 2 gate, not blocking Phase 1 merge):**
  1. Before Phase 2 enforcement lands, add an explicit `FailurePolicy` migration guide in ADR-039 Phase 2 spec documenting that every entry defaults to `FailOpen`.
  2. Consider whether critical gate plugins (those currently `on_error = "block"`) should be defaulted to `FailClosed` instead. If `FailurePolicy` is intended to replace or complement `on_error`, the default should align with the conservative choice for security-critical gate hooks.
  3. At Phase 2 implementation time, add a registry-load validation that warns (or fails-closed) when `on_error = "block"` is present but `failure_policy` is absent — forcing operators to make an explicit choice rather than inheriting `FailOpen` silently.
- **Phase 1 disposition:** No immediate action required. This finding should be tracked as a mandatory input to Phase 2 implementation (ADR-039 Phase 2 story). It does not block this PR.

---

### SEC-002: TOML Deserialization Input Validation (Informational)

- **Severity:** INFORMATIONAL
- **CWE:** CWE-20 (Improper Input Validation)
- **Assessment:** The `FailurePolicy` enum uses `#[derive(Deserialize)]` with `#[serde(rename_all = "kebab-case")]`. Serde will reject any TOML value for `failure_policy` that is not `"fail-open"` or `"fail-closed"` at parse time — the same pattern used by `OnError` (verified by existing test `rejects_unknown_on_error_value` in `registry.rs` lines 732–743). No additional validation is needed. The `#[serde(deny_unknown_fields)]` attribute on `RegistryEntry` prevents injection of new fields via TOML. The existing `Registry::parse_str` test harness will reject entries like `failure_policy = "panic"` automatically.
- **No action required.**

---

### SEC-003: Public API Surface Expansion (Informational)

- **Severity:** INFORMATIONAL
- **CWE:** CWE-749 (Exposed Dangerous Method or Function) — does not apply here; documented for completeness
- **Assessment:** `FailurePolicy` is re-exported from `lib.rs` as a public type. This increases the crate's public API surface. There is no current unsafe use; the type is `Copy + Clone + PartialEq + Eq + Serialize + Deserialize`. The re-export is consistent with how other registry types (`OnError`, `RegistryEntry`, `Capabilities`, etc.) are exposed. External crates that depend on `factory-dispatcher` can now reference `FailurePolicy` directly. This is the intended design for a library crate. No security issue.
- **No action required.**

---

## Injection Risk Assessment

The `failure_policy` parsed enum value is stored on `RegistryEntry` but is NOT consumed by any code path in Phase 1 — it is never passed to a subprocess command, filesystem path, SQL query, shell interpolation, or security gate function. CWE-78 (OS Command Injection), CWE-89 (SQL Injection), and CWE-22 (Path Traversal) do not apply.

## Denial of Service Assessment

Malformed TOML for `failure_policy` (e.g., integer, array, unknown string) will produce a `RegistryError::Toml` at load time — the same fail-closed behavior as all other parse errors. The dispatcher refuses to start on registry parse failure. No amplification or resource exhaustion path exists. CWE-400 (Resource Exhaustion) does not apply.

## Authentication/Authorization Bypass Assessment

In Phase 1, `FailurePolicy::FailOpen` as the serde default does not open any security gap because the `plugin_fail_closed()` enforcement function is structurally independent of this field (see Phase 1 boundary verification above). No bypass of gate hooks is possible. CWE-285 (Improper Authorization) does not apply in Phase 1.

## Information Disclosure Assessment

The `RegistryError` variants do not carry `failure_policy` values in their error messages. Toml parse errors (`RegistryError::Toml`) surface the field name and TOML position but not any sensitive runtime data — the registry is a static configuration file under operator control. CWE-200 (Exposure of Sensitive Information) does not apply.

## Cryptographic Misuse Assessment

No cryptographic operations are added or modified. Not applicable.

## Dependency Vulnerability Assessment

No new dependencies are introduced. The change uses existing crate dependencies (`serde`, `toml`). No CVE lookup required.

---

## Risk Register Dispositions

No L2 Domain Spec Risk Register entries with Category=security were identified for the S-21.10 scope (ADR-039 Phase 1 schema extension). If any are added in future cycle artifacts, they should be reviewed at Phase 2 implementation time.

---

## Conclusion

PR #780 / S-21.10 is a **schema-only extension** that is correctly scoped to Phase 1. The `plugin_fail_closed()` enforcement boundary is structurally preserved. The single MEDIUM finding (SEC-001) is a latent Phase 2 risk that requires no remediation now but must be fed into the ADR-039 Phase 2 implementation specification to prevent a silent `FailOpen` regression when enforcement lands.

**Verdict: PASS_WITH_OBSERVATIONS — merge approved with SEC-001 tracked to Phase 2.**
