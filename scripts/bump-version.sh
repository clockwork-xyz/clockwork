#!/bin/bash

set -e

# Cross-platform sed
replace_in_file() {
  if [ "$(uname)" == "Darwin" ]; then
    sed -i '' -e "$1" "$2"
  else
    sed -i'' -e "$1" "$2"
  fi
}

# Get new version
current_version=$(cat ./VERSION)
echo "Current version: $current_version"
read -r -p "    New version: " new_version

# Build
#RUSTFLAGS="--deny warnings" cargo build || (echo "Build failed" && exit)
cargo build || (echo "Build failed" && exit)

# Bump the shared version in the root Cargo.toml
(
  set -x
  replace_in_file "1,/^version =/ s/\(version = \"\)[^\"]*\"/\1$new_version\"/" Cargo.toml
)

# Bump the version for all the deps
crates=(
  clockwork-client
  clockwork-cron
  clockwork-network-program
  clockwork-relayer-api
  clockwork-sdk
  clockwork-utils
  clockwork-thread-program
  clockwork-thread-program-v2
  clockwork-webhook-program
)

for crate in "${crates[@]}"; do
  (
    set -x
    replace_in_file "
      s/^$crate = { *path *= *\"\([^\"]*\)\" *, *version *= *\"[^\"]*\"\(.*\)} *\$/$crate = \{ path = \"\1\", version = \"=$new_version\"\2\}/
    " Cargo.toml
    replace_in_file "
      s/^$crate = { *package *= *\"\([^\"]*\)\" *, *path *= *\"\([^\"]*\)\" *, *version *= *\"[^\"]*\"\(.*\)} *\$/$crate = \{ package = \"\1\", path = \"\2\", version = \"=$new_version\"\3\}/
    " Cargo.toml
  )
done

# Force thread program v1 to stay on 1.4.2
thread_v1="1.4.2"
(
  set -x
  replace_in_file "
    s/^clockwork-thread-program-v1 = { *package *= *\"\([^\"]*\)\" *, *path *= *\"\([^\"]*\)\" *, *version *= *\"[^\"]*\"\(.*\)} *\$/clockwork-thread-program-v1 = \{ package = \"\1\", path = \"\2\", version = \"=$thread_v1\"}/
  " Cargo.toml
)


# Update version
echo $new_version > VERSION

# Rebuild
cargo build

# Git commit 
echo "$(git diff --stat | tail -n1)"
git add .
git commit -m "Bump from $current_version to $new_version"
git tag "v$new_version"
git push && git push --tags
