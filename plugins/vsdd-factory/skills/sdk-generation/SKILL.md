---
name: sdk-generation
description: >
  Generates production-ready SDKs in multiple languages from API contracts (OpenAPI,
  protobuf, or GraphQL schemas). Uses Fern, Speakeasy, or OpenAPI Generator patterns.
  Produces idiomatic code following each language's conventions — TypeScript uses
  async/await, Python uses snake_case, Go uses error handling idioms.
---

# SDK Generation

## When This Skill Runs

- **After contracts approved (story decomposition):** SDK repos are populated with
  generated code. SDK stories are auto-created.
- **Feature Mode (after contract change detected):** SDK regeneration is triggered
  for all consumer repos.
- **Maintenance Mode:** Periodic check that SDKs are up-to-date with current contracts.

## Prerequisites

- API contract exists at the path specified in `project.yaml`
- `project.yaml` specifies `generation.tool` for each SDK repo
- Target SDK repo directory exists (created by DF-012 multi-repo setup)

## Workflow

### Step 1: Read Contract

Read the source contract from the path specified in `project.yaml`:

```bash
# For OpenAPI:
CONTRACT_PATH=$(yq '.repos.api-server.contracts.produces[0].path' project.yaml)
# Validate:
npx @apidevtools/swagger-cli validate "$CONTRACT_PATH"
```

### Step 2: Select Generation Tool

Based on `project.yaml` configuration and contract format:

| Contract Format | Recommended Tool | Alternatives |
|----------------|-----------------|-------------|
| OpenAPI 3.x | Fern | Speakeasy, OpenAPI Generator |
| Protocol Buffers | Buf + protoc | grpc-tools |
| GraphQL | graphql-codegen | Apollo Codegen |

### Step 3: Generate SDK per Language

For each SDK repo specified in `project.yaml`:

#### TypeScript SDK Generation

```bash
# Using Fern (recommended for TypeScript):
cd sdk-typescript
fern generate --api ../api-server/openapi.yaml --language typescript

# Or using OpenAPI Generator:
npx @openapitools/openapi-generator-cli generate \
  -i ../api-server/openapi.yaml \
  -g typescript-fetch \
  -o ./src/generated \
  --additional-properties=supportsES6=true,npmName=@acme/api-client
```

Generated TypeScript SDK includes:
- Typed client class with methods for each endpoint
- TypeScript interfaces for all request/response types
- Async/await patterns with proper error handling
- Built-in authentication (API key, OAuth2, Bearer token)
- Retry logic with configurable backoff
- Pagination helpers

#### Python SDK Generation

```bash
# Using Fern:
cd sdk-python
fern generate --api ../api-server/openapi.yaml --language python

# Or using OpenAPI Generator:
npx @openapitools/openapi-generator-cli generate \
  -i ../api-server/openapi.yaml \
  -g python \
  -o ./src/generated \
  --additional-properties=packageName=acme_api,projectName=acme-api-client
```

Generated Python SDK includes:
- Client class with methods for each endpoint (snake_case naming)
- Pydantic models for all request/response types
- Type hints with mypy compatibility
- Built-in authentication
- Retry logic with tenacity
- Async support via httpx

#### Go SDK Generation

```bash
# Using Fern:
cd sdk-go
fern generate --api ../api-server/openapi.yaml --language go

# Or using OpenAPI Generator:
npx @openapitools/openapi-generator-cli generate \
  -i ../api-server/openapi.yaml \
  -g go \
  -o ./generated \
  --additional-properties=packageName=acmeapi
```

Generated Go SDK includes:
- Client struct with methods for each endpoint
- Go structs for all request/response types
- Idiomatic error handling (return value + error)
- Context support for cancellation/timeout
- Built-in authentication

#### Protocol Buffer Generation (Multi-Language)

