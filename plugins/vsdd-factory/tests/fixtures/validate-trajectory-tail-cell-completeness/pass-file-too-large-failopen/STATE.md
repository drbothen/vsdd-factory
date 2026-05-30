---
document_type: state
version: "1.0"
current_step: "Phase 3 — trajectory-tail →9→9→9→9"
current_cycle: "v1.0-brownfield-backfill"
---

This file is used for the file-too-large failopen test.
The bats test simulates HostError::OutputTooLarge by using an envelope
that points to a file path where the dispatcher will attempt a read_file
that exceeds MAX_BYTES. Since we cannot actually create a 512KiB+ file
easily in bats, this test verifies the hook logic via the dispatcher
behavior when file_path causes an OutputTooLarge error.

In practice the bats test points the file_path to a real file but
the test validates that the hook exits 0 (Continue) even when it
cannot read the content — verified by checking no blocking_plugins signal.
