;; long_running_resolver.wat — WASM resolver fixture that spins in an infinite
;; loop, exercising the epoch-deadline timeout path.
;;
;; The resolver exports `resolve(i32, i32) -> i64` per BC-4.12.002 PC1.
;; The function enters a tight loop (`loop` / `br`) that never exits.
;; When the epoch deadline fires, wasmtime interrupts execution and returns
;; a trap that the dispatcher classifies as ResolverError::Timeout.
;;
;; Used by: test_F_P2_001_epoch_deadline_fires_resolver_timeout (F-P2-001 regression).
;;
;; Build: wasm-tools parse long_running_resolver.wat -o long_running_resolver.wasm
(module
  (memory (export "memory") 1)
  (func (export "resolve") (param i32 i32) (result i64)
    (block $exit
      (loop $loop
        ;; spin forever — epoch interruption fires ResolverError::Timeout
        (br $loop)
      )
    )
    i64.const 0))
