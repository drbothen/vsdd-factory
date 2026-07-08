---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-08T00:00:00Z
phase: 12
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-brownfield-backfill
cascade: E-19-story
pass: 12
previous_review: adv-E19-pass-11.md
perimeter: E-19 epic + S-19.01..S-19.07 + STORY-INDEX
verdict: NOT-CLEAN
blocker_count: 0
high_count: 1
medium_count: 3
low_count: 2
observation_count: 5
streak: 0/3
parent_decision: D-763
---

# Adversarial Review — E-19 Pass 12 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law; rubric = policies.yaml read directly; 20 policies)
**Date:** 2026-07-08
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 1 / MEDIUM 3 / LOW 2 (6 findings + 5 observations; counts matched enumeration; all findings artifact-grounded; live-vs-history adjudication held — zero noise findings)
**Streak:** 0/3

---

## Finding ID Convention

This review uses the in-cycle finding ID format established for the `v1.0-brownfield-backfill` cascade:

`F-P<PASS>-<SEQ>` — e.g., `F-P12-001`, `F-P12-002`, etc. Severity is stated inline in the finding header. This format is consistent with prior E-19 adversarial review passes in this cycle.

---

## Part A — Fix Verification (pass >= 2 only)

Pass-11 NOT-CLEAN B0/H1/M4/L1 (6 findings + 9 observations; 0 false-positives; orchestrator D-757 sequencing deviation self-attributed + quad-race repair; closed D-762). Fresh-context adversary reads only prior Part A — findings F-P11-001..F-P11-006. All 6 findings verified CLOSED by artifact evidence at pass-12 perimeter entry:

- **F-P11-001 CLOSED** (S-19.01 v1.9 AC-001 EC-001 stderr literal corrected — story-invented "cannot pin covered HEAD SHA" replaced with normative BC-5.42.001 EC-001 literal "gh pr view failed for PR #<pr_number>" at all 5 AC body sites; `grep -c "cannot pin covered HEAD SHA" .factory/stories/S-19.01-pr-manager-hardening.md` → 0; `grep -c "gh pr view failed" .factory/stories/S-19.01-pr-manager-hardening.md` → 5; D-762 SW leg.)
- **F-P11-002 CLOSED** (TD-VSDD-060 full sweep at S-19.01 AC-001 EC-001 locus complete — both locus name (check-stale-verdict.sh) and message literal corrected in same pass-9/pass-11 combined sweep; v1.9 changelog records "TD-VSDD-060 full sweep — locus name and message literal both verified at 5 AC body sites"; D-762 SW leg.)
- **F-P11-003 CLOSED** (S-19.05 v1.10 AC-001 entry_index pre-filter removed; ASYNC_SINK_EMPTY non-empty guard added; jq slurp assertion evaluates .entry_index != null directly; `grep -c "entry_index.*grep\|grep.*entry_index" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md` → 0; `grep -c "ASYNC_SINK_EMPTY" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md` → 1; D-762 SW leg.)
- **F-P11-004 CLOSED** (S-19.05 v1.10 AC-002 ABANDONED_SET_EMPTY non-empty guard added; positive-control assertion requiring ≥1 plugin.abandoned record with all BC-3.08.001 Event 5 fields; `grep -c "ABANDONED_SET_EMPTY" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md` → 1; D-762 SW leg.)
- **F-P11-005 CLOSED** (ADR-025 v1.10 D18 test bullet (e) reworded — fixture body padded past 8192 bytes (approaching 262144-byte Phase-A cap) to test correct frontmatter parsing from 8192-byte prefix; 262144 value explicitly labels FIXTURE SIZE, not the read bound; max_bytes=8192 is the read bound per BC-4.13.001 v1.8 Phase-B; `grep -n "8192\|max_bytes" .factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md | grep "D18"` confirms disambiguation; D-762 architect leg.)
- **F-P11-006 CLOSED** (S-19.06 v1.7 deferral gate updated to bolded hard-gate language matching S-19.03: "MUST NOT be merged until S-19.04 merges"; `grep -c "MUST NOT.*S-19\.04" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md` → 1; `grep -c "should not.*deploy\|should not.*S-19\.04" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md` → 0; D-762 SW leg.)

New findings from pass-12 review below in Part B.

---

## Part B — New Findings

*Verbatim adversary output per D-448(a) source-attestation. Do not summarize or soften. Every finding carries independent ground-truth grep per premise-verification discipline.*

