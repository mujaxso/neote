#!/bin/sh

# Simple run script for Zaroxi Desktop App
set -e

echo "Starting Zaroxi Desktop App..."

# Go to desktop directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

# Check if we're in the right place
if [ ! -f "package.json" ]; then
    echo "Error: package.json not found. Make sure you're in apps/desktop directory"
    echo "Current directory: $(pwd)"
    exit 1
fi

# Kill processes using ports 1420 and 1421
echo "Clearing ports 1420 and 1421..."

# Try different methods to kill processes on these ports
# Method 1: Use ss and kill (common on modern Linux)
if command -v ss >/dev/null 2>&1; then
    for port in 1420 1421; do
        PID=$(ss -lptn "sport = :$port" 2>/dev/null | grep -o "pid=[0-9]*" | cut -d= -f2 | head -1)
        if [ -n "$PID" ]; then
            echo "Killing process $PID on port $port"
            kill -9 "$PID" 2>/dev/null || true
        fi
    done
# Method 2: Use fuser if available
elif command -v fuser >/dev/null 2>&1; then
    fuser -k 1420/tcp 2>/dev/null || true
    fuser -k 1421/tcp 2>/dev/null || true
# Method 3: Use lsof if available  
elif command -v lsof >/dev/null 2>&1; then
    lsof -ti:1420 2>/dev/null | xargs kill -9 2>/dev/null || true
    lsof -ti:1421 2>/dev/null | xargs kill -9 2>/dev/null || true
# Method 4: Try to kill vite and tauri processes directly
else
    # Kill vite processes
    pkill -f "node.*vite" 2>/dev/null || true
    # Kill tauri dev processes
    pkill -f "tauri dev" 2>/dev/null || true
fi

# Wait a moment for ports to be released
sleep 1

# Check if node_modules exists, install if not
if [ ! -d "node_modules" ]; then
    echo "Installing dependencies..."
    npm install
fi

# Run the app
echo "Starting development server..."
exec npm run tauri dev
