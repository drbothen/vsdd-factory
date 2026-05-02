# AC-8: ABI catalog documentation

**Criterion:** `crates/hook-sdk/HOST_ABI.md` lists `write_file` in the host export catalog
with full signature, input-pointer protocol note, timeout semantics, byte-cap semantics,
and safety policy.

**Trace:** BC-2.02.011 postcondition 6 (ABI catalog update).

---

## HOST_ABI.md Excerpt

File: `crates/hook-sdk/HOST_ABI.md`

```markdown
### `write_file(path_ptr, path_len, contents_ptr, contents_len, max_bytes, timeout_ms) -> i32`

Write a guest-owned byte slice to the filesystem through the dispatcher's
bounded host function (BC-2.02.011 — additive ABI extension, D-6 Option A;
`HOST_ABI_VERSION` stays at 1).

**Protocol:** input-pointer — the SDK passes guest-owned bytes;
the dispatcher copies them via `read_wasm_bytes`. This is the **inverse**
of `read_file`'s output-pointer protocol and the two must not be confused.

**Parameters:**

| Parameter | Type | Description |
|---|---|---|
| `path_ptr` | `u32` | Pointer to UTF-8 path in guest memory |
| `path_len` | `u32` | Byte length of path |
| `contents_ptr` | `u32` | Pointer to content bytes in guest memory |
| `contents_len` | `u32` | Byte length of content |
| `max_bytes` | `u32` | Mandatory byte cap; content exceeding this returns `-3` |
| `timeout_ms` | `u32` | Mandatory timeout budget; accepted for ABI stability (epoch interruption enforced in S-1.5) |

**Return values:**

| Code | Meaning |
|---|---|
| `0` | Success; full byte slice durably written to `path` |
| `-1` | Capability denied: path not in `capabilities.write_file.path_allow`, path traversal attempt, or no `write_file` capability block present |
| `-2` | Timeout exceeded `timeout_ms` |
| `-3` | Content length exceeded `max_bytes` cap; **no bytes written to disk** |
| `-4` | Invalid argument (e.g. UTF-8 path decoding failure) |
| `-99` | Filesystem I/O error or missing parent directory |

**Safety policy:** path must be within the plugin's declared
`capabilities.write_file.path_allow` list. Traversal attempts (`..`) return
`-1` (same as `read_file`). Deny-by-default: no capability block → `-1`.
```

---

## Coverage Checklist

| Requirement | Present |
|-------------|---------|
| Signature: `vsdd::write_file(path_ptr, path_len, contents_ptr, contents_len, max_bytes, timeout_ms) -> i32` | yes |
| Protocol: input-pointer (dispatcher copies via `read_wasm_bytes`) | yes |
| Differs from `read_file` output-pointer protocol | yes |
| Timeout semantics: epoch-interruption policy (same as `read_file`) | yes |
| Byte-cap: `max_bytes` mandatory; exceeding returns `-3` (OutputTooLarge) | yes |
| Safety policy: `write_file.path_allow` required; traversal -> `-1` | yes |
| Deny-by-default noted | yes |
| `HOST_ABI_VERSION` stays at 1 noted | yes |

All 8 required elements present.

**Status: PASS**
