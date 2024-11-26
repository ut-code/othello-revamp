{
  description = "Rust WASM on Nix";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { nixpkgs, flake-utils, fenix, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
      toolchain = import ./toolchain.nix { inherit system fenix; };
    in
    {
      devShell = pkgs.mkShell {
        name = "WASM dev-shell";
        buildInputs = with pkgs; [
          nodejs-slim_latest
          pnpm
          just
          toolchain
          wasm-pack
          cargo-generate
        ];
      };
    });
}
