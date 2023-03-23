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

# Update wasm target
rustup target add wasm32-unknown-unknown

# Build the project
trunk build --release

# Move favicon into /dist folder.
cp favicon.ico dist/favicon.ico

# Create the _redirects file in the dist folder
echo "/* /index.html 200" > dist/_redirects

ls dist