```bash
# Using Buf (recommended for protobuf):
cd api-server
buf generate  # Generates code for all configured languages

# buf.gen.yaml configuration:
# version: v1
# plugins:
#   - plugin: go
#     out: ../sdk-go/proto
#     opt: paths=source_relative
#   - plugin: ts
#     out: ../sdk-typescript/proto
#     opt: target=web
#   - plugin: python
#     out: ../sdk-python/proto
```

### Step 4: Post-Generation Customization

After generation, the implementer agent customizes the SDK:
- Add package metadata (README, LICENSE, package.json/setup.py/go.mod)
- Add integration tests specific to the SDK
- Add examples and usage documentation
- Customize error types to match the project's domain language
- Add any manual extensions not covered by generation

### Step 5: Validate Generated SDK

For each generated SDK:
1. Run the SDK's own test suite (generated tests + custom tests)
2. Run contract tests (see next deliverable) to verify SDK matches contract
3. Verify the SDK can successfully call each endpoint defined in the contract
4. Verify error handling for all documented error responses

### Step 6: Publish (if configured)

For SDKs with publishing configured:
- TypeScript: `npm publish` to npm registry
- Python: `twine upload` to PyPI
- Go: Tag and push to GitHub (Go modules fetch by tag)

Publishing is gated by convergence -- SDKs are only published after the full pipeline converges.

## Output Artifacts

- `[sdk-repo]/src/generated/` -- generated SDK code
- `[sdk-repo]/package.json` or `setup.py` or `go.mod` -- package metadata
- `[sdk-repo]/tests/` -- generated + custom tests
- `[sdk-repo]/README.md` -- usage documentation
- `.factory-project/sdk-generation-report.md` -- generation log per language

---

## Contract Testing Integration

Contract testing validates that implementations comply with API contracts.

### Consumer-Driven Contract Testing (Pact Pattern)

The **consumer** (frontend, SDK) writes tests that describe what it expects from the
provider (API server). These expectations are published as contracts. The provider's CI
verifies it satisfies all consumer contracts.

**Workflow:**

```
Consumer repo (frontend):
  1. Write Pact consumer tests defining expected API interactions
  2. Run tests -> generate Pact contract files (.json)
  3. Publish contracts to Pact Broker (or shared directory)

Provider repo (api-server):
  4. Download all consumer contracts from Pact Broker
  5. Start the API server
  6. Verify each contract against the running server
  7. If any verification fails -> provider has broken a consumer's expectations
```

**Integration with Dark Factory phases:**

| Pipeline Point | Contract Testing Role |
|----------------|----------------------|
| During implementation | Consumer tests run as part of the test suite |
| During holdout evaluation | Contract compliance is a holdout dimension |
| During adversarial refinement | Adversary checks for contract violations |
| Feature mode (delta implementation) | Contract changes trigger consumer re-verification |
| Feature mode (scoped adversarial) | Cross-repo contract compliance validation |

### Specification-Driven Contract Testing (Specmatic Pattern)

Instead of consumer-driven contracts, use the **OpenAPI spec itself** as the contract.
Specmatic auto-generates tests from the spec and validates both consumers and providers.

**Workflow:**

```
OpenAPI spec (source of truth):
  1. Specmatic reads openapi.yaml
  2. For consumer testing: Specmatic creates a mock provider from the spec
     -> Consumer tests run against the mock
  3. For provider testing: Specmatic creates synthetic requests from the spec
     -> Provider must handle all spec-defined request shapes correctly
```

**Advantage:** No manual contract writing needed -- the OpenAPI spec IS the contract.

### When to Use Which Pattern

| Scenario | Pattern | Tool |
|----------|---------|------|
| Consumer expectations must be validated | Consumer-driven | Pact |
| OpenAPI spec is the source of truth | Specification-driven | Specmatic |
| Need to auto-discover breaking changes | Diff-based | openapi-diff, Optic |
| Property-based API testing | Generative | Schemathesis |

### Running Contract Tests

