#!/bin/bash

set -e  # Exit immediately if any command exits with a non-zero status

echo "========================================"
echo "Running tests with pastey crate..."
echo "========================================"
cargo test

echo "========================================"
echo "Running tests with pastey-test-suite crate..."
echo "========================================"
cd pastey-test-suite

# Set some packages to their precise version to avoid MSRV conflict
if [[ "$1" == "1.56.0" ]]; then
    cargo update -p quote --precise 1.0.40
    cargo update -p glob --precise 0.3.2
    cargo update -p serde_derive --precise 1.0.210
    cargo update -p proc-macro2 --precise 1.0.101
    cargo update -p ryu --precise 1.0.18
    cargo update -p itoa --precise 1.0.6
    cargo update -p unicode-ident --precise 1.0.13
    cargo update -p dissimilar --precise 1.0.10
fi

cargo test

cd ../
cd paste-compat
./test.sh
cd ../
