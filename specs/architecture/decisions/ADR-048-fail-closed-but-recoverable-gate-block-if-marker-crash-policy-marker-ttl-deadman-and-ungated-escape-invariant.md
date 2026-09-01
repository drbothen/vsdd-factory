---
document_type: adr
adr_id: ADR-048
version: "1.2"
title: "ADR-048: Fail-Closed-But-Recoverable Gate — block_if_marker Crash Policy, Marker TTL Deadman, and Ungated-Escape Invariant"
status: accepted
date: 2026-08-31
producer: architect
timestamp: 2026-08-31T00:00:00Z
deciders:
  - architect
  - human (directed redesign 2026-08-31)
subsystems_affected: [SS-01, SS-04, SS-07]
supersedes: ADR-047
superseded_by: null
extends: ADR-047
traces_to: .factory/specs/architecture/ARCH-INDEX.md
last_amended: "2026-08-31 (v1.2) — Architect-directed (S-25.01 LOCAL adversary pass 2 F-P2-002 HIGH + F-P2-003 MED resolution): §Decision 4 emission-point architecture corrected. Root cause: `marker.cleared` emitted via the WASM `emit_event` host ABI is subject to RESERVED_FIELDS enrichment (`crates/factory-dispatcher/src/host/emit_event.rs`) — the host unconditionally overwrites plugin-supplied `trace_id`/`plugin_name` with the CURRENT gate-plugin's own dispatch identity, so a WASM plugin can never emit an event carrying a FOREIGN (marker-owned) trace_id/plugin_name. TTL_EXPIRED detection+auto-delete+emission is MOVED from the WASM gate plugin's `evaluate_gate` to a new dispatcher-native pre-check (`indeterminate_marker.rs`) that runs before every Arm 1/Arm 2 plugin invocation on the normal (non-crash) path, mirroring the already-correct REVALIDATED emission architecture. OPERATOR_OVERRIDE/RAW_DELETE_DETECTED reconciliation (previously entirely unimplemented) is likewise implemented dispatcher-native, in the same pre-check's marker-absent branch, with a bounded/best-effort FileSink scan. `evaluate_gate` is simplified to a pure presence check (no `expires_at` math, no delete, no emission). PROPOSED — awaiting human ratification (unchanged from v1.0/v1.1; this is a further pre-ratification revision, not a reopening of an already-ratified decision). [Prior: 2026-08-31 (v1.1) — Human-directed (HIGH-1 resolution): §Decision 3 amended — recovery model reframed (re-validation = primary agent recovery; human out-of-band rm = break-glass; agent-tool rm de-sanctioned; shared-crate fix rejected as unnecessary + unsound per Rice's theorem); §Decision 4 added — marker.cleared audited event (clear_mode ∈ {REVALIDATED,TTL_EXPIRED,OPERATOR_OVERRIDE}; trace_id linkage; RAW_DELETE_DETECTED reconciliation mode) + TTL-loudness. [v1.0 — Initial authoring. Human-directed gate redesign reversing D-1135 fail-open-on-crash ratification.]]"
modified:
  - "2026-09-01 (status: proposed→accepted) — Human-Ratified (POLICY 22; D-1139; state-manager; no content change, status flip only)"
  - "2026-08-31 (v1.2) — §D4 emission-point correction: TTL_EXPIRED + OPERATOR_OVERRIDE moved dispatcher-native (S-25.01 pass-2 F-P2-002/F-P2-003)"
  - "2026-08-31 (v1.1) — §D3 recovery model reframe + §D4 audited clear event + TTL-loudness (HIGH-1 resolution)"
  - "2026-08-31 (v1.0) — Initial authoring"
---

<!-- BROWNFIELD: You MUST cite implementation evidence (file:line from crates/ or
     legacy-design-docs/) before this ADR can be accepted. Omitting evidence is a
     template-compliance failure. -->

# ADR-048: Fail-Closed-But-Recoverable Gate — block_if_marker Crash Policy, Marker TTL Deadman, and Ungated-Escape Invariant

## Status

**ACCEPTED — Human-Ratified 2026-09-01** (POLICY 22 — ratification complete. S-25.01 LOCAL adversary
pass 2 F-P2-001/F-P2-002/F-P2-003 fix-burst COMPLETE; ADR-048 v1.0/v1.1/v1.2 all ratified as a
single decision, v1.2's emission-point correction included. D-1139.)

POLICY 22 ratification (2026-09-01) is to be recorded authoritatively in the decision-log (D-1139)
by state-manager in the spec-burst commit. The ADR frontmatter `status: accepted` reflects the
architectural decision; the decision-log entry is the authoritative ratification record.

**v1.2 amendment (2026-08-31 — S-25.01 LOCAL adversary pass 2 F-P2-002/F-P2-003 resolution):**
§Decision 4's emission-point architecture is corrected. F-P2-002 (HIGH) found that the
TTL_EXPIRED `marker.cleared` emission, as specified in v1.1, was placed inside the WASM gate
plugin — but the `emit_event` host ABI's RESERVED_FIELDS enrichment (`trace_id`, `plugin_name`)
makes it STRUCTURALLY IMPOSSIBLE for a WASM plugin to emit an event carrying another dispatch's
identity: the host always overwrites plugin-supplied `trace_id`/`plugin_name` with the CURRENT
invocation's own values (the gate plugin's own trace and name), never the marker's. The v1.0
draft implementation's workaround (`marker_trace_id`/`marker_plugin_name` custom fields +
`reason=""`) is not the Event 9 wire contract and silently breaks trace-id correlation for the
deadman path. F-P2-003 (MED) found OPERATOR_OVERRIDE/RAW_DELETE_DETECTED reconciliation entirely
unimplemented, and noted it would hit the identical ABI wall if implemented WASM-side. This
amendment moves BOTH TTL_EXPIRED and OPERATOR_OVERRIDE emission to dispatcher-native code
(`indeterminate_marker.rs`), mirroring the already-correct REVALIDATED architecture (dispatcher
`emit_marker_cleared`, which sets `.with_trace_id(&marker_fields.trace_id)` directly via
`InternalLog`, entirely bypassing the WASM `emit_event` ABI and its RESERVED_FIELDS filter) and
`plugin.indeterminate` (Event 8, also dispatcher-native per BC-3.08.001 Event 8 trigger:
`invoke_plugin`/`executor.rs`). See the amended §Decision 4 subsections below for the corrected
mechanism.

