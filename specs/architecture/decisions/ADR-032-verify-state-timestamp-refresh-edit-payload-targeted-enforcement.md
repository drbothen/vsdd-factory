---
document_type: architecture-decision-record
level: L3
adr_id: ADR-032
version: "1.10"
title: "ADR-032: verify-state-timestamp-refresh — Edit/MultiEdit payload-targeted timestamp enforcement"
status: proposed
date: 2026-07-20
producer: architect
timestamp: 2026-07-20T00:00:00Z
deciders:
  - architect
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
related_adrs:
  - ADR-025 (single-writer factory lock/lease — STATE.md write discipline; factory_lock schema; §12.2/§12.3 superseded-in-part by ADR-032 for payload-neutral Edit/MultiEdit; amended to v1.22 for §12.3 annotation first-sentence correction)
  - ADR-014 (Tier-2 native WASM migration — standing mandate: new hooks MUST be native WASM; POLICY 21)
anchors:
  - SS-04
  - SS-05
subsystems_affected:
  - SS-04
  - SS-05
last_amended: "2026-07-20 (v1.10) — P6/P7 provenance relabel burst (orchestrator-verified, D-868): F-ADR032-P6-001..P6-008 and F-ADR032-P7-001..P7-007 attribution corrected — both sets were UNAUTHORIZED self-review bursts by a shadow subagent chain (no adversary pass 6 or pass 7 ran; prefixes retained for append-only ID stability; content pending adversarial verification). Freeze commit 87745b8e human-authorized. No normative content changed. cascade-log v1.0→v1.1. [Prior: 2026-07-20 (v1.9) — UNAUTHORIZED shadow-chain burst: F-ADR032-P7-001 through P7-007 (no adversary pass 7 ran; prefixes retained for append-only ID stability; content pending adversarial verification). Original content — drop factory-artifacts branch discriminator field (tautology; worktree separation makes Pre-condition 2 sufficient); priority 155→159 (155 occupied by validate-stable-anchors); §Source/Origin Continue phrasing corrected; step-numbering scheme annotations added to Decisions 1/3/5; AC-020 entry made byte-identical across Decision 5 and Work Spec Part 2; dispatcher-side tests updated to three keys + negative branch; `async = false` rationale corrected to explicitness/parity justification. [Prior: 2026-07-20 (v1.8) — UNAUTHORIZED shadow-chain burst: F-ADR032-P6-001 through P6-008 (no adversary pass 6 ran; prefixes retained for append-only ID stability; content pending adversarial verification). Original content — stateless AC-021 discriminator redesign (eliminates cross-invocation Mutex cache); to_json() explicit mandate; async = false restored; BC traces Steps 4–8; dispatcher emission tests mandated; Decision 1 phrasing unified. [Prior: 2026-07-20 (v1.7) — provenance correction (orchestrator-verified via dispatcher-log session audit): v1.3 and v1.6 changelog attributions corrected — F-ADR032-P3-001..007 was architect self-review during fix burst 3 (not adversary findings; adversary pass-3 finding set is P3B — F-ADR032-P3B-001..003 — closed at v1.4); F-ADR032-P5-001..008 was architect self-review during fix burst 4 (no adversary pass 5 ran; P5 prefix retained for append-only ID stability). No normative content changed. ADR-032 v1.7. ARCH-INDEX v3.17→v3.18. [Prior: 2026-07-20 (v1.6) — architect self-review during fix burst 4: 8-finding closure. Note: F-ADR032-P5-001..008 set was architect self-review, NOT from adversary pass 5; no adversary pass 5 ran; the P5 prefix is retained for append-only ID stability. F-ADR032-P5-001+P5-006: guard_logic docstring update section split — Part 1 (module //! docstring step 3a) + Part 2 (guard_logic function # BC traces AC-020); module step numbering clarification note added to Part 1; AC-020 # BC traces entry uses module //! step numbering. F-ADR032-P5-002: Third Deliverable item 2 comment mandate replaced with implementation checklist note. F-ADR032-P5-003: test #11 Red Gate YES (Block(TimestampStale) pre-fix); four Red Gate tests; §Status updated. F-ADR032-P5-004: AC-021 two pre-conditions (is_factory_artifacts_commit; state_md_in_commit); prereq deliverable extended; fail-open table updated; §Consequences Negative (c) updated; async=false removed; explicit stanza added. F-ADR032-P5-005: Third Deliverable item 1 awk: front==1 frontmatter gate + inserted guard + insertion order; post-condition strengthened. F-ADR032-P5-007: Decision 1 vacuous parenthetical deleted. F-ADR032-P5-008: AC-021 explicit registry stanza. [Prior: 2026-07-20 (v1.5) — F-ADR032-P4 fix burst (architect): 6-finding closure. F-ADR032-P4-001: ARCH-INDEX ADR-032 row refreshed — Decision 1 two-field payload-neutral condition; Decision 5 guard_logic function docstring; AC-021 exec-free mechanism (ADR-032-AC021-prereq); ADR-025 amended to v1.22 (ARCH-INDEX v3.15→v3.16). F-ADR032-P4-002: Option A architectural ruling — AC-021 redesigned exec-free: dispatcher injects head_state_timestamp + head_parent_state_timestamp into git_context (invoke.rs GitContext struct + build_git_context + inject_git_context_if_qualifying touchpoints; ADR-032-AC021-prereq Work Spec section added); WASM reads payload.extra[git_context] fields; exec_subprocess capability removed; BC-5.41.003 PC1 unconditionally preserved; no carve-out. F-ADR032-P4-003: dissolved by Option A — factory_dir resolves worktree path host-side; WASM exec-free; noted in Work Spec evidence block. F-ADR032-P4-004: evidence block corrected — verbatim 4-match grep stdout (954, 976, 1065, 1087); bare line-number pins removed; references by entry name + tool regex. F-ADR032-P4-005: AC-020 label renamed timestamp-neutrality → payload-neutrality at Decision 5 body and Work Spec docstring (changelog historical mentions retained). F-ADR032-P4-006: v1.4 last_amended + Changelog findings renamed F-ADR032-P3B-001/002/003 (v1.3 set retains F-ADR032-P3-001/002/003 per append-only); volatile line pin 'validate-burst-log/validate-dispatch-advance registry lines 976/1087' corrected to entry-name + tool-regex anchor; clarifying sentence added. [Prior: 2026-07-20 (v1.4) — F-ADR032-P3 additional findings (architect): 3-finding closure. F-ADR032-P3B-001: AC-021 re-scoped from ^(Edit|Write|MultiEdit)$ per-write trigger to ^Bash$ per-commit trigger (option (i) — ABI feasible: ADR-029 git_context injection on PostToolUse Bash git-commit events confirmed; ^Bash$-triggered validate-burst-log and validate-dispatch-advance registry entries confirm pattern; BC-5.41.003 PC1 restricts exec_subprocess for commit-context acquisition only; factory-artifacts worktree at .factory/; git -C .factory show HEAD:STATE.md + HEAD^:STATE.md comparison); §Consequences bullet (c) updated to per-commit detection; POLICY 15 evidence block in Work Spec. F-ADR032-P3B-002: Decision 1 "If NO new_string sets timestamp: → Continue; Steps 4–8" corrected to "If NO new_string sets EITHER timestamp: OR factory_lock: (payload-neutral — neither field) → Continue; Steps 4–7"; Decision 5 AC-020 text corrected ("sets timestamp:" → "sets EITHER timestamp: OR factory_lock:"); §Source/Origin "per Decision 1" corrected to "per Decision 1+3" with explicit payload-neutral definition; ADR-025 v1.22 cross-referenced. F-ADR032-P3B-003: scope-deviation doc comment removed from crates/factory-lock/src/lib.rs acquire_lock (git diff crates/ empty post-removal). [Prior: 2026-07-20 (v1.3) — F-ADR032-P3 fix burst (architect): 7-finding closure (architect self-review during fix burst 3; F-ADR032-P3-001..007 set was NOT produced by any adversary pass; the adversary pass-3 finding set is the P3B set — F-ADR032-P3B-001..003 — closed at v1.4). F-ADR032-P3-001: AC-021 broadened to ^(Edit|Write|MultiEdit)$ — read_file capability added; post-write on-disk comparison via host::read_file uniform across all tool types; advisory gates on last-committed HEAD comparison (not per-Edit unconditionally); §Consequences Negative bullet (c) updated to reflect broadened scope. F-ADR032-P3-002: two volatile line pins removed — (line 7) from §Context timestamp: reference; lines ~207–223 from Third Deliverable ground-truth heading; function-anchor citations retained. F-ADR032-P3-003: §Rationale heading corrected — 'Why the lock-expiry fix is symmetric' replaced with 'Why the same payload-scan technique also eliminates spurious LockExpiryStale blocks (Decision 3)'. F-ADR032-P3-004: Decision 5 AC-020 insertion target corrected to guard_logic function docstring # BC traces list after AC-019; false 'module docstring AC-005..AC-019 matrix' characterization removed (ground truth: grep shows AC-011..AC-019 only in # BC traces). F-ADR032-P3-005: §Consequences Negative 'pre-burst reference' replaced with 'last committed STATE.md (git show HEAD:.factory/STATE.md)'. F-ADR032-P3-006: volatile line pins (line 864/867 references) replaced with 'volatile line pins removed from Work Spec insertion-point prose (function-anchor citations retained)' in frontmatter last_amended and Changelog v1.2 row. F-ADR032-P3-007: Required new tests heading corrected from 'minimum 9' to 'minimum 10' (table has 10 rows). [Prior: 2026-07-20 (v1.2) — F-ADR032-P2 fix burst (architect): 7-finding closure per adversary pass 2. F-ADR032-P1-009: volatile line pins removed from Work Spec insertion-point prose (function-anchor citations retained); stable function anchors only (TD-VSDD-091/POLICY 19). F-ADR032-P2-001: factory_lock insertion anchor ground-truthed — factory-lock-write.sh _write_factory_lock_block: awk fires on front==2 (closing --- fence), inserts factory_lock: block before the closing ---, i.e. after last_amended:; branch (b) taken — normative placement mandate added (factory_lock: MUST be adjacent to timestamp:, above last_amended:); Third Deliverable scoped into Work Spec (factory-lock-write.sh _write_factory_lock_block + crates/factory-lock/src/lib.rs acquire_lock stub); §Consequences Positive bullet qualified (small ~50-char Edit = unlocked-path only). F-ADR032-P2-002: §Source/Origin ADR-025 §12.3 scope corrected — TimestampStale rows gated on sets_timestamp; LockExpiryStale rows gated on (sets_timestamp OR sets_factory_lock); ADR-025 v1.20→v1.21 cross-referenced. F-ADR032-P2-003: AC-021 made implementable — path_filter field removed (grep -n 'path_filter' plugins/vsdd-factory/hooks-registry.toml → 0 matches); old_content field confirmed absent from PostToolUse payload (payload.rs HookPayload struct: tool_input + tool_response + extra only); mechanism: compare timestamp: from tool_input.content vs git show HEAD:STATE.md output via exec_subprocess; capabilities: exec_subprocess binary_allow=[git] env_allow=[HOME,GIT_CONFIG_GLOBAL,XDG_CONFIG_HOME]; no read_file capability needed. F-ADR032-P2-004: test #9 (ac020_edit_factory_lock_in_new_string_stale_expires_blocks) fixture mandated to include timestamp: NEW (advancing) in new_string — Step 6 passes pre-fix; LockExpiryStale is load-bearing assertion both pre- and post-fix (true regression guard). F-ADR032-P2-005: ARCH-INDEX v3.12→v3.13 with corrected 4-index cite (BC v4.11 / VP v2.72 / STORY v4.228). F-ADR032-P2-006: §Status heading version-neutralized (was 'Status as of v1.0'). [Prior: 2026-07-20 (v1.1) — F-ADR032-P1 fix burst (architect): 13-finding closure per adversary pass 1. F-001: §Context + §Source/Origin corrected to attribute timestamp advancement requirement to ADR-025 §12.2 (not BC-5.40.001 PC4); PC4 scope clarified as factory_lock.expires_at renewal only. F-002: ADR-025 §12.2/§12.3 superseded-in-part by ADR-032 for payload-neutral Edit/MultiEdit; ADR-032 framing corrected (enforcement matrix IS changed; §12.2/§12.3 remain authoritative for payload-containing edits). F-003: false 'existing injection point' claim in §Source/Origin removed; new_string_sets_field confirmed 0 matches in crates/ (grep -rn new_string_sets_field crates/ → 0). F-004: Decision 3 PC4 gap closed — when sets_timestamp=true, guard runs full Steps 4–7 including lock expiry check regardless of sets_factory_lock; 'symmetric' claim removed; option (a) chosen (production-grade). F-005: '28-test unit suite' corrected to 50 tests (grep -c '#[test]' src/lib.rs → 50). F-006: test #8 label corrected (pre-fix Block is TimestampStale not LockExpiryStale); new regression guard added for Decision 3. F-007: Work Spec step numbering disambiguated; insertion point corrected. F-008: document_type + level aligned to sibling convention. F-009: volatile line pins removed. F-010: PostToolUse advisory scoped into Implementer Work Spec as second deliverable (AC-021). F-011: PC6 characterization corrected (single-dev zero-friction; fail-open attributed to ADR-025 Decision 7). F-012: §Alternatives D-866 candidate directions traceability added. F-013: ARCH-INDEX ADR-032 row enriched to sibling parity. [Prior: 2026-07-20 (v1.0) — initial ruling (architect): D-866 edit-mechanism defect resolution.]"
modified:
  - "2026-07-20 (v1.0)"
  - "2026-07-20 (v1.1)"
  - "2026-07-20 (v1.2)"
  - "2026-07-20 (v1.3)"
  - "2026-07-20 (v1.4)"
  - "2026-07-20 (v1.5)"
  - "2026-07-20 (v1.6)"
  - "2026-07-20 (v1.7)"
  - "2026-07-20 (v1.8)"
  - "2026-07-20 (v1.9)"
  - "2026-07-20 (v1.10)"
