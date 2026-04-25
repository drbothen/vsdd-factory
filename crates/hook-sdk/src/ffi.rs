//! Raw `extern "C"` declarations for vsdd-factory host functions.
//!
//! Plugin authors should not call into this module directly. Use the
//! ergonomic wrappers in [`crate::host`] instead.
//!
//! On `wasm32` targets these resolve to imports the wasmtime host
//! provides at link time. On non-wasm targets (used for unit tests of
//! the SDK itself) they are stubs so the crate compiles and tests can
//! exercise the surrounding logic.

#![allow(dead_code)]

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "vsdd")]
unsafe extern "C" {
    pub safe fn log(level: u32, msg_ptr: *const u8, msg_len: u32);

    pub safe fn emit_event(
        type_ptr: *const u8,
        type_len: u32,
        fields_ptr: *const u8,
        fields_len: u32,
    );

    pub safe fn read_file(
        path_ptr: *const u8,
        path_len: u32,
        max_bytes: u32,
        timeout_ms: u32,
        out_ptr_out: *mut u32,
        out_len_out: *mut u32,
    ) -> i32;

    pub safe fn exec_subprocess(
        cmd_ptr: *const u8,
        cmd_len: u32,
        args_ptr: *const u8,
        args_len: u32,
        stdin_ptr: *const u8,
        stdin_len: u32,
        timeout_ms: u32,
        max_output_bytes: u32,
        result_ptr_out: *mut u32,
        result_len_out: *mut u32,
    ) -> i32;

    pub safe fn session_id(out_ptr: *mut u8, out_cap: u32) -> u32;

    pub safe fn dispatcher_trace_id(out_ptr: *mut u8, out_cap: u32) -> u32;

    pub safe fn plugin_root(out_ptr: *mut u8, out_cap: u32) -> u32;

    pub safe fn plugin_version(out_ptr: *mut u8, out_cap: u32) -> u32;

    pub safe fn cwd(out_ptr: *mut u8, out_cap: u32) -> u32;

    pub safe fn env(name_ptr: *const u8, name_len: u32, out_ptr: *mut u8, out_cap: u32) -> i32;
}

// Host-side stubs so `cargo test` and `cargo check` work on non-wasm
// targets. Plugins built for production never link these.
#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::too_many_arguments)]
pub mod host_stubs {
    pub fn log(_level: u32, _msg_ptr: *const u8, _msg_len: u32) {}

    pub fn emit_event(
        _type_ptr: *const u8,
        _type_len: u32,
        _fields_ptr: *const u8,
        _fields_len: u32,
    ) {
    }

    pub fn read_file(
        _path_ptr: *const u8,
        _path_len: u32,
        _max_bytes: u32,
        _timeout_ms: u32,
        _out_ptr_out: *mut u32,
        _out_len_out: *mut u32,
    ) -> i32 {
        -1
    }

    pub fn exec_subprocess(
        _cmd_ptr: *const u8,
        _cmd_len: u32,
        _args_ptr: *const u8,
        _args_len: u32,
        _stdin_ptr: *const u8,
        _stdin_len: u32,
        _timeout_ms: u32,
        _max_output_bytes: u32,
        _result_ptr_out: *mut u32,
        _result_len_out: *mut u32,
    ) -> i32 {
        -1
    }

    pub fn session_id(_out_ptr: *mut u8, _out_cap: u32) -> u32 {
        0
    }

    pub fn dispatcher_trace_id(_out_ptr: *mut u8, _out_cap: u32) -> u32 {
        0
    }

    pub fn plugin_root(_out_ptr: *mut u8, _out_cap: u32) -> u32 {
        0
    }

    pub fn plugin_version(_out_ptr: *mut u8, _out_cap: u32) -> u32 {
        0
    }

    pub fn cwd(_out_ptr: *mut u8, _out_cap: u32) -> u32 {
        0
    }

    pub fn env(_name_ptr: *const u8, _name_len: u32, _out_ptr: *mut u8, _out_cap: u32) -> i32 {
        -1
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use host_stubs::*;