**v1.1 amendment (2026-08-31 — HIGH-1 resolution):** §Decision 3 recovery model reframed: re-
validation via Edit/Write elevated as primary sanctioned agent recovery (T1 — inherently ungated);
human out-of-band `rm` formalized as break-glass (T3 — never intercepted); agent-tool `rm` de-
sanctioned (crash-path blockage ACCEPTABLE — not INV6 violation; agent's real recovery unaffected);
shared-crate/native `rm`-filter rejected as unnecessary + unsound (Rice's theorem; same undecidable
class as BC-1.18.002 §out-of-scope). §Decision 4 added: `marker.cleared` audited event
(REVALIDATED / TTL_EXPIRED / OPERATOR_OVERRIDE) + TTL-loudness (emit `marker.cleared(TTL_EXPIRED)`
on auto-delete, replacing silent clear). Nine-event dispatcher domain model (BC-3.08.001).

Human-directed redesign (2026-08-31): reverse D-1135 fail-open-on-crash ratification and replace
with fail-closed-but-recoverable design for the S-25.01 unvalidated-mutation next-advance gate.
This ADR SUPERSEDES ADR-047 §Decision 4 (gate `failure_policy = "fail-open"` on crash) and D-1135
(the ratification of that fail-open posture). All other ADR-047 decisions remain in force.

This ADR EXTENDS ADR-047 and through it ADR-039. It does not amend ADR-039's normative content.
ADR-039 remains the authoritative source for the `failure_policy` field schema, the two-axis model,
and the axes-independence invariant. ADR-047 remains the authoritative source for Decisions 1–3
and 5–9. ADR-048 supplants only ADR-047 §Decision 4's crash behavior (the `failure_policy =
"fail-open"` on-crash posture) and BC-1.18.002 §INV2's corollary that a crash → unconditional
allow.

**Partial supersession scope:** ADR-047 Decisions 1, 2, 3, 5, 6, 7, 8, 9 are UNCHANGED. Only
ADR-047 §Decision 4's on-crash behavior is superseded by ADR-048 §Decision 1.

---

## Context

### D-1135 Ratification and Its Limitation

ADR-047 §Decision 4 established the `validate-unvalidated-mutation-marker` gate with
`failure_policy = "fail-open"` (in earlier BC versions `on_error = "block"` was briefly
incorrect; BC-1.18.002 v1.3 corrected this to `on_error = "continue"`). The on-crash semantics
were: if the WASM gate plugin itself cannot complete (fuel exhaustion, epoch timeout, crash),
the current dispatch proceeds unconditionally — fail-open-on-crash.

The rationale recorded in D-1135 and ADR-047 §Decision 4 was: "a fail-open PreToolUse gate that
guards Agent dispatch must not introduce a new unconditional self-lock class." This reasoning is
correct but incomplete. The problem it avoids — unconditional self-lock — is specifically the
case where NO marker exists. When a marker DOES exist, fail-open-on-crash defeats the entire
quarantine-forward design: a plugin that crashes while a valid marker is present silently allows
the state-advancing dispatch that the marker was designed to block.

### The Two Failure Sub-Cases of a Gate Crash

A gate plugin crash has two distinct sub-cases that were previously conflated under a single
`on_error = "continue"` policy:

| Sub-case | Marker state | D-1135 behavior | Correct behavior |
|----------|-------------|-----------------|-----------------|
| Crash + no marker | Absent | Allow (fail-open) | Allow — nothing to enforce |
| Crash + marker exists | Present | Allow (fail-open) ← **the defect** | Block — marker is the quarantine signal |

Treating both sub-cases identically is a CWE-636 (Fail Open) defect specifically in the
crash+marker-exists case. The marker is the DURABLE quarantine signal, written by the dispatcher
itself (not by the crashable WASM plugin), to a plain file on the filesystem. The dispatcher can
perform a NATIVE, non-WASM filesystem existence check on that file without depending on the WASM
plugin at all. This native check is crash-proof relative to the plugin.

### The Self-Lock Hazard Is Solvable Without Abandoning Fail-Closed

The original self-lock concern was: if the gate crashes and we block unconditionally, we create
an unclearable dead-end. ADR-048 resolves this in three orthogonal ways:

1. **block_if_marker (Decision 1):** The native check blocks ONLY when a real (non-expired)
   marker exists. Crash with no marker → allow (no self-lock, nothing to protect).

2. **Marker TTL deadman (Decision 2):** Any residual stuck marker auto-expires after 24 hours,
   bounding the worst-case stuck state without operator action.

3. **Ungated-escape invariant (Decision 3):** The three recovery operations (`rm` the marker,
   re-validate via Edit/Write, wait for TTL expiry) are NEVER gated by this plugin, guaranteeing
   the system is always escapable in place.

### Relationship to ADR-039 Two-Axis Model

ADR-039 §Decision 1 established two orthogonal registry axes:
- `on_error`: behavior when the plugin ITSELF cannot complete (crash, fuel exhaustion, epoch
  timeout); values `"block"` (block current dispatch unconditionally) and `"continue"` (allow).
- `failure_policy`: behavior for PostToolUse INDETERMINATE outcomes; values `"fail-closed"` (write
  durable marker + trigger gate) and `"fail-open"` (advisory event only, no marker).

ADR-048 adds a third `on_error` value: `"block_if_marker"`. This EXTENDS the existing axis; it
does NOT collapse or unify the two axes. The orthogonality invariant is preserved. The new value
is: "on crash, check the marker file natively; block if marker exists and non-expired, allow
otherwise." It is most meaningful for PreToolUse gate plugins that guard against marker presence —
it would be semantically meaningless for a PostToolUse validator.

---

## Decisions

### Decision 1 — block_if_marker: New on_error Value for Conditional Fail-Closed on Crash

**A new `on_error` value `"block_if_marker"` is added to the registry field schema.** When a
plugin registered with `on_error = "block_if_marker"` produces a crash, fuel-exhaustion, or
epoch-timeout outcome, the dispatcher executes the following NATIVE (non-WASM) codepath:

```
block_if_marker_handler(marker_path, now):
  if marker_path does not exist → Allow
  parse marker TOML content:
    if parse fails or expires_at field absent → treat as non-expired (conservative)
    if expires_at ≤ now (UTC) → Allow  (expired marker → treat as absent)
    else → Block (marker present and non-expired → fail-closed)
```

**Crash + no marker → Allow.** Nothing to enforce; blocking here creates the unconditional
self-lock that ADR-047 §Decision 4 was designed to avoid.

**Crash + marker exists + non-expired → Block.** The marker is the durable quarantine signal.
The dispatcher wrote it (not the WASM plugin), so a native check is crash-proof relative to the
plugin. This is the fail-closed path for real quarantine events.

**Crash + marker exists + TTL expired → Allow.** The marker has self-healed (see Decision 2). No
longer a valid quarantine signal; treat as absent.

**Block message on crash-path block:** The dispatcher emits a structured block message including
the marker's `plugin_name`, `artifact_path`, `cause`, and `expires_at` fields plus the recovery
options: `rm .factory/unvalidated-mutation.marker` or wait for TTL expiry.

**Registry change for S-25.01:**

```toml
# Arm 1: Agent dispatch gate
[[hook]]
name = "validate-unvalidated-mutation-marker"
plugin = "hook-plugins/validate-unvalidated-mutation-marker.wasm"
event = "PreToolUse"
tool = "^Agent$"
on_error = "block_if_marker"        # ADR-048: replaces "continue" (D-1135)
async = false
failure_policy = "fail-open"

# Arm 2: git commit/push durable-propagation gate
[[hook]]
name = "validate-unvalidated-mutation-marker-git"
plugin = "hook-plugins/validate-unvalidated-mutation-marker.wasm"
event = "PreToolUse"
tool = "^Bash$"
on_error = "block_if_marker"        # ADR-048: replaces "continue" (D-1135)
async = false
failure_policy = "fail-open"
```

**Mechanism choice:** `"block_if_marker"` as a new `on_error` value is PREFERRED over a
dispatcher special-case keyed to this gate's name. Rationale: (a) the value is reusable by any
future gate that follows the same marker-based quarantine pattern; (b) named special-cases are
fragile (plugin rename → special case silently breaks); (c) `on_error` is the correct axis for
crash behavior per ADR-039's two-axis model.

**Reconciliation with ADR-039:**

| on_error value | Crash behavior |
|----------------|---------------|
| `"block"` (existing) | Block unconditionally |
| `"continue"` (existing) | Allow unconditionally |
| `"block_if_marker"` (NEW) | Block iff marker exists and non-expired (native check) |

The `failure_policy` axis is UNCHANGED. `"block_if_marker"` is a value on the `on_error` axis
only. The three existing `failure_policy` values (`"fail-closed"`, `"fail-open"`, absent) are
unaffected. The axes-independence invariant from ADR-039 §Decision 1 is preserved.

**Marker path for native check:** `.factory/unvalidated-mutation.marker` — same constant as
ADR-047 §Decision 3. The native check reads the same path the `indeterminate_marker` module
writes. Implementation: the crash-handler in `crates/factory-dispatcher/src/executor.rs` calls
`block_if_marker_check(factory_root, now)` when the plugin result is a crash/timeout and
`on_error == OnError::BlockIfMarker`.

**Existence and expiry determination:** `std::fs::try_exists(marker_path)` for existence check.
For TTL, read file content, parse TOML, extract `expires_at` field as ISO-8601 UTC string,
compare to `chrono::Utc::now()`. Parsing failures → treat as non-expired (conservative; no
crashes on old markers lacking the field).

### Decision 2 — Marker TTL Deadman: expires_at Field and 86400-Second Constant

**The `.factory/unvalidated-mutation.marker` TOML content gains a REQUIRED `expires_at` field,**
written by the dispatcher at marker creation time:

```toml
timestamp = "<ISO-8601 UTC timestamp of the INDETERMINATE event>"
plugin_name = "<name field from the [[hook]] entry in hooks-registry.toml>"
artifact_path = "<absolute path of the artifact that was written>"
cause = "<one of: fuel | epoch | output-too-large>"
trace_id = "<dispatcher trace_id of the event that produced INDETERMINATE>"
expires_at = "<ISO-8601 UTC timestamp = timestamp + 86400s>"
```

**TTL constant:** `UNVALIDATED_MUTATION_MARKER_TTL_SECONDS: u64 = 86_400` (24 hours), defined in
`crates/factory-dispatcher/src/indeterminate_marker.rs`. `expires_at` is computed as
`timestamp + Duration::seconds(UNVALIDATED_MUTATION_MARKER_TTL_SECONDS as i64)`.

**Relationship to factory_lock TTL (2700 seconds / 45 minutes):**

The factory_lock TTL is a KEEP-ALIVE signal. It is refreshed on every STATE.md write via the
`renew_lock_with_now` function in `crates/factory-lock/src/lib.rs`. The hardcoded value
(`Duration::seconds(2700)` in `renew_lock_with_now`) represents "will expire 45 minutes after the
most recent state write" — it is NOT a deadman; it is actively renewed. There is no public
`TTL_SECONDS` constant in the `factory-lock` or `factory-lock-parse` crates to reuse.

The mutation marker TTL is a DEADMAN CIRCUIT: written ONCE, no renewal, decays unconditionally.
The 86400-second value is chosen for different reasons than the 2700-second factory_lock value:

| Property | factory_lock TTL (2700s) | mutation marker TTL (86400s) |
|----------|-------------------------|------------------------------|
| Semantics | Keep-alive; renewed on each state write | Deadman; written once, decays |
| Purpose | Prevent stale lock orphans | Bound worst-case stuck quarantine |
| Expected lifetime | Continuously renewed while session active | One-shot; max 24h from write |
| Self-healing? | No (renewed = intentionally not expired) | Yes (expires after 24h) |

The two constants MUST NOT be unified. They model orthogonal expiry semantics.

**Who stamps `expires_at`:** The dispatcher's `write_indeterminate_marker` function in
`crates/factory-dispatcher/src/indeterminate_marker.rs`.
Value: `Utc::now() + Duration::seconds(UNVALIDATED_MUTATION_MARKER_TTL_SECONDS as i64)`.

**Who checks `expires_at` (v1.2 — corrected; supersedes the v1.0/v1.1 "gate plugin normal path"
assignment per the Decision 4 v1.2 Emission-Point Correction below):**

1. **Dispatcher-native pre-check (normal path — MOVED here in v1.2; was WASM gate plugin in
   v1.0/v1.1):** `check_and_clear_expired_marker` in `indeterminate_marker.rs`, invoked from
   `executor.rs`'s tier-execution loop before every Arm 1/Arm 2 (`on_error = "block_if_marker"`)
   plugin invocation. Reads the marker natively, parses `expires_at`. If `expires_at <= now
   (UTC)`: auto-delete the marker file (idempotent; swallow `NotFound`) and emit
   `marker.cleared(TTL_EXPIRED)` (Decision 4). The subsequent WASM plugin invocation then sees a
   marker that is guaranteed either absent or non-expired — `evaluate_gate` performs NO
   `expires_at` parsing of its own (v1.2 simplification; see Decision 4). The auto-delete
   prevents the marker from accumulating as a dead artifact, exactly as in v1.0/v1.1 — only the
   locus of the check has moved.

2. **Dispatcher native `block_if_marker` check (crash path, Decision 1 — UNCHANGED by v1.2):**
   After detecting plugin crash and finding the marker file exists, parses `expires_at`. If
   expired: allow (treat as absent). If `expires_at` field absent (backward-compat: old marker
   written before ADR-048 implementation): treat as non-expired (conservative — old markers are
   not silently cleared). Does NOT auto-delete (keep crash handler simple) and does NOT emit
   `marker.cleared` (BC-1.18.003 EC-014; VP-108 PC4) — this remains a genuinely distinct code
   path from item 1 above, since a crash means the pre-check in item 1 either did not run or was
   interrupted before completing.

**Backward compatibility:** Markers written before ADR-048 implementation lack `expires_at`. These
are treated as non-expired by both the plugin (normal path) and the dispatcher native check (crash
path). They remain in effect until explicitly cleared via `rm` or until ADR-048 is implemented and
new markers with `expires_at` replace them. This is the conservative, production-grade default.

**Interaction with BC-1.18.003 artifact-scoped clear (v1.1):** TTL expiry is a THIRD clear path
in addition to successful re-validation (PC1) and operator manual rm (PC3). The artifact-scoped
clear condition (same plugin + same non-empty artifact_path) applies only to the re-validation
path (PC1). TTL expiry is marker-level, not artifact-scoped: any marker expires after 86400s
regardless of which artifact triggered the INDETERMINATE event.

**Interaction with INV3 (single-marker policy, BC-1.18.001):** Single-marker policy is preserved.
The `expires_at` field is additional TOML content in the same single marker file. Last-writer-wins
on marker overwrite (existing semantics) means the new `expires_at` field is always written with
the latest event's timestamp, so the TTL always runs from the most recent INDETERMINATE event.

### Decision 3 — Ungated-Escape Invariant (amended v1.1 — Recovery Model and Recoverability Tiers)

**(v1.1 amendment)** The prior v1.0 formulation classified agent-tool `rm` as an ungated escape
path (path (i)). Crash-path analysis reveals this claim does not hold under `on_error =
"block_if_marker"`: on a gate-plugin crash, the dispatcher's native `block_if_marker_check` runs
for ALL `^Bash$` dispatches — including a Bash `rm` of the marker — and blocks if the marker is
present and non-expired. Prior-art alignment (circuit-breaker reset, EDR Restore, Google
break-glass, GitLab dequarantine, NIST AU/AC, LOTO/e-stop) establishes that the controlled agent
must NOT hold an unmediated, unaudited path to erase its own constraint; recovery comes from
independent RE-VALIDATION, an authenticated HUMAN override, or a TIMEOUT. The recovery model is
reframed accordingly.

**Four-tier recovery model:**

| Tier | Actor | Mechanism | Gate status |
|------|-------|-----------|-------------|
| **T1 — Primary agent recovery** | Agent | Re-validate artifact via Edit/Write → gate plugin PASSes → `delete_marker_if_pass` clears marker | **Inherently ungated:** Edit/Write match neither `^Agent$` nor `^Bash$`; no arm fires; crash-path native check not invoked. Holds even through a gate-plugin crash. |
| **T2 — Passive recovery** | Deadman | Wait 24h; marker TTL expires → treated as absent on next gate eval | **No dispatch needed.** TTL self-heals unconditionally; `marker.cleared(TTL_EXPIRED)` emitted (Decision 4). |
| **T3 — Human out-of-band recovery** | Operator | `rm .factory/unvalidated-mutation.marker` in the operator's own terminal | **Never intercepted.** PreToolUse gate only intercepts agent tool calls; the human's shell is not mediated by the dispatcher. |
| **T4 — Agent-tool `rm` (de-sanctioned)** | Agent | `Bash(rm .factory/unvalidated-mutation.marker)` via dispatcher | **NOT sanctioned; may be blocked on crash path.** Not a recovery path the invariant relies upon; see "Agent-tool rm de-sanctioned" below. |

**T1 — Re-validation is the primary sanctioned agent recovery.**

When the agent re-writes or re-validates the artifact that triggered the INDETERMINATE event (via
Edit or Write → gate plugin's `evaluate_gate` PASSes with exit_code=0 → `delete_marker_if_pass`
removes the marker file), this path is INHERENTLY ungated:

- Edit and Write tool calls are neither `^Agent$` nor `^Bash$`.
- Arm 1 tool filter (`^Agent$`) does not match Edit/Write.
- Arm 2 tool filter (`^Bash$`) does not match Edit/Write.
- The crash-path native `block_if_marker_check` runs only when a matched arm's plugin crashes.
  Since no arm matches Edit/Write, the crash check never runs for Edit/Write dispatches.

This is invariant to: (a) the gate plugin's crash behavior; (b) the marker's presence or expiry
state; (c) any `on_error` value on any arm. A future refactor breaks this ONLY by adding Edit or
Write to a gate arm's tool pattern — which VP-107 explicitly verifies will not occur.

**(Reaffirmed — BC-1.18.002 PC3):** Non-advancing dispatches are NOT gated per BC-1.18.002 PC3
and ADR-047 §Decision 4. Edit and Write tool dispatches do not match either arm's `tool` pattern.
Confirmed ungated by construction — now formalized as the PRIMARY agent recovery path.

**T2 — TTL deadman.**

After `UNVALIDATED_MUTATION_MARKER_TTL_SECONDS` (86400) seconds have elapsed, any gate evaluation
treats the marker as absent and allows the dispatch. No operator action required. The
`marker.cleared(TTL_EXPIRED)` event is emitted on auto-delete (Decision 4 — TTL-loudness).
Confirmed ungated — no dispatch needed to trigger TTL expiry.

**T3 — Human out-of-band rm.**

The PreToolUse hook chain is invoked only when the AI agent calls a tool through the dispatcher.
A terminal `rm` executed by the human operator in their own shell is not a dispatcher-mediated
agent tool call and is never intercepted by the hook chain. This is the break-glass pattern: the
human principal, acting outside the mediation boundary, retains unconditional filesystem access.
The `marker.cleared(OPERATOR_OVERRIDE)` event is reconciled retrospectively via RAW_DELETE_DETECTED
(Decision 4). Confirmed ungated — human shell is outside dispatcher mediation.

**Agent-tool rm de-sanctioned:**

The v1.0 §Decision 3(i) analyzed agent-tool `rm` via the Arm 2 `is_git_commit_or_push` filter:

- Phase 1: no shell operators; single segment.
- Phase 1b: tokenize → `["rm", ".factory/unvalidated-mutation.marker"]`.
- Phase 2: executable = `rm`; `basename("rm") = "rm" != "git"` → return `false`.

`is_git_commit_or_push` returns `false` → Arm 2 plugin does NOT gate `rm` on the **normal path**.
This analysis remains correct for the non-crash path.

On the **crash path** (`on_error = "block_if_marker"`): when the `^Bash$` gate plugin crashes, the
dispatcher's native `block_if_marker_check` runs for EVERY `^Bash$` dispatch — including
`rm .factory/unvalidated-mutation.marker`. The native check has no `is_git_commit_or_push` filter.
If the marker is present and non-expired, the native check BLOCKS the `rm` dispatch.

**CONSEQUENCE:** The crash path blocking the agent's Bash `rm` is ACCEPTABLE — NOT an INV6
violation. Reasoning:

1. The agent's primary recovery (T1 — re-validation via Edit/Write) is GENUINELY UNGATED even
   during a gate-plugin crash. The agent can always recover without `rm`.
2. A command-filter to whitelist `rm` in the crash handler is UNSOUND (see "Shared-crate fix
   rejected" below) and UNNECESSARY given T1.
3. On the crash path, blocking is the conservative-correct default: gate crashing while a marker
   is present is an anomalous condition; allowing the agent to erase its own constraint in this
   state would be a CWE-636 regression.

**Shared-crate fix rejected:**

A proposed mitigation was a shared-crate or native command-filter whitelisting
`rm .factory/unvalidated-mutation.marker` in the `block_if_marker` crash handler. REJECTED as
UNNECESSARY AND UNSOUND:

- **Unnecessary:** T1 (re-validation via Edit/Write) provides the agent's genuine ungated recovery.
  No second agent-tool escape path is needed.
- **Unsound per Rice's theorem:** Any command-filter classifying "this Bash dispatch deletes the
  marker" is undecidable in the general case. `mv .factory/unvalidated-mutation.marker /tmp/x`,
  `truncate -s 0 .factory/unvalidated-mutation.marker`, `python -c "import os;
  os.remove('.factory/unvalidated-mutation.marker')"` all neutralize the marker but evade an
  `rm`-specific filter. The same undecidable class is documented in BC-1.18.002 §out-of-scope and
  already accepted as out of scope for the `is_git_commit_or_push` filter.
- **Unnecessary surface expansion:** command-filter logic in the crash handler adds complexity to
  a security-critical codepath with no commensurate benefit.

**Recoverability invariant (three independent paths):**

T1, T2, and T3 provide three independent, complementary recovery paths not susceptible to the
same failure mode:

- T1 (re-validation) fails only if the artifact itself is irrecoverable (pathological).
- T2 (TTL) fails only if the system clock is broken (pathological).
- T3 (human rm) fails only if the operator cannot access the filesystem (outside the threat model
  for a single-operator factory).

No single failure mode can disable all three paths. The gate cannot create an irrecoverable state.

**VP-107 scope (amended):**

VP-107 is amended to verify T1: the Edit/Write tool dispatch does NOT match either gate arm's tool
pattern (`^Agent$`, `^Bash$`). VP-107 does NOT verify "rm is never gated" (that claim is de-
sanctioned). T3 is verified by architectural argument (human shell outside dispatcher mediation).
T2 is covered by the TTL invariant test (Decision 2 + Decision 4 TTL-loudness).

**Why formalize this as an invariant?** Decision 1 tightens the gate's crash behavior. A future
refactor that added Edit/Write to a gate arm's tool pattern would silently break T1 recoverability.
The recoverability invariant provides an explicit design constraint: any change to the gate's
`tool` pattern MUST verify that T1 (Edit/Write ungated) remains intact after the change.

**Net safety argument (updated):**

| Scenario | block_if_marker | TTL | Recoverability |
|----------|-----------------|-----|---------------|
| Gate crash, no marker | Allow | N/A | N/A |
| Gate crash, marker + non-expired | **Block** (fail-closed) | Expires ≤24h | T1 (Edit/Write); T2 (TTL); T3 (human rm) |
| Gate crash, marker + TTL expired | Allow | Self-healed | N/A |
| Normal path, no marker | Allow | N/A | N/A |
| Normal path, marker + non-expired | **Block** (plugin exit_code=2) | Expires ≤24h | T1; T2; T3 |
| Normal path, marker + TTL expired | Allow (plugin auto-deletes; `marker.cleared(TTL_EXPIRED)` emitted) | Self-healed | N/A |
| Crash path + agent-tool rm with marker present | **Block** (acceptable; not INV6) | Expires ≤24h | T1; T2; T3 |

The ONLY allow-on-failure cases are crash-with-no-marker and crash-with-expired-marker, which
enforce nothing because there is no valid quarantine signal. Every real (non-expired) quarantine
signal is enforced even through plugin crashes. Three independent recoverability guarantees
(T1/T2/T3) ensure the system is never in an irrecoverable state.

### Decision 4 — Audited Clear Event `marker.cleared` + TTL-Loudness

Marker clearance is a security-relevant access-control state change: it transitions the gate from
BLOCKING to ALLOWING. NIST AU-3/AU-9/AU-10 and PCI-DSS 10.2 require that every such state change
be audit-logged with sufficient detail for incident investigation. The three clear paths
(re-validation, TTL-expiry, operator override) were previously silent — no event record was emitted
when the marker was cleared. This decision adds the `marker.cleared` event to the dispatcher's
domain event set (BC-3.08.001) and requires that TTL auto-delete emit an audited event.

**Event field contract:**

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `type` | literal `"marker.cleared"` | YES | Domain event discriminant |
| `clear_mode` | enum | YES | `REVALIDATED` \| `TTL_EXPIRED` \| `OPERATOR_OVERRIDE` |
| `actor_type` | enum | YES | `validator` \| `deadman` \| `operator` |
| `trace_id` | string | YES | Must match `trace_id` of the originating `plugin.indeterminate` event |
| `plugin_name` | string | YES | From marker TOML `plugin_name` field |
| `artifact_path` | string | YES | From marker TOML `artifact_path` field |
| `reason` | string | Conditional | Mandatory when `clear_mode = "OPERATOR_OVERRIDE"`; `null` or omitted otherwise |
| `timestamp` | ISO-8601 UTC | YES | Time of the clear event (not the original INDETERMINATE event) |

**clear_mode values and emission points (v1.2 — dispatcher-native for all three modes):**

| `clear_mode` | `actor_type` | Trigger | Emission point (v1.2) |
|---|---|---|---|
| `REVALIDATED` | `validator` | Executor observes a PostToolUse PASS on the same plugin+artifact as the marker → `delete_marker_if_pass` removes marker file | Dispatcher-native: `emit_marker_cleared` in `indeterminate_marker.rs`, called from `delete_marker_if_pass`'s PostToolUse callsites in `executor.rs`, immediately after `std::fs::remove_file(marker_path)` succeeds. **UNCHANGED from v1.1 — this was already correctly dispatcher-native.** |
| `TTL_EXPIRED` | `deadman` | Dispatcher-native pre-check (new — see below) finds `expires_at ≤ now` before invoking the Arm 1/Arm 2 WASM gate plugin → auto-deletes marker file | **MOVED (v1.2) to dispatcher-native**: `emit_marker_cleared` in `indeterminate_marker.rs`, called from the new `check_and_clear_expired_marker` pre-check in `executor.rs`'s tier-execution loop, immediately after the native `std::fs::remove_file` succeeds. The WASM plugin's `evaluate_gate` no longer performs TTL date-math, deletion, or emission — see §Decision 4 v1.2 Emission-Point Correction below. |
| `OPERATOR_OVERRIDE` | `operator` | Human operator clears marker via T3 (out-of-band `rm` in operator shell); not mediated by dispatcher | **IMPLEMENTED (v1.2) as dispatcher-native**: retroactive reconciliation via `reconcile_raw_delete` in `indeterminate_marker.rs`, invoked from the same pre-check's marker-absent branch (see below). Previously unimplemented in any form (F-P2-003). |

**RAW_DELETE_DETECTED reconciliation for OPERATOR_OVERRIDE (v1.2 — dispatcher-native):**

The T3 (human out-of-band `rm`) clear path is not mediated by the dispatcher. A real-time
`marker.cleared(OPERATOR_OVERRIDE)` event cannot be emitted at the moment of deletion.
Reconciliation runs DISPATCHER-NATIVE (not in the WASM plugin — see rationale in the v1.2
Emission-Point Correction subsection below): as part of the same native pre-check that runs
before every Arm 1/Arm 2 dispatch, in the branch where the marker is found absent, the dispatcher
performs a bounded, best-effort scan of the current day's FileSink `events-*.jsonl` for an
unmatched `plugin.indeterminate` (fail-closed) event — one for which no corresponding
`marker.cleared` was subsequently written for the same `(plugin_name, artifact_path)`. If found,
`reconcile_raw_delete` calls `emit_marker_cleared` with:

- `reason = "RAW_DELETE_DETECTED: marker absent without prior marker.cleared event; inferred operator out-of-band clear"`
- `timestamp` = current evaluation time (not the deletion time, which is unobservable)
- `trace_id` = trace_id from the unmatched `plugin.indeterminate` event
- `plugin_name` / `artifact_path` = from the same unmatched event

**Best-effort and bounded (production-grade requirement, v1.2):** if the FileSink log is
unavailable or the unmatched record cannot be found, the annotation is omitted — no hard failure.
An unreconciled gap is observable by tooling that monitors the event stream for
`plugin.indeterminate` events without subsequent `marker.cleared`. Because this check runs before
EVERY Arm 1 (`^Agent$`) and Arm 2 (`^Bash$` git commit/push) dispatch in the common
marker-absent case, the scan MUST be bounded to avoid unbounded I/O growth on the hot dispatch
path: implementations MUST cap the scan to the current day's events file only (never scan prior
days) and MUST cap total bytes/records read (e.g., a fixed-size tail read, not a full-file scan).
This is an explicit production-grade constraint, not an optional optimization — an unbounded scan
on every Agent/git-commit dispatch would reintroduce the same class of large-artifact resource
cost this whole feature (S-25.01) exists to eliminate. The exact bound (byte cap or record cap)
and any memoization strategy (e.g., a lightweight on-disk checkpoint to avoid re-scanning already-
reconciled ranges across dispatcher process invocations) are implementation details left to the
story spec / test-writer AC, not fixed by this ADR. This reconciliation step never gates the
dispatch decision (BC-3.08.001 Invariant 3) — it MAY run either synchronously before the Allow
result is returned or as a best-effort step after, at implementer's discretion.

