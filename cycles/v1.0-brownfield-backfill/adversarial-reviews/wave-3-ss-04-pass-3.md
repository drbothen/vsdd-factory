---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-26T00:00:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-2.01-legacy-bash-adapter.md
  - .factory/stories/S-3.01-port-capture-commit-activity.md
  - .factory/stories/S-3.02-port-capture-pr-activity.md
  - .factory/stories/S-3.03-port-block-ai-attribution.md
  - .factory/stories/S-5.01-session-start-hook.md
  - .factory/stories/S-5.02-session-end-hook.md
  - .factory/stories/S-5.03-worktree-hooks.md
  - .factory/stories/S-5.04-post-tool-use-failure.md
  - .factory/specs/prd.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/behavioral-contracts/ss-02/BC-2.01.002.md
  - .factory/specs/verification-properties/VP-INDEX.md
input-hash: "7ec1aac"
traces_to: ".factory/specs/prd.md"
cycle: v1.0-brownfield-backfill
sub_cycle: wave-3-ss-04-re-anchor
pass: 3
verdict: FINDINGS_REMAIN
finding_count: 4
convergence_step: 0_of_3
po_commit_reviewed: 7ec1aac
previous_review: wave-3-ss-04-pass-2.md
---

# Adversarial Review — Wave 3 SS-04 Re-anchor — Pass 3

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`. Examples: `ADV-W3SS04-P03-HIGH-001`, `ADV-W3SS04-P03-MED-002`.

## Pass-2 Closure Verification

| Pass-2 Finding | Status | Evidence |
|---|---|---|
| F-101 (HIGH) — VP-044 removed from S-3.03 | CLOSED | S-3.03:23 `verification_properties: []`; body lines 60-65 disclose v1.1 candidate `VP-NNN-native-plugin-block-result` |
| F-102 (MED) — CAP-008 add SS-02 | CLOSED | capabilities.md:49 `Subsystems: SS-01, SS-02, SS-04, SS-07` with process-gap comment line 50 |
| F-103 (MED) — CAP-013 add SS-01 | CLOSED | capabilities.md:69 `Subsystems: SS-01, SS-04, SS-07` with process-gap comment line 70 |
| F-104 (HIGH) — S-5.01-04 BC-1.01 → BC-1.01.001 | CLOSED-WITH-RESIDUAL — see ADV-P03-LOW-002 | All four files cite BC-1.01.001 with HTML-comment placeholder |
| F-105 (HIGH) — POLICY 8 [process-gap] markers | CLOSED | All five AC traces converted; v1.1 BC candidate rows present |
| F-106 (LOW) — S-3.01:58 self-contradiction | CLOSED | Internally consistent |
| F-107 (LOW) — S-5.03 SS-03 in subsystems | CLOSED | S-5.03:28 `subsystems: ["SS-01", "SS-03", "SS-04"]`; CAP-003 added |

All 7 pass-2 findings closed at primary site. New findings identify propagation gaps (F-102/F-103 to PRD §8) and a residual semantic anchor weakness (acknowledged in F-104 fix prose).

## Part B — New Findings (4 total: 0 CRIT, 1 HIGH, 1 MED, 2 LOW)

### ADV-W3SS04-P03-HIGH-001 [HIGH] — F-102/F-103 propagation gap: PRD §8 CAP-to-BC subsystem columns out of sync with capabilities.md

**Files:**
- `.factory/specs/prd.md:1094` — CAP-008 row: Subsystems `SS-01, SS-04, SS-07`
- `.factory/specs/prd.md:1099` — CAP-013 row: Subsystems `SS-04, SS-07`
- `.factory/specs/domain-spec/capabilities.md:49` — CAP-008: `Subsystems: SS-01, SS-02, SS-04, SS-07`
- `.factory/specs/domain-spec/capabilities.md:69` — CAP-013: `Subsystems: SS-01, SS-04, SS-07`

**Policies:** POLICY 4, POLICY 6. S-7.01 partial-fix-regression discipline (c).
**Confidence:** HIGH

The pass-2 F-102 and F-103 fixes added SS-02 to CAP-008 and SS-01 to CAP-013 in `capabilities.md`. Per S-7.01(c), the fix burst must propagate to every file referencing those CAP subsystem lists. PRD §8 still carries pre-fix subsystem columns:

- CAP-008 PRD row: `SS-01, SS-04, SS-07` — missing SS-02
- CAP-013 PRD row: `SS-04, SS-07` — missing SS-01

Blast radius = 2 files; per S-7.01 "Blast radius = 2+ files: HIGH". This is sibling-file regression analogous to pass-2 F-104.

The drift is not theoretical: PRD §8 is the cited "CAP-to-BC anchoring" source for downstream story creators. A future story-writer reading PRD §8 to anchor a CAP-008 story would get the wrong subsystem list and exclude SS-02 — the exact mis-anchor F-102 was meant to prevent.

**Fix:** Update PRD §8 CAP-008 row to `SS-01, SS-02, SS-04, SS-07` and CAP-013 row to `SS-01, SS-04, SS-07`. Same-burst propagation.

### ADV-W3SS04-P03-MED-001 [MEDIUM] — S-3.03 Verification Properties disclosure misses VP-038 — semantic anchor exists in catalog

**Files:**
- `.factory/stories/S-3.03-port-block-ai-attribution.md:60-70`
- `.factory/specs/verification-properties/VP-INDEX.md:91` — VP-038 SDK HookResult Exit Codes Are Stable

**Policies:** POLICY 4, POLICY 9
**Confidence:** HIGH

S-3.03 declares "No existing VPs directly verify block-ai-attribution WASM behavior" and proposes v1.1 VP candidate anchored to `BC-2.01.002 postcondition 2`. VP-INDEX.md line 91 already contains **VP-038**: "SDK HookResult Exit Codes Are Stable — Continue=0, Error=1, Block=2", scope `SS-02`, type `invariant`, anchored on the same BC. The story is proposing a v1.1 candidate that already exists.

Per POLICY 9, VP-INDEX is the source of truth — proposing duplicate VPs without checking the catalog is a creators_justify_anchors gap.

**Fix:** Replace "No existing VPs" with: "VP-038 (SDK HookResult Exit Codes Are Stable — SS-02, BC-2.01.002 anchor) verifies the SDK-side exit-code contract that block-ai-attribution must satisfy. A SS-04-scoped 'native plugin Block result causes dispatcher exit 2 at the process boundary' VP is a v1.1 candidate that complements VP-038 by extending it across the dispatcher integration layer." Add VP-038 to `verification_properties:` frontmatter.

### ADV-W3SS04-P03-LOW-001 [LOW] — S-3.02:50 stale BC-2.01 reference still names "BC-1.01.001 or BC-4.03.NNN"

**File:** `.factory/stories/S-3.02-port-capture-pr-activity.md:50`

**Policies:** POLICY 4, POLICY 7
**Confidence:** HIGH

Line 50 reads:
> "Note: BC-2.01 referenced in architecture compliance rules is a stale reference (should be BC-4.01.001 or BC-4.03.NNN). Pre-emptive BC candidates below cover the uncontracted behavior."

Current Architecture Compliance Rules (line 139) cites **BC-2.01.002** ("HookResult exit codes Continue=0 / Block=2 / Error=1 — SS-02 SDK core") as the rule source. BC-2.01.002 is a real, semantically appropriate anchor — not stale. The note is vestigial pass-1 prose.

**Fix:** Delete the note OR rewrite to: "Note: BC-2.01.002 is the active SS-02 SDK anchor cited in Architecture Compliance Rules; SS-04 plugin BCs are v1.1 candidates listed below."

### ADV-W3SS04-P03-LOW-002 [LOW] — F-104 residual: BC-1.01.001 anchor remains a sanctioned-stretch with placeholder comments

**Files:**
- `.factory/stories/S-5.01-session-start-hook.md:133`
- `.factory/stories/S-5.02-session-end-hook.md:134`
- `.factory/stories/S-5.03-worktree-hooks.md:143`
- `.factory/stories/S-5.04-post-tool-use-failure.md:133`

**Policies:** POLICY 4
**Confidence:** MEDIUM

All four S-5.NN stories carry the F-104 fix using BC-1.01.001 as the registry-schema anchor for `once:true`/`async:true` config fields. Each carries an HTML-comment placeholder. BC-1.01.001's H1 in BC-INDEX.md line 42 is "Registry rejects unknown schema version" — semantically validates schema_version mismatches, not generic field validation.

Per pass-2 F-001 sanctioned-template-anchor precedent, deliberate stretch-anchors are acceptable when explicitly disclosed and v1.1 candidates logged. The current v1.1 candidates contract hooks.json template inclusion but **none** contract the registry-schema validation of the `once:true`/`async:true` field at SS-01.

**Fix options (author adjudication):**
(a) Remove "Pending: select most precise" comment and add disclosure: "BC-1.01.001 sanctioned-stretch anchor per Wave 3 F-104 closure — pending v1.1 BC for the once:true/async:true field validation."
(b) Add v1.1 BC candidates for the SS-01 once:true/async:true field-validation BCs.

## Observations

- [process-gap] **CAP→PRD §8 propagation discipline** — pass-2 F-102/F-103 fixes correctly updated capabilities.md but did not propagate to PRD §8. Recommend codifying: any change to CAP Subsystems in capabilities.md must include same-burst PRD §8 row update. State-burst hook could enforce.
- [process-gap] **VP-INDEX consultation gate** — pass-2 introduced 1 v1.1 VP candidate without VP-INDEX search. ADV-P03-MED-001 (VP-038 missed) is the symptom. Recommend v1.1 VP candidate logging include mandatory VP-INDEX grep for the BC anchor.
- PRD §14 "FRs defined | 43" vs §7 "Total: 44 FRs" — pass-1 F-009 / pass-2 / pass-3 confirmed open; FR-045/FR-046 remain proposed-not-defined. Out-of-scope for Wave 3 close.

## CAP Subsystem Drift Sweep — DRIFT FOUND (PRD §8)

| CAP | capabilities.md Subsystems | PRD §8 Subsystems | Drift |
|-----|----------------------------|-------------------|-------|
| CAP-008 | SS-01, SS-02, SS-04, SS-07 | SS-01, SS-04, SS-07 | Missing SS-02 |
| CAP-013 | SS-01, SS-04, SS-07 | SS-04, SS-07 | Missing SS-01 |

See ADV-P03-HIGH-001.

## Bidirectional `depends_on`↔`blocks` Symmetry Sweep

Spot-checked; out-of-scope for SS-04 review (`pending intent verification` consistent with pass-2). No new findings.

## Cross-Subsystem Leakage Sweep

S-5.03 SS-03 leakage closed by F-107. S-3.03 SS-02 leakage closed by F-002. No new findings.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 1 |
| LOW | 2 |

**Overall Assessment:** pass-with-findings
**Convergence:** clock RESETS per BC-5.04.003 (1 HIGH + 1 MED)
**Readiness:** requires revision before pass-4

## Novelty Assessment

| Field | Value |
|---|---|
| **Pass** | 3 |
| **New findings count** | 4 |
| **Duplicate count** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | LOW |
| **Severity distribution** | 0 CRIT, 1 HIGH, 1 MED, 2 LOW |
| **Trajectory** | pass-1=11 → pass-2=7 → pass-3=4 (decreasing); HIGH 4→3→1 |
| **Verdict** | FINDINGS_REMAIN |

Genuinely novel:
- ADV-P03-HIGH-001: sibling-file propagation gap (CAP→PRD §8) prior passes did not sweep
- ADV-P03-MED-001: missed-anchor in VP catalog (VP-038)
- LOW findings: cleanup of pass-2 closure residuals

## Convergence Status

**0 of 3.** 1 HIGH + 1 MED reset clock per BC-5.04.003. Cannot advance to 1_of_3.

If pass-3 had returned only the 2 LOW findings, this would have been NITPICK_ONLY → 1_of_3.

## Findings by Axis

| Axis | Findings |
|---|---|
| Semantic Anchoring (B) | ADV-P03-MED-001, ADV-P03-LOW-002 |
| FR/Subsystem Hygiene (F) | ADV-P03-HIGH-001 |
| Bookkeeping (L) | ADV-P03-LOW-001 |
| Pre-existing TD/out-of-scope | (Observations) |

## Trajectory Baseline

| Pass | Total | CRIT | HIGH | MED | LOW |
|------|-------|------|------|-----|-----|
| 1 | 11 | 0 | 4 | 4 | 3 |
| 2 | 7 | 0 | 3 | 2 | 2 |
| 3 | 4 | 0 | 1 | 1 | 2 |

Net reduction pass-2→pass-3: 3 (43%). HIGH down 2 (3→1). MED down 1 (2→1). Severity shifting to LOW. Fresh-context value validated.

## Verdict

**FINDINGS_REMAIN.** Two findings require remediation:
1. ADV-P03-HIGH-001 — PRD §8 propagation gap
2. ADV-P03-MED-001 — S-3.03 missed VP-038

Two LOW findings are cleanup items. Pass-4 target: confirm closure; if NITPICK_ONLY (≤3 LOW only), advance convergence_step to 1_of_3.
