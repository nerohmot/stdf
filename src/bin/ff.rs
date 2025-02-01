use file_format::{FileFormat, Kind};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "tests/fixtures/diamond07_1_3855_rev_F3E_M_Default_01_07122011_155821.std.7z";

    // Check if the file exists
    if !Path::new(file_path).exists() {
        eprintln!("File does not exist: {}", file_path);
        return Ok(());
    }

    let fmt = FileFormat::from_file(file_path)?;

    println!("{:?}", fmt);
    println!("{:?}", fmt.name());
    println!("{:?}", fmt.short_name());
    println!("{:?}", fmt.media_type());
    println!("{:?}", fmt.extension());
    println!("{:?}", fmt.kind());

    Ok(())
}
