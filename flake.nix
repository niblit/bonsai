{
  description = "Rust development environment with latest stable toolchain";

  inputs = {
    # Official NixOS package source
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    # Community overlay for latest Rust toolchains
    rust-overlay.url = "github:oxalica/rust-overlay";

    # Utility for simple system-agnostic flake creation
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Apply the rust-overlay to nixpkgs
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Configure the Rust toolchain
        # usage of 'latest.default' ensures we get the newest stable release
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

      in
      {
        devShells.default = pkgs.mkShell {
          # Tools included in the environment
          buildInputs = with pkgs; [
            rustToolchain
            
            # Common system dependencies for Rust crates
            pkg-config
            clang

            # For profiling
            samply

            # For webdev
            trunk
            leptosfmt
            tailwindcss_4

            # Audit dependencies
            cargo-audit

            # Running rust as an interpreted language
            evcxr
          ];

          # Environment variables
          shellHook = ''
            export RUST_SRC_PATH=${rustToolchain}/lib/rustlib/src/rust/library
            echo "🦀 Rust $(rustc --version) environment activated!"
          '';
        };
      }
    );
}