**Emission path:**

`marker.cleared` events are emitted via the same FileSink/InternalLog path as `plugin.indeterminate`
(BC-3.08.001 domain event catalog) — for ALL three clear_modes, via the dispatcher-native
`emit_marker_cleared` function, never via the WASM `emit_event` host ABI (see v1.2 Emission-Point
Correction below for why).

### Decision 4 v1.2 Emission-Point Correction (S-25.01 LOCAL Adversary Pass 2 — F-P2-002 HIGH, F-P2-003 MED)

**The defect:** `crates/factory-dispatcher/src/host/emit_event.rs` enforces `RESERVED_FIELDS`
(`trace_id`, `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`, `ts`,
`ts_epoch`, `schema_version`, `type`) on every event a WASM plugin emits via
`vsdd_hook_sdk::host::emit_event`. The host enrichment path is unconditional:

```
let ctx = caller.data();
let mut ev = InternalEvent::now(&event_type)
    .with_trace_id(&ctx.dispatcher_trace_id)   // ALWAYS the calling plugin's own dispatch trace
    .with_plugin_name(&ctx.plugin_name)         // ALWAYS the calling plugin's own registry name
    ...
for (k, v) in pairs {
    if is_reserved_field(&k) { continue; }      // plugin-supplied trace_id/plugin_name silently dropped
    ev = ev.with_field(&k, Value::String(v));
}
```

