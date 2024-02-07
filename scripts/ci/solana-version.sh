#!/usr/bin/env bash

# Prints the Solana version.

set -e

cd "$(dirname "$0")/../../plugin"

cargo tree -p solana-geyser-plugin-interface | grep solana-geyser-plugin-interface | awk '{print $2}'
