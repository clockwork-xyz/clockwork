#!/bin/bash

# Publish cron
cargo publish -p clockwork-cron
sleep 25

# Publish programs
cargo publish -p clockwork-pool-program
sleep 25
cargo publish -p clockwork-queue-program
sleep 25
cargo publish -p clockwork-network-program
sleep 25
cargo publish -p clockwork-webhook-program
sleep 25

# Publish downstream bins and libs
cargo publish -p clockwork-client
sleep 25
cargo publish -p clockwork-cli
sleep 25 
cargo publish -p clockwork-bench
sleep 25
cargo publish -p clockwork-sdk
