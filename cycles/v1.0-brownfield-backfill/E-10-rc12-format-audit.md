---
audit: E-10 spec ↔ rc.12 canonical hook block-message format alignment
date: 2026-05-06
producer: architect
engine_baseline: v1.0.0-rc.12 @ 4cf59bc
spec_baseline: factory-artifacts @ 9175b7e
verdict: DRIFT_MINOR
---

# E-10 ↔ rc.12 Format Audit

## Verdict

DRIFT_MINOR — two spec artifacts carry stale "first stderr line" semantics that were
superseded by the rc.12 full-stderr (4 KiB cap) change; the `vsdd.block.plugin_blocked.v1`
audit-event payload in BC-1.12.006 is missing the `reason` field that the rc.12 telemetry
contract (`emit-event type=hook.block … reason=<code>`) delivers; and BC-2.06.001 does not
document `HookResult::block_with_fix` as an additive ABI addition distinct from the MAJOR
SDK semver signal it already documents. No structural gaps in the core E-10 architecture
(ADR-015, single-stream, FileSink wiring, Resource attributes, per-event stamping) — those
are ALIGNED. Two brownfield BCs (BC-4.02.002 and BC-4.01.003) are the primary stale
artifacts; one E-10 BC (BC-1.12.006) has a field-completeness gap; one SDK BC (BC-2.06.001)
has a minor documentation gap.


## ADR-015 alignment

- **D-15.3 block-path audit trail:** ALIGNED — ADR-015 D-15.3 specifies
  `vsdd.block.plugin_blocked.v1` with `outcome=blocked`, `plugin.name`, `hook.tool_name`.
  rc.12 adds a `reason` field carrying the `Code:` value from the canonical block message
  on the `hook.block` telemetry event, not on the `vsdd.block.plugin_blocked.v1` audit
  event. These are distinct: `hook.block` is the bash/WASM telemetry event emitted by
  `block_pre`/`block_pre_json` helpers; `vsdd.block.plugin_blocked.v1` is the dispatcher
  audit event emitted when `HookResult::Block` is returned. The ADR's D-15.3 block-path
  audit trail clause does not pin the `permissionDecisionReason` string shape, which is
  correct — that shape is a Claude Code ABI concern, not an OTel schema concern. No ADR
  amendment needed.

- **D-15.2 Resource attributes (15-field block):** ALIGNED — no block-path Resource
  fields changed in rc.12. The canonical block message format is a `permissionDecisionReason`
  / stderr concern, not a Resource field concern.

- **D-15.4 exec_subprocess trace propagation:** ALIGNED — unaffected by rc.12.

- **D-15.1 single-stream / FileSink:** ALIGNED — unaffected by rc.12.

