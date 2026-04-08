---
document_type: prd-supplement-error-taxonomy
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

# Error Taxonomy: [Product Name]

> PRD supplement — extracted from PRD Section 5.
> Referenced by: implementer, test-writer.

## Error Categories

| Category Code | Category | Description |
|--------------|----------|-------------|
| NET | Network | DNS, HTTP, TLS errors |
| FS | Filesystem | Read, write, permission errors |
| CFG | Configuration | Invalid config, missing keys |
| VAL | Validation | Input validation failures |
| IO | I/O | General I/O errors |

## Error Catalog

| Error Code | Category | Severity | Exit Code | Message Format |
|-----------|----------|----------|-----------|---------------|
| E-NET-001 | Network | broken | 1 | |
| E-CFG-001 | Configuration | broken | 1 | |
| E-VAL-001 | Validation | degraded | 0 | |

## Severity Definitions

| Severity | Meaning | Exit Code Impact |
|----------|---------|-----------------|
| broken | Cannot continue | Non-zero exit |
| degraded | Partial result possible | Zero exit with warnings |
| cosmetic | Formatting/display issue | Zero exit |
