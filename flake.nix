{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.default = pkgs.stdenv.mkDerivation
        {
          name = "ircbot";
          src = ./.;
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          buildInputs = with pkgs; [
            meson
            ninja
            openssl
          ];
          buildPhase = ''
            meson compile
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp ircbot $out/bin
          '';
        };
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          gtk3
	   	  webkitgtk
	   	  glib
	   	  cairo
	   	  pango
	   	  atk
	   	  gdk-pixbuf
	   	  gobject-introspection
	   	  libsoup
	   	  harfbuzz
	   	  pkg-config
	   	  libwebsockets
	   	  jsoncpp
	   	  openssl
        ];
      };
    }
  );
}
