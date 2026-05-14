{
  description = "assembler dev-cli";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        tauriLibs = with pkgs; [
          glib
          gtk3
          gdk-pixbuf
          pango
          cairo
          webkitgtk_4_1
          libsoup_3
          libayatana-appindicator
          librsvg
        ];
      in
      {
        # devShells.default = pkgs.mkShell {
        #   packages = with pkgs; [
        #     rust-bin.stable.latest.default
        #     pkg-config
        #     wrapGAppsHook3
        #     xdg-utils
        #   ];

        #   buildInputs = with pkgs; [
        #     openssl
        #     dbus
        #   ] ++ tauriLibs;

        #   OPENSSL_NO_VENDOR = 1;
        #   shellHook = ''
        #     export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath tauriLibs}:$LD_LIBRARY_PATH
        #   '';
        # };

        devShells.default = (pkgs.buildFHSEnv {
          name = "assembler-fhs";

          targetPkgs = pkgs: with pkgs; [
            rust-bin.stable.latest.default
            pkg-config
            xdg-utils
            # appimagekit
            openssl
            dbus
          ] ++ tauriLibs;

        }).env;
        }
    );
}
