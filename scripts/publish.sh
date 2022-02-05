#!/bin/sh

echo "\n\n\n⬆️  Publishing @cronos-so/$1\n"
cd programs/$1/client
yarn
yarn build
yarn publish
cd ../../../


git add .
git commit -m 'Publishing client'

exit
