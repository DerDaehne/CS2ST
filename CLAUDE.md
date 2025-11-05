# Counter-Strafe Trainer - Rust Rewrite Specification

## Project Overview

Rewrite the CS2 Counter-Strafe Training Tool from Python to Rust for maximum performance.

**Goal:** Native Rust application with <1ms event latency and 144+ FPS UI rendering.

## Current Python Implementation Issues

- Event latency: 5-15ms (unacceptable for 80ms measurements)
- UI rendering: 60-120 FPS with occasional frame drops
- Memory usage: 50-80MB
- Requires sudo for keyboard monitoring
- Processing locks causing delays during fast strafing

## Target Performance Metrics

- **Event Latency:** <1ms
- **UI FPS:** 144+ (locked, no drops)
- **Binary Size:** 2-5MB (statically linked)
- **Memory Usage:** 8-15MB
- **Timing Precision:** Microsecond-level for all measurements

## Technology Stack

### Core Libraries

```toml
[dependencies]
rdev = "0.5.3"              # Low-level keyboard event capture
egui = "0.28"               # Immediate mode GUI
eframe = "0.28"             # egui framework
chrono = "0.4"              # Timestamps
serde = { version = "1.0", features = ["derive"] }  # Serialization

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### Why These Libraries?

- **rdev:** Cross-platform keyboard/mouse event capture with <1ms latency
- **egui:** Immediate mode GUI, perfect for real-time updates, GPU-accelerated
- **eframe:** Native window management for egui
- **chrono:** Precise timestamp handling

## Architecture

### Main Components

```
src/
├── main.rs              # Entry point, event loop
├── state.rs             # Counter-strafe state machine
├── ui.rs                # egui UI rendering
├── events.rs            # Keyboard event handling
├── timing.rs            # Precise timing measurements
├── feedback.rs          # Feed entry management
└── stats.rs             # Session statistics
```

## Core Game Mechanics (CRITICAL - Must Be Exact!)

### Counter-Strafe Sequence

1. **Strafe:** User presses A or D
2. **Release:** User releases the key
3. **Counter:** User presses opposite key (timing doesn't matter, just must be AFTER release)
4. **Hold:** User holds counter-key for 80ms (measured precisely)
5. **Release:** User releases counter-key
6. **Shoot:** (Optional) User presses Space

### Timing Thresholds (DO NOT CHANGE!)

```rust
const OPTIMAL_HOLD_TIME: f32 = 0.080;      // 80ms
const MIN_HOLD_TIME: f32 = 0.060;          // 60ms  
const MAX_HOLD_TIME: f32 = 0.120;          // 120ms
const PERFECT_TOLERANCE: f32 = 0.015;      // ±15ms from optimal
const TIMEOUT_NO_COUNTER: f32 = 0.180;     // 180ms timeout if no counter-key
```

### Quality Evaluation

```rust
enum Quality {
    Perfect,  // Within 65-95ms (80ms ±15ms)
    Good,     // 60-120ms
    Failed,   // <60ms or >120ms
}
```

### Error Detection

Must detect these errors:
- **"Both keys pressed"** - Counter-key pressed before original released
- **"Too fast XXms"** - Hold time <60ms
- **"Too slow XXms"** - Hold time >120ms
- **"Shot too early"** - Shot before counter-key released
- **"No release first"** - Original key not released before counter

## State Machine

```rust
#[derive(Debug, Clone)]
enum CounterStrafeState {
    Idle,
    Strafing {
        key: Key,           // A or D
        start_time: Instant,
    },
    Released {
        original_key: Key,
        release_time: Instant,
    },
    CounterStrafing {
        original_key: Key,
        counter_key: Key,
        start_time: Instant,
    },
    Completed {
        hold_time: f32,
        quality: Quality,
    },
}
```

### State Transitions

```
Idle → Strafing (on A/D press)
Strafing → Released (on key release)
Released → CounterStrafing (on opposite key press)
Released → Idle (after 180ms timeout)
CounterStrafing → Completed (on key release)
Completed → Idle (after evaluation)
```

### Critical: Reset Conditions

1. **180ms Timeout:** If no counter-key after release
2. **Same Key Again:** If user presses same key while waiting for counter
3. **New Strafe:** If user starts new strafe before completing counter

## UI Design Specification

### Window Properties

```rust
const WINDOW_WIDTH: f32 = 420.0;
const WINDOW_HEIGHT: f32 = 320.0;
const WINDOW_TITLE: &str = "CS2 Counter-Strafe Trainer";
```

### Color Scheme (Exact Hex Values!)

```rust
// Background
const BG_COLOR: Color32 = Color32::from_rgba_premultiplied(12, 12, 18, 190);
const CARD_BG: Color32 = Color32::from_rgba_premultiplied(20, 22, 30, 200);

