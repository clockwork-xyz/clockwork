#!/bin/bash

# Delete target folder
cargo clean

# Build with Anchor
anchor build 

# Get pubkey addresses
program_id_network=$(solana address -k target/deploy/clockwork_network_program-keypair.json)
program_id_thread=$(solana address -k target/deploy/clockwork_thread_program-keypair.json)
program_id_webhook=$(solana address -k target/deploy/clockwork_webhook_program-keypair.json)

# Update declared program IDs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_network}'");/g' programs/network/src/lib.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_thread}'");/g' programs/thread/src/lib.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_webhook}'");/g' programs/webhook/src/lib.rs

# Update Anchor config
sed -i '' -e 's/^clockwork_network_program = ".*"/clockwork_network_program = "'${program_id_network}'"/g' Anchor.toml
sed -i '' -e 's/^clockwork_thread_program = ".*"/clockwork_thread_program = "'${program_id_thread}'"/g' Anchor.toml
sed -i '' -e 's/^clockwork_webhook_program = ".*"/clockwork_webhook_program = "'${program_id_webhook}'"/g' Anchor.toml

# Rebuild
anchor build
