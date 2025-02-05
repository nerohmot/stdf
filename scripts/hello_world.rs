fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    println!("Hello, World! {}", crate_dir);
    Ok(())
}