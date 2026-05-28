# Fixture: decision-log-monotonic-rows / FAIL

**Scenario:** decision-log.md contains D-NNN rows that are NOT in strict
ascending order — an inversion is present.

**Injected defect:** Row order is D-450, D-453, D-452, D-454.
The inversion is between D-453 and D-452: D-453 appears before D-452, which
is a descending step (453 > 452). This violates strict monotonic ascending order.

**Expected exit code:** 1 (FAIL)
**Expected output:** cites the first inversion pair (D-453 before D-452).

**Script invocation:**
```
decision-log-monotonic-rows.sh decision-log.md
```
