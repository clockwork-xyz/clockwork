#!/bin/bash

# TODO Borrow the increment-cargo-version.sh script from Solana

# Get new version
current_version=$(cat ./VERSION)
echo "Current version: $current_version"
read -r -p "    New version: " new_version

# Build
RUSTFLAGS="--deny warnings" cargo build || (echo "Build failed" && exit)

# Bump clockwork-cron
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' cron/Cargo.toml

# Bump programs
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/crank/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/http/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/network/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' programs/pool/Cargo.toml

# Bump inter-program dependencies
sed -i '' -e 's/^clockwork-cron =.*/clockwork-cron = { path = "..\/..\/cron", version = "'${new_version}'" }/g' programs/crank/Cargo.toml
sed -i '' -e 's/^clockwork-pool =.*/clockwork-pool = { path = "..\/pool", features = ["cpi"], version = "'${new_version}'" }/g' programs/crank/Cargo.toml
sed -i '' -e 's/^clockwork-pool =.*/clockwork-pool = { path = "..\/pool", features = ["cpi"], version = "'${new_version}'" }/g' programs/network/Cargo.toml
sed -i '' -e 's/^clockwork-crank =.*/clockwork-crank = { path = "..\/crank", features = ["cpi"], version = "'${new_version}'" }/g' programs/network/Cargo.toml
sed -i '' -e 's/^clockwork-pool =.*/clockwork-pool = { path = "..\/pool", features = ["cpi"], version = "'${new_version}'" }/g' programs/http/Cargo.toml

# Bump clockwork-client
sed -i '' -e 's/^clockwork-http =.*/clockwork-http = { path = "..\/programs\/http", features = ["no-entrypoint"], version = "'${new_version}'" }/g' client/Cargo.toml
sed -i '' -e 's/^clockwork-network =.*/clockwork-network = { path = "..\/programs\/network", features = ["no-entrypoint"], version = "'${new_version}'" }/g' client/Cargo.toml
sed -i '' -e 's/^clockwork-pool =.*/clockwork-pool = { path = "..\/programs\/pool", features = ["no-entrypoint"], version = "'${new_version}'" }/g' client/Cargo.toml
sed -i '' -e 's/^clockwork-crank =.*/clockwork-crank = { path = "..\/programs\/crank", features = ["no-entrypoint"], version = "'${new_version}'" }/g' client/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' client/Cargo.toml

# Bump clockwork-bench
sed -i '' -e 's/^clockwork-client =.*/clockwork-client = { path = "..\/client", version = "'${new_version}'" }/g' bench/Cargo.toml
sed -i '' -e 's/^clockwork-cron =.*/clockwork-cron = { path = "..\/cron", version = "'${new_version}'" }/g' bench/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' bench/Cargo.toml

# Bump clockwork-cli
sed -i '' -e 's/^clockwork-client =.*/clockwork-client = { path = "..\/client", version = "'${new_version}'" }/g' cli/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' cli/Cargo.toml

# Bump clockwork-plugin
sed -i '' -e 's/^clockwork-client =.*/clockwork-client = { path = "..\/client", version = "'${new_version}'" }/g' plugin/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' plugin/Cargo.toml

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
