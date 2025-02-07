use std::process;
fn main() {
    println!("cargo:error=Boe");
    process::exit(1);
    println!("cargo:warning=shouldn't see this one");
//     let env_vars = [
//         "CARGO_MANIFEST_DIR",
//         "OUT_DIR",
//         "TARGET",
//         "HOST",
//         "NUM_JOBS",
//         "OPT_LEVEL",
//         "DEBUG",
//         "PROFILE",
//         "CARGO_PKG_VERSION",
//         "CARGO_PKG_VERSION_MAJOR",
//         "CARGO_PKG_VERSION_MINOR",
//         "CARGO_PKG_VERSION_PATCH",
//         "CARGO_PKG_VERSION_PRE",
//         "CARGO_PKG_NAME",
//         "CARGO_PKG_DESCRIPTION",
//         "CARGO_PKG_HOMEPAGE",
//         "CARGO_PKG_REPOSITORY",
//         "CARGO_PKG_LICENSE",
//         "CARGO_PKG_AUTHORS",
//     ];

//     for &var in &env_vars {
//         if let Ok(value) = std::env::var(var) {
//             println!("cargo:warning={}: {}", var, value);
//         }
//     }

//     // Print all CARGO_FEATURE_* variables
//     for (key, value) in std::env::vars() {
//         if key.starts_with("CARGO_FEATURE_") {
//             println!("cargo:warning={}: {}", key, value);
//         }
//     }

//     // Print all CARGO_CFG_* variables
//     for (key, value) in std::env::vars() {
//         if key.starts_with("CARGO_CFG_") {
//             println!("cargo:warning={}: {}", key, value);
//         }
//     }

}