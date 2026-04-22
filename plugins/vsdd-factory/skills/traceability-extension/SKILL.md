---
name: traceability-extension
description: >
  Rules for extending the VSDD traceability chain when adding features
  incrementally. New links are appended, never replaced.
---

# Traceability Chain Extension

## The VSDD Traceability Chain

Every line of code in a VSDD project traces through a complete bidirectional chain
spanning the 4-level specification hierarchy:

```
L1 Brief Section ↔ L2 CAP-NNN ↔ L3 BC-S.SS.NNN ↔ L4 VP-NNN
    ↔ Story ↔ AC-NNN ↔ Test Case ↔ Implementation ↔ Adversarial Review ↔ Formal Proof
```

### Bidirectional Traceability (L1↔L4)

The chain is navigable in both directions:
- **Forward (L1→Proof):** From brief section, trace through L2 capability, L3 contract,
  L4 property, story, test, implementation, to proof status
- **Reverse (Proof→L1):** From any proof or VP, trace back through the hierarchy to
  the originating L1 brief section

When extending the chain, ensure both forward and reverse links are maintained.

### Architecture Section Traceability (DF-021)

When tracing architecture references, use specific section file paths instead of
monolithic `architecture/ARCH-INDEX.md`:
- Implementation traces -> `architecture/module-decomposition.md`, `architecture/api-surface.md`
- Verification traces -> `architecture/verification-architecture/ARCH-INDEX.md`, `architecture/purity-boundary-map.md`
- Story traces -> `architecture/module-decomposition.md`, `architecture/dependency-graph.md`

In Feature Mode, new features EXTEND this chain. They never replace or
overwrite existing chain links.

## Rules for Extension

### Rule 1: New Requirements Get New IDs

Continue the existing ID sequence. If the last functional requirement is FR-024,
new requirements start at FR-025.

Never reuse or reassign existing IDs. An ID is permanent once assigned, even if
the requirement is later removed (mark as DEPRECATED, do not delete).

### Rule 2: New Stories Link to Both New and Existing Requirements

A new story may satisfy:
- New requirements only (e.g., STORY-008 implements FR-025)
- A combination of new and existing requirements (e.g., STORY-010 implements
  FR-026 and extends FR-003)

When a new story extends an existing requirement, the traceability chain records
BOTH links:
```
STORY-010:
  implements: [FR-026]     # new requirement
  extends: [FR-003]        # existing requirement (adds capability)
```

### Rule 3: New Tests Trace to New Stories

Every new test must reference the story it validates:
```
# Test: test_notification_delivery
# Story: STORY-008
# Requirements: FR-025
# Verification Property: VP-014
```

Test file headers or inline comments carry this traceability metadata.

### Rule 4: Cross-References Link Features

When a new feature interacts with an existing feature, record the cross-reference:

```markdown
## Cross-References

| New Feature Element | Existing Feature Element | Relationship |
|--------------------|-----------------------|-------------|
| FR-025 (notifications) | FR-001 (authentication) | depends_on: notifications require authenticated user |
| STORY-010 (integrate) | STORY-003 (auth service) | extends: adds notification hooks to auth flow |
| VP-014 (delivery guarantee) | VP-001 (auth correctness) | assumes: auth is correct per VP-001 |
```

### Rule 5: Chain Links Are Append-Only

The traceability chain file (`.factory/cycles/**/convergence/traceability-chain.md`)
is append-only during Feature Mode:

```markdown
# Existing chain (DO NOT MODIFY):
FR-001 -> VP-001 -> test_auth -> src/auth.rs -> ADV-PASS-3 -> KANI-auth-PASS
FR-002 -> VP-002 -> test_session -> src/session.rs -> ADV-PASS-2 -> KANI-session-PASS

# Feature: Notification System (appended YYYY-MM-DD, spec v1.3.0):
FR-025 -> VP-014 -> test_notification_delivery -> src/notification_service.rs -> ADV-PASS-1 -> KANI-notification-PASS
FR-026 -> VP-015 -> test_notification_preferences -> src/notification_preferences.rs -> ADV-PASS-1 -> KANI-preferences-PASS

# Cross-references:
FR-025 depends_on FR-001
STORY-010 extends STORY-003
```

### Rule 6: Verify Chain Completeness

Before Phase F7 convergence, verify that every new requirement has a complete chain
spanning L1→L4:

| L1 Section | L2 CAP | L3 BC | Requirement | VP | Test | Impl | Adversarial | Proof |
|------------|--------|-------|-------------|-----|------|------|-------------|-------|
| Notifications | CAP-012 | BC-3.01.001 | FR-025 | VP-014 | test_BC_3_01_001_delivery | notification_service.rs | ADV-PASS-1 | KANI-notification-PASS |

Any gap in the chain means convergence is not achieved. The missing link must be
filled before Phase F7 can complete.

### Rule 7: Deprecated Requirements Stay in Chain

If a future feature deprecates an existing requirement:
- Mark the requirement as DEPRECATED in the PRD (do not delete)
- Mark the chain link as DEPRECATED (do not remove)
- Tests for deprecated requirements can be removed, but the chain link remains
  as historical record

```
FR-005 -> VP-005 -> ... -> DEPRECATED (removed in spec v2.0.0, replaced by FR-030)
```

## Chain Verification Command

The orchestrator can verify chain completeness with:

```bash
# Count requirements without complete chains:
# (pseudocode -- actual implementation depends on chain file format)
grep "^FR-" .factory/cycles/**/convergence/traceability-chain.md | while read -r line; do
  # Verify each chain has all 7 links
  link_count=$(echo "$line" | tr '->' '\n' | grep -c '[A-Z]')
  if [[ "$link_count" -lt 7 ]]; then
    echo "INCOMPLETE CHAIN: $line"
  fi
done
```

## Applicability

Reference document -- rules for extending traceability. Consumed by story-writer and spec-steward.
