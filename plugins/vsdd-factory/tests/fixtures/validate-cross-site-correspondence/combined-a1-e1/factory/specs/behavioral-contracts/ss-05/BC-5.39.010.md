---
document_type: behavioral-contract
level: L3
version: "1.33"
status: active
bc_id: BC-5.39.010
section: "5.39"
last_amended: "2026-07-29 (v1.31) — test fixture: BOTH A1 stale AND E1 mismatch"
modified:
  - "2026-01-01"
  - "2026-07-29 (v1.31)"
---

# BC-5.39.010: test fixture for AC-018 combined violations

Fixture: BC version "1.33"; BC-INDEX has "v1.5" (A1 stale); last_amended "(v1.31)" (E1 mismatch).
Both Class A Arm1 AND Class E1 violations fire.
Expected: exit 2, single combined block containing both [Class A Arm1] and [Class E1].

Derives from BC-5.39.010 postcondition 23: "multiple violations combined into ONE block_with_fix".
Also §Canonical Test Vectors: "Combined A+E | BC 1.33; INDEX v1.5; last_amended (v1.31) | Block".
