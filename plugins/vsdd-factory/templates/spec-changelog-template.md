---
document_type: spec-changelog
project: "[Project Name]"
---

# Spec Changelog

Track all spec version changes. Most recent version first.

## [X.Y.Z] - YYYY-MM-DD

### Type: MAJOR / MINOR / PATCH

### Summary

[One-sentence summary of what changed and why.]

### New Requirements

| ID | Description |
|----|-------------|
| FR-XXX | [Requirement summary] |
| NFR-XXX | [Requirement summary] |

### Modified Requirements

| ID | Previous | Updated | Rationale |
|----|----------|---------|-----------|
| FR-YYY | [Old text or summary] | [New text or summary] | [Why it changed] |

### Removed Requirements

| ID | Description | Rationale |
|----|-------------|-----------|
| FR-ZZZ | [What was removed] | [Why it was removed] |

### New Verification Properties

| ID | Description | Proof Strategy |
|----|-------------|---------------|
| VP-XXX | [Property description] | [Kani / proptest / fuzz / manual] |

### Architecture Changes

- [Component added/modified/removed and brief rationale]

### Impact Assessment

- **Affected stories:** [List story IDs that need review]
- **Affected tests:** [List test files/modules that need updates]
- **Migration needed:** YES / NO
- **Migration notes:** [If yes, what must change in existing code]

### Feature Request Link

- [Link to the feature request that triggered this version bump]

---

<!-- Copy the section above for each new version entry. Keep reverse chronological order. -->

## [1.0.0] - YYYY-MM-DD

### Type: MAJOR

### Summary

Initial spec release. Baseline for all subsequent versions.

### New Requirements

[All initial requirements listed]

### Impact Assessment

- **Affected stories:** None (initial release)
- **Migration needed:** NO
