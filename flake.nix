
{
  description = "Rust + wasm32 devshell using oxalica/rust-overlay";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "rust-wasm-devshell";

          buildInputs = [
            # Rust nightly pinned using selectLatestNightlyWith (reliable)
            (pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
              targets = [ "wasm32-unknown-unknown" ];
            }))
          ];

          shellHook = ''
            echo "Welcome to Rust + wasm32 devshell"
            rustc --version
            cargo --version
            fish
          '';
        };
      }
    );
}

