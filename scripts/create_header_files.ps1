#!/usr/bin/env pwsh
# PowerShell script to call the Rust script 'create_header_files.rs' and pass through the arguments

param (
    [string] $Target
    [string] $Language
)

$allowedTargets = @(
    "x86_64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "x86_64-pc-windows-msvc",
    "aarch64-unknown-linux-gnu",
    "aarch64-apple-darwin",
    "aarch64-pc-windows-msvc"
)

if (-not $Target -or -not $allowedTargets -contains $Target) {
    Write-Host "Error: The -Target argument is required and must be one of the following values:"
    Write-Host $allowedTargets -join ", "
    Write-Host "Usage: ./create_header_files.ps1 -Target <target> -Language <language>"
    exit 1
} 

$sllowedLanguages = @(
    "c",
    "cxx",
    "cython"
)

if (-not $Language -or -not $allowedLanguages -contains $Language) {
    Write-Host "Error: The -Language argument is required and must be one of the following values:"
    Write-Host $allowedLanguages -join ", "
    Write-Host "Usage: ./create_header_files.ps1 -Target <target> -Language <language>"
    exit 1
}

$rustArgs = @("--target", $Target, "--language", $Language)

Write-Host "Checking if rust-script is installed..." -NoNewline
if (-not (Get-Command rust-script -ErrorAction SilentlyContinue)) {
    Write-Host " Done. (Not installed)"
    Write-Host "Installing rust-script ..." -NoNewline
    cargo install rust-script
}
Write-Host " Done. (rust-script is installed)"

$scriptDir = $PSScriptRoot

Write-Host "Script directory: $scriptDir"
write-Host "arguments : $rustArgs" 

# rust-script $rustScriptPath @args