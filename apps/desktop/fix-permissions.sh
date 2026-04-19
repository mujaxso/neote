#!/bin/bash

# Fix permissions script for Zaroxi Desktop
# If this script is not executable, run: chmod +x fix-permissions.sh

echo "Fixing script permissions..."

# First, make this script executable if it's not
if [ ! -x "$0" ]; then
    echo "Making this script executable..."
    chmod +x "$0"
fi

# Make all scripts in the current directory executable
for script in run.sh start.sh setup.sh build.sh fix-permissions.sh; do
    if [ -f "$script" ]; then
        chmod +x "$script"
        echo "✓ Made $script executable"
    fi
done

# Make check-setup.js executable if it exists
if [ -f "check-setup.js" ]; then
    chmod +x check-setup.js
    echo "✓ Made check-setup.js executable"
fi

echo ""
echo "✅ Permissions fixed!"
echo ""
echo "Now you can run:"
echo "  ./run.sh      # Start development"
echo "  ./start.sh    # Alternative start"
echo "  ./setup.sh    # Install dependencies"
echo "  ./build.sh    # Build for production"
echo ""
echo "If you still get 'permission denied', try:"
echo "  chmod +x *.sh"
echo "  chmod +x check-setup.js"
