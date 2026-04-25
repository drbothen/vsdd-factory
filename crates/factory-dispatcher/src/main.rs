fn main() {
    eprintln!("{STUB_BANNER}");
    std::process::exit(0);
}

const STUB_BANNER: &str = "vsdd-factory-dispatcher pre-implementation stub";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_banner_is_non_empty() {
        assert!(!STUB_BANNER.is_empty());
    }

    #[test]
    fn stub_banner_mentions_dispatcher() {
        assert!(STUB_BANNER.contains("dispatcher"));
    }
}
