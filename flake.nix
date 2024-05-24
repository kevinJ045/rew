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
