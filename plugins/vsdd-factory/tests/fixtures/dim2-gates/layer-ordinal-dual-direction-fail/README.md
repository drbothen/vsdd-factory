# Fixture: layer-ordinal-dual-direction / FAIL

**Scenario:** File(s) contain references to "42nd-layer" (positive form, N=42)
but ALSO contain a reference to "43rd-layer" (N+1 drift class).

**Injected defect:** The word "43rd-layer" appears in lessons.md alongside the
correct "42nd-layer" references. The 43rd-layer reference is the drift pattern
that the script must detect.

**Expected exit code:** 1 (FAIL)
**Expected output:** cites the offending line containing "43rd-layer" with file and line number.

**Script invocation:**
```
layer-ordinal-dual-direction.sh 42 lessons.md
```
