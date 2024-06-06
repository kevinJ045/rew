{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          efm-langserver
          nil
          nodePackages_latest.nodejs
          nodePackages_latest.typescript-language-server
          nodePackages_latest.prettier
          vscode-langservers-extracted
          nodePackages_latest.bash-language-server
        ];
      };
    }
  );
}
