---
name: factory-unlock
description: Release the cross-session factory lock. Plain release (self-held only) or --force break-glass release (any lock, mandatory audit event). No direct STATE.md write — all writes go through state-manager (BC-6.23.001 Invariant 5 / AC-011).

allowed-tools: Bash, Read
---

# /factory-unlock — Release Factory Lock

Explicitly release the cross-session factory lock (CAP-031). Supports plain self-release
and `--force` break-glass release with mandatory `factory.lock.stolen` audit event.

## Step 1 — Invoke factory-unlock-decide.sh

<!-- TODO(S-17.03): implementer fills -->

Run the unlock decision helper with the STATE.md path, current git email, and optional --force flag:

```bash
plugins/vsdd-factory/bin/factory-unlock-decide.sh \
  <STATE_MD_PATH> \
  "$(git config user.email)" \
  [--force]
```

The helper inspects the `factory_lock` block and returns a decision token on stdout.

**Decision routing:**

- **"NOOP_ABSENT"** (exit 0): Factory already unlocked (EC-003 / EC-005).
  Exit 0 silently. No event emitted.

- **"REFUSED_NOT_HOLDER"** (exit 1): Plain unlock, foreign lock held (PC5).
  Helper already printed to stderr:
  `"Cannot unlock — factory is held by <holder_email>. Use /factory-unlock --force to force-release."`
  Propagate exit 1. STATE.md MUST NOT be modified.

- **"PROCEED_RELEASE"** (exit 0): Plain unlock, self-held lock (PC4). Proceed to Step 2.

- **"PROCEED_RELEASE_SELF_FORCE"** (exit 0): --force on self-held lock (EC-010).
  Treat as PROCEED_RELEASE — emit `factory.lock.released` NOT `factory.lock.stolen`
  (stolen_by == stolen_from is not a meaningful audit event). Proceed to Step 2.

- **"PROCEED_FORCE_STEAL"** (exit 0): --force on foreign lock (PC6). Proceed to Step 2.

**Neither this skill nor the helpers write STATE.md directly.** All writes go through
state-manager which invokes `factory-lock-write.sh` + `factory-cas-push.sh` from S-17.01.

## Step 2 — Delegate clear to state-manager (factory-lock-write.sh + factory-cas-push.sh)

<!-- TODO(S-17.03): implementer fills -->

On PROCEED_RELEASE, PROCEED_RELEASE_SELF_FORCE, or PROCEED_FORCE_STEAL, delegate to
state-manager. State-manager runs:

1. `plugins/vsdd-factory/bin/factory-lock-write.sh clear <STATE_MD_PATH>`
   Removes the `factory_lock` key ENTIRELY from STATE.md frontmatter.
   Key deletion, NOT null assignment (BC-5.40.001 PC2 / Invariant "not null").

2. `plugins/vsdd-factory/bin/factory-cas-push.sh`
   CAS push to origin/factory-artifacts.

On CAS push rejection: Error variant `UnlockCASRejected`; user retries.

## Step 3 — Emit factory.lock.released or factory.lock.stolen via emit-event

<!-- TODO(S-17.03): implementer fills -->

The decision token from Step 1 determines which event to emit:

**For PROCEED_RELEASE or PROCEED_RELEASE_SELF_FORCE:**
```bash
plugins/vsdd-factory/bin/emit-event \
  type=factory.lock.released \
  holder=<holder_from_decide_output> \
  locked_at=<locked_at_from_decide_output> \
  released_at=<released_at_from_decide_output>
```

**For PROCEED_FORCE_STEAL:**
```bash
plugins/vsdd-factory/bin/emit-event \
  type=factory.lock.stolen \
  stolen_by=<stolen_by_from_decide_output> \
  stolen_from=<stolen_from_from_decide_output> \
  holder_locked_at=<holder_locked_at_from_decide_output> \
  stolen_at=<stolen_at_from_decide_output>
```

`emit-event` is failure-tolerant (always exits 0). SS-03 unavailability does NOT abort
the force-release — the lock is already cleared (EC-008 / BC-6.23.001 Invariant 2).
Emission failure logged as log_warn.

`factory.lock.stolen` is mandatory on force-release of a foreign lock — it MUST be
emitted and cannot be suppressed (BC-6.23.001 Invariant 2).

## Step 4 — Display result message

<!-- TODO(S-17.03): implementer fills -->

**For PROCEED_RELEASE or PROCEED_RELEASE_SELF_FORCE:**
```
Factory lock released.
```

**For PROCEED_FORCE_STEAL:**
```
Factory lock force-released. Audit event 'factory.lock.stolen' emitted.
```

Exit 0.
