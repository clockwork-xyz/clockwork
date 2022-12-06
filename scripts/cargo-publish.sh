#!/bin/bash

# Publish shared libs
cargo publish -p clockwork-cron
sleep 25
cargo publish -p clockwork-utils
sleep 25

# Publish programs
cargo publish -p clockwork-network-program
sleep 25
cargo publish -p clockwork-thread-program
sleep 25
cargo publish -p clockwork-webhook-program
sleep 25

# Publish SDK
cargo publish -p clockwork-sdk
sleep 25

# Publish downstream bins and libs
# These are most likely to fail due to Anchor dependency issues.
cargo publish -p clockwork-client
sleep 25
cargo publish -p clockwork-cli
