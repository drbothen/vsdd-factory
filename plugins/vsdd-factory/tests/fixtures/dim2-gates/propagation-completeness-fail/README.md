# Fixture: propagation-completeness / FAIL

**Scenario:** The derived value "D-453" appears in STATE.md but NOT in INDEX.md.

**Injected defect:** `.factory/INDEX.md` omits "D-453" — propagation is incomplete.

**Expected exit code:** 1 (FAIL)
**Expected output:** per-site report showing STATE.md PASS and INDEX.md FAIL.

**Script invocation:**
```
propagation-completeness.sh "D-453" sites.txt
```
