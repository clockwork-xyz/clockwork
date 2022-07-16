#!/bin/bash

# Delete target folder
cargo clean

# Build with Anchor
anchor build 

# Get pubkey addresses
program_id_health=$(solana address -k target/deploy/cronos_health-keypair.json)
program_id_http=$(solana address -k target/deploy/cronos_http-keypair.json)
program_id_network=$(solana address -k target/deploy/cronos_network-keypair.json)
program_id_pool=$(solana address -k target/deploy/cronos_pool-keypair.json)
program_id_scheduler=$(solana address -k target/deploy/cronos_scheduler-keypair.json)

# Update declared program IDs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_health}'");/g' programs/health/src/id.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_http}'");/g' programs/http/src/id.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_network}'");/g' programs/network/src/id.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_pool}'");/g' programs/pool/src/id.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_scheduler}'");/g' programs/scheduler/src/id.rs

# Update Anchor config
sed -i '' -e 's/^health = ".*"/health = "'$program_id_health'"/g' Anchor.toml
sed -i '' -e 's/^http = ".*"/http = "'$program_id_http'"/g' Anchor.toml
sed -i '' -e 's/^network = ".*"/network = "'${program_id_network}'"/g' Anchor.toml
sed -i '' -e 's/^pool = ".*"/pool = "'$program_id_pool'"/g' Anchor.toml
sed -i '' -e 's/^scheduler = ".*"/scheduler = "'${program_id_scheduler}'"/g' Anchor.toml

# Rebuild
anchor build