This is correct and load-bearing for every OTHER event in the BC-3.08.001 catalog — it prevents a
plugin from spoofing another plugin's identity (BC-3.08.001 Invariant 5). But it makes
`marker.cleared` a structural exception: the wire contract (BC-3.08.001 Event 9) requires
`trace_id` and `plugin_name` to be the MARKER's stored values (linking back to the originating
`plugin.indeterminate` event, itself produced by a DIFFERENT, earlier, already-completed
dispatch) — not the CURRENT gate-plugin's own identity (`validate-unvalidated-mutation-marker` /
`-git`, with the CURRENT dispatch's trace_id). No WASM plugin can ever emit an event carrying a
foreign trace_id/plugin_name through `emit_event`, by design. The v1.0/v1.1 TTL_EXPIRED
implementation in `guard_logic::evaluate_gate` (`crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs`)
attempted a workaround — emitting `marker_trace_id`/`marker_plugin_name` as non-reserved custom
field names, with `reason=""` — but this is NOT the Event 9 wire contract (which requires
`trace_id` and `plugin_name` as top-level fields) and silently breaks the
`plugin.indeterminate → marker.cleared` audit-correlation for the deadman path (F-P2-002 HIGH).
The OPERATOR_OVERRIDE path (F-P2-003 MED) would hit the identical wall if implemented WASM-side —
it was in fact never implemented at all for exactly this reason.

