#!/bin/bash

# Get the current keypair
current_keypair=$(solana config get | grep "Keypair Path:" | cut -c 15-)

# Switch to local validator keypair
solana config set -k $(pwd)/test-ledger/validator-keypair.json

# Stake node with the Cronos network
cd cli
cargo run -- node register
sleep 2
cargo run -- node stake 100000000000 # 100 tokens

# Switch back to the user's keypair
solana config set -k $current_keypair