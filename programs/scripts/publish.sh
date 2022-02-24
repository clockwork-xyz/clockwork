#!/bin/sh

echo "\n\n\n⬆️  Publishing @cronos-so/cronos\n"
cd client
yarn
yarn build
yarn publish
cd ../../../

git add .
git commit -m 'Publishing @cronos-co/client'

exit