F-P12-001 — HIGH — S-19.04 keep-assertion is structurally unsatisfiable as written: the gate asserts `test -f plugins/vsdd-factory/hook-plugins/vsdd-context-resolvers.wasm` exists in the release `artifact/` staging area, but the hyphen-named resolver is a git-tracked source artifact in `hook-plugins/`, not a Cargo-produced binary. Ground-truth verification: (1) `grep -n "hook-plugins/vsdd-context-resolvers.wasm\|artifact/.*vsdd-context-resolvers\|keep.*vsdd-context-resolvers" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | head -10` — S-19.04 v1.11 AC-001 keep-leg asserts presence of `plugins/vsdd-factory/hook-plugins/vsdd-context-resolvers.wasm` (gate (i): `test -f plugins/vsdd-factory/hook-plugins/vsdd-context-resolvers.wasm`). (2) `grep -n "vsdd-context-resolvers\|vsdd_context_resolvers\|# vsdd\|copy.*vsdd" .github/workflows/release.yml | head -15` — release.yml itself contains an inline note explaining that `vsdd-context-resolvers.wasm` (hyphen form) is the git-tracked hook-plugins artifact, not a Cargo-compiled product; the release pipeline does not produce a hyphen-named `.wasm` from any Cargo workspace crate (all cargo-built WASMs use underscore names per Cargo conventions). (3) The AC-001 keep-leg gate fires against the `hook-plugins/` directory path, not against `artifact/` staging. The assertion `test -f plugins/vsdd-factory/hook-plugins/vsdd-context-resolvers.wasm` tests for the pre-existing git-tracked file — it would pass unconditionally on any clean checkout of the repository regardless of whether the release script ran, because the file is committed to the repo. An AC that passes on a clean checkout before any release-script execution cannot distinguish a correctly-running release pipeline from a no-op. The gate is structurally unsatisfiable as a release-verification step: it cannot detect the absence of the artifact from the staged bundle because it probes the source path, not the bundle path. Fix: story-writer S-19.04 v1.11 — AC-001 keep-leg restructured: gate (i) confirms the git-tracked source exists at `plugins/vsdd-factory/hook-plugins/vsdd-context-resolvers.wasm`; gate (ii) independently confirms `resolvers-registry.toml` references this exact path (`grep -q "hook-plugins/vsdd-context-resolvers.wasm" plugins/vsdd-factory/resolvers-registry.toml`); add canonical skip-arm form to Architecture Mapping noting that `vsdd_context_resolvers.wasm|wasm_resolver_export.wasm` are passed through via the skip case-arm in release.yml (they are source-committed, not cargo-built), so the correct AC is registry-presence verification, not artifact/-staging verification.

F-P12-002 — MEDIUM — S-19.06 AC-007 and BC-1.17.001 conflate the safe-wrapper layer and the extern/"wire ABI" layer in a single gate assertion, preventing independently verifiable specification of the two-layer design. Ground-truth verification: (1) `grep -n "AC-007\|fn read_prefix\|extern\|host\.rs\|ffi\.rs\|->.*i32\|Result.*Vec" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -15` — S-19.06 v1.7 AC-007 at pass-12 perimeter entry is a single grep gate that searches for `fn read_prefix(` in `crates/hook-sdk/src/host.rs`. This gate verifies only the safe-wrapper signature presence but says nothing about the extern FFI entry point in `ffi.rs`. (2) `grep -n "read_prefix\|extern\|ffi\|->.*i32\|Result.*Vec\|safe.*wrapper\|wire.*ABI\|ABI.*wire" .factory/specs/behavioral-contracts/ss-01/BC-1.17.001.md | head -15` — BC-1.17.001 v1.1 does not distinguish the safe-wrapper layer (Result<Vec<u8>, HostError> return, &str path parameter, Rust-safe interface) from the extern FFI layer (ptr/len decomposed path, ptr/len output buffer, -> i32 return encoding CAPABILITY_DENIED/OK/NOT_FOUND/OUTPUT_TOO_LARGE). A conforming implementation must provide BOTH layers: the safe wrapper in `hook-sdk/src/host.rs` mirrors the existing `read_file` safe wrapper, and the extern entry point in `hook-sdk/src/ffi.rs` mirrors the existing `read_file` extern. (3) The current single-site AC-007 gate would pass if only the safe wrapper existed without the extern entry point — which would produce a compilation failure the moment any WASM plugin calls `read_prefix` at the FFI boundary. An extern-only implementation (no safe wrapper) would fail the AC-007 gate but still compile; S-19.07 consumes the safe-wrapper form (`host::read_prefix(path: &str) -> Result<Vec<u8>, HostError>`), so an extern-only implementation would leave S-19.07 without a callable API. Fix: architect text ruling (Ruling 1) — mirror read_file at BOTH layers: (a) safe wrapper `pub fn read_prefix(path: &str, max_bytes: u32) -> Result<Vec<u8>, HostError>` in `hook-sdk/src/host.rs`; (b) 6-param ptr/len extern `fn read_prefix(path_ptr, path_len, max_bytes, out_ptr, out_len_ptr, out_filled_ptr) -> i32` in `hook-sdk/src/ffi.rs`; (c) non-wasm cfg stub in `hook-sdk/src/host.rs` (mirrors existing read_file non-wasm stub). BC-1.17.001 amendment adding the layering parenthetical is recommended-not-required (EC-007 is already correct; the layering note is clarifying prose). Story-writer S-19.06 v1.7 — AC-007 restructured as three independent gates: Gate 1 (safe-wrapper layer: `grep -qE 'pub fn read_prefix\(path: &str'` in `hook-sdk/src/host.rs`); Gate 2 (extern/wire-ABI layer: `grep -qE 'fn read_prefix\(path_ptr' in hook-sdk/src/ffi.rs`); Gate 3 (dispatcher dispatch-table cite confirming the extern entry point is registered). Each gate independently verifiable.

