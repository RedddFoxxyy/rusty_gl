#!/bin/bash

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
colored_echo() {
    local color="$1"
    local message="$2"
    echo -e "${color}${message}${NC}"
}

# Store the project root directory (parent of the scripts directory)
project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Function to find virtual environment
find_venv() {
    # Look for typical virtual environment indicators
    local venv_path=$(find "$project_root" -maxdepth 3 \( \
        -path "*/bin/activate" -o \
        -path "*/Scripts/activate" \) -print -quit)

    # If found, extract the directory path
    if [ -n "$venv_path" ]; then
        dirname "$(dirname "$venv_path")"
        return 0
    fi
    return 1
}

# Try to find and activate virtual environment
colored_echo "$BLUE" "🔍 Searching for virtual environment..."
venv_dir=$(find_venv)

if [ -n "$venv_dir" ]; then
    colored_echo "$GREEN" "✅ Found virtual environment: $venv_dir"

    # Determine activation script
    if [ -f "$venv_dir/bin/activate" ]; then
        # Unix-like systems
        colored_echo "$YELLOW" "🚀 Activating virtual environment (Unix-style)..."
        source "$venv_dir/bin/activate"
    elif [ -f "$venv_dir/Scripts/activate" ]; then
        # Windows-style virtual environments
        colored_echo "$YELLOW" "🚀 Activating virtual environment (Windows-style)..."
        source "$venv_dir/Scripts/activate"
    else
        colored_echo "$RED" "❌ Could not activate virtual environment"
        exit 1
    fi

    # Change to project root directory
    cd "$project_root"

    # Build the terra_graphics_engine python module
    colored_echo "$BLUE" "🧰 Building terra_graphics_engine..."
    cd engine-src
    maturin develop --release || {
        colored_echo "$RED" "❌ Failed to build terra_graphics_engine"
        exit 1
    }
    cd ..

else
    colored_echo "$RED" "❌ No virtual environment found."
    exit 1
fi