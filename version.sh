#!/bin/bash

# Get new version
current_version=$(cat ./VERSION)
echo "Current version: $current_version"
read -r -p "    New version: " new_version

# Build
RUSTFLAGS="--deny warnings" cargo build || (echo "Build failed" && exit)

# Bump cronos-crono 
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' cron/Cargo.toml

# Bump programs
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/cronos/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/heartbeat/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/pool/Cargo.toml

# Bump cronos-sdk
sed -i '' -e 's/^cronos-program =.*/cronos-program = { path = "..\/programs\/cronos", features = ["no-entrypoint"], version = "'${new_version}'" }/g' sdk/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' sdk/Cargo.toml

# Bump cronos-bot
sed -i '' -e 's/^cronos-sdk =.*/cronos-sdk = { path = "..\/sdk", version = "'${new_version}'" }/g' bot/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' bot/Cargo.toml

# Bump cronos-cli
sed -i '' -e 's/^cronos-sdk =.*/cronos-sdk = { path = "..\/sdk", version = "'${new_version}'" }/g' cli/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' cli/Cargo.toml

# Bump cronos-telemetry
sed -i '' -e 's/^cronos-sdk =.*/cronos-sdk = { path = "..\/sdk", version = "'${new_version}'" }/g' telemetry/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' telemetry/Cargo.toml

# Bump cronos-tests
sed -i '' -e 's/^cronos-sdk =.*/cronos-sdk = { path = "..\/sdk", version = "'${new_version}'" }/g' tests/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' tests/Cargo.toml

# Update version
echo $new_version > VERSION

# Wait for Cargo.toml update
sleep 10

# Git commit 
echo "$(git diff --stat | tail -n1)"
git checkout -b release/${new_version}
git add .
git commit -m "Bump to $new_version"
git tag "v$new_version"
git push --set-upstream origin release/${new_version} --tags

exit 
