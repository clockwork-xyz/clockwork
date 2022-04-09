#!/bin/bash

cargo publish -p cronos-cron
sleep 10
cargo publish -p cronos-program
sleep 10
cargo publish -p cronos-sdk
sleep 10
cargo publish -p cronos-bot
cargo publish -p cronos-cli
cargo publish -p cronos-telemetry
cargo publish -p cronos-tests