**The working counterexample, and why it works:** REVALIDATED's emission (`emit_marker_cleared`
in `crates/factory-dispatcher/src/executor.rs`, called from `delete_marker_if_pass`) is correct
because it runs entirely dispatcher-side: it constructs an `InternalEvent` directly
(`InternalEvent::now(PLUGIN_MARKER_CLEARED).with_trace_id(&marker_fields.trace_id)...`) and writes
it to `InternalLog` directly — never crossing the WASM `emit_event` ABI boundary, so
RESERVED_FIELDS enrichment never applies. `plugin.indeterminate` (Event 8) is emitted the same
way — dispatcher-native, from `executor.rs`'s outcome-classification logic, per BC-3.08.001 Event
8's own trigger description ("`invoke_plugin` (`executor.rs`) classifies a plugin's outcome as
INDETERMINATE"). Dispatcher-native emission is therefore the ESTABLISHED pattern for any event in
this catalog that must carry provenance from something other than the current WASM invocation's
own identity — TTL_EXPIRED and OPERATOR_OVERRIDE are members of that class; they were simply
misplaced in v1.0/v1.1.

**The fix — decision: full dispatcher-native ownership of TTL detection, deletion, and emission
(not a plugin-to-dispatcher signal-forwarding protocol).** Two designs were evaluated:

- **(A) — SELECTED: Move TTL detection + auto-delete + emission entirely into a new
  dispatcher-native pre-check.** A new function `check_and_clear_expired_marker(factory_root,
  now) -> io::Result<Option<MarkerFields>>` in `indeterminate_marker.rs` performs: read the marker
  via `read_all_marker_fields`; if present and `expires_at ≤ now`, delete it (idempotent, swallow
  `NotFound`) and return `Some(fields)`. The dispatcher's tier-execution loop in `executor.rs`
  calls this function for every registry entry with `on_error == OnError::BlockIfMarker`
  (currently both `validate-unvalidated-mutation-marker` arms) BEFORE invoking the plugin, on the
  normal (non-crash) path — every dispatch, not just on crash. If it returns `Some(fields)`, the
  dispatcher immediately calls `emit_marker_cleared(..., "TTL_EXPIRED", "deadman", None)`. Because
  the marker is guaranteed absent-or-non-expired by the time the WASM plugin's `evaluate_gate`
  subsequently runs, `evaluate_gate` is SIMPLIFIED to a pure presence check with NO `expires_at`
  parsing, NO deletion, and NO emission logic at all — the entire TTL branch (and its
  non-compliant `marker_trace_id`/`marker_plugin_name`/`reason=""` workaround) is removed from the
  WASM plugin. OPERATOR_OVERRIDE reconciliation is added to the SAME native pre-check's
  marker-absent branch (see RAW_DELETE_DETECTED subsection above).