---

# ADR-032: verify-state-timestamp-refresh — Edit/MultiEdit payload-targeted timestamp enforcement

## Context

The `verify-state-timestamp-refresh` WASM hook plugin (SS-04; BC-5.40.001 PC4; S-17.04;
`crates/hook-plugins/verify-state-timestamp-refresh/`) enforces timestamp freshness on every
PreToolUse Edit/Write/MultiEdit targeting `.factory/STATE.md`. For Edit and MultiEdit operations,
it reconstructs the proposed full-file content by applying the edit's `old_string → new_string`
substitution to the current on-disk bytes, then extracts and compares the `timestamp:` frontmatter
field between the reconstructed proposed content and the current on-disk content. Specifically,
Step 6 of `guard_logic` fires: `proposed_ts == on_disk_ts → Block: TimestampStale`.

This reconstruction-based approach has a specific failure mode under multi-Edit burst patterns.
After state-manager issues Edit-1 to advance the `timestamp:` frontmatter field (on-disk timestamp
becomes NEW), any subsequent Edit-2 targeting body content (for example the `## Session Resume
Checkpoint` section body, or the Decisions Log table) reconstructs a proposed content whose
`timestamp:` equals the current on-disk value (NEW), because the reconstruction base is the
post-Edit-1 on-disk file. Step 6 then compares NEW == NEW and blocks, even though the timestamp
was legitimately advanced in Edit-1.

The practical consequence is that state-manager cannot split a STATE.md burst into multiple Edit
calls (one small Edit for timestamp advancement, one per body section). Every Edit must include a
single contiguous region spanning from `timestamp:` through the target body region in
one `old_string`/`new_string` pair. This produces 150–350-line verbatim payloads. Such payloads
have corrupted `.factory/STATE.md` in 3 of the 4 remediation bursts preceding D-866: one
truncation event and two duplicate-block incidents, consuming approximately 2.8M tokens in
recovery work. The MultiEdit harness tool, which would allow atomic multi-site edits, is not
available in this Claude Code session.

The root cause is an over-constraint: the hook enforces timestamp advancement per-Edit-operation
rather than per-burst-commit. ADR-025 §12.2's requirement is "every STATE.md write must advance
`timestamp:`" — this was intended to mean every burst commit that produces a STATE.md write, not
every individual Edit within a burst. BC-5.40.001 PC4 governs `factory_lock.expires_at` mid-burst
renewal (not the `timestamp:` field); the per-Edit comparison against the current on-disk value is
an implementation artifact that does not match either invariant's intent.

An analogous over-constraint exists for `factory_lock.expires_at` (Step 7 in `guard_logic`): a
body-only Edit that does not touch the `factory_lock:` block reconstructs a proposed content with
the same lock state as on-disk, and the `proposed_expires == on_disk_expires` check fires a
spurious `Block: LockExpiryStale` whenever a lock is held.

This defect was root-caused and documented at D-866 (`.factory/cycles/v1.0-brownfield-backfill/
session-checkpoints.md` §"## D-866 Checkpoint" item 4) and flagged as requiring an architect
ruling before any fix is implemented.

## Decision

For Edit and MultiEdit operations on `.factory/STATE.md`, the `verify-state-timestamp-refresh`
guard fires timestamp enforcement ONLY when the edit's payload EXPLICITLY writes the `timestamp:`
frontmatter field. Symmetrically, lock-expiry enforcement fires ONLY when the payload explicitly
writes a `factory_lock:` block. Write operations are UNCHANGED.

**Decision 1 — Payload-targeted timestamp check for Edit/MultiEdit.** After extracting the
proposed content (current Step 3 of `guard_logic`) and before the timestamp extraction (current
Step 4), add a payload-scan step for Edit and MultiEdit tools only: scan each `new_string` value
in the payload for a top-level `timestamp:` field line. Detection uses
`factory_lock_parse::extract_yaml_string_value(line, "timestamp")` applied to each non-indented
line of `new_string`, skipping lines starting with whitespace (indented sub-fields or list items).
If NO `new_string` in the payload sets EITHER `timestamp:` OR `factory_lock:` (payload-neutral —
neither field is set), the guard skips Steps 4–7 entirely, then returns `Continue` at Step 8 (code-inline numbering). A `guard_ran (continue: payload-neutral)`
sentinel is emitted to stderr (parity with other Continue paths per AC-R5). A payload that sets
`factory_lock:` but not `timestamp:` is not payload-neutral; see Decision 3.

**Decision 2 — Write tool: UNCHANGED.** The Write tool provides the complete proposed file
content; the full timestamp check (Steps 4–8) applies unconditionally. No behavioral change for
Write operations.

**Decision 3 — Payload-targeted lock-expiry check for Edit/MultiEdit.** For Edit/MultiEdit, scan
`new_string` values for a top-level `factory_lock:` line (a non-indented line starting with
`factory_lock:`). This sets the boolean `sets_factory_lock`. The lock-expiry check (Step 7) fires
under the following conditions:

