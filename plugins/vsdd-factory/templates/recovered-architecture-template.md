# Recovered Architecture: [PROJECT_NAME]

> Recovered by Dark Factory Phase 0b
> Source codebase: `[CODEBASE_PATH]`
> Date: [GENERATION_DATE]
> Confidence: [high / medium / low]

---

## Components

| # | Component | Path | Layer | Purpose | Dependents | Dependencies |
|---|-----------|------|-------|---------|------------|-------------|
| C-1 | [name] | `[path]` | [layer] | [purpose] | [list] | [list] |
| C-2 | [name] | `[path]` | [layer] | [purpose] | [list] | [list] |

## Component Map (Machine-Readable)

A structured YAML map matching the greenfield architecture template schema (DF-003
Section 3b). This enables downstream agents to consume brownfield component data
identically to greenfield component data.

```yaml
components:
  - id: C-1
    name: "[component name]"
    path: "[src/path]"
    layer: "[presentation | business-logic | data-access | infrastructure | shared]"
    purity: "[pure-core | effectful-shell | mixed | opaque]"
    criticality: "[CRITICAL | HIGH | MEDIUM | LOW]"
    dependencies: ["C-2", "C-3"]
    interfaces_provided: ["[public API surface]"]
    interfaces_consumed: ["[dependencies used]"]
    confidence: "[high | medium | low]"
```

> **Note:** The `criticality` field is populated during Step 0e.5 (Module Criticality
> Classification). Initially set to `MEDIUM` (the default tier per DF-004) until
> classification runs. The `confidence` field is brownfield-specific -- it indicates
> how certain the recovery is (not present in greenfield where the architect designs
> from scratch). The `path` field is also brownfield-specific -- greenfield components
> don't have existing source paths at architecture time.

---

## Layers

### Layer Diagram

```text
+--------------------------------------------------+
| [Layer 1: e.g., Presentation / API / CLI]        |
|   Components: [C-1, C-2]                         |
+--------------------------------------------------+
                      |
                      v
+--------------------------------------------------+
| [Layer 2: e.g., Business Logic / Domain]         |
|   Components: [C-3, C-4, C-5]                    |
+--------------------------------------------------+
                      |
                      v
+--------------------------------------------------+
| [Layer 3: e.g., Data Access / Infrastructure]    |
|   Components: [C-6, C-7]                         |
+--------------------------------------------------+
```

### Layer Rules (Observed)

| Rule | Observed? | Violations |
|------|----------|------------|
| Upper layers depend on lower layers only | [yes / no] | [list violations] |
| No circular dependencies between layers | [yes / no] | [list violations] |
| Data access layer does not import presentation | [yes / no] | [list violations] |

---

## Dependencies (DAG)

### Textual Representation

```text
[component-a]
  +-- [component-b]
  |     +-- [component-d]
  +-- [component-c]
        +-- [component-d]

[component-e] (standalone -- no dependencies)
```

### Circular Dependencies

| Cycle | Components | Severity | Notes |
|-------|-----------|----------|-------|
| [none detected / list cycles] | | | |

### External Dependencies (Top 10 by Importance)

| Dependency | Version | Purpose | Used By |
|-----------|---------|---------|---------|
| [dep-1] | [version] | [purpose] | [components] |

---

## API Surface

### HTTP Endpoints

| Method | Path | Handler | Auth Required | Request Body | Response |
|--------|------|---------|--------------|-------------|----------|
| [GET] | `/api/[path]` | `[file:function]` | [yes/no] | [type or none] | [type] |

### CLI Commands

| Command | Subcommand | Flags | Handler | Description |
|---------|-----------|-------|---------|-------------|
| [cmd] | [sub] | [--flag] | `[file:function]` | [description] |

### Library Exports

| Export | Type | Path | Description |
|--------|------|------|-------------|
| [name] | [function / struct / trait / type] | `[file:line]` | [description] |

---

## Data Models

### Core Domain Models

| Model | Path | Key Fields | Relationships |
|-------|------|-----------|---------------|
| [Model-1] | `[file:line]` | [field1: Type, field2: Type] | [has-many Model-2, belongs-to Model-3] |

