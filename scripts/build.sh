#!/bin/sh

yarn_build () {
    echo "\n\n\n🧱 Building $1\n"
    cd $1
    yarn
    yarn build
    cd $2
}

build_clients () {
    yarn_build programs/indexer/client ../../..
    yarn_build programs/cronos/client ../../..
}

build_utils () {
    yarn_build utils ..
}

build_utils
build_clients

exit
