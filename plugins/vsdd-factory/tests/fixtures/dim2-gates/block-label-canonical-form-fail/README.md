# Fixture: block-label-canonical-form / FAIL

**Scenario:** burst-log.md is missing the "Dim-7" canonical block label.

**Injected defect:** The `**Dim-7 (dispatched-count sweep):**` block is absent.
All other 8 of the 9 D-444(c) labels are present.

**Expected exit code:** 1 (FAIL)
**Expected output:** lists the missing label "Dim-7 (dispatched-count sweep)"

**Script invocation:**
```
block-label-canonical-form.sh burst-log.md
```
