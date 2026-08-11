---
document_type: adr
level: L3
adr_id: ADR-043
version: "1.5"
title: "ADR-043: exec-subprocess capability sandbox — binary-allow load-time path resolution and guest-cmd substitution"
status: proposed
date: 2026-08-10
producer: architect
timestamp: 2026-08-10T00:00:00Z
deciders:
  - architect
  - human (ratification required per POLICY 22)
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
related_adrs:
  - ADR-031 (E-21 factory state data-loss hardening — validate-factory-path-staging is P0 plugin triggering this analysis)
  - ADR-025 (single-writer factory lock/lease; Decision 13 allocates NOT_FOUND=-5 as next compact-negative slot after INVALID_ARGUMENT=-4)
  - ADR-039 (validator failure policy — per-plugin failure_policy; governs which per-plugin `on_error` mode handles degraded-binary outcomes; outcome taxonomy basis for D-970 Cod-1 matrix)
  - ADR-024 (dispatcher log-dir resolution and plugin-root fail-loud — degraded-continue registry-load pattern precedent; demonstrates load to completion + per-plugin sentinel on unresolvable resource)
anchors:
  - SS-01
subsystems_affected:
  - SS-01
related_bcs:
  - BC-4.04.002 (session-start-telemetry: factory-health not found → factory_health="unknown"; session.started emitted normally; EC-001 governs expected fail-open; no amendment required)
  - BC-1.05.002 (v2.0: binary allow-list matching updated for resolved-absolute-path entries; Inv 1 rewritten; PO adjudication complete 2026-08-11; PENDING implementation of ADR-043 Decisions 1+3)
  - BC-1.05.004 (exec_subprocess refuses setuid/setgid on resolved binary path — aligns with Decision 3; PO adjudication complete 2026-08-11)
  - BC-1.05.028 (binary_allow_matches_basename — updated per BC-1.05.002 PO adjudication 2026-08-11; PENDING implementation)
  - BC-1.05.035 (v2.0: load-time trusted-prefix resolution; no-canonicalize normative; TOCTOU accepted residual risk; PO adjudication complete 2026-08-11)
