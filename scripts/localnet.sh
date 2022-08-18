#!/bin/bash

# Build
anchor build
cargo build

# Configure solana cli for localnet
solana config set --url localhost

# Start a Solana validator with the Clockwork programs and plugin
clear
solana-test-validator -r \
    --bpf-program target/deploy/clockwork_crank-keypair.json target/deploy/clockwork_crank.so \
    --bpf-program target/deploy/clockwork_http-keypair.json target/deploy/clockwork_http.so \
    --bpf-program target/deploy/clockwork_network-keypair.json target/deploy/clockwork_network.so \
    --bpf-program target/deploy/clockwork_pool-keypair.json target/deploy/clockwork_pool.so \
    --geyser-plugin-config plugin/config.json
