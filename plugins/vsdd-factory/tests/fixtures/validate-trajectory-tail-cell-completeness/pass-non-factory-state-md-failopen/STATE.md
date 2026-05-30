---
document_type: state
version: "1.0"
current_step: "Phase 3 step — no trajectory tail, but this is NOT in .factory/"
---

This STATE.md is written to a non-factory path (e.g., /tmp/STATE.md).
It has no trajectory tail but the hook MUST NOT fire for it.
Precondition 4 parent-guard: Path::new(file_path).components().any(|c| c.as_os_str() == ".factory")
returns false for /tmp/STATE.md — hook emits Continue immediately (EC-019).