- If `sets_timestamp == true` (the payload explicitly advances `timestamp:`): **always run Step 7**
  (the full lock-expiry check), regardless of `sets_factory_lock`. A timestamp-advancing Edit is
  a burst commit action and MUST also renew `factory_lock.expires_at` when a lock is held (PC4
  — BC-5.40.001 §Postcondition 4). Skipping Step 7 for timestamp-advancing Edits when
  `sets_factory_lock == false` would allow a burst to advance `timestamp:` without renewing
  `expires_at`, silently violating PC4. Under the correct state-manager workflow, the same Edit
  that advances `timestamp:` must also include the `factory_lock:` block renewal in `new_string`.
- If `!sets_timestamp && sets_factory_lock == true`: skip Steps 4–6 (no timestamp check), run
  Step 7 only.
- If `!sets_timestamp && !sets_factory_lock` (payload-neutral): skip Steps 4–7 entirely
  (continue to Step 8 (code-inline numbering) → Continue).

The lock-expiry check is NOT symmetric with the timestamp check: both checks fire whenever
`sets_timestamp == true`. The `sets_factory_lock` gate only applies to the case where a payload
modifies the `factory_lock:` block without advancing `timestamp:` (an unusual but valid pattern,
e.g., lock release or acquire by state-manager). Step 7 must be wrapped in:
`if sets_factory_lock || sets_timestamp { ... }` in `guard_logic`.

**Decision 4 — New helper `new_string_sets_field`.** Add
`pub fn new_string_sets_field(new_string: &str, field_key: &str) -> bool` to `src/lib.rs`. It
iterates `new_string.lines()`, skips lines whose first character is a space or tab, and returns
`true` if any non-indented line yields `Some(_)` from
`factory_lock_parse::extract_yaml_string_value(line, field_key)`. The helper is independently
unit-tested. The `factory_lock:` detection in Decision 3 uses a separate inline scan
(`new_string.lines().any(|l| !l.starts_with(' ') && !l.starts_with('\t') && l.starts_with("factory_lock:"))`)
because `factory_lock:` is a block key (no value on the same line) that `extract_yaml_string_value`
does not match.

**Decision 5 — AC-020 in `guard_logic` function docstring.** Add `AC-020: Edit/MultiEdit payload-neutrality —
if no new_string in the payload sets EITHER timestamp: OR factory_lock:, guard returns Continue
(payload-neutral; module //! Steps 4–8 skipped (falls through to Step 9 → Continue))` to the `guard_logic`
function docstring `# BC traces` list, after AC-019.

## Rationale

**Why payload-targeted PreToolUse over PostToolUse enforcement (Options B/C):** A PostToolUse
hook fires after the write has landed on disk. It cannot prevent a bad write — only detect it.
All PostToolUse hooks in this registry carry `on_error = continue`, so even a `block_intent=true`
result from a PostToolUse hook has no effect on the already-completed write. The distinction is
load-bearing: BC-5.40.001 PC4's goal is prevention, not detection. Additionally, the "detect
stale compared to what?" question at PostToolUse requires a pre-burst reference value. That
reference requires cross-invocation state persistence, which is not available in the WASM sandbox
without a dedicated sidecar file — a new class of complexity and failure mode. Payload-targeted
PreToolUse requires no cross-invocation state.

**Why payload-targeted check over full-Edit-check removal (Option A):** Fully removing timestamp
enforcement for Edit operations would allow any Edit whose `new_string` contains an explicit
`timestamp: "STALE"` assignment to slip through without a block. The payload-targeted approach
preserves enforcement for the critical case: if a `new_string` explicitly contains a `timestamp:`
assignment, that value must be advanced. This is the exact feedback state-manager relies on to
detect a timestamp-regression error.

**Why the `new_string` scan is reliable:** STATE.md body content uses `## `-heading and
indented/list-item formatting enforced by the factory-write discipline. A bare column-0 line
starting with `timestamp: "..."` appearing in the body (not in the frontmatter) is structurally
impossible under current formatting conventions. The indented-lines skip in `new_string_sets_field`
also prevents false positives from YAML sub-fields or list items containing `timestamp:`.
Critically, a false positive (a body line incorrectly detected as a timestamp field) causes
unnecessary enforcement, not a bypass: the guard will check advancement and may block if the
extracted value matches on-disk, which is a safe failure mode.

**Why the same payload-scan technique also eliminates spurious LockExpiryStale blocks (Decision 3):** The same structural over-constraint that
produces false timestamp blocks for body-only Edits also produces false lock-expiry blocks. The
reconstructed proposed content of a body-only Edit inherits the on-disk `factory_lock:` block
unchanged. If a lock is held, the guard fires `proposed_expires == on_disk_expires →
Block: LockExpiryStale`. Decision 3 eliminates this spurious block by the same payload-scan
logic.

**Invariant preservation:** The invariant "every STATE.md commit advances `timestamp:`" is
preserved by: (a) the hook continues to enforce advancement for any Edit/Write that explicitly
sets `timestamp:` — the exact operation state-manager must issue at least once per burst; and (b)
the Write-tool path (used for full-file STATE.md writes) enforces unconditionally. What the new
design no longer catches is a burst that issues only body-only Edits and never explicitly advances
the timestamp. That omission has not occurred historically; the Write-path fallback covers
full-rewrite scenarios; and the process discipline for state-manager already requires an explicit
timestamp-advancing Edit per burst. Additionally, a per-commit advisory is provided (see below).

## Consequences

### Positive

- Eliminates the large-`old_string` constraint for body-only Edits. On the **unlocked path**,
  state-manager can issue a small, targeted Edit for the timestamp field (unique `old_string`
  of ~50 characters) and one or more separate Edits for each body section without triggering
  the hook. See §Consequences Negative for the lock-held path qualification.
- Removes the root cause of the 3/4-burst STATE.md corruption pattern (not a workaround).
- Eliminates the ~2.8M tokens per cycle consumed in truncation and duplicate-block recovery.
- No behavior change for Write operations or for Edits that explicitly set `timestamp:`.
- Decision 3 symmetrically eliminates spurious LockExpiryStale blocks on body-only Edits when
  a lock is held.
- The `new_string_sets_field` helper is independently testable and reusable for future
  payload-targeted checks in other guards.

### Negative / Trade-offs

- Slight weakening of the per-Edit guarantee: a burst that issues only body-only Edits (zero
  explicit timestamp Edit) and then commits will pass through the hook without a TimestampStale
  block. Under the prior design, the final body-only Edit would have blocked (its reconstructed
  content compared against the same stale on-disk timestamp). Mitigation: (a) state-manager
  process discipline requires an explicit timestamp-advancing Edit per burst; (b) the Write-tool
  path enforces unconditionally for full rewrites; (c) a PostToolUse advisory sentinel (AC-021,
  scoped into the Implementer Work Spec below) fires on Bash git-commit events; factory-artifacts
  exclusivity is guaranteed structurally (`.factory/` is a separate git worktree on the orphan
  `factory-artifacts` branch — no develop-branch commit can include `.factory/STATE.md` in its
  changed files) and STATE.md appears in the commit diff (`state_md_in_commit == true`); it then
  compares STATE.md `timestamp:` between HEAD and HEAD^ after each commit, detecting "burst
  committed without advancing timestamp" at the per-commit boundary — the correct detection point
  for the ADR-025 §12.2 invariant. Advisory-only; `on_error=continue`; no blocking. False positives
  on develop-branch commits are structurally impossible; false positives on factory-artifacts commits
  that do not touch STATE.md are eliminated by Pre-condition 2 (`state_md_in_commit == true`,
  gated at the dispatcher prereq injection layer).
- The `new_string_sets_field` line-scan is a heuristic with a theoretical false-positive if
  STATE.md body ever acquires a column-0 line starting with `timestamp: "..."`. Under current
  factory-write discipline this is structurally impossible, but it is not separately enforced.
  If STATE.md formatting conventions ever change, this assumption should be revisited.
- **Lock-held path payload size (load-bearing constraint, F-P2-001 branch b).** Decision 3
  requires that any Edit advancing `timestamp:` under a held lock MUST also renew
  `factory_lock.expires_at` in the same `new_string`. The payload size therefore depends on
  the frontmatter distance between `timestamp:` and `factory_lock:`. Ground truth:
  `factory-lock-write.sh` `_write_factory_lock_block` uses awk that fires on `front == 2`
  (the closing `---` fence) to insert the `factory_lock:` block — placing it AFTER
  `last_amended:` (~32 KB; ADR-025 v1.17 measurement). At this position, a combined
  `old_string`/`new_string` spanning both `timestamp:` and `factory_lock:` must include the
  entire `last_amended:` field, reintroducing the large-payload corruption pattern this ADR
  was designed to eliminate. **Mitigation (mandatory per Third Deliverable):** move
  `factory_lock:` insertion to immediately after `timestamp:`, above `last_amended:`. Until
  the Third Deliverable is implemented, the lock-held burst path retains the large-payload
  risk. See §Implementer Work Spec for the concrete change scope.

### Status

PROPOSED. Awaiting implementation assignment. The implementing story must target
`crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs` with the changes specified in
§Implementer Work Spec below, the four Red Gate tests must fail against the current unmodified
`guard_logic` and pass only after the fix, and the full existing unit suite (50 tests as of this
writing; `grep -c '#\[test\]' src/lib.rs → 50`) must remain green.

## Alternatives Considered

