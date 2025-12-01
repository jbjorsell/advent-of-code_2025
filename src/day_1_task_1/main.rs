use tracing::{debug, info};

fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let input = read_input()?;
    info!("Input length: {}", input.len());
    debug!("Input content: {}", input);
    Ok(())
}

fn read_input() -> Result<String, std::io::Error> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = std::path::Path::new(manifest_dir).join("input.txt");
    std::fs::read_to_string(path)
}
