#!/bin/bash

# Delete target folder
cargo clean

# Build with Anchor
anchor build 

# Get pubkey addresses
program_id_http=$(solana address -k target/deploy/clockwork_webhook-keypair.json)
program_id_network=$(solana address -k target/deploy/clockwork_network-keypair.json)
program_id_pool=$(solana address -k target/deploy/clockwork_pool-keypair.json)
program_id_crank=$(solana address -k target/deploy/clockwork_queue-keypair.json)

# Update declared program IDs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_http}'");/g' programs/http/src/id.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_network}'");/g' programs/network/src/id.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_pool}'");/g' programs/pool/src/id.rs
sed -i '' -e 's/^declare_id!(".*");/declare_id!("'${program_id_queue}'");/g' programs/queue/src/lib.rs

# Update Anchor config
sed -i '' -e 's/^http = ".*"/http = "'${program_id_http}'"/g' Anchor.toml
sed -i '' -e 's/^network = ".*"/network = "'${program_id_network}'"/g' Anchor.toml
sed -i '' -e 's/^pool = ".*"/pool = "'${program_id_pool}'"/g' Anchor.toml
sed -i '' -e 's/^queue = ".*"/queue = "'${program_id_queue}'"/g' Anchor.toml

# Rebuild
anchor build
