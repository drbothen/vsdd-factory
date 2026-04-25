//! `emit_event` host function.
//!
//! Decodes the length-prefixed key/value fields buffer the SDK writes,
//! enriches the event with dispatcher-owned identity fields
//! (`dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`,
//! `ts`, `ts_epoch`, `schema_version`) and pushes onto the
//! [`HostContext::events`] stub queue. S-1.8's file sink becomes the
//! consumer.

use serde_json::Value;
use wasmtime::Linker;

use super::memory::{read_wasm_bytes, read_wasm_string};
use super::{HostCallError, HostCaller, HostContext};
use crate::internal_log::InternalEvent;

pub fn register(linker: &mut Linker<HostContext>) -> Result<(), HostCallError> {
    linker
        .func_wrap(
            "vsdd",
            "emit_event",
            |mut caller: HostCaller<'_>,
             type_ptr: u32,
             type_len: u32,
             fields_ptr: u32,
             fields_len: u32| {
                let event_type = match read_wasm_string(&mut caller, type_ptr, type_len) {
                    Ok(s) => s,
                    Err(_) => return, // best-effort; drop malformed
                };
                let fields_buf = match read_wasm_bytes(&mut caller, fields_ptr, fields_len) {
                    Ok(b) => b,
                    Err(_) => return,
                };
                let pairs = decode_fields(&fields_buf).unwrap_or_default();

                let ctx = caller.data();
                let mut ev = InternalEvent::now(&event_type)
                    .with_trace_id(&ctx.dispatcher_trace_id)
                    .with_session_id(&ctx.session_id)
                    .with_plugin_name(&ctx.plugin_name)
                    .with_plugin_version(&ctx.plugin_version);
                for (k, v) in pairs {
                    // Plugins cannot override dispatcher-owned identity
                    // fields. See HOST_ABI.md "Event enrichment".
                    if is_reserved_field(&k) {
                        continue;
                    }
                    ev = ev.with_field(&k, Value::String(v));
                }
                ctx.emit_internal(ev);
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;
    Ok(())
}

const RESERVED_FIELDS: &[&str] = &[
    "dispatcher_trace_id",
    "session_id",
    "plugin_name",
    "plugin_version",
    "ts",
    "ts_epoch",
    "schema_version",
    "type",
];

fn is_reserved_field(name: &str) -> bool {
    RESERVED_FIELDS.contains(&name)
}

/// Decode the length-prefixed key/value buffer emitted by
/// `vsdd_hook_sdk::host::encode_fields`.
///
/// ```text
/// [ key_len u32 LE | key bytes | value_len u32 LE | value bytes ]+
/// ```
///
/// Returns `Err` only on truncation — the host does not reject
/// non-UTF-8 key/value bytes (they're lossily converted with
/// `String::from_utf8_lossy` so malformed input doesn't drop the
/// whole event).
pub fn decode_fields(bytes: &[u8]) -> Result<Vec<(String, String)>, &'static str> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if i + 4 > bytes.len() {
            return Err("truncated key length");
        }
        let klen = u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap()) as usize;
        i += 4;
        if i + klen > bytes.len() {
            return Err("truncated key bytes");
        }
        let key = String::from_utf8_lossy(&bytes[i..i + klen]).into_owned();
        i += klen;
        if i + 4 > bytes.len() {
            return Err("truncated value length");
        }
        let vlen = u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap()) as usize;
        i += 4;
        if i + vlen > bytes.len() {
            return Err("truncated value bytes");
        }
        let val = String::from_utf8_lossy(&bytes[i..i + vlen]).into_owned();
        i += vlen;
        out.push((key, val));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(pairs: &[(&str, &str)]) -> Vec<u8> {
        let mut buf = Vec::new();
        for (k, v) in pairs {
            buf.extend_from_slice(&(k.len() as u32).to_le_bytes());
            buf.extend_from_slice(k.as_bytes());
            buf.extend_from_slice(&(v.len() as u32).to_le_bytes());
            buf.extend_from_slice(v.as_bytes());
        }
        buf
    }

    #[test]
    fn decode_single_pair() {
        let buf = encode(&[("k", "v")]);
        let out = decode_fields(&buf).unwrap();
        assert_eq!(out, vec![("k".to_string(), "v".to_string())]);
    }

    #[test]
    fn decode_multiple_pairs() {
        let buf = encode(&[("a", "1"), ("bb", "22"), ("ccc", "333")]);
        let out = decode_fields(&buf).unwrap();
        assert_eq!(out.len(), 3);
        assert_eq!(out[2].0, "ccc");
        assert_eq!(out[2].1, "333");
    }

    #[test]
    fn decode_empty_buffer_yields_empty_vec() {
        assert_eq!(decode_fields(&[]).unwrap(), Vec::new());
    }

    #[test]
    fn decode_rejects_truncated_key_length() {
        let bad = [0u8, 0, 0]; // 3 bytes, need 4 for length prefix
        assert!(decode_fields(&bad).is_err());
    }

    #[test]
    fn decode_rejects_truncated_key_bytes() {
        let mut bad = Vec::new();
        bad.extend_from_slice(&5u32.to_le_bytes());
        bad.extend_from_slice(b"ab"); // only 2 of 5 bytes
        assert!(decode_fields(&bad).is_err());
    }

    #[test]
    fn reserved_fields_rejected() {
        for name in RESERVED_FIELDS {
            assert!(is_reserved_field(name), "{name} should be a reserved field");
        }
    }

    #[test]
    fn non_reserved_field_accepted() {
        assert!(!is_reserved_field("commit_sha"));
        assert!(!is_reserved_field("file_path"));
    }
}