F-P12-003 — MEDIUM — S-19.05 v1.10 T-006 literal gate will break under idiomatic Rust import consolidation. Ground-truth verification: (1) `grep -n "T-006\|use std::sync::Mutex\|sync.*Mutex\|Mutex.*sync" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -10` — S-19.05 v1.10 encodes T-006 as a grep gate: `grep -nE "^use std::sync::Mutex;" crates/factory-dispatcher/src/main.rs` must exit 0 (import present unconditionally). (2) The gate's regex `^use std::sync::Mutex;` matches ONLY the standalone import form. Idiomatic Rust allows consolidating this import with other `std::sync` items as `use std::sync::{Arc, Mutex};` or `use std::sync::{Condvar, Mutex, RwLock};`. A refactor that consolidates the Mutex import with any sibling `std::sync` item would silently break T-006 — the gate would fail, emitting a false-positive failure against a correct implementation. The test ID T-006 is a normative test gate in the acceptance criteria, not a style preference, so this is a specification defect rather than an implementation risk. Fix: story-writer S-19.05 v1.10→v1.11 — T-006 gate broadened to tolerate idiomatic import consolidation: `grep -qE '^use std::sync::(Mutex|\{[^}]*Mutex[^}]*\});'`; additional negative clause: the matching line must NOT be cfg-gated (`grep -v '#\[cfg'` pre-filter before the positive assertion); this matches both standalone form `use std::sync::Mutex;` and consolidated form `use std::sync::{..., Mutex, ...};` while excluding cfg-gated debug-only imports.

F-P12-004 — MEDIUM — The path_resolution_failed emission mechanism for the two-step decomposed prepare() pattern is unspecified in the story artifacts that will implement it. Ground-truth verification: (1) `grep -n "path_resolution_failed\|emit.*capability_denied\|internal\.capability_denied\|reason.*path_resolution_failed\|path_allowed.*bool\|prepare.*fn\|two.*step\|decomposed" .factory/stories/S-19.03-warn-pending-wave-gate-file-not-found.md | head -20` — S-19.03 v1.9 (pre-pass-12) AC-001 gate documents the negative-control B test: inject mock canonicalize fn returning Err → assert `path_allowed()==false + reason=path_resolution_failed`. However, the current story Architecture Mapping specifies `path_allowed() -> bool` as the signature — a boolean return that collapses both failure modes (path_resolution_failed and path_not_allowed) into a single `false` return with no mechanism for the caller to distinguish which reason token should be emitted. The `bool` return type cannot carry reason information; the caller of `path_allowed()` must infer the reason from external state, which is not specified. (2) `grep -n "write_file\|sibling.*sweep\|write_file\.rs" .factory/stories/S-19.03-warn-pending-wave-gate-file-not-found.md | head -5` — no write_file.rs sibling-sweep clause present at v1.9. The `write_file.rs` module contains the canonical `path_allowed()` definition; if S-19.03 extracts `resolve_path_for_allowlist` into `path_util.rs` and the two-step decomposed prepare() pattern, `write_file.rs` must also adopt the same pattern (TD-VSDD-060 mandatory sibling sweep). Fix: (a) architect text ruling (Ruling 2) — two-step decomposed prepare() in BOTH `read_file.rs` AND `write_file.rs`: Step 1 calls `resolve_path_for_allowlist` (returns Option<PathBuf>); None → emit `path_resolution_failed`; Step 2 pure prefix check → false → emit `path_not_allowed`; no BC amendment required (EC-007 already declares the behavior); (b) story-writer S-19.03 v1.9 — Architecture Mapping updated with two-step decomposed prepare() pattern; write_file.rs sibling-sweep clause added (mandatory TD-VSDD-060); File Structure write_file.rs row gains decomposition note; unit test added asserting mock canonicalize returning Err produces reason=path_resolution_failed (not path_not_allowed).

F-P12-005 — LOW — S-19.03 AC-006 zero-count assertion conflicts with the new legitimate path_resolution_failed emission introduced by F-P12-004's two-step decomposed prepare(). Ground-truth verification: `grep -n "AC-006\|zero.*capability_denied\|capability_denied.*zero\|path_resolution_failed\|reason.*count" .factory/stories/S-19.03-warn-pending-wave-gate-file-not-found.md | head -10` — S-19.03 AC-006 currently asserts that the warn-pending-wave-gate plugin emits zero `capability_denied` events on a fresh install with absent wave-state.yaml. After the Ruling-2 two-step decomposed prepare() pattern is implemented, a filesystem error during path resolution (e.g., permission denied on a symlink traversal) WILL legitimately emit `capability_denied reason=path_resolution_failed` — these are NOT bugs, they are correct behavior per EC-007. A blanket zero-count assertion on all capability_denied events would fail the AC against a correct implementation that correctly emits path_resolution_failed on genuine resolution errors. Fix: story-writer S-19.03 v1.9 — AC-006 gate scoped to `reason=path_not_allowed` zero-count only (the false-positive case this story addresses); `path_resolution_failed` events are explicitly excluded from the zero-count assertion per EC-007 (they are legitimate signals of a filesystem error, not a policy violation masquerading as a resolution error).