- **Legacy-bash-adapter stderr capture (D-15.3 consequential):** DRIFT — ADR-015 D-15.3
  and the block-path audit narrative did not specify a stderr-capture limit for bash hooks.
  rc.12 codifies 4 KiB full-stderr (not first-line) as the capture policy for the
  `legacy-bash-adapter`'s exit-2 path. This is a behavioral change downstream of D-15.3
  that no E-10 spec explicitly governs — it falls into the gap between E-10 BCs (which cover
  the dispatcher audit event) and the brownfield BCs BC-4.02.002 / BC-4.01.003 (which
  describe the adapter's behavior). See D-1 and D-2.


## BC-by-BC assessment

### BC-1.11.001 (exec_subprocess VSDD_TRACE_ID injection)

**Status:** ALIGNED

No hook-block reference. Governs trace-context propagation into subprocesses. Unaffected
by the canonical block-message format change.

---

### BC-1.11.002 (FileSink partial-write recovery)

**Status:** ALIGNED

No hook-block reference. Governs FileSink write semantics. Unaffected.

---

### BC-1.11.003 (emit_pair atomic dual-emit)

**Status:** ALIGNED

No hook-block reference. Governs Wave 2 dual-emit helper. Unaffected.

---

### BC-1.12.001 (FileSink single-stream routing)

**Status:** ALIGNED

No hook-block reference beyond routing block-audit events through the same FileSink path.
That routing is correct and unchanged.

---

### BC-1.12.002 (debug stream VSDD_DEBUG_LOG gate)

**Status:** ALIGNED

No hook-block reference. Governs debug stream gate semantics (OQ-W16-011 resolution).
Unaffected by rc.12.

---

### BC-1.12.003 (Resource attributes startup stamping)

**Status:** ALIGNED

No hook-block reference. Governs the 15-field Resource attribute block. Unaffected.

---

### BC-1.12.004 (per-event host stamping and event.category registry)

**Status:** ALIGNED

Governs per-event field stamping including `hook.tool_name` and `hook.event_name`.
The `vsdd.block.*` → `audit` category mapping is present and correct in Postcondition 2.
No hook-block message format referenced; the per-event stamping contract is orthogonal
to the message body of `permissionDecisionReason`. No amendment needed.

---

### BC-1.12.005 (host_field_override visibility)

**Status:** ALIGNED

No hook-block reference. Governs override-visibility on domain events. Unaffected.

---

### BC-1.12.006 (primary target — block-path audit)

**Status:** DRIFT (LOW severity)

**Findings:**

The BC correctly specifies the `vsdd.block.plugin_blocked.v1` event with `outcome=blocked`,
`plugin.name`, and `hook.tool_name`. This aligns with ADR-015 D-15.3 and rc.12's
dispatcher behavior.

The gap is a missing `reason` field. rc.12 establishes that blocking hooks emit
`emit-event type=hook.block hook=<name> reason=<code>` — where `reason` is the
snake_case `Code:` value from the canonical block message. This `hook.block` telemetry
event is emitted by `block_pre` / `block_pre_json` bash helpers and by WASM hooks via
`HookResult::block_with_fix`. It is conceptually distinct from the
`vsdd.block.plugin_blocked.v1` dispatcher audit event.

However, `BC-1.12.006 Postcondition 2` lists the minimum required fields on
`vsdd.block.plugin_blocked.v1`. A natural question is: should the dispatcher also surface
the `Code:` reason identifier from `HookResult::Block`'s `reason` string into the audit
event as a structured `block.reason_code` field? The current spec leaves this gap open.

With rc.12, the `HookResult::Block { reason }` value for WASM hooks is now the
canonical formatted string `"BLOCKED by <hook>: <reason>. Fix: <recommendation>. Code: <code>."`.
The `permissionDecisionReason` surfaced to Claude Code is this formatted string.
The dispatcher currently extracts only `plugin.name` and `hook.tool_name` for the audit
event — the structured `Code:` identifier remains embedded in the opaque `reason` string.

**Assessment:** This is low-severity drift rather than a hard misalignment. BC-1.12.006
uses "at minimum the following fields", so the field list is extensible by the
implementation. However, a story implementer would not know to extract and surface the
`Code:` value as a structured field on the audit event unless the BC says so. This is a
spec-completeness gap, not a contradiction.

**Suggested amendment:** Add a note to BC-1.12.006 Postcondition 2 acknowledging that
the `reason` string for WASM hooks implementing `block_with_fix` is the canonical
`"BLOCKED by <hook>: <reason>. Fix: <recommendation>. Code: <code>."` format, and that
a `block.reason_code` field (the snake_case `Code:` value) MAY be extracted and stamped
on the audit event for structured querying. Whether this field is REQUIRED vs OPTIONAL
is a product decision for the PO; the BC currently provides no guidance.

**Routing:** PO (field requirement decision) + architect (if it becomes required, adds
to Postcondition 2 table).

---

### BC-1.12.007 (call-graph invariant — deprecated types)

**Status:** ALIGNED

No hook-block reference. Governs that Router/SinkRegistry/DlqWriter are not called
from the production path after Wave 1. Unaffected.

---

### BC-1.12.009 (dual-emit five-state)

**Status:** ALIGNED

No hook-block reference. Governs Wave 2 event-pair identity fields. Unaffected.

---

### BC-2.06.001 (SDK semver bump)

**Status:** DRIFT (LOW severity)

**Findings:**

BC-2.06.001 correctly specifies the MAJOR semver bump for the `vsdd-hook-sdk` crate when
ADR-015 D-15.3 host-field-precedence semantics ship. The BC migration guidance lists
host-owned fields that plugin authors must remove from their payloads — this is correct
and complete for its stated scope.

The drift: rc.12 introduces `HookResult::block_with_fix(hook, reason, recommendation, code)`
as a NEW constructor alongside the retained `HookResult::block(reason)`. This is an
ADDITIVE ABI extension (not a breaking change to existing callers). BC-2.06.001 does not
mention this addition. The HOST_ABI.md on develop explicitly documents
`block_with_fix` under the "Block-message convention" section as the preferred constructor
for new blocking plugins going forward.

The MAJOR bump BC-2.06.001 governs is the host-field-precedence behavioral change, which
is still correct. But the CHANGELOG.md requirement in BC-2.06.001 Postcondition 2 should
note the `block_with_fix` addition as a "New API" item so plugin authors are aware of it
when upgrading. The current BC would produce a changelog that is silent about
`block_with_fix`.

**Assessment:** Low-severity gap — no existing spec is wrong; the BC just under-specifies
the changelog content for the additive `block_with_fix` constructor.

**Suggested amendment:** BC-2.06.001 Postcondition 2 "Breaking Changes" and changelog
requirement should be extended to also document a "New API" section covering
`HookResult::block_with_fix` as the preferred constructor for agent-actionable block
messages going forward, replacing bare `HookResult::block(reason)` for new sites.

**Routing:** architect (minor amendment to BC-2.06.001 Postcondition 2).

---

### BC-3.05.004 (observability-config v2 schema)

**Status:** ALIGNED

No hook-block reference. Governs `observability-config.toml` v2 schema fields. rc.12
block-message format is not a config schema concern. Unaffected.

---

### BC-4.09.001 (plugin migration — Wave 2 event naming)

**Status:** ALIGNED

Governs plugin-side `event.name` migration to reverse-DNS canonical form via `emit_pair`.
The hook-block message format is orthogonal to event naming. No block-message format
referenced. Unaffected.

---

### BC-4.02.002 (primary stale artifact — adapter exit-code mapping)

**Status:** DRIFT (MEDIUM severity)

**Findings:**

BC-4.02.002 Postcondition 2 and EC-001 specify:

> `N == 2` and stderr non-empty → `Block { reason = first non-empty stderr line }`

> EC-001: exit 2 with multi-line stderr → First non-empty stderr line is used as the reason

rc.12 changes the legacy-bash-adapter's exit-2 path to capture the **full stderr** up to a
**4 KiB cap**, not just the first stderr line. The implementation in
`crates/hook-plugins/legacy-bash-adapter/src/lib.rs` now reads:

```rust
// Capture the full stderr (up to 4 KiB), trimmed. Bash hooks now emit
// a single canonical line via lib/block.sh, but if a hook emits
// multi-line stderr we surface all of it rather than dropping
// everything after the first line.
let raw = outcome.stderr.trim();
let reason = if raw.is_empty() {
    format!("legacy bash hook {script_path} blocked (no stderr)")
} else if raw.len() > 4096 {
    let safe = raw.floor_char_boundary(4096);
    format!("{}…[truncated]", &raw[..safe])
} else {
    raw.to_string()
};
```

The BC's title, description, postconditions, and EC-001 all say "first stderr line" which
is now incorrect. The bc `Source Evidence` section cites
`crates/hook-plugins/legacy-bash-adapter/src/lib.rs:103-119` (pre-rc.12 line numbers)
which no longer reflects the implementation.

The BC's Canonical Test Vector `N=2, stderr "denied: foo" → Block { reason: "denied: foo" }`
still passes (single-line case). But a multi-line stderr test would now pass where it
previously would have failed the old contract (only first line returned), meaning the BC
under-specifies the current behavior.

**Why this matters beyond "brownfield BC":** This BC is cited by BC-4.01.003 (see below)
and the description text propagates into story test authorship. If an E-10 story author
(e.g., S-10.04 which implements block-path audit trail) writes integration tests against
BC-4.02.002, the "first stderr line" expectation would cause a test written to the old BC
to pass on a pre-rc.12 adapter and silently diverge on post-rc.12 — or vice versa.

**Suggested fix:** Amend BC-4.02.002 to:
1. Replace "first non-empty stderr line" with "full stderr trimmed to 4 KiB UTF-8-safe cap"
   in Description, Postcondition 2, EC-001, and the Canonical Test Vectors.
2. Add a new EC for the truncation path (stderr > 4096 bytes → truncated with `…[truncated]`
   suffix).
3. Update the title: `2 → Block (reason=full stderr trimmed to 4KiB OR synthetic)`.
4. Update Source Evidence path annotation to note rc.12 changed this path.

**Routing:** architect (behavioral amendment to a brownfield BC).
**Bundled with:** D-2 (BC-4.01.003 has the same gap — can be a single burst).

---

### BC-4.01.003 (legacy-bash-adapter exit-code mapping — brownfield)

**Status:** DRIFT (MEDIUM severity, same root cause as D-1 / BC-4.02.002)

**Findings:**

BC-4.01.003 Postcondition 2 states:

> `N == 2` → `HookResult::Block { reason }` where reason is the first non-empty stderr
> line (or empty).

This is the same pre-rc.12 description as BC-4.02.002. BC-4.01.003 is the earlier
brownfield extraction; BC-4.02.002 is a more detailed version of the same behavior. Both
say "first stderr line" and both are now wrong for the same reason.

The BC title is `legacy-bash-adapter maps bash exit codes to HookResult` — the title does
not pin the "first line" behavior, but the postconditions and test vectors do.

**Suggested fix:** Apply the same amendment as BC-4.02.002: replace "first non-empty stderr
line" with "full stderr trimmed to 4 KiB UTF-8-safe cap" in Postcondition 2 and EC-001.

**Routing:** architect.
**Bundled with:** D-1 (single amendment burst covers both).


## Drift items requiring amendment

### D-1 [MEDIUM]: BC-4.02.002 Postcondition 2 and EC-001 pin "first stderr line" — stale since rc.12

**Artifact:** `.factory/specs/behavioral-contracts/ss-04/BC-4.02.002.md`

**Why drift:** rc.12 changed `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` exit-2
path to capture full stderr (up to 4 KiB, `floor_char_boundary`-safe). The BC still says
"first non-empty stderr line" in its title, Description, Postcondition 2, EC-001, and
Canonical Test Vector. A test authored against the BC's multi-line edge case (EC-001) would
now pass the full captured stderr rather than just the first line.

**Suggested fix:**
- Title: `2 → Block (reason=full stderr trimmed to 4 KiB OR synthetic)`
- Description, Postcondition 2: Replace "first non-empty stderr line" with "full stderr,
  trimmed, up to a 4 KiB UTF-8-safe cap; if stderr exceeds 4096 bytes the reason string
  is truncated with `…[truncated]` suffix"
- EC-001: Update to "full stderr is used as the reason (trimmed, up to 4 KiB)"
- Add EC-003: "exit 2 with stderr > 4096 bytes → reason is first 4096 bytes
  (UTF-8 safe boundary) with `…[truncated]` appended"
- Canonical Test Vectors: add "multi-line stderr → full stderr returned"; add
  "stderr > 4096 bytes → truncated with `…[truncated]`"
- Source Evidence: note rc.12 changed the implementation

**Routing:** architect
**Bundled with:** D-2 (single burst fixes both stale exit-2 BCs together)

---

### D-2 [MEDIUM]: BC-4.01.003 Postcondition 2 pins "first non-empty stderr line" — stale since rc.12

**Artifact:** `.factory/specs/behavioral-contracts/ss-04/BC-4.01.003.md`

**Why drift:** Same root cause as D-1. BC-4.01.003 is the earlier extraction of the same
adapter behavior. Postcondition 2 says "reason is the first non-empty stderr line (or
empty)". EC-001 covers "exit 2 with empty stderr" but not the multi-line or truncation
case — the multi-line case now produces the full stderr, not just the first line.

