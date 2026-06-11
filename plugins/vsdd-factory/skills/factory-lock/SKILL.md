---
name: factory-lock
description: Acquire the cross-session factory lock on the factory-artifacts orphan branch. Performs fetch-before-check (BC-6.23.001 Precondition 2), then CAS-protected push via state-manager delegation. No direct STATE.md write — all writes go through state-manager (BC-6.23.001 Invariant 5 / AC-011).

allowed-tools: Bash, Read
---

# /factory-lock — Acquire Factory Lock

Explicitly acquire the cross-session factory lock (CAP-031). Cooperative lock management
for the factory-artifacts orphan branch. No auto-acquire — this skill is the ONLY
acquisition path (BC-6.23.001 Invariant 1 / ADR-025 Decision 6).

## Step 1 — Invoke factory-lock-acquire-precheck.sh

Run the acquire precheck helper with the STATE.md path:

```bash
plugins/vsdd-factory/bin/factory-lock-acquire-precheck.sh <STATE_MD_PATH>
```

The helper performs (in order):
1. `git fetch origin factory-artifacts` — EC-006 guard; exits 2 on failure with:
   `"Fetch failed before lock check. Cannot acquire safely."`
2. `git config user.email` — EC-007 guard; exits 2 on empty/unset with:
   `"git user.email not configured — cannot acquire factory lock."`
3. Reads local STATE.md `factory_lock` block.
4. Returns a decision token on stdout (exit 0) or on stderr (exit 1).

**Exit 0 + "PROCEED_ACQUIRE"**: Proceed to Step 2.
**Exit 0 + "NOOP_SELF_HELD"**: Display `Factory lock already held by this session.` Exit 0. Skip remaining steps.
**Exit 1 (REFUSED_FOREIGN_LOCK)**: Helper already printed the 5-field refusal message to stderr. Propagate exit 1. Skip remaining steps.
**Exit 2 (EC-006 or EC-007)**: Helper already printed error to stderr. Propagate exit 2.

## Step 2 — Delegate write to state-manager (factory-lock-write.sh + factory-cas-push.sh)

On PROCEED_ACQUIRE, delegate to state-manager. State-manager runs:

1. `plugins/vsdd-factory/bin/factory-lock-write.sh acquire <STATE_MD_PATH>`
   Writes `factory_lock = { holder: <email>, locked_at: <now>, expires_at: <now+2700s> }`.

2. `plugins/vsdd-factory/bin/factory-cas-push.sh`
   CAS push: `git fetch origin factory-artifacts` then
   `git push --force-with-lease=factory-artifacts:"${EXPECTED_SHA}" origin factory-artifacts`.

**On CAS push rejection (non-zero)**: Display:
```
Acquire failed — concurrent lock write detected. Fetch and retry.
(TOCTOU acquire-race CWE-367: another session wrote the lock between your fetch and your push.)
```
Exit non-zero. No STATE.md remains in partial written state (push rejection is the CAS gate).
Error variant: `AcquireRaceRejected` (BC-6.23.001 PC2).

**Neither this skill nor the helpers write STATE.md directly.** All writes go through
state-manager which invokes `factory-lock-write.sh` + `factory-cas-push.sh` from S-17.01.

## Step 3 — Emit factory.lock.acquired via emit-event

On successful CAS push, emit the acquisition event via the SS-03 event pipeline:

```bash
plugins/vsdd-factory/bin/emit-event \
  type=factory.lock.acquired \
  holder=<email> \
  locked_at=<locked_at_iso8601> \
  expires_at=<expires_at_iso8601>
```

`emit-event` is failure-tolerant (always exits 0). SS-03 unavailability does not abort
the acquire — the lock is already written and pushed (BC-6.23.001 Invariant 2 pattern).

## Step 4 — Display success message

Display to user:

```
Factory lock acquired by <email>. Expires at <expires_at>.
```

Exit 0.
