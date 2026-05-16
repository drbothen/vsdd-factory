# Fixture: meta-level-ack-grep / PASS

**Scenario:** At least one of the 4 canonical factory documents contains the
string "META-LEVEL-24 CANDIDATE CONFIRMED".

**Acknowledgment present in:** burst-log.md (2 occurrences)
**Acknowledgment absent from:** lessons.md, decision-log.md, state.md

Total count across all 4 files is >= 1, so the gate passes.

**Expected exit code:** 0 (PASS)

**Script invocation:**
```
meta-level-ack-grep.sh 24 burst-log.md lessons.md decision-log.md state.md
```

The script runs `grep -c "META-LEVEL-24 CANDIDATE CONFIRMED"` against each
file and reports per-file counts. Exits 0 if total >= 1.