last_amended: |-
  2026-08-11 (v1.5) — AMENDED (architect): nine blockers + four HIGH items from third
  fresh-context adversarial review.
  D-1 RESOLVED: Resolution algorithm SPLIT by PATH-presence. WITH-PATH plugins: user-PATH
  FIRST then trusted-prefix fallback (preserves today's effective behavior; prevents bash 3.2
  downgrade for the 37 adapter plugins). WITHOUT-PATH plugins: trusted-prefix only (no user-PATH
  fallback; scoped correctly to declared trust boundary). Shadowing concern applies only to
  WITHOUT-PATH plugins. Rationale replaces trusted-prefix-first + advisory-fallback framing.
  D-2 RESOLVED: /home/linuxbrew paths gated with cfg(target_os="linux") (autofs on macOS
  triggers 12-17ms per stat; removed from macOS prefix list). Memoize per (binary_name,
  plugin_has_path) within one registry-load call: 91 entries → 12 distinct lookups max.
  Per-event cost restated with measured numbers (≈200µs worst-case macOS after both fixes).
  D-3 RESOLVED: binary_allowed returns AllowResult 3-way enum (Allowed/Denied/BinaryUnresolvable).
  run() matches all three arms; execute_bounded NEVER called on Denied or BinaryUnresolvable.
  CAPABILITY_DENIED(-1) for Denied; BINARY_NOT_FOUND(-6) for BinaryUnresolvable directly from
  run(). BC Precedence Ladder "execute_bounded additionally returns -6 for sentinel" is
  mechanically impossible — routed to team-lead for BC correction.
  A-1 RESOLVED: All seven sites asserting open PO adjudication updated. All four BCs
  adjudicated 2026-08-11 per team-lead report. Implementer dispatch no longer gated on BC
  adjudication. Two specific BC text corrections remain pending (D-3 Precedence Ladder; D-4
  is_shell claim).
  A-3 RESOLVED: Outcome count corrected to 8 (5 load-time + 3 spawn-time). C-10 added for
  post-spawn INTERNAL_ERROR(-99).
  A-4 RESOLVED: C-9 fixture corrected — previous fixture (git in /usr/bin → no effect for
  resolver test) was insufficiently specific. New C-9 uses resolver injection seam with a
  binary name absent from all trusted prefixes to test WITHOUT-PATH scoping invariant.
  A-6 RESOLVED: Resolver injection seam specified in Decision 1: resolve_binary_allow function
  signature with injectable trusted_prefixes and path_env parameters for testing.
  D-4 HIGH RESOLVED: Withdrew "is_shell fix" claim. For all reachable inputs, basename(resolved)
  == basename(cmd) by matching-algorithm construction, so is_shell(resolved) ≡ is_shell(cmd).
  Real residual disclosed: operator entry "/opt/tools/runner" (symlink to bash) bypasses shell
  gate; unclosable under no-canonicalize. BC-1.05.035 "./notbash" bypass claim is impossible;
  routed to team-lead for product-owner correction.
  D-5 HIGH RESOLVED: Withdrew no-trust-widening claim for WITHOUT-PATH P0 guards. 10-prefix
  list is strict superset of _CS_PATH by 6 dirs (2 user-writable). Decision 1 for P0 guards
  is either a no-op or a widening; never a fix without one of those. Restated in §Negative.
  D-6 HIGH RESOLVED: "silently" deleted from blast-radius severity column. BC-4.16.001
  Invariant 3 governs validate-factory-path-staging fail-open on crash/branch-detection
  failures (not on git-absent); these are different cases. Severity criterion clarified.
  C-1 HIGH RESOLVED: P0 severity restated as host-conditional. Zero WITHOUT-PATH plugins
  exhibit the defect on the authoring host (git at /usr/bin/git). Defect manifests on NixOS,
  mise-only, asdf-only, Nix user-level hosts.
  A-9: Removed fabricated "ADR-025 Decisions 5+6" anchor (grep confirms 0 hits for
  BINARY_NOT_FOUND in ADR-025; Decision 13 is the only valid anchor).
  A-10: Removed S-21.14 reference (story ceiling is S-21.13; S-21.14 does not exist); replaced
  with "next unallocated story ID after S-21.13, per story-writer".
  A-11: registry.rs grep command returns 15 (not 0); corrected — 15 hits are field declarations
  and test fixtures; conclusion unchanged.
  A-13: WITH-PATH decomposition corrected to 37 adapter + 2 non-adapter = 39 (not 28+7=35).
  A-14: session-start-telemetry has no on_error field (inherited from RegistryDefaults; event
  is "SessionStart", not a tool regex). capture-commit-activity has no tool field (fires on
  every PostToolUse event, not only Bash).
  A-15: All line-number pins removed per TD-VSDD-091.
  A-16: All three paraphrased doc comment quotes replaced with verbatim source text.
  A-17: Fourth false doc comment added (exec_subprocess.rs module doc "Binary basename must be
  in binary_allow" + "S-1.5 tightens a few corners (pre-resolved full paths in binary_allow)").
  A-18: §Downstream Routing updated from C-1–C-8 to C-1–C-10.
  A-21: last_amended history chain now reads in chronological order.
  §Alternatives Considered: "Reject any guest cmd containing path separator" option added with
  explicit argument.
  [Prior: 2026-08-11 (v1.4) — three targeted fixes: D-1 scoped user-PATH fallback to WITH-PATH
  plugins; cfg(unix) Windows scope; stale Occupied-codes comment enumeration. [Prior: 2026-08-11
  (v1.3) — ten blockers from second adversarial review. [Prior: 2026-08-10 (v1.2) — three items
  from team-lead. [Prior: 2026-08-10 (v1.1) — four blockers + 7 HIGH items. [Prior: 2026-08-10
  (v1.0) — initial authoring.]]]]]
modified:
  - "2026-08-11 (v1.5)"
  - "2026-08-11 (v1.4)"
  - "2026-08-11 (v1.3)"
  - "2026-08-10 (v1.2)"
  - "2026-08-10 (v1.1)"
  - "2026-08-10 (v1.0)"
---

# ADR-043: exec-subprocess capability sandbox — binary-allow load-time path resolution and guest-cmd substitution

## Context

### Defect 1 (informally: HOST-PORT-001) — PATH/env_clear portability gap

`execute_bounded` in `crates/factory-dispatcher/src/host/exec_subprocess.rs` calls
`command.env_clear()` to strip the subprocess environment, then passes the guest-supplied `cmd`
directly to `Command::new(cmd)`. When `cmd` is a bare basename like `"git"` and PATH was cleared,
the OS falls back to POSIX `_CS_PATH` (`/usr/bin:/bin:/usr/sbin:/sbin`). On a host where a
binary lives only outside `_CS_PATH` — NixOS `/run/current-system/sw/bin`, mise/asdf shim dirs,
`~/.nix-profile/bin` — `command.spawn()` returns `Err(io::ErrorKind::NotFound)`, which
`execute_bounded` collapses to `Err(codes::INTERNAL_ERROR)` (-99) with the io::Error discarded.

**Severity — host-conditional (C-1):** On the authoring host, `git` is at `/usr/bin/git`
(within `_CS_PATH`). Zero WITHOUT-PATH plugins exhibit the defect here. The defect manifests on
hosts where git is absent from `_CS_PATH`: NixOS, mise/asdf-exclusive installs where git lives
only at `~/.local/share/mise/shims/git`, or Nix user-level installs at `~/.nix-profile/bin/git`.
Severity P0 is conditional on such hosts being in the deployment set.

**Per-event process model:** The dispatcher is NOT a daemon. `main.rs` shows `async fn main()`
reads stdin → `run()` → `std::process::exit(code)` at each event. Registry loads fresh on every
PreToolUse/PostToolUse/SessionStart/Stop invocation. Binary resolution must happen at load time
per event.

### Defect 2 — exec_subprocess capability-sandbox path-traversal escape

`binary_allowed(cmd, allow)` in `crates/factory-dispatcher/src/host/exec_subprocess.rs` currently
returns `bool` and computes `b == cmd || b == &basename(cmd)`. A WASM guest can pass an arbitrary
absolute path like `"/tmp/evil/git"` whose basename is `"git"`. If `"git"` is in `binary_allow`,
the check returns `true` and `execute_bounded` receives and spawns the guest's original
`"/tmp/evil/git"`. The capability gate is bypassed for any binary reachable by a path whose
basename matches an allow-listed entry.

Severity: parallel security-reviewer triage assigned CWE-706; see §Downstream Routing. Practical
exploitability is LOW — all 44 plugins use compile-time bare literals. Structural severity HIGH.
Decision 3 closes this via path substitution.

### Four False Doc Comments

**Comment 1 (verbatim) — `plugins/vsdd-factory/hooks-registry.toml`, `validate-factory-path-staging`
exec_subprocess block:**
```
# PATH omitted: the dispatcher process inherits PATH from its parent Claude session;
# the child git subprocess resolves the binary via that inherited PATH without needing
# PATH re-injected into the plugin sandbox. Proven by AC-001 dispatcher-level block tests.
```
False. `env_clear()` strips PATH before spawn. AC-001 uses `run_hook_with_branch` whose helper
injects `exec_subprocess: move |_bin, _args| Ok((0, branch_output.clone(), String::new()))` — git
is never spawned. No test exercises the real spawn path for this plugin.

**Comment 2 (verbatim) — `crates/factory-dispatcher/src/registry.rs`, `ExecSubprocessCaps::binary_allow`
field doc:**
```
/// Binary basename allow-list. The dispatcher resolves each entry
/// to a full path at registry load time (S-1.5 enforces).
```
False today. Literal shell: `grep -c "resolve\|binary_allow" crates/factory-dispatcher/src/registry.rs`
returns **15**. All 15 hits are field declarations, serde derive, and test fixtures — no
path-resolution logic is present. The comment describes the intended post-ADR-043 state, not
current behavior.

**Comment 3 (verbatim) — `crates/factory-dispatcher/src/host/exec_subprocess.rs` module doc:**
```
//! - Setuid / setgid binaries are refused categorically on Unix.
```
Partially false today. `refuse_setuid` calls `fs::metadata(PathBuf::from(cmd))` on Unix; for a
bare name `"git"` this stats `<cwd>/git`, which almost always returns `Err` → function returns
`false` → setuid gate is inert. Becomes true after Decision 3 threads the resolved absolute path
through `run()`.

**Comment 4 (verbatim) — `crates/factory-dispatcher/src/host/exec_subprocess.rs` module doc:**
```
//! - Binary basename must be in `binary_allow`.
//! S-1.5 tightens a few corners (pre-resolved full paths in
//! `binary_allow`, fuel-aware interruption); S-1.4 ships the logical
//! surface + capability gate.
```
Two errors: (a) "Binary basename must be in `binary_allow`" — after Decision 3, allow-list
entries are stored as resolved absolute paths; operator writes basenames but stored values are
absolute paths; the comment is misleading. (b) "S-1.5 tightens a few corners (pre-resolved full
paths in `binary_allow`...)" — S-1.5 is the prospective implementing story; at the time of this
ADR it does not exist; this comment claims S-1.5 has already shipped, which is false.

### Outer vs Inner `env_allow` — Clarification

`Capabilities::env_allow` (outer `[hooks.capabilities]`) controls which host environment variables
the WASM plugin can READ via `host::env`. `ExecSubprocessCaps::env_allow` (inner
`[hooks.capabilities.exec_subprocess]`) controls what is forwarded to the subprocess `envp`.
Only the inner field determines WITH-PATH / WITHOUT-PATH status.

### Blast-Radius Table

Registry scanned via Python literal extraction (2026-08-11). 44 `[hooks.capabilities.exec_subprocess]`
blocks confirmed. `resolvers-registry.toml` has one `wave_context` resolver with no exec_subprocess
block — unaffected.

**Binary counts** (literal from registry): bash×37, jq×36, git×12, gh×4, factory-health×1,
curl×1 = **91 total entries** across 44 blocks. **6 distinct binary names.**

**5 blocks WITHOUT PATH in exec_subprocess.env_allow (directly affected):**

