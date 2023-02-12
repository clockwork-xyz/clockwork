#!/usr/bin/env bash

set -e

# Rebuid programs
rm -rf lib/clockwork_thread_program.so
cd programs/thread && anchor build; cd -;
cp -fv target/deploy/clockwork_thread_program.so lib/

# Rebuild plugin
rm -rf lib/libclockwork_plugin.dylib
cargo build --manifest-path plugin/Cargo.toml
cp -fv target/debug/libclockwork_plugin.dylib lib/

# bpf-program
crate_name="hello_clockwork"
cd ~/examples/$crate_name
anchor build
cd -

# Clean ledger
rm -rf test-ledger

RUST_LOG=clockwork_plugin clockwork localnet \
    --bpf-program ~/examples/$crate_name/target/deploy/$crate_name-keypair.json \
    --bpf-program ~/examples/$crate_name/target/deploy/$crate_name.so

