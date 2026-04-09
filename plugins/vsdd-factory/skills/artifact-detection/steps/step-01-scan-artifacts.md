# Step 1: Scan for Existing Artifacts

> **Parent skill:** `artifact-detection` — see `../SKILL.md` for the full workflow.
> **Agent:** orchestrator
> **This step:** mechanically discover every planning artifact that exists in the project and record what was found.

## Inputs

- Project root (current working directory)
- The artifact patterns table in SKILL.md § Step 1

## Outputs

- `.factory/planning/artifact-inventory.md` — a complete list of what was found, with paths and detected formats
- In-memory inventory passed to step 2

## Procedure

1. **Verify `.factory/` exists.** If not, record `factory_present: false` and continue scanning the rest of the project. The absence of `.factory/` is a strong L0 signal but not conclusive — artifacts may live in `docs/` or the root.

2. **Run each glob from the artifact table.** Use Glob (not bash find). For each pattern, record matched paths.

   | Artifact | Patterns |
   |---|---|
   | Product Brief | `**/product-brief.md`, `**/brief.md`, `**/*brief*.md` |
   | Domain Spec L2 | `.factory/specs/domain-spec/L2-INDEX.md`, `.factory/specs/domain-spec/*.md` |
   | PRD | `**/prd.md`, `**/PRD.md`, `.factory/specs/prd.md` |
   | Behavioral Contracts | `.factory/specs/behavioral-contracts/BC-INDEX.md`, `.factory/specs/behavioral-contracts/BC-*.md` |
   | Verification Properties | `.factory/specs/verification-properties/VP-INDEX.md`, `.factory/specs/verification-properties/VP-*.md` |
   | Architecture | `**/architecture/ARCH-INDEX.md`, `.factory/specs/architecture/ARCH-*.md` |
   | Architecture Feasibility | `.factory/specs/architecture-feasibility-report.md` |
   | Verification Architecture | `.factory/specs/architecture/verification-architecture/ARCH-INDEX.md` |
   | Adversarial Reviews | `.factory/specs/adversarial-reviews/**`, `.factory/cycles/**/adversarial-reviews/**` |
   | Evaluations | `.factory/holdout-scenarios/evaluations/EVAL-INDEX.md` |
   | PRD Supplements | `.factory/specs/prd-supplements/**` |
   | UX Spec | `**/ux-spec/UX-INDEX.md`, `**/ux-design.md` |
   | Epics/Stories | `.factory/stories/epics.md`, `.factory/stories/stories/**` |
   | Project Context | `**/project-context.md` |

3. **Scan loose locations** the human may have used outside `.factory/`:
   - Project root (`./*.md`)
   - `docs/` directory
   - Any path the human explicitly mentioned in the initial message

4. **For each matched file, detect format and structure:**
   - **Spec hierarchy format:** open the file (or its index) and check for `BC-S.SS.NNN` (current 4-level) vs `FR-NNN` (legacy flat). Record the detected format. If `FR-NNN`, flag for migration.
   - **Architecture sharding:** if architecture exists, check whether it is sharded (`ARCH-INDEX.md` + section files in `architecture/`) or single-file legacy. Flag the legacy form.

5. **For each matched file, record:**
   - Absolute path
   - Size (bytes — small files are suspicious)
   - Last modified
   - Detected format (current / legacy / unknown)
   - Parseability flag (frontmatter readable? truncated?)

6. **Write the inventory** to `.factory/planning/artifact-inventory.md`:

```markdown
# Artifact Inventory

**Scanned:** <YYYY-MM-DD HH:MM>
**Project root:** <path>
**.factory/ present:** yes | no

## Found

| Artifact | Path | Size | Format | Parseable |
|---|---|---|---|---|
| Product Brief | <path> | <bytes> | current | yes |
| PRD | <path> | <bytes> | legacy FR-NNN | yes |
| ... | ... | ... | ... | ... |

## Not Found

- <artifact name>
- <artifact name>

## Format Flags

- <path>: legacy FR-NNN format — migration required
- <path>: legacy single-file architecture — sharding required
- <path>: unparseable frontmatter — flag for re-creation

## Loose Files (outside .factory/)

- <path>: <inferred type>
```

## Failure modes

- **Glob returns nothing for `.factory/` patterns** → do not assume L0. Scan loose locations first; the human may have a brief in `docs/`.
- **Matched file is unparseable** (binary, truncated, broken frontmatter) → record `parseable: no` and flag in step 4 (gap analysis). Do not skip silently.
- **Multiple briefs found** (e.g., `brief.md` and `product-brief.md`) → record both, flag the conflict for routing in step 5.

## Quality gate

- [ ] Every pattern in the table has been searched
- [ ] Loose locations (root, `docs/`) checked
- [ ] Inventory file written with paths, sizes, formats, parseability
- [ ] Format flags raised for any legacy or unparseable artifacts

## Hand-off to next step

Pass the inventory to `step-02-classify-readiness.md`.