- **(B) — REJECTED: Plugin performs the TTL delete as today, but signals back to the dispatcher
  (via a reserved-but-distinguishable field, or a new host function) so the dispatcher can emit a
  correctly-attributed event on the plugin's behalf.** Rejected because: (i) it requires inventing
  new ABI surface (a signal-forwarding protocol) purely to work around a wall that dispatcher-side
  ownership avoids entirely; (ii) it creates a TOCTOU-shaped trust problem — the dispatcher would
  have to believe the plugin's self-report of "I deleted marker X as TTL_EXPIRED" rather than
  observing the deletion directly; (iii) it does not simplify anything — the plugin still needs
  full `expires_at` date-math and delete logic, so WASM fuel consumption and attack surface are
  unchanged, whereas (A) removes that logic entirely; (iv) it is inconsistent with the established
  dispatcher-native pattern for Events 8/9-REVALIDATED, introducing an unnecessary second pattern
  for the same conceptual operation (marker mutation + its audit event).

**Rationale for (A) over (B) — production-grade default, not the cheap path:** (A) is a larger
diff (it touches both `executor.rs`/`indeterminate_marker.rs` and the WASM plugin's
`evaluate_gate`) than a minimal (B)-style patch that only touches the emission call. It is
selected anyway because it is the ARCHITECTURALLY CORRECT fix: it eliminates the ABI-wall class of
bug rather than routing around it, it aligns TTL_EXPIRED and OPERATOR_OVERRIDE with the same
dispatcher-native pattern already governing REVALIDATED and `plugin.indeterminate`, and it reduces
rather than increases total system complexity (WASM plugin shrinks; no new ABI surface is added).

**Consequence for Decision 2 ("Who checks `expires_at`"):** superseded by this v1.2 amendment —
see the updated Decision 2 subsection above. Consequence for Decision 1 (`block_if_marker`
crash-path check): UNCHANGED — `block_if_marker_check` retains its own independent TTL-awareness
for the crash-path case (does not auto-delete, does not emit; see Decision 1 and BC-1.18.003
EC-014/VP-108 PC4), which remains a distinct code path from the new normal-path native pre-check.

**Event enumeration — nine-event dispatcher domain model:**

ADR-048 §Decision 4 adds `marker.cleared` as the ninth domain event. Prior count (ADR-039 /
BC-3.08.001): eight events. Downstream sweep REQUIRED (PO + test-writer + arch docs):

| # | Event type | Source |
|---|-----------|--------|
| 1 | `plugin.completed` | ADR-039 |
| 2 | `plugin.abandoned` | ADR-039 |
| 3 | `plugin.timeout` | ADR-039 |
| 4 | `plugin.log` | ADR-039 |
| 5 | `plugin.crashed` | ADR-039 |
| 6 | `plugin.indeterminate` | ADR-047 |
| 7 | `plugin.fuel_exhausted` | ADR-039 |
| 8 | `plugin.epoch_timeout` | ADR-039 |
| 9 | `marker.cleared` | **ADR-048 §D4** |

Every reference to "seven dispatcher events" or "eight dispatcher events" in BC-3.08.001, story
tests, and architecture docs MUST be updated to "nine dispatcher events." Specific downstream flags
(see Consequences).

**Proportionality rationale:**

The cooperating-agent threat model (VSDD baseline) does not include adversarial agents forging or
deleting audit records. The primary threat is accidental silent state change. A durable
append-only FileSink event is proportionate:

- **Signed digests:** rejected — requires key management infrastructure; disproportionate for a
  single-operator factory under cooperating-agent threat model.
- **Dual-control:** rejected — requires a second human principal; factory operates under single-
  operator model during a VSDD run.

GitHub branch-protection and the `factory-artifacts` append-only worktree provide tamper-evidence
at the VCS layer, which is sufficient for this threat model.

**TTL-loudness:**

Decision 2 (TTL deadman) specified that when `evaluate_gate` finds `expires_at ≤ now`, it treats
the marker as absent and auto-deletes. Prior to ADR-048 v1.1, this auto-delete was SILENT.
TTL-loudness requires that `marker.cleared(TTL_EXPIRED)` be emitted EVERY time the auto-delete
fires.

The clear-and-advance-on-expiry SEMANTICS are UNCHANGED: a TTL-expired marker is treated as absent
and the dispatch proceeds — no re-validation probe is triggered. The only change is the addition
of the audited `marker.cleared(TTL_EXPIRED)` emission. Silent TTL clear was acceptable only
because the marker is defense-in-depth, not the sole authority; the v1.1 posture is "defense-in-
depth allows AUDITED TTL clear," which is strictly stronger.

Note: the dispatcher's NATIVE crash-path TTL check (Decision 1 — crash + marker + expired → Allow)
does NOT auto-delete the marker (crash handler kept simple). The TTL-loudness requirement applies
only to the gate PLUGIN's normal-path TTL auto-delete. A crash-path TTL allow does NOT emit
`marker.cleared(TTL_EXPIRED)` — the marker remains for the next normal plugin execution to clear.

---

## Rationale

### Why block_if_marker rather than re-executing the plugin on crash?

An alternative was: on gate crash, re-attempt a fresh plugin execution. Rejected because: (a) a
re-attempted execution may re-crash (same fuel-exhaustion root cause); (b) the native filesystem
check is the correct architectural choice — the marker is a dispatcher artifact, so the dispatcher
reading it natively without WASM intermediation is the most reliable mechanism; (c) WASM
re-execution complexity is not warranted when a native file-existence check solves the problem.

### Why 86400 seconds rather than reusing factory_lock's 2700 seconds?

2700 seconds (45 minutes) is appropriate as a session keep-alive TTL (renewed on each state write
via `renew_lock_with_now` in `crates/factory-lock/src/lib.rs`). The mutation marker is NOT a
keep-alive: it persists after session restart (intentionally, per ADR-047 §Rationale). A 45-minute
TTL would cause markers to expire mid-session if the session is paused for an hour, creating
spurious auto-clears while the operator is actively investigating. 24 hours bounds the worst-case
stuck state to a session day: the operator can investigate and resolve within a working day;
markers forgotten overnight auto-clear before the next workday.

### Why formalize escape paths as an invariant rather than an informal note?

ADR-047 mentioned the escape hatch informally. ADR-048's tightened crash behavior makes
recoverability more critical: if the gate now blocks on crash+marker, any accidental gating of
the escape operations would create an irrecoverable state. Formalizing as Decision 3 with VP-107
verification ensures future refactors cannot accidentally close the escape paths.

### ADR-047 §Decision 4 Was Correct but Incomplete

D-1135 correctly avoided unconditional self-lock. It erred in treating both crash sub-cases
(crash+no-marker and crash+marker-exists) identically under a single `on_error = "continue"`.
ADR-048 splits the policy: `"block_if_marker"` preserves the no-marker allow while adding the
marker-conditional block. It is a targeted correction, not a rejection of D-1135's core insight.

### Why re-validation is the primary sanctioned agent recovery (v1.1)

Prior-art patterns (circuit-breaker reset, EDR Restore, Google break-glass, NIST AU/AC, LOTO/
e-stop) share a common principle: a controlled agent must not hold an unmediated, unaudited path
to erase its own constraint. Recovery must come from independent RE-VALIDATION (prove the
constraint is satisfied), an authenticated HUMAN override, or a TIMEOUT:

- **Re-validation (T1):** recovery by demonstration. The agent proves the unsafe condition no
  longer holds by re-validating the artifact. Inherently self-auditing — the same gate plugin
  that marked the artifact INDETERMINATE confirms it PASSES, providing end-to-end audit closure.
- **Human override (T3):** recovery by authorization. A principal with higher authority than the
  agent explicitly clears the constraint out-of-band. Break-glass / out-of-band pattern (Google
  SRE, GitLab dequarantine) — always available but requires human intent.
- **TTL (T2):** recovery by timeout. The constraint self-releases after a bounded interval,
  preventing permanent lockout. Bounded deadman design (LOTO e-stop, IEC 60204-1).

Agent-tool `rm` fits none of these patterns: it is neither a validation, nor an authenticated
human override, nor a timeout. The agent asserting "I should clear my own quarantine" via `rm`
is semantically equivalent to the agent modifying its own gate logic — the exact class of action
the quarantine-forward design protects against.

### Why marker.cleared and why no signed digests (v1.1)

The `marker.cleared` event design is proportionate to the cooperating-agent threat model. The
primary audit failure mode is not adversarial log forgery but silent state change: the gate
disengages with no trace. A durable append-only FileSink event addresses this directly. Signed
digests (requiring key management) and dual-control (requiring multi-principal coordination) are
appropriate for adversarial environments (PCI-DSS §10.5 financial audit trails) but
disproportionate here. GitHub branch-protection and the `factory-artifacts` worktree already
provide tamper-evidence at the VCS layer.

