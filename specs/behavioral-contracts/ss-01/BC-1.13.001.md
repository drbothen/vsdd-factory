---
document_type: behavioral-contract
level: L3
version: "1.12"
status: draft
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
phase: 1a
inputs:
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-platform-amendment-delta-analysis.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
  - .factory/specs/architecture/decisions/ADR-024-dispatcher-log-dir-resolution-and-plugin-root-fail-loud.md
input-hash: "3932fc5"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-platform-amendment-delta-analysis.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-pass-1
modified:
  - "2026-06-22 (v1.3) — S-18.14 spec-evolution (D-676 / ADR-024 v1.3): INV-8 (resolver WASM path resolution base must be TOML parent dir); PC-9 (successful load when artifacts present at TOML-parent-relative path); PC-10 (log_dir field in dispatcher.started payload — placed here because no dedicated dispatcher.started payload BC exists; ADR-024 Decision 5); EC-010 (relative WASM path exists at PLUGIN_ROOT but not CWD → must load successfully). ADR Reference updated to cite ADR-024 v1.3. Changelog and Architecture Anchors extended."
  - "2026-06-22 (v1.4) — S-18.14 fix-burst adversary pass-1: F-1 phantom-symbol fix (Architecture Anchors §PC-10 change site: replaced non-existent `InternalLog::write_started` with correct anchor — `InternalEvent::now(DISPATCHER_STARTED)` builder chain emitted via `internal_log.write(...)` in `main.rs`; `InternalLog::log_dir()` accessor in `internal_log.rs` §324); F-2 S-18.14 story anchor added to §Traceability Stories, §Story Anchor, and BC-INDEX; F-3 ADR version tokens dropped from §Traceability ADR Reference (POLICY 19); F-6 VP-074 proof-method token aligned to `kani-proof` (POLICY 9)."
  - "2026-06-22 (v1.5) — S-18.14 fix-burst adversary pass-2: F-1 INV-8 single-call-site correction (false 'two call sites / both fail_closed arms' claim replaced with ground-truth: exactly ONE production `get_or_compile` call site at line 361 of `resolver_loader.rs`; line 1057 is inside `#[cfg(test)]`; TD-VSDD-060 sibling-sweep confirms no second production call site); F-2 BC-1.01.004 sibling cross-reference added to §Related BCs (same path-join contract for hooks-registry.toml via `registry.rs::resolve_plugin_paths`; INV-8 is the resolvers-registry analogue) and EC-010 idempotent-absolute-passthrough guarantee cross-referenced to BC-1.01.004 EC-001 / EC-002."
  - "2026-06-22 (v1.6) — S-18.14 fix-burst adversary pass-4: F-1 VP-073 proof-method token corrected from `unit-test (integration test of resolver module compilation)` to authoritative `integration (resolver module compilation test)` per VP-INDEX Full Index line 408 and Proof Method Breakdown (POLICY 9 VP-INDEX-SoT). VP-075 sibling-sweep: token `proptest (200 trials, 5s timeout)` base-token `proptest` matches VP-INDEX (`proptest`) — no change needed."
  - "2026-06-22 (v1.7) — S-18.14 fix-burst adversary pass-7: F-1 PC-10 absolute-path guarantee made satisfiable — updated to state that `log_dir` is absolutized at `DISPATCHER_STARTED` emission time in `main.rs` via `std::path::absolute(internal_log.log_dir())` with verbatim-path fallback when CWD is inaccessible, per ADR-024 §Decision 5 v1.7; the `InternalLog::log_dir()` accessor stays verbatim (absolutization is NOT in the accessor). PC-10 canonical test vector and EC-F references updated to be consistent with absolutized value. O-3 INV-8 explicit is_relative() guard made normative: implementer MUST use explicit `entry.plugin.is_relative()` guard before `toml_path.parent().join(...)` (mirroring `registry.rs::resolve_plugin_paths`) — NOT relying solely on `PathBuf::join` absolute-replacement semantics — because on Windows a rooted-but-not-absolute path (`\\foo`) passes `PathBuf::join` replacement yet `is_absolute()` is false; `is_relative()` is the only portable discriminant (Windows is a release target). EC-010 idempotent-absolute-passthrough cross-ref to BC-1.01.004 EC-001/EC-002 preserved. [NOTE: v1.7 INV-8 rationale contained a factual error — the claim that `is_relative()` is false for rooted-but-not-absolute Windows paths (`\\foo`) is WRONG; `is_relative()` ≡ `!is_absolute()` and `\\foo` is rooted-but-NOT-absolute so `is_relative()` returns TRUE for it. Corrected in v1.8.]"
  - "2026-06-22 (v1.8) — S-18.14 fix-burst adversary pass-9 (F-1 BLOCKER POLICY 5): INV-8 rationale corrected — removed inverted `is_relative()` Windows-portability claim (v1.7 falsely stated `is_relative()` is false for rooted-but-not-absolute paths; in fact `is_relative()` ≡ `!is_absolute()` and `\\foo` on Windows has `is_absolute()=false` → `is_relative()=true`; bare `PathBuf::join` and explicit-`is_relative()`-guarded join are BEHAVIORALLY IDENTICAL for all paths including `\\foo`). Replaced with correct justification: precedent-consistency with `registry.rs::resolve_plugin_paths` (sibling call site uses explicit guard; divergent code path is a maintenance risk) and intent-clarity (leaves genuinely-absolute PathBuf unchanged rather than re-routing through join). Architecture Anchors `resolver_loader.rs` and `main.rs` INV-8 cross-references updated to drop false Windows-portability rationale. Changelog v1.7 entry annotated with correction note. EC-010 idempotent-absolute-passthrough cross-ref to BC-1.01.004 EC-001/EC-002 preserved and unaffected."
  - "2026-06-22 (v1.9) — S-18.14 pre-ready hardening burst: A-1 phantom-`toml_path` fix (INV-8 + Architecture Anchors: replaced non-existent binding `toml_path` with actual parameter `path`; alias `let toml_parent = path.parent()` per ADR-024 v1.9 + `registry.rs::resolve_plugin_paths` precedent `if let Some(base) = path.parent()`); A-2 `parent()==None` arm added to INV-8 normative pattern (`if let Some(base) = path.parent() { if entry.plugin.is_relative() { entry.plugin = base.join(&entry.plugin); } }` with None arm pass-through); A-4 volatile line-pins removed from INV-8 normative body (TD-VSDD-091: struck `~line 361` and `line 1057`; function-name anchors + `#[cfg(test)]` module description retained); p10-O1 VP-073 Property description aligned to VP-INDEX verbatim wording with `[non-authoritative paraphrase]` label on prior; VP-074/VP-075 sibling-sweep: descriptions confirmed consistent with VP-INDEX (paraphrase form with `[non-authoritative annotation]` on proof method)."
  - "2026-06-22 (v1.10) — S-18.14 confirmatory-pass-13 fix burst F-1 (POLICY 19 / TD-VSDD-091): volatile ADR version-pins removed from normative body — PC-10 body (~line 121) `§Decision 5 v1.7` → `§Decision 5`; Architecture Anchors `main.rs` entry `ADR-024 §Decision 5 v1.7` → `ADR-024 §Decision 5`. Exhaustive POLICY 5/19 sibling-sweep confirmed these were the only two normative version-pins; all other grep hits are exempt (modified: array, last_amended:, Changelog table rows). Changelog and Traceability ADR Reference unaffected (already stable form)."
  - "2026-06-22 (v1.11) — S-18.14 pass-16 fix burst F-1 (MAJOR POLICY 19): Architecture Anchors line-294 inverse-word-order version-pin fixed — `v1.3 Decision 1 Addendum ... and Decision 5` → `§Decision 1 Addendum ... and §Decision 5` (stable form, no version token). The v1.10 sweep regex (`ADR-[0-9]+ v[0-9]|§Decision [0-9]+ v[0-9]`) matched version-AFTER-keyword order only and MISSED the inverse word-order `v1.3 Decision` (version-BEFORE-keyword). Dual-word-order sweep `grep -nE 'v[0-9]+\\.[0-9]+ *(Decision|Addendum|§)|(ADR-[0-9]+|§Decision [0-9]+|Decision [0-9]+) *v[0-9]+\\.[0-9]+|ADR-[0-9]+ v[0-9]'` confirms zero residual normative version-pins in either word-order (all other hits in exempt sections: modified: array, last_amended:, Changelog table rows). v1.10 last_amended and Changelog 1.10 row FALSE attestation corrected — this row supersedes that false claim."
  - "2026-06-22 (v1.12) — S-18.14 pass-18 fix burst F-1 (MAJOR POLICY 5): INV-8 phantom-signature corrected — `pub fn load_registry(&self, path: &Path, ...)` falsely implied additional parameters via trailing `, ...`; ground truth (`resolver_loader.rs:281-284`): exactly ONE parameter `path: &Path`; corrected citation: `pub fn load_registry(&self, path: &Path) -> Result<...>` (exact one-param list; `Result<...>` return elision). Phantom-paramlist sweep `grep -n ', \\.\\.\\.)' BC-1.13.001.md` confirms zero residual normative `, ...`-in-paramlist occurrences remaining (the only other `, ...` in the file is `internal_log.write(...)` at Architecture Anchors — a method-call ellipsis in prose, NOT a parameter list)."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-1.13.001
