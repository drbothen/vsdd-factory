---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.00-perf-baseline-bc-anchor-verification.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: 68f3d16
traces_to: prd.md
pass: 2
previous_review: adv-s8.00-p1.md
target: S-8.00 v1.1
target_file: .factory/stories/S-8.00-perf-baseline-bc-anchor-verification.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 8
findings_high: 0
findings_med: 4
findings_low: 3
findings_nit: 1
---

# Adversarial Review: S-8.00 v1.1 (Pass 2)

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix identifying adversarial findings
- `<CYCLE>`: Cycle prefix — `S8P2` for this S-8.00 pass-2 review
- `<PASS>`: Two-digit pass number (`P02`)
- `<SEV>`: Severity abbreviation (`MED`, `LOW`, `NIT`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples used here: `F-P2-001` through `F-P2-008` (short form per pass-1 convention).

## Executive Summary

Pass-2 fresh-context review of S-8.00 v1.1 (473 lines). All 14 pass-1 findings
verified closed. 8 new findings discovered: 0 HIGH, 4 MED, 3 LOW, 1 NIT.
Verdict: **SUBSTANTIVE** — fix burst required before pass-3. Clock held at
0_of_3 per ADR-013 (MED findings prevent advance).

Trajectory: 14 → 8 (43% decay). Dominant issue class: identifier/wording
precision (double-prefix residue, anchor relationship, measurement path
ambiguity). Pass-3 expected to drop to LOW/NIT-only if fix burst is
comprehensive.

---

## Part A — Fix Verification (Pass-1 Closure Audit)

All 14 pass-1 findings verified closed in v1.1.

| Finding | Description | Closed? | Evidence |
|---------|-------------|---------|---------|
| F-P1-001 | CAP-022 cross-CAP stretch disclosure | CLOSED | Disclosure paragraph present before Architecture Mapping |
| F-P1-002 | STORY-INDEX priority P1→P2 | CLOSED | STORY-INDEX line 164 shows P2 |
| F-P1-003 | AC-7 identifier sweep ("E-8 epic" disambiguation) | CLOSED (PARTIAL — see F-P2-001, F-P2-002) | Sweep applied but double-prefix residue remains at 2 locations |
| F-P1-004 | STORY-INDEX title restored "Tier 1" | CLOSED | "Tier 1 BC-anchor verification" present in title |
| F-P1-005 | Tasks renumbered A.N section | CLOSED | Tasks §A.1..A.6 present |
| F-P1-006 | Tasks renumbered B.N/C.N sections | CLOSED | Tasks §B.1..B.5, §C.1..C.2 present |
| F-P1-007 | BC-7.00 sub-family rationale | CLOSED | Rationale paragraph present; grep confirms 0 BC-7.00 files |
| F-P1-008 | AC-1 hook list locked | CLOSED | handoff-validator/validate-bc-title/protect-bc with rationale |
| F-P1-009 | input-hash convention | CLOSED | Comment block + git command present |
| F-P1-010 | AC-9 BC-8.26.001 mis-anchor replaced | CLOSED | [process-gap] E-8 D-2 epic-fix-burst-integrity anchor |
| F-P1-011 | policies.yaml cross-ref format | CLOSED | Format corrected |
| F-P1-012 | EC-004 majority-threshold rationale | CLOSED | Rationale present |
| F-P1-013 | STORY-INDEX E-8 transitional-points footnote | CLOSED | Footnote present in STORY-INDEX |
| F-P1-014 | EC-007 git command | CLOSED | git command specified |

---

## Part B — New Findings (Pass-2 Fresh-Context Discovery)

### F-P2-001 [MED] Double-prefix residue at AC-3 line 229

**Location:** AC-3 acceptance criteria block, approximately line 229.

**Issue:** The fix-burst for F-P1-003 swept most "S-8.00 AC-7" / "E-8 AC-7"
conflations, but the canonical reference form "E-8 epic E-8 AC-7b" still
appears at line ~229 inside AC-3. The outer "E-8 epic" qualifier already
establishes the epic context; the inner "E-8 AC-7b" then re-prefixes with
"E-8" redundantly. Canonical form should be "E-8 epic AC-7b".

**Evidence:** Line 229 reads (approximately):
```
...per E-8 epic E-8 AC-7b, the benchmark harness...
```

**Suggested fix:** Change "E-8 epic E-8 AC-7b" → "E-8 epic AC-7b".

**Policy:** POLICY 7 (cross-document identifier consistency). The sweep from
F-P1-003 was applied correctly in most locations but missed this instance —
validates the fresh-context compounding value pattern.

---

### F-P2-002 [MED] Double-prefix residue at Goal §2 line 201

**Location:** Goal §2 narrative, approximately line 201.

**Issue:** Same class as F-P2-001. Goal §2 contains "E-8 epic E-8 AC-7b"
double-prefix. As above, canonical form is "E-8 epic AC-7b".

**Evidence:** Line ~201 reads (approximately):
```
...fulfilling E-8 epic E-8 AC-7b (Tier 2 benchmark coverage)...
```

**Suggested fix:** Change "E-8 epic E-8 AC-7b" → "E-8 epic AC-7b".

**Policy:** POLICY 7. Part of the same residue class as F-P2-001; both must
be fixed in the same burst to avoid a third-pass recurrence.

---

### F-P2-003 [MED] OQ-6 anchor relationship ambiguous — regression-gate subprocess capability

**Location:** Open Questions table (OQ-6 row) and AC-6 acceptance criterion.

**Issue:** OQ-6 reads as if S-8.00 must resolve the regression-gate subprocess
capability question before story completion. However, per E-8 epic design and
S-8.09's spec scope, regression-gate subprocess execution is a downstream
S-8.09 deliverable. S-8.00's BC-anchor verification table (Task B) audits
whether an anchor exists — it does not deliver the subprocess implementation.

The OQ-6 wording ("must resolve before implementation") implies a blocking
relationship that does not exist. S-8.00 can close OQ-6 by documenting that
the capability is deferred to S-8.09; it does not need to implement it.

EC-005 similarly references OQ-6 without clarifying that S-8.00 only
establishes the audit record, not the resolution.

**Evidence:** OQ-6 row and EC-005 both use language that implies S-8.00
delivers the subprocess capability, rather than auditing its absence and
recording the S-8.09 deferral.

**Suggested fix:** Reframe OQ-6 as: "OQ-6 is downstream — S-8.00 audits
regression-gate's BC-anchor status and records S-8.09 as the resolution
vehicle. S-8.00 itself does not resolve OQ-6." Update EC-005 to match.
Add an assumption_validations entry confirming OQ-1 and OQ-8 are in scope
for S-8.00 (not OQ-6).

**Policy:** POLICY 3 (scope boundary). Blurring the audit vs. implementation
boundary inflates S-8.00 scope and could mislead the implementer.

---

### F-P2-004 [MED] measurement_method ambiguous — "or time builtin" fallback undermines reproducibility

**Location:** AC-4 acceptance criterion and measurement_method field.

**Issue:** AC-4 specifies hyperfine as the measurement tool, but the
measurement_method field (or nearby fallback prose) retains a "or time
builtin" branch. The `time` builtin has substantially lower resolution
(~1ms vs ~0.1ms), different statistical properties (single sample vs
hyperfine's configurable warmup+runs), and inconsistent output format
across shells. Allowing `time builtin` as a fallback means the AC-4 threshold
(±10% coefficient of variation) could be satisfied by a lower-quality
measurement method, making the baseline non-reproducible.

**Evidence:** measurement_method YAML block or inline prose retains "or time
builtin" as an alternative after F-P1-008 fix.

**Suggested fix:** Drop the "or time builtin" branch entirely. Lock
measurement_method to hyperfine. Add hyperfine to the Library & Framework
table as REQUIRED (not optional). Add a Task A.0 pre-flight step: "Verify
hyperfine is installed (hyperfine --version); if absent, install via cargo
install hyperfine or system package manager. Do not proceed to A.1 without
hyperfine." This makes the dependency explicit and blocks silent fallback.

**Policy:** POLICY 6 (measurability). A baseline that can be established via
two different tools with different precision is not a reproducible baseline.

---

### F-P2-005 [LOW] STORY-INDEX line 166 — version/line-count/narrative stale (state-manager scope)

**Location:** `.factory/stories/STORY-INDEX.md` line 166 footnote.

**Issue:** The S-8.00 STORY-INDEX footnote still reads "v1.0" and "444 lines"
and "Adversarial pass-1 next". After the pass-1 fix burst, the story is now
v1.1 (473 lines), and pass-2 is complete. The footnote should reflect the
current v1.2 version (post-pass-2 fix), 504 lines, and pass-3 intent.

**Evidence:** STORY-INDEX line 166 (pre-fix): "S-8.00 authored 2026-04-30
v1.0 (status=draft). 444 lines; ... Adversarial pass-1 next. D-164 sealed."

**Suggested fix (state-manager scope):** Update to:
"S-8.00 v1.2 (status=draft, pass-2 fix burst applied 2026-04-30). 504 lines;
9 ACs; 5pts; depends_on=[]; blocks S-8.01..S-8.09. ... Adversarial pass-1
closed (14 findings, all closed in v1.1 fix burst); pass-2 closed (8
findings, 6 closed in v1.2 fix burst); pass-3 next. D-164 + D-165 + D-166
sealed."

**Policy:** POLICY 3 (state-manager-runs-last). STORY-INDEX is a state-manager
artifact; update belongs in the same commit as adv-s8.00-p2.md persistence.

---

### F-P2-006 [LOW] STORY-INDEX pass narrative needs pass-1+pass-2 closure record (state-manager scope)

**Location:** `.factory/stories/STORY-INDEX.md` line 166 footnote (same line
as F-P2-005, but distinct issue).

**Issue:** The footnote's "Adversarial pass-1 next" phrase needs to be replaced
with a full closure record: pass-1 closed (14 findings, all closed), pass-2
closed (8 findings, 6 closed in v1.2 fix burst), pass-3 next. The D-164 and
D-165 sealed references need D-166 appended.

**Evidence:** Same line 166. The phrase "Adversarial pass-1 next. D-164
sealed." is stale across two dimensions: (a) it predates pass-1 closure;
(b) it predates pass-2 entirely.

**Suggested fix (state-manager scope):** Covered by F-P2-005 suggested fix
above (both fixes apply to the same footnote rewrite). Treat as a single edit.

**Policy:** POLICY 3 (state-manager-runs-last).

---

### F-P2-007 [LOW] bundle_size.measured_at duality — single vs multi-plugin ambiguity

**Location:** AC-4 output schema / JSON block for bundle_size measurement.

**Issue:** The bundle_size.measured_at field (or equivalent) does not clarify
whether the measurement is per-plugin (one WASM binary) or for the full set
of 9 Tier 1 hook plugins collectively. The OQ-8 baseline (~10ms warm
invocation) is per-plugin, but bundle_size is a different metric. If the
schema captures aggregate size, the duality should be noted so implementers
don't conflate the two.

**Evidence:** JSON schema block for the output artifact shows
bundle_size.measured_at without a clarifying note distinguishing
per-plugin vs aggregate measurement.

**Suggested fix:** Append a duality note to the JSON schema block:
"# bundle_size: per-plugin (single WASM binary); not an aggregate across all
9 Tier 1 hooks. Aggregate is out of scope for S-8.00."

**Policy:** POLICY 6 (measurability — measurement scope must be unambiguous).

---

### F-P2-008 [NIT] jq dependency undeclared in Library & Framework table

**Location:** Library & Framework dependency table.

**Issue:** Task A.7 (schema validation step) uses `jq` for JSON output
validation. jq is an external tool that is not universally installed and is
not listed in the Library & Framework table. This is a minor consistency gap
— jq is widely available but the table claims to be complete. Adding it
prevents a future "tool not found" surprise for implementers on minimal
environments.

**Evidence:** Task A.7 references `jq` for output JSON schema validation but
the Library & Framework table does not list it.

**Suggested fix:** Add a row to the Library & Framework table:
`jq | system | JSON output schema validation (Task A.7) | REQUIRED`.

**Policy:** NIT / POLICY 6 (toolchain completeness). Not a blocking issue;
fix in the same burst as other findings for cleanliness.

---

## Part C — Cross-Document Consistency Audit

| Check | Target | Result |
|-------|--------|--------|
| S-8.00 v1.1 subsystems=[SS-01, SS-07] vs ARCH-INDEX | ARCH-INDEX SS-01 + SS-07 present | PASS |
| S-8.00 behavioral_contracts=[] with [process-gap] disclosure | D-2 Option C; behavioral_contracts=[] intentional | PASS |
| S-8.00 blocks S-8.01..S-8.09 vs E-8 epic story list | E-8 v1.7 lists S-8.01..S-8.09 as Tier 1 | PASS |
| S-8.00 depends_on=[] vs STORY-INDEX | No blocking deps declared | PASS |
| OQ-8 (~10ms) baseline cross-reference | E-8 D-154 records OQ-8 as ungrounded estimate | PASS (S-8.00 documents auditing responsibility) |
| CAP-022 stretch disclosure vs Wave 7 F-204 pattern | Disclosure paragraph present per F-P1-001 | PASS |
| STORY-INDEX line 166 version/line count | v1.0 / 444 lines — STALE (F-P2-005) | FAIL |
| E-8 epic AC-7b references vs S-8.00 text | Double-prefix residue at 2 locations (F-P2-001, F-P2-002) | FAIL |
| AC-6 / EC-005 OQ-6 relationship | Misleading blocking relationship (F-P2-003) | FAIL |

---

## Part D — Policy Compliance Sweep

| Policy | Description | Result | Notes |
|--------|-------------|--------|-------|
| POLICY 1 — Lifecycle completeness | Story has all required sections | PASS | All sections present in v1.1 |
| POLICY 2 — BC anchor integrity | behavioral_contracts=[] with [process-gap] | PASS | Intentional per D-2 Option C |
| POLICY 3 — State-manager-runs-last | F-P2-005/006 are state-manager scope | FLAG | STORY-INDEX needs update (state-manager task) |
| POLICY 4 — Input-hash currency | input-hash=68f3d16 present with git command | PASS | F-P1-009 fix confirmed |
| POLICY 5 — Dependency symmetry | depends_on=[] / blocks=[S-8.01..S-8.09] | PASS | Symmetric with E-8 epic |
| POLICY 6 — Measurability | AC-4 threshold defined; hyperfine ambiguity (F-P2-004) | PARTIAL | Fix burst needed |
| POLICY 7 — Cross-document identifier consistency | E-8 epic AC-7b double-prefix residue | FAIL | F-P2-001, F-P2-002 |
| POLICY 8 — Scope boundary clarity | OQ-6 scope ambiguity (F-P2-003) | PARTIAL | Needs reframe |
| POLICY 9 — Same-burst VP registration | behavioral_contracts=[] deferred; no VPs needed at draft | PASS | Consistent with D-2 Option C |
| POLICY 10 — Open question resolution path | OQ-6 needs resolution-vehicle clarification | PARTIAL | F-P2-003 |
| POLICY 11 — Test tautology avoidance | Not applicable (story spec, not code) | N/A | |
| POLICY 12 — BC TV emitter consistency | Not applicable (story spec, not code) | N/A | |

---

## Part E — Self-Validation Loop

**Iteration 1:** Re-read all 8 findings. Confirmed: F-P2-001 and F-P2-002 are
distinct locations (AC-3 vs Goal §2) and both are genuine double-prefix
residues, not false positives — the fix-burst for F-P1-003 applied "E-8 epic
AC-7b" in some locations but left "E-8 epic E-8 AC-7b" in two others.
F-P2-003 is grounded in the E-8 epic design: S-8.09 is the subprocess
delivery vehicle; S-8.00 audits only. F-P2-004 is grounded in measurement
science: time builtin is single-sample, ~1ms resolution vs hyperfine's
multi-sample, ~0.1ms resolution.

**Iteration 2:** Checked severity assignments. F-P2-001 and F-P2-002 are MED
(not HIGH): the double-prefix is a consistency defect but does not create a
false specification claim; implementer would understand the intent. F-P2-003
is MED: the scope ambiguity could cause an implementer to attempt subprocess
work in S-8.00, inflating complexity — genuine implementation risk. F-P2-004
is MED: a non-reproducible measurement baseline is a spec integrity defect,
not merely cosmetic.

**Iteration 3:** Confirmed no false positives. F-P2-005 and F-P2-006 are
legitimate state-manager scope items (STORY-INDEX is state-manager artifact).
F-P2-007 is LOW: bundle_size duality is clarifying information, not a
blocking gap. F-P2-008 is NIT: jq is widely available; the gap is cosmetic.
No findings withdrawn.

---

## Part F — Novelty Assessment

| Finding | Novel or Pass-1 Prior? | Notes |
|---------|----------------------|-------|
| F-P2-001 | **Flagged as pass-1 prior** (AC-3 double-prefix location) | F-P1-003 sweep partial closure; fresh-context found residue |
| F-P2-002 | **Flagged as pass-1 prior** (Goal §2 double-prefix location) | Same class; 2nd residue location missed by pass-1 sweep |
| F-P2-003 | **Novel** | OQ-6 scope ambiguity not visible at pass-1 (OQ table not yet present; v1.0 didn't have the OQ-6 entry in current form) |
| F-P2-004 | **Novel** | measurement_method fallback not examined at pass-1 |
| F-P2-005 | **Novel** (state-manager scope) | STORY-INDEX freshness is state-manager discipline; pass-1 state-manager fixed 4 items but line 166 narrative was not fully updated |
| F-P2-006 | **Novel** (state-manager scope) | Same as F-P2-005 |
| F-P2-007 | **Novel** | JSON schema duality not examined at pass-1 |
| F-P2-008 | **Novel** | jq dependency gap introduced by A.7 task (Task renumbering in F-P1-005/006 added A.7 scope) |

Fresh-context compounding value confirmed: 2 pass-1 partial closures
re-surfaced + 6 net-new findings in 8-finding pass-2 baseline.

---

## Part G — Process-Gap Tags

None in this pass.

---

## Part H — Priority Fix Order

### Story-writer scope (6 findings)

1. **F-P2-004** [MED] — measurement_method: drop "or time builtin"; lock
   to hyperfine; add to Library & Framework as REQUIRED; add Task A.0 pre-flight.
2. **F-P2-003** [MED] — OQ-6 anchor relationship: reframe as downstream S-8.09
   gate; update AC-6 + EC-005; confirm assumption_validations=[OQ-1, OQ-8].
3. **F-P2-001** [MED] — AC-3 line ~229 double-prefix: "E-8 epic E-8 AC-7b" →
   "E-8 epic AC-7b". Sweep all story text for remaining residue.
4. **F-P2-002** [MED] — Goal §2 line ~201 double-prefix: same fix.
5. **F-P2-007** [LOW] — bundle_size.measured_at: append per-plugin vs aggregate
   duality note to JSON schema block.
6. **F-P2-008** [NIT] — jq: add to Library & Framework table as REQUIRED.

### State-manager scope (2 findings)

7. **F-P2-005 + F-P2-006** [LOW] — STORY-INDEX line 166: combined rewrite to
   v1.2, 504 lines, pass-1+pass-2 closure narrative, D-164+D-165+D-166 cite,
   pass-3 next.
8. Persist adv-s8.00-p2.md to cycles/v1.0-brownfield-backfill/.

---

## Verdict

**SUBSTANTIVE** — 4 MED findings require fix burst before pass-3.

**Clock:** 0_of_3 → 0_of_3 (held per ADR-013; MED findings prevent advance).

**Trajectory:** 14 → 8 (43% decay). Dominant residue class: identifier
precision (double-prefix) + scope boundary + measurement reproducibility.
Pass-3 expected to drop to 1–3 LOW/NIT-only and advance clock 0_of_3 →
1_of_3 if fix burst is comprehensive.

**Pass-3 priors for adversary:**
- Verify A.0..A.7 task sequencing is coherent (A.0 pre-flight new in fix burst)
- Verify EC-005 OQ-6 deferral wording is precise and does not re-introduce scope ambiguity
- Full double-prefix sweep (grep for "E-8 epic E-8") to confirm zero residue
- Verify Library & Framework table completeness (hyperfine REQUIRED + jq REQUIRED)
