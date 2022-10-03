#!/bin/bash

# Publish cron
cargo publish -p clockwork-cron
sleep 25

# Publish programs
cargo publish -p clockwork-pool
sleep 25
cargo publish -p clockwork-queue
sleep 25
cargo publish -p clockwork-network
sleep 25
cargo publish -p clockwork-webhook
sleep 25

# Publish downstream bins and libs
cargo publish -p clockwork-client
sleep 25
cargo publish -p clockwork-cli
sleep 25 
cargo publish -p clockwork-bench
sleep 25
cargo publish -p clockwork-sdk
