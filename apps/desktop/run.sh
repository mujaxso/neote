#!/usr/bin/env bash

# Run script for Zaroxi Desktop App
# Can be run from anywhere within the zaroxi repository

set -e

# Find the zaroxi root directory
find_zaroxi_root() {
    local dir="$PWD"
    while [ "$dir" != "/" ]; do
        if [ -f "$dir/Cargo.toml" ] && [ -d "$dir/apps/desktop" ]; then
            echo "$dir"
            return 0
        fi
        dir="$(dirname "$dir")"
    done
    return 1
}

ZAROXI_ROOT="$(find_zaroxi_root 2>/dev/null || echo "$PWD")"
DESKTOP_DIR="$ZAROXI_ROOT/apps/desktop"

echo "Starting Zaroxi Desktop App..."

# Check if we found the right directories
if [ ! -f "$DESKTOP_DIR/package.json" ]; then
    echo "Error: Could not find apps/desktop/package.json"
    echo "Make sure you're in the zaroxi repository"
    exit 1
fi

cd "$DESKTOP_DIR"

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
    echo "node_modules not found. Running npm install..."
    npm install
    if [ $? -ne 0 ]; then
        echo "npm install failed"
        exit 1
    fi
fi

# Check if Rust dependencies are built
if [ ! -d "$ZAROXI_ROOT/target" ]; then
    echo "Rust dependencies not built. Building..."
    cd "$ZAROXI_ROOT"
    cargo build --workspace
    if [ $? -ne 0 ]; then
        echo "cargo build failed"
        exit 1
    fi
    cd "$DESKTOP_DIR"
fi

echo "Starting development server..."
npm run tauri dev
