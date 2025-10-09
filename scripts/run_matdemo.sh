#!/bin/bash
set -e

echo "Building matdemo application..."
(cd matdemo && trunk build)

echo "Starting server on port 8080..."
python3 -m http.server --directory matdemo/dist 8080