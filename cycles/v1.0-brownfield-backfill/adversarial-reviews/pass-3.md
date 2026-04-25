---
document_type: adversarial-review-pass
level: ops
producer: adversary
phase: 1d
pass: 3
timestamp: 2026-04-25T23:45:00Z
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/specs/PRD.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/architecture/SS-05-orchestration.md
  - .factory/specs/architecture/decisions/ADR-002-wasm-plugin-abi.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/behavioral-contracts/ss-09/BC-9.01.001.md
  - .factory/specs/domain-spec/L2-INDEX.md
  - .factory/specs/domain-spec/invariants.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/stories/STORY-INDEX.md
  - .factory/tech-debt-register.md
  - .factory/policies.yaml
  - .factory/cycles/v1.0-brownfield-backfill/adversarial-reviews/pass-1.md
  - .factory/cycles/v1.0-brownfield-backfill/adversarial-reviews/pass-2.md
traces_to: phase-1d-pass-4
---

# Adversarial Review Pass 3 — Phase 1 Spec Package

## Coverage

Full re-read of all 6 indexes plus: PRD.md §1, §7, §10.4, §14, SS-05-orchestration.md,
ADR-002, BC-9.01.001.md, STORY-INDEX.md, VP-INDEX.md. Cross-checked pass-1 and pass-2
findings disposition against current file state.

---

## Part 1: Pass 1 + Pass 2 Follow-Up Audit

10 prior findings reviewed. 9 cleanly fixed. 1 partial regression caught and logged as F-031.

| Prior ID | Description | Pass-3 Verdict |
|----------|-------------|----------------|
| F-001 | PRD §14 missing status block | FIXED — status block present |
| F-002 | ARCH-INDEX module count vs file count mismatch | FIXED — counts reconciled |
| F-003 | BC-INDEX vs actual BC file count | FIXED — counts verified |
| F-004 | VP catalog deferred supplement files | FIXED — KL-001 acknowledges |
| F-005 | DRIFT-011 not propagated to §14 | FIXED in pass-2 (partially — see F-029 below) |
| F-018 | KL-005 concurrent self-modification in PRD §10.4 | FIXED — KL-005 section present |
| F-021 | SS-05 line 24 "34" vs body inconsistency | FIXED — line 24 says 34 |
| F-022 | SS-05 line 49 "(33 files)" not updated | PARTIAL — line 49 still says 33 → F-031 |
| F-025 | PRD §10.4 DRIFT-011 forward ref missing | FIXED — DRIFT-011 entry present in §10.1 |
| F-028 | tech-debt-register TD-012 wording | FIXED — wording corrected |

---

## Part 2: Pass 3 New Findings

### F-029 — HIGH — PRD §14 DRIFT count stale (10 vs 11)

**Location:** `PRD.md` line 1152
**Observed:** `| DRIFT items open | 10 (DRIFT-001 through DRIFT-010) |`
**Expected:** DRIFT-011 was added in pass-2 fix for F-025. Count and range must reflect this.
**Impact:** Status snapshot gives auditors incorrect open DRIFT count. POLICY 8 (cascade PRD
§14 when adding DRIFTs) was not applied when DRIFT-011 was inserted.
**Fix:** Update line 1152 to `| DRIFT items open | 11 (DRIFT-001 through DRIFT-011) |`

---

### F-030 — HIGH — PRD §14 story status taxonomy conflicts with STORY-INDEX

**Location:** `PRD.md` lines 85, 1153–1154
**Observed:**
- Line 85: "Tiers A through D (22 stories) are closed."
- Line 1153: `| Stories shipped | 22 (Tier A–D) |`
- Line 1154: `| Stories pending | 19 (Tiers E–H) |`

**Expected:** STORY-INDEX shows: merged=22, partial=4, draft=15 (total 41).
S-2.05 (Tier C) is `partial`, not `merged`. The 4 partial stories (S-2.05, S-3.04, S-4.06,
S-5.05) are conflated into either "closed" or "pending" with no explicit accounting.
§1.2 line 85 says "Tiers A through D (22 stories) are closed" — this implies S-2.05 is
fully closed, which contradicts STORY-INDEX.

