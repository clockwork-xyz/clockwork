#!/bin/bash

cargo publish -p clockwork-cron
sleep 25
cargo publish -p clockwork-pool
sleep 25
cargo publish -p clockwork-http
sleep 25
cargo publish -p clockwork-crank
sleep 25
cargo publish -p clockwork-network
sleep 25 
cargo publish -p clockwork-client
sleep 25
cargo publish -p clockwork-cli
