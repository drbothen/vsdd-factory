# Fixture: trajectory-tail-cell-grep / FAIL

**Scenario:** STATE.md contains a trajectory tail of `→9→9→9` (LENGTH=3) at the
prescribed site, but the expected tail is `→9→9→9→9` (LENGTH=4, per D-433(e)+D-439(c)).

**Injected defect:** The Convergence Trajectory table and banner use `→9→9→9`
instead of the required `→9→9→9→9`.

**Expected exit code:** 1 (FAIL)
**Expected output:** cites the failing site and shows expected vs found value.