| Plugin | binary_allow | Event / Trigger | on_error | Severity |
|--------|-------------|-----------------|---------|----------|
| `session-start-telemetry` | `["factory-health"]` | event = `"SessionStart"` (no tool field; not a tool regex) | (none — inherited from RegistryDefaults) | GOVERNED: BC-4.04.002 EC-001; factory-health absent on all hosts; fail-open is correct expected behavior |
| `capture-commit-activity` | `["git"]` | event = `"PostToolUse"` (no tool field — fires on every PostToolUse) | `continue` | P0 conditional: advisory telemetry lost on NixOS/mise-only hosts |
| `validate-factory-path-staging` | `["git"]` | event = `"PostToolUse"`, tool `^Bash$` | `continue` | P0 conditional: staging guard fails open on hosts where git absent from _CS_PATH. BC-4.16.001 Invariant 3 governs fail-open on crash/branch-detection failures (NOT on binary-absent); these are different cases |
| `verify-factory-lock` | `["git"]` | event = `"PostToolUse"`, tool `^(Edit\|Write\|MultiEdit\|Agent)$` | `continue` | P0 conditional: lock guard fails open |
| `verify-factory-lock-bash` | `["git"]` | event = `"PostToolUse"`, tool `^Bash$` | `continue` | P0 conditional: companion Bash-tool arm |

**39 blocks WITH PATH in exec_subprocess.env_allow (safe today; D-1 version-downgrade risk
under the prior trusted-prefix-first design, resolved in v1.5 by user-PATH-first algorithm):**

Corrected decomposition (A-13): **37 legacy-bash-adapter plugins + 2 non-adapter native-WASM**
= 39. The 37 adapter plugins: `brownfield-discipline`, `check-factory-commit`,
`check-harness-version`, `convergence-tracker`, `destructive-command-guard`, `factory-branch-guard`,
`postcompact-reanchor`, `protect-bc`, `protect-secrets`×2, `protect-vp`, `purity-check`,
`red-gate`, `validate-anchor-capabilities-union`, `validate-bc-title`,
`validate-changelog-monotonicity`, `validate-count-propagation`, `validate-demo-evidence-story-scoped`,
`validate-factory-path-root`, `validate-finding-format`, `validate-index-self-reference`,
`validate-input-hash`, `validate-novelty-assessment`, `validate-pr-description-completeness`,
`validate-pr-merge-prerequisites`, `validate-red-ratio`, `validate-state-index-status-coherence`,
`validate-state-pin-freshness`, `validate-state-size`, `validate-story-bc-sync`,
`validate-subsystem-names`, `validate-table-cell-count`, `validate-template-compliance`,
`validate-vp-consistency`, `validate-wave-gate-completeness`, `validate-wave-gate-prerequisite`,
`verify-git-push`. The 2 non-adapter: `capture-pr-activity` (`["gh"]`),
`precompact-flush` (`["git"]`).

**D-1 / Version-downgrade risk (RESOLVED in v1.5):** The prior trusted-prefix-first design
(v1.3–v1.4) would have searched `/usr/bin` before `/opt/homebrew/bin`, silently downgrading bash
(3.2.57 from `/bin/bash` instead of 5.3.9 from `/opt/homebrew/bin/bash`) for all 37 adapter
plugins. Measured: `/bin/bash` = GNU bash 3.2.57; `/opt/homebrew/bin/bash` = GNU bash 5.3.9
(user's active version). `validate-count-propagation.sh` uses `declare -A` (bash 4+ associative
arrays); `/bin/bash -n` exits 0 silently — wrong-answer risk class, not a crash. Decision 1 v1.5
resolves this with user-PATH-first for WITH-PATH plugins.

**D-5 — Trust-widening analysis for WITHOUT-PATH P0 guards:** The 10-prefix trusted list is a
strict superset of `_CS_PATH` by 6 directories (`/opt/homebrew/bin`, `/opt/homebrew/sbin`,
`/usr/local/bin`, `/usr/local/sbin`, plus the two Linux-gated linuxbrew dirs). Two of these are
user-writable. For any WITHOUT-PATH P0 guard, Decision 1 has exactly two outcomes: (a) git is in
`/usr/bin` → no-op (same binary as `_CS_PATH` fallback); (b) git is only in a non-`_CS_PATH`
prefix → Decision 1 resolves the defect, but this is a trust widening. There is no case where
Decision 1 fixes a P0 guard without being either a no-op or a widening. The claim "no new trust
expansion for P0 guards" (v1.3–v1.4) was incorrect.

## Decision

### Decision 1: Resolve `binary_allow` entries at load time — PATH-scoped split algorithm

When the dispatcher loads `hooks-registry.toml` (per event, per per-event process model), for
each `ExecSubprocessCaps.binary_allow` entry that is not already an absolute path, the loader
uses one of two algorithms based on whether the entry's plugin declares `"PATH"` in its
`exec_subprocess.env_allow`:

**Algorithm A — WITH-PATH plugins** (declare `"PATH"` in exec_subprocess.env_allow):
1. Search `std::env::var("PATH")` (the dispatcher process's own PATH) first.
2. If found → `RESOLVED-from-user-PATH`. No load_warn — this is the expected primary path
   for WITH-PATH plugins; behavior matches today's spawn behavior exactly.
3. If not found in PATH → search the hardcoded trusted-prefix list (in order).
4. If found in trusted-prefix → `RESOLVED-from-trusted-prefix-fallback`. Emit `internal.load_warn`
   with `reason="RESOLVED-from-trusted-prefix-fallback"` — binary absent from user PATH but
   present in a system dir; unusual; operators can detect and investigate.
5. If not found in either → `BINARY-UNRESOLVABLE-NAME-NOT-FOUND`. Emit load_warn.

**Algorithm B — WITHOUT-PATH plugins** (do NOT declare `"PATH"` in exec_subprocess.env_allow):
1. Search only the hardcoded trusted-prefix list (in order).
2. If found → `RESOLVED-from-trusted-prefix`. No load_warn.
3. If not found → `BINARY-UNRESOLVABLE-NAME-NOT-FOUND`. Emit load_warn.
User PATH is NOT consulted under Algorithm B. No user-PATH fallback exists for WITHOUT-PATH plugins.

**Rationale for user-PATH-first (D-1 resolution):** WITH-PATH plugins already forward user PATH
to the subprocess — that is the semantic of declaring `"PATH"` in exec_subprocess.env_allow.
User-PATH-first at load time makes load-time resolution consistent with spawn-time behavior: the
same binary versions the subprocess would see today. The prior trusted-prefix-first design would
have silently downgraded bash (3.2.57 replacing 5.3.9) for all 37 adapter plugins; `declare -A`
in `validate-count-propagation.sh` fails silently on bash 3.2.

**Rationale for WITHOUT-PATH trusted-prefix only (D-1 scoping):** WITHOUT-PATH plugins do not
forward user PATH to subprocess; user PATH is not in their declared trust domain. Adding a
user-PATH fallback would introduce new trust not warranted by their registry declarations. The
three P0 guards resolve git only from the trusted-prefix list (equivalent to a superset of today's
`_CS_PATH`). The split is derived from each plugin's declared trust model, not an arbitrary
carve-out.

**Hardcoded trusted-prefix list:**
```
/usr/bin            — standard Linux + macOS
/bin                — standard Linux + macOS
/usr/sbin           — standard Linux + macOS
/sbin               — standard Linux + macOS
/opt/homebrew/bin   — Homebrew on Apple Silicon macOS
/opt/homebrew/sbin
/usr/local/bin      — Homebrew on Intel macOS; many Linux installs
/usr/local/sbin
# cfg(target_os = "linux") gate — EXCLUDED on macOS:
/home/linuxbrew/.linuxbrew/bin
/home/linuxbrew/.linuxbrew/sbin
```

**`cfg(target_os = "linux")` gate on linuxbrew prefixes (D-2 fix):** `/home` is an autofs
automount on macOS (`/etc/auto_master` entry `auto_home`). Every `Path::exists()` call under
`/home` triggers an automounter lookup costing 12.04–17.47ms per call (measured). Without the
gate, the two linuxbrew entries on every macOS event add ~29ms of synchronous latency per event.
With the gate, macOS uses 8 prefixes.

**Memoization (D-2 fix):** Within a single registry-load call, memoize by `(binary_name, algorithm)`
(i.e., `(String, bool)` where bool = `plugin_has_path`). The 91 entries reduce to at most 6 names
× 2 algorithms = 12 distinct resolution lookups per event. Maximum stat calls: 6 names × 8 macOS
prefixes = 48 after Linux-gating and memoization. Measured per-prefix cost: `/usr/bin` ≈
2.61µs/stat, `/opt/homebrew/bin` ≈ 1.58µs/stat. Worst-case with both fixes: ≈48 × 4µs average
≈ 200µs per event on macOS. Without fixes (pre-v1.5 design): up to 910 stat calls including
autofs-triggered lookups = measured 28.8–69.1ms per event on macOS.

**Entry already an absolute path:** check via `Path::exists()`. If exists →
`ALREADY-ABSOLUTE-and-exists` (store verbatim, no warn). If not → `BINARY-UNRESOLVABLE-ABSOLUTE-MISSING`
(emit load_warn; Decision 2). NixOS operators: declare absolute paths directly in `binary_allow`
(`binary_allow = ["/run/current-system/sw/bin/git"]`).

**No-canonicalize rule (normative, aligned with BC-1.05.035 v2.0 Invariant 1):** Resolution
MUST construct the candidate path as `prefix_dir.join(binary_basename)` and store VERBATIM.
Resolution MUST NOT call `fs::canonicalize()` or any equivalent that resolves symlinks. Rationale:
`/opt/homebrew/bin/git` is a symlink into `Cellar/git/<version>/bin/git`. `brew upgrade` deletes
the old Cellar directory and updates the symlink atomically; the Cellar-internal path becomes
dangling after upgrade. BC-1.05.035 v2.0 (PO adjudication 2026-08-11) now mandates no-canonicalize
and records TOCTOU as accepted residual risk; the prior BC-1.05.035 conflict is resolved.

**`cfg(unix)` scope (Decisions 1–5):** All five Decisions are scoped to `#[cfg(unix)]`. The
implementation MUST gate new resolution logic under `#[cfg(unix)]` and compile a no-change path
for Windows — Windows retains today's behavior: `Command::new(bare_name)` with OS PATH lookup at
spawn time. Ratifying this ADR introduces zero Windows behavior change. A future architecture work
item must address Windows before Windows binary resolution is changed.

**Test-only resolver injection seam (A-6):** Decision 1 MUST be implemented as an injectable function:
```rust
pub(crate) fn resolve_binary_allow(
    entries: &[String],
    plugin_has_path: bool,
    path_env: Option<&str>,        // production: std::env::var("PATH").ok().as_deref()
    trusted_prefixes: &[&Path],    // production: &TRUSTED_PREFIXES (static slice)
) -> Vec<ResolvedEntry>
```
Tests inject controlled `trusted_prefixes` and `path_env` values without touching global state.
This seam is required by Controls C-1, C-2, C-3, C-5, C-8, C-9.

**Load-warn dedup:** At most one `internal.load_warn` per `(plugin_name, binary_name)` pair per
registry load.

**Ownership facts (verified via `ls -ld`, 2026-08-11):**

| Directory | Mode | User-writable? |
|-----------|------|---------------|
| `/bin`, `/sbin`, `/usr/bin`, `/usr/sbin` | `drwxr-xr-x` root:wheel | No |
| `/usr/local/bin` | `drwxr-xr-x` root:wheel | No (this host) |
| `/opt/homebrew/bin`, `/opt/homebrew/sbin` | `drwxrwxr-x` zious:admin | **YES** |

`/usr/local/sbin` does not exist on this host. `/home/linuxbrew/` — Linux only.

### Decision 2: Per-plugin graceful degradation on unresolvable binaries

The registry always loads to completion. When a binary_allow entry is unresolvable:

1. Emit `internal.load_warn` with reason string (one per distinct case below), `plugin_name`,
   `binary_name`. Deduped per `(plugin_name, binary_name)` within one load.
2. Store a `BINARY_UNRESOLVABLE` sentinel (an internal tag, not a path string).
3. At spawn time, `binary_allowed` returns `AllowResult::BinaryUnresolvable` (see Decision 3)
   → `run()` returns `codes::BINARY_NOT_FOUND (-6)` without calling `execute_bounded`.

**Three distinct reason strings:**
- `reason = "RESOLVED-from-user-PATH"` — Algorithm A (WITH-PATH) found binary in user PATH
  before trusted-prefix (advisory; not an error; spawn proceeds).
- `reason = "RESOLVED-from-trusted-prefix-fallback"` — Algorithm A (WITH-PATH): not in user
  PATH, found in trusted-prefix list (advisory; spawn proceeds).
- `reason = "BINARY-UNRESOLVABLE-ABSOLUTE-MISSING"` — entry was absolute; `Path::exists()` = false.
- `reason = "BINARY-UNRESOLVABLE-NAME-NOT-FOUND"` — basename; all applicable tiers exhausted.

**BC-4.04.002 EC-001 compliance:** `session-start-telemetry` declares
`binary_allow=["factory-health"]`. `factory-health` is absent from PATH on all hosts. Every event
emits `BINARY-UNRESOLVABLE-NAME-NOT-FOUND` for factory-health. At spawn time,
`AllowResult::BinaryUnresolvable` → `BINARY_NOT_FOUND (-6)` → `factory_health="unknown"` →
`session.started` emitted normally. BC-4.04.002 EC-001 preserved.

### Decision 3: `binary_allowed` returns `AllowResult` enum; guest `cmd` substituted throughout `run()`

**Location:** `binary_allowed` is in `crates/factory-dispatcher/src/host/exec_subprocess.rs`.

**Signature change:** `binary_allowed(cmd: &str, allow: &[String]) -> bool` changes to
`binary_allowed(cmd: &str, allow: &[ResolvedEntry]) -> AllowResult`.

**`AllowResult` enum (3-way):**
```rust
enum AllowResult {
    Allowed(String),       // resolved absolute path; caller spawns this path
    Denied,               // no entry's basename matches cmd's basename
    BinaryUnresolvable,   // an entry's basename matched, but it was a BINARY_UNRESOLVABLE sentinel
}
```

**`run()` control flow (all three arms must be matched):**
- `AllowResult::Denied` → `emit_denial("binary_not_on_allow_list")` → return
  `CAPABILITY_DENIED (-1)`. `execute_bounded` is NEVER called.
- `AllowResult::BinaryUnresolvable` → return `BINARY_NOT_FOUND (-6)`.
  `execute_bounded` is NEVER called.
- `AllowResult::Allowed(resolved_path)` → proceed to `is_shell(resolved_path)`,
  `refuse_setuid(resolved_path)`, `execute_bounded(resolved_path, ...)`.

**Post-resolution matching algorithm:** For each `ResolvedEntry` `e` in allow:
- If `e` is a `BINARY_UNRESOLVABLE` sentinel: check `basename(e.original_name) == basename(cmd)`.
  If matched, return `AllowResult::BinaryUnresolvable`.
- If `e` is `Resolved(path)`: check `basename(path) == basename(cmd)` (or exact `path == cmd`).
  If matched, return `AllowResult::Allowed(path.clone())`.
Guest string `cmd` is used only for basename extraction then discarded.

**Security property:** guest passes `cmd="/tmp/evil/git"`. Entry `"git"` resolved to
`"/usr/bin/git"`. Basename of entry = `"git"` = basename of cmd → `AllowResult::Allowed("/usr/bin/git")`.
`execute_bounded` receives `"/usr/bin/git"`, not `"/tmp/evil/git"`. Defect 2 closed.

**Full path threading in `run()`:** `is_shell(resolved_path)`, `refuse_setuid(resolved_path)`,
and `execute_bounded(resolved_path, ...)` ALL receive the resolved path. Guest `cmd` is discarded
after `binary_allowed`. `refuse_setuid` becomes non-inert (previously, bare `"git"` → stat
`<cwd>/git` → Err → false; now receives absolute path → real stat → mode bits checked).

**`is_shell` note — threading resolved path is NOT a security fix (D-4):** By construction of
the matching algorithm, `basename(resolved_path) == basename(cmd)` for all reachable inputs.
Therefore `is_shell(resolved_path) ≡ is_shell(cmd)` for every reachable case. No shell-bypass
scenario is closed by this change.

**Real `is_shell` structural residual (D-4):** An operator who declares
`binary_allow = ["/opt/tools/runner"]` where `/opt/tools/runner` is a symlink to bash creates a
structural bypass: `binary_allowed("runner", [...])` matches on basename `"runner"`;
`is_shell("/opt/tools/runner")` checks whether `"runner"` is in SHELL_NAMES — it is not →
shell_bypass_acknowledged check is skipped → bash executes without shell bypass declaration.
This bypass exists before and after Decision 3. The no-canonicalize rule makes it structurally
unclosable. Disclosed here for operator awareness and security-reviewer classification.
BC-1.05.035 contains a claim about a `./notbash` bypass scenario (the "C-5 fix") that is
mechanically impossible: basename("./notbash") = "notbash" ≠ "bash", so `binary_allowed` returns
`Denied` before reaching `is_shell`. Route to team-lead for product-owner correction.

**BC conflicts resolved (A-1):** All four BC adjudications completed 2026-08-11. BC-1.05.035 v2.0
mandates no-canonicalize with TOCTOU as accepted risk; prior BC-1.05.035 conflict resolved.
BC-1.05.002 v2.0 Inv 1 rewritten (the phrase "Allow-list is enforced by basename, never by full
path" now appears only in that BC's v2.0 changelog row). BC-1.05.028 updated per PO adjudication.
BC-1.05.004 aligned with Decision 3's refuse_setuid path threading. Implementer dispatch is no
longer gated on BC adjudication.

**BC Precedence Ladder correction required (D-3 pending):** BC-1.05.035 Precedence Ladder states
"for the sentinel case, `execute_bounded` additionally returns `codes::BINARY_NOT_FOUND = -6`."
This is mechanically impossible under the `AllowResult` design: `AllowResult::BinaryUnresolvable`
causes `run()` to return `BINARY_NOT_FOUND (-6)` directly WITHOUT reaching `execute_bounded`.
BC EC-004 (outcome code -6, not CAPABILITY_DENIED) is correct; the "execute_bounded additionally
returns" mechanism is wrong. Route to team-lead for product-owner correction before implementer
dispatch.

### Decision 4: Correct all four false doc comments; update dispatch-package-authoring.md

All four doc comments (see §Context) MUST be corrected in the same commit as Decisions 1–3.

- **Comment 1** (hooks-registry.toml validate-factory-path-staging): Replace false "inherits
  PATH" claim with description of Decision 1 behavior (binary resolved via trusted-prefix list at
  load time; PATH forwarding into subprocess is governed by exec_subprocess.env_allow separately).
- **Comment 2** (`ExecSubprocessCaps::binary_allow` field doc in registry.rs): The forward
  reference to "S-1.5 enforces" must be replaced with a citation of this ADR. The comment will
  become true after implementation.
- **Comment 3** (exec_subprocess.rs module doc setuid line): Becomes fully true after Decision 3.
  Retain as-is or update to note the resolved-path requirement.
- **Comment 4** (exec_subprocess.rs module doc "Binary basename must be in `binary_allow`" and
  "S-1.5 tightens a few corners"): Update to describe the actual post-Decision-3 behavior
  (operator declares basenames; dispatcher stores resolved absolute paths; matching is by basename
  comparison); remove the S-1.5 forward reference and replace with a citation of this ADR.

`docs/dispatch-package-authoring.md` MUST document: (a) `binary_allow` basenames are resolved to
absolute paths at every event's registry load via Algorithm A or B based on PATH declaration;
(b) declaring `"PATH"` in exec_subprocess.env_allow governs both subprocess envp forwarding AND
which resolution algorithm applies; (c) for NixOS or other non-standard locations, declare
absolute paths in `binary_allow` directly.

### Decision 5: Host-layer `BINARY_NOT_FOUND` and `SPAWN_FAILED` codes; `INTERNAL_ERROR` scope narrowed

`execute_bounded` currently: `command.spawn().map_err(|_| codes::INTERNAL_ERROR)`. Decision 5:

- `spawn()` returns `Err(e)` where `e.kind() == io::ErrorKind::NotFound` → `codes::BINARY_NOT_FOUND = -6`
- `spawn()` returns `Err(e)` for other `ErrorKind` variants → `codes::SPAWN_FAILED = -7`
- Post-spawn failures (stdin/stdout pipe, `try_wait`) → preserve `codes::INTERNAL_ERROR = -99`
- `AllowResult::BinaryUnresolvable` → `codes::BINARY_NOT_FOUND = -6` (from `run()`, never reaches
  `execute_bounded`)

**Post-spawn conflation note:** Decision 5 disambiguates spawn-time failures from post-spawn
failures. The 5 post-spawn failure modes (stdin take, stdin write_all, stdout take, stderr take,
`try_wait Err`) remain collapsed under `INTERNAL_ERROR (-99)`. This is a v1 known limitation.

**Numeric code allocation — corrected enumeration (confirmed with literal shell):**

`grep -n "Occupied\|NOT_FOUND\|INTERNAL_ERROR\|CAPABILITY\|TIMEOUT\|OUTPUT_TOO\|INVALID" crates/factory-dispatcher/src/host/mod.rs`
outputs (relevant subset):
```
pub const CAPABILITY_DENIED: i32 = -1;
pub const TIMEOUT: i32 = -2;
pub const OUTPUT_TOO_LARGE: i32 = -3;
pub const INVALID_ARGUMENT: i32 = -4;
/// Occupied codes: 0 (OK), -1 (CAPABILITY_DENIED), -2 (TIMEOUT),
/// -3 (OUTPUT_TOO_LARGE), -4 (INVALID_ARGUMENT), -99 (INTERNAL_ERROR).
pub const NOT_FOUND: i32 = -5;
pub const INTERNAL_ERROR: i32 = -99;
```

The Occupied-codes doc comment is **already stale before ADR-043** (omits `-5` which exists on
the next line). After adding -6 and -7, ALL five sibling sites MUST use the corrected enumeration:
`0 (OK), -1 (CAPABILITY_DENIED), -2 (TIMEOUT), -3 (OUTPUT_TOO_LARGE), -4 (INVALID_ARGUMENT),
-5 (NOT_FOUND), -6 (BINARY_NOT_FOUND), -7 (SPAWN_FAILED), -99 (INTERNAL_ERROR).`

`codes::NOT_FOUND = -5` is for `read_file` semantics (path in allow-list, file not on disk, per
ADR-025 Decision 13). `BINARY_NOT_FOUND = -6` is for exec_subprocess spawn-time binary not found.
Distinct semantics; MUST NOT be conflated.

**TD-VSDD-060 sibling sites** (5 locations requiring lock-step update):
(a) Occupied-codes doc comment in `crates/factory-dispatcher/src/host/mod.rs` (the doc comment
above `NOT_FOUND`). Current stale text: `Occupied codes: 0 (OK), -1 (CAPABILITY_DENIED), -2 (TIMEOUT), -3 (OUTPUT_TOO_LARGE), -4 (INVALID_ARGUMENT), -99 (INTERNAL_ERROR).` Corrected: full
9-code enumeration above.
(b) Red Gate test doc comment in `crates/factory-dispatcher/src/host/mod.rs` — same stale
enumeration repeats in the test documentation block.
(c) Red Gate test assertion string in `crates/factory-dispatcher/src/host/mod.rs` — contains
inline `"...occupied: 0/-1/-2/-3/-4/-99"`. Corrected: `"...occupied: 0/-1/-2/-3/-4/-5/-6/-7/-99"`.
(d) `HostError::from_code` in `crates/hook-sdk/src/host.rs` — add named variants `BinaryNotFound`
and `SpawnFailed` mapping to -6 and -7 (currently forces plugin authors to match `Other(-6)`).
(e) Companion Red Gate tests for -6 and -7 following the `test_S19_03_T005_NOT_FOUND_constant_equals_minus_5` pattern.

## Outcome / Control Matrix

8 outcomes total (5 registry-load + 3 spawn-time). Per D-970 Codification 1, each outcome must
be individually identifiable with a reachable trigger, distinct reason identifier, and a control.

**Registry-load outcomes:**

| Outcome ID | Trigger | Load-time action | Reason string |
|------------|---------|-----------------|---------------|
| `RESOLVED-from-user-PATH` | Algorithm A (WITH-PATH): found in user PATH | Store resolved abs path | (no warn — expected primary path) |
| `RESOLVED-from-trusted-prefix` | Algorithm B (WITHOUT-PATH): found in trusted-prefix | Store resolved abs path | (no warn — expected primary path) |
| `RESOLVED-from-trusted-prefix-fallback` | Algorithm A (WITH-PATH): not in user PATH; found in trusted-prefix | Store resolved abs path | `"RESOLVED-from-trusted-prefix-fallback"` (advisory) |
| `ALREADY-ABSOLUTE-and-exists` | Entry is already absolute; `Path::exists()` = true | Store as-is | (no warn) |
| `ABSOLUTE-but-missing` | Entry is already absolute; `Path::exists()` = false | Store sentinel | `"BINARY-UNRESOLVABLE-ABSOLUTE-MISSING"` |
| `BINARY-UNRESOLVABLE-NAME-NOT-FOUND` | All applicable tiers exhausted | Store sentinel | `"BINARY-UNRESOLVABLE-NAME-NOT-FOUND"` |

**Spawn-time outcomes:**

| Outcome ID | Trigger |
|------------|---------|
| `BINARY_NOT_FOUND (-6)` | `AllowResult::BinaryUnresolvable` OR `spawn()` returns `io::ErrorKind::NotFound` |
| `SPAWN_FAILED (-7)` | `spawn()` returns other `io::ErrorKind` (PermissionDenied, ResourceBusy, etc.) |
| `INTERNAL_ERROR (-99)` | Post-spawn failure (stdin/stdout pipe, `try_wait`) |

**Controls (10 outcomes; resolver injection seam from Decision 1 required for C-1, C-2, C-3, C-5, C-8, C-9):**

- **C-1 — RESOLVED-from-user-PATH (Algorithm A; negative control):**
  Fixture: WITH-PATH plugin; inject `trusted_prefixes=[]` (empty); inject `path_env` pointing to a
  temp dir containing the test binary. Assert: `RESOLVED-from-user-PATH` outcome; no load_warn;
  spawn path = the temp-dir binary path.

- **C-2 — BINARY-UNRESOLVABLE-NAME-NOT-FOUND via Algorithm A (positive):**
  Fixture: WITH-PATH plugin; inject `trusted_prefixes=[]`; inject `path_env=""` (empty);
  `binary_allow=["nonexistent-vsdd-test-xyz"]`. Before fix: spawn returns `INTERNAL_ERROR (-99)`.
  After fix: load_warn reason `"BINARY-UNRESOLVABLE-NAME-NOT-FOUND"`; spawn returns
  `BINARY_NOT_FOUND (-6)`. Assert reason string (not just code).

- **C-3 — RESOLVED-from-trusted-prefix via Algorithm B (negative control):**
  Fixture: WITHOUT-PATH plugin; inject `trusted_prefixes=["/tmp/test-prefix"]`; binary
  `"vsdd-test-git"` exists at `/tmp/test-prefix/vsdd-test-git` (created by test). Assert:
  `RESOLVED-from-trusted-prefix`; no load_warn; resolved path = `/tmp/test-prefix/vsdd-test-git`.
  (No host-dependency on any real system directory.)

- **C-4 — RESOLVED-from-trusted-prefix-fallback advisory (Algorithm A; observable):**
  Fixture: WITH-PATH plugin; inject `path_env=""` (empty); inject `trusted_prefixes` containing
  the binary. Assert: load_warn with reason `"RESOLVED-from-trusted-prefix-fallback"` IS emitted;
  spawn proceeds. Distinguishes from C-1 (which emits no warn).

- **C-5 — ALREADY-ABSOLUTE-and-exists (negative control):**
  Fixture: inject `binary_allow=["/tmp/vsdd-test-git"]` (absolute; file created by test). Assert:
  no load_warn; spawn succeeds using that exact path.

- **C-6 — ABSOLUTE-but-missing (positive; distinct reason from C-2):**
  Fixture: `binary_allow=["/nonexistent/absolute/vsdd-test-git"]` (absolute; file does not exist).
  Assert: load_warn reason `"BINARY-UNRESOLVABLE-ABSOLUTE-MISSING"` (NOT
  `"BINARY-UNRESOLVABLE-NAME-NOT-FOUND"`). Assert distinct reason string.

- **C-7 — SPAWN_FAILED (-7 distinct from -6; positive):**
  Fixture: binary exists and resolves via injection; mode set to 0o644 (no execute bit). Spawn
  returns `io::ErrorKind::PermissionDenied`. Assert `execute_bounded` returns `SPAWN_FAILED (-7)`,
  NOT `BINARY_NOT_FOUND (-6)` and NOT `INTERNAL_ERROR (-99)`.

- **C-8 — setuid gate via `run()` (regression guard for refuse_setuid path threading):**
  Fixture: inject trusted_prefixes with a test binary; set setuid bit (Unix `chmod 4755`). Call
  through `run()` (not `refuse_setuid` directly). Assert `run()` returns `CAPABILITY_DENIED (-1)`
  via `refuse_setuid(resolved_path)`. Before fix: `refuse_setuid("git")` stats `<cwd>/git` → Err
  → false (inert). After Decision 3: resolved absolute path → real stat → mode bits → gate fires.

- **C-9 — WITHOUT-PATH plugin does NOT fall back to user PATH (boundary guard; D-1 scoping invariant):**
  Fixture: WITHOUT-PATH plugin config; `binary_allow=["vsdd-test-only"]`; inject `path_env`
  containing a dir with `vsdd-test-only`; inject `trusted_prefixes=[]` (empty — binary absent
  from all system dirs). Assert: outcome is `BINARY-UNRESOLVABLE-NAME-NOT-FOUND`; load_warn
  with reason `"BINARY-UNRESOLVABLE-NAME-NOT-FOUND"` IS emitted; `RESOLVED-from-user-PATH`
  does NOT occur. This control guards the scoping invariant: user-PATH fallback is inaccessible
  to WITHOUT-PATH plugins regardless of what user PATH contains.

- **C-10 — post-spawn INTERNAL_ERROR (-99; distinguished from BINARY_NOT_FOUND and SPAWN_FAILED):**
  Fixture: binary resolves and spawns; stdout `take()` returns `None` (simulated pipe failure via
  test double). Assert `execute_bounded` returns `INTERNAL_ERROR (-99)`, NOT `BINARY_NOT_FOUND
  (-6)` and NOT `SPAWN_FAILED (-7)`. Confirms INTERNAL_ERROR is narrowed to post-spawn only.

**ARCH-INDEX coverage gap:** The entire `host/` subtree is missing from SS-01's module catalog
(`host/mod.rs`, `exec_subprocess.rs`, `read_file.rs`, `memory.rs`, `path_util.rs` — none appear
in ARCH-INDEX). The implementing story MUST extend SS-01 module catalog to cover the full `host/`
subtree. Routing: state-manager (ARCH-INDEX owner).

## Rationale

Primary goals: **portability** (P0 guards work on all common deployment hosts where git may not
be in `_CS_PATH`) and **observability** (spawn failure produces a distinct, diagnosable code).

For Decision 1, the split PATH-scoped algorithm is chosen because:

1. **Algorithm A (user-PATH-first)** preserves current effective behavior for the 37 adapter
   plugins and 2 non-adapter WITH-PATH plugins. They get the same binary versions today. The
   alternative (trusted-prefix-first, as in v1.3–v1.4) silently downgrades bash to 3.2.57 for
   all 37 adapter plugins, breaking `declare -A` in `validate-count-propagation.sh` with a
   silent wrong-answer failure mode.

2. **Algorithm B (trusted-prefix-only)** preserves WITHOUT-PATH plugins' existing trust model.
   They do not declare PATH, so user PATH is not in their trust domain. The user-PATH fallback
   scoped to those that already trusted it is derived from declared registry semantics.

3. **D-5 acknowledged:** For WITHOUT-PATH P0 guards, Decision 1 is either a no-op (git in
   `/usr/bin`) or a trust widening (git only in a non-`_CS_PATH` prefix). This widening is
   accepted as the cost of fixing the portability defect on non-standard hosts. See §Negative.

4. **Memoization + Linux-gate** eliminate the autofs-induced latency (measured 28.8–69.1ms/event
   → ≈200µs worst-case with both fixes on macOS).

5. **No-canonicalize** is now mandated by BC-1.05.035 v2.0 (PO adjudication 2026-08-11). The
   TOCTOU residual is documented as accepted risk.

## Consequences

### Positive

- P0-critical guards (`validate-factory-path-staging`, `verify-factory-lock`,
  `verify-factory-lock-bash`) resolve git via trusted-prefix on hosts where git is absent from
  `_CS_PATH` (conditional on such hosts being in deployment set; no change on this authoring host).
- `gh` resolves correctly on macOS ARM and Intel (in trusted-prefix list); all 4 gh-declaring
  plugins are WITH-PATH and thus also resolve via user-PATH-first.
- Defect 2 (CWE-706 capability-sandbox path-traversal escape) closed by Decision 3 substitution.
- `refuse_setuid` becomes non-inert for trusted-prefix-resolved entries (receives absolute path).
- `BINARY_NOT_FOUND (-6)` and `SPAWN_FAILED (-7)` are distinct, diagnosable spawn-time outcomes.
  `INTERNAL_ERROR (-99)` is narrowed to post-spawn failures.
- BC-4.04.002 EC-001 preserved for session-start-telemetry.

### Negative / Trade-offs

- **D-5 — Decision 1 is either a no-op or a trust widening for WITHOUT-PATH P0 guards.** The
  10-prefix list is a strict superset of `_CS_PATH` by 6 directories (including 2 user-writable).
  For a P0 guard on this host (git at `/usr/bin`): Decision 1 is a no-op. On a host where git is
  only in `/opt/homebrew/bin`: Decision 1 resolves the defect but also expands trust beyond
  `_CS_PATH`. Accepted as the cost of fixing portability.
- **TOCTOU window for user-writable prefix symlinks (accepted risk, BC-1.05.035 v2.0):**
  `/opt/homebrew/bin` is `drwxrwxr-x zious:admin`. Between `Path::exists()` (load time) and
  `execve()` (spawn time), an attacker in the `admin` group could repoint a symlink there. Scope:
  WITH-PATH plugins where the binary resolves via `/opt/homebrew/bin`. Risk accepted per BC-1.05.035
  v2.0: attack requires admin group membership + sub-millisecond precision; primary trust boundary
  is the capability gate, not protection against a locally-compromised session user.
- **`is_shell` structural residual (D-4):** Operator absolute entries whose basename is not in
  SHELL_NAMES but whose target IS a shell (symlink-to-bash pattern) bypass shell-bypass-acknowledgement
  check. Unclosable under no-canonicalize. Disclosed for operator and security-reviewer awareness.
- **Windows: no behavior change** (all Decisions gated `#[cfg(unix)]`). Future architecture work
  item required before Windows binary resolution changes.
- **Per-event cost:** ≈200µs worst-case macOS (after memoization + Linux-gate). Without fixes:
  up to 69ms on macOS with autofs-triggered linuxbrew lookups.
- **`allow_exec` test-support scope:** `allow_exec` is inside `#[cfg(test)] pub(crate) mod test_support`
  in `crates/factory-dispatcher/src/host/mod.rs` — unit tests only. Controls C-1 through C-9 use
  the `resolve_binary_allow` injection seam, not `allow_exec`.

### Status as of 2026-08-11 (v1.5)

Proposed. Decisions 1–5 not yet implemented. Both defects confirmed live by static analysis.
BC adjudication complete (2026-08-11 per team-lead report). Pending before implementer dispatch:
(i) human ratification of this ADR per POLICY 22;
(ii) BC-1.05.035 Precedence Ladder correction (D-3: "execute_bounded additionally returns -6 for
sentinel" is mechanically impossible; route to team-lead → product-owner);
(iii) BC-1.05.035 is_shell/C-5-fix correction (D-4: "./notbash" bypass claim is impossible;
real residual is unclosable symlink case; route to team-lead → product-owner).

## Alternatives Considered

- **Option (a) — Auto-inject `_CS_PATH`:** Rejected. Does not fix portability on Homebrew-only
  hosts. Makes the POSIX accident reliable without addressing the root cause.

- **Option (b) — Require PATH in exec_subprocess.env_allow:** Rejected. Forces user session PATH
  into every subprocess environment, defeating `env_clear()` for all plugins.

- **Option (c) — Resolve against `std::env::var("PATH")` only:** Rejected for WITHOUT-PATH plugins
  (would widen trust model). Algorithm A user-PATH-first for WITH-PATH plugins incorporates this
  as the primary tier, which is correct.

- **Option (d) — Per-plugin env_allow additions:** Rejected. Same widening for those plugins.

- **Option (e-split) — Split algorithm by PATH-presence (selected):** Algorithm A user-PATH-first
  for WITH-PATH; Algorithm B trusted-prefix-only for WITHOUT-PATH. Rationale in §Decision 1 and
  §Rationale.

- **Option (f) — Reject any guest `cmd` containing a path separator:**
  Reject any exec_subprocess call where `cmd` contains `/` (POSIX) or `\` (Windows) before the
  capability check; return `CAPABILITY_DENIED (-1)`.

  Arguments for this option: (1) Closes CWE-706 structurally — any absolute-path cmd is rejected;
  only bare basenames reach `Command::new`; the OS resolves via `_CS_PATH` / subprocess envp, which
  the dispatcher controls. (2) No trusted-prefix list → no D-1 version-downgrade, no D-2 autofs
  latency, no D-2 suffix complexity. (3) cfg-neutral — works identically on Windows and Unix.
  (4) Preserves basename-only matching; BC-1.05.002/.028 need no amendment. (5) All 44 production
  plugins use compile-time bare literals (confirmed: bash×37, jq×36, git×12, gh×4, factory-health×1,
  curl×1) — zero operational impact from the rejection itself.

  **Why the chosen design (Option e-split) is worth its additional cost relative to Option (f):**
  Path-separator rejection closes CWE-706 (Defect 2) but does NOT close HOST-PORT-001 (Defect 1).
  On a host where git is only at `~/.local/share/mise/shims/git` or `/nix/store/.../bin/git`, a
  guest passing bare `"git"` + `env_clear()` still returns `INTERNAL_ERROR` → P0 guard fails open.
  Option (f) restricts what the *guest* can pass; it does not affect what the *host* can find after
  `env_clear()`. Fixing both defects under Option (f) requires a second mechanism: either operators
  declare absolute entries everywhere (registry churn; fragile across platforms), or the dispatcher
  retains `_CS_PATH` resolution (doesn't fix Homebrew-only hosts), or `env_clear()` is not called
  (defeats the sandbox model). Option (e-split) addresses both defects in one mechanism: load-time
  resolution provides the correct binary regardless of subprocess PATH, AND path substitution
  prevents the guest from influencing which copy of a basename is executed.

  Option (f) remains valuable as **defense-in-depth** on top of the chosen design: adding
  path-separator rejection in `run()` before the `binary_allowed` check would close any potential
  future scenario where a WASM plugin is updated to accept dynamic cmd values. The implementer
  should consider adding it as an additional input-validation step.

## Source / Origin

**Literal shell evidence for load-bearing premises:**

```bash
getconf PATH
# → /usr/bin:/bin:/usr/sbin:/sbin  (POSIX default confirmed 2026-08-11)

which -a git
# → /opt/homebrew/bin/git
# → /usr/bin/git  (accidental POSIX success on this authoring host; defect not visible here)

which -a gh
# → /opt/homebrew/bin/gh  (absent from POSIX default; WITH-PATH resolves via user-PATH-first)

which -a bash
# → /opt/homebrew/bin/bash  (5.3.9 — user's active version)
# → /bin/bash                (3.2.57 — trusted-prefix-first would have returned this; D-1 fix)

ls -ld /opt/homebrew/bin /opt/homebrew/sbin /usr/local/bin /usr/bin /bin
# → drwxrwxr-x zious:admin  /opt/homebrew/bin  (user-writable)
# → drwxrwxr-x zious:admin  /opt/homebrew/sbin (user-writable)
# → drwxr-xr-x root:wheel   /usr/local/bin     (root-owned on this host)
# → drwxr-xr-x root:wheel   /usr/bin           (root-owned)
# → drwxr-xr-x@ root:wheel  /bin               (root-owned)

grep -c "resolve\|binary_allow" crates/factory-dispatcher/src/registry.rs
# → 15  (all hits are field declarations, serde derive, test fixtures; no resolution logic)

grep -n "fn binary_allowed" crates/factory-dispatcher/src/host/exec_subprocess.rs
# → binary_allowed in exec_subprocess.rs, NOT mod.rs

grep -n "pub(crate) mod test_support\|fn allow_exec" crates/factory-dispatcher/src/host/mod.rs
# → allow_exec inside #[cfg(test)] pub(crate) mod test_support; unit-test scope only

# Python literal extraction: 37 legacy-bash-adapter + 2 non-adapter = 39 WITH-PATH;
#   5 WITHOUT-PATH; bash×37, jq×36, git×12, gh×4, factory-health×1, curl×1 = 91 total

grep -cE "BINARY_NOT_FOUND|Decisions 5.6" \
  .factory/specs/architecture/decisions/ADR-025*.md
# → 0  (fabricated "ADR-025 Decisions 5+6" anchor confirmed absent; removed)

ls .factory/stories/ | grep "^S-21\." | sort -t. -k2 -n | tail -3
# → S-21.11-..., S-21.12-..., S-21.13-read-file-range-targeted-bc-index-row-lookup.md
#   (ceiling is S-21.13; S-21.14 does not exist)
```

## Downstream Routing

| Work item | Owner | Gate |
|-----------|-------|------|
| BC-1.05.035 Precedence Ladder correction: "execute_bounded additionally returns BINARY_NOT_FOUND -6 for sentinel" is mechanically impossible under AllowResult design (run() handles BinaryUnresolvable directly; execute_bounded not called); EC-004 outcome code is correct but mechanism description is wrong | team-lead → product-owner | Before implementer dispatch |
| BC-1.05.035 is_shell residual correction: "./notbash" bypass claim (the "C-5 fix" in BC body) is mechanically impossible (binary_allowed returns Denied before is_shell); real residual is unclosable symlink-to-shell pattern under no-canonicalize | team-lead → product-owner | Before implementer dispatch |
| Implement Decisions 1–5: split resolution algorithm (resolve_binary_allow injection seam); AllowResult 3-way enum; full path threading through run(); BINARY_NOT_FOUND=-6 and SPAWN_FAILED=-7 in codes; HostError::from_code named variants; five TD-VSDD-060 sibling sites; all four false doc comment corrections | implementer (next unallocated story ID after S-21.13, per story-writer) | After human ratification + above BC corrections |
| Update docs/dispatch-package-authoring.md per Decision 4 | technical-writer | |
| Extend SS-01 ARCH-INDEX module catalog for full host/ subtree (host/mod.rs, exec_subprocess.rs, read_file.rs, memory.rs, path_util.rs) | state-manager (ARCH-INDEX owner) | |
| ARCH-INDEX row for ADR-043 + total_adrs bump (42 → 43) | state-manager | |
| Story spec authoring incorporating all five Decisions, Controls C-1–C-10, and resolver injection seam | story-writer | After human ratification |
| Security triage of Defect 2 (CWE-706 confirmed) and is_shell structural residual (D-4) | security-reviewer | In parallel |
| Windows binary resolution future ADR | architect (future scope; does not block ADR-043 ratification) | |
