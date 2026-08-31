---
document_type: adr
adr_id: ADR-048
version: "1.0"
title: "ADR-048: Fail-Closed-But-Recoverable Gate — block_if_marker Crash Policy, Marker TTL Deadman, and Ungated-Escape Invariant"
status: proposed
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
last_amended: "2026-08-31 (v1.0) — Initial authoring. Human-directed gate redesign reversing D-1135 fail-open-on-crash ratification."
modified:
  - "2026-08-31 (v1.0) — Initial authoring"
---

<!-- BROWNFIELD: You MUST cite implementation evidence (file:line from crates/ or
     legacy-design-docs/) before this ADR can be accepted. Omitting evidence is a
     template-compliance failure. -->

# ADR-048: Fail-Closed-But-Recoverable Gate — block_if_marker Crash Policy, Marker TTL Deadman, and Ungated-Escape Invariant

## Status

**PROPOSED — Awaiting human ratification.**

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

**Who checks `expires_at`:**

1. **Gate plugin (normal non-crash path):** After reading the marker TOML, parses `expires_at`.
   If `expires_at <= now (UTC)`: treat as absent → return exit_code=0 (allow); auto-delete the
   marker file (idempotent; swallow `NotFound`). The auto-delete prevents the marker from
   accumulating as a dead artifact.

2. **Dispatcher native `block_if_marker` check (crash path, Decision 1):** After detecting plugin
   crash and finding the marker file exists, parses `expires_at`. If expired: allow (treat as
   absent). If `expires_at` field absent (backward-compat: old marker written before ADR-048
   implementation): treat as non-expired (conservative — old markers are not silently cleared).
   Does NOT auto-delete (keep crash handler simple; auto-delete happens on next normal-path
   plugin execution).

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

### Decision 3 — Ungated-Escape Invariant

**The three recovery operations MUST NEVER be gated by the `validate-unvalidated-mutation-marker`
PreToolUse gate plugin (either arm):**

**(i) `rm .factory/unvalidated-mutation.marker` is ungated.**

This is a Bash dispatch. The Arm 2 filter (`is_git_commit_or_push`, BC-1.18.002 PC2 v1.3
algorithm) evaluates the command:
- Phase 1: no shell operators; single segment.
- Phase 1b: tokenize → `["rm", ".factory/unvalidated-mutation.marker"]`.
- Phase 2: executable = `rm`; `basename("rm") = "rm" != "git"` → return `false`.

`is_git_commit_or_push` returns `false` → Arm 2 does NOT gate this dispatch.
Arm 1 only fires on `^Agent$` dispatches; a Bash `rm` dispatch is not `^Agent$` → Arm 1 does NOT
gate this dispatch. Confirmed ungated by construction.

**(ii) Re-validation (Edit or Write to the artifact that triggered INDETERMINATE) is ungated.**

Edit and Write tool dispatches are NOT gated per BC-1.18.002 PC3 ("Non-advancing dispatches are
NOT gated") and ADR-047 §Decision 4 ("Read, Edit, Write, MultiEdit, and non-git-commit/push Bash
dispatches are NOT gated"). The gate is PreToolUse on `^Agent$` and `^Bash$` tools only; Edit
and Write tools do not match either arm's `tool` pattern. Confirmed ungated by construction.

**(iii) TTL auto-expiry requires no dispatch at all.**

After `UNVALIDATED_MUTATION_MARKER_TTL_SECONDS` (86400) seconds have elapsed since the
INDETERMINATE event, any subsequent gate evaluation (normal path or native crash-path check) will
treat the marker as absent and allow the dispatch. The operator need not take any action; the
system self-heals. Confirmed ungated — no dispatch is required.

**Why formalize this as an invariant?** Decision 1 tightens the gate's crash behavior. A future
refactor that (a) moved Edit/Write dispatches into the gate's tool pattern, or (b) expanded Arm 2's
command filter to match `rm`, would silently break recoverability. The ungated-escape invariant
provides an explicit design constraint: any change to the gate's `tool` pattern or command filter
MUST verify that all three escape paths remain ungated after the change. This must be verified by
VP-107 (new verification property to be authored by PO and test-writer; see Consequences).

**Net safety argument:**

The fail-closed-but-recoverable design achieves the following safety profile:

| Scenario | block_if_marker | TTL | Recoverability |
|----------|-----------------|-----|---------------|
| Gate crash, no marker | Allow | N/A | N/A (no quarantine signal) |
| Gate crash, marker + non-expired | **Block** (fail-closed) | Expires ≤24h | (a) rm; (b) Edit/Write to re-validate; (c) TTL |
| Gate crash, marker + TTL expired | Allow | Self-healed | N/A |
| Normal path, no marker | Allow | N/A | N/A |
| Normal path, marker + non-expired | **Block** (plugin exit_code=2) | Expires ≤24h | (a) rm; (b) re-validate; (c) wait |
| Normal path, marker + TTL expired | Allow (plugin auto-deletes) | Self-healed | N/A |

The ONLY allow-on-failure case is crash-with-no-marker (or crash with expired marker), which
enforces nothing because there is no quarantine signal. Every real (non-expired) quarantine signal
is enforced even through plugin crashes. Three independent recoverability guarantees ensure the
system is never in an irrecoverable state.

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

---

## Consequences

### Positive

- Gate enforces its quarantine even through plugin crashes, eliminating the CWE-636 gap where
  crash+marker-exists → silent allow.
- TTL deadman bounds worst-case stuck state to 24 hours with no operator intervention required.
- Ungated-escape invariant, verified by VP-107, prevents future regressions in recoverability.
- `"block_if_marker"` generalizes to any future marker-based quarantine gate.
- ADR-039 two-axis model and orthogonality invariant are fully preserved.

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

### Status as of 2026-08-31 (v1.0)

Proposed. Implementation scoped to S-25.01. The `on_error = "block_if_marker"` value requires:
(a) a new Rust enum variant `OnError::BlockIfMarker` in the registry schema (S-21.10 pattern);
(b) `block_if_marker_check` crash-handler logic in `executor.rs`; (c) `expires_at` field in
`write_indeterminate_marker` + TTL check + auto-delete in the gate plugin. The BC amendments
(BC-1.18.001/002/003) and new VP-107 (ungated-escape verification) are downstream deliverables
for PO and test-writer after human ratification.

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

- **Human direction:** Human-directed gate redesign 2026-08-31. Specified: `block_if_marker` new
  `on_error` value + ungated-escape invariant + TTL deadman (block_if_marker + ungated-escape +
  TTL option selected). D-1135 fail-open-on-crash ratification explicitly reversed by this decision.
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
  PC3 ungated-escape reaffirmed as formal invariant by §Decision 3.
- **BC-1.18.003 v1.1:** `.factory/specs/behavioral-contracts/ss-01/BC-1.18.003.md` —
  marker-clear protocol; TTL expiry becomes PC4 (new clear path, to be authored by PO).
- **Architecture as-built (crates — PLANNED for S-25.01):**
  `crates/factory-dispatcher/src/indeterminate_marker.rs` (add `expires_at` to
  `write_indeterminate_marker`; add `UNVALIDATED_MUTATION_MARKER_TTL_SECONDS = 86_400`);
  `crates/factory-dispatcher/src/executor.rs` (add `block_if_marker_check` call in crash-handler,
  new `OnError::BlockIfMarker` match arm);
  `crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs` (add `expires_at` TTL
  check + auto-delete on expiry in `guard_logic::evaluate_gate`);
  `plugins/vsdd-factory/hooks-registry.toml` (set `on_error = "block_if_marker"` for both arms).