// Colors
const TEXT_COLOR: Color32 = Color32::from_rgb(240, 240, 245);
const ACCENT_COLOR: Color32 = Color32::from_rgb(100, 200, 255);  // Cyan
const GOOD_COLOR: Color32 = Color32::from_rgb(80, 220, 120);     // Green
const BAD_COLOR: Color32 = Color32::from_rgb(255, 90, 90);       // Red
const WARNING_COLOR: Color32 = Color32::from_rgb(255, 180, 60);  // Yellow
const NEUTRAL_COLOR: Color32 = Color32::from_rgb(130, 135, 150); // Gray
```

### Layout Structure

```
┌─────────────────────────────────────┐
│  Main Display Card (140px high)     │
│  ┌─────────────────────────────┐   │
│  │   56pt Number with Symbol   │   │
│  │      (or Status Text)        │   │
│  │                              │   │
│  │    TARGET: 80ms (if holding) │   │
│  └─────────────────────────────┘   │
├─────────────────────────────────────┤
│  Feed Card (90px high)              │
│  ┌─────────────────────────────┐   │
│  │  [*] PERFECT 78ms (fading)  │   │
│  │  [+] Good 85ms              │   │
│  │  [X] Too fast 45ms          │   │
│  └─────────────────────────────┘   │
├─────────────────────────────────────┤
│  Stats Bar                          │
│  [*] 5 (50%)  [+] 3  [X] 2        │
├─────────────────────────────────────┤
│  Controls hint (small gray text)    │
└─────────────────────────────────────┘
```

### Font Sizes

```rust
const HUGE_FONT: f32 = 56.0;    // Hold timer
const BIG_FONT: f32 = 28.0;     // Status text
const NORMAL_FONT: f32 = 18.0;  // Feed entries
const SMALL_FONT: f32 = 14.0;   // Stats & hints
```

### UI States Display

When **holding counter-key:**
```
        75 +
    TARGET: 80ms
```

When **idle:**
```
      READY
  Press A or D
```

When **strafing:**
```
     RELEASE
   Release A
```

When **released (waiting for counter):**
```
     COUNTER
     Press D
```

## Feed System

### Feed Entry Structure

```rust
struct FeedEntry {
    message: String,
    color: Color32,
    symbol: String,  // "[*]", "[+]", "[!]", "[X]"
    timestamp: Instant,
}
```

### Feed Properties

- **Max Entries:** 5 visible at once
- **Fade Duration:** 3 seconds visible, 1 second fade-out
- **Newest First:** Display in reverse chronological order

### Symbol Meanings

- `[*]` - Perfect (65-95ms)
- `[+]` - Good (60-120ms)
- `[!]` - Warning (too slow but within max)
- `[X]` - Failed (too fast, too slow, or error)

## Event Handling (CRITICAL FOR PERFORMANCE!)

### Requirements

1. **Non-blocking:** Event capture must not block UI thread
2. **Low latency:** <1ms from physical key press to state update
3. **Thread-safe:** Use channels for cross-thread communication

### Implementation Pattern

```rust
use std::sync::mpsc::{channel, Sender, Receiver};
use rdev::{listen, Event, EventType, Key};

// Spawn event listener thread
let (tx, rx) = channel();
std::thread::spawn(move || {
    listen(move |event| {
        tx.send(event).unwrap();
    }).expect("Could not listen");
});

