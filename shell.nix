with import <nixpkgs> {};

mkShell {
  buildInputs = [
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
}
