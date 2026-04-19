#!/usr/bin/env bash

# Fix permissions script for Zaroxi Desktop
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

echo "Fixing script permissions..."

# Check if we found the right directories
if [ ! -f "$DESKTOP_DIR/package.json" ]; then
    echo "Error: Could not find apps/desktop/package.json"
    echo "Make sure you're in the zaroxi repository"
    exit 1
fi

cd "$DESKTOP_DIR"

# Make all scripts executable
chmod +x run.sh start.sh setup.sh build.sh fix-permissions.sh

# Make check-setup.js executable (if needed)
if [ -f "check-setup.js" ]; then
    chmod +x check-setup.js
fi

echo "Permissions fixed!"
echo ""
echo "Now you can run from anywhere in zaroxi repository:"
echo "  ./run.sh      # Start development"
echo "  ./start.sh    # Alternative start"
echo "  ./setup.sh    # Install dependencies"
echo "  ./build.sh    # Build for production"