**Suggested fix:** Apply same amendment as D-1: replace "first non-empty stderr line"
with "full stderr trimmed to 4 KiB UTF-8-safe cap" in Postcondition 2.

**Routing:** architect
**Bundled with:** D-1

---

### D-3 [LOW]: BC-1.12.006 Postcondition 2 does not mention reason-code field from canonical block message

**Artifact:** `.factory/specs/behavioral-contracts/ss-01/BC-1.12.006.md`

**Why drift:** rc.12 standardizes that all blocking hooks include a snake_case `Code:`
value in `HookResult::Block.reason` (via `block_with_fix`). The dispatcher receives this
formatted string. BC-1.12.006 Postcondition 2's minimum field table does not include a
`block.reason_code` field, nor does it acknowledge that the `reason` string of
`HookResult::Block` now has a parseable structure. A story implementer has no spec guidance
on whether to extract the `Code:` identifier as a structured field on the audit event.

**Suggested fix:** Add a note to BC-1.12.006 Postcondition 2 below the field table:
> NOTE (rc.12): For plugins using `HookResult::block_with_fix`, the `reason` field in
> `HookResult::Block` is the canonical formatted string
> `"BLOCKED by <hook>: <reason>. Fix: <recommendation>. Code: <code>."`. The
> `<code>` value (snake_case) is the telemetry reason code emitted by `block_pre` /
> `block_pre_json` helpers and by the `hook.block` telemetry event's `reason` field.
> Whether `block.reason_code` (the extracted `<code>`) is added as a structured field
> on `vsdd.block.plugin_blocked.v1` is a PO product decision. If added, it MUST be
> populated from the canonical `Code:` suffix of `HookResult::Block.reason`.

