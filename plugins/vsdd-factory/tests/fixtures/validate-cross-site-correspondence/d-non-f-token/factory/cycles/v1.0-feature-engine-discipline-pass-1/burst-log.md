# Burst Log — v1.0-feature-engine-discipline-pass-1

## Fix Burst Pass 28 (historical, should NOT be scanned)

This historical section has legitimate P-OLD-001, C-OLD-003.
Closes: P-OLD-001
Refs: C-OLD-003

---

## Fix Burst Pass 29 (LATEST SECTION — only this gets scanned)

This is the last H2 section. B01 is NOT an F- prefixed token — advisory expected.

**Closes:** B01, F-S2104-P29-H01
**Refs:** F-S2104-P29-H02

<!-- Fixture: AC-013 mutant — B01 in Closes is non-F- token, not in excluded namespace -->
<!-- Class D must produce an advisory for B01 -->
<!-- on_error=continue → advisory only (Class D never blocks per invariant 6) -->
<!-- Expected: exit 0 (advisory-only), [Class D] advisory in output -->
