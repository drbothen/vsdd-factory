# Legacy Design Documents

Historical intent documents from the manual-VSDD era of vsdd-factory development
(2026-04-08 through 2026-04-24). These predate the formal VSDD onboarding (2026-04-25).

## Why preserved

- They captured the design intent for the v1.0 work that shipped as `1.0.0-beta.4`.
- They are inputs to the formal Phase 1 spec backfill (PRD with BCs, sharded architecture).
- They contain decisions, trade-off analyses, and open-question resolutions that should not be lost.

## Authoritative successor

After formal Phase 1 backfill completes, the authoritative spec sources are:

- `.factory/specs/PRD.md` and `.factory/specs/BC-INDEX.md` for behavioral contracts
- `.factory/specs/architecture/` and `.factory/specs/ARCH-INDEX.md` for architecture
- `.factory/specs/domain-spec/` for the L2 domain spec

The files here remain as historical references. New design work goes through the formal pipeline.

## Index

### Design specs (`.factory/specs/` → moved here)

| Date | Topic | Notes |
|------|-------|-------|
| 2026-04-13 | early-phase-gaps | Manual-VSDD pipeline gap analysis |
| 2026-04-13 | excalidraw-integration | Diagram tooling design |
| 2026-04-13 | release-infrastructure | Release pipeline design |
| 2026-04-13 | remaining-superpowers-gaps | Gap fill from superpowers ingestion |
| 2026-04-13 | scaffold-claude-md | CLAUDE.md scaffolding skill design |
| 2026-04-13 | subagent-driven-gaps | Subagent-driven workflow design |
| 2026-04-13 | writing-plans-gaps | Writing-plans skill design |
| 2026-04-24 | v1.0-factory-plugin-kit | **Major** — full v1.0 rearchitecture spec (dispatcher + WASM ABI + multi-sink obs + cross-platform) |

### Plans (`.factory/plans/` → `plans/` here)

| Date | Topic |
|------|-------|
| 2026-04-13 | early-phase-gaps |
| 2026-04-13 | excalidraw-integration |
| 2026-04-13 | release-infrastructure |
| 2026-04-13 | scaffold-claude-md |
