---
pass_id: 59
angle: "Capability anchoring per POLICY 4/5 audit — BC frontmatter capability anchors, E-9 prd_capabilities, story capability claims, BC-INDEX title sync, ARCH-INDEX subsystem name verification"
surface: "E-9 epic v1.51 (3de4342) + BC-1.05.035 + BC-1.05.036 + BC-INDEX v1.3 + ARCH-INDEX + capabilities.md v1.0 + STORY-INDEX v2.10 + S-9.00 + S-9.30 + open-questions.md + lessons.md (TD-VSDD-093) + pass-58 review"
anchor_commit: "3de4342"
date: "2026-05-06"
adversary_model: "claude-opus-4-7[1m]"
prior_clock_state: "0_of_3 (RESET by D-303)"
final_verdict: "SUBSTANTIVE — 1 HIGH / 0 MEDIUM / 0 LOW + 1 non-blocking observation"
findings_count:
  HIGH: 1
  MEDIUM: 0
  LOW: 0
  observations: 1
clock_state_output: "0_of_3 → 0_of_3 (HOLD; SUBSTANTIVE verdict — 1 HIGH found; clock cannot advance with HIGH finding open)"
defect_class: "POLICY 7 BC-INDEX title drift + POLICY 4 semantic-mis-anchor; partial-fix regression from D-279 (v1.33) reframe; sibling-file propagation gap per S-7.01(b)"
td_vsdd_093_application: "FIRST adversary-side application — 8-row quote-verification log (BC-035 H1, BC-035 body line 52, BC-INDEX line 122, v1.33 changelog, BC-INDEX last changelog, capabilities.md CAP-022, ARCH-INDEX SS-01); all PASS"
---

# Adversarial Review Pass 59 — E-9 v1.51

## Capability Anchoring per POLICY 4/5 Audit

**Pass ID:** 59
**Surface:** E-9 epic v1.51 (3de4342) + BC-1.05.035 + BC-1.05.036 + BC-INDEX v1.3 + ARCH-INDEX + capabilities.md v1.0 + STORY-INDEX v2.10 + S-9.00 + S-9.30 + open-questions.md + lessons.md (TD-VSDD-093) + pass-58 review
**Angle:** Capability anchoring per POLICY 4/5 audit — BC frontmatter capability anchors, E-9 prd_capabilities, story capability claims, BC-INDEX title sync, ARCH-INDEX subsystem name verification
**Anchor commit:** 3de4342
**Date:** 2026-05-06
**Prior clock state:** 0_of_3 (RESET by D-303)
**Model:** claude-opus-4-7[1m]

---

## Procedure Summary

This pass audits the E-9 v1.51 surface from a capability-anchoring perspective, addressing POLICY 4 (all behavioral claims anchored to defined capabilities) and POLICY 5 (capability anchors consistent across artifacts). The audit proceeds in 11 steps:

1. **E-9 frontmatter `prd_capabilities` field verification** — Cross-check `[CAP-002, CAP-008, CAP-013, CAP-022]` against `capabilities.md` for each capability's existence, name, and description accuracy.

2. **Story-level capability claims** — Verify S-9.00 and S-9.01..S-9.07 story stubs in STORY-INDEX carry capability references consistent with their epic anchors.

3. **BC frontmatter `capability` field audit** — Read `capability:` field in BC-1.05.035 and BC-1.05.036 frontmatter. Verify that `CAP-TBD` status is correctly declared (not stale from an earlier version).

4. **E-9 Capability Anchor Justification section** — Verify that the prose anchor justifications reference only real capabilities (CAP-022, CAP-002, CAP-008, CAP-013) with correct descriptions.

5. **ARCH-INDEX subsystem name consistency** — Verify SS-01 ("Hook Dispatcher Core"), SS-02 ("Hook SDK and Plugin ABI"), SS-03 ("Event Emission (OTel-Aligned)"), SS-04 ("Plugin Ecosystem") names match across E-9 body, ARCH-INDEX, BC-INDEX section headers.

6. **BC-INDEX section headers vs ARCH-INDEX** — Verify BC-INDEX SS-NN section header names match ARCH-INDEX subsystem names exactly (full name parity, not partial match).

7. **BC-INDEX title sync vs BC body H1** — For each BC cited in the E-9 surface (BC-1.05.035 and BC-1.05.036), verify BC-INDEX row title matches BC body H1 verbatim at the trailing fragment level.

8. **lessons.md TD-VSDD-093 first-application coverage** — Verify that the just-codified TD-VSDD-093 rule has its Enforcement section consistent with the self-application verification log at the end of lessons.md.

9. **Pass-58 review file consistency** — Verify `anchor_commit` in pass-58 frontmatter (`e6c8f4a`) resolves to a legitimate prior surface version of E-9; verify `prior_clock_state` is consistent with D-302 seal state.

10. **S-9.00 capability anchor** — Read S-9.00 story frontmatter for capability references; verify consistency with E-9 CAP-022 anchor.

