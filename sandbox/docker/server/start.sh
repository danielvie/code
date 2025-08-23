#!/bin/sh
# start.sh

# Ensure uv is in the PATH (this is optional if you've already configured your Dockerfile correctly,
# but it's a good practice for resilience)
export PATH="/usr/local/bin:$PATH"

# Run the server with uv
echo "starting gRPc server..."
exec uv run server.py