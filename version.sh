#!/bin/bash

# Get new version
current_version=$(cat ./VERSION)
echo "Current version: $current_version"
read -p "    New version: $new_version" 
echo "Bumping from $current_version to $new_version"

echo $new_version

exit

# Build
RUSTFLAGS="--deny warnings" cargo build || (echo "Build failed" && exit)

# Bump cronos-program 
path=programs/cronos/Cargo.toml
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' path

# Bump cronos-sdk
path=sdk/Cargo.toml
sed -i '' -e 's/^cronos-program =.*/cronos-program = { path = "..\/programs\/cronos", features = ["no-entrypoint"], version = "'${new_version}'" }/g' path
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' path

# Bump cronos-bot
path=bot/Cargo.toml
sed -i '' -e 's/^cronos-sdk =.*/cronos-sdk = { path = "..\/sdk", version = "'${new_version}'" }/g' path
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' path

# Bump cronos-cli
path=cli/Cargo.toml
sed -i '' -e 's/^cronos-sdk =.*/cronos-sdk = { path = "..\/sdk", version = "'${new_version}'" }/g' path
sed -i '' -e '3s/^version = "'${current_version}'"/version = "'${new_version}'"/g' path

# Update version
echo $new_version > VERSION

# Git commit 
echo "$(git diff --stat | tail -n1)"
git checkout -b release/${new_version}
git add ..
git commit -m "Bump to $new_version"
git tag "v$new_version"
git push --set-upstream origin release/${new_version} --tags

exit 
