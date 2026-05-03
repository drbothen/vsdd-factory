---
document_type: adr
adr_id: ADR-014
status: proposed
date: 2026-05-03
subsystems_affected: [SS-02, SS-04, SS-07]
supersedes: null
superseded_by: null
---

# ADR-014: Tier 2 Native WASM Migration (W-16) — rewrite-clean Port Strategy + host::run_subprocess ABI

## Amendment 2026-05-03: D-9.2 withdrawn (gap analysis)

- **Pre-decision:** "(a) Add new host::run_subprocess ABI" — accepted by user when ADR-014 was authored
- **Amendment date:** 2026-05-03
- **Reason:** Post-decision audit discovered `host::exec_subprocess` already in production (BC-1.05.001..034, SS-01 cluster; used by `session-start-telemetry` per BC-4.04.002). Gap analysis (`.factory/architecture/gap-analysis-w16-subprocess.md`) confirms existing `exec_subprocess` is sufficient for the only W-16 use case (`validate-wave-gate-prerequisite` invoking `verify-sha-currency.sh`). The gap analysis Section 5 "fundamentally insufficient" list is **empty** for the W-16 use case: binary path arg, multiple args, shell_bypass gate, env forwarding, timeout, and exit-code capture are all present and production-verified. The two headline features unique to `run_subprocess` (glob binary_allowlist, arg_allowlist, truncated-as-Ok, per-stream caps) are not required by the use case.
- **Decision:** D-9.2 withdrawn. W-16 uses existing `host::exec_subprocess`. Two minor additive extensions to BC-1.05.035 (path traversal guard) + BC-1.05.036 (success telemetry) in SS-01 cover defense-in-depth gaps: (1) path traversal guard on binary arg, (2) success-path telemetry event. These fit within S-9.07's scope.
- **Consequences:**
  - BC-2.02.013 withdrawn (preserved as audit trail with `lifecycle_status: withdrawn`)
  - SS-02 `host::run_subprocess` section marked WITHDRAWN
  - S-9.30 withdrawn from W-16 scope (story_count 9→8)
  - S-9.07 (`validate-wave-gate-prerequisite` WASM port) uses `exec_subprocess` capability directly
  - `run_subprocess` never built; no FFI extern, no SDK wrapper, no `SubprocessCaps` schema
  - Bundle-size delta: -1 plugin (positive impact; smaller bundles)
  - New exec_subprocess extensions: BC-1.05.035 + BC-1.05.036 (NEW 2026-05-03)
- **Cost:** ~2 engineering-days saved; 922+ lines of new spec eliminated
- **Authority:** gap analysis Section 7 + user approval 2026-05-03

> **Correction 2026-05-03:** Initial gap-analysis incorrectly identified
> BC-2.02.005 as the exec_subprocess BC. BC-2.02.005 actually documents the
> SDK `read_string` re-call protocol. The exec_subprocess BCs live in SS-01
> cluster (BC-1.05.001..034). The 2 minor additive extensions are authored
> as NEW BC-1.05.035 (path traversal guard) and BC-1.05.036 (success telemetry).

---

## Amendment 2026-05-03: R-8.09 ceiling model revised (research)

- **Pre-decision:** R-8.09 mandated ≤25% bundle growth vs v1.0.0 GA dispatcher binary. Set when only Tier 1 was envisioned; the Tier 2/3 scope was not yet clarified.
- **Amendment date:** 2026-05-03
- **Reason:** Research (`.factory/research/W-16-spec-foundation-research.md` Q3) found wasmtime cold-start latency is decoupled from module size at our scale. Industry comparables (Lapce, Zellij, Spin, Cloudflare Workers, Fermyon) routinely tolerate 10-50MB bundles. Cranelift compiles ~1-3ms/MB; AOT pre-compilation reduces this to sub-ms. Bundle size matters for distribution speed (clawhub download), not runtime performance. The original 25% ceiling was an uninformed conservative bound that would block W-16 even when cold-start budget is fully satisfied.
- **Revised decision (replaces R-8.09):** Latency-primary gate + bundle-size advisory + hard kill-switch:
  - **Primary gate (HARD):** cold-start p95 ≤ 500ms (inherited from S-8.00 / E-8 R-8.08; note: original amendment cited R-8.10 in error — E-8 v1.10 risk table is the source of truth; R-8.08 is "Cumulative WASM startup overhead" with AC-7b ceiling = 500ms p95).
  - **Advisory soft cap:** cumulative bundle growth ≤ 100% vs pre-W-15 baseline at end of W-17 (~14MB target).
  - **Hard kill-switch:** cumulative bundle ≤ 30MB. Crossing requires fresh project-level architecture review. Rationale: at 30MB on 5 platforms, distribution payload reaches ~150MB total — the threshold at which package-manager users notice download time on slow connections.
  - **Required telemetry per wave:** publish `(bundle_size_delta_bytes, cold_start_p95_delta_ms)`. Pause wave if cold-start regresses >10%.
