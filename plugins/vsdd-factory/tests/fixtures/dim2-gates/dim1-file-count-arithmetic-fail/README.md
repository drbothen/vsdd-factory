# Fixture: dim1-file-count-arithmetic / FAIL

**Scenario:** The Dim-1 headline says "5 unique files" but the comma-delimited
list on the following line contains only 3 file paths.

**Injected defect:** Headline count (5) does not match enumerated list count (3).

**Expected exit code:** 1 (FAIL)
**Expected output:** per-headline report showing expected 5, found 3.

**Script invocation:**
```
dim1-file-count-arithmetic.sh burst-log.md
```
