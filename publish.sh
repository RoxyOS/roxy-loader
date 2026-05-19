#!/usr/bin/env bash

set -e

cargo publish -p roxy-loader-api
echo "waiting 10 seconds..."
sleep 10
cargo publish -p roxy-loader
echo "waiting 10 seconds..."
sleep 10
cargo publish -p roxy-loader-utils
