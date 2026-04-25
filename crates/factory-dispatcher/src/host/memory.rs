//! Bounds-checked accessors for guest-provided memory pointers.
//!
//! wasmtime's linear memory is accessed through the `Memory` handle
//! exported by the guest module. Every helper here refuses to touch a
//! (ptr, len) span that would read past the current memory size, and
//! refuses arithmetic that would overflow a `usize`. Callers propagate
//! the `HostCallError` as a negative return code via
//! [`super::codes::INVALID_ARGUMENT`].

use wasmtime::{Caller, Memory};

use super::{HostCallError, HostContext};

/// Resolve the guest's exported `memory`.
pub fn get_memory(caller: &mut Caller<'_, HostContext>) -> Result<Memory, HostCallError> {
    caller
        .get_export("memory")
        .and_then(|e| e.into_memory())
        .ok_or(HostCallError::MissingMemory)
}

/// Read `len` bytes from guest memory starting at `ptr`, returning an
/// owned copy so the caller can release the guest memory borrow.
pub fn read_wasm_bytes(
    caller: &mut Caller<'_, HostContext>,
    ptr: u32,
    len: u32,
) -> Result<Vec<u8>, HostCallError> {
    let memory = get_memory(caller)?;
    let data = memory.data(&caller);
    let start = ptr as usize;
    let end = start
        .checked_add(len as usize)
        .ok_or(HostCallError::MemoryOverflow)?;
    if end > data.len() {
        return Err(HostCallError::OutOfBounds {
            ptr,
            len,
            memory_size: data.len(),
        });
    }
    Ok(data[start..end].to_vec())
}

/// Read a UTF-8 string from guest memory. Non-UTF-8 bytes are an error;
/// plugins built through `vsdd-hook-sdk` always produce UTF-8.
pub fn read_wasm_string(
    caller: &mut Caller<'_, HostContext>,
    ptr: u32,
    len: u32,
) -> Result<String, HostCallError> {
    let bytes = read_wasm_bytes(caller, ptr, len)?;
    String::from_utf8(bytes).map_err(|_| HostCallError::InvalidUtf8)
}

/// Write `bytes` into guest memory at `out_ptr`.
///
/// Semantics match the SDK's `(out_ptr, out_cap) -> bytes_written`
/// contract:
/// - If `bytes.len() > out_cap`, returns `bytes.len()` **without**
///   writing (so the caller can grow and retry).
/// - Otherwise writes and returns the number of bytes actually written.
pub fn write_wasm_bytes(
    caller: &mut Caller<'_, HostContext>,
    out_ptr: u32,
    out_cap: u32,
    bytes: &[u8],
) -> Result<u32, HostCallError> {
    let needed = bytes.len() as u32;
    if needed > out_cap {
        return Ok(needed);
    }
    let memory = get_memory(caller)?;
    let start = out_ptr as usize;
    let end = start
        .checked_add(bytes.len())
        .ok_or(HostCallError::MemoryOverflow)?;
    let data_len = memory.data(&caller).len();
    if end > data_len {
        return Err(HostCallError::OutOfBounds {
            ptr: out_ptr,
            len: needed,
            memory_size: data_len,
        });
    }
    memory
        .write(caller, start, bytes)
        .map_err(|_| HostCallError::OutOfBounds {
            ptr: out_ptr,
            len: needed,
            memory_size: data_len,
        })?;
    Ok(needed)
}

/// Write a u32 little-endian value into guest memory. Used by
/// `read_file` / `exec_subprocess` to return `(ptr, len)` out-params.
pub fn write_wasm_u32(
    caller: &mut Caller<'_, HostContext>,
    out_ptr: u32,
    value: u32,
) -> Result<(), HostCallError> {
    let bytes = value.to_le_bytes();
    let written = write_wasm_bytes(caller, out_ptr, bytes.len() as u32, &bytes)?;
    if written != bytes.len() as u32 {
        return Err(HostCallError::OutOfBounds {
            ptr: out_ptr,
            len: bytes.len() as u32,
            memory_size: 0,
        });
    }
    Ok(())
}