F-P12-006 — LOW — Epic E-19 v1.9 EAC-003 still uses the "no existing ancestor" framing for the negative-control B test of warn-pending-wave-gate, which was retired in favor of the injectable mock canonicalize approach per BC-2.07.001 v1.2 EC-007. Ground-truth verification: `grep -n "EAC-003\|no existing ancestor\|mock.*canonicalize\|injectable\|path_resolution_failed" .factory/stories/epics/E-19-post-rc22-operator-hardening.md | head -10` — EAC-003 at epic v1.9 reads: "warn-pending-wave-gate emits no false-positive capability_denied reason=path_not_allowed on fresh install with absent .factory/wave-state.yaml" with the test description referencing "path with NO existing ancestor" as the negative-control B fixture design. BC-2.07.001 v1.2 EC-007 (added pass-7 fix burst) retired the "no existing ancestor" framing in favor of injectable mock canonicalize because a real filesystem test with no existing ancestor is inherently non-portable and environment-dependent (macOS tmpfs, Linux overlayfs, container bind mounts all behave differently). The S-19.03 story body (AC-001 negative-control B) was correctly updated at pass-7 to the injectable-mock framing, but the epic's EAC-003 cell still references the retired framing. Fix: story-writer E-19 epic v1.9 — EAC-003 negative-control B updated to injectable mock canonicalize fn form per BC-2.07.001 v1.2 EC-007; "path with NO existing ancestor" framing removed; epic BC-1.17.001 cites swept to v1.2 (three body-scope cite sites: PRD Capabilities, Out-of-Scope, BC Traceability).

---

## HUMAN DIRECTIVE (recorded prominently per orchestrator request)

**Continuation policy = STRICT 3-CLEAN (BC-5.39.001), no pass cap, no asymptotic acceptance. Directive established 2026-07-07 (D-761) by human over three presented alternatives: (1) accept at floor, (2) 2 more passes then accept, (3) strict BC-5.39.001 no cap. Human chose Option C. This directive carries across CLEAR per §3 User Directives.**

---

## Verifications That PASSED

The following structural checks were confirmed clean at pass-12 perimeter entry:

1. BC-cite preflight PASS (per-file loop; D-760 canonical form): all 9 E-19 artifacts across all 6 E-19 BCs (BC-4.13.001 v1.8 / BC-2.07.001 v1.2 / BC-2.02.011 v1.4 / BC-3.08.001 v1.19 / BC-5.42.001 v1.1 / BC-1.17.001 v1.1); zero stale live cites confirmed at perimeter entry.
2. F-P11-001..F-P11-006 all CLOSED (verified above in Part A; 6/6 confirmed closed).
3. E-19 epic subsystems_affected PASS: `[SS-01, SS-02, SS-03, SS-04, SS-05, SS-07, SS-09]`; SS-06 absent (F-P9-001 fix held).
4. STORY-INDEX intro block PASS: `grep -oE "S-19\.[0-9]+ v[0-9]+\.[0-9]+" .factory/stories/STORY-INDEX.md | head -5` → zero hits in intro block (F-P10-003 fix held).
5. S-19.01 AC-001 normative EC-001 literal PASS: `grep -c "cannot pin covered HEAD SHA" .factory/stories/S-19.01-pr-manager-hardening.md` → 0; `grep -c "gh pr view failed" .factory/stories/S-19.01-pr-manager-hardening.md` → 5 (F-P11-001 fix held).
6. S-19.05 pre-filter removal PASS: `grep -c "entry_index.*grep\|grep.*entry_index" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md` → 0; ASYNC_SINK_EMPTY + ABANDONED_SET_EMPTY guards confirmed present (F-P11-003/004 fix held).
7. 4-index at perimeter entry PASS: BC v3.75 / VP v2.53 / STORY v4.143 / ARCH v2.90 consistent with D-762 state.

---

## Observations

O-P12-01 — [observation] S-19.04 v1.10 AC-001 keep-leg gate (i) (`test -f plugins/vsdd-factory/hook-plugins/vsdd-context-resolvers.wasm`) also constitutes a valid canonical skip-arm documentation anchor: the gate confirms the hyphen-named file is a committed source artifact (not cargo-built). The Architecture Mapping should add an explicit note on the skip-arm case-arm form used in release.yml for vsdd_context_resolvers.wasm|wasm_resolver_export.wasm so that future implementers understand why these two artifacts are in the skip case-arm rather than the exclusion set. (ACTIONABLE: story-writer S-19.04 v1.11 — Architecture Mapping gains canonical skip-arm notation (vsdd_context_resolvers.wasm|wasm_resolver_export.wasm echo skip stale resolver artifact: $name; continue ;;); load-bearing distinction from the orphan-exclusion set is explicit.)