// Main loop processes events from channel
loop {
    // Process all pending events (non-blocking)
    while let Ok(event) = rx.try_recv() {
        handle_keyboard_event(event, &mut state);
    }
    
    // Update UI at 144 FPS
    app.update();
}
```

### Key Mappings

```rust
// Only care about these keys
match event.event_type {
    EventType::KeyPress(Key::KeyA) => handle_strafe_press('a'),
    EventType::KeyPress(Key::KeyD) => handle_strafe_press('d'),
    EventType::KeyRelease(Key::KeyA) => handle_strafe_release('a'),
    EventType::KeyRelease(Key::KeyD) => handle_strafe_release('d'),
    EventType::KeyPress(Key::Space) => handle_shot(),
    EventType::KeyPress(Key::Escape) => quit(),
    _ => {} // Ignore all other keys
}
```

## Timing Precision

### Use std::time::Instant

```rust
use std::time::Instant;

// High precision timestamps
let start = Instant::now();
let elapsed = start.elapsed().as_secs_f32();  // Microsecond precision
```

### Hold Time Calculation

```rust
fn calculate_hold_time(start: Instant, end: Instant) -> f32 {
    end.duration_since(start).as_secs_f32()
}

fn evaluate_hold_time(hold_time: f32) -> Quality {
    if hold_time < MIN_HOLD_TIME {
        Quality::Failed
    } else if hold_time > MAX_HOLD_TIME {
        Quality::Failed
    } else if (hold_time - OPTIMAL_HOLD_TIME).abs() <= PERFECT_TOLERANCE {
        Quality::Perfect
    } else {
        Quality::Good
    }
}
```

## Statistics Tracking

```rust
struct Stats {
    total_attempts: u32,
    perfect_count: u32,
    good_count: u32,
    failed_count: u32,
}

impl Stats {
    fn perfect_percentage(&self) -> f32 {
        if self.total_attempts == 0 {
            0.0
        } else {
            (self.perfect_count as f32 / self.total_attempts as f32) * 100.0
        }
    }
}
```

## Window Management

### Requirements

- **Always on top:** Window should stay on top (optional, configurable)
- **Transparent background:** Use alpha channel for modern look
- **No window decorations:** Frameless window (NOFRAME in Python)
- **Draggable:** Can be moved by dragging anywhere
- **Resizable:** Should NOT be resizable (fixed size)

### eframe Configuration

```rust
use eframe::egui;

let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
        .with_decorations(false)
        .with_resizable(false)
        .with_transparent(true)
        .with_always_on_top(true),
    ..Default::default()
};
```

## Build Configuration

### Cargo.toml Profile

```toml
[profile.release]
opt-level = 3           # Maximum optimization
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization
strip = true            # Strip debug symbols
panic = 'abort'         # Smaller binary
```

### Compilation

```bash
# Release build (optimized)
cargo build --release

# The binary will be tiny: 2-5MB
ls -lh target/release/cs2-counter-strafe-trainer
```

## Nix Integration

### flake.nix Structure

```nix
{
  description = "CS2 Counter-Strafe Trainer (Rust)";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  
  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      
      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" "rustfmt" "clippy" ];
      };
      
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustToolchain
          pkg-config
          libxkbcommon
          wayland
          libGL
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
        ];
        
        LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [
          pkgs.libxkbcommon
          pkgs.wayland
          pkgs.libGL
          pkgs.xorg.libX11
          pkgs.xorg.libXcursor
          pkgs.xorg.libXrandr
          pkgs.xorg.libXi
        ]}";
        
        shellHook = ''
          echo "CS2 Counter-Strafe Trainer - Rust Development"
          echo "Commands:"
          echo "  cargo build --release  - Build optimized binary"
          echo "  cargo run --release    - Run optimized"
          echo ""
        '';
      };
      
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "cs2-counter-strafe-trainer";
        version = "2.0.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        
        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = with pkgs; [
          libxkbcommon
          wayland
          libGL
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
        ];
      };
    };
}
```

## Testing Requirements

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_perfect_timing() {
        let hold_time = 0.080;
        assert_eq!(evaluate_hold_time(hold_time), Quality::Perfect);
    }
    
    #[test]
    fn test_good_timing() {
        let hold_time = 0.100;
        assert_eq!(evaluate_hold_time(hold_time), Quality::Good);
    }
    
    #[test]
    fn test_too_fast() {
        let hold_time = 0.050;
        assert_eq!(evaluate_hold_time(hold_time), Quality::Failed);
    }
    
    #[test]
    fn test_state_transitions() {
        let mut state = CounterStrafeState::Idle;
        state = state.on_key_press(Key::KeyA);
        assert!(matches!(state, CounterStrafeState::Strafing { .. }));
    }
}
```

