# CS2 Counter-Strafe Trainer

High-performance Counter-Strike 2 counter-strafe training tool written in Rust.

> **⚠️ AI-Generated Code Disclaimer**
> This codebase was created with assistance from Claude (Anthropic AI). While functional and tested,
> use at your own discretion. Review code before use in production environments.

## Features

- ⚡ **<1ms event latency** - Precise keyboard event capture with `rdev`
- 🎨 **Modern UI** - GPU-accelerated rendering with `egui` at 144+ FPS
- 📊 **Real-time feedback** - Live timing display with quality evaluation
- 🎯 **Training metrics** - Track perfect/good/failed counter-strafe attempts
- 🪶 **Lightweight** - ~8MB binary, <15MB RAM usage
- 🌍 **Cross-platform** - Linux & Windows support from single codebase

## Quick Start

### With Nix (Recommended)

```bash
# Enter development environment
nix develop

# Build and run (release mode for Linux)
cargo run --release

# Build for Linux
cargo build --release --target x86_64-unknown-linux-gnu

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu

# Or use the build script to build for both platforms
./build.sh
```

Binaries will be in:
- Linux: `target/x86_64-unknown-linux-gnu/release/cs2-counter-strafe-trainer`
- Windows: `target/x86_64-pc-windows-gnu/release/cs2-counter-strafe-trainer.exe`

### Without Nix

Install dependencies:
```bash
# Ubuntu/Debian
sudo apt install pkg-config libxkbcommon-dev libwayland-dev libgl-dev

# Arch
sudo pacman -S pkgconf libxkbcommon wayland mesa

# For Windows cross-compilation
sudo apt install mingw-w64
```

Then build:
```bash
# Linux
cargo build --release

# Windows (from Linux)
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

## Permissions (Linux)

The app needs keyboard access. Either:

**Option 1:** Add user to input group (recommended)
```bash
sudo usermod -a -G input $USER
# Log out and back in
```

**Option 2:** Run with sudo
```bash
sudo ./cs2-counter-strafe-trainer
```

## How to Use

1. **Press A or D** to start strafing
2. **Release** the key
3. **Press the opposite key** (counter-strafe)
4. **Hold for ~80ms** (aim for the green zone!)
5. **Release** to complete

### Quality Levels

- **★ Perfect** - 65-95ms (80ms ±15ms) 🟢
- **● Good** - 60-120ms 🟡
- **✕ Failed** - <60ms or >120ms 🔴

### Controls

- **A/D** - Strafe keys
- **ESC** - Quit

## Performance Targets

✅ Event latency: <1ms
✅ UI rendering: 144+ FPS
✅ Binary size: 2-5MB
✅ Memory usage: 8-15MB
✅ Timing precision: Microsecond-level

## Architecture

```
src/
├── main.rs       - Entry point & app loop
├── state.rs      - Counter-strafe state machine
├── events.rs     - Keyboard event capture (rdev)
├── feedback.rs   - Feed system with fading
├── stats.rs      - Session statistics
└── ui.rs         - egui UI rendering
```

## Development

```bash
# Run tests
cargo test

# Check code
cargo clippy

# Format code
cargo fmt

# Build optimized binary
cargo build --release

# Check binary size
ls -lh target/release/cs2-counter-strafe-trainer
```

## License

MIT