- **Option A — Remove Edit/MultiEdit timestamp check entirely:** Edit operations would bypass
  timestamp validation unconditionally. Rejected: eliminates enforcement for explicit timestamp
  regressions (an Edit intentionally setting `timestamp:` to an older value would pass through).

- **Option B — Move to PostToolUse actual-state check:** After each STATE.md write, read the
  on-disk file and compare the timestamp to a pre-burst reference value stored in a sidecar file.
  Corresponds to D-866 item 4 candidate direction: "relax the hook to accept a `timestamp:` that
  was advanced anywhere in the resulting file (not necessarily in the same Edit call)."
  Rejected: (1) PostToolUse hooks cannot prevent the write. (2) A pre-burst sidecar file
  introduces a new class of failure mode (stale sidecar, missing sidecar, sidecar races). (3)
  Changes enforcement model from prevention to detection.

- **Option C — PostToolUse advisory warning only:** PostToolUse fires and warns if the
  resulting STATE.md timestamp looks stale. Corresponds to D-866 item 4 candidate direction:
  "accept a sufficiently recent prior refresh within the same burst's commit" — i.e., detect
  the absence of a refresh event after the fact rather than blocking before.
  Rejected as primary enforcement: advisory-only warnings add noise without safety guarantees.
  Retained as a complementary second deliverable (AC-021) in the Implementer Work Spec.

- **Option D — Payload-targeted PreToolUse (chosen):** Scan `new_string` to determine whether
  the edit explicitly sets `timestamp:`. Enforce only when yes. Requires no cross-invocation state,
  no sidecar files, minimal code change, and is independently testable.

## Source / Origin

- **Root-cause and failure record:** `.factory/cycles/v1.0-brownfield-backfill/
  session-checkpoints.md` §"## D-866 Checkpoint" item 4 (3/4 bursts corrupted,
  ~2.8M tokens recovery, architect ruling flag).
- **Full decision record:** `.factory/cycles/v1.0-brownfield-backfill/decision-log.md` §D-866.
- **Hook implementation (current behavior):** `crates/hook-plugins/verify-state-timestamp-refresh/
  src/lib.rs` — `guard_logic` Step 6 (`if proposed_ts == on_disk_ts { return Block; }`) is the
  over-constraining check; Step 7 (`factory_lock.expires_at` comparison) has the symmetric issue.
  The injectable-callback architecture (`GuardCallbacks`) enables the required unit tests.
  `new_string_sets_field` is a NEW function (0 matches in `crates/` as of this ruling); it does
  not yet exist and must be added per ADR-032 Decision 4.
- **Behavioral contract:** BC-5.40.001 PC4 (mid-burst `factory_lock.expires_at` renewal;
  governs `expires_at` only, not the `timestamp:` field) and PC6 (single-developer zero-friction:
  guard returns `Continue` on self-held lock, preserved by this ruling). Fail-open behavior on
  error paths is governed by ADR-025 Decision 7 (efficiency-class lock path), not PC6.
- **ADR-025 §12.2** (every STATE.md commit must advance `timestamp:`; authoritative source of the
  timestamp advancement invariant) remains in effect for all payload-containing Edits. **§12.3**
  (fail-open/fail-closed table for Edit/MultiEdit) is superseded-in-part by this ADR: the
  payload-neutral (neither `timestamp:` nor `factory_lock:` in any `new_string`) Edit/MultiEdit
  invocations return `Continue` after skipping the timestamp and lock-expiry checks (Decision 1 and Decision 3).
  ADR-025 §12.3 rows remain authoritative for payload-containing Edits and all Write operations,
  with row scope split per Decision 3 (corrected in ADR-032 v1.2 / ADR-025 v1.21): (a) the
  **TimestampStale rows** apply ONLY when `sets_timestamp == true` — a factory_lock-only Edit
  (`!sets_timestamp && sets_factory_lock`) bypasses Steps 4–6 entirely; the TimestampStale rows
  do NOT apply to factory_lock-only payloads; (b) the **LockExpiryStale rows** apply when
  `sets_timestamp == true` OR `sets_factory_lock == true`. This ADR does NOT introduce fail-open
  behavior for the critical case — any Edit whose `new_string` explicitly contains a `timestamp:`
  assignment still goes through the full Steps 4–7 comparison. The §12.2 warning about
  "implementations that only check the fragment" (fail-open-on-all-Edit) does not apply:
  ADR-032 is payload-targeted, not fragment-only. See ADR-025 v1.22 §12.3 for the matching
  annotation correction.

---

## Implementer Work Spec

> Route via fix-pr-delivery to `vsdd-factory:implementer`. This section is the complete
> specification. Do NOT implement before the human has confirmed the routing.

### Target crate

`crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs`

### Function to add

```
pub fn new_string_sets_field(new_string: &str, field_key: &str) -> bool
```

Iterates `new_string.lines()`. Skips any line whose first byte is a space (0x20) or tab (0x09).
For each non-indented line, calls `factory_lock_parse::extract_yaml_string_value(line, field_key)`.
Returns `true` on the first `Some(_)` result. Returns `false` if no match is found or
`new_string` is empty.

### Changes to `guard_logic`

**Step numbering note:** The module-level `//!` docstring uses one numbering scheme (Step 2 =
extract proposed content, Step 3 = read on-disk). The code-inline function comments inside
`guard_logic` use a different scheme (Step 2 block = determine proposed content + read on-disk,
Step 3 = extract proposed content via `match payload.tool_name.as_str()`). The Work Spec below
uses **code-inline function numbering** throughout. Do not conflate the two.

**Insertion point:** AFTER the `match payload.tool_name.as_str()` block (code Step 3) completes
and `proposed_content: String` is fully bound, and BEFORE the `extract_top_level_field` call that
opens code Step 4 (timestamp extraction from proposed content). Insert after the
`match payload.tool_name.as_str()` arm that yields `proposed_content`, before the
`let proposed_ts = match extract_top_level_field(&proposed_content, "timestamp")` binding.

The logic reads `payload.tool_name` and the raw `tool_input` fields to compute two booleans:

```
sets_timestamp: bool  // any new_string sets timestamp: at column 0
sets_factory_lock: bool  // any new_string contains a non-indented line starting with factory_lock:
```

For `tool_name == "Write"`: both are `true` (Write always enforces all checks).

For `tool_name == "Edit"`:
- `sets_timestamp` = `new_string_sets_field(new_string, "timestamp")` where `new_string` is
  `payload.tool_input["new_string"].as_str().unwrap_or("")`.
- `sets_factory_lock` = `new_string.lines().any(|l| !l.starts_with(' ') && !l.starts_with('\t') && l.starts_with("factory_lock:"))`.

For `tool_name == "MultiEdit"`:
- `sets_timestamp` = `edits.iter().any(|e| new_string_sets_field(e["new_string"].as_str().unwrap_or(""), "timestamp"))`.
- `sets_factory_lock` = `edits.iter().any(|e| e["new_string"].as_str().unwrap_or("").lines().any(|l| !l.starts_with(' ') && !l.starts_with('\t') && l.starts_with("factory_lock:")))`.

For unknown `tool_name`: both `true` (conservative, same as Write).

After computing the booleans, apply the following routing:

If `!sets_timestamp && !sets_factory_lock`:
- Emit `guard_ran (continue: payload-neutral)` via `write_stderr`.
- Return `HookResult::Continue`.

If `!sets_timestamp && sets_factory_lock`:
- Skip Steps 4–6 (timestamp checks).
- Proceed directly to Step 7 (lock-expiry check) using the already-extracted `proposed_content`.

If `sets_timestamp` (with or without `sets_factory_lock`):
- Run Steps 4–6 normally (timestamp extraction and byte-comparison).
- Run Step 7 unconditionally (lock-expiry check). A timestamp-advancing Edit under a held lock
  MUST also renew `factory_lock.expires_at` in the same payload (PC4 — BC-5.40.001 §PC4). If
  the payload advances `timestamp:` but the proposed content carries a stale `expires_at`
  (because the payload did not include the `factory_lock:` block renewal), Step 7 fires
  Block(LockExpiryStale). This is the correct enforcement — the state-manager must include lock
  renewal in the same Edit that advances `timestamp:`.

Step 7 in `guard_logic` must be wrapped in `if sets_factory_lock || sets_timestamp { ... }`.
(For Write, both are `true`, so Step 7 always runs — no behavioral change for Write.)

### `guard_logic` function docstring update

**Part 1. Module `//!` docstring — insert step 3a:**

> Note: the snippet below uses **module `//!` step numbering** (exception to the
> code-inline-throughout rule stated in the Step numbering note above), because it is being
> inserted into the module `//!` docstring. Module `//!` Step 3 = read on-disk; module `//!`
> Step 4 = timestamp extraction from proposed content. The code-inline function numbering used
> elsewhere in this Work Spec does not apply to this snippet.

Add as a new step between module `//!` Step 3 (read on-disk) and Step 4 (timestamp extraction):

```
///   3a. For Edit/MultiEdit: scan new_string value(s) for top-level `timestamp:` and
///       `factory_lock:` fields (ADR-032 Decision 1+3).
///       - If neither is set: return Continue (guard_ran payload-neutral). AC-020.
///       - If only factory_lock: is set: skip Steps 4–7; proceed to Step 8.
///       - If timestamp: is set (with or without factory_lock:): run full check (Steps 4–8).
///       For Write: skip this step (full content always checked).
```

**Part 2. `guard_logic` function docstring — add AC-020 to `# BC traces` list:**

