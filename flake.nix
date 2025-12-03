{
  description = "Rust development environment";

  inputs = {
    # Official NixOS package source
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # Community overlay for latest Rust toolchains
    rust-overlay.url = "github:oxalica/rust-overlay";

  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          # Rust Toolchain (pinned to stable or specific version)
          (rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
          })
          
          # Build Tools
          pkg-config
          clang
        ];
      };
    };
}
