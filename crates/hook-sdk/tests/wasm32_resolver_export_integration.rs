//! Integration test: verify that the `#[resolver]` macro generates a `resolve`
//! WASM export when `wasm-resolver-export` is compiled for `wasm32-wasip1`.
//!
//! # Setup required to run (remove `#[ignore]`)
//!
//! 1. Install wasm32-wasip1 toolchain:
//!    `rustup target add wasm32-wasip1`
//!
//! 2. Build the example crate:
//!    `cargo build --target wasm32-wasip1 -p wasm-resolver-export --release`
//!
//! 3. Run this test without `--ignored`:
//!    `cargo test --features resolver-authoring -p vsdd-hook-sdk --test wasm32_resolver_export_integration`
//!
//! # What this test verifies
//!
//! The `#[resolver]` macro generates `#[cfg(target_arch = "wasm32")] pub extern "C" fn resolve(...)`.
//! On a host build this block is compiled out (correct — pointers are 64-bit on host).
//! This test shells out to `cargo build --target wasm32-wasip1` to compile the example
//! cdylib, then uses `wasmparser` to parse the export section and assert that
//! `resolve` appears as a function export.
//!
//! Traces: BC-4.12.002 PC1, PC5, F-P2-001.

#[cfg(feature = "resolver-authoring")]
mod wasm_export_tests {
    use std::path::PathBuf;
    use std::process::Command;

    /// Build the wasm-resolver-export cdylib for wasm32-wasip1 and return the
    /// path to the resulting .wasm file, or an error string if the build fails.
    fn build_wasm_cdylib() -> Result<PathBuf, String> {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .map_err(|_| "CARGO_MANIFEST_DIR not set".to_string())?;

        // The workspace root is two levels up from crates/hook-sdk/
        let workspace_root = PathBuf::from(&manifest_dir)
            .parent()
            .and_then(|p| p.parent())
            .ok_or("could not find workspace root")?
            .to_path_buf();

        let output = Command::new("cargo")
            .args([
                "build",
                "--target",
                "wasm32-wasip1",
                "-p",
                "wasm-resolver-export",
                "--release",
            ])
            .current_dir(&workspace_root)
            .output()
            .map_err(|e| format!("failed to invoke cargo: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "cargo build --target wasm32-wasip1 failed:\n{stderr}"
            ));
        }

        let wasm_path = workspace_root
            .join("target")
            .join("wasm32-wasip1")
            .join("release")
            .join("wasm_resolver_export.wasm");

        if !wasm_path.exists() {
            return Err(format!("expected .wasm at {}", wasm_path.display()));
        }

        Ok(wasm_path)
    }

    /// Parse the WASM export section and return all exported function names.
    fn wasm_function_exports(wasm_bytes: &[u8]) -> Vec<String> {
        use wasmparser::{ExternalKind, Parser, Payload};

        let mut exports = Vec::new();
        for payload in Parser::new(0).parse_all(wasm_bytes) {
            let payload = payload.expect("wasmparser: invalid payload");
            if let Payload::ExportSection(reader) = payload {
                for export in reader {
                    let export = export.expect("wasmparser: invalid export");
                    if export.kind == ExternalKind::Func {
                        exports.push(export.name.to_string());
                    }
                }
            }
        }
        exports
    }

    /// BC-4.12.002 PC1/PC5: `#[resolver]` generates a `resolve` WASM export.
    ///
    /// This test builds the `wasm-resolver-export` cdylib for `wasm32-wasip1`
    /// and verifies that `resolve` appears in the binary's export section.
    ///
    /// The test is ignored by default so CI hosts without the wasm32-wasip1
    /// toolchain don't fail. Run manually after `rustup target add wasm32-wasip1`.
    ///
    /// Traces: BC-4.12.002 PC1, PC5, F-P2-001.
    #[test]
    #[ignore = "WASM-export verification — requires wasm32-wasip1 toolchain. \
                Install: rustup target add wasm32-wasip1. \
                Run: cargo test --features resolver-authoring -p vsdd-hook-sdk \
                --test wasm32_resolver_export_integration -- --include-ignored"]
    fn test_BC_4_12_002_resolver_macro_generates_wasm_export() {
        let wasm_path = build_wasm_cdylib().unwrap_or_else(|e| panic!("{e}"));

        let wasm_bytes = std::fs::read(&wasm_path)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", wasm_path.display()));

        let exports = wasm_function_exports(&wasm_bytes);

        assert!(
            exports.contains(&"resolve".to_string()),
            "#[resolver] must generate a `resolve` function export in the WASM binary \
             (BC-4.12.002 PC1/PC5). Found exports: {:?}",
            exports
        );
    }
}