### Database Schema (if detected)

| Table/Collection | Model | Migration File | Notes |
|-----------------|-------|---------------|-------|
| [table-1] | [Model-1] | `[migration path]` | [notes] |

### Data Flow

```text
[Input Source] --> [Model-1] --> [Processing Component] --> [Model-2] --> [Output/Storage]
```

---

## Integration Points

### External Services

| Service | Protocol | Configuration | Purpose | Error Handling |
|---------|----------|--------------|---------|----------------|
| [service-1] | [HTTP / gRPC / WebSocket] | `[env var or config]` | [purpose] | [retry / circuit-breaker / none] |

### Databases

| Database | Type | Connection Config | ORM/Driver | Purpose |
|----------|------|------------------|-----------|---------|
| [db-1] | [PostgreSQL / SQLite / Redis] | `[env var]` | [diesel / sqlx / prisma] | [purpose] |

### Message Queues

| Queue | Type | Configuration | Publishers | Consumers |
|-------|------|--------------|-----------|-----------|
| [queue-1] | [RabbitMQ / Kafka / SQS] | `[config]` | [components] | [components] |

### File System Dependencies

| Path/Pattern | Purpose | Read/Write | Configuration |
|-------------|---------|-----------|---------------|
| [path] | [purpose] | [R / W / RW] | [hardcoded / configurable via env] |

### DTU Candidate Assessment (DF-011)

Prioritized list of external services recommended for Digital Twin Universe cloning.
Each service is assessed for clone priority based on testing impact.

| # | Service | Priority | Recommended Fidelity | Rationale |
|---|---------|----------|---------------------|-----------|
| DTU-1 | [service-1] | [high / medium / low] | [L1 / L2 / L3 / L4] | [rationale] |
| DTU-2 | [service-2] | [high / medium / low] | [L1 / L2 / L3 / L4] | [rationale] |

#### Assessment Criteria

| Factor | [service-1] | [service-2] |
|--------|------------|------------|
| **Call volume** | [high / medium / low] | |
| **Rate limit risk** | [yes: N req/min / no] | |
| **Failure mode complexity** | [simple timeout / complex state / cascading] | |
| **Auth complexity** | [none / API key / OAuth / mTLS] | |
| **State requirements** | [stateless / CRUD / complex state machine] | |
| **Test data sensitivity** | [can use synthetic / needs realistic / has PII concerns] | |

#### Fidelity Level Reference (DF-011)

| Level | Description | When to Recommend |
|-------|-------------|-------------------|
| **L1: API Shape** | Correct endpoints, valid-shaped static responses | Read-only APIs, schema validation only |
| **L2: Stateful** | L1 + state persistence across requests | CRUD operations, integration testing |
| **L3: Behavioral** | L2 + edge cases, rate limiting, auth lifecycle | Holdout evaluation, production-realistic testing |
| **L4: Adversarial** | L3 + failure injection, latency simulation | Formal hardening, reliability testing |

---

## Technology Stack

| Category | Technology | Version | Lock File | Configuration |
|----------|-----------|---------|-----------|---------------|
| **Language** | [lang] | [version] | [lock file] | [version file] |
| **Framework** | [framework] | [version] | [lock file] | [config file] |
| **Test framework** | [test-fw] | [version] | [lock file] | [config file] |
| **Build tool** | [tool] | [version] | -- | [config file] |
| **Linter** | [tool] | [version] | [lock file] | [config file] |
| **Formatter** | [tool] | [version] | [lock file] | [config file] |
| **CI/CD** | [platform] | -- | -- | [workflow file] |
| **Containerization** | [Docker / Podman / none] | [version] | -- | [Dockerfile path] |

---

## Architecture Smells (Detected)

| Smell | Location | Severity | Description |
|-------|----------|----------|-------------|
| [God module] | `[path]` | [high/medium/low] | [module does too many things] |
| [Circular dependency] | `[path] <-> [path]` | [high/medium/low] | [description] |
| [Layer violation] | `[path]` | [high/medium/low] | [lower layer imports upper] |
| [None detected] | -- | -- | -- |
