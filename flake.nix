{
  description = "A simple Rust project using Flakes and Direnv";

  inputs = {
    # Import nixpkgs and provide Rust and other dependencies
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux"; # adjust for your system
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlay ];
      };
    in {
      devShell.${system} = pkgs.mkShell {
        buildInputs = [
          pkgs.rust-bin.stable.latest.default
        ];
      };
    };
}
