---
title: "E-19 Post-Epic Architecture Closure Report"
type: arch-closure
epic: E-19
version: v1.0
date: 2026-07-17
status: final
author: vsdd-factory:architect
stories_covered:
  - S-19.01
  - S-19.02
  - S-19.03
  - S-19.04
  - S-19.05
  - S-19.06
  - S-19.07
  - S-19.08
  - S-19.09
adr_references:
  - ADR-025 v1.18
  - ADR-030 v1.4
develop_tip_at_close: 6db4c9fc
---

# E-19 Post-Epic Architecture Closure Report

E-19 (Post-rc.22 Operator Hardening) closed 2026-07-17 at develop tip 6db4c9fc with all
nine stories merged. This report records the as-built host ABI surface, confirms delivery
of ADR-025 Deliverables D19–D22, assesses the ADR-030 §D3 Rust migration question,
routes the three carried S-19.09 items, and identifies the standing release-gate risk
from platform binary staleness. E-20 architecture candidates are enumerated in Section 6.


## 1. Post-E-19 Host ABI Surface State

### 1.1 Entry points

E-19 delivers two host function entry points accessible to WASM hook plugins:

**host::read_file** — all-or-nothing bounded read. Added in pre-E-19 history; extended in
E-19 by:
- `codes::NOT_FOUND = -5` for absent-but-allowlisted paths (S-19.03, ADR-025 Decision 13)
- `internal.file_not_found` telemetry event on absent-file case (S-19.03)
- Named constant `INTERNAL_FILE_NOT_FOUND` in production code (S-19.09 D21)
- Corrected `timeout_ms` framing comment (S-19.09 D20)
- Two-linker protocol documentation (S-19.09 D20)

