;; trapping_resolver.wat — minimal WASM resolver fixture for VP-074 integration tests.
;;
;; Exports `resolve(i32, i32) -> i64` which immediately executes `unreachable`.
;; Used to verify that a trapping resolver does not abort dispatch
;; (BC-4.12.004 crash isolation contract — S-12.04 AC-010/AC-011).
;;
;; Build: wasm-tools parse trapping_resolver.wat -o trapping_resolver.wasm
(module
  (memory (export "memory") 1)
  (func (export "resolve") (param i32 i32) (result i64)
    unreachable))