---

## Consequences

### Positive

- Gate enforces its quarantine even through plugin crashes, eliminating the CWE-636 gap where
  crash+marker-exists → silent allow.
- TTL deadman bounds worst-case stuck state to 24 hours with no operator intervention required.
- Recoverability invariant (T1/T2/T3), verified by VP-107 (T1 scope: Edit/Write ungated), ensures
  the system is never in an irrecoverable state. Agent-tool `rm` de-sanctioned from the invariant
  simplifies the proof obligation and eliminates the shared-crate fix requirement.
- `"block_if_marker"` generalizes to any future marker-based quarantine gate.
- ADR-039 two-axis model and orthogonality invariant are fully preserved.
- `marker.cleared` audited events provide forensic trail for every marker clearance path,
  satisfying NIST AU-3/AU-9/AU-10 proportionately (append-only FileSink; no signed digests).
- TTL expiry is now LOUD/audited: `marker.cleared(TTL_EXPIRED)` emitted on every auto-delete,
  eliminating the silent-clear gap.

### Negative / Trade-offs

- Dispatcher crash-handler gains complexity: `on_error = "block_if_marker"` requires a native
  file read and TOML parse in the crash-handling codepath. I/O failures (ENOENT, EACCES) are
  treated as "absent" (allow), not hard errors — keeping the crash-handler fail-open on its own
  I/O failures.
- Old markers lacking `expires_at` (written before ADR-048 implementation) are treated as
  non-expired (conservative). They remain in effect until explicitly cleared via `rm`. Once
  ADR-048 is implemented, new markers always carry `expires_at`.
- `UNVALIDATED_MUTATION_MARKER_TTL_SECONDS` is a new constant that must be kept consistent with
  documentation and any external tooling that reasons about marker lifetimes.
- (v1.2 — supersedes the v1.1 bullet below) All three `marker.cleared` emissions — REVALIDATED,
  TTL_EXPIRED, and OPERATOR_OVERRIDE — are dispatcher-native (`indeterminate_marker.rs` /
  `executor.rs`), never WASM-plugin-side, per the Decision 4 v1.2 Emission-Point Correction. The
  dispatcher gains a new pre-invocation native check (`check_and_clear_expired_marker`) that runs
  before every Arm 1/Arm 2 dispatch on the normal path, plus the RAW_DELETE_DETECTED bounded
  FileSink-scan reconciliation (`reconcile_raw_delete`). ~~Gate plugin gains `emit_marker_cleared`
  call responsibilities for REVALIDATED and TTL_EXPIRED clear modes~~ (v1.1 framing — INCORRECT;
  the gate plugin never gains this responsibility because the WASM ABI cannot honor the Event 9
  wire contract; see v1.2 amendment). RAW_DELETE_DETECTED reconciliation remains a best-effort
  FileSink read dependency (now bounded per-day and per-scan, dispatcher-native); no hard failure
  if FileSink is unavailable — annotation omitted.
- BC-3.08.001 event enumeration must be updated from eight to nine events (PO: BC amendment +
  new Event 9 wire format; test-writer: AC coverage for all three `clear_mode` values +
  RAW_DELETE_DETECTED form). This is a REQUIRED downstream flag, not optional.

### Status as of 2026-08-31 (v1.0 / v1.1 / v1.2)

Proposed. Implementation scoped to S-25.01. The `on_error = "block_if_marker"` value requires:
(a) a new Rust enum variant `OnError::BlockIfMarker` in the registry schema (S-21.10 pattern);
(b) `block_if_marker_check` crash-handler logic in `executor.rs`; (c) `expires_at` field in
`write_indeterminate_marker` + TTL check + auto-delete in the gate plugin. The BC amendments
(BC-1.18.001/002/003) and new VP-107 (ungated-escape verification) are downstream deliverables
for PO and test-writer after human ratification.

