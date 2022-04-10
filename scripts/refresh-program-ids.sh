#!/bin/bash

# Delete target folder
rm -rf target

# Build with Anchor
anchor build 

# Get pubkey addresses
program_id_heartbeat=$(solana address -k target/deploy/cronos_heartbeat-keypair.json)
program_id_pool=$(solana address -k target/deploy/cronos_pool-keypair.json)
program_id_scheduler=$(solana address -k target/deploy/cronos_scheduler-keypair.json)

# Update declared program IDs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_heartbeat}'");/g' programs/heartbeat/src/lib.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_pool}'");/g' programs/pool/src/lib.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_scheduler}'");/g' programs/scheduler/src/lib.rs

# Update Anchor config
sed -i '' -e 's/^heartbeat = ".*"/heartbeat = "'${program_id_heartbeat}'"/g' Anchor.toml
sed -i '' -e 's/^pool = ".*"/pool = "'${program_id_pool}'"/g' Anchor.toml
sed -i '' -e 's/^scheduler = ".*"/scheduler = "'${program_id_scheduler}'"/g' Anchor.toml

# Rebuild
anchor build
