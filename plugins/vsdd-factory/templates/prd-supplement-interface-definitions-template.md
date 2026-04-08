---
document_type: prd-supplement-interface-definitions
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1a
inputs: [prd.md]
input-hash: "[md5]"
traces_to: prd.md
---

# Interface Definitions: [Product Name]

> PRD supplement — extracted from PRD Section 3.
> Referenced by: implementer, test-writer, devops-engineer.

## CLI Interface

```
[Full help text with type constraints]
```

## Exit Code Semantics

| Code | Meaning | When |
|------|---------|------|
| 0 | Success | |
| 1 | Error | |

## JSON Output Schema

```json
{
  "$schema": "...",
  "type": "object",
  "properties": {}
}
```

## Config File Schema

```toml
# [Full TOML with key-to-CLI-flag mapping]
```

## Flag Interactions

| Flag A | Flag B | Interaction | Resolution |
|--------|--------|-------------|------------|
| | | conflicts / overrides / requires | |
