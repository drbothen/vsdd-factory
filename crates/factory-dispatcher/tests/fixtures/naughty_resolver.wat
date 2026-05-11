;; naughty_resolver.wat — WASM resolver fixture for VP-076 capability confinement tests.
;;
;; Attempts to read /etc/passwd via the vsdd::read_file host function.
;; When path_allow = [".factory/"] (or narrower), the host MUST return
;; CAPABILITY_DENIED (i32 code -3) and NOT return any /etc/passwd bytes.
;;
;; The resolver returns an empty JSON object ("{}") as its output regardless
;; of whether the read succeeds or fails. The test verifies:
;;   1. The dispatcher does NOT crash.
;;   2. resolver.capability_denied appears in VSDD_SINK_FILE.
;;   3. /etc/passwd content does NOT appear in any dispatcher output.
;;
;; Host ABI (vsdd::read_file per BC-4.12.003 PC2):
;;   read_file(path_ptr, path_len, max_bytes, timeout_ms, out_ptr_out, out_len_out) -> i32
;;   Returns: 0 = OK, negative = error (e.g., -3 = CAPABILITY_DENIED).
;;
;; Resolver ABI (BC-4.12.002 PC1):
;;   resolve(input_ptr: i32, input_len: i32) -> i64  (packed (ptr << 32 | len))
;;
;; The output is written at memory offset 65536 (page 1, safely past input).
;;
;; Build: wasm-tools parse naughty_resolver.wat -o naughty_resolver.wasm
(module
  ;; Import vsdd::read_file from the host (imports must come before memory).
  (import "vsdd" "read_file" (func $read_file
    (param i32 i32 i32 i32 i32 i32)
    (result i32)))

  (memory (export "memory") 2)

  ;; Path string: /etc/passwd — stored at offset 1024 in page 0.
  ;; Length: 11 bytes.
  (data (i32.const 1024) "/etc/passwd")

  ;; Output JSON: {} — stored at offset 65536 (page 1).
  ;; Length: 2 bytes.
  (data (i32.const 65536) "{}")

  ;; resolve(input_ptr: i32, input_len: i32) -> i64
  ;; Attempts /etc/passwd read (capability_denied expected), then returns {}.
  (func (export "resolve") (param i32 i32) (result i64)
    ;; Attempt to read /etc/passwd via vsdd::read_file.
    ;; path_ptr=1024, path_len=11, max_bytes=65536, timeout_ms=1000
    ;; out_ptr_out=2048, out_len_out=2052 (scratch area for host to write output ptr/len)
    (drop
      (call $read_file
        (i32.const 1024)  ;; path_ptr: offset of "/etc/passwd"
        (i32.const 11)    ;; path_len: len("/etc/passwd")
        (i32.const 65536) ;; max_bytes
        (i32.const 1000)  ;; timeout_ms
        (i32.const 2048)  ;; out_ptr_out (scratch)
        (i32.const 2052)  ;; out_len_out (scratch)
      )
    )
    ;; Return "{}" at offset 65536, length 2.
    ;; packed i64: (ptr << 32) | len
    (i64.or
      (i64.shl
        (i64.const 65536)  ;; ptr
        (i64.const 32))
      (i64.const 2)        ;; len
    )
  )
)
