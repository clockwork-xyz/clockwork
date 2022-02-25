#!/bin/bash

declare -a arr=(program sdk bot cli)
declare -a paths=(../programs/cronos/Cargo.toml ../sdk/Cargo.toml ../bot/Cargo.toml ../cli/Cargo.toml)
declare -a versions=()
declare -a flag=()

for(( c = 0 ; c < 4 ; c++))
do
  echo cronos ${arr[$c]} version bump:
  read bump_version
  versions+=(${bump_version})

  if [ ${arr[$c]} == program ];
  then
    # bump cronos program version
    sed -i '' -e 's/version =.*/version = "'${bump_version}'"/g' ${paths[$c]}
    
    # cargo publish cronos program
    cargo publish ${paths[$c]} && flag+=( true ) ||flag+=( false )
    
  elif [ ${arr[$c]} == sdk ];
  then
    # bump cronos bot version
    sed -i '' -e 's/^version =.*/version = "'${bump_version}'"/g' ${paths[$c]}

    # cargo publish cronos sdk
    cargo publish ${paths[$c]} && flag+=( true ) || flag+=( false )

    # if cronos-program crate got published then update dependency
    if [ ${flag[0]} == true ];
    then
      sed -i '' -e 's/^cronos-program =.*/cronos-program = { path = "..\/programs\/cronos", features = ["no-entrypoint"], version = "'${versions[0]}'" }/g' ${paths[$c]}
    fi

  # update cronos bot and  sdk dependency
  elif [ ${arr[$c]} == bot ];
  then
    # bump cronos bot version
    sed -i '' -e 's/^version =.*/version = "'${bump_version}'"/g' ${paths[$c]}

    # cargo publish cronos bot
    cargo publish ${paths[$c]} && flag+=( true ) || flag+=( false )

    # if sdk crate got published then update dependency
    if [ ${flag[1]} == true ];
    then
      sed -i '' -e 's/^cronos-sdk =.*/cronos-sdk = { path = "..\/sdk", version = "'${versions[1]}'" }/g' ${paths[$c]}
    fi

  if [ ${arr[$c]} == cli ];
  then
    # bump cronos cli version
    sed -i '' -e 's/^version =.*/version = "'${bump_version}'"/g' ${paths[$c]}

    # cargo publish cronos cli
    cargo publish ${paths[$c]} && flag+=( true ) || flag+=( false )

    # if sdk crate got published then update dependency
    if [ ${flag[1]} == true ];
    then
      sed -i '' -e 's/^cronos-sdk =.*/cronos-sdk = { path = "..\/sdk", version = "'${versions[1]}'" }/g' ${paths[$c]}
    fi   

  fi
done


echo ${versions[*]}