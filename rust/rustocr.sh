#!/bin/bash
# Enhanced wrapper script for rustocr that sets up the Python environment and library paths

# Get Python library paths
PYTHON_LIB_DIR=$(python3 -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))")
PYTHON_PREFIX=$(python3 -c "import sys; print(sys.prefix)")

# Set library paths for macOS
export DYLD_LIBRARY_PATH="${PYTHON_LIB_DIR}:${PYTHON_PREFIX}/lib:${DYLD_LIBRARY_PATH}"

# Add additional library paths that might be needed for OpenCV
if [ -d "/opt/homebrew/lib" ]; then
    export DYLD_LIBRARY_PATH="/opt/homebrew/lib:${DYLD_LIBRARY_PATH}"
fi

if [ -d "/usr/local/lib" ]; then
    export DYLD_LIBRARY_PATH="/usr/local/lib:${DYLD_LIBRARY_PATH}"
fi

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Run the actual binary
exec "${SCRIPT_DIR}/target/release/rustocr" "$@"
