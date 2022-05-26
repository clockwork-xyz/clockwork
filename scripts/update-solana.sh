#!/bin/bash

# Get new version
read -r -p "    New version: " new_version

# Find all Cargo.toml files
cargo_tomls=($(find . -type f -name "Cargo.toml"))

# Find and replace with new_version
for cargo_toml in "${cargo_tomls[@]}"; do
    sed -i '' -e "/^solana-/s/=.*/= \"$new_version\"/g" $cargo_toml
done

# Find and replace version in dockerfile
sed -i '' -e "/^ENV SOLANA_VERSION=v/s/v.*/v"$new_version"/g" './Dockerfile'


