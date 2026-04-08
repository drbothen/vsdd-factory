---
document_type: prd
level: L3
version: "1.0"
status: draft
producer: "[agent-id]"
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1a
inputs: [domain-spec/L2-INDEX.md, research/RESEARCH-INDEX.md]
input-hash: "[md5]"              # advisory — used for drift detection, not gating
traces_to: domain-spec/L2-INDEX.md
supplements: [interface-definitions.md, error-taxonomy.md, test-vectors.md, nfr-catalog.md]
---

# Product Requirements Document: [Product Name]

> **Context Engineering Principle -- Extended ToC Pattern:**
> Each section below should provide a concise summary (2-5 sentences) with references
> to where full detail can be found. Frontload critical constraints. Place validation
> rules at the end. This keeps the PRD within the agent's high-attention zone and
> avoids "lost in the middle" degradation. Full detail is loaded on-demand by agents
> that need it. Signal density matters more than coverage.

> **BC Index Model:** This PRD is an index document. Each Behavioral Contract (BC)
> lives in its own file under the `behavioral-contracts/` directory. The tables in
> Section 2 provide one-line summaries linking to the individual BC files. Do NOT
> inline full contract details here.

> **PRD Supplement Model:** Sections 3 (Interface Definition), 4 (Non-Functional
> Requirements), 5 (Error Taxonomy), and test vectors are extracted to separate files
> under the `prd-supplements/` directory. The core PRD retains references to these
> supplements via the `supplements:` frontmatter field. Each supplement is consumed by
> a different agent — extracting them reduces context loading for agents that do not
> need them. See `prd-supplements/interface-definitions.md`,
> `prd-supplements/error-taxonomy.md`, `prd-supplements/test-vectors.md`, and
> `prd-supplements/nfr-catalog.md`.

## 1. Product Overview

> **Sections 1.1-1.4** are human-audience context. They provide strategic framing
> for human reviewers and are not directly machine-consumed by downstream agents.
> Agents consume the structured tables in Sections 2-7.

### 1.1 Problem Statement

[Specific, measurable user/business pain point]

### 1.2 Solution Vision

[Agent-understandable solution architecture -- high-level approach and core capabilities]

### 1.3 Key Differentiators

| ID | Differentiator | Description |
|----|---------------|-------------|
| KD-001 | [differentiator name] | [why this matters competitively] |

### 1.4 Target Users

| Persona | Description | Volume | Pain Level |
|---------|-------------|--------|------------|
| | | | |

### 1.5 Out of Scope

> **Machine-consumed (Criterion 51).** The adversary and consistency-validator check
> that no story AC implements features listed here. Be explicit and unambiguous.

- [Explicitly what we are NOT building]

## 2. Behavioral Contracts Index

> Individual BC files live in the `behavioral-contracts/` directory.
> Grouped by L2 domain subsystem (CAP-NNN).
> Each BC uses hierarchical numbering: BC-S.SS.NNN where S = section,
> SS = subsection (matching L2 subsystem), NNN = sequential within subsystem.

### 2.1 [Subsystem Name] (CAP-001)

| BC ID | Title | Priority |
|-------|-------|----------|
| BC-2.1.001 | [one-line summary] | P0 |
| BC-2.1.002 | [one-line summary] | P1 |

> Full contracts: `behavioral-contracts/BC-2.1.001.md`, `behavioral-contracts/BC-2.1.002.md`

### 2.2 [Subsystem Name] (CAP-002)

| BC ID | Title | Priority |
|-------|-------|----------|
| BC-2.2.001 | [one-line summary] | P0 |

> Full contract: `behavioral-contracts/BC-2.2.001.md`

### 2.N [Subsystem Name] (CAP-NNN)

[Repeat for each L2 subsystem]

## 3. Interface Definition

> **Supplement:** Full interface definitions are in `prd-supplements/interface-definitions.md`.
> This section provides a summary reference only.

[Summary: CLI interface, exit codes, JSON output schema, config file schema, flag interactions.
See `prd-supplements/interface-definitions.md` for complete definitions.]

## 4. Non-Functional Requirements

> **Supplement:** Full NFR catalog is in `prd-supplements/nfr-catalog.md`.
> This section provides a summary reference only.

[Summary: NFR-NNN IDs with categories, numerical targets, and validation methods.
See `prd-supplements/nfr-catalog.md` for the complete catalog.]

## 5. Error Taxonomy

> **Supplement:** Full error taxonomy is in `prd-supplements/error-taxonomy.md`.
> This section provides a summary reference only.

[Summary: E-xxx-NNN error codes grouped by subsystem with severity and exit codes.
See `prd-supplements/error-taxonomy.md` for the complete taxonomy.]

## 5b. Test Vectors

> **Supplement:** Canonical test vectors are in `prd-supplements/test-vectors.md`.

[Summary: Golden test data for test generation and holdout evaluation.
See `prd-supplements/test-vectors.md` for the complete test vector tables.]

## 6. Competitive Differentiator Traceability

> Maps each key differentiator (from Section 1.3) to the behavioral contracts
> that implement it. Ensures every claimed differentiator has verifiable backing.

### 6.1 [Differentiator KD-001] -- [Name]

| BC ID | Contribution |
|-------|-------------|
| BC-S.SS.NNN | [how this contract supports the differentiator] |
| BC-S.SS.NNN | [how this contract supports the differentiator] |

### 6.2 [Differentiator KD-002] -- [Name]

| BC ID | Contribution |
|-------|-------------|
| BC-S.SS.NNN | [how this contract supports the differentiator] |

## 7. Requirements Traceability Matrix

| BC ID | Source (L2 CAP) | Module(s) | Priority | Test Type |
|-------|----------------|-----------|----------|-----------|
| BC-2.1.001 | CAP-001 | [filled by architect] | P0 | [unit/integration/property/fuzz] |
| BC-2.1.002 | CAP-001 | [filled by architect] | P1 | [unit/property] |
| BC-2.2.001 | CAP-002 | [filled by architect] | P0 | [integration/fuzz] |
