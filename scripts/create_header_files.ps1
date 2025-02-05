#!/usr/bin/env pwsh
# PowerShell script to call the Rust script 'create_header_files.rs' and pass through the arguments

Write-Host "Checking if rust-script is installed..." -NoNewline
# Ensure rust-script is installed
if (-not (Get-Command rust-script -ErrorAction SilentlyContinue)) {
    Write-Host " Done. (Not installed)"
    Write-Host "rust-script is not installed. Installing..." -NoNewline
    cargo install rust-script
}
Write-Host " Done."

$cwd = (Get-Location).Path
Write-Host "Current working directory: $cwd"

$scriptDir = $PSScriptRoot
Write-Host "Script directory: $scriptDir"

Write-Host "Checking if rust script exists ..." -NoNewline
$rustScriptPath = "create_header_files.rs"
# Check if the Rust script exists
if (-not (Test-Path $rustScriptPath)) {
    Write-Host " Done. (Not found)"
    Write-Host "Rust script '$rustScriptPath' not found."
    exit 1
}
Write-Host " Done. (found)"

Write-Host @args

Write-Host "Running Rust script..." -NoNewline
rust-script $rustScriptPath @args
Write-Host " Done."