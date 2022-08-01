#!/bin/bash

# Get the current keypair
current_keypair=$(solana config get | grep "Keypair Path:" | cut -c 15-)

# Stake local node with the Clockwork network
cd cli
cargo run -- node register ../test-ledger/validator-keypair.json
sleep 2
cargo run -- node stake 100000000000 $(solana address -k ../test-ledger/validator-keypair.json) # 100 tokens

# Switch back to the user's keypair
solana config set -k $current_keypair
