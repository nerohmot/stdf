#!/bin/bash

# Create a directory for the installer
mkdir -p hvcl_tc_installer

# Copy the binary and any necessary files
cp target/release/hvcl_tc hvcl_tc_installer/

# Create the makeself installer
makeself hvcl_tc_installer hvcl_tc_installer.run "HVCL_TC Installer" ./hvcl_tc
