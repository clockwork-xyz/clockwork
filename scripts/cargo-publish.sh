#!/bin/bash

cargo publish -p cronos-cron
sleep 25
cargo publish -p cronos-health
cargo publish -p cronos-network
cargo publish -p cronos-pool
cargo publish -p cronos-scheduler
sleep 25
cargo publish -p cronos-sdk
sleep 25
cargo publish -p cronos-cli
cargo publish -p cronos-plugin
cargo publish -p cronos-stress
cargo publish -p cronos-telemetry
