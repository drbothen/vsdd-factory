# ID Reference

Complete reference for all 30 identifier formats used in VSDD. Every artifact in the pipeline is identified by a structured ID that enables traceability, validation, and cross-referencing.

---

## Quick Lookup

| ID | Format | What it identifies | Producer | Registry |
|----|--------|-------------------|----------|----------|
| CAP-NNN | `CAP-001` | Domain capability | business-analyst | L2-INDEX.md |
| DI-NNN | `DI-001` | Domain invariant | business-analyst | L2-INDEX.md |
| DEC-NNN | `DEC-001` | Domain edge case | business-analyst | L2-INDEX.md |
| ASM-NNN | `ASM-001` | Assumption requiring validation | business-analyst | L2-INDEX.md |
| R-NNN | `R-001` | Risk | business-analyst | L2-INDEX.md |
| FM-NNN | `FM-001` | Failure mode | business-analyst | L2-INDEX.md |
| SS-NN | `SS-01` | Architecture subsystem | architect | ARCH-INDEX.md |
| BC-S.SS.NNN | `BC-2.3.045` | Behavioral contract | product-owner | BC-INDEX.md |
| NFR-NNN | `NFR-001` | Non-functional requirement | product-owner | prd.md |
| E-xxx-NNN | `E-NET-001` | Error taxonomy entry | product-owner | error-taxonomy.md |
| EC-NNN | `EC-001` | Edge case (within a BC) | product-owner | (within BC file) |
| VP-NNN | `VP-001` | Verification property | architect | VP-INDEX.md |
| STORY-NNN | `STORY-001` | Implementation story | story-writer | STORY-INDEX.md |
| EPIC-NNN | `EPIC-001` | Epic (story group) | story-writer | EPIC-INDEX.md |
| AC-NNN | `AC-001` | Acceptance criterion | story-writer | (within story file) |
| EAC-NNN | `EAC-001` | Epic acceptance criterion | story-writer | (within epic file) |
| GAP-NNN | `GAP-001` | Gap register entry (deferred requirement) | story-writer | traceability-matrices |
| HS-NNN | `HS-001` | Holdout scenario | product-owner | HS-INDEX.md |
| WHS-W[N]-NNN | `WHS-W2-001` | Wave holdout scenario | state-manager | HS-INDEX.md |
| ADV-\<CYCLE\>-P[N]-[SEV]-NNN | `ADV-P1CONV-P03-CRIT-001` | Adversarial finding | adversary | ADV-INDEX.md |
| CR-NNN | `CR-001` | Code review finding | code-reviewer | (within review file) |
| SEC-NNN | `SEC-001` | Security finding | security-reviewer | (within review file) |
| FIX-P[N]-NNN | `FIX-P4-001` | Fix PR from phase review | pr-manager | (within cycle) |
| TD-NNN | `TD-001` | Tech debt register entry | orchestrator | tech-debt-register.md |
| EVAL-NNN | `EVAL-HS-001-P1` | Holdout evaluation result | holdout-evaluator | EVAL-INDEX.md |
| SCR-NNN | `SCR-001` | UX screen specification | ux-designer | UX-INDEX.md |
| CMP-NNN | `CMP-001` | UI component (within screen) | ux-designer | (within SCR file) |
| ELM-NNN | `ELM-001` | UI element (within screen) | ux-designer | (within SCR file) |
| INT-NNN | `INT-001` | UI interaction (within screen) | ux-designer | (within SCR file) |
| AD-NNN | `AD-001` | Architecture decision | architect | ARCH-INDEX.md |

---

## Scope Rules

Every ID has a **scope** that determines its persistence and uniqueness boundaries.

### Lifecycle IDs (append-only, never reused)

These IDs persist across all convergence cycles. Once assigned, an ID is permanently consumed — even if the artifact is retired, the ID is never reused. This is enforced by Policy 1 (`append_only_numbering`).

**IDs:** CAP, DI, DEC, ASM, R, FM, SS, BC, NFR, E-xxx, VP, STORY, EPIC, GAP, HS, FIX, TD, SCR, AD

**Rules:**
- Sequential numbering (001, 002, 003...)
- Never renumber, never reuse
- Retired artifacts keep their ID with `status: retired` in their index
- Filename slugs are immutable — even when titles change

### Cycle IDs (reset per convergence cycle)

These IDs reset per convergence cycle. A cycle prefix identifies which cycle produced them.

**IDs:** ADV-\<CYCLE\>-P[N]-[SEV]-NNN, WHS-W[N]-NNN, EVAL, CR, SEC

**Rules:**
- Sequential within the cycle
- Cycle prefix is mandatory (read from `.factory/current-cycle`)
- Do not carry over across cycles

### Local IDs (scoped to parent artifact)

These IDs are meaningful only within their parent file. EC-001 in BC-2.01.001 is unrelated to EC-001 in BC-3.05.012.

**IDs:** EC (within BC), AC (within story), EAC (within epic), CMP/ELM/INT (within SCR)

**Rules:**
- Sequential within the parent file
- Reset per parent file (EC-001 exists in every BC that has edge cases)
- Not globally unique — must always be qualified by parent ID (e.g., "BC-2.01.001 EC-001")

---

## ID Format Details

### CAP-NNN (Domain Capability)

