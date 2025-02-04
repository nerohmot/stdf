// cargo-deps: toml = "0.5.8"
// Version: 1.0.0

use std::env;
use std::fs;
use std::path::Path;
use toml::Value;

fn main() {
    // Get the path to Cargo.toml from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path-to-Cargo.toml>", args[0]);
        std::process::exit(1);
    }
    let cargo_toml_path = &args[1];

    // Get the new version from the environment variable
    let new_version = env::var("VERSION").expect("VERSION environment variable not set");

    // Read the contents of Cargo.toml
    let cargo_toml_content = fs::read_to_string(cargo_toml_path)
        .expect("Failed to read Cargo.toml");

    // Parse the TOML content
    let mut cargo_toml: Value = cargo_toml_content.parse()
        .expect("Failed to parse Cargo.toml");

    // Update the version string
    if let Some(package) = cargo_toml.get_mut("package") {
        if let Some(version) = package.get_mut("version") {
            *version = Value::String(new_version.clone());
        } else {
            eprintln!("No version field found in Cargo.toml");
            std::process::exit(1);
        }
    } else {
        eprintln!("No package section found in Cargo.toml");
        std::process::exit(1);
    }

    // Serialize the updated TOML content
    let updated_cargo_toml_content = toml::to_string(&cargo_toml)
        .expect("Failed to serialize updated Cargo.toml");

    // Write the updated content back to Cargo.toml
    fs::write(cargo_toml_path, updated_cargo_toml_content)
        .expect("Failed to write updated Cargo.toml");

    println!("Updated version number to {}", new_version);
}