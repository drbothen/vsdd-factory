# Demo Evidence — S-18.04a: precompact-flush Native WASM Plugin Core

Visual evidence that the precompact-flush WASM hook plugin works as specified.
Each segment is a VHS terminal recording that runs the real `factory-dispatcher` binary
against the real `precompact-flush.wasm` using a live git fixture.

| Segment | ACs | Recording |
|---------|-----|-----------|
| Positive flush — factory-artifacts HEAD advances | AC-004, AC-005 | `AC-005-positive-flush.gif` |
| Flush log entry — newline-terminated `<ts> <SHA> <cycle>/<step> commit` | AC-007 | `AC-007-log-entry.gif` |
| Remote push — bare remote ref advances | AC-001, AC-009 | `AC-009-push.gif` |
| DURABILITY DEGRADED advisory + exit 0 (error path) | AC-017 | `AC-017-durability-degraded.gif` |
| Hermetic: binary_allow=["git"] only, no bash subprocess | AC-001, AC-014 | `AC-014-hermetic-git-only.gif` |

See `evidence-report.md` for full coverage mapping and BC/ADR anchors.