O-P12-02 — [observation] S-19.05 v1.10 EC-005 describes a schema-level defense for concurrent entry_index traceability. BC-3.08.001 v1.19 establishes Invariant 6 (terminal key: trace_id+plugin_name+entry_index). EC-005 as currently written is a stand-alone SYNTHETIC edge case section with no reconciliation preface explaining the relationship between Invariant 6 (name-uniqueness at the registry level) and EC-005 (entry_index uniqueness at the event-emission level). A reader cannot readily deduce that registry name-uniqueness holds at runtime while EC-005 is a schema-level defense for wire format. (ACCEPTED-WITH-RECORD: one-line reconciliation preface to EC-005 body recommended on next story-writer touch; non-blocking this pass.)

O-P12-03 — [observation] S-19.02 v1.8 AC-005 contains a boundary-correctness assertion that does not include an affirmative statement that the extracted slice INCLUDES the opening `---\n` bytes (bytes 0..delimiter_start_offset). The gate verifies delimiter exclusion/inclusion per the `boundary_correctness` framing but does not confirm the slice start from byte 0 (the opening `---\n` marker) is included in the extracted prefix. A reader could infer that a correct implementation might skip the opening `---\n` bytes and only return the frontmatter content body. (ACTIONABLE: story-writer S-19.02 v1.8→v1.9 — AC-005 unit test A adds affirmative statement: extracted slice INCLUDES opening ---\\n bytes (bytes 0..delimiter_start_offset); parse_factory_lock handles document-start marker; non-blocking but clarifying for implementer correctness.)

O-P12-04 — [observation] S-19.06 v1.7 Task 12 and File Structure and Architecture Mapping contain rows referencing the Capability Schemas section update for `capabilities.read_prefix`. These rows are currently embedded within the existing Capability Schemas subsection that covers S-19.04's `tool-filter-anchoring` work — the read_prefix schema addition is a DISTINCT capability schema entry from the tool-filter work in S-19.04. A future reviewer could confuse the two capability schema additions as a single update. (ACTIONABLE: story-writer S-19.06 v1.7 — Task 12 + File Structure + Architecture Mapping gain an explicit preamble block noting that the capabilities.read_prefix schema addition is a DISTINCT Capability Schemas entry from S-19.04's tool-filter work; Previous Story Intel S-19.04 row updated to note the tool-filter preamble block is pre-existing and must not be overwritten; non-blocking but clarity-improving.)

O-P12-05 — [observation] S-19.05 v1.10 AC-002 counting grep uses `grep -c "plugin.abandoned" .factory/...` form, which is sensitive to JSON key ordering and multi-key JSONL lines that contain `plugin.abandoned` as a substring in a non-type field. A JSON-aware counting form would be more robust for a gate that is verifying dispatcher telemetry fidelity. (ACTIONABLE: story-writer S-19.05 v1.10→v1.11 — AC-002 counting grep swapped to JSON-aware form: `jq -c 'select(.type == "plugin.abandoned")' <sink_file> | grep -c .` (robust to key order and whitespace; non-empty guard semantics preserved); non-blocking but correctness-improving for gate fidelity.)

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 1 |
| MEDIUM | 3 |
| LOW | 2 |
| Observations | 5 |

*Actionable findings: 6 (F-P12-001..F-P12-006). Trajectory 16→14→20→9→8→5→12→11→4→7→6→6 (6 findings; same count as pass-11; HIGH held at 1; MEDIUM decreased from 4 to 3; LOW increased from 1 to 2). Five observations (O-P12-01/03/04 actionable; O-P12-02/05 accepted-with-record).*

**Overall Assessment:** block
**Convergence:** findings remain — iterate (strict 3-CLEAN per human directive D-761; no cap)
**Severity decay from pass 11 (enumerated):** B0/H1/M4/L1 (6 total) → B0/H1/M3/L2 (6 total; MEDIUM decreased by 1; LOW increased by 1; net trajectory flat at 6 — same finding count, severity shift toward LOW end)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 12 |
| **New findings** | 6 (F-P12-001..F-P12-006) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (6 / 6) |
| **Median severity** | MEDIUM |
| **Trajectory (findings per pass)** | 16 → 14 → 20 → 9 → 8 → 5 → 12 → 11 → 4 → 7 → 6 → 6 |
| **Verdict** | FINDINGS_REMAIN — pass 13 dispatched under strict-3-CLEAN (no cap per human directive D-761) |

**Note on pass-12 composition:** The trajectory holds flat at 6. F-P12-001 (HIGH) is a structural unsatisfiability in the AC-001 keep-assertion that probes the source path (always present on clean checkout) rather than the bundle path. F-P12-002 (MEDIUM) is a layer conflation in AC-007 and BC-1.17.001 between the safe-wrapper layer and the extern wire-ABI layer — a single-gate assertion cannot independently verify both layers. F-P12-003 (MEDIUM) is an import-form brittleness in the T-006 literal gate that breaks under idiomatic import consolidation. F-P12-004 (MEDIUM) is an emission-mechanism gap for path_resolution_failed in the two-step decomposed prepare() pattern — the existing path_allowed() -> bool signature collapses failure modes. F-P12-005 (LOW) is a scoping conflict where the blanket zero-count on capability_denied would incorrectly reject correct path_resolution_failed emissions. F-P12-006 (LOW) is a residual stale framing in the epic that still uses the retired "no existing ancestor" language.

