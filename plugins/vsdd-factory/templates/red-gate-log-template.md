---
document_type: red-gate-log
level: ops
version: "1.0"
status: draft
producer: test-writer
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 3
inputs: []
input-hash: "[md5]"
traces_to: ""
stub_architect_agent: "[agent session ID]"
stub_compile_verified: false
test_writer_agent: "[agent session ID]"
red_gate_verified: false
---

# Red Gate Log: [Wave/Story identifier]

## Summary
| Story | Tests Written | All Fail (Red)? | Gate |
|-------|-------------|-----------------|------|

## Stubs Created
### STORY-NNN: [Story Title]
- `fn function_name() -> ReturnType` -- [stub description]

## Red Gate Verification
### STORY-NNN
- AC-001 (BC-S.SS.NNN): [test name] -- FAIL (expected)
- AC-002 (BC-S.SS.NNN): [test name] -- FAIL (expected)

## Regression Check
| Existing Tests | Status |
|---------------|--------|
| [count] pre-existing tests | all pass / [N] broken |

## Hand-Off to Implementer
- Stories ready for implementation: [list]
- Implementation guidance: [notes]
