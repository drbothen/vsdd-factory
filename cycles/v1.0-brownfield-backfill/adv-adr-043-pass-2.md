# ADR-043 Adversarial Review Pass 2 — v1.1 — DO-NOT-RATIFY

**Reviewed artifact:** ADR-043 v1.1 (exec-subprocess binary-allow load-time path resolution)
**Review date:** 2026-08-11
**Verdict:** DO-NOT-RATIFY
**Pass:** 2 of 3 (ADR-scoped; NOT cycle-level pass-11; streak UNCHANGED)
**D-chain:** D-972

## Part A — Finding Set

**BLOCKER (2):**

- **B-1**: `without_path` graceful-degradation semantics insufficiently specified — no delineation between per-plugin degradation (plugin proceeds with unresolvable binary; log warning) and session-level degradation (plugin startup fails; session blocked). v1.0 B-1 (global-refusal total outage) was addressed, but the replacement text leaves ambiguous whether a single unresolvable binary in `without_path` mode causes (a) that plugin to run without exec_subprocess, (b) that plugin to fail, or (c) the entire dispatcher to abort. All three readings are consistent with v1.1 text.
- **B-2**: Outcome/Control Matrix present but missing boundary cases: empty `binary_allow` list (should allow all? allow none?), allow-list entry with embedded whitespace (parse ambiguity), allow-list entry that is already an absolute path (double-resolution risk).

**HIGH (4):**

- **H-1**: §Decision 2 "per-plugin graceful degradation" does not specify what value `cmd` receives when the binary is unresolvable — is it the original bare name, empty string, or an error sentinel? The implementer cannot derive the correct behavior.
- **H-2**: `cfg(unix)` scoping for trusted-prefix list leaves Windows builds with no resolution path — `cfg(windows)` behavior unspecified.
- **H-3**: No test strategy section; §Verification lists no mutation variants for the path-substitution critical path.
- **H-4**: §Consequences does not address the on-disk race between resolution and execution (TOCTOU window explicitly accepted in §Threat Model but not reflected in §Consequences risk column).

**MEDIUM (3):**

- **M-1/M-2/M-3**: Minor text issues (version fields, stale cross-references, missing rationale for trusted-prefix list ordering).

## Part B — Disposition

Amended to v1.2 (B-1 and B-2 addressed; per-plugin scoping clarified; boundary cases documented). H-2 (`cfg(windows)`) NOT addressed in v1.2 — carried to v1.3. Review passed to pass-3.
