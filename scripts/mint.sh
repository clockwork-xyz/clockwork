#!/bin/bash

# Create mint
log=$(spl-token create-token | grep 'Creating token')
mint=${log: -44}

# Create token account
log=$(spl-token create-account $mint | grep 'Creating account')
account=${log: -44}

# Mint 10 tokens to the current keypair
balance=10
spl-token mint $mint $balance

# Transfer 100 to the validator keypair
# validator_identity=$(solana address -k ./test-ledger/validator-keypair.json)
# spl-token transfer $mint 100 $validator_identity --fund-recipient
