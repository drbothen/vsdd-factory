# Fixture: dim7-dispatched-count-sweep / PASS (3-burst)

**Scenario:** burst-log.md contains three burst entries (pass-72, pass-73, pass-74).
No Dim-7 cell contains an anachronism — each burst's dispatched count is consistent
with its own pass number, and sweep references only cite prior passes.

- pass-72 Dim-7: "Dispatched agent count for pass-72: 4 agents dispatched. Sweep of prior Dim-7 cells: pass-71 cited 3 dispatched — consistent."
- pass-73 Dim-7: "Dispatched agent count for pass-73: 5 agents dispatched. Sweep of prior Dim-7 cells: pass-72 cited 4 dispatched — consistent."
- pass-74 Dim-7: "Dispatched agent count for pass-74: 6 agents dispatched. Sweep of prior Dim-7 cells: pass-73 cited 5 dispatched — consistent."

**Why this fixture:** The original 2-burst PASS fixture (dim7-dispatched-count-sweep-pass/)
exercises the F-001 fix for bursts 1 and 2. This 3-burst fixture verifies that the
line-mapping formula in the script correctly identifies the 3rd burst's Dim-7 cell
regardless of absolute file line numbers. Specifically, it confirms that when
burst sections shift further down in the file (because prior burst content is longer),
the script still correctly identifies all three Dim-7 cells and passes when none
contain a forward reference.

**Expected exit code:** 0 (PASS)

**Script invocation:**
```
dim7-dispatched-count-sweep.sh burst-log.md
```