Add the following line to the `guard_logic` function docstring `# BC traces` list, after
AC-019. Step numbers in this entry use module `//!` step numbering (consistent with the
step-1-through-9 enumeration in the module `//!` docstring; module `//!` Steps 4–7 = timestamp
extraction, absent-in-proposed, absent-in-on-disk, byte-identical — the same range referenced
by the 3a snippet above):

`AC-020: Edit/MultiEdit payload-neutrality — if no new_string in the payload sets EITHER timestamp: OR factory_lock:, guard returns Continue (payload-neutral; module //! Steps 4–8 skipped (falls through to Step 9 → Continue))`

### Required new tests (minimum 11)

All new tests belong in the existing `#[cfg(test)] mod tests` block in `src/lib.rs`. They use
the same injectable-callback pattern as the existing 50 tests.

| Test ID | Description | Expected result | Red Gate? |
|---------|-------------|-----------------|-----------|
| `ac020_edit_body_only_no_timestamp_continues` | Edit where `new_string` is body text (no `timestamp:` line); on-disk OLD timestamp; reconstructed proposed has same OLD timestamp | `Continue` | YES — currently Block |
| `ac020_edit_explicit_stale_timestamp_blocks` | Edit where `new_string` contains `timestamp: "OLD"` explicitly | `Block(TimestampStale)` | No — regression guard |
| `ac020_edit_explicit_advanced_timestamp_continues` | Edit where `new_string` contains `timestamp: "NEW"` | `Continue` | No — regression guard |
| `ac020_multiedit_no_timestamp_in_any_new_string_continues` | MultiEdit where no `edits[i].new_string` contains `timestamp:`; on-disk OLD | `Continue` | YES — currently Block |
| `ac020_multiedit_one_new_string_stale_blocks` | MultiEdit where one `edits[i].new_string` contains `timestamp: "OLD"` | `Block(TimestampStale)` | No — regression guard |
| `ac020_write_stale_timestamp_still_blocks` | Write with stale full content (regression: Write path unchanged) | `Block(TimestampStale)` | No — regression guard |
| `ac020_new_string_sets_field_helper` | Unit test for `new_string_sets_field`: found at col-0; not found; indented skipped; multi-line mix | Various `bool` | No — new helper |
| `ac020_edit_body_lock_held_no_factory_lock_continues` | Edit: lock held on-disk with stale `expires_at`; `new_string` is body text with no `factory_lock:` line; on-disk timestamp is OLD | `Continue` | YES — currently Block(TimestampStale) |
| `ac020_edit_factory_lock_in_new_string_stale_expires_blocks` | Edit: `new_string` contains BOTH `factory_lock:` block (stale `expires_at`) AND `timestamp: "NEW"` (advancing); lock held on-disk with stale `expires_at` | `Block(LockExpiryStale)` | No — regression guard |
| `ac020_edit_sets_timestamp_no_factory_lock_stale_expires_blocks` | Edit: `new_string` sets `timestamp: "NEW"` (advancing); no `factory_lock:` in `new_string`; lock held on-disk with stale `expires_at` | `Block(LockExpiryStale)` | No — Decision 3 option (a) regression guard: timestamp-advancing Edit must include factory_lock renewal |
| `ac020_edit_factory_lock_only_stale_expires_blocks` | Edit where `new_string` sets `factory_lock:` block with stale `expires_at` but no `timestamp:` line; lock held on-disk with stale `expires_at` | `Block(LockExpiryStale)` | YES — currently Block(TimestampStale) |

The four **Red Gate** tests (`ac020_edit_body_only_no_timestamp_continues`,
`ac020_multiedit_no_timestamp_in_any_new_string_continues`,
`ac020_edit_body_lock_held_no_factory_lock_continues`,
`ac020_edit_factory_lock_only_stale_expires_blocks`) must FAIL on `cargo test` against the
current unmodified `guard_logic` and pass only after the fix is applied. The implementer must
verify this by running the test suite before and after the change.

Note on test `ac020_edit_body_lock_held_no_factory_lock_continues`: before the fix, `guard_logic`
processes a body-only Edit by reconstructing a proposed content that inherits both the on-disk
`timestamp:` (unchanged) and the on-disk `factory_lock:` (unchanged). Step 6 fires first
(`proposed_ts == on_disk_ts → Block(TimestampStale)`) before Step 7 is reached. The pre-fix block
is therefore `TimestampStale`, not `LockExpiryStale`. Post-fix the test passes (payload-neutral
Edit → Continue).

Note on test `ac020_edit_factory_lock_only_stale_expires_blocks`: before the fix, `guard_logic`
reconstructs the proposed content of this factory_lock-only Edit. The `new_string` does not
include `timestamp:`, so the reconstructed proposed content inherits the unchanged on-disk
`timestamp:` value. Module `//!` Step 7 fires (`proposed_ts == on_disk_ts →
Block(TimestampStale)`). Post-fix, `sets_timestamp = false` → module `//!` Steps 4–7 (timestamp
checks) are skipped; Step 8 runs the lock-expiry check → `Block(LockExpiryStale)`. Pre-fix
result (TimestampStale) ≠ post-fix result (LockExpiryStale) → this is a Red Gate. The expected
post-fix result is `Block(LockExpiryStale)`.

Note on test `ac020_edit_factory_lock_in_new_string_stale_expires_blocks` (F-ADR032-P2-004):
the fixture MUST include `timestamp: "NEW"` (advancing) in `new_string` alongside the
`factory_lock:` block with stale `expires_at`. Without this, the block code changes across the
fix (TimestampStale pre-fix → LockExpiryStale post-fix), making the test a Red Gate rather than
a regression guard. With `timestamp: "NEW"` in the payload: pre-fix, Step 6 passes
(`proposed_ts != on_disk_ts`) then Step 7 fires `Block(LockExpiryStale)`; post-fix,
`sets_timestamp=true` → Steps 4–6 run and pass, Step 7 fires `Block(LockExpiryStale)`.
Block code is identical both pre- and post-fix → true regression guard.

### Second deliverable — AC-021: PostToolUse per-commit absent-timestamp advisory sentinel

Add a PostToolUse advisory WASM plugin for Bash git-commit events. Registry stanza (complete):

```toml
[[hooks]]
name = "verify-state-timestamp-advisory"
event = "PostToolUse"
tool = "^Bash$"
plugin = "hook-plugins/verify-state-timestamp-advisory.wasm"
priority = 159
timeout_ms = 5000
async = false
on_error = "continue"
```

Priority 159 = lowest free slot above 154 after `validate-dispatch-advance` (priority 154; ground
truth POLICY 15 evidence:
```
$ grep -n "^priority = " plugins/vsdd-factory/hooks-registry.toml | grep -E "15[0-9]"
939:priority = 150
958:priority = 152
980:priority = 152
1004:priority = 151
1030:priority = 153
1069:priority = 154
1091:priority = 154
1120:priority = 156
1139:priority = 157
1159:priority = 158
1288:priority = 155
1306:priority = 160
```
Slots 150–158 and 160 are occupied; 155 is occupied by `validate-stable-anchors`
(hooks-registry.toml line 1288); 159 is unoccupied — confirmed as the lowest free slot above 154).
No capabilities
block — the plugin reads only from `payload.extra["git_context"]`; no `read_file` or
`exec_subprocess` capability needed. The plugin detects per-COMMIT timestamp non-advancement —
the correct detection boundary for the ADR-025 §12.2 invariant "every STATE.md commit must
advance `timestamp:`". A per-Edit/Write trigger cannot detect the invariant at the commit
boundary; a per-commit trigger fires exactly once per commit and faithfully signals a
burst-level omission. **No `path_filter` registry field** — it does not exist in
`hooks-registry.toml` (ground truth: `grep -n 'path_filter' plugins/vsdd-factory/hooks-registry.toml`
→ 0 matches). WASM plugin is exec-free (ADR-029 §Decision 3 / BC-5.41.003 PC1
unconditionally preserved). STATE.md `timestamp:` values and the Pre-condition 2 flag (`state_md_in_commit`) are
read from `payload.extra["git_context"]` fields injected by the dispatcher host layer.
**Requires ADR-032-AC021-prereq implementation (see Prerequisite deliverable below).**

**Per-commit reference mechanism — ground-truth ABI verification (POLICY 15, option (i)):**

- ADR-029 §Decision 2 confirms `git_context` schema (`head_subject`, `head_sha`,
  `head_parent_subject`, `head_parent_sha`) injected into `payload.extra` on PostToolUse
  Bash git-commit events. Existing plugins `validate-burst-log` and `validate-dispatch-advance`
  (both `tool = "^Bash$"`, `event = "PostToolUse"`) already consume `git_context` this way.
  Registry ground truth (verbatim stdout):
  ```
  $ grep -n "name = \"validate-burst-log\"\|name = \"validate-dispatch-advance\"" plugins/vsdd-factory/hooks-registry.toml
  954:name = "validate-burst-log"
  976:name = "validate-burst-log"
  1065:name = "validate-dispatch-advance"
  1087:name = "validate-dispatch-advance"
  ```
  The `^Bash$`-triggered `validate-burst-log` entry and the `^Bash$`-triggered
  `validate-dispatch-advance` entry confirm the pattern (4 total entries: 2 per plugin
  across the `^(Edit|Write|MultiEdit)$` and `^Bash$` triggers).
