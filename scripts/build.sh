#!/bin/sh

build_package () {
    echo "\n\n\n🧱 Building $1\n"
    cd $1
    yarn
    yarn build
    cd $2
}

build programs/indexer/client ../../..
build programs/cronos/client ../../..

exit