section: "1.13"
last_amended: "2026-06-22 (v1.12) — S-18.14 pass-18 fix burst F-1 (MAJOR POLICY 5): INV-8 phantom-signature `pub fn load_registry(&self, path: &Path, ...)` corrected to `pub fn load_registry(&self, path: &Path) -> Result<...>` — exact one-param list, `, ...` removed. Phantom-paramlist sweep confirms zero residual normative `, ...`-in-paramlist occurrences."
---

# BC-1.13.001: Dispatcher MUST load `resolvers-registry.toml` at startup and inject resolver context into `plugin_config` before each hook dispatch

## Description

The dispatcher (`crates/factory-dispatcher/`) loads `resolvers-registry.toml` at startup
using the same mtime-based cache invalidation pattern as `plugin_loader.rs`. Before each
hook dispatch, the dispatcher inspects the hooks-registry entry's `needs_context` field; for
each declared resolver name, it invokes the corresponding WASM resolver module with a
`ResolverInput` struct, merges the returned `Option<Value>` output into `plugin_config` under
the resolver's `context_key`, and then dispatches the hook with the enriched payload. If
`resolvers-registry.toml` is absent, the dispatcher starts with zero resolvers configured —
this is an expected, non-error operational state for factories that have not adopted resolvers.

## Preconditions

