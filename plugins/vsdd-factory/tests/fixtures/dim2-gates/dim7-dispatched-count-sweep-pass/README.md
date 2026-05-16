# Fixture: dim7-dispatched-count-sweep / PASS

**Scenario:** burst-log.md contains two burst entries. Neither Dim-7 cell
contains an anachronism (each burst's dispatched count is consistent with its
pass number and prior references).

**Expected exit code:** 0 (PASS)

**Script invocation:**
```
dim7-dispatched-count-sweep.sh burst-log.md
```