## Error Handling

### No Panic in Production

```rust
// Use Result types
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Graceful degradation
match event_receiver.try_recv() {
    Ok(event) => handle_event(event),
    Err(std::sync::mpsc::TryRecvError::Empty) => {}, // No events, continue
    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
        eprintln!("Event channel disconnected!");
        // Attempt recovery or exit gracefully
    }
}
```

## Performance Monitoring (Development)

Add FPS counter in debug mode:

```rust
#[cfg(debug_assertions)]
fn update(&mut self, ctx: &egui::Context) {
    let fps = ctx.frame_nr() as f32 / ctx.input().time;
    eprintln!("FPS: {:.1}", fps);
    
    // Should be 144+
    if fps < 100.0 {
        eprintln!("WARNING: Low FPS detected!");
    }
}
```

## Known Pitfalls to Avoid

1. **Don't use std::thread::sleep()** - Kills FPS
2. **Don't lock mutexes in event handler** - Use channels
3. **Don't allocate in hot path** - Pre-allocate feed entries
4. **Don't use String for known values** - Use &'static str
5. **Don't render every frame** - Use egui's request_repaint() only when needed
6. **Watch for key repeat events** - Filter duplicate key presses

## Accessibility

### Permissions (Linux)

User needs to be in `input` group OR run with sudo:

```bash
# Add user to input group
sudo usermod -a -G input $USER

# Or run with sudo
sudo ./cs2-counter-strafe-trainer
```

### Check at Runtime

```rust
#[cfg(target_os = "linux")]
fn check_permissions() -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let metadata = std::fs::metadata("/dev/input/event0")?;
    if metadata.permissions().mode() & 0o4 == 0 {
        eprintln!("Warning: May need sudo or input group membership");
    }
    Ok(())
}
```

## Success Criteria

The Rust implementation is successful when:

1. ✅ **Event latency <1ms** (measure with println! timestamps)
2. ✅ **UI renders at 144 FPS** without drops
3. ✅ **Binary size <5MB** (release build, stripped)
4. ✅ **Memory usage <15MB** during operation
5. ✅ **All timing thresholds exact** (80ms optimal, etc.)
6. ✅ **No false "Both keys pressed"** during fast strafing
7. ✅ **180ms timeout works** correctly
8. ✅ **Feed entries fade smoothly** over 4 seconds
9. ✅ **Statistics calculate correctly**
10. ✅ **Compiles on NixOS** with provided flake.nix

## Development Workflow

```bash
# 1. Create project
cargo init cs2-counter-strafe-trainer
cd cs2-counter-strafe-trainer

# 2. Add dependencies to Cargo.toml

# 3. Implement in order:
#    - Basic window (eframe)
#    - Event capture (rdev)
#    - State machine
#    - UI rendering
#    - Feed system
#    - Statistics

# 4. Test frequently
cargo run --release

# 5. Profile if needed
cargo install cargo-flamegraph
cargo flamegraph

# 6. Build final binary
cargo build --release
strip target/release/cs2-counter-strafe-trainer
```

## Questions to Ask if Stuck

- Is event capture non-blocking?
- Are we using Instant for all timestamps?
- Is the state machine handling all transitions?
- Are feed entries being cleaned up?
- Is the UI requesting repaint appropriately?
- Are we filtering key repeat events?

## Final Notes

- **Priority:** Performance over features
- **Target Platform:** Linux (NixOS) primary, Windows secondary
- **Code Style:** Follow Rust conventions (rustfmt, clippy)
- **Documentation:** Inline comments for complex logic
- **Git:** Commit frequently with clear messages

This specification should give you everything needed to implement a high-performance Rust version that eliminates all the latency issues of the Python implementation.

Good luck! 🦀🚀
