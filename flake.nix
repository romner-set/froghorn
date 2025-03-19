{
  description = "froghorn";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    oxalica.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, oxalica }:
    with flake-utils.lib;
    eachSystem allSystems (system:
      let
        pkgs = nixpkgs.legacyPackages.${system}.extend oxalica.overlays.default;
      in rec {

        packages = {
          froghorn = let
            rustPlatform = pkgs.makeRustPlatform {
              cargo = pkgs.rust-bin.stable.latest.minimal;
              rustc = pkgs.rust-bin.stable.latest.minimal;
            };
          in rustPlatform.buildRustPackage rec {
            name = "froghorn";
            src = self;

            nativeBuildInputs = with pkgs; [
              autoPatchelfHook
              pkg-config
              makeWrapper
            ];
            buildInputs = with pkgs; [
              stdenv.cc.cc.lib
              glibc.dev
              alsa-lib.dev
              xorg.libX11.dev
              xorg.libXcursor.dev
            ];

            cargoLock = { lockFile = ./Cargo.lock; };
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          };
        };
        defaultPackage = packages.froghorn;
        formatter = pkgs.nixfmt;
      });
}
