{
  description = "Rew Flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
  };

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      shaTable = {
        x86_64-linux = "sha256-9M67FZ4TzjoiGL73B8Jtwn38lW521yCLIqyvGzYCc50=";
        aarch64-linux = "sha256-J4E32qZNyqmJyFKBuU+6doRYL3ZSaEMSBlML+hSkj+o=";
        x86_64-darwin = "sha256-UnulsDS1LlrVR2+cz+4zgWxKqbkB5ch3T9UofGCZduQ=";
        aarch64-darwin = "sha256-mU7N/1vXzCP+mwjzLTsDkT+8YOJifwNju3Rv9Cq5Loo=";
      };

      forEachSystem =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          let
            pkgs = import nixpkgs { inherit system; };
            clang = pkgs.llvmPackages_16.clang;
            rustPlatform = pkgs.rustPlatform;
            version = "137.2.0";
            librusty_v8 = pkgs.fetchurl {
              name = "librusty_v8-${version}";
              url = "https://github.com/denoland/rusty_v8/releases/download/v${version}/librusty_v8_release_${pkgs.stdenv.hostPlatform.rust.rustcTarget}.a.gz";
              sha256 = shaTable.${system};
            };
          in
          f {
            inherit
              pkgs
              system
              clang
              rustPlatform
              librusty_v8
              ;
          }
        );
    in
    {
      packages = forEachSystem (
        {
          pkgs,
          system,
          rustPlatform,
          clang,
          librusty_v8,
        }:
        {
          default = rustPlatform.buildRustPackage {
            pname = "rew_runtime";
            version = "0.1.0";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [
              pkg-config
            ];

            RUSTY_V8_ARCHIVE = librusty_v8;

            buildInputs = [ ];

          };
        }
      );

      devShells = forEachSystem (
        { pkgs, system, ... }:
        {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              cargo
              clippy
              pkg-config
              rust-analyzer
              rustc
              rustfmt
            ];
          };
        }
      );
    };
}