- **Consequences:**
  - S-9.00 deliverables updated: latency telemetry is the primary gate; bundle size is advisory measurement
  - W-16 unblocked even if bundle delta > 25% (would have blocked under old R-8.09)
  - W-17 has explicit headroom up to 30MB hard cap
  - E-9 (W-16) and future E-10 (W-17) reference this revised model; original R-8.09 in E-8 (closed) stays as written
- **Authority:** research findings 2026-05-03 + user approval

---

## Context

W-15 (Tier 1) shipped the first wave of native WASM hook ports under epic E-8. By
rc.4 (v1.0.0-rc.4, 2026-04-25), all Tier 1 hooks had been ported and the
`legacy-bash-adapter` continued routing the remaining unported bash hooks.

**Tier 2 scope:** 23 `validate-*.sh` hooks currently routed through
`legacy-bash-adapter.wasm` in `hooks-registry.toml` (lines 145–797). These hooks
implement the factory's structural and behavioral validation layer: frontmatter
compliance, BC traceability, VP consistency, wave-gate prerequisites, story/state
coherence checks. They represent the majority of the remaining bash hook surface
that blocks Windows compatibility (DRIFT-010) and introduces subprocess overhead on
every tool-use event.

**W-16 objective:** Port all 23 validate hooks from bash to native WASM (Rust +
`vsdd-hook-sdk`), eliminating the `legacy-bash-adapter` routing for this hook class
and moving toward the Phase H end-state (v1.3.0: zero bash hooks, adapter deleted).

**Pre-work:** The architect produced `audit-w16.md` (2026-05-03) surveying all 23
hooks across capability requirements, subprocess needs, and behavioral complexity.
The audit recommended D-9.1=rewrite-clean, D-9.2=skip new host fns, D-9.3=7
batched stories. The orchestrator reviewed the audit and accepted D-9.1 and D-9.3
but overrode D-9.2 on the grounds that the SHA-currency check in
`validate-wave-gate-prerequisite` is critical defense-in-depth and must remain
native in W-16, and that a generalizable `host::run_subprocess` ABI is the correct
long-term approach for Tier 3+ hooks (see D-9.2 decision and rationale below).

---

## Decision

### D-9.1 — Port Strategy: rewrite-clean

All 23 Tier 2 hooks are ported as idiomatic Rust using `regex`, `serde_json`,
`serde_yaml`, and `walkdir`/`std::fs` where appropriate. No 1:1 bash-to-Rust
translation is performed. Bash-specific idioms (awk field-splitting, sed
substitution, jq pipelines, shell quoting behavior) are replaced with their
semantically equivalent Rust stdlib or crate-provided counterparts.

**Rationale:**

1. **20/23 hooks are trivial pipelines.** The audit's complexity distribution shows
   9 hooks in the "55–100 lines, jq + grep only" tier and 10 hooks in the
   "100–160 lines, jq + awk + multi-file" tier. In both tiers, every external tool
   (jq, awk, sed, grep, tr) is cleanly replaceable with `serde_json`, `serde_yaml`,
   `regex`, and Rust string operations. No behavioral complexity demands port-as-is.

