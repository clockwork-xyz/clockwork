#!/bin/bash

set -e

if [[ $# -eq 0 ]]; then
  echo "Usage: $0 <new_version> [--dry-run] [<cargo-set-version arguments>]"
  exit 1
fi

new_version=$1
shift

while [[ $# -gt 0 ]]; do
  case $1 in
  --dry-run)
    dry_run="--dry-run"
    ;;
  *)
    args+=("$1")
    ;;
  esac
  shift
done

# Print current version
current_version=$(cat ./VERSION)
echo "Current version: $current_version"

# Run cargo set-version
cargo set-version --locked --workspace --bump $new_version $dry_run "${args[@]}"
if [ -n "$dry_run" ]; then
 echo "Dry run, exiting..."
   exit 0
fi

# We need to retrieve the actual semver version from the Cargo.toml files
actual_version=$(cargo metadata --format-version=1 | jq -r '.packages[] | select(.name == "mat-clockwork-sdk") | .version')
echo $actual_version >VERSION
echo "New version: $actual_version"

# Build
cargo build

# Git commit
echo "$(git diff --stat | tail -n1)"
git add .
git commit -m "Bump from $current_version to $actual_version"
git tag "v$actual_version"
git push && git push --tags
