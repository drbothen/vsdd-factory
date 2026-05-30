---
document_type: state
version: "1.0"
current_step: "Phase 3 — trajectory-tail →9→9→9→9"
current_cycle: "v1.0-brownfield-backfill"
---

This file is used for the read-failure failopen test.
The bats test simulates read failure by pointing file_path to a
non-existent path (HostError::CapabilityDenied or HostError::Other).
Hook must exit 0 (Continue) on any HostError variant.
