# Lessons Log — v1.0-brownfield-backfill

## L-EDP1-055 — Cross-site staleness is advisory only in Phase 1

Category: process
Date: 2026-04-22

Lesson: in Phase 1, a correctly-formatted cite ID that references a nonexistent D-NNN
(e.g., D-999) produces only an advisory log_warn, NOT a block. Cross-site staleness
detection is Phase 2 scope per BC-5.39.007 Phase 1/2 boundary table.

**Closes:** D-999
