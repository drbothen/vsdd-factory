# Demo Evidence — S-18.08: Pure-Parse Invariant Consistency Gate

**Story:** S-18.08 — O-P8-002 Pure-Parse Invariant Consistency Gate  
**Version:** v1.9  
**Gate file:** `plugins/vsdd-factory/tests/pure-parse-invariant-gate.bats`  
**Evidence date:** 2026-06-27

---

## What the Gate Enforces

BCs (behavioral contracts) that declare a pure-parse invariant in their `## Invariants` section must not contain a load-bearing substrate-read verb (`reads`, `loads`, `fetches`, `derives`, `accesses`, `retrieves`, `opens`, `parses`) collocated with a substrate identifier (`sprint-state.yaml`, `git-log`, `git-cat-file`) in their normative sections. A "substrate read" in a pure-parse BC constitutes an invariant contradiction — the BC claims no filesystem or git side effects but its prose describes performing one. The gate uses the three-layer detection pipeline mandated by ADR-026 §Decision 14: (1) awk normative-section extraction (scopes to `## Preconditions` through the first non-normative heading, structurally excluding HANDOFF.md payload descriptions and traceability prose); (2) verb+substrate collocation grep (distinguishes active reads from affirmations); (3) negation/comment exclusion grep (strips prohibition statements such as "does not read sprint-state.yaml" which would be false positives). Associated VP files (VP-081, VP-083, VP-091) are scanned whole-file through layers 2 and 3 only (no section extraction needed since VPs are fully normative). A positive-control test (AC-005) and a verb-recall-guard test (AC-006) ensure the verb pattern cannot silently regress to under-matching.

---

## 1. Gate Running Green — Verbatim bats Output

Command:

```
bats plugins/vsdd-factory/tests/pure-parse-invariant-gate.bats
```

Output:

```
1..6
ok 1 test_bc_4_14_001_pure_parse_invariant_zero_verb_substrate_hits_normative
ok 2 test_bc_4_15_001_pure_parse_invariant_zero_verb_substrate_hits_normative
ok 3 test_all_pure_parse_bcs_dynamic_discovery_zero_verb_substrate_hits
ok 4 test_vp_083_081_091_zero_verb_substrate_hits_whole_file
ok 5 test_positive_control_genuine_substrate_read_yields_exactly_one_hit
ok 6 test_positive_control_opens_parses_verbs_detected
```

6 tests, 0 failures.

---

## 2. Per-AC Evidence

### AC-001 — BC-4.14.001 normative-section scan: 0 hits

Command run:

```bash
BC_FILE=".factory/specs/behavioral-contracts/ss-04/BC-4.14.001.md"
HITS=$(awk '/^## Preconditions$/{ found=1 }
            found && /^## / && !/^## (Preconditions|Postconditions|Invariants|Edge Cases|Error Paths|Canonical Test Vectors)$/{ exit }
            found{ print }' "$BC_FILE" \
  | grep -Ei "(reads?|loads?|fetches|derives?|access(es)?|retrieves?|opens?|parses?)[[:space:]]+.{0,80}(sprint-state\.yaml|git-log|git-cat-file)" \
  | grep -Eiv "no |not |NOT |without|never|does not|MUST NOT|is NOT|cannot|do NOT|only from|exclusively" \
  | wc -l) || true
echo "HITS=$HITS"
```

Captured output:

```
HITS=       0
```

Result: PASS — 0 verb+substrate collocation hits in BC-4.14.001 normative sections.

---

### AC-002 — BC-4.15.001 normative-section scan: 0 hits

Command run:

```bash
BC_FILE=".factory/specs/behavioral-contracts/ss-04/BC-4.15.001.md"
HITS=$(awk '/^## Preconditions$/{ found=1 }
            found && /^## / && !/^## (Preconditions|Postconditions|Invariants|Edge Cases|Error Paths|Canonical Test Vectors)$/{ exit }
            found{ print }' "$BC_FILE" \
  | grep -Ei "(reads?|loads?|fetches|derives?|access(es)?|retrieves?|opens?|parses?)[[:space:]]+.{0,80}(sprint-state\.yaml|git-log|git-cat-file)" \
  | grep -Eiv "no |not |NOT |without|never|does not|MUST NOT|is NOT|cannot|do NOT|only from|exclusively" \
  | wc -l) || true
echo "HITS=$HITS"
```

Captured output:

```
HITS=       0
```

Result: PASS — 0 verb+substrate collocation hits in BC-4.15.001 normative sections.

---

### AC-003 — Dynamic discovery: exactly BC-4.14.001 + BC-4.15.001, 0 hits each

Discovery command (Invariants-section anchored, tree-wide):