**Routing:** PO (product decision on field requirement) → architect (add to Postcondition
2 if PO decides REQUIRED)

---

### D-4 [LOW]: BC-2.06.001 CHANGELOG requirement does not mention block_with_fix additive API addition

**Artifact:** `.factory/specs/behavioral-contracts/ss-02/BC-2.06.001.md`

**Why drift:** rc.12 adds `HookResult::block_with_fix` as a preferred constructor across
all 24 blocking hooks. This is an additive SDK change that accompanies the MAJOR semver
bump BC-2.06.001 governs. The BC's CHANGELOG requirement (Postcondition 2) specifies a
"Breaking Changes" section but no "New API" section. Plugin authors upgrading to the new
major version would not learn about `block_with_fix` from a changelog produced per this
BC's current spec.

**Suggested fix:** Extend BC-2.06.001 Postcondition 2 to require a "New API" section in
CHANGELOG.md documenting:
- `HookResult::block_with_fix(hook, reason, recommendation, code)` — preferred constructor
  for agent-actionable block messages; formats the canonical
  `"BLOCKED by <hook>: <reason>. Fix: <recommendation>. Code: <code>."` single-line shape
- `HookResult::block(reason)` retained for backward compatibility; new plugin sites SHOULD
  use `block_with_fix`

**Routing:** architect


