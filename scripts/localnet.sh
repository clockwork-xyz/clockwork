#!/bin/bash

# Build
anchor build
cargo build

# Configure solana cli for localnet
solana config set --url localhost

# Start a Solana validator with the Cronos programs and plugin
clear
solana-test-validator -r \
    --bpf-program target/deploy/cronos_health-keypair.json target/deploy/cronos_health.so \
    --bpf-program target/deploy/cronos_http-keypair.json target/deploy/cronos_http.so \
    --bpf-program target/deploy/cronos_network-keypair.json target/deploy/cronos_network.so \
    --bpf-program target/deploy/cronos_pool-keypair.json target/deploy/cronos_pool.so \
    --bpf-program target/deploy/cronos_scheduler-keypair.json target/deploy/cronos_scheduler.so \
    --geyser-plugin-config plugin/config.json
