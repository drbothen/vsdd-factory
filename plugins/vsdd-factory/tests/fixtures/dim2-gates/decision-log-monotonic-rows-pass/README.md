# Fixture: decision-log-monotonic-rows / PASS

**Scenario:** decision-log.md contains D-NNN rows in strict ascending order:
D-450, D-451, D-452, D-452, D-453, D-454, D-454, D-454, D-454.

Note: sub-clause variants (D-452(a), D-452(b)) are treated as the same base
integer (452) for monotonic checking — the script extracts the integer from
`D-NNN` patterns. Multiple rows with the same base integer are allowed (non-strict
within the same D-NNN family); strict ascending applies across families.

**Expected exit code:** 0 (PASS)

**Script invocation:**
```
decision-log-monotonic-rows.sh decision-log.md
```
