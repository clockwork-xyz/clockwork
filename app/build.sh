#!/bin/sh
set -e

# Set the Cargo and Rustup home directories
export HOME="/root"
export CARGO_HOME="$HOME/.cargo"
export RUSTUP_HOME="$HOME/.rustup"

# Install Rust
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
export PATH="$CARGO_HOME/bin:$PATH"

# Install trunk
cargo install trunk
set +e
cargo update -p time:0.3.20 --precise 0.3.16
set -e

# Update wasm target
rustup target add wasm32-unknown-unknown

# Build the project
trunk build --release

# Move favicon into /dist folder.
cp favicon.ico dist/favicon.ico