## No-impact zones (positive findings)

- **ADR-015** — D-15.3 block path audit trail specifies `vsdd.block.plugin_blocked.v1`
  fields abstractly (`outcome`, `plugin.name`, `hook.tool_name`). The canonical message
  format is a Claude Code delivery concern, not an OTel schema concern. No amendment needed.
  The `vsdd.block.*` → `audit` category mapping in the compile-time registry is unchanged.

- **BC-1.11.001** (exec_subprocess trace injection) — no hook-block scope. ALIGNED.

- **BC-1.11.002** (FileSink partial-write) — no hook-block scope. ALIGNED.

- **BC-1.11.003** (emit_pair dual-emit) — no hook-block scope. ALIGNED.

- **BC-1.12.001** (FileSink single-stream routing) — routes block-audit events through
  the same FileSink path as all other events. Unchanged. ALIGNED.

- **BC-1.12.002** (debug stream gate) — no block-message format reference. ALIGNED.

- **BC-1.12.003** (Resource attribute startup stamping) — 15-field block unchanged in
  rc.12. ALIGNED.

- **BC-1.12.004** (per-event host stamping, event.category registry) — `vsdd.block.*` →
  `audit` in the compile-time registry is correct. `hook.tool_name` and `hook.event_name`
  are host-stamped per-event fields, unchanged. ALIGNED.

