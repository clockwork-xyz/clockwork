#!/bin/bash

# Create mint 
log=$(./scripts/mint.sh | grep Token:)
mint=${log: -44}

# Initialize the Clockwork programs
cd cli && cargo run -- initialize -m $mint
