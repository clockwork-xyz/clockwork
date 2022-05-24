#!/bin/bash

# Get new version
current_version=$(cat ./VERSION)
echo "Current version: $current_version"
read -r -p "    New version: " new_version

# Find all Cargo.toml files
cargo_tomls=($(find . -type f -name "Cargo.toml"))

# Find and replace with new_version
for cargo_toml in "${cargo_tomls[@]}"; do
    sed -i '' -e "/^solana-/s/=.*/= \"$new_version\"/g" $cargo_toml
done