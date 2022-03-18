#!/bin/bash
cargo publish -p cronos-cron
sleep 5
cargo publish -p cronos-program
sleep 5 
cargo publish -p cronos-sdk
sleep 5 
cargo publish -p cronos-bot
cargo publish -p cronos-cli
cargo publish -p cronos-telemetry
