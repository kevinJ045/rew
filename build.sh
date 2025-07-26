#!/bin/bash

# Build script for Rew workspace

set -e

echo "Building Rew workspace..."

# Build each crate in dependency order
echo "Building rew-core..."
cd rew-core
cargo build
cd ..

echo "Building rew-compiler..."
cd rew-compiler  
cargo build
cd ..

echo "Building rew-extensions..."
cd rew-extensions
cargo build
cd ..

echo "Building rew-runtime..."
cd rew-runtime
cargo build
cd ..

echo "Building rew-cli..."
cd rew-cli
cargo build
cd ..

echo "Building workspace..."
cargo build

echo "Build complete!"
