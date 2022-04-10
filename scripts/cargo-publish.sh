#!/bin/bash

cargo publish -p cronos-cron
sleep 25
cargo publish -p cronos-heartbeat
cargo publish -p cronos-pool
cargo publish -p cronos-scheduler
sleep 25
cargo publish -p cronos-sdk
sleep 25
cargo publish -p cronos-cli
cargo publish -p cronos-plugin
cargo publish -p cronos-telemetry
cargo publish -p cronos-tests