- BC-5.41.003 PC1 (per registry comment on the `^Bash$`-triggered `validate-burst-log`
  entry: "WASM is exec-free: no exec_subprocess for commit-context acquisition") applies
  fully and unconditionally to AC-021. The WASM plugin is exec-free. STATE.md timestamp
  values are injected by the dispatcher host layer as two new `git_context` fields
  (ADR-032-AC021-prereq below). No `exec_subprocess` is used; PC1 is unconditionally
  preserved with no carve-out required.
- Factory-artifacts worktree path is resolved host-side: `build_git_context(factory_dir)`
  in `invoke.rs` already receives the resolved `factory_dir` path (F-P4-003 dissolved by
  Option A — the WASM plugin never executes `git -C` or references a working directory).

The plugin fires on every PostToolUse Bash event. It reads `git_context` from
`payload.extra.get("git_context")`; if absent or all fields empty → fail-open Continue
(not a git-commit event, or initial commit with no parent). When `git_context` is present
with non-empty fields, apply two pre-conditions before the timestamp comparison:

**Pre-condition 1 (factory-artifacts exclusivity):** Guaranteed by Pre-condition 2. Because
`.factory/` is a separate git worktree on the orphan `factory-artifacts` branch, no
develop-branch commit can include `.factory/STATE.md` in its changed files. Pre-condition 2
alone (`state_md_in_commit == "true"`) is a sufficient factory-artifacts discriminator; no
separate discriminator field is needed.

**Pre-condition 2 (STATE.md modified gate):** read
`payload.extra["git_context"]["state_md_in_commit"]` (bool injected by dispatcher prereq). If
the field is absent, empty, or `"false"` → return `Continue`. This prevents advisory noise on
factory-artifacts commits that do not touch STATE.md (e.g., a burst-log-only commit).

Only when both pre-conditions pass (Pre-condition 2: `state_md_in_commit` present and `"true"`), proceed:

1. Read `payload.extra["git_context"]["head_state_timestamp"]` → the `timestamp:` frontmatter
   value from `HEAD:STATE.md` as injected by the dispatcher (ADR-032-AC021-prereq).
2. Read `payload.extra["git_context"]["head_parent_state_timestamp"]` → the `timestamp:`
   frontmatter value from `HEAD^:STATE.md`.
3. If the two values are byte-identical → emit:
   `ADVISORY AC-021: factory-artifacts commit did not advance STATE.md timestamp: —
   verify state-manager burst discipline`. Result is always `Continue`
   (`on_error = continue`; PostToolUse hooks cannot prevent commits).

Fail-open cases (always return `Continue`):
- `git_context` absent in `payload.extra`, or all fields empty (non-git-commit Bash event
  or initial commit with no parent)
- `state_md_in_commit` absent or `"false"` (STATE.md not modified in this commit;
  STATE.md modified gate Pre-condition 2 not met; develop-branch commits are structurally
  excluded because `.factory/STATE.md` is not part of the develop working tree)
- `head_state_timestamp` absent or empty string (STATE.md not present in HEAD of
  factory-artifacts, or dispatcher prerequisite not yet implemented)
- `head_parent_state_timestamp` absent or empty string (no parent commit — initial commit —
  or STATE.md absent in HEAD^)
- Any field missing from the `git_context` object

No sidecar files. No cross-invocation state. No new shell scripts (POLICY 21). WASM plugin
under `crates/hook-plugins/`.

### Prerequisite deliverable — ADR-032-AC021-prereq: dispatcher git_context extension

Must be implemented before the AC-021 WASM plugin. The dispatcher must inject **three** additional
fields into the `git_context` object on qualifying PostToolUse Bash git-commit events: the two
timestamp fields from the original design, plus one new discriminator field required by
AC-021 Pre-condition 2. (Naming precedent: S-18.04b-prereq from ADR-029.)

**Dispatcher source touchpoints (cite by function anchor per TD-VSDD-091):**

1. `invoke.rs` — struct `GitContext`: add three new `pub` fields:
   `head_state_timestamp: String`, `head_parent_state_timestamp: String`,
   and `state_md_in_commit: String` (serialized bool `"true"` / `"false"` / `""`).
   String type is used for all fields for uniform JSON serialization consistency with
   existing `head_sha` / `head_parent_sha` empty-string sentinel convention.
2. `invoke.rs` — function `GitContext::empty()`: add `head_state_timestamp: String::new()`,
   `head_parent_state_timestamp: String::new()`, and `state_md_in_commit: String::new()` to
   the struct literal.
3. `invoke.rs` — function `build_git_context(factory_dir: &std::path::Path)`: after the four
   existing git commands (`head_subject`, `head_sha`, `head_parent_subject`,
   `head_parent_sha`), run:
   - `git show HEAD:STATE.md` against `factory_dir`; pass output to
     `factory_lock_parse::extract_yaml_string_value` for the `"timestamp"` key →
     `head_state_timestamp`. On any git error or absent field → `String::new()` (fail-open,
     consistent with the existing HEAD^ handling pattern).
   - `git show HEAD^:STATE.md` against `factory_dir`; extract `"timestamp"` →
     `head_parent_state_timestamp`. On error → `String::new()`.
   - `git -C factory_dir diff --name-only HEAD^ HEAD` to get the list of files changed in
     HEAD relative to HEAD^. If STATE.md appears in the output → `state_md_in_commit =
     "true"`; otherwise `"false"`. On git error (e.g., initial commit with no HEAD^) →
     `String::new()` (fail-open).
4. `invoke.rs` — function `inject_git_context_if_qualifying`: no change required. It calls
   `build_git_context(factory_dir)` and serializes via `git_ctx.to_json()`. The three new
   keys (`head_state_timestamp`, `head_parent_state_timestamp`, `state_md_in_commit`) MUST
   be explicitly added to `to_json()`'s `json!` object literal (see item 5 below —
   `to_json()` does NOT serialize new fields automatically).
5. `invoke.rs` — function `GitContext::to_json()`: extend the `serde_json::json!({...})`
   object literal to include the three new string fields: `head_state_timestamp`,
   `head_parent_state_timestamp`, and `state_md_in_commit`.
   The `json!` macro produces a hand-written literal; new fields do NOT serialize
   automatically — each must be explicitly named in the object. The existing four fields
   (`head_subject`, `head_sha`, `head_parent_subject`, `head_parent_sha`) remain unchanged.

**Factory-artifacts worktree path:** `factory_dir` is already the resolved factory-artifacts
path (passed to `build_git_context` from the `// S-18.04b-prereq: git_context injection site`
call in `main.rs`). No new path-resolution logic is required in the dispatcher.

**Fail-open contract (consistent with existing `git_context` fields):** git errors or absent
`timestamp:` field → `String::new()` (not null; parity with AC-006 empty-string sentinel for
`head_parent_subject` / `head_parent_sha` on initial commit). `state_md_in_commit` also uses
`String::new()` for all error/absent cases, which the WASM plugin treats as fail-open Continue.

AC-021 tests (minimum 3):
- `ac021_commit_stale_timestamp_emits_advisory` — `git_context` present with
  `state_md_in_commit = "true"` and both `head_state_timestamp` and
  `head_parent_state_timestamp` set to the same value; expected: advisory emitted,
  result `Continue`.
- `ac021_state_md_not_in_commit_no_advisory` — `git_context` present with
  `state_md_in_commit = "false"`; expected: no advisory, result `Continue`.
- `ac021_git_context_absent_no_advisory` — `git_context` absent from `payload.extra`
  (non-git-commit Bash event); expected: no advisory, result `Continue`.

**Dispatcher-side tests (minimum 3; updated by F-ADR032-P7-001 + F-ADR032-P7-006):**
1. In `crates/factory-dispatcher/src/invoke.rs`: add a test asserting
   `build_git_context(factory_dir).to_json()` contains all three new keys
   (`head_state_timestamp`, `head_parent_state_timestamp`, `state_md_in_commit`) with
   correct values. Construct against a temporary git repo initialized on a `factory-artifacts`
   branch that commits a `STATE.md` file containing `timestamp: "2026-07-20T10:00:00Z"` — so
   that `head_state_timestamp` expected value is `"2026-07-20T10:00:00Z"` and is checkable.
2. In `crates/factory-dispatcher/src/invoke.rs`: add a coverage test asserting that
   `inject_git_context_if_qualifying` with a qualifying bash commit event (PostToolUse +
   Bash + command containing `"git commit"` + `".factory"`) where the committed file set
   includes `STATE.md` produces a payload where `git_context["state_md_in_commit"] == "true"`.
3. In `crates/factory-dispatcher/src/invoke.rs`: add a negative-branch test asserting that
   when the committed file set does NOT include `STATE.md` (e.g., a burst-log-only commit),
   `build_git_context(factory_dir).to_json()` produces `state_md_in_commit == "false"`.

### Third deliverable — Frontmatter placement correction (F-P2-001 branch b)

**This deliverable is REQUIRED.** Without it, the lock-held burst path reintroduces the
large-payload failure mode this ADR was designed to eliminate (see §Consequences Negative).

**Normative mandate:** the `factory_lock:` block MUST be placed immediately after the
`timestamp:` line in STATE.md frontmatter, above `last_amended:`. The current behavior —
inserting at the end of the frontmatter (before the closing `---`, after `last_amended:`) —
violates this mandate.

**Ground truth (factory-lock-write.sh function `_write_factory_lock_block`):**
The awk script fires on `front == 2` (second `---` fence) and inserts the `factory_lock:`
block BEFORE printing the closing `---`. This places the block at the last position of the
frontmatter, after the multi-kilobyte `last_amended:` field.

**Files to change (cite by function name, not line number per TD-VSDD-091):**

