#!/bin/bash

# Delete target folder
cargo clean

# Build with Anchor
anchor build 

# Get pubkey addresses
program_id_healthcheck=$(solana address -k target/deploy/cronos_healthcheck-keypair.json)
program_id_network=$(solana address -k target/deploy/cronos_network-keypair.json)
program_id_pool=$(solana address -k target/deploy/cronos_pool-keypair.json)
program_id_scheduler=$(solana address -k target/deploy/cronos_scheduler-keypair.json)

# Update declared program IDs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_healthcheck}'");/g' programs/healthcheck/src/id.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_network}'");/g' programs/network/src/id.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_pool}'");/g' programs/pool/src/id.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_scheduler}'");/g' programs/scheduler/src/id.rs

# Update Anchor config
sed -i '' -e 's/^healthcheck = ".*"/healthcheck = "'$program_id_healthcheck'"/g' Anchor.toml
sed -i '' -e 's/^network = ".*"/network = "'${program_id_network}'"/g' Anchor.toml
sed -i '' -e 's/^pool = ".*"/pool = "'$program_id_pool'"/g' Anchor.toml
sed -i '' -e 's/^scheduler = ".*"/scheduler = "'${program_id_scheduler}'"/g' Anchor.toml

# Rebuild
anchor build
