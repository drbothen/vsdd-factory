# Adversarial Review — S-5.05 Pass 2 (Wave 14)

**Reviewer:** adversary
**Artifact:** .factory/stories/S-5.05-migration-guide.md v1.5 (factory-artifacts commit cbb6b3b)
**Date:** 2026-04-29
**Convergence clock entering pass-2:** 0_of_3 (pass-1 SUBSTANTIVE)

## Pass-1 Fix Verification
All 10 pass-1 findings landed cleanly per pass-2 inspection.

## High Findings

### F-S5.05-P02-001 — HIGH — Task 11 5th issue ("Windows git-bash") collides with skeleton hint #3
Skeleton at docs/guide/migrating-from-0.79.md:80-83 hint #3 = "legacy-bash-adapter 'command not found' (git-bash missing on Windows?)". Implementer following Task 11 verbatim could ship 4 effective unique issues and fail AC-7 ≥5.
Policy: POLICY 5.

### F-S5.05-P02-002 — HIGH — Status banner removal not covered by any AC or Task
Skeleton lines 3-7 contain a `> **Status:** skeleton — filled in by S-5.5...` blockquote. AC-1's grep target `TODO(S-5.5)` wouldn't catch a blockquote.
Policy: POLICY 5, POLICY 4.

### F-S5.05-P02-003 — HIGH — Task ordering conflict S-5.05 Task 15 ↔ S-5.06 Task 9 both edit README L264
Both stories grep-target literal `"skeleton, finalized in S-5.5"` with different replacements. First-shipper invalidates second's grep.
Policy: POLICY 8 (extension to multi-story content ownership), POLICY 5.

### F-S5.05-P02-004 — HIGH — STORY-INDEX line 21 "16 findings closed" disagrees with story changelogs (12+10=22)
Pass-1 reviews tally: S-5.05 10 findings + S-5.06 10 findings = 20 substantive. Story changelogs: S-5.05 v1.5 says 12; S-5.06 v1.5 says 10. STORY-INDEX claim of 16 is unattributable.
Policy: POLICY 3.

## Medium Findings

### F-S5.05-P02-005 — MED — Phantom "register if not present" qualifier on BC-8.31.008 reference
### F-S5.05-P02-006 — MED — PRD §2.8 + capabilities.md still cite "BC-8.31.001-007" (under-counts after BC-8.31.008 added)
### F-S5.05-P02-007 — MED — docs/guide/v1.0-index.md retains S-5.7 legacy short-form (lines 6+40); no story owns the fix
### F-S5.05-P02-008 — MED — STORY-INDEX `done` status not in declared enum (4 E-4 stories use it)

## Low Findings

### F-S5.05-P02-009 — LOW — Status banner skeleton hint says "filled in by S-5.5" (covered by F-002 fix)
### F-S5.05-P02-010 — LOW — PRD §14 severely stale (pre-existing; deferred)
### F-S5.05-P02-011 — LOW [process-gap] — F-204 sanctioning artifact citation indirect
### F-S5.05-P02-012 — LOW — "Blocked by:" sentence parse-confusing

## Verdict
`VERDICT: SUBSTANTIVE` — fix burst required.
`CRIT=0 HIGH=4 MED=4 LOW=4 NIT=0`
Convergence clock: remains 0_of_3.
