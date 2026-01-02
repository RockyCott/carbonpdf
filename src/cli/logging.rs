use tracing::Level;

pub fn init(verbose: bool) {
    if verbose {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    }
}