**Impact:** A reader of PRD §1.2 believes all 22 Tier A–D stories are fully shipped.
S-2.05's crates.io publish step is open (DRIFT-002 tracks it). The "22 closed" claim
is misleading.

**Fix (PRD §14 line 1153–1154):**
```
| Stories shipped (merged) | 22 (Tier A–D fully merged) |
| Stories partial          | 4 (S-2.05, S-3.04, S-4.06, S-5.05) |
| Stories pending (draft)  | 15 (Tiers E–H draft) |
```
**Fix (PRD §1.2 line 85):** Change to:
"Tiers A through D (22 merged + 4 partial stories; 26 total) are substantially closed.
Tiers E through H (15 draft stories) are the active backlog for rc.1 and 1.0 GA."

---

### F-031 — MEDIUM — Pass-2 partial regression: SS-05 line 49 still says "(33 files)"

**Location:** `SS-05-orchestration.md` line 49
**Observed:** `| ... (33 files) | Specialist sub-personas; ...`
**Expected:** Pass-2 fixed line 24 to say "34 specialist sub-agents" but pass-2 fix did
not update line 49's parenthetical. Line 24 and line 49 are now inconsistent.
**Impact:** Low operational impact but spec is internally inconsistent. The F-022 fix
was incomplete.
**Fix:** Edit line 49: `(33 files)` → `(34 files)`

---

### F-032 — MEDIUM — VP-005/VP-021/VP-044 cite NFR-SEC/NFR-PERF IDs with no NFR catalog in L3 spec tree

**Location:** VP-INDEX.md entries for VP-005, VP-021, VP-044; PRD §10.4
**Observed:** Multiple VPs reference NFR-SEC-NNN and NFR-PERF-NNN identifiers as if they
are addressable artifacts. The 76-item NFR catalog is summarized in PRD §4 and stored in
`prd-supplements/nfr-catalog.md` (per PRD line 768). However, the specific NFR-NNN IDs
cited by individual VPs (e.g., NFR-SEC-001, NFR-PERF-003) are not enumerated anywhere in
the L3 spec tree — there is no `specs/nfr-catalog.md` or equivalent L3 document with
addressable NFR-NNN identifiers.
**Impact:** VPs that reference NFR-NNN IDs cannot be fully traced. A reviewer cannot
look up "NFR-SEC-001" in the L3 spec tree.
**Fix:** Add KL-006 to PRD §10.4 acknowledging the gap. NFR catalog lift to L3 deferred
to v1.1.

---

### F-033 — MEDIUM — PRD §7 FR-005 status row conflates shipped and partial

**Location:** `PRD.md` line 883 (FR-005 row in §7 traceability table)
**Observed:** `| FR-005 | ... | shipped/partial | E-1 |`
**Expected:** "shipped/partial" is ambiguous. CAP-008 deny gates are fully shipped;
CAP-002 `read_file` is partial (DRIFT-001). The status column should distinguish which
sub-capability is partial.
**Impact:** A reader cannot tell which BCs are shipped vs partial without cross-referencing
§7 FR-005 prose. The inline note in §7 prose already clarifies, but the table status row
does not.
**Fix:** Update FR-005 table row status to:
`shipped (CAP-008 deny gates); partial (CAP-002 read_file per DRIFT-001)`

---

### F-034 — MEDIUM — BC-9.01.001 Architecture Module field cites wrong script

