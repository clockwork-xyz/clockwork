#!/bin/bash

# Get new version
read -p 'New version: ' new_version
echo "Bumping from $(cat VERSION) to ${new_version}"
old_version=$(cat VERSION)

# Build
RUSTFLAGS="--deny warnings" cargo build || (echo "Build failed" && exit)

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

# Cargo publish
cargo publish --allow-dirty --manifest-path programs/cronos/Cargo.toml || (echo "Failed to publish cronos-program" && exit)
cargo publish --allow-dirty --manifest-path sdk/Cargo.toml || (echo "Failed to publish cronos-sdk" && exit)
cargo publish --allow-dirty --manifest-path bot/Cargo.toml || (echo "Failed to publish cronos-bot" && exit)
cargo publish --allow-dirty --manifest-path cli/Cargo.toml || (echo "Failed to publish cronos-cli" && exit)

# Update version
echo $new_version > VERSION

# Git commit 
echo "$(git diff --stat | tail -n1)"
git checkout -b release/${new_version}
git add .
git commit -m "Bump to $new_version"
git tag "v$new_version"
git push --set-upstream origin release/${new_version}

exit 
