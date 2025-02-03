//! ```cargo
//! [dependencies]
//! cargo-msrv = "0.17.1"
//! 
//! [package]
//! edition = "2021"
//! ```

use std::process::Command;
use std::str;

fn main() {
    // Run the cargo-msrv command
    let output = Command::new("cargo")
        .args(&["msrv", "find", "--output-format", "json"])
        .output()
        .expect("Failed to execute cargo-msrv");

    if output.status.success() {
        // Parse the output as JSON and extract the MSRV
        let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");
        println!("cargo-msrv output: {:?}", stdout);

        // Here you would parse the JSON to extract the MSRV
        // This is a placeholder for JSON parsing logic
        // For example, you might use serde_json to parse the output
    } else {
        let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8 error output");
        eprintln!("Error running cargo-msrv: {:?}", stderr);
    }
}