```bash
BC_DIR=".factory/specs/behavioral-contracts"
while IFS= read -r -d '' f; do
  MATCH=$(awk '/^## Invariants$/{ found=1; next } found && /^## /{ exit } found{ print }' "$f" \
    | grep -i "pure-parse") || true
  [ -n "$MATCH" ] && echo "$f"
done < <(find "$BC_DIR" -name "BC-*.md" -print0)
```

Discovery output (2 files resolved):

```
/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.15.001.md
/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.14.001.md
```

Per-file normative-section scan:

```
BC-4.15.001.md: HITS=       0
BC-4.14.001.md: HITS=       0
```

Result: PASS — discovery guard satisfied (2 files found, not 0); scannability guard satisfied (both files have `## Preconditions`); 0 hits per file.

---

### AC-004 — VP-083, VP-081, VP-091 whole-file scans: 0 hits each

Command run:

```bash
for VP_FILE in \
  ".factory/specs/verification-properties/VP-083.md" \
  ".factory/specs/verification-properties/VP-081.md" \
  ".factory/specs/verification-properties/VP-091.md"; do
  HITS=$(grep -Ei "(reads?|loads?|fetches|derives?|access(es)?|retrieves?|opens?|parses?)[[:space:]]+.{0,80}(sprint-state\.yaml|git-log|git-cat-file|factory-artifacts)" "$VP_FILE" \
    | grep -Eiv "no |not |NOT |without|never|does not|MUST NOT|is NOT|cannot|do NOT|only from|exclusively" \
    | grep -Ev "^[[:space:]]*//" \
    | wc -l) || true
  echo "$(basename "$VP_FILE"): HITS=$HITS"
done
```

Captured output:

```
VP-083.md: HITS=       0
VP-081.md: HITS=       0
VP-091.md: HITS=       0
```

Result: PASS — all three VP files exist; 0 verb+substrate collocation hits per file.

---

### AC-005 — Positive control: "reads ... sprint-state.yaml" yields exactly 1 hit

Command run:

```bash
HITS=$(echo "The gate reads wave context directly from sprint-state.yaml before parsing the payload." \
  | grep -Ei "(reads?|loads?|fetches|derives?|access(es)?|retrieves?|opens?|parses?)[[:space:]]+.{0,80}(sprint-state\.yaml|git-log|git-cat-file)" \
  | grep -Eiv "no |not |NOT |without|never|does not|MUST NOT|is NOT|cannot|do NOT|only from|exclusively" \
  | wc -l) || true
echo "HITS=$HITS"
```

Captured output:

```
HITS=       1
```

Result: PASS — verb pattern is non-tautological; a genuine substrate-read sentence traverses both filter layers and produces exactly 1 hit. If this were 0, the verb pattern would be over-restrictive and AC-001 through AC-004 results could not be trusted.

---

### AC-006 — Recall-guard: "opens ... / parses git-log" yields >= 1 hit

Command run:

```bash
HITS=$(echo "The gate opens sprint-state.yaml and parses git-log output to derive wave context." \
  | grep -Ei "(reads?|loads?|fetches|derives?|access(es)?|retrieves?|opens?|parses?)[[:space:]]+.{0,80}(sprint-state\.yaml|git-log|git-cat-file)" \
  | grep -Eiv "no |not |NOT |without|never|does not|MUST NOT|is NOT|cannot|do NOT|only from|exclusively" \
  | wc -l) || true
echo "HITS=$HITS"
```

Captured output:

```
HITS=       1
```

Result: PASS — the `opens?` and `parses?` verbs (added in v1.8 per F-P2-001 ADR-026 §Decision 14 recall-gap fix) are detected. A regression to the 6-verb set would produce 0 hits here, immediately surfacing the recall gap before any other AC results are trusted.

---

## 3. Coverage Map

| AC | Test name | Status |
|----|-----------|--------|
| AC-001 | `test_bc_4_14_001_pure_parse_invariant_zero_verb_substrate_hits_normative` | ok 1 — PASS |
| AC-002 | `test_bc_4_15_001_pure_parse_invariant_zero_verb_substrate_hits_normative` | ok 2 — PASS |
| AC-003 | `test_all_pure_parse_bcs_dynamic_discovery_zero_verb_substrate_hits` | ok 3 — PASS |
| AC-004 | `test_vp_083_081_091_zero_verb_substrate_hits_whole_file` | ok 4 — PASS |
| AC-005 | `test_positive_control_genuine_substrate_read_yields_exactly_one_hit` | ok 5 — PASS |
| AC-006 | `test_positive_control_opens_parses_verbs_detected` | ok 6 — PASS |

All 6 tests GREEN. Gate deliverable: `plugins/vsdd-factory/tests/pure-parse-invariant-gate.bats`.