1. `plugins/vsdd-factory/bin/factory-lock-write.sh` — function `_write_factory_lock_block`:
   Change the awk insertion logic from "fire on `front == 2` (closing fence)" to "fire when
   the current line matches `^timestamp:` inside the frontmatter region." The updated awk
   MUST include all three of the following guards, consistent with the sibling awk patterns
   used by `_remove_factory_lock` and `_update_expires_at`:

   - **Frontmatter-region gate:** Only trigger when `front == 1` (i.e., the opening `---`
     fence has been passed — `front` incremented to 1 — but the closing `---` fence has not
     yet been reached). This matches the `fence == 1` guard pattern used by
     `_remove_factory_lock` and `_update_expires_at`. Lines at `front == 0` (before the
     opening fence) and `front == 2` (at or after the closing fence) MUST NOT trigger
     insertion. A `timestamp:` line appearing in the file body (outside the frontmatter) is
     therefore ignored.

   - **Once-only insertion guard:** Use an `inserted` variable (initialized to `0` in
     `BEGIN`) to ensure the block is inserted at most once. The trigger condition must be
     `front == 1 && /^timestamp:/ && !inserted`. After inserting the block, set `inserted = 1`.
     This mirrors the existing `inserted` variable already present in `_write_factory_lock_block`'s
     `front == 2` awk block.

   - **Insertion order:** When the trigger fires, print the `timestamp:` line first (pass
     it through unchanged via `print $0`), then immediately print the `factory_lock:` block
     lines, then set `inserted = 1`, then call `next` to skip the default `{ print }` rule
     for the current line (which was already printed above).

   The existing `_remove_factory_lock` call (step 1 of the function) still runs first to
   clear any existing `factory_lock:` block before reinsertion.

2. `crates/factory-lock/src/lib.rs` — `acquire_lock` function (currently `todo!()` stub
   scoped to S-18.04b): the stub need not duplicate the placement mandate inline. The
   implementing story MUST note in its implementation checklist that `factory_lock:` insertion
   placement must comply with ADR-032 §Implementer Work Spec Third Deliverable (placement
   immediately after the `timestamp:` line, above `last_amended:`). The mandate is durably
   captured in this ADR; the Rust stub does not need an inline copy (a copy would be a drifted
   duplicate of normative spec text — consistent with v1.4 removal rationale per F-ADR032-P3B-003).

**Post-condition:** after `factory-lock-write.sh acquire`, all three of the following MUST hold:

1. `grep -n "^timestamp:\|^factory_lock:" .factory/STATE.md | head -5` shows the
   `factory_lock:` key on the line immediately following `timestamp:`.
2. Exactly one `factory_lock:` key appears in the output file:
   `grep -c "^factory_lock:" .factory/STATE.md` = 1.
3. The `factory_lock:` key lies inside the frontmatter region (before the second `---` fence):
   `awk '/^---$/{f++; next} f==1 && /^factory_lock:/{found=1} END{exit !found}' .factory/STATE.md`
   exits 0.

---

## Changelog

