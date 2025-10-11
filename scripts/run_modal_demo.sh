#!/bin/bash
set -e

echo ">>> Building modaldemo application..."
trunk build modaldemo/index.html

echo ">>> Starting server..."
python -m http.server 8080 --directory modaldemo/dist
