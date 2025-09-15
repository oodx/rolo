#!/bin/bash
set -e

# Configuration
INSTALL_DIR="$HOME/.local/bin/odx"
BINARY_NAME="rolo"
# Resolve repository root from bin/
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
DEPLOYABLE="${BINARY_NAME}"

# Extract version from Cargo.toml at repo root
VERSION=$(grep '^version' "$ROOT_DIR/Cargo.toml" 2>/dev/null | head -1 | cut -d'"' -f2 || echo "0.1.0")

# Display deployment ceremony
echo "╔══════════════════════════════════════════════════╗"
echo "║             ROLO DEPLOYMENT CEREMONY             ║"
echo "╠══════════════════════════════════════════════════╣"
echo "║ Package: $BINARY_NAME                            ║"
echo "║ Version: v$VERSION (Text Layout Engine)          ║"
echo "║ Target:  $INSTALL_DIR/$BINARY_NAME               ║"
echo "║ Features: Columns, Tables, Lists, ANSI-aware     ║"
echo "╚══════════════════════════════════════════════════╝"
echo ""

echo "🔨 Building rolo v$VERSION..."
cd "$ROOT_DIR"

# Check if we have a Cargo.toml
if [ ! -f "Cargo.toml" ]; then
    echo "❌ No Cargo.toml found! Run this from rolo project root."
    exit 1
fi

# Build with default features (minimal build)
if ! cargo build --release; then
    echo "❌ Build failed!"
    exit 1
fi

# Check if binary was created (at repo root)
if [ ! -f "$ROOT_DIR/target/release/${DEPLOYABLE}" ]; then
    echo "❌ Binary not found at target/release/${DEPLOYABLE}"
    exit 1
fi

echo "📦 Deploying to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"

if ! cp "$ROOT_DIR/target/release/${DEPLOYABLE}" "$INSTALL_DIR/$BINARY_NAME"; then
    echo "❌ Failed to copy binary to $INSTALL_DIR"
    exit 1
fi

if ! chmod +x "$INSTALL_DIR/$BINARY_NAME"; then
    echo "❌ Failed to make binary executable"
    exit 1
fi

# Verify deployment
if [ ! -x "$INSTALL_DIR/$BINARY_NAME" ]; then
    echo "❌ Binary is not executable at $INSTALL_DIR/$BINARY_NAME"
    exit 1
fi

# Test the binary
echo "🧪 Testing binary..."
if ! echo "Test Column1 Column2" | "$INSTALL_DIR/$BINARY_NAME" --cols 2 > /dev/null 2>&1; then
    echo "⚠️  Basic test failed, but binary is deployed (may need implementation)"
fi

echo ""
echo "╔══════════════════════════════════════════════════╗"
echo "║            DEPLOYMENT SUCCESSFUL!                ║"
echo "╠══════════════════════════════════════════════════╣"
echo "║  Deployed: rolo v$VERSION                        ║"
echo "║  Location: $INSTALL_DIR/$BINARY_NAME             ║"
echo "║  Features: Column/Table Layout, ANSI-aware       ║"
echo "╚══════════════════════════════════════════════════╝"
echo ""
echo "💡 Usage examples:"
echo "   echo \"a b c d\" | $INSTALL_DIR/$BINARY_NAME --cols 2"
echo "   cat data.tsv | $INSTALL_DIR/$BINARY_NAME --table"
echo "   ls -la | $INSTALL_DIR/$BINARY_NAME --list"
echo ""
echo "🔗 Pipeline integration:"
echo "   command | jynx | $INSTALL_DIR/$BINARY_NAME --cols 3 | boxy"
echo ""
echo "📖 Explore features:"
echo "   $INSTALL_DIR/$BINARY_NAME --help      # Show all options"
echo "   $INSTALL_DIR/$BINARY_NAME --version   # Show version info"
echo "   $ROOT_DIR/bin/test.sh run uat-columns # Demo column mode"
echo ""