- **BC-1.12.005** (host_field_override visibility) — no hook-block scope. ALIGNED.

- **BC-1.12.007** (call-graph invariant) — no hook-block scope. ALIGNED.

- **BC-1.12.009** (dual-emit five-state) — no hook-block scope. ALIGNED.

- **BC-3.05.004** (observability-config v2 schema) — no hook-block format reference.
  Schema fields unchanged by rc.12. ALIGNED.

- **BC-4.09.001** (plugin event-name migration) — governs reverse-DNS event naming via
  `emit_pair`. Block-message format is orthogonal. ALIGNED.

- **Stories S-10.02 / S-10.03 / S-10.04 / S-10.05 / S-10.09** — no AC references to
  specific block-message string formats. S-10.04 AC-004 tests `vsdd.block.plugin_blocked.v1`
  emission at the event-payload level; it does not assert the canonical `reason` string
  shape, so it is unaffected. ALIGNED.

- **Capabilities CAP-008, CAP-029, CAP-010** — no block-message format references.
  ALIGNED.

- **Invariants DI-007, DI-008, DI-013, DI-017** — no block-message format references.
  ALIGNED.

- **BC-INDEX, ARCH-INDEX, STORY-INDEX** — no hook-block-format-specific anchor
  descriptions that would need updating. BC-4.02.002 / BC-4.01.003 titles include "first
  stderr line" language that would need updating as part of D-1 / D-2, but the INDEX
  title rows are secondary to the BC files themselves and will be updated in the same
  burst.


## Recommendation

Verdict is DRIFT_MINOR. Do NOT proceed directly to adversary pass-5.

Dispatch a single amendment burst (D-1 through D-4) before pass-5:

**D-326 format-alignment burst** (architect):
1. Amend BC-4.02.002 per D-1 (full stderr 4 KiB, truncation edge case)
2. Amend BC-4.01.003 per D-2 (same root cause, co-located fix)
3. Amend BC-1.12.006 per D-3 (add rc.12 NOTE on block_with_fix reason-code structure)
4. Amend BC-2.06.001 per D-4 (add block_with_fix to CHANGELOG New API requirement)

D-3 requires a PO product decision before the architect can mark `block.reason_code`
REQUIRED vs OPTIONAL. If the PO decides OPTIONAL (no requirement to extract it), the
D-3 amendment is a note-only addition to BC-1.12.006 Postcondition 2 and can be done
entirely by the architect. If REQUIRED, the PO must update BC-1.12.006 Postcondition 2
field table and the story-writer must update S-10.04 AC-004. Recommend PO routing for
D-3 resolution first, then fold into D-326 architect burst.

**Ordering:** PO resolves D-3 product question → architect writes D-1 + D-2 + D-3 note
+ D-4 in one burst → state-manager seals → adversary pass-5 proceeds.


## Process-gap candidates

- **Spec-vs-engine drift detection:** The D-1/D-2 drift (brownfield BCs pinning
  implementation details at extraction time) was not caught by the existing adversarial
  passes because those passes focused on the E-10 architecture, not on pre-existing
  brownfield BCs that the E-10 engine changes made stale. A CI gate that re-runs a
  "behavioral contract vs implementation" diff after each engine release (even just a
  grep for "first stderr line" against the implementation) would have surfaced D-1/D-2
  automatically at rc.12 tag time. Currently the only detection mechanism is this manual
  audit. Codification follow-up recommended for v1.1: add a `make check-bc-impl-drift`
  target that greps BC postcondition text against known-changed implementation patterns
  after each release.

- **block_with_fix adoption tracking:** rc.12 introduces `block_with_fix` as the
  canonical preferred constructor, but no spec or story currently REQUIRES new blocking
  plugin sites to use it rather than bare `HookResult::block(reason)`. A policy BC or
  story AC requiring `block_with_fix` for any new blocking plugin site added after rc.12
  would ensure the canonical format spreads consistently. Currently the spec says
  `block_with_fix` is "preferred" (HOST_ABI.md) but leaves brownfield sites using bare
  `block()` unaddressed. Consider a Wave 5 / post-rc.12 cleanup story for the hook-plugin
  crates.
