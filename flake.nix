{
  description = "Home page";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rust_toolchain = fenix.packages.${system}.complete.toolchain;
        run_command = pkgs.writeShellApplication {
          name = "run";
          text = ''
            cargo run --offline --features bevy/dynamic_linking --features bevy/file_watcher \$@
          '';
        };
      in with pkgs; {
        nixpkgs.overlays = [ fenix.overlays.complete ];
        devShells.default = mkShell rec {

          nativeBuildInputs =
            [ pkg-config cmake pkg-config freetype expat fontconfig lld ];

          buildInputs = [
            run_command
            rust_toolchain
            lldb_15
            udev
            alsa-lib
            vulkan-loader
            libxkbcommon
            wayland 
            wasm-bindgen-cli
          ];
          LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
        };
      });
}

