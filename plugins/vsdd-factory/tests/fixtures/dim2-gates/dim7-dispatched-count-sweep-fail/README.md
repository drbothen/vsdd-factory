# Fixture: dim7-dispatched-count-sweep / FAIL

**Scenario:** burst-log.md has a Dim-7 cell containing an anachronism: the
pass-72 burst's Dim-7 cell references "pass-74" dispatched count.

**Injected defect:** In the pass-72 burst's Dim-7 section, the text reads
"Dispatched agent count for pass-74" — a forward reference to a pass that
doesn't exist yet at pass-72 time. This is the anachronism pattern.

**Expected exit code:** 1 (FAIL)
**Expected output:** lists the offending line(s) with file location.

**Script invocation:**
```
dim7-dispatched-count-sweep.sh burst-log.md
```