11. **open-questions.md OQ-W16-007/008/009/010** — Spot-check open questions for capability-anchor drift (e.g., OQ language referencing withdrawn capability anchors or stale SS-XX names).

---

## Findings

### HIGH-P59-001 — BC-INDEX line 122 BC-035 title fragment differs from BC-035 H1 trailing fragment

**Severity:** HIGH
**Location:** `specs/behavioral-contracts/BC-INDEX.md` line 122, BC-1.05.035 index entry
**Finding class:** POLICY 7 BC-INDEX title drift; sibling-file propagation gap from D-279 (v1.33) reframe; concurrent POLICY 4 semantic-mis-anchor (BC-INDEX phrase misrepresents the BC-035 mechanism)

**Evidence:**

TD-VSDD-093 Quote-Verification Log (8 rows):

| # | Source | Quoted text | Status |
|---|--------|-------------|--------|
| 1 | BC-035 H1 (line 28) | `factory-dispatcher::host::exec_subprocess::canonicalizes_binary_path_before_allow_check — Path::canonicalize() applied before binary_allow match; TOCTOU prevention via canonicalize-then-allow-list-check ordering` | PASS — direct read |
| 2 | BC-035 body line 52 (§Description TOCTOU paragraph) | `The canonicalize step's value is TOCTOU (time-of-check-time-of-use) prevention.` | PASS — confirms TOCTOU framing in body |
| 3 | BC-INDEX line 122 (current state) | `factory-dispatcher::host::exec_subprocess::canonicalizes_binary_path_before_allow_check — Path::canonicalize() applied before binary_allow match; symlink-based traversal rejected` | PASS — directly read; trailing fragment is `symlink-based traversal rejected` |
| 4 | v1.33 changelog entry (E-9 summary table) | `D-279 architectural-reframe burst — HIGH-P36-001 + HIGH-P36-002 closed via architectural reframe: dropped "trusted project-root prefix" coinage, dropped "symlink_traversal_escape" concept…BC-1.05.035 reframed around TOCTOU prevention` | PASS — confirms v1.33 D-279 reframed BC-035 around TOCTOU |
| 5 | BC-INDEX last changelog entry date | `2026-05-04` (timestamp field in BC-INDEX frontmatter) | PASS — confirms BC-INDEX last updated 2026-05-04, before D-279 which occurred 2026-05-05 |
| 6 | capabilities.md CAP-022 | `Port hook plugins from bash to native WASM` | PASS — E-9 CAP-022 anchor correctly cited |
| 7 | ARCH-INDEX SS-01 name | `Hook Dispatcher Core` | PASS — consistent across BC-INDEX SS-01 header and ARCH-INDEX |
| 8 | BC-INDEX SS-01 section header | `### SS-01 — Hook Dispatcher Core (BC-1) — 106 BCs (104 active; 2 retired)` | PASS — matches ARCH-INDEX |

**Root cause:** D-279 (v1.33, 2026-05-05) reframed BC-035 around TOCTOU prevention:
- BC-035 H1 was updated to: `…Path::canonicalize() applied before binary_allow match; TOCTOU prevention via canonicalize-then-allow-list-check ordering`
- BC-035 body was updated to reflect TOCTOU framing throughout
- BC-INDEX line 122 was NOT updated — last BC-INDEX changelog entry is 2026-05-04 (before D-279)
- BC-INDEX still shows: `…Path::canonicalize() applied before binary_allow match; symlink-based traversal rejected`

The BC-INDEX trailing fragment `symlink-based traversal rejected` is the pre-D-279 framing. After D-279 reframe, symlink rejection is no longer described as a separate mechanism — it is subsumed by the allow-list-miss path under the TOCTOU framing. The old phrase therefore now semantically misrepresents the BC-035 mechanism (POLICY 4 violation: the BC-INDEX title no longer accurately describes what BC-035 specifies).

**Detection step:** Step 7 of the procedure — BC-INDEX title sync vs BC body H1 cross-check for E-9 surface BCs.

**Required fix:** Update BC-INDEX line 122 trailing fragment from `symlink-based traversal rejected` to `TOCTOU prevention via canonicalize-then-allow-list-check ordering` — verbatim from BC-035 H1.

---

### Obs-P59-001 [process-gap] — Missing BC-INDEX-update sweep in post-edit grep verification discipline

**Severity:** Non-blocking observation
**Class:** Process gap — current post-edit grep verification scope does not include BC-INDEX scan when BC body H1 is modified

**Detail:** HIGH-P59-001 demonstrates that when a BC body's H1 is reframed (as in D-279 v1.33), the BC-INDEX sibling row is not automatically updated. The current grep verification discipline (TD-VSDD-078 + S-7.01(b)) does not mandate a BC-INDEX scan as part of BC-H1-modification verification. This gap allowed the BC-035 H1 reframe to propagate to BC-035 body but not to BC-INDEX — the defect went undetected for approximately 24 burst-iterations (D-279 through D-304 pre-fix).

