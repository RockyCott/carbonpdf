#[cfg(feature = "cli")]
fn main() -> carbonpdf::Result<()> {
    carbonpdf::cli::run()
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("The carbonpdf CLI is disabled. Rebuild with --features cli.");
}