**host::read_prefix** — head-c bounded partial read. Added in S-19.06 (PR #657);
production path registered in S-19.09 (PR #659, D19). NEVER returns OUTPUT_TOO_LARGE.
Returns at most `max_bytes` bytes; truncates silently. Returns NOT_FOUND (-5) for absent
paths following the same code convention as `read_file` (Decision 13 precedent).
Requires a separate `[hooks.capabilities.read_prefix]` block (BC-1.17.001 Invariant 3;
defense-in-depth; `read_file` capability does not extend to `read_prefix`).

HOST_ABI_VERSION remains 1 across all E-19 work. Every change was purely additive.

### 1.2 Production-path registration (ADR-025 Decision 19)

ADR-025 Decision 16 (v1.16) identified that `read_prefix` was registered only in the test
path (`setup_linker`, `Linker<HostContext>`) and was absent from the production path
(`setup_host_on_store_data`, `Linker<StoreData>`). A 0-hit grep on `invoke.rs` confirmed
the gap at the time of adjudication. Decision 19 mandated the fix.

S-19.09 (PR #659, commit 13ece92c) delivered the registration. The production-path
`read_prefix` handler follows the same memory-grow protocol as the `read_file` handler:
grow by `ceil(len / 65536)` pages, write at `current_bytes > 0`, return that offset as
`out_ptr`. This distinguishes the production-path response from the test-path sentinel
described in §1.3 below.

### 1.3 Two-linker protocol (ADR-025 Decision 17)

The dispatcher employs two distinct linker configurations that differ in how they return
data to the WASM guest:

- **Test path — `Linker<HostContext>` via `setup_linker`** (defined in `host/mod.rs`):
  writes data at WASM memory address 0 and returns `out_ptr = 0`. The hook-sdk
  `read_owned_bytes` function treats `ptr == 0` as a sentinel, returning `Vec::new()`.
  This path exists to keep test WASM guests free from memory-grow bookkeeping.

- **Production path — `Linker<StoreData>` via `setup_host_on_store_data`** (defined in
  `invoke.rs`): grows WASM linear memory by the required number of pages, writes data
  starting at `current_bytes` (the pre-grow byte count), and returns that offset as
  `out_ptr`. The hook-sdk `read_owned_bytes` function on this path receives `ptr > 0`
  and reads the real data.

The SEC-001 CRITICAL finding associated with the `out_ptr = 0` test-path behavior was
accepted-with-record in ADR-025 Decision 17. The sentinel is correct by protocol
contract: the hook-sdk's `ptr == 0` guard is the architectural coupling point. Any
refactor that removes the guard without also changing the test-path protocol would break
the contract. Both the guard and the protocol are documented in the `host/read_file.rs`
handler comment adjacent to the `prepare()` call.

### 1.4 timeout_ms framing (ADR-025 Decision 18)

Prior to S-19.09, comments in the `read_file.rs` and `read_prefix.rs` handlers stated
that `timeout_ms` was "enforced via epoch interruption." This framing was incorrect.
Epoch interruption ticks only at WASM yield points; it cannot preempt a blocking
`func_wrap` host call executing synchronously on the dispatcher thread.

The correct framing, delivered by S-19.09 D20, is: `timeout_ms` is accepted for
ABI forward-compatibility. Per-host-function timeout is structurally unenforced in the
current synchronous `func_wrap` dispatch path. The store-level epoch deadline
(`limits.timeout_ms`) governs coarse plugin-level time.

ADR-025 Decision 18 confirms SEC-003 CWE-833 at LOW severity: `path_allow` is
operator-configured; normal local-SSD reads never block; operators who configure
`path_allow` entries for FIFOs or NFS mounts accept the blocking risk with full
knowledge of the framing. The fix is architectural documentation (D20), not a change to
the dispatch model.

### 1.5 Error code taxonomy

All codes are i32 values returned as the host function's FFI return to the WASM guest:

| Code | Constant | Meaning | Notes |
|------|----------|---------|-------|
| 0 | `OK` | Success | Data written to WASM memory; `out_ptr > 0` on production path |
| -1 | `CAPABILITY_DENIED` | Path not in `path_allow` | Operator-visible; in capability schema |
| -2 | `TIMEOUT` | Plugin exceeded time limit | Store-level epoch deadline; in capability schema |
| -3 | `OUTPUT_TOO_LARGE` | File exceeds `max_bytes` (read_file only) | read_prefix NEVER emits this |
| -4 | `INVALID_ARGUMENT` | Bad UTF-8 path or guest ptr out-of-bounds | Marshalling-internal; NOT operator-visible; intentionally omitted from read_prefix schema (ADR-025 Decision 19) |
| -5 | `NOT_FOUND` | Path allowlisted but file absent | Added E-19 (S-19.03, Decision 13); in capability schema for both entry points |
| -99 | `INTERNAL_ERROR` | I/O failure or memory.grow failure | In capability schema |

The `INVALID_ARGUMENT (-4)` omission from the `read_prefix` capability schema preamble
table is a deliberate architectural choice, not an oversight. Well-formed SDK calls cannot
trigger -4; it is unreachable through the public hook-sdk surface. The hooks-registry.toml
preamble table `(0, -1, -2, -5, -99)` is correct and complete.

### 1.6 Residual ABI gaps as-built

None. The ABI surface as defined by ADR-025 Decisions 13–22 is fully implemented and
documented. The only outstanding risk is the platform binary staleness described in
Section 5, which is a packaging concern rather than an ABI design gap.


## 2. ADR-025 Deliverables D19–D22 Disposition

### D19: read_prefix production-path registration

**Delivered by:** S-19.09 (PR #659, commit 13ece92c)

**Behavioral anchor:** `read_prefix` registered in `setup_host_on_store_data` inside the
production `invoke` path, mirroring the `read_file` memory-grow protocol. The
registration block follows the same page-rounding and `current_bytes`-based offset
pattern. Integration tests T-001/T-002/T-003 in the `invoke` module's `#[cfg(test)]`
block verify production-path behavior.

**Gap that preceded delivery:** A 0-hit grep on `invoke.rs` for the `read_prefix` symbol
confirmed the absence. Any plugin that declared `vsdd::read_prefix` in its imports would
have received a wasmtime link error on the production path before S-19.09.

### D20: timeout_ms non-enforcement framing + two-linker protocol comment

**Delivered by:** S-19.09 (PR #659, same commit set as D19)

**Behavioral anchor (timeout_ms):** `let _ = timeout_ms;` drop expression in both
`host/read_file.rs` and `host/read_prefix.rs` handlers; each is accompanied by the
corrected comment (verbatim: "accepted for ABI forward-compatibility; per-host-function
timeout is structurally unenforced in the current synchronous func_wrap dispatch path;
the store-level epoch deadline governs coarse plugin-level time").

**Behavioral anchor (two-linker):** Documentary comment in `host/read_file.rs` in the
vicinity of the `prepare()` call, explaining the `Linker<HostContext>` (test path,
`out_ptr = 0` sentinel) versus `Linker<StoreData>` (production path, memory-grow, real
address) duality and the hook-sdk guard that bridges them.

### D21: named constants for telemetry event type strings

**Delivered by:** S-19.09 (PR #659)

**Behavioral anchor:** `internal_log.rs` exports four constants:
- `INTERNAL_FILE_NOT_FOUND` = `"internal.file_not_found"` (new in S-19.09)
- `PLUGIN_ABANDONED` = `"plugin.abandoned"` (new in S-19.09)
- `PLUGIN_COMPLETED` = `"plugin.completed"` (pre-existing, value-pinned in S-19.09)
- `PLUGIN_TIMEOUT` = `"plugin.timeout"` (pre-existing, value-pinned in S-19.09)

Each constant has a corresponding `#[cfg(test)]` value-pin test in `internal_log.rs` that
asserts the string literal has not drifted. Production code in `read_file.rs`,
`read_prefix.rs`, and `emit_event.rs` references the named constants throughout. Test
assertion code (awk-scoped by the AC-008 production-code-only gate) is explicitly
excluded from the bare-literal sweep — isolated test strings are acceptable.

### D22: plugin.completed async event carries timestamp field

**Delivered by:** S-19.09 (PR #659)

**Behavioral anchor:** `emit_plugin_completed_async` in `host/emit_event.rs` calls
`.with_field("timestamp", ts.as_str())` before `ctx.emit_internal(ev)`. This matches
the pattern established by all sibling async event emitters in the same file
(`emit_plugin_abandoned_async`, `emit_plugin_timeout_async`).

**BC closure:** BC-3.08.001 §Event 6 (plugin.completed) mandatory `timestamp` field is
satisfied. The field was previously absent from `plugin.completed` while present on the
other events — an asymmetry identified as finding F-WG-003 and closed by D22.


## 3. ADR-030 §D3 State and D-846 Follow-up Story

### §Decision 3 current state

ADR-030 §Decision 3 (v1.4) governs `plugins/vsdd-factory/bin/enforce-merge-strategy.sh`,
a bash bin tool invoked explicitly by the orchestrator (not dispatcher-fired). The script:

- Accepts positional arguments: `$1` = repo, `$2` = strategy flag, `${@:3}` = residual
  `gh pr merge` args forwarded verbatim
- Enforces `--merge` for release PRs (branch name matches `^release/v`)
- Maintains a deny-list blocking strategy-smuggling via `--squash`, `--merge`, `--rebase`,
  and `--admin` in all normalized forms, preventing injected flags from overriding the
  positional strategy argument
- Carries a best-effort-with-verify branch-deletion obligation (ADR-030 v1.4)

This is the complete delivered state. No regression or gap was identified during E-19.

### Rust migration question (§D3 research)

ADR-030 §Decision 3 left the Rust migration question open. The architectural assessment is:

**No Rust/WASM migration is appropriate in E-20 scope.** The constraints are:

1. `check-stale-verdict.sh` requires access to `covered_sha` from the SubagentStop
   payload. The WASM SubagentStop hook API does not expose this field in `tool_input` at
   the plugin boundary. The script resolves `covered_sha` by calling `gh pr view` from
   its subprocess execution context — a capability pattern that requires
   `exec_subprocess.binary_allow = ["gh"]`. This is the same silent-no-op footgun class
   documented in ADR-025 §Decision 2 and §Decision 3 history: omitting `gh` from
   `binary_allow` would silently make the gate inert.

2. `enforce-merge-strategy.sh` must forward arbitrary residual `gh pr merge` arguments.
   A WASM reimplementation would need the same `exec_subprocess` capability with
   `env_allow` entries for the GitHub CLI's runtime requirements, replicating the same
   footgun surface.

3. Neither script fires in a per-tool-use hook context. They are explicit orchestrator
   obligations — correct in their current form as bin tools invoked at known decision
   points.

A WASM migration becomes viable only when SubagentStop `tool_input` exposes `covered_sha`
natively AND `exec_subprocess` + `gh` capability patterns have been proven safe in the
WASM sandbox context. Neither precondition holds today. **Recommendation:** defer to E-21+
with an explicit prerequisite: SubagentStop payload schema evolution that exposes
`covered_sha` in-payload. Anchor to a named follow-up story when that schema evolution is
roadmapped.

### D-846 authorization scope disposition

The D-846 authorization (design-brief-post-e19-host-abi-fixes.md) covered four finding
classes: EC-006 (production-path registration gap, D19), the SEC-003 -4 schema question
(ADR-025 Decision 19 adjudication), F-WG-002 (bare literals, D21 named constants), and
F-WG-003 (missing timestamp field, D22). All four were closed by S-19.09 (PR #659, commit
13ece92c). ADR-025 Decision 19 is the authoritative record of the INVALID_ARGUMENT (-4)
schema question: marshalling-internal, not operator-visible, correctly absent from the
`read_prefix` capability schema preamble table. No residual open items remain from the
D-846 authorization scope.


## 4. Carried S-19.09 Anchored Records

The following three items were accepted-with-record at the S-19.09 merge gate (adversary
pass-13, CONVERGED B0/H0/M0/L0). They were assessed as outside the S-19.09 diff
perimeter — not deferrals of in-scope work. Each item carries a routing recommendation
for E-20 story decomposition.

### Item 1: emit_dispatcher_schema_mismatch doc/code field name drift

**Description:** The `emit_dispatcher_schema_mismatch` event emitter carries a doc comment
that names a `fields` array as its wire payload field. The actual wire format uses `field`
(singular). The emitter behavior is correct; the doc comment is wrong. This is purely
documentary drift with no behavioral consequence and no operator-visible contract
violation.

**Severity:** LOW

**Routing recommendation:** `vsdd-factory:implementer` — one-line doc comment correction
to align the comment's field name with the emitter's actual wire format. No test change
required unless a value-pin test on the field name is warranted (judgment call for the
implementer).

**Story anchor:** E-20 polish / host-ABI documentation wave.

### Item 2: darwin mktemp non-trailing-Xs pattern in resolver-integration.bats

**Description:** A `mktemp` invocation in `resolver-integration.bats` uses a suffix pattern
that does not end in `XXXXXX` (trailing Xs). GNU mktemp (Linux) accepts non-trailing-X
patterns and replaces the Xs regardless of position. darwin mktemp requires trailing Xs;
a non-trailing pattern produces a predictable, non-unique suffix — a test-isolation risk
and a source of latent CI divergence between darwin and linux runs.

**Severity:** LOW (current darwin CI passes because the pattern does not collide in
practice; the risk is latent rather than active)

**Routing recommendation:** `vsdd-factory:devops-engineer` — update the `mktemp` call to
a portable trailing-Xs form such as `mktemp -t resolver.XXXXXX` or `mktemp
/tmp/resolver.XXXXXX`. Applies to all similar `mktemp` calls in the bats suite if a
portability sweep is in scope.

**Story anchor:** E-20 bats portability / test-infra hygiene wave.

### Item 3: memory.grow-failure branch untested for read_file and read_prefix

**Description:** Both the `read_file.rs` and `read_prefix.rs` handlers contain a
`memory.grow()` failure branch — the path taken when `memory.grow()` returns `None`,
meaning the WASM guest has reached the 4 GiB linear memory ceiling. This path returns
`INTERNAL_ERROR (-99)`. No test currently exercises this branch for either entry point.
The branch is reachable in production under adversarial inputs (large-file reads against
memory-constrained plugins). The exit code is assumed to be -99 but is not mechanically
verified.

**Severity:** MEDIUM — reachable production path; untested; exit code assumption unverified

**Routing recommendation:** `vsdd-factory:test-writer` — write a memory-budget fixture
test that preallocates WASM memory pages to exhaust the 4 GiB ceiling, then invokes
`read_file` / `read_prefix` on a file large enough to require memory growth, and asserts
the host function returns `INTERNAL_ERROR (-99)`. Follow with `vsdd-factory:implementer`
if the test reveals any behavioral gap (unexpected return code or panic path).

**Story anchor:** E-20 host-ABI hardening wave; host-function boundary testing.


## 5. Standing RELEASE-GATE Item — Linux/Windows Binary Staleness

### State as of epic close (2026-07-17)

| Platform | Bundle state | Includes E-19 changes |
|----------|-------------|----------------------|
| darwin-arm64 | Rebuilt at 4d4f60bc (2026-07-17) | YES |
| darwin-x64 | Rebuilt at 4d4f60bc (2026-07-17) | YES |
| linux-x64 | Stale at a04cb303 (rc.22 era) | NO |
| linux-arm64 | Stale at a04cb303 (rc.22 era) | NO |
| windows-x64 | Stale at a04cb303 (rc.22 era) | NO |

### Architectural risk assessment

The stale linux/windows binaries pre-date the `ReadPrefixCaps` struct field added to the
`Capabilities` type in S-19.06. The `Capabilities` struct uses `#[serde(deny_unknown_fields)]`.

S-19.07 (PR #670, the last story to merge) added `[hooks.capabilities.read_prefix]` blocks
to `hooks-registry.toml` as part of the verify-factory-lock Phase-B migration. The
`read_prefix` key maps to `ReadPrefixCaps` in the struct. A linux or windows operator who
installs from the current develop branch state will have:

- The updated `hooks-registry.toml` (with `[hooks.capabilities.read_prefix]` entries)
- The rc.22 binary (which does not know the `ReadPrefixCaps` struct field)

The rc.22 binary will fail to parse the registry at startup with a serde deserialization
error (`unknown field "read_prefix"` under `deny_unknown_fields`). This is a hard
REGISTRY-LOAD FAIL — the entire hook chain becomes inert rather than silently degrading.
The fail mode is detectable (the dispatcher errors on startup) but not self-healing.

**Scope boundary:** This risk is bounded to operators who install from develop directly.
The marketplace tarball at `~/.claude/plugins/cache/claude-mp/vsdd-factory/` is
version-pinned; it updates only when a release is published. CI builds from source and is
unaffected. The darwin operator path is unblocked (binaries are post-S-19.09).

**Classification:** RELEASE-GATE BLOCKER under the production-grade default.

### Mitigation

The only correct mitigation is cutting a new release (rc.23 or v1.0.0 GA) that triggers
the `release.yml` cross-compilation pipeline across all five platforms. No workaround
exists for a linux/windows operator trying to use develop-branch binaries against the E-19
`hooks-registry.toml`.

As of 2026-07-17, develop is at 6db4c9fc with all nine E-19 stories merged and CI green.
This is the correct base for a release cut. The next release should be treated as
mandatory for linux/windows operator usability, not discretionary timing.


## 6. E-20 Candidates from the Architecture Lens

These are architecture-derived inputs to E-20 story decomposition. Story authorship,
wave scheduling, and acceptance criteria remain with `vsdd-factory:story-writer` and
`vsdd-factory:product-owner`. Items are ordered by priority within tier.

### High priority

**E-20-ARCH-01: Cut release rc.23 / v1.0.0 — rebuild all-platform binaries**

Prerequisite for linux/windows operator usability. The `release.yml` pipeline handles
cross-compilation automatically. Prerequisites: factory-artifacts in clean state (no open
fix bursts), CI green at develop tip, RELEASING.md checklist satisfied.

Routing: `vsdd-factory:devops-engineer` (release execution);
`vsdd-factory:state-manager` (post-release factory-artifacts state advance).

---

**E-20-ARCH-02: memory.grow-failure branch test coverage for read_file and read_prefix**

Carried from S-19.09 Item 3 above. MEDIUM severity hardening gap. The
`INTERNAL_ERROR (-99)` memory.grow failure path in both `read_file.rs` and `read_prefix.rs`
is reachable in production but has never been exercised by any test. Exit code assumption
is unverified.

Routing: `vsdd-factory:test-writer` (write boundary test fixture);
`vsdd-factory:implementer` (fix if test reveals gap).

### Medium priority

**E-20-ARCH-03: S-15.03 PRIORITY-A structured changelog migration**

Root disease for the STATE.md byte-bloat that drove the 65536→262144 cap escalations in
S-19.02 and S-19.08: the inlined `last_amended` string in STATE.md frontmatter grows
unboundedly with each amendment. ADR-025 §Decision 17 adjudicated 262144 bytes as
structurally sufficient under the current structural guarantee (on-envelope STATE.md has
its closing `---` within the first 262144 bytes). That guarantee holds only while
`last_amended` growth remains within the envelope — it is a bounded-approximation fix, not
a structural cure. S-15.03 PRIORITY-A (structured `changelog:` array replacing inlined
`last_amended`) eliminates this constraint entirely by removing the unbounded field.

Until S-15.03 PRIORITY-A is delivered, the 262144-byte cap is the architectural safety
valve. Any future story that causes STATE.md to exceed 262144 bytes to its first closing
`---` will silently inert the verify-factory-lock guard (the same failure mode that
motivated S-19.02).

Routing: `vsdd-factory:story-writer` (scope S-15.03);
`vsdd-factory:state-manager` (execute migration);
`vsdd-factory:product-owner` (BC-4.13.001 Phase-B Precondition 3 cleanup post-migration).

---

**E-20-ARCH-04: ADR-030 §D2+D3 orchestrator obligation integration test**

`check-stale-verdict.sh` must be called before every merge. This obligation is codified in
ADR-030 §Decision 2 and §Decision 3 but has no automated gate that enforces or verifies
the orchestrator honored it. A simulated-merge integration test that presents a stale
verdict and asserts the script blocks is missing, leaving the enforcement gap as a
documentation-only claim.

Routing: `vsdd-factory:test-writer` + `vsdd-factory:devops-engineer`.

### Low priority

**E-20-ARCH-05: emit_dispatcher_schema_mismatch doc comment correction**

Carried from S-19.09 Item 1. The `emit_dispatcher_schema_mismatch` doc comment names a
`fields` array; the wire format uses `field` singular. One-line doc comment correction.

Routing: `vsdd-factory:implementer`.

---

**E-20-ARCH-06: Portable mktemp pattern in resolver-integration.bats**

Carried from S-19.09 Item 2. Non-trailing-Xs mktemp pattern is a latent darwin/linux
portability divergence in the bats test suite. Correct to a trailing-Xs portable form;
consider a portability sweep across the full bats suite.

Routing: `vsdd-factory:devops-engineer`.

---

**E-20-ARCH-07: ADR-030 §D3 Rust migration deferral anchor story**

No implementation required. The architectural prerequisite for a WASM migration of
`check-stale-verdict.sh` and `enforce-merge-strategy.sh` is SubagentStop `tool_input`
exposing `covered_sha` in-payload. Until that prerequisite exists, the migration cannot
close the functional requirement. A named follow-up story should be created to anchor
the deferral and surface the prerequisite, preventing the question from being reopened
without new evidence.

Routing: `vsdd-factory:architect` (ADR-030 amendment to record the deferral anchor with
explicit prerequisite; no implementation needed in E-20).

---

**E-20-ARCH-08: HOST_ABI_VERSION bump policy codification**

HOST_ABI_VERSION=1 has been stable throughout E-19 because all changes were additive. The
two-linker `out_ptr = 0` protocol (ADR-025 §Decision 17) creates a behavioral asymmetry
that is correct by contract but undocumented in the operator-visible capability schema. If
future stories require non-additive ABI changes, a VERSION bump policy should be codified
before the first non-additive change lands. The policy does not need to be a full story —
a lightweight ADR-025 amendment capturing the bump criteria (what constitutes a breaking
change, migration window obligations) is sufficient.

Routing: `vsdd-factory:architect` (ADR-025 amendment; no implementation in E-20).
