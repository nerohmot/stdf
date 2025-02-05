#!/usr/bin/env rust-script
//! ```cargo
//! [package]
//! edition = "2021"
//! version = "1.0.0"
//!
//! [dependencies]
//! cbindgen = "0.20"
//! clap = { version = "4.0", features = ["derive"] }
//! ```

use std::path::PathBuf;
use cbindgen::{Config, Language};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Tom Hören", version = "1.0.0", about = "Cenerates C and C++ header files from Rust code", long_about = None)]
struct Args {
    #[arg(short, long)]
    out_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let config = Config::from_file("cbindgen.toml")?;

    let c_header_path = args.out_dir.join("bindings.h");
    cbindgen::Builder::new()
        .with_config(config.clone())
        .with_crate(&crate_dir)
        .with_language(Language::C)
        .generate()?
        .write_to_file(&c_header_path);


    let cpp_header_path = args.out_dir.join("bindings.hpp");
    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(&crate_dir)
        .with_language(Language::Cxx)
        .generate()?
        .write_to_file(&cpp_header_path);

    Ok(())
}