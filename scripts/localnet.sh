#!/bin/bash

solana-test-validator -r \
    --bpf-program $(pwd)/target/deploy/cronos_heartbeat-keypair.json $(pwd)/target/deploy/cronos_heartbeat.so \
    --bpf-program $(pwd)/target/deploy/cronos_pool-keypair.json $(pwd)/target/deploy/cronos_pool.so \
    --bpf-program $(pwd)/target/deploy/cronos_scheduler-keypair.json $(pwd)/target/deploy/cronos_scheduler.so \
    --geyser-plugin-config $(pwd)/plugin/config.json
