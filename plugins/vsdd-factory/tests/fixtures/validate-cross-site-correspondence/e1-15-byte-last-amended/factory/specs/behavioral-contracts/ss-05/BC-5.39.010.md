---
document_type: behavioral-contract
bc_id: BC-5.39.010
version: "2"
status: draft
last_amended: "2026-07-30 (v2)"
modified:
  - "2026-06-01"
  - "2026-07-30 (v2)"
---

# BC-5.39.010: validate-cross-site-correspondence

Test fixture for T-045: 15-byte last_amended CONTROL.

`last_amended: "2026-07-30 (v2)"` is 15 bytes (no sub-version like "v2.1").
BC-5.39.010 v1.3 §E1: accepts YYYY-MM-DD (vN) format where N can be single-digit.
extract_last_amended_outer_version should return "2" for this string.
Current code: `if len < 17 { return None }` → 15 < 17 → returns None → advisory.
"2" (BC version) vs None (extracted) → advisory fires for unparseable format.
Expected: exit 0 with NO advisory (format is valid and versions match).
Test should assert both: exit 0 AND no "unparseable" in output.