2. **W-15 OQ-001 lesson (S-8.04).** The `update-wave-state-on-merge.sh` hook
   contained a bash ERE quirk (`|` lower precedence than concatenation) that was
   preserved as a deliberate choice in W-15, filed as TD for v1.2. This is the
   canonical "preserved-quirk" case: port-as-is translates a known behavioral defect
   into the Rust implementation, and the bats test suite (which tests the `.sh` file,
   not the WASM) provides a false green. Rewrite-clean forces explicit documentation
   of expected behavior, making WASM tests the ground truth.

3. **jq absence causes silent pass in all 23 bash hooks.** Every validate hook opens
   with `if ! command -v jq &>/dev/null; then exit 0; fi`. A native WASM plugin has
   no such fragility — stdin is deserialized to `HookPayload` by the SDK, eliminating
   the silent-skip class of failure.

4. **Expected line-count reduction: ~40–50%.** Audit estimates bash averages 143 lines
   per hook; Rust equivalents 80–120 lines. Complex hooks (vp-consistency at 248 lines)
   reduce to ~150 lines.

5. **Bats test suite tests bash, not WASM.** After porting, `.sh` files remain on
   disk until Phase H (R-W16-001). Bats tests continue passing on the `.sh` files
   but do not cover the WASM execution path. Rewrite-clean requires each story spec
   to document all behavioral edge cases as ACs, making the WASM Rust integration
   tests the authoritative behavioral contract.