**Location:** `behavioral-contracts/ss-09/BC-9.01.001.md` line 73
**Observed:** `| Architecture Module | SS-09 — \`scripts/generate-registry-from-hooks-json.sh\`, release tooling (bump-version.sh) |`
**Expected:** `generate-registry-from-hooks-json.sh` is a registry utility (SS-02/SS-04
scope), not the architecture module for BC-9.01.001. BC-9.01.001 covers bump-version.sh
prerelease semver acceptance. The leading script reference is incorrect.
**Impact:** Misleads a reader about which script the BC is testing. The evidence table
(line 80) correctly cites `<TBD> (bump-version.sh location not cited)` — the module
field should match.
**Fix:** Edit line 73 to:
`| Architecture Module | SS-09 — \`scripts/bump-version.sh\` (prerelease semver; see Source Evidence for location TBD) |`

---

### F-035 — MEDIUM — Draft stories S-3.01 and S-4.01 have empty behavioral_contracts arrays with no policy covering this

**Location:** `stories/STORY-INDEX.md` and individual draft story files
**Observed:** S-3.01 and S-4.01 (and other draft stories) cite BCs in their body prose
but have `behavioral_contracts: []` in frontmatter.
**Expected:** Either the frontmatter should anchor BCs, OR the STORY-INDEX should state
explicitly that draft stories MAY defer BC anchoring to elaboration.
**Impact:** A traceability tool that reads `behavioral_contracts:` frontmatter will
incorrectly report zero BCs for these stories. No policy covers the omission.
**Fix (Option 2 — less invasive):** Add a policy note to STORY-INDEX:
> **Draft story policy:** Stories with `status: draft` MAY have empty
> `behavioral_contracts: []` arrays. BC anchoring is deferred to the elaboration phase
> (status transitions to `ready`).

---

### F-036 — LOW — PRD KL-003 says "6 of 57 VPs" — actual count is 7

**Location:** `PRD.md` line 1042
**Observed:** `6 of 57 VPs are not derived from domain invariants.`
**Expected:** KL-003 lists: VP-024, VP-048, VP-053..VP-057.
Count: VP-024 (1) + VP-048 (1) + VP-053, VP-054, VP-055, VP-056, VP-057 (5) = 7 total.
The prose says 6 but enumerates 7.
**Impact:** Off-by-one in a known-limitations statement. Low operational impact.
**Fix:** Edit line 1042: `6 of 57 VPs` → `7 of 57 VPs`

---

### F-037 — LOW — ADR-002 "45 registry entries" is a bare magic number with no anchor

**Location:** `architecture/decisions/ADR-002-wasm-plugin-abi.md` line 66
**Observed:** `IN-EFFECT. All 45 registry entries load \`legacy-bash-adapter.wasm\` via this ABI.`
**Expected:** "45" is a count that will drift as entries are added/removed. It is not
anchored to `hooks-registry.toml` or any BC/ARCH-INDEX breakdown.
**Impact:** Low — this is a status note, not a contract. But a future auditor cannot
verify "45" without counting manually.
**Fix:** Append a parenthetical: `(45 registry entries per \`hooks-registry.toml\` head as
of beta.4; see ARCH-INDEX SS-04 + SS-07 BCs for breakdown)`

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 9 |
| **Duplicate/variant findings** | 0 (F-031 is a pass-2 partial regression, not a duplicate) |
| **Novelty score** | 9 / (9 + 0) = 1.0 |
| **Median severity** | 2.5 (2 HIGH + 5 MEDIUM + 2 LOW) |
| **Trajectory** | 17→11→9 |
| **Verdict** | FINDINGS_REMAIN |

Finding distribution: 2 HIGH + 5 MEDIUM + 2 LOW.

The two HIGH findings (F-029, F-030) are both cascade failures from the DRIFT-011 addition
in pass 2 — the count in §14 was not updated and the story taxonomy was not reconciled.
These are systematic bookkeeping gaps, not architectural flaws.

F-031 closes the open tail of F-022 (pass-2 partial fix).

F-032 through F-035 are structural traceability gaps that are real but non-blocking for v1.0.
F-036 and F-037 are nitpick-level corrections.

**Pass** — convergence is progressing. Pass 4 projection: NITPICK (0–3 findings, all LOW).
Convergence requires 3 consecutive NITPICK passes (passes 4, 5, 6) per POLICY 11.
