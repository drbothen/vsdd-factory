# Fixture: dim7-dispatched-count-sweep / FAIL (3-burst)

**Scenario:** burst-log.md contains three burst entries (pass-72, pass-73, pass-74).
The third burst's Dim-7 cell contains an anachronism: pass-74 references "pass-76"
dispatched count — a forward reference to a pass that doesn't exist yet at pass-74
time.

**Injected defect:** In the pass-74 burst's Dim-7 section (the third burst section),
the text reads "Dispatched agent count for pass-76: 6 agents dispatched" — a forward
reference where 76 > 74. The first two burst sections (pass-72, pass-73) are clean.

**Why this fixture:** The original 2-burst FAIL fixture (dim7-dispatched-count-sweep-fail/)
injects the anachronism in the first burst. This 3-burst fixture verifies that the
anachronism is detected in the THIRD burst section — exercising the script's sweep logic
over the complete section sequence (not just the first section). A pre-fix script that
only scanned the first section would emit a false-green on this fixture; the post-fix
script correctly sweeps all sections and reports FAIL.

**Expected exit code:** 1 (FAIL)

**Expected output:** contains "FAIL:" and references the offending line (the "pass-76"
anachronism in the pass-74 burst's Dim-7 cell, at approximately line 92 of the fixture
burst-log.md).

**Script invocation:**
```
dim7-dispatched-count-sweep.sh burst-log.md
```
