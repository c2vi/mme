{
  description = "mme - the Main Mize Explorer";

  inputs = {
    # nixpkgs 35.11 still contains rust 1.73
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

 		flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, flake-utils, nixpkgs, fenix, crane, ... }@inputs: flake-utils.lib.eachDefaultSystem (system: 

############################## LET BINDINGS ##############################
let
  pkgs = nixpkgs.legacyPackages.${system};
  wasmToolchain = fenix.packages.${system}.combine [
    fenix.packages.${system}.targets.wasm32-unknown-unknown.latest.toolchain
    fenix.packages.${system}.latest.toolchain
  ];
  osToolchain = fenix.packages.${system}.latest.toolchain;
  wasmCrane = crane.lib.${system}.overrideToolchain wasmToolchain;
  osCrane = crane.lib.${system}.overrideToolchain osToolchain;

  wasmArtifacts = wasmCrane.buildDepsOnly ({
    src = self;
    doCheck = false; # tests does not work in wasm
    CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
  });

in {
############################## PACKAGES ##############################
    packages.default = osCrane.buildPackage {
      src = "${self}";
      cargoExtraArgs = "--bin mize --features os-binary";
    };

    packages = rec {
      wasm = wasmCrane.buildPackage {
        src = "${self}";
        CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        doCheck = false;
      };

    };

############################## DEV SHELLS ##############################
    devShells.default = pkgs.mkShell {

      nativeBuildInputs = with pkgs; [
        wasm-pack #pkg-config openssl #cargo rustc
        cargo-generate
        pkg-config 
        patchelf
        webkitgtk_4_1
        (fenix.packages.${system}.combine [ wasmToolchain osToolchain ])
        /*
        (fenix.packages.${system}.complete.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
        ])
        */
        lldb gdb
      ];

      buildInputs = with pkgs; [
        #pango
        #libsoup_3
        webkitgtk_4_1
        # gobject-introspection gtk4 atkmm
      ];

      shellHook = ''
        echo hiiiiiiiiiiiiii
        export LD_LIBRARY_PATH=${pkgs.webkitgtk_4_1}/lib:${pkgs.libsoup_3}/lib:${pkgs.glib.out}/lib:${pkgs.gtk3}/lib:${pkgs.cairo}/lib:${pkgs.gdk-pixbuf}/lib:${pkgs.libxkbcommon}/lib:${pkgs.fontconfig.lib}/lib:${pkgs.libsForQt5.full}/lib:${pkgs.stdenv.cc.cc.lib}/lib
        export CPLUS_INCLUDE_PATH=${pkgs.libsForQt5.full}/include
        export MME_QT_LIB=${pkgs.libsForQt5.full}/lib

        # i found that this is the env war to set where QT looks for platform plugins
        # at: https://forums.fedoraforum.org/showthread.php?326508-How-to-set-QT_QPA_PLATFORM_PLUGIN_PATH
        export QT_QPA_PLATFORM_PLUGIN_PATH=${pkgs.libsForQt5.full}/lib/qt-5.15.12/plugins/platforms/
      '';

    };


  }) // {

############################## SOME GLOBAL OUTPUTS ##############################
    inherit inputs self;
  };
}

