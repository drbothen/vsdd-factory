---
name: create-domain-spec
description: Create a sharded L2 domain specification from the product brief. Models the problem domain — entities, relationships, processes, invariants, and ubiquitous language. Writes to .factory/specs/domain-spec/.
disable-model-invocation: true
allowed-tools: Read, Write, Edit, Bash, AskUserQuestion
---

# Create Domain Specification

Build an L2 domain specification that models the problem space independent of implementation choices.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/L2-domain-spec-template.md` — L2 domain spec structure
- `${CLAUDE_PLUGIN_ROOT}/templates/L2-domain-spec-index-template.md` — index for sharded domain specs
- `${CLAUDE_PLUGIN_ROOT}/templates/L2-domain-spec-section-template.md` — per-section format This bridges the product brief (what we're building) and the PRD (how it behaves).

## Prerequisites

- `.factory/specs/product-brief.md` must exist. Read it first.
- Check `.factory/specs/research/RESEARCH-INDEX.md` for domain research. Read all `domain-*` reports — they contain competitive landscape, domain patterns, and pitfalls that should inform the domain model.

## Discovery Process

Use a **two-pass extraction approach** (validated by domain modeling research). Work with the user, asking questions one at a time.

### Pass 1: Structural Extraction

Focus on the **nouns** — what exists in this domain.

#### 1a. Entity Discovery
- What are the core things (nouns) in this domain?
- What properties does each entity have?
- Which properties are identifiers? Which are mutable?

#### 1b. Relationship Mapping
- How do entities relate to each other? (has-many, belongs-to, references)
- Are there aggregate boundaries? (things that must be consistent together)
- What are the cardinality constraints?

#### 1c. Value Objects & Enums
- What domain vocabulary is encoded as types? (status codes, categories, roles)
- What values are constrained? (email format, ID patterns, valid ranges)

#### 1d. Ubiquitous Language
- What terms does the domain use?
- Are there terms that are ambiguous or overloaded?
- Define each term precisely — this becomes the glossary.

### Pass 2: Behavioral Extraction

Focus on the **verbs** — what happens in this domain.

#### 2a. Process Modeling
- What are the key processes (verbs) in this domain?
- What triggers each process?
- What state transitions does each process cause?
- What are the happy paths? What are the failure paths?

#### 2b. Domain Events
- What events does the system emit? (user created, workflow completed, error occurred)
- Who produces each event? Who consumes it?
- What ordering constraints exist between events?

#### 2c. Business Rules & Invariants
- What must always be true? (constraints that can never be violated)
- What must be true at transaction boundaries?
- What are the ordering/sequencing rules?
- What validation rules govern input at trust boundaries?

#### 2d. State Machines
- Which entities have lifecycle states? (draft → active → archived)
- What transitions are allowed? What are forbidden?
- What side effects occur on each transition?

### Pass 3: Context Boundaries

#### 3a. Bounded Contexts
- Are there distinct subdomains with different models?
- Where do contexts overlap or communicate?
- What translation happens at context boundaries?

### Brownfield Enhancement

If `.factory/semport/` contains analysis from `/brownfield-ingest`, read:
- Pass 2 domain model for entity and relationship extraction
- Pass 3 behavioral contracts for process and invariant extraction
- Use these as starting points — validate with the user rather than asking from scratch

## Output

Always shard into an index + sections:

```
.factory/specs/domain-spec/
├── L2-INDEX.md                    # Index linking all sections
├── capabilities.md                # Domain capabilities and processes
├── entities.md                    # Entity definitions and relationships
├── invariants.md                  # Domain invariants and constraints
├── bounded-contexts.md            # Context boundaries and translations
└── ubiquitous-language.md         # Glossary of domain terms
```

Use `L2-domain-spec-index-template.md` for the index and `L2-domain-spec-section-template.md` for each section. Add or remove sections as needed — these are the common ones, not a fixed list.

## Self-Review (before adversarial review)

Before routing to the next pipeline stage, check your own work:

1. **Placeholder scan:** Any "TBD", "TODO", incomplete entity definitions, or vague invariants? Fix them now.
2. **Internal consistency:** Do entity relationships in `entities.md` match references in `capabilities.md`? Does the ubiquitous language glossary cover all terms used in other sections?
3. **Scope check:** Are bounded contexts clearly delineated? Is each section focused on one aspect of the domain?
4. **Ambiguity check:** Could any invariant or business rule be interpreted two different ways? Pick one and make it explicit.

Fix issues inline. This is a cheap filter — catch obvious gaps before the next stage.

## After Writing

1. Commit to factory-artifacts.
2. Tell the user: "Domain spec created. Next: `/create-prd` to elaborate requirements, or review and refine this spec first."
