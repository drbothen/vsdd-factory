---
document_type: story-index
version: "4.999"
last_amended: "2026-08-04 (v4.999)"
---

# STORY-INDEX

<!-- Fixture: PC40 volatile-input test -->
<!-- B1 = story frontmatter input-hash = "abc123" -->
<!-- B2 = catalog row input-hash = "xyz789"  (MISMATCH — deliberate) -->
<!-- B3 = blockquote S-21.07=xyz789 (MISMATCH — deliberate) -->
<!-- WITHOUT PC40: three-way comparison → B1≠B2 → exit 2 (BLOCK) -->
<!-- WITH PC40: .factory/STATE.md is volatile → advisory + Continue → exit 0 -->
<!-- BC-5.39.010 v1.6 PC40: volatile inputs suppress the three-way comparison -->

| S-21.07 | validate-cross-site-correspondence | E-21 | S-21.06 | [] | input-hash xyz789 |

> **E-21 S-21 delivery inputs:** S-21.07=xyz789.
