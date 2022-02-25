#!/bin/bash

# Get new version
read -p 'New version: ' new_version
echo "Bumping from $(cat VERSION) to ${new_version}"
old_version=$(cat VERSION)

# Build
RUSTFLAGS="--deny warnings" cargo build || (echo "❌  Build failed!" && exit)

# Bump cronos-program 
sed -i '' -e '3s/^version = "'${old_version}'"/version = "'${new_version}'"/g' programs/cronos/Cargo.toml

# Bump cronos-sdk
sed -i '' -e 's/^cronos-program =.*/cronos-program = { path = "..\/programs\/cronos", features = ["no-entrypoint"], version = "'${new_version}'" }/g' sdk/Cargo.toml
sed -i '' -e '3s/^version = "'${old_version}'"/version = "'${new_version}'"/g' sdk/Cargo.toml

# Bump cronos-bot
sed -i '' -e 's/^cronos-sdk =.*/cronos-sdk = { path = "..\/sdk", version = "'${new_version}'" }/g' bot/Cargo.toml
sed -i '' -e '3s/^version = "'${old_version}'"/version = "'${new_version}'"/g' bot/Cargo.toml

# Bump cronos-cli
sed -i '' -e 's/^cronos-sdk =.*/cronos-sdk = { path = "..\/sdk", version = "'${new_version}'" }/g' cli/Cargo.toml
sed -i '' -e '3s/^version = "'${old_version}'"/version = "'${new_version}'"/g' cli/Cargo.toml

# Update version
echo $new_version > VERSION
echo "$(git diff --stat | tail -n1)"

# Cargo publish
cargo publish --allow-dirty --manifest-path programs/cronos/Cargo.toml
cargo publish --allow-dirty --manifest-path sdk/Cargo.toml
cargo publish --allow-dirty --manifest-path bot/Cargo.toml
cargo publish --allow-dirty --manifest-path cli/Cargo.toml

# Re-build
cargo build

# Git commit 
git checkout -b release/${new_version}
git add .
git commit -m "Bump to $new_version"
git tag "v$new_version"

exit 
