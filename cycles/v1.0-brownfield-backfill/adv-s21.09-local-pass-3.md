# S-21.09 LOCAL Adversarial Review Pass 3 — DO-NOT-RATIFY

**Reviewed artifact:** S-21.09 implementation at `12f280d1` (post pass-2 fix attempt; extract_hook_plugin_name redesigned with golden-value approach; T-014 FAIL arm added; run_t012_gate renamed)
**Review date:** 2026-08-11
**Verdict:** DO-NOT-RATIFY
**LOCAL streak:** 0/3 (reset by this pass)
**D-chain:** D-972

## Part A — Finding Set

**HIGH (1):**

- **H-1**: T-030 test uses hardcoded path `plugins/vsdd-factory/hook-plugins/` to construct the expected tracked-file set. This path is derived from the human reading of `hooks-registry.toml`, not from parsing the registry's `hook_plugins_dir` field. If `hook_plugins_dir` changes in the registry (e.g., to `hook-plugins/` relative), T-030 would continue to pass because it checks the hardcoded path independently of the registry parse path. The test would pass even if the registry-path parsing was broken. This is a registry-divergence blind spot.

**MEDIUM (1):**

- **M-1**: The 5 gate arms (T-012, T-014, T-016, T-018, T-020) cover declared+tracked, declared+untracked, undeclared+tracked, with_path variants. Missing: the case where a plugin declares a relative path AND the gate resolves it to absolute — gate behavior on path normalization is untested.

## Part B — Disposition

H-1 requires the test to derive its expected path from parsing `hooks-registry.toml hook_plugins_dir` at test time, not from a hardcoded string. Findings routed to implementer. S-21.09 streak reset to 0/3. **LOCAL streak: 0/3 after 3 passes.** PR cannot be opened until 3-CLEAN achieved. Story remains in-flight at `12f280d1`.
