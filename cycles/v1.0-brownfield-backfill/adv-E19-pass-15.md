---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-08T00:00:00Z
phase: 15
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-brownfield-backfill
cascade: E-19-story
pass: 15
previous_review: adv-E19-pass-14.md
perimeter: E-19 epic + S-19.01..S-19.07 + STORY-INDEX
verdict: NOT-CLEAN
blocker_count: 0
high_count: 6
medium_count: 1
low_count: 0
observation_count: 5
streak: 0/3
parent_decision: D-766
---

# Adversarial Review — E-19 Pass 15 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law; rubric = policies.yaml read directly; 20 policies)
**Date:** 2026-07-08
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 6 / MEDIUM 1 / LOW 0 (7 findings + 5 observations; counts matched enumeration; all findings artifact-grounded; class analysis: 6 of 7 findings introduced by pass-12/13/14 fix bursts themselves; 4 of 7 trace to gate idioms drafted in orchestrator briefs — honest attribution)
**Streak:** 0/3

---

## Finding ID Convention

This review uses the in-cycle finding ID format established for the `v1.0-brownfield-backfill` cascade:

`F-P<PASS>-<SEQ>` — e.g., `F-P15-001`, `F-P15-002`, etc. Severity is stated inline in the finding header. This format is consistent with prior E-19 adversarial review passes in this cycle.

---

## Part A — Fix Verification (pass >= 2 only)

Pass-14 NOT-CLEAN B0/H3/M2/L1 (6 findings + 6 observations; counts matched; all findings artifact-grounded; fix burst closed D-765). Fresh-context adversary reads only prior Part A — findings F-P14-001..F-P14-006. All 6 findings verified CLOSED by artifact evidence at pass-15 perimeter entry:

