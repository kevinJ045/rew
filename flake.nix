{
  description = "Rew Flakes";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      rustToolchain = pkgs.rustPlatform;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = [
          pkgs.pkg-config
          pkgs.cargo
          pkgs.rustc
          pkgs.rustfmt
          pkgs.rust-analyzer
        ];
        buildInputs = [];
        QT_QPA_PLATFORM = "xcb";
        QT_STYLE_OVERRIDE = "kvantum";
        QT_QPA_PLATFORMTHEME = "";
        # QT_LOGGING_RULES = "qt6.debug=false";
      };

      packages.${system}.default = rustToolchain.buildRustPackage {
        pname = "rew";
        version = "0.1.0";
        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
        nativeBuildInputs = [
          pkgs.pkg-config
        ];
        buildInputs = [];
      };
    };
}
