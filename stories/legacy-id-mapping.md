---
document_type: legacy-id-mapping
version: "1.0"
producer: story-writer
timestamp: 2026-04-25T00:00:00
phase: 1.8
total_rows: 41
---

# Legacy ID Mapping: S-N.M → S-N.MM

Translation table from manual-VSDD-era single-digit story IDs (`S-N.M`)
to canonical prism-style zero-padded IDs (`S-N.MM`).

Generated during Phase 1.8 migration. Story bodies preserved verbatim.
Only frontmatter updated with new IDs, epic assignments, and subsystem lists.

## Mapping Table

| Old ID | New ID | Description | Epic | CHANGELOG ref | Status |
|--------|--------|-------------|------|---------------|--------|
| S-0.1 | S-0.01 | bump-version-prerelease | E-0 | v1.0.0-beta.1 | merged |
| S-0.2 | S-0.02 | release-workflow-prerelease | E-0 | v1.0.0-beta.1 | merged |
| S-0.3 | S-0.03 | activation-skill-platform-detection | E-0 | v1.0.0-beta.1 | merged |
| S-0.4 | S-0.04 | hooks-json-template-generation | E-0 | v1.0.0-beta.1 | merged |
| S-0.5 | S-0.05 | docs-scaffolding | E-0 | v1.0.0-beta.1 | merged |
| S-1.1 | S-1.01 | cargo-workspace-setup | E-1 | v1.0.0-beta.1 | merged |
| S-1.2 | S-1.02 | dispatcher-core | E-1 | v1.0.0-beta.1 | merged |
| S-1.3 | S-1.03 | hook-sdk-crate | E-1 | v1.0.0-beta.1 | merged |
| S-1.4 | S-1.04 | host-function-surface | E-1 | v1.0.0-beta.1 | merged |
| S-1.5 | S-1.05 | wasmtime-integration | E-1 | v1.0.0-beta.1 | merged |
| S-1.6 | S-1.06 | tokio-parallel-tier-execution | E-1 | v1.0.0-beta.1 | merged |
| S-1.7 | S-1.07 | dispatcher-internal-log | E-1 | v1.0.0-beta.1 | merged |
| S-1.8 | S-1.08 | sink-file-driver | E-1 | v1.0.0-beta.1 | merged |
| S-1.9 | S-1.09 | sink-otel-grpc-driver | E-1 | v1.0.0-beta.1 | merged |
| S-2.1 | S-2.01 | legacy-bash-adapter | E-2 | v1.0.0-beta.1 | merged |
| S-2.2 | S-2.02 | registry-toml-generation | E-2 | v1.0.0-beta.1 | merged |
| S-2.3 | S-2.03 | ci-cross-platform-matrix | E-2 | v1.0.0-beta.1 | merged |
| S-2.4 | S-2.04 | release-binary-commit | E-2 | v1.0.0-beta.1 | merged |
| S-2.5 | S-2.05 | hook-sdk-publish | E-2 | v1.0.0-beta.1 (dry-run only) | partial |
| S-2.6 | S-2.06 | activation-skill-integration | E-2 | v1.0.0-beta.1 | merged |
| S-2.7 | S-2.07 | regression-test-validation | E-2 | v1.0.0-beta.1 | merged |
| S-2.8 | S-2.08 | beta1-release-gate | E-2 | v1.0.0-beta.1 | merged |
| S-3.1 | S-3.01 | port-capture-commit-activity | E-3 | (not yet shipped) | draft |
| S-3.2 | S-3.02 | port-capture-pr-activity | E-3 | (not yet shipped) | draft |
| S-3.3 | S-3.03 | port-block-ai-attribution | E-3 | (not yet shipped) | draft |
| S-3.4 | S-3.04 | emit-event-host-function | E-3 | v1.0.0-beta.4 (partial) | partial |
| S-4.1 | S-4.01 | sink-http-driver | E-4 | (not yet shipped) | draft |
| S-4.2 | S-4.02 | sink-datadog-driver | E-4 | (not yet shipped) | draft |
| S-4.3 | S-4.03 | sink-honeycomb-driver | E-4 | (not yet shipped) | draft |
| S-4.4 | S-4.04 | retry-circuit-breaker | E-4 | (not yet shipped) | draft |
| S-4.5 | S-4.05 | dead-letter-queue | E-4 | (not yet shipped) | draft |
| S-4.6 | S-4.06 | routing-tag-enrichment | E-4 | v1.0.0-beta.4 (partial) | partial |
| S-4.7 | S-4.07 | observability-integration-tests | E-4 | (not yet shipped) | draft |
| S-4.8 | S-4.08 | rc1-release-gate | E-4 | (not yet reached) | draft |
| S-5.1 | S-5.01 | session-start-hook | E-5 | (not yet shipped) | draft |
| S-5.2 | S-5.02 | session-end-hook | E-5 | (not yet shipped) | draft |
| S-5.3 | S-5.03 | worktree-hooks | E-5 | (not yet shipped) | draft |
| S-5.4 | S-5.04 | post-tool-use-failure | E-5 | (not yet shipped) | draft |
| S-5.5 | S-5.05 | migration-guide | E-5 | v1.0.0-beta.1 (skeleton only) | partial |
| S-5.6 | S-5.06 | semver-commitment-docs | E-5 | (not yet shipped) | draft |
| S-5.7 | S-5.07 | v1.0-release-gate | E-5 | (not yet reached) | draft |

## Notes

- Zero-padding applies to the story sequence number only (M → MM).
- Epic IDs use single-digit form: E-0 through E-5.
- Old `EPIC.md` (monolithic) archived to `.factory/stories/v1.0-legacy/EPIC.md`.
- Legacy story files moved to `.factory/stories/v1.0-legacy/` with git history intact.
- `behavioral_contracts: []` — left empty in all migrated stories pending iterative
  BC anchoring (product-owner refines in Phase 1.5).
