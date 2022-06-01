#!/bin/bash

# TODO Borrow the increment-cargo-version.sh script from Solana

# Get new version
current_version=$(cat ./VERSION)
echo "Current version: $current_version"
read -r -p "    New version: " new_version

# Build
RUSTFLAGS="--deny warnings" cargo build || (echo "Build failed" && exit)

# Bump cronos-crono 
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' cron/Cargo.toml

# Bump programs
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/health/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/network/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/pool/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/scheduler/Cargo.toml

# Bump inter-program dependencies
sed -i '' -e 's/^cronos-pool =.*/cronos-pool = { path = "..\/pool", features = ["cpi"], version = "'${new_version}'" }/g' programs/network/Cargo.toml
sed -i '' -e 's/^cronos-scheduler =.*/cronos-scheduler = { path = "..\/scheduler", features = ["cpi"], version = "'${new_version}'" }/g' programs/network/Cargo.toml
sed -i '' -e 's/^cronos-cron =.*/cronos-cron = { path = "..\/..\/cron", version = "'${new_version}'" }/g' programs/scheduler/Cargo.toml
sed -i '' -e 's/^cronos-pool =.*/cronos-pool = { path = "..\/pool", features = ["cpi"], version = "'${new_version}'" }/g' programs/scheduler/Cargo.toml


# Bump cronos-client
sed -i '' -e 's/^cronos-health =.*/cronos-health = { path = "..\/programs\/health", features = ["no-entrypoint"], version = "'${new_version}'" }/g' client/Cargo.toml
sed -i '' -e 's/^cronos-network =.*/cronos-network = { path = "..\/programs\/network", features = ["no-entrypoint"], version = "'${new_version}'" }/g' client/Cargo.toml
sed -i '' -e 's/^cronos-pool =.*/cronos-pool = { path = "..\/programs\/pool", features = ["no-entrypoint"], version = "'${new_version}'" }/g' client/Cargo.toml
sed -i '' -e 's/^cronos-scheduler =.*/cronos-scheduler = { path = "..\/programs\/scheduler", features = ["no-entrypoint"], version = "'${new_version}'" }/g' client/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' client/Cargo.toml

# Bump cronos-cli
sed -i '' -e 's/^cronos-client =.*/cronos-client = { path = "..\/client", version = "'${new_version}'" }/g' cli/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' cli/Cargo.toml

# Bump cronos-metrics
sed -i '' -e 's/^cronos-client =.*/cronos-client = { path = "..\/client", version = "'${new_version}'" }/g' metrics/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' metrics/Cargo.toml

# Bump cronos-plugin
sed -i '' -e 's/^cronos-client =.*/cronos-client = { path = "..\/client", version = "'${new_version}'" }/g' plugin/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' plugin/Cargo.toml

# Bump cronos-stress
sed -i '' -e 's/^cronos-client =.*/cronos-client = { path = "..\/client", version = "'${new_version}'" }/g' stress/Cargo.toml
sed -i '' -e 's/^cronos-cron =.*/cronos-cron = { path = "..\/cron", version = "'${new_version}'" }/g' stress/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' stress/Cargo.toml

# Update version
echo $new_version > VERSION

# Wait for Cargo.toml update
sleep 25

# Git commit 
echo "$(git diff --stat | tail -n1)"
git add .
git commit -m "Bump from $current_version to $new_version"
git tag "v$new_version"
git push && git push --tags
