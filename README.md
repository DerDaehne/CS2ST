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

## Installation

### Pre-built Binaries (Recommended)

Download the latest release for your platform from the [Releases](../../releases) page.

**Linux:**
```bash
# Download and make executable
chmod +x cs2-counter-strafe-trainer-linux-x64
./cs2-counter-strafe-trainer-linux-x64
```

**Windows:**
Just run `cs2-counter-strafe-trainer-windows-x64.exe`

### NixOS Users

You can run directly from the flake without installation:

```bash
# Run from GitHub (always latest main branch)
nix run github:DerDaehne/CS2ST

# Or clone and run locally
git clone https://github.com/DerDaehne/CS2ST.git
cd CS2ST
nix run
```

Or add to your `configuration.nix` or `home.nix`:

```nix
{ pkgs, ... }:
{
  environment.systemPackages = [
    (pkgs.callPackage (builtins.fetchGit {
      url = "https://github.com/DerDaehne/CS2ST";
      ref = "main";
    }) {})
  ];
}
```

### Building from Source

#### With Nix

```bash
# Enter development environment
nix develop

# Build and run
cargo run --release

# Or build with Nix directly
nix build
./result/bin/cs2-counter-strafe-trainer
```

#### Without Nix

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

## Releases

Pre-built binaries for Linux and Windows are available on the [Releases](../../releases) page.

### Creating a Release

Releases are automated via GitHub Actions. To create a new release:

```bash
# Use the release script
./release.sh 2.0.1

# Review changes
git show HEAD

# Push to trigger automated build
git push && git push origin v2.0.1
```

GitHub Actions will:
1. Build Linux binary with Nix (reproducible)
2. Cross-compile Windows binary
3. Create GitHub release with binaries
4. Generate SHA256 checksums

The workflow runs on every `v*.*.*` tag push.

### Manual Testing of CI Build Process

To test the build process locally before releasing:

```bash
# Install act (GitHub Actions local runner)
# https://github.com/nektos/act

# Test Linux build
act -j build-linux

# Test Windows build
act -j build-windows

# Test full release workflow
act -j create-release
```

## License

MIT
