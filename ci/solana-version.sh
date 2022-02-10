#!/usr/bin/env bash

# Prints the Solana version.

set -e

cd "$(dirname "$0")/.."

cargo read-manifest | jq -r '.dependencies[] | select(.name == "solana-accountsdb-plugin-interface") | .req'
