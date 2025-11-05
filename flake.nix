{
  description = "CS2 Counter-Strafe Trainer - High-performance Rust implementation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" "rust-analyzer" ];
          targets = [ "x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu" ];
        };

        # Libraries needed for egui/eframe and rdev
        buildInputs = with pkgs; [
          libxkbcommon
          wayland
          libGL
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libXxf86vm
          xorg.libXtst
          libevdev
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
          rustToolchain
          # Cross-compilation tools for Windows
          pkgsCross.mingwW64.stdenv.cc
        ];

      in {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

          shellHook = ''
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo "  CS2 Counter-Strafe Trainer - Rust Development"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo ""
            echo "Performance Targets:"
            echo "  • Event Latency: <1ms"
            echo "  • UI FPS: 144+"
            echo "  • Binary Size: 2-5MB"
            echo "  • Memory: 8-15MB"
            echo ""
            echo "Commands:"
            echo "  cargo build --release                              Build Linux binary"
            echo "  cargo build --release --target x86_64-pc-windows-gnu   Build Windows binary"
            echo "  cargo run --release                                Run optimized"
            echo "  cargo test                                         Run tests"
            echo "  cargo clippy                                       Lint code"
            echo "  cargo fmt                                          Format code"
            echo ""
            echo "Note: May need sudo or 'input' group for keyboard access on Linux"
            echo "      sudo usermod -a -G input \$USER"
            echo ""
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "cs2-counter-strafe-trainer";
          version = "2.0.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit nativeBuildInputs buildInputs;

          meta = with pkgs.lib; {
            description = "High-performance Counter-Strike 2 counter-strafe training tool";
            license = licenses.mit;
            platforms = platforms.linux;
          };
        };

        # Enable 'nix run' to directly execute the app
        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/cs2-counter-strafe-trainer";
        };
      }
    );
}
