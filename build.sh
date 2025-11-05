#!/usr/bin/env bash
set -e

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  CS2 Counter-Strafe Trainer - Multi-Platform Build"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Build for Linux
echo "Building for Linux (x86_64-unknown-linux-gnu)..."
cargo build --release --target x86_64-unknown-linux-gnu
echo "✓ Linux build complete"
echo ""

# Build for Windows
echo "Building for Windows (x86_64-pc-windows-gnu)..."
cargo build --release --target x86_64-pc-windows-gnu
echo "✓ Windows build complete"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Build Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ -f "target/x86_64-unknown-linux-gnu/release/cs2-counter-strafe-trainer" ]; then
    SIZE_LINUX=$(du -h "target/x86_64-unknown-linux-gnu/release/cs2-counter-strafe-trainer" | cut -f1)
    echo "Linux binary:   target/x86_64-unknown-linux-gnu/release/cs2-counter-strafe-trainer ($SIZE_LINUX)"
fi

if [ -f "target/x86_64-pc-windows-gnu/release/cs2-counter-strafe-trainer.exe" ]; then
    SIZE_WINDOWS=$(du -h "target/x86_64-pc-windows-gnu/release/cs2-counter-strafe-trainer.exe" | cut -f1)
    echo "Windows binary: target/x86_64-pc-windows-gnu/release/cs2-counter-strafe-trainer.exe ($SIZE_WINDOWS)"
fi

echo ""
echo "✓ All builds complete!"
echo ""
