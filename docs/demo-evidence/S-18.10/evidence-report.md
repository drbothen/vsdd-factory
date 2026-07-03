# Demo Evidence — S-18.10: check-state-health CLAUDE_AUTOCOMPACT_PCT_OVERRIDE Verification

**Story:** S-18.10 (v1.4)
**Branch:** feature/S-18.10
**Helper:** `plugins/vsdd-factory/skills/check-state-health/lib/check-autocompact-setting.sh`
**Product type:** CLI bash helper (shell script)
**Recording method:** VHS terminal recordings (7 tapes; `.gif` + `.webm` per tape)
**BC gate:** BC-6.25.001 v1.1 (PC1–PC5, INV1–INV5, EC-012)
**ADR anchor:** ADR-026 §Decision 5 (canonical value 70; ceiling 80) / §F-11 (check-state-health mandate)

---

## Coverage Mapping

| Tape | GIF | WEBM | Tape Script | AC(s) / EC(s) | BC-6.25.001 Clause | What Is Demonstrated |
|------|-----|------|-------------|----------------|---------------------|----------------------|
| AC-003-pass-canonical | [gif](AC-003-pass-canonical.gif) | [webm](AC-003-pass-canonical.webm) | [tape](AC-003-pass-canonical.tape) | AC-003, AC-005, AC-007 | PC3, INV1, INV5 | Value 70 (canonical per ADR-026 §Decision 5) → PASS; value 80 (boundary ≤80) → PASS; `echo exit:0` confirms non-blocking |
| AC-002-advisory-exceeds-ceiling | [gif](AC-002-advisory-exceeds-ceiling.gif) | [webm](AC-002-advisory-exceeds-ceiling.webm) | [tape](AC-002-advisory-exceeds-ceiling.tape) | AC-002, AC-005, AC-007 | PC2, INV1, INV5 | Value 85 (>80 ceiling) → ADVISORY with "exceeds ADR-026 §Decision 5 ceiling of 80"; exit 0 |
| AC-001-advisory-key-absent | [gif](AC-001-advisory-key-absent.gif) | [webm](AC-001-advisory-key-absent.webm) | [tape](AC-001-advisory-key-absent.tape) | AC-001, AC-005, AC-007 | PC1, INV1, INV5 | `{"env":{}}` (env block present, key absent) → ADVISORY with full remediation hint; exit 0 |
| AC-008-advisory-ec012-out-of-range | [gif](AC-008-advisory-ec012-out-of-range.gif) | [webm](AC-008-advisory-ec012-out-of-range.webm) | [tape](AC-008-advisory-ec012-out-of-range.tape) | AC-008, AC-005, AC-007 | INV3 (EC-012), INV1, INV5 | Value 0 (≤0 out-of-range) → ADVISORY "not a valid compaction percentage (must be in range 1–100)"; message distinct from PC2 and PC1 advisories; exit 0 |
| AC-006-advisory-non-numeric | [gif](AC-006-advisory-non-numeric.gif) | [webm](AC-006-advisory-non-numeric.webm) | [tape](AC-006-advisory-non-numeric.tape) | AC-006, AC-005, AC-007 | INV3, INV1, INV5 | Value `"auto"` (non-numeric string) → ADVISORY with INV3 note "is not a valid integer; treating as absent"; exit 0 |
| EC-011-advisory-malformed-json | [gif](EC-011-advisory-malformed-json.gif) | [webm](EC-011-advisory-malformed-json.webm) | [tape](EC-011-advisory-malformed-json.tape) | AC-005, AC-007 | INV1, INV5 (EC-011) | Malformed JSON in settings.json → ADVISORY with jq parse error detail; exit 0; never blocks |
| AC-004-precedence-local-over-global | [gif](AC-004-precedence-local-over-global.gif) | [webm](AC-004-precedence-local-over-global.webm) | [tape](AC-004-precedence-local-over-global.tape) | AC-004, AC-005, AC-007 | PC4, INV2, INV1, INV5 | Case 1: local=85 wins over global=70 → ADVISORY (local takes precedence); Case 2: local absent, global=70 → PASS (fallback used) |
| AC-005-jq-absent-graceful | [gif](AC-005-jq-absent-graceful.gif) | [webm](AC-005-jq-absent-graceful.webm) | [tape](AC-005-jq-absent-graceful.tape) | AC-005 | INV1, PC5 | PATH restricted to `/bin` (no jq) → graceful ADVISORY "install with: brew install jq"; exits 0; never blocks |

