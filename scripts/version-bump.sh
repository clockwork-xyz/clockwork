#!/bin/bash

declare -a arr=("programs" "sdk" "cli" "bot")
declare -a paths=(../programs/cronos/Cargo.toml ../cli/Cargo.toml ../bot/Cargo.toml ../sdk/Cargo.toml)

for(( c = 0 ; c < 4 ; c++))
do
  echo ${arr[$c]} version bump:
  read bump_version
  sed -i '' -e '3s/.*/version = "'${bump_version}'"/g' ${paths[$c]}
done