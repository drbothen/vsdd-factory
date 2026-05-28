# Fixture: propagation-completeness / FAIL

**Scenario:** The derived value "D-453" is present in STATE.md but the
INDEX.md does NOT contain the per-site pattern "D-453 canonical".

**Injected defect:** `.factory/INDEX.md` contains only generic "canonical bash
template registry" text — the per-site pattern `D-453 canonical` does not match.
Propagation is incomplete.

**Per-site pattern design (sites.txt is IDENTICAL to the PASS fixture):**

The same sites.txt is used in both PASS and FAIL fixtures:
- `.factory/STATE.md:Decisions Log.*D-453` — PASS in both fixtures (STATE.md
  contains "D-453 codification is active" in the Decisions Log section)
- `.factory/INDEX.md:D-453 canonical` — PASS in PASS fixture only; FAIL here
  because INDEX.md text omits "D-453" from the canonical entry

This validates that the script correctly applies per-site patterns per
AC-005 (Option A), not global derived-value matching. If the script naively
used `grep -cF "D-453"` on INDEX.md it would incorrectly PASS (INDEX.md still
contains text about canonical templates, just not "D-453" specifically).

**Expected exit code:** 1 (FAIL)
**Expected output:** per-site report showing `.factory/STATE.md` PASS and
`.factory/INDEX.md` FAIL with pattern `D-453 canonical`.

**Script invocation:**
```
propagation-completeness.sh "D-453" sites.txt
```
(called from the fixture directory so relative paths in sites.txt resolve correctly)
