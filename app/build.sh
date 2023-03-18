#!/bin/sh
set -e

# Install Rust
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
export PATH="$HOME/.cargo/bin:$PATH"

# Install trunk
cargo install trunk

# Update wasm target
rustup target add wasm32-unknown-unknown

# Build the project
trunk build --release

# Move the built files to /www folder
mkdir www
mv dist www/