---

## Acceptance Criteria Full Coverage

| AC | Title | Status | Tape(s) |
|----|-------|--------|---------|
| AC-001 | key absent → ADVISORY with remediation hint | COVERED | AC-001-advisory-key-absent |
| AC-002 | value > 80 → ADVISORY | COVERED | AC-002-advisory-exceeds-ceiling |
| AC-003 | value ≤ 80 → PASS (including value 70 canonical and value 80 boundary) | COVERED | AC-003-pass-canonical |
| AC-004 | project-local precedence over global fallback | COVERED | AC-004-precedence-local-over-global |
| AC-005 | advisory-only; exit 0 in all cases; never blocks | COVERED (all tapes) | all tapes show exit 0 |
| AC-006 | non-numeric/empty value → ADVISORY with INV3 note | COVERED | AC-006-advisory-non-numeric |
| AC-007 | row always emitted | COVERED (all tapes) | all tapes show check row |
| AC-008 | value ≤ 0 → distinct ADVISORY (EC-012) | COVERED | AC-008-advisory-ec012-out-of-range |

---

## Edge Case Coverage

| EC | Description | Tape | Result Shown |
|----|-------------|------|--------------|
| EC-001 | value "70" → PASS (canonical) | AC-003-pass-canonical | PASS |
| EC-002 | env block present, key absent → ADVISORY PC1 | AC-001-advisory-key-absent | ADVISORY |
| EC-003 | value "85" → ADVISORY PC2 | AC-002-advisory-exceeds-ceiling | ADVISORY |
| EC-004 | value "80" → PASS boundary | AC-003-pass-canonical | PASS |
| EC-005 | local absent, global value "70" → PASS fallback | AC-004-precedence-local-over-global | PASS |
| EC-009 | value "auto" → ADVISORY INV3 note | AC-006-advisory-non-numeric | ADVISORY |
| EC-010 | both present: local=85 wins → ADVISORY | AC-004-precedence-local-over-global | ADVISORY |
| EC-011 | malformed JSON → ADVISORY parse error | EC-011-advisory-malformed-json | ADVISORY |
| EC-012 | value 0 (≤0 out-of-range) → distinct ADVISORY | AC-008-advisory-ec012-out-of-range | ADVISORY |

---

## Exit Code Invariant (AC-005 / BC-6.25.001 INV1)

Every tape includes `echo exit:0` after the helper invocation, confirming exit code 0 across all paths (PASS, ADVISORY, malformed JSON, jq-absent). The helper never exits non-zero. This satisfies BC-6.25.001 INV1 (advisory-only; never blocks any downstream operation).

---

## Fixture Setup

Fixture `settings.json` files were pre-created in isolated `/tmp/s1810-demo-*/` directories using a sandboxed `HOME=/tmp/nonexistent` to prevent any real `~/.claude/settings.json` from being read during recording. The AC-004 precedence test used separate `/tmp/s1810-demo-prec/proj/.claude/` (local=85) and `/tmp/s1810-demo-prec/home/.claude/` (global=70) directories. The jq-absent test used `PATH=/bin` to strip `/usr/bin` and `/opt/homebrew/bin` from the PATH, completely hiding the jq binary.

---

## Spec References

| Reference | Clause | Coverage |
|-----------|--------|----------|
| BC-6.25.001 | PC1 | AC-001 |
| BC-6.25.001 | PC2 | AC-002 |
| BC-6.25.001 | PC3 | AC-003 |
| BC-6.25.001 | PC4 | AC-004 |
| BC-6.25.001 | PC5 / INV1 | AC-005 (all tapes) |
| BC-6.25.001 | INV2 | AC-004 |
| BC-6.25.001 | INV3 | AC-006, AC-008 |
| BC-6.25.001 | INV5 | AC-007 (all tapes) |
| BC-6.25.001 | EC-012 | AC-008 |
| ADR-026 §Decision 5 | canonical value 70; ceiling 80 | AC-002, AC-003 |
| ADR-026 §F-11 | check-state-health must verify this key | AC-001, AC-007 |