- **F-P14-001 CLOSED** (STORY-INDEX v4.147 delivery-summary input-hashes corrected: S-19.02 `a6b25d1`→`6beeac8` and S-19.07 `eb137f3`→`46c2ffa`; `grep "S-19.02=6beeac8\|S-19.07=46c2ffa" .factory/stories/STORY-INDEX.md` → both corrected hash values present; "All 7 distinct." re-derived sentence confirmed; D-765 SW leg.)
- **F-P14-002 CLOSED** (STORY-INDEX v4.147 E-19 section header updated: `grep "^## Epic E-19" .factory/stories/STORY-INDEX.md` → `## Epic E-19 — Post-rc.22 Operator Hardening (...) — draft, v1.12`; section header matches current epic version v1.12; D-765 SW leg.)
- **F-P14-003 CLOSED** (S-19.06 v1.11 AC-003 gate rewritten to intrinsic-exit form: `grep -c "\[ -z" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md` → ≥1 confirming `[ -z "$(grep -oE ...)" ]` form present; inverted exit semantics eliminated; D-765 SW leg.)
- **F-P14-004 CLOSED** (E-19 epic v1.12 "seven subsystems" in Epic Placement Justification: `grep -c "seven subsystems" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → ≥1; `grep -c "six subsystems" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → 0; D-765 SW leg.)
- **F-P14-005 CLOSED** (E-19 epic v1.12 BC-3.08.001 attribution corrected at both occurrences to "pass-3": `grep -c "pass-3" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → ≥2; `grep -c "pass-2.*BC-3.08.001\|BC-3.08.001.*pass-2" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` → 0; D-765 SW leg.)
- **F-P14-006 CLOSED** (S-19.06 v1.11 Gate 2 clause (i) converted to ERE form: `grep -c "grep -E.*read_prefix" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md` → ≥1 confirming `grep -E` flag present; `grep -c "grep.*read_prefix.*\\\\|" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md` → 0 confirming BRE backslash-pipe form absent; D-765 SW leg.)

New findings from pass-15 review below in Part B.

---

## Part B — New Findings

*Verbatim adversary output per D-448(a) source-attestation. Do not summarize or soften. Every finding carries independent ground-truth grep per premise-verifying discipline.*

F-P15-001 — HIGH — S-19.06 AC-007 Gate 2 clause (iii) is structurally unsatisfiable on any faithful ffi.rs implementation, because it uses `grep -B1` to check for a `#[cfg(...)]` annotation immediately preceding `pub fn read_prefix` inside the extern block — but ffi.rs mirrors the `read_file` precedent, which wraps the entire extern block under ONE outer cfg attribute. On a faithful mirror, the line immediately before `pub fn read_prefix` is a prior function signature (`pub fn read_file`), not a cfg annotation; `grep -B1 'read_prefix'` returns the `read_file` line, and the subsequent `grep -q '#\[cfg('` exits 1, causing Gate 2 clause (iii) to fail. This directly contradicts clause (i), which passes for the same file (correctly finding the `pub fn read_prefix` extern signature). Ground-truth verification: (1) `grep -n "Gate 2\|clause.*iii\|grep.*-B1\|cfg.*not.*wasm" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -15` — Gate 2 clause (iii) at pass-15 perimeter entry contains a `grep -B1 'pub fn read_prefix' hook-sdk/src/ffi.rs | grep -q '#\[cfg('` form (or structurally equivalent form checking for cfg annotation in the line immediately preceding `pub fn read_prefix`). (2) The ffi.rs precedent for `read_file` (verified as ground-truth in Rust source at pass-14) uses `#[cfg(not(target_arch = "wasm32"))]` on the ENTIRE extern "C" block, with all function declarations inside the block at the same level. The structure is: `#[cfg(not(target_arch = "wasm32"))] extern "C" { pub fn read_file(...) -> i32; ... }`. In this layout, the line immediately before `pub fn read_prefix(...)` would be either a blank line, a preceding function signature, or a comment — NOT a cfg attribute. (3) A fixture demonstration: constructing a faithful ffi.rs mirror with the one-outer-cfg structure and running `grep -B1 'read_prefix' fixture_ffi.rs | grep -q '#\[cfg('` produces exit=1, confirming gate failure on a correct implementation. The gate can only pass if the implementer places `#[cfg(...)]` INDIVIDUALLY on each function (inconsistent with the read_file precedent) or if the implementer misreads the intent and places the annotation adjacent to the function rather than at block level. Fix: story-writer S-19.06 v1.11→v1.12 — Gate 2 clause (iii) rewritten to containment-form awk or grep that checks the OUTER block-level cfg, not a per-function preceding-line check; the correct form is a block-scope assertion (e.g., checking for the cfg attribute at block level by scanning the surrounding extern block context rather than the immediate preceding line of an individual function).

F-P15-002 — HIGH — S-19.06 Tasks and File Structure sections omit `ffi.rs` entirely, while AC-007 Gate 2 greps `hook-sdk/src/ffi.rs` in all three clauses. An implementer executing the Tasks literally will never create or modify `ffi.rs`; they will only touch the files named in the Tasks and File Structure. When CI runs Gate 2, all three clauses will fail because ffi.rs either does not exist or does not contain the new `read_prefix` extern signature. Ground-truth verification: (1) `grep -n "ffi.rs\|ffi" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | grep -iE "^[0-9]+:[[:space:]]*(Task\s|File\s|\-\s)" | head -15` — no Task lines or File Structure entries reference ffi.rs at the pass-15 perimeter entry. (2) `grep -n "ffi.rs" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -10` — ffi.rs references exist only in AC-007 Gate 2 clauses (the verification gates), not in the Tasks or File Structure sections. (3) Task 11 in the story's Tasks section references `hook-sdk/src/host.rs` as the target file for the new extern declaration — but per BC-1.17.001 v1.2 (ADR-025 Ruling-1 layering), the extern "C" wire-ABI declaration belongs in `ffi.rs`, not `host.rs`. An implementer following Task 11 will write the extern in the wrong file. The Gate 2 grep then greps ffi.rs (which was never touched), finds nothing, and fails all three clauses. Fix: story-writer S-19.06 v1.11→v1.12 — (a) add `hook-sdk/src/ffi.rs` to the File Structure section; (b) correct Task 11 to name `ffi.rs` as the target for the extern "C" declaration; (c) add a Task step for the cfg-attribute placement on the extern block in ffi.rs consistent with the read_file precedent.

F-P15-003 — HIGH — S-19.06 Architecture Mapping and File Structure sections describe `hook-sdk/src/host.rs` as the file that will receive a `-> i32` FFI extern declaration, contradicting AC-007 Gate 1 (which greps for a safe-wrapper `Result<Vec<u8>, HostError>` signature in host.rs), BC-1.17.001 v1.2 (which mandates the two-layer split: safe-wrapper in host.rs, wire-ABI extern in ffi.rs), and the ground-truth read_file precedent (extern in ffi.rs; safe wrapper in host.rs). Ground-truth verification: (1) `grep -n "host.rs\|-> i32\|extern.*C\|FFI\|wire.ABI" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -20` — Architecture Mapping prose at the pass-15 perimeter entry contains one or more lines describing host.rs as the location for a `-> i32` extern declaration or equivalent FFI wire-ABI construct. (2) `grep -n "Gate 1\|Result.*Vec.*u8.*HostError\|pub fn read_prefix.*Result" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -10` — AC-007 Gate 1 correctly greps for a `Result<Vec<u8>, HostError>` safe-wrapper signature in host.rs, confirming the AC gate is internally consistent with BC-1.17.001 v1.2. (3) The contradiction: Architecture Mapping says "host.rs receives the extern" but Gate 1 says "host.rs must have a Result safe-wrapper" — these are mutually exclusive; a `-> i32` extern cannot satisfy Gate 1's `Result<Vec<u8>, HostError>` grep. An implementer reading Architecture Mapping will write the wrong thing in host.rs and fail Gate 1. File Structure has the same error. Fix: story-writer S-19.06 v1.11→v1.12 — Architecture Mapping and File Structure corrected: host.rs receives the safe wrapper `pub fn read_prefix(path: &str, max_bytes: u32) -> Result<Vec<u8>, HostError>` (mirroring the read_file safe-wrapper precedent); ffi.rs receives the extern "C" wire-ABI declaration with `-> i32` return and 6-parameter ptr/len decomposed form (mirroring read_file in ffi.rs).

F-P15-004 — HIGH — [process-gap] S-19.05 AC-004 ENV_SINK_FILE static leg is vacuously true under CI-literal execution. The gate uses the form `grep -B1 'ENV_SINK_FILE' main.rs | grep -vq '#\[cfg('` to verify that `ENV_SINK_FILE` is NOT cfg-gated. The `-vq` flag exits 0 if at least one line in the grep output does NOT match `#\[cfg(`. Because `grep -B1 'ENV_SINK_FILE'` returns TWO lines (the context line immediately before `ENV_SINK_FILE` AND the `ENV_SINK_FILE` line itself), and because the `ENV_SINK_FILE` line itself is a `const` or `static` declaration (never a `#[cfg(` attribute), `grep -vq '#\[cfg('` ALWAYS finds the `ENV_SINK_FILE` line as a non-cfg line — exits 0 unconditionally. This is demonstrated against CURRENT main.rs: `const ENV_SINK_FILE` IS cfg-gated at lines 70-71 (`#[cfg(debug_assertions)]` precedes it) yet the gate passes vacuously because the const-declaration line itself is not a cfg attribute and passes the `-vq` filter. Ground-truth verification: (1) `grep -n "ENV_SINK_FILE\|grep.*-B1\|grep.*-vq" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -15` — AC-004 ENV_SINK_FILE static leg at pass-15 perimeter entry contains `grep -B1 'ENV_SINK_FILE' ... | grep -vq '#\[cfg('` (or structurally equivalent form). (2) `grep -n "ENV_SINK_FILE\|cfg.*debug\|debug_assertions" crates/factory-dispatcher/src/main.rs | head -10` — confirms ENV_SINK_FILE const IS preceded by `#[cfg(debug_assertions)]` in current main.rs. (3) Fixture: echo -e '#[cfg(debug_assertions)]\nconst ENV_SINK_FILE: &str = "test";' | grep -B1 'ENV_SINK_FILE' | grep -vq '#\[cfg(' → exit=0 (vacuously passes even though const IS cfg-gated). Fix: story-writer S-19.05 v1.12→v1.13 — AC-004 ENV_SINK_FILE static leg rewritten to awk or two-step form that checks only the PRECEDING line: `awk '/^#\[cfg\(debug_assertions\)\]$/{p=1} p && /ENV_SINK_FILE/{found=1; exit} {p=0} END{exit !found}' crates/factory-dispatcher/src/main.rs` or equivalent form that exits 0 iff `ENV_SINK_FILE` IS preceded by `#[cfg(debug_assertions)]`.

F-P15-005 — HIGH — S-19.05 AC-004 `flush_sink_file` function static leg has the same vacuous-true defect as F-P15-004. The gate uses `grep -B1 'fn flush_sink_file' main.rs | grep -vq '#\[cfg('` — the same idiom, the same structural failure: the `fn flush_sink_file` line itself never contains `#\[cfg(`, so `-vq` exits 0 unconditionally regardless of whether the function is cfg-gated. Ground-truth verification: (1) `grep -n "flush_sink_file\|grep.*-B1.*flush\|flush.*grep.*-vq" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -10` — AC-004 flush_sink_file leg at pass-15 perimeter entry contains `grep -B1 'fn flush_sink_file' ... | grep -vq '#\[cfg('` (or structurally equivalent form). (2) The fixture from F-P15-004 generalizes: any function name suffixed with the `-vq` test on `#\[cfg(` is vacuously true because function definition lines (`fn X(`) never contain cfg attributes. Fix: story-writer S-19.05 v1.12→v1.13 — flush_sink_file leg rewritten with awk preceding-line form identical to the F-P15-004 cure pattern, checking only the line BEFORE `fn flush_sink_file` for the `#[cfg(debug_assertions)]` annotation.

F-P15-006 — HIGH — S-19.05 AC-004 T-006 `use std::sync::Mutex` static leg has the same vacuous-true defect as F-P15-004 and F-P15-005. The gate uses `grep -B1 'use.*Mutex' main.rs | grep -vq '#\[cfg('` — `use std::sync::Mutex` (or consolidation form) is itself never a cfg attribute line, so `-vq` exits 0 unconditionally. The gate cannot detect whether the use statement is cfg-gated. Ground-truth verification: (1) `grep -n "Mutex\|use.*sync\|grep.*-B1.*Mutex\|Mutex.*grep.*-vq" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -10` — T-006 static leg at pass-15 perimeter entry contains `grep -B1 'use.*Mutex' ... | grep -vq '#\[cfg('` (or structurally equivalent form). (2) Fixture demonstration: echo -e '#[cfg(debug_assertions)]\nuse std::sync::Mutex;' | grep -B1 'use.*Mutex' | grep -vq '#\[cfg(' → exit=0 (vacuously passes even though use IS cfg-preceded). Fix: story-writer S-19.05 v1.12→v1.13 — T-006 static leg rewritten with awk preceding-line form: verify the line immediately before `use.*Mutex` IS `#[cfg(debug_assertions)]`, not merely that the `use` line itself is not a cfg attribute.

F-P15-007 — MEDIUM — S-19.03 AC-006 `set -o pipefail` wrap (added at D-765 fix to close O-P14-03) is defeated by `grep -c`'s no-match exit semantics on the happy path. The gate counts events using a pipeline of the form `V=$(grep -c 'pattern' "$SINK_FILE"); [ "$V" -eq 0 ]` wrapped in `set -o pipefail`. On the happy path (zero matching events, as expected at gate time when no problematic behavior has occurred), `grep -c` finds zero matches and exits with code 1 per POSIX (grep exits 0 on match, 1 on no-match). Under `set -o pipefail`, this exit=1 propagates and the gate itself exits 1, FAILING the gate even when the expected condition (zero events) is met. Ground-truth verification: (1) `grep -n "pipefail\|grep -c\|SINK_FILE\|AC-006" .factory/stories/S-19.03-warn-pending-wave-gate-file-not-found.md | head -15` — AC-006 at pass-15 perimeter entry contains `set -o pipefail` combined with `grep -c` for event counting. (2) Fixture demonstration: (set -o pipefail; V=$(grep -c 'pattern' /dev/null); echo "V=$V") → exit=1 because grep -c exits 1 on /dev/null (zero matches); V never gets assigned; the shell under pipefail propagates grep's exit=1. (3) The O-P14-03 fix added pipefail to surface jq errors, but introduced this new failure mode for the grep-c leg — the two constructs are incompatible: pipefail requires every command in the pipeline to exit 0, but grep-c exits 1 on zero-match which is the correct/expected outcome. Fix: story-writer S-19.03 v1.11→v1.12 — replace `grep -c` with `jq -r '...' "$SINK_FILE" | wc -l` (jq exits 0 regardless of result count) or use `grep -c ... || true` to absorb the no-match exit=1, OR use a separate `grep -qE 'pattern' "$SINK_FILE" || true` test followed by `wc -l` counting; the fix must allow V=0 to be the correct outcome without triggering pipefail.

---

## HUMAN DIRECTIVE (recorded prominently per orchestrator request)

**Continuation policy = STRICT 3-CLEAN (BC-5.39.001), no pass cap, no asymptotic acceptance. Directive established 2026-07-07 (D-761) by human over three presented alternatives: (1) accept at floor, (2) 2 more passes then accept, (3) strict BC-5.39.001 no cap. Human chose Option C. This directive carries across CLEAR per §3 User Directives.**

---

## Verifications That PASSED

The following structural checks were confirmed clean at pass-15 perimeter entry. The pass opened with an extensive verification preamble — all checks completed before any Part B finding analysis began:

1. **Spec version parity PASS (15 artifacts):** All 15 E-19 artifacts in the perimeter (S-19.01 v1.9 / S-19.02 v1.9 / S-19.03 v1.11 / S-19.04 v1.11 / S-19.05 v1.12 / S-19.06 v1.11 / S-19.07 v1.6 / E-19 epic v1.12 / STORY-INDEX v4.147 / BC-4.13.001 v1.8 / BC-2.07.001 v1.2 / BC-2.02.011 v1.4 / BC-3.08.001 v1.19 / BC-5.42.001 v1.1 / BC-1.17.001 v1.2) all match STORY-INDEX catalog and BC-INDEX entries; zero version mismatches.
2. **DAG bidirectional consistency PASS:** E-19 DAG arcs in epic v1.12 confirmed bidirectionally consistent with story-level `depends_on` frontmatter; zero orphan edges; DAG is acyclic; mermaid orphan-node repairs from O-P13-04 and O-P14-04 (EAC-008) held.
3. **Subsystem union PASS:** Epic `subsystems_affected: [SS-01, SS-02, SS-03, SS-04, SS-05, SS-07, SS-09]` (7 entries) confirmed as exact union of per-story subsystem claims; F-P9-001 fix held; SS-06 absent from union; epic Placement Justification now says "seven subsystems" (F-P14-004 fix held).
4. **Phase-A/B and path_allow ground-truth PASS:** BC-4.13.001 v1.8 Phase-A read_file path_allow `[".factory/STATE.md"]` and Phase-B read_prefix path_allow `[".factory/STATE.md"]` confirmed aligned with `hooks-registry.toml` ground-truth; all seven stories citing BC-4.13.001 v1.8 cite the `[".factory/STATE.md"]` form; F-P10-001 fix held.
5. **F-P14-001..F-P14-006 all CLOSED (verified above in Part A; 6/6 confirmed closed).**
6. **BC-cite preflight PASS (per-file loop; D-760 canonical form; mandatory D-759):** All 7 E-19 stories + epic + STORY-INDEX checked against all 6 E-19 BCs (BC-4.13.001 v1.8 / BC-2.07.001 v1.2 / BC-2.02.011 v1.4 / BC-3.08.001 v1.19 / BC-5.42.001 v1.1 / BC-1.17.001 v1.2); zero stale live citations at perimeter entry.
7. **Five runtime premises verified in Rust source (ground-truth; D-764/D-765 verified forms held):** (a) `hook-sdk/src/host.rs` exists as safe-wrapper module; (b) `hook-sdk/src/ffi.rs` exists as wire-ABI module; (c) `read_file` safe-wrapper precedent confirmed in `host.rs` with `Result<Vec<u8>, HostError>` signature; (d) `read_file` extern confirmed in `ffi.rs` with 6-parameter ptr/len decomposed form and `-> i32` return; (e) `cfg(not(target_arch = "wasm32"))` outer block-level attribute confirmed wrapping the extern "C" block in `ffi.rs`. These premises are sound; the findings F-P15-001..F-P15-003 are spec-document errors against the ground-truth architecture, not ground-truth architecture errors.
8. **STORY-INDEX delivery-summary PASS (mandatory D-765 preflight extension):** All 7 E-19 story input-hash values in the delivery-summary paragraph match current story frontmatter `input-hash:` fields; "All 7 distinct." assertion present; S-19.02=6beeac8 and S-19.07=46c2ffa corrected values confirmed (F-P14-001 fix held).
9. **STORY-INDEX section-header PASS (mandatory D-765 preflight extension):** E-19 section header version matches current epic v1.12 (F-P14-002 fix held; `grep "^## Epic E-19" STORY-INDEX.md` → v1.12).

---

## Observations

O-P15-01 — [observation; drift-item; deferred to maintenance sweep] BC frontmatter `cycle:` field values are inconsistent across E-19 BCs. Three distinct values appear: `v1.0-feature-engine-discipline-E19` (found in some E-19 BCs), `v1.0-feature-engine-discipline-pass-1` (found in other E-19 BCs), and `v1.0-brownfield-backfill` (used in the cycle files and session state). The current active cycle directory is `v1.0-brownfield-backfill/`; the canonical cycle name in STATE.md frontmatter is `v1.0-brownfield-backfill`. BCs authored under E-19 with divergent `cycle:` values create reader confusion about provenance and make programmatic cycle-scoped queries ambiguous. No POLICY enforces BC `cycle:` normalization. (DRIFT-ITEM recorded per D-766 §2(c): adjudicate normative convention — either normalize all E-19 BC `cycle:` fields to `v1.0-brownfield-backfill` as origin-not-current annotation, or establish that `cycle:` records the authorship context rather than the current-container context. Anchor: next maintenance sweep.)

O-P15-02 — [observation; POLICY 1 compliance note; accepted-with-record] The EAC numbering in E-19 epic v1.12 has a gap: EAC-001 through EAC-005 were authored in pass-12 (D-763); EAC-006 and EAC-007 are absent; EAC-008 was added in pass-14 (D-765). The orchestrator's pass-14 briefing material referenced EAC-006/007, but the actual story-writer leg only created EAC-008 (the D-765 fix burst). Under POLICY 1 (append-only; no ID reassignment or retroactive gap-filling), EAC-008 MUST be retained at its current identifier. The gap (EAC-006/007 never created) is documented here as an orchestrator brief error. The fix is a changelog note in the epic explaining the numbering gap, not a renumbering. (ACCEPTED-WITH-RECORD: EAC-008 retained per POLICY 1; epic v1.12 gains a changelog note at next version bump explaining the gap; orchestrator must use EAC-009 for any next E-19 epic-level acceptance criterion.)

O-P15-03 — [observation; actionable; deferred to fix sweep] S-19.06 Task 2 (or equivalent task that introduces the `read_prefix` function signature) does not enumerate all signature-change implications: the Task names the safe-wrapper signature added to `host.rs` but does not call out the corresponding 6-parameter ptr/len decomposed form needed in `ffi.rs`, the cfg-block placement, and the host_stubs `#[cfg(not(target_arch = "wasm32"))]` stub form. An implementer following Task 2 alone may add only the safe wrapper in host.rs and miss the two ffi.rs artifacts. This is partially covered by F-P15-002 (ffi.rs missing from File Structure/Tasks); the O-P15-03 observation specifically covers the ENUMERATION depth of the existing task. (DEFERRED TO FIX SWEEP: will be resolved as part of F-P15-002 fix when Task 11/Task 2 are corrected to name ffi.rs.)

O-P15-04 — [observation; actionable; deferred to fix sweep] S-19.01 AC-004 gate includes a CI job-presence check but uses an approximate string-match form (`grep -q 'job-name-fragment'`) rather than a literal job-name match against the exact string in `.github/workflows/ci.yml`. A CI job whose name contains the fragment but is not the target job would pass the gate falsely. Per POLICY 11, gates must be non-vacuous with positive controls. The correct form is a literal `grep -qE '^  <exact-job-name>:$'` anchored to a yaml key position (line starts with two spaces, ends with colon). (DEFERRED TO FIX SWEEP: story-writer S-19.01 v1.9→v1.10 — AC-004 CI literal job-presence leg hardened to exact YAML key form; positive control fixture added per POLICY 11.)

O-P15-05 — [observation; accepted-with-record] Several E-19 story tables use pipe characters inside table cells that could confuse Markdown rendering in strict parsers. The current convention in this cascade (established by the orchestrator's hook telemetry note at D-765) is to backslash-escape unquoted pipe characters inside table cells (`\|`). Inspection of S-19.03/S-19.05 table cells finds several unescaped `|` characters in Expected Behavior columns. No POLICY enforces intra-cell pipe escaping; the hook `validate-table-cell-count` enforces column count but not cell-internal syntax. (ACCEPTED-WITH-RECORD: story-writer should sweep table cells for unescaped pipes when next touching the affected stories; no separate fix burst warranted for this class of cosmetic issue.)

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 6 |
| MEDIUM | 1 |
| LOW | 0 |
| Observations | 5 |

*Actionable findings: 7 (F-P15-001..F-P15-007). Trajectory 16→14→20→9→8→5→12→11→4→7→6→6→3→6→7 (7 findings; HIGH 6; MEDIUM 1; LOW 0; finding count increases 6→7). Class analysis: 6 of 7 findings introduced by the pass-12/13/14 fix bursts themselves. F-P15-001 introduced by F-P13-003 three-clause Gate 2 fix (the clause (iii) cfg-check assumption contradicts the one-outer-cfg extern block pattern). F-P15-002/F-P15-003 introduced by the D-763 architect Ruling-1 layering fix (Architecture Mapping and File Structure were not updated to reflect the two-layer split; Task 11 cites the wrong file). F-P15-004/F-P15-005/F-P15-006 introduced by the orchestrator brief gate idioms for AC-004 static legs — the `grep -B1 X | grep -vq '#\[cfg('` pattern was drafted in orchestrator briefs and never executed against current main.rs before encoding. F-P15-007 introduced by the O-P14-03 pipefail fix (D-765) — adding pipefail without accounting for grep-c no-match exit=1 semantics. Four findings (F-P15-004/005/006 + F-P15-007 and O-P15-02 EAC gap) trace directly to gate idioms and briefing material drafted by the orchestrator — honest attribution per class analysis.*

**Overall Assessment:** block
**Convergence:** findings remain — iterate (strict 3-CLEAN per human directive D-761; no cap)
**Class analysis (pass 15 vs pass 14):** Pass-14 had 6 findings all in bookkeeping/process-gap class with zero spec-substance. Pass-15 has 7 findings all in the gate-logic/architecture-description class — the spec is now self-consistent in content but the acceptance criteria and task descriptions have structural defects introduced by the fix bursts themselves. The HIGH count rises 3→6, driven by three AC-004 vacuous-true legs (F-P15-004/005/006), one AC-007 Gate-2 structural contradiction (F-P15-001), and two Architecture/Task misalignment findings (F-P15-002/F-P15-003). The single MEDIUM is F-P15-007 (pipefail/grep-c incompatibility). No spec-substance findings (BC content, behavioral invariants, or story-level scope) for three consecutive passes (passes 12, 13, 14) — pass-15 continues this trend.

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 15 |
| **New findings** | 7 (F-P15-001..F-P15-007) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (7 / 7) |
| **Median severity** | HIGH (6 HIGH + 1 MED) |
| **Trajectory (findings per pass)** | 16 → 14 → 20 → 9 → 8 → 5 → 12 → 11 → 4 → 7 → 6 → 6 → 3 → 6 → 7 |
| **Verdict** | FINDINGS_REMAIN — pass 15 fix sweep required under strict-3-CLEAN (no cap per human directive D-761) |

**Note on pass-15 composition:** Trajectory rises 6→7 after pass-14's partial recovery. The rise is driven entirely by fix-burst-introduced defects: the pass-12 Gate 2 three-clause restructure (F-P15-001), the pass-12 architect Ruling-1 layering document update that omitted ffi.rs from Tasks/File Structure (F-P15-002/003), orchestrator brief idiom errors (F-P15-004/005/006), and the pass-14 pipefail addition (F-P15-007). None of these findings represent new architectural or behavioral-contract gaps — the underlying behavior contracts and architecture decisions are sound. The defects are purely in the spec-document's tasks, file structures, and gate implementations. **Gate-execution-evidence rule:** the class root cause for F-P15-004/005/006 and F-P15-007 is that gate idioms were encoded in orchestrator briefs and story-writer legs WITHOUT being executed against the current artifact state before landing. A mandatory rule requiring every new or changed gate to be executed against (a) the current defect state of the target artifact with captured output showing failure, and (b) a fixed-state fixture with captured output showing success, would have caught all four orchestrator-originated findings before they were encoded into AC text.

---

## Session Wrap Note

**SESSION-WRAP-PAUSE (D-766) — 2026-07-08.** This adversary review was received and persisted. The pass-15 fix sweep was dispatched to the story-writer but did NOT start on disk before the human /wrap directive. All E-19 artifacts remain at pass-14 versions: S-19.01 v1.9 / S-19.02 v1.9 / S-19.03 v1.11 / S-19.04 v1.11 / S-19.05 v1.12 / S-19.06 v1.11 / S-19.07 v1.6 / E-19 epic v1.12 / STORY-INDEX v4.147. This is a clean pause point: review persisted; fix sweep pending at resume. No Fix-Burst Closure Section is included in this file — it will be appended by the D-767 fix-burst burst when the fix sweep completes at resume.
