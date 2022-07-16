#!/bin/bash

cargo publish -p cronos-cron
sleep 25
cargo publish -p cronos-health
sleep 25
cargo publish -p cronos-http
sleep 25
cargo publish -p cronos-pool
sleep 25
cargo publish -p cronos-scheduler
sleep 25
cargo publish -p cronos-network