1. The dispatcher binary is starting up (or restarting after a WASM artifact mtime change).
2. The hooks-registry.toml file is present and valid (standard dispatcher precondition).
3. `resolvers-registry.toml` MAY or MAY NOT be present at the plugin registry path
   (`plugins/<factory>/resolvers-registry.toml` or the dispatcher's configured registry root).
   **Absence is not an error — see PC1 Critical Constraint below.**
4. A hook dispatch is being processed: the dispatcher has matched an event to a registered
   hooks-registry entry.
5. The matched hooks-registry entry has been deserialized with the `needs_context: Vec<String>`
   field (which defaults to `[]` when absent — backward-compatible with all existing entries).

### PC1 (Critical Constraint — Backward Compatibility)

**If `resolvers-registry.toml` is absent at startup, the dispatcher MUST initialize with zero
resolvers configured. This MUST NOT be a startup error. Existing deployments without the file
behave identically to before this feature.** No error is emitted, no warning is emitted to
stderr, and no hook dispatch is blocked as a result of the absent file. The dispatcher simply
treats the resolver registry as empty. This is the highest-priority backward-compatibility
invariant for the resolver platform: factory deployments that have not adopted resolvers must
be unaffected.

## Postconditions

1. **Registry loading:** The dispatcher loads and compiles all resolver WASM artifacts listed
   in `resolvers-registry.toml` at startup. The count of successfully loaded resolvers is
   written to the dispatcher log at startup (e.g., `"Loaded N context resolvers"`).
2. **Registry parse error (fail-loud):** If `resolvers-registry.toml` is present but
   malformed (TOML parse error, schema validation error, or a referenced `.wasm` path does
   not exist), the dispatcher MUST emit a `resolver.load_error` event with the specific error
   detail and MUST NOT start with a partial resolver set that silently omits the failed entry.
   The dispatcher startup fails loudly; it does not silently degrade to zero resolvers when the
   file exists but is broken.
3. **`needs_context = []` (no-op path):** If the matched hooks-registry entry has
   `needs_context: []` (or the field is absent), the dispatcher skips resolver invocation
   entirely and dispatches the hook with the unmodified `plugin_config`. This is a zero-cost
   path for all existing hooks.
4. **Resolver invocation:** For each resolver name in `needs_context`, the dispatcher invokes
   `ResolverRegistry::invoke_resolver(name, ResolverInput { event_type, hook_event_name,
   agent_type, project_dir, plugin_config })`. The invocation produces `ResolverOutput { key,
   value: Option<Value> }` or a `ResolverError`.
5. **Merge into `plugin_config`:** Each resolver's output is merged into `plugin_config` under
   the resolver's declared `context_key`. The static `plugin_config` from the hooks-registry
   entry is preserved; resolver outputs are overlaid additively (resolver output wins on key
   collision per the merge contract in BC-4.12.005). The hook sees the merged `plugin_config`.
6. **Unknown resolver name (fail-loud at dispatch):** If a `needs_context` entry names a
   resolver that is not registered in `resolvers-registry.toml`, the dispatcher MUST emit a
   `resolver.not_found` event with the hook name and the missing resolver name, and MUST NOT
   silently inject empty context. The hook dispatch proceeds without the missing context; the
   hook is responsible for treating the absent key as an error if the context is required.
7. **Resolver invocation order:** Resolvers in `needs_context` are invoked in declaration order.
   The merge is applied in the same order (first resolver's output is merged first).
8. **Hook receives enriched payload:** The `invoke_plugin` call receives the fully merged
   `plugin_config` — including all resolver outputs — as its `plugin_config` field. The hook
   plugin has no visibility into whether its `plugin_config` was enriched by resolvers or came
   entirely from the static registry config.
9. **PC-9 (Successful load when artifacts present):** When `resolvers-registry.toml` is present
   and valid and the referenced WASM artifacts exist at TOML-parent-relative paths, the dispatcher
   MUST load all declared resolvers with zero `resolver.load_error` events for any declared
   resolver. A `resolver.load_error` for a resolver whose WASM file exists at the TOML-parent-
   relative path is a specification violation. (Anchor: see INV-8 below; root-cause fix for the
   empirically observed 8,560 `resolver.load_error` / 0 successful loads since rc.21.)
10. **PC-10 (`log_dir` observability in `dispatcher.started`):** The `dispatcher.started` event
    payload MUST include a `log_dir` string field whose value is the resolved log directory
    **absolutized at emission time** in `main.rs` at the `DISPATCHER_STARTED` emission site —
    NOT in the `InternalLog::log_dir()` accessor (which stays verbatim). The absolutization is
    performed via `std::path::absolute(internal_log.log_dir())` with a verbatim-path fallback
    (`unwrap_or_else(|_| internal_log.log_dir().to_path_buf())`) when CWD is inaccessible, per
    ADR-024 §Decision 5. Functional anchors: `std::path::absolute`, `DISPATCHER_STARTED`
    (event name constant), `InternalLog::log_dir()` (path accessor, stays verbatim). The field
    is emitted unconditionally on every startup, is NOT optional, is NOT behind a feature flag,
    is NEVER null or empty, and is absolute whenever CWD is accessible (the common case); on
    the rare CWD-inaccessible fallback the value equals whatever `InternalLog::log_dir()` returns
    verbatim (which may itself be absolute depending on how it was constructed).
    > **Placement note:** No dedicated `dispatcher.started`-payload BC exists in the SS-01 catalog
    > (BC-1.06.007 and BC-1.06.009 govern brownfield `InternalLog` JSONL envelope shape; BC-1.12.001
    > cites `dispatcher.started` only as a test-vector example of lifecycle routing). This
    > postcondition is placed here because ADR-024 Decision 5 is architecturally coupled to the
    > resolver-platform startup path (same `main.rs` startup sequence), and S-18.14 is the
    > implementing story. If a dedicated `dispatcher.started` payload BC is authored in a future
    > cycle, PC-10 should migrate there; cite this BC as the origin in that migration.

## Invariants

1. **Factory-agnostic dispatcher:** The dispatcher core (`crates/factory-dispatcher/`) has
   zero compile-time dependency on any per-factory resolver crate. All resolver logic lives
   in WASM plugins loaded at runtime. The dispatcher knows only the `ResolverInput` /
   `ResolverOutput` ABI (BC-4.12.002), not the semantic meaning of any resolver's output.
2. **Absent registry = zero resolvers (not error):** `resolvers-registry.toml` absent ALWAYS
   yields zero resolvers and NEVER yields a startup error. This invariant is non-negotiable;
   any code path that converts a missing resolver registry file into a hard error violates this
   BC and must be treated as a regression.
3. **`needs_context` defaults to empty:** The `needs_context` field on `RegistryEntry` in
   `registry.rs` MUST use `#[serde(default)]` so that existing `hooks-registry.toml` entries
   without the field parse without error. No `deny_unknown_fields` constraint may be added to
   `RegistryEntry` without a schema_version bump and migration path.
4. **Resolver loading at startup, not per-dispatch:** Resolvers are compiled into `Module`
   objects once per dispatcher lifetime (mtime-cache per BC-4.12.001). Each dispatch creates
   a fresh `Store` per resolver invocation. The compilation cost is amortized; dispatch
   latency for resolver invocation is limited to `Store` creation + WASM function execution.
5. **Context injection precedes `invoke_plugin`:** The resolver invocation and merge step
   MUST complete before `invoke_plugin` is called. Hooks MUST see the fully merged
   `plugin_config`; partial injection is not permitted.
6. **EXPLICIT registry only (no auto-discovery):** The dispatcher MUST NOT scan filesystem
   directories for WASM resolver artifacts. Only resolvers explicitly listed in
   `resolvers-registry.toml` are loaded. This is a load-bearing invariant for the factory-
   agnostic design: auto-discovery would require naming conventions baked into the dispatcher.
7. **Separate registry file:** `resolvers-registry.toml` is a distinct file from
   `hooks-registry.toml`. The two files have different schemas, different lifecycle roles
   (pre-dispatch data providers vs. event handlers), and are versioned independently. The
   dispatcher loads both files at startup.
8. **INV-8 (Resolver WASM path resolution base):** Relative `plugin` paths in
   `resolvers-registry.toml` (e.g., `plugin = "hook-plugins/vsdd-context-resolvers.wasm"`)
   MUST be resolved against the TOML file's parent directory — which equals `CLAUDE_PLUGIN_ROOT`
   at runtime — NOT against the dispatcher's process working directory (CWD). Absolute `plugin`
   paths pass through unchanged (no re-joining). There is a SINGLE `get_or_compile` call site
   in `load_registry` (the second occurrence inside `#[cfg(test)]` is test-only and not a
   production call site; TD-VSDD-060 sibling-sweep confirms this). **The implementer MUST use
   the normative pattern mirroring `registry.rs::resolve_plugin_paths`:
   ```rust
   let toml_parent = path.parent();
   if let Some(base) = toml_parent {
       if entry.plugin.is_relative() {
           entry.plugin = base.join(&entry.plugin);
       }
   }
   // None arm: path.parent() returns None only for a bare root or empty path;
   // entry.plugin passes through unchanged (no base to join against).
   ```
   The `path` parameter of `load_registry` (`pub fn load_registry(&self, path: &Path) -> Result<...>`)
   is the actual binding; `toml_parent` is a local alias for `path.parent()`. This mirrors
   the `registry.rs::resolve_plugin_paths` precedent (`if let Some(base) = path.parent() { ... }`)
   and handles the `None` arm explicitly.** The justification for the explicit guard is twofold:
   (a) **Precedent-consistency:** The sibling call site `registry.rs::resolve_plugin_paths` (which
   resolves `hooks-registry.toml` plugin paths) uses the same explicit `if entry.plugin.is_relative()`
   guard pattern. Using a bare unconditional `base.join()` in `resolver_loader.rs` would create a
   divergent second code path for the same semantic operation; the explicit guard mirrors the proven
   precedent and makes the intent immediately readable.
   (b) **Intent-clarity:** The explicit guard makes it unambiguous that a genuinely-absolute
   `PathBuf` is left unchanged, rather than re-routed through the join expression.
   > **Rust semantics note:** `Path::is_relative()` is defined as `!Path::is_absolute()` — there is
   > no separate "rooted" category. On Windows, `C:\x` is absolute (`is_relative()`=false); a
   > rooted-but-not-absolute path (`\foo`) has `is_absolute()`=false and therefore `is_relative()`=
   > **true** — meaning the explicit guard would join it, and `PathBuf::join(\foo)` would also
   > replace the base. The two forms (explicit guard + join vs bare join) are **behaviorally
   > identical** on all platforms including Windows rooted paths. The guard is required for
   > precedent-consistency and intent-clarity, NOT as a Windows-portability fix vs bare join.
   The path-join MUST precede the single `get_or_compile` call so the resolved absolute path
   feeds both the `fail_closed: true` and `fail_closed: false` error-handling arms identically;
   the `fail_closed` divergence is in the post-call error `match`, not at separate call sites.
   A `resolver.load_error` for a resolver whose WASM file exists at the correct TOML-parent-relative
   path is a violation of this invariant and MUST be treated as a regression. EC-010's
   idempotent-absolute-passthrough guarantee (see below) cross-references BC-1.01.004 EC-001/EC-002
   and MUST be preserved by any INV-8 fix.
   > **Why CWD-relative is wrong:** The dispatcher is invoked by the Claude Code hook
   > infrastructure with CWD set to the host project directory (e.g., `/Users/<user>/project/`).
   > WASM plugin files live under `CLAUDE_PLUGIN_ROOT` (e.g.,
   > `~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/`). A relative path from the
   > TOML resolves correctly only when the base is the TOML's own parent directory. CWD as
   > base yields a path that does not exist, causing `path.canonicalize()` to return `Err(ENOENT)`
   > — the root cause of 8,560 `resolver.load_error` / 0 successful loads observed since rc.21.
   > (Anchor: `resolver_loader::load_registry` with `path.parent()` joined to `entry.plugin`
   > before the single production `get_or_compile` call; function-name anchor per TD-VSDD-091.)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `resolvers-registry.toml` absent at startup | Zero resolvers configured. Dispatcher starts normally. No error, no warning. All existing hooks function identically to pre-resolver behavior. |
| EC-002 | `resolvers-registry.toml` present but TOML parse error | Fail-loud: emit `resolver.load_error` with parse error detail. Dispatcher startup fails. Do not silently proceed with zero resolvers. |
| EC-003 | `resolvers-registry.toml` present; a `plugin` path does not exist on disk | Fail-loud: emit `resolver.load_error` with the missing path. Startup fails for that resolver entry. |
| EC-004 | Hook entry has `needs_context = ["wave_context"]` but `wave_context` is not in the resolver registry | Emit `resolver.not_found` event at dispatch time. Dispatch proceeds without context injection. Hook sees absent key in `plugin_config`. |
| EC-005 | Hook entry has `needs_context = []` (or field absent) | Resolver invocation skipped entirely. Zero overhead on the dispatch hot path. |
| EC-006 | Resolver WASM mtime changes while dispatcher is running | Mtime-based cache invalidation triggers reload of the changed resolver module on next dispatch that needs it (same pattern as `plugin_loader.rs`). |
| EC-007 | Two hooks in the same dispatch share the same `needs_context` resolver | Each hook's dispatch independently invokes the resolver (no cross-hook caching per OD-4). Each invocation creates a fresh `Store`. |
| EC-008 | `resolvers-registry.toml` has zero `[[resolvers]]` entries | Equivalent to absent file: zero resolvers configured. Valid state; not an error. |
| EC-009 | Resolver returns `None` for its `value` field | Key is NOT written to `plugin_config`. The key is absent from the hook's `plugin_config`. The hook must treat the absent key as appropriate for its logic. |
| EC-010 | `resolvers-registry.toml` present; `plugin` path relative (e.g., `hook-plugins/vsdd-context-resolvers.wasm`); WASM file exists at `CLAUDE_PLUGIN_ROOT/<rel>` but NOT at `<CWD>/<rel>` | Resolver MUST load successfully (TOML-parent-relative resolution per INV-8 wins). Zero `resolver.load_error` events for this resolver. The CWD-relative path's non-existence is irrelevant. A CWD-relative resolution attempt that produces `path.canonicalize()` `Err(ENOENT)` is the bug that INV-8 and PC-9 are designed to prevent. Additionally: if `plugin` is already absolute, the path-join MUST pass it through unchanged — idempotent-absolute-passthrough guarantee per BC-1.01.004 EC-001 ("Plugin path already absolute → returned as-is") and BC-1.01.004 EC-002 ("Resolution called twice → same result both times (idempotent)"); INV-8's fix MUST preserve this guarantee. |

## Canonical Test Vectors

| Scenario | Registry State | `needs_context` | Expected Behavior |
|----------|---------------|-----------------|-------------------|
| Registry absent | File not found | any | Zero resolvers; dispatcher starts. Hooks dispatch normally. |
| Registry present; no resolvers | `[[resolvers]]` empty | `["wave_context"]` | `resolver.not_found` event; hook proceeds with unmodified `plugin_config`. |
| Registry present; resolver loaded | `wave_context` registered | `["wave_context"]` | Resolver invoked; output merged into `plugin_config["wave_context"]`; hook sees merged config. |
| Registry present; resolver loaded | `wave_context` registered | `[]` | Resolver NOT invoked; hook sees unmodified `plugin_config`. Zero overhead. |
| Resolver returns `None` | `wave_context` registered | `["wave_context"]` | `plugin_config["wave_context"]` key is absent (not null, not empty). |
| Resolver returns value | `wave_context` registered; returns `{stories: [...]}` | `["wave_context"]` | `plugin_config["wave_context"] = {stories: [...]}`. |
| TOML parse error | Malformed TOML | — | Startup fails; `resolver.load_error` emitted. |
| Unknown resolver name | `foo` not registered | `["foo"]` | `resolver.not_found`; dispatch proceeds without context. |
| Relative WASM path; file at PLUGIN_ROOT, absent at CWD | `wave_context` registered with relative path; WASM exists at TOML-parent dir, not at process CWD | `["wave_context"]` | Resolver loads successfully (zero `resolver.load_error`); context injected into `plugin_config`. CWD non-existence irrelevant. (Witnesses INV-8 / PC-9 / EC-010.) |
| `dispatcher.started` event | Dispatcher starts with valid `resolvers-registry.toml`; CWD accessible | — | `dispatcher.started` event payload includes `log_dir` string field whose value is an absolute path — produced by `std::path::absolute(internal_log.log_dir())` at the `DISPATCHER_STARTED` emission site in `main.rs`; field is non-empty. (Witnesses PC-10; absolutization is at emission time, NOT in `InternalLog::log_dir()` accessor.) |
| `dispatcher.started` event (CWD inaccessible) | Dispatcher starts; CWD not accessible (e.g., deleted between process start and `DISPATCHER_STARTED` emission) | — | `dispatcher.started` event payload includes `log_dir` string field whose value is the verbatim return of `InternalLog::log_dir()` (fallback path, may or may not be absolute). Field is non-empty. (Witnesses PC-10 verbatim-fallback arm.) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-073 | Resolver-Load Purity — resolver WASM module loading must be pure: same registry file always produces same resolver set, no side effects [VP-INDEX verbatim; prior BC paraphrase "loading a `.wasm` resolver artifact is deterministic and has no observable side effects at load time" superseded by this per POLICY 9 VP-INDEX-SoT] | integration (resolver module compilation test) [non-authoritative annotation; authoritative token: `integration` per VP-INDEX] |
| VP-074 | Resolver-Error Isolation — resolver crash, trap, or timeout must not propagate to dispatcher process [VP-INDEX verbatim paraphrase; BC summary: a resolver crash or trap does not propagate to the dispatcher process] | kani-proof (pure error-classification logic); + integration test (trap injection) [non-authoritative annotation] |
| VP-075 | Context-Injection Determinism — same resolver input always produces same output; merging is order-independent when keys are disjoint [VP-INDEX verbatim paraphrase; BC summary: identical `ResolverInput` yields identical `ResolverOutput`] | proptest (200 trials, 5s timeout) |
| (unit-test) | Absent `resolvers-registry.toml` yields zero resolvers and no startup error | Rust unit test |
| (unit-test) | `needs_context = []` skips resolver invocation (zero overhead path) | Rust unit test (assert resolver mock not called) |
| (unit-test) | `needs_context` with unknown resolver name emits `resolver.not_found` and does not panic | Rust unit test |
| (unit-test) | Resolver output merged into `plugin_config` under correct key | Rust unit test |
| (unit-test) | Resolver `None` output leaves key absent (not present-but-null) from `plugin_config` | Rust unit test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls and session/worktree lifecycle events with sandboxed WASM plugins") per capabilities.md §CAP-002 — this BC governs the dispatcher core's pre-dispatch context-injection mechanism, which is the runtime wiring that makes WASM hook plugins receive enriched `plugin_config` at dispatch time. CAP-002 defines the full dispatch pipeline from Claude Code lifecycle events through the sandboxed WASM plugin invocation; context injection via resolvers is an extension of that pipeline, operating at the dispatcher layer (SS-01) before `invoke_plugin` is called. The resolver platform is factory-agnostic infrastructure within CAP-002's sandboxed plugin dispatch model. |
| Secondary Capability Reference | CAP-009 ("Author and publish WASM hook plugins using the Rust SDK") per capabilities.md §CAP-009 — resolver plugins are authored using the SDK's `resolver-authoring` feature flag (BC-4.12.002); CAP-009 governs the SDK surface used by resolver authors. |
| L2 Domain Invariants | none |
| Architecture Module | `crates/factory-dispatcher/src/resolver.rs` (ContextResolver trait, ResolverRegistry); `crates/factory-dispatcher/src/resolver_loader.rs` (WASM module loading + mtime-cache); `crates/factory-dispatcher/src/executor.rs` (pre-dispatch resolver invocation); `crates/factory-dispatcher/src/main.rs` (resolvers-registry.toml load at startup); `crates/factory-dispatcher/src/registry.rs` (RegistryEntry.needs_context field) |
| Stories | S-12.03, S-12.04, S-12.06, S-12.08, S-18.14 |
| FR | FR-RESOLVER-001 (factory-agnostic runtime context injection for hooks via sandboxed WASM-plugin resolvers) |
| ADR Reference | ADR-018 (WASM-plugin Context Resolvers — Design and Layering) — codifies the separate registry, factory-agnostic dispatcher, and explicit-registration decisions (OD-1 through OD-6) that this BC encodes as behavioral contracts. ADR-024 §Decision 1 Addendum (Resolver WASM plugin path resolution) — establishes that relative `plugin` paths MUST resolve against `path.parent()` (the `load_registry` parameter; alias `toml_parent`), not CWD; functional anchor is `resolver_loader::load_registry` (TD-VSDD-091). ADR-024 §Decision 5 (`log_dir` observability) — contracts that `dispatcher.started` payload MUST include `log_dir` from `InternalLog::log_dir()`; see PC-10. |

## Related BCs

- BC-4.12.001 — composes with (resolver lifecycle invariant — loaded once at startup with mtime-cache; this BC describes the startup loading step)
- BC-4.12.002 — composes with (resolver ABI and payload schema — defines `ResolverInput` and `ResolverOutput` types used in resolver invocation)
- BC-4.12.003 — composes with (resolver capability model — capability declarations are read at registry-load time)
- BC-4.12.004 — composes with (resolver error and crash isolation — error handling for failed resolver invocations)
- BC-4.12.005 — composes with (context-injection merging contract — defines how resolver outputs are merged into `plugin_config`)
- BC-1.12.001 — sibling (dispatcher startup and registry loading — this BC extends startup with the resolver registry step; PC-10 `log_dir` in `dispatcher.started` is also anchored here because BC-1.12.001 contracts that lifecycle events route to `events-*.jsonl`)
- BC-1.01.004 — sibling (same path-join contract for `hooks-registry.toml` implemented in `registry.rs::resolve_plugin_paths`; INV-8 is the resolvers-registry analogue — the implementer MUST mirror the proven precedent from BC-1.01.004 rather than hand-rolling a divergent join)

## Architecture Anchors

- `crates/factory-dispatcher/src/resolver.rs` — ContextResolver trait, ResolverRegistry, ResolverInput, ResolverOutput, ResolverError types
- `crates/factory-dispatcher/src/resolver_loader.rs` — WASM module compilation + mtime-cache; **INV-8 change site**: `load_registry` (parameter: `path: &Path`) MUST use `let toml_parent = path.parent();` then the normative `if let Some(base) = toml_parent { if entry.plugin.is_relative() { entry.plugin = base.join(&entry.plugin); } }` pattern (mirroring `registry.rs::resolve_plugin_paths` `if let Some(base) = path.parent()` for precedent-consistency and intent-clarity — see INV-8 for full rationale; the guard is NOT a Windows-portability fix vs bare join, as both forms are behaviorally identical; `None` arm passes `entry.plugin` through unchanged); the path-join MUST precede the SINGLE production `get_or_compile` call (the `fail_closed: true` / `fail_closed: false` divergence is in the post-call error `match`, not at separate call sites; TD-VSDD-060 sibling-sweep confirms no second production call site — the second `get_or_compile` occurrence is inside the `#[cfg(test)]` module); the resolved absolute path then feeds `path.canonicalize()`
- `crates/factory-dispatcher/src/executor.rs` — pre-dispatch resolver invocation step (between registry lookup and invoke_plugin)
- `crates/factory-dispatcher/src/registry.rs` — RegistryEntry.needs_context field (`#[serde(default)]`)
- `crates/factory-dispatcher/src/main.rs` — **PC-10 change site**: the `InternalEvent::now(DISPATCHER_STARTED)` builder chain emitted via `internal_log.write(...)` MUST include a `.with_field("log_dir", ...)` call populated from `std::path::absolute(internal_log.log_dir()).unwrap_or_else(|_| internal_log.log_dir().to_path_buf())` — absolutization happens HERE at the emission site, NOT inside `InternalLog::log_dir()` (the accessor stays verbatim); functional anchors: `std::path::absolute`, `DISPATCHER_STARTED`, `InternalLog::log_dir()` per ADR-024 §Decision 5 (TD-VSDD-091 function-name anchors). Also the startup entry point that calls `load_registry` where INV-8 path resolution begins. **INV-8 change site**: `load_registry` must use `path.parent()` (not a non-existent `toml_path` binding) with the normative `if let Some(base) = toml_parent` pattern (mirroring `registry.rs::resolve_plugin_paths` for precedent-consistency and intent-clarity) — see INV-8 for full rationale.
- `plugins/vsdd-factory/resolvers-registry.toml` — resolver registration file (distinct from hooks-registry.toml)
- `.factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md` — design decision (OD-1 through OD-6)
- `.factory/specs/architecture/decisions/ADR-024-dispatcher-log-dir-resolution-and-plugin-root-fail-loud.md` — §Decision 1 Addendum (TOML-parent-relative path resolution) and §Decision 5 (`log_dir` in `dispatcher.started`)

## Story Anchor

S-12.03 (ContextResolver trait + ResolverRegistry in-memory) and S-12.04 (WASM resolver loading + lifecycle) — v1.0-feature-engine-discipline-pass-1 F3-amendment decomposition. S-18.14 (resolver WASM path-resolution fix + log_dir observability) — S-18 engine-discipline wave.

## VP Anchors

- VP-073 — Resolver-load purity
- VP-074 — Resolver-error isolation
- VP-075 — Context-injection determinism

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.12 | 2026-06-22 | S-18.14 pass-18 fix burst (F-1 MAJOR POLICY 5): INV-8 phantom-signature corrected — `pub fn load_registry(&self, path: &Path, ...)` implied additional parameters via trailing `, ...`; ground truth (`resolver_loader.rs:281-284`): exactly ONE parameter `path: &Path`; corrected to `pub fn load_registry(&self, path: &Path) -> Result<...>` (exact one-param list; `Result<...>` return elision). Phantom-paramlist sweep `grep -n ', \.\.\.)' BC-1.13.001.md` stdout: zero normative `, ...`-in-paramlist occurrences remaining. KK-N 5-leg parity: frontmatter version, last_amended, modified array entry, Changelog row, INV-8 body — all updated. |
| 1.11 | 2026-06-22 | S-18.14 pass-16 fix burst (F-1 MAJOR POLICY 19): Architecture Anchors line-294 inverse-word-order version-pin fixed — `v1.3 Decision 1 Addendum (TOML-parent-relative path resolution) and Decision 5` → `§Decision 1 Addendum (TOML-parent-relative path resolution) and §Decision 5` (stable form, no version token). The v1.10 sweep regex missed this because it only matched version-AFTER-keyword order (`ADR-NNN v1.x`), not inverse order (`v1.x Decision`). Broad dual-word-order sweep `grep -nE 'v[0-9]+\.[0-9]+ *(Decision|Addendum|§)\|(ADR-[0-9]+|§Decision [0-9]+|Decision [0-9]+) *v[0-9]+\.[0-9]+\|ADR-[0-9]+ v[0-9]'` stdout confirms zero residual normative version-pins in either word-order; all other hits are in exempt sections (modified: array, last_amended:, Changelog table rows). Corrects false v1.10 attestation of "zero remaining normative version-pins." KK-N 5-leg parity: frontmatter version, last_amended, modified array entry, Changelog row, Architecture Anchors body — all updated. |
| 1.10 | 2026-06-22 | S-18.14 confirmatory-pass-13 fix burst (F-1 MAJOR POLICY 19 / TD-VSDD-091): volatile ADR version-pins removed from normative body. PC-10 body: `§Decision 5 v1.7` → `§Decision 5` (stable form). Architecture Anchors `main.rs` entry: `ADR-024 §Decision 5 v1.7` → `ADR-024 §Decision 5`. Exhaustive POLICY 5/19 sibling-sweep via `grep -nE 'ADR-[0-9]+ v[0-9]|§Decision [0-9]+ v[0-9]'` confirms zero remaining normative version-pins; all remaining grep hits are in exempt sections (modified: array, last_amended:, Changelog table rows). Traceability ADR Reference row already used stable form — unaffected. |
| 1.9 | 2026-06-22 | S-18.14 pre-ready hardening burst: (A-1) Phantom-`toml_path` anchor fixed — `load_registry` actual parameter is `path: &Path`, not `toml_path`; INV-8 and Architecture Anchors now use `path.parent()` with `let toml_parent = path.parent()` alias, matching ADR-024 v1.9 and `registry.rs::resolve_plugin_paths` precedent `if let Some(base) = path.parent()`. (A-2) `parent()==None` arm made explicit in INV-8 normative pattern: `if let Some(base) = toml_parent { if entry.plugin.is_relative() { entry.plugin = base.join(&entry.plugin); } }` with `None` arm passing `entry.plugin` through unchanged — mirrors the BC-1.01.004 / `registry.rs` pattern exactly. (A-4) Volatile line-pins removed from INV-8 normative body per TD-VSDD-091: `~line 361` and `line 1057` struck; replaced with function-name anchors and stable description "the second `get_or_compile` occurrence inside the `#[cfg(test)]` module". (p10-O1) VP-073 Property description aligned to VP-INDEX verbatim: "Resolver-Load Purity — resolver WASM module loading must be pure: same registry file always produces same resolver set, no side effects"; prior BC paraphrase superseded; VP-074 and VP-075 sibling-sweep: descriptions updated to VP-INDEX verbatim with `[VP-INDEX verbatim paraphrase]` label per POLICY 9. EC-010 + PC-10 unchanged. |
| 1.8 | 2026-06-22 | S-18.14 fix-burst adversary pass-9 (F-1 BLOCKER POLICY 5 — factually-wrong rationale): INV-8 rationale corrected — v1.7 falsely claimed that `is_relative()` returns `false` for rooted-but-not-absolute Windows paths (`\foo`), making it "the only guard that correctly identifies all paths that should pass through unchanged." This is inverted: `is_relative()` ≡ `!is_absolute()` in Rust, and `\foo` on Windows has `is_absolute()=false` → `is_relative()=true`. Consequently, a bare `PathBuf::join` and an explicit-`is_relative()`-guarded join are **behaviorally identical** for all path forms including `\foo` (join replaces the base in both cases). The correct justification for the explicit guard is: (a) **precedent-consistency** with `registry.rs::resolve_plugin_paths` (the proven sibling call site that uses the same explicit guard; divergent code paths for the same semantic operation are a maintenance risk); (b) **intent-clarity** (leaves a genuinely-absolute PathBuf unchanged rather than re-routing through join, making the intent readable). INV-8 body rewritten with correct Rust semantics and correct justification. Architecture Anchors `resolver_loader.rs` and `main.rs` INV-8 references updated to drop false Windows-portability rationale. Changelog v1.7 entry annotated with correction note. EC-010 idempotent-absolute-passthrough cross-ref to BC-1.01.004 EC-001/EC-002 preserved and unaffected. PC-10 unaffected. Matches ADR-024 v1.8 corrected wording. |
| 1.7 | 2026-06-22 | S-18.14 fix-burst adversary pass-7 (F-1 + O-3→normative): (F-1) PC-10 absolute-path guarantee made satisfiable — `log_dir` field value is absolutized at `DISPATCHER_STARTED` emission time in `main.rs` via `std::path::absolute(internal_log.log_dir()).unwrap_or_else(|_| internal_log.log_dir().to_path_buf())`; `InternalLog::log_dir()` accessor stays verbatim (absolutization NOT in accessor); per ADR-024 §Decision 5 v1.7; functional anchors: `std::path::absolute`, `DISPATCHER_STARTED`, `InternalLog::log_dir()`. PC-10 canonical test vector split into two rows: CWD-accessible (emits absolute path) and CWD-inaccessible (verbatim fallback). Architecture Anchors `main.rs` entry updated with emission-site absolutization details. (O-3→normative) INV-8 explicit `is_relative()` guard made normative: implementer MUST use `entry.plugin.is_relative()` before `toml_path.parent().join(...)` (mirroring `registry.rs::resolve_plugin_paths`) rather than relying solely on `PathBuf::join` absolute-replacement — Windows rooted-but-not-absolute path (`\foo`) causes `is_absolute()=false` yet `PathBuf::join` replaces base, so `is_relative()` is the only portable discriminant; Windows is a release target. Architecture Anchors `resolver_loader.rs` entry updated. EC-010 idempotent-absolute-passthrough cross-ref to BC-1.01.004 EC-001/EC-002 preserved. **[CORRECTION: The v1.7 INV-8 Windows rationale contains a factual error — `is_relative()` is NOT false for `\foo`; it equals `!is_absolute()` and returns true for rooted-but-not-absolute paths. Corrected in v1.8.]** |
| 1.6 | 2026-06-22 | S-18.14 fix-burst adversary pass-4 (F-1): VP-073 proof-method token in §Verification Properties corrected from `unit-test (integration test of resolver module compilation)` to authoritative `integration (resolver module compilation test) [non-authoritative annotation]` per VP-INDEX Full Index row 408 and Proof Method Breakdown (POLICY 9 VP-INDEX-SoT). This is the missed sibling of pass-1's VP-074 `kani`→`kani-proof` fix. VP-075 sibling-sweep: base token `proptest` in `proptest (200 trials, 5s timeout)` matches VP-INDEX `proptest` — no change required. |
| 1.5 | 2026-06-22 | S-18.14 fix-burst adversary pass-2 (F-1/F-2): (F-1) INV-8 single-call-site correction — false "two call sites / both fail_closed arms as separate call sites" claim replaced with ground truth: exactly ONE production `get_or_compile` call site in `load_registry`; `fail_closed` divergence is in the post-call error `match`, not at separate call sites; TD-VSDD-060 sibling-sweep confirms `resolver_loader.rs` line 1057 is inside `#[cfg(test)]`. Architecture Anchors `resolver_loader.rs` entry updated to match. (F-2) BC-1.01.004 ("Relative plugin paths resolve against registry file's parent directory") added to §Related BCs as sibling — same path-join contract for `hooks-registry.toml` via `registry.rs::resolve_plugin_paths`; INV-8 is the resolvers-registry analogue. EC-010 extended with idempotent-absolute-passthrough cross-reference citing BC-1.01.004 EC-001 and EC-002 verbatim. |
| 1.4 | 2026-06-22 | S-18.14 fix-burst adversary pass-1 (F-1/F-2/F-3/F-6): (F-1) Architecture Anchors §PC-10 change site: replaced phantom symbol `InternalLog::write_started` (non-existent) with correct anchor — `InternalEvent::now(DISPATCHER_STARTED)` builder chain emitted via `internal_log.write(...)` in `main.rs`; `InternalLog::log_dir()` accessor confirmed at `internal_log.rs` `pub fn log_dir(&self) -> &Path` (line 324). (F-2) S-18.14 added to all three story-anchor sites: §Traceability Stories row, §Story Anchor, and BC-INDEX body cell. (F-3) ADR version tokens `v1.3` dropped from §Traceability ADR Reference cites — `ADR-024 §Decision 1 Addendum (Resolver WASM plugin path resolution)` and `ADR-024 §Decision 5` (POLICY 19). (F-6) VP-074 proof-method token aligned from bare `kani` to `kani-proof` per VP-INDEX authoritative token; non-authoritative `+ integration test (trap injection)` annotation retained explicitly labeled. |
| 1.3 | 2026-06-22 | S-18.14 spec-evolution (D-676 / ADR-024 v1.3): (1) INV-8 — Resolver WASM path resolution base MUST be TOML file's parent directory (`CLAUDE_PLUGIN_ROOT` at runtime), NOT process CWD; absolute paths pass through unchanged; applies to ALL `get_or_compile` call sites in `resolver_loader`; root cause of 8,560 `resolver.load_error` / 0 successful loads since rc.21. (2) PC-9 — Successful load when artifacts present at TOML-parent-relative paths; zero `resolver.load_error` for any declared resolver is the spec. (3) PC-10 — `dispatcher.started` event payload MUST include `log_dir` string field from `InternalLog::log_dir()`; unconditional; non-empty; absolute path (ADR-024 Decision 5). Placed here because no dedicated `dispatcher.started`-payload BC exists in the SS-01 catalog; see PC-10 placement note for migration guidance. (4) EC-010 — Relative WASM exists at PLUGIN_ROOT but not CWD → must load successfully. (5) Architecture Anchors updated: path-anchors migrated from absolute user-local paths to repo-relative paths (TD-VSDD-091); `resolver_loader.rs` INV-8 change site and `main.rs` PC-10 change site documented. (6) ADR Reference extended with ADR-024 v1.3 §Decision 1 Addendum and §Decision 5 cites. |
| 1.2 | 2026-05-10 | Pass-4 fix-burst: canonical key wave-context → wave_context per BC-4.12.005 PC7 / S-12.07 v1.2 / ADR-018. EC-004 and Canonical Test Vectors truth table (rows 150-154) updated to use underscore form throughout. Added missing `extracted_from: null` frontmatter field (greenfield artifact). |
| 1.1 | 2026-05-09 | F-P45-001 — Traceability Stories row propagated from BC-INDEX v1.57: S-12.03, S-12.04 → S-12.03, S-12.04, S-12.06, S-12.08. BC-INDEX was updated in fix-burst-39 (v1.55) to add S-12.06 + S-12.08; body was not updated in that burst. Refs: F-P45-001, fix-burst-42. |
| 1.0 | 2026-05-07 | Initial authoring (product-owner; F2-amendment phase of v1.0-feature-engine-discipline-pass-1). Encodes architectural decisions OD-1 through OD-6 (user-authorized per D-361). PC1 Critical Constraint explicitly states "absent resolvers-registry.toml = zero resolvers, NOT a startup error" per orchestrator directive and F1-amendment R-PLAT-005 regression risk mitigation. Factory-agnostic dispatcher invariant encodes D-361 generality requirement. |
