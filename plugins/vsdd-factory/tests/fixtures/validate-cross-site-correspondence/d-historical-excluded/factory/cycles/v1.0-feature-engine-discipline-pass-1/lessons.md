# Lessons — v1.0-feature-engine-discipline-pass-1

L-EDP1-001:
**What happened:** Historical lesson about P45-001.
**Closes:** P45-001
**Refs:** P-OLD-002, C-OLD-003

This historical block is NOT the latest L-EDP1 anchor block.
Class D extracts only the LAST L-EDP1-NNN block (positional anchor, not pattern match).
P45-001 here must NOT trigger an advisory.

---

L-EDP1-062:
**What happened:** Latest lesson block with only F- tokens.
**Closes:** F-S2104-P30-H01
**Refs:** F-S2104-P30-H02

This is the LAST L-EDP1 anchor block. Only F- tokens → no advisory.

<!-- Fixture: AC-014 historical exclusion -->
<!-- Class D scans only L-EDP1-062 (last anchor block, positional) -->
<!-- P45-001 is in L-EDP1-001 (historical) — must NOT be flagged -->
<!-- Expected: exit 0, no [Class D] advisory -->