**Recommendation:** Extend the post-edit grep verification scope to mandate a BC-INDEX scan whenever a BC body H1 is modified. The specific check: after any BC H1 modification, grep BC-INDEX for the BC's ID and verify the title fragment matches the new H1 trailing fragment.

**Codification candidacy (S-7.02):** If this sweep (recommended in Obs-P59-001) finds ≥2 additional BC-INDEX-vs-H1 drifts beyond HIGH-P59-001, the S-7.02 3-occurrence threshold is met and TD-VSDD-094 (BC-INDEX-update sweep mandate) should be codified.

---

## Self-Application Audit (TD-VSDD-093 — First Adversary-Side Application)

This pass-59 review constitutes the FIRST adversary-side application of TD-VSDD-093 (codified at D-303). All claim-evidence pairs in the finding above have been quote-verified against source-of-truth artifacts.

**Verification log:**
1. BC-035 H1 (line 28): quoted verbatim above from direct read of `specs/behavioral-contracts/ss-01/BC-1.05.035.md`
2. BC-035 body TOCTOU paragraph: quoted verbatim above from §Description
3. BC-INDEX line 122: quoted verbatim above from direct read of `specs/behavioral-contracts/BC-INDEX.md`
4. v1.33 D-279 reframe history: quoted from E-9 epic summary table v1.33 row (D-279 entry)
5. BC-INDEX last-updated date: quoted from BC-INDEX frontmatter `timestamp: 2026-05-04T00:00:00`
6. CAP-022 description: verified against capabilities.md
7. ARCH-INDEX SS-01 name: verified against ARCH-INDEX
8. BC-INDEX SS-01 header: quoted verbatim

**TD-VSDD-093 PASS** — all 8 quote-verification log rows PASS; no fabricated claims in finding evidence.

---

## 5-Axis Sibling Sweep (TD-VSDD-089)

1. **Postcondition ↔ EC parity:** BC-035 Postcondition 1 (TOCTOU framing — canonicalize-before-allow-check) is internally consistent with BC-035 body throughout. The fix required is in BC-INDEX only; BC-035 body is correct.
2. **Cross-BC reference accuracy:** BC-INDEX line 123 BC-1.05.036 entry verified: `factory-dispatcher::host::exec_subprocess::emits_completed_event_on_success — host.exec_subprocess.completed event on every successful subprocess completion`. This matches BC-036 H1 verbatim. No drift in BC-036 row.
3. **Numeric enumeration:** 1 HIGH finding (HIGH-P59-001 BC-INDEX line 122 drift). 0 MEDIUM. 0 LOW.
4. **Parenthetical lists:** N/A.
5. **Codification artifact sibling integrity:** Obs-P59-001 recommends proactive sweep; if sweep finds ≥2 additional drifts, TD-VSDD-094 codification would be required (state-manager to perform sweep and codify if threshold met).

---

## Novelty Assessment

**Angle novelty:** CONFIRMED NEW — capability anchoring per POLICY 4/5 audit has not been explicitly performed in any of the 58 prior passes. Prior passes covered glossary/terminology (pass-58), frontmatter schema compliance (pass-57), markdown-table well-formedness (pass-56), NORMATIVE rule cross-application (pass-55), external-reference link integrity (pass-54), glossary sweep (pass-53), TV-derivation (pass-52), signal-flow/data-flow (pass-51), SOUL #4 (pass-50), whole-document fresh-eyes (pass-49), NITPICK (pass-48), stable-anchor citations (pass-47), TD-090 self-application (pass-46, pass-45), pass-44 convention meta audit, pass-43 partial-fix regression, etc.

The specific sub-angle of **BC-INDEX cross-reference audit** (Step 7) is also novel — no prior pass explicitly mapped BC-INDEX row titles to BC body H1 for drift detection.

---

## Files Referenced

- `stories/epics/E-9-tier-2-native-wasm-migration.md` (v1.51, anchor_commit 3de4342)
- `specs/behavioral-contracts/ss-01/BC-1.05.035.md`
- `specs/behavioral-contracts/ss-01/BC-1.05.036.md`
- `specs/behavioral-contracts/BC-INDEX.md` (v1.3, last changelog 2026-05-04)
- `specs/architecture/ARCH-INDEX.md`
- `specs/domain-spec/capabilities.md`
- `stories/STORY-INDEX.md` (v2.10)
- `stories/S-9.00-perf-baseline-and-bundle-ceiling.md`
- `stories/S-9.30-sdk-run-subprocess-extension.md` (withdrawn)
- `architecture/open-questions.md`
- `cycles/v1.0-brownfield-backfill/lessons.md` (TD-VSDD-093 section)
- `cycles/v1.0-brownfield-backfill/adv-e9-v1.7-amendment-pass-58.md`