---

## Fix-Burst Closure Section (D-763)

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. Same-burst fixes do NOT advance streak.

**HUMAN DIRECTIVE (carried through fix burst):** Continuation policy = STRICT 3-CLEAN (BC-5.39.001), no pass cap, no asymptotic acceptance. Directive carried from D-761.

**All 6 findings closed. Legs sequenced per D-757. Orchestrator independently ran BC-cite preflight post-sweep and caught epic 3-site residual (O-P12-06 fifth consecutive-burst preflight catch); repaired in-burst as part of epic leg.**

### Architect leg (text rulings — Ruling 1 + Ruling 2)

**Ruling 1 (F-P12-002):** Mirror `read_file` at BOTH layers for `read_prefix`. (a) Safe wrapper: `pub fn read_prefix(path: &str, max_bytes: u32) -> Result<Vec<u8>, HostError>` in `hook-sdk/src/host.rs` (same Rust-safe interface as `read_file`; max_bytes is a u32 parameter following ADR-025 D-15 semantics). (b) Extern/wire-ABI: 6-param ptr/len extern `fn read_prefix(path_ptr: *const u8, path_len: u32, max_bytes: u32, out_ptr: *mut u8, out_len_ptr: *mut u32, out_filled_ptr: *mut u32) -> i32` in `hook-sdk/src/ffi.rs` (i32 return encoding CAPABILITY_DENIED/OK/NOT_FOUND/OUTPUT_TOO_LARGE per ADR-025 code table). (c) Non-wasm cfg stub in `hook-sdk/src/host.rs` (mirrors existing `read_file` non-wasm stub). BC amendment to BC-1.17.001 adding the layering parenthetical is RECOMMENDED-NOT-REQUIRED (EC-007 already correctly declares the behavior); adopted under the production-grade default to close F-P12-002 BC leg.

**Ruling 2 (F-P12-004):** Two-step decomposed prepare() in BOTH `read_file.rs` AND `write_file.rs` (mandatory TD-VSDD-060 sibling sweep — NOT deferred). Step 1: call `resolve_path_for_allowlist` (returns `Option<PathBuf>`); if `None` (all ancestors exhausted without canonicalizing), emit `internal.capability_denied reason=path_resolution_failed` and return `CAPABILITY_DENIED`. Step 2: pure prefix check — if resolved path does not start_with an allowed prefix, emit `internal.capability_denied reason=path_not_allowed` and return `CAPABILITY_DENIED`. No BC amendments required (BC-2.07.001 v1.2 EC-007 already declares the path_resolution_failed behavior; the story implementation follows the existing contract without extension). Both `read_file.rs` and `write_file.rs` MUST adopt the two-step pattern in the same implementation burst — the sibling-sweep is not optional and must not be deferred.

### Product-owner leg

- **BC-1.17.001 v1.1→v1.2 (F-P12-002 BC leg):** Ruling 1 layering parenthetical added to §(a) signature description — explicitly distinguishes safe-wrapper layer (Result<Vec<u8>, HostError>) and wire-ABI layer (-> i32 extern). Amendment closes the BC prose ambiguity that enabled F-P12-002; the two-layer design is now normative at the BC level. **Body-amendment evidence (Evidence Rule (a)):** `grep -n "safe.*wrapper\|wire.*ABI\|Result.*Vec.*HostError\|->.*i32\|ffi\.rs\|host\.rs" .factory/specs/behavioral-contracts/ss-01/BC-1.17.001.md | head -10` confirms layering parenthetical in §(a). Closes F-P12-002 BC leg.

- **BC-INDEX v3.75→v3.76 (BC-1.17.001 bump):** BC-1.17.001 row updated v1.1→v1.2. **Body-amendment evidence (Evidence Rule (a)):** `grep "^version:" .factory/specs/behavioral-contracts/BC-INDEX.md` → `"3.76"`. Closes BC-INDEX advance.

### Story-writer leg (sequenced per D-757)

- **S-19.02 v1.8→v1.9 (O-P12-03):** AC-005 unit test A gains affirmative statement: extracted slice INCLUDES opening `---\n` bytes (bytes 0..delimiter_start_offset); `parse_factory_lock` handles document-start marker. **Body-amendment evidence (Evidence Rule (a)):** `grep -n "opening.*---\\\\n\|bytes 0\.\.\|document-start\|delimiter_start_offset" .factory/stories/S-19.02-verify-factory-lock-output-too-large.md | head -5` confirms affirmative statement present. Closes O-P12-03.