**Mitigation for rewrite-clean risk:** Each story spec (S-9.01..S-9.07) must list
every behavioral edge case from the bash original as explicit Acceptance Criteria
(mirroring S-8.04's EC tables for YAML trichotomy, TOCTOU, and OQ-001). The
adversarial review discipline per ADR-013 surfaces behavioral divergences before
implementation.

---

### D-9.2 — Subprocess Capability ~~(SUPERSEDED 2026-05-03 — see Amendment above)~~

A new host function `host::run_subprocess` is added to the `vsdd-hook-sdk` ABI,
anchored by BC-2.02.013 (authored by PO). This function exposes a capability-gated
subprocess invocation path governed by a `SubprocessCaps` schema that enforces a
binary+arg allow-list, no shell interpretation, no path traversal, and output/timeout
bounds.

**Note:** The audit's D-9.2 recommendation was (c) — skip new host fns, simplify the
two optional subprocess calls away. The user overrode this recommendation. The
decisions documented here reflect the user-approved override.

**Rationale for override:**

1. **SHA-currency check is critical defense-in-depth.** `validate-wave-gate-prerequisite`
   calls `verify-sha-currency.sh` via subprocess to verify the SHA currency of the
   adversary gate. The audit classified this as "optional/graceful-skip" because the
   bash hook guards the call with `if [[ -z "$SHA_HOOK" ]]; then exit 0; fi`. However,
   the user confirmed this check is a critical security property: if `verify-sha-currency.sh`
   is present, it MUST run. Dropping it silently in the WASM port would degrade the
   security posture of the wave-gate enforcement. The WASM port must preserve the
   `verify-sha-currency.sh` invocation as native subprocess capability.

2. **Generalizable for Tier 3+ hooks.** Audit Section 1 shows that Tier 3 "specialty"
   hooks (11 hooks, deferred to W-17) include hooks with genuine subprocess dependencies
   that cannot be simplified away. Adding `host::run_subprocess` in W-16 amortizes the
   BC authoring, dispatcher implementation, and security review overhead across all
   future tiers rather than repeating the work in W-17.

3. **Capability schema mirrors the WriteFileCaps pattern (W-15, S-8.10).** The W-15
   cycle established the `WriteFileCaps` pattern (binary path allow-list, deny-by-default,
   explicit capability declaration in `hooks-registry.toml`). `SubprocessCaps` follows
   the same structural pattern: an allow-list of permitted binaries (glob), an allow-list
   of permitted argument prefixes (glob), an env allowlist, and output/timeout bounds.
   No shell interpretation; no path traversal. This is the established security model
   for host fn capability gates in this project.

4. **HOST_ABI_VERSION stays at 1.** `host::run_subprocess` is an additive host function.
   Following the precedent established by D-6 Option A (`host::write_file`, S-8.10) and
   D-183 (`HookPayload` SubagentStop fields, S-8.30), additive-only extensions leave
   `HOST_ABI_VERSION = 1` unchanged. Existing plugins compiled against the current SDK
   continue to function; they simply do not use the new function.

**SubprocessCaps schema:**

```rust
pub struct SubprocessCaps {
    /// Glob patterns for permitted binary paths.
    /// Example: ["/usr/bin/git", "*/verify-sha-currency.sh"]
    /// NO shell interpreter (bash, sh, zsh) except via explicit entry.
    pub binary_allowlist: Vec<String>,

    /// Glob patterns for permitted argument strings.
    /// Example: ["--version", "show*", "HEAD:*"]
    pub arg_allowlist: Vec<String>,

    /// Environment variable names the subprocess may inherit.
    /// All other env vars are stripped before exec.
    pub env_allowlist: Vec<String>,

    /// Maximum bytes captured from stdout. Default: 1_048_576 (1 MiB).
    pub max_stdout_bytes: u64,

    /// Maximum bytes captured from stderr. Default: 262_144 (256 KiB).
    pub max_stderr_bytes: u64,

    /// Maximum wall-clock milliseconds for subprocess execution.
    /// Default: 30_000 (30 seconds).
    pub max_timeout_ms: u64,
}
```

**Security boundaries enforced by dispatcher:**
- Binary path is resolved to an absolute path; no shell glob expansion is performed
  by the dispatcher. The `binary_allowlist` patterns are matched against the resolved
  absolute path.
- `args` are passed directly to `execvp`-equivalent; no shell interpretation (`/bin/sh -c`)
  is ever used in the execution path.
- Path traversal in the binary field (`../../../bin/evil`) is rejected: the resolved
  path must match at least one `binary_allowlist` pattern or the call is denied with
  `HostError::CapabilityDenied`.
- Env vars not in `env_allowlist` are removed from the subprocess environment before
  exec. An empty `env_allowlist` means the subprocess inherits zero env vars.

---

### D-9.3 — Story Granularity: 7 Capability-Cluster Batches + S-9.00 + SDK Extension Story (~9 stories)

Stories follow the capability-cluster batching scheme from the audit, with two
additions: an S-9.00 perf baseline story and an SDK extension story for
`host::run_subprocess` dispatcher implementation (analogous to S-8.10 + S-8.30 for W-15).

**Story sequence:**

| Story | Scope | Dependencies |
|-------|-------|--------------|
| S-9.00 | Perf baseline: measure WASM bundle size before first W-16 port; set W-16 bundle ceiling | none |
| SDK-ext | Implement `host::run_subprocess` in dispatcher + SDK shim (BC-2.02.013); analogous to S-8.10 | BC-2.02.013 authored by PO |
| S-9.01 | Batch 1: validate-demo-evidence-story-scoped, validate-factory-path-root, validate-finding-format, validate-novelty-assessment | SDK-ext |
| S-9.02 | Batch 2: validate-bc-title, validate-changelog-monotonicity, validate-red-ratio, validate-input-hash | SDK-ext |
| S-9.03 | Batch 3: validate-pr-description-completeness, validate-table-cell-count, validate-pr-merge-prerequisites | SDK-ext |
| S-9.04 | Batch 4: validate-state-index-status-coherence, validate-state-pin-freshness, validate-state-size | SDK-ext |
| S-9.05 | Batch 5: validate-story-bc-sync, validate-count-propagation, validate-index-self-reference | SDK-ext |
| S-9.06 | Batch 6: validate-anchor-capabilities-union, validate-subsystem-names, validate-template-compliance | SDK-ext |
| S-9.07 | Batch 7: validate-vp-consistency, validate-wave-gate-completeness, validate-wave-gate-prerequisite | SDK-ext (run_subprocess needed for S-9.07) |

**Capability profile per batch (host functions needed):**
- S-9.01: `emit_event`, `log` (no file reads; stdin-parse + path-check only)
- S-9.02–S-9.06: `read_file`, `emit_event`, `log`
- S-9.07: `read_file`, `run_subprocess`, `emit_event`, `log`

**Batching rationale:** Capability clustering produces uniform BC anchoring patterns
per story, enabling focused adversarial review. The "9-hook frontmatter" behavioral
category from audit Section 1 is split across batches by capability profile rather
than grouped monolithically. The 9-hook "frontmatter" class would exceed the balanced
scope target (3–4 hooks per story) if grouped as a single story.

**S-9.00 rationale:** Analogous to S-8.00 for W-15 (E-8). Bundle size tracking is
required given the R-W16-003 risk that 23 new WASM plugins may exceed the current
R-8.09 25% ceiling. S-9.00 measures the baseline and sets the W-16-specific ceiling
before any plugin lands.

**SDK extension story rationale:** `host::run_subprocess` requires dispatcher-side
implementation (ABI registration, capability enforcement, exec logic) and SDK-side
shim (`extern "C"` wrapper + `SubprocessResult` type), analogous to S-8.10
(`host::write_file`). This story is sequenced before S-9.01..S-9.07 so all batch
stories can declare `run_subprocess` capability in their TOML capability blocks and
use the shim from day one.

---

## Consequences

### Positive

- All 23 Tier 2 validate hooks land in W-16 with native WASM execution, eliminating
  bash subprocess overhead for this hook class.
- `verify-sha-currency.sh` invocation is preserved in the W-16 WASM port via
  `host::run_subprocess`, maintaining the SHA-currency defense-in-depth check.
- `host::run_subprocess` is available to Tier 3+ hooks (W-17) with no additional
  ABI or dispatcher work; the capability schema is generalizable to any binary
  the plugin declares in its allow-list.
- `HOST_ABI_VERSION = 1` is maintained. No plugin migration required.
- DRIFT-010 Windows compatibility gap shrinks by 23 hooks after W-16.

### Negative / Trade-offs

- **R-W16-001: bats orphan migration deferred to Phase H.** After W-16, `.sh` files
  remain on disk and bats tests continue to pass on the bash execution path, not
  the WASM path. Each story spec must include a task to create a bats deletion
  checklist and file a TD per hook. Phase H (v1.3.0) removes both adapter and `.sh`
  files.
- **R-W16-003: bundle size ceiling.** 23 new WASM plugins estimated at 4.6–6.9 MB
  may exceed the current R-8.09 25% growth ceiling (~9.0 MB). S-9.00 measures the
  actual baseline and sets a W-16-specific ceiling. The E-9 epic should propose
  revising R-8.09 to a per-wave ceiling or raising the absolute cap.
- **BC-2.02.013 PO work.** `host::run_subprocess` requires PO to author BC-2.02.013
  (SubprocessCaps invariants, dispatcher security guarantees, `SubprocessResult`
  typed contract). This adds adversarial convergence work before the SDK extension
  story can begin.
- **Security review for run_subprocess.** Following OQ-6 pattern (S-8.09 precedent),
  the SDK extension story requires a security review of the dispatcher's
  `run_subprocess` implementation (binary resolution, arg passthrough, env stripping).

### Audit Risk Items Carried Forward

- **R-W16-002 (WASI preopens):** 19 of 23 hooks read `FILE_PATH`. Canonical capability
  is `path_allow = [".factory/"]` for spec-file readers; `path_allow = ["."]` for
  hooks that may read files outside `.factory/`. Each story spec must pin per-hook
  path_allow declarations.
- **R-W16-004 (bats/WASM test infrastructure):** Each story spec includes a WASM
  integration test task (Rust `factory-dispatcher/tests/`) and defers bats migration
  to Phase H, citing the TD-020 class problem.

---

## Alternatives Considered

### D-9.1 alternatives

- **port-as-is:** 1:1 bash-to-Rust translation preserving all awk/sed/jq behavior.
  Rejected: W-15 OQ-001 (S-8.04) established that preserved bash quirks produce
  correct-looking bats results (testing `.sh` file) while the WASM port behaves
  differently. Port-as-is in a clean language (Rust) without explicit quirk
  documentation is a latent correctness risk. The adversarial spec discipline of
  ADR-013 can catch divergences only if rewrite-clean is chosen (because
  rewrite-clean requires explicit AC tables for every behavioral edge case).
- **hybrid (rewrite-clean for simple, port-as-is for complex):** 20/23 hooks qualify
  as "simple" by the audit's criteria. The 3 complex hooks (vp-consistency,
  wave-gate-prerequisite, wave-gate-completeness) have no documented bash quirks
  beyond the YAML parsing (replaceable with `serde_yaml`). Hybrid provides
  complexity without benefit.

