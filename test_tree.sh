#!/bin/bash

echo "Testing Rolo tree rendering..."
echo "Building..."
cd /home/xnull/repos/code/rust/oodx/rolo
cargo build --quiet

echo "Running tree demo (press Enter when you see the colored output):"
echo ""

# Run the demo with proper terminal settings
TERM=xterm-256color ./target/debug/rolo