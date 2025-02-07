use std::process::{Command, exit};
fn main() {
    let env_vars = [
        "CARGO_MANIFEST_DIR",
        "OUT_DIR",
        "TARGET",
        "HOST",
        "NUM_JOBS",
        "OPT_LEVEL",
        "DEBUG",
        "PROFILE",
        "CARGO_PKG_VERSION",
        "CARGO_PKG_VERSION_MAJOR",
        "CARGO_PKG_VERSION_MINOR",
        "CARGO_PKG_VERSION_PATCH",
        "CARGO_PKG_VERSION_PRE",
        "CARGO_PKG_NAME",
        "CARGO_PKG_DESCRIPTION",
        "CARGO_PKG_HOMEPAGE",
        "CARGO_PKG_REPOSITORY",
        "CARGO_PKG_LICENSE",
        "CARGO_PKG_AUTHORS",
    ];

    for &var in &env_vars {
        if let Ok(value) = std::env::var(var) {
            println!("cargo:warning={}: {}", var, value);
        }
    }

    // Print all CARGO_FEATURE_* variables
    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_FEATURE_") {
            println!("cargo:warning={}: {}", key, value);
        }
    }

    // Print all CARGO_CFG_* variables
    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_CFG_") {
            println!("cargo:warning={}: {}", key, value);
        }
    }

    if let Ok(host) = std::env::var("HOST") {
        let host = host.as_str();
        if let Ok(target) = std::env::var("TARGET") {
            let target = target.as_str();
            match host {
                "x86_64-pc-windows-msvc" => {
                    let choco_installed = Command::new("cmd")
                        .args(&["/C", "choco --version"])
                        .output()
                        .expect("Failed to check if Chocolatey is installed");

                    if !choco_installed.status.success() {
                        println!("cargo:warning=Chocolatey is not installed. Installing Chocolatey...");
                        let choco_installed = Command::new("cmd")
                            .args(&["/C", "Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))"])
                            .status()
                            .expect("Failed to install Chocolatey");
                        if !choco_installed.success() {
                            println!("cargo:error=Failed to install Chocolatey");
                            std::process::exit(1);
                        }
                    }
                    match target {
                        "x86_64-pc-windows-msvc" => {
                        }
                        "aarch64-pc-windows-msvc" => {
                            let vs_is_installed = Command::new("cmd")
                                .args(&["/C", "choco list --local-only visualstudio2019buildtools"])
                                .output()
                                .expect("Failed to check if Visual Studio Build Tools is installed");
                            if !vs_is_installed.status.success() || !String::from_utf8_lossy(&vs_is_installed.stdout).contains("visualstudio2019buildtools") {
                                // Install Visual Studio Build Tools with the required components
                                println!("cargo:warning=Installing Visual Studio Build Tools...");
                                let vs_installed = Command::new("cmd")
                                    .args(&["/C", "choco install visualstudio2019buildtools --package-parameters \"--add Microsoft.VisualStudio.Workload.VCTools --add Microsoft.VisualStudio.Component.VC.14.29.16.11.ARM64 --add Microsoft.VisualStudio.Component.Windows10SDK.19041 --includeRecommended --quiet --wait --norestart\""])
                                    .status()
                                    .expect("Failed to install Visual Studio Build Tools");
                                if !vs_installed.success() {
                                    println!("cargo:error=Failed to install Visual Studio Build Tools");
                                    std::process::exit(1);
                                }
                            };
                            println!("cargo:warning=setting environment variables for the cross compiler/linker");
                            std::env::set_var("CC", "cl.exe");
                            std::env::set_var("CXX", "cl.exe");
                            std::env::set_var("AR", "lib.exe");
                            std::env::set_var("CARGO_TARGET_AARCH64_PC_WINDOWS_MSVC_LINKER", "link.exe");
                            std::env::set_var("CARGO_TARGET_AARCH64_PC_WINDOWS_MSVC_RUNNER", "link.exe");                         
                        }
                        _ => {
                            println!("cargo:error=Cross compiling from {} to {} not supported!", host, target);
                            std::process::exit(1);
    
                        }
                    }
                }
                "x86_64-unknown-linux-gnu" => {
                    match target {
                        "x86_64-unknown-linux-gnu" => {
                        }
                        "aarch64-unknown-linux-gnu" => {
                            println!("cargo:warning=Cross compiling from x86_64-unknown-linux-gnu to aarch64-unknown-linux-gnu");
                            //TODO: implement the installation of the cross compiler/linker here
                            //TODO: set the environment variables for the cross compiler/linker here
                        }
                        _ => {
                            println!("cargo:error=Cross compiling from {} to {} not supported!", host, target);
                            std::process::exit(1);
                        }
                    }
                }
                _ => {
                    println!("cargo:warning=Cross compiling from {} to {} not supported!", host, target);
                    std::process::exit(1);
                }
            }
        } else {
            println!("cargo:error=TARGET not found");
            std::process::exit(1);
        }
    } else {
        println!("cargo:error=HOST not found");
        std::process::exit(1);
    }
}