### D-9.2 alternatives

- **Audit recommendation (c) — skip new host fns:** The audit recommended dropping
  both optional subprocess calls and simplifying all 23 hooks to use only the existing
  ABI. This is technically sound but was overridden: (1) SHA-currency check is
  defense-in-depth that should be preserved, not dropped; (2) generalizing to
  `host::run_subprocess` now avoids repeating BC/dispatcher/security-review work in W-17.
- **Specific host fns (host::git_log, host::gh_pr_view):** Narrow, hook-specific
  functions. Rejected: over-engineered for two hooks; `host::run_subprocess` with a
  capability allow-list provides equivalent security with broader applicability.
- **23 individual stories:** Per-hook stories matching the W-15 single-hook pattern.
  Rejected: 23 × ~8 adversarial passes = ~184 passes; Tier 2 hooks are more uniform
  than Tier 1 (all use `read_file` + regex + `emit_event`) so per-hook granularity
  is overkill. Audit's 7-batch estimate is balanced.

---

## Implementation Notes

### SS-02 update (this ADR, task 2)
- Add `host::run_subprocess` function signature and `SubprocessResult`/`SubprocessCaps`
  types to SS-02-hook-sdk.md.
- Add BC-2.02.013 anchor row to SS-02 Schema Evolution table.
- Modules table: add `crates/hook-sdk/src/host/run_subprocess.rs` row.

