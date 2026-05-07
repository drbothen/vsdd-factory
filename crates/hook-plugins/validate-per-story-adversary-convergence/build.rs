fn main() {
    // Allow `#[cfg(kani)]` in VP-071 proof harnesses without triggering the
    // unexpected_cfgs lint during normal `cargo check` / `cargo test`.
    // Kani sets this cfg itself when invoked via `cargo kani`.
    println!("cargo::rustc-check-cfg=cfg(kani)");
}
