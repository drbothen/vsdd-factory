# Fixture: propagation-completeness / PASS

**Scenario:** The derived value "D-453" appears at all prescribed sites
listed in sites.txt.

**Prescribed sites:**
- `.factory/STATE.md` — must contain "D-453"
- `.factory/INDEX.md` — must contain "D-453"

Both files contain the derived value.

**Expected exit code:** 0 (PASS)

**Script invocation:**
```
propagation-completeness.sh "D-453" sites.txt
```
(called from the fixture directory so relative paths in sites.txt resolve correctly)
