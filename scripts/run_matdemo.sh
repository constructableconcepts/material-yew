#!/bin/bash
set -e

# Ensure cleanup happens on exit
trap "echo '>>> Shutting down server...'; kill \$SERVER_PID 2>/dev/null" EXIT

echo ">>> Cleaning previous build artifacts..."
rm -rf matdemo/dist

echo ">>> Installing required Rust target..."
rustup target add wasm32-unknown-unknown

echo ">>> Building matdemo application..."
# Build the application first and log the output. If this fails, the script will exit.
(cd matdemo && trunk build)

echo ">>> Build successful. Starting server..."
# Serve the created `dist` directory in the background.
python3 -m http.server --directory matdemo/dist 8080 &
SERVER_PID=$!
echo ">>> Server started with PID: $SERVER_PID"

# Give the server a moment to start up.
sleep 3

# Run the verification script
echo ">>> Running verification script..."
python3 scripts/verify_matdemo.py

# The trap will handle killing the server
echo ">>> Test script finished."