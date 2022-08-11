#!/bin/bash

# Stake local node with the Clockwork network
cd cli
cargo run -- node register ../test-ledger/validator-keypair.json
sleep 2
cargo run -- node stake 100000000000 $(solana address -k ../test-ledger/validator-keypair.json) # 100 tokens