- **S-19.03 v1.9→v1.10 (F-P12-004 + F-P12-005):** (a) Architecture Mapping updated with two-step decomposed prepare() pattern (Ruling-2: resolve→None→path_resolution_failed; pure prefix_check→false→path_not_allowed); (b) write_file.rs sibling-sweep clause added to Architecture Mapping (mandatory TD-VSDD-060 — not deferred); (c) unit test added: inject mock canonicalize fn returning Err for every ancestor → assert reason=path_resolution_failed (not path_not_allowed); (d) File Structure `write_file.rs` row gains decomposition note; (e) AC-006 gate scoped to `reason=path_not_allowed` zero-count only — `path_resolution_failed` events explicitly excluded per EC-007. **Body-amendment evidence (Evidence Rule (a)):** `grep -n "two-step\|decomposed\|path_resolution_failed\|write_file.*sibling\|AC-006.*path_not_allowed\|reason=path_not_allowed" .factory/stories/S-19.03-warn-pending-wave-gate-file-not-found.md | head -15` confirms two-step pattern + write_file sibling clause + AC-006 scoping present. Closes F-P12-004 + F-P12-005.

- **S-19.04 v1.10→v1.11 (F-P12-001 + O-P12-01):** AC-001 and AC-007 keep-leg restructured: gate (i) `test -f plugins/vsdd-factory/hook-plugins/vsdd-context-resolvers.wasm` (confirms git-tracked source present); gate (ii) `grep -q "hook-plugins/vsdd-context-resolvers.wasm" plugins/vsdd-factory/resolvers-registry.toml` (confirms registry reference); File Structure `release.yml` note updated to distinguish cargo-built artifacts from source-committed hook-plugins; Task 15 updated; Architecture Mapping gains canonical skip-arm notation (`vsdd_context_resolvers.wasm|wasm_resolver_export.wasm` → echo skip stale resolver artifact: $name; continue ;;). **Body-amendment evidence (Evidence Rule (a)):** `grep -n "resolvers-registry.toml\|hook-plugins/vsdd-context-resolvers.wasm\|skip.*stale.*resolver\|canonical skip" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | head -10` confirms both keep-gates and skip-arm notation present. Closes F-P12-001 + O-P12-01.

- **S-19.05 v1.10→v1.11 (F-P12-003 + O-P12-02 + O-P12-05):** (a) T-006 gate broadened to consolidation-tolerant form: `grep -qE '^use std::sync::(Mutex|\{[^}]*Mutex[^}]*\});'` with negative clause (matching line NOT cfg-gated: `grep -v '#\[cfg'` pre-filter applied first); (b) EC-005 one-line reconciliation preface added: registry name-uniqueness (Invariant 6) holds at runtime; schema-level defense (EC-005) hardens wire format independently; (c) AC-002 counting grep swapped to JSON-aware jq form: `jq -c 'select(.type == "plugin.abandoned")' <sink_file> | grep -c .` (robust to key order/whitespace; non-empty guard semantics preserved). **Body-amendment evidence (Evidence Rule (a)):** `grep -n "std::sync::.*Mutex\|\[^\}.*Mutex\|EC-005\|jq -c.*plugin.abandoned\|select.*plugin.abandoned" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -10` confirms consolidation-tolerant T-006 gate + EC-005 preface + jq counting form present. Closes F-P12-003 + O-P12-02 + O-P12-05.

- **S-19.06 v1.7→v1.9 (F-P12-002 story leg + O-P12-04):** v1.8: AC-007 restructured as three independent gates (Gate 1: `grep -qE 'pub fn read_prefix\(path: &str'` in `hook-sdk/src/host.rs`; Gate 2: `grep -qE 'fn read_prefix\(path_ptr'` in `hook-sdk/src/ffi.rs`; Gate 3: dispatcher dispatch-table cite); signature-disambiguation note added to AC-007 and Architecture Compliance Rules (BC-1.17.001 v1.2 layering parenthetical: Result<Vec<u8>, HostError> safe wrapper vs -> i32 FFI wire ABI); BC table updated v1.1→v1.2; Task 12 + File Structure + Architecture Mapping + Previous Story Intel S-19.04 row updated with DISTINCT Capability Schemas preamble block noting separation from S-19.04's tool-filter work. v1.9 (in-burst preflight catch): BC-1.17.001 body-scope cite in Narrative "The fix" paragraph corrected v1.1→v1.2. **Body-amendment evidence (Evidence Rule (a)):** `grep -n "Gate 1\|Gate 2\|Gate 3\|pub fn read_prefix.*path: &str\|fn read_prefix.*path_ptr\|DISTINCT.*Capability\|BC-1.17.001 v1.2" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -15` confirms three-gate structure + DISTINCT preamble + v1.2 cite. Closes F-P12-002 story leg + O-P12-04.

