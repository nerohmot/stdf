#!/usr/bin/env rust-script
//! ```cargo
//! [package]
//! edition = "2021"
//! version = "1.0.0"
//!
//! [dependencies]
//! cbindgen = "0.20"
//! clap = "4.0"
//! lazy_static = "1.5"
//! ```

use std::env;
use std::fs;
use std::path::PathBuf;
use cbindgen::{Config, Language};
use clap::{Arg, Command};

lazy_static! {
    static ref EXTENSIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("c", "h");
        m.insert("cxx", "hpp");
        m.insert("cython", "pxd");
        m
    };    
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("create_header_files")
        .arg(Arg::new("target")
            .short('t')
            .long("target")
            .required(true)
            .possible_values(["x86_64-unknown-linux-gnu", 
                              "x86_64-apple-darwin", 
                              "x86_64-pc-windows-msvc",
                              "aarch64-unknown-linux-gnu",
                              "aarch64-apple-darwin",
                              "aarch64-pc-windows-msvc"])
        )
        .arg(Arg::new("language")
            .short('l')
            .long("language")
            .required(true)
            .possible_values(["c", "cxx", "cython"])
        )
        .get_matches();

    let build_target = matches.get_one::<String>("target").unwrap();

    let mut current_dir: PathBuf = env::current_dir()?;
    while !current_dir.join("Cargo.toml").exists() {
        if !current_dir.pop() {
            println!("Couldn't find the crate root directory!");
            return Err("Couldn't find the crate root directory".into());
        }
    }
    let crate_root_dir = current_dir.clone();
    let cbindgen_toml_path = current_dir.clone().join("cbindgen.toml");
    if !cbindgen_toml_path.exists() {
        println!("cbindgen.toml not found in the crate root directory: {}", crate_root_dir.display());
        return Err("cbindgen.toml not found".into());
    }
    if let Some(target) = matches.get_one::<String>("target") {
        let release_dir = current_dir.clone().join("target").join(target).join("release");
        if !release_dir.exists() {
            println!("No release build found : {}", release_dir.display());
            return Err("No release build found".into());
        }
    } else {
        println!("No --target argument provided!");
        return Err("No --target argument provided.".into());
    }
    let include_dir = current_dir
        .clone()
        .join("target")
        .join(build_target)
        .join("release")
        .join("include");
    if include_dir.exists() {
        fs::remove_dir_all(&include_dir)?;
    }
    fs::create_dir(&include_dir)?;






    let config = Config::from_file(cbindgen_toml_path)?;





    
    let c_header_path = include_dir.join("stdf.h");
    cbindgen::Builder::new()
        .with_config(config.clone())
        .with_crate(&crate_root_dir)
        .with_language(Language::C)
        .generate()?
        .write_to_file(&c_header_path);

    let cpp_header_path = include_dir.join("stdf.hpp");
    cbindgen::Builder::new()
        .with_config(config.clone())
        .with_crate(&crate_root_dir)
        .with_language(Language::Cxx)
        .generate()?
        .write_to_file(&cpp_header_path);

    let cython_header_path = include_dir.join("stdf.pxd");
    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(&crate_root_dir)
        .with_language(Language::Cython)
        .generate()?
        .write_to_file(&cython_header_path);

    Ok(())
}