v1.1 adds: (d) `emit_marker_cleared` function in `crates/factory-dispatcher/src/indeterminate_marker.rs`
(or gate plugin — implementer's choice based on which component owns the clear operation); wired
from `delete_marker_if_pass` (REVALIDATED), TTL auto-delete branch (TTL_EXPIRED), and
RAW_DELETE_DETECTED detection path; (e) BC-3.08.001 Event 9 `marker.cleared` wire format — PO
BC amendment required; (f) VP-107 scope amendment: verify T1 (Edit/Write tool dispatch not matched
by either gate arm `^Agent$` / `^Bash$`) — not "rm is never gated"; (g) BC-1.18.002 INV6/AC-020
block-message reframe: crash-path block on agent-tool rm is ACCEPTABLE — update block message
and INV6 text accordingly; (h) BC-1.18.003 clear-model amendment: audited clears (REVALIDATED,
TTL_EXPIRED, OPERATOR_OVERRIDE); (i) BC-3.08.001 event enumeration seven→eight→nine sweep.

**v1.1's item (d) "or gate plugin — implementer's choice" is RETRACTED by v1.2 — it was the seed
of F-P2-002.** v1.2 makes the choice explicit and non-optional: dispatcher-native ONLY, for all
three clear_modes. v1.2 adds/corrects: (j) `check_and_clear_expired_marker` function in
`indeterminate_marker.rs` (new — native TTL detect+delete, called from `executor.rs` before every
Arm 1/Arm 2 invocation on the normal path); (k) `reconcile_raw_delete` function in
`indeterminate_marker.rs` (new — dispatcher-native, bounded FileSink scan, implements
OPERATOR_OVERRIDE which was previously entirely unimplemented per F-P2-003); (l)
`guard_logic::evaluate_gate` in `crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs`
SIMPLIFIED — removes `expires_at` parsing, TTL auto-delete, and the `marker_trace_id`/
`marker_plugin_name`/`reason=""` `emit_event` workaround entirely; becomes a pure
presence-check (absent → Allow; present → Block with fields read from the marker); (m) BC-1.18.003
PC4 + Architecture Anchors + BC-3.08.001 Event 9 "Emission point" table + VP-108 (all currently
attribute TTL_EXPIRED emission to the gate plugin's "TTL-check branch") REQUIRE product-owner
amendment to attribute it to the dispatcher-native pre-check instead — see the enumerated PO/
story-writer change list this ADR amendment hands off (recorded in the architect's S-25.01 pass-2
adjudication response, not duplicated verbatim here to avoid drift between this ADR and the BC/VP
files themselves).

---

## Alternatives Considered

- **Keep fail-open (D-1135 as-is):** Preserves D-1135 posture. Rejected: known CWE-636 defect in
  the crash+marker-exists sub-case. Human has explicitly directed the redesign.

- **Use `on_error = "block"` (unconditional crash-block):** Blocks on every gate crash regardless
  of marker state. Rejected: creates the unconditional self-lock D-1135 was designed to prevent
  (crash with no marker → still block → irrecoverable until external process intervention).

- **Dispatcher special-case keyed to gate plugin name:** Code-path: `if plugin_name ==
  "validate-unvalidated-mutation-marker" { check_marker() }`. Rejected in favor of the new
  `on_error` value: the value generalizes to future gates; the special-case is fragile on rename
  and not discoverable via registry inspection.

- **Per-session in-memory flag (cache marker-read result in session state):** Cache whether the
  marker was seen on the last successful plugin run; use cache on crash. Rejected: in-memory state
  is lost on session restart; defeats the cross-session durability guarantee (ADR-047 §Rationale).

---

## Source / Origin

<!-- BROWNFIELD: You MUST cite implementation evidence (file:line from crates/ or
     legacy-design-docs/) before this ADR can be accepted. Omitting evidence is a
     template-compliance failure. -->

- **Human direction (v1.0):** Human-directed gate redesign 2026-08-31. Specified: `block_if_marker`
  new `on_error` value + ungated-escape invariant + TTL deadman. D-1135 fail-open-on-crash
  ratification explicitly reversed.
- **Human direction (v1.1 — HIGH-1 resolution):** Human-directed recovery model reframe 2026-08-31.
  Specified: re-validation as primary agent recovery; human out-of-band rm as break-glass; agent-
  tool rm de-sanctioned; shared-crate fix rejected; `marker.cleared` audited event with
  REVALIDATED/TTL_EXPIRED/OPERATOR_OVERRIDE modes; RAW_DELETE_DETECTED reconciliation; TTL-
  loudness; no signed digests / dual-control (cooperating-agent threat model).
- **Prior art (v1.1 — recovery model):** Circuit-breaker reset (IEEE); EDR Restore patterns
  (endpoint detection and response quarantine/release flows); Google break-glass (SRE out-of-band
  override pattern); GitLab dequarantine; NIST SP 800-53 AU-3 (event content), AU-9 (audit
  information protection), AU-10 (non-repudiation), AC-6 (least privilege); PCI-DSS Req 10.2
  (audit event types); LOTO/e-stop (IEC 60204-1 §10.7 — bounded-deadman design). Common
  principle: the controlled agent must not hold an unmediated, unaudited path to erase its own
  constraint; recovery = independent re-validation | authenticated human override | timeout.
- **S-25.01 LOCAL adversary pass 2 (v1.2 origin):** F-P2-002 (HIGH) — TTL_EXPIRED `marker.cleared`
  emitted from inside the WASM gate plugin cannot honor the Event 9 wire contract's `trace_id`/
  `plugin_name` fields because `emit_event`'s RESERVED_FIELDS enrichment unconditionally
  overwrites plugin-supplied values with the current gate-plugin's own dispatch identity,
  silently breaking `plugin.indeterminate → marker.cleared` audit correlation for the deadman
  path. F-P2-003 (MED) — OPERATOR_OVERRIDE/RAW_DELETE_DETECTED reconciliation (AC-023,
  BC-1.18.003 PC3, BC-3.08.001 Event 9 EC-013) is entirely unimplemented, and would hit the
  identical wall if implemented WASM-side. Architect adjudication (2026-08-31, architect agent,
  dispatched per CLAUDE.md Agent Routing Table): both findings resolved by moving TTL_EXPIRED and
  OPERATOR_OVERRIDE emission to dispatcher-native code, mirroring the already-correct REVALIDATED
  architecture. PROPOSED pending human ratification per POLICY 22 — this is a further revision to
  a not-yet-ratified ADR (ADR-048 v1.0/v1.1 were themselves never ratified), not a reopening of an
  accepted decision.
- **Rice's theorem (v1.1 — shared-crate rejection):** Rice, H.G. (1953). "Classes of recursively
  enumerable sets and their decision problems." Trans. AMS 89(1):25–59. Applied: any command-
  filter classifying "this Bash dispatch deletes the marker file" is undecidable in the general
  case; partial filters (`rm`-specific allowlist) are evaded by equivalent operations
  (`mv`/`truncate`/`python os.remove`). Same undecidable class BC-1.18.002 §out-of-scope accepts.
- **ADR-047:** `decisions/ADR-047-indeterminate-outcome-model-durable-mutation-marker-next-advance-gate.md`
  v1.3 — §Decision 4 superseded by ADR-048 §Decision 1; §Decisions 1, 2, 3, 5–9 unchanged.
- **ADR-039:** `decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md`
  — two-axis model (§Decision 1); axes-independence invariant; `failure_policy` field schema.
  ADR-048 extends `on_error` axis with `"block_if_marker"` value; `failure_policy` axis unchanged.
- **factory_lock TTL (2700s):** `crates/factory-lock/src/lib.rs` function `renew_lock_with_now` —
  `Duration::seconds(2700)` is the hardcoded keep-alive renewal interval; no public constant;
  confirmed distinct semantics from ADR-048's 86400s deadman constant.
- **BC-1.18.001 v1.0:** `.factory/specs/behavioral-contracts/ss-01/BC-1.18.001.md` —
  marker write specification; PC4 TOML fields to be extended with `expires_at` (PO amendment).
- **BC-1.18.002 v1.4:** `.factory/specs/behavioral-contracts/ss-01/BC-1.18.002.md` —
  gate behavior; PC1 `on_error = "continue"` superseded to `on_error = "block_if_marker"`;
  INV6/AC-020 block-message reframe: crash-path block on agent-tool rm is ACCEPTABLE (v1.1 PO).
- **BC-1.18.003 v1.3 (v1.2 — supersession note):** `.factory/specs/behavioral-contracts/ss-01/BC-1.18.003.md` —
  marker-clear protocol; TTL expiry is PC4; audited-clears (REVALIDATED/TTL_EXPIRED/
  OPERATOR_OVERRIDE) already present as of v1.3, but PC4's "Emission point" and the Architecture
  Anchors section still attribute TTL_EXPIRED emission to "the gate plugin's TTL-check branch" —
  this is SUPERSEDED by the v1.2 Decision 4 Emission-Point Correction above and REQUIRES a PO
  amendment (v1.4) to attribute it to the dispatcher-native pre-check instead; OPERATOR_OVERRIDE
  language ("the plugin emits a retroactive marker.cleared") requires the same correction.
- **BC-3.08.001 v1.30 (v1.2 — supersession note):** dispatcher domain event catalog, nine events
  including `marker.cleared` (Event 9). Event 9's "clear_mode / actor_type correspondence" table
  "Emission point" column for TTL_EXPIRED ("Plugin TTL-check branch after auto-delete") and
  OPERATOR_OVERRIDE ("RAW_DELETE_DETECTED reconciliation path in gate plugin") is SUPERSEDED by
  the v1.2 Decision 4 Emission-Point Correction above and REQUIRES a PO amendment (v1.31) to
  attribute both to dispatcher-native code.
- **VP-108 (v1.2 — supersession note):** `.factory/specs/verification-properties/VP-108.md` —
  `marker.cleared` clear-path emission-correctness VP. Its `module` frontmatter field already
  correctly names `crates/factory-dispatcher/src/indeterminate_marker.rs` (dispatcher-native), but
  its Property Statement/Proof Harness (Postcondition 2, `test_ttl_expired_clear_emits_marker_cleared`)
  still narrates "the gate plugin auto-deletes... and emits" and calls a non-existent
  `evaluate_gate_with_sink` function that blends WASM-plugin logic with a dispatcher-side
  `CapturingSink` — impossible across the WASM/host boundary. REQUIRES architect amendment
  (v1.1) to correct the PC2/PC3 narration and harness to call the new
  `check_and_clear_expired_marker`/`reconcile_raw_delete` dispatcher-native functions directly,
  with no WASM invocation involved.
- **Architecture as-built (crates — PLANNED for S-25.01, v1.2 emission-point-corrected):**
  `crates/factory-dispatcher/src/indeterminate_marker.rs` (add `expires_at` to
  `write_indeterminate_marker`; add `UNVALIDATED_MUTATION_MARKER_TTL_SECONDS = 86_400`; add
  `emit_marker_cleared` — ALL THREE clear modes REVALIDATED/TTL_EXPIRED/OPERATOR_OVERRIDE are
  emitted from this single dispatcher-native function; add `check_and_clear_expired_marker(
  factory_root, now) -> io::Result<Option<MarkerFields>>` [NEW, v1.2] — native TTL detect+delete,
  called before every Arm 1/Arm 2 invocation on the normal path; add `reconcile_raw_delete(
  factory_root, log, session_id) -> io::Result<()>` [NEW, v1.2] — dispatcher-native bounded
  FileSink scan + retroactive OPERATOR_OVERRIDE emission, implementing what was previously
  entirely unimplemented per F-P2-003);
  `crates/factory-dispatcher/src/executor.rs` (add `block_if_marker_check` call in crash-handler,
  new `OnError::BlockIfMarker` match arm; [NEW, v1.2] call `check_and_clear_expired_marker` +
  `reconcile_raw_delete` from the tier-execution loop before invoking any `on_error =
  "block_if_marker"` plugin on the normal path);
  `crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs` (`guard_logic::evaluate_gate`
  SIMPLIFIED [v1.2] — REMOVE `expires_at` parsing, TTL auto-delete, and `marker.cleared(TTL_EXPIRED)`
  emission via `emit_event` (the v1.0/v1.1 `marker_trace_id`/`marker_plugin_name`/`reason=""`
  workaround is deleted, not fixed-in-place); the plugin becomes a pure marker-presence check:
  absent → Allow; present → Block with fields read from the marker. `delete_marker_if_pass` and
  `marker.cleared(REVALIDATED)` emission were NEVER in this plugin — they remain, unchanged, in
  `executor.rs`/`indeterminate_marker.rs` dispatcher-side, as v1.1's own architecture already
  correctly had them; the v1.1 ADR text's item (d) "or gate plugin" framing was the drafting error
  that seeded F-P2-002, not an actual code placement that needs undoing);
  `plugins/vsdd-factory/hooks-registry.toml` (set `on_error = "block_if_marker"` for both arms;
  unchanged by v1.2).