- **E-19 epic v1.9→v1.10 (F-P12-006 + in-burst BC-cite sweep):** (a) EAC-003 negative-control B framing updated to injectable mock canonicalize fn form per BC-2.07.001 v1.2 EC-007; "path with NO existing ancestor" framing removed from epic EAC-003 cell; (b) BC-1.17.001 body-scope cite sweep: three sites (PRD Capabilities line 113 layering note added, PRD Capabilities follow-on line 115, Out-of-Scope line 199) all updated v1.1→v1.2. Orchestrator post-sweep preflight caught the three-site residual as fifth consecutive-burst preflight catch; repaired in-burst without separate burst-log entry. **Body-amendment evidence (Evidence Rule (a)):** `grep -n "EAC-003\|mock.*canonicalize\|BC-1.17.001 v1.2\|no.*existing.*ancestor" .factory/stories/epics/E-19-post-rc22-operator-hardening.md | head -10` confirms mock-canonicalize framing + v1.2 cites + zero "no existing ancestor" in EAC-003 cell. Closes F-P12-006 + in-burst BC-cite sweep.

- **STORY-INDEX v4.143→v4.145 (story cell updates):** v4.144: S-19.03/04/05/06 cells updated to v1.10/v1.11/v1.11/v1.8 respectively; E-19 epic cell updated to v1.9. v4.145: S-19.02 and S-19.06 cells updated to v1.9 respectively; E-19 epic cell updated to v1.10. All other E-19 story cells UNCHANGED. **Body-amendment evidence (Evidence Rule (a)):** `grep "^version:" .factory/stories/STORY-INDEX.md` → `"4.145"`. `grep -E "S-19\.(02|03|04|05|06)|E-19" .factory/stories/STORY-INDEX.md | head -15` confirms v1.9, v1.10, v1.11, v1.11, v1.9, v1.10 in catalog rows.

### Orchestrator independent verification (before declaring closure)

Orchestrator independently verified the following closure claims by reading production artifact bodies and running the mandatory BC-cite preflight per D-759/D-760:

1. **BC-1.17.001 v1.2 body amendment (F-P12-002):** `grep -n "safe.*wrapper\|wire.*ABI\|Result.*Vec\|->.*i32\|ffi\.rs\|layering" .factory/specs/behavioral-contracts/ss-01/BC-1.17.001.md | head -10` → layering parenthetical confirmed present in §(a).
2. **S-19.03 two-step decomposed pattern (F-P12-004):** `grep -c "two-step\|decomposed\|path_resolution_failed" .factory/stories/S-19.03-warn-pending-wave-gate-file-not-found.md` → ≥3 hits confirming pattern and reason tokens present.
3. **S-19.03 AC-006 scoping (F-P12-005):** `grep -c "reason=path_not_allowed" .factory/stories/S-19.03-warn-pending-wave-gate-file-not-found.md` → ≥1; `grep -c "path_resolution_failed.*excluded\|excluded.*path_resolution_failed" .factory/stories/S-19.03-warn-pending-wave-gate-file-not-found.md` → ≥1; zero-count scope confirmed to path_not_allowed only.
4. **S-19.04 restructured keep-gates (F-P12-001):** `grep -c "resolvers-registry.toml" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md` → ≥1; `grep -c "hook-plugins/vsdd-context-resolvers.wasm" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md` → ≥2 (keep-gate (i) + architecture note).
5. **S-19.05 T-006 consolidation-tolerant gate (F-P12-003):** `grep -c "Mutex\|\\\[^\\\}.*Mutex" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md` → ≥1 (extended gate present).
6. **S-19.06 three-gate AC-007 (F-P12-002 story leg):** `grep -c "Gate 1\|Gate 2\|Gate 3" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md` → 3 (each gate independently present).
7. **Epic EAC-003 mock-canonicalize (F-P12-006):** `grep -c "mock.*canonicalize\|injectable.*mock" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → ≥1; `grep -c "no.*existing.*ancestor" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → 0.
8. **BC-cite preflight — ALL SIX BCs ZERO-DRIFT (mandatory post-sweep):** Per-file loop across all 9 E-19 artifacts (epic + S-19.01..S-19.07) × 6 BCs (BC-4.13.001 v1.8 / BC-2.07.001 v1.2 / BC-2.02.011 v1.4 / BC-3.08.001 v1.19 / BC-5.42.001 v1.1 / BC-1.17.001 v1.2): ZERO stale live tokens. Orchestrator post-sweep caught epic BC-1.17.001 three-site residual (v1.1 in three body-scope positions); repaired in-burst as part of epic leg; confirmed zero residual after repair. Fifth consecutive-burst preflight catch recorded.
9. **4-index at D-763 closure:** `grep "^version:" .factory/specs/behavioral-contracts/BC-INDEX.md .factory/specs/verification-properties/VP-INDEX.md .factory/stories/STORY-INDEX.md .factory/specs/architecture/ARCH-INDEX.md` → BC-INDEX: "3.76" / VP-INDEX: "2.53" / STORY-INDEX: "4.145" / ARCH-INDEX: "2.90".

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. **NEXT:** E-19 adversarial pass-13 (fresh context; 20-policy rubric; strict-3-CLEAN no-cap per human directive D-761; per-file BC-cite preflight mandatory before dispatch; Evidence Rules (a)+(b) mandatory; index-writing legs SEQUENCED per D-757; trajectory 16→14→20→9→8→5→12→11→4→7→6→6→pass-13).
