#!/bin/bash

# Configure solana cli for localnet
solana config set --url localhost

# Start a Solana validator with the Cronos programs and plugin
solana-test-validator -r \
    --bpf-program target/deploy/cronos_heartbeat-keypair.json target/deploy/cronos_heartbeat.so \
    --bpf-program target/deploy/cronos_network-keypair.json target/deploy/cronos_network.so \
    --bpf-program target/deploy/cronos_scheduler-keypair.json target/deploy/cronos_scheduler.so \
    --geyser-plugin-config plugin/config.json
