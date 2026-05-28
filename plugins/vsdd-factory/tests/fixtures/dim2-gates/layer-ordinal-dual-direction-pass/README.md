# Fixture: layer-ordinal-dual-direction / PASS

**Scenario:** File(s) contain references to "42nd-layer" (positive form, N=42)
but NO references to "41st-layer" or "43rd-layer" (drift classes N-1 and N+1).

**Expected exit code:** 0 (PASS — no drift-class occurrences found)

Positive occurrences ("42nd-layer") are informational only and do not affect
the exit code. Only drift-class occurrences (N-1)th or (N+1)th cause failure.

**Script invocation:**
```
layer-ordinal-dual-direction.sh 42 lessons.md
```
