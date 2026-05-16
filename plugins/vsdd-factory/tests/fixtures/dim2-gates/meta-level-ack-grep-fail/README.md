# Fixture: meta-level-ack-grep / FAIL

**Scenario:** NONE of the 4 canonical factory documents contains the string
"META-LEVEL-24 CANDIDATE CONFIRMED".

**Injected defect:** All 4 files (burst-log.md, lessons.md, decision-log.md,
state.md) have been written without any META-LEVEL-24 acknowledgment string.

Total count across all 4 files is 0 (< 1), so the gate fails.

**Expected exit code:** 1 (FAIL)
**Expected output:** per-file counts (all 0) and total count 0.

**Script invocation:**
```
meta-level-ack-grep.sh 24 burst-log.md lessons.md decision-log.md state.md
```
