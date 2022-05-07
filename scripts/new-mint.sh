#!/bin/bash

# Create mint
log=$(spl-token create-token | grep 'Creating token')
mint=${log: -44}

# Create token account
log=$(spl-token create-account $mint | grep 'Creating account')
account=${log: -44}

# Mint 1000 tokens
balance=1000
spl-token mint $mint $balance
