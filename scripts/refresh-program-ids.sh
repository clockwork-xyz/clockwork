#!/bin/bash

# Delete target folder
cargo clean

# Build with Anchor
anchor build 

# Get pubkey addresses
program_id_network=$(solana address -k target/deploy/clockwork_network_program-keypair.json)
program_id_queue=$(solana address -k target/deploy/clockwork_queue_program-keypair.json)
program_id_webhook=$(solana address -k target/deploy/clockwork_webhook_program-keypair.json)

# Update declared program IDs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_network}'");/g' programs/network/src/lib.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_queue}'");/g' programs/queue/src/lib.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_webhook}'");/g' programs/webhook/src/lib.rs

# Update Anchor config
sed -i '' -e 's/^network = ".*"/network = "'${program_id_network}'"/g' Anchor.toml
sed -i '' -e 's/^queue = ".*"/queue = "'${program_id_queue}'"/g' Anchor.toml
sed -i '' -e 's/^webhook = ".*"/webhook = "'${program_id_webhook}'"/g' Anchor.toml

# Rebuild
anchor build
