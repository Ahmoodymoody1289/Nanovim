#!/bin/bash

set -e  # Exit on any error

echo "Installing NanoVim..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: cargo (Rust) is not installed."
    echo "   Install Rust from: https://rustup.rs/"
    exit 1
fi

# Clean previous builds
echo "🔨 Building release version..."
cargo build --release

# Detect operating system
if [[ "$OSTYPE" == "linux-gnu"* ]] || [[ "$OSTYPE" == "darwin"* ]]; then
    # Linux or macOS
    if [[ "$OSTYPE" == "darwin"* ]]; then
        INSTALL_DIR="$HOME/bin"
        echo "Detected: macOS"
    else
        INSTALL_DIR="$HOME/.local/bin"
        echo "Detected: Linux"
    fi
    
    mkdir -p "$INSTALL_DIR"
    cp target/release/nanovim "$INSTALL_DIR/nvim"
    
    chmod +x "$INSTALL_DIR/nvim"
    
    # Check if directory is in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        SHELL_RC=""
        if [ -f "$HOME/.bashrc" ]; then
            SHELL_RC="$HOME/.bashrc"
        elif [ -f "$HOME/.zshrc" ]; then
            SHELL_RC="$HOME/.zshrc"
        fi
        
        if [ -n "$SHELL_RC" ]; then
            echo "" >> "$SHELL_RC"
            echo "# NanoVim path" >> "$SHELL_RC"
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_RC"
            
            echo "✅ Added $INSTALL_DIR to PATH in $(basename $SHELL_RC)"
            echo "⚠️  Run: source $SHELL_RC"
        else
            echo "⚠️  Please manually add $INSTALL_DIR to your PATH"
        fi
    fi
    
    echo "✅ Installed to: $INSTALL_DIR/nvim"
    
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    # Windows (Git Bash)
    echo "Detected: Windows"
    echo "Binary located at: target\release\nanovim.exe"
    echo "⚠️  Please manually copy it to a folder in your PATH"
else
    echo "⚠️  Unknown OS: $OSTYPE"
    echo "Binary located at: target/release/nanovim"
    echo "Please manually copy to a folder in your PATH"
fi

echo ""
echo "🎉 Installation complete!"
echo "💡 Test: nvim test.txt"