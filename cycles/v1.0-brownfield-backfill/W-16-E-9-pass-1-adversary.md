# W-16 E-9 pass-1 adversarial review

Date: 2026-05-03
Adversary agentId: aa9b4344fbdff4952
Target: .factory/stories/epics/E-9-tier-2-native-wasm-migration.md (377L)
Verdict: SUBSTANTIVE (18 findings: 5 HIGH, 8 MEDIUM, 5 LOW)
Pass: 1 of N (per ADR-013, need 3 NITPICK_ONLY for convergence)

> **Note:** Full 18-finding table in agent transcript at agentId aa9b4344fbdff4952.
> This file captures the 5 HIGH severities verbatim; MEDIUM and LOW summaries follow.
> Story-writer fix-burst MUST consult the full transcript before applying fixes.

---

## Findings Table

| ID | Severity | Category | Location | Description | Suggested Fix |
|----|----------|----------|----------|-------------|---------------|
| F-1 | HIGH | cross-doc | E-9 lines 273-277 vs ADR-014 lines 257-263 vs audit-w16.md §5 | Risk ID drift. ADR-014 defines R-W16-002 = WASI preopens and R-W16-004 = bats/WASM test infrastructure. E-9 redefines R-W16-001..005 with different semantics. Cross-doc references break. | Rename E-9's risks to fresh IDs OR align definitions to ADR-014/audit-w16.md and add the missing WASI preopens risk. |
| F-2 | HIGH | cross-doc | E-9 frontmatter vs E-8 D-13 (line 632) | Wave-16 designation collision. E-8 status `ready` with W-16 stories S-8.11..S-8.19; E-9 also claims W-16. Two competing authoritative sources for W-16 scope. | Update E-8 to `status: superseded` for Tier 2/3 portions and explicitly delegate W-16 to E-9. |
| F-3 | HIGH | library-pinning / semantic | E-9 line 322 | hyperfine listed as "Bundle-size measurement harness" — wrong tool. Bundle size measured by `du -sb`. hyperfine is a CLI benchmarking tool for latency/throughput, not bundle bytes. POLICY 4 violation. | Replace row with `du` (coreutils) or remove hyperfine from the bundle-size row. Add hyperfine to the perf-benchmarking row if desired. |
| F-4 | HIGH | AC-quality | E-9 AC-6 (line 290) | Stale line-number anchors `crates/hook-sdk/src/lib.rs:58` etc. Brittle on refactor — will silently diverge when the file is touched. | Replace with grep-based assertion: `grep -c 'fn host_fn' crates/hook-sdk/src/lib.rs` style. |
| F-5 | HIGH | scope / risk-coverage | E-9 Risks vs audit-w16.md §5 R-W16-002 | Missing WASI preopens / path_allow risk. 19 of 23 hooks need `path_allow = [".factory/"]` in WASM sandbox config. Not captured as a risk in E-9. | Add R-W16-006 for WASI preopens path_allow coverage: define scope, owner, and mitigation. |
| F-6 | MEDIUM | cross-doc | E-9 story topology vs ADR-014 §4 | Story count discrepancy. ADR-014 §4 lists ~9 stories but E-9 wave topology shows 9 stories split across 3 waves with different grouping than ADR-014's batch groupings. | Reconcile: either update ADR-014 §4 batch-to-story mapping to match E-9 waves, or add a cross-ref note in E-9. |
| F-7 | MEDIUM | scope | E-9 lines 180-190 — Wave 0 stories | S-9.30 (host::run_subprocess) listed in Wave 0 alongside S-9.00 (perf baseline). S-9.30 is a story, not a prerequisite infra story; placing it in Wave 0 implies it gates Wave 1. Clarify whether S-9.30 truly gates S-9.01..S-9.06 or if it can be parallelized. | Move S-9.30 to Wave 1 if it does not gate other work, or add an explicit gating rationale. |
| F-8 | MEDIUM | library-pinning | E-9 §lib-table — wasm-bindgen | wasm-bindgen listed without pinned version. POLICY 4 requires pinned versions for all external library rows in an epic lib table. | Add minimum version constraint (e.g., `0.2.92`) or a version range. |
| F-9 | MEDIUM | library-pinning | E-9 §lib-table — wasmtime | wasmtime listed without pinned version. Same POLICY 4 violation as F-8. | Pin to the version currently used in factory-dispatcher Cargo.toml. |
| F-10 | MEDIUM | library-pinning | E-9 §lib-table — wit-bindgen | wit-bindgen listed without pinned version. POLICY 4 violation. | Pin to the version currently in use or add minimum constraint. |
| F-11 | MEDIUM | scope | E-9 — hook count | E-9 intro says "23 hooks" but audit-w16.md §3 lists 23 validate-*.sh + 1 generate-registry.sh. E-9 scope does not clarify whether generate-registry.sh is in scope (rewrite-clean decision unclear for that hook). | Add explicit scope note: is generate-registry.sh included in W-16 rewrite-clean or deferred to W-17? |
| F-12 | MEDIUM | semantic | E-9 AC-3 (bundle ceiling) | AC-3 references S-9.00 ceiling values as if they are finalized. S-9.00 is a Wave 0 story that PRODUCES the baseline — the ceiling values are not yet defined at E-9 authoring time. | Rewrite AC-3 to reference S-9.00 via "values established in S-9.00 AC-3" rather than embedding specific kB numbers. |
| F-13 | MEDIUM | cross-doc | E-9 risk R-W16-003 vs SS-02 §5 | R-W16-003 (bundle size ceiling) risk description does not reference SS-02's SubprocessCaps schema or the additional payload weight it introduces. Could drift. | Add cross-reference in R-W16-003 to SS-02 SubprocessCaps and ADR-014's size guidance. |
| F-14 | LOW | editorial | E-9 frontmatter `status` | E-9 frontmatter has `status: draft`. After D-4 Burst 1 authoring and per project lifecycle, the status should reflect the current state (e.g., `status: in-review` while adversarial convergence is underway). | Update status to `in-review` or add a note about the adversarial convergence phase. |
| F-15 | LOW | formatting | E-9 §story-topology table | Wave 2 has only S-9.07 as a single-story wave. The story-topology table heading row for Wave 2 is not aligned with Wave 0 and Wave 1 formatting. | Normalize formatting across all wave sub-tables. |
| F-16 | LOW | scope | E-9 §success-criteria | Success criteria do not reference the ADR-013 convergence requirement (3 NITPICK_ONLY passes per story). | Add a note that convergence per ADR-013 is required before implementation for each story. |
| F-17 | LOW | cross-doc | E-9 vs S-9.00 frontmatter | E-9 lists S-9.00 as `wave: 0` but S-9.00's frontmatter may not have a `wave` field. If not, cross-doc diverges on wave assignment. | Verify S-9.00 frontmatter includes `wave: 0` consistent with E-9 topology. |
| F-18 | LOW | editorial | E-9 §background | Background section refers to "20 of 23 trivial rewrite-clean" hooks but ADR-014 §2 uses "20/23" in a slightly different context. Minor wording alignment issue. | Align phrasing with ADR-014 for traceability. |

---

## Convergence Status

- **Pass 1:** SUBSTANTIVE — 18 findings (5 HIGH, 8 MEDIUM, 5 LOW)
- **Next action:** Story-writer fix-burst on all 18 findings; priority order: F-1 (risk ID alignment with ADR-014), F-2 (E-8 superseded delegation), F-3 (hyperfine semantic correction), F-5 (WASI preopens risk R-W16-006 addition), F-4 (stale line-number anchors)
- **Convergence clock:** 0 of 3 NITPICK_ONLY passes