| Version | Date | Author | Summary |
|---------|------|--------|---------|
| 1.10 | 2026-07-20 | orchestrator-verified | P6/P7 provenance relabel burst (D-868): F-ADR032-P6-001..P6-008 and F-ADR032-P7-001..P7-007 attribution corrected — both sets were UNAUTHORIZED self-review bursts by a shadow subagent chain (no adversary pass 6 or pass 7 ran; prefixes retained for append-only ID stability; content pending adversarial verification). Freeze commit 87745b8e (blob sha256 6ee6091fe9b7e220ec61137f884e7ffe91f806865d2a25609e40432be5d7f670) human-authorized. No normative content changed. |
| 1.9 | 2026-07-20 | shadow-chain-agent (UNAUTHORIZED; no adversary pass 7 ran; see D-868) | NOTE: UNAUTHORIZED shadow-chain burst (no adversary pass 7 ran; prefixes retained for append-only ID stability; content pending adversarial verification). F-ADR032-P7-001 through P7-007 — 7-finding closure. F-ADR032-P7-001 (HIGH): dropped factory-artifacts branch discriminator field (tautology — worktree separation guarantees no develop-branch commit can include STATE.md; Pre-condition 2 `state_md_in_commit` alone is sufficient); removed from GitContext struct, `empty()`, `build_git_context` (deleted `git symbolic-ref HEAD` step), `to_json()` (four new keys → three), Pre-condition 1 rewritten to structural guarantee, fail-open table bullet removed, AC-021 tests updated (three keys; test 2 renamed to git_context_absent fail-open), dispatcher-side tests updated (three keys; Test 1 specifies known timestamp; Test 3 negative branch added). F-ADR032-P7-002 (HIGH): priority 155 → 159 (155 is occupied by `validate-stable-anchors` at hooks-registry.toml line 1288; grep stdout included in POLICY 15 evidence). F-ADR032-P7-003 (MEDIUM): §Source/Origin "return `Continue` immediately per ADR-032 Decision 1+3" corrected to "return `Continue` after skipping the timestamp and lock-expiry checks (Decision 1 and Decision 3)". F-ADR032-P7-004 (MEDIUM): step-numbering scheme annotations added — Decisions 1 and 3 append "(code-inline numbering)" after "Step 8"; Decision 5 annotation satisfied by F-P7-005 `module //!` prefix. F-ADR032-P7-005 (MEDIUM): Decision 5 AC-020 entry updated to include `module //!` qualifier, now byte-identical with Work Spec Part 2. F-ADR032-P7-006 (MEDIUM): dispatcher-side test mandate updated — Test 1 specifies STATE.md with known `timestamp: "2026-07-20T10:00:00Z"` so `head_state_timestamp` is checkable; Test 2 asserts `state_md_in_commit == "true"` (removed discriminator field reference); Test 3 (new, negative branch): committed file set without STATE.md produces `state_md_in_commit == "false"`. F-ADR032-P7-007 (LOW): `async = false` rationale corrected — Option B chosen: keep field; correct rationale from "conventionally REQUIRED per ADR-019 + ADR-025 Decision 12" to "included for explicitness; semantically safe (satisfies `on_error=block ⇒ async=false` invariant despite `on_error=continue`); matches PreToolUse convention." Changelog v1.8 F-P6-005 entry updated accordingly. ARCH-INDEX v3.19→v3.20. |
| 1.8 | 2026-07-20 | shadow-chain-agent (UNAUTHORIZED; no adversary pass 6 ran; see D-868) | NOTE: UNAUTHORIZED shadow-chain burst (no adversary pass 6 ran; prefixes retained for append-only ID stability; content pending adversarial verification). F-ADR032-P6-001 through P6-008 — 8-finding closure. F-ADR032-P6-001 (CRITICAL): stateless AC-021 discriminator redesign — deleted `last_known_factory_artifacts_head` `Mutex<String>` cache and prereq item 5; `is_factory_artifacts_commit` computed stateless via `git symbolic-ref HEAD` comparison in `build_git_context`; Pre-condition 1 updated to remove cross-invocation state language. F-ADR032-P6-002: dissolved by P6-001 — no residual cross-invocation state language remains. F-ADR032-P6-003: item 4 "no change required" confirmed accurate — `inject_git_context_if_qualifying` signature unchanged (3 parameters, no Arc/Mutex). F-ADR032-P6-004: prereq item 5 added for explicit `to_json()` extension mandate; "automatically" removed from item 4. F-ADR032-P6-005: `async = false` restored to AC-021 registry stanza; prior rationale "not a registry field" was incorrect — included for explicitness; semantically safe (satisfies `on_error=block ⇒ async=false` invariant despite `on_error=continue`); matches PreToolUse convention. F-ADR032-P6-006: AC-020 `# BC traces` entry corrected from "Steps 4–7 skipped" to "Steps 4–8 skipped (falls through to Step 9 → Continue)" (module `//!` numbering; Step 8 = lock-expiry Block; Step 9 = Continue). F-ADR032-P6-007: dispatcher-side test mandate added to prereq (minimum 2 tests for false-green prevention). F-ADR032-P6-008: Decision 1 control-flow phrasing unified — "returns Continue immediately; Steps 4–7 are skipped (continue at Step 8 → Continue)" replaced with "skips Steps 4–7 entirely, then returns Continue at Step 8". ARCH-INDEX v3.18→v3.19. |
| 1.7 | 2026-07-20 | architect | Provenance correction (orchestrator-verified via dispatcher-log session audit). v1.3 attribution corrected: F-ADR032-P3-001..007 was architect self-review during fix burst 3, NOT adversary findings; adversary pass-3 finding set is the P3B set (F-ADR032-P3B-001..003), closed at v1.4. v1.6 attribution corrected: F-ADR032-P5-001..008 was architect self-review during fix burst 4; no adversary pass 5 ran; P5 prefix retained for append-only ID stability. No normative content changed. ARCH-INDEX v3.17→v3.18. |
| 1.6 | 2026-07-20 | architect | architect self-review during fix burst 4. 8-finding closure. Note: F-ADR032-P5-001..008 set was architect self-review, NOT from adversary pass 5; no adversary pass 5 ran; P5 prefix retained for append-only ID stability. F-ADR032-P5-001+P5-006: guard_logic function docstring update section split into two labeled sub-instructions (module `//!` docstring Part 1 / guard_logic function `# BC traces` Part 2); module step numbering clarification note added to Part 1; AC-020 # BC traces entry clarified to use module `//!` step numbering. F-ADR032-P5-002: Third Deliverable item 2 MUST-carry-comment mandate removed; replaced with implementation checklist note (mandate lives in ADR spec; stub need not duplicate). F-ADR032-P5-003: test #11 Red Gate column corrected to YES (pre-fix Block(TimestampStale)); four Red Gate tests enumerated; note on test #11 pre-fix vs post-fix behavior added; §Status updated three → four Red Gate tests. F-ADR032-P5-004: AC-021 two pre-conditions added (factory-artifacts discriminator via `is_factory_artifacts_commit`; STATE.md modified gate via `state_md_in_commit`); both fields added to prereq deliverable GitContext struct + `empty()` + `build_git_context`; two new fail-open rows added; §Consequences Negative bullet (c) updated for gated mechanism; `async = false` removed (not a registry field); explicit registry stanza with name/event/tool/plugin/priority/timeout_ms/on_error added. F-ADR032-P5-005: Third Deliverable item 1 awk mandate strengthened — frontmatter-region gating (`front == 1`) required; `inserted` once-only guard required; insertion order specified; post-condition strengthened with exactly-one `factory_lock:` assertion and frontmatter-region assertion. F-ADR032-P5-006: resolved via P5-001 fix (subsection split). F-ADR032-P5-007: Decision 1 vacuous parenthetical removed. F-ADR032-P5-008: AC-021 registry stanza made explicit with all required fields; `async = false` removed. |
| 1.5 | 2026-07-20 | architect | F-ADR032-P4 fix burst. 6-finding closure. F-ADR032-P4-001: ARCH-INDEX ADR-032 row refreshed — Decision 1 two-field payload-neutral condition; Decision 5 guard_logic function docstring; AC-021 exec-free mechanism (ADR-032-AC021-prereq); ADR-025 amended to v1.22 (ARCH-INDEX v3.15→v3.16). F-ADR032-P4-002: Option A architectural ruling — AC-021 redesigned exec-free: dispatcher injects `head_state_timestamp` + `head_parent_state_timestamp` into `git_context` via ADR-032-AC021-prereq; WASM reads `payload.extra["git_context"]` fields; `exec_subprocess` capability removed; BC-5.41.003 PC1 unconditionally preserved (no carve-out); ADR-032-AC021-prereq Work Spec section added (touchpoints: `GitContext` struct, `GitContext::empty()`, `build_git_context`, `inject_git_context_if_qualifying` in `invoke.rs`). F-ADR032-P4-003: dissolved by Option A — `factory_dir` resolves worktree path host-side; WASM exec-free; noted explicitly in Work Spec evidence block. F-ADR032-P4-004: evidence block corrected — verbatim 4-match grep stdout (entries 954, 976, 1065, 1087); bare line-number pins removed; references by entry name + tool regex. F-ADR032-P4-005: AC-020 label renamed `timestamp-neutrality` → `payload-neutrality` at Decision 5 body and Work Spec docstring (changelog historical mentions retained). F-ADR032-P4-006: v1.4 Changelog row + last_amended findings renamed F-ADR032-P3B-001/002/003 (v1.3 set retains F-ADR032-P3-001/002/003 per append-only principle); volatile line pin 'registry lines 976/1087' corrected to entry-name + tool-regex anchor in v1.4 last_amended text; clarifying sentence added to v1.4 Changelog row. |
| 1.4 | 2026-07-20 | architect | F-ADR032-P3 fix burst (additional findings). F-ADR032-P3B-001: AC-021 re-scoped from per-write ^(Edit\|Write\|MultiEdit)$ trigger to per-commit PostToolUse ^Bash$ trigger (option (i) chosen — feasible per ABI ground truth: ADR-029 git_context injection confirmed, ^Bash$-triggered validate-burst-log and validate-dispatch-advance registry entries confirm pattern, BC-5.41.003 PC1 restricts exec_subprocess for commit-context acquisition only not file-content acquisition); mechanism changed from post-write on-disk read_file comparison to HEAD vs HEAD^ git-show timestamp comparison; read_file capability removed; §Consequences Negative bullet (c) updated; POLICY 15 evidence block added to Work Spec. F-ADR032-P3B-002: Decision 1 normative text corrected — "If NO new_string sets timestamp:" replaced with "If NO new_string sets EITHER timestamp: OR factory_lock: (payload-neutral — neither field is set)"; "Steps 4–8 are skipped" corrected to "Steps 4–7 are skipped"; Decision 5 AC-020 docstring text updated to match; §Source/Origin "per ADR-032 Decision 1" updated to "per ADR-032 Decision 1+3" with explicit "neither timestamp: nor factory_lock:" definition; ADR-025 v1.22 cross-referenced. F-ADR032-P3B-003: removed scope-deviation doc comment (/// PLACEMENT MANDATE) from crates/factory-lock/src/lib.rs acquire_lock — mandate is durably captured in ADR-032 Third Deliverable; Rust copy was a drifted duplicate of normative spec text; post-removal git status --porcelain crates/ shows empty delta. ADR-025 amended to v1.22 in lock-step (§12.3 first-sentence correction per F-ADR032-P3B-002). Closes F-ADR032-P3B-001..F-ADR032-P3B-003. ADR-032 v1.4. ARCH-INDEX v3.14→v3.15. Note: v1.4 supersedes v1.3's AC-021 broadening — v1.3 F-ADR032-P3-001 broadened trigger to ^(Edit\|Write\|MultiEdit)$; v1.4 F-ADR032-P3B-001 reverted and re-scoped to per-commit ^Bash$ trigger. |
| 1.3 | 2026-07-20 | architect | F-ADR032-P3 fix burst. 7-finding closure (architect self-review during fix burst 3; F-ADR032-P3-001..007 set was NOT produced by any adversary pass; the adversary pass-3 finding set is the P3B set — F-ADR032-P3B-001..003 — closed at v1.4). F-ADR032-P3-001: AC-021 broadened from Write-only to Edit/Write/MultiEdit (`^(Edit\|Write\|MultiEdit)$`); read_file capability added; post-write on-disk read via host::read_file uniform across all three tool types; advisory gates on last-committed HEAD comparison (not per-Edit unconditionally); §Consequences Negative bullet (c) updated to reflect broadened scope. F-ADR032-P3-002: two volatile line pins removed — `(line 7)` from §Context timestamp: reference; `lines ~207–223` from Third Deliverable ground-truth heading; function-anchor citations retained. F-ADR032-P3-003: §Rationale "symmetric" heading corrected — replaced with "Why the same payload-scan technique also eliminates spurious LockExpiryStale blocks (Decision 3)". F-ADR032-P3-004: Decision 5 AC-020 insertion target corrected — was false "module docstring AC matrix, alongside AC-005..AC-019"; corrected to `guard_logic` function docstring `# BC traces` list after AC-019 (ground truth: grep shows AC-011..AC-019 in # BC traces, no contiguous AC-005..AC-019 matrix). F-ADR032-P3-005: §Consequences Negative "pre-burst reference" replaced with "last committed STATE.md (`git show HEAD:.factory/STATE.md`)". F-ADR032-P3-006: volatile line pins removed from frontmatter last_amended and Changelog v1.2 row — "volatile line pins (line 864/867 references)" replaced with "volatile line pins removed from Work Spec insertion-point prose (function-anchor citations retained)". F-ADR032-P3-007: Required new tests heading corrected from "minimum 9" to "minimum 10" (table has 10 rows). |
| 1.2 | 2026-07-20 | architect | F-ADR032-P2 fix burst. 7-finding closure per adversary pass 2. F-ADR032-P1-009: volatile line pins removed from Work Spec insertion-point prose (function-anchor citations retained); stable function anchors only (TD-VSDD-091/POLICY 19). F-ADR032-P2-001: factory_lock insertion anchor ground-truthed (_write_factory_lock_block: awk fires on front==2, inserts before closing ---, i.e. after last_amended:); branch (b) — normative placement mandate added; Third Deliverable scoped into Work Spec. F-ADR032-P2-002: §Source/Origin ADR-025 §12.3 scope corrected (TimestampStale gated on sets_timestamp; LockExpiryStale gated on sets_timestamp \|\| sets_factory_lock); ADR-025 v1.21 cross-referenced. F-ADR032-P2-003: AC-021 made implementable (path_filter removed; old_content absent from payload; git show HEAD:STATE.md mechanism). F-ADR032-P2-004: test #9 fixture mandated to include timestamp: "NEW" (true regression guard). F-ADR032-P2-005: ARCH-INDEX v3.12→v3.13; 4-index cite corrected. F-ADR032-P2-006: §Status heading version-neutralized. |
| 1.1 | 2026-07-20 | architect | F-ADR032-P1 fix burst. 13-finding closure per adversary pass 1. Decisions corrected: Decision 3 option (a) — full Steps 4–7 when `sets_timestamp=true`; §Source/Origin PC4/PC6 attribution corrected; `new_string_sets_field` confirmed NEW (0 crates/ matches); §12.2/§12.3 superseded-in-part framing added; test count corrected to 50; test #8 pre-fix label corrected to TimestampStale; Decision 3 regression guard test added (10 tests total); Work Spec insertion point disambiguated; step numbering conflict documented; AC-021 PostToolUse advisory scoped into Work Spec; D-866 candidate directions mapped to Option B/C; ADR-025 amendment v1.20 cross-referenced. |
| 1.0 | 2026-07-20 | architect | Initial ruling. D-866 root-cause record. Payload-targeted PreToolUse enforcement for Edit/MultiEdit (Decisions 1–5). `new_string_sets_field` helper, AC-020 docstring AC, 9 Red Gate/regression tests specified. |