- **Level:** L2 (Domain Specification)
- **Format:** `CAP-` + three-digit sequential number
- **Producer:** business-analyst during Phase 1a
- **Registry:** `domain-spec/L2-INDEX.md` (ID Registry Summary table)
- **Referenced by:** BCs (`capability:` frontmatter), epics, stories
- **Hook validation:** None (gate-only via consistency-validator criterion 3)
- **Example:** `CAP-001: Parse markdown files and extract URLs`

### SS-NN (Architecture Subsystem)

- **Level:** L3 (Architecture)
- **Format:** `SS-` + two-digit sequential number
- **Producer:** architect during Phase 1b
- **Registry:** `architecture/ARCH-INDEX.md` (Subsystem Registry table)
- **Referenced by:** BCs (`subsystem:` frontmatter), stories (`subsystems:` frontmatter)
- **Hook validation:** `validate-subsystem-names.sh` — validates SS-IDs exist in ARCH-INDEX
- **Example:** `SS-01: Sensor Adapters`
- **Note:** BCs and stories reference by ID (`SS-01`), not by name (`Sensor Adapters`)

### BC-S.SS.NNN (Behavioral Contract)

- **Level:** L3 (PRD)
- **Format:** `BC-` + section `.` subsection `.` three-digit sequential
- **Producer:** product-owner during Phase 1a
- **Registry:** `behavioral-contracts/BC-INDEX.md`
- **Referenced by:** Stories (`behavioral_contracts:` frontmatter, AC traces), VPs (`source_bc:`), holdout scenarios
- **Hook validation:** `validate-bc-title.sh` (H1 ↔ INDEX sync), `protect-bc.sh` (immutability), `validate-story-bc-sync.sh` (story ↔ BC sync)
- **Example:** `BC-2.3.045: Notification timeout handling`

### VP-NNN (Verification Property)

- **Level:** L4
- **Format:** `VP-` + three-digit sequential number
- **Producer:** architect during Phase 1b
- **Registry:** `verification-properties/VP-INDEX.md`
- **Referenced by:** Stories (`verification_properties:` frontmatter), architecture docs
- **Hook validation:** `validate-vp-consistency.sh` (VP-INDEX ↔ arch-doc sync), `protect-vp.sh` (immutability)
- **Example:** `VP-001: Slug GitHub-equivalence`

### STORY-NNN (Story)

- **Level:** Stories
- **Format:** `STORY-` + three-digit sequential number
- **Producer:** story-writer during Phase 2
- **Registry:** `stories/STORY-INDEX.md`
- **Referenced by:** Epics, sprint-state.yaml, adversarial findings, fix PRs
- **Hook validation:** `validate-story-bc-sync.sh` (frontmatter ↔ body BC sync), `validate-template-compliance.sh` (structural compliance)
- **Example:** `STORY-042: Implement credential rotation`

### ADV-\<CYCLE\>-P[N]-[SEV]-NNN (Adversarial Finding)

- **Level:** Reviews
- **Format:** `ADV-` + cycle prefix + `-P` + pass number + `-` + severity + `-` + three-digit sequential
- **Producer:** adversary during Phase 1d/4
- **Registry:** `cycles/<cycle>/adversarial-reviews/ADV-INDEX.md`
- **Referenced by:** Fix PRs (`source_finding:`), convergence reports
- **Hook validation:** `validate-finding-format.sh` — blocks legacy ADV-NNN and ADV-P[N]-NNN formats
- **Example:** `ADV-P1CONV-P03-CRIT-001: Missing edge case in BC-2.01.005`
- **Severity codes:** CRIT, HIGH, MED, LOW

### FIX-P[N]-NNN (Fix PR)

- **Level:** Reviews
- **Format:** `FIX-P` + source phase + `-` + three-digit sequential
- **Producer:** pr-manager during Phase 4/5/6
- **Registry:** None (tracked in cycle manifests)
- **Referenced by:** Adversarial reviews (resolution tracking)
- **Hook validation:** `validate-finding-format.sh` — blocks legacy STORY-NNN-FIX format
- **Example:** `FIX-P4-001: Fix BC-2.01.005 timeout handling`
- **Phase codes:** P4 = adversarial, P5 = hardening, P6 = convergence

---

## Automated Enforcement

| Hook | What it validates | IDs checked |
|------|------------------|-------------|
| `validate-subsystem-names.sh` | SS-NN exists in ARCH-INDEX | SS-NN |
| `validate-bc-title.sh` | BC H1 matches BC-INDEX | BC-S.SS.NNN |
| `validate-story-bc-sync.sh` | Story frontmatter ↔ body BC sync | BC-S.SS.NNN |
| `validate-vp-consistency.sh` | VP-INDEX ↔ arch-doc sync | VP-NNN |
| `validate-finding-format.sh` | Current ADV/FIX format used | ADV-\<CYCLE\>-P[N]-[SEV]-NNN, FIX-P[N]-NNN |
| `validate-template-compliance.sh` | Correct frontmatter field names | behavioral_contracts (not bcs), target_module (not crate) |
| `protect-bc.sh` | Green BC immutability | BC-S.SS.NNN |
| `protect-vp.sh` | Green VP immutability | VP-NNN |

IDs not validated by hooks are checked at phase gates by the consistency-validator (80 criteria).