```bash
# Pact: Consumer test (in frontend repo)
npm test -- --reporter=pact  # Generates pact contracts

# Pact: Provider verification (in api-server repo)
npx pact-verifier \
  --provider-base-url=http://localhost:3000 \
  --pact-urls=../frontend/pacts/frontend-api.json

# Specmatic: Auto-test from OpenAPI spec
npx specmatic test \
  --contract=./openapi.yaml \
  --host=http://localhost:3000

# Schemathesis: Property-based API testing
schemathesis run \
  --url=http://localhost:3000 \
  --schema=./openapi.yaml \
  --checks all
```

---

## Schema Registry

For multi-repo projects, API contracts must be versioned, discoverable, and authoritative.

**Implementation:** A directory structure in the project root:

```
contracts/
+-- registry.yaml
+-- api-server/
|   +-- openapi.yaml
|   +-- openapi-v1.0.0.yaml
|   +-- changelog.md
+-- events/
|   +-- events.proto
|   +-- changelog.md
+-- graphql/
    +-- schema.graphql
    +-- changelog.md
```

**`registry.yaml` format:**

```yaml
contracts:
  - name: "acme-api"
    format: "openapi"
    version: "2.1.0"
    path: "./api-server/openapi.yaml"
    producers: ["api-server"]
    consumers: ["frontend", "sdk-typescript", "sdk-python"]
    last_updated: "2026-03-15"
```

---

## Contract Evolution Rules

Contracts follow **semantic versioning**:

### MAJOR: Breaking Changes

Changes that break existing consumers:
- Removing an endpoint
- Removing a required response field
- Changing a field's type
- Changing authentication requirements
- Removing a status code from the contract

**Protocol:**
1. Contract-steward flags the change as BREAKING
2. All consumer repos are notified
3. Consumer repos must be updated before the contract change is merged
4. SDK regeneration is triggered for all consumers
5. Full cross-repo integration testing (DF-012 integration gate)

### MINOR: Backward-Compatible Additions

Changes that don't break consumers:
- Adding a new endpoint
- Adding an optional response field
- Adding a new query parameter (optional)
- Adding a new error status code

### PATCH: Corrections

Changes that fix documentation without changing behavior:
- Fixing typos in descriptions
- Adding examples
- Clarifying field constraints

### Breaking Change Detection

```bash
# OpenAPI:
npx openapi-diff ./contracts/api-server/openapi-v2.0.0.yaml ./contracts/api-server/openapi.yaml

# Protocol Buffers:
buf breaking --against ./contracts/events/events-v1.0.0.proto ./contracts/events/events.proto

# GraphQL:
npx graphql-inspector diff ./contracts/graphql/schema-v1.0.0.graphql ./contracts/graphql/schema.graphql
```

---

## Cross-Language Type Synchronization

When the same types must exist in multiple languages, the contract is the single
source of truth for type generation.

### Pattern: Contract -> Generated Types

The OpenAPI schema definition maps to language-specific types:
- Rust: structs with serde derives
- TypeScript: interfaces with typed properties
- Python: dataclasses with type hints

### When to Use Each Pattern

| Scenario | Pattern | Tool |
|----------|---------|------|
| OpenAPI-based REST API | Generate from OpenAPI | Fern, Speakeasy, OpenAPI Generator |
| gRPC services | Generate from protobuf | Buf, protoc |
| GraphQL API | Generate from schema | graphql-codegen |
| Rust -> TypeScript (no API contract) | Direct type sync | tsync |
| Any -> Any (complex migration) | Semantic porting | Semport (DF-014) |

## Quality Gate

- [ ] Generated SDK compiles without errors in each target language
- [ ] Contract tests pass (SDK matches the source API contract)
- [ ] Generated code follows target language idioms (async/await for TS, snake_case for Python, error returns for Go)
- [ ] SDK generation report produced in `.factory-project/sdk-generation-report.md`

## Failure Modes

- If contract validation fails before generation: halt and report invalid contract to orchestrator
- If generated SDK fails to compile: run syntax fixer, retry generation with alternate tool
- If contract tests fail for a specific endpoint: isolate the endpoint, regenerate, and re-test
