---
name: step-c-collect-findings
description: Capture adversary findings and persist them via state-manager as sharded per-finding files.
---

# Step C: Collect Findings

> **Shared context:** Read `./_shared-context.md` before executing this step.

The adversary returns findings as chat text (it has read-only tools). The orchestrator must capture and persist them.

## Procedure

1. **Capture the adversary's full output** verbatim. Do not summarize, filter, or editorialize.

2. **Run the filename collision guard** (see shared context):
   - Compute target path: `.factory/specs/adversarial-reviews/pass-<N>.md`
   - If the target exists with different content, refuse the write

3. **Dispatch state-manager** to write the findings:
   - **Per-finding files:** `.factory/specs/adversarial-reviews/ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>.md`
     - Each finding is its own file, following `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-finding-template.md`
     - Frontmatter includes: finding_id, pass, severity, category, status, phase, traces_to, source_artifacts
   - **Pass index:** `.factory/specs/adversarial-reviews/ADV-P<N>-INDEX.md`
     - Following `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-review-index-template.md`
     - Contains finding catalog table, dependency graph, category groups
   - **Cross-pass tracker:** `.factory/specs/adversarial-reviews/FINDINGS.md`
     - Aggregates findings across all passes for this cycle

4. **Verify persistence** — read back each written file to confirm content matches adversary output.

## Artifacts

| Artifact | Path | Format |
|----------|------|--------|
| Per-finding files | `.factory/specs/adversarial-reviews/ADV-<CYCLE>-P<N>-<SEV>-<SEQ>.md` | DF-021 |
| Pass index | `.factory/specs/adversarial-reviews/ADV-P<N>-INDEX.md` | DF-021 |
| Cross-pass tracker | `.factory/specs/adversarial-reviews/FINDINGS.md` | DF-021 |

## Success Criteria

- All adversary findings persisted as individual files
- Pass index created with complete finding catalog
- Cross-pass tracker updated with this pass's findings
- No findings lost between adversary output and written files