### SS-04 update (this ADR, task 3)
- Add E-9 epic positioning entry: Tier 2 native WASM migration, W-16, ~9 stories,
  depends on E-8 (W-15) closure.
- Reference S-9.00 bundle ceiling and 25% W-16-specific cap.
- Document `legacy-bash-adapter` routing status: continues to handle unported hooks
  until Phase H; 23 entries disabled/removed after W-16 ship.

### BC-2.02.013 (PO authors next)
BC-2.02.013 defines the `host::run_subprocess` invariants:
- Preconditions: SubprocessCaps.binary_allowlist non-empty; binary resolved to
  absolute path matching at least one allowlist pattern.
- Postconditions: `SubprocessResult.exit_code` reflects actual process exit status;
  `SubprocessResult.truncated = true` if stdout or stderr exceeded cap; env vars
  not in `env_allowlist` are absent from subprocess environment.
- Invariants: no shell interpretation; no path traversal; timeout enforced;
  `HOST_ABI_VERSION` stays 1.

### Story sequence
`S-9.00` → `SDK extension story` → `S-9.01..S-9.07` (in capability-cluster order).
Each story follows the W-15 per-story-delivery cycle pattern: test-writer RED gate +
implementer GREEN + demo-recorder + pr-manager 9-step.

---

## Source / Origin

- **Audit:** `.factory/architecture/audit-w16.md` (2026-05-03) — hook inventory,
  capability matrix, D-9.1/D-9.2/D-9.3 analysis and recommendations.
- **User override:** Orchestrator/user approved D-9.1=rewrite-clean, D-9.3=7 batches,
  and overrode D-9.2 to (a) host::run_subprocess. Recorded in D-9 decision table
  (post-audit-w16.md).
- **W-15 precedents:** S-8.04 (OQ-001 bash ERE quirk), S-8.10 (`host::write_file`
  WriteFileCaps pattern), S-8.30 (additive ABI extension), S-8.00 (perf baseline).
- **ADR references:** ADR-006 (HOST_ABI_VERSION additive policy), ADR-012
  (legacy-bash-adapter multi-instance routing), ADR-013 (adversarial review